import {createHash} from "node:crypto";
import {fetchGraphEntryV1,publishGraphEntryOutcomeV1,publishGraphEntryV1,tryGraphEntryV1,type GraphDocumentV1,type GraphFleetClientV1,type GraphPutTimeoutV1,type GraphRefusalV1} from "../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import type {GraphStateV1} from "../../../../🪪️graph-state/🟦️.ts";
import type {ProjectClaimV1} from "../../../../🪪️graph-state/📦️project-records/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The authored numbers of the pool: poll intervals, retry delays, claim windows, how many child processes a machine runs and how much memory each may hold.
 * @see ./🧬️schema/🔣️.json */
export const OWNER_POOL_POLICY_V1=schema.definitions.Policy.const;
const P=OWNER_POOL_POLICY_V1,ZERO="0".repeat(64),sha=(text:string)=>createHash("sha256").update(text).digest("hex");

/** 🏷️ One owner of the pool as every machine reads it: the name, the kind it is filed under, how long it is expected to run (the claim lives that long), its `tier` (0 or none: the reading machine waits for it now, selection and priority owners first; 1: it is one of the next owners of the same read, inferred ahead while no tier 0 owner is open) and its project JSON. */
export type OwnerPoolUnitV1=Readonly<{name:string;kind:string;expectedMilliseconds:number;megabytes?:number;tier?:number;admission?:Readonly<{targets?:readonly string[];inputs:readonly string[];generated?:unknown}>;json:Readonly<Record<string,unknown>>}>;
/** 🧾️ What a puller needs besides the units: see the schema definition `Head`. */
export type OwnerPoolHeadV1=Readonly<{version:1;revision?:string;priority?:number;id:string;createdMs:number;publisher:string;identity:string;globals:Readonly<{registryDigest:string;commandRevision:string;late:boolean}>}>;
/** 🥇️ The order in which a machine serves the open waves of the fleet: the wave of the lowest `priority` first (a read a command waits for before a background read of the whole graph), then the wave with the fewest children of this machine, then the newest.
 * @see ./🧪️tests/🧭️fleet-schedule/🟦️.ts */
export const ownerPoolWaveOrderV1=(a:Readonly<{priority?:number;children:number;createdMs:number}>,b:Readonly<{priority?:number;children:number;createdMs:number}>):number=>(a.priority??0)-(b.priority??0)||a.children-b.children||b.createdMs-a.createdMs;
/** 📦️ The published pool of one wave: the head, the session the units are inferred in (opaque bytes of the publisher's runtime) and the units in the order the publisher infers them. */
export type OwnerPoolManifestV1=Readonly<{head:OwnerPoolHeadV1;session:Uint8Array;units:readonly OwnerPoolUnitV1[]}>;
/** 📍️ One open wave as the pointer lists it. */
export type OwnerPoolWaveV1=Readonly<{id:string;createdMs:number;units:number}>;
/** 📍️ The tiny entry machines poll: every open wave, newest last. */
export type OwnerPoolPointerV1=Readonly<{version:2;waves:readonly OwnerPoolWaveV1[]}>;

const stateOf=(name:string,id:string|undefined):GraphStateV1=>Object.freeze({version:1,frame:Object.freeze({kind:"project-inference" as const,tooling:sha(P.domain+":"+name+(id===undefined?"":":"+id)),implementation:ZERO,platform:"neutral",extra:Object.freeze({})}),names:ZERO,content:ZERO,files:0,contentFiles:0,digest:sha(JSON.stringify([P.domain,name,id??null]))}) as unknown as GraphStateV1;

/** 🧮️ The state a pointer, a manifest or a failure note is stored under: the pointer has one fixed state (a swap replaces it), a manifest one per wave id, a note one per record state, and every one of them its own retention slot (the slot of a state is made of its kind, platform and tooling): with one slot for all manifests the cache kept only the newest wave and collected every older one ten minutes after a newer wave was published, so a wave of 4774 owners was gone as soon as any machine published a wave of 75 and no puller could open it again. */
export const ownerPoolStateV1=(name:"pointer"|"manifest",id?:string):GraphStateV1=>stateOf(name,name==="pointer"?undefined:id);

/** 🆔️ The id of a wave: the digest of its head parts and the set of its owners, each as its name and the digest of its project JSON, in name order, so two machines (or one reader after another landing that only reorders, re-prices or re-sorts the owners) that publish the same owners publish the same entry and its counters continue; one changed owner changes one row. */
export const ownerPoolIdV1=(head:Omit<OwnerPoolHeadV1,"id"|"createdMs"|"publisher">,units:readonly OwnerPoolUnitV1[])=>sha(JSON.stringify([head.identity,head.revision??null,head.globals,units.map(unit=>[unit.name,sha(JSON.stringify(unit.admission===undefined?unit.json:[unit.json,unit.admission]))]).sort((a,b)=>a[0]!<b[0]!?-1:a[0]!>b[0]!?1:0)]));

const documentOf=(documents:readonly GraphDocumentV1[],name:string)=>documents.find(row=>row.name===name)?.bytes;
const encode=(value:unknown)=>Buffer.from(JSON.stringify(value));

/** 🧮️ The state a failure note is stored under: one per record state of the owner (the digest the claim is made by). */
export const ownerPoolFailureStateV1=(recordDigest:string):GraphStateV1=>stateOf(P.failureName,recordDigest);

/** 🚫️ Tells the fleet that an owner failed with a deterministic cause on this machine, so the other pullers leave it to the machine that runs the wave. A live note of the same record state and revision is never written again: the same cause writes nothing, another cause is added to the note (`causes`, at most four, the first stays `cause`) while its origin and time stay those of the first failure, so the note neither flaps between machines nor outlives its first failure and every cause stays visible. Never throws. */
export async function publishOwnerPoolFailureV1(input:Readonly<{client:GraphFleetClientV1;owner:string;origin:string;recordDigest:string;revision:string;cause:string;now?:()=>number}>):Promise<boolean>{
 try{
  const now=input.now??Date.now,cause=input.cause.slice(0,300),live=await readOwnerPoolFailureV1(input.client,input.recordDigest,input.revision,now);
  if(live!==undefined&&live.causes.includes(cause))return true;
  const note=live===undefined?{cause,causes:[cause],origin:input.origin,atMs:now()}:{cause:live.cause,causes:[...live.causes,cause].slice(0,4),origin:live.origin,atMs:live.atMs};
  return await publishGraphEntryV1({client:input.client,state:ownerPoolFailureStateV1(input.recordDigest),owner:input.owner,documents:[{name:P.failureName,bytes:encode({version:1,revision:input.revision,...note})}]});
 }catch{return false;}
}

/** 🔎️ The failure note of a record state if it is younger than the policy's maximum age and of the same record-code revision. */
export async function readOwnerPoolFailureV1(client:GraphFleetClientV1,recordDigest:string,revision:string,now:()=>number=Date.now):Promise<Readonly<{cause:string;causes:readonly string[];origin:string;atMs:number}>|undefined>{
 try{
  const entry=await fetchGraphEntryV1(client,ownerPoolFailureStateV1(recordDigest));
  if(entry===undefined||entry==="fault")return undefined;
  const bytes=documentOf(entry.documents,P.failureName);if(bytes===undefined)return undefined;
  const note=JSON.parse(Buffer.from(bytes).toString("utf8")) as {version:number;revision:string;cause:string;causes?:string[];origin:string;atMs:number};
  return note.version===1&&note.revision===revision&&now()-note.atMs<=P.failureMaximumAgeMilliseconds?{cause:note.cause,causes:note.causes??[note.cause],origin:note.origin,atMs:note.atMs}:undefined;
 }catch{return undefined;}
}

const readPointerRaw=async(client:GraphFleetClientV1):Promise<Readonly<{pointer?:OwnerPoolPointerV1;expected:string}>>=>{
 let expected="none";
 const entry=await fetchGraphEntryV1(client,ownerPoolStateV1("pointer"),undefined,undefined,undefined,undefined,digest=>{expected=digest;});
 if(entry==="fault")throw new Error("the fleet cache did not answer");
 if(entry===undefined)return {expected};
 const bytes=documentOf(entry.documents,P.pointerName);
 if(bytes===undefined)return {expected};
 const pointer=JSON.parse(Buffer.from(bytes).toString("utf8")) as OwnerPoolPointerV1;
 return pointer.version===2&&Array.isArray(pointer.waves)?{pointer,expected}:{expected};
};

/** 🧷️ Whether a pool that holds a wave keeps it when the pointer is read: a wave the pointer lists is kept, every wave is kept while the pointer lists none (the last known wave: the pointer forgets a wave after `manifestMaximumAgeMilliseconds`, a machine whose wave expired or was never republished must keep working what it knows), a wave is dropped only when the pointer lists other waves, or when the revision of its children declined it, and only while no child works on it.
 * @see ./🧪️tests/🟦️.ts */
export const ownerPoolKeepWaveV1=(input:Readonly<{listed:boolean;anyListed:boolean;children:number;declined:boolean}>):boolean=>input.children>0||input.listed||!input.declined&&!input.anyListed;

/** 🕰️ How long a finished wave rests before the pool looks at it again: `reopenMilliseconds`, and `staleReopenMilliseconds` once the pointer would no longer list it (an old wave is mostly stored, a look costs claims).
 * @see ./🧪️tests/🟦️.ts */
export const ownerPoolReopenMillisecondsV1=(createdMs:number,now:number,policy:typeof P=P):number=>now-createdMs>policy.manifestMaximumAgeMilliseconds?policy.staleReopenMilliseconds:policy.reopenMilliseconds;

/** 🧬️ Whether a process that runs record-code revision `mine` can work a wave published under `waveRevision`: a wave without a revision, or a process that does not know its own yet, is accepted (the child decides when it opens the wave).
 * @see ./🧪️tests/🌊️early-wave/🟦️.ts */
export const ownerPoolAcceptsRevisionV1=(waveRevision:string|undefined,mine:string|undefined):boolean=>waveRevision===undefined||mine===undefined||waveRevision===mine;

/** 📣️ Whether a reader publishes its wave now: the first time as soon as `announceMinimumOwners` owners are open (the unit list is known, nothing is inferred yet), later only when the open owners shrank by `republishShrinkBasisPoints` of what the last wave listed and `republishMinimumMilliseconds` passed (a manifest carries the whole session, so republishing is paid for). `published` is the owner count of the last wave, `undefined` when none was published yet.
 * @see ./🧪️tests/🌊️early-wave/🟦️.ts */
export const ownerPoolRepublishV1=(input:Readonly<{published:number|undefined;count:number;sinceMs:number}>,policy:typeof P=P):boolean=>input.published===undefined?input.count>=policy.announceMinimumOwners:input.count<=input.published*(1-policy.republishShrinkBasisPoints/10000)&&input.sinceMs>=policy.republishMinimumMilliseconds;

/** 🫀️ Whether a reader that has published a wave puts it on the pointer again although nothing else changed: the wave is listed for `manifestMaximumAgeMilliseconds` from its creation and a pointer write can be lost, so a read that runs long keeps its wave listed every `republishKeepAliveMilliseconds`. `sinceMs` is the time since the last publication or attempt.
 * @see ./🧪️tests/🔔️keep-alive/🟦️.ts */
export const ownerPoolKeepAliveV1=(input:Readonly<{published:number|undefined;sinceMs:number}>,policy:typeof P=P):boolean=>input.published!==undefined&&input.sinceMs>=policy.republishKeepAliveMilliseconds;

/** 💤️ Why a pool that holds waves works none: the key of the schema's `Status.reason`, `null` when it works or nothing needs saying. A target of the capacity gate that says why (waiting, yielding) is passed through while it holds the pool where it is or below (a target above the children that run is a pool that is starting, whatever bound the gate names); no wave at all is `no-wave`; waves that are all declined or of another revision are `no-current-wave`; waves that rest are `waves-done`, waves the children could not open `waves-backoff`; open waves with a target of zero children are `nothing-fits`.
 * @see ./🧪️tests/🌊️early-wave/🟦️.ts */
export function ownerPoolWhyIdleV1(input:Readonly<{waves:readonly Readonly<{state:string;backoffUntil:number;acceptable:boolean}>[];open:number;children:number;targetChildren:number;targetReason:string|null;now:number}>):string|null{
 if(input.targetReason!==null&&input.targetChildren<=input.children)return input.targetReason;
 if(input.children>0)return null;
 if(input.waves.length===0)return"no-wave";
 if(input.open===0)return input.waves.every(wave=>wave.state==="declined"||!wave.acceptable)?"no-current-wave":input.waves.some(wave=>wave.backoffUntil>input.now)?"waves-backoff":"waves-done";
 return input.targetChildren===0?"nothing-fits":null;
}

/** 🧺️ The waves of a pointer that are still open at `now`: not older than the policy's maximum age, at most `wavesKept` of the newest, equal ids once, oldest first. */
export function ownerPoolOpenWavesV1(waves:readonly OwnerPoolWaveV1[],now:number,policy:typeof P=P):readonly OwnerPoolWaveV1[]{
 const seen=new Set<string>();
 return waves.filter(wave=>/^[0-9a-f]{64}$/u.test(wave.id)&&now-wave.createdMs<=policy.manifestMaximumAgeMilliseconds).sort((a,b)=>a.createdMs-b.createdMs||(a.id<b.id?-1:1)).reverse().filter(wave=>!seen.has(wave.id)&&seen.add(wave.id)).slice(0,policy.wavesKept).reverse();
}

/** 🧾️ The stage and closed cache refusal of a wave offer. */
export type OwnerPoolPublishRefusalV1=Readonly<{stage:(typeof schema.definitions.PublishStage.enum)[number];code:GraphRefusalV1["code"]|"unavailable";status:number|null;transient:boolean;detail?:string;timeout?:GraphPutTimeoutV1}>;

/** 🧾️ How the cache server opens the capacity detail of its 507 (the only response text a refusal log carries: the server writes it itself, as bytes, limits and holders, never a request text). */
const CAPACITY_DETAIL_PREFIX="Insufficient storage: ";

const WRITTEN=new Set<string>();

/** ⬆️ Publishes the pool of a wave to the fleet cache: the manifest first, then the pointer with this wave added to the waves that are open (a swap against the pointer read just before, so a wave of another machine is kept and nobody replaces anybody; the id of a wave is the digest of its owners, so a manifest that is stored already is verified and never written a second time by another machine: its head, session and cost sheet are the first writer's, a second write would only be refused as a divergence after its upload, and machines that publish the same owners at once take turns by the claim of the manifest state; a machine that wrote the manifest itself renews it by writing it again), so a puller that sees the pointer finds the manifest and a new wave never makes a puller drop the wave it is working on. `false` when the fleet refuses (nothing is lost: the publisher infers every owner itself). */
export async function publishOwnerPoolV1(input:Readonly<{client:GraphFleetClientV1;owner:string;manifest:OwnerPoolManifestV1;now?:()=>number;onRefused?:(refusal:OwnerPoolPublishRefusalV1)=>void}>):Promise<boolean>{
 const {client,owner,manifest}=input,{head}=manifest,now=(input.now??Date.now)();
 const refused=(stage:OwnerPoolPublishRefusalV1["stage"],why?:GraphRefusalV1,timeout?:GraphPutTimeoutV1)=>{
  const evidence=why?.code==="no-answer"&&timeout!==undefined&&(timeout.request==="cas-put"||timeout.request==="action-result-put")&&Number.isSafeInteger(timeout.deadlineMilliseconds)&&timeout.deadlineMilliseconds>0;
  input.onRefused?.({stage,code:why?.code??"unavailable",status:why?.status??null,transient:why?.transient??true,...(why?.code==="capacity"&&why.detail.startsWith(CAPACITY_DETAIL_PREFIX)?{detail:why.detail.slice(CAPACITY_DETAIL_PREFIX.length)}:{}),...(evidence?{timeout:{request:timeout.request,deadlineMilliseconds:timeout.deadlineMilliseconds}}:{})});
  return false;
 };
 const state=ownerPoolStateV1("manifest",head.id),site=client.settings.endpoint+"\u0000"+client.settings.instance+"\u0000"+owner+"\u0000"+head.id,intact=async():Promise<boolean>=>{const present=await readOwnerPoolV1(client,head.id);return present!==undefined&&ownerPoolIdV1(present.head,present.units)===head.id;};
 let hold:Awaited<ReturnType<typeof tryGraphEntryV1>>|undefined,found=false;
 if(!WRITTEN.has(site)){
  hold=await tryGraphEntryV1({client,state,owner,expectedMilliseconds:P.manifestExpectedMilliseconds});
  for(let wait=0;hold.outcome==="held"&&!found&&wait<P.manifestWaitPolls;wait++){await new Promise<void>(done=>setTimeout(done,P.manifestPollMilliseconds));found=await intact();}
  if(hold.outcome==="stored")found=await intact();
 }
 if(!found){
  const stored=await publishGraphEntryOutcomeV1({client,state,owner,...hold?.outcome==="granted"?{hold:hold.hold}:{},documents:[{name:P.headName,bytes:encode(head)},{name:P.sessionName,bytes:manifest.session},{name:P.unitsName,bytes:encode(manifest.units)}]});
  if(!stored.stored){
   if(stored.refusal?.status!==409||ownerPoolIdV1(head,manifest.units)!==head.id)return refused("manifest",stored.refusal,stored.timeout);
   if(!await intact())return refused("manifest-verification",stored.refusal);
  }
 }
 WRITTEN.add(site);if(WRITTEN.size>P.wavesKept*8)WRITTEN.delete(WRITTEN.values().next().value as string);
 for(let attempt=0;attempt<4;attempt++){
  let current:Awaited<ReturnType<typeof readPointerRaw>>;
  try{current=await readPointerRaw(client);}catch{return refused("pointer-read");}
  const waves=ownerPoolOpenWavesV1([...(current.pointer?.waves??[]).filter(wave=>wave.id!==head.id),{id:head.id,createdMs:head.createdMs,units:manifest.units.length}],now);
  const result=await publishGraphEntryOutcomeV1({client,state:ownerPoolStateV1("pointer"),owner,head:true,expectedHead:current.expected,documents:[{name:P.pointerName,bytes:encode({version:2,waves})}]});
  if(result.stored)return true;
  if(result.refusal?.status!==409||attempt===3)return refused("pointer-write",result.refusal);
 }
 return refused("pointer-write");
}

/** 📍️ The waves that are open, `undefined` when none is published or every pointer entry is older than the policy's maximum age, and `"fault"` when the fleet cache did not answer: a fault says nothing about the waves, so a puller that gets it keeps what it has (a busy cache server answered one request in five with a fault, and a puller that read "no wave" from it dropped its wave and its session each time). */
export async function readOwnerPoolPointerV1(client:GraphFleetClientV1,now:()=>number=Date.now):Promise<OwnerPoolPointerV1|undefined|"fault">{
 try{
  const {pointer}=await readPointerRaw(client);if(pointer===undefined)return undefined;
  const waves=ownerPoolOpenWavesV1(pointer.waves,now());
  return waves.length===0?undefined:{version:2,waves};
 }catch{return "fault";}
}

/** ⬇️ The manifest of one wave, or `undefined` when it is not (yet) readable. */
export async function readOwnerPoolV1(client:GraphFleetClientV1,id:string):Promise<OwnerPoolManifestV1|undefined>{
 try{
  const entry=await fetchGraphEntryV1(client,ownerPoolStateV1("manifest",id));
  if(entry===undefined||entry==="fault")return undefined;
  const head=documentOf(entry.documents,P.headName),session=documentOf(entry.documents,P.sessionName),units=documentOf(entry.documents,P.unitsName);
  if(head===undefined||session===undefined||units===undefined)return undefined;
  const parsed=JSON.parse(Buffer.from(head).toString("utf8")) as OwnerPoolHeadV1;
  return parsed.version===1&&parsed.id===id?{head:parsed,session,units:JSON.parse(Buffer.from(units).toString("utf8")) as OwnerPoolUnitV1[]}:undefined;
 }catch{return undefined;}
}

/** 🌱️ A deterministic 32 bit number from a text: the first eight hex digits of its digest. */
export const ownerPoolSeedV1=(text:string)=>parseInt(sha(text).slice(0,8),16)>>>0;

/** 🧭️ The rendezvous score of an owner for one puller, in [0, 1): the lower it is the earlier the puller takes the owner, so every puller has an order of its own over the same owners and N pullers start on N different ones; a pure function of both names. */
export const ownerPoolRendezvousV1=(puller:string,name:string)=>parseInt(sha(puller+"\u0000"+name).slice(0,8),16)/4294967296;

/** 🎲️ The next number in [0, 1) of a small generator (mulberry32) from a state, so a puller's walk is reproducible in tests and across languages. */
export function ownerPoolRandomV1(state:number):Readonly<{value:number;state:number}>{
 const next=(state+0x6D2B79F5)>>>0;let t=next;t=Math.imul(t^(t>>>15),t|1);t^=t+Math.imul(t^(t>>>7),t|61);
 return{value:((t^(t>>>14))>>>0)/4294967296,state:next};
}

/** 🙋️ What the machine's own read claims from the one budget through the capacity governor: the megabytes its first waiting owner needs (`alone`: the owner is one the cost table says needs the whole machine, claimed from the first request until the owner has ended).
 * @see ../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🙋️asks/🟦️.ts */
export type OwnerPoolAskV1=Readonly<{megabytes:number;alone:boolean}>;

/** 🏠️ What the host says about itself for the sizing of the pool: the machine, the memory it has available, how loud it is and what its own work (builds, setup, a wave) holds. */
export type OwnerPoolHostV1=Readonly<{cores:number;totalMegabytes:number;availableMegabytes:number;pressure:"normal"|"warn"|"critical";load:number|null;idleCores?:number|null;swapOutMegabytesPerSecond:number|null;foreground:Readonly<{cpu:number;unrealizedMegabytes:number;retainedUnrealizedMegabytes?:number;waiters:number;waitingMegabytes?:number;oldestWaitMilliseconds?:number;ask?:OwnerPoolAskV1}>}>;
/** 🎯️ How many children the pool runs and why it runs fewer than it could. */
export type OwnerPoolTargetV1=Readonly<{children:number;reason:"pressure"|"yielding"|"nothing-fits"|"waiting"|null;limit?:OwnerPoolLimitV1}>;
/** 🔢️ The numbers behind a target that is below the cores: which rule binds (`memory`, `cores`, `load`, `pressure`, `waiting`, `none`), what one more child needs, what is free for it above the floor, what the machine's own work holds back, the floor, and the cores (all, booked by the machine's own work, idle by measurement, `null` when not measured). */
export type OwnerPoolLimitV1=Readonly<{limit:"memory"|"cores"|"load"|"pressure"|"waiting"|"none";needMegabytes:number;freeMegabytes:number;heldMegabytes:number;floorMegabytes:number;children:number;cores:number;bookedCores:number;idleCores:number|null;swapOutMegabytesPerSecond:number;waitedMilliseconds:number}>;

const clamp=(value:number,low:number,high:number)=>Math.max(low,Math.min(high,value));

/** 🧱️ The two memory limits of one child on this machine: it is replaced when it passes the first once an owner ended (the allocator never gives memory back) and killed when it passes the second. */
export function ownerPoolChildLimitsV1(totalMegabytes:number,policy:typeof P=P):Readonly<{recycleMegabytes:number;hardMegabytes:number}>{
 return{recycleMegabytes:clamp(Math.floor(totalMegabytes*policy.childRecycleShareBasisPoints/10000),policy.childRecycleMinimumMegabytes,policy.childRecycleMaximumMegabytes),hardMegabytes:Math.max(policy.childHardMinimumMegabytes,Math.floor(totalMegabytes*policy.childHardShareBasisPoints/10000))};
}

/** 🏋️ Whether a unit is heavy: its known or learned cost reaches the declared heavy size, so every puller that can hold it takes it before anything else. */
export const ownerPoolHeavyV1=(unit:Readonly<{megabytes?:number}>,policy:typeof P=P):boolean=>(unit.megabytes??0)>=policy.heavyMegabytes;

/** 🧲️ Whether a puller whose children may hold `grantMegabytes` may claim a unit: an owner whose known or learned cost exceeds the grant is left to the machines that can hold it (a unit of unknown cost fits everywhere). */
export const ownerPoolFitsV1=(unit:Readonly<{megabytes?:number}>,grantMegabytes:number):boolean=>(unit.megabytes??0)<=grantMegabytes;

/** 📏️ What one more child is expected to hold: the `peakPercentile` of the newest `peakSamples` resident sizes the children reported after an owner together with the resident sizes the children hold right now (`live`: a child that waits for an owner or loads its program is a real child too, so a pool of many idle children is not priced as if every one ran the heaviest owner), nearest rank, plus the margin, so a single giant owner does not price every light one and the price does not jump with the last five; the declared default while nobody reported. These are the children's own measurements, never a share of a parent's growth.
 * @see ./🧪️tests/🧫️fixtures/🐍️oracle.py */
export function ownerPoolChildMegabytesV1(samples:readonly number[],live:readonly number[]=[],policy:typeof P=P):number{
 const recent=[...samples.filter(value=>Number.isFinite(value)&&value>0).slice(-policy.peakSamples),...live.filter(value=>Number.isFinite(value)&&value>0)].sort((a,b)=>a-b);
 return recent.length===0?policy.childMegabytesDefault:Math.ceil(recent[Math.max(0,Math.ceil(recent.length*policy.peakPercentile/100)-1)]!*(100+policy.peakMarginPercent)/100);
}

/** ♻️ Whether a child is replaced after an owner: its resident size passed the recycle limit AND grew by at least `recycleGrowthMegabytes` over what it held when it was ready, so a child whose bare start is already near the limit is not restarted after every owner (a start costs far more than a light owner).
 * @see ./🧪️tests/🧫️fixtures/🐍️oracle.py */
export const ownerPoolRecycleV1=(input:Readonly<{rss:number;readyRss:number;recycleMegabytes:number}>,policy:typeof P=P)=>input.rss>input.recycleMegabytes&&input.rss>=input.readyRss+policy.recycleGrowthMegabytes;

/** 🗂️ The first directory a child may stage its owners in: one where a probe file of `stagingProbeMegabytes` can be written and removed (a full, tiny or quota-limited temporary directory answers EDQUOT or ENOSPC at the first big write and would fail every owner of the machine), the answer is the first candidate that took it, the last one when none did.
 * @see ./🧪️tests/🟦️.ts */
export function ownerPoolStagingDirectoryV1(candidates:readonly string[],io:Readonly<{probe(directory:string,megabytes:number):void}>,policy:typeof P=P):string{
 for(const directory of candidates){try{io.probe(directory,policy.stagingProbeMegabytes);return directory;}catch{}}
 return candidates[candidates.length-1]!;
}

/** 🧾️ Which pids of the registry text are still alive: the registry holds one JSON line per child (`pid`, `wave`), a line whose process is gone is dropped, a row older than a child can live (its lifetime plus the drain it is given) names a pid that the system gave to another process, so it counts for nothing, and a supervisor that restarts counts the children of the one before it (they finish their owner and end by themselves) and never starts the machine's whole capacity on top of them.
 * @see ./🧪️tests/🟦️.ts */
export const ownerPoolLiveChildrenV1=(rows:readonly Readonly<{pid:number;startedMs?:number}>[],alive:(pid:number)=>boolean,own:ReadonlySet<number>,now:number=Date.now(),policy:typeof P=P):readonly number[]=>rows.filter(row=>row.startedMs===undefined||now-row.startedMs<=policy.childLifetimeMilliseconds+policy.drainExitMilliseconds).map(row=>row.pid).filter(pid=>!own.has(pid)&&alive(pid));

/** 💾️ Writes the files of a wave to the first directory that takes them (a full or quota-limited temporary directory must not keep the machine from every later wave): the answer is the directory used, the error of the last one when none took them. */
export function writeOwnerPoolWaveFilesV1(directories:readonly string[],files:Readonly<Record<string,string|Uint8Array>>,io:Readonly<{mkdir(path:string):void;write(path:string,data:string|Uint8Array):void;remove(path:string):void}>):string{
 let last:unknown;
 for(const directory of directories){
  try{io.mkdir(directory);for(const [name,data] of Object.entries(files))io.write(directory+"/"+name,data);return directory;}
  catch(error){last=error;try{io.remove(directory);}catch{}}
 }
 throw last;
}

/** 🎯️ How many children this machine runs now, from one budget of memory and cores that the machine's own read and the pool share (the read has the first claim, the pool gets what it leaves): none while the read's owner is one that needs the whole machine (`ask.alone`, from its first request until the owner ended); none while a request of the machine's own work has waited `yieldAfterMilliseconds` in the gate and the children are what holds it up (its memory fits only once they are gone, or no core is idle although they would free one: a wait the children did not cause is no reason to stop them, and there is no time after which they come back); otherwise every core and every free megabyte that the system floor, the part of its own work's untouched reservations, what its waiting requests ask for and the first claim of its read (only when draining the children would make it fit) leave, so a claim that does not fit drains exactly as many children as it needs, the children that run are never more than the spare memory allows (a child can run only when its memory fits above the floor), and swap shrinks the pool before it stops anything else: from `swapActiveMegabytesPerSecond` to the `swapOutMegabytesPerSecond` at which the gate stops the farm the children are cut in proportion (the same number, a law test pins both). Cores: the machine's, less what its own grants book, less a share while its own requests wait; the idle cores the host measured (`idleCores`, the children's own load not counted) grow the pool by as many children, and where nothing was measured the load that does not come from the children is held against the load ceiling (`loadCeilingBasisPointsPerCore` per core); at most `maximumChildren`; memory: the available megabytes less the floor and the held part, divided by the size of one child, added to the children that already hold theirs. A pure function.
 * @see ./🧪️tests/🧫️fixtures/🐍️oracle.py */
export function ownerPoolSizingV1(input:Readonly<{host:OwnerPoolHostV1;children:number;childMegabytes:number}>,policy:typeof P=P):OwnerPoolTargetV1&Readonly<{limit:OwnerPoolLimitV1}>{
 const {host}=input,floor=clamp(Math.floor(host.totalMegabytes*policy.floorShareBasisPoints/10000),policy.floorMinimumMegabytes,policy.floorMaximumMegabytes),foreground=host.foreground,need=input.childMegabytes,ask=foreground.ask;
 const forecastHold=foreground.waiters>0&&foreground.waitingMegabytes===undefined?foreground.unrealizedMegabytes:Math.min(foreground.unrealizedMegabytes,Math.floor(host.totalMegabytes*policy.ownHoldShareBasisPoints/10000));
 const hold=forecastHold+(foreground.retainedUnrealizedMegabytes??0),roomNow=host.availableMegabytes-floor-hold,roomDrained=roomNow+input.children*need;
 const blocked=(megabytes:number)=>megabytes>roomNow&&megabytes<=roomDrained,counted=(megabytes:number)=>megabytes<=roomDrained?megabytes:0;
 const held=hold+counted(foreground.waitingMegabytes??0)+counted(ask?.megabytes??0),spare=host.availableMegabytes-floor-held;
 const reserved=foreground.waiters>0?Math.ceil(host.cores*policy.foregroundCoreShareBasisPoints/10000):0,booked=Math.max(0,host.cores-foreground.cpu-reserved),idle=host.idleCores??null,waited=foreground.oldestWaitMilliseconds??0,swap=host.swapOutMegabytesPerSecond??0;
 const limit=(kind:OwnerPoolLimitV1["limit"],children:number):OwnerPoolLimitV1=>({limit:kind,needMegabytes:need,freeMegabytes:Math.max(0,spare-Math.max(0,children-input.children)*need),heldMegabytes:held,floorMegabytes:floor,children:input.children,cores:host.cores,bookedCores:Math.min(host.cores,foreground.cpu),idleCores:idle,swapOutMegabytesPerSecond:swap,waitedMilliseconds:waited});
 const swapLow=host.totalMegabytes*policy.swapActiveBasisPointsPerSecond/10000,swapHigh=host.totalMegabytes*policy.swapOutBasisPointsPerSecond/10000,swapShare=host.swapOutMegabytesPerSecond===null||swap<swapLow?1:Math.max(0,(swapHigh-swap)/(swapHigh-swapLow)),swapCap=Math.floor(input.children*swapShare);
 if(host.pressure==="critical"||swapCap<1&&swapShare<1)return{children:0,reason:"pressure",limit:limit("pressure",0)};
 if(ask?.alone===true)return{children:0,reason:"waiting",limit:limit("waiting",0)};
 const coresHeld=idle===null||idle<1&&idle+input.children>=1,memoryHeld=foreground.waitingMegabytes===undefined||blocked(foreground.waitingMegabytes);
 if(foreground.waiters>0&&waited>=policy.yieldAfterMilliseconds&&(memoryHeld||coresHeld))return{children:0,reason:"waiting",limit:limit("waiting",0)};
 const minimum=Math.max(1,Math.ceil(host.cores*policy.minimumShareBasisPoints/10000)),byCores=idle!==null?(foreground.waiters>0?input.children+Math.floor(Math.max(0,idle-reserved)):host.cores):foreground.waiters>0?booked:Math.max(booked,minimum);
 const external=host.load===null?0:Math.max(0,host.load-input.children),byLoad=Math.max(minimum,idle===null?Math.floor(host.cores*policy.loadCeilingBasisPointsPerCore/10000-external):input.children+Math.floor(idle));
 const byMemory=input.children+Math.floor(spare/Math.max(1,need));
 let children=Math.min(byCores,byLoad,byMemory,policy.maximumChildren,...swapShare<1?[swapCap]:[]);
 if(host.pressure==="warn")children=Math.min(children,Math.max(1,input.children));
 const wanted=Math.min(host.cores,policy.maximumChildren),kind:OwnerPoolLimitV1["limit"]=byMemory<=Math.min(byCores,byLoad)?"memory":byLoad<byCores?"load":"cores";
 if(children<1)return byMemory<1?{children:0,reason:"nothing-fits",limit:limit(kind,0)}:{children:1,reason:"yielding",limit:limit(kind,1)};
 if(children<wanted)return{children,reason:kind==="memory"?"nothing-fits":"yielding",limit:limit(kind,children)};
 return{children,reason:null,limit:limit("none",children)};
}

/** 📌️ The target of the pool without its numbers: how many children and why fewer than the cores.
 * @see ./🧪️tests/🧫️fixtures/🐍️oracle.py */
export function ownerPoolTargetV1(input:Readonly<{host:OwnerPoolHostV1;children:number;childMegabytes:number}>,policy:typeof P=P):OwnerPoolTargetV1{
 const {children,reason}=ownerPoolSizingV1(input,policy);
 return{children,reason};
}

/** 🌡️ The idle cores between two samples of the operating system's cpu times (`os.cpus()`): the idle share of all the time that passed, times the cores; `null` when no time passed or the samples differ in length.
 * @see ./🧪️tests/🎚️lanes/🟦️.ts */
export function ownerPoolIdleCoresV1(before:readonly Readonly<{times:Readonly<{user:number;nice:number;sys:number;idle:number;irq:number}>}>[],after:readonly Readonly<{times:Readonly<{user:number;nice:number;sys:number;idle:number;irq:number}>}>[]):number|null{
 if(before.length===0||before.length!==after.length)return null;
 let idle=0,total=0;
 for(let at=0;at<after.length;at++){const a=before[at]!.times,b=after[at]!.times;idle+=b.idle-a.idle;total+=b.user-a.user+b.nice-a.nice+b.sys-a.sys+b.idle-a.idle+b.irq-a.irq;}
 return total<=0?null:Math.max(0,idle)/total*after.length;
}

/** 📊️ What one pull run did. */
export type OwnerPoolStatsV1={claimed:number;published:number;stored:number;held:number;faulted:number;failed:number;refused:number;attempts:number};

const defaultSleep=(milliseconds:number,signal?:AbortSignal)=>new Promise<void>(done=>{const timer=setTimeout(done,milliseconds);signal?.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});});

/** 🌐️ Pulls owners of the pool until every one is stored by someone, the signal aborts or `stop` says the wave is over. One puller walks the list from a start that is its own (a digest of its name, so fifteen pullers start apart and never meet before the end), claiming a window of `claimWindow` neighbours in one round trip (neighbours share their imports, so a walk keeps the programs of its process warm), runs as many as `slots()` allows and keeps `prefetchClaims` granted ahead so a finished owner is followed at once; a run of `gallopRun` owners that others hold or stored ends in a jump to another place of the list, and `doneStoredMinimum` stored owners in a row end the pull (the wave is done as far as this puller can tell; the supervisor looks again later: with 3 percent of a wave open the old twelve were enough to end 7 of 10 pulls while 1100 owners were open), so no puller has to look at every owner of a wave, and while the run of stored owners is long (`2 x claimWindow`) and nothing is waiting, one round claims `claimWindowMaximum` owners instead of the window. Owners are taken tier by tier (`tier` of the unit, none is 0): no owner of a later tier is claimed while one of an earlier tier is open for this puller, so the owners a read waits for (selection and priority owners) go before the owners it only looks ahead to. Within a tier heavy owners that fit this puller are taken first, in the order of its rendezvous scores ({@link ownerPoolRendezvousV1}), so no two pullers meet on the same heavy owner first. Each owner is claimed by the digest of its record state before it is computed (a lapsed claim returns the owner to the pool, a stored record is never computed again), published when finished and given back when it fails. Nothing here waits for another machine and nothing schedules for one. */
/** 🕳️ How the pool files one failed owner: a native dependency without a build project owner row is the registry's gap and not the owner's fault, so it is skipped (code `ownerless`, never published as a deterministic failure note that other machines would honour after the row exists) and reported by its own code; every other failure keeps its code and determinism. */
export function ownerPoolFailureClassV1(input:Readonly<{message:string;deterministic:boolean;code?:string}>):Readonly<{code:string;deterministic:boolean;skipped:boolean}>{
 const ownerless=(input.code===undefined||input.code==="")&&/Native dependency has no build project owner: /u.test(input.message);
 return Object.freeze(ownerless?{code:"ownerless",deterministic:false,skipped:true}:{code:input.code??"",deterministic:input.deterministic,skipped:false});
}
export async function pullOwnerPoolV1<U extends Readonly<{name:string;megabytes?:number}>>(input:Readonly<{units:()=>readonly U[];fits?(unit:U):boolean;turn?(unit:U):number;claim(unit:U):Promise<ProjectClaimV1>;run(unit:U):Promise<boolean>;slots():number;puller:string;signal?:AbortSignal;stop?():boolean;busy?(running:number):void;progress?(stats:Readonly<OwnerPoolStatsV1>,running:number):void;done?(unit:U,ok:boolean,milliseconds:number):void;stored?(unit:U):void;known?:Set<string>;fail?(name:string,message:string,deterministic:boolean,code?:string):void;now?():number;sleep?(milliseconds:number,signal?:AbortSignal):Promise<void>;policy?:typeof P}>):Promise<OwnerPoolStatsV1>{
 const policy=input.policy??P,now=input.now??Date.now,sleep=input.sleep??defaultSleep,finished=input.known??new Set<string>(),retryAt=new Map<string,number>(),running=new Map<string,Promise<void>>(),ready:{unit:U;claim:Extract<ProjectClaimV1,{state:"granted"}>}[]=[],stats:OwnerPoolStatsV1={claimed:0,published:0,stored:0,held:0,faulted:0,failed:0,refused:0,attempts:0};
 let random=ownerPoolSeedV1(input.puller),streak=0,storedRun=0;
 const over=()=>input.signal?.aborted===true||input.stop?.()===true,draw=()=>{const next=ownerPoolRandomV1(random);random=next.state;return next.value;};
 const execute=async(unit:U,claim:Extract<ProjectClaimV1,{state:"granted"}>)=>{
  const began=now();let ok=false,held=claim;
  try{
   for(let again=0;;again++){
    if(await input.run(unit)&&await held.publish()){stats.published++;ok=true;break;}
    if(held.code()==="no-record"&&again<policy.noRecordRetries&&!over()){const next=await input.claim(unit).catch(()=>undefined);if(next?.state==="granted"){held=next;continue;}}
    stats.failed++;input.fail?.(unit.name,"the record was not stored: "+held.why(),held.deterministic(),held.code());await held.abandon();break;
   }
  }catch(error){stats.failed++;input.fail?.(unit.name,String((error as Error)?.message??error),true);await held.abandon().catch(()=>undefined);}
  finally{running.delete(unit.name);finished.add(unit.name);input.done?.(unit,ok,now()-began);input.busy?.(running.size);input.progress?.(stats,running.size);}
 };
 const seen=new Map<string,number>(),turnOf=input.turn,waiting=(unit:U)=>{const wait=turnOf?.(unit)??0;if(wait<=0)return false;const first=seen.get(unit.name)??(seen.set(unit.name,now()),now());return now()-first<wait;};
 const fits=input.fits??(()=>true),usable=():readonly U[]=>input.fits===undefined?input.units():input.units().filter(unit=>fits(unit));
 const tierOf=(unit:U)=>(unit as {tier?:number}).tier??0,places=new Map<number,{cursor:number;placed:boolean}>(),replace=()=>{for(const place of places.values())place.placed=false;};
 const pickIn=(list:readonly U[],tier:number,count:number):U[]=>{
  const picked:U[]=[];if(list.length===0||count<=0)return picked;
  const place=places.get(tier)??(places.set(tier,{cursor:-1,placed:false}),places.get(tier)!),queuedHeavy=new Set(ready.map(row=>row.unit.name));
  const heavy:{unit:U;score:number}[]=[];
  for(const unit of list)if(ownerPoolHeavyV1(unit,policy)&&!finished.has(unit.name)&&!running.has(unit.name)&&!queuedHeavy.has(unit.name)&&(retryAt.get(unit.name)??0)<=now()&&!waiting(unit))heavy.push({unit,score:ownerPoolRendezvousV1(input.puller,unit.name)});
  heavy.sort((a,b)=>a.score-b.score);
  for(const row of heavy){if(picked.length>=count)break;picked.push(row.unit);}
  if(picked.length>=count)return picked;
  if(!place.placed||place.cursor>=list.length){place.cursor=Math.floor(draw()*list.length);place.placed=true;}
  const queued=new Set(ready.map(row=>row.unit.name));
  for(let step=0;step<list.length&&picked.length<count;step++){
   if(place.cursor<0)place.cursor=list.length-1;
   const unit=list[place.cursor--]!;
   if(finished.has(unit.name)||running.has(unit.name)||queued.has(unit.name)||(retryAt.get(unit.name)??0)>now()||ownerPoolHeavyV1(unit,policy)&&waiting(unit))continue;
   if(picked.includes(unit))continue;
   picked.push(unit);
  }
  return picked;
 };
 const pick=(count:number):U[]=>{
  const list=usable(),picked:U[]=[];
  for(const tier of[...new Set(list.map(tierOf))].sort((a,b)=>a-b)){
   if(picked.length>=count)break;
   picked.push(...pickIn(list.filter(unit=>tierOf(unit)===tier),tier,count-picked.length));
  }
  return picked;
 };
 const total=()=>usable().length;
 while(!over()&&finished.size<total()&&storedRun<policy.doneStoredMinimum){
  const slots=Math.max(0,input.slots());
  let progressed=false;
  const searching=storedRun>=2*policy.claimWindow&&ready.length===0,want=searching?policy.claimWindowMaximum:Math.min(policy.claimWindow,slots+policy.prefetchClaims-running.size-ready.length);
  if(slots>0&&want>0){
   const picked=pick(want);
   stats.attempts+=picked.length;
   const answers=await Promise.all(picked.map(async unit=>({unit,claim:await input.claim(unit).catch(()=>({state:"fault"}) as ProjectClaimV1)})));
   for(const {unit,claim} of answers){
    if(claim.state==="stored"){finished.add(unit.name);input.stored?.(unit);stats.stored++;streak++;storedRun++;}
    else if(claim.state==="refused"){finished.add(unit.name);stats.refused++;streak++;storedRun++;}
    else if(claim.state==="held"){retryAt.set(unit.name,now()+Math.max(2000,Math.min(policy.heldRetryMilliseconds,Math.floor(((unit as {expectedMilliseconds?:number}).expectedMilliseconds??policy.expectedMillisecondsDefault)/4))));stats.held++;streak++;storedRun=0;}
    else if(claim.state==="fault"){retryAt.set(unit.name,now()+policy.faultRetryMilliseconds);stats.faulted++;streak++;storedRun=0;}
    else if(searching&&ready.length+running.size>=slots+policy.prefetchClaims){await claim.abandon().catch(()=>undefined);}
    else{ready.push({unit,claim});stats.claimed++;streak=0;storedRun=0;progressed=true;}
   }
   if(streak>=policy.gallopRun){replace();streak=0;}
  }
  while(ready.length>0&&running.size<slots&&!over()){const row=ready.shift()!;running.set(row.unit.name,execute(row.unit,row.claim));progressed=true;}
  input.busy?.(running.size);input.progress?.(stats,running.size);
  if(over())break;
  await Promise.race([sleep(progressed?0:policy.sweepPollMilliseconds,input.signal),...running.values()]);
 }
 for(const row of ready.splice(0))await row.claim.abandon().catch(()=>undefined);
 await Promise.allSettled([...running.values()]);
 return stats;
}
