import{join}from"node:path";
import{brotliDecompressSync}from"node:zlib";
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{CLOSURE_POLICY_V1,closureReferencePairsOfTextV1}from"../../🩹️closure-repair/🟨️.mjs";
import{canonicalJsonV1,compareCodePointsV1}from"../🧮️fact-codec/🟨️.mjs";
import{P as generation,sha256,requireKeys}from"../🧱️guards/🟨️.mjs";
import{readGenerationFragmentV1,readGenerationManifestV1,readGenerationPointerV1}from"../📖️documents/🟨️.mjs";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 📜️ Bounds the immutable repair evidence and its exact fragment census. */
export const CLOSURE_REPAIR_POLICY_V1=Object.freeze({...schema.definitions.Policy.const});
const p=CLOSURE_REPAIR_POLICY_V1,utf8=new TextDecoder("utf-8",{fatal:true}),hex=/^[a-f0-9]{64}$/u;
const compatible=(manifest,identity,implementation)=>manifest.identity===identity&&manifest.implementation===implementation;
const repairPointer=pointer=>pointer?.version===1&&pointer.manifest===null&&hex.test(pointer.parent??"")&&Number.isSafeInteger(pointer.generation)&&pointer.generation>0&&pointer.reason===p.reason&&pointer.verified===undefined;
const compared=(a,b)=>canonicalJsonV1(a)===canonicalJsonV1(b);
const referenceKey=({namespace,key})=>namespace+"/"+key;
const receiptName=(generation,parent)=>generation+"-"+parent+".json";
const receiptPath=(root,pointer)=>join(root,p.directory,receiptName(pointer.generation,pointer.parent));

/** 🏷️ Recognizes only an exact receipt or its file port's atomic-write temporary. */
export function isClosureRepairReceiptNameV1(name){
 const match=/^([1-9][0-9]{0,15})-[a-f0-9]{64}\.json(?:\.[a-f0-9]{16}\.tmp)?$/u.exec(name);
 return match!==null&&Number.isSafeInteger(Number(match[1]));
}

/** 🧹️ Removes aged receipt records under the caller's retirement custody for one exact pointer. */
export async function retireClosureRepairReceiptsV1({root,fs,pointer,cutoff,signal,fence}){
 const keep=repairPointer(pointer)?receiptName(pointer.generation,pointer.parent)
  :pointer.manifest!==null&&pointer.generation<Number.MAX_SAFE_INTEGER?receiptName(pointer.generation+1,pointer.manifest):undefined;
 const directory=join(root,p.directory);
 for(const file of await fs.list(directory)){
  signal?.throwIfAborted();
  if(file.name===keep||!(file.modifiedMs<cutoff)||!isClosureRepairReceiptNameV1(file.name))continue;
  const remove=async()=>{
   if(!await samePointer({root,fs,signal},pointer))return false;
   signal?.throwIfAborted();
   await fs.remove(join(directory,file.name),{singleLink:true,signal});
   return true;
  };
  if(!await(fence?fence(remove):remove()))return;
 }
}
const validate=(name,value)=>!validateJsonSchemaSubset({$ref:"#/definitions/"+name},value,schema).length;
const sorted=(values,key)=>values.map(value=>[key(value),value]).sort(([a],[b])=>compareCodePointsV1(a,b)).map(([,value])=>value);

async function refused(signal,action){
 signal?.throwIfAborted();
 try{const result=await action();signal?.throwIfAborted();return result;}
 catch(error){if(signal?.aborted)throw signal.reason??error;return undefined;}
}

async function currentPointer({root,fs,signal}){
 const bytes=await fs.read(join(root,generation.pointer));signal?.throwIfAborted();
 if(bytes?.length&&bytes.length<=p.maximumPointerBytes)return readGenerationPointerV1(bytes);
}

async function parentOf({root,fs,signal},pointer,identity,implementation){
 const bytes=await fs.read(join(root,generation.manifests,pointer.parent+".json"));signal?.throwIfAborted();
 if(!bytes?.length||bytes.length>p.maximumManifestBytes||sha256(bytes)!==pointer.parent)return;
 const parent=readGenerationManifestV1(bytes);
 if(parent.generation===pointer.generation-1&&compatible(parent,identity,implementation))return parent;
}

/** 🔍️ Scans one digest-verified fragment at a time, keeping bounded lists of owners and counting repeated tokens once per owner. */
async function referenceCensus({root,fs,signal,progress},manifest){
 const fragments=Object.entries(manifest.owners);
 if(fragments.length+1>p.maximumFragments)return;
 const references=new Map(),total=fragments.length+1;let links=0,packed=0,expanded=0,done=0;
 await progress?.(0,total);
 async function include(name,digest,owner){
  signal?.throwIfAborted();
  if(owner?.bytes>p.maximumFragmentBytes||!hex.test(digest))return false;
  const bytes=await fs.read(join(root,generation.fragments,digest+".json.br"));
  signal?.throwIfAborted();
  if(!bytes?.length||bytes.length>p.maximumFragmentBytes||packed+bytes.length>p.maximumPackedBytes||sha256(bytes)!==digest)return false;
  packed+=bytes.length;
  const remaining=Math.min(p.maximumFragmentBytes,p.maximumExpandedBytes-expanded);
  if(remaining<1)return false;
  const raw=brotliDecompressSync(bytes,{maxOutputLength:remaining});expanded+=raw.length;
  const text=utf8.decode(raw),body=JSON.parse(text);
  if(owner){
   const fragment=readGenerationFragmentV1(body);
   if(fragment.name!==name||fragment.root!==owner.root||fragment.kind!==owner.kind)return false;
  }else{
   requireKeys(body,["version","name","value"],[],"global repair fragment");
   if(body.version!==1||body.name!=="@global")return false;
  }
  for(const [key,namespace] of closureReferencePairsOfTextV1(text)){
   const ref={namespace,key},id=referenceKey(ref);
   let entry=references.get(id);
   if(!entry){
    if(references.size===p.maximumReferences)return false;
    entry={ref,owners:[],global:false};references.set(id,entry);
   }
   if(owner){if(entry.owners.at(-1)!==name){if(++links>p.maximumLinks)return false;entry.owners.push(name);}}
   else entry.global=true;
  }
  await progress?.(++done,total);
  signal?.throwIfAborted();
  return true;
 }
 if(!await include(undefined,manifest.global,undefined))return;
 for(const [name,owner] of fragments)if(!await include(name,owner.fragment,owner))return;
 return references;
}

function receiptOf(pointer,manifest,missing,references){
 if(!Array.isArray(missing)||!missing.length||missing.length>p.maximumMissing||missing.some(value=>!validate("Reference",value)))return;
 const canonical=sorted([...new Map(missing.map(ref=>[referenceKey(ref),ref])).values()],referenceKey),owners=new Set();
 for(const ref of canonical){
  const observed=references.get(referenceKey(ref));
  if(!observed||observed.global||!observed.owners.length)return;
  for(const name of observed.owners){owners.add(name);if(owners.size>p.maximumAffectedOwners)return;}
 }
 return{version:1,generation:pointer.generation,parent:pointer.parent,identity:manifest.identity,implementation:manifest.implementation,missing:canonical,owners:[...owners].sort(compareCodePointsV1)};
}

function intent(pointer,manifest,receipt){
 return{pointer,manifest,generation:receipt.generation,parent:receipt.parent,receipt:sha256(Buffer.from(canonicalJsonV1(receipt))),owners:receipt.owners,missing:receipt.missing};
}

/** 📝️ Stages immutable repair evidence under caller-held structural custody before the ordinary invalidation swap. */
export function prepareClosureRepairV1({expected,identity,implementation,missing,verifyClosure,...store}){
 const{fs,root,signal}=store;
 return refused(signal,async()=>{
  const observed=await currentPointer(store);
  if(!observed?.manifest||!compared(observed,expected)||observed.generation===Number.MAX_SAFE_INTEGER)return;
  const pointer={version:1,generation:observed.generation+1,manifest:null,parent:observed.manifest,reason:p.reason};
  const parent=await parentOf(store,pointer,identity,implementation);
  if(!parent)return;
  const references=await referenceCensus(store,parent);
  if(!references)return;
  const observedReceipt=receiptOf(pointer,parent,missing,references);
  if(!observedReceipt)return;
  const absent=verifyClosure?await absentReferences(store,references,verifyClosure):undefined;
  const receipt=verifyClosure?receiptOf(pointer,parent,absent,references):observedReceipt;
  if(!receipt||!validate("Receipt",receipt))return;
  const bytes=Buffer.from(canonicalJsonV1(receipt)),path=receiptPath(root,pointer);
  if(bytes.length>p.maximumReceiptBytes)return;
  const existing=await fs.read(path);
  if(existing&&!bytes.equals(existing))return;
  if(!await samePointer(store,observed))return;
  signal?.throwIfAborted();
  if(!existing&&await fs.write(path,bytes,{ifAbsent:true})===false){
   const held=await fs.read(path);if(!held||!bytes.equals(held))return;
  }
  if(await samePointer(store,observed))return intent(pointer,parent,receipt);
 });
}

async function samePointer(store,pointer){
 const current=await currentPointer(store);
 return current!==undefined&&compared(current,pointer);
}

/** 📖️ Qualifies a builder-only archived repair base from the exact current quarantine pointer and its immutable evidence. */
export function readClosureRepairV1({pointer,identity,implementation,...store}){
 return refused(store.signal,async()=>{
  if(!repairPointer(pointer)||!await samePointer(store,pointer))return;
  const bytes=await store.fs.read(receiptPath(store.root,pointer));
  if(!bytes?.length||bytes.length>p.maximumReceiptBytes)return;
  const receipt=JSON.parse(utf8.decode(bytes));
  if(!validate("Receipt",receipt)||!Buffer.from(canonicalJsonV1(receipt)).equals(bytes)||receipt.generation!==pointer.generation||receipt.parent!==pointer.parent||!compatible(receipt,identity,implementation))return;
  const parent=await parentOf(store,pointer,identity,implementation);
  if(!parent)return;
  const references=await referenceCensus(store,parent);
  if(!references)return;
  const proved=receiptOf(pointer,parent,receipt.missing,references);
  if(proved&&compared(proved,receipt)&&await samePointer(store,pointer))return intent(pointer,parent,receipt);
 });
}

/** 🔬️ Drains one bounded batch of typed leaf checks before admitting any further I/O. */
async function verifyReferenceBatches(signal,references,verifyClosure,accept){
 signal?.throwIfAborted();
 const check=async batch=>{
  const results=await Promise.allSettled(batch.map(async({ref})=>{
   signal?.throwIfAborted();
   return verifyClosure(ref,signal);
  }));
  signal?.throwIfAborted();
  return results.every((row,at)=>row.status==="fulfilled"&&accept(batch[at],row.value));
 };
 const batch=[];
 for(const entry of references.values()){
  batch.push(entry);
  if(batch.length<CLOSURE_POLICY_V1.concurrency)continue;
  if(!await check(batch))return false;
  batch.length=0;
 }
 return !batch.length||check(batch);
}

/** 🧪️ Checks every named leaf through the caller's verified local store and refuses unknown or missing global evidence. */
async function absentReferences({signal},references,verifyClosure){
 const missing=[];
 const complete=await verifyReferenceBatches(signal,references,verifyClosure,({ref,global},found)=>{
  if(found===true)return true;
  if(found!==false||global||missing.length===p.maximumMissing)return false;
  missing.push(ref);
  return true;
 });
 if(complete)return missing;
}

/** 🧭️ Derives ephemeral evidence for an earlier quarantine only from a complete bounded census of actually absent leaves. */
export function recoverClosureRepairV1({pointer,identity,implementation,verifyClosure,...store}){
 return refused(store.signal,async()=>{
  if(!repairPointer(pointer)||!await samePointer(store,pointer)||await store.fs.read(receiptPath(store.root,pointer)))return;
  const parent=await parentOf(store,pointer,identity,implementation);
  if(!parent)return;
  const references=await referenceCensus(store,parent);
  if(!references)return;
  const missing=await absentReferences(store,references,verifyClosure),receipt=receiptOf(pointer,parent,missing,references);
  if(!receipt||!validate("Receipt",receipt)||Buffer.byteLength(canonicalJsonV1(receipt))>p.maximumReceiptBytes)return;
  if(await samePointer(store,pointer)&&!await store.fs.read(receiptPath(store.root,pointer))&&await samePointer(store,pointer)){
   return{...intent(pointer,parent,receipt),receipt:sha256(Buffer.from(canonicalJsonV1(["census",receipt])))};
  }
 });
}

/** ✅️ Verifies every typed reference of a replacement, including global references, before allowing publication. */
export async function verifyClosureRepairCandidateV1({manifest,verifyClosure,...store}){
 return (await refused(store.signal,async()=>{
  const references=await referenceCensus(store,manifest);
  if(!references)return false;
  return verifyReferenceBatches(store.signal,references,verifyClosure,(_row,found)=>found===true);
 }))===true;
}
