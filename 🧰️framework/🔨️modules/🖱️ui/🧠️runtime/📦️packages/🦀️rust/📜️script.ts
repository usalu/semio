#!/usr/bin/env bun
/** @emoji ⚙️ Runs the `semio-framework-ui-runtime` test suite and the guest-target compile gates.
 *
 * The wasm gates are the point of this crate: the contract is what `wasm32-wasip2` plugin components
 * and `wasm32-unknown-unknown` browser renderers both speak, so a dependency that fails either target
 * is a design error, not a build error. Native `cargo check` cannot see that — it never compiles
 * `#[cfg(target_arch = "wasm32")]` code — which is why these run on every acceptance. */
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runExactCargoLaws, runCmd } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { testRuntimeTreeRetirement } from "../../♻️retirement/🌲️tree/🧪️tests/🔬️runtime-tree-retirement/🟦️.ts";
import { surfaceOwnershipSelfTests } from "../../🧪️tests/🔬️surface-ownership/🟦️.ts";

const packageRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));


//#region 🔖️test
class TreeRetirementScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    testRuntimeTreeRetirement();
    if (segments.length === 1 && segments[0] === "--oracle-only") return;
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot, cargoArgs: segments, buildBudgetMs: 3_600_000,
      groups: [{ package: "semio-framework-ui-runtime", target: { kind: "lib" }, laws: [
        "runtime_tree_retirement_preserves_occupied_sources_and_closes_exact_payloads",
        "runtime_tree_retirement_handback_preserves_partial_owner_until_full_readmission",
        "runtime_tree_retirement_rejected_close_preserves_source_until_handback_admission",
      ] }],
    });
    console.log(`[DEBUG] runtime-tree exact native laws:${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed`);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    console.log(`[DEBUG] surface-ownership-oracle checks=${surfaceOwnershipSelfTests()}`);
    await runCargoTestBudgeted([], packageRoot, ["--all-features", ...rest]);
  }
}
//#endregion 🔖️test

//#region 🔖️check-wasm
/** @emoji 🌐️ Both guest flavours: wasip2 (plugin components) and unknown-unknown (browser renderers). */
class CheckWasmScript extends BundleScript {
  run(): void {
    const check = (args: string[]) => runCmd("cargo", ["check", "-p", "semio-framework-ui-runtime", ...args], { cwd: packageRoot, budgetMs: buildBudgetMs() });
    check(["--target", "wasm32-wasip2"]);
    check(["--target", "wasm32-unknown-unknown"]);
  }
}
//#endregion 🔖️check-wasm

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check-wasm", CheckWasmScript).register("tree-retirement-check", TreeRetirementScript);
  await runBundleScriptMain(router, import.meta.url);
}
