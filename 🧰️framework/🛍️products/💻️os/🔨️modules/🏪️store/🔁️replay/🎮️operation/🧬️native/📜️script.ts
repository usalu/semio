
import { BundleScript, ScriptRouter } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {receiveScriptProcessInvocation} from '../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
import {runScriptMain} from '../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts';
import { runNativeSemanticPreparationScript } from "./🟦️.ts";
/** 🔗️ Verifies the shared original history read schema, independent semantic output and physical native custody. */
class NativeSemanticPreparationScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runNativeSemanticPreparationScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router=new ScriptRouter(import.meta.dir).register('verify',NativeSemanticPreparationScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));
