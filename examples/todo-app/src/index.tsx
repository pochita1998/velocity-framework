import { createSignal, createMemo, render } from './velocity-wasm-runtime';

// Types
interface Todo {
  id: number;
  text: string;
  completed: boolean;
  createdAt: number;
}

type Filter = 'all' | 'active' | 'completed';

// Main App Component
function TodoApp() {
  // State
  const [todos, setTodos] = createSignal<Todo[]>([]);
  const [inputValue, setInputValue] = createSignal('');
  const [filter, setFilter] = createSignal<Filter>('all');

  // Computed values using memos
  const filteredTodos = createMemo(() => {
    const allTodos = todos();
    const currentFilter = filter();

    if (currentFilter === 'active') {
      return allTodos.filter(todo => !todo.completed);
    }
    if (currentFilter === 'completed') {
      return allTodos.filter(todo => todo.completed);
    }
    return allTodos;
  });

  const stats = createMemo(() => {
    const allTodos = todos();
    return {
      total: allTodos.length,
      active: allTodos.filter(t => !t.completed).length,
      completed: allTodos.filter(t => t.completed).length,
    };
  });

  // Actions
  const addTodo = () => {
    const text = inputValue().trim();
    if (!text) return;

    const newTodo: Todo = {
      id: Date.now(),
      text,
      completed: false,
      createdAt: Date.now(),
    };

    setTodos([...todos(), newTodo]);
    setInputValue('');
  };

  const toggleTodo = (id: number) => {
    setTodos(
      todos().map(todo =>
        todo.id === id ? { ...todo, completed: !todo.completed } : todo
      )
    );
  };

  const deleteTodo = (id: number) => {
    setTodos(todos().filter(todo => todo.id !== id));
  };

  const clearCompleted = () => {
    setTodos(todos().filter(todo => !todo.completed));
  };

  const handleKeyPress = (e: KeyboardEvent) => {
    if (e.key === 'Enter') {
      addTodo();
    }
  };

  const test = "hello"; 
  
  for (const char of test) {
    if (char === "h"){

    }
  }

  return (
    <div class="app-container">
      <div class="header">
        <h1>⚡ Velocity Todo app</h1>
        <p>A lightning-fast todo app built with fine-grained reactivity</p>
        <div class="wasm-badge">🦀 Powered by Rust + WebAssembly</div>
      </div>


      <div class="content">
        {/* Input */}
        <div class="input-container">
          <input
            type="text"
            placeholder="What needs to be done?"
            value={inputValue()}
            onInput={(e) => setInputValue((e.target as HTMLInputElement).value)}
            onKeyPress={handleKeyPress}
          />
          <button class="btn-add" onClick={addTodo}>
            Add
          </button>
        </div>

        {/* Filters */}
        <div class="filters">
          <button
            class={() => `btn-filter ${filter() === 'all' ? 'active' : ''}`}
            onClick={() => setFilter('all')}
          >
            All ({() => stats().total})
          </button>
          <button
            class={() => `btn-filter ${filter() === 'active' ? 'active' : ''}`}
            onClick={() => setFilter('active')}
          >
            Active ({() => stats().active})
          </button>
          <button
            class={() => `btn-filter ${filter() === 'completed' ? 'active' : ''}`}
            onClick={() => setFilter('completed')}
          >
            Completed ({() => stats().completed})
          </button>
        </div>

        {/* Stats */}
        {() => stats().total > 0 && (
          <div class="stats">
            <span>Total: {stats().total}</span>
            <span>Active: {stats().active}</span>
            <span>Completed: {stats().completed}</span>
          </div>
        )}

        {/* Todo List */}
        {() => filteredTodos().length > 0 ? (
          <ul class="todo-list">
            {() => filteredTodos().map(todo => (
              <li
                key={todo.id}
                class={() => `todo-item ${todo.completed ? 'completed' : ''}`}
              >
                <input
                  type="checkbox"
                  class="checkbox"
                  checked={todo.completed}
                  onChange={() => toggleTodo(todo.id)}
                />
                <span class="todo-text">{todo.text}</span>
                <button
                  class="btn-delete"
                  onClick={() => deleteTodo(todo.id)}
                >
                  Delete
                </button>
              </li>
            ))}
          </ul>
        ) : (
          <div class="empty-state">
            <div class="empty-state-icon">
              {() => filter() === 'completed' ? '✓' : '📝'}
            </div>
            <h3>
              {() => {
                if (filter() === 'completed') return 'No completed todos';
                if (filter() === 'active') return 'No active todos';
                return 'No todos yet';
              }}
            </h3>
            <p>
              {() =>
                filter() === 'all'
                  ? 'Add a todo to get started!'
                  : 'Try a different filter'
              }
            </p>
          </div>
        )}

        {/* Clear completed button */}
        {() => stats().completed > 0 && (
          <button class="btn-clear" onClick={clearCompleted}>
            Clear Completed ({stats().completed})
          </button>
        )}
      </div>
    </div>
  );
}

// Render the app
(async () => {
  await render(() => <TodoApp />, document.getElementById('root')!);
})();
