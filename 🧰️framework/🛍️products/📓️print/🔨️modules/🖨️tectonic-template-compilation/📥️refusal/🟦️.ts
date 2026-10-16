import{createHash}from"node:crypto";
import{writeFile}from"node:fs/promises";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{captureOwnedCommandResultV1,readOwnedCommandCaptureV1}from"./../../../../../🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🟦️.ts";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1,scriptInvocationEnvironment,type ScriptInvocation}from"./../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{withPrintScratchV1,withPrintExecutionV1,withPrintStorageTaskV1,type PrintExecutionV1}from"./../../../📦️artifacts/🎛️operation/🟦️.ts";
import{requireNativeStoragePathV1}from"./../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import{readNativeCallerAuthorityV1}from"./../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import{printCompilationIntentV1,type PrintProbeIntentV1}from"../📋️intent/🟦️.ts";
import{OwnedCommandFailure}from"./../../../../../🔨️modules/🏃️process/🎛️owned-execution/📪️outcome/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"./../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import{validateJsonSchemaSubset}from"./../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import{readPrintMaterialFileV1}from"./../../../📦️artifacts/📥️material/📤️producer/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
const policy=Object.freeze({...schema.definitions.Policy.const});
type Diagnostic=Readonly<{data:Readonly<{status:number;signal:null;log:string}>;cancellation:AbortSignal}>;
const diagnostics=new WeakMap<OwnedCommandFailure,Diagnostic>();
/** 📪️ Names bounded compiler-refusal DATA after the actual child Original has failed. */
export type PrintCompilerRefusalResultV1=Readonly<{version:1;kind:"compiler-refusal";requestSha256:string;status:number;signal:null;log:string}>;
/** 🔎️ Admits only a nonzero unsignalled compiler refusal and finite UTF-8 diagnostic DATA. */
export function readPrintCompilerRefusalResultV1(value:unknown):PrintCompilerRefusalResultV1{
 if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed Print compiler refusal DATA required");const row=value as PrintCompilerRefusalResultV1;if(Buffer.byteLength(row.log)>policy.maximumLogBytes||Buffer.byteLength(JSON.stringify(row))>policy.maximumResultBytes)throw Error("Finite Print compiler refusal DATA required");return Object.freeze({...row});
}
/** 🛑️ Captures finite diagnostics before cleanup and always rethrows the exact compiler primary. */
export async function capturePrintCompilerRefusalV1(primary:unknown,logPath:string,actualControl:ResourceLeaseControl):Promise<never>{
 try{if(!(primary instanceof OwnedCommandFailure))throw primary;const{status,signal}=primary.outcome;if(status===null||status===0||signal!==null)throw primary;const bytes=await readPrintMaterialFileV1(logPath,policy.maximumLogBytes,actualControl),log=new TextDecoder("utf-8",{fatal:true}).decode(bytes);if(log)diagnostics.set(primary,Object.freeze({data:Object.freeze({status,signal:null,log}),cancellation:actualControl.signal}));}finally{throw primary;}
}

/** ⏱️ Interprets finite terminal observations without issuing or renewing any operation control. */
export function printCompilerRefusalTerminalV1(value:unknown):boolean{if(validateJsonSchemaSubset(schema.definitions.TerminalState,value).length)throw Error("Closed Print terminal observation DATA required");const row=value as{originalCancelled:boolean;compilerCancelled:boolean;remainingMilliseconds:number|null};return !row.originalCancelled&&!row.compilerCancelled&&(row.remainingMilliseconds===null||row.remainingMilliseconds>0);}
/** 📥️ Validates a joined failed child report against its exact immutable parent request. */
export function readPrintCompilerRefusalCaptureV1(value:unknown,requestSha256:string):PrintCompilerRefusalResultV1{
 const capture=readOwnedCommandCaptureV1(value);if(!/^[a-f0-9]{64}$/u.test(requestSha256)||capture.status!==1||capture.signal!==null||Buffer.byteLength(capture.stdout)+Buffer.byteLength(capture.stderr)>policy.maximumResultBytes)throw Error("Joined failed Print inspector required");const lines=capture.stdout.split("\n");if(lines.length>policy.maximumResultLines)throw Error("Finite Print inspector terminal output required");const records=lines.filter(line=>line.startsWith(policy.prefix));if(records.length!==1)throw Error("One actual Print compiler refusal report required");const report=readPrintCompilerRefusalResultV1(JSON.parse(records[0]!.slice(policy.prefix.length)));if(report.requestSha256!==requestSha256)throw Error("Original Print compiler request digest required");return report;
}
function printRefusalIntentV1(value:unknown):PrintProbeIntentV1{const intent=printCompilationIntentV1(value);if(intent.kind!=="probe")throw Error("One finite Print probe refusal intent required");return intent;}
/** 🧪️ Runs a fixed first-party inspector child while retaining the parent's funded request and Original phase. */
export function inspectPrintCompilerRefusalV1(execution:PrintExecutionV1,value:unknown):Promise<PrintCompilerRefusalResultV1>{
 const intent=printRefusalIntentV1(value);return withPrintScratchV1(execution,"compiler-refusal",async scratch=>{const bytes=Buffer.from(JSON.stringify({version:1,intent}));if(bytes.length>policy.maximumRequestBytes)throw Error("Finite immutable Print refusal request required");const path=join(scratch.directory,"request.json"),requestSha256=createHash("sha256").update(bytes).digest("hex");checkResourceLeaseControl(scratch.control);await writeFile(path,bytes,{flag:"wx"});checkResourceLeaseControl(scratch.control);const transport=readNativeCallerAuthorityV1(scratch.invocation.capabilities).transport,result=await captureOwnedCommandResultV1(process.execPath,[fileURLToPath(new URL("./../../../📦️packages/🟦️typescript/📜️script.ts",import.meta.url)),"test","inspect-refusal",path,requestSha256],scratch.workspace,{control:scratch.control,env:scriptInvocationEnvironment(scratch.invocation,scratch.environment),output:{maximumBytes:Math.min(policy.maximumResultBytes,transport.maximumBytes),maximumLines:Math.min(policy.maximumResultLines,transport.maximumLines)}}),report=readPrintCompilerRefusalCaptureV1(result,requestSha256);if(createHash("sha256").update(await readPrintMaterialFileV1(path,policy.maximumRequestBytes,scratch.control)).digest("hex")!==requestSha256)throw Error("Immutable Print refusal request changed");return report;});
}
/** 🛑️ Receives the real parent Original, lets it fail, then emits only bounded terminal diagnostic DATA. */
export async function runPrintCompilerRefusalInspectorV1(environment:Readonly<Record<string,string|undefined>>,args:readonly string[]):Promise<void>{
 if(args.length!==2||!/^[a-f0-9]{64}$/u.test(args[1]??""))throw Error("Exact Print refusal request and digest required");const[path,requestSha256]=args as readonly[string,string];let failed=false,primary:unknown,observation:ScriptInvocation["control"]|undefined;
 try{await receiveScriptProcessInvocation(environment,invocation=>{observation=invocation.control;const authority=readNativeCallerAuthorityV1(invocation.capabilities),phaseControl=createIssuedResourceControlV1(invocation,"print:compiler-refusal-inspector",0);return withPrintExecutionV1({workspace:authority.repositoryRoot,invocation,phaseControl,kind:"probe"},execution=>withPrintStorageTaskV1(execution.operation,execution.phaseControl,async(operation,task)=>{const request=requireNativeStoragePathV1(task.store("diagnostics").root,path),bytes=await readPrintMaterialFileV1(request,policy.maximumRequestBytes,task.phaseControl);if(createHash("sha256").update(bytes).digest("hex")!==requestSha256)throw Error("Current owned Print refusal request required");const value:unknown=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes));if(validateJsonSchemaSubset(schema.definitions.Request,value).length)throw Error("Closed Print refusal request required");const intent=printRefusalIntentV1((value as{intent:unknown}).intent),{withCompiledPrintProbeV1}=await import("../🟦️.ts");await withCompiledPrintProbeV1({operation,phaseControl:task.phaseControl},intent,async()=>{throw Error("Actual Print probe unexpectedly compiled successfully");});}));});}catch(error){failed=true;primary=error;}
 if(!failed)throw Error("Actual failed Print inspector required");const diagnostic=primary instanceof OwnedCommandFailure?diagnostics.get(primary):undefined;if(!diagnostic||!observation||!printCompilerRefusalTerminalV1({version:1,originalCancelled:observation.signal.aborted,compilerCancelled:diagnostic.cancellation.aborted,remainingMilliseconds:observation.remainingMilliseconds()}))throw primary;const report=readPrintCompilerRefusalResultV1({version:1,kind:"compiler-refusal",requestSha256,...diagnostic.data}),payload=policy.prefix+JSON.stringify(report)+"\n";process.exitCode=1;await new Promise<void>((accept,reject)=>{process.stdout.write(payload,error=>error?reject(error):accept());});
}
