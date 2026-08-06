#!/usr/bin/env bun
/** 🦀️ `@semio-tech/flow-core` router: `bun ./📜️script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FLOW_CORE_SKIP_WASM_BUILD",
      logPrefix: "flow/core",
      wasmBaseName: "flow_core",
      threads: false,
      shipProfile: "wasm-release",
      pkg: {
        name: "@semio-tech/flow-core",
        files: ["flow_core_bg.wasm", "flow_core.js", "flow_core.d.ts", "flow_core_bg.wasm.d.ts"],
        main: "flow_core.js",
        module: "flow_core.js",
        types: "flow_core.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
