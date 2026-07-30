#!/usr/bin/env bun
/** 🧭 Repo CLI task router. */
import { existsSync } from "node:fs";
import { join } from "node:path";
import type { FileLinter } from "../../../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🟦typescript/🕸️graph/🗣️dsl/🫀core/🟦typescript/📦index.ts";
import { BundleScript, ScriptRouter, buildBudgetMs, defineLint, resolveCliBin, runBundleScriptMain, runCmd, runTestBudgeted } from "../../../../../../../../🧰framework/🔨module/🧮math/⚡️implementation/🟦typescript/🕸️graph/🗣️dsl/🫀core/🟦typescript/📦index.ts";

export const policyFile = "main.go";

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

class DevScript extends BundleScript {
  run(segments: string[]): void {
    const bin = resolveCliBin(this.repoRoot);
    if (!existsSync(bin)) {
      runCmd("go", ["build", "-o", bin, "./repo/client/mcp/go"], {
        cwd: this.repoRoot,
        env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") },
        budgetMs: buildBudgetMs(),
      });
    }
    runCmd(bin, [...segments], {
      cwd: this.repoRoot,
      env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") },
    });
  }
}

class BuildScript extends BundleScript {
  run(): void {
    runCmd("go", ["build", "-o", join(this.repoRoot, "repo", "client", process.platform === "win32" ? "client.exe" : "client"), "./repo/client/mcp/go"], {
      cwd: this.repoRoot,
      env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") },
      budgetMs: buildBudgetMs(),
    });
  }
}

/** ⏱️Default `test` MUST stay ≤30s — `-short` skips the `testing.Short()`-gated real-monorepo-scan tests in `main_test.go`; run `bun ./script.ts test -- -run TestX` or drop `-short` for the full suite. */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    runTestBudgeted("go", ["test", "./repo/client/cli/go", "-short", ...segments], {
      cwd: this.repoRoot,
      env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
