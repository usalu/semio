#!/usr/bin/env bun
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🧭️ `@semio-tech/repo-coordinator-go` router: `bun ./📜️script.ts build|test|dev`. */
import { mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { goCoverageArgs, goLevelTestArgs, runCanonicalGoBuild, runCanonicalGoTests } from "../../../../📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { daemonBudgetMs } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const PACKAGE_DIR = import.meta.dir;
const BINARY_DIR = join(PACKAGE_DIR, "📦️main");
const BINARY = process.platform === "win32" ? "semio-repo-coordinator.exe" : "semio-repo-coordinator";

/** 🗃️ The coordinator binary is build output, so it lives in the marked repository cache and never in the tree. */
export function coordinatorBinaryPath(repoRoot: string): string {
  const path = join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", "🗃️bin", readBuildMode(process.env.SEMIO_BUILD_MODE ?? "prototype"), "coordinator", BINARY);
  mkdirSync(dirname(path), { recursive: true });
  return path;
}

/** 🐹️ Every Go invocation of this package shares the repository workspace file. */
export function coordinatorGoEnvironment(repoRoot: string): NodeJS.ProcessEnv {
  return { ...process.env, GOWORK: join(repoRoot, "go.work") };
}

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    await runCanonicalGoBuild(BINARY_DIR, ["-o", coordinatorBinaryPath(this.repoRoot), "."], { control: scriptCompilerAdmissionControlV1(this.invocation, "go:build", 0),  env: coordinatorGoEnvironment(this.repoRoot), budgetMs: buildBudgetMs(),  });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runCanonicalGoTests(PACKAGE_DIR, [...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, PACKAGE_DIR), ...rest], { env: coordinatorGoEnvironment(this.repoRoot) });
  }
}

/** ▶️ Serves the coordinator artifact completed by the owning build dependency. */
class DevScript extends BundleScript {
  async run(): Promise<void> {
    const binary = coordinatorBinaryPath(this.repoRoot);
    await runOwnedCommand(binary, [], this.repoRoot, "repo-coordinator:serve", daemonBudgetMs(), { env: coordinatorGoEnvironment(this.repoRoot), signal: this.invocation.control.signal });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("dev", DevScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
