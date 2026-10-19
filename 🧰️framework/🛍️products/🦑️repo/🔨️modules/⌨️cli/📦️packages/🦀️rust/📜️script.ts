
import { runDeclaredRustBuildV1, runRepositoryCargoTests } from "../../../📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepoScript, runMcpScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

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
  async run(segments:string[]): Promise<void> {
    await runRepoScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/**
 * 🔌️ Forwards `semio-repo mcp …`: builds the repo command line, then serves the repo MCP server on
 * stdio with the profile taken from the first argument.
 */
class McpScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runMcpScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("build-cli", BuildCliScript).register("test", TestScript).register("mcp", McpScript).register("repo", RepoScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
