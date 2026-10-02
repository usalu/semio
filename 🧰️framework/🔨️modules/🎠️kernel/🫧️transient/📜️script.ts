#!/usr/bin/env bun
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runOwnedCommand } from "../../🏃️process/🎛️owned-execution/🟦️.ts";

/** 🫧️ Executes the owned transient identity contract without loading a shell runtime. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const suite = args.shift();
    if (suite !== "contract" && suite !== "retained") throw new Error("Expected test contract|retained");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "🧪️tests/🟦️.ts"), ...(suite === "retained" ? ["--test-name-pattern", "ephemeralBox"] : []), ...args], this.root, "transient:contract", 15_000);
  }
}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { defaultCommand: "test" });
