/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_statistics_free(a: number): void;
export function __wbg_petri_free(a: number): void;
export function petri_new(): number;
export function petri_new_with_params(a: number, b: number, c: number): number;
export function petri_tick(a: number, b: number): void;
export function petri_get_stats_string(a: number, b: number): void;
export function petri_get_iteration(a: number): number;
export function petri_get_bacters_number(a: number): number;
export function petri_get_algae_number(a: number): number;
export function petri_get_all_bacters_aggros(a: number): number;
export function petri_get_all_bacters_sizes(a: number): number;
export function petri_get_all_bacters_position_interlaced(a: number): number;
export function petri_get_all_algae_position_interlaced(a: number): number;
export function wasm_memory(): number;
export function __wbindgen_exn_store(a: number): void;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_free(a: number, b: number): void;
