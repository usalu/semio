#!/usr/bin/env bun
/** 🦀️ `@semio-tech/flow-module-math` router: `bun ./📜️script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
      skipEnvVar: "FLOW_MODULE_MATH_SKIP_WASM_BUILD",
      logPrefix: "flow/module/math",
      wasmBaseName: "flow_extension_math",
      pkg: {
        name: "@semio-tech/flow-module-math",
        files: ["flow_extension_math_bg.wasm", "flow_extension_math.js", "flow_extension_math.d.ts", "flow_extension_math_bg.wasm.d.ts"],
        main: "flow_extension_math.js",
        module: "flow_extension_math.js",
        types: "flow_extension_math.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
