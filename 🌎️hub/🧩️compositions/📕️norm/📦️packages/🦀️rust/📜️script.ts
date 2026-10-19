#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runCargo } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { runRepositoryCargoTests } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runResultsWindowConfigSourceScript, runSurfaceRenderSourceScript, runComponentBudgetScript, runMutationLeafTaxonomyGenerateScript, runMutationLeafTaxonomyCheckScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-hub-norm"], this.repoRoot, this.invocation, rest);
  }
}

class ResultsWindowConfigSourceScript extends BundleScript {
  async run(): Promise<void> {
    await runResultsWindowConfigSourceScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

class ResultsWindowConfigTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-s-artifact-norm-contract"], this.repoRoot, this.invocation, ["--test", "config_mutation", ...rest]);
  }
}

class SurfaceRenderSourceScript extends BundleScript {
  async run(): Promise<void> {
    await runSurfaceRenderSourceScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

class SurfaceRenderTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-hub-norm"], this.repoRoot, this.invocation, ["--test", "surface_render", ...rest], { ...process.env, RUST_MIN_STACK: "67108864" });
  }
}

/** ⚖️ Law: the shipped `wasm-release` component this plugin's `describe` verifies fits the admission bound with
 * norm's own headroom, and carries no `name` custom section — 55.9 % of the refused 2026-09-22
 * component was mangled symbol names, so their absence is what keeps the bound met. */
class ComponentBudgetScript extends BundleScript {
  async run(): Promise<void> {
    await runComponentBudgetScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

class CheckScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["check", "--manifest-path", "Cargo.toml", "--lib", ...segments], this.root);
  }
}

class MutationLeafTaxonomyGenerateScript extends BundleScript {
  async run(): Promise<void> {
    await runMutationLeafTaxonomyGenerateScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

class MutationLeafTaxonomyCheckScript extends BundleScript {
  async run(): Promise<void> {
    await runMutationLeafTaxonomyCheckScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("surface-render-source", SurfaceRenderSourceScript)
  .register("surface-render-test", SurfaceRenderTestScript)
  .register("test", TestScript)
  .register("check", CheckScript)
  .register("results-window-config-source", ResultsWindowConfigSourceScript)
  .register("results-window-config-test", ResultsWindowConfigTestScript)
  .register("component-budget-check", ComponentBudgetScript)
  .register("mutation-leaf-taxonomy-generate", MutationLeafTaxonomyGenerateScript)
  .register("mutation-leaf-taxonomy-check", MutationLeafTaxonomyCheckScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
