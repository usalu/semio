#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-surface-rs` router: `bun ./📜️script.ts <wasm|test>` — one wasm-bindgen crate for the paint/board-2d/terrain/node-graph/tiled-map surface family. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FRAMEWORK_SURFACE_RS_SKIP_WASM_BUILD",
      logPrefix: "framework/surface/rs",
      wasmBaseName: "framework_surface",
      shipProfile: "wasm-release",
      pkg: {
        name: "@semio-tech/framework-surface-rs",
        files: ["framework_surface_bg.wasm", "framework_surface.js", "framework_surface.d.ts", "framework_surface_bg.wasm.d.ts"],
        main: "framework_surface.js",
        module: "framework_surface.js",
        types: "framework_surface.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["semio-framework-surface"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
