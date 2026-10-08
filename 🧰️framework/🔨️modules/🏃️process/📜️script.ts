#!/usr/bin/env bun
/** 🏃️ Runs neutral process ownership and execution contracts. */
import { resolve } from "node:path";
import { mkdirSync } from "node:fs";
import { runOwnedCommand } from "./🎛️owned-execution/🟦️.ts";
import { TEST_LEVEL_BUDGET_MS } from "./🧪️testing/🎚️budget/🟦️.ts";
import { BundleScript, ScriptRouter } from "./🧭️routing/🟦️.ts";
import { runScriptMain } from "./🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 1) throw Error("Expected one process contract suite");
    if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("Process contract tests require explicit SEMIO_TEST_ARTIFACT_DIR");
    const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR);
    mkdirSync(output, { recursive: true });
    if(segments[0]==="resource-leases"){
      await runOwnedCommand(process.execPath,["-e","const {testResourceLeases}=await import(process.argv[1]);await testResourceLeases(process.argv[2]);",resolve(this.root,"🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts"),output],this.repoRoot,"process:resource-leases",TEST_LEVEL_BUDGET_MS.fundamental);return;
    }
    if(segments[0]==="artifact-publication"){await (await import("./📦️artifacts/📤️publication/🧪️tests/🟦️.ts")).testArtifactPublication(output);return;}
    if (segments[0] === "artifact-files") {
      const { testArtifactFiles } = await import("./📦️artifacts/🗂️files/🧪️tests/🟦️.ts");
      await testArtifactFiles(output);
      return;
    }
    const suites: Readonly<Record<string, { source: string; budgetMs: number }>> = {
      capture: { source: "📥️capture/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      "wasm-build": { source: "📦️artifacts/🕸️wasm-build/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      "native-artifacts": { source: "📦️artifacts/🏗️native-build/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      "exact-cargo-laws": { source: "🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      "owner-context": { source: "📋️context/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      "vitest-driver": { source: "🧪️testing/🧪️vitest/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      "cargo-driver": { source: "🧪️testing/🦀️cargo/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.quick },
      "test-command": { source: "🧪️testing/🎛️execution/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      routing: { source: "🧭️routing/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      "execution-budget": { source: "⏱️budget/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      budget: { source: "🧪️testing/🎚️budget/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.fundamental },
      "owned-execution": { source: "🎛️owned-execution/🧪️tests/🟦️.ts", budgetMs: 120_000 },
      "process-tree-termination": { source: "🪓️termination/🧪️tests/🟦️.ts", budgetMs: 120_000 },
    };
    const suite = suites[segments[0]!];
    if (!suite) throw Error(`Unknown process contract suite: ${segments[0]}`);
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, suite.source)], this.repoRoot, `process:${segments[0]}`, suite.budgetMs, { env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: output } });
  }
}

if (import.meta.main) await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript));
