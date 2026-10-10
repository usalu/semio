#!/usr/bin/env bun
import { configuredExactCargoLawPolicyV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../../../🧰️framework/🔨️modules/🏃️process/⏱️budget/🟦️.ts";
/** 🧪️ Concrete implementation composition law routing. */
import { runRepositoryExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { compositionLawGroups, testCompositionOwnership } from "../../🧪️tests/🔬️ownership/🟦️.ts";
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
    const groups = compositionLawGroups().filter(group => target === undefined || group.target.name === target).map(group => ({ ...group, laws: group.laws.filter(law => pattern === undefined || law.includes(pattern)) })).filter(group => group.laws.length > 0);
    if (!groups.length || (selected >= 0 && !target)) throw new Error("unknown composition target or empty law selection");
    testCompositionOwnership(target);
    const cargoArgs = rest.filter((_value, index) => (selected < 0 || index !== selected && index !== selected + 1) && (filtered < 0 || index !== filtered && index !== filtered + 1));
    await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), lawMilliseconds: 600_000 }, cwd: this.repoRoot, cargoArgs, env: process.env, nativeEnv: { RUST_MIN_STACK: process.env.RUST_MIN_STACK ?? "134217728" }, groups, progress(event) { console.log(`[composition-laws] ${event.stage}: ${event.law ?? ""}`); } });
  }
}
const router = new ScriptRouter(import.meta.dir).register("source-check", SourceScript).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "source-check" }) }));
