#!/usr/bin/env bun
import{scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1}from"../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { buildBudgetMs } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🌊️ First-party Flow composition has its own bounded native and portable law runner. */
import { runRepositoryExactCargoLaws, runBun, configuredRepositoryCargoPreparationPolicyV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { testFlowCompositionOwnership, flowCompositionLaws } from "../../🧪️tests/🏷️ownership/🟦️.ts";
class PortSidesScript extends BundleScript { async run(): Promise<void> { runBun(["test", "../../🧪️tests/🔌️port-sides/🟦️.ts"], import.meta.dir); } }
class SourceScript extends BundleScript {
  async run(): Promise<void> { testFlowCompositionOwnership(); }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    testFlowCompositionOwnership();
    await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), preparation: configuredRepositoryCargoPreparationPolicyV1(), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), lawMilliseconds: 600_000 }, cwd: this.repoRoot, cargoArgs: segments, groups: flowCompositionLaws().map(group => ({ package: "semio-s-flow-composition", target: { kind: "test", name: group.target }, laws: group.laws })) });
  }
}
class CanonicalScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length !== 0) throw new Error("canonical-architecture takes no arguments");
    testFlowCompositionOwnership();
    runBun(["test","../../🧪️tests/🔌️port-sides/🟦️.ts"],import.meta.dir);
    await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), preparation: configuredRepositoryCargoPreparationPolicyV1(), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), lawMilliseconds: 600_000 }, cwd:this.repoRoot, cargoArgs:["--locked"], groups:flowCompositionLaws().map(group => ({ package:"semio-s-flow-composition",target:{kind:"test",name:group.target},laws:group.laws })) });
  }
}
const router = new ScriptRouter(import.meta.dir).register("canonical-architecture", CanonicalScript).register("port-sides-test", PortSidesScript).register("source-check", SourceScript).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "source-check" }) }));
