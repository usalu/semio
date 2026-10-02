#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🧭️ `repo-cli-go` router: `bun ./📜️script.ts test`. */
import type { FileLinter } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { defineLint, goCoverageArgs, goLevelTestArgs, runRepositoryTestCommand } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runRepoScriptMain } from "../../../📚️library/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

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

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    const tags = level === "exhaustive" ? ["-tags", "exhaustive"] : [];
    await runRepositoryTestCommand("go", ["test", "...", ...tags, ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, import.meta.dir), ...rest], { cwd: import.meta.dir });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runRepoScriptMain(router, import.meta.url);
