import{createHash,randomBytes}from"node:crypto";
import{hostname}from"node:os";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{graphPlatformV1,type GraphStateV1}from"../../🟦️.ts";
import{recordPlatformV1}from"../../🌍️platform-class/🟦️.ts";
import{bundleKeysV1,closureDocumentHeadV1,closureKeyV1,dependencyGenerationV1,hostOfRootV1,localBundleV1,neutralTextV1,packBundleV1,portableRowsPolicyV1,unpackBundleV1,type ClosureBundleV1,type PackedClosureV1}from"../🧳️portable/🟦️.ts";
import{decideGraphEntryV1,fetchGraphEntryV1,graphFleetClientV1,publishGraphEntryOutcomeV1,type GraphDeferredV1,type GraphEntryNoteV1,type GraphFleetClientV1,type GraphHoldV1}from"../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../🧶️nx/🟦️.ts";
import{remoteCacheFetchV1}from"../../../🌐️remote-cache/🛰️fleet/🟦️.ts";

/** 🚛️ How closure entries are shared on their own: the wait for another machine's derivation, the expected derivation time of a claim, the fetch width and the offers in flight. See `./🧬️schema/🔣️.json`. */
export const closureEntryPolicyV1=Object.freeze({...schema.definitions.Policy.const});
type Environment=Readonly<Record<string,string|undefined>>;
type Bundles=readonly Readonly<{recipeKey:string;stamp?:string;bundle:unknown}>[];
/** 🚚️ The two operations of the closure store the lane needs, as the inference registers them (`closureTravelV1`): the bundles a recipe table names, and the import of bundles into this workspace's store. */
export type ClosureEntryTravelV1=Readonly<{export(root:string,image:string):Promise<Bundles>;import(root:string,bundles:readonly Readonly<{recipeKey:string;bundle:unknown;source?:string;neutral?:string}>[]):Promise<number>;held?(root:string,source:string,neutral?:string,refusedMilliseconds?:number):Promise<string|undefined>;claim?(root:string,neutral:string,staleMilliseconds:number):Promise<(()=>Promise<void>)|undefined>;claimed?(root:string,neutral:string):Promise<boolean>}>;
const ZERO="0".repeat(64),sha=(text:string)=>createHash("sha256").update(text).digest("hex"),DOCUMENT=/^closure-([0-9a-f]{64})\.br$/u;

/** 🔑️ The name of a closure entry on every machine: the digest of its recipe with the checkout root replaced by the root token. Two checkouts at different paths whose recipes differ only in the root have the same key. */
export function closureEntryKeyV1(recipe:unknown,root:string):string{
 const host=hostOfRootV1(root);
 return closureKeyV1(JSON.parse(neutralTextV1(JSON.stringify(recipe),host)));
}

/** 🏷️ The state an entry is shared under: its key, and the platform tag of a record whose recipe names a host-bound declaration, the neutral tag otherwise; one slot per key, so a newer entry supersedes only its own older one. */
export function closureEntryStateV1(input:Readonly<{root:string;recipe:unknown;platform:string}>):GraphStateV1{
 const key=closureEntryKeyV1(input.recipe,input.root),platform=recordPlatformV1(JSON.stringify(input.recipe),input.platform),tooling=sha(JSON.stringify([closureEntryPolicyV1.form,key]));
 return Object.freeze({version:1,frame:Object.freeze({kind:"project-inference" as const,tooling,implementation:ZERO,platform,extra:Object.freeze({closureEntry:key})}),names:ZERO,content:ZERO,files:0,contentFiles:0,digest:sha(JSON.stringify([closureEntryPolicyV1.form,key,platform]))});
}

/** 🚚️ The fleet lane of closure entries of one process: take an entry another machine derived (claiming its derivation when nobody did, waiting for the holder when somebody did), offer an entry this machine derived, and take a batch before any derivation. Every fault ends in "derive here". */
export type ClosureEntryLaneV1=Readonly<{
 take(recipe:unknown,travel:ClosureEntryTravelV1,signal?:AbortSignal):Promise<Readonly<{outcome:"taken"}|{outcome:"compute";hold:GraphHoldV1|undefined}>>;
 offer(recipe:unknown,travel:ClosureEntryTravelV1,hold?:GraphHoldV1):Promise<boolean>;
 prefetch(recipes:readonly unknown[],travel:ClosureEntryTravelV1,signal?:AbortSignal):Promise<number>;
 flush():Promise<string>;
 stats():Readonly<Record<"fetched"|"prefetched"|"computed"|"published"|"refused"|"missed"|"imported",number>>;
}>;

/** 🛻️ Opens the lane of this process, or undefined when no fleet cache is configured. */
export function createClosureEntryLaneV1(input:Readonly<{root:string;environment:Environment;client?:GraphFleetClientV1;owner?:string;platform?:string;providers?():Promise<readonly string[]>;generation?(root:string):string|undefined}>):ClosureEntryLaneV1|undefined{
 const settings=input.client===undefined?graphFleetSettingsV1(input.root,input.environment):undefined;
 if(input.client===undefined&&settings===undefined)return undefined;
 const client=input.client??graphFleetClientV1(settings!,{fetch:remoteCacheFetchV1(input.environment,{root:input.root})}),platform=input.platform??graphPlatformV1(),owner=input.owner??`${hostname()}:${process.pid}:${randomBytes(3).toString("hex")}`,host=hostOfRootV1(input.root),policy=closureEntryPolicyV1;
 const counts={fetched:0,prefetched:0,computed:0,published:0,refused:0,missed:0,imported:0},pending=new Set<Promise<unknown>>();let waitedMilliseconds=0;
 const track=<T>(work:Promise<T>):Promise<T>=>{pending.add(work);void work.finally(()=>pending.delete(work)).catch(()=>undefined);return work;};
 const stateOf=(recipe:unknown)=>closureEntryStateV1({root:input.root,recipe,platform});
 const providersOf=async():Promise<readonly string[]>=>{try{return await(input.providers??defaultProvidersV1)();}catch{return[];}};
 type Scope=Readonly<{held:Map<string,string>;claims:Map<string,()=>Promise<void>>}>;
 const scopeOf=():Scope=>({held:new Map(),claims:new Map()});
 const releaseOf=async(scope:Scope,names:Iterable<string>=[...scope.claims.keys()])=>{for(const name of names){const release=scope.claims.get(name);if(release===undefined)continue;scope.claims.delete(name);await release().catch(()=>undefined);}};
 const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
 const heldOf=(travel:ClosureEntryTravelV1,scope:Scope,signal?:AbortSignal)=>async(entry:Readonly<{path:string;hash:string}>)=>{
  const named=DOCUMENT.exec(entry.path);
  if(named===null||travel.held===undefined)return false;
  const neutral=named[1]!;
  if(scope.claims.has(neutral))return false;
  try{
   signal?.throwIfAborted();
   let local=await travel.held(input.root,entry.hash,neutral,policy.refusedMilliseconds);
   signal?.throwIfAborted();
   if(local===undefined&&travel.claim!==undefined){
    const release=await travel.claim(input.root,neutral,policy.claimStaleMilliseconds);
    if(release!==undefined){scope.claims.set(neutral,release);return false;}
    const until=Date.now()+policy.claimWaitMilliseconds;
    while(travel.claimed!==undefined&&Date.now()<until&&await travel.claimed(input.root,neutral)){signal?.throwIfAborted();await pause(150);}
    signal?.throwIfAborted();
    local=await travel.held(input.root,entry.hash,neutral,policy.refusedMilliseconds);
   }
   if(local===undefined)return false;
   scope.held.set(neutral,local);return true;
  }catch{return false;}
 };
 const documentsOf=async(recipe:unknown,travel:ClosureEntryTravelV1)=>{
  try{
   const local=closureKeyV1(recipe),bundles=(await travel.export(input.root,JSON.stringify({closureRecipes:{[local]:recipe}}))) as readonly Readonly<{recipeKey:string;stamp?:string;bundle:ClosureBundleV1}>[];
   if(!bundles.some(row=>row.recipeKey===local))return undefined;
   const keys=bundleKeysV1(bundles,host,"neutral"),documents:Readonly<{name:string;bytes:Uint8Array}>[]=[];
   for(const entry of [...bundles].sort((left,right)=>left.recipeKey<right.recipeKey?-1:1)){
    const moved=packBundleV1(entry,host,keys);
    if(moved===undefined){if(entry.recipeKey===local)return undefined;continue;}
    documents.push({name:`closure-${moved.recipeKey}`,bytes:Buffer.from(JSON.stringify(moved.document))});
   }
   return documents.sort((left,right)=>left.name<right.name?-1:1);
  }catch{return undefined;}
 };
 const unpackOf=(row:Readonly<{name:string;bytes:Uint8Array}>)=>{try{return{recipeKey:row.name.slice(8),bundle:unpackBundleV1(JSON.parse(Buffer.from(row.bytes.buffer,row.bytes.byteOffset,row.bytes.byteLength).toString("utf8")) as PackedClosureV1)};}catch{return undefined;}};
 let queue:Promise<unknown>=Promise.resolve();
 const serial=<T>(work:()=>Promise<T>):Promise<T>=>{const run=queue.then(work,work);queue=run.catch(()=>undefined);return run;};
 const closureName=(name:string)=>name.startsWith("closure-");
 const intake=(deferred:ReadonlyMap<string,GraphDeferredV1>,travel:ClosureEntryTravelV1,scope:Scope,signal?:AbortSignal)=>serial(async()=>{
  const names=[...deferred.keys()].filter(closureName).sort();
  try{
   const found=new Map<string,string>(scope.held);let any=false,owned=false;
   for(const name of names){
    signal?.throwIfAborted();
    const document=await deferred.get(name)!.read({keep:true});if(document===undefined)continue;
    const head=closureDocumentHeadV1(document.bytes);if(head===undefined)continue;
    any=true;owned||=head.owned;
    for(const [from,to] of bundleKeysV1([{recipeKey:name.slice(8),bundle:{recipe:head.recipe} as unknown as ClosureBundleV1}],host,"local"))found.set(from,to);
   }
   if(!any)return 0;
   const context={providers:owned?await providersOf():[],generation:(input.generation??dependencyGenerationV1)(input.root)};
   let taken=0;
   for(const name of names){
    try{
     signal?.throwIfAborted();
     const waiting=deferred.get(name)!,document=await waiting.read();if(document===undefined)continue;
     const entry=unpackOf(document);if(entry===undefined)continue;
     const moved=localBundleV1(entry,host,found,context),source=waiting.hash;
     if(moved!==undefined)taken+=await travel.import(input.root,[{...moved,...source===undefined?{}:{source},neutral:entry.recipeKey}]);
    }finally{await releaseOf(scope,[name.slice(8)]);}
   }
   counts.imported+=taken;return taken;
  }catch{return 0;}finally{await releaseOf(scope,names.map(name=>name.slice(8)));}
 });
 const forEach=async<T>(items:readonly T[],width:number,work:(item:T)=>Promise<void>)=>{let next=0;await Promise.all(Array.from({length:Math.max(1,Math.min(width,items.length))},async()=>{while(next<items.length)await work(items[next++]!);}));};
 return Object.freeze({
  async take(recipe,travel,signal){
   const scope=scopeOf();
    let taken=false,hold:GraphHoldV1|undefined;
   try{
     signal?.throwIfAborted();
     const decision=await decideGraphEntryV1({client,state:stateOf(recipe),owner,signal,expectedMilliseconds:policy.expectedMilliseconds,waitLimitMilliseconds:Math.max(1,Math.min(policy.waitMilliseconds,policy.waitBudgetMilliseconds-waitedMilliseconds)),skip:heldOf(travel,scope,signal),defer:closureName});
    waitedMilliseconds+=decision.waitedMs;
     if(decision.outcome==="hit"){await intake(decision.deferred,travel,scope,signal);taken=true;}
     else hold=decision.hold;
    }catch{signal?.throwIfAborted();}finally{await releaseOf(scope);}
    if(signal?.aborted){await hold?.release().catch(()=>undefined);signal.throwIfAborted();}
    if(taken){counts.fetched++;return{outcome:"taken"} as const;}
    counts.computed++;return{outcome:"compute",hold} as const;
  },
  async offer(recipe,travel,hold){
   try{
    if(!client.settings.upload||!client.usable()){await hold?.release().catch(()=>undefined);return false;}
    while(pending.size>=policy.offersInFlight)await Promise.race([...pending].map(work=>work.catch(()=>undefined)));
    return await track((async()=>{
     const documents=await documentsOf(recipe,travel);
     if(documents===undefined){await hold?.release().catch(()=>undefined);counts.refused++;return false;}
     const outcome=await publishGraphEntryOutcomeV1({client,state:stateOf(recipe),owner,documents,head:true,...(hold===undefined?{}:{hold})});
     if(outcome.stored)counts.published++;else counts.refused++;
     return outcome.stored;
    })());
   }catch{await hold?.release().catch(()=>undefined);counts.refused++;return false;}
  },
  async prefetch(recipes,travel,signal){
   let taken=0;
   await forEach(recipes,policy.fetchConcurrency,async recipe=>{
    if(signal?.aborted)return;
    const scope=scopeOf();
    try{
     const found=await fetchGraphEntryV1(client,stateOf(recipe),heldOf(travel,scope,signal),closureName,signal);
     if(found===undefined||found==="fault"){counts.missed++;return;}
     if(await intake(found.deferred,travel,scope,signal)>0){taken++;counts.prefetched++;}
    }catch{if(!signal?.aborted)counts.missed++;}finally{await releaseOf(scope);}
   });
   signal?.throwIfAborted();
   return taken;
  },
  async flush(){await Promise.allSettled([...pending]);const texts=schema.definitions.Texts.const;return(["en","de"] as const).map(language=>texts[language].summary.replace(/\{(\w+)\}/gu,(_all,name:string)=>String((counts as Record<string,number>)[name]??0))).join(" | ");},
  stats:()=>Object.freeze({...counts}),
 });
}

const SOURCE_ANALYSES="../../../🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs",SCRIPT_INPUTS="../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
async function defaultProvidersV1():Promise<readonly string[]>{
 const loaded=await import(SOURCE_ANALYSES) as {prepareSourceAnalysesV1():Readonly<Record<string,{id:string}>>},own=await import(SCRIPT_INPUTS).then(module=>(module as {scriptProviderIdsV1():readonly string[]}).scriptProviderIdsV1(),()=>[] as readonly string[]);
 return[...Object.values(loaded.prepareSourceAnalysesV1()).map(provider=>provider.id),...own];
}
