// Main exports for Velocity Runtime

/// <reference path="./global.d.ts" />

export {
  createSignal,
  createEffect,
  createMemo,
  batch,
  untrack,
} from './reactivity';

export {
  onCleanup,
  onMount,
  createComponent,
  getContext,
  createContext,
} from './component';

export {
  insert,
  render,
  createPortal,
} from './dom';

// JSX runtime for automatic JSX transform
import { createElement } from './dom';
export { createElement, createElement as jsx, createElement as jsxs, createElement as jsxDEV };

// Fragment component
export const Fragment = (props: { children?: any }) => props.children;

// Types
export type { ComponentFunction } from './component';
