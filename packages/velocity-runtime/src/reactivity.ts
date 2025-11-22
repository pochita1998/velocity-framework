// Fine-grained reactivity system inspired by SolidJS

type Listener = () => void;
type Computation<T> = () => T;

interface Context {
  listener: Listener | null;
  sources: Set<Signal<any>> | null;
}

const context: Context = {
  listener: null,
  sources: null,
};

const runningComputations = new Set<Effect>();

class Signal<T> {
  private value: T;
  private listeners = new Set<Listener>();

  constructor(initialValue: T) {
    this.value = initialValue;
  }

  read(): T {
    // Track dependency
    if (context.listener) {
      this.listeners.add(context.listener);
      context.sources?.add(this);
    }
    return this.value;
  }

  write(newValue: T | ((prev: T) => T)): void {
    const nextValue = typeof newValue === 'function'
      ? (newValue as (prev: T) => T)(this.value)
      : newValue;

    if (this.value !== nextValue) {
      this.value = nextValue;
      // Notify all listeners
      this.listeners.forEach(listener => listener());
    }
  }

  dispose(): void {
    this.listeners.clear();
  }
}

class Effect {
  private sources = new Set<Signal<any>>();
  private fn: Listener;
  private disposed = false;

  constructor(fn: Listener) {
    this.fn = fn;
    this.execute();
  }

  execute(): void {
    if (this.disposed) return;

    // Cleanup old dependencies
    this.cleanup();

    const prevListener = context.listener;
    const prevSources = context.sources;

    context.listener = () => this.execute();
    context.sources = this.sources;

    try {
      this.fn();
    } finally {
      context.listener = prevListener;
      context.sources = prevSources;
    }
  }

  cleanup(): void {
    this.sources.forEach(signal => {
      const listeners = (signal as any).listeners as Set<Listener>;
      listeners.delete(() => this.execute());
    });
    this.sources.clear();
  }

  dispose(): void {
    this.cleanup();
    this.disposed = true;
  }
}

/**
 * Create a signal - a reactive primitive for storing state
 * @example
 * const [count, setCount] = createSignal(0);
 * console.log(count()); // 0
 * setCount(1);
 * console.log(count()); // 1
 */
export function createSignal<T>(initialValue: T): [() => T, (value: T | ((prev: T) => T)) => void] {
  const signal = new Signal(initialValue);
  return [
    () => signal.read(),
    (value) => signal.write(value)
  ];
}

/**
 * Create an effect - automatically runs when dependencies change
 * @example
 * const [count, setCount] = createSignal(0);
 * createEffect(() => {
 *   console.log('Count is:', count());
 * });
 */
export function createEffect(fn: Listener): () => void {
  const effect = new Effect(fn);
  return () => effect.dispose();
}

/**
 * Create a memo - a cached computed value
 * @example
 * const [count, setCount] = createSignal(0);
 * const doubled = createMemo(() => count() * 2);
 */
export function createMemo<T>(computation: Computation<T>): () => T {
  const [value, setValue] = createSignal<T>(undefined as T);

  createEffect(() => {
    setValue(computation());
  });

  return value;
}

/**
 * Batch multiple signal updates together
 */
export function batch(fn: () => void): void {
  // Simple implementation - could be optimized
  fn();
}

/**
 * Untrack a computation - don't create dependencies
 */
export function untrack<T>(fn: () => T): T {
  const prevListener = context.listener;
  const prevSources = context.sources;

  context.listener = null;
  context.sources = null;

  try {
    return fn();
  } finally {
    context.listener = prevListener;
    context.sources = prevSources;
  }
}
