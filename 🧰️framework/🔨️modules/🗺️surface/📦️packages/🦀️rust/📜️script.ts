#!/usr/bin/env bun
import { buildWasmWebV1, readWasmBuildPolicyV1 } from "../../../🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts";
import { BROWSER_CANVAS_HOT_CRATES } from "../../../🖱️ui/🖌️render/🏗️build/🕸️browser/🟦️.ts";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { cmdBudgetMs } from "../../../🏃️process/⏱️budget/🟦️.ts";
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
/** 🦀️ `@semio-tech/framework-surface-rs` router: `bun ./📜️script.ts <wasm|test>` — one wasm-bindgen crate for the paint/terrain/node-graph/tiled-map surface family (puzzle's `board-2d` surface now lives in the puzzle plugin crate itself). */
import { join } from "node:path";

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class WasmScript extends BundleScript {
  async run(): Promise<void> {
    await buildWasmWebV1({
      rsDir: this.root,
      logPrefix: "framework/surface/rs",
      wasmBaseName: "framework_surface",
      outputDirectory: "🕸️bindings",
      shipProfile: "wasm-release",
      devOptimizedCrates: BROWSER_CANVAS_HOT_CRATES,
      noDefaultFeatures: true,
      cargoFeatures: ["session-bindgen"],
      pkg: {
        name: "@semio-tech/framework-surface-rs",
        files: ["framework_surface_bg.wasm", "framework_surface.js", "framework_surface.d.ts", "framework_surface_bg.wasm.d.ts"],
        main: "framework_surface.js",
        module: "framework_surface.js",
        types: "framework_surface.d.ts",
      },
    }, readWasmBuildPolicyV1(process.env,this.root));
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedCommand("bun", ["test", join(this.root, "../../🧪️tests/🧩️suite/🟦️.ts")], this.repoRoot, "tool:owner", cmdBudgetMs(), {env: process.env});
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-surface"], cwd: this.root, extraArgs: segments }, readCargoTestPolicyV1(process.env));
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runScriptMain(router, { defaultCommand: "wasm" });
