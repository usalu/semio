/** 📦️ Child: child-process script that receives the issued script-process invocation and runs the case its arguments select. */
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🟦️.ts";

/** 🚪️ `missing` — runs the program of this script. */
class MissingScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧭️ `copied` — runs the program of this script. */
class CopiedScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🏭️ `retired` — runs the program of this script. */
class RetiredScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧪️ `drain` — runs the program of this script. */
class DrainScript extends BundleScript {
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
/** 🔧️ `undefined` — runs the program of this script. */
class UndefinedScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🗂️ `cancel` — runs the program of this script. */
class CancelScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("missing", MissingScript).register("copied", CopiedScript).register("retired", RetiredScript).register("drain", DrainScript).register("error", ErrorScript).register("undefined", UndefinedScript).register("cancel", CancelScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
