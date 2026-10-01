#!/usr/bin/env bun
/** 🗄️ Dispatches commands contributed by present Stdio composition owners. */
import { BundleScript, ScriptRouter, dispatchOwnedScriptRoute, runBundleScriptMain } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
class ContractScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (!await dispatchOwnedScriptRoute(this.repoRoot, ["stdio", "package-contract", ...segments])) throw new Error("No admitted Stdio package-contract owner");
  }
}
class GraphScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (!await dispatchOwnedScriptRoute(this.repoRoot, ["stdio", "package-graph", ...segments])) throw new Error("No admitted Stdio package-graph owner");
  }
}
const router = new ScriptRouter(import.meta.dir).register("package-contract", ContractScript).register("package-graph", GraphScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "package-contract" });
