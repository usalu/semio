import assert from"node:assert/strict";
import{createHash}from"node:crypto";
import{basename,dirname,join,resolve}from"node:path";
import{fileURLToPath}from"node:url";
import{setTimeout as delay}from"node:timers/promises";
import{checkScriptInvocation,createIssuedResourceControlV1,readIssuedScriptProcessEnvelopeV1,scriptInvocationEnvironment,type ScriptInvocation}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{captureOwnedCommandResultV1}from"@semio-tech/process/execution";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import{repositoryCachePartitionRootV1,repositoryLeaseDirectoryV1,openRepositoryCachePartitionV1}from"../../../../../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import{repositoryCacheBudgetV1}from"../../../../../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import*as owner from"../../../../../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import*as leases from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import schema from"./🧬️schema.json";
import corpus from"./🔣️.json";
import physical from"../🔣️.json";
import{privateRepositoryWorkspaceV1}from"../../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1();
type Purpose=typeof corpus.cases[number]["purpose"];
const ackPrefix="[TRACE] Retention negative receiver ",failurePrefix="[TRACE] Retention negative result ";
function requirePurpose(value:string):Purpose{if(validateJsonSchemaSubset(schema.definitions.Purpose,value,schema).length)throw Error("Closed retention negative purpose required");return value as Purpose;}
function requireWorkspace(workspace:string):void{if(resolve(workspace)!==workspace||workspace.length>256||dirname(workspace)!==repositoryCachePartitionRootV1(repository.root,"products")||!/^tr-[A-Za-z0-9_-]+$/u.test(basename(workspace)))throw Error("Exact products-funded private retention workspace required");}
function acknowledgement(original:ScriptInvocation,workspace:string,purpose:Purpose){const envelope=readIssuedScriptProcessEnvelopeV1(original),wire=scriptInvocationEnvironment(original,process.env).SEMIO_SCRIPT_PROCESS_INVOCATION!;return{version:1,purpose,workspace,wireSha256:createHash("sha256").update(wire).digest("hex"),policy:envelope.policy,deadlineEpochMilliseconds:envelope.deadlineEpochMilliseconds,capabilities:envelope.capabilities};}
/** 🧪️ Receives one exact negative operation without converting its failure into accepted success. */
export async function runRetentionNegativePurposeV1(original:ScriptInvocation,workspace:string,value:string):Promise<never>{
 const purpose=requirePurpose(value);requireWorkspace(workspace);checkScriptInvocation(original);console.log(ackPrefix+JSON.stringify(acknowledgement(original,workspace,purpose)));let waited=0,entered=false;const abort=new AbortController(),control=createIssuedResourceControlV1(original,"retention:negative:"+purpose,0,purpose==="queued-cancel"?abort.signal:undefined),resource="native-fixed-image:"+repositoryCacheBudgetV1(workspace).partitions.tools.directory,body=async()=>{entered=true;throw Error("Negative maintenance unexpectedly entered");};
 try{
  if(purpose.startsWith("queued-invalid-")){const index=Number(purpose.slice(-1));await owner.withRepositoryCacheStoreQueuedMaintenanceV1(workspace,"tools",original,control,physical.queuedMaintenance.invalid[index] as never,body);}
  else if(purpose==="maintenance-held"||purpose==="queued-reader")await owner.withRepositoryCacheStoreV1(workspace,"tools",original,control,store=>purpose==="maintenance-held"?owner.withRepositoryCacheStoreMaintenanceV1(workspace,"tools",original,store.control,body):owner.withRepositoryCacheStoreQueuedMaintenanceV1(workspace,"tools",original,store.control,physical.queuedMaintenance.request as never,body));
  else if(purpose==="shared-held")await owner.withRepositoryCacheStoreMaintenanceV1(workspace,"tools",original,control,async store=>{const lease=await leases.acquireResourceLease({directory:repositoryLeaseDirectoryV1(workspace),resource,mode:"shared",control:store.control,signal:store.control.signal,timeoutMs:0});lease.release();throw Error("Negative shared lease unexpectedly entered");});
  else{
   const reader=await openRepositoryCachePartitionV1(workspace,"tools",control);let observer:{mockRestore():void}|undefined,primary:{error:unknown}|undefined;
   try{const{spyOn}=await import("bun:test"),actual=leases.acquireQueuedResourceLease;observer=spyOn(leases,"acquireQueuedResourceLease").mockImplementation(options=>actual(options.resource===resource&&options.mode==="exclusive"?{...options,onWait:async event=>{await options.onWait?.(event);waited++;if(purpose==="queued-cancel")abort.abort(Object.freeze(Error("Owned queued maintenance cancellation")));else await delay(physical.queuedMaintenance.deadlineRequest.maximumWaitMilliseconds,undefined,{signal:original.control.signal});}}:options));await owner.withRepositoryCacheStoreQueuedMaintenanceV1(workspace,"tools",original,control,(purpose==="queued-cancel"?physical.queuedMaintenance.request:physical.queuedMaintenance.deadlineRequest) as never,body);}catch(error){primary={error};throw error;}finally{const errors:unknown[]=[];try{observer?.mockRestore();}catch(error){errors.push(error);}try{await reader.close();}catch(error){errors.push(error);}if(errors.length&&!primary)throw errors[0];}

  }
  throw Error("Negative retention operation unexpectedly succeeded");
 }catch(error){const reason=error instanceof Error?error.message:String(error);console.log(failurePrefix+JSON.stringify({version:1,purpose,reason,waited,entered}));throw error;}
}
/** 📥️ Accepts a resolved child status while retaining the genuine parent operation ledger. */
export async function observeRetentionNegativePurposeV1(original:ScriptInvocation,workspace:string,value:string):Promise<void>{
 const purpose=requirePurpose(value),row=corpus.cases.find(row=>row.purpose===purpose)!;requireWorkspace(workspace);const result=await captureOwnedCommandResultV1(process.execPath,[fileURLToPath(new URL("../📜️script.ts",import.meta.url)),"negative",workspace,purpose],process.cwd(),{control:createIssuedResourceControlV1(original,"retention:negative-result",0),env:scriptInvocationEnvironment(original,process.env),output:{maximumBytes:1048576,maximumLines:4096}}),lines=result.stdout.split(/\r?\n/u),acks=lines.filter(line=>line.startsWith(ackPrefix)),failures=lines.filter(line=>line.startsWith(failurePrefix));assert.equal(result.signal,null);assert.equal(result.status,1);assert.equal(acks.length,1);const ack=JSON.parse(acks[0]!.slice(ackPrefix.length));assert.equal(validateJsonSchemaSubset(schema.definitions.Ack,ack,schema).length,0);assert.deepEqual(ack,acknowledgement(original,workspace,purpose));assert.equal(failures.length,1);const failure=JSON.parse(failures[0]!.slice(failurePrefix.length));assert.equal(validateJsonSchemaSubset(schema.definitions.Failure,failure,schema).length,0);assert.equal(failure.purpose,purpose);assert.match(failure.reason,new RegExp(row.reason,"iu"));assert.equal(failure.entered,false);assert.equal(failure.waited>0,row.waited);checkScriptInvocation(original);
}
