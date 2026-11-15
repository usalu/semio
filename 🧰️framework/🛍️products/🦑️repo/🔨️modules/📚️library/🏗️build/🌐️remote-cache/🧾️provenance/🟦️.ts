import {createHash} from "node:crypto";
import {appendFile,mkdir,readFile,rename,rm,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {requireFleetIdentityShapeV1} from "../🛰️fleet/🪪️identity/🟦️.ts";

/** 🧬️ Who stored a cache entry and when, and for action results the fingerprint of what it names (the sorted output digests, which two honest machines produce identically for one content-derived unit key). */
export type ProvenanceRowV1=Readonly<{origin:string;atMs:number;fingerprint?:string}>;

/** ⚠️ One unit key that two origins produced differently: the first entry stays, the later write was refused. */
export type DivergenceV1=Readonly<{namespace:string;kind:"ac"|"cas"|"nx";key:string;keptOrigin:string;keptSha256:string;refusedOrigin:string;refusedSha256:string;atMs:number}>;

/** 📒️ The server's memory of origins and disagreements. */
export interface ProvenanceV1{
 record(namespace:string,kind:string,key:string,origin:string,fingerprint?:string):void;
 of(namespace:string,kind:string,key:string):ProvenanceRowV1|undefined;
 diverged(row:Omit<DivergenceV1,"atMs">):void;
 summary():Readonly<{count:number;recent:readonly DivergenceV1[]}>;
 flush():Promise<void>;
}

/** 🧮️ The fingerprint of the digests an action result names: SHA-256 of the sorted, newline-joined hashes. */
export const provenanceFingerprintV1=(hashes:readonly string[]):string=>createHash("sha256").update([...new Set(hashes)].sort().join("\n")).digest("hex");

const MAXIMUM=500000,RECENT=20,ROWS=1000;
const address=(namespace:string,kind:string,key:string):string=>`${namespace}\u0000${kind}\u0000${key}`;

/** 📒️ Opens the provenance of a server directory: `<directory>/fleet/provenance.jsonl` (appended, compacted when it holds twice as many lines as entries) and `<directory>/fleet/divergences.json`. Entries beyond 500 000 drop the oldest. Disk trouble never fails a request: the memory view keeps working. */
export async function openProvenanceV1(directory:string,options:Readonly<{now?:()=>number}>={}):Promise<ProvenanceV1>{
 const clock=options.now??Date.now,journal=join(directory,"fleet","provenance.jsonl"),sheet=join(directory,"fleet","divergences.json"),rows=new Map<string,ProvenanceRowV1>();
 let divergences:DivergenceV1[]=[],pending:string[]=[],lines=0,writing:Promise<void>=Promise.resolve();
 await mkdir(dirname(journal),{recursive:true,mode:0o700}).catch(()=>{});
 try{
  for(const line of(await readFile(journal,"utf8")).split("\n")){
   if(line==="")continue;lines++;
   try{const row=JSON.parse(line) as {n:string;k:string;y:string;o:string;t:number;f?:string};rows.delete(address(row.n,row.k,row.y));rows.set(address(row.n,row.k,row.y),{origin:row.o,atMs:row.t,...(row.f===undefined?{}:{fingerprint:row.f})});}catch{}
  }
 }catch{}
 try{const value=JSON.parse(await readFile(sheet,"utf8")) as unknown;requireFleetIdentityShapeV1("Divergences",value,"Closed divergences required");divergences=(value as {rows:DivergenceV1[]}).rows;}catch{}
 const trim=():void=>{while(rows.size>MAXIMUM){const oldest=rows.keys().next().value;if(oldest===undefined)break;rows.delete(oldest);}};
 trim();
 const flush=():Promise<void>=>writing=writing.then(async()=>{
  const batch=pending;pending=[];
  if(batch.length===0)return;
  try{
   if(lines+batch.length>2*Math.max(rows.size,1000)){
    const text=[...rows].map(([name,row])=>{const [n,k,y]=name.split("\u0000");return JSON.stringify({n,k,y,o:row.origin,t:row.atMs,...(row.fingerprint===undefined?{}:{f:row.fingerprint})});}).join("\n")+"\n",temporary=`${journal}.${process.pid}.tmp`;
    await writeFile(temporary,text,{mode:0o600});await rename(temporary,journal);lines=rows.size;
   }else{await appendFile(journal,batch.join(""),{mode:0o600});lines+=batch.length;}
  }catch{}
 });
 const timer=setInterval(()=>{void flush();},1000);timer.unref?.();
 return Object.freeze<ProvenanceV1>({
  record(namespace,kind,key,origin,fingerprint){
   const name=address(namespace,kind,key),row:ProvenanceRowV1={origin,atMs:clock(),...(fingerprint===undefined?{}:{fingerprint})};
   rows.delete(name);rows.set(name,row);trim();
   pending.push(JSON.stringify({n:namespace,k:kind,y:key,o:origin,t:row.atMs,...(fingerprint===undefined?{}:{f:fingerprint})})+"\n");
  },
  of:(namespace,kind,key)=>rows.get(address(namespace,kind,key)),
  diverged(row){
   const entry:DivergenceV1={...row,atMs:clock()};
   divergences=[...divergences.filter(old=>!(old.namespace===entry.namespace&&old.kind===entry.kind&&old.key===entry.key&&old.refusedOrigin===entry.refusedOrigin)),entry].slice(-ROWS);
   writing=writing.then(async()=>{const temporary=`${sheet}.${process.pid}.tmp`;try{await writeFile(temporary,JSON.stringify({version:1,rows:divergences})+"\n",{mode:0o600});await rename(temporary,sheet);}catch{await rm(temporary,{force:true}).catch(()=>{});}});
  },
  summary:()=>({count:divergences.length,recent:divergences.slice(-RECENT)}),
  flush:async()=>{await flush();await writing;},
 });
}
