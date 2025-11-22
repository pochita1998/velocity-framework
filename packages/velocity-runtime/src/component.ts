// Component system with lifecycle hooks

import { createEffect } from './reactivity';

export type ComponentFunction = (props?: any) => any;
type CleanupFunction = () => void;

interface ComponentContext {
  cleanups: CleanupFunction[];
  owner: ComponentContext | null;
}

let currentContext: ComponentContext | null = null;

/**
 * Run a function with a component context
 */
function runWithContext<T>(fn: () => T, parent: ComponentContext | null = null): T {
  const prevContext = currentContext;
  currentContext = {
    cleanups: [],
    owner: parent,
  };

  try {
    return fn();
  } finally {
    currentContext = prevContext;
  }
}

/**
 * Cleanup the current component context
 */
function cleanupContext(context: ComponentContext): void {
  context.cleanups.forEach(cleanup => cleanup());
  context.cleanups = [];
}

/**
 * Register a cleanup function to run when component unmounts
 * @example
 * onCleanup(() => {
 *   console.log('Component unmounting');
 * });
 */
export function onCleanup(fn: CleanupFunction): void {
  if (currentContext) {
    currentContext.cleanups.push(fn);
  }
}

/**
 * Run a function when component mounts
 * @example
 * onMount(() => {
 *   console.log('Component mounted');
 * });
 */
export function onMount(fn: () => void | CleanupFunction): void {
  createEffect(() => {
    const cleanup = fn();
    if (cleanup) {
      onCleanup(cleanup);
    }
  });
}

/**
 * Create a component
 */
export function createComponent<T extends ComponentFunction>(
  component: T,
  props?: Parameters<T>[0]
): ReturnType<T> {
  return runWithContext(() => component(props), currentContext);
}

/**
 * Get the current component context for advanced use cases
 */
export function getContext(): ComponentContext | null {
  return currentContext;
}

/**
 * Create a context for passing data through component tree
 */
export function createContext<T>(defaultValue: T): {
  Provider: (props: { value: T; children: any }) => any;
  use: () => T;
} {
  const contexts = new WeakMap<ComponentContext, T>();

  return {
    Provider: (props: { value: T; children: any }) => {
      if (currentContext) {
        contexts.set(currentContext, props.value);
      }
      return props.children;
    },
    use: () => {
      let context = currentContext;
      while (context) {
        if (contexts.has(context)) {
          return contexts.get(context)!;
        }
        context = context.owner;
      }
      return defaultValue;
    }
  };
}
