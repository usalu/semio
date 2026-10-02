#!/usr/bin/env bun
import { resolveTestLevel } from "../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts";
import contract from "./🧫️fixtures/🔮️ownership/🔣️.json";
import { resolve } from "node:path";

/** 🧩️ Proves real retained source ownership and hostile fixture semantics. */
class OwnershipTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryTestCommand(process.execPath, ["test", "--timeout", "30000", "./🧪️tests/🔮️ownership/🟦️.ts", ...rest], { cwd: this.root, budgetMs: 120_000 });
  }
}

/** 🦀️ Runs every original selected artifact cohort with its existing assembly feature. */
class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    if (rest.length) throw new Error("The full Puzzle native cohort accepts no test or scenario filters.");
    for (const artifact of contract.artifacts) await runRepositoryTestCommand(process.execPath, ["nx", "run", `${artifact.project}:test`, "--skip-nx-cache", "--", "--features", "component-app-assembly"], { cwd: resolve(this.root, "../../../.."), budgetMs: 3_600_000 });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test-ownership", OwnershipTestScript).register("test-native", NativeTestScript);
if (import.meta.main) await runScriptMain(router, { defaultCommand: "test-ownership" });
