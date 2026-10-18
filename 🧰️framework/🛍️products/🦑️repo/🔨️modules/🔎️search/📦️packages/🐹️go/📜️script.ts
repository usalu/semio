#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
/** 🧭️ `@semio-tech/repo-search-go` router: `bun ./📜️script.ts test`. */
import { dirname, join } from "node:path";
import { goLevelTestArgs } from "../../../📚️library/🧪️testing/🎚️levels/🟦️.ts";
import { goCoverageArgs } from "../../../📚️library/🧪️testing/📊️coverage/🟦️.ts";
import { runCanonicalGoTests } from "../../../📚️library/🧪️testing/🐹️go-plan/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const moduleRoot = import.meta.dir;
const ownerRoot = join(dirname(dirname(moduleRoot)));

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runCanonicalGoTests(moduleRoot, [...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, ownerRoot), ...rest], { env: { ...process.env, GOWORK: join(this.repoRoot, "go.work") } });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
