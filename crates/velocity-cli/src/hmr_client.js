// Velocity HMR Client
// Connects to the dev server via WebSocket and applies hot updates

console.log('🔥 Velocity HMR Client loaded');

class VelocityHMR {
  constructor() {
    this.ws = null;
    this.modules = new Map();
    this.isReconnecting = false;
    this.connect();
  }

  connect() {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}/__hmr`;

    console.log(`[HMR] Connecting to ${wsUrl}...`);

    this.ws = new WebSocket(wsUrl);

    this.ws.onopen = () => {
      console.log('[HMR] ✅ Connected to dev server');
      this.isReconnecting = false;
    };

    this.ws.onmessage = (event) => {
      try {
        const message = JSON.parse(event.data);
        this.handleMessage(message);
      } catch (e) {
        console.error('[HMR] Failed to parse message:', e);
      }
    };

    this.ws.onerror = (error) => {
      console.error('[HMR] WebSocket error:', error);
    };

    this.ws.onclose = () => {
      console.log('[HMR] ⚠️  Disconnected from dev server');
      if (!this.isReconnecting) {
        this.isReconnecting = true;
        console.log('[HMR] Reconnecting in 1s...');
        setTimeout(() => this.connect(), 1000);
      }
    };
  }

  handleMessage(message) {
    switch (message.type) {
      case 'connected':
        console.log('[HMR] 🎉 Hot Module Replacement enabled');
        break;

      case 'update':
        this.applyUpdate(message);
        break;

      case 'full-reload':
        console.log(`[HMR] 🔄 Full reload: ${message.reason}`);
        window.location.reload();
        break;

      case 'error':
        this.showError(message.error);
        break;

      default:
        console.warn('[HMR] Unknown message type:', message.type);
    }
  }

  applyUpdate(message) {
    const { module, code, timestamp, dependents } = message;
    console.log(`[HMR] 📦 Updating module: ${module} (${timestamp})`);

    // Log cascade info if there are dependents
    if (dependents && dependents.length > 0) {
      console.log(`[HMR] 🔗 Cascade update will affect ${dependents.length} dependent module(s)`);
    }

    try {
      // Capture current state before update
      const savedState = this.captureState();

      // Create a blob URL for the new module
      const blob = new Blob([code], { type: 'application/javascript' });
      const url = URL.createObjectURL(blob);

      // Dynamically import the updated module
      import(url)
        .then((newModule) => {
          console.log(`[HMR] ✅ Module ${module} updated successfully`);

          // Store the module
          const oldModule = this.modules.get(module);
          this.modules.set(module, newModule);

          // Clean up the blob URL
          URL.revokeObjectURL(url);

          // Try hot replacement with state preservation
          if (this.tryHotReplace(module, oldModule, newModule, savedState)) {
            console.log('[HMR] 🔥 Hot replaced without reload');

            // Apply cascade updates to dependent modules
            if (dependents && dependents.length > 0) {
              this.applyCascadeUpdates(dependents, savedState);
            }

            this.showNotification(`Updated: ${module}`, 'success');
          } else {
            // Fall back to full reload if hot replacement fails
            console.log('[HMR] 🔄 Reloading page to apply changes...');
            setTimeout(() => window.location.reload(), 100);
          }
        })
        .catch((error) => {
          console.error(`[HMR] ❌ Failed to update ${module}:`, error);
          this.showError(error.message);
        });
    } catch (error) {
      console.error('[HMR] ❌ Update error:', error);
      this.showError(error.message);
    }
  }

  applyCascadeUpdates(dependents, savedState) {
    console.log(`[HMR] 🔗 Applying cascade updates to ${dependents.length} module(s)...`);

    dependents.forEach((depModule) => {
      console.log(`[HMR] ↳ Reloading dependent: ${depModule}`);

      // Request the dependent module to be recompiled and sent
      // For now, we'll just invalidate the cached module
      // In a full implementation, the server would detect and recompile dependents
      this.modules.delete(depModule);
    });

    // After invalidating dependents, we could trigger a targeted re-render
    // This would require deeper integration with the Velocity runtime
  }

  captureState() {
    const state = {
      signals: new Map(),
      dom: null,
      timestamp: Date.now()
    };

    // Capture Velocity signal values if the runtime is loaded
    if (window.__velocity_runtime__) {
      const runtime = window.__velocity_runtime__;

      // Capture all active signals
      if (runtime.signals) {
        runtime.signals.forEach((signal, id) => {
          try {
            // Store current value of each signal
            if (typeof signal === 'function') {
              state.signals.set(id, signal());
            }
          } catch (e) {
            console.warn(`[HMR] Could not capture signal ${id}:`, e);
          }
        });
      }
    }

    // Capture current DOM state
    const root = document.getElementById('root');
    if (root) {
      state.dom = root.cloneNode(true);
    }

    console.log(`[HMR] 💾 Captured state: ${state.signals.size} signals`);
    return state;
  }

  tryHotReplace(modulePath, oldModule, newModule, savedState) {
    try {
      // Check if this is a component module
      const isComponentModule = modulePath.includes('.tsx') || modulePath.includes('.jsx');

      if (!isComponentModule) {
        // Non-component modules need full reload
        return false;
      }

      // If Velocity runtime is not available, can't hot replace
      if (!window.__velocity_runtime__) {
        console.log('[HMR] Runtime not available for hot replace');
        return false;
      }

      const runtime = window.__velocity_runtime__;

      // Re-render with the new module while restoring signal values
      if (savedState.signals.size > 0 && runtime.signals) {
        console.log(`[HMR] 🔄 Restoring ${savedState.signals.size} signal values...`);

        // Restore signal values after a brief delay to allow new signals to be created
        setTimeout(() => {
          let restored = 0;
          savedState.signals.forEach((value, id) => {
            const signal = runtime.signals.get(id);
            if (signal && signal.set) {
              try {
                signal.set(value);
                restored++;
              } catch (e) {
                console.warn(`[HMR] Could not restore signal ${id}:`, e);
              }
            }
          });
          console.log(`[HMR] ✅ Restored ${restored}/${savedState.signals.size} signals`);
        }, 50);
      }

      // Module successfully replaced with state preservation
      return true;

    } catch (e) {
      console.error('[HMR] Hot replace failed:', e);
      return false;
    }
  }

  showNotification(message, type = 'info') {
    // Create notification element
    const notification = document.createElement('div');
    notification.style.cssText = `
      position: fixed;
      top: 20px;
      right: 20px;
      padding: 12px 20px;
      background: ${type === 'success' ? '#10b981' : type === 'error' ? '#ef4444' : '#3b82f6'};
      color: white;
      border-radius: 8px;
      box-shadow: 0 4px 6px rgba(0,0,0,0.1);
      font-family: system-ui, -apple-system, sans-serif;
      font-size: 14px;
      z-index: 999999;
      animation: slideIn 0.3s ease-out;
    `;
    notification.textContent = message;

    // Add animation
    const style = document.createElement('style');
    style.textContent = `
      @keyframes slideIn {
        from {
          transform: translateX(400px);
          opacity: 0;
        }
        to {
          transform: translateX(0);
          opacity: 1;
        }
      }
    `;
    document.head.appendChild(style);
    document.body.appendChild(notification);

    // Auto-remove after 3 seconds
    setTimeout(() => {
      notification.style.animation = 'slideIn 0.3s ease-out reverse';
      setTimeout(() => notification.remove(), 300);
    }, 3000);
  }

  showError(errorMessage) {
    console.error('[HMR] Error:', errorMessage);

    // Create error overlay
    const overlay = document.createElement('div');
    overlay.id = 'velocity-error-overlay';
    overlay.style.cssText = `
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: rgba(0, 0, 0, 0.9);
      color: #ef4444;
      padding: 40px;
      font-family: 'Monaco', 'Menlo', monospace;
      font-size: 14px;
      z-index: 9999999;
      overflow: auto;
    `;

    overlay.innerHTML = `
      <div style="max-width: 800px; margin: 0 auto;">
        <h1 style="color: #ef4444; margin-bottom: 20px;">
          ❌ Compilation Error
        </h1>
        <pre style="background: #1e1e1e; padding: 20px; border-radius: 8px; overflow-x: auto; color: #fca5a5;">
${errorMessage}</pre>
        <button onclick="document.getElementById('velocity-error-overlay').remove()"
                style="margin-top: 20px; padding: 10px 20px; background: #ef4444; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 14px;">
          Close
        </button>
        <p style="margin-top: 20px; color: #9ca3af;">
          Fix the error and save the file - HMR will automatically update.
        </p>
      </div>
    `;

    // Remove existing overlay if any
    const existing = document.getElementById('velocity-error-overlay');
    if (existing) {
      existing.remove();
    }

    document.body.appendChild(overlay);
  }

  send(message) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
    }
  }
}

// Initialize HMR client
window.__velocity_hmr__ = new VelocityHMR();

// Export for module usage
export default window.__velocity_hmr__;
