#!/usr/bin/env bun
/** 🦀️ `@semio-tech/flow-module-brep` router: `bun ./📜️script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FLOW_MODULE_BREP_SKIP_WASM_BUILD",
      logPrefix: "flow/module/brep",
      wasmBaseName: "flow_extension_brep",
      threads: false,
      pkg: {
        name: "@semio-tech/flow-module-brep",
        files: ["flow_extension_brep_bg.wasm", "flow_extension_brep.js", "flow_extension_brep.d.ts", "flow_extension_brep_bg.wasm.d.ts"],
        main: "flow_extension_brep.js",
        module: "flow_extension_brep.js",
        types: "flow_extension_brep.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
