import{staleImplementationV1}from"../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import{captureBoundedSourceRevisionV1,implementationPinnedV1,physicalSourceOriginV1}from"../../📥️inputs/🔏️revision/🟨️.mjs";
import{fileURLToPath}from"node:url";
import{lstatSync,openSync,fstatSync,closeSync,constants}from"node:fs";
import{resolve,join,parse,relative,sep}from"node:path";
import{withRepositoryBuildGraphConsumerV1}from"../../📥️inputs/💾️storage/🟨️.mjs";
import{types}from"node:util";
import{requireIssuedResourceControlV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{readInventoryRecordLeanV1,rememberInventoryRecordV1,inventoryRecordStillHoldsV1,inventoryRecordCorruptV1}from"../../📥️inputs/🪶️lean-inventory/🟨️.mjs";
import{captureSourceInputInventoryV1,captureCargoLockInventoryV1}from"../../📥️inputs/📋️data/📥️load/🟨️.mjs";
import{createPresenceCoalescerV1}from"./🧺️coalesce/🟨️.mjs";
import{createInferenceInventoryTimingsV1}from"./⏱️timing/🟨️.mjs";
import{observationRoundV1}from"../🟨️.mjs";
import{nativeStorageMappingQueueV1}from"../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔒️mapping/🟦️.ts";
const epochURLs=[new URL(physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),new URL("./🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),new URL("../../📥️inputs/🧮️digest/📥️read/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL))],epochCaptures=epochURLs.map((url,index)=>index===0?captureBoundedSourceRevisionV1(fileURLToPath(url),8388608):captureBoundedSourceRevisionV1(fileURLToPath(url),65536,true)),loadedEpoch=epochCaptures.map(row=>row.sha256),revision=(url,index)=>captureBoundedSourceRevisionV1(fileURLToPath(url),index===0?8388608:65536).sha256,policy=Object.freeze({...JSON.parse(epochCaptures[1].text??"").definitions.Policy.const}),readPolicy=Object.freeze({...JSON.parse(epochCaptures[2].text??"").definitions.Policy.const});
const reference=/^SEMIO_(?:SOURCE_CLOSURE|CARGO_LOCK)_[a-f0-9]{64}$/u,prefix=/^SEMIO_(?:SOURCE_CLOSURE|CARGO_LOCK)_/u;

/** 🧷️ Rejects a retained helper whose Source or privately captured policies changed. */
export function requireCurrentInferenceInventoryReuseV1(){if(implementationPinnedV1())return;if(epochURLs.some((url,index)=>revision(url,index)!==loadedEpoch[index]))throw staleImplementationV1("Stale inference inventory reuse implementation; restart required");}

/** 📋️ Captures finite inert compact references without retaining mutable caller DATA. */
export function captureInferenceInventoryReferencesV1(value){
 let values=0,text=0;const active=new Set(),names=new Set(),charge=value=>{text+=Buffer.byteLength(value);if(text>policy.maximumTextBytes)throw Error("Inference reference text capacity exceeded");};
 const visit=(current,depth)=>{
  if(++values>policy.maximumValues||depth>policy.maximumDepth)throw Error("Inference reference traversal capacity exceeded");
  if(typeof current==="string"){charge(current);return;}if(current===null||typeof current==="boolean"||typeof current==="number"&&Number.isFinite(current))return;
  if(typeof current!=="object"||types.isProxy(current)||active.has(current))throw Error("Inert acyclic inference DATA required");
  const array=Array.isArray(current),prototype=Object.getPrototypeOf(current);if(array?prototype!==Array.prototype:prototype!==Object.prototype&&prototype!==null)throw Error("Plain inference DATA required");const keys=Reflect.ownKeys(current);if(keys.length>policy.maximumMembers+(array?1:0)||keys.some(key=>typeof key!=="string"))throw Error("Bounded own inference members required");if(array&&(current.length>policy.maximumMembers||keys.length!==current.length+1))throw Error("Dense inference DATA array required");
  active.add(current);try{for(const key of keys){if(array&&key==="length")continue;if(array&&(!/^(?:0|[1-9][0-9]*)$/u.test(key)||Number(key)>=current.length))throw Error("Dense inference index required");const row=Object.getOwnPropertyDescriptor(current,key);if(!row||!("value"in row)||!row.enumerable)throw Error("Own inference DATA required");charge(key);if(key==="env"&&typeof row.value==="string"&&prefix.test(row.value)){if(!reference.test(row.value))throw Error("Closed compact inventory name required");names.add(row.value);if(names.size>policy.maximumReferences)throw Error("Inference inventory reference capacity exceeded");}visit(row.value,depth+1);}}finally{active.delete(current);}
 };visit(value,0);return Object.freeze([...names].sort());
}

/** 🧮️ Refuses the next cohort's maximum possible bytes before any metadata allocation. */
export function admitInferenceInventoryCohortV1(usedBytes,records,maximumBytes=readPolicy.maximumClosureBytes){if(!Number.isSafeInteger(usedBytes)||usedBytes<0||!Number.isSafeInteger(records)||records<1||records>readPolicy.maximumInventories||!Number.isSafeInteger(maximumBytes)||maximumBytes<1||maximumBytes>readPolicy.maximumClosureBytes)throw Error("Finite reducing inventory cohort required");const maximum=records*maximumBytes;if(!Number.isSafeInteger(maximum)||maximum>readPolicy.maximumMetadataBytes||usedBytes>policy.maximumInventoryBytes-maximum)throw Error("Inference inventory aggregate bytes exceeded before allocation");return maximum;}

/** 📦️ Captures finite size hints into cohorts whose uniform descriptor caps fit the unchanged metadata budget. */
export function planInferenceInventoryCohortsV1(extents){
 if(!Array.isArray(extents)||types.isProxy(extents)||Object.getPrototypeOf(extents)!==Array.prototype||extents.length>policy.maximumReferences||Reflect.ownKeys(extents).length!==extents.length+1)throw Error("Finite inventory extent roster required");
 const cohorts=[];let offset=0,records=0,maximumBytes=0;
 const flush=()=>{if(records)cohorts.push(Object.freeze({offset,records,maximumBytes}));offset+=records;records=0;maximumBytes=0;};
 for(let index=0;index<extents.length;index++){const row=Object.getOwnPropertyDescriptor(extents,String(index)),size=row&&"value"in row&&row.enumerable?row.value:undefined;if(!Number.isSafeInteger(size)||size<1||size>readPolicy.maximumClosureBytes)throw Error("Finite reducing inventory extent required");const cap=Math.max(maximumBytes,size);if(records&&(records>=readPolicy.maximumInventories||(records+1)*cap>readPolicy.maximumMetadataBytes))flush();records++;maximumBytes=Math.max(maximumBytes,size);}
 flush();return Object.freeze(cohorts);
}

/** 📋️ Validates inert exact-record DATA independently of its descriptor or managed custody. */
export function inferenceInventoryRecordHoldsV1(root,name,file){if(typeof name!=="string"||!reference.test(name))throw Error("Closed compact inventory name required");const source=name.startsWith("SEMIO_SOURCE_CLOSURE_"),key=name.slice(source?"SEMIO_SOURCE_CLOSURE_".length:"SEMIO_CARGO_LOCK_".length);if(!file||file.sha256!==key)return false;if(source)captureSourceInputInventoryV1(file.value,root);else captureCargoLockInventoryV1(file.value,root);return true;}

/** 🧱️ Captures no-follow directory identities from the filesystem root through one actual image namespace. */
/** 🫙️ The no-follow directory identities from the filesystem root down to the namespace directory of a store; the namespace directory itself may be absent (a machine that never stored a closure of that namespace), every ancestor above it must exist. */
export function inventoryAncestryV1(storeRoot,directory,check){
 const base=parse(directory).root,parts=relative(base,directory).split(sep).filter(Boolean);if(parts.length>readPolicy.maximumAncestors)throw Error("Inference inventory ancestry capacity exceeded");const rows=[];let path=base,device,absent;
 for(const[at,part]of["",...parts].entries()){check();if(part)path=join(path,part);let row;try{row=lstatSync(path,{bigint:true});}catch(error){if(at===parts.length&&at>0&&device!==undefined&&inferenceInventoryLeafMissingV1(error?.code)){absent=path;break;}throw error;}if(!row.isDirectory()||row.isSymbolicLink())throw Error("Original no-follow inventory ancestry required");if(path===storeRoot)device=row.dev;if(device!==undefined&&row.dev!==device)throw Error("Original inventory image device required");rows.push({path,dev:row.dev,ino:row.ino,mode:row.mode});}
 if(device===undefined)throw Error("Exact managed inventory image ancestry required");return Object.freeze({rows:Object.freeze(rows),device,...(absent===undefined?{}:{absent})});
}

function inventoryParentsV1(storeRoot,directory,control){return inventoryAncestryV1(storeRoot,directory,()=>checkResourceLeaseControl(control));}

/** 🪪️ Rechecks every admitted ancestor without treating missing or replaced directories as an inventory miss. */
/** 🫙️ Rechecks the ancestry of a namespace directory: an absent namespace directory must still be absent, one that appeared meanwhile is a typed change. */
export function recheckInventoryAncestryV1(parents,check){for(const before of parents.rows){check();const row=lstatSync(before.path,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink()||row.dev!==before.dev||row.ino!==before.ino||row.mode!==before.mode)throw inventoryChangedV1("Inference inventory ancestry changed");}if(parents.absent!==undefined){check();try{lstatSync(parents.absent,{bigint:true});}catch(error){if(inferenceInventoryLeafMissingV1(error?.code))return;throw error;}throw inventoryChangedV1("Inference inventory ancestry changed");}}
function recheckInventoryParentsV1(parents,control){recheckInventoryAncestryV1(parents,()=>checkResourceLeaseControl(control));}
/** 🪻️ A refusal that a concurrent writer of the store caused (a record appeared, was replaced or was removed while a batch looked at it): the batch is looked at again, and an owner whose look still changes is requeued by its reader.
 * @see ../../🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs */
const inventoryChangedV1=text=>Object.assign(Error(text),{code:"BUILD_STORE_CHANGED"});
const inventoryStampV1=row=>[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":");

/** 🕳️ Classifies only the exact leaf ENOENT spelling; ancestry and custody proofs remain mandatory at the caller. */
export function inferenceInventoryLeafMissingV1(code){return code==="ENOENT";}

/** ❓️ Refuses a changed absence observation and propagates every error except the exact immutable leaf's ENOENT. */
function requireInventoryLeafAbsentV1(path,parents,control){recheckInventoryParentsV1(parents,control);try{lstatSync(path,{bigint:true});}catch(error){if(!inferenceInventoryLeafMissingV1(error?.code))throw error;recheckInventoryParentsV1(parents,control);checkResourceLeaseControl(control);return;}throw inventoryChangedV1("Inference inventory absence changed");}

/** 📥️ The inventory names of `names` that are not stored, under one storage consumer: genuine image custody is retained while the physical identities are revalidated before each immutable-record memo hit. */
async function missingInventoryNamesV1(root,names,control,withdraw,held){
 const timings=presenceStateV1(root,control).timings,phase=held===undefined?timings.batch():()=>{};phase("reader");
 try{
  const body=(store,actual)=>inventoryNamesSettledV1(async names=>{
   if(withdraw?.aborted)return undefined;
   const parents=new Map(),missing=[],present=[];let bytes=0,done=0;
   const measure=(operation,run)=>{const finish=timings.body(operation);try{return run();}finally{finish();}};
   const recheck=async()=>{const finish=timings.body("storeRecheck");try{await store.recheck();}finally{finish();}};
   const continuation=async()=>{const finish=timings.body("continuation");try{await actual.yieldContinuation();}finally{finish();}};
   await recheck();
   for(const name of names){
    checkResourceLeaseControl(actual);if(withdraw?.aborted)return undefined;
    const {directory,key,path,ancestry,before}=measure("fileChecks",()=>{
     const source=name.startsWith("SEMIO_SOURCE_CLOSURE_"),directory=resolve(store.root,source?"build-input-closures":"build-lock-closures"),key=name.slice(source?"SEMIO_SOURCE_CLOSURE_".length:"SEMIO_CARGO_LOCK_".length),path=join(directory,key+".json");
     if(path.length>policy.maximumPathUnits)throw Error("Inference inventory path exceeds256 units");
     let ancestry=parents.get(directory);
     if(!ancestry){ancestry=inventoryParentsV1(store.root,directory,actual);parents.set(directory,ancestry);}
     recheckInventoryParentsV1(ancestry,actual);
     let before;
     if(ancestry.absent===undefined){try{before=lstatSync(path,{bigint:true});}catch(error){if(!inferenceInventoryLeafMissingV1(error?.code))throw error;}}
     return {directory,key,path,ancestry,before};
    });
    if(ancestry.absent!==undefined){missing.push({name,path,ancestry});continue;}
    if(before===undefined){
     await continuation();checkResourceLeaseControl(actual);await recheck();
     measure("fileChecks",()=>requireInventoryLeafAbsentV1(path,ancestry,actual));
     missing.push({name,path,ancestry});continue;
    }
    if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.dev!==ancestry.device||before.size<1n||before.size>BigInt(readPolicy.maximumClosureBytes))throw Error("Single-link bounded original inventory record required");
    const size=Number(before.size);if(bytes>policy.maximumInventoryBytes-size)throw Error("Inference inventory aggregate bytes exceeded before read");bytes+=size;
    let corrupt=false;
    if(!measure("fileChecks",()=>inventoryRecordStillHoldsV1(root,directory,key)))measure("recordRead",()=>{
     let file;try{file=readInventoryRecordLeanV1(directory,key,readPolicy.maximumClosureBytes);}catch(error){if(!inventoryRecordCorruptV1(error))throw error;corrupt=true;return;}
     if(!inferenceInventoryRecordHoldsV1(root,name,file))throw Error("Inference inventory record identity refused");
     rememberInventoryRecordV1(root,directory,key,file.stamp);
    });else measure("fileChecks",()=>{
     const descriptor=openSync(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0));
     try{const opened=fstatSync(descriptor,{bigint:true});if(!opened.isFile()||inventoryStampV1(opened)!==inventoryStampV1(before))throw inventoryChangedV1("Inference inventory descriptor changed");}finally{closeSync(descriptor);}
    });
    measure("fileChecks",()=>{
     recheckInventoryParentsV1(ancestry,actual);const after=lstatSync(path,{bigint:true});
     if(!after.isFile()||after.isSymbolicLink()||inventoryStampV1(after)!==inventoryStampV1(before))throw inventoryChangedV1("Inference inventory leaf changed");
     if(corrupt)missing.push({name,path,ancestry,corrupt:inventoryStampV1(after)});else present.push({path,ancestry,stamp:inventoryStampV1(after)});
    });
    if(++done%128===0){await recheck();await continuation();checkResourceLeaseControl(actual);}
   }
   checkResourceLeaseControl(actual);if(withdraw?.aborted)return undefined;await recheck();if(withdraw?.aborted)return undefined;
   measure("fileChecks",()=>{
    for(const ancestry of parents.values())recheckInventoryParentsV1(ancestry,actual);
    for(const row of missing){if(row.corrupt===undefined){requireInventoryLeafAbsentV1(row.path,row.ancestry,actual);continue;}recheckInventoryParentsV1(row.ancestry,actual);const after=lstatSync(row.path,{bigint:true});if(!after.isFile()||after.isSymbolicLink()||inventoryStampV1(after)!==row.corrupt)throw inventoryChangedV1("Inference inventory corrupt leaf changed before delivery");}
    for(const row of present){recheckInventoryParentsV1(row.ancestry,actual);const after=lstatSync(row.path,{bigint:true});if(!after.isFile()||after.isSymbolicLink()||inventoryStampV1(after)!==row.stamp)throw inventoryChangedV1("Inference inventory leaf changed before delivery");}
   });
   checkResourceLeaseControl(actual);measure("implementationCheck",()=>requireCurrentInferenceInventoryReuseV1());return new Set(missing.map(row=>row.name));
  },names,{say:inventoryLogV1});
  const result=held!==undefined?await body(held.store,held.actual):await withRepositoryBuildGraphConsumerV1(root,control,body,withdraw,phase,timings.native);
  return result??new Set(names);
 }finally{phase();}
}

/** 🛝️ Looks at `names` again while a concurrent writer changed the store under the look, at most `attempts` times, each time against the state that is there now (a record that was absent and was stored meanwhile is simply present in the next look; the answer is always one state of the store); a look that still changes after the last attempt ends with its change, which the owner that asked is answered with, not the read. A pure function of its arguments.
 * @see ./🧬️schema/🔣️.json */
export async function inventoryNamesSettledV1(run,names,{attempts=policy.changeAttempts,say=()=>{}}={}){
 for(let attempt=0;;attempt++){
  try{return await run(names);}
  catch(error){
   if(error?.code!=="BUILD_STORE_CHANGED"||attempt>=attempts)throw error;
   say("inference inventory changed while it was looked at ("+error.message+"), looking again "+(attempt+1)+" of "+attempts+" / Inferenz-Inventar während der Prüfung geändert ("+error.message+"), erneute Prüfung "+(attempt+1)+" von "+attempts);
  }
 }
}

/** 🧺️ The presence coalescers of this process, one per storage control and workspace root: every look that asks while a batch is running joins the next batch instead of taking a lease of its own. */
const presenceStatesV1=new WeakMap(),inventoryLogV1=line=>{try{process.stderr.write("[inventory] "+line+"\n");}catch{}};
const presenceStateV1=(root,control)=>{let byRoot=presenceStatesV1.get(control);if(byRoot===undefined){byRoot=new Map();presenceStatesV1.set(control,byRoot);}let state=byRoot.get(root);if(state===undefined){state={timings:createInferenceInventoryTimingsV1(),coalescer:createPresenceCoalescerV1({run:(names,withdraw)=>missingInventoryNamesV1(root,names,control,withdraw,state.held),session:work=>presenceSessionV1(root,control,state,work),lingerMilliseconds:policy.sessionLingerMilliseconds,maximumSessionMilliseconds:policy.sessionMaximumMilliseconds,sessionOverheadFactor:policy.sessionOverheadFactor,sessionCeilingMilliseconds:policy.sessionCeilingMilliseconds,maximumNames:policy.maximumReferences>512?512:policy.maximumReferences})};byRoot.set(root,state);}return state;};
/** 🪟️ One storage consumer for the batches that follow each other: the consumer verifies the reservation of the mounted image before and after its body (15 s on a loaded 16 GB worker: 283 batches, 4,377,613 ms, against 15,310 ms of checks), so the batches of a take pass run inside one consumer while asks keep coming and every batch still rechecks the store identity itself; the consumer ends after the linger of the coalescer or its maximum, and the next asks open a new one.
 * @see ./🧺️coalesce/🟨️.mjs */
function presenceSessionV1(root,control,state,work){
 const phase=state.timings.batch();phase("reader");
 return withRepositoryBuildGraphConsumerV1(root,control,async(store,actual)=>{const mine=Object.freeze({store,actual});state.held=mine;try{return await work();}finally{if(state.held===mine)state.held=undefined;}},undefined,phase,state.timings.native).finally(()=>phase());
}
const presenceCoalescerV1=(root,control)=>presenceStateV1(root,control).coalescer;

/** 📊️ How the inventory presence checks of `control` and `root` went: asked, answered from the memory of the round, batches that took a lease, names they checked, batches asked again caller by caller. */
export function inferenceInventoryPresenceStatsV1(root,control){const state=presenceStatesV1.get(control)?.get(root),nativeVerification=state?.timings.verification(),bodyOperations=state?.timings.bodyOperations();return {...(state?.coalescer.stats()??{asked:0,remembered:0,batches:0,retried:0,names:0}),timings:(state?.timings??emptyPresenceTimingsV1).stats(),...bodyOperations?{bodyOperations}:{},...nativeVerification?{nativeVerification}:{},mappingQueue:nativeStorageMappingQueueV1()};}
const emptyPresenceTimingsV1=createInferenceInventoryTimingsV1();

/** 📥️ Whether every inventory record `value` names is stored: the names are captured once, answered from the memory of the running validation round when they were found before, else checked with the other looks that ask at the same time under one storage consumer. */
export async function inferenceInventoryReferencesHoldV1(root,value,control,withdraw){
 requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);requireCurrentInferenceInventoryReuseV1();if(withdraw?.aborted)return false;const names=captureInferenceInventoryReferencesV1(value);if(!names.length)return true;
 const holds=process.env.SEMIO_INVENTORY_COALESCE==="off"?(await missingInventoryNamesV1(root,names,control,withdraw)).size===0:await presenceCoalescerV1(root,control).ask(names,observationRoundV1(),withdraw);
 checkResourceLeaseControl(control);requireCurrentInferenceInventoryReuseV1();return !withdraw?.aborted&&holds;
}
