#!/usr/bin/env bun
/** 🧮️ `@semio-tech/math-algebra` — dense/sparse linear algebra: vectors, matrices, eigensolvers. */
import { BundleScript, ScriptRouter, resolveTestLevel, runCargoTestBudgeted, runCargoLint, runBundleScriptMain } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/⚡️implementations/🟦️typescript/📦️index.ts";
import { join } from "node:path";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["mathematical_algebra"], join(this.root, "rs"), rest);
  }
}

class LintScript extends BundleScript {
  run(): void {
    runCargoLint(["mathematical_algebra"], join(this.root, "rs"));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("lint", LintScript);
await runBundleScriptMain(router, import.meta.url);
