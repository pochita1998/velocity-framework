/**
 * Velocity Framework
 *
 * Blazingly fast JavaScript framework powered by Rust/WASM
 *
 * Main exports - Velocity native API
 */

import * as wasm from './velocity_wasm.js';

// Export Velocity primitives (native API)
export const createSignal = wasm.createSignal;
export const createEffect = wasm.createEffect;
export const createMemo = wasm.createMemo;

// Export React-compatible hooks
export const useState = wasm.useState;
export const useEffect = wasm.useEffect;
export const useMemo = wasm.useMemo;
export const useCallback = wasm.useMemo; // Same implementation

// Export DOM utilities
export const createElement = wasm.createElement;
export const createTextNode = wasm.createTextNode;
export const setText = wasm.setText;
export const appendChild = wasm.appendChild;
export const setAttribute = wasm.setAttribute;
export const addClass = wasm.addClass;
export const removeClass = wasm.removeClass;

// Export SSR/Hydration
export const renderToString = wasm.renderToString;
export const renderToStream = wasm.renderToStream;
export const hydrateRoot = wasm.hydrateRoot;
export const isSSR = wasm.isSSR;
export const serializeState = wasm.serializeState;
export const deserializeState = wasm.deserializeState;

// Export Resource management
export const createResource = wasm.createResource;
export const invalidateResource = wasm.invalidateResource;
export const refetchResource = wasm.refetchResource;
export const setResourceOptimistic = wasm.setResourceOptimistic;
export const getResourceState = wasm.getResourceState;
export const clearResourceCache = wasm.clearResourceCache;

// Export Error handling
export const createErrorBoundary = wasm.createErrorBoundary;
export const onError = wasm.onError;

// Export DevTools
export const enableDevTools = wasm.enableDevTools;
export const getMetrics = wasm.getMetrics;
export const mark = wasm.mark;
export const measure = wasm.measure;

// Export Fragment
export const Fragment = 'fragment';

// Export Signal class
export const Signal = wasm.Signal;

// Default export
export default {
  // Velocity API
  createSignal,
  createEffect,
  createMemo,

  // React API
  useState,
  useEffect,
  useMemo,
  useCallback,

  // DOM
  createElement,
  createTextNode,
  setText,
  appendChild,
  setAttribute,
  addClass,
  removeClass,
  Fragment,

  // SSR
  renderToString,
  renderToStream,
  hydrateRoot,
  isSSR,
  serializeState,
  deserializeState,

  // Resources
  createResource,
  invalidateResource,
  refetchResource,
  setResourceOptimistic,
  getResourceState,
  clearResourceCache,

  // Error handling
  createErrorBoundary,
  onError,

  // DevTools
  enableDevTools,
  getMetrics,
  mark,
  measure,

  // Signal class
  Signal,
};
