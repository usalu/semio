import{captureBoundedSourceRevisionV1}from"../../📥️inputs/🔏️revision/🟨️.mjs";
import{fileURLToPath}from"node:url";
import{withRepositoryBuildGraphConsumerV1}from"../../📥️inputs/💾️storage/🟨️.mjs";
import{types}from"node:util";
import{requireIssuedResourceControlV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import{withManagedGraphSourceReadBatchV1,readExactSourceInventoryV1}from"../../📥️inputs/🧮️digest/📥️read/🟨️.mjs";
import{captureSourceInputInventoryV1,captureCargoLockInventoryV1}from"../../📥️inputs/📋️data/📥️load/🟨️.mjs";
const epochURLs=[new URL(import.meta.url),new URL("./🧬️schema/🔣️.json",import.meta.url),new URL("../../📥️inputs/🧮️digest/📥️read/🧬️schema/🔣️.json",import.meta.url)],epochCaptures=epochURLs.map((url,index)=>index===0?captureBoundedSourceRevisionV1(fileURLToPath(url),8388608):captureBoundedSourceRevisionV1(fileURLToPath(url),65536,true)),loadedEpoch=epochCaptures.map(row=>row.sha256),revision=(url,index)=>captureBoundedSourceRevisionV1(fileURLToPath(url),index===0?8388608:65536).sha256,policy=Object.freeze({...JSON.parse(epochCaptures[1].text??"").definitions.Policy.const}),readPolicy=Object.freeze({...JSON.parse(epochCaptures[2].text??"").definitions.Policy.const});
const reference=/^SEMIO_(?:SOURCE_CLOSURE|CARGO_LOCK)_[a-f0-9]{64}$/u,prefix=/^SEMIO_(?:SOURCE_CLOSURE|CARGO_LOCK)_/u;

/** 🧷️ Rejects a retained helper whose Source or privately captured policies changed. */
export function requireCurrentInferenceInventoryReuseV1(){if(epochURLs.some((url,index)=>revision(url,index)!==loadedEpoch[index]))throw Error("Stale inference inventory reuse implementation; restart required");}

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
export function admitInferenceInventoryCohortV1(usedBytes,records){const width=Math.min(readPolicy.maximumInventories,Math.floor(readPolicy.maximumMetadataBytes/readPolicy.maximumClosureBytes)),maximum=records*readPolicy.maximumClosureBytes;if(!Number.isSafeInteger(usedBytes)||usedBytes<0||!Number.isSafeInteger(records)||records<1||records>width||!Number.isSafeInteger(maximum)||usedBytes>policy.maximumInventoryBytes-maximum)throw Error("Inference inventory aggregate bytes exceeded before allocation");return maximum;}

/** 📋️ Validates inert exact-record DATA independently of its descriptor or managed custody. */
export function inferenceInventoryRecordHoldsV1(root,name,file){if(typeof name!=="string"||!reference.test(name))throw Error("Closed compact inventory name required");const source=name.startsWith("SEMIO_SOURCE_CLOSURE_"),key=name.slice(source?"SEMIO_SOURCE_CLOSURE_".length:"SEMIO_CARGO_LOCK_".length);if(!file||file.sha256!==key)return false;if(source)captureSourceInputInventoryV1(file.value,root);else captureCargoLockInventoryV1(file.value,root);return true;}

/** 📥️ Rechecks every unique compact record through fresh bounded descriptor batches. */
export async function inferenceInventoryReferencesHoldV1(root,value,control){
 requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);requireCurrentInferenceInventoryReuseV1();const names=captureInferenceInventoryReferencesV1(value),width=Math.max(1,Math.min(readPolicy.maximumInventories,Math.floor(readPolicy.maximumMetadataBytes/readPolicy.maximumClosureBytes)));let bytes=0;
 return withRepositoryBuildGraphConsumerV1(root,control,async(_store,actual)=>{control=actual;for(let offset=0;offset<names.length;offset+=width){checkResourceLeaseControl(control);const cohort=names.slice(offset,offset+width);admitInferenceInventoryCohortV1(bytes,cohort.length);const holds=await withManagedGraphSourceReadBatchV1(root,control,async batch=>{for(const name of cohort){const source=name.startsWith("SEMIO_SOURCE_CLOSURE_"),key=name.slice(source?"SEMIO_SOURCE_CLOSURE_".length:"SEMIO_CARGO_LOCK_".length),file=await readExactSourceInventoryV1(batch,source?"build-input-closures":"build-lock-closures",key);if(!inferenceInventoryRecordHoldsV1(root,name,file))return false;bytes+=file.byteLength;}return true;});checkResourceLeaseControl(control);requireCurrentInferenceInventoryReuseV1();if(!holds)return false;}
 checkResourceLeaseControl(control);requireCurrentInferenceInventoryReuseV1();return true;});
}
