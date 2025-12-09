import * as wasm from "./connections_bg.wasm";
export * from "./connections_bg.js";
import { __wbg_set_wasm } from "./connections_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
