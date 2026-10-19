import { receiveScriptProcessInvocation } from "../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { BundleScript, ScriptRouter, runScriptMain } from "../../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🟦️.ts";
/** 🚪️ `undefined` — runs the program of this script. */
class UndefinedScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧭️ `overgrant` — runs the program of this script. */
class OvergrantScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🏭️ `detached` — runs the program of this script. */
class DetachedScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧪️ `replay` — runs the program of this script. */
class ReplayScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 📦️ `error` — runs the program of this script. */
class ErrorScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🔧️ `crash` — runs the program of this script. */
class CrashScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("undefined", UndefinedScript).register("overgrant", OvergrantScript).register("detached", DetachedScript).register("replay", ReplayScript).register("error", ErrorScript).register("crash", CrashScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
