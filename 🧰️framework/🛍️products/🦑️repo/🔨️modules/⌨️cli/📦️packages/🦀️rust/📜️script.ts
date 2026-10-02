#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../../../📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { cargoTargetDirectory } from "../../../📚️library/⚡️caching/🦀️cargo/🟦️.ts";
/** ⚙️ Builds/tests the `repo_cli` crate and execs the `semio` binary (nx bridge for `repo/cli/rs`). */
import { join } from "node:path";
import { devToolingEnv, runRepositoryCargoTests, runCmd, runCmdStatus } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    await buildRepositoryCargoArtifacts(join(this.root, "Cargo.toml"), ["--release", "--bin", "semio"], this.repoRoot);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-framework-repo-cli"], this.repoRoot, rest);
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
  const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("run", RunScript).register("daemon", DaemonScript).register("workflow", WorkflowScript).register("mcp", McpScript).register("repo", RepoScript);
  await runScriptMain(router);
}
