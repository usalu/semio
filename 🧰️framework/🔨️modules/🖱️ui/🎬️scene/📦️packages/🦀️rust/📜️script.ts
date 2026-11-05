#!/usr/bin/env repo
import{scriptCompilerAdmissionControlV1}from"../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { runAdmittedCargoCommandV1, runAdmittedCargoPackageTestsV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { resolve } from "node:path";
import { resolveTestLevel } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** ⚙️ Runs the test suites of `semio-ui-scene` and the producer crates carved from it, and the guest-target compile gates.
 *
 * This crate carries the pack-encoded `SurfaceDoc` payload every wasm32-wasip2 plugin component and
 * wasm32-unknown-unknown browser renderer moves across the `Component::Surface` boundary, so a
 * dependency that fails either guest target is a design error — see `ui_runtime`'s own `script.ts`
 * header for why native `cargo check` cannot see that on its own. */
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { isMainModuleV1 } from "../../../../⚙️runtime/🏁️main/🟦️.ts";

const packageRoot = import.meta.dirname ?? dirname(fileURLToPath(import.meta.url));

/** 🎬️ The scene crates, bottom up: the math producers, the pack codec, the doc contract, the product scenes, the surface bridge, then the facade. */
const SCENE_PACKAGES = ["semio-ui-scene-algebra", "semio-ui-scene-camera", "semio-ui-scene-mesh", "semio-ui-scene-pass", "semio-ui-scene-picking", "semio-ui-scene-pack", "semio-ui-scene-doc", "semio-ui-scene-modelling", "semio-ui-scene-world3d", "semio-ui-scene-node-graph", "semio-ui-scene-plane", "semio-ui-scene-views", "semio-ui-scene-surface", "semio-ui-scene"];

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
    const check = (args: string[]) => runAdmittedCargoCommandV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"cargo:caller",0),this.repoRoot,["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", "semio-ui-scene", ...args],packageRoot,process.env,buildBudgetMs());
    await check(["--target", "wasm32-wasip2"]);
    await check(["--target", "wasm32-unknown-unknown"]);
  }
}
//#endregion 🔖️check-wasm

if (isMainModuleV1(import.meta.url)) {
  const router = new ScriptRouter(import.meta.dirname).register("test", TestScript).register("check-wasm", CheckWasmScript);
  await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
}
