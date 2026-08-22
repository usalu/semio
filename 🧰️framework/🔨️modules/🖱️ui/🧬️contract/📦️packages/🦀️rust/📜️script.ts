#!/usr/bin/env bun
/** @emoji ⚙️ Runs the `semio-framework-ui-contract` test suite and the guest-target compile gates.
 *
 * The wasm gates are the point of this crate: the contract is what `wasm32-wasip2` plugin components
 * and `wasm32-unknown-unknown` browser renderers both speak, so a dependency that fails either target
 * is a design error, not a build error. Native `cargo check` cannot see that — it never compiles
 * `#[cfg(target_arch = "wasm32")]` code — which is why these run on every acceptance. */
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { mkdirSync } from "node:fs";
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd, runCmdStatus } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

const packageRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));

//#region 🔖️test
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted([], packageRoot, ["--all-features", ...rest]);
  }
}
//#endregion 🔖️test

//#region 🔖️conformance
/** @emoji 🧪️ Runs only `🦀️conformance.rs`'s corpus harness — every fixture under
 * `📚️examples/🧪️conformance/` deserializes, validates/patches through this crate's own
 * `validate_snapshot`/`apply_patch`, and matches its declarative expectation. Same test binary as
 * `test`, filtered to the `conformance::` module path so iterating on the corpus does not pay for the
 * whole crate's suite. */
class ConformanceScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted([], packageRoot, ["--all-features", ...rest, "--", "conformance::"]);
  }
}
//#endregion 🔖️conformance

//#region 🔖️check-wasm
/** @emoji 🌐️ Both guest flavours: wasip2 (plugin components) and unknown-unknown (browser renderers). */
class CheckWasmScript extends BundleScript {
  run(): void {
    const check = (args: string[]) => runCmd("cargo", ["check", "-p", "semio-framework-ui-contract", ...args], { cwd: packageRoot, budgetMs: buildBudgetMs() });
    check(["--target", "wasm32-wasip2"]);
    check(["--target", "wasm32-unknown-unknown"]);
    check(["--target", "wasm32-wasip2", "--features", "typegen"]);
  }
}
//#endregion 🔖️check-wasm

//#region 🔖️typegen
const TYPEGEN_TEST_NAME = "typegen_export";

function generatedUiContractPath(root: string): string {
  return join(root, "..", "..", "..", "..", "🛂️manifest", "🤖️generated", "🟦️ui-contract.ts");
}

/** 🧬️ Runs the owned schema export test, optionally writing its deterministic projection. */
function runTypegenExportTest(root: string, outPath?: string): void {
  const env = outPath === undefined ? process.env : { ...process.env, SEMIO_TYPEGEN_OUT: outPath };
  const status = runCmdStatus("cargo", ["test", "-p", "semio-framework-ui-contract", "--features", "typegen", "--test", TYPEGEN_TEST_NAME], {
    cwd: root,
    env,
    budgetMs: buildBudgetMs(),
  });
  if (status !== 0) {
    console.error("ui-contract typegen: owned schema export failed — see output above.");
    process.exit(status);
  }
}

class GenerateScript extends BundleScript {
  run(_segments: string[]): void {
    const outPath = generatedUiContractPath(this.root);
    mkdirSync(join(this.root, "..", "..", "..", "..", "🛂️manifest", "🤖️generated"), { recursive: true });
    runTypegenExportTest(this.root, outPath);
    console.log(`ui-contract typescript mirror refreshed -> ${outPath}`);
  }
}

/** 🔎️ Validates metadata and byte-compares the owned projection with the committed mirror. */
class CheckScript extends BundleScript {
  run(_segments: string[]): void {
    runTypegenExportTest(this.root);
    console.log("ui-contract typescript mirror is fresh.");
  }
}
//#endregion 🔖️typegen

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir)
    .register("test", TestScript)
    .register("conformance", ConformanceScript)
    .register("check-wasm", CheckWasmScript)
    .register("generate", GenerateScript)
    .register("check", CheckScript);
  await runBundleScriptMain(router, import.meta.url);
}
