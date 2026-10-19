import{BundleScript,ScriptRouter,receiveScriptProcessInvocation,createIssuedResourceControlV1}from"./../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withPrintOperationV1 } from "./../../../📦️artifacts/🎛️operation/🟦️.ts";
import { preparePrintMaterialV1 } from "./../../../📦️artifacts/📥️material/📤️producer/🟦️.ts";
import { runVerifyScript } from "./🟦️.ts";
class PrepareScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw new Error("Tectonic preparation accepts no arguments");const phaseControl=createIssuedResourceControlV1(this.invocation,"print:toolchain:prepare",0);const result=await withPrintOperationV1({workspace:this.repoRoot,invocation:this.invocation,phaseControl,kind:"toolchain"},(operation,actualPhase)=>preparePrintMaterialV1(operation,"tectonic",actualPhase));console.log(`[print-toolchain] Published ${result.files} files/${result.bytes} bytes`);}}
class VerifyScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
const router=new ScriptRouter(import.meta.dir).register("prepare",PrepareScript).register("verify",VerifyScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>router.run(process.argv.slice(2),original));
