import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🟦️.ts";

/** 🚪️ `cancel` — runs the program of this script. */
class CancelScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧭️ `retire` — runs the program of this script. */
class RetireScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🏭️ `claim-read` — runs the program of this script. */
class ClaimReadScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("cancel", CancelScript).register("retire", RetireScript).register("claim-read", ClaimReadScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
