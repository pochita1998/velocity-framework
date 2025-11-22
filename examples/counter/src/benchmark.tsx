import { createSignal, createEffect, render } from 'velocity-runtime';

/**
 * Simple benchmark comparing Velocity's fine-grained reactivity
 * vs traditional Virtual DOM approaches
 */

interface Item {
  id: number;
  value: number;
}

function Benchmark() {
  const [items, setItems] = createSignal<Item[]>([]);
  const [selectedId, setSelectedId] = createSignal<number | null>(null);
  const [updateCount, setUpdateCount] = createSignal(0);

  // Create 1000 items
  const createItems = () => {
    const newItems: Item[] = [];
    for (let i = 0; i < 1000; i++) {
      newItems.push({ id: i, value: 0 });
    }
    setItems(newItems);
  };

  // Update every 10th item - demonstrates fine-grained updates
  const updateEveryTenth = () => {
    const start = performance.now();

    setItems(prevItems =>
      prevItems.map((item, idx) =>
        idx % 10 === 0
          ? { ...item, value: item.value + 1 }
          : item
      )
    );

    const end = performance.now();
    setUpdateCount(c => c + 1);

    console.log(`Update ${updateCount() + 1} took ${(end - start).toFixed(2)}ms`);
  };

  // Update single item - showcases granular reactivity
  const updateSingleItem = () => {
    const start = performance.now();

    const randomId = Math.floor(Math.random() * items().length);
    setItems(prevItems =>
      prevItems.map(item =>
        item.id === randomId
          ? { ...item, value: item.value + 1 }
          : item
      )
    );

    const end = performance.now();
    console.log(`Single update took ${(end - start).toFixed(2)}ms`);
  };

  // Swap two items
  const swapItems = () => {
    const start = performance.now();

    setItems(prevItems => {
      const newItems = [...prevItems];
      if (newItems.length >= 2) {
        [newItems[0], newItems[newItems.length - 1]] =
        [newItems[newItems.length - 1], newItems[0]];
      }
      return newItems;
    });

    const end = performance.now();
    console.log(`Swap took ${(end - start).toFixed(2)}ms`);
  };

  return (
    <div style={{ padding: '2rem', fontFamily: 'system-ui' }}>
      <h1>⚡ Velocity Performance Benchmark</h1>

      <div style={{ marginTop: '2rem', marginBottom: '2rem' }}>
        <button onClick={createItems} style={{ margin: '0.5rem' }}>
          Create 1000 Items
        </button>
        <button onClick={updateEveryTenth} style={{ margin: '0.5rem' }}>
          Update Every 10th Item
        </button>
        <button onClick={updateSingleItem} style={{ margin: '0.5rem' }}>
          Update Random Item
        </button>
        <button onClick={swapItems} style={{ margin: '0.5rem' }}>
          Swap First/Last
        </button>
      </div>

      <div style={{ marginBottom: '1rem' }}>
        <strong>Total Items:</strong> {() => items().length} |
        <strong> Updates:</strong> {updateCount}
      </div>

      <div
        style={{
          maxHeight: '400px',
          overflow: 'auto',
          border: '1px solid #ccc',
          padding: '1rem'
        }}
      >
        {() => items().map(item => (
          <div
            key={item.id}
            onClick={() => setSelectedId(item.id)}
            style={{
              padding: '0.5rem',
              cursor: 'pointer',
              background: selectedId() === item.id ? '#e3f2fd' : 'transparent'
            }}
          >
            Item {item.id}: {item.value}
          </div>
        ))}
      </div>

      <div style={{ marginTop: '2rem', padding: '1rem', background: '#f5f5f5' }}>
        <h3>Why Velocity is Fast:</h3>
        <ul>
          <li>
            <strong>Fine-grained reactivity:</strong> Only updates the exact DOM
            nodes that changed
          </li>
          <li>
            <strong>No Virtual DOM:</strong> No diffing algorithm, no reconciliation
            overhead
          </li>
          <li>
            <strong>Direct DOM updates:</strong> Changes go straight to the DOM via
            signals
          </li>
          <li>
            <strong>Rust-powered tooling:</strong> Lightning-fast builds and dev
            server
          </li>
        </ul>
      </div>
    </div>
  );
}

// Uncomment to run benchmark
// render(() => <Benchmark />, document.getElementById('root')!);
