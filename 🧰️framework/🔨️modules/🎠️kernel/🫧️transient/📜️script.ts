#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runOwnedCommand } from "../../🏃️process/🎛️owned-execution/🟦️.ts";

/** 🫧️ Executes the owned transient identity contract without loading a shell runtime. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [suite, ...rest] = args;
    if (suite !== "contract" && suite !== "retained") throw new Error("Expected test contract|retained");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "🧪️tests/🟦️.ts"), ...(suite === "retained" ? ["--test-name-pattern", "ephemeralBox"] : []), ...rest], this.root, "transient:contract", 15_000);
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { invocation: original, ...({ defaultCommand: "test" }) }));
