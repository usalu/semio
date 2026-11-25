import {appendFileSync,mkdirSync,readdirSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {randomBytes} from "node:crypto";
import {dirname,join,resolve} from "node:path";
import {fleetCacheRootV1,remoteCacheFetchV1,resolveRemoteCacheV1} from "../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {fleetHeadersV1} from "../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {httpRemoteExecutionContentV1,outageTolerantRemoteExecutionContentV1,type RemoteExecutionContentV1} from "../🗄️content/🟦️.ts";
import {readRemoteExecutionPolicyV1,remoteExecutionHostPropertiesV1,remoteExecutionMatchPropertiesV1,type RemoteExecutionPropertyV1} from "../🟦️.ts";
import {buildOffloadActionV1,offloadPlatformV1,shipOffloadActionV1,parseRustcIdentityV1,type OffloadToolchainV1} from "./📜️action/🟦️.ts";
import {executeOffloadV1,offloadExecuteBudgetMillisecondsV1,readOffloadOutputsV1,runBoundedOffloadPhaseV1,writeOffloadOutputsV1,type OffloadOutputsV1,type OffloadStageV1} from "./🔌️client/🟦️.ts";
import {classifyOffloadExitV1,offloadIncompleteDetailV1} from "./🩺️failure/🟦️.ts";
import {resolveRemoteExecutionEndpointV1} from "../🧭️endpoint/🟦️.ts";
import {offloadRealTextV1,planOffloadV1} from "./🗺️paths/🟦️.ts";
import {OFFLOAD_POLICY_V1,offloadActionSecondsV1,offloadDeadlineMillisecondsV1,offloadCrateTypesOfV1,offloadLinksNativelyV1,offloadKilledPeakMegabytesV1,offloadPeakMegabytesV1,offloadPinV1,offloadTextV1,offloadTextsV1,type OffloadReasonV1} from "./📐️policy/🟦️.ts";
import {remoteExecutionOriginV1} from "../🌡️load/🟦️.ts";
import {bumpOffloadCountersV1} from "./📊️counters/🟦️.ts";
import {awaitBeyondHostV1} from "./🚛️beyond-host/🟦️.ts";
import {eligibleFleetWorkersV1,fleetWorkersV1,offloadDeclaredThreadsV1,readOffloadMemoV1,readUnitCostV1,readUnitExpenseV1,readUnitPeakV1,writeOffloadMemoV1,writeOffloadOverheadV1,writeUnitPeakV1} from "./🚦️admission/🟦️.ts";
import type {FleetWorkerV1} from "./🚦️admission/🟦️.ts";
import {releaseImplicitTokenV1} from "./🎟️token/🟦️.ts";
import {offloadLinkerIdentityV1,offloadSdkIdentityV1,offloadSysrootV1,rustcVersionTextV1} from "./🧰️toolchain/🟦️.ts";
import {COMMAND_POLICY_V1,commandProgramV1,commandTextsV1,commandToolPeakMegabytesV1,commandToolPropertyV1,commandToolSourceV1} from "./🧰️command/📐️policy/🟦️.ts";

export {offloadModeV1,offloadPinV1,offloadSettingV1,offloadTextsV1,OFFLOAD_POLICY_V1} from "./📐️policy/🟦️.ts";
export {localUnitAdmissionV1,sharedUnitAdmissionV1,slotQueueMillisecondsV1,worthARoundTripV1,readOffloadOverheadV1,writeOffloadOverheadV1,readUnitCostV1,writeUnitCostV1,offloadDeclaredThreadsV1,readUnitPeakV1,readUnitPeaksV1,writeUnitPeakV1,readFamilyPeakV1,readFamilyCostV1,readUnitExpenseV1,awaitFleetFitV1,offloadFanOutJobsV1,offloadGateV1,fleetQueueV1} from "./🚦️admission/🟦️.ts";
export {detectWorkerRustcV1} from "./🧰️toolchain/🟦️.ts";
export {COMMAND_POLICY_V1,commandProgramV1,commandToolKeyV1,commandToolPropertyV1} from "./🧰️command/📐️policy/🟦️.ts";

const policy=OFFLOAD_POLICY_V1;
type Environment=Readonly<Record<string,string|undefined>>;
/** 🏁️ How an offload attempt ended: the compiler's answer, or the reason the unit compiles here. */
export type OffloadOutcomeV1=Readonly<{kind:"done";status:number;stdout:string;stderr:string;origin:string;milliseconds:number;cached:boolean}>|Readonly<{kind:"local";reason:OffloadReasonV1;detail:string}>;
/** 🔌️ Seams for tests: the content store and the endpoint a test cluster runs on, and a shorter upload time limit. */
export type OffloadPortsV1=Readonly<{content?:RemoteExecutionContentV1;endpoint?:string;token?:string;instance?:string;cacheEndpoint?:string;uploadMilliseconds?:number;workers?:Readonly<{own:string;workers:readonly FleetWorkerV1[]}>}>;

/** 📒️ Appends one decision to the machine's offload log (`x/offload.jsonl` of the unit cache) and its human progress lines (`x/offload.progress`, English and German). */
export function emitOffloadEventV1(root:string,event:Readonly<{lane?:string;kind:"offload-run"|"offload-declined"|"offload-failed"|"offload-local-fallback"|"offload-reuploaded";crate:string;key:string|null;origin?:string;reason?:OffloadReasonV1;detail?:string;milliseconds:number;waitedMs?:number;bytesShipped?:number;bytesUploaded?:number;bytesReturned?:number;progress?:string}>):void{
 try{
  const directory=join(root,"x"),{progress,...recorded}=event;mkdirSync(directory,{recursive:true});
  appendFileSync(join(directory,"offload.jsonl"),JSON.stringify({version:1,atMs:Date.now(),pid:process.pid,waitedMs:0,...recorded})+"\n");
  const both=(key:string):string=>(["en","de"] as const).map(language=>offloadTextV1(language,key,{crate:event.crate,reason:offloadTextV1(language,"reason."+event.reason)})).join("\n");
  const text=progress!==undefined?progress:event.kind==="offload-run"?offloadTextsV1("offloaded",{crate:event.crate,origin:event.origin??"",milliseconds:Math.round(event.milliseconds)}):both(event.kind==="offload-declined"?"declined":"fallback");
  appendFileSync(join(directory,"offload.progress"),text+"\n");
 }catch{}
}

/** 📣️ Appends the stage an offloaded unit reached (waiting for a machine, compiling there) to the machine's human progress lines (`x/offload.progress`, English and German). */
export function emitOffloadStageV1(root:string,crate:string,stage:OffloadStageV1):void{
 if(stage!=="QUEUED"&&stage!=="EXECUTING")return;
 try{const directory=join(root,"x");mkdirSync(directory,{recursive:true});appendFileSync(join(directory,"offload.progress"),offloadTextsV1(stage==="QUEUED"?"stage.queued":"stage.executing",{crate})+"\n");}catch{}
}

/** 📣️ Appends one human progress line (already in both languages) to the machine's offload progress file. */
export function emitOffloadProgressV1(root:string,text:string):void{
 try{const directory=join(root,"x");mkdirSync(directory,{recursive:true});appendFileSync(join(directory,"offload.progress"),text+"\n");}catch{}
}

/** 🎟️ Gives the implicit job token back to the pool of the build while a unit runs on a worker, unless the build's pool is already sized for the fleet (`SEMIO_UNIT_CACHE_LIGHT_SLOTS` is set: its tokens count the units that are away, the light slots bound them, and taking a token back at the end would wait behind the units that started in the meantime for as long as `tokenWaitMilliseconds`). */
function tokenLeaseV1(environment:Environment):ReturnType<typeof releaseImplicitTokenV1>{
 return Number(environment.SEMIO_UNIT_CACHE_LIGHT_SLOTS??0)>0?undefined:releaseImplicitTokenV1(environment);
}

/** 🔁️ Records that blobs the store had lost between the existence check and the fetch were sent again and the same action ran once more (event `offload-reuploaded`, with the English and German progress line): the round, how many blobs the failure named, how many were sent and their bytes. */
function emitOffloadReuploadV1(root:string,lane:string|undefined,crate:string,key:string|null,started:number,row:Readonly<{round:number;named:number;blobs:number;bytes:number}>):void{
 emitOffloadEventV1(root,{...lane?{lane}:{},kind:"offload-reuploaded",crate,key,detail:`round ${row.round}: ${row.named} blobs named missing, ${row.blobs} sent again, ${row.bytes} bytes`,milliseconds:Date.now()-started,bytesUploaded:row.bytes,progress:offloadTextsV1("reuploaded",{crate,round:row.round,blobs:row.blobs,bytes:row.bytes})});
}

/** 🧾️ The bookkeeping of one offload attempt, the same for a rustc unit and a command: the clock, the time marks and byte counts of its phases, the local answer with its reason and the settling of an outcome (event line, progress line, machine counters). `describe` renders the progress line of a kind in both languages; without it the line is the rustc unit's. */
function offloadRecorderV1(input:Readonly<{root:string;lane?:string;crate:string;key:string|null;environment:Environment;describe?:(kind:"offloaded"|"declined"|"fallback",variables:Readonly<Record<string,string|number>>)=>string}>){
 const marks:number[]=[],bytes:{shipped?:number;uploaded?:number;returned?:number;worker?:string}={},started=Date.now(),{environment}=input,pin=offloadPinV1(environment),local=(reason:OffloadReasonV1,detail=""):OffloadOutcomeV1=>({kind:"local",reason,detail});
 const pinned=new Set<OffloadReasonV1>(["no-idle-machine","no-compatible-worker","queue-timeout","infrastructure","incomplete-inputs","remote-failed-local-redo","worker-policy","path-dependent-output","unreadable-input","inputs-too-large","no-endpoint","no-credential","toolchain-missing"]);
 const line=(kind:"offloaded"|"declined"|"fallback",variables:Readonly<Record<string,string|number>>):string|undefined=>input.describe?.(kind,variables);
 const settle=(given:OffloadOutcomeV1,origin?:string):OffloadOutcomeV1=>{
  const outcome:OffloadOutcomeV1=pin!==undefined&&given.kind==="local"&&pinned.has(given.reason)?{kind:"local",reason:"pin-failed",detail:`${pin}: ${given.reason}${given.detail===""?"":` ${given.detail}`}`}:given;
  if(outcome.kind==="local"&&outcome.reason!=="cancelled"&&!(outcome.reason==="no-endpoint"&&pin===undefined)){
   const declined=outcome.reason==="no-idle-machine"||outcome.reason==="no-compatible-worker"||outcome.reason==="queue-timeout"||outcome.reason==="not-portable"||outcome.reason==="unmapped-path"||outcome.reason==="inputs-too-large"||outcome.reason==="no-endpoint"||outcome.reason==="no-credential",reason=offloadTextsV1("reason."+outcome.reason).split("\n")[0]!,progress=line(declined?"declined":"fallback",{crate:input.crate,reason});
   emitOffloadEventV1(input.root,{...input.lane?{lane:input.lane}:{},kind:declined?"offload-declined":"offload-local-fallback",crate:input.crate,key:input.key,reason:outcome.reason,detail:outcome.detail.slice(0,200),milliseconds:Date.now()-started,...progress===undefined?{}:{progress}});
  }
  const workspace_=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||fleetCacheRootV1(environment);
  if(workspace_!==undefined)bumpOffloadCountersV1(workspace_,outcome.kind==="done"?{out:1}:outcome.reason==="no-idle-machine"||outcome.reason==="no-compatible-worker"||outcome.reason==="queue-timeout"?{refused:1}:outcome.reason==="infrastructure"||outcome.reason==="incomplete-inputs"||outcome.reason==="path-dependent-output"||outcome.reason==="worker-policy"||outcome.reason==="pin-failed"?{fellBack:1}:{},outcome.kind==="local"&&outcome.reason!=="cancelled"?outcome.reason:undefined);
  if(workspace_!==undefined&&outcome.kind==="local"&&outcome.reason==="infrastructure"){const cause=/^deadline\b/u.test(outcome.detail)?"deadline":outcome.detail==="operation lost"?"operation-lost":/^silent\b/u.test(outcome.detail)?"silent":undefined;if(cause!==undefined)bumpOffloadCountersV1(workspace_,{},cause);}
  if(outcome.kind==="done"){
   const shown=origin??outcome.origin,progress=line("offloaded",{crate:input.crate,origin:shown,milliseconds:Math.round(outcome.milliseconds)});
   emitOffloadEventV1(input.root,{...input.lane?{lane:input.lane}:{},kind:"offload-run",crate:input.crate,key:input.key,origin:shown,milliseconds:outcome.milliseconds,...bytes.shipped!==undefined?{bytesShipped:bytes.shipped,bytesUploaded:bytes.uploaded??0,bytesReturned:bytes.returned??0}:{},...marks.length>=3?{detail:`plan ${marks[0]!-started} ms, upload ${marks[1]!-marks[0]!} ms, queue and execute ${marks[2]!-marks[1]!} ms, download and write ${Date.now()-marks[2]!} ms${bytes.worker===undefined?"":", "+bytes.worker}`}:{},...progress===undefined?{}:{progress}});
  }
  return outcome;
 };
 return Object.freeze({marks,bytes,started,local,settle});
}

/** 🚫️ Records that a unit never reaches the fleet before an attempt starts (its kind is compiled here, or it is cheaper than a round trip): the counter of its reason and, for a refusal the build's owner may want to see, the `offload-declined` event with its English and German line. Never throws. */
export function declineOffloadV1(root:string,environment:Environment,input:Readonly<{lane?:string;crate:string;key:string|null;reason:OffloadReasonV1;detail?:string;silent?:boolean}>):void{
 const workspace=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||fleetCacheRootV1(environment);
 if(workspace!==undefined)bumpOffloadCountersV1(workspace,{},input.reason);
 if(input.silent!==true)emitOffloadEventV1(root,{...input.lane?{lane:input.lane}:{},kind:"offload-declined",crate:input.crate,key:input.key,reason:input.reason,detail:(input.detail??"").slice(0,200),milliseconds:0});
}

/** 🔗️ What every offload attempt needs before it plans anything: the scheduler endpoint, the cache store with its instance and token, and the checkout the machine's facts live in. */
export type OffloadSessionV1=Readonly<{endpoint:string;content:RemoteExecutionContentV1;instance:string;token:string|undefined;workspace:string|undefined}>;

/** 🔌️ Opens the session of one offload attempt: names the endpoint and the credentials, honours the decline memo of the last seconds and checks that the cache store answers (remembered for 15 seconds). A decline is answered as its reason and detail; nothing throws for a missing fleet. */
export async function openOffloadSessionV1(input:Readonly<{root:string;environment:Environment;ports?:OffloadPortsV1;queue?:boolean}>):Promise<OffloadSessionV1|Readonly<{decline:OffloadReasonV1;detail:string}>>{
 const {environment,ports}=input,workspace=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||fleetCacheRootV1(environment);
 const endpoint=ports?.endpoint??resolveRemoteExecutionEndpointV1({environment,...workspace===undefined?{}:{root:workspace}})?.endpoint;
 if(!endpoint)return{decline:"no-endpoint",detail:""};
 const cache=ports?.content&&ports.token!==undefined?undefined:resolveRemoteCacheV1(environment,workspace===undefined?{}:{root:workspace});
 const token=ports?.token??cache?.token,cacheEndpoint=ports?.cacheEndpoint??cache?.contract.endpoint,instance=ports?.instance??cache?.contract.instance;
 if(!ports?.content&&(!cacheEndpoint||!instance))return{decline:"no-credential",detail:""};
 const full=readOffloadMemoV1<{atMs:number}>(input.root,"store-full",Date.now(),policy.limits.storeFullMemoMilliseconds);
 if(full)return{decline:"infrastructure",detail:"memo the content store is full (507)"};
 const declined=input.queue===true?undefined:readOffloadMemoV1<{atMs:number;reason?:OffloadReasonV1;detail?:string}>(input.root,"declined",Date.now(),policy.limits.declineMemoMilliseconds);
 if(declined)return{decline:declined.reason??"no-idle-machine",detail:`memo${declined.detail?` ${declined.detail}`:""}`};
 if(!ports?.content&&readOffloadMemoV1<{atMs:number;endpoint:string}>(input.root,"reachable",Date.now(),policy.limits.reachableMemoMilliseconds)?.endpoint!==cacheEndpoint){
  const reachable=await fetch(cacheEndpoint!.replace(/\/+$/u,"")+"/healthz",{signal:AbortSignal.timeout(policy.timeouts.probeMilliseconds)}).then(async response=>{await response.body?.cancel();return response.status===200;},()=>false);
  if(!reachable){writeOffloadMemoV1(input.root,"declined",{atMs:Date.now(),reason:"infrastructure",detail:"cache endpoint unreachable"});return{decline:"infrastructure",detail:"cache endpoint unreachable"};}
  writeOffloadMemoV1(input.root,"reachable",{atMs:Date.now(),endpoint:cacheEndpoint!});
 }
 const content=ports?.content??httpRemoteExecutionContentV1({endpoint:cacheEndpoint!,token,...(workspace===undefined||cache===undefined?{}:{fetch:remoteCacheFetchV1({...environment,...cache.environment,SEMIO_WORKSPACE_ROOT:workspace},{root:workspace})}),timeoutSeconds:Math.ceil(policy.timeouts.requestMilliseconds/1000),retries:policy.limits.contentRetries,retryAfterCapMilliseconds:policy.limits.contentRetryAfterCapMilliseconds});
 return{endpoint,content,instance:instance!,token,workspace};
}

/** ⏳️ The store of an upload that rides through a restart of the cache server and through a 507 while the store collects room: existence checks and byte writes are repeated with a doubling pause for at most `worker.outage.seconds` (30 s at most) before the unit falls back to the local compile. */
const patientUpload=(content:RemoteExecutionContentV1):RemoteExecutionContentV1=>{const outage=readRemoteExecutionPolicyV1().worker.outage;return outageTolerantRemoteExecutionContentV1(content,{...outage,seconds:Math.min(outage.seconds,30)});};

/** 🚀️ Hands one rustc unit to another machine. Local first is the caller's rule (it calls this only when the gate offers the unit: in `distribute` mode every unit worth a round trip, in `pin` mode every unit); this builds the hermetic action, releases the job token while it waits, and answers either the compiler's own result translated into the requester's paths (exit status, streams byte for byte, output files written into the real output directory) or the reason the unit compiles here. Never throws: any fault is a reason. The result is NOT stored in the caches here: the caller publishes it like a local compilation, under the unit key it computed, so the entry exists exactly once. */
export async function offloadRustcUnitV1(input:Readonly<{args:readonly string[];environment:Environment;cwd:string;recorded?:readonly string[];root:string;lane?:string;crate:string;key:string;compiler:readonly string[];digest?:(path:string)=>string|null;signal?:AbortSignal;onStage?:(stage:OffloadStageV1)=>void;ports?:OffloadPortsV1;peakMegabytes?:number;queue?:boolean;beyondHost?:Readonly<{report:(text:string)=>void}>}>):Promise<OffloadOutcomeV1>{
 const beyond=input.beyondHost;
 if(beyond===undefined)return offloadRustcAttemptV1(input);
 return awaitBeyondHostV1<OffloadOutcomeV1>({crate:input.crate,...input.signal?{signal:input.signal}:{},report:beyond.report,attempt:remaining=>offloadRustcAttemptV1({...input,queue:true,queueMilliseconds:remaining}),queue:async()=>{const row=await fleetQueueV1(input.environment);return row;}});
}

/** 🚀️ One attempt of {@link offloadRustcUnitV1} (`queueMilliseconds` overrides how long the unit may queue). */
async function offloadRustcAttemptV1(input:Readonly<{args:readonly string[];environment:Environment;cwd:string;recorded?:readonly string[];root:string;lane?:string;crate:string;key:string;compiler:readonly string[];digest?:(path:string)=>string|null;signal?:AbortSignal;onStage?:(stage:OffloadStageV1)=>void;ports?:OffloadPortsV1;peakMegabytes?:number;queue?:boolean;queueMilliseconds?:number}>):Promise<OffloadOutcomeV1>{
 const {environment,cwd,ports}=input,pin=offloadPinV1(environment),queue=input.queue===true||pin!==undefined,waitMs=input.queueMilliseconds??(pin!==undefined?policy.timeouts.pinQueueMilliseconds:queue?policy.timeouts.queueSaturatedMilliseconds:0),{marks,bytes,started,local,settle}=offloadRecorderV1({root:input.root,...input.lane?{lane:input.lane}:{},crate:input.crate,key:input.key,environment});
 try{
  const text=rustcVersionTextV1(input.root,input.compiler,environment),toolchainParsed=text===undefined?undefined:parseRustcIdentityV1(text);
  if(!toolchainParsed)return settle(local("toolchain-missing"));
  const session=await openOffloadSessionV1({root:input.root,environment,queue,...ports?{ports}:{}});
  if("decline" in session)return settle(local(session.decline,session.detail));
  const {endpoint,content,instance,token,workspace}=session;
  const plan=planOffloadV1({args:input.args,environment,cwd,...input.recorded?{recorded:input.recorded}:{}});
  if("declined" in plan)return settle(local(plan.declined,plan.detail));
  const toolchain:OffloadToolchainV1={...toolchainParsed,targets:[]};
  const sdk=offloadLinksNativelyV1(offloadCrateTypesOfV1(plan.arguments),plan.target)?offloadSdkIdentityV1(environment):undefined;
  if(sdk===undefined&&offloadLinksNativelyV1(offloadCrateTypesOfV1(plan.arguments),plan.target)&&process.platform==="darwin")return settle(local("not-portable","the SDK of this link is unknown"));
  const types=offloadCrateTypesOfV1(plan.arguments),linked=offloadPeakMegabytesV1(types,plan.target)>0,sysroot=linked?offloadSysrootV1(input.root,input.compiler,environment):undefined,linker=sysroot===undefined?undefined:offloadLinkerIdentityV1(sysroot,input.root);
  if(linked&&linker===undefined)return settle(local("not-portable","the linkers of this compiler are unknown"));
  const learned=readUnitPeakV1(input.root,input.crate,offloadDeclaredThreadsV1(linked)),peak=input.peakMegabytes??readUnitExpenseV1(input.root,input.crate,types,plan.target).peak,platform=offloadPlatformV1(remoteExecutionHostPropertiesV1(),toolchain,plan.target,(workspace===undefined?undefined:remoteExecutionOriginV1(workspace,environment)),peak,{queue,...pin===undefined?{}:{pin},...sdk===undefined?{}:{sdk},...linker===undefined?{}:{linker}}),cancelled=input.signal??new AbortController().signal;
  const fleet=ports?.workers??(ports?.endpoint!==undefined?undefined:await fleetWorkersV1(environment,input.root));
  if(fleet!==undefined){const fit=eligibleFleetWorkersV1(fleet,remoteExecutionMatchPropertiesV1(platform));if(fit.eligible===0)return settle(local("no-compatible-worker",`none of ${fit.total} workers advertises ${fit.unmet.length>0?fit.unmet.join(", "):"all of it together"}`));}
  marks.push(Date.now());
  const phase=await runBoundedOffloadPhaseV1(signal=>buildOffloadActionV1({plan,content:patientUpload(content),instance,platform,signal,...input.digest?{digest:input.digest}:{}}),{cancelled,milliseconds:ports?.uploadMilliseconds??policy.timeouts.uploadMilliseconds});
  if(phase.kind==="timeout")return settle(local("infrastructure","upload"));
  if(phase.kind==="cancelled")return local("cancelled");
  const built=phase.value;
  if("declined" in built)return settle(local(built.declined,built.detail));
  marks.push(Date.now());bytes.shipped=built.bytes;bytes.uploaded=built.uploaded;
  const lease=tokenLeaseV1(environment);let tokenMilliseconds=0;
  let execution,seen:OffloadStageV1|undefined;
  const staged=(stage:OffloadStageV1):void=>{if(stage===seen)return;seen=stage;emitOffloadStageV1(input.root,input.crate,stage);try{input.onStage?.(stage);}catch{}};
  const outage=readRemoteExecutionPolicyV1().worker.outage,patience=linked?outage.seconds*1000:0;
  try{execution=await executeOffloadV1({deadlineMilliseconds:offloadDeadlineMillisecondsV1(readUnitCostV1(input.root,input.crate),linked),executeMilliseconds:offloadExecuteBudgetMillisecondsV1(offloadActionSecondsV1(types,plan.target))+waitMs+patience,...queue?{queueMilliseconds:waitMs}:{},endpoint,token,...(workspace===undefined||token!==undefined?{}:{sign:(method,content)=>fleetHeadersV1(workspace,"submit","execution","POST",`http://x/${method}`,content)}),instance,action:built.digest,signal:cancelled,onStage:staged,repair:(missing,signal)=>built.repair(missing,signal),onRepair:row=>emitOffloadReuploadV1(input.root,input.lane,input.crate,input.key,started,row),rideThroughMilliseconds:Math.max(policy.timeouts.rideThroughMilliseconds,patience)});}
  finally{void built.release();const waiting=Date.now();await lease?.reacquire();tokenMilliseconds=Date.now()-waiting;lease?.close();}
  if(execution.kind==="declined"){
   if(!queue&&!linked&&(execution.reason==="no-idle-machine"||execution.reason==="queue-timeout"||execution.reason==="worker-policy"))writeOffloadMemoV1(input.root,"declined",{atMs:Date.now(),reason:execution.reason,detail:execution.detail.slice(0,120)});
   return settle(local(execution.reason,execution.detail));
  }
  const downloading=AbortSignal.any([cancelled,AbortSignal.timeout(policy.timeouts.downloadMilliseconds+patience)]),fetched=linked?outageTolerantRemoteExecutionContentV1(content,outage):content;
  marks.push(Date.now());
  const outputs=await readOffloadOutputsV1(fetched,instance,execution.actionResult,policy.virtual.out,[policy.virtual.root],downloading);
  bytes.returned=[...outputs.directory.values()].reduce((sum,file)=>sum+file.digest.size_bytes,0)+outputs.stdout.length+outputs.stderr.length;
  const phases=outputs.phases;
  if(phases!==undefined)bytes.worker=`worker fetch ${phases.fetchMilliseconds} ms, run ${phases.executeMilliseconds} ms, upload ${phases.uploadMilliseconds} ms, away from it ${Math.max(0,marks[2]!-marks[1]!-phases.workerMilliseconds)} ms of which token wait ${tokenMilliseconds} ms${outputs.threads===undefined?"":`, ${outputs.threads} codegen threads`}`;
  const root_=Buffer.from(outputs.files.get(policy.virtual.root)??new Uint8Array(0)).toString("utf8")||undefined,translate=(bytes:Buffer):string=>offloadRealTextV1(bytes.toString("utf8"),plan.mappings,root_,cwd);
  const stdout=translate(outputs.stdout),stderr=translate(outputs.stderr);
  if(outputs.exitCode!==0){
   const verdict=classifyOffloadExitV1({exitCode:outputs.exitCode,stderr}),kind=verdict.kind;
   if(kind==="incomplete-inputs")return settle(local("incomplete-inputs",offloadIncompleteDetailV1(stderr)));
   if(verdict.kind==="environmental")return settle(local("remote-failed-local-redo",`${verdict.class} on ${execution.origin}, exit ${outputs.exitCode}: ${offloadIncompleteDetailV1(stderr)}`));
   if(kind==="infrastructure"){
    const taught=offloadKilledPeakMegabytesV1(outputs.exitCode,peak,learned);
    if(taught!==undefined){writeUnitPeakV1(input.root,input.crate,taught,offloadDeclaredThreadsV1(linked),true);return settle(local("infrastructure",`exit ${outputs.exitCode}: killed on ${execution.origin} with ${peak} MB placed, now ${taught} MB`));}
    return settle(local("infrastructure",`exit ${outputs.exitCode}`));
   }
   return settle({kind:"done",status:outputs.exitCode,stdout,stderr,origin:execution.origin,milliseconds:Date.now()-started,cached:execution.cached});
  }
  if(outputs.peakMegabytes!==undefined)writeUnitPeakV1(input.root,input.crate,outputs.peakMegabytes,outputs.threads??1);
  if(phases!==undefined)writeOffloadOverheadV1(input.root,Math.max(0,Date.now()-started-phases.executeMilliseconds));
  const outDir=resolve(cwd,plan.outDir),marker=root_===undefined?undefined:Buffer.from(root_);
  let written:readonly string[]=[];
  try{
   written=await writeOffloadOutputsV1(fetched,instance,outputs,outDir,(name,bytes)=>{
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
  if((error as {code?:unknown})?.code===8&&/answered 507\b/u.test(String((error as Error)?.message)))writeOffloadMemoV1(input.root,"store-full",{atMs:Date.now()});
  return settle(local("infrastructure",String((error as Error)?.message??error).slice(0,120)));
 }
}

/** 🏷️ The platform properties of an offloaded command: the machine kind of the requester, the scheduler's control properties (`semio-offload=if-idle`, `semio-origin`, and for a step with a predicted peak `semio-peak-mb`), the command format and, for a tool the worker resolves itself, the property that names the tool and its digest, so only a machine that owns exactly that tool is chosen. Sorted, so equal commands are equal actions. */
export function offloadCommandPlatformV1(host:readonly RemoteExecutionPropertyV1[],tool:Readonly<{name:string;digest:string}>,origin:string|undefined,peakMegabytes:number=0):readonly RemoteExecutionPropertyV1[]{
 const p=policy.property,c=COMMAND_POLICY_V1.property,rows=[...host,{name:p.offload,value:p.offloadValue},{name:c.format,value:c.formatValue},...commandToolSourceV1(tool.name)==="resolved"?[commandToolPropertyV1(tool.name,tool.digest)]:[],...origin?[{name:p.origin,value:origin}]:[],...peakMegabytes>0?[{name:p.peak,value:String(Math.ceil(peakMegabytes))}]:[]];
 return rows.sort((left,right)=>left.name<right.name?-1:left.name>right.name?1:0);
}

/** 🛠️ One non-rustc build step to run on another machine: the allow-listed tool (name, the sha-256 of its file and, for a shipped tool, the real file to send), its arguments with `@root/<relative path>` for every path, the input files (relative path in the worker's private root, real file), the output files (`out/<name>`, the real file the result is written to) and the output directories (`out/<name>`, the real directory every file the tool wrote below it is written to; a directory the tool left empty or absent is a missing output). */
export type OffloadCommandInputV1=Readonly<{
 tool:Readonly<{name:string;digest:string;file?:string}>;arguments:readonly string[];inputs:ReadonlyMap<string,string>;outputs:ReadonlyMap<string,string>;directories?:ReadonlyMap<string,string>;variables?:readonly (readonly [string,string])[];
 root:string;environment:Environment;lane?:string;label:string;key:string|null;seconds?:number;predictedMilliseconds?:number;peakMegabytes?:number;digest?:(path:string)=>string|null;signal?:AbortSignal;onStage?:(stage:OffloadStageV1)=>void;ports?:OffloadPortsV1;
}>;

/** 🚀️ Hands one non-rustc build step to another machine and writes its declared outputs where a local run would: the same plan, headroom check, placement, progress, cancel, timeouts and decline memo as the rustc unit offload (one session, one recorder, one shipping path, one execution and ride-through client), and the same promise: it never throws and never leaves a half-written output; every fault, a refusal of the worker, a missing output or a non-zero exit of the tool is a reason to run the step here, where its own diagnostics are produced. The caller publishes the result under its own key, so the entry exists once. */
export async function offloadCommandV1(input:OffloadCommandInputV1):Promise<OffloadOutcomeV1>{
 const {environment,ports}=input,directories=input.directories??new Map<string,string>(),describe=(kind:"offloaded"|"declined"|"fallback",variables:Readonly<Record<string,string|number>>):string=>commandTextsV1(kind,variables);
 const {marks,bytes,started,local,settle}=offloadRecorderV1({root:input.root,...input.lane?{lane:input.lane}:{},crate:input.label,key:input.key,environment,describe});
 try{
  if(commandToolSourceV1(input.tool.name)===undefined)return settle(local("not-portable",`tool ${input.tool.name}`));
  const shipped=commandToolSourceV1(input.tool.name)==="shipped";
  if(shipped&&input.tool.file===undefined)return settle(local("not-portable","shipped tool without file"));
  const limits=COMMAND_POLICY_V1.limits;
  if(input.outputs.size+directories.size>limits.maximumOutputFiles)return settle(local("not-portable","too many outputs"));
  if(input.inputs.size>limits.maximumInputFiles)return settle(local("inputs-too-large",`${input.inputs.size} files`));
  const session=await openOffloadSessionV1({root:input.root,environment,...ports?{ports}:{}});
  if("decline" in session)return settle(local(session.decline,session.detail));
  const {endpoint,content,instance,token,workspace}=session,files=new Map(input.inputs),executable=new Set<string>();
  if(shipped){const path=`${COMMAND_POLICY_V1.root.tool}/${input.tool.name}`;files.set(path,input.tool.file!);executable.add(path);}
  const peak=input.peakMegabytes??commandToolPeakMegabytesV1(input.tool.name),platform=offloadCommandPlatformV1(remoteExecutionHostPropertiesV1(),input.tool,workspace===undefined?undefined:remoteExecutionOriginV1(workspace,environment),peak),cancelled=input.signal??new AbortController().signal,seconds=input.seconds??COMMAND_POLICY_V1.timeouts.actionSeconds;
  marks.push(Date.now());
  const phase=await runBoundedOffloadPhaseV1(signal=>shipOffloadActionV1({spec:{files,executable,arguments:[commandProgramV1(input.tool.name,input.tool.digest),...input.arguments],environment:[...input.variables??[]].sort(([left],[right])=>left<right?-1:1),outputPaths:[...input.outputs.keys(),...directories.keys()].sort(),seconds},content:patientUpload(content),instance,platform,signal,...input.digest?{digest:input.digest}:{}}),{cancelled,milliseconds:ports?.uploadMilliseconds??COMMAND_POLICY_V1.timeouts.uploadMilliseconds});
  if(phase.kind==="timeout")return settle(local("infrastructure","upload"));
  if(phase.kind==="cancelled")return local("cancelled");
  const built=phase.value;
  if("declined" in built)return settle(local(built.declined,built.detail));
  marks.push(Date.now());bytes.shipped=built.bytes;bytes.uploaded=built.uploaded;
  const lease=tokenLeaseV1(environment);
  let execution,seen:OffloadStageV1|undefined;
  const staged=(stage:OffloadStageV1):void=>{
   if(stage===seen)return;seen=stage;
   if(stage==="QUEUED"||stage==="EXECUTING")emitOffloadProgressV1(input.root,commandTextsV1(stage==="QUEUED"?"stage.queued":"stage.executing",{crate:input.label}));
   try{input.onStage?.(stage);}catch{}
  };
  try{execution=await executeOffloadV1({deadlineMilliseconds:offloadDeadlineMillisecondsV1(input.predictedMilliseconds??readUnitCostV1(input.root,input.label),peak>0),executeMilliseconds:offloadExecuteBudgetMillisecondsV1(seconds),endpoint,token,...(workspace===undefined||token!==undefined?{}:{sign:(method,body)=>fleetHeadersV1(workspace,"submit","execution","POST",`http://x/${method}`,body)}),instance,action:built.digest,signal:cancelled,onStage:staged,repair:(missing,signal)=>built.repair(missing,signal),onRepair:row=>emitOffloadReuploadV1(input.root,input.lane,input.label,input.key,started,row),rideThroughMilliseconds:peak>0?Math.max(policy.timeouts.rideThroughMilliseconds,readRemoteExecutionPolicyV1().worker.outage.seconds*1000):policy.timeouts.rideThroughMilliseconds});}
  finally{void built.release();await lease?.reacquire();lease?.close();}
  if(execution.kind==="declined"){
   if(peak<=0&&(execution.reason==="no-idle-machine"||execution.reason==="queue-timeout"||execution.reason==="worker-policy"))writeOffloadMemoV1(input.root,"declined",{atMs:Date.now(),reason:execution.reason,detail:execution.detail.slice(0,120)});
   return settle(local(execution.reason,execution.detail));
  }
  const downloading=AbortSignal.any([cancelled,AbortSignal.timeout(COMMAND_POLICY_V1.timeouts.downloadMilliseconds)]);
  marks.push(Date.now());
  const outputs=await readOffloadOutputsV1(content,instance,execution.actionResult,"",[...input.outputs.keys()],downloading);
  bytes.returned=[...outputs.files.values()].reduce((sum,file)=>sum+file.length,0)+outputs.stdout.length+outputs.stderr.length;
  if(outputs.exitCode!==0)return settle(local("incomplete-inputs",`exit ${outputs.exitCode}`));
  for(const name of input.outputs.keys())if(!outputs.files.has(name))return settle(local("incomplete-inputs",`output ${name} missing`));
  const trees=new Map<string,OffloadOutputsV1>();
  for(const name of directories.keys()){
   const tree=await readOffloadOutputsV1(content,instance,execution.actionResult,name,[],downloading);
   if(tree.directory.size===0)return settle(local("incomplete-inputs",`output ${name} missing`));
   trees.set(name,tree);
  }
  const written:string[]=[],landed:string[]=[];
  try{
   for(const [name,destination] of directories){
    const staging=`${destination}.${randomBytes(4).toString("hex")}.tmp`;
    try{
     await writeOffloadOutputsV1(content,instance,trees.get(name)!,staging,(_name,bytes)=>bytes,downloading);
     mkdirSync(destination,{recursive:true});
     for(const entry of readdirSync(staging)){const target=join(destination,entry);rmSync(target,{recursive:true,force:true});renameSync(join(staging,entry),target);landed.push(target);}
    }finally{rmSync(staging,{recursive:true,force:true});}
   }
   for(const [name,destination] of input.outputs){
    const temporary=`${destination}.${randomBytes(4).toString("hex")}.tmp`;
    mkdirSync(dirname(destination),{recursive:true});
    try{writeFileSync(temporary,outputs.files.get(name)!);rmSync(destination,{force:true});renameSync(temporary,destination);written.push(destination);}catch(error){rmSync(temporary,{force:true});throw error;}
   }
  }catch(error){
   for(const path of written)rmSync(path,{force:true});
   for(const path of landed)rmSync(path,{recursive:true,force:true});
   return settle(local("infrastructure",String((error as Error)?.message??error).slice(0,120)));
  }
  return settle({kind:"done",status:0,stdout:outputs.stdout.toString("utf8"),stderr:outputs.stderr.toString("utf8"),origin:execution.origin,milliseconds:Date.now()-started,cached:execution.cached});
 }catch(error){
  if(input.signal?.aborted)return local("cancelled");
  return settle(local("infrastructure",String((error as Error)?.message??error).slice(0,120)));
 }
}
