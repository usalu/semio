#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../../../📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { cargoTargetDirectory } from "../../../📚️library/⚡️caching/🦀️cargo/🟦️.ts";
/** ⚙️ Builds/tests the `repo_cli` crate and execs the `semio` binary (nx bridge for `repo/cli/rs`). */
import { join } from "node:path";
import { mkdir } from "node:fs/promises";
import { installedDashboard, installDashboard } from "../../📦️installation/🟦️.ts";
import { devToolingEnv, runRepositoryCargoTests, runCmd, runCmdStatus } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

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
  async run(segments: string[]): Promise<void> { process.exit(runCmdStatus(installedDashboard(this.repoRoot), ["preferences", ...segments], { cwd: this.repoRoot, env: devToolingEnv() })); }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "execution") {
      const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR ?? join(this.repoRoot, ".🧬semio/🦑️repo/⚡️cache/tests/dashboard-execution");
      await mkdir(artifacts, { recursive: true });
      runCmd("bun", ["test", join(this.root, "../../🧪️tests/🧊️execution/🟦️.ts")], { cwd: this.repoRoot, env: devToolingEnv({ SEMIO_TEST_ARTIFACT_DIR: artifacts }) }); return;
    }
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-framework-repo-cli"], this.repoRoot, rest);
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

/** 🚀️ Resolves the installed immutable executable without reading its contents. */
export async function dashboardExecutable(_packageRoot: string, workspace: string): Promise<string> {
  return installedDashboard(workspace);
}


/**
 * 🧭️ Runs the repo command line: builds `semio-repo`, then execs it with the forwarded verb.
 */
class RepoScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("cargo", ["build", "-p", "semio-framework-repo-cli", "--bin", "semio-repo"], { cwd: this.repoRoot, env: devToolingEnv() });
    const binName = process.platform === "win32" ? "semio-repo.exe" : "semio-repo";
    process.exit(runCmdStatus(join(cargoTargetDirectory(this.repoRoot), "debug", binName), segments, { cwd: this.repoRoot, env: devToolingEnv() }));
  }
}

/**
 * 🔌️ Forwards `semio-repo mcp …`: builds the repo command line, then serves the repo MCP server on
 * stdio with the profile taken from the first argument.
 */
class McpScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("cargo", ["build", "-p", "semio-framework-repo-cli", "--bin", "semio-repo"], { cwd: this.repoRoot, env: devToolingEnv() });
    const binName = process.platform === "win32" ? "semio-repo.exe" : "semio-repo";
    const bin = join(cargoTargetDirectory(this.repoRoot), "debug", binName);
    const status = runCmdStatus(bin, ["mcp", ...segments], { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("install", InstallScript).register("preferences", PreferencesScript).register("test", TestScript).register("run", RunScript).register("daemon", DaemonScript).register("workflow", WorkflowScript).register("mcp", McpScript).register("repo", RepoScript);
  await runScriptMain(router);
}
