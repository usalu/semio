
import { BundleScript, ScriptRouter } from "../../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import {receiveScriptProcessInvocation} from '../../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts';
import {runScriptMain} from '../../../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts';
import { runEnergyRetainedModelStateScript } from "./🟦️.ts";
/** 🧳️ Verifies Energy typed original model authority, sparse inverses and bounded immutable row ownership. */
class EnergyRetainedModelStateScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runEnergyRetainedModelStateScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router=new ScriptRouter(import.meta.dir).register('verify',EnergyRetainedModelStateScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));