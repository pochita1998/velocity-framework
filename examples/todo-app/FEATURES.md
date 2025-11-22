# Todo App Features & Implementation

## Architecture Overview

### State Management (Signals)
```tsx
// Reactive primitives
const [todos, setTodos] = createSignal<Todo[]>([]);
const [inputValue, setInputValue] = createSignal('');
const [filter, setFilter] = createSignal<Filter>('all');
```

### Computed Values (Memos)
```tsx
// Automatically recalculates when dependencies change
const filteredTodos = createMemo(() => {
  const allTodos = todos();
  const currentFilter = filter();
  // Filter logic...
});

const stats = createMemo(() => {
  const allTodos = todos();
  return {
    total: allTodos.length,
    active: allTodos.filter(t => !t.completed).length,
    completed: allTodos.filter(t => t.completed).length,
  };
});
```

## Key Features

### 1. Add Todos
- Input field with two-way binding
- Enter key support
- Automatic focus management
- Validation (no empty todos)

### 2. Toggle Completion
- Checkbox with reactive state
- Visual feedback (strikethrough, opacity)
- Updates stats automatically

### 3. Delete Todos
- Individual delete buttons
- Smooth removal animation
- Updates stats automatically

### 4. Filter System
- Three filters: All, Active, Completed
- Active filter highlighting
- Count badges on each filter
- Filtered list updates instantly

### 5. Statistics
- Total todos count
- Active todos count
- Completed todos count
- Computed in real-time

### 6. Clear Completed
- Bulk deletion of completed todos
- Only visible when there are completed todos
- Shows count of items to be cleared

### 7. Empty States
- Different messages for each filter
- Helpful guidance for users
- Icon changes based on context

## Reactivity in Action

### Example 1: Adding a Todo
```
User types "Buy milk" → inputValue() updates
User clicks Add → addTodo() runs
  → New todo added to todos()
  → filteredTodos() automatically recalculates
  → stats() automatically recalculates
  → DOM updates surgically:
    - New <li> element added
    - Stats numbers update
    - Filter badges update
    - Input clears
```

### Example 2: Toggling a Todo
```
User clicks checkbox → toggleTodo(id) runs
  → todos() updates (one item's completed flag flips)
  → filteredTodos() recalculates (may remove/keep item)
  → stats() recalculates (counts change)
  → DOM updates surgically:
    - That specific <li> gets/removes 'completed' class
    - Checkbox updates
    - Stats numbers update
    - Filter badges update
```

### Example 3: Changing Filters
```
User clicks "Active" filter → setFilter('active')
  → filter() updates
  → filteredTodos() recalculates
  → DOM updates surgically:
    - Only visible todos change (add/remove from DOM)
    - Active button gets highlighted
    - Other buttons lose highlight
    - Empty state may appear/disappear
```

## Performance Optimizations

### 1. Memoization
Computed values cache results:
```tsx
const stats = createMemo(() => {
  // Only runs when todos() changes
  // Not when filter() changes!
  const allTodos = todos();
  return { /* calculated stats */ };
});
```

### 2. Fine-Grained Updates
Only exact DOM nodes that need updating change:
- Toggle a todo → Only that `<li>` and stats update
- Change filter → Only visible todos and button classes update
- Add todo → Only new `<li>` and stats update

### 3. No Re-renders
Unlike React:
- ❌ React: Toggle todo → Entire TodoList component re-renders
- ✅ Velocity: Toggle todo → Only that specific checkbox/li updates

### 4. Reactive Attributes
```tsx
<div class={() => `todo-item ${todo.completed ? 'completed' : ''}`}>
```
Only the `class` attribute updates, not the entire element.

## Code Patterns Used

### 1. Immutable Updates
```tsx
// Don't mutate
todos().push(newTodo); // ❌

// Create new array
setTodos([...todos(), newTodo]); // ✅
```

### 2. Derived State
```tsx
// Don't store filtered todos separately
const [filteredTodos, setFilteredTodos] = createSignal([]); // ❌

// Compute from source of truth
const filteredTodos = createMemo(() => /* ... */); // ✅
```

### 3. Reactive Rendering
```tsx
// Static
<div>{todos().length}</div>

// Reactive - updates when todos changes
{() => todos().length}
```

### 4. Conditional Rendering
```tsx
{() => condition() && <Component />}
{() => condition() ? <ComponentA /> : <ComponentB />}
```

## TypeScript Benefits

### Type Safety
```tsx
interface Todo {
  id: number;
  text: string;
  completed: boolean;
  createdAt: number;
}

type Filter = 'all' | 'active' | 'completed';

// Full autocomplete and type checking!
const [filter, setFilter] = createSignal<Filter>('all');
```

### JSX Type Checking
All HTML elements have proper types:
- Autocomplete for all attributes
- Event handlers typed correctly
- Refs typed properly

## Styling Approach

### Embedded CSS
- All styles in index.html
- No CSS-in-JS overhead
- Modern CSS features:
  - CSS Grid/Flexbox
  - CSS Variables (gradients)
  - Transitions
  - Hover effects

### Reactive Classes
```tsx
class={() => `btn-filter ${filter() === 'all' ? 'active' : ''}`}
```
Classes update without re-rendering the element.

## Possible Enhancements

### Easy Additions
1. **Persistence**: Add localStorage save/load
2. **Edit Mode**: Double-click to edit todo text
3. **Priorities**: Add priority levels (high, medium, low)
4. **Search**: Filter by text search

### Advanced Features
1. **Drag & Drop**: Reorder todos
2. **Categories**: Group todos by category
3. **Due Dates**: Add calendar integration
4. **Animations**: Add/remove transitions
5. **Undo/Redo**: State history management

## Learning Resources

Compare this with:
- [React TodoMVC](https://todomvc.com/examples/react/)
- [Vue TodoMVC](https://todomvc.com/examples/vue/)
- [SolidJS TodoMVC](https://todomvc.com/examples/solidjs/)

Notice how similar the code is, but with better performance!
