#!/usr/bin/env bun
import { buildWasmWebV1, readWasmBuildPolicyV1 } from "../../../🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts";
import { BROWSER_CANVAS_HOT_CRATES } from "../../../🖱️ui/🖌️render/🏗️build/🕸️browser/🟦️.ts";
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🦀️ `@semio-tech/framework-editor-rs` router: `bun ./📜️script.ts wasm`. */

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class WasmScript extends BundleScript {
  async run(): Promise<void> {
    await buildWasmWebV1({
      rsDir: this.root,
      logPrefix: "framework/editor/rs",
      wasmBaseName: "framework_editor",
      shipProfile: "wasm-release",
      devOptimizedCrates: BROWSER_CANVAS_HOT_CRATES,
      pkg: {
        name: "@semio-tech/framework-editor-rs",
        files: ["framework_editor_bg.wasm", "framework_editor.js", "framework_editor.d.ts", "framework_editor_bg.wasm.d.ts"],
        main: "framework_editor.js",
        module: "framework_editor.js",
        types: "framework_editor.d.ts",
      },
    }, readWasmBuildPolicyV1(process.env,this.root));
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-editor"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runScriptMain(router, { defaultCommand: "wasm" });
