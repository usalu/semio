#!/usr/bin/env bun
import { buildCargoArtifacts } from "../../../📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
/** @emoji ⚙️ Builds/tests the `repo_cli` crate and execs the `semio` binary (nx bridge for `repo/cli/rs`). */
import { join } from "node:path";
import { BundleScript, ScriptRouter, devToolingEnv, runBundleScriptMain, runCargoTestBudgeted, runCmdStatus, resolveTestLevel } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    await buildCargoArtifacts(join(this.root, "Cargo.toml"), ["--release", "--bin", "semio"], this.repoRoot);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-repo-cli"], this.repoRoot, rest);
  }
}

/** ▶️ Runs the executable restored by the Nx build prerequisite. */
class RunScript extends BundleScript {
  run(segments: string[]): void {
    const binName = process.platform === "win32" ? "semio.exe" : "semio";
    const bin = join(this.root, "dist", "build", binName);
    const status = runCmdStatus(bin, segments, { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}


/**
 * 🌀 Forwards `semio daemon …` using the executable restored by Nx.
 */
class DaemonScript extends BundleScript {
  run(segments: string[]): void {
    const binName = process.platform === "win32" ? "semio.exe" : "semio";
    const bin = join(this.root, "dist", "build", binName);
    const status = runCmdStatus(bin, ["daemon", ...segments], { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}

/**
 * 🌊️ Forwards `semio workflow …` using the executable restored by Nx.
 */
class WorkflowScript extends BundleScript {
  run(segments: string[]): void {
    const binName = process.platform === "win32" ? "semio.exe" : "semio";
    const bin = join(this.root, "dist", "build", binName);
    const status = runCmdStatus(bin, ["workflow", ...segments], { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("run", RunScript).register("daemon", DaemonScript).register("workflow", WorkflowScript);
  await runBundleScriptMain(router, import.meta.url);
}
