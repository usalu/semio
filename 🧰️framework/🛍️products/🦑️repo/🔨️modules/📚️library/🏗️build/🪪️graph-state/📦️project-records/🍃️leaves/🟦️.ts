import{createHash,randomBytes}from"node:crypto";
import{mkdirSync,renameSync,rmSync,statSync,writeFileSync}from"node:fs";
import{availableParallelism}from"node:os";
import{readFile}from"node:fs/promises";
import{join}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import type{GraphFleetClientV1,GraphLeafSourceV1,GraphLeafV1}from"../../🛰️exchange/🟦️.ts";
import type{ClosurePortV1}from"../../../🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts";

/** 🍃️ The authored numbers of the leaf pieces of a record.
 * @see ./🧬️schema/🔣️.json */
export const LEAVES_POLICY_V1=schema.definitions.Policy.const;
/** 🍃️ What a taker did with the leaves an entry lists: the ones it held, the ones it fetched, and the ones nobody could give (the entry is incomplete). */
export type LeafAdoptionV1=Readonly<{present:number;fetched:number;missing:readonly GraphLeafV1[]}>;

const NAMESPACES=LEAVES_POLICY_V1.namespaces as Readonly<Record<string,GraphLeafV1["namespace"]>>;

/** 🔢️ How many leaves are moved at once: the cores of this machine, within the declared bounds. */
export function leafWidthV1(cores:number=availableParallelism()):number{return Math.max(LEAVES_POLICY_V1.minimumWidth,Math.min(LEAVES_POLICY_V1.maximumWidth,cores));}

/** 🧭️ The leaves a text names (the value of a record), each digest once, in the order of their digests. A pure function. */
export function leafReferencesOfTextV1(text:string):readonly Readonly<{namespace:GraphLeafV1["namespace"];hash:string}>[]{
 const found=new Map<string,GraphLeafV1["namespace"]>();
 for(const match of text.matchAll(new RegExp(LEAVES_POLICY_V1.token,"gu")))found.set(match[2]!,NAMESPACES[match[1]!]!);
 return[...found].sort((left,right)=>left[0]<right[0]?-1:1).map(([hash,namespace])=>({namespace,hash}));
}

/** 📤️ The leaves a record names as sources for its publication: the ones this machine holds are read from its store (verified against their digest by the publication), the ones it lacks are given a source that reads nothing, so the publication refuses with `leaf-missing` instead of storing a record nobody can use. */
export function leafSourcesV1(port:ClosurePortV1,text:string):readonly GraphLeafSourceV1[]{
 return leafReferencesOfTextV1(text).map(({namespace,hash})=>{
  const size=port.size(namespace,hash);
  return size===undefined||size<1||size>LEAVES_POLICY_V1.maximumLeafBytes?{namespace,hash,size:1,read:async()=>undefined}:{namespace,hash,size,read:()=>port.read(namespace,hash)};
 });
}

/** 📥️ Fetches the listed leaves this machine lacks from the blob store of the fleet cache, `width` at a time: each arrives verified by size and digest (the client refuses other bytes) and is stored in the own store under the same digest. A leaf nobody can give, or whose fetch fails, is reported, never thrown. */
export async function adoptLeavesV1(input:Readonly<{client:GraphFleetClientV1;port:ClosurePortV1;leaves:readonly GraphLeafV1[];width?:number;signal?:AbortSignal}>):Promise<LeafAdoptionV1>{
 const{client,port}=input,unique=[...new Map(input.leaves.map(leaf=>[leaf.hash,leaf] as const)).values()],missing:GraphLeafV1[]=[];let present=0,fetched=0,next=0;
 await Promise.all(Array.from({length:Math.min(input.width??leafWidthV1(),unique.length)},async()=>{
  for(;;){
   const leaf=unique[next++];if(leaf===undefined)return;
   if(port.size(leaf.namespace,leaf.hash)!==undefined){present++;continue;}
   if(input.signal?.aborted===true||leaf.size<1||leaf.size>LEAVES_POLICY_V1.maximumLeafBytes){missing.push(leaf);continue;}
   try{
     const bytes=await client.getBlob(leaf.hash,leaf.size,input.signal);
     input.signal?.throwIfAborted();
    if(bytes===undefined){missing.push(leaf);continue;}
    await port.put(leaf.namespace,leaf.hash,bytes);
    if(port.size(leaf.namespace,leaf.hash)===undefined)missing.push(leaf);else fetched++;
   }catch{missing.push(leaf);}
  }
 }));
 return Object.freeze({present,fetched,missing:missing.sort((left,right)=>left.hash<right.hash?-1:1)});
}

/** 💾️ The store of this machine for taking leaves: the writer custody of the graph store when the process holds it, else the same immutable file written directly under its digest (temporary file and rename, bytes verified against the name first), so a process without the custody (a look beside the inference) never turns a fetched leaf into a missing one. */
export function leafPortWithDirectPutV1(port:ClosurePortV1):ClosurePortV1{
 return{...port,async put(namespace,key,bytes){
  try{await port.put(namespace,key,bytes);if(port.size(namespace,key)!==undefined)return;}catch{}
  if(createHash("sha256").update(bytes).digest("hex")!==key)throw Error("Leaf content differs from its name: "+key);
  const directory=port.directory(namespace),temporary=join(directory,"~"+randomBytes(8).toString("hex"));
  mkdirSync(directory,{recursive:true});
  try{writeFileSync(temporary,bytes,{flag:"wx"});renameSync(temporary,join(directory,key+".json"));}finally{rmSync(temporary,{force:true});}
 }};
}

/** 🧷️ The leaves of a record that is published from a staging directory (the pool child infers into one and removes it after the publication): a leaf the staging directory holds under `<namespace>/<digest>.json` is read from there, any other from `base` (the store of the machine), so the leaves the inference just made travel with the record instead of refusing it as `leaf-missing`. Putting goes to `base`. */
export function stagedLeafPortV1(staging:string,base:ClosurePortV1|undefined):ClosurePortV1{
 const staged=(namespace:Parameters<ClosurePortV1["size"]>[0],key:string)=>join(staging,namespace,key+".json");
 return{
  directory:namespace=>base?.directory(namespace)??join(staging,namespace),
  size:(namespace,key)=>{try{const row=statSync(staged(namespace,key));if(row.isFile())return row.size;}catch{}return base?.size(namespace,key);},
  read:async(namespace,key)=>{try{return await readFile(staged(namespace,key));}catch{}return base?.read(namespace,key);},
  put:async(namespace,key,bytes)=>{if(base===undefined)throw Error("No store to put a leaf into");await base.put(namespace,key,bytes);}
 };
}
