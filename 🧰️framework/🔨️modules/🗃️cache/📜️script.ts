#!/usr/bin/env repo
import { resolve } from "node:path";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

/** 🚦 Run both neutral implementations and installed development oracles through registered tools. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args[0] !== "contract") throw Error("Expected test contract");
    await runOwnedCommand("repo", ["tools", "exec", "--light", "python", "--", "-B", resolve(this.root, "🧪️tests/🐍️.py")], this.root, "cache:python", 15000);
    await runOwnedCommand("repo", ["tools", "exec", "--light", "node", "--", "--test", resolve(this.root, "🧪️tests/🟨️.cjs")], this.root, "cache:typescript-oracle", 15000);
  }
}
await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dirname).register("test", TestScript), { invocation: original, ...({ defaultCommand: "test" }) }));
