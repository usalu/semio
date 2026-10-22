#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** 🧭️ `@semio-tech/repo-search-go` router: `repo ./📜️script.ts test`. */
import { dirname, join } from "node:path";
import { goLevelTestArgs, goCoverageArgs, runCanonicalGoTests } from "../../../📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const moduleRoot = import.meta.dirname;
const ownerRoot = join(dirname(dirname(moduleRoot)));

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runCanonicalGoTests(moduleRoot, [...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, ownerRoot), ...rest], this.invocation, { env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") } });
  }
}

const router = new ScriptRouter(import.meta.dirname).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
