
import { BundleScript, ScriptRouter } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {receiveScriptProcessInvocation} from '../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
import {runScriptMain} from '../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts';
import { runOriginalTupleReceivingScript } from "./🧪️tests/🔬️script-commands/🟦️.ts";
import { isMainModuleV1 } from "../../../../../⚙️runtime/🏁️main/🟦️.ts";
/** 🧳️ Verifies original heterogeneous values, fixed native frontiers and physical partial-child cancellation. */
class OriginalTupleReceivingScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runOriginalTupleReceivingScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router=new ScriptRouter(import.meta.dirname).register('verify',OriginalTupleReceivingScript);
if(isMainModuleV1(import.meta.url))await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));
