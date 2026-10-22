#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🔣️ Checks controlled lossless JSON syntax and explicit member selection. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Expected test");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🟦️.ts")], this.root, "pack:json:decode", 30000);
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test", TestScript), { invocation: original, ...({ defaultCommand: "test" }) }));
