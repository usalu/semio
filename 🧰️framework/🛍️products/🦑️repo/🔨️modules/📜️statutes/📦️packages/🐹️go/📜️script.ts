#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** 🧭️ `repo-statutes-go` router: `repo ./📜️script.ts test`. */
import { goCoverageArgs, goLevelTestArgs, runCanonicalGoTests } from "../../../📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    const tags = level === "exhaustive" ? ["-tags", "exhaustive"] : [];
    await runCanonicalGoTests(import.meta.dirname, [...tags, ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, import.meta.dirname), ...rest], this.invocation);
  }
}

const router = new ScriptRouter(import.meta.dirname).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
