#!/usr/bin/env bun
/** 🌈️ `@semio-tech/math-graph-spectral` — Graph matrices and spectra: adjacency/Laplacian/modularity matrices, algebraic connectivity, spectral ordering. */
import { BundleScript, ScriptRouter, resolveTestLevel, runCargoTestBudgeted, runCargoLint, runBundleScriptMain } from "../../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";
import { join } from "node:path";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["mathematical_graph_spectral"], join(this.root, "rs"), rest);
  }
}

class LintScript extends BundleScript {
  run(): void {
    runCargoLint(["mathematical_graph_spectral"], join(this.root, "rs"));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("lint", LintScript);
await runBundleScriptMain(router, import.meta.url);
