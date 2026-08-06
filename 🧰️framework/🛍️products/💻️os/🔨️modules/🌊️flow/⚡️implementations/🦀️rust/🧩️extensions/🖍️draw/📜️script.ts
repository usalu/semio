#!/usr/bin/env bun
/** 🦀️ `@semio-tech/flow-module-draw` router: `bun ./📜️script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild, runCargoTestBudgeted, resolveTestLevel } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FLOW_MODULE_DRAW_SKIP_WASM_BUILD",
      logPrefix: "flow/module/draw",
      wasmBaseName: "flow_extension_draw",
      threads: false,
      pkg: {
        name: "@semio-tech/flow-module-draw",
        files: ["flow_extension_draw_bg.wasm", "flow_extension_draw.js", "flow_extension_draw.d.ts", "flow_extension_draw_bg.wasm.d.ts"],
        main: "flow_extension_draw.js",
        module: "flow_extension_draw.js",
        types: "flow_extension_draw.d.ts",
      },
    });
  }
}

/** ⏱️Level-budgeted; unmarked `mod tests` cases are `fundamental`. */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["flow_extension_draw"], join(this.root, "rs"), rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
