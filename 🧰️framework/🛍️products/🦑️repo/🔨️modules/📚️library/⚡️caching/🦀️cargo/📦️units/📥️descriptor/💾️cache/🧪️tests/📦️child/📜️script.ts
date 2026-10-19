/** 📦️ Child: child-process script that receives the issued script-process invocation and runs the case its arguments select. */
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptProgram } from "./🟦️.ts";

/** 🚪️ `catalog-page-pressure` — runs the program of this script. */
class CatalogPagePressureScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧭️ `COMMIT` — runs the program of this script. */
class COMMITScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🏭️ `ROLLBACK` — runs the program of this script. */
class ROLLBACKScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🧪️ `waiting-maintenance` — runs the program of this script. */
class WaitingMaintenanceScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 📦️ `joining-reader` — runs the program of this script. */
class JoiningReaderScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
/** 🔧️ `hold-reader` — runs the program of this script. */
class HoldReaderScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("catalog-page-pressure", CatalogPagePressureScript).register("COMMIT", COMMITScript).register("ROLLBACK", ROLLBACKScript).register("waiting-maintenance", WaitingMaintenanceScript).register("joining-reader", JoiningReaderScript).register("hold-reader", HoldReaderScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
