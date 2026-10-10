#!/usr/bin/env bun
import { configuredExactCargoLawPolicyV1 } from "../../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { runExactCargoLaws } from "../../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { runOwnedCommand } from "../../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../🏃️process/⏱️budget/🟦️.ts";
/** ⚙️ Runs the `semio-framework-ui-runtime` test suite and the guest-target compile gates.
 *
 * The wasm gates are the point of this crate: the contract is what `wasm32-wasip2` plugin components
 * and `wasm32-unknown-unknown` browser renderers both speak, so a dependency that fails either target
 * is a design error, not a build error. Native `cargo check` cannot see that — it never compiles
 * `#[cfg(target_arch = "wasm32")]` code — which is why these run on every acceptance. */
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { testRuntimeTreeRetirement } from "../../♻️retirement/🌲️tree/🧪️tests/🔬️runtime-tree-retirement/🟦️.ts";
import { surfaceOwnershipSelfTests } from "../../🧪️tests/🔬️surface-ownership/🟦️.ts";

const packageRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));


//#region 🔖️test
class TreeRetirementScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    testRuntimeTreeRetirement();
    if (segments.length === 1 && segments[0] === "--oracle-only") return;
    const receipts = await runExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 3_600_000 }, manifestPaths: { "semio-framework-ui-runtime": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory, cwd: this.repoRoot, cargoArgs: segments, groups: [{ package: "semio-framework-ui-runtime", target: { kind: "lib" }, laws: [
        "runtime_tree_retirement_preserves_occupied_sources_and_closes_exact_payloads",
        "runtime_tree_retirement_handback_preserves_partial_owner_until_full_readmission",
        "runtime_tree_retirement_rejected_close_preserves_source_until_handback_admission",
      ] }] });
    console.log(`runtime-tree exact native laws:${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed`);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    console.log(`surface-ownership-oracle checks=${surfaceOwnershipSelfTests()}`);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: [], cwd: this.root, extraArgs: ["--all-features", ...rest] }, readCargoTestPolicyV1(process.env));
  }
}
//#endregion 🔖️test

//#region 🔖️check-wasm
/** 🌐️ Both guest flavours: wasip2 (plugin components) and unknown-unknown (browser renderers). */
class CheckWasmScript extends BundleScript {
  async run(): Promise<void> {
    const check = (args: string[]) => runOwnedCommand("cargo", ["check", "--manifest-path",resolve(this.root,"Cargo.toml"), "-p", "semio-framework-ui-runtime", ...args], packageRoot, "tool:owner", buildBudgetMs(), {env: process.env});
    await check(["--target", "wasm32-wasip2"]);
    await check(["--target", "wasm32-unknown-unknown"]);
  }
}
//#endregion 🔖️check-wasm

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check-wasm", CheckWasmScript).register("tree-retirement-check", TreeRetirementScript);
  await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
}
