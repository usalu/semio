import{availableParallelism}from"node:os";
import{isAbsolute,relative,resolve,sep}from"node:path";
import{lstatSync}from"node:fs";
import{types}from"node:util";
import{captureOwnedCommandResultV1,type OwnedCommandCaptureV1}from"../../../🎛️owned-execution/🟦️.ts";
import{requireIssuedResourceControlInvocationV1,withIssuedResourceOperationV1,withRequestedIssuedResourceControlV1,scriptInvocationEnvironment}from"../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type{ScriptInvocation}from"../../../🧭️routing/📥️invocation/🟦️.ts";
import type{ResourceLeaseControl}from"../../../🔒️leases/🟦️.ts";
import{checkResourceLeaseControl}from"../../../🔒️leases/🟦️.ts";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import schema from"./🧬️schema/🔣️.json" with { type: "json" };
const contract=structuredClone(schema);

/** 🧮️ Bounds actual file concurrency and both per-child and complete-cohort output. */
export interface OwnedBunFilesPolicyV1{readonly version:1;readonly maximumParallelFiles:number;readonly maximumFileOutputBytes:number;readonly maximumFileOutputLines:number;readonly maximumTotalOutputBytes:number}
/** 📋️ Selects explicit test files beneath one caller's working directory and live Original phase. */
export interface OwnedBunFilesRequestV1{readonly invocation:ScriptInvocation;readonly control:ResourceLeaseControl;readonly cwd:string;readonly files:readonly string[];readonly arguments:readonly string[];readonly environment:Readonly<Record<string,string|undefined>>;readonly policy:OwnedBunFilesPolicyV1;readonly onFileResult:(result:OwnedBunFileResultV1,index:number,control:ResourceLeaseControl)=>Promise<void>}
/** 📬️ Returns terminal child DATA in authored file order without escaping a process owner. */
export interface OwnedBunFileResultV1{readonly file:string;readonly capture:OwnedCommandCaptureV1}

/** 🔐️ Captures closed primitive policy fields before any deferred child work. */
export function readOwnedBunFilesPolicyV1(value:unknown):OwnedBunFilesPolicyV1{
 if(!value||typeof value!=="object"||types.isProxy(value)||![Object.prototype,null].includes(Object.getPrototypeOf(value)))throw Error("Plain owned Bun policy required");const row:Record<string,unknown>=Object.create(null);
 const keys=Reflect.ownKeys(value);if(keys.length>5)throw Error("Finite Bun policy fields required");for(const key of keys){if(typeof key!=="string")throw Error("String Bun policy fields required");const field=Object.getOwnPropertyDescriptor(value,key);if(!field||!("value"in field)||!field.enumerable)throw Error("Primitive Bun policy fields required");row[key]=field.value;}
 if(validateJsonSchemaSubset(contract.definitions.Policy,row,contract).length)throw Error("Complete finite owned Bun policy required");const policy=row as unknown as OwnedBunFilesPolicyV1;if(policy.maximumFileOutputBytes>policy.maximumTotalOutputBytes||policy.maximumParallelFiles*policy.maximumFileOutputBytes>contract.definitions.Protocol.const.maximumBufferedOutputBytes)throw Error("Coherent bounded Bun output policy required");return Object.freeze(policy);
}

/** 🧪️ Gives each file its own genuine received process birth while retaining the caller through every child drain. */
export function runOwnedBunFilesV1(options:OwnedBunFilesRequestV1):Promise<readonly OwnedBunFileResultV1[]>{
 try{
  const invocation=options.invocation,control=options.control;requireIssuedResourceControlInvocationV1(control,invocation);
  const onFileResult=options.onFileResult;if(typeof onFileResult!=="function")throw Error("Actual owned Bun terminal result sink required");
  const policy=readOwnedBunFilesPolicyV1(options.policy),selection=Object.freeze({version:1,cwd:options.cwd,files:Object.freeze([...options.files]),arguments:Object.freeze([...options.arguments])}),environment=Object.freeze({...options.environment});
  if(validateJsonSchemaSubset(contract.definitions.Selection,selection,contract).length||!process.versions.bun||!isAbsolute(selection.cwd)||[selection.cwd,...selection.files].some(path=>path.length>contract.definitions.Protocol.const.maximumPathCodeUnits||path.includes("\0")))throw Error("Explicit finite owned Bun file selection required");
  const cwd=resolve(selection.cwd),files=selection.files.map(file=>{const local=relative(cwd,file);if(!isAbsolute(file)||local===".."||local.startsWith(".."+sep)||isAbsolute(local))throw Error("Bun file must belong to the actual working directory");return resolve(file);});
  if(new Set(files).size!==files.length)throw Error("Unique actual Bun files required");
  return withIssuedResourceOperationV1(control,async()=>{
   for(const file of files){checkResourceLeaseControl(control);const stat=lstatSync(file);if(!stat.isFile()||stat.isSymbolicLink()||stat.size>contract.definitions.Protocol.const.maximumSourceBytes)throw Error("Bounded regular Bun source file required");}
   const cancellation=new AbortController();return withRequestedIssuedResourceControlV1(control,0,async phase=>{
    const results=new Array<OwnedBunFileResultV1>(files.length),started=performance.now();let next=0,total=0,failed=false,failure:unknown;
    const worker=async()=>{while(!failed&&next<files.length){const index=next++,file=files[index];try{if(file===undefined)throw Error("Exact admitted Bun file required");
     checkResourceLeaseControl(phase);const capture=await captureOwnedCommandResultV1(process.execPath,["test",file,...selection.arguments],cwd,{control:phase,env:scriptInvocationEnvironment(invocation,environment),output:{maximumBytes:policy.maximumFileOutputBytes,maximumLines:policy.maximumFileOutputLines}});
     total+=Buffer.byteLength(capture.stdout)+Buffer.byteLength(capture.stderr);if(total>policy.maximumTotalOutputBytes)throw Error("Owned Bun cohort output limit exceeded");const result=Object.freeze({file,capture});results[index]=result;
     await withRequestedIssuedResourceControlV1(phase,0,actual=>onFileResult(result,index,actual));
     await phase.publish({resource:"bun:test:file:"+index,mode:"shared",elapsedMs:performance.now()-started});await phase.yieldContinuation();checkResourceLeaseControl(phase);
    }catch(error){if(!failed){failed=true;failure=error;cancellation.abort(error);}}}};
    await Promise.all(Array.from({length:Math.min(files.length,availableParallelism(),policy.maximumParallelFiles)},worker));if(failed)throw failure;return Object.freeze(results);
   },cancellation.signal);
  });
 }catch(error){const failure=Promise.reject(error);void failure.catch(()=>{});return failure;}
}
