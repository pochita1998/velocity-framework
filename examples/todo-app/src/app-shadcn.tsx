import { createSignal, createMemo } from './velocity-wasm-runtime';
import {
  Button,
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
  Input,
  Badge,
  Checkbox,
  Separator,
} from './components/ui';

// Types
interface Todo {
  id: number;
  text: string;
  completed: boolean;
  createdAt: number;
}

type Filter = 'all' | 'active' | 'completed';

// Main App Component with Shadcn UI
export function TodoAppShadcn() {
  // State
  const [todos, setTodos] = createSignal<Todo[]>([]);
  const [inputValue, setInputValue] = createSignal('');
  const [filter, setFilter] = createSignal<Filter>('all');

  // Computed values
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

  return (
    <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800 p-8">
      <div class="max-w-4xl mx-auto space-y-6">
        {/* Header Card */}
        <Card>
          <CardHeader>
            <div class="flex items-center justify-between">
              <div>
                <CardTitle>⚡ Velocity Todo</CardTitle>
                <CardDescription>
                  Lightning-fast todo app powered by Rust + WebAssembly
                </CardDescription>
              </div>
              <Badge variant="secondary" className="text-xs">
                🦀 Rust WASM
              </Badge>
            </div>
          </CardHeader>
        </Card>

        {/* Input Card */}
        <Card>
          <CardContent className="pt-6">
            <div class="flex gap-2">
              <Input
                placeholder="What needs to be done?"
                value={inputValue()}
                onInput={(e) => setInputValue((e.target as HTMLInputElement).value)}
                onKeyPress={handleKeyPress}
                className="flex-1"
              />
              <Button onClick={addTodo}>Add Todo</Button>
            </div>
          </CardContent>
        </Card>

        {/* Stats Card */}
        {() => stats().total > 0 && (
          <Card>
            <CardContent className="pt-6">
              <div class="grid grid-cols-3 gap-4 text-center">
                <div>
                  <div class="text-2xl font-bold text-primary">
                    {() => stats().total}
                  </div>
                  <div class="text-sm text-muted-foreground">Total</div>
                </div>
                <div>
                  <div class="text-2xl font-bold text-blue-600">
                    {() => stats().active}
                  </div>
                  <div class="text-sm text-muted-foreground">Active</div>
                </div>
                <div>
                  <div class="text-2xl font-bold text-green-600">
                    {() => stats().completed}
                  </div>
                  <div class="text-sm text-muted-foreground">Completed</div>
                </div>
              </div>
            </CardContent>
          </Card>
        )}

        {/* Filter Buttons */}
        <div class="flex gap-2 justify-center">
          <Button
            variant={() => filter() === 'all' ? 'default' : 'outline'}
            size="sm"
            onClick={() => setFilter('all')}
          >
            All ({() => stats().total})
          </Button>
          <Button
            variant={() => filter() === 'active' ? 'default' : 'outline'}
            size="sm"
            onClick={() => setFilter('active')}
          >
            Active ({() => stats().active})
          </Button>
          <Button
            variant={() => filter() === 'completed' ? 'default' : 'outline'}
            size="sm"
            onClick={() => setFilter('completed')}
          >
            Completed ({() => stats().completed})
          </Button>
        </div>

        {/* Todos Card */}
        {() => filteredTodos().length > 0 ? (
          <Card>
            <CardContent className="pt-6">
              <div class="space-y-2">
                {() => filteredTodos().map((todo, index) => (
                  <div key={todo.id}>
                    {index > 0 && <Separator className="my-2" />}
                    <div class="flex items-center gap-3 p-3 rounded-lg hover:bg-accent transition-colors">
                      <Checkbox
                        checked={todo.completed}
                        onChange={() => toggleTodo(todo.id)}
                      />
                      <span
                        class={() =>
                          `flex-1 ${todo.completed ? 'line-through text-muted-foreground' : ''}`
                        }
                      >
                        {todo.text}
                      </span>
                      <Button
                        variant="destructive"
                        size="sm"
                        onClick={() => deleteTodo(todo.id)}
                      >
                        Delete
                      </Button>
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
            {() => stats().completed > 0 && (
              <CardFooter>
                <Button
                  variant="outline"
                  onClick={clearCompleted}
                  className="w-full"
                >
                  Clear Completed ({() => stats().completed})
                </Button>
              </CardFooter>
            )}
          </Card>
        ) : (
          <Card>
            <CardContent className="py-12">
              <div class="text-center space-y-4">
                <div class="text-6xl">
                  {() => filter() === 'completed' ? '✓' : '📝'}
                </div>
                <div>
                  <h3 class="text-xl font-semibold text-muted-foreground">
                    {() => {
                      if (filter() === 'completed') return 'No completed todos';
                      if (filter() === 'active') return 'No active todos';
                      return 'No todos yet';
                    }}
                  </h3>
                  <p class="text-sm text-muted-foreground mt-2">
                    {() =>
                      filter() === 'all'
                        ? 'Add a todo to get started!'
                        : 'Try a different filter'
                    }
                  </p>
                </div>
              </div>
            </CardContent>
          </Card>
        )}
      </div>
    </div>
  );
}
