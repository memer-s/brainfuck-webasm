/* tslint:disable */
/* eslint-disable */
/**
* This function runs after initialization.
*/
export function run(): void;
/**
*/
export class BF {
  free(): void;
/**
* @returns {BF}
*/
  static new(): BF;
/**
* This function is called with the code passed as a string,
* @param {string} c
*/
  set_code(c: string): void;
/**
* This function prints the internal state
* of the interpreter to the js console
*/
  print(): void;
/**
* This function gets the internal state of the interpreter
* as a stringified JSON object.
* @returns {string}
*/
  get_state(): string;
/**
* @returns {boolean}
*/
  step(): boolean;
/**
* @param {number} steps
* @returns {boolean}
*/
  steps(steps: number): boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly run: () => void;
  readonly __wbg_bf_free: (a: number) => void;
  readonly bf_new: () => number;
  readonly bf_set_code: (a: number, b: number, c: number) => void;
  readonly bf_print: (a: number) => void;
  readonly bf_get_state: (a: number, b: number) => void;
  readonly bf_step: (a: number) => number;
  readonly bf_steps: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
