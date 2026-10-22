
import { BundleScript, ScriptRouter } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {receiveScriptProcessInvocation} from '../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
import {runScriptMain} from '../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts';
import { runOriginalSlotScript } from "./🧪️tests/🔬️script-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../⚙️runtime/🏁️main/🟦️.ts";
/** 🔗️ Verifies the shared original history read schema, independent semantic output and physical native custody. */
class OriginalSlotScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runOriginalSlotScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router=new ScriptRouter(import.meta.dirname).register('verify',OriginalSlotScript);
if(isMainModuleV1(import.meta.url))await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));
