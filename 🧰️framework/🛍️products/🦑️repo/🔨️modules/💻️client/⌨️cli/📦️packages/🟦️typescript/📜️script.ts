#!/usr/bin/env bun
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🧭️ Repo CLI task router. */
import { existsSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import type { FileLinter } from "../../../../📚️library/📦️packages/🟦️typescript/🚨️policy/🟦️.ts";
import { defineLint, resolveCliBin } from "../../../../📚️library/📦️packages/🟦️typescript/🚨️policy/🟦️.ts";
import { goLevelTestArgs, runCanonicalGoBuild, runCanonicalGoTests } from "../../../../📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts";
import { runCmd, runRepoScriptMain } from "../../../../📚️library/📦️packages/🟦️typescript/🏃️process/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

import { readEntityCatalog } from "../../../../../../../🔨️modules/🧬️schema/📦️packages/🟦️typescript/🏷️entity-kinds/📥️source/🟦️.ts";
import { writeGeneratedFileIfChanged } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";
import { GENERATOR_ID, REFRESH_COMMAND, REPO_CLI_GO, repoEntityKindTarget } from "../../🏷️entity-kinds/📽️projection/🟦️.ts";

export const policyFile = "🐹️.go";

const REPO_CLI_MAIN = join(REPO_CLI_GO,"📦️packages","🐹️go","📦️main");

export const policy = defineLint("repo-client-cli-main-go", (l: FileLinter) => {
  const n = l.lines().length;
  if (n > 10000) {
    return [
      l.breach({
        id: "line-budget",
        summary: `File has ${n} lines (> 10000)`,
        kind: "lint/file/line-budget",
        priority: "medium",
        reason: "Large files are harder to review",
        solution: "Split into smaller modules",
      }),
    ];
  }
  return [];
});

/** ▶️ Runs the executable restored by the `build` prerequisite instead of rebuilding inline. */
class DevScript extends BundleScript {
  run(segments: string[]): void {
    const bin = resolveCliBin(this.repoRoot);
    if (!existsSync(bin)) throw new Error(`repo client binary is missing at ${bin}; run: bun repo run @semio-tech/repo-client:build`);
    runCmd(bin, [...segments], {
      cwd: this.repoRoot,
      env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") },
    });
  }
}

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    await runCanonicalGoBuild(join(this.repoRoot, REPO_CLI_MAIN), ["-o", resolveCliBin(this.repoRoot), "."], { control: scriptCompilerAdmissionControlV1(this.invocation, "go:build", 0), 
      env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") },
      budgetMs: buildBudgetMs(),
      
    });
  }
}

/** ⏱️Default `test` MUST stay ≤30s — `-short` skips the `testing.Short()`-gated real-monorepo-scan tests in `main_test.go`; run `bun ./📜️script.ts test -- -run TestX` or drop `-short` for the full suite. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    const oracle = await import("../../🧪️tests/🃏️glob/🟦️.ts");
    oracle.verifyFixtureGlobOracle();
    oracle.verifyTicketRetentionOracle();
    await runCanonicalGoTests(join(this.repoRoot, REPO_CLI_GO), [...goLevelTestArgs(level), ...rest], {
      env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") },
    });
  }
}

class GenerateEntityKindsScript extends BundleScript {
  run(): void {
    const source = readEntityCatalog(this.repoRoot);
    const target = repoEntityKindTarget(this.repoRoot, source);
    writeGeneratedFileIfChanged(target.path, target.content);
    console.log(`Repo CLI entity kinds refreshed (${source.kinds.length} entries, sha256 ${source.sha256}).`);
  }
}

class PreviewEntityKindsScript extends BundleScript {
  run(): void {
    const target = repoEntityKindTarget(this.repoRoot, readEntityCatalog(this.repoRoot));
    const nodes = [{ bytesBase64: Buffer.from(target.content).toString("base64"), mode: 0o644, nodeKind: "file", path: relative(this.repoRoot, target.path).replaceAll("\\", "/").normalize("NFC") }];
    process.stdout.write(`${JSON.stringify({ contractId: GENERATOR_ID, nodes, schemaVersion: 1, staleRemovals: [] })}\n`);
  }
}

class CheckEntityKindsScript extends BundleScript {
  run(): void {
    const target = repoEntityKindTarget(this.repoRoot, readEntityCatalog(this.repoRoot));
    if (!existsSync(target.path) || readFileSync(target.path, "utf8") !== target.content) throw Error(`Repo CLI entity kinds are stale; run ${REFRESH_COMMAND}`);
    console.log("Repo CLI entity kinds are fresh.");
  }
}

class TestEntityKindsScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("test-entity-kinds accepts no arguments");
    const { proveRepoEntityKindsOwnership } = await import("../../🧪️tests/🏷️entity-kinds/🟦️.ts");
    proveRepoEntityKindsOwnership(this.repoRoot);
    await runCanonicalGoTests(join(this.repoRoot, REPO_CLI_GO), ["-short", "-count=1", "-v", "-run", "^TestAllEntityEmojisProjectsTheFrameworkCatalog$"], { env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") }, packages: ["."] });
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript)
  .register("generate-entity-kinds", GenerateEntityKindsScript)
  .register("preview-generated", PreviewEntityKindsScript)
  .register("check-entity-kinds", CheckEntityKindsScript)
  .register("test-entity-kinds", TestEntityKindsScript);

if (import.meta.main) await runRepoScriptMain(router, import.meta.url);
