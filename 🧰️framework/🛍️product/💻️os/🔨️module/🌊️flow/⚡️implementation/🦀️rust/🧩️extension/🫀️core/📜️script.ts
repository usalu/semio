#!/usr/bin/env bun
/** 🦀️ `@semio-tech/flow-module-core` router: `bun ./📜️script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FLOW_MODULE_CORE_SKIP_WASM_BUILD",
      logPrefix: "flow/module/core",
      wasmBaseName: "flow_extension_core",
      pkg: {
        name: "@semio-tech/flow-module-core",
        files: ["flow_extension_core_bg.wasm", "flow_extension_core.js", "flow_extension_core.d.ts", "flow_extension_core_bg.wasm.d.ts"],
        main: "flow_extension_core.js",
        module: "flow_extension_core.js",
        types: "flow_extension_core.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
