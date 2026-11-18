import{createHash}from"node:crypto";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import type{GraphStateV1}from"../🟦️.ts";
import{fetchGraphEntryV1,publishGraphEntryV1,type GraphFleetClientV1}from"../🛰️exchange/🟦️.ts";
import{platformClassV1,splitRequestV1}from"../🌍️platform-class/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 📏️ The measured numbers a read decides by: the memory of a base read and of one wave, the wave size, how many waves one machine may run, the bounded wait and the age of a request. See `./🧬️schema/🔣️.json`. */
export const wantedPolicyV1=Object.freeze({...schema.definitions.Policy.const});
export type WantedPolicyV1=typeof wantedPolicyV1;
export type WantedRequestV1=Readonly<{version:1;requester:string;platform:string;graphKey:string;projects:readonly string[];heavy?:readonly string[];left?:readonly string[];createdMs:number;expiresMs:number}>;
export type WantedPlanV1=Readonly<{mode:"local"|"fleet";waveProjects:number;localProjects:number;takenProjects:number;wantedProjects:number}>;
const ZERO="0".repeat(64),sha=(text:string)=>createHash("sha256").update(text).digest("hex");

/** 🧮️ How many projects one wave may hold in `admittedMegabytes` (at most the declared wave, zero below the useful minimum) and how many projects the machine infers before it asks the fleet: the waves it may run in the time the wait allows. */
export function inferableProjectsV1(admittedMegabytes:number,policy:WantedPolicyV1=wantedPolicyV1):Readonly<{waveProjects:number;projects:number}>{
 const perProject=(policy.waveMegabytes-policy.baseMegabytes)/policy.waveProjects,fit=Number.isFinite(admittedMegabytes)?Math.floor((admittedMegabytes-policy.baseMegabytes)/perProject):0,wave=Math.min(policy.waveProjects,Math.max(0,fit));
 return wave<policy.minimumWaveProjects?{waveProjects:0,projects:0}:{waveProjects:wave,projects:wave*policy.maximumLocalWaves};
}

/** 🧭️ The decision of a read that found `stale` projects of which the fleet already had `taken`: infer the rest here when the admitted memory carries it and none of them is `heavy` (too heavy for this machine whatever their number), else ask the fleet for all the rest and infer here only what still fits afterwards. */
export function planStaleReadV1(input:Readonly<{stale:number;taken:number;admittedMegabytes:number;heavy?:number}>,policy:WantedPolicyV1=wantedPolicyV1):WantedPlanV1{
 const rest=Math.max(0,input.stale-input.taken),fit=inferableProjectsV1(input.admittedMegabytes,policy);
 return rest<=fit.projects&&(input.heavy??0)===0?{mode:"local",waveProjects:fit.waveProjects,localProjects:rest,takenProjects:input.taken,wantedProjects:0}:{mode:"fleet",waveProjects:fit.waveProjects,localProjects:fit.projects,takenProjects:input.taken,wantedProjects:Math.min(rest,policy.maximumProjects)};
}

/** 🔑️ The exchange state a machine's request lives under: one slot per requester, whatever its platform (the request carries the platform), so a newer request replaces the older one and every serving machine finds it by the requester alone; a slot per platform was read by machines of one platform only, and nobody read the slot of a Linux worker among macOS machines. */
export function wantedStateV1(requester:string):GraphStateV1{
 return Object.freeze({version:1,frame:Object.freeze({kind:"project-inference" as const,tooling:sha("project-wanted"),implementation:sha(requester),platform:"neutral",extra:Object.freeze({wanted:"1"})}),names:ZERO,content:ZERO,files:0,contentFiles:0,digest:sha(JSON.stringify(["project-wanted",requester]))}) as GraphStateV1;
}

/** 🙋️ Publishes the request: the projects of the graph state `graphKey` this machine cannot infer. False when the fleet cannot take it (the caller waits for nothing then). A request is idempotent: publishing again with the `createdMs` of the first one renews it (a later `expiresMs`, a subset of the projects) and stays the same piece of work for the serving machine, so its claim and its running read are kept. */
export async function publishWantedV1(input:Readonly<{client:GraphFleetClientV1;requester:string;platform:string;graphKey:string;projects:readonly string[];heavy?:readonly string[];createdMs?:number;now?:()=>number;policy?:WantedPolicyV1}>):Promise<boolean>{
 const policy=input.policy??wantedPolicyV1,now=(input.now??Date.now)(),request:WantedRequestV1={version:1,requester:input.requester,platform:input.platform,graphKey:input.graphKey,projects:[...input.projects].sort().slice(0,policy.maximumProjects),...input.heavy===undefined||input.heavy.length===0?{}:{heavy:[...input.heavy].sort().slice(0,policy.maximumProjects)},createdMs:input.createdMs??now,expiresMs:now+policy.ttlMilliseconds};
 return publishGraphEntryV1({client:input.client,state:wantedStateV1(input.requester),owner:input.requester,documents:[{name:"wanted",bytes:Buffer.from(JSON.stringify(request))}]});
}

/** 👀️ The live request of one requester, or undefined when there is none, it expired, it is damaged or the fleet cannot answer. */
export async function readWantedV1(input:Readonly<{client:GraphFleetClientV1;requester:string;now?:()=>number}>):Promise<WantedRequestV1|undefined>{
 const found=await fetchGraphEntryV1(input.client,wantedStateV1(input.requester));
 if(found===undefined||found==="fault")return undefined;
 try{
  const document=found.documents.find(row=>row.name==="wanted");if(document===undefined)return undefined;
  const request=JSON.parse(Buffer.from(document.bytes).toString("utf8")) as WantedRequestV1;
  if(validateJsonSchemaSubset({$ref:"#/definitions/WantedRequest"},request,schema).length)return undefined;
  return request.requester===input.requester&&request.expiresMs>(input.now??Date.now)()?request:undefined;
 }catch{return undefined;}
}

export type WantedProgressV1=Readonly<{arrived:number;wanted:number;waitedMilliseconds:number}>;
export type WantedEndV1="silent"|"stalled"|"limit";
/** ⏳️ Waits for the records of `projects`, taking each as soon as the fleet has it: `take(name)` answers whether the record is now installed here. The wait is bound by the absence of progress, not by a fixed time: progress is a record that arrived or a change of `progressSign()` (the time the serving machine says its work advanced: a wave published, a project fresh), and the wait ends (cause `stalled`) when none was seen for `waitMilliseconds`, when the server showed no sign of life (`alive()` answers the time of its newest answer: the claim is alive) for `silenceMilliseconds` (cause `silent`: no machine holds the request), or after `waitMaximumMilliseconds` altogether (cause `limit`, a safety net against a server that reports progress for ever); `end(cause)` names which. While it waits it calls `renew(missing)` every `renewMilliseconds`, so the request lives as long as the wait does. Every round reports progress, and it stops at once on cancellation or when `settled(missing)` (the names still missing) says the rest is little enough for the caller. Returns the projects still missing. */
export async function awaitWantedV1(input:Readonly<{projects:readonly string[];take(name:string):Promise<boolean>;alive?():number|undefined;progressSign?():number|undefined;renew?(missing:readonly string[]):Promise<void>;end?(cause:WantedEndV1):void;signal?:AbortSignal;progress?(progress:WantedProgressV1):void;settled?(missing:readonly string[]):boolean;sleep?(milliseconds:number):Promise<void>;now?:()=>number;policy?:WantedPolicyV1}>):Promise<readonly string[]>{
 const policy=input.policy??wantedPolicyV1,now=input.now??Date.now,sleep=input.sleep??(milliseconds=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds))),started=now();
 let missing=[...input.projects],best=0,progressAt=started,aliveAt=started,renewedAt=started,heard:number|undefined,moved:number|undefined;
 for(;;){
  const still:string[]=[];
  for(const name of missing){input.signal?.throwIfAborted();if(!await input.take(name).catch(()=>false))still.push(name);}
  missing=still;
  const arrived=input.projects.length-missing.length,sign=input.alive?.(),step=input.progressSign?.();
  if(arrived>best){best=arrived;progressAt=now();aliveAt=progressAt;}
  if(sign!==undefined&&sign!==heard){heard=sign;aliveAt=now();}
  if(step!==undefined&&step!==moved){moved=step;progressAt=now();}
  input.progress?.({arrived,wanted:input.projects.length,waitedMilliseconds:now()-started});
  const at=now(),cause:WantedEndV1|undefined=at-progressAt>=policy.waitMilliseconds?"stalled":at-aliveAt>=policy.silenceMilliseconds?"silent":at-started>=policy.waitMaximumMilliseconds?"limit":undefined;
  if(!missing.length||input.settled?.(missing)===true)return missing;
  if(cause!==undefined){input.end?.(cause);return missing;}
  if(input.renew!==undefined&&at-renewedAt>=policy.renewMilliseconds){renewedAt=at;await input.renew(missing).catch(()=>undefined);}
  input.signal?.throwIfAborted();
  await sleep(policy.pollMilliseconds);
 }
}

export type WantedHeadroomV1=Readonly<{freeMegabytes:number;pressure:"normal"|"warn"|"critical"}>;
/** 🔑️ The claim key of one request: requester, graph state and creation time, so a newer request of the same requester is a new piece of work. */
export const wantedClaimKeyV1=(request:WantedRequestV1):string=>createHash("sha256").update(JSON.stringify(["serve",request.requester,request.graphKey,request.createdMs])).digest("hex");

/** 🙋️ The live requests of the other machines that this machine takes over: for each requester of the fleet (never itself), the live request of a machine of any platform class (a request of another class is served except its bound owners: `cross` is told, the taken request holds the served projects and names the left ones in `left`; a request of another class that holds nothing but bound owners goes to `foreign` and is left to a machine of that class) of the same graph state (when `graphKey` is given) whose claim this machine gets and has not got yet (`known` says which it holds). Nobody else serves a request this machine holds, and a request another machine holds is left to it. */
export async function claimWantedV1(input:Readonly<{client:GraphFleetClientV1;owner:string;platform:string;requesters:readonly string[];known(key:string):boolean;graphKey?():Promise<string>;foreign?(request:WantedRequestV1):void;cross?(request:WantedRequestV1,split:Readonly<{served:readonly string[];left:readonly string[]}>):void;live?(request:WantedRequestV1):void;signal?:AbortSignal;now?:()=>number;policy?:WantedPolicyV1}>):Promise<readonly WantedRequestV1[]>{
 const policy=input.policy??wantedPolicyV1,taken:WantedRequestV1[]=[];
 for(const requester of input.requesters){
  input.signal?.throwIfAborted();
  if(requester===input.owner)continue;
  const request=await readWantedV1({client:input.client,requester,...(input.now===undefined?{}:{now:input.now})});
  if(request===undefined)continue;
  const split=splitRequestV1(request.projects,input.platform,request.platform);
  if(split.served.length===0){input.foreign?.(request);continue;}
  if(input.graphKey!==undefined&&request.graphKey!==await input.graphKey())continue;
  const mine=split.left.length===0?request:{...request,projects:split.served,left:split.left};
  input.live?.(mine);
  const key=wantedClaimKeyV1(request);
  if(input.known(key))continue;
  if((await input.client.claim(key,input.owner,policy.claimTtlMilliseconds,policy.claimExpectedMilliseconds)).state==="granted"){taken.push(mine);if(platformClassV1(input.platform)!==platformClassV1(request.platform))input.cross?.(mine,split);}
 }
 return taken;
}

export type WantedServedV1=Readonly<{version:1;requester:string;platform:string;server:string;projects:readonly string[];left?:readonly string[];generation:number|null;atMs:number;total?:number;progressAtMs?:number;remainingMilliseconds?:number|null}>;
export type WantedProgressRecordV1=Readonly<{total:number;progressAtMs:number;remainingMilliseconds:number|null}>;

/** 🗣️ A duration the way a waiting person reads it: unknown, under a minute, minutes, hours and minutes. */
export function wantedEstimateTextV1(milliseconds:number|null|undefined,language:"en"|"de"):string{
 if(milliseconds===null||milliseconds===undefined)return language==="en"?"unknown":"unbekannt";
 const minutes=Math.ceil(milliseconds/60000),about=language==="en"?"about":"etwa";
 return minutes<=0?(language==="en"?"under a minute":"unter einer Minute"):minutes<60?`${about} ${minutes} min`:`${about} ${Math.floor(minutes/60)} h ${minutes%60} min`;
}
/** 🔑️ The exchange state of the answer to one requester, whatever its platform: the projects the serving machine finished for it, replaced by every newer answer. */
export function servedStateV1(requester:string):GraphStateV1{
 return Object.freeze({version:1,frame:Object.freeze({kind:"project-inference" as const,tooling:sha("project-served"),implementation:sha(requester),platform:"neutral",extra:Object.freeze({served:"1"})}),names:ZERO,content:ZERO,files:0,contentFiles:0,digest:sha(JSON.stringify(["project-served",requester]))}) as GraphStateV1;
}

/** 📣️ Tells the requester which of its projects are done (their records and the generation are in the fleet cache) and, with `progress`, how its request stands: how many projects it holds, when the server's work last advanced and how long the rest is estimated to take (null when nothing measured says), so the requester keeps waiting while this advances. False when the fleet cannot take it. */
export async function publishServedV1(input:Readonly<{client:GraphFleetClientV1;server:string;request:WantedRequestV1;projects:readonly string[];generation:number|null;progress?:WantedProgressRecordV1;now?:()=>number}>):Promise<boolean>{
 const served:WantedServedV1={version:1,requester:input.request.requester,platform:input.request.platform,server:input.server,projects:[...input.projects].sort(),...input.request.left===undefined||input.request.left.length===0?{}:{left:[...input.request.left].sort()},generation:input.generation,atMs:(input.now??Date.now)(),...input.progress===undefined?{}:input.progress};
 return publishGraphEntryV1({client:input.client,state:servedStateV1(input.request.requester),owner:input.server,head:true,documents:[{name:"served",bytes:Buffer.from(JSON.stringify(served))}]});
}

/** 👂️ The newest answer to this requester, or undefined when there is none, it is damaged or the fleet cannot answer. */
export async function readServedV1(input:Readonly<{client:GraphFleetClientV1;requester:string}>):Promise<WantedServedV1|undefined>{
 const found=await fetchGraphEntryV1(input.client,servedStateV1(input.requester));
 if(found===undefined||found==="fault")return undefined;
 try{
  const document=found.documents.find(row=>row.name==="served");if(document===undefined)return undefined;
  const served=JSON.parse(Buffer.from(document.bytes).toString("utf8")) as WantedServedV1;
  if(validateJsonSchemaSubset({$ref:"#/definitions/Served"},served,schema).length)return undefined;
  return served.requester===input.requester?served:undefined;
 }catch{return undefined;}
}

/** 🎚️ The memory a machine can run inferences in (the waves): half of what it CAN give, its memory less the floor the others keep, and never what is free at this moment nor the small first booking of the process that asks. A momentary reading says nothing about what the machine can do (measured: 464 MB spare while test runs held memory made a 16 GB worker "infer only 0 of 674 projects" and exit 75): the inference itself is admitted by the capacity gate, which queues it with progress until the memory is there, and a project that outgrows its grant is stopped by the memory governor. A machine refuses only when it can never fit one wave. */
export function fleetAdmittedMegabytesV1(input:Readonly<{hostMegabytes:number}>,policy:WantedPolicyV1=wantedPolicyV1):number{
 return Math.max(0,Math.min(Math.floor(input.hostMegabytes/2),Math.floor(input.hostMegabytes-policy.availableFloorMegabytes)));
}

/** 🎁️ The memory the per-project grant is judged by: the share of the machine (half of its memory, never a serving machine's headroom) as long as the machine can ever run a wave, else what it is admitted (0 MB per project). Like {@link fleetAdmittedMegabytesV1} it never depends on the free memory of the moment. */
export function fleetGrantMegabytesV1(input:Readonly<{hostMegabytes:number}>,policy:WantedPolicyV1=wantedPolicyV1):number{
 const admitted=fleetAdmittedMegabytesV1(input,policy);
 return inferableProjectsV1(admitted,policy).waveProjects>0?Math.min(Math.floor(input.hostMegabytes/2),policy.serveHeadroomMegabytes-1):admitted;
}

/** 💾️ What a serving machine keeps between two processes of its graph server (a restart for new code must not lose them): the requests it holds with the projects it already told the requesters about, the claims it completed and whether a read was running. See `ServeState` in the schema. */
export type ServeStateV1=Readonly<{version:1;owner:string;platform:string;savedMs:number;held:readonly Readonly<{request:WantedRequestV1;list:readonly string[];told:boolean;toldAt:number;heldAt:number}>[];told:readonly string[];reading:"none"|"request"|"refresh"}>;

/** 🔭️ What the detached runner of a graph read keeps in `read-record.json` (see `ReadRecord` in the schema): who runs the read, since when, when the runner last wrote, where the output is and, once the read ended, its exit status. */
export type ReadRecordV1=Readonly<{version:1;reason:"request"|"refresh";runner:number;read:number|null;startedMs:number;beatMs:number;log:string;status:number|null;endedMs:number|null}>;

/** 📼️ The record of a graph read if it has the shape of the schema, else undefined. */
export function readReadRecordV1(value:unknown):ReadRecordV1|undefined{
 return validateJsonSchemaSubset({$ref:"#/definitions/ReadRecord"},value,schema).length===0?value as ReadRecordV1:undefined;
}

/** 📼️ The state if it has the shape of the schema, else undefined. */
export function readServeStateV1(value:unknown):ServeStateV1|undefined{
 return validateJsonSchemaSubset({$ref:"#/definitions/ServeState"},value,schema).length===0?value as ServeStateV1:undefined;
}

/** 🗣️ One message of the texts table in one language. */
export function wantedTextV1(key:keyof typeof schema.definitions.Texts.const.en,language:"en"|"de",fields:Readonly<Record<string,string|number>>={}):string{
 return schema.definitions.Texts.const[language][key].replace(/\{(\w+)\}/gu,(_all,name:string)=>String(fields[name]??""));
}
