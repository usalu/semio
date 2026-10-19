#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
/** 🗄️ Dispatches commands contributed by present Stdio composition owners. */
import { dispatchOwnedScriptRoute } from "../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
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
