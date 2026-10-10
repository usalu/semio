#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** ⚙️ Builds/tests the `semio-framework-repo-mcp` protocol crate; the `repo` stdio server binary lives in `⌨️cli` because it carries the production repository. */
import { join } from "node:path";
import { cargoTargetDirectory } from "../../../📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { devToolingEnv, runRepositoryCargoTests, runCmd, runCmdStatus } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

const CRATE = "semio-framework-repo-mcp";
const SERVER_CRATE = "semio-framework-repo-cli";
const BINARY = process.platform === "win32" ? "repo.exe" : "repo";

class BuildScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["build", "-p", CRATE, "--release"], { cwd: this.repoRoot, env: devToolingEnv() });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests([CRATE], this.repoRoot, this.invocation.control, rest);
  }
}

/** ▶️ Builds then execs the stdio server with the profile taken from the first argument. */
class RunScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("cargo", ["build", "-p", SERVER_CRATE, "--bin", "repo"], { cwd: this.repoRoot, env: devToolingEnv() });
    const environment = devToolingEnv();
    const profile = segments[0]?.trim();
    if (profile) environment.SEMIO_REPO_MCP_CLIENT = profile;
    process.exit(runCmdStatus(join(cargoTargetDirectory(this.repoRoot), "debug", BINARY), [], { cwd: this.repoRoot, env: environment }));
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("run", RunScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
