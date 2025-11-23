import { createSignal, createMemo, render, createElement } from 'velocity-runtime';

function Counter() {
  const [count, setCount] = createSignal(0);
  const [clicks, setClicks] = createSignal(0);

  const doubled = createMemo(() => count() * 2);
  const isEven = createMemo(() => count() % 2 === 0);

  const increment = () => {
    setCount(c => c + 1);
    setClicks(c => c + 1);
  };

  const decrement = () => {
    setCount(c => c - 1);
    setClicks(c => c + 1);
  };

  const reset = () => {
    setCount(0);
    setClicks(c => c + 1);
  };

  return (
    <div>
      <h1>🚀 Velocity Counter - HMR Test</h1>

      <div class="counter">{count}</div>

      <div class="buttons">
        <button class="decrement" onClick={decrement}>
          - Decrement
        </button>
        <button class="reset" onClick={reset}>
          Reset
        </button>
        <button class="increment" onClick={increment}>
          + Increment
        </button>
      </div>

      <div class="stats">
        <div class="stat">
          <strong>Doubled:</strong> {doubled}
        </div>
        <div class="stat">
          <strong>Is Even:</strong> {() => isEven() ? 'Yes' : 'No'}
        </div>
        <div class="stat">
          <strong>Total Clicks:</strong> {clicks}
        </div>
      </div>
    </div>
  );
}

render(() => <Counter />, document.getElementById('root') as HTMLElement);
