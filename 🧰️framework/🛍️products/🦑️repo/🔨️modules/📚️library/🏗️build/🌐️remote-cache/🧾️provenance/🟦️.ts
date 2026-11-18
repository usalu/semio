import {createHash} from "node:crypto";
import {appendFile,mkdir,readFile,rename,rm,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";
import {requireFleetIdentityShapeV1} from "../🛰️fleet/🪪️identity/🟦️.ts";

/** 🧬️ Who stored a cache entry and when, and for action results the fingerprint of what it names (the sorted output digests without the volatile manifests, which two honest machines produce identically for one content-derived unit key) and whether it was a compiled unit (it carried such a manifest). */
export type ProvenanceRowV1=Readonly<{origin:string;atMs:number;fingerprint?:string;unit?:true}>;

/** ⚠️ One unit key that two origins produced differently: the first entry stays, the later write was refused. */
export type DivergenceV1=Readonly<{namespace:string;kind:"ac"|"cas"|"nx";key:string;keptOrigin:string;keptSha256:string;refusedOrigin:string;refusedSha256:string;atMs:number}>;

/** 🔀️ What one producer-consumer pair of origins exchanged: action results read (`hits`) and, among them, compiled units (`units`). */
export type ConsumptionPairV1=Readonly<{hits:number;units:number}>;

/** 🔀️ Who read whose results: one row per `producer>consumer` pair of origin ids (`unknown` for an entry stored before the server kept provenance, `static` and `anonymous` for readers without a fleet identity, `*>*` for pairs beyond the bound), counted from `since` (milliseconds, 0 before the first read). A pair with the same origin on both sides is a machine reading its own result back. */
export type ConsumptionV1=Readonly<{since:number;total:number;pairs:Readonly<Record<string,ConsumptionPairV1>>}>;

/** 📒️ The server's memory of origins and disagreements. */
export interface ProvenanceV1{
 record(namespace:string,kind:string,key:string,origin:string,fingerprint?:string,unit?:boolean):void;
 of(namespace:string,kind:string,key:string):ProvenanceRowV1|undefined;
 diverged(row:Omit<DivergenceV1,"atMs">,sort?:string):void;
 consume(namespace:string,kind:string,key:string,consumer:string):void;
 consumption():ConsumptionV1;
 summary():Readonly<{count:number;recent:readonly DivergenceV1[];since:number;byClass:Readonly<Record<string,number>>}>;
 flush():Promise<void>;
}

/** 🧮️ The fingerprint of the digests an action result names: SHA-256 of the sorted, newline-joined hashes. */
export const provenanceFingerprintV1=(hashes:readonly string[]):string=>createHash("sha256").update([...new Set(hashes)].sort().join("\n")).digest("hex");

/** 🕰️ The output names that record when and how long a unit was built (`createdMs`, `costMs`), so two honest builds of one key differ in them. They are left out of the fingerprint, otherwise every duplicate build would count as a divergence and be refused; the unit-cache manifest name (`manifestName` of its fleet policy) is the one entry. */
export const PROVENANCE_VOLATILE_OUTPUTS_V1:readonly string[]=Object.freeze(["unit.json"]);

const MAXIMUM=500000,RECENT=20,ROWS=1000,PAIRS=1024,OVERFLOW="*>*";
const address=(namespace:string,kind:string,key:string):string=>`${namespace}\u0000${kind}\u0000${key}`;

/** 📒️ Opens the provenance of a server directory: `<directory>/fleet/provenance.jsonl` (appended, compacted when it holds twice as many lines as entries), `<directory>/fleet/divergences.json` and `<directory>/fleet/consumption.json` (at most 1024 pairs). Entries beyond 500 000 drop the oldest. Disk trouble never fails a request: the memory view keeps working. */
export async function openProvenanceV1(directory:string,options:Readonly<{now?:()=>number}>={}):Promise<ProvenanceV1>{
 const clock=options.now??Date.now,journal=join(directory,"fleet","provenance.jsonl"),sheet=join(directory,"fleet","divergences.json"),tally=join(directory,"fleet","consumption.json"),rows=new Map<string,ProvenanceRowV1>(),pairs=new Map<string,{hits:number;units:number}>();
 const classes=new Map<string,number>(),opened=clock();
 let divergences:DivergenceV1[]=[],pending:string[]=[],lines=0,writing:Promise<void>=Promise.resolve(),total=0,since=0,dirty=false;
 await mkdir(dirname(journal),{recursive:true,mode:0o700}).catch(()=>{});
 try{
  for(const line of(await readFile(journal,"utf8")).split("\n")){
   if(line==="")continue;lines++;
   try{const row=JSON.parse(line) as {n:string;k:string;y:string;o:string;t:number;f?:string;u?:1};rows.delete(address(row.n,row.k,row.y));rows.set(address(row.n,row.k,row.y),{origin:row.o,atMs:row.t,...(row.f===undefined?{}:{fingerprint:row.f}),...(row.u===1?{unit:true as const}:{})});}catch{}
  }
 }catch{}
 try{const value=JSON.parse(await readFile(sheet,"utf8")) as unknown;requireFleetIdentityShapeV1("Divergences",value,"Closed divergences required");divergences=(value as {rows:DivergenceV1[]}).rows;}catch{}
 try{const value=JSON.parse(await readFile(tally,"utf8")) as {since:number;total:number;pairs:Record<string,ConsumptionPairV1>};requireFleetIdentityShapeV1("Consumption",value,"Closed consumption required");since=value.since;total=value.total;for(const [name,pair] of Object.entries(value.pairs).slice(0,PAIRS))pairs.set(name,{hits:pair.hits,units:pair.units});}catch{}
 const trim=():void=>{while(rows.size>MAXIMUM){const oldest=rows.keys().next().value;if(oldest===undefined)break;rows.delete(oldest);}};
 trim();
 const flush=():Promise<void>=>writing=writing.then(async()=>{
  const batch=pending;pending=[];
  if(batch.length===0)return;
  try{
   if(lines+batch.length>2*Math.max(rows.size,1000)){
    const text=[...rows].map(([name,row])=>{const [n,k,y]=name.split("\u0000");return JSON.stringify({n,k,y,o:row.origin,t:row.atMs,...(row.fingerprint===undefined?{}:{f:row.fingerprint}),...(row.unit===true?{u:1}:{})});}).join("\n")+"\n",temporary=`${journal}.${process.pid}.tmp`;
    await writeFile(temporary,text,{mode:0o600});await rename(temporary,journal);lines=rows.size;
   }else{await appendFile(journal,batch.join(""),{mode:0o600});lines+=batch.length;}
  }catch{}
 });
 const snapshot=():ConsumptionV1=>({since,total,pairs:Object.fromEntries([...pairs].map(([name,pair])=>[name,{hits:pair.hits,units:pair.units}]))});
 const persist=():Promise<void>=>writing=writing.then(async()=>{
  if(!dirty)return;dirty=false;
  const temporary=`${tally}.${process.pid}.tmp`;
  try{await writeFile(temporary,JSON.stringify({version:1,...snapshot()})+"\n",{mode:0o600});await rename(temporary,tally);}catch{dirty=true;await rm(temporary,{force:true}).catch(()=>{});}
 });
 const timer=setInterval(()=>{void flush();void persist();},1000);timer.unref?.();
 return Object.freeze<ProvenanceV1>({
  record(namespace,kind,key,origin,fingerprint,unit){
   const name=address(namespace,kind,key),row:ProvenanceRowV1={origin,atMs:clock(),...(fingerprint===undefined?{}:{fingerprint}),...(unit===true?{unit:true as const}:{})};
   rows.delete(name);rows.set(name,row);trim();
   pending.push(JSON.stringify({n:namespace,k:kind,y:key,o:origin,t:row.atMs,...(fingerprint===undefined?{}:{f:fingerprint}),...(unit===true?{u:1}:{})})+"\n");
  },
  of:(namespace,kind,key)=>rows.get(address(namespace,kind,key)),
  diverged(row,sort="other"){
   classes.set(sort,(classes.get(sort)??0)+1);
   const entry:DivergenceV1={...row,atMs:clock()};
   divergences=[...divergences.filter(old=>!(old.namespace===entry.namespace&&old.kind===entry.kind&&old.key===entry.key&&old.refusedOrigin===entry.refusedOrigin)),entry].slice(-ROWS);
   writing=writing.then(async()=>{const temporary=`${sheet}.${process.pid}.tmp`;try{await writeFile(temporary,JSON.stringify({version:1,rows:divergences})+"\n",{mode:0o600});await rename(temporary,sheet);}catch{await rm(temporary,{force:true}).catch(()=>{});}});
  },
  consume(namespace,kind,key,consumer){
   const row=rows.get(address(namespace,kind,key));let name=`${row?.origin??"unknown"}>${consumer}`;
   if(!pairs.has(name)&&pairs.size>=PAIRS)name=OVERFLOW;
   const pair=pairs.get(name)??{hits:0,units:0};pair.hits++;if(row?.unit===true)pair.units++;pairs.set(name,pair);
   total++;if(since===0)since=clock();dirty=true;
  },
  consumption:snapshot,
  summary:()=>({count:divergences.length,recent:divergences.slice(-RECENT),since:opened,byClass:Object.fromEntries([...classes].sort(([a],[b])=>a<b?-1:1))}),
  flush:async()=>{await flush();await persist();await writing;},
 });
}
