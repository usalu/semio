#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../../../📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
/** ⚙️ Builds/tests the `semio-framework-repo-cli` crate and execs its `semio-repo` binary (nx bridge for `repo/cli/rs`). */
import { join } from "node:path";
import { devToolingEnv, runRepositoryCargoTests, runCmdStatus } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

import { buildModeEnvironment, buildModePolicy, readBuildMode } from "../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🟦️.ts";

const crate = "semio-framework-repo-cli";

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    const mode = readBuildMode(process.env.SEMIO_BUILD_MODE);
    await buildRepositoryCargoArtifacts(join(this.root,"Cargo.toml"), ["-p", crate, "--profile", buildModePolicy(mode).cargoProfile], this.repoRoot, {environment:devToolingEnv(buildModeEnvironment(mode)),signal:this.invocation.control.signal});
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
  async run(segments: string[]): Promise<void> {
    await buildRepositoryCargoArtifacts(join(this.root,"Cargo.toml"), ["-p", crate, "--bin", "semio-repo", "--profile", "prototype"], this.repoRoot, {environment:devToolingEnv(buildModeEnvironment("prototype")),signal:this.invocation.control.signal});
    const binName = process.platform === "win32" ? "semio-repo.exe" : "semio-repo";
    process.exit(runCmdStatus(join(this.root, "dist/build/prototype", binName), segments, { cwd: this.repoRoot, env: devToolingEnv() }));
  }
}

/**
 * 🔌️ Forwards `semio-repo mcp …`: builds the repo command line, then serves the repo MCP server on
 * stdio with the profile taken from the first argument.
 */
class McpScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildRepositoryCargoArtifacts(join(this.root,"Cargo.toml"), ["-p", crate, "--bin", "semio-repo", "--profile", "prototype"], this.repoRoot, {environment:devToolingEnv(buildModeEnvironment("prototype")),signal:this.invocation.control.signal});
    const binName = process.platform === "win32" ? "semio-repo.exe" : "semio-repo";
    const bin = join(this.root, "dist/build/prototype", binName);
    const status = runCmdStatus(bin, ["mcp", ...segments], { cwd: this.repoRoot, env: devToolingEnv() });
    process.exit(status);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("mcp", McpScript).register("repo", RepoScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
