import{captureBoundedSourceRevisionV1,implementationPinnedV1,physicalSourceOriginV1}from"../../📥️inputs/🔏️revision/🟨️.mjs";
import{fileURLToPath}from"node:url";
import{lstatSync,openSync,fstatSync,closeSync,constants}from"node:fs";
import{resolve,join,parse,relative,sep}from"node:path";
import{withRepositoryBuildGraphConsumerV1}from"../../📥️inputs/💾️storage/🟨️.mjs";
import{types}from"node:util";
import{requireIssuedResourceControlV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{readInventoryRecordLeanV1,rememberInventoryRecordV1,inventoryRecordStillHoldsV1}from"../../📥️inputs/🪶️lean-inventory/🟨️.mjs";
import{captureSourceInputInventoryV1,captureCargoLockInventoryV1}from"../../📥️inputs/📋️data/📥️load/🟨️.mjs";
const epochURLs=[new URL(physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),new URL("./🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),new URL("../../📥️inputs/🧮️digest/📥️read/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL))],epochCaptures=epochURLs.map((url,index)=>index===0?captureBoundedSourceRevisionV1(fileURLToPath(url),8388608):captureBoundedSourceRevisionV1(fileURLToPath(url),65536,true)),loadedEpoch=epochCaptures.map(row=>row.sha256),revision=(url,index)=>captureBoundedSourceRevisionV1(fileURLToPath(url),index===0?8388608:65536).sha256,policy=Object.freeze({...JSON.parse(epochCaptures[1].text??"").definitions.Policy.const}),readPolicy=Object.freeze({...JSON.parse(epochCaptures[2].text??"").definitions.Policy.const});
const reference=/^SEMIO_(?:SOURCE_CLOSURE|CARGO_LOCK)_[a-f0-9]{64}$/u,prefix=/^SEMIO_(?:SOURCE_CLOSURE|CARGO_LOCK)_/u;

/** 🧷️ Rejects a retained helper whose Source or privately captured policies changed. */
export function requireCurrentInferenceInventoryReuseV1(){if(implementationPinnedV1())return;if(epochURLs.some((url,index)=>revision(url,index)!==loadedEpoch[index]))throw Error("Stale inference inventory reuse implementation; restart required");}

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
function inventoryParentsV1(storeRoot,directory,control){
 const base=parse(directory).root,parts=relative(base,directory).split(sep).filter(Boolean);if(parts.length>readPolicy.maximumAncestors)throw Error("Inference inventory ancestry capacity exceeded");const rows=[];let path=base,device;
 for(const part of["",...parts]){checkResourceLeaseControl(control);if(part)path=join(path,part);const row=lstatSync(path,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink())throw Error("Original no-follow inventory ancestry required");if(path===storeRoot)device=row.dev;if(device!==undefined&&row.dev!==device)throw Error("Original inventory image device required");rows.push({path,dev:row.dev,ino:row.ino,mode:row.mode});}
 if(device===undefined)throw Error("Exact managed inventory image ancestry required");return Object.freeze({rows:Object.freeze(rows),device});
}

/** 🪪️ Rechecks every admitted ancestor without treating missing or replaced directories as an inventory miss. */
function recheckInventoryParentsV1(parents,control){for(const before of parents.rows){checkResourceLeaseControl(control);const row=lstatSync(before.path,{bigint:true});if(!row.isDirectory()||row.isSymbolicLink()||row.dev!==before.dev||row.ino!==before.ino||row.mode!==before.mode)throw Error("Inference inventory ancestry changed");}}
const inventoryStampV1=row=>[row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].join(":");

/** 🕳️ Classifies only the exact leaf ENOENT spelling; ancestry and custody proofs remain mandatory at the caller. */
export function inferenceInventoryLeafMissingV1(code){return code==="ENOENT";}

/** ❓️ Refuses a changed absence observation and propagates every error except the exact immutable leaf's ENOENT. */
function requireInventoryLeafAbsentV1(path,parents,control){recheckInventoryParentsV1(parents,control);try{lstatSync(path,{bigint:true});}catch(error){if(!inferenceInventoryLeafMissingV1(error?.code))throw error;recheckInventoryParentsV1(parents,control);checkResourceLeaseControl(control);return;}throw Error("Inference inventory absence changed");}

/** 📥️ Retains genuine image custody while revalidating physical identities before each immutable-record memo hit. */
export async function inferenceInventoryReferencesHoldV1(root,value,control){
 requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);requireCurrentInferenceInventoryReuseV1();const names=captureInferenceInventoryReferencesV1(value);if(!names.length)return true;
 return withRepositoryBuildGraphConsumerV1(root,control,async(store,actual)=>{
  const parents=new Map(),missing=[],present=[];let bytes=0,done=0;await store.recheck();
  for(const name of names){
   checkResourceLeaseControl(actual);const source=name.startsWith("SEMIO_SOURCE_CLOSURE_"),directory=resolve(store.root,source?"build-input-closures":"build-lock-closures"),key=name.slice(source?"SEMIO_SOURCE_CLOSURE_".length:"SEMIO_CARGO_LOCK_".length),path=join(directory,key+".json");if(path.length>policy.maximumPathUnits)throw Error("Inference inventory path exceeds256 units");let ancestry=parents.get(directory);if(!ancestry){ancestry=inventoryParentsV1(store.root,directory,actual);parents.set(directory,ancestry);}recheckInventoryParentsV1(ancestry,actual);
   let before;try{before=lstatSync(path,{bigint:true});}catch(error){if(!inferenceInventoryLeafMissingV1(error?.code))throw error;await actual.yieldContinuation();checkResourceLeaseControl(actual);await store.recheck();requireInventoryLeafAbsentV1(path,ancestry,actual);missing.push({path,ancestry});continue;}
   if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.dev!==ancestry.device||before.size<1n||before.size>BigInt(readPolicy.maximumClosureBytes))throw Error("Single-link bounded original inventory record required");const size=Number(before.size);if(bytes>policy.maximumInventoryBytes-size)throw Error("Inference inventory aggregate bytes exceeded before read");bytes+=size;
   if(!inventoryRecordStillHoldsV1(root,directory,key)){const file=readInventoryRecordLeanV1(directory,key,readPolicy.maximumClosureBytes);if(!inferenceInventoryRecordHoldsV1(root,name,file))throw Error("Inference inventory record identity refused");rememberInventoryRecordV1(root,directory,key,file.stamp);}else{const descriptor=openSync(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0));try{const opened=fstatSync(descriptor,{bigint:true});if(!opened.isFile()||inventoryStampV1(opened)!==inventoryStampV1(before))throw Error("Inference inventory descriptor changed");}finally{closeSync(descriptor);}}
   recheckInventoryParentsV1(ancestry,actual);const after=lstatSync(path,{bigint:true});if(!after.isFile()||after.isSymbolicLink()||inventoryStampV1(after)!==inventoryStampV1(before))throw Error("Inference inventory leaf changed");present.push({path,ancestry,stamp:inventoryStampV1(after)});
   if(++done%128===0){await store.recheck();await actual.yieldContinuation();checkResourceLeaseControl(actual);}
  }
  checkResourceLeaseControl(actual);await store.recheck();for(const ancestry of parents.values())recheckInventoryParentsV1(ancestry,actual);for(const row of missing)requireInventoryLeafAbsentV1(row.path,row.ancestry,actual);for(const row of present){recheckInventoryParentsV1(row.ancestry,actual);const after=lstatSync(row.path,{bigint:true});if(!after.isFile()||after.isSymbolicLink()||inventoryStampV1(after)!==row.stamp)throw Error("Inference inventory leaf changed before delivery");}checkResourceLeaseControl(actual);requireCurrentInferenceInventoryReuseV1();return missing.length===0;
 });
}
