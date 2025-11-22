/* tslint:disable */
/* eslint-disable */
export function createEffect(func: Function): void;
export function addClass(element: Element, _class: string): void;
export function greet(name: string): void;
export function setText(element: Element, text: string): void;
export function removeClass(element: Element, _class: string): void;
export function setAttribute(element: Element, name: string, value: string): void;
export function appendChild(parent: Node, child: Node): void;
export function createElement(tag: string): HTMLElement;
export function main(): void;
export function createTextNode(text: string): Node;
export function createSignal(initial_value: any): any[];
export class Signal {
  free(): void;
  [Symbol.dispose](): void;
  get(): any;
  constructor(initial_value: any);
  set(value: any): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_signal_free: (a: number, b: number) => void;
  readonly addClass: (a: any, b: number, c: number) => [number, number];
  readonly appendChild: (a: any, b: any) => [number, number];
  readonly createEffect: (a: any) => void;
  readonly createElement: (a: number, b: number) => [number, number, number];
  readonly createSignal: (a: any) => [number, number];
  readonly createTextNode: (a: number, b: number) => [number, number, number];
  readonly greet: (a: number, b: number) => void;
  readonly removeClass: (a: any, b: number, c: number) => [number, number];
  readonly setAttribute: (a: any, b: number, c: number, d: number, e: number) => [number, number];
  readonly setText: (a: any, b: number, c: number) => void;
  readonly signal_get: (a: number) => any;
  readonly signal_new: (a: any) => number;
  readonly signal_set: (a: number, b: any) => void;
  readonly main: () => void;
  readonly wasm_bindgen__convert__closures_____invoke__hb8c0e498495b0aad: (a: number, b: number) => any;
  readonly wasm_bindgen__closure__destroy__h0281d76ef968f9d1: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h14f275fd79d55e8b: (a: number, b: number, c: any) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
