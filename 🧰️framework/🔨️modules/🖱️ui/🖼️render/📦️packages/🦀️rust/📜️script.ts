#!/usr/bin/env bun
/** @emoji ⚙️ Runs the `semio-framework-ui-render` suite, the browser-target gate, and the dependency
 * boundary assertion that keeps this crate backend-neutral.
 *
 * `boundaries` is the load-bearing one: the whole point of this crate is that it describes frames
 * without owning a device, so `wgpu`, `winit` and every graphics binding must be absent from its
 * dependency tree. That is a property no type signature can express, so it is asserted here. */
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd, runProbe } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

const packageRoot = import.meta.dir ?? dirname(fileURLToPath(import.meta.url));

/** @emoji 🚫️ Crates that must never appear in this crate's dependency tree, and why. */
const FORBIDDEN_DEPENDENCIES: ReadonlyArray<readonly [string, string]> = [
  ["wgpu", "belongs to the browser backend target only — natively we hand-write D3D12/Metal/Vulkan"],
  ["winit", "windowing belongs to semio-framework-ui-host"],
  ["semio-framework-actor", "the renderer must not own or link the actor kernel"],
];

//#region 🔖️test
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted([], packageRoot, ["--all-features", ...rest]);
  }
}
//#endregion 🔖️test

//#region 🔖️check-wasm
class CheckWasmScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["check", "-p", "semio-framework-ui-render", "--target", "wasm32-unknown-unknown"], { cwd: packageRoot, budgetMs: buildBudgetMs() });
  }
}
//#endregion 🔖️check-wasm

//#region 🔖️boundaries
/** @emoji 🧭️ Fails when a forbidden crate reaches this crate's normal (non-dev, non-build) dep tree. */
class BoundariesScript extends BundleScript {
  run(): void {
    const violations: string[] = [];
    for (const [crate, reason] of FORBIDDEN_DEPENDENCIES) {
      // 🧭️ `cargo tree --invert <crate>` fails with "did not match any packages" when the crate is
      // absent from the graph entirely — that failure IS the passing state. A zero exit means cargo
      // resolved it and printed the dependents, i.e. the crate reaches us.
      const result = runProbe("cargo", ["tree", "-p", "semio-framework-ui-render", "--edges", "normal", "--prefix", "none", "--invert", crate], { cwd: packageRoot, budgetMs: buildBudgetMs() });
      if (result.status === 0 && result.stdout.trim().length > 0) violations.push(`${crate}: ${reason}`);
    }
    if (violations.length > 0) {
      console.error("semio-framework-ui-render must stay backend-neutral, but its dependency tree contains:");
      for (const violation of violations) console.error(`  - ${violation}`);
      process.exit(1);
    }
    console.log(`ui-render dependency boundaries hold (${FORBIDDEN_DEPENDENCIES.length} forbidden crates absent).`);
  }
}
//#endregion 🔖️boundaries

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check-wasm", CheckWasmScript).register("boundaries", BoundariesScript);
  await runBundleScriptMain(router, import.meta.url);
}
