#!/usr/bin/env bun
/** 🧭️ Fleet conformance remains an explicit test leaf, separate from kernel unit tests. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runExactCargoLaws, resolveTestLevel, buildBudgetMs } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { testFixtureSweepExtraction } from "../../📜️script.ts";

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
    const fixture = JSON.parse(readFileSync(join(this.root, "../../🧫️fixture/🔣️.json"), "utf8"));
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot, cargoArgs: rest, buildBudgetMs: buildBudgetMs(), lawBudgetMs: 600_000,
      env: { ...process.env, RUST_TEST_NOCAPTURE: "1" },
      groups: [{ package: fixture.package, target: { kind: "test", name: fixture.target }, laws: fixture.laws }],
    });
    assert.equal(receipts.length, 1);
    assert.equal(receipts[0]!.assertions, 2);
    const output = [0, 1].map(index => [".stdout", ".stderr"].map(suffix => readFileSync(join(receipts[0]!.artifactDir, `law-${index}${suffix}`), "utf8")).join("\n")).join("\n");
    const sweep = [...output.matchAll(/\[dsl-fixture-sweep\] (\d+) example dir\(s\), (\d+) \.semio fixture file\(s\) found, (\d+) law-check\(s\) run across (\d+) registered app kind\(s\), (\d+) unmapped fixture\(s\)/g)];
    const coverage = [...output.matchAll(/example asset coverage: (\d+) slug\(s\) on new 🖼️assets layout, (\d+) soft-skipped mid-migration/g)];
    assert.equal(sweep.length, 1, "one actual fleet sweep summary is required");
    assert.equal(coverage.length, 1, "one actual asset coverage summary is required");
    assert(Number(sweep[0]![1]) > 0 && Number(sweep[0]![2]) > 0 && Number(sweep[0]![3]) > 0, "an empty or all-unmapped fleet cannot pass");
    assert.equal(Number(sweep[0]![4]), 54);
    assert(Number(coverage[0]![1]) > 0, "all-soft-skipped asset coverage cannot pass");
    console.log(`[DEBUG] ${sweep[0]![0]}; ${coverage[0]![0]}; exact assertions=2; executable=${receipts[0]!.sha256}; evidence=${receipts[0]!.artifactDir}`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("source-check", SourceScript).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "source-check" });
