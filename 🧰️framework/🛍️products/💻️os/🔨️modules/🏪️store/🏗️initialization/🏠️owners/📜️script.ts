
import { BundleScript, ScriptRouter } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {receiveScriptProcessInvocation} from '../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
import {runScriptMain} from '../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts';
import { runSharedSemanticSourceScript } from "./🟦️.ts";
import { isMainModuleV1 } from "../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";
/** 🏭️ Validates one quoted concrete semantic source and both persistent native ports. */
export class SharedSemanticSourceScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runSharedSemanticSourceScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router=new ScriptRouter(import.meta.dirname).register('verify',SharedSemanticSourceScript);
if(isMainModuleV1(import.meta.url))await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));
