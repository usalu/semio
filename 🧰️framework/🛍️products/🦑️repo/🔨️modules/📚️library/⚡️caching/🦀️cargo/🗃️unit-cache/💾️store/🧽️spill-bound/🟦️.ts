import {existsSync,readdirSync,readFileSync,rmSync,statSync,statfsSync,utimesSync} from "node:fs";
import {dirname,join} from "node:path";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1.spill,sha=/^[0-9a-f]{64}$/;
/** 📐️ The authored bound of the spill folder.
 * @see ../../🧬️schema/🔣️.json */
export const SPILL_POLICY_V1=policy;
/** 📏️ What the spill folder may hold in bytes: at most `maximumBytes`, `sharePercent` of its volume, and what leaves `keepFreePercent` of the volume free once the spill is counted as the room it already uses. Pure and language-agnostic.
 * @see ./🧪️tests/🐍️.py */
export function spillBoundBytesV1(facts:Readonly<{totalBytes:number;availableBytes:number;spillBytes:number}>,override:Partial<typeof policy>={}):number{
 const rule={...policy,...override};
 return Math.max(0,Math.min(rule.maximumBytes,Math.floor(facts.totalBytes*rule.sharePercent/100),facts.spillBytes+facts.availableBytes-Math.floor(facts.totalBytes*rule.keepFreePercent/100)));
}
/** 💧️ One spilled object of the folder: its name, size and last use. */
export type SpillObjectV1=Readonly<{sha256:string;bytes:number;usedMs:number}>;
/** 💧️ One spilled entry: its key, its last use and the objects it names. */
export type SpillEntryV1=Readonly<{key:string;usedMs:number;objects:readonly string[]}>;
/** 🧮️ Plans what leaves the spill folder (objects sorted by name, entries in eviction order): objects no entry names, entries without any object, then the least recently used entries (and the objects nothing else names) until the rest fits `boundBytes`; whatever was used within `protectMilliseconds` before `nowMs` stays, whatever the bound. Pure and deterministic. */
export function planSpillEvictionV1(input:Readonly<{nowMs:number;boundBytes:number;objects:readonly SpillObjectV1[];entries:readonly SpillEntryV1[]}>,override:Partial<typeof policy>={}):Readonly<{entries:string[];objects:string[];bytes:number}>{
 const protect=input.nowMs-{...policy,...override}.protectMilliseconds,present=new Map(input.objects.map(row=>[row.sha256,row])),refs=new Map<string,number>(),usedOf=(entry:SpillEntryV1)=>Math.max(entry.usedMs,...entry.objects.map(name=>present.get(name)?.usedMs??0));
 for(const entry of input.entries)for(const name of new Set(entry.objects))refs.set(name,(refs.get(name)??0)+1);
 const objects:string[]=[],entries:string[]=[];let total=input.objects.reduce((sum,row)=>sum+row.bytes,0),bytes=0;
 const drop=(name:string)=>{const row=present.get(name);if(row===undefined)return;present.delete(name);objects.push(name);total-=row.bytes;bytes+=row.bytes;};
 for(const row of input.objects)if(!refs.has(row.sha256)&&row.usedMs<protect)drop(row.sha256);
 const kept:SpillEntryV1[]=[];
 for(const entry of input.entries){if(usedOf(entry)<protect&&!entry.objects.some(name=>present.has(name)))entries.push(entry.key);else kept.push(entry);}
 for(const entry of kept.filter(row=>usedOf(row)<protect).sort((left,right)=>usedOf(left)-usedOf(right)||(left.key<right.key?-1:1))){
  if(total<=input.boundBytes)break;
  entries.push(entry.key);
  for(const name of new Set(entry.objects)){const left=(refs.get(name)??1)-1;refs.set(name,left);if(left===0)drop(name);}
 }
 return Object.freeze({entries,objects:objects.sort(),bytes});
}
/** 🕰️ Marks a spilled object as used now, so a running upload keeps what it reads. */
export function touchSpillObjectV1(path:string,nowMs=Date.now()):void{try{utimesSync(path,nowMs/1000,nowMs/1000);}catch{}}
/** 🧽️ What one pruning of the spill folder did. */
export type SpillPruneV1=Readonly<{boundBytes:number;keptBytes:number;removedBytes:number;removedEntries:number;removedObjects:number}>;
/** 🧽️ Bounds the spill folder `directory` to what its volume can spare ({@link spillBoundBytesV1}, less `incomingBytes` the caller is about to write): stale temporary files go, then the least recently used entries beyond the bound ({@link planSpillEvictionV1}); what a running build or upload used lately stays. Never throws; an absent folder holds nothing. */
export function pruneSpillV1(directory:string,options:Readonly<{nowMs?:number;incomingBytes?:number;boundBytes?:number}>={}):SpillPruneV1{
 const nowMs=options.nowMs??Date.now(),objects:SpillObjectV1[]=[],entries:SpillEntryV1[]=[],entryDirectory=join(directory,"entries");let removedBytes=0;
 const names=(path:string)=>{try{return readdirSync(path);}catch{return[];}};
 for(const path of[directory,entryDirectory])for(const name of names(path)){
  if(sha.test(name)&&path===directory){try{const stat=statSync(join(path,name));if(stat.isFile())objects.push({sha256:name,bytes:stat.size,usedMs:stat.mtimeMs});}catch{}continue;}
  if(name.endsWith(".tmp"))try{const stat=statSync(join(path,name));if(nowMs-stat.mtimeMs>policy.temporaryMilliseconds){rmSync(join(path,name),{force:true});removedBytes+=stat.size;}}catch{}
 }
 for(const name of names(entryDirectory).filter(row=>row.endsWith(".json"))){
  try{const path=join(entryDirectory,name),entry=JSON.parse(readFileSync(path,"utf8")) as{outputs?:{sha256:string}[];stdout?:string|null;stderr?:string|null};entries.push({key:name.slice(0,-5),usedMs:statSync(path).mtimeMs,objects:[...(entry.outputs??[]).map(row=>row.sha256),entry.stdout,entry.stderr].filter((value):value is string=>typeof value==="string")});}catch{}
 }
 const held=objects.reduce((sum,row)=>sum+row.bytes,0);let boundBytes=options.boundBytes;
 if(boundBytes===undefined){try{let probe=directory;while(!existsSync(probe)&&dirname(probe)!==probe)probe=dirname(probe);const stats=statfsSync(probe),size=Number(stats.bsize);boundBytes=spillBoundBytesV1({totalBytes:Number(stats.blocks)*size,availableBytes:Number(stats.bavail)*size,spillBytes:held});}catch{boundBytes=policy.maximumBytes;}}
 const plan=planSpillEvictionV1({nowMs,boundBytes:Math.max(0,boundBytes-(options.incomingBytes??0)),objects,entries});
 for(const key of plan.entries)rmSync(join(entryDirectory,key+".json"),{force:true});
 for(const name of plan.objects)rmSync(join(directory,name),{force:true});
 return Object.freeze({boundBytes,keptBytes:held-plan.bytes,removedBytes:removedBytes+plan.bytes,removedEntries:plan.entries.length,removedObjects:plan.objects.length});
}
