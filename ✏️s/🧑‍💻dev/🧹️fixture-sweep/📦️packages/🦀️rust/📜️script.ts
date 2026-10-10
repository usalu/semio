#!/usr/bin/env bun
import { configuredExactCargoLawPolicyV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../🧰️framework/🔨️modules/🏃️process/⏱️budget/🟦️.ts";
/** 🧭️ Fleet conformance remains an explicit test leaf, separate from kernel unit tests. */
import { runRepositoryExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { assertFixtureSweepLawCoverage, fixtureSweepLawGroup, testFixtureSweepExtraction } from "../../🧪️tests/🔬️ownership/🟦️.ts";

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
    const receipts = await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), lawMilliseconds: 600_000 }, cwd: this.repoRoot, cargoArgs: rest, env: { ...process.env, RUST_TEST_NOCAPTURE: "1" }, groups: [fixtureSweepLawGroup()] });
    assertFixtureSweepLawCoverage(receipts);
  }
}

const router = new ScriptRouter(import.meta.dir).register("source-check", SourceScript).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "source-check" }) }));
