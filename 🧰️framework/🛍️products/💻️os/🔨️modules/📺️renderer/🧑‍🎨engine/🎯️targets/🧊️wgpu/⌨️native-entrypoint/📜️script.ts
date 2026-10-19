
import { receiveScriptProcessInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRunScript, runMcpScript, runScaleScript } from "./🟦️.ts";

const ownerPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";

/** 🖥️ Runs completed native artifacts without compiling or invoking another build graph. */
class RunScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runRunScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

/** 🌉 Runs the exact caller-declared MCP producer with its authored service inventory. */
class McpScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runMcpScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

/** ⚖️ Runs the native scale consumer with only its binary and fixture prerequisites. */
class ScaleScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runScaleScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir).register("run", RunScript).register("scale", ScaleScript).register("mcp",McpScript)).run(process.argv.slice(2), original));
