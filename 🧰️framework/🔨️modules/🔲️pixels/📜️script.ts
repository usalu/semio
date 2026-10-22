#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

import { BundleScript, ScriptRouter } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runTestScript } from "./🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test", TestScript), { invocation: original, defaultCommand: "test" }));
