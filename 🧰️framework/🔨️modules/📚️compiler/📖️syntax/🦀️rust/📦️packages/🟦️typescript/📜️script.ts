#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🦀️ Checks general Rust source syntax with owned original and independent grammar laws. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Expected test");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🟦️.ts"), resolve(this.root, "../../📁️paths/🧪️tests/🟦️.ts")], this.root, "compiler:syntax:rust", 60000);
  }
}
/** 🚮️ Checks all syntax laws with unrelated owners physically absent. */
class AbsenceScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Expected test-absence");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🚮️absence/🟦️.ts")], this.root, "compiler:syntax:rust:absence", 30000);
  }
}
/** 📁️ Checks the closed neutral path argument contract with independent Rust grammar. */
class PathLiteralsScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Expected test-path-literals");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../📁️paths/🧪️tests/🟦️.ts")], this.root, "compiler:syntax:rust:paths", 30000);
  }
}
/** 🌳️ Checks module tree reading of split sources against the Rust grammar, the Rust compiler and the TypeScript parser. */
class ModuleTreeScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Expected test-module-tree");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../../🌳️module-tree/🧪️tests/🟦️.ts")], this.root, "compiler:syntax:module-tree", 60000);
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test", TestScript).register("test-absence", AbsenceScript).register("test-path-literals", PathLiteralsScript).register("test-module-tree", ModuleTreeScript), { invocation: original, ...({ defaultCommand: "test" }) }));
