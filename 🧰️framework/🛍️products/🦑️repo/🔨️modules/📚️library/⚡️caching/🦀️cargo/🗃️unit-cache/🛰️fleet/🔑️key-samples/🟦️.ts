import {createHash,randomBytes} from "node:crypto";
import {mkdirSync,readFileSync,renameSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {machinePathV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {readLookupsV1,type UnitLookupV1} from "../../💾️store/🟦️.ts";
import {type FleetClientV1} from "../📡️client/🟦️.ts";

/** 🔑️ One recent miss of a unit as digests only: the crate, its slot (the identity without content inputs) and one digest per key component (`toolchain`, `source`, `platform`, `roots`, `extern:<name>`, `env:<NAME>`, `args.<class>`), never a path, an argument text or a value. */
export type KeySampleV1=Readonly<{crate:string;slot:string;atMs:number;components:Readonly<Record<string,string>>;values?:Readonly<Record<string,string>>}>;

/** 🔑️ The bounded document an origin publishes as one content-addressed blob (`KeySamples` of the unit cache fleet schema); its sha256 travels in the counters (`keySamples`). */
export type KeySamplesV1=Readonly<{version:1;atMs:number;samples:readonly KeySampleV1[]}>;

/** 🔑️ How a machine names its published sample: the blob, how many units it holds and when it was taken. */
export type KeySamplesRefV1=Readonly<{blob:string;count:number;atMs:number}>;

/** 🔑️ The units two or more origins saw under different keys: the crate, its slot, the origins and the key components whose digests are not equal in all of them. `sameSource` is true when every origin built the same source text, so a difference in another component is machine- or build-dependent and no edit. */
export type KeyDiffV1=Readonly<{crate:string;slot:string;origins:readonly string[];differs:readonly string[];sameSource:boolean;values?:Readonly<Record<string,Readonly<Record<string,string>>>>}>;

/** 🔑️ The comparison of the samples of all origins. */
export type KeyComparisonV1=Readonly<{origins:number;units:number;diffs:readonly KeyDiffV1[];byComponent:Readonly<Record<string,number>>}>;

const LIMIT=400,FILE=(workspace:string)=>machinePathV1(workspace,".cache","f","key-samples.json"),sha=(text:string)=>createHash("sha256").update(text).digest("hex");

/** 🧼️ A flag text safe to publish: every token that holds a path separator is replaced by `<path#` and eight digits of its digest, so two machines with different directories show different tokens without anyone seeing a directory. */
export const safeKeyValueV1=(text:string):string=>text.replace(/[^\s,\u001f]*[\\/][^\s,\u001f]*/gu,match=>"<path#"+sha(match).slice(0,8)+">").slice(0,300);

/** 🔑️ The crates every origin samples in one window: the crate names whose digest falls in the bucket of the window (`BUCKETS` buckets, the window changes every `WINDOW` milliseconds), the same set on every machine, so units that no two machines happened to miss are still compared. */
export const KEY_SAMPLE_BUCKETS_V1=8,KEY_SAMPLE_WINDOW_MILLISECONDS_V1=1800000;
export const inCommonKeySampleV1=(crate:string,now:number):boolean=>parseInt(sha(crate).slice(0,8),16)%KEY_SAMPLE_BUCKETS_V1===Math.floor(now/KEY_SAMPLE_WINDOW_MILLISECONDS_V1)%KEY_SAMPLE_BUCKETS_V1;

const valuesOf=(row:UnitLookupV1):{values?:Readonly<Record<string,string>>}=>{
 const entries=Object.entries(row.values??{}).filter(([name])=>name!=="args.remap").slice(0,12).map(([name,text])=>[name,safeKeyValueV1(text)] as const).sort(([left],[right])=>left<right?-1:left>right?1:0);
 return entries.length===0?{}:{values:Object.freeze(Object.fromEntries(entries))};
};

/** 🔑️ The units to publish: the newest lookup of every unit (crate and slot) among `rows` that was a miss or whose crate is in the common sample of the window, the `limit` most recent, as a stable document: units sorted by crate and slot, components by name. */
export function collectKeySamplesV1(rows:readonly UnitLookupV1[],now:number,limit=LIMIT):KeySamplesV1{
 const latest=new Map<string,Readonly<{row:UnitLookupV1;slot:string}>>();
 for(const row of rows)if(typeof row.slot==="string"&&(row.outcome==="miss"||inCommonKeySampleV1(row.crate,now))){const name=row.crate+"\u0000"+row.slot,held=latest.get(name);if(held===undefined||held.row.atMs<=row.atMs)latest.set(name,{row,slot:row.slot});}
 const picked=[...latest.values()].sort((left,right)=>right.row.atMs-left.row.atMs).slice(0,limit).sort((left,right)=>left.row.crate<right.row.crate?-1:left.row.crate>right.row.crate?1:left.slot<right.slot?-1:left.slot>right.slot?1:0);
 return Object.freeze({version:1,atMs:now,samples:picked.map(({row,slot})=>Object.freeze({crate:row.crate,slot,atMs:row.atMs,components:Object.freeze(Object.fromEntries(Object.entries(row.components).sort(([left],[right])=>left<right?-1:left>right?1:0))),...valuesOf(row)}))});
}

/** 🔑️ Whether a value is a `KeySamples` document of the closed shape. */
export function isKeySamplesV1(value:unknown):value is KeySamplesV1{
 const row=value as {version?:unknown;atMs?:unknown;samples?:unknown};
 if(row===null||typeof row!=="object"||row.version!==1||!Number.isSafeInteger(row.atMs)||!Array.isArray(row.samples)||row.samples.length>LIMIT)return false;
 return row.samples.every(sample=>{const s=sample as KeySampleV1;return s!==null&&typeof s==="object"&&typeof s.crate==="string"&&s.crate.length<=200&&typeof s.slot==="string"&&/^[0-9a-f]{64}$/u.test(s.slot)&&Number.isSafeInteger(s.atMs)&&s.components!==null&&typeof s.components==="object"&&Object.keys(s.components).length<=400&&Object.values(s.components).every(digest=>typeof digest==="string"&&/^[0-9a-f]{4,64}$/u.test(digest))&&(s.values===undefined||(s.values!==null&&typeof s.values==="object"&&Object.keys(s.values).length<=12&&Object.values(s.values).every(text=>typeof text==="string"&&text.length<=300&&!/[\\/]/u.test(text))));});
}

type StoredRefV1=KeySamplesRefV1&Readonly<{stable:string}>;
const readStored=(workspace:string):StoredRefV1|undefined=>{
 try{const row=JSON.parse(readFileSync(FILE(workspace),"utf8")) as StoredRefV1;return /^[0-9a-f]{64}$/u.test(row.blob)&&/^[0-9a-f]{64}$/u.test(row.stable)&&Number.isSafeInteger(row.count)&&Number.isSafeInteger(row.atMs)?row:undefined;}catch{return undefined;}
};

/** 🔑️ Reads the reference of the last published sample (`<worktree>/.cache/f/key-samples.json`), `undefined` when there is none. */
export function readKeySamplesRefV1(workspace:string):KeySamplesRefV1|undefined{
 const row=readStored(workspace);return row===undefined?undefined:{blob:row.blob,count:row.count,atMs:row.atMs};
}

/** 📮️ Publishes the recent misses of this machine as one blob of the fleet cache and remembers its reference; nothing is sent when the document equals the last one or the cache is unusable. Never throws. */
export async function publishKeySamplesV1(client:FleetClientV1,input:Readonly<{root:string;workspace:string;now?:number;sinceMilliseconds?:number}>):Promise<KeySamplesRefV1|undefined>{
 try{
  const now=input.now??Date.now(),document=collectKeySamplesV1(readLookupsV1(input.root,now-(input.sinceMilliseconds??3600000)),now);
  if(document.samples.length===0||!client.settings.upload||!client.usable())return undefined;
  const body=JSON.stringify(document.samples.map(row=>[row.crate,row.slot,row.components,row.values??null])),previous=readStored(input.workspace),stable=sha(body);
  const text=JSON.stringify(document),bytes=new TextEncoder().encode(text),blob=sha(text);
  if(previous!==undefined&&previous.count===document.samples.length&&previous.stable===stable)return {blob:previous.blob,count:previous.count,atMs:previous.atMs};
  if(await client.hasBlob(blob)!==true&&!await client.putBlob(blob,bytes))return undefined;
  const ref:StoredRefV1={blob,count:document.samples.length,atMs:now,stable};
  const path=FILE(input.workspace),temporary=path+"."+randomBytes(3).toString("hex");mkdirSync(dirname(path),{recursive:true});writeFileSync(temporary,JSON.stringify(ref));renameSync(temporary,path);
  return {blob,count:ref.count,atMs:now};
 }catch{return undefined;}
}

/** 📥️ Reads the samples the origins published (`refs` by origin, the blob fetched by `get`, bounded to one mebibyte, its sha256 and its closed shape checked; an origin whose blob is missing, altered or malformed is left out) and compares them; `undefined` when no origin has a readable sample. */
export async function readKeyComparisonV1(refs:Readonly<Record<string,KeySamplesRefV1>>,get:(blob:string)=>Promise<Uint8Array|undefined>):Promise<KeyComparisonV1|undefined>{
 const found:Record<string,KeySamplesV1>={};
 await Promise.all(Object.entries(refs).map(async([origin,ref])=>{
  try{
   const bytes=await get(ref.blob);
   if(bytes===undefined||bytes.length>1<<20||createHash("sha256").update(bytes).digest("hex")!==ref.blob)return;
   const document:unknown=JSON.parse(new TextDecoder().decode(bytes));
   if(isKeySamplesV1(document))found[origin]=document;
  }catch{}
 }));
 return Object.keys(found).length===0?undefined:compareKeySamplesV1(found);
}

/** 🔬️ Compares the samples of several origins: every unit (crate and slot) that at least two origins sampled and whose key components are not equal in all of them is a difference; a component differs when its digests are not the same everywhere or some origin lacks it. `diffs` is sorted by the number of origins, then crate; `byComponent` counts the units each component differs in. */
export function compareKeySamplesV1(samples:Readonly<Record<string,KeySamplesV1>>):KeyComparisonV1{
 const groups=new Map<string,{crate:string;slot:string;rows:Map<string,KeySampleV1>}>();
 for(const [origin,document] of Object.entries(samples))for(const sample of document.samples){const name=sample.crate+"\u0000"+sample.slot;let group=groups.get(name);if(group===undefined){group={crate:sample.crate,slot:sample.slot,rows:new Map()};groups.set(name,group);}group.rows.set(origin,sample);}
 const diffs:KeyDiffV1[]=[],byComponent:Record<string,number>={};let units=0;
 for(const group of groups.values()){
  if(group.rows.size<2)continue;units++;
  const rows=[...group.rows.values()],names=new Set(rows.flatMap(row=>Object.keys(row.components))),differs=[...names].filter(name=>new Set(rows.map(row=>row.components[name]??"")).size>1).sort();
  if(differs.length===0)continue;
  for(const name of differs.map(name=>name.replace(/^(extern|env):.*$/u,"$1:*")))byComponent[name]=(byComponent[name]??0)+1;
  const shown:Record<string,Record<string,string>>={};
  for(const name of differs)for(const [origin,sample] of group.rows){const text=sample.values?.[name];if(text!==undefined)(shown[name]??={})[origin]=text;}
  diffs.push(Object.freeze({crate:group.crate,slot:group.slot,origins:[...group.rows.keys()].sort(),differs,sameSource:!differs.includes("source"),...Object.keys(shown).length===0?{}:{values:shown}}));
 }
 diffs.sort((left,right)=>right.origins.length-left.origins.length||(left.crate<right.crate?-1:left.crate>right.crate?1:0));
 return Object.freeze({origins:Object.keys(samples).length,units,diffs,byComponent:Object.freeze(byComponent)});
}
