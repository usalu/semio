#!/usr/bin/env bun
/** 🖥️ `@semio-tech/puzzle-3d-ui-rs` router: `bun ./📜️script.ts <wasm|test [fundamental|quick|long|exhaustive]>`.
 * Hosts the wasm-bindgen build this app's `⚙️engine` slot used to own — moved here
 * (`HEADLESS-ENGINE-LAW-AND-OFFENDER-FIXES`) since the engine (constitutional: engine) slot must not
 * depend on `wasm-bindgen`, and `🖱️ui` is where every wasm-bindgen-exported puzzle-3d surface
 * (`Puzzle3dDocumentVcs`, `puzzle3dParseDslJson`) now lives. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "PUZZLE_3D_UI_RS_SKIP_WASM_BUILD",
      logPrefix: "puzzle/3d/ui/rs",
      wasmBaseName: "puzzle_3d",
      pkg: {
        name: "@semio-tech/puzzle-3d-ui-rs",
        files: ["puzzle_3d_bg.wasm", "puzzle_3d.js", "puzzle_3d.d.ts", "puzzle_3d_bg.wasm.d.ts"],
        main: "puzzle_3d.js",
        module: "puzzle_3d.js",
        types: "puzzle_3d.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-s-app-puzzle-3d-ui"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
