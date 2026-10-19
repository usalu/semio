import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🟦️.ts";

/** 🚪️ `dev` — runs the program of this script. */
class DevScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧭️ `build` — runs the program of this script. */
class BuildScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🏭️ `test` — runs the program of this script. */
class TestScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧪️ `view` — runs the program of this script. */
class ViewScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript).register("view", ViewScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
