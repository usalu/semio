import {createHash} from "node:crypto";
import {mkdir,readdir,readFile,rm,writeFile} from "node:fs/promises";
import {join} from "node:path";

/** 📐️ What the evidence of divergences may hold: the newest 64 per class of result and, for units, the newest 16 per kind of divergence (a flood of one kind never pushes out the evidence of the others), 256 MiB in all (the oldest go first), 16 MiB per kept blob (a larger blob is named by its digest only). */
export const EVIDENCE_LIMITS_V1=Object.freeze({perClass:64,perKind:16,bytes:268435456,blob:16777216});

/** 🏷️ The kind of a divergence from the output files that differ: `stream` (the captured stdout and stderr, named `⟨stdout⟩`, `⟨stderr⟩`), `rmeta`, `rlib`, `dep-info` (`.d`), `dylib`, `binary` (a name without extension), `other`; several kinds of one divergence are joined with `+` in sorted order, `none` when no file is named. */
export function divergenceKindV1(differing:readonly string[]):string{
 const kind=(path:string):string=>{
  if(path.startsWith("⟨"))return "stream";
  const name=path.split("/").at(-1)!;
  if(/\.rmeta(\.|$)/u.test(name))return "rmeta";
  if(name.endsWith(".rlib"))return "rlib";
  if(name.endsWith(".d"))return "dep-info";
  if(/\.(dylib|so|dll)$/u.test(name))return "dylib";
  return name.includes(".")?"other":"binary";
 };
 return[...new Set(differing.map(kind))].sort().join("+")||"none";
}

/** 🗂️ One file of the evidence of a divergence: the action result of one side (`kept.ac`, `refused.ac`) or one output that differs (`kept.1`, `refused.1`, named by `path`); `stored` is false when the blob was gone or larger than the limit. */
export type EvidenceFileV1=Readonly<{name:string;side:"kept"|"refused";path:string;sha256:string;bytes:number;stored:boolean}>;

/** 🗂️ What the server keeps of one write that met another result for its key, so that both sides can be compared later: who, what class of result, which outputs differ, and the files. `outcome` says which result the server serves: `refused` keeps the first result (the `kept` side) and refuses the new one, `superseded` replaces the first by the new one (the `refused` side, the class of graph records where the newest tree state wins). */
export type EvidenceV1=Readonly<{id:string;atMs:number;outcome:"refused"|"superseded";sort:string;namespace:string;key:string;keptOrigin:string;refusedOrigin:string;differing:readonly string[];files:readonly EvidenceFileV1[]}>;

/** 🗂️ The inputs of one piece of evidence: both action results and, for every differing path, the digest each side names (`blob` reads one from the store). */
export type EvidenceInputV1=Readonly<{outcome:"refused"|"superseded";sort:string;namespace:string;key:string;keptOrigin:string;refusedOrigin:string;differing:readonly string[];kept:Uint8Array;refused:Uint8Array;keptRows:readonly Readonly<{path:string;hash:string}>[];refusedRows:readonly Readonly<{path:string;hash:string}>[];blob(hash:string):Promise<Uint8Array|undefined>}>;

/** 🗂️ The evidence store of a server directory: `<directory>/fleet/evidence/<id>/` with `meta.json` written last. */
export interface EvidenceStoreV1{
 keep(input:EvidenceInputV1):Promise<void>;
 list():readonly EvidenceV1[];
 read(id:string,name:string):Promise<Uint8Array<ArrayBuffer>|undefined>;
 summary():Readonly<{count:number;bytes:number;byClass:Readonly<Record<string,number>>}>;
}

const SAFE=/^[0-9A-Za-z._-]{1,160}$/u,sizeOf=(entry:EvidenceV1):number=>entry.files.reduce((total,file)=>total+(file.stored?file.bytes:0),0);
const sha=(bytes:Uint8Array):string=>createHash("sha256").update(bytes).digest("hex");

/** 🗂️ Opens the evidence store: the entries found on disk are indexed, a directory without `meta.json` (a write cut short) is removed. Disk trouble never fails a request. */
export async function openEvidenceV1(directory:string,now:()=>number=Date.now):Promise<EvidenceStoreV1>{
 const root=join(directory,"fleet","evidence"),index=new Map<string,EvidenceV1>();
 await mkdir(root,{recursive:true,mode:0o700}).catch(()=>{});
 for(const name of await readdir(root).catch(()=>[] as string[])){
  try{index.set(name,JSON.parse(await readFile(join(root,name,"meta.json"),"utf8")) as EvidenceV1);}catch{await rm(join(root,name),{recursive:true,force:true}).catch(()=>{});}
 }
 let queue:Promise<void>=Promise.resolve();
 const drop=async(id:string):Promise<void>=>{index.delete(id);await rm(join(root,id),{recursive:true,force:true}).catch(()=>{});};
 const prune=async():Promise<void>=>{
  const rows=[...index.values()].sort((a,b)=>a.atMs-b.atMs),perClass=new Map<string,EvidenceV1[]>();
  for(const row of rows)perClass.set(row.sort,[...(perClass.get(row.sort)??[]),row]);
  for(const list of perClass.values())for(const row of list.slice(0,Math.max(0,list.length-EVIDENCE_LIMITS_V1.perClass)))await drop(row.id);
  const perKind=new Map<string,EvidenceV1[]>();
  for(const row of [...index.values()].filter(entry=>entry.sort==="unit").sort((a,b)=>a.atMs-b.atMs)){const name=divergenceKindV1(row.differing);perKind.set(name,[...(perKind.get(name)??[]),row]);}
  for(const list of perKind.values())for(const row of list.slice(0,Math.max(0,list.length-EVIDENCE_LIMITS_V1.perKind)))await drop(row.id);
  let total=[...index.values()].reduce((sum,row)=>sum+sizeOf(row),0);
  for(const row of [...index.values()].sort((a,b)=>a.atMs-b.atMs)){if(total<=EVIDENCE_LIMITS_V1.bytes)break;total-=sizeOf(row);await drop(row.id);}
 };
 const write=async(input:EvidenceInputV1):Promise<void>=>{
  const atMs=now(),id=`${atMs}-${input.sort}-${input.key.slice(0,12)}-${input.refusedOrigin}`.replace(/[^0-9A-Za-z._-]/gu,"_");
  for(const old of [...index.values()])if(old.namespace===input.namespace&&old.key===input.key&&old.refusedOrigin===input.refusedOrigin)await drop(old.id);
  const folder=join(root,id),files:EvidenceFileV1[]=[];
  await mkdir(folder,{recursive:true,mode:0o700});
  const put=async(name:string,side:"kept"|"refused",path:string,bytes:Uint8Array|undefined,digest?:string):Promise<void>=>{
   if(bytes!==undefined&&bytes.byteLength<=EVIDENCE_LIMITS_V1.blob){await writeFile(join(folder,name),bytes,{mode:0o600});files.push({name,side,path,sha256:sha(bytes),bytes:bytes.byteLength,stored:true});}
   else files.push({name,side,path,sha256:digest??"",bytes:bytes?.byteLength??0,stored:false});
  };
  await put("kept.ac","kept","",input.kept);await put("refused.ac","refused","",input.refused);
  let number=0;
  for(const path of input.differing){
   number++;
   for(const side of ["kept","refused"] as const){
    const hash=(side==="kept"?input.keptRows:input.refusedRows).find(row=>row.path===path)?.hash;
    if(hash===undefined)continue;
    await put(`${side}.${number}`,side,path,await input.blob(hash).catch(()=>undefined),hash);
   }
  }
  const entry:EvidenceV1={id,atMs,outcome:input.outcome,sort:input.sort,namespace:input.namespace,key:input.key,keptOrigin:input.keptOrigin,refusedOrigin:input.refusedOrigin,differing:input.differing,files};
  await writeFile(join(folder,"meta.json"),JSON.stringify(entry)+"\n",{mode:0o600});
  index.set(id,entry);await prune();
 };
 return Object.freeze<EvidenceStoreV1>({
  keep(input){queue=queue.then(()=>write(input)).catch(()=>{});return queue;},
  list:()=>[...index.values()].sort((a,b)=>b.atMs-a.atMs),
  async read(id,name){
   if(!SAFE.test(id)||!SAFE.test(name)||!index.has(id))return undefined;
   try{return new Uint8Array(await readFile(join(root,id,name)));}catch{return undefined;}
  },
  summary:()=>{const byClass:Record<string,number>={};let bytes=0;for(const row of index.values()){byClass[row.sort]=(byClass[row.sort]??0)+1;bytes+=sizeOf(row);}return{count:index.size,bytes,byClass:Object.fromEntries(Object.entries(byClass).sort(([a],[b])=>a<b?-1:1))};},
 });
}
