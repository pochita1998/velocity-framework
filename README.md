# ⚡ Velocity Framework

A blazingly fast JavaScript framework powered by Rust. Velocity combines the fine-grained reactivity of SolidJS with the speed of Rust-based build tooling to deliver unmatched performance.

## ✨ Features

- **⚡ Lightning Fast**: ~1ms compile time, <50ms Hot Module Replacement
- **🦀 Rust-Powered**: 10-40x faster than Webpack/Babel using SWC
- **⚛️ Fine-Grained Reactivity**: Signal-based with automatic tracking (no Virtual DOM)
- **🎯 Familiar Syntax**: Use JSX/TSX like React
- **🔥 Hot Module Replacement**: Instant updates with full state preservation
- **📦 Tiny Bundle**: Just 33KB gzipped runtime
- **🎮 Game-Ready**: Built-in game loop, input handling, and Canvas refs
- **⚙️ Zero Config**: Works out of the box

## 🚀 Quick Start

### Installation

**Easy install (recommended):**

```bash
# Clone and run the installer
git clone https://github.com/pochita1998/velocity-framework
cd velocity-framework
./install.sh
```

The installer will:
- ✅ Build and install the CLI globally
- ✅ Automatically add `~/.cargo/bin` to your PATH
- ✅ Set up everything you need to start developing

**Manual install:**

```bash
# Clone the repository
git clone https://github.com/pochita1998/velocity-framework
cd velocity-framework

# Install globally
cargo install --path crates/velocity-cli

# Add to PATH (if needed)
export PATH="$HOME/.cargo/bin:$PATH"
```

### Create a New Project

```bash
# Create a new Velocity project
velocity create my-app

# Start development
cd my-app
velocity dev
```

That's it! Your app is now running at http://localhost:3000 with hot reload enabled.

### Manual Setup

Or create a project manually:

**1. Create project structure:**

```bash
mkdir my-app && cd my-app
mkdir src dist public
```

**2. Create `src/index.tsx`:**

```tsx
import { createSignal, render, createElement } from 'velocity-runtime';

function Counter() {
  const [count, setCount] = createSignal(0);

  return (
    <div>
      <h1>Count: {count}</h1>
      <button onClick={() => setCount(count() + 1)}>
        Increment
      </button>
    </div>
  );
}

render(() => <Counter />, document.getElementById('root') as HTMLElement);
```

**3. Create `index.html`:**

```html
<!DOCTYPE html>
<html>
<head>
  <title>My Velocity App</title>
</head>
<body>
  <div id="root"></div>
  <script type="module" src="/dist/index.js"></script>
</body>
</html>
```

**4. Start development:**

```bash
velocity dev
```

The dev server will:
- ✅ Watch for file changes
- ✅ Auto-compile on save (<1ms)
- ✅ Hot reload the browser (<50ms total)
- ✅ Show compilation errors in an overlay

## 📖 CLI Commands

```bash
# Create a new project
velocity create <name> [--template counter|minimal]

# Start development server with HMR
velocity dev [--port 3000] [--root .]

# Build for production
velocity build [--root .] [--out-dir dist] [--minify]

# Compile a single file
velocity compile <file> [-o output.js] [--minify]

# Watch and recompile on changes
velocity watch <file> -o output.js

# Analyze bundle size
velocity analyze [--root .] [--format text|json]

# Show version and info
velocity info
```

## 🏗️ Architecture

### Why Velocity is Fast

**1. Fine-Grained Reactivity**
- Tracks dependencies at the signal level
- Updates only the exact DOM nodes that changed
- No Virtual DOM diffing overhead

**2. Rust-Based Compiler**
- 5-stage pipeline: Parser → Analyzer → Transformer → Optimizer → Codegen
- Uses SWC for 10-40x faster compilation
- ~1ms per file compile time

**3. Direct DOM Manipulation**
- No reconciliation algorithm
- Surgical updates to the DOM
- Maximum performance, minimal overhead

**4. Smart Hot Module Replacement**
- WebSocket-based with <50ms latency
- State preservation across updates
- Visual notifications and error overlay

## 📊 Performance

```
Compile Time:  ~1ms per file
Build Time:    ~5ms for 3 files
HMR Latency:   <50ms file save → browser update
Runtime Size:  33KB gzipped
```

## 🎯 API Reference

### Reactive Primitives

```tsx
import { createSignal, createEffect, createMemo } from 'velocity-runtime';

// Signals - reactive state
const [count, setCount] = createSignal(0);
setCount(count() + 1);

// Effects - run when dependencies change
createEffect(() => {
  console.log('Count is:', count());
});

// Memos - cached computed values
const doubled = createMemo(() => count() * 2);
```

### Control Flow

```tsx
import { For, Show } from 'velocity-runtime';

// List rendering with efficient keyed updates
<For each={items()}>
  {(item, index) => <div>{item.name}</div>}
</For>

// Conditional rendering
<Show when={isLoggedIn()} fallback={<Login />}>
  <Dashboard />
</Show>
```

### Game Development

```tsx
import { useAnimationFrame, useKeyboard, useMouse, createRef } from 'velocity-runtime';

function Game() {
  const canvasRef = createRef();
  const keys = useKeyboard();
  const mouse = useMouse();

  // Game loop with automatic cleanup
  useAnimationFrame((time, deltaTime) => {
    // Update game state
    if (keys()['ArrowUp']) movePlayer();

    // Render at 60 FPS
    const ctx = canvasRef.current.getContext('2d');
    ctx.fillRect(0, 0, 800, 600);
  });

  return <canvas ref={canvasRef} width="800" height="600" />;
}
```

### Rendering

```tsx
import { render, createElement } from 'velocity-runtime';

function App() {
  return <div>Hello Velocity!</div>;
}

render(() => <App />, document.getElementById('root') as HTMLElement);
```

### Lifecycle

```tsx
import { onMount, onCleanup } from 'velocity-runtime';

function Component() {
  onMount(() => {
    console.log('Component mounted');
  });

  onCleanup(() => {
    console.log('Component cleaning up');
  });

  return <div>Component</div>;
}
```

## 📁 Project Structure

```
my-app/
├── src/
│   └── index.tsx          # Your app code
├── dist/                  # Compiled output
│   ├── index.js
│   └── velocity-runtime.js
├── public/                # Static assets
│   └── style.css
├── index.html             # HTML entry point
├── package.json
└── README.md
```

## 🔧 Advanced Usage

### Custom Dev Server Port

```bash
velocity dev --port 8080
```

### Production Build with Minification

```bash
velocity build --minify
```

### Analyze Bundle Size

```bash
velocity analyze --format json > bundle-analysis.json
```

## 🌟 Examples

Check out the `/examples` directory:

- **counter** - Simple counter with signals and memos
- **doom-demo** - 🎮 DOOM-style raycaster game (game dev showcase)

Run examples:

```bash
cd examples/doom-demo
velocity dev
```

### DOOM Demo Features
The DOOM raycaster demo showcases Velocity's game development capabilities:
- Real-time 3D raycasting at 60 FPS
- Keyboard (WASD/Arrows) + Mouse controls
- Collision detection
- Minimap rendering
- FPS counter
- All powered by `useAnimationFrame`, `useKeyboard`, `useMouse`, and `createRef`

See [examples/doom-demo/README.md](examples/doom-demo/README.md) for technical details.

## 🛠️ Development

### Building from Source

```bash
# Clone the repo
git clone https://github.com/pochita1998/velocity-framework
cd velocity-framework

# Build all crates
cargo build --release

# Run tests
cargo test

# Build WASM runtime
cd crates/velocity-wasm
wasm-pack build --target web
```

### Project Structure

```
velocity-framework/
├── crates/
│   ├── velocity-cli/       # CLI tool
│   ├── velocity-compiler/  # Rust JSX compiler (SWC-based)
│   ├── velocity-wasm/      # WASM reactive runtime
│   └── velocity-bundler/   # Module bundler
├── examples/               # Example applications
├── docs/                   # Documentation
└── README.md
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

MIT License - see LICENSE file for details

## 🙏 Acknowledgments

- Inspired by [SolidJS](https://www.solidjs.com/) for fine-grained reactivity
- Built with [SWC](https://swc.rs/) for blazing fast compilation
- Powered by [Rust](https://www.rust-lang.org/) 🦀

---

Built with ⚡ by the Velocity team
