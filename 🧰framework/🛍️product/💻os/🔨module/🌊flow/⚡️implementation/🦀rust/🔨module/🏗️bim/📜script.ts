#!/usr/bin/env bun
/** 🦀 `@semio-tech/flow-module-bim` router: `bun ./📜script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../../../../../../🧰framework/🛍️product/🦑repo/🔨module/📚lib/⚡️implementation/🟦typescript/📦index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
      skipEnvVar: "FLOW_MODULE_BIM_SKIP_WASM_BUILD",
      logPrefix: "flow/module/bim",
      wasmBaseName: "flow_module_bim",
      pkg: {
        name: "@semio-tech/flow-module-bim",
        files: ["flow_module_bim_bg.wasm", "flow_module_bim.js", "flow_module_bim.d.ts", "flow_module_bim_bg.wasm.d.ts"],
        main: "flow_module_bim.js",
        module: "flow_module_bim.js",
        types: "flow_module_bim.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
