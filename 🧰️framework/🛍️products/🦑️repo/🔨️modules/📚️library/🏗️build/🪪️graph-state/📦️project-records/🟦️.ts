import{machinePathV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import{AsyncLocalStorage}from"node:async_hooks";
import{createHash,randomBytes}from"node:crypto";
import{existsSync,mkdirSync,readdirSync,readFileSync,statSync,unlinkSync,writeFileSync}from"node:fs";
import{hostname}from"node:os";
import{join}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{graphPlatformV1,type GraphStateV1}from"../🟦️.ts";
import{recordPlatformV1}from"../🌍️platform-class/🟦️.ts";
import{bundleKeysV1,closureDocumentHeadV1,consultedHoldV1,dependencyGenerationV1,hostOfRootV1,localBundleV1,localRecordV1,neutralRecordV1,packBundleV1,packRecordV1,portableRowsPolicyV1,unpackBundleV1,unpackRecordV1,type ClosureBundleV1,type ConsultedKeysV1,type ObservationRowV1,type PackedClosureV1,type PackedRecordV1,type PortableRecordV1}from"./🧳️portable/🟦️.ts";
import{decideGraphEntryV1,fetchGraphEntryV1,graphEntryKeyV1,graphFleetClientV1,neutralizeRootV1,publishGraphEntryOutcomeV1,publishGraphEntryV1,tryGraphEntryV1,type GraphDeferredV1,type GraphEntryNoteV1,type GraphFleetClientV1,type GraphHoldV1,type GraphLookupFaultV1,type GraphRefusalV1}from"../🛰️exchange/🟦️.ts";
import{FRESH_POLICY_V1,fileDigestV1,freshVerdictV1,refreshStateV1}from"./🌱️fresh/🟦️.ts";
import{unsentDropV1,unsentListV1,unsentPruneV1,unsentPutV1,unsentReadV1}from"./📮️unsent/🟦️.ts";
import{graphFleetSettingsV1}from"../🧶️nx/🟦️.ts";
import{remoteCacheFetchV1}from"../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{workspaceClosurePortV1,type ClosurePortV1}from"../../🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts";
import schemaLeaves from"./🍃️leaves/🧬️schema/🔣️.json"with{type:"json"};
import{adoptLeavesV1,leafPortWithDirectPutV1,leafSourcesV1,leafWidthV1}from"./🍃️leaves/🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;
/** 📦️ What the inference seam hands over for one project: where its record lives, which project it is, the key text it was computed under, and how to read and write the record locally. */
/** 🚚️ How the closure entries a record names travel with it: bundles of the recipes it refers to, exported from and imported into the closure store of this machine. An exported bundle reads its layers from the store while it is iterated (one layer at a time) and carries the stamp of the stored entry; `source` names the stored document an imported bundle came from, and `held` answers the local key of the entry that was imported from the document of that digest while it is still in the store. */
export type ProjectClosureTravelV1=Readonly<{export(root:string,image:string):Promise<readonly Readonly<{recipeKey:string;stamp?:string;bundle:unknown}>[]>;import(root:string,bundles:readonly Readonly<{recipeKey:string;bundle:unknown;source?:string;neutral?:string}>[]):Promise<number>;held?(root:string,source:string,neutral?:string,refusedMilliseconds?:number):Promise<string|undefined>}>;
export type ProjectRecordInfoV1=Readonly<{cachePath:string;fleet:string;keyText:string;exportRecord():unknown;import(portable:unknown):Promise<boolean>;closures?:ProjectClosureTravelV1|undefined;expectedMilliseconds?:number|undefined;ttlMilliseconds?:number|undefined}>;
const SOURCE_ANALYSES="../../🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs",SCRIPT_INPUTS="../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs",OBSERVATION="../../👁️observation/🟨️.mjs";
/** 🪪️ The ids of the source analyses this process registered for its inference, in the order of their roles: a taken record names the providers of its producer by role and the id of a provider contains a hash of the module location, so the taker puts in the ids it has registered (the module is loaded only when a record is taken; a process that cannot load it takes records whose analysis rows cannot hold, and infers those projects itself). */
async function registeredProvidersV1():Promise<readonly string[]>{
 const loaded=await import(SOURCE_ANALYSES) as {prepareSourceAnalysesV1():Readonly<Record<string,{id:string}>>},own=await import(SCRIPT_INPUTS).then(module=>(module as {scriptProviderIdsV1():readonly string[]}).scriptProviderIdsV1(),()=>[] as readonly string[]);
 return[...Object.values(loaded.prepareSourceAnalysesV1()).map(provider=>provider.id),...own];
}

/** 🧬️ Whether one closure entry a stored record carries still holds on this tree, the verdict a taker gives the entries it imported: the rows of the entry and of its layers probed against the files, listings and analyses of the tree, in steps that give the event loop its turns. A record whose entries walked files that changed since is stale for every reader although its own rows hold, so a pool that counted it as stored never refreshed it.
 * @see ./🧪️tests/🤝️pool-reader/🟦️.ts */
export async function closureBundleHoldsV1(bundle:Readonly<{rows:readonly unknown[];layers:Iterable<readonly unknown[]>}>):Promise<boolean>{
 const observation=await import(OBSERVATION) as {observationRowsHoldAsyncV1(rows:readonly unknown[],step:()=>unknown):Promise<boolean>;observationLayerHoldsAsyncV1(layer:readonly unknown[],step:()=>unknown):Promise<boolean>};
 let turns=0;const step=()=>++turns%2048===0?new Promise<void>(done=>setImmediate(done)):undefined;
 if(!await observation.observationRowsHoldAsyncV1([...bundle.rows],step))return false;
 for(const layer of bundle.layers)if(!await observation.observationLayerHoldsAsyncV1(layer,step))return false;
 return true;
}

/** 🔒️ The question a taker puts to the `closure` rows of a record, answered from the closures of this tree after the entries they name are settled (the script inputs register the provider when they load); undefined when it cannot be asked here, and a row the provider cannot answer (`!stale`, `!unreadable`, `!unregistered`: the entry could not be settled in this process) counts as fresh, never as stale. */
async function closureHoldsOfV1(rows:readonly ObservationRowV1[],image:string):Promise<((row:ObservationRowV1)=>boolean)|undefined>{
 const closures=rows.filter(row=>row.kind==="closure");if(closures.length===0)return undefined;
 try{await import(SCRIPT_INPUTS);const observation=await import(OBSERVATION) as {settleClosureRowsV1(rows:readonly unknown[],value:unknown):Promise<void>;observationRowsHoldV1(rows:readonly unknown[]):boolean};await observation.settleClosureRowsV1(closures,JSON.parse(image));return row=>observation.observationRowsHoldV1([row])||["!stale","!unreadable","!unregistered"].some(answer=>observation.observationRowsHoldV1([{...row,fact:answer}]));}catch{return undefined;}
}

/** 📂️ The question a taker puts to every row that is neither a file digest nor a closure (a directory listing, an existence, a real path): the verdict of `observationRowFreshV1` on this tree; a row it cannot judge counts as fresh, never as stale. */
async function rowHoldsOfV1():Promise<((row:ObservationRowV1)=>boolean)|undefined>{
 try{const observation=await import(OBSERVATION) as {observationRowFreshV1(row:unknown):boolean};return row=>{try{return observation.observationRowFreshV1(row);}catch{return true;}};}catch{return undefined;}
}

/** 🚪️ The answer of the gate: whether the record was taken from the fleet, and how to store the computed one or give the claim back. */
export type ProjectGateV1=Readonly<{seeded:boolean;deferred?:boolean;publish():void;abandon():Promise<void>}>;
/** 🎯️ The answer of a pulled claim: the record is stored or held by another machine (leave it), or this machine holds it and `publish` stores the finished record (true when stored) or `abandon` gives the claim back. */
export type ProjectClaimV1=Readonly<{state:"stored"|"fault"|"refused"}>|Readonly<{state:"held";by:string}>|Readonly<{state:"granted";publish():Promise<boolean>;abandon():Promise<void>;why():string;code():string;deterministic():boolean;digest:string}>;
type GatePhaseV1=keyof typeof schema.definitions.GateTimings.properties;
type GateTimingV1={started:number;active:number;milliseconds:number};
type GateTimingsV1=Record<GatePhaseV1,GateTimingV1>;
export type ProjectFleetPortV1=Readonly<{producers():Readonly<Record<string,number>>;claim(info:ProjectRecordInfoV1):Promise<ProjectClaimV1>;gate(info:ProjectRecordInfoV1):Promise<ProjectGateV1>;offer(info:ProjectRecordInfoV1):Promise<void>;resend(options?:Readonly<{force?:boolean}>):Promise<number>;flush():Promise<string>;stats():Readonly<Record<"seeded"|"published"|"offered"|"waited"|"computed"|"closures"|"refused"|"reinferred"|"diverged"|"resent"|"kept"|"shed"|"refreshed"|"retried"|"leaves"|"incomplete",number>&{gateTimings:GateTimingsV1}>}>;

/** 🔍️ What a look of a reader found for one owner in the fleet cache: `fetched` (the record was taken into the local store, the reader checks it against its tree), `miss` (nothing is stored), `fault` (the cache did not answer). */
export type TakeLookV1={outcome?:"fetched"|"miss"|"fault"|"incomplete";fault?:GraphLookupFaultV1;readonly signal?:AbortSignal};
const takeScopes=new AsyncLocalStorage<TakeLookV1>();
/** 🔍️ Runs `work` as a look and nothing else: inside it the `gate` of the port only looks the record up in the fleet cache and takes it when it is there; it never claims the computation, never waits for another machine and never publishes. `look` tells what the gate found, so the reader can tell a taken record from a missing, a stale and an unanswered one. */
export function runTakeLookV1<T>(work:(look:TakeLookV1)=>Promise<T>,signal?:AbortSignal):Promise<T>{const look:TakeLookV1={signal};return takeScopes.run(look,()=>work(look));}
/** 🔍️ What the gate found in the look that is running now (undefined outside of a look or before the gate was asked). */
export function takeLookOutcomeV1():TakeLookV1["outcome"]{return takeScopes.getStore()?.outcome;}

/** 🚩️ Where the fleet marker of a stored project record lives: beside the record, named after it; the closure proof of the graph store knows this name. */
export const projectRecordMarkerPathV1=(cachePath:string)=>cachePath+".fleet";
const ZERO="0".repeat(64),sha=(text:string)=>createHash("sha256").update(text).digest("hex"),MAXIMUM_OFFERS=64,MAXIMUM_PACKED=32,CLOSURE_WINDOW=2,MAXIMUM_RECIPES=4096,isClosureDocument=(name:string)=>name.startsWith("closure-"),view=(bytes:Uint8Array)=>Buffer.from(bytes.buffer,bytes.byteOffset,bytes.byteLength),RETRY=schema.definitions.Publish.const;

/** 🧮️ How many of the closure documents, taken in the order of their sizes as given, fit the byte budget: the longest prefix whose sum stays within it. A pure function; the port stops packing at the first document that does not fit, so an owner with hundreds of closures never holds more than the budget and one document.
 * @see ./🧪️tests/📮️retry/🐍️.py */
export function projectClosurePrefixV1(sizes:readonly number[],budget:number=RETRY.closureBudgetBytes):number{let used=0,kept=0;for(const size of sizes){if(used+size>budget)break;used+=size;kept++;}return kept;}

/** ⏱️ The wait before the next try of a publication: the back-off of the policy for the try that just failed (the last one repeats), stretched by up to the jitter percent of `unit` (a number in [0, 1)), and never shorter than the rest of the breaker of the machine up to its maximum. A pure function.
 * @see ./🧪️tests/📮️retry/🐍️.py */
export function projectPublishWaitV1(attempt:number,breakerMilliseconds:number,unit:number,policy:typeof RETRY=RETRY):number{const base=policy.backoffMilliseconds[Math.min(attempt,policy.backoffMilliseconds.length-1)]??0;return Math.max(Math.round(base*(1+unit*policy.backoffJitterPercent/100)),Math.min(Math.max(0,breakerMilliseconds),policy.breakerWaitMaximumMilliseconds));}

const KIB=(documents:readonly Readonly<{bytes:Uint8Array}>[])=>Math.round(documents.reduce((sum,each)=>sum+each.bytes.length,0)/1024);
const explain=(refusal:GraphRefusalV1,documents:readonly Readonly<{bytes:Uint8Array}>[])=>`${refusal.code.startsWith("limit-")?refusal.detail:refusal.text+(refusal.status!==null?` [HTTP ${refusal.status}${refusal.detail===""?"":": "+refusal.detail}]`:"")} (${documents.length} documents, ${KIB(documents)} KiB)`;

/** 🧮️ The state a project record is stored under: the digest of the record key (inference revision and the project's own identity) with the root replaced by the token, the platform tag (the neutral tag of every platform, the exact platform for a project whose key names a host-bound declaration), and one slot per project so that a newer record of a project supersedes only its own older ones. It names nothing of the machine, the generation or the module-load state of the producer: the record carries every observation it consumed, and a taker checks them against its own tree before it uses the record, so a record is reusable by every machine whose project inputs equal the producer's. A record that does not hold in the taker's tree is replaced by the taker's own. */
export function projectRecordStateV1(input:Readonly<{root:string;fleet:string;keyText:string;platform:string}>):GraphStateV1{
 const neutral=(text:string)=>Buffer.from(neutralizeRootV1(Buffer.from(text),input.root)).toString("utf8"),platform=recordPlatformV1(input.keyText,input.platform),digest=sha(JSON.stringify(["project-inference",schema.definitions.Policy.const.form,neutral(input.keyText),platform,neutral(input.fleet)]));
 return Object.freeze({version:1,frame:Object.freeze({kind:"project-inference" as const,tooling:sha(neutral(input.fleet)),implementation:ZERO,platform,extra:Object.freeze({})}),names:ZERO,content:ZERO,files:0,contentFiles:0,digest});
}

/** 🌐️ The port of one process: it takes a missing project record from the fleet cache, claims the computation of a record nobody has (so eight machines split the projects), stores finished records and offers the records that were computed earlier. Every fault ends in "compute locally". `undefined` when no fleet cache is configured. */
export function createProjectFleetPortV1(input:Readonly<{root:string;environment:Environment;busy?():boolean;strong?:boolean;client?:GraphFleetClientV1;owner?:string;platform?:string;providers?():Promise<readonly string[]>;generation?(root:string):string|undefined;say?(line:string):void;sleep?(milliseconds:number):Promise<void>;random?():number;now?():number;unsentDirectory?:string;policy?:typeof RETRY;leaves?:ClosurePortV1|undefined}>):ProjectFleetPortV1|undefined{
 const settings=input.client===undefined?graphFleetSettingsV1(input.root,input.environment):undefined;
 if(input.client===undefined&&settings===undefined)return undefined;
 const client=input.client??graphFleetClientV1(settings!,{fetch:remoteCacheFetchV1(input.environment,{root:input.root})}),platform=input.platform??graphPlatformV1(),owner=input.owner??`${hostname()}:${process.pid}:${randomBytes(3).toString("hex")}`,pending=new Set<Promise<unknown>>(),counts={seeded:0,published:0,offered:0,waited:0,computed:0,closures:0,refused:0,reinferred:0,diverged:0,resent:0,kept:0,shed:0,refreshed:0,retried:0,leaves:0,incomplete:0},host=hostOfRootV1(input.root),retry=input.policy??RETRY,clock=input.now??Date.now,pause=input.sleep??((milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds))),draw=input.random??Math.random,unsentDirectory=input.unsentDirectory??machinePathV1(input.root,".cache","f",retry.unsent.directory);
 const gatePhases=Object.keys(schema.definitions.GateTimings.properties) as GatePhaseV1[];
 const gateTiming=Object.fromEntries(gatePhases.map(phase=>[phase,{started:0,active:0,milliseconds:0}])) as GateTimingsV1;
 const timedGate=async<T>(phase:GatePhaseV1,run:()=>Promise<T>):Promise<T>=>{
  const row=gateTiming[phase],started=performance.now();
  row.started++;row.active++;
  try{return await run();}finally{row.active--;row.milliseconds+=performance.now()-started;}
 };
 const gateClient:GraphFleetClientV1={
  get settings(){return client.settings;},
  usable:()=>client.usable(),
  claimsUsable:()=>client.claimsUsable(),
  getResult:(...args)=>timedGate("metadata",()=>client.getResult(...args)),
  getBlob:(...args)=>timedGate("blob",()=>client.getBlob(...args)),
  hasBlob:(...args)=>client.hasBlob(...args),
  putBlob:(...args)=>client.putBlob(...args),
  putResult:(...args)=>client.putResult(...args),
  claim:(...args)=>timedGate("claim",()=>client.claim(...args)),
  renew:(...args)=>client.renew(...args),
  release:(...args)=>client.release(...args),
  await:(...args)=>timedGate("peerWait",()=>client.await(...args)),
 };
 const leafPort=input.leaves??((base:ClosurePortV1|undefined)=>base===undefined?undefined:leafPortWithDirectPutV1(base))(workspaceClosurePortV1(input.root)),incompleteDigests=new Set<string>(),leafWidth=leafWidthV1();
 const leavesOf=(documents:readonly Readonly<{name:string;bytes:Uint8Array}>[])=>{const record=documents.find(row=>row.name==="record");return leafPort===undefined||record===undefined?{}:{leaves:leafSourcesV1(leafPort,Buffer.from(record.bytes).toString("utf8")),leafWidth};};
 const say=input.say??((line:string)=>{try{process.stderr.write(line+"\n");}catch{}}),language=(input.environment.SEMIO_LANGUAGE==="de"?"de":"en") as "en"|"de";
 const decided=()=>{if((counts.seeded+counts.computed)%50===0)say(schema.definitions.Texts.const[language].progress.replace(/\{(\w+)\}/gu,(_all,name:string)=>String((counts as Record<string,number>)[name]??0)));};
 const stateOf=(info:ProjectRecordInfoV1)=>projectRecordStateV1({root:input.root,fleet:info.fleet,keyText:info.keyText,platform});
 const track=<T>(work:Promise<T>):Promise<T>=>{pending.add(work);void work.finally(()=>pending.delete(work)).catch(()=>undefined);return work;};
 const producers:Record<string,number>={},packed=new Map<string,Readonly<{document:Readonly<{name:string;bytes:Uint8Array}>;consulted:ConsultedKeysV1}>>();
 const providersOf=async():Promise<readonly string[]>=>{try{return await(input.providers??registeredProvidersV1)();}catch{return[];}};
 const closureDocsOf=async(info:ProjectRecordInfoV1,image:string,keys:ReadonlyMap<string,string>)=>{
  if(info.closures===undefined)return[];
  try{
   const bundles=(await info.closures.export(input.root,image)) as readonly Readonly<{recipeKey:string;stamp?:string;bundle:ClosureBundleV1}>[],all=new Map([...keys,...bundleKeysV1(bundles,host,"neutral")]);
   const ordered=[...bundles].sort((left,right)=>left.recipeKey<right.recipeKey?-1:1),documents:Readonly<{name:string;bytes:Uint8Array}>[]=[];let used=0;
   for(const [index,entry] of ordered.entries()){
    const id=entry.stamp===undefined?undefined:entry.recipeKey+":"+entry.stamp,kept=id===undefined?undefined:packed.get(id);
    let document:Readonly<{name:string;bytes:Uint8Array}>|undefined;
    if(kept!==undefined&&consultedHoldV1(kept.consulted,all))document=kept.document;
    else{
     const moved=packBundleV1(entry,host,all);
     if(moved===undefined)continue;
     document={name:`closure-${moved.recipeKey}`,bytes:Buffer.from(JSON.stringify(moved.document))};
     if(id!==undefined){packed.delete(id);packed.set(id,{document,consulted:moved.consulted});while(packed.size>MAXIMUM_PACKED)packed.delete(packed.keys().next().value!);}
    }
    if(used+document.bytes.length>retry.closureBudgetBytes){counts.shed+=ordered.length-index;break;}
    used+=document.bytes.length;documents.push(document);
   }
   return documents.sort((left,right)=>left.name<right.name?-1:1);
  }catch{return[];}
 };
 const documentsOf=async(info:ProjectRecordInfoV1)=>{
  const portable=info.exportRecord() as PortableRecordV1|undefined;if(portable===undefined)return undefined;
  let neutral:ReturnType<typeof neutralRecordV1>;
  try{neutral=neutralRecordV1(portable,host);}catch{counts.refused++;return undefined;}
  return[{name:"record",bytes:Buffer.from(JSON.stringify(packRecordV1(neutral.record)))},...await closureDocsOf(info,portable.image,neutral.keys)];
 };
 const heldOf=(info:ProjectRecordInfoV1,held:Map<string,string>)=>async(entry:Readonly<{path:string;hash:string}>)=>{
  const named=/^closure-([0-9a-f]{64})\.br$/u.exec(entry.path);
  if(named===null||info.closures?.held===undefined)return false;
   try{const local=await timedGate("held",()=>info.closures!.held!(input.root,entry.hash,named[1]!));if(local===undefined)return false;held.set(named[1]!,local);return true;}catch{return false;}
 };
 const recipeMemo=new Map<string,Readonly<{recipe:unknown;owned:boolean}>>(),importing=new Map<string,Readonly<{work:Promise<number>;signal?:AbortSignal}>>(),waiting:(()=>void)[]=[];let free=CLOSURE_WINDOW;
 const within=async<T>(work:()=>Promise<T>,signal?:AbortSignal):Promise<T>=>{
  signal?.throwIfAborted();
  if(free===0)await new Promise<void>((resolve,reject)=>{
   const grant=()=>{signal?.removeEventListener("abort",cancel);resolve();};
   const cancel=()=>{const index=waiting.indexOf(grant);if(index>=0)waiting.splice(index,1);reject(signal?.reason);};
   waiting.push(grant);signal?.addEventListener("abort",cancel,{once:true});
  });else free--;
  try{signal?.throwIfAborted();return await work();}finally{const next=waiting.shift();if(next!==undefined)next();else free++;}
 };
 const laneOf=(decision:Readonly<{documents:readonly Readonly<{name:string;bytes:Uint8Array}>[];deferred?:ReadonlyMap<string,GraphDeferredV1>}>,sources:ReadonlyMap<string,string>)=>{
  const lane=new Map<string,GraphDeferredV1>(decision.deferred??[]);
  for(const row of decision.documents)if(isClosureDocument(row.name))lane.set(row.name,{hash:sources.get(row.name),size:row.bytes.length,read:async()=>row});
  return new Map([...lane].sort((left,right)=>left[0]<right[0]?-1:1));
 };
 const recipesOf=async(lane:ReadonlyMap<string,GraphDeferredV1>,signal?:AbortSignal)=>{
  const recipes=new Map<string,unknown>();let owned=false;
  for(const [name,entry] of lane){
   signal?.throwIfAborted();
   const known=entry.hash===undefined?undefined:recipeMemo.get(entry.hash);
   if(known!==undefined){recipes.set(name.slice(8),known.recipe);owned||=known.owned;continue;}
   await within(async()=>{
    const document=await entry.read({keep:true});if(document===undefined)return;
    try{
     const head=closureDocumentHeadV1(document.bytes);
     if(head===undefined)return;
     recipes.set(name.slice(8),head.recipe);owned||=head.owned;
     if(entry.hash!==undefined){recipeMemo.delete(entry.hash);recipeMemo.set(entry.hash,{recipe:head.recipe,owned:head.owned});while(recipeMemo.size>MAXIMUM_RECIPES)recipeMemo.delete(recipeMemo.keys().next().value!);}
    }catch{}
   },signal);
  }
  return{recipes,owned};
 };
 const takeClosures=async(info:ProjectRecordInfoV1,lane:ReadonlyMap<string,GraphDeferredV1>,recipes:ReadonlyMap<string,unknown>,held:ReadonlyMap<string,string>,keys:ReadonlyMap<string,string>,context:Readonly<{providers:readonly string[];generation:string|undefined}>,signal?:AbortSignal)=>{
  if(info.closures===undefined)return;
  try{
   const all=new Map([...keys,...held,...bundleKeysV1([...recipes].map(([recipeKey,recipe])=>({recipeKey,bundle:{recipe} as unknown as ClosureBundleV1})),host,"local")]);
   for(const [name,entry] of lane){
    signal?.throwIfAborted();
    const hash=entry.hash,flying=hash===undefined?undefined:importing.get(hash);
    if(flying!==undefined&&flying.signal===signal){await flying.work.catch(()=>undefined);continue;}
    const work=within(async()=>{
     const document=await entry.read();if(document===undefined)return 0;
     signal?.throwIfAborted();
     let neutral:Readonly<{recipeKey:string;bundle:ClosureBundleV1}>;
     try{neutral={recipeKey:name.slice(8),bundle:unpackBundleV1(JSON.parse(view(document.bytes).toString("utf8")) as PackedClosureV1)};}catch{return 0;}
     const moved=localBundleV1(neutral,host,all,context);
     return moved===undefined?0:await info.closures!.import(input.root,[{...moved,...hash===undefined?{}:{source:hash},neutral:name.slice(8)}]);
    },signal);
    if(hash!==undefined)importing.set(hash,{work,signal});
    try{counts.closures+=await work;}catch{}finally{if(hash!==undefined&&importing.get(hash)?.work===work)importing.delete(hash);}
   }
  }catch{}
 };
 const seedFrom=async(info:ProjectRecordInfoV1,state:GraphStateV1,decision:Readonly<{documents:readonly Readonly<{name:string;bytes:Uint8Array}>[];deferred?:ReadonlyMap<string,GraphDeferredV1>;note:GraphEntryNoteV1;via:string}>,held:Map<string,string>,signal?:AbortSignal):Promise<boolean>=>{
  const document=decision.documents.find(row=>row.name==="record"),look=takeScopes.getStore();
  if(document===undefined&&look!==undefined)look.fault="record";
  try{
   if(document!==undefined){
    let neutral:ReturnType<typeof unpackRecordV1>;
    try{neutral=unpackRecordV1(JSON.parse(Buffer.from(document.bytes).toString("utf8")) as PackedRecordV1);}catch(error){if(look!==undefined)look.fault="payload";throw error;}
    const sources=new Map(decision.note.documents.flatMap(row=>row.name.endsWith(".br")?[[row.name.slice(0,-3),row.hash] as const]:[])),lane=info.closures===undefined?new Map<string,GraphDeferredV1>():laneOf(decision,sources),found=await recipesOf(lane,signal);
    signal?.throwIfAborted();
    const owned=decision.documents.some(row=>view(row.bytes).includes(portableRowsPolicyV1.ownerToken))||found.owned,context={providers:owned?await providersOf():[],generation:(input.generation??dependencyGenerationV1)(input.root)},taken=localRecordV1(neutral,host,context);
    await takeClosures(info,lane,found.recipes,held,taken.keys,context,signal);
    signal?.throwIfAborted();
    const listed=decision.note.leaves??[];
    if(listed.length>0&&leafPort!==undefined){
      const adopted=await adoptLeavesV1({client:gateClient,port:leafPort,leaves:listed,width:leafWidth,signal});
     signal?.throwIfAborted();
     counts.leaves+=adopted.fetched;
     if(adopted.missing.length>0){
      counts.incomplete++;incompleteDigests.add(state.digest);
      const store=takeScopes.getStore();if(store!==undefined)store.outcome="incomplete";
      say((["en","de"] as const).map(tongue=>schemaLeaves.definitions.Texts.const[tongue].incomplete.replace("{name}",info.fleet).replace("{count}",String(adopted.missing.length)).replace("{leaves}",adopted.missing.slice(0,3).map(leaf=>leaf.hash.slice(0,12)).join(", "))).join(" | "));
      return false;
     }
    }
    const ok=await info.import(taken.record);
    signal?.throwIfAborted();
    if(ok){counts.seeded++;{const origin=decision.note.producer.split(":")[0]??"";if(origin!=="")producers[origin]=(producers[origin]??0)+1;}decided();if(decision.via==="wait")counts.waited++;mark(info,state);return true;}
   }
  }catch{signal?.throwIfAborted();counts.refused++;}
  return false;
 };
 const freshDirectory=machinePathV1(input.root,".cache","f",FRESH_POLICY_V1.directory),verdicts=new Map<string,Readonly<{fresh:boolean;atMs:number}>>();
 const keptFresh=(noteHash:string)=>{try{return Date.now()-statSync(join(freshDirectory,noteHash)).mtimeMs<FRESH_POLICY_V1.keptMilliseconds;}catch{return false;}};
 const keepFresh=(noteHash:string)=>{try{mkdirSync(freshDirectory,{recursive:true});writeFileSync(join(freshDirectory,noteHash),"");if(draw()<0.02)for(const name of readdirSync(freshDirectory))if(Date.now()-statSync(join(freshDirectory,name)).mtimeMs>FRESH_POLICY_V1.keptMilliseconds*6)unlinkSync(join(freshDirectory,name));}catch{}};
 const closureVerdicts=new Map<string,Readonly<{holds:boolean;atMs:number}>>();
 const closuresHoldOf=async(decision:Readonly<{documents:readonly Readonly<{name:string;bytes:Uint8Array}>[];deferred:ReadonlyMap<string,GraphDeferredV1>;note:GraphEntryNoteV1}>,keys:ReadonlyMap<string,string>,owned:boolean):Promise<boolean>=>{
  const lane=laneOf(decision,new Map(decision.note.documents.flatMap(row=>row.name.endsWith(".br")?[[row.name.slice(0,-3),row.hash] as const]:[])));
  if(lane.size===0)return true;
  const found=await recipesOf(lane),context={providers:owned||found.owned?await providersOf():[],generation:(input.generation??dependencyGenerationV1)(input.root)},all=new Map([...keys,...bundleKeysV1([...found.recipes].map(([recipeKey,recipe])=>({recipeKey,bundle:{recipe} as unknown as ClosureBundleV1})),host,"local")]);
  for(const [name,entry] of lane){
   const known=entry.hash===undefined?undefined:closureVerdicts.get(entry.hash);
   if(known!==undefined&&Date.now()-known.atMs<FRESH_POLICY_V1.keptMilliseconds){if(!known.holds)return false;continue;}
   const holds=await within(async()=>{
    const document=await entry.read({keep:true});if(document===undefined)return true;
    try{
     const moved=localBundleV1({recipeKey:name.slice(8),bundle:unpackBundleV1(JSON.parse(view(document.bytes).toString("utf8")) as PackedClosureV1)},host,all,context);
     if(moved===undefined)return false;
     return await closureBundleHoldsV1(moved.bundle);
    }catch{return false;}
   });
   if(entry.hash!==undefined){closureVerdicts.delete(entry.hash);closureVerdicts.set(entry.hash,{holds,atMs:Date.now()});while(closureVerdicts.size>FRESH_POLICY_V1.verdictsKept)closureVerdicts.delete(closureVerdicts.keys().next().value!);}
   if(!holds)return false;
  }
  return true;
 };
 const staleNoteOf=async(state:GraphStateV1):Promise<string|undefined>=>{
  try{
   const found=await fetchGraphEntryV1(client,state,entry=>entry.path!=="record.br"&&!isClosureDocument(entry.path),isClosureDocument);
   if(found===undefined||found==="fault")return undefined;
   const noteHash=found.note.documents.find(row=>row.name==="record.br")?.hash,document=found.documents.find(row=>row.name==="record");
   if(noteHash===undefined||document===undefined)return undefined;
   const known=verdicts.get(noteHash);
   if(known!==undefined&&(!known.fresh||Date.now()-known.atMs<FRESH_POLICY_V1.keptMilliseconds))return known.fresh?undefined:noteHash;
   if(keptFresh(noteHash))return undefined;
   const text=Buffer.from(document.bytes).toString("utf8"),taken=localRecordV1(unpackRecordV1(JSON.parse(text) as PackedRecordV1),host,{providers:text.includes(portableRowsPolicyV1.ownerToken)?await providersOf():[],generation:(input.generation??dependencyGenerationV1)(input.root)});
   const verdict=freshVerdictV1({rows:taken.record.observations,hashOf:fileDigestV1,closureHolds:await closureHoldsOfV1(taken.record.observations,taken.record.image),rowHolds:await rowHoldsOfV1(),createdMs:found.note.createdMs,now:clock()});
   const fresh=verdict.fresh&&(found.deferred.size===0&&!found.documents.some(row=>isClosureDocument(row.name))||clock()-found.note.createdMs<FRESH_POLICY_V1.minimumAgeMilliseconds||await closuresHoldOf(found,taken.keys,text.includes(portableRowsPolicyV1.ownerToken)));
   if(fresh)keepFresh(noteHash);verdicts.set(noteHash,{fresh,atMs:Date.now()});while(verdicts.size>FRESH_POLICY_V1.verdictsKept)verdicts.delete(verdicts.keys().next().value!);
   return fresh?undefined:noteHash;
  }catch{return undefined;}
 };
 let nextResend=0;
 const marker=(info:ProjectRecordInfoV1)=>projectRecordMarkerPathV1(info.cachePath);
 const versionOf=(info:ProjectRecordInfoV1,state:GraphStateV1)=>{const form=info.closures===undefined?"":":closures-1";try{const status=statSync(info.cachePath);return`${state.digest}:${status.size}:${Math.floor(status.mtimeMs)}${form}`;}catch{return state.digest+form;}};
 const mark=(info:ProjectRecordInfoV1,state:GraphStateV1)=>{try{writeFileSync(marker(info),versionOf(info,state));}catch{}};
 const send=async(state:GraphStateV1,documents:readonly Readonly<{name:string;bytes:Uint8Array}>[],start:GraphHoldV1|undefined,attempts:number,head:boolean=false):Promise<Readonly<{stored:true;refusal?:GraphRefusalV1}>|Readonly<{stored:false;refusal:GraphRefusalV1}>>=>{
  let hold=start,last:GraphRefusalV1|undefined;
  for(let attempt=0;attempt<attempts;attempt++){
   const outcome=await publishGraphEntryOutcomeV1({client,state,owner,documents,...leavesOf(documents),keepHold:true,...(head?{head:true}:{}),...(hold===undefined?{}:{hold})});
   if(outcome.stored){await hold?.release().catch(()=>undefined);return{stored:true};}
   last=outcome.refusal;
   if(last.stored){await hold?.release().catch(()=>undefined);counts.diverged++;return{stored:true,refusal:last};}
   if(!last.transient||attempt===attempts-1)break;
   await pause(projectPublishWaitV1(attempt,client.breakerRemainingMilliseconds?.()??0,draw(),retry));
   if(hold?.lost()===true||last.code==="claim-lost"){
    const again=await tryGraphEntryV1({client,state,owner,ttlMilliseconds:retry.unsent.claimTtlMilliseconds});
    hold?.stop();
    if(again.outcome==="stored")return{stored:true,refusal:last};
    if(again.outcome==="held"){hold=undefined;break;}
    hold=again.outcome==="granted"?again.hold:undefined;
   }
  }
  await hold?.release().catch(()=>undefined);
  return{stored:false,refusal:last!};
 };
 const publisher=(info:ProjectRecordInfoV1,state:GraphStateV1,hold:GraphHoldV1|undefined,seeded:boolean=false)=>{let done=false;return{
  publish(){if(done)return;done=true;if(seeded)counts.reinferred++;void track((async()=>{const documents=await documentsOf(info);if(documents===undefined){await hold?.release().catch(()=>undefined);return false;}const stored=await publishGraphEntryV1({client,state,owner,documents,...leavesOf(documents),...(seeded?{head:true}:{}),...(hold===undefined?{}:{hold})});if(stored){counts.published++;mark(info,state);}return stored;})());},
  async abandon(){if(done)return;done=true;await hold?.release().catch(()=>undefined);}};};
 return Object.freeze({
  producers:()=>({...producers}),
  async claim(info){
   const state=stateOf(info),ask=(over:GraphStateV1)=>tryGraphEntryV1({client,state:over,owner,...(info.expectedMilliseconds===undefined?{}:{expectedMilliseconds:info.expectedMilliseconds}),...(info.ttlMilliseconds===undefined?{}:{ttlMilliseconds:info.ttlMilliseconds})});
   let answer=await ask(state),head=false;
   if(answer.outcome==="stored"){
    const stale=await staleNoteOf(state);
    if(stale===undefined)return{state:"stored"};
    const again=await ask(refreshStateV1(state,stale));
    if(again.outcome==="held")return{state:"held",by:again.by};
    if(again.outcome!=="granted")return again.outcome==="fault"?{state:"fault"}:{state:"stored"};
    answer=again;head=true;counts.refreshed++;
   }
   if(answer.outcome==="held")return{state:"held",by:answer.by};
   if(answer.outcome==="fault")return{state:"fault"};
   if(answer.outcome!=="granted")return{state:"stored"};
   const hold=answer.hold;let done=false,why="",sure=false,code="";
   return{state:"granted",why:()=>why,code:()=>code,deterministic:()=>sure,digest:state.digest,
    async publish(){if(done)return false;done=true;const stored=await track((async()=>{
     const documents=await documentsOf(info);
     if(documents===undefined){const missing=info.exportRecord()===undefined;sure=!missing;why=missing?"the inference left no record to export (inputs changed while it ran)":"the record names the root in a spelling that cannot be shared";code=missing?"no-record":"unshareable";await hold.release().catch(()=>undefined);return false;}
     const result=await send(state,documents,hold,retry.attempts,head);
     if(result.stored){if(result.refusal===undefined)counts.published++;else{code=result.refusal.code;why=explain(result.refusal,documents);}mark(info,state);return true;}
     const refusal=result.refusal;code=refusal.code;sure=refusal.deterministic;why=explain(refusal,documents);
     if(!refusal.deterministic&&!refusal.stored&&refusal.code!=="no-upload"&&refusal.code!=="leaf-missing"&&unsentPutV1(unsentDirectory,{state,owner,documents,cause:why,attempts:retry.attempts,now:clock()},retry.unsent)){counts.kept++;why+=" - kept on this machine for a later try / auf dieser Maschine für einen späteren Versuch behalten";}
     return false;
    })());return stored;},
    async abandon(){if(done)return;done=true;await hold.release().catch(()=>undefined);}};
  },
  async gate(info){
   const held=new Map<string,string>(),state=stateOf(info),look=takeScopes.getStore(),nothing={publish(){},async abandon(){}};
   if(look!==undefined){
      const found=await fetchGraphEntryV1(gateClient,state,heldOf(info,held),info.closures===undefined?undefined:isClosureDocument,look.signal,reason=>{look.fault=reason;});
    if(found===undefined){look.outcome="miss";return{seeded:false,...nothing};}
    if(found==="fault"){look.outcome="fault";return{seeded:false,...nothing};}
    look.outcome="fetched";
     return{seeded:await timedGate("seed",()=>seedFrom(info,state,{documents:found.documents,deferred:found.deferred,note:found.note,via:"cache"},held,look.signal)),...nothing};
   }
    const decide=():Promise<Awaited<ReturnType<typeof decideGraphEntryV1>>|Readonly<{outcome:"skip"}>>=>decideGraphEntryV1({client:gateClient,state,owner,skip:heldOf(info,held),...input.busy===undefined?{}:{busy:input.busy},...input.strong===undefined?{}:{strong:input.strong},...info.closures===undefined?{}:{defer:isClosureDocument}});
   let decision=await decide();
   if(decision.outcome==="skip")return{seeded:false,deferred:true,...nothing};
    for(let attempt=0;attempt<2&&decision.outcome==="compute"&&decision.reason==="fault";attempt++){counts.retried++;await timedGate("retry",()=>pause(projectPublishWaitV1(attempt,client.breakerRemainingMilliseconds?.()??0,draw(),retry)));decision=await decide();}
    if(decision.outcome==="hit"){const hit=decision,seeded=await timedGate("seed",()=>seedFrom(info,state,hit,held));return{seeded,...publisher(info,state,undefined,seeded||incompleteDigests.delete(state.digest))};}
   counts.computed++;decided();
   return{seeded:false,...publisher(info,state,decision.hold)};
  },
  async offer(info){
   if(!client.settings.upload||!client.usable())return;
   while(pending.size>=MAXIMUM_OFFERS)await Promise.race([...pending].map(work=>work.catch(()=>undefined)));
   const state=stateOf(info);
   try{if(existsSync(marker(info))&&readFileSync(marker(info),"utf8")===versionOf(info,state))return;}catch{}
   void track((async()=>{
    const documents=await documentsOf(info);if(documents===undefined)return false;
    const stored=await publishGraphEntryV1({client,state,owner,documents,...leavesOf(documents)});if(stored){counts.offered++;mark(info,state);}return stored;
   })().catch(()=>false));
  },
  async resend(options){
   const at=clock();
   if(options?.force!==true&&at<nextResend)return 0;
   nextResend=at+retry.unsent.resendEveryMilliseconds;
   if(!client.settings.upload||!client.usable())return 0;
   let sent=0;unsentPruneV1(unsentDirectory,at,0,retry.unsent);
   for(const row of unsentListV1(unsentDirectory).slice(0,retry.unsent.resendPerLook)){
    const record=unsentReadV1(unsentDirectory,row.digest);
    if(record===undefined){unsentDropV1(unsentDirectory,row.digest);continue;}
    const tried=await tryGraphEntryV1({client,state:record.meta.state,owner,ttlMilliseconds:retry.unsent.claimTtlMilliseconds});
    if(tried.outcome==="stored"){unsentDropV1(unsentDirectory,row.digest);counts.resent++;sent++;continue;}
    if(tried.outcome!=="granted")continue;
    const result=await send(record.meta.state,record.documents,tried.hold,1);
    if(result.stored){unsentDropV1(unsentDirectory,row.digest);counts.resent++;sent++;}
    else if(result.refusal.deterministic)unsentDropV1(unsentDirectory,row.digest);
    else unsentPutV1(unsentDirectory,{state:record.meta.state,owner,documents:record.documents,cause:explain(result.refusal,record.documents),attempts:record.meta.attempts+1,now:clock()},retry.unsent);
   }
   return sent;
  },
  async flush(){await Promise.allSettled([...pending]);const texts=schema.definitions.Texts.const;return(["en","de"] as const).map(language=>texts[language].summary.replace(/\{(\w+)\}/gu,(_all,name:string)=>String((counts as Record<string,number>)[name]??0))).join(" | ");},
   stats:()=>Object.freeze({...counts,gateTimings:Object.fromEntries(gatePhases.map(phase=>[phase,{...gateTiming[phase],milliseconds:Math.round(gateTiming[phase].milliseconds)}])) as GateTimingsV1}),
 });
}
