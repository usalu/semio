#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import {runOwnedCommand} from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
/** 🧱️ Block source, schema, and publication-authority laws. */
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runTestScript } from "../../🧪️tests/🔬️typescript-package-commands/🟦️.ts";

class TestScript extends BundleScript {
  async run(): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}
class SharedSchemaScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("shared-schema accepts no arguments");await runOwnedCommand(process.execPath,["test",resolve(this.root,"../../🧬️schema/🧱️document-fields/🧪️tests/🟦️.ts")],this.repoRoot,"block-shared-schema",45000,{env:process.env});}
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("publication-authority-audit", TestScript).register("shared-schema",SharedSchemaScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
