/* tslint:disable */
/* eslint-disable */
/**
* @returns {any}
*/
export function wasm_memory(): any;
/**
*/
export class Petri {
  free(): void;
/**
* @returns {Petri}
*/
  static new(): Petri;
/**
* @param {number} w
* @param {number} h
* @param {number} number
* @returns {Petri}
*/
  static new_with_params(w: number, h: number, number: number): Petri;
/**
* @param {number} steps
*/
  tick(steps: number): void;
/**
* @returns {string}
*/
  get_stats_string(): string;
/**
* @returns {number}
*/
  get_iteration(): number;
/**
* @returns {number}
*/
  get_bacters_number(): number;
/**
* @returns {number}
*/
  get_algae_number(): number;
/**
* @returns {number}
*/
  get_all_bacters_aggros(): number;
/**
* @returns {number}
*/
  get_all_bacters_sizes(): number;
/**
* @returns {number}
*/
  get_all_bacters_position_interlaced(): number;
/**
* @returns {number}
*/
  get_all_algae_position_interlaced(): number;
}
/**
*/
export class Statistics {
  free(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_statistics_free: (a: number) => void;
  readonly __wbg_petri_free: (a: number) => void;
  readonly petri_new: () => number;
  readonly petri_new_with_params: (a: number, b: number, c: number) => number;
  readonly petri_tick: (a: number, b: number) => void;
  readonly petri_get_stats_string: (a: number, b: number) => void;
  readonly petri_get_iteration: (a: number) => number;
  readonly petri_get_bacters_number: (a: number) => number;
  readonly petri_get_algae_number: (a: number) => number;
  readonly petri_get_all_bacters_aggros: (a: number) => number;
  readonly petri_get_all_bacters_sizes: (a: number) => number;
  readonly petri_get_all_bacters_position_interlaced: (a: number) => number;
  readonly petri_get_all_algae_position_interlaced: (a: number) => number;
  readonly wasm_memory: () => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
