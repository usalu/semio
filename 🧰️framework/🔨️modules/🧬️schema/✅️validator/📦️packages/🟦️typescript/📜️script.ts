#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runOwnedCommand } from "../../../../🏃️process/🎛️owned-execution/🟦️.ts";

/** 🛡️ Executes the import-free unknown-value shape guard contract. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.shift() !== "shape") throw new Error("Expected test shape");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🧩️shape/🟦️.ts"), ...args], this.root, "schema-validator:shape", 15_000);
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { invocation: original, ...({ defaultCommand: "test" }) }));
