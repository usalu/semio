
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { testCompositionOwnership } from "../../🧪️tests/🔬️ownership/🟦️.ts";
import { runTestScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
class SourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("source-check accepts no arguments");
    testCompositionOwnership();
  }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("source-check", SourceScript).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "source-check" }));
