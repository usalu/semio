import{BundleScript,ScriptRouter,receiveScriptProcessInvocation,createIssuedResourceControlV1}from"./../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{withPrintOperationV1}from"./../../../📦️artifacts/🎛️operation/🟦️.ts";
import{preparePrintMaterialV1}from"./../../../📦️artifacts/📥️material/📤️producer/🟦️.ts";
class PrepareScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw new Error("Print bundle preparation accepts no arguments");const phaseControl=createIssuedResourceControlV1(this.invocation,"print:bundle:prepare",0),result=await withPrintOperationV1({workspace:this.repoRoot,invocation:this.invocation,phaseControl,kind:"bundle"},(operation,actualPhase)=>preparePrintMaterialV1(operation,"tex",actualPhase));console.log(`[print-bundle] Published ${result.files} files/${result.bytes} bytes`);}}
const router=new ScriptRouter(import.meta.dir).register("prepare",PrepareScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>router.run(process.argv.slice(2),original));
