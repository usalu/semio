#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runOwnedCommand } from "../../../../../🏃️process/🎛️owned-execution/🟦️.ts";

/** 🔣️ Checks controlled lossless JSON syntax and explicit member selection. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("Expected test");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🟦️.ts")], this.root, "pack:json:decode", 30000);
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { invocation: original, ...({ defaultCommand: "test" }) }));
