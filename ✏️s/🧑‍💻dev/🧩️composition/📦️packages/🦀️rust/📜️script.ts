#!/usr/bin/env bun
/** 🧪️ Concrete implementation composition law routing. */
import { runExactCargoLaws, resolveTestLevel, buildBudgetMs } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
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
    const groups = compositionLawGroups().filter(group => target === undefined || group.target.name === target);
    if (!groups.length || (selected >= 0 && !target)) throw new Error("unknown composition test target");
    testCompositionOwnership(target);
    const cargoArgs = selected < 0 ? rest : rest.filter((_value, index) => index !== selected && index !== selected + 1);
    await runExactCargoLaws({ cwd: this.repoRoot, cargoArgs, buildBudgetMs: buildBudgetMs(), lawBudgetMs: 600_000, env: process.env, nativeEnv: { RUST_MIN_STACK: process.env.RUST_MIN_STACK ?? "134217728" }, groups, progress(event) { console.log(`[composition-laws] ${event.stage}: ${event.law ?? ""}`); } });
  }
}
const router = new ScriptRouter(import.meta.dir).register("source-check", SourceScript).register("test", TestScript);
await runScriptMain(router, { defaultCommand: "source-check" });
