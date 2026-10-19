
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBuildScript, runTestScript, runDevScript } from "../../🧪️tests/🔬️go-package-commands/🟦️.ts";

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

/** ▶️ Serves the coordinator artifact completed by the owning build dependency. */
class DevScript extends BundleScript {
  async run(): Promise<void> {
    await runDevScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript).register("dev", DevScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
