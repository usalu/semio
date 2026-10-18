#!/usr/bin/env bun
import{scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1}from"../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
/** 🧪️ Concrete implementation composition law routing. */
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { compositionLawGroups, inTargetFamily, RETAINED_ARTIFACT_UNITS, retainedLawGroups, testCompositionOwnership } from "../../🧪️tests/🔬️ownership/🟦️.ts";
class SourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("source-check accepts no arguments");
    testCompositionOwnership();
  }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    const selected = rest.indexOf("--test");
    const target = selected < 0 ? undefined : rest[selected + 1];
    const filtered = rest.indexOf("--law-filter");
    const pattern = filtered < 0 ? undefined : rest[filtered + 1];
    if (filtered >= 0 && !pattern) throw new Error("law filter requires a name fragment");
    const groups = (target === RETAINED_ARTIFACT_UNITS ? retainedLawGroups() : compositionLawGroups().filter(group => inTargetFamily(group.target.name, target))).map(group => ({ ...group, laws: group.laws.filter(law => pattern === undefined || law.includes(pattern)) })).filter(group => group.laws.length > 0);
    if (!groups.length || (selected >= 0 && !target)) throw new Error("unknown composition target or empty law selection");
    testCompositionOwnership(target);
    const cargoArgs = rest.filter((_value, index) => (selected < 0 || index !== selected && index !== selected + 1) && (filtered < 0 || index !== filtered && index !== filtered + 1));
    await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), preparation: configuredRepositoryCargoPreparationPolicyV1(), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), lawMilliseconds: 600_000 }, cwd: this.repoRoot, cargoArgs, env: process.env, nativeEnv: { RUST_MIN_STACK: process.env.RUST_MIN_STACK ?? "134217728" }, groups, progress(event) { console.log(`[composition-laws] ${event.stage}: ${event.law ?? ""}`); } });
  }
}
const router = new ScriptRouter(import.meta.dir).register("source-check", SourceScript).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "source-check" }) }));
