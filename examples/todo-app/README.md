# ⚡ Velocity Todo App

A beautiful, fully-functional todo application built with the Velocity Framework to showcase its features.

## 🎯 Features Demonstrated

### Fine-Grained Reactivity
- **Signals**: `createSignal` for state management
- **Memos**: `createMemo` for computed values (filtered todos, stats)
- **Surgical Updates**: Only changed DOM nodes update, not the entire component

### State Management
- Todo list with add, toggle, delete operations
- Filter system (All, Active, Completed)
- Real-time statistics
- Local state without external libraries

### UI Features
- ✨ Add new todos
- ✅ Mark todos as complete
- 🗑️ Delete individual todos
- 🧹 Clear all completed todos
- 📊 Real-time statistics
- 🔍 Filter todos by status
- 💅 Beautiful gradient design
- 📱 Responsive layout

## 🚀 Running the App

### Development
```bash
cd /home/jarreau/projects/velocity-framework/examples/todo-app
pnpm install
pnpm dev
```

Then open http://localhost:5173 in your browser!

### Production Build
```bash
pnpm build
pnpm preview
```

## 📁 Project Structure

```
todo-app/
├── src/
│   ├── index.tsx          # Main app component
│   └── velocity.d.ts      # Type definitions
├── index.html             # HTML with embedded styles
├── vite.config.ts         # Vite configuration
├── tsconfig.json          # TypeScript configuration
└── package.json           # Dependencies
```

## 🎨 Code Highlights

### Reactive State
```tsx
const [todos, setTodos] = createSignal<Todo[]>([]);
const [filter, setFilter] = createSignal<Filter>('all');
```

### Computed Values (Memos)
```tsx
const filteredTodos = createMemo(() => {
  const allTodos = todos();
  const currentFilter = filter();

  if (currentFilter === 'active') {
    return allTodos.filter(todo => !todo.completed);
  }
  return allTodos;
});
```

### Reactive DOM
```tsx
<button
  class={() => `btn-filter ${filter() === 'all' ? 'active' : ''}`}
  onClick={() => setFilter('all')}
>
  All ({() => stats().total})
</button>
```

## 🔬 Performance

This app showcases Velocity's performance advantages:

1. **No Virtual DOM** - Direct DOM updates via signals
2. **Fine-grained reactivity** - Only changed elements update
3. **Computed values cached** - Memos only recalculate when dependencies change
4. **Minimal re-renders** - Surgical updates, not full component re-renders

### Performance Comparison

Try this in the browser console:
```javascript
// Add 1000 todos
for (let i = 0; i < 1000; i++) {
  // Click add button or use app
}

// Toggle one todo - only that single element updates!
// In React, this might re-render the entire list
```

## 🧪 Try These Features

1. **Add todos** - Type and press Enter or click Add
2. **Toggle completion** - Click the checkbox
3. **Filter** - Click All/Active/Completed buttons
4. **Watch stats update** - Real-time computed values
5. **Delete todos** - Click the Delete button
6. **Clear completed** - Remove all completed todos at once

## 💡 Learning Points

### Signals vs State Hooks
```tsx
// React
const [count, setCount] = useState(0);
return <div>{count}</div>

// Velocity
const [count, setCount] = createSignal(0);
return <div>{count()}</div>  // Note: call as function
```

### Reactive Attributes
```tsx
// Static
<div class="todo-item">

// Reactive - updates only this attribute
<div class={() => `todo-item ${completed() ? 'completed' : ''}`}>
```

### Conditional Rendering
```tsx
{() => todos().length > 0 && (
  <div class="stats">Statistics here</div>
)}
```

## 🔧 Tech Stack

- **Velocity Runtime** - Fine-grained reactive framework
- **Vite** - Fast build tool and dev server
- **TypeScript** - Type safety
- **Pure CSS** - No CSS-in-JS libraries needed

## 📚 What Makes This Different

Compared to similar React/Vue apps:

| Feature | Velocity | React | Vue |
|---------|----------|-------|-----|
| Re-renders | None (direct updates) | Component tree | Component + descendants |
| Bundle Size | ~5kb | ~40kb | ~30kb |
| Fine-grained | ✅ Yes | ❌ No (VDOM) | ⚠️ Proxies |
| Learning Curve | Easy (if you know React) | Medium | Medium |

## 🎓 Next Steps

Extend this app to learn more:

1. **Add persistence** - Save to localStorage
2. **Add animations** - Use CSS transitions with reactive classes
3. **Add categories** - Expand the data model
4. **Add due dates** - More complex state management
5. **Add drag & drop** - Reorder todos

## 📖 Documentation

- [Velocity Framework](../../README.md)
- [Comparison with other frameworks](../../COMPARISON.md)
- [Getting Started Guide](../../GETTING_STARTED.md)

---

Built with ⚡ Velocity Framework - Fine-grained reactivity meets blazing speed!
