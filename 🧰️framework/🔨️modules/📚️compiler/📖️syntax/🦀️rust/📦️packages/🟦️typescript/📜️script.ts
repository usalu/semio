#!/usr/bin/env bun
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runOwnedCommand } from "../../../../../🏃️process/🎛️owned-execution/🟦️.ts";

/** 🦀️ Checks general Rust source syntax with owned original and independent grammar laws. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("Expected test");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🟦️.ts")], this.root, "compiler:syntax:rust", 30000);
  }
}
/** 🚮️ Checks all syntax laws with unrelated owners physically absent. */
class AbsenceScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("Expected test-absence");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🚮️absence/🟦️.ts")], this.root, "compiler:syntax:rust:absence", 30000);
  }
}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-absence", AbsenceScript), { defaultCommand: "test" });
