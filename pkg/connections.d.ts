/* tslint:disable */
/* eslint-disable */
export function greet(s: string): void;
export function remove_element_by_id(id: string): void;
export enum AnswerGroup {
  Yellow = 0,
  Green = 1,
  Blue = 2,
  Purple = 3,
}
export class AnswerGrid {
  private constructor();
  free(): void;
  static new(width: number, height: number): AnswerGrid;
  setup(): void;
  lives_string(): string;
  submit_selection(): boolean | undefined;
  toggle_square(x: number, y: number): void;
  width(): number;
  height(): number;
  get_idx(width: number, height: number): number;
}
export class Square {
  private constructor();
  free(): void;
  static new(x: number, y: number, answer_group: AnswerGroup, text: string): Square;
  id(): string;
  toggle(): void;
  solve(): void;
  update_color(): void;
  text(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly greet: (a: number, b: number) => void;
  readonly __wbg_answergrid_free: (a: number, b: number) => void;
  readonly remove_element_by_id: (a: number, b: number) => void;
  readonly answergrid_new: (a: number, b: number) => number;
  readonly answergrid_setup: (a: number) => [number, number];
  readonly answergrid_lives_string: (a: number) => [number, number];
  readonly answergrid_submit_selection: (a: number) => number;
  readonly answergrid_toggle_square: (a: number, b: number, c: number) => void;
  readonly answergrid_width: (a: number) => number;
  readonly answergrid_height: (a: number) => number;
  readonly answergrid_get_idx: (a: number, b: number, c: number) => number;
  readonly __wbg_square_free: (a: number, b: number) => void;
  readonly square_new: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly square_id: (a: number) => [number, number];
  readonly square_toggle: (a: number) => void;
  readonly square_solve: (a: number) => void;
  readonly square_update_color: (a: number) => void;
  readonly square_text: (a: number) => [number, number];
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
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
