
import { receiveScriptProcessInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBuildScript, runTestScript } from "./🟦️.ts";

class BuildScript extends BundleScript {
  async run(): Promise<void> {
    await runBuildScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
