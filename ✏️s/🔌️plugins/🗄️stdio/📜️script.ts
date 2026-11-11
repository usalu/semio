#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🗄️ Dispatches commands contributed by present Stdio composition owners. */
import { dispatchOwnedScriptRoute } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
class ContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (!await dispatchOwnedScriptRoute(this.invocation,this.repoRoot, ["stdio", "package-contract", ...segments])) throw new Error("No admitted Stdio package-contract owner");
  }
}
class GraphScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (!await dispatchOwnedScriptRoute(this.invocation,this.repoRoot, ["stdio", "package-graph", ...segments])) throw new Error("No admitted Stdio package-graph owner");
  }
}
const router = new ScriptRouter(import.meta.dir).register("package-contract", ContractScript).register("package-graph", GraphScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "package-contract" }) }));
