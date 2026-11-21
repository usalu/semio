#!/usr/bin/env repo
import { writeSync } from "node:fs";
import { receiveScriptProcessInvocation } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BoundedPreparationScript, BoundedSynchronizeScript } from "./🧪️tests/🔬️script-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import { cargoPreparationSourceEditDiagnosticV1 } from "../../../../../🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🟦️.ts";


const router = new ScriptRouter(import.meta.dirname).register("prepare", BoundedPreparationScript).register("synchronize", BoundedSynchronizeScript);
if (isMainModuleV1(import.meta.url)) {
  const args = process.argv.slice(2);
  await receiveScriptProcessInvocation(process.env, async original => {
    try {
      await router.run(args, original);
    } catch (error) {
      const diagnostic = args[0] === "synchronize" && !original.control.signal.aborted && cargoPreparationSourceEditDiagnosticV1(error);
      if (diagnostic) {
        try { writeSync(2, diagnostic + "\n"); } catch {}
      }
      throw error;
    }
  });
}
