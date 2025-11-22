# Getting Started with Velocity Framework

Velocity works just like **Vite** and **create-react-app** - it provides instant development with hot module replacement and zero configuration!

## Quick Start

### 1. Create a New Project

```bash
# Create project directory
mkdir my-velocity-app
cd my-velocity-app

# Create basic structure
mkdir -p src
```

### 2. Create Your Files

**package.json**:
```json
{
  "name": "my-velocity-app",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "velocity dev",
    "build": "velocity build"
  }
}
```

**index.html**:
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>My Velocity App</title>
</head>
<body>
  <div id="root"></div>
  <script type="module" src="/src/index.tsx"></script>
</body>
</html>
```

**src/index.tsx**:
```tsx
import { createSignal, render } from 'velocity-runtime';

function App() {
  const [count, setCount] = createSignal(0);

  return (
    <div>
      <h1>Hello Velocity!</h1>
      <p>Count: {count}</p>
      <button onClick={() => setCount(count() + 1)}>
        Click me
      </button>
    </div>
  );
}

render(() => <App />, document.getElementById('root')!);
```

### 3. Start Development Server

```bash
velocity dev
```

That's it! Open http://localhost:3000 in your browser.

## How It Works (Just Like Vite)

### Development Mode

```bash
velocity dev
```

This starts a development server with:
- ⚡ **Instant startup** - No bundling required
- 🔥 **Hot Module Replacement (HMR)** - Changes appear instantly (<50ms)
- 🎯 **State preservation** - Your app state persists across updates
- 📝 **TypeScript support** - Native TSX/JSX compilation
- 🗺️ **Source maps** - Debug your original code

### Production Build

```bash
velocity build
```

Creates an optimized production build:
- 📦 Minified JavaScript
- 🗜️ Tree-shaking and dead code elimination
- 📊 Bundle analysis
- 🗺️ Source maps for debugging

### Watch Mode

```bash
velocity watch src/App.tsx -o dist/App.js
```

Auto-recompile on file changes - perfect for library development.

## Features

### 🎯 Fine-grained Reactivity

Unlike React's Virtual DOM, Velocity updates only the exact DOM nodes that changed:

```tsx
import { createSignal, createEffect } from 'velocity-runtime';

function Counter() {
  const [count, setCount] = createSignal(0);

  // Effect automatically re-runs when count changes
  createEffect(() => {
    console.log('Count is now:', count());
  });

  return <button onClick={() => setCount(count() + 1)}>{count}</button>;
}
```

### 💾 Memoization

Cache expensive computations:

```tsx
import { createSignal, createMemo } from 'velocity-runtime';

function ExpensiveCalculation() {
  const [numbers, setNumbers] = createSignal([1, 2, 3, 4, 5]);

  // Only recalculates when numbers change
  const sum = createMemo(() => {
    console.log('Calculating sum...');
    return numbers().reduce((a, b) => a + b, 0);
  });

  return <div>Sum: {sum}</div>;
}
```

### 🔄 Lifecycle Hooks

```tsx
import { onMount, onCleanup } from 'velocity-runtime';

function Timer() {
  onMount(() => {
    const interval = setInterval(() => console.log('tick'), 1000);

    onCleanup(() => {
      clearInterval(interval);
    });
  });

  return <div>Check the console!</div>;
}
```

### 🌐 Data Fetching

Built-in resource management with caching:

```tsx
import { createResource } from 'velocity-runtime';

function UserProfile() {
  const [data, loading, error] = createResource('user-1', () =>
    fetch('/api/user/1').then(r => r.json())
  );

  if (loading()) return <div>Loading...</div>;
  if (error()) return <div>Error: {error()}</div>;

  return <div>Hello, {data().name}!</div>;
}
```

### 🏝️ Partial Hydration (Islands)

Only hydrate the interactive parts of your page:

```tsx
import { markIsland } from 'velocity-runtime';

function StaticContent() {
  return <p>This is static, no JavaScript needed!</p>;
}

function InteractiveButton() {
  const [count, setCount] = createSignal(0);

  return (
    <div ref={el => markIsland(el, 'counter-island')}>
      <button onClick={() => setCount(count() + 1)}>
        Clicks: {count}
      </button>
    </div>
  );
}
```

### 🖥️ Server-Side Rendering

```tsx
import { renderToString, hydrateRoot } from 'velocity-runtime';

// Server
const html = renderToString(() => <App />);

// Client
hydrateRoot('root'); // Hydrates the SSR content
```

### 🛡️ Error Boundaries

Catch errors at the component level:

```tsx
import { createErrorBoundary } from 'velocity-runtime';

const SafeComponent = createErrorBoundary(
  () => <MaybeProblematicComponent />,
  () => <div>Something went wrong!</div>
);
```

### 🔧 DevTools Integration

```tsx
import { enableDevTools } from 'velocity-runtime';

// Enable DevTools (development only)
if (import.meta.env.DEV) {
  enableDevTools();
}

// Now access in browser console:
// window.__VELOCITY_DEVTOOLS__.getSignals()
// window.__VELOCITY_DEVTOOLS__.getResources()
```

### 📊 Performance Monitoring

```tsx
import { mark, measure } from 'velocity-runtime';

function render() {
  mark('render-start');
  // ... rendering logic
  mark('render-end');

  const duration = measure('render', 'render-start', 'render-end');
  console.log(`Render took ${duration}ms`);
}
```

## CLI Commands

```bash
# Start dev server
velocity dev [--port 3000] [--root .]

# Build for production
velocity build [--root .] [--out-dir dist] [--minify]

# Compile single file
velocity compile src/App.tsx -o dist/App.js [--minify]

# Watch mode
velocity watch src/App.tsx -o dist/App.js

# Analyze bundle
velocity analyze [--root .] [--out-dir dist] [--format text|json]

# Show info
velocity info
```

## Comparison with Other Tools

| Feature | Velocity | Vite | create-react-app |
|---------|----------|------|------------------|
| Dev server startup | < 100ms | ~1s | ~10s |
| HMR speed | < 50ms | ~100ms | ~1s |
| Reactivity model | Fine-grained | VDOM | VDOM |
| Runtime size | ~5kb | ~40kb (React) | ~40kb |
| Build tool | Rust | esbuild/Rollup | Webpack |
| TypeScript | Native | Native | Babel |

## Project Structure

```
my-velocity-app/
├── src/
│   ├── index.tsx          # Entry point
│   ├── App.tsx            # Main app component
│   └── components/        # Your components
│       ├── Counter.tsx
│       └── TodoList.tsx
├── dist/                  # Build output
├── index.html             # HTML template
├── package.json           # Project config
└── tsconfig.json          # TypeScript config (optional)
```

## Next Steps

- Check out the [examples/counter](examples/counter) for a complete example
- Read the [API Reference](README.md#api-reference) for all available APIs
- See [docs/COMPILER.md](docs/COMPILER.md) for compiler details
- Learn about [HMR internals](docs/HMR.md)

## Tips

1. **Use signals for state** - They're more efficient than useState
2. **Memoize expensive computations** - Use createMemo() liberally
3. **Leverage HMR** - Your state persists across code changes
4. **Enable DevTools in dev** - Inspect signals and resources in real-time
5. **Use islands for SSR** - Only hydrate interactive parts

## Troubleshooting

### Dev server won't start
- Check if port is already in use
- Ensure you're in the project root
- Verify index.html exists

### HMR not working
- Check WebSocket connection in browser DevTools
- Ensure you're editing files in the watched directory
- Hard refresh (Cmd/Ctrl + Shift + R) if needed

### Build errors
- Check TypeScript/JSX syntax
- Ensure all imports are correct
- Look for circular dependencies

## Community

- [GitHub Issues](https://github.com/pochita1998/velocity-framework/issues)
- [Roadmap](docs/ROADMAP.md)

---

Built with ❤️ using Rust and TypeScript
