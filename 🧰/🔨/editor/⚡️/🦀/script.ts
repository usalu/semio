#!/usr/bin/env bun
/** 🦀 `@semio-tech/framework-editor-rs` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../🛍️/🦑/🔨/lib/⚡️/🟦/📦.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FRAMEWORK_EDITOR_RS_SKIP_WASM_BUILD",
      logPrefix: "framework/editor/rs",
      wasmBaseName: "framework_editor",
      pkg: {
        name: "@semio-tech/framework-editor-rs",
        files: ["framework_editor_bg.wasm", "framework_editor.js", "framework_editor.d.ts", "framework_editor_bg.wasm.d.ts"],
        main: "framework_editor.js",
        module: "framework_editor.js",
        types: "framework_editor.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["framework_editor"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
