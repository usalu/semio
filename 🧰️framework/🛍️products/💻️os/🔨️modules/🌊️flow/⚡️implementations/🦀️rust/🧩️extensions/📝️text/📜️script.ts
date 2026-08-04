#!/usr/bin/env bun
/** 🦀️ `@semio-tech/flow-module-text` router: `bun ./📜️script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FLOW_MODULE_TEXT_SKIP_WASM_BUILD",
      logPrefix: "flow/module/text",
      wasmBaseName: "flow_extension_text",
      pkg: {
        name: "@semio-tech/flow-module-text",
        files: ["flow_extension_text_bg.wasm", "flow_extension_text.js", "flow_extension_text.d.ts", "flow_extension_text_bg.wasm.d.ts"],
        main: "flow_extension_text.js",
        module: "flow_extension_text.js",
        types: "flow_extension_text.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
