#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../../../📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
/** ⚙️ Builds, installs and tests the `semio-framework-repo-dashboard` crate and execs its `semio` binary (nx bridge for `repo/dashboard/rs`). */
import { join } from "node:path";
import { mkdir } from "node:fs/promises";
import { dashboardInstalled, installedDashboard, installDashboard } from "../../📦️installation/🟦️.ts";
import { devToolingEnv, runRepositoryCargoTests, runCmd, runCmdStatus } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

const crate = "semio-framework-repo-dashboard";

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    await buildRepositoryCargoArtifacts(join(this.root, "Cargo.toml"), ["--release", "--bin", "semio"], this.repoRoot);
    await installDashboard(this.root, this.repoRoot);
  }
}

class InstallScript extends BundleScript {
  async run(): Promise<void> { console.log(`[dashboard] Installed ${await installDashboard(this.root, this.repoRoot)}`); }
}

class PreferencesScript extends BundleScript {
  async run(segments: string[]): Promise<void> { process.exit(runCmdStatus(await dashboardExecutable(this.root, this.repoRoot), ["preferences", ...segments], { cwd: this.repoRoot, env: devToolingEnv() })); }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "execution") {
      const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR ?? join(this.repoRoot, ".🧬semio/🦑️repo/⚡️cache/tests/dashboard-execution");
      await mkdir(artifacts, { recursive: true });
      runCmd("bun", ["test", join(this.root, "../../🧪️tests/🧊️execution/🟦️.ts")], { cwd: this.repoRoot, env: devToolingEnv({ SEMIO_TEST_ARTIFACT_DIR: artifacts }) }); return;
    }
    const { rest } = resolveTestLevel(segments);
    process.env.SEMIO_TEST_PATH ??= process.env.PATH;
    runCmd("bun", ["test", join(this.root, "../../🧪️tests/🌀️control-plane/🟦️.ts")], { cwd: this.repoRoot, env: devToolingEnv() });
    await runRepositoryCargoTests([crate], this.repoRoot, rest);
  }
}

/** ▶️ Runs the installed native dashboard without a build prerequisite. */
class RunScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const bin = await dashboardExecutable(this.root, this.repoRoot);
    const status = runCmdStatus(bin, segments, { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}

/**
 * 🌀 Forwards `semio daemon …` using the installed native executable.
 */
class DaemonScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const bin = await dashboardExecutable(this.root, this.repoRoot);
    const status = runCmdStatus(bin, ["daemon", ...segments], { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}

/**
 * 🌊️ Forwards `semio workflow …` using the installed native executable.
 */
class WorkflowScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const bin = await dashboardExecutable(this.root, this.repoRoot);
    const status = runCmdStatus(bin, ["workflow", ...segments], { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}

/** 🚀️ Resolves the installed immutable executable, building and installing it once when none is recorded. */
export async function dashboardExecutable(packageRoot: string, workspace: string): Promise<string> {
  if (dashboardInstalled(workspace)) return installedDashboard(workspace);
  console.log("[dashboard] Not installed yet; building and installing once");
  await buildRepositoryCargoArtifacts(join(packageRoot, "Cargo.toml"), ["--release", "--bin", "semio"], workspace);
  return await installDashboard(packageRoot, workspace);
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("install", InstallScript).register("preferences", PreferencesScript).register("test", TestScript).register("run", RunScript).register("daemon", DaemonScript).register("workflow", WorkflowScript);
  await runScriptMain(router);
}
