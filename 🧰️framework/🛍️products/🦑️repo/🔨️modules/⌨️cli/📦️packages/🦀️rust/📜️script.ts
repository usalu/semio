#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { cargoTargetDirectory } from "../../../📚️library/⚡️caching/🦀️cargo/🟦️.ts";
/** ⚙️ Builds/tests the `semio-framework-repo-cli` crate and execs its `semio-repo` binary (nx bridge for `repo/cli/rs`). */
import { join } from "node:path";
import { devToolingEnv, runRepositoryCargoTests, runCmd, runCmdStatus } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

const crate = "semio-framework-repo-cli";

class BuildScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["build", "-p", crate], { cwd: this.repoRoot, env: devToolingEnv() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests([crate], this.repoRoot, this.invocation.control, rest);
  }
}

/**
 * 🧭️ Runs the repo command line: builds `semio-repo`, then execs it with the forwarded verb.
 */
class RepoScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("cargo", ["build", "-p", crate, "--bin", "semio-repo"], { cwd: this.repoRoot, env: devToolingEnv() });
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
    runCmd("cargo", ["build", "-p", crate, "--bin", "semio-repo"], { cwd: this.repoRoot, env: devToolingEnv() });
    const binName = process.platform === "win32" ? "semio-repo.exe" : "semio-repo";
    const bin = join(cargoTargetDirectory(this.repoRoot), "debug", binName);
    const status = runCmdStatus(bin, ["mcp", ...segments], { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("mcp", McpScript).register("repo", RepoScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
