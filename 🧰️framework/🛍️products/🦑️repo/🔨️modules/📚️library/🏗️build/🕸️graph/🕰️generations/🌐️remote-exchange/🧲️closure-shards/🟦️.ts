import {createHash} from "node:crypto";
import {brotliDecompressSync} from "node:zlib";
import {CLOSURE_POLICY_V1,closureReferencePairsOfTextV1,type ClosureNamespaceV1} from "../../../🩹️closure-repair/🟨️.mjs";
import {GENERATION_POLICY_V1 as generationPolicy} from "../../🧱️guards/🟨️.mjs";
import {REAPI_V1,decodeProtobufV1,encodeProtobufV1} from "../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const,entryName=new RegExp(policy.closureEntry,"u"),shardName=new RegExp(policy.closureShard,"u"),hex=/^[a-f0-9]{64}$/u;
const folder:Readonly<Record<ClosureNamespaceV1,string>>={"build-input-closures":"i","build-lock-closures":"l"};
const sha=(bytes:string|Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
const shardPath=(index:number)=>"c/"+String(index).padStart(4,"0");
const validSize=(size:number,maximum:number)=>Number.isSafeInteger(size)&&size>0&&size<=maximum;

export type ClosureLeafV1=Readonly<{namespace:ClosureNamespaceV1;key:string;size:number}>;
export type ClosureShardV1=Readonly<{path:string;hash:string;size:number}>;
export type PackedClosureShardV1=ClosureShardV1&Readonly<{bytes:Uint8Array}>;

/** 🪪️ Includes the namespace in a closure's authority even when two entries share underlying CAS bytes. */
export const closureEntryV1=(namespace:ClosureNamespaceV1,key:string)=>folder[namespace]+"-"+key+".json";

type ClosureMemoV1 = Map<string, readonly string[]>;

const memoEntriesLimit = policy.maximumDocuments;
const memoReferencesLimit = policy.maximumClosureReferences;
const textBytes = (text: string) => Buffer.byteLength(text, "utf8");
const sorted = (entries: readonly string[]) => entries.every((entry,index) => index === 0 || entries[index - 1]! < entry);

function readMemoEntry(value: unknown): value is [string, string[]] {
 return Array.isArray(value) && value.length === 2 && typeof value[0] === "string" && hex.test(value[0])
  && Array.isArray(value[1]) && value[1].length <= memoReferencesLimit
  && value[1].every(entry => typeof entry === "string" && entryName.test(entry)) && sorted(value[1]);
}

/** 📥️ Reads only a bounded, canonical local memo, discarding a malformed or incomplete file. */
export function readClosureMemoV1(bytes?: Uint8Array): ClosureMemoV1 {
 if (!bytes || bytes.byteLength > policy.maximumClosureMemoBytes) return new Map();
 try {
  const data: unknown = JSON.parse(Buffer.from(bytes).toString("utf8"));
  if (typeof data !== "object" || data === null || Array.isArray(data) || Object.keys(data).length !== 3) return new Map();
  const {version, sha256, entries} = data as Record<string, unknown>;
  if (version !== 1 || !Array.isArray(entries) || entries.length > memoEntriesLimit
   || !entries.every(readMemoEntry) || !sorted(entries.map(([name]) => name))
   || entries.reduce((total, [, references]) => total + references.length, 0) > policy.maximumClosureLinks
   || sha256 !== sha(JSON.stringify(entries))) return new Map();
  return new Map(entries);
 } catch {
  return new Map();
 }
}

/** 📤️ Encodes the references only when every selected fragment has a complete bounded entry. */
export function encodeClosureMemoV1(names: readonly string[], memo: ReadonlyMap<string, readonly string[]>): string | undefined {
 if (names.length > memoEntriesLimit) return undefined;
 const selected = [...names].sort();
 if (!sorted(selected)) return undefined;
 const entries: [string, readonly string[]][] = [];
 let links = 0;
 for (const name of selected) {
  const references = memo.get(name);
  if (!references || !readMemoEntry([name, references]) || (links += references.length) > policy.maximumClosureLinks) return undefined;
  entries.push([name, references]);
 }
 const entryText = JSON.stringify(entries);
 const text = '{"version":1,"sha256":"' + sha(entryText) + '","entries":' + entryText + '}';
 return textBytes(text) <= policy.maximumClosureMemoBytes ? text : undefined;
}

/** 🧭️ Scans each content-addressed fragment once, checking its address before admitting bounded closure references. */
export async function inventoryFragmentClosuresV1(names:readonly string[],read:(name:string)=>Promise<Uint8Array|undefined>,signal?:AbortSignal,memo?:ClosureMemoV1){
 const inventory=new Map<string,{namespace:ClosureNamespaceV1;key:string;fragments:Set<string>}>();let links=0;
 for(const name of names){
  signal?.throwIfAborted();
  let references=memo?.get(name);
  if(!references){
   const bytes=await read(name);
   if(!bytes||bytes.length>generationPolicy.maximumFragmentBytes||sha(bytes)!==name)throw Error("Generation fragment unavailable: "+name);
   const text=brotliDecompressSync(bytes,{maxOutputLength:generationPolicy.maximumFragmentBytes}).toString("utf8");
   references=[...new Set(Array.from(closureReferencePairsOfTextV1(text),([key,namespace])=>closureEntryV1(namespace,key)))].sort();
   if (references.length > memoReferencesLimit) throw Error("Generation fragment exceeds its finite memo reference limit");
   memo?.set(name,references);
  }
  for(const entry of references){
   const key=entry.slice(2,-5),namespace:ClosureNamespaceV1=entry[0]==="i"?"build-input-closures":"build-lock-closures";
   let row=inventory.get(entry);
   if(!row){if(inventory.size>=policy.maximumClosureReferences)throw Error("Generation closure inventory exceeds its finite reference limit");row={namespace,key,fragments:new Set()};inventory.set(entry,row);}
   if(!row.fragments.has(name)){if(++links>policy.maximumClosureLinks)throw Error("Generation closure inventory exceeds its finite owner-link limit");row.fragments.add(name);}
  }
 }
 return inventory;
}

const encodeTree=(leaves:readonly ClosureLeafV1[])=>encodeProtobufV1(REAPI_V1.tree,{root:{files:leaves.map(row=>({name:closureEntryV1(row.namespace,row.key),digest:{hash:row.key,size_bytes:row.size}}))}});

/** 🌳️ Packs canonical sorted closure leaves as REAPI trees whose files the cache validates and retains transitively. */
export function packClosureShardsV1(leaves:readonly ClosureLeafV1[]):PackedClosureShardV1[]{
 if(leaves.length>policy.maximumClosures)throw Error("Generation closure transport exceeds its finite leaf limit");
 const ordered=[...leaves].sort((a,b)=>folder[a.namespace].localeCompare(folder[b.namespace])||(a.key<b.key?-1:a.key>b.key?1:0)),shards:PackedClosureShardV1[]=[];
 let previous="";
 for(const row of ordered){
  const name=closureEntryV1(row.namespace,row.key);
  if(!entryName.test(name)||name<=previous||!validSize(row.size,CLOSURE_POLICY_V1.maximumLeafBytes))throw Error("Closed unique generation closure entry required");
  previous=name;
 }
 for(let at=0;at<ordered.length;at+=policy.closureShardEntries){
  const bytes=encodeTree(ordered.slice(at,at+policy.closureShardEntries));
  if(bytes.length>policy.maximumClosureShardBytes)throw Error("Generation closure shard exceeds its finite byte limit");
  shards.push({path:shardPath(shards.length),hash:sha(bytes),size:bytes.length,bytes});
 }
 return shards;
}

/** 🔍️ Reads the full closure inventory, proving every shard's identity, canonical form and position. */
export async function readClosureShardsV1(shards:readonly ClosureShardV1[],stream:(row:ClosureShardV1)=>Promise<AsyncIterable<Uint8Array>|undefined>,signal?:AbortSignal):Promise<ReadonlyMap<string,ClosureLeafV1>>{
 if(shards.length>Math.ceil(policy.maximumClosures/policy.closureShardEntries))throw Error("Generation closure shard count exceeds its finite limit");
 const inventory=new Map<string,ClosureLeafV1>();let previous="";
 const ordered=[...shards].sort((a,b)=>a.path<b.path?-1:a.path>b.path?1:0);
 for(const [i,row] of ordered.entries()){
  signal?.throwIfAborted();
  if(!shardName.test(row.path)||row.path!==shardPath(i)||!hex.test(row.hash)||!validSize(row.size,policy.maximumClosureShardBytes))throw Error("Closed generation closure shard required");
  const body=await stream(row);
  if(!body)throw Error("Generation closure shard unavailable: "+row.path);
  const bytes=Buffer.allocUnsafe(row.size);let read=0;
  for await(const chunk of body){
   signal?.throwIfAborted();
   if(chunk.length>bytes.length-read)throw Error("Generation closure shard exceeds its declared length: "+row.path);
   bytes.set(chunk,read);read+=chunk.length;
  }
  if(read!==row.size||sha(bytes)!==row.hash)throw Error("Generation closure shard differs from its address: "+row.path);
  const tree=decodeProtobufV1(REAPI_V1.tree,bytes),files=tree.root?.files as {name:string;digest?:{hash:string;size_bytes:number|string}}[]|undefined;
  if(!files?.length||files.length>policy.closureShardEntries||i<ordered.length-1&&files.length!==policy.closureShardEntries)throw Error("Closed generation closure shard entries required");
  const leaves:ClosureLeafV1[]=[];
  for(const {name,digest} of files){
   const key=name.slice(2,-5),size=Number(digest?.size_bytes),namespace=name[0]==="i"?"build-input-closures":"build-lock-closures";
   if(!entryName.test(name)||name<=previous||key!==digest?.hash||!validSize(size,CLOSURE_POLICY_V1.maximumLeafBytes))throw Error("Closed unique generation closure entry required");
   const leaf:ClosureLeafV1={namespace,key,size};leaves.push(leaf);inventory.set(name,leaf);previous=name;
  }
  if(!bytes.equals(encodeTree(leaves)))throw Error("Canonical generation closure shard required");
 }
 return inventory;
}
