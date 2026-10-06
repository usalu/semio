#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
/** 🧭️ Repo CLI task router. */
import { existsSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import type { FileLinter } from "../../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { defineLint, goLevelTestArgs, resolveCliBin, runCanonicalGoBuild, runCanonicalGoTests, runCmd } from "../../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runRepoScriptMain } from "../../../../📚️library/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

import { readEntityCatalog, type EntityCatalogSource } from "../../../../../../../🔨️modules/🧬️schema/🏷️entity-kinds/📥️source/🟦️.ts";
import { writeGeneratedFileIfChanged } from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🗂️files/🟦️.ts";
import { emitGo, GENERATOR_ID, REFRESH_COMMAND } from "../../🏷️entity-kinds/📽️projection/🟦️.ts";

export const policyFile = "🐹️.go";

const REPO_CLIENT_DIR = join("🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "💻️client");
const REPO_CLI_GO = join(REPO_CLIENT_DIR, "⌨️cli");

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

/** ▶️ Runs the executable restored by the Nx `build` prerequisite instead of rebuilding inline. */
class DevScript extends BundleScript {
  run(segments: string[]): void {
    const bin = resolveCliBin(this.repoRoot);
    if (!existsSync(bin)) throw new Error(`repo client binary is missing at ${bin}; run: bun nx run @semio-tech/repo-client:build`);
    runCmd(bin, [...segments], {
      cwd: this.repoRoot,
      env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") },
    });
  }
}

class BuildScript extends BundleScript {
  run(): void {
    runCanonicalGoBuild(join(this.repoRoot, REPO_CLI_GO), ["-trimpath", "-ldflags=-s -w", "-o", join(this.repoRoot, REPO_CLIENT_DIR, process.platform === "win32" ? "client.exe" : "client"), "./cmd/repo"], {
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

/** 📋️ Plans only the Go projection owned by this Repo CLI. */
export function repoEntityKindTarget(repoRoot: string, source: EntityCatalogSource): { readonly path: string; readonly content: string } {
  return { path: join(repoRoot, REPO_CLI_GO, "🏷️entity-kinds/🐹️.go"), content: emitGo(source) };
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
