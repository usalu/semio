#!/usr/bin/env bun
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runOwnedCommand } from "../../../../../🏃️process/🎛️owned-execution/🟦️.ts";

/** 🦀️ Checks general Rust source syntax with owned original and independent grammar laws. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("Expected test");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🟦️.ts"), resolve(this.root, "../../📁️paths/🧪️tests/🟦️.ts")], this.root, "compiler:syntax:rust", 60000);
  }
}
/** 🚮️ Checks all syntax laws with unrelated owners physically absent. */
class AbsenceScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("Expected test-absence");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🚮️absence/🟦️.ts")], this.root, "compiler:syntax:rust:absence", 30000);
  }
}
/** 📁️ Checks the closed neutral path argument contract with independent Rust grammar. */
class PathLiteralsScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("Expected test-path-literals");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../📁️paths/🧪️tests/🟦️.ts")], this.root, "compiler:syntax:rust:paths", 30000);
  }
}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-absence", AbsenceScript).register("test-path-literals", PathLiteralsScript), { defaultCommand: "test" });
