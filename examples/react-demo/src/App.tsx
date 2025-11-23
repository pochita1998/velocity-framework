/**
 * React Component using Velocity
 *
 * This is a standard React component that uses React hooks.
 * Velocity compiles it to optimized Rust/WASM while maintaining
 * the same React API you're familiar with.
 */

import { useState, useEffect, useMemo } from '../../crates/velocity-wasm/pkg/react.js';

function Counter() {
  // Standard React useState hook - powered by Velocity signals
  const [count, setCount] = useState(0);
  const [name, setName] = useState('React Developer');

  // Standard React useEffect hook - powered by Velocity effects
  useEffect(() => {
    console.log(`Count changed to: ${count()}`);
  });

  // Standard React useMemo hook - powered by Velocity memos
  const doubledCount = useMemo(() => count() * 2);

  const increment = () => setCount(count() + 1);
  const decrement = () => setCount(count() - 1);

  return (
    <div className="counter">
      <h1>Hello, {name()}!</h1>
      <p>This is a React component powered by Velocity</p>

      <div className="controls">
        <button onClick={decrement}>-</button>
        <span className="count">{count()}</span>
        <button onClick={increment}>+</button>
      </div>

      <div className="info">
        <p>Doubled: {doubledCount()}</p>
        <p>Using React hooks with Velocity's Rust/WASM runtime</p>
      </div>

      <div className="name-input">
        <label>Your name: </label>
        <input
          type="text"
          value={name()}
          onChange={(e) => setName(e.target.value)}
          placeholder="Enter your name"
        />
      </div>
    </div>
  );
}

// Mount the component
document.addEventListener('DOMContentLoaded', () => {
  const root = document.getElementById('root');
  if (root) {
    root.appendChild(Counter());
  }
});

export default Counter;
