#!/usr/bin/env bun
import { receiveScriptProcessInvocation,scriptInvocationEnvironment,scriptInvocationBudget } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🛡️ Executes the import-free unknown-value shape guard contract. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.shift() !== "shape") throw new Error("Expected test shape");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🧪️tests/🧩️shape/🟦️.ts"), ...args], this.root, "schema-validator:shape", scriptInvocationBudget(this.invocation,15_000),{signal:this.invocation.control.signal,env:scriptInvocationEnvironment(this.invocation,process.env)});
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { invocation: original, ...({ defaultCommand: "test" }) }));
