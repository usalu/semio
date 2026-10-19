#!/usr/bin/env bun
import {scriptCompilerAdmissionControlV1} from "../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { readCargoTestPolicyV1, runAdmittedCargoCommandV1, runAdmittedCargoTestsV1 as runCargoTestsV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** ⚙️ Runs the `semio-framework-ui-runtime` test suite and the guest-target compile gates.
 *
 * The wasm gates are the point of this crate: the contract is what `wasm32-wasip2` plugin components
 * and `wasm32-unknown-unknown` browser renderers both speak, so a dependency that fails either target
 * is a design error, not a build error. Native `cargo check` cannot see that — it never compiles
 * `#[cfg(target_arch = "wasm32")]` code — which is why these run on every acceptance. */
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { surfaceOwnershipSelfTests } from "../../🧪️tests/🔬️surface-ownership/🟦️.ts";
import { runTreeRetirementScript, runNativeMeshReceivingSourceScript, runNativeResourceScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

const packageRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));
/** 🧠️ The headless runtime crates, bottom up: exchange and presentation, the reconcile primitives, the cursor, the reconcile authority, then the facade. */
const UI_RUNTIME_PACKAGES = ["semio-framework-ui-runtime-exchange", "semio-framework-ui-runtime-presentation", "semio-framework-ui-runtime-reconcile-primitives", "semio-framework-ui-runtime-reconcile-cursor", "semio-framework-ui-runtime-reconcile", "semio-framework-ui-runtime"];


//#region 🔖️test
class TreeRetirementScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runTreeRetirementScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** 📥️ Validates the native receiver corpus against Three, SQLite and the closed schema. */
class NativeMeshReceivingSourceScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runNativeMeshReceivingSourceScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
class NativeResourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runNativeResourceScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
    const { rest } = resolveTestLevel(segments);
    console.log(`surface-ownership-oracle checks=${surfaceOwnershipSelfTests()}`);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: UI_RUNTIME_PACKAGES, cwd: this.root, extraArgs: ["--all-features", ...rest] }, readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
  }
}
//#endregion 🔖️test

//#region 🔖️check-wasm
/** 🌐️ Both guest flavours: wasip2 (plugin components) and unknown-unknown (browser renderers). */
class CheckWasmScript extends BundleScript {
  async run(): Promise<void> {
    const check = (args: string[]) => runAdmittedCargoCommandV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"cargo:caller",0),this.repoRoot,["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", "semio-framework-ui-runtime", ...args],packageRoot,process.env,buildBudgetMs());
    await check(["--target", "wasm32-wasip2"]);
    await check(["--target", "wasm32-unknown-unknown"]);
  }
}
//#endregion 🔖️check-wasm

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check-wasm", CheckWasmScript).register("tree-retirement-check", TreeRetirementScript).register("test-native-resource-custody", NativeResourceScript).register("test-native-mesh-receiving-source",NativeMeshReceivingSourceScript);
  await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
}
