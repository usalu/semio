#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-surface-paint-rs` router: `bun ./📜️script.ts <wasm|test>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FRAMEWORK_SURFACE_PAINT_RS_SKIP_WASM_BUILD",
      logPrefix: "framework/surface/paint/rs",
      wasmBaseName: "framework_surface_paint",
      profile: "wasm-release",
      pkg: {
        name: "@semio-tech/framework-surface-paint-rs",
        files: ["framework_surface_paint_bg.wasm", "framework_surface_paint.js", "framework_surface_paint.d.ts", "framework_surface_paint_bg.wasm.d.ts"],
        main: "framework_surface_paint.js",
        module: "framework_surface_paint.js",
        types: "framework_surface_paint.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["framework_surface_paint"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
