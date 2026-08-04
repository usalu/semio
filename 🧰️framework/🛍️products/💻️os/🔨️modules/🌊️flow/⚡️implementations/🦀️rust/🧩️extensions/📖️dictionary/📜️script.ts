#!/usr/bin/env bun
/** 🦀️ `@semio-tech/flow-module-dictionary` router: `bun ./📜️script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FLOW_MODULE_DICTIONARY_SKIP_WASM_BUILD",
      logPrefix: "flow/module/dictionary",
      wasmBaseName: "flow_extension_dictionary",
      pkg: {
        name: "@semio-tech/flow-module-dictionary",
        files: ["flow_extension_dictionary_bg.wasm", "flow_extension_dictionary.js", "flow_extension_dictionary.d.ts", "flow_extension_dictionary_bg.wasm.d.ts"],
        main: "flow_extension_dictionary.js",
        module: "flow_extension_dictionary.js",
        types: "flow_extension_dictionary.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
