#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryTestCommand } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import { runNativeTestScript } from "./🟦️.ts";

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
    await runNativeTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test-ownership", OwnershipTestScript).register("test-native", NativeTestScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test-ownership" }));
