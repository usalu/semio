import{prepareRepoCpuProfileArgumentsV1}from"../../⏱️profile/📊️cpu/🟨️.mjs";
import{withDependencyReadmissionV1}from"../../📥️source/🔏️closure/🔁️dependencies/🟦️.ts";
import{executeCapturedBuildSourceV1}from"../../📥️source/🔏️closure/🚪️command/🟦️.ts";
import closureSchema from"../../📥️source/🔏️closure/🧬️schema/🔣️.json"with{type:"json"};
import{profileMarkV1}from"../../⏱️profile/🟨️.mjs";
import{runOwnedCommand,writeOwnedCommandOutputV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import{requireIssuedResourceControlInvocationV1,requireIssuedScriptInvocationV1,scriptResourceProcessEnvironmentV1,scriptInvocationBudget,withIssuedResourceOperationV1,type ScriptInvocation}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type{ResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{relative}from"node:path";
import{fileURLToPath}from"node:url";
import{implementationModeV1,readSealedLoaderV1,readSealedPointerV1,readSealedSnapshotV1,sealedEntryOfV1,sealedLanguagesV1,sealedPolicyV1,sealedTextV1}from"../🟦️.ts";

/** 🚦️ Runs one command of the build tool from the implementation its mode selects: `live` captures the live tree for this command exactly as before (the opt-in of the agent that edits the tool), anything else runs the stored snapshot the front-door trampoline promoted (the same snapshot, digest and loader the front door itself runs from). A store without a readable snapshot says so in both languages and runs the live tree, so the sealed layer is never the reason a command does not run.
 * @see ../🚦️trampoline/🟦️.ts who promotes, ../🟦️.ts the policy and the store */
async function executeImplementationOnceV1(workspace:string,entry:string,args:readonly string[],original:ScriptInvocation,control:ResourceLeaseControl,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<void>{
 requireIssuedScriptInvocationV1(original);requireIssuedResourceControlInvocationV1(control,original);
 if(implementationModeV1(environment)==="live")return executeCapturedBuildSourceV1(workspace,entry,args,original,control,environment);
 const store=sealedEntryOfV1(workspace,relative(workspace,fileURLToPath(new URL("../🚪️entry/🟦️.ts",import.meta.url))).split("\\").join("/")),pointer=readSealedPointerV1(store),text=pointer&&readSealedSnapshotV1(store,pointer),loader=pointer&&readSealedLoaderV1(store,pointer);
 if(!pointer||text===undefined||loader===undefined){void writeOwnedCommandOutputV1(process.stderr,sealedTextV1(sealedLanguagesV1(environment),"fallback",{reason:"no readable sealed snapshot"})+"\n",control);return executeCapturedBuildSourceV1(workspace,entry,args,original,control,environment);}
 profileMarkV1("source.served","sealed:"+pointer.digest.slice(0,12));
 const argv=[...prepareRepoCpuProfileArgumentsV1(environment),"--eval",loader,"--",pointer.digest,...args],quote=(value:string)=>'"'+value.replace(/(\\*)"/gu,'$1$1\\"').replace(/(\\+)$/u,'$1$1')+'"';
 if(Buffer.byteLength(text)>closureSchema.definitions.Policy.const.maximumSnapshotBytes||[process.execPath,...argv].map(quote).join(" ").length+1>sealedPolicyV1.maximumCommandLineCharacters)throw Error(sealedTextV1(sealedLanguagesV1(environment),"commandPublicationBounded"));
 requireIssuedResourceControlInvocationV1(control,original);
 await withIssuedResourceOperationV1(control,()=>runOwnedCommand(process.execPath,argv,process.cwd(),"repository:command",scriptInvocationBudget(original,0),{control,signal:control.signal,env:{...scriptResourceProcessEnvironmentV1(control,environment),BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"},auxiliaryInput:{version:1,text},outputStyle:"stream-without-prefixes",onProgress:line=>writeOwnedCommandOutputV1(process.stderr,line+"\n",control)}));
 requireIssuedResourceControlInvocationV1(control,original);
}

/** 🌱️ Yields dependency publication to the existing external restart boundary after this Original and its phase retire. */
export async function executeImplementationV1(workspace:string,entry:string,args:readonly string[],original:ScriptInvocation,control:ResourceLeaseControl,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<void>{await withDependencyReadmissionV1(()=>executeImplementationOnceV1(workspace,entry,args,original,control,environment));}
