#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "@semio-tech/process/compiler-capacity";
import { buildBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🧭️ Builds and tests the Go repository implementation used by dashboard tasks. */
import { mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import type { FileLinter } from "../../../📚️library/📦️packages/🟦️typescript/🚨️policy/🟦️.ts";
import { defineLint } from "../../../📚️library/📦️packages/🟦️typescript/🚨️policy/🟦️.ts";
import { goCoverageArgs, goLevelTestArgs, runCanonicalGoTests, runCanonicalGoBuild } from "../../../📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepoScriptMain } from "../../../📚️library/📦️packages/🟦️typescript/🏃️process/🟦️.ts";

export const policyFile = "🐹️.go";

/** 📏️Keeps the moved repo CLI godfile under the reviewable line budget. */
export const policy = defineLint("repo-cli-go-godfile", (l: FileLinter) => {
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

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    const binary = join(this.repoRoot, ".🧬semio/🦑️repo/⚡️cache/🗃️bin", process.platform === "win32" ? "semio-repo.exe" : "semio-repo");
    mkdirSync(dirname(binary), { recursive: true });
    await runCanonicalGoBuild(import.meta.dir, ["-trimpath", "-o", binary, "./🚀️bin"], { control: scriptCompilerAdmissionControlV1(this.invocation, "repo-cli:build", buildBudgetMs()), env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") }, budgetMs: buildBudgetMs() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    const tags = level === "exhaustive" ? ["-tags", "exhaustive"] : [];
    await runCanonicalGoTests(import.meta.dir, [...tags, ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, import.meta.dir), ...rest]);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await runRepoScriptMain(router, import.meta.url);
