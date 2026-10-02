#!/usr/bin/env bun
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runOwnedCommand } from "../🏃️process/🎛️owned-execution/🟦️.ts";

/** 🌱️ Runs the schema-first neutral value ownership and actual products-absent compiler laws. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const suite=args.shift();
    if(suite!=="neutral-owner"&&suite!=="controlled-construction")throw new Error("value test requires an explicitly owned suite");
    await runOwnedCommand(process.execPath, ["test", resolve(this.root,suite==="neutral-owner"?"🧪️tests/🧩️neutral-owner/🟦️.ts":"🔁️codec/🧪️tests/🛬️controlled/🟦️.ts"), ...args], this.root, "value:neutral-owner", 120000);
  }
}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript), { defaultCommand: "test" });
