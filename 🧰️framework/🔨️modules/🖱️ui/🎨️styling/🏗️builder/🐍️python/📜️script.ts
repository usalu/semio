import{BundleScript,ScriptRouter,receiveScriptProcessInvocation,type ScriptInvocation}from"../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{scriptCompilerAdmissionControlV1}from"../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import{buildBudgetMs}from"../../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import{defineScriptProviderV1,loadScriptProviderV1}from"../../../../🏃️process/📦️packages/🟦️typescript/🔌️providers/🟦️.ts";
import type{ResourceLeaseControl}from"../../../../🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type{StylingPythonOperationV1,StylingPythonProviderV1,StylingPythonWheelViewV1}from"./🔌️provider/🟦️.ts";
import{resolve}from"node:path";
/** 🧷️ Names the port and the exact functions its repository implementation must expose. */
export const STYLING_PYTHON_PROVIDER_V1=defineScriptProviderV1<StylingPythonProviderV1<ScriptInvocation,ResourceLeaseControl>>("styling-python",["run","withWheel"]);
/** 🏗️ Runs one producer operation under the provider's storage and custody. */
export async function runAdmittedStylingPythonV1(invocation:ScriptInvocation,phase:ResourceLeaseControl,operation:StylingPythonOperationV1):Promise<void>{return(await loadScriptProviderV1(STYLING_PYTHON_PROVIDER_V1)).run(invocation,phase,operation);}
/** 👓️ Retains the published wheel and its environment through the consumer. */
export async function withAdmittedStylingPythonWheelV1<T>(invocation:ScriptInvocation,phase:ResourceLeaseControl,body:(view:StylingPythonWheelViewV1<ResourceLeaseControl>)=>Promise<T>):Promise<T>{return(await loadScriptProviderV1(STYLING_PYTHON_PROVIDER_V1)).withWheel(invocation,phase,body);}
/** 📥️ Mutates only the genuinely held locked Python dependency environment. */
export class StylingPythonDepsScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw new Error("Python deps accepts no options");await runAdmittedStylingPythonV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"styling:python-deps",buildBudgetMs()),"deps");}}
/** 🎡️ Publishes one wheel after actual compiler CPU and accepted children settle. */
export class StylingPythonBuildScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw new Error("Python build accepts no options");await runAdmittedStylingPythonV1(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"styling:python-build",buildBudgetMs()),"build");}}
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>new ScriptRouter(resolve(import.meta.dir,"../../📦️packages/🐍️python")).register("deps",StylingPythonDepsScript).register("build",StylingPythonBuildScript).run(process.argv.slice(2),original));
