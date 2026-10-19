import { resolve } from "node:path";
import { runOwnedCommand } from "../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter, scriptInvocationBudget, runScriptMain, receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 📤️ Checks the original authored Value interfaces and their actual published consumers. */
abstract class ValueApiScript extends BundleScript {
  protected abstract readonly testFile: string;
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("Value API checks accept no arguments");
    const test = resolve(this.root, "../../🧪️tests/📤️api", this.testFile);
    await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun,node", test], this.repoRoot, "value:public-api:types", scriptInvocationBudget(this.invocation, 30_000), { signal: this.invocation.control.signal });
    await runOwnedCommand(process.execPath, ["test", test], this.repoRoot, "value:public-api:source", scriptInvocationBudget(this.invocation, 30_000), { signal: this.invocation.control.signal });
  }
}

/** 🗂️ Verifies original numeric index ownership through its published interfaces. */
class NumericApiScript extends ValueApiScript { protected readonly testFile = "🟦️.ts"; }
/** ⚠️ Verifies typed refusal and exact IEEE scalar authority through their published interfaces. */
class ScalarsApiScript extends ValueApiScript { protected readonly testFile = "⚠️scalars/🟦️.ts"; }

/** 🧾️ Verifies question dictionaries against their independent JSON Schema oracle and the native JSON limbs. */
class DictionaryScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("Dictionary checks accept no arguments");
    const test = resolve(this.root, "../../🧾️dictionary/🧪️tests/🟦️.ts");
    await runOwnedCommand(process.execPath, ["test", test], this.repoRoot, "value:dictionary:source", scriptInvocationBudget(this.invocation, 30_000), { signal: this.invocation.control.signal });
  }
}

/** 🗺️ Verifies actual fixed hash ownership against independent dictionary and JSON Patch oracles. */
class ScratchHashScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("Scratch hash checks accept no arguments");
    const test = resolve(this.root, "../../🔑️hashed/🧮️scratch/🧪️tests/🟦️.ts");
    await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun,node", test], this.repoRoot, "value:scratch-hash:types", scriptInvocationBudget(this.invocation, 30_000), { signal: this.invocation.control.signal });
    await runOwnedCommand(process.execPath, ["test", test], this.repoRoot, "value:scratch-hash:source", scriptInvocationBudget(this.invocation, 30_000), { signal: this.invocation.control.signal });
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, invocation => runScriptMain(new ScriptRouter(import.meta.dir).register("test-numeric-api", NumericApiScript).register("test-scalars-api", ScalarsApiScript).register("test-dictionary", DictionaryScript).register("test-scratch-hash", ScratchHashScript), { invocation }));
