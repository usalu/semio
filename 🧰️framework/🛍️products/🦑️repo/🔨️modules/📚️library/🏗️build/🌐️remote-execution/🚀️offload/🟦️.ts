import {appendFileSync,mkdirSync,rmSync} from "node:fs";
import {join,resolve} from "node:path";
import {fleetCacheRootV1,remoteCacheFetchV1,resolveRemoteCacheV1} from "../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {fleetHeadersV1} from "../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {httpRemoteExecutionContentV1,type RemoteExecutionContentV1} from "../🗄️content/🟦️.ts";
import {remoteExecutionHostPropertiesV1} from "../🟦️.ts";
import {buildOffloadActionV1,offloadPlatformV1,parseRustcIdentityV1,type OffloadToolchainV1} from "./📜️action/🟦️.ts";
import {executeOffloadRidingThroughV1,readOffloadOutputsV1,runBoundedOffloadPhaseV1,writeOffloadOutputsV1,type OffloadStageV1} from "./🔌️client/🟦️.ts";
import {classifyOffloadExitV1} from "./🩺️failure/🟦️.ts";
import {resolveRemoteExecutionEndpointV1} from "../🧭️endpoint/🟦️.ts";
import {offloadRealTextV1,planOffloadV1} from "./🗺️paths/🟦️.ts";
import {OFFLOAD_POLICY_V1,offloadTextsV1,type OffloadReasonV1} from "./📐️policy/🟦️.ts";
import {remoteExecutionOriginV1} from "../🌡️load/🟦️.ts";
import {bumpOffloadCountersV1} from "./📊️counters/🟦️.ts";
import {readOffloadMemoV1,writeOffloadMemoV1} from "./🚦️admission/🟦️.ts";
import {releaseImplicitTokenV1} from "./🎟️token/🟦️.ts";
import {rustcVersionTextV1} from "./🧰️toolchain/🟦️.ts";

export {offloadModeV1,OFFLOAD_POLICY_V1} from "./📐️policy/🟦️.ts";
export {localUnitAdmissionV1,sharedUnitAdmissionV1,slotQueueMillisecondsV1,worthARoundTripV1,readUnitCostV1,writeUnitCostV1,offloadFanOutJobsV1} from "./🚦️admission/🟦️.ts";
export {detectWorkerRustcV1} from "./🧰️toolchain/🟦️.ts";

const policy=OFFLOAD_POLICY_V1;
type Environment=Readonly<Record<string,string|undefined>>;
/** 🏁️ How an offload attempt ended: the compiler's answer, or the reason the unit compiles here. */
export type OffloadOutcomeV1=Readonly<{kind:"done";status:number;stdout:string;stderr:string;origin:string;milliseconds:number;cached:boolean}>|Readonly<{kind:"local";reason:OffloadReasonV1;detail:string}>;
/** 🔌️ Seams for tests: the content store and the endpoint a test cluster runs on, and a shorter upload time limit. */
export type OffloadPortsV1=Readonly<{content?:RemoteExecutionContentV1;endpoint?:string;token?:string;instance?:string;cacheEndpoint?:string;uploadMilliseconds?:number}>;

/** 📒️ Appends one decision to the machine's offload log (`x/offload.jsonl` of the unit cache) and its human progress lines (`x/offload.progress`, English and German). */
export function emitOffloadEventV1(root:string,event:Readonly<{lane?:string;kind:"offload-run"|"offload-declined"|"offload-failed"|"offload-local-fallback";crate:string;key:string|null;origin?:string;reason?:OffloadReasonV1;detail?:string;milliseconds:number;waitedMs?:number;bytesShipped?:number;bytesUploaded?:number;bytesReturned?:number}>):void{
 try{
  const directory=join(root,"x");mkdirSync(directory,{recursive:true});
  appendFileSync(join(directory,"offload.jsonl"),JSON.stringify({version:1,atMs:Date.now(),pid:process.pid,waitedMs:0,...event})+"\n");
  const text=event.kind==="offload-run"?offloadTextsV1("offloaded",{crate:event.crate,origin:event.origin??"",milliseconds:Math.round(event.milliseconds)}):event.kind==="offload-declined"?offloadTextsV1("declined",{crate:event.crate,reason:offloadTextsV1("reason."+event.reason).split("\n")[0]!}):offloadTextsV1("fallback",{crate:event.crate,reason:offloadTextsV1("reason."+event.reason).split("\n")[0]!});
  appendFileSync(join(directory,"offload.progress"),text+"\n");
 }catch{}
}

/** 📣️ Appends the stage an offloaded unit reached (waiting for a machine, compiling there) to the machine's human progress lines (`x/offload.progress`, English and German). */
export function emitOffloadStageV1(root:string,crate:string,stage:OffloadStageV1):void{
 if(stage!=="QUEUED"&&stage!=="EXECUTING")return;
 try{const directory=join(root,"x");mkdirSync(directory,{recursive:true});appendFileSync(join(directory,"offload.progress"),offloadTextsV1(stage==="QUEUED"?"stage.queued":"stage.executing",{crate})+"\n");}catch{}
}

/** 🚀️ Hands one rustc unit to another machine. Local first is the caller's rule (it calls this only when the machine would wait or the mode is `always`); this builds the hermetic action, releases the job token while it waits, and answers either the compiler's own result translated into the requester's paths (exit status, streams byte for byte, output files written into the real output directory) or the reason the unit compiles here. Never throws: any fault is a reason. The result is NOT stored in the caches here: the caller publishes it like a local compilation, under the unit key it computed, so the entry exists exactly once. */
export async function offloadRustcUnitV1(input:Readonly<{args:readonly string[];environment:Environment;cwd:string;root:string;lane?:string;crate:string;key:string;compiler:readonly string[];digest?:(path:string)=>string|null;signal?:AbortSignal;onStage?:(stage:OffloadStageV1)=>void;ports?:OffloadPortsV1}>):Promise<OffloadOutcomeV1>{
 const marks:number[]=[],bytes:{shipped?:number;uploaded?:number;returned?:number}={},started=Date.now(),local=(reason:OffloadReasonV1,detail=""):OffloadOutcomeV1=>({kind:"local",reason,detail}),{environment,cwd,ports}=input;
 const settle=(outcome:OffloadOutcomeV1,origin?:string):OffloadOutcomeV1=>{
  if(outcome.kind==="local"&&outcome.reason!=="cancelled")emitOffloadEventV1(input.root,{...input.lane?{lane:input.lane}:{},kind:outcome.reason==="no-idle-machine"||outcome.reason==="queue-timeout"||outcome.reason==="not-portable"||outcome.reason==="unmapped-path"||outcome.reason==="inputs-too-large"||outcome.reason==="no-endpoint"||outcome.reason==="no-credential"?"offload-declined":"offload-local-fallback",crate:input.crate,key:input.key,reason:outcome.reason,detail:outcome.detail.slice(0,200),milliseconds:Date.now()-started});
  const workspace_=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||fleetCacheRootV1(environment);
  if(workspace_!==undefined)bumpOffloadCountersV1(workspace_,outcome.kind==="done"?{out:1}:outcome.reason==="no-idle-machine"||outcome.reason==="queue-timeout"?{refused:1}:outcome.reason==="infrastructure"||outcome.reason==="incomplete-inputs"||outcome.reason==="path-dependent-output"||outcome.reason==="worker-policy"?{fellBack:1}:{},outcome.kind==="local"&&outcome.reason!=="cancelled"?outcome.reason:undefined);
  if(outcome.kind==="done")emitOffloadEventV1(input.root,{...input.lane?{lane:input.lane}:{},kind:"offload-run",crate:input.crate,key:input.key,origin:origin??outcome.origin,milliseconds:outcome.milliseconds,...bytes.shipped!==undefined?{bytesShipped:bytes.shipped,bytesUploaded:bytes.uploaded??0,bytesReturned:bytes.returned??0}:{},...marks.length>=3?{detail:`plan ${marks[0]!-started} ms, upload ${marks[1]!-marks[0]!} ms, queue and execute ${marks[2]!-marks[1]!} ms, download and write ${Date.now()-marks[2]!} ms`}:{}});
  return outcome;
 };
 try{
  const text=rustcVersionTextV1(input.root,input.compiler,environment),toolchainParsed=text===undefined?undefined:parseRustcIdentityV1(text);
  if(!toolchainParsed)return settle(local("toolchain-missing"));
  const workspace=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||fleetCacheRootV1(environment);
  const endpoint=ports?.endpoint??resolveRemoteExecutionEndpointV1({environment,...workspace===undefined?{}:{root:workspace}})?.endpoint;
  if(!endpoint)return settle(local("no-endpoint"));
  const cache=ports?.content&&ports.token!==undefined?undefined:resolveRemoteCacheV1(environment,workspace===undefined?{}:{root:workspace});
  const token=ports?.token??cache?.token,cacheEndpoint=ports?.cacheEndpoint??cache?.contract.endpoint,instance=ports?.instance??cache?.contract.instance;
  if(!ports?.content&&(!cacheEndpoint||!instance))return settle(local("no-credential"));
  const declined=readOffloadMemoV1<{atMs:number;reason?:OffloadReasonV1;detail?:string}>(input.root,"declined",Date.now(),policy.limits.declineMemoMilliseconds);
  if(declined)return settle(local(declined.reason??"no-idle-machine",`memo${declined.detail?` ${declined.detail}`:""}`));
  if(!ports?.content&&readOffloadMemoV1<{atMs:number;endpoint:string}>(input.root,"reachable",Date.now(),policy.limits.reachableMemoMilliseconds)?.endpoint!==cacheEndpoint){
   const reachable=await fetch(cacheEndpoint!.replace(/\/+$/u,"")+"/healthz",{signal:AbortSignal.timeout(policy.timeouts.probeMilliseconds)}).then(async response=>{await response.body?.cancel();return response.status===200;},()=>false);
   if(!reachable){writeOffloadMemoV1(input.root,"declined",{atMs:Date.now(),reason:"infrastructure",detail:"cache endpoint unreachable"});return settle(local("infrastructure","cache endpoint unreachable"));}
   writeOffloadMemoV1(input.root,"reachable",{atMs:Date.now(),endpoint:cacheEndpoint!});
  }
  const plan=planOffloadV1({args:input.args,environment,cwd});
  if("declined" in plan)return settle(local(plan.declined,plan.detail));
  const content=ports?.content??httpRemoteExecutionContentV1({endpoint:cacheEndpoint!,token,...(workspace===undefined||cache===undefined?{}:{fetch:remoteCacheFetchV1({...environment,...cache.environment,SEMIO_WORKSPACE_ROOT:workspace},{root:workspace})}),timeoutSeconds:Math.ceil(policy.timeouts.requestMilliseconds/1000),retries:0}),toolchain:OffloadToolchainV1={...toolchainParsed,targets:[]};
  const platform=offloadPlatformV1(remoteExecutionHostPropertiesV1(),toolchain,plan.target,(workspace===undefined?undefined:remoteExecutionOriginV1(workspace,environment))),cancelled=input.signal??new AbortController().signal;
  marks.push(Date.now());
  const phase=await runBoundedOffloadPhaseV1(signal=>buildOffloadActionV1({plan,content,instance:instance!,platform,signal,...input.digest?{digest:input.digest}:{}}),{cancelled,milliseconds:ports?.uploadMilliseconds??policy.timeouts.uploadMilliseconds});
  if(phase.kind==="timeout")return settle(local("infrastructure","upload"));
  if(phase.kind==="cancelled")return local("cancelled");
  const built=phase.value;
  if("declined" in built)return settle(local(built.declined,built.detail));
  marks.push(Date.now());bytes.shipped=built.bytes;bytes.uploaded=built.uploaded;
  const lease=releaseImplicitTokenV1(environment);
  let execution,seen:OffloadStageV1|undefined;
  const staged=(stage:OffloadStageV1):void=>{if(stage===seen)return;seen=stage;emitOffloadStageV1(input.root,input.crate,stage);try{input.onStage?.(stage);}catch{}};
  try{execution=await executeOffloadRidingThroughV1({endpoint,token,...(workspace===undefined||token!==undefined?{}:{sign:(method,content)=>fleetHeadersV1(workspace,"submit","execution","POST",`http://x/${method}`,content)}),instance:instance!,action:built.digest,signal:cancelled,onStage:staged},policy.timeouts.rideThroughMilliseconds);}
  finally{await lease?.reacquire();lease?.close();}
  if(execution.kind==="declined"){
   if(execution.reason==="no-idle-machine"||execution.reason==="queue-timeout"||execution.reason==="worker-policy")writeOffloadMemoV1(input.root,"declined",{atMs:Date.now(),reason:execution.reason,detail:execution.detail.slice(0,120)});
   return settle(local(execution.reason,execution.detail));
  }
  const downloading=AbortSignal.any([cancelled,AbortSignal.timeout(policy.timeouts.downloadMilliseconds)]);
  marks.push(Date.now());
  const outputs=await readOffloadOutputsV1(content,instance!,execution.actionResult,policy.virtual.out,[policy.virtual.root],downloading);
  bytes.returned=[...outputs.directory.values()].reduce((sum,file)=>sum+file.digest.size_bytes,0)+outputs.stdout.length+outputs.stderr.length;
  const root_=Buffer.from(outputs.files.get(policy.virtual.root)??new Uint8Array(0)).toString("utf8")||undefined,translate=(bytes:Buffer):string=>offloadRealTextV1(bytes.toString("utf8"),plan.mappings,root_,cwd);
  const stdout=translate(outputs.stdout),stderr=translate(outputs.stderr);
  if(outputs.exitCode!==0){
   const kind=classifyOffloadExitV1({exitCode:outputs.exitCode,stderr});
   if(kind==="incomplete-inputs")return settle(local("incomplete-inputs",stderr.split("\n")[0]!.slice(0,120)));
   if(kind==="infrastructure")return settle(local("infrastructure",`exit ${outputs.exitCode}`));
   return settle({kind:"done",status:outputs.exitCode,stdout,stderr,origin:execution.origin,milliseconds:Date.now()-started,cached:execution.cached});
  }
  const outDir=resolve(cwd,plan.outDir),marker=root_===undefined?undefined:Buffer.from(root_);
  let written:readonly string[]=[];
  try{
   written=await writeOffloadOutputsV1(content,instance!,outputs,outDir,(name,bytes)=>{
    if(name.endsWith(".d"))return Buffer.from(translate(bytes));
    if(marker!==undefined&&bytes.includes(marker))throw Object.assign(Error("path-dependent"),{offload:"path-dependent-output",file:name});
    return bytes;
   },downloading);
  }catch(error){
   for(const path of written)rmSync(path,{force:true});
   if((error as {offload?:string}).offload==="path-dependent-output"){const file=(error as {file?:string}).file??"";return settle(local("path-dependent-output",`${file.slice(-60)}: ${root_??""}`));}
   return settle(local("infrastructure",String((error as Error)?.message??error).slice(0,120)));
  }
  return settle({kind:"done",status:0,stdout,stderr,origin:execution.origin,milliseconds:Date.now()-started,cached:execution.cached});
 }catch(error){
  if(input.signal?.aborted)return local("cancelled");
  return settle(local("infrastructure",String((error as Error)?.message??error).slice(0,120)));
 }
}
