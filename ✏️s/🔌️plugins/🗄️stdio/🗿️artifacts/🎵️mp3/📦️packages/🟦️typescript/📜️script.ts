#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter, runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptProgram } from "../../🧪️tests/🔬️typescript-package-commands/🟦️.ts";
/** 🚪️ `metadata-oracle` — runs the program of this script. */
class MetadataOracleScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧭️ `metadata-native` — runs the program of this script. */
class MetadataNativeScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🏭️ `metadata-source` — runs the program of this script. */
class MetadataSourceScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("metadata-oracle", MetadataOracleScript).register("metadata-native", MetadataNativeScript).register("metadata-source", MetadataSourceScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: ["metadata-oracle"] }));
