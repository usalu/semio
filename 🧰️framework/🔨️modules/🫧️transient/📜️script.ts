#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🫧️ Executes the owned transient identity contract without loading a shell runtime. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [suite, ...rest] = args;
    if (suite !== "contract" && suite !== "retained") throw new Error("Expected test contract|retained");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "🧪️tests/🟦️.ts"), ...(suite === "retained" ? ["--test-name-pattern", "ephemeralBox"] : []), ...rest], this.root, "transient:contract", 15_000);
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { invocation: original, ...({ defaultCommand: "test" }) }));
