import {randomBytes} from "node:crypto";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {decodeProtobufV1,REAPI_V1} from "../../../📡️wire/📦️protobuf/🟦️.ts";
import {fleetHeadersV1} from "../../../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {remoteExecutionHostPropertiesV1} from "../../../🟦️.ts";
import {remoteExecutionOriginV1} from "../../../🌡️load/🟦️.ts";
import {shipOffloadActionV1} from "../../📜️action/🟦️.ts";
import {executeOffloadV1,offloadExecuteBudgetMillisecondsV1,readOffloadOutputsV1,runBoundedOffloadPhaseV1,type OffloadStageV1} from "../../🔌️client/🟦️.ts";
import {openOffloadSessionV1,type OffloadSessionV1} from "../../🟦️.ts";
import type {OffloadReasonV1} from "../../📐️policy/🟦️.ts";
import {checkoutTestForwardedV1,CHECKOUT_TEST_POLICY_V1,checkoutTestBothV1,checkoutTestChunkKeyV1,checkoutTestLinesV1,checkoutTestPlatformV1,checkoutTestProgramV1,checkoutTestProblemV1,decodeCheckoutTestChunkV1,requireCheckoutTestShapeV1,type CheckoutTestRequestV1,type CheckoutTestResultV1} from "../📐️policy/🟦️.ts";
import {fleetCheckoutReplicaV1,type CheckoutTestReplicaV1} from "../🧭️replica/🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;

/** 🔌️ The seams of one remote test run; tests replace them (a fake scheduler answers `execute`, an in-memory store backs the session), production uses the fleet. */
export type CheckoutTestPortsV1=Readonly<{replica?:CheckoutTestReplicaV1;openSession?:typeof openOffloadSessionV1;execute?:typeof executeOffloadV1;ship?:typeof shipOffloadActionV1;read?:typeof readOffloadOutputsV1;readChunk?:(hash:string,signal:AbortSignal)=>Promise<Uint8Array|undefined>;sleep?:(milliseconds:number,signal:AbortSignal)=>Promise<void>;now?:()=>number}>;

/** 🔁️ The reasons after which the next Ready machine is tried: the machine had no room, refused the file (not Ready, replica behind, other file content) or offers no checkout tests. */
const NEXT_MACHINE_V1:ReadonlySet<OffloadReasonV1>=new Set<OffloadReasonV1>(["no-idle-machine","no-compatible-worker","queue-timeout","worker-policy"]);

/** 🏁️ How one remote test run ended: the origin that ran the file with its structured result, its exit status and its output; or a reason to run the file here. A failing test file is a result, never a reason to run it again here. */
export type CheckoutTestOutcomeV1=Readonly<{kind:"done";origin:string;result:unknown;exitCode:number;stderr:string;streamed:boolean;seconds:number}>|Readonly<{kind:"local";reason:OffloadReasonV1;detail:string}>;

/** 🚀️ Hands one test file to an idle Ready machine of the fleet and waits for it: takes the Ready machines of this platform from their readiness records (`peers`), ships the request (the file's digest and this replica's applied cut) as the one input file of an action pinned to one of them at a time (`if-idle`: a machine without room or that refuses the file answers at once and the next one is tried, at most `maximumCandidates`; without `peers` the scheduler picks; when every one had no room all are asked again every `pauseMilliseconds` for at most `retryMilliseconds`), follows its stages, cancels it when `signal` aborts, and reads the worker's result from the standard output of the action. Every fault (no scheduler, no credential, no idle machine, refusal, replica behind, lost operation, deadline, unreadable answer) is a `local` outcome with its reason and the caller runs the file here; nothing throws. Output of the worker is delivered when its file ends, with the stages (`QUEUED`, `EXECUTING`) as progress meanwhile. */
export async function offloadCheckoutTestV1(input:Readonly<{root:string;memoRoot:string;environment:Environment;file:string;budgetSeconds:number;stretchBasisPoints?:number;language:"en"|"de";retryMilliseconds?:number;signal?:AbortSignal;claim?:Set<string>;onOutput?:(text:string,origin:string)=>void;onStage?:(stage:OffloadStageV1)=>void;ports?:CheckoutTestPortsV1}>):Promise<CheckoutTestOutcomeV1>{
 const policy=CHECKOUT_TEST_POLICY_V1,local=(reason:OffloadReasonV1,detail=""):CheckoutTestOutcomeV1=>({kind:"local",reason,detail:detail.slice(0,300)}),ports=input.ports??{},cancelled=input.signal??new AbortController().signal,started=Date.now();
 try{
  const replica=ports.replica??fleetCheckoutReplicaV1(input.root);
  if(replica===undefined)return local("no-endpoint",checkoutTestBothV1("detail.no-fleet"));
  const cut=await replica.cut(),sha=replica.digest(input.file);
  if(cut===undefined)return local("infrastructure",checkoutTestBothV1("detail.no-cut"));
  if(sha===undefined)return local("unreadable-input",input.file);
  const peers=await replica.peers?.();
  if(peers!==undefined&&peers.length===0)return local("no-idle-machine",checkoutTestBothV1("detail.no-ready"));
  const session=await (ports.openSession??openOffloadSessionV1)({root:input.memoRoot,environment:input.environment});
  if("decline" in session)return local(session.decline,session.detail);
  const {endpoint,content,instance,executionToken,workspace}=session as OffloadSessionV1,origin=workspace===undefined?replica.origin:remoteExecutionOriginV1(workspace,input.environment),seconds=input.budgetSeconds+policy.timeouts.marginSeconds;
  const attempt=async(pin:string|undefined):Promise<CheckoutTestOutcomeV1>=>{
   const request:CheckoutTestRequestV1={version:1,nonce:randomBytes(16).toString("hex"),file:input.file,fileSha256:sha,cut,language:input.language,budgetSeconds:input.budgetSeconds,...input.stretchBasisPoints===undefined?{}:{stretchBasisPoints:input.stretchBasisPoints},...checkoutTestForwardedV1(input.environment)},text=JSON.stringify(request);
   requireCheckoutTestShapeV1("Request",request,"Closed checkout test request required");
   if(Buffer.byteLength(text)>policy.limits.maximumRequestBytes)return local("inputs-too-large","request");
   const staging=mkdtempSync(join(tmpdir(),"semio-checkout-test-"));
   try{
    const real=join(staging,"request.json");
    writeFileSync(real,text);
    const platform=checkoutTestPlatformV1(remoteExecutionHostPropertiesV1(),origin,pin);
    const phase=await runBoundedOffloadPhaseV1(signal=>(ports.ship??shipOffloadActionV1)({spec:{files:new Map([[policy.request.path,real]]),arguments:[checkoutTestProgramV1()],environment:[],outputPaths:[],seconds},content,instance,platform,signal}),{cancelled,milliseconds:policy.timeouts.uploadMilliseconds});
    if(phase.kind==="timeout")return local("infrastructure",checkoutTestBothV1("detail.upload"));
    if(phase.kind==="cancelled")return local("cancelled");
    const built=phase.value;
    if("declined" in built)return local(built.declined,built.detail);
    let execution;
    const following:{current?:ChunkFollowV1}={},begin=():void=>{if(input.onOutput!==undefined&&following.current===undefined)following.current=followCheckoutTestChunksV1({nonce:request.nonce,read:(hash,signal)=>(ports.readChunk??((key,abort)=>content.readActionResult(instance,key,abort)))(hash,signal),onText:input.onOutput,signal:cancelled,pollMilliseconds:policy.stream.pollMilliseconds});};
    try{
     execution=await (ports.execute??executeOffloadV1)({deadlineMilliseconds:seconds*1000,executeMilliseconds:offloadExecuteBudgetMillisecondsV1(seconds),endpoint,token:executionToken,...(workspace===undefined||executionToken!==undefined?{}:{sign:(method:string,body:string)=>fleetHeadersV1(workspace,"submit","execution","POST",`http://x/${method}`,body)}),instance,action:built.digest,signal:cancelled,onStage:stage=>{if(stage==="EXECUTING")begin();input.onStage?.(stage);},repair:(missing,signal)=>built.repair(missing,signal)});
    }finally{void built.release();await following.current?.stop();}
    if(execution.kind==="declined")return local(execution.reason,execution.detail);
    const downloading=AbortSignal.any([cancelled,AbortSignal.timeout(policy.timeouts.downloadMilliseconds)]),outputs=await (ports.read??readOffloadOutputsV1)(content,instance,execution.actionResult,"",[],downloading);
    const full=outputs.stderr.toString("utf8"),live=following.current,streamedBytes=live?.bytes()??0,stderr=streamedBytes>0?live!.finish(outputs.stderr.subarray(Math.min(streamedBytes,outputs.stderr.length))):full,limit=policy.limits.maximumResultBytes;
    if(outputs.stdout.length>limit)return local("remote-failed-local-redo",checkoutTestBothV1("detail.too-large",{maximum:limit}));
    if(outputs.exitCode!==0&&outputs.exitCode!==1)return local("remote-failed-local-redo",checkoutTestBothV1("detail.exit",{code:outputs.exitCode,tail:full.slice(-policy.limits.maximumTailCharacters)}));
    let parsed:unknown;
    try{parsed=JSON.parse(outputs.stdout.toString("utf8"));}catch{return local("remote-failed-local-redo",checkoutTestBothV1("detail.not-json"));}
    const problem=checkoutTestProblemV1("Result",parsed);
    if(problem!==undefined)return local("remote-failed-local-redo",checkoutTestBothV1("detail.invalid",{detail:problem}));
    const answer=parsed as CheckoutTestResultV1;
    return{kind:"done",origin:answer.origin,result:answer.result,exitCode:outputs.exitCode,stderr,streamed:streamedBytes>0,seconds:Math.round((Date.now()-started)/100)/10};
   }finally{rmSync(staging,{recursive:true,force:true});}
  };
  const now=ports.now??Date.now,until=now()+(input.retryMilliseconds??policy.wait.retryMilliseconds),pause=ports.sleep??((milliseconds,signal)=>new Promise<void>(done=>{const timer=setTimeout(done,milliseconds);signal.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});}));
  let last:CheckoutTestOutcomeV1=local("no-idle-machine"),candidates=peers;
  for(;;){
   const free=candidates===undefined?[undefined]:candidates.filter(pin=>input.claim===undefined||!input.claim.has(pin)).slice(0,policy.limits.maximumCandidates);
   if(candidates!==undefined&&candidates.length>0&&free.length===0)return local("no-idle-machine",checkoutTestBothV1("detail.claimed"));
   for(const pin of free){
    if(cancelled.aborted)return local("cancelled");
    if(pin!==undefined)input.claim?.add(pin);
    try{last=await attempt(pin);}finally{if(pin!==undefined)input.claim?.delete(pin);}
    if(last.kind==="done"||!NEXT_MACHINE_V1.has(last.reason))return last;
   }
   if(candidates===undefined||last.kind!=="local"||last.reason!=="no-idle-machine"||now()+policy.wait.pauseMilliseconds>until)return last;
   await pause(policy.wait.pauseMilliseconds,cancelled);
   if(cancelled.aborted)return local("cancelled");
   candidates=await replica.peers?.();
   if(candidates===undefined||candidates.length===0)return last;
  }
 }catch(error){
  if(input.signal?.aborted)return local("cancelled");
  return local("infrastructure",String((error as Error)?.message??error));
 }
}

/** 🌊️ A follower of the live output of one remote run: reads piece after piece from the action cache (`read` by key, in order, each checked against the nonce, the index and the byte offset it must have), hands the decoded text to `onText` while the file runs, and stops at the last piece, at the first piece that is not the expected one or when stopped. `stop` makes one more pass over what the worker wrote before it answered; `bytes` is how many bytes were shown; `finish` decodes the rest of the output (what the final answer holds beyond them) with the same decoder, so no character is split. */
export type ChunkFollowV1=Readonly<{stop:()=>Promise<void>;bytes:()=>number;finish:(rest:Uint8Array)=>string}>;

/** 🌊️ Starts a follower of the live output of the request `nonce`; it never throws and ends with its signal. */
export function followCheckoutTestChunksV1(input:Readonly<{nonce:string;read:(hash:string,signal:AbortSignal)=>Promise<Uint8Array|undefined>;onText:(text:string,origin:string)=>void;signal:AbortSignal;pollMilliseconds:number;sleep?:((milliseconds:number,signal:AbortSignal)=>Promise<void>)|undefined}>):ChunkFollowV1{
 const stopper=new AbortController(),decoder=new TextDecoder(),link=():void=>{stopper.abort();},pause=input.sleep??((milliseconds:number,signal:AbortSignal)=>new Promise<void>(done=>{const timer=setTimeout(done,milliseconds);signal.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});}));
 input.signal.addEventListener("abort",link,{once:true});
 let index=0,offset=0,ended=false;
 const next=async():Promise<boolean>=>{
  if(ended||input.signal.aborted)return false;
  let bytes:Uint8Array|undefined;
  try{bytes=await input.read(checkoutTestChunkKeyV1(input.nonce,index),input.signal);}catch{return false;}
  if(bytes===undefined)return false;
  let raw:Uint8Array|undefined;
  try{raw=(decodeProtobufV1(REAPI_V1.actionResult,bytes) as {stderr_raw?:Uint8Array}).stderr_raw;}catch{ended=true;return false;}
  const chunk=decodeCheckoutTestChunkV1(raw);
  if(chunk===undefined||chunk.nonce!==input.nonce||chunk.index!==index||chunk.offset!==offset){ended=true;return false;}
  const data=Buffer.from(chunk.data,"base64"),text=decoder.decode(data,{stream:true});
  index++;offset+=data.length;if(chunk.last)ended=true;
  if(text!=="")input.onText(text,chunk.origin);
  return true;
 };
 const loop=(async()=>{while(!ended&&!stopper.signal.aborted){if(!await next())await pause(input.pollMilliseconds,stopper.signal);}})().catch(()=>{});
 return Object.freeze({
  async stop():Promise<void>{stopper.abort();await loop;input.signal.removeEventListener("abort",link);while(await next()){}},
  bytes:()=>offset,
  finish:(rest:Uint8Array)=>decoder.decode(rest),
 });
}

/** 💬️ The progress line of a stage of a remote test run, in both languages. */
export const checkoutTestStageLinesV1=(stage:OffloadStageV1,file:string):string|undefined=>stage==="QUEUED"?checkoutTestLinesV1("stage.queued",{file}):stage==="EXECUTING"?checkoutTestLinesV1("stage.executing",{file}):undefined;
