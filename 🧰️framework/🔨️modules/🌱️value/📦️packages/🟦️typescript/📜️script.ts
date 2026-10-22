import { resolve } from "node:path";
import { runOwnedCommand, resolveTypescriptCompilerV1 } from "../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter, scriptInvocationBudget, runScriptMain, receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { NumericApiScript, ScalarsApiScript } from "../../🧪️tests/🔬️typescript-package-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../⚙️runtime/🏁️main/🟦️.ts";


/** 🧾️ Verifies question dictionaries against their independent JSON Schema oracle and the native JSON limbs. */
class DictionaryScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Dictionary checks accept no arguments");
    const test = resolve(this.root, "../../🧾️dictionary/🧪️tests/🟦️.ts");
    await runOwnedCommand(process.execPath, ["test", test], this.repoRoot, "value:dictionary:source", scriptInvocationBudget(this.invocation, 30_000), { signal: this.invocation.control.signal });
  }
}

/** 🗺️ Verifies actual fixed hash ownership against independent dictionary and JSON Patch oracles. */
class ScratchHashScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Scratch hash checks accept no arguments");
    const test = resolve(this.root, "../../🔑️hashed/🧮️scratch/🧪️tests/🟦️.ts");
    await runOwnedCommand(process.execPath, [resolveTypescriptCompilerV1(this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun,node", test], this.repoRoot, "value:scratch-hash:types", scriptInvocationBudget(this.invocation, 30_000), { signal: this.invocation.control.signal });
    await runOwnedCommand(process.execPath, ["test", test], this.repoRoot, "value:scratch-hash:source", scriptInvocationBudget(this.invocation, 30_000), { signal: this.invocation.control.signal });
  }
}

if (isMainModuleV1(import.meta.url)) await receiveScriptProcessInvocation(process.env, invocation => runScriptMain(new ScriptRouter(import.meta.dirname).register("test-numeric-api", NumericApiScript).register("test-scalars-api", ScalarsApiScript).register("test-dictionary", DictionaryScript).register("test-scratch-hash", ScratchHashScript), { invocation }));