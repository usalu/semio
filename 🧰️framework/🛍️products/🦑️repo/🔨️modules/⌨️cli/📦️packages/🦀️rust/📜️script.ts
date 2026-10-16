#!/usr/bin/env bun
import {runDeclaredRustBuildV1} from "../../../📚️library/🏗️build/🕸️graph/🦀️rust/📥️intent/🏃️routing/🟦️.ts";
import {runRepositoryPublishedCargoExecutableV1} from "../../../📚️library/⚡️caching/📦️artifacts/🏗️native-build/📥️published/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** ⚙️ Builds/tests the `semio-framework-repo-cli` crate and execs its `semio-repo` binary (owned build route for `repo/cli/rs`). */
import { join } from "node:path";
import { runRepositoryCargoTests } from "../../../📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

import { readBuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";

const crates = ["semio-framework-repo-cli-event-stream", "semio-framework-repo-cli-command-framework", "semio-framework-repo-cli-mcp-verb", "semio-framework-repo-cli-tree-source-and-search", "semio-framework-repo-cli-verbs", "semio-framework-repo-cli"];

class BuildScript extends BundleScript {
  async run(segments:string[]):Promise<void>{await runDeclaredRustBuildV1(this.repoRoot,"@semio-tech/repo-cli-rs",this.root,this.invocation,segments);}
}

class BuildCliScript extends BundleScript {
  async run(segments:string[]):Promise<void>{await runDeclaredRustBuildV1(this.repoRoot,"@semio-tech/repo-cli-rs",this.root,this.invocation,segments,"build-cli");}
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(crates, this.repoRoot, this.invocation, rest);
  }
}

/**
 * 🧭️ Runs the repo command line: builds `semio-repo`, then execs it with the forwarded verb.
 */
class RepoScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    await runDeclaredRustBuildV1(this.repoRoot,"@semio-tech/repo-cli-rs",this.root,this.invocation,[],"build-cli");
    process.exitCode=await runRepositoryPublishedCargoExecutableV1(join(this.root,"Cargo.toml"),"semio-repo"+(process.platform==="win32"?".exe":""),segments,this.repoRoot,readBuildMode(process.env.SEMIO_BUILD_MODE),this.invocation);
  }
}

/**
 * 🔌️ Forwards `semio-repo mcp …`: builds the repo command line, then serves the repo MCP server on
 * stdio with the profile taken from the first argument.
 */
class McpScript extends BundleScript {
  async run(segments:string[]):Promise<void>{
    await runDeclaredRustBuildV1(this.repoRoot,"@semio-tech/repo-cli-rs",this.root,this.invocation,[],"build-cli");
    process.exitCode=await runRepositoryPublishedCargoExecutableV1(join(this.root,"Cargo.toml"),"semio-repo"+(process.platform==="win32"?".exe":""),["mcp",...segments],this.repoRoot,readBuildMode(process.env.SEMIO_BUILD_MODE),this.invocation);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("build-cli", BuildCliScript).register("test", TestScript).register("mcp", McpScript).register("repo", RepoScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
