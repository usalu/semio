#!/usr/bin/env bun
/** 🗺️ `@semio-tech/reasoning-mindmap-rs` router: `bun ./script.ts <wasm|test>`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../🧰framework/🛍️product/🦑repo/🔨module/📚lib/⚡️implementation/🟦typescript/📦index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
      skipEnvVar: "REASONING_MINDMAP_RS_SKIP_WASM_BUILD",
      logPrefix: "reasoning/mindmap",
      wasmBaseName: "reasoning_mindmap",
      pkg: {
        name: "@semio-tech/reasoning-mindmap-rs",
        files: ["reasoning_mindmap_bg.wasm", "reasoning_mindmap.js", "reasoning_mindmap.d.ts", "reasoning_mindmap_bg.wasm.d.ts"],
        main: "reasoning_mindmap.js",
        module: "reasoning_mindmap.js",
        types: "reasoning_mindmap.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["reasoning_mindmap"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
