#!/usr/bin/env bun
/** @emoji ⚙️ Builds/tests the `repo_cli` crate and execs the `semio` binary (nx bridge for `repo/cli/rs`). */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runCmd, runCmdStatus, resolveTestLevel } from "../../../../../../../🧰framework/🛍️product/🦑repo/🔨module/📚lib/⚡️implementation/🟦typescript/📦index.ts";

class BuildScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["build", "-p", "repo_cli", "--release"], { cwd: this.repoRoot });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["repo_cli"], this.repoRoot, rest);
  }
}

/**
 * ▶️ Builds `semio` (always — cargo's own incremental cache makes a no-operation rebuild fast, and skipping
 * the build whenever the binary happened to already exist silently ran a stale binary after any
 * source edit) then execs it with forwarded argv and inherited stdio.
 */
class RunScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("cargo", ["build", "-p", "repo_cli", "--release"], { cwd: this.repoRoot });
    const binName = process.platform === "win32" ? "semio.exe" : "semio";
    const bin = join(this.repoRoot, "target", "release", binName);
    const status = runCmdStatus(bin, segments, { cwd: this.repoRoot });
    process.exit(status);
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("run", RunScript);
  await runBundleScriptMain(router, import.meta.url);
}
