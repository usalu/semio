#!/usr/bin/env bun
import { runExactCargoLaws } from "../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🦀️ `@semio-tech/framework-job` task router: `bun ./📜️script.ts <test>`. */

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-job"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

class CanonicalArchitectureScript extends BundleScript {
  async run(): Promise<void> {
    const { proveJobReconcileContract } = await import("../../🔎️reconcile/🧪️tests/🔬️contract/🟦️.ts");
    proveJobReconcileContract();
    await runExactCargoLaws({ manifestPaths: { "semio-framework-job": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory, cwd: this.repoRoot,
      groups: [{ package: "semio-framework-job", target: { kind: "lib" }, laws: ["neutral_reconcile_envelopes_match_language_agnostic_vectors"] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs: 3_600_000, listBudgetMs: 60_000, lawBudgetMs: 120_000 });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("canonical-architecture", CanonicalArchitectureScript);

await runScriptMain(router, { defaultCommand: "test" });
