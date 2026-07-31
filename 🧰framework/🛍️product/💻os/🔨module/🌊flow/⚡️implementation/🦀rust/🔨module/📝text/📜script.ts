#!/usr/bin/env bun
/** 🦀 `@semio-tech/flow-module-text` router: `bun ./📜script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../../../../../../🧰framework/🛍️product/🦑repo/🔨module/📚lib/⚡️implementation/🟦typescript/📦index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
      skipEnvVar: "FLOW_MODULE_TEXT_SKIP_WASM_BUILD",
      logPrefix: "flow/module/text",
      wasmBaseName: "flow_module_text",
      pkg: {
        name: "@semio-tech/flow-module-text",
        files: ["flow_module_text_bg.wasm", "flow_module_text.js", "flow_module_text.d.ts", "flow_module_text_bg.wasm.d.ts"],
        main: "flow_module_text.js",
        module: "flow_module_text.js",
        types: "flow_module_text.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
