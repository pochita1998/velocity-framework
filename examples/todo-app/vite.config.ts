import { defineConfig } from 'vite';

export default defineConfig({
  esbuild: {
    jsx: 'transform',
    jsxFactory: 'createElement',
    jsxFragment: 'Fragment',
    jsxInject: `import { createElement, Fragment } from '/src/velocity-wasm-runtime'`,
  },
  server: {
    fs: {
      allow: ['..']
    }
  }
});
