#!/usr/bin/env bun
/** @emoji ⚙️ Runs the `semio-framework-ui-runtime` test suite and the guest-target compile gates.
 *
 * The wasm gates are the point of this crate: the contract is what `wasm32-wasip2` plugin components
 * and `wasm32-unknown-unknown` browser renderers both speak, so a dependency that fails either target
 * is a design error, not a build error. Native `cargo check` cannot see that — it never compiles
 * `#[cfg(target_arch = "wasm32")]` code — which is why these run on every acceptance. */
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

const packageRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));

//#region 🔖️test
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted([], packageRoot, ["...rest]);
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
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check-wasm", CheckWasmScript);
  await runBundleScriptMain(router, import.meta.url);
}
