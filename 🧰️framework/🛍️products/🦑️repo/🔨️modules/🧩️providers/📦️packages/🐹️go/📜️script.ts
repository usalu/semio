#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** 🧭️ `@semio-tech/repo-providers-go` router: `bun ./📜️script.ts test`. */
import { goCoverageArgs } from "../../../📚️library/🧪️testing/📊️coverage/🟦️.ts";
import { goLevelTestArgs } from "../../../📚️library/🧪️testing/🎚️levels/🟦️.ts";
import { runCanonicalGoTests } from "../../../📚️library/🧪️testing/🐹️go-plan/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const packageRoot = import.meta.dir;

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runCanonicalGoTests(packageRoot, [...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, packageRoot), ...rest], { env: { ...process.env, GOWORK: "off" } });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
