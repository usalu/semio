#!/usr/bin/env bun
/** @emoji ⚙️ Runs the `semio-framework-ui-scene` test suite and the guest-target compile gates.
 *
 * This crate carries the pack-encoded `SurfaceDoc` payload every wasm32-wasip2 plugin component and
 * wasm32-unknown-unknown browser renderer moves across the `Component::Surface` boundary, so a
 * dependency that fails either guest target is a design error — see `ui_runtime`'s own `script.ts`
 * header for why native `cargo check` cannot see that on its own. */
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const packageRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));

//#region 🔖️test
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted([], packageRoot, ["--all-features", ...rest]);
  }
}
//#endregion 🔖️test

//#region 🔖️check-wasm
/** @emoji 🌐️ Both guest flavours: wasip2 (plugin components) and unknown-unknown (browser renderers). */
class CheckWasmScript extends BundleScript {
  run(): void {
    const check = (args: string[]) => runCmd("cargo", ["check", "-p", "semio-framework-ui-scene", ...args], { cwd: packageRoot, budgetMs: buildBudgetMs() });
    check(["--target", "wasm32-wasip2"]);
    check(["--target", "wasm32-unknown-unknown"]);
  }
}
//#endregion 🔖️check-wasm

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check-wasm", CheckWasmScript);
  await runBundleScriptMain(router, import.meta.url);
}
