#!/usr/bin/env bun
/** @emoji ⚙️ Builds/tests the `repo_cli` crate and execs the `semio` binary (nx bridge for `repo/cli/rs`). */
import { join } from "node:path";
import { BundleScript, ScriptRouter, devToolingEnv, runBundleScriptMain, runCargoTestBudgeted, runCmd, runCmdStatus, resolveTestLevel } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class BuildScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["build", "-p", "semio-framework-repo-cli", "--release"], { cwd: this.repoRoot, env: devToolingEnv() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-repo-cli"], this.repoRoot, rest);
  }
}

/**
 * ▶️ Builds `semio` (always — cargo's own incremental cache makes a no-operation rebuild fast, and skipping
 * the build whenever the binary happened to already exist silently ran a stale binary after any
 * source edit) then execs it with forwarded argv and inherited stdio.
 */
class RunScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("cargo", ["build", "-p", "semio-framework-repo-cli"], { cwd: this.repoRoot, env: devToolingEnv() });
    const binName = process.platform === "win32" ? "semio.exe" : "semio";
    const bin = join(this.repoRoot, "target", "debug", binName);
    const status = runCmdStatus(bin, segments, { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}


/**
 * 🌀 Forwards `semio daemon …` after ensuring the binary is built.
 */
class DaemonScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("cargo", ["build", "-p", "semio-framework-repo-cli"], { cwd: this.repoRoot, env: devToolingEnv() });
    const binName = process.platform === "win32" ? "semio.exe" : "semio";
    const bin = join(this.repoRoot, "target", "debug", binName);
    const status = runCmdStatus(bin, ["daemon", ...segments], { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}

/**
 * 🌊️ Forwards `semio workflow …` after ensuring the binary is built.
 */
class WorkflowScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("cargo", ["build", "-p", "semio-framework-repo-cli"], { cwd: this.repoRoot, env: devToolingEnv() });
    const binName = process.platform === "win32" ? "semio.exe" : "semio";
    const bin = join(this.repoRoot, "target", "debug", binName);
    const status = runCmdStatus(bin, ["workflow", ...segments], { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("run", RunScript).register("daemon", DaemonScript).register("workflow", WorkflowScript);
  await runBundleScriptMain(router, import.meta.url);
}
