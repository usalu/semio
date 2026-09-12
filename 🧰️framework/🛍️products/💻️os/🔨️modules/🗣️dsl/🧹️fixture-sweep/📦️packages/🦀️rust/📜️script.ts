#!/usr/bin/env bun
/** 🧭️ Fleet conformance remains an explicit test leaf, separate from kernel unit tests. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runExactCargoLaws, resolveTestLevel, buildBudgetMs } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { assertFixtureSweepLawCoverage, fixtureSweepLawGroup, testFixtureSweepExtraction } from "../../../🧪️tests/🧹️fixture-sweep/🟦️.ts";

class SourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("source-check accepts no arguments");
    await testFixtureSweepExtraction();
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await testFixtureSweepExtraction();
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot, cargoArgs: rest, buildBudgetMs: buildBudgetMs(), lawBudgetMs: 600_000,
      env: { ...process.env, RUST_TEST_NOCAPTURE: "1" },
      groups: [fixtureSweepLawGroup()],
    });
    assertFixtureSweepLawCoverage(receipts);
  }
}

const router = new ScriptRouter(import.meta.dir).register("source-check", SourceScript).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "source-check" });
