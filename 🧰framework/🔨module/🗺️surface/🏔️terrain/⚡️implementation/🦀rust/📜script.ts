#!/usr/bin/env bun
/** 🦀 `@semio-tech/framework-surface-terrain-rs` router: `bun ./📜script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../../🧰framework/🛍️product/🦑repo/🔨module/📚lib/⚡️implementation/🟦typescript/📦index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FRAMEWORK_SURFACE_TERRAIN_RS_SKIP_WASM_BUILD",
      logPrefix: "framework/surface/terrain/rs",
      wasmBaseName: "framework_surface_terrain",
      pkg: {
        name: "@semio-tech/framework-surface-terrain-rs",
        files: ["framework_surface_terrain_bg.wasm", "framework_surface_terrain.js", "framework_surface_terrain.d.ts", "framework_surface_terrain_bg.wasm.d.ts"],
        main: "framework_surface_terrain.js",
        module: "framework_surface_terrain.js",
        types: "framework_surface_terrain.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["framework_surface_terrain"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
