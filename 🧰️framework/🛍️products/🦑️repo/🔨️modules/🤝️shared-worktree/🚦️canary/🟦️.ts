import {readFile} from "node:fs/promises";
import {join} from "node:path";
import {compileJsonSchemaSubset,type CompiledJsonSchemaSubset} from "../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {listOriginsV1,replaceOriginFileV1,storePathsV1} from "../📒️journal/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const validators=new Map<string,CompiledJsonSchemaSubset>(),bytewise=(left:string,right:string):number=>Buffer.compare(Buffer.from(left),Buffer.from(right)),missing=(error:unknown):boolean=>(error as {code?:string}).code==="ENOENT";

/** 🛡️ Validates a value against one definition of the canary schema. */
export function requireCanaryShapeV1(definition:"Refusals"|"Input"|"Verdict"|"Fixture",value:unknown,message:string):void{
 let validator=validators.get(definition);
 if(validator===undefined){validator=compileJsonSchemaSubset({$ref:`#/definitions/${definition}`},schema);validators.set(definition,validator);}
 const errors=validator.validate(value);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 🕰️ The times of the admission in milliseconds: an attestation is fresh while its writer's clock is within `freshMs` of ours, code that nobody runs waits `waitMs` before the first candidate may go, every further candidate waits `turnMs` more than the one before it (a canary needs its jitter, self-check and boot), and a follower waits until a canary has stayed healthy for `canaryMs`. */
export type CanaryLimitsV1=Readonly<{freshMs:number;waitMs:number;turnMs:number;canaryMs:number}>;

/** 🕰️ The default times: fresh 60 s, wait 2 min, one turn 5 min, canary healthy for 1 min. */
export const canaryLimitsV1:CanaryLimitsV1={freshMs:60_000,waitMs:120_000,turnMs:300_000,canaryMs:60_000};

/** 📡️ What the admission needs of one attestation: who wrote it, which group, the code it runs, its writer's clock, whether sync is on and whether it runs a fallback code. */
export type CanaryPeerV1=Readonly<{originId:string;groupId:string;code:string;wallMs:number;sync:boolean;fallback:boolean}>;

/** 🚫️ A refusal record of one origin: the code digest, why, and when. */
export type CanaryRefusalV1=Readonly<{digest:string;reason:string;wallMs:number}>;

/** 🚫️ A refusal together with the origin that published it. */
export type CanaryRefusalByV1=CanaryRefusalV1&Readonly<{originId:string}>;

/** 📥️ Everything the pure decision reads: the code (`digest`) and what runs (`running`), this origin and group, the clock, how long this digest has been waiting (`waitedMs`), the attestations (`peers`, own included), retired origins, the digests with a store snapshot, the refusals, whether every changed file was written here (`authored`), how long each other origin has been healthy on the digest (`healthy`) and the times. */
export type CanaryInputV1=Readonly<{digest:string;running:string;originId:string;groupId:string;now:number;waitedMs:number;authored:boolean;peers:readonly CanaryPeerV1[];retired:readonly string[];snapshots:readonly string[];refusals:readonly CanaryRefusalByV1[];healthy:Readonly<Record<string,number>>;limits:CanaryLimitsV1}>;

/** 🚦️ The verdict: `ok` with the `role` that grants it (`running` the code already runs, `snapshot` a healthy run left a snapshot in the store, `follower` a canary stayed healthy, `author` every changed file was written here, `leader` it is this origin's turn), or not `ok` with the reason `why` (`refused` somebody refused the digest, `canary` a canary runs it but has not been healthy long enough, `mixed` nobody runs it and the first candidate has not waited long enough, `turn` an earlier candidate has to go first), the origin it concerns and the seconds to wait at least. */
export type CanaryVerdictV1=Readonly<{ok:true;role:"running"|"snapshot"|"follower"|"author"|"leader"}>|Readonly<{ok:false;why:"refused"|"canary"|"turn"|"mixed";origin:string|null;seconds:number}>;

const seconds=(milliseconds:number):number=>Math.max(0,Math.ceil(milliseconds/1000));

/** 🎖️ The candidates in turn order: this origin and every other fresh, syncing origin of the group that runs no fallback and is not retired, the group server (`<group>-server`) first, the rest bytewise by id. */
export function canaryCandidatesV1(input:Pick<CanaryInputV1,"originId"|"groupId"|"now"|"peers"|"retired"|"limits">):readonly string[]{
 const retired=new Set(input.retired),found=new Set<string>([input.originId]);
 for(const peer of input.peers)if(peer.originId!==input.originId&&peer.groupId===input.groupId&&!retired.has(peer.originId)&&peer.sync&&!peer.fallback&&Math.abs(input.now-peer.wallMs)<=input.limits.freshMs)found.add(peer.originId);
 const server=`${input.groupId}-server`,rest=[...found].filter(origin=>origin!==server).sort(bytewise);
 return found.has(server)?[server,...rest]:rest;
}

/** 🚦️ Decides whether this origin may switch to the code on disk. Deterministic and free of I/O, so that every origin that reads the same store reaches the same order. A digest somebody refused never passes; a digest with a healthy store snapshot, or that another origin has run healthy for `canaryMs`, passes; code written entirely here passes; otherwise nobody has proven it and the candidates go one after another (group server first, then ascending ids), the first after `waitMs`, each further one `turnMs` later, so that the fleet can never wait on itself and one bad code never reaches two machines before the first has shown it healthy. */
export function decideCanaryV1(input:CanaryInputV1):CanaryVerdictV1{
 if(input.digest===input.running)return {ok:true,role:"running"};
 const refusal=input.refusals.filter(item=>item.digest===input.digest&&!transientRefusalExpiredV1(item,input.now)).sort((left,right)=>left.wallMs-right.wallMs||bytewise(left.originId,right.originId))[0];
 if(refusal!==undefined)return {ok:false,why:"refused",origin:refusal.originId,seconds:0};
 if(input.snapshots.includes(input.digest))return {ok:true,role:"snapshot"};
 const retired=new Set(input.retired),runners=input.peers.filter(peer=>peer.originId!==input.originId&&peer.groupId===input.groupId&&!retired.has(peer.originId)&&peer.code===input.digest&&peer.sync&&!peer.fallback&&Math.abs(input.now-peer.wallMs)<=input.limits.freshMs).map(peer=>peer.originId).sort(bytewise);
 if(runners.length>0){
  let best=runners[0]!;
  for(const origin of runners)if((input.healthy[origin]??0)>(input.healthy[best]??0))best=origin;
  const age=input.healthy[best]??0;
  return age>=input.limits.canaryMs?{ok:true,role:"follower"}:{ok:false,why:"canary",origin:best,seconds:seconds(input.limits.canaryMs-age)};
 }
 if(input.authored)return {ok:true,role:"author"};
 const order=canaryCandidatesV1(input),rank=order.indexOf(input.originId),due=input.limits.waitMs+rank*input.limits.turnMs;
 if(input.waitedMs>=due)return {ok:true,role:"leader"};
 if(input.waitedMs<input.limits.waitMs)return {ok:false,why:"mixed",origin:order[0]!,seconds:seconds(input.limits.waitMs-input.waitedMs)};
 return {ok:false,why:"turn",origin:order[rank-1]!,seconds:seconds(due-input.waitedMs)};
}

const transientStartPatternsV1=[/the watchdog of the daemon fell silent/u,/the event loop was blocked for/u,/the daemon was not ready after/u,/the readiness side/u,/did not stop within/u,/the watchdog ended this code while it was new/u],transientMarkerV1=/ \[transient ([0-9]+)\]$/u;

/** 🌫️ Tells whether a start failure says something about the load of the machine (a silent watchdog, a blocked event loop, a daemon not ready in time, a stop that took too long, a watchdog end of new code) and not about the code: such a failure is refused only for a back-off, a deterministic one (an exit status, a self-check, a start error) for good. */
export function transientStartV1(reason:string):boolean{return transientStartPatternsV1.some(pattern=>pattern.test(reason));}

/** 🔢️ How many times in a row a transient refusal was published for its digest (`[transient n]` ends its reason); 0 for a refusal that stays. */
export function transientRefusalCountV1(reason:string):number{return Number(transientMarkerV1.exec(reason)?.[1]??0);}

/** ⏳️ The back-off of a transient refusal: five minutes, doubled with every further failure of the same digest, at most half an hour. */
export function transientBackoffMsV1(count:number):number{return Math.min(1_800_000,300_000*2**Math.max(0,count-1));}

/** 🕰️ Tells whether a transient refusal has served its back-off, so that the digest may be tried again. */
export function transientRefusalExpiredV1(refusal:CanaryRefusalV1,now:number):boolean{
 const count=transientRefusalCountV1(refusal.reason);
 return count>0&&now-refusal.wallMs>=transientBackoffMsV1(count);
}

/** ⏱️ The longest a change set waits for its turn: a machine adopts the newest digest after fifteen minutes of continuous waiting, whatever newer edits arrived meanwhile, if nobody refused it (`maximumWaitMsV1`). */
export const maximumWaitMsV1=900_000;

/** 📖️ The words of a waiting verdict, English and German in one line (`English / Deutsch`), as the attestation and `status` show them. */
export function canaryReasonV1(verdict:Extract<CanaryVerdictV1,{ok:false}>):string{
 const origin=verdict.origin??"-",wait=verdict.seconds;
 if(verdict.why==="refused")return `the code was refused by ${origin} / der Code wurde von ${origin} abgelehnt`;
 if(verdict.why==="canary")return `${origin} runs this code and has to stay healthy for ${wait} s more / ${origin} führt diesen Code aus und muss noch ${wait} s gesund bleiben`;
 if(verdict.why==="mixed")return `no origin runs this code yet and it was not all written on this machine; ${origin} goes first in ${wait} s / noch kein Ursprung führt diesen Code aus und er wurde nicht ganz auf diesem Rechner geschrieben; ${origin} geht in ${wait} s zuerst`;
 return `no origin runs this code yet; ${origin} goes before this machine, which is next in ${wait} s / noch kein Ursprung führt diesen Code aus; ${origin} geht vor diesem Rechner, der in ${wait} s an der Reihe ist`;
}

/** 👁️ Follows how long each other origin has been healthy on one digest: healthy means a fresh attestation with that code, sync on and no fallback. The time starts at the first reading and is kept while readings follow each other within `freshMs`; a gap, another digest or an unhealthy reading starts it again. */
export type CanaryTrackerV1={observe(digest:string,originId:string,peers:readonly CanaryPeerV1[],now:number):Readonly<Record<string,number>>};

/** 🏭️ Creates a tracker with the freshness window `freshMs`. */
export function createCanaryTrackerV1(freshMs:number=canaryLimitsV1.freshMs):CanaryTrackerV1{
 let current="";
 const seen=new Map<string,{since:number;last:number}>();
 return {
  observe(digest,originId,peers,now){
   if(digest!==current){seen.clear();current=digest;}
   const healthy=new Set<string>();
   for(const peer of peers)if(peer.originId!==originId&&peer.code===digest&&peer.sync&&!peer.fallback&&Math.abs(now-peer.wallMs)<=freshMs)healthy.add(peer.originId);
   for(const origin of [...seen.keys()])if(!healthy.has(origin))seen.delete(origin);
   const ages:Record<string,number>={};
   for(const origin of [...healthy].sort(bytewise)){
    const before=seen.get(origin),since=before!==undefined&&now-before.last<=freshMs?before.since:now;
    seen.set(origin,{since,last:now});
    ages[origin]=now-since;
   }
   return ages;
  }
 };
}

/** 📍️ The path of the refusal record of an origin in the store: `<store>/v1/origins/<origin>/refused.json`. */
export function refusalsPathV1(storeRoot:string,originId:string):string{return join(storePathsV1(storeRoot).origins,originId,"refused.json");}

/** 👀️ Reads the refusal record of one origin; `undefined` when there is none, it is torn or it is not closed. */
export async function readRefusalsOfV1(storeRoot:string,originId:string):Promise<readonly CanaryRefusalV1[]|undefined>{
 let text:string;
 try{text=await readFile(refusalsPathV1(storeRoot,originId),"utf8");}catch(error){if(missing(error))return undefined;throw error;}
 try{
  const value=JSON.parse(text) as unknown;
  requireCanaryShapeV1("Refusals",value,"Closed refusal record required");
  const record=value as {originId:string;refused:readonly CanaryRefusalV1[]};
  return record.originId===originId?record.refused:undefined;
 }catch{return undefined;}
}

/** 📚️ Reads the refusals of all origins of a store. */
export async function readRefusalsV1(storeRoot:string,signal?:AbortSignal):Promise<readonly CanaryRefusalByV1[]>{
 const found:CanaryRefusalByV1[]=[];
 for(const origin of await listOriginsV1(storeRoot)){
  signal?.throwIfAborted();
  for(const item of await readRefusalsOfV1(storeRoot,origin)??[])found.push({...item,originId:origin});
 }
 return found;
}

/** 🚫️ Publishes that this origin refuses a code digest (the own record only, tmp + fsync + rename): every origin that reads the store then waits for a different code. Idempotent; keeps the newest 64; `true` when a record was written. */
export async function publishRefusalV1(storeRoot:string,originId:string,input:Readonly<{digest:string;reason:string}>,now:number):Promise<boolean>{
 const before=await readRefusalsOfV1(storeRoot,originId)??[],existing=before.find(item=>item.digest===input.digest),transient=transientStartV1(input.reason);
 if(existing!==undefined&&(!transient||transientRefusalCountV1(existing.reason)===0||!transientRefusalExpiredV1(existing,now)))return false;
 const count=transient?transientRefusalCountV1(existing?.reason??"")+1:0,reason=(transient?`${input.reason.slice(0,480)} [transient ${count}]`:input.reason.slice(0,512))||"refused";
 const refused=[...before.filter(item=>item.digest!==input.digest),{digest:input.digest,reason,wallMs:now}].slice(-64);
 await replaceOriginFileV1(storeRoot,originId,"refused.json",JSON.stringify({version:1,originId,refused}));
 return true;
}

/** 🔎️ The start of the text that the boot stage gives as the reason of a fallback because the worktree code did not start (`🔁️replica/🛟️supervisor/🟦️.ts`, `supervise`), as opposed to a code that only ran before and was superseded. */
export const failedStartPrefixV1="the worktree code does not start";

/** 🪂️ Tells whether a fallback reason means that the code of the worktree failed to start, so that this code is to be refused. */
export function blamesStartV1(reason:string):boolean{return reason.startsWith(failedStartPrefixV1);}

/** 🐕️ Tells whether the watchdog ended an earlier daemon of the code that runs now while that code was new: the code first ran at `firstRunMs`, the boot stage ended a blocked daemon at `endedMs`, and that was after the first run and within `windowMs` of it. A code that ran longer than the window without such an end is not blamed. */
export function blamesWatchdogV1(input:Readonly<{firstRunMs:number|undefined;endedMs:number|undefined;windowMs:number}>):boolean{
 return input.firstRunMs!==undefined&&input.endedMs!==undefined&&input.endedMs>=input.firstRunMs&&input.endedMs-input.firstRunMs<=input.windowMs;
}

/** 🧪️ Tells whether a failed self-check blames the code: a check that timed out, was cancelled or could not start says something about this machine (load, a missing runtime), not about the code, so it is not published as a refusal. */
export function blamesCheckV1(reason:string):boolean{return !/^self-check (timed out|cancelled|could not start)/u.test(reason);}

/** 🕰️ The window in which a watchdog end blames the new code: three minutes after its first run (the canary window: a code that cannot run blocks at once, a block after minutes is as likely the load or the data, e.g. the apply of a multi-gigabyte pack, and must not refuse the digest of a fleet that runs it). */
export const watchdogWindowMsV1=180_000;

/** ↩️ Withdraws a refusal this origin published: the origin runs the digest and the digest has proven itself (see `refusalProofMsV1`), so the record in `origins/<origin>/refused.json` is removed. Returns whether a record was removed. */
export async function withdrawRefusalV1(storeRoot:string,originId:string,digest:string):Promise<boolean>{
 const before=await readRefusalsOfV1(storeRoot,originId)??[],refused=before.filter(item=>item.digest!==digest);
 if(refused.length===before.length)return false;
 await replaceOriginFileV1(storeRoot,originId,"refused.json",JSON.stringify({version:1,originId,refused}));
 return true;
}

/** 🧾️ How long a daemon must have run a digest, without a watchdog end since it started, before it withdraws its own refusal of that digest: ten minutes. */
export const refusalProofMsV1=600_000;
