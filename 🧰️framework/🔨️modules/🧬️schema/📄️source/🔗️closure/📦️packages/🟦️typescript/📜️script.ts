#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runOwnedCommand } from "../../../../../🏃️process/🎛️owned-execution/🟦️.ts";

/** 🗂️ Checks captured schema resource closure and operation authority. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Expected test");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🟦️.ts")], this.root, "schema:source:closure", 30000);
  }
}
/** 🚮️ Checks the complete lower laws with every higher product physically absent. */
class AbsenceScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Expected test-absence");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🚮️absence/🟦️.ts")], this.root, "schema:source:closure:absence", 30000);
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-absence", AbsenceScript), { invocation: original, ...({ defaultCommand: "test" }) }));
