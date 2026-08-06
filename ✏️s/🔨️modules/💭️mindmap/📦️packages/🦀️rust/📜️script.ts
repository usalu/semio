#!/usr/bin/env bun
/** 🗺️ `@semio-tech/mindmap-rs` router: `bun ./📜️script.ts <wasm|test>`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
      skipEnvVar: "MINDMAP_RS_SKIP_WASM_BUILD",
      logPrefix: "s/mindmap",
      wasmBaseName: "mindmap",
      pkg: {
        name: "@semio-tech/mindmap-rs",
        files: ["mindmap_bg.wasm", "mindmap.js", "mindmap.d.ts", "mindmap_bg.wasm.d.ts"],
        main: "mindmap.js",
        module: "mindmap.js",
        types: "mindmap.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-s-mindmap"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
