#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🟨️ Checks owned ECMA lexical evidence against independent compiler oracles. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("Expected test");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🟦️.ts")], this.root, "compiler:syntax:ecma", 60_000);
  }
}

await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { invocation: original, ...({ defaultCommand: "test" }) }));
