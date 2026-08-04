#!/usr/bin/env bun
/** 🦀️ `@semio-tech/flow-module-logic` router: `bun ./📜️script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FLOW_MODULE_LOGIC_SKIP_WASM_BUILD",
      logPrefix: "flow/module/logic",
      wasmBaseName: "flow_extension_logic",
      pkg: {
        name: "@semio-tech/flow-module-logic",
        files: ["flow_extension_logic_bg.wasm", "flow_extension_logic.js", "flow_extension_logic.d.ts", "flow_extension_logic_bg.wasm.d.ts"],
        main: "flow_extension_logic.js",
        module: "flow_extension_logic.js",
        types: "flow_extension_logic.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
