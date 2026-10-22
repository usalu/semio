
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBuildScript, runTestScript } from "../../🧪️tests/🔬️go-package-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";

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

if (isMainModuleV1(import.meta.url)) {
  const router = new ScriptRouter(import.meta.dirname).register("build", BuildScript).register("test", TestScript);
  await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
}
