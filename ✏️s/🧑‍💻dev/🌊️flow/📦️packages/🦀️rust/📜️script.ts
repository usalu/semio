
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBun } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🍞️bun/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { testFlowCompositionOwnership } from "../../🧪️tests/🏷️ownership/🟦️.ts";
import { runTestScript, runCanonicalScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
class PortSidesScript extends BundleScript { async run(): Promise<void> { runBun(["test", "../../🧪️tests/🔌️port-sides/🟦️.ts"], import.meta.dir); } }
class SourceScript extends BundleScript {
  async run(): Promise<void> { testFlowCompositionOwnership(); }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
class CanonicalScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runCanonicalScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router = new ScriptRouter(import.meta.dir).register("canonical-architecture", CanonicalScript).register("port-sides-test", PortSidesScript).register("source-check", SourceScript).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "source-check" }));
