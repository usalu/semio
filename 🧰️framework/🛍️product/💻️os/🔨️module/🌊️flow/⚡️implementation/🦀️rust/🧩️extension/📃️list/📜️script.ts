#!/usr/bin/env bun
/** 🦀️ `@semio-tech/flow-module-list` router: `bun ./📜️script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
      skipEnvVar: "FLOW_MODULE_LIST_SKIP_WASM_BUILD",
      logPrefix: "flow/module/list",
      wasmBaseName: "flow_extension_list",
      pkg: {
        name: "@semio-tech/flow-module-list",
        files: ["flow_extension_list_bg.wasm", "flow_extension_list.js", "flow_extension_list.d.ts", "flow_extension_list_bg.wasm.d.ts"],
        main: "flow_extension_list.js",
        module: "flow_extension_list.js",
        types: "flow_extension_list.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
