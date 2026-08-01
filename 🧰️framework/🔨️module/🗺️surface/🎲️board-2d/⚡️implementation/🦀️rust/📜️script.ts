#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-surface-board-2d-rs` router: `bun ./📜️script.ts wasm|test`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FRAMEWORK_SURFACE_BOARD_2D_RS_SKIP_WASM_BUILD",
      logPrefix: "framework/surface/board-2d/rs",
      wasmBaseName: "framework_surface_board_2d",
      pkg: {
        name: "@semio-tech/framework-surface-board-2d-rs",
        files: ["framework_surface_board_2d_bg.wasm", "framework_surface_board_2d.js", "framework_surface_board_2d.d.ts", "framework_surface_board_2d_bg.wasm.d.ts"],
        main: "framework_surface_board_2d.js",
        module: "framework_surface_board_2d.js",
        types: "framework_surface_board_2d.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-framework-os-kernel-surface-board-2d"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
