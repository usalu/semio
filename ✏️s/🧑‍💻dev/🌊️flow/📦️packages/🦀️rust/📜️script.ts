#!/usr/bin/env bun
/** 🌊️ First-party Flow composition has its own bounded native and portable law runner. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runExactCargoLaws, buildBudgetMs, runBun } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { testFlowCompositionOwnership, flowCompositionLaws } from "../../🧪️tests/🏷️ownership/🟦️.ts";
class PortSidesScript extends BundleScript { async run(): Promise<void> { runBun(["test", "../../🧪️tests/🔌️port-sides/🟦️.ts"], import.meta.dir); } }
class SourceScript extends BundleScript {
  async run(): Promise<void> { testFlowCompositionOwnership(); }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    testFlowCompositionOwnership();
    await runExactCargoLaws({ cwd: this.repoRoot, cargoArgs: segments, buildBudgetMs: buildBudgetMs(), lawBudgetMs: 600_000,
      groups: flowCompositionLaws().map(group => ({ package: "semio-s-flow-composition", target: { kind: "test", name: group.target }, laws: group.laws })) });
  }
}
class CanonicalScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length !== 0) throw new Error("canonical-architecture takes no arguments");
    testFlowCompositionOwnership();
    runBun(["test","../../🧪️tests/🔌️port-sides/🟦️.ts"],import.meta.dir);
    await runExactCargoLaws({ cwd:this.repoRoot,cargoArgs:["--locked"],buildBudgetMs:buildBudgetMs(),lawBudgetMs:600_000,
      groups:flowCompositionLaws().map(group => ({ package:"semio-s-flow-composition",target:{kind:"test",name:group.target},laws:group.laws })) });
  }
}
const router = new ScriptRouter(import.meta.dir).register("canonical-architecture", CanonicalScript).register("port-sides-test", PortSidesScript).register("source-check", SourceScript).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "source-check" });
