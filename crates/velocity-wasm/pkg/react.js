/**
 * React API Compatibility Layer for Velocity
 *
 * Drop-in replacement for React - uses Velocity's optimized Rust/WASM runtime
 * under the hood while maintaining React's familiar API.
 *
 * Usage:
 *   import { useState, useEffect, useMemo } from 'velocity/react';
 */

import * as Velocity from './index.js';

// Export React hooks (powered by Velocity)
export const useState = Velocity.useState;
export const useEffect = Velocity.useEffect;
export const useMemo = Velocity.useMemo;
export const useCallback = Velocity.useCallback;

// Export Velocity hooks (for interop)
export const createSignal = Velocity.createSignal;
export const createEffect = Velocity.createEffect;
export const createMemo = Velocity.createMemo;

// Export DOM utilities
export const createElement = Velocity.createElement;
export const Fragment = Velocity.Fragment;

// Default export for React compatibility
export default {
  // React hooks
  useState,
  useEffect,
  useMemo,
  useCallback,

  // Velocity hooks (for interop)
  createSignal,
  createEffect,
  createMemo,

  // DOM
  createElement,
  Fragment,
};
