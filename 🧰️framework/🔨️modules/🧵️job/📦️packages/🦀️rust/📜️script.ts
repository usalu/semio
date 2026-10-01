#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-job` task router: `bun ./📜️script.ts <test>`. */
import { runCargoTestBudgeted, resolveTestLevel, runExactCargoLaws } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-job"], this.repoRoot, rest);
  }
}

class CanonicalArchitectureScript extends BundleScript {
  async run(): Promise<void> {
    const { proveJobReconcileContract } = await import("../../🔎️reconcile/🧪️tests/🔬️contract/🟦️.ts");
    proveJobReconcileContract();
    await runExactCargoLaws({ cwd: this.repoRoot,
      groups: [{ package: "semio-framework-job", target: { kind: "lib" }, laws: ["neutral_reconcile_envelopes_match_language_agnostic_vectors"] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs: 3_600_000, listBudgetMs: 60_000, lawBudgetMs: 120_000 });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("canonical-architecture", CanonicalArchitectureScript);

await runScriptMain(router, { defaultCommand: "test" });
