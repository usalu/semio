
import { BundleScript, ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {receiveScriptProcessInvocation} from '../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
import {runScriptMain} from '../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts';
import { runOriginalSlotScript } from "./🧪️tests/🔬️script-commands/🟦️.ts";
/** 🔗️ Verifies the shared original history read schema, independent semantic output and physical native custody. */
class OriginalSlotScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runOriginalSlotScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router=new ScriptRouter(import.meta.dir).register('verify',OriginalSlotScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));
