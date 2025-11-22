// Velocity WASM Runtime Wrapper
// Provides a nice TypeScript API over the Rust/WASM runtime

import init, * as wasm from './wasm/velocity_wasm.js';

// Initialize WASM module
let wasmInitialized = false;

export async function initWasm() {
  if (!wasmInitialized) {
    await init();
    wasmInitialized = true;
    console.log('✨ Velocity WASM Runtime initialized');
  }
}

// Re-export createEffect directly
export { createEffect } from './wasm/velocity_wasm.js';
import { createEffect } from './wasm/velocity_wasm.js';

// Wrap createSignal to provide the right API
import { createSignal as wasmCreateSignal } from './wasm/velocity_wasm.js';

export function createSignal<T>(initialValue: T): [() => T, (value: T) => void] {
  if (!wasmInitialized) {
    throw new Error('WASM not initialized! Call initWasm() before using createSignal');
  }
  const [getter, setter] = wasmCreateSignal(initialValue);
  return [getter, setter];
}

// Helper to create a memo (computed value)
// This is a simple cached computation - it just returns a function that computes the value
export function createMemo<T>(computation: () => T): () => T {
  // For now, just return the computation function
  // In a full implementation, this would cache the result and track dependencies
  return computation;
}

// Render function that handles WASM initialization
export async function render(component: () => any, container: Element | null) {
  if (!container) {
    throw new Error('Container element not found');
  }

  // Initialize WASM first
  await initWasm();

  // Call the component to get the initial result
  const result = component();

  if (typeof result === 'string') {
    container.textContent = result;
  } else if (result instanceof Node) {
    container.innerHTML = '';
    container.appendChild(result);
  } else {
    console.error('Component returned unexpected type:', typeof result, result);
  }
}

// JSX createElement function
export function createElement(
  tag: string | Function,
  props: any,
  ...children: any[]
): HTMLElement | any {
  // Handle function components
  if (typeof tag === 'function') {
    return tag({ ...props, children });
  }

  // Create DOM element
  const element = document.createElement(tag);

  // Apply props
  if (props) {
    for (const [key, value] of Object.entries(props)) {
      if (key === 'children') continue;

      if (key.startsWith('on')) {
        // Event listener
        const eventName = key.slice(2).toLowerCase();
        element.addEventListener(eventName, value as EventListener);
      } else if (key === 'class' || key === 'className') {
        // Handle reactive and static classes
        if (typeof value === 'function') {
          createEffect(() => {
            element.className = (value as () => string)();
          });
        } else {
          element.className = value;
        }
      } else if (key === 'style') {
        if (typeof value === 'object') {
          Object.assign(element.style, value);
        } else {
          element.setAttribute('style', value);
        }
      } else if (typeof value === 'function') {
        // Reactive attribute
        createEffect(() => {
          const result = value();
          if (result != null && result !== false) {
            element.setAttribute(key, String(result));
          } else {
            element.removeAttribute(key);
          }
        });
      } else if (value != null && value !== false) {
        element.setAttribute(key, String(value));
      }
    }
  }

  // Append children
  const appendChildren = (children: any[]) => {
    for (const child of children) {
      if (Array.isArray(child)) {
        appendChildren(child);
      } else if (typeof child === 'function') {
        // Reactive child - create text node and set up effect
        const textNode = document.createTextNode('');
        element.appendChild(textNode);

        // Use the imported createEffect
        createEffect(() => {
          const value = child();
          textNode.textContent = String(value);
        });
      } else if (child != null && child !== false && child !== true) {
        if (child instanceof Node) {
          element.appendChild(child);
        } else {
          element.appendChild(document.createTextNode(String(child)));
        }
      }
    }
  };

  appendChildren(children);

  return element;
}

// Fragment component
export const Fragment = (props: { children?: any }) => props.children;

// Export for JSX transform
export const jsx = createElement;
export const jsxs = createElement;
export const jsxDEV = createElement;
