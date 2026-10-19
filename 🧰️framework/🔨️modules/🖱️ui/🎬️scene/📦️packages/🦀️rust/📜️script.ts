#!/usr/bin/env bun
import{scriptCompilerAdmissionControlV1}from"../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { runAdmittedCargoCommandV1, runAdmittedCargoPackageTestsV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { resolve } from "node:path";
import { resolveTestLevel } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** ⚙️ Runs the test suites of `semio-framework-ui-scene` and the producer crates carved from it, and the guest-target compile gates.
 *
 * This crate carries the pack-encoded `SurfaceDoc` payload every wasm32-wasip2 plugin component and
 * wasm32-unknown-unknown browser renderer moves across the `Component::Surface` boundary, so a
 * dependency that fails either guest target is a design error — see `ui_runtime`'s own `script.ts`
 * header for why native `cargo check` cannot see that on its own. */
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const packageRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));

/** 🎬️ The scene crates, bottom up: the math producers, the pack codec, the doc contract, the product scenes, the surface bridge, then the facade. */
const SCENE_PACKAGES = ["semio-framework-ui-scene-algebra", "semio-framework-ui-scene-camera", "semio-framework-ui-scene-mesh", "semio-framework-ui-scene-pass", "semio-framework-ui-scene-picking", "semio-framework-ui-scene-pack", "semio-framework-ui-scene-doc", "semio-framework-ui-scene-modelling", "semio-framework-ui-scene-world3d", "semio-framework-ui-scene-node-graph", "semio-framework-ui-scene-plane", "semio-framework-ui-scene-views", "semio-framework-ui-scene-surface", "semio-framework-ui-scene"];

//#region 🔖️test
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runAdmittedCargoPackageTestsV1(SCENE_PACKAGES, this.repoRoot, this.invocation, ["--all-features", ...rest]);
  }
}
//#endregion 🔖️test

//#region 🔖️check-wasm
/** 🌐️ Both guest flavours: wasip2 (plugin components) and unknown-unknown (browser renderers). */
class CheckWasmScript extends BundleScript {
  async run(): Promise<void> {
    const check = (args: string[]) => runAdmittedCargoCommandV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"cargo:caller",0),this.repoRoot,["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", "semio-framework-ui-scene", ...args],packageRoot,process.env,buildBudgetMs());
    await check(["--target", "wasm32-wasip2"]);
    await check(["--target", "wasm32-unknown-unknown"]);
  }
}
//#endregion 🔖️check-wasm

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check-wasm", CheckWasmScript);
  await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
}
