import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {createHash} from "node:crypto";
import {cpSync,existsSync,mkdtempSync,readdirSync,readFileSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {installKillPointsV1,KillPointV1} from "../../🛡️resilience/🧪️tests/💥️killpoints.ts";

setDefaultTimeout(280_000);
const kill=await installKillPointsV1();
const {appendEventsV1,blobPathV1,formatHlcV1,storeBlobV1,storePathsV1}=await import("../../📒️journal/🟦️.ts");
const {fleetCompactV1,foldStoreV1}=await import("../../🔁️replica/🟦️.ts");
type FleetEventV1=import("../../📒️journal/🟦️.ts").FleetEventV1;
type FleetManifestEntryV1=import("../../📒️journal/🟦️.ts").FleetManifestEntryV1;

const scratch:string[]=[],day=86_400_000,hour=3_600_000,settings={automatic:true,snapshotEvents:1000,safetyHours:24,staleHours:12,catchUpEvents:1000};
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`compaction-kill-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const hlc=(ms:number,counter:number,origin:string):string=>formatHlcV1({ms,counter},origin);
const link=(origin:string,ms:number,counter:number,path:string):FleetEventV1=>({version:1,hlc:hlc(ms,counter,origin),operation:"link",path,target:"x",base:null});
const put=(origin:string,ms:number,counter:number,path:string,sha:string):FleetEventV1=>({version:1,hlc:hlc(ms,counter,origin),operation:"put",path,sha256:sha,bytes:2,executable:false,base:null});
const canonical=(value:unknown):string=>JSON.stringify(value,(_key,item:unknown)=>item!==null&&typeof item==="object"&&!Array.isArray(item)?Object.fromEntries(Object.entries(item as object).sort(([left],[right])=>left<right?-1:left>right?1:0)):item);
const digestOf=(manifest:ReadonlyMap<string,FleetManifestEntryV1>):string=>createHash("sha256").update([...manifest].sort(([left],[right])=>Buffer.compare(Buffer.from(left),Buffer.from(right))).map(([path,entry])=>`${path}\t${canonical(entry)}\n`).join("")).digest("hex");
async function folded(store:string):Promise<Map<string,FleetManifestEntryV1>>{
 const fold=await foldStoreV1(store),found=new Map<string,FleetManifestEntryV1>();
 for(const path of fold.manifest.keys())found.set(path,fold.manifest.get(path)!);
 return found;
}
const segmentsOf=(store:string,origin:string):number[]=>readdirSync(join(storePathsV1(store).origins,origin,"segments")).map(name=>Number(name.slice(0,12))).sort((left,right)=>left-right);

async function template():Promise<Readonly<{store:string;expected:string;kept:readonly string[];garbage:readonly string[]}>>{
 const store=temp("template"),kept:string[]=[],garbage:string[]=[],events:FleetEventV1[]=[];
 const blob=async(text:string,age:number):Promise<string>=>{
  const source=join(temp("source"),"file");
  writeFileSync(source,text);
  const sha=(await storeBlobV1(store,"a",source)).sha256,stamp=new Date(Date.now()-age);
  utimesSync(blobPathV1(store,sha),stamp,stamp);
  return sha;
 };
 for(const name of ["k1","k2"]){const sha=await blob(name,5*day);kept.push(sha);events.push(put("a",1_700_000_000_000+kept.length,kept.length,`kept/${name}`,sha));}
 for(const name of ["g1","g2","g3"])garbage.push(await blob(name,5*day));
 events.push(...Array.from({length:9000},(_,index)=>link("x",1_700_000_100_000+index,index,`bulk/${index}`)));
 for(const origin of ["a","x"]){await appendEventsV1(store,origin,events.filter(event=>event.hlc.endsWith(`-${origin}`)));}
 const expected=new Map<string,FleetManifestEntryV1>();
 for(const event of events)expected.set(event.path,event.operation==="put"?{hlc:event.hlc,operation:"put",sha256:event.sha256!,bytes:event.bytes!,executable:false}:{hlc:event.hlc,operation:"link",target:event.target!});
 return {store,expected:digestOf(expected),kept,garbage};
}

test("a kill at any mutation of a fleet compaction (snapshot, lock, retirement of segments, set-aside and removal of blobs, record) leaves a store that folds to the same manifest with every referenced blob in place, and the next compaction finishes the work",async()=>{
 const made=await template();
 const copy=(label:string):string=>{const store=temp(label);cpSync(made.store,store,{recursive:true,preserveTimestamps:true});return store;};
 const run=(store:string,now:number=Date.now()+2*day)=>fleetCompactV1(store,"a",settings,"apply",undefined,()=>{},()=>now);
 kill.reset();
 const dry=copy("dry"),first=await run(dry);
 const total=kill.count();
 expect(first.skipped).toBeNull();expect(first.segments).toBeGreaterThanOrEqual(2);expect(first.blobs).toBe(3);
 expect(total).toBeGreaterThan(15);
 expect(digestOf(await folded(dry))).toBe(made.expected);
 let killed=0;
 for(const mode of ["before","after"] as const)for(let index=0;index<total;index++){
  const store=copy(`${mode}${index}`);
  kill.reset();kill.arm(index,mode);
  let died=false;
  try{await run(store);}catch(error){if(!(error instanceof KillPointV1))throw error;died=true;}
  finally{await kill.disarm();}
  const context=`${mode} ${index} (${kill.last()[index]??"?"})`;
  if(died)killed++;
  expect(digestOf(await folded(store)),context).toBe(made.expected);
  for(const sha of made.kept)expect(existsSync(blobPathV1(store,sha)),`${context} kept blob`).toBe(true);
  const left=segmentsOf(store,"x");
  expect(left,context).toEqual(Array.from({length:left.length},(_,position)=>left[0]!+position));
  kill.reset();
  const again=await run(store,Date.now()+2*day+3*hour);
  expect(again.skipped,context).toBeNull();
  expect(digestOf(await folded(store)),context).toBe(made.expected);
  for(const sha of made.kept)expect(existsSync(blobPathV1(store,sha)),`${context} kept blob after`).toBe(true);
  for(const sha of made.garbage)expect(existsSync(blobPathV1(store,sha)),`${context} garbage after`).toBe(false);
  expect(existsSync(join(storePathsV1(store).root,"trash"))?readdirSync(join(storePathsV1(store).root,"trash")):[],context).toEqual([]);
  expect(existsSync(join(storePathsV1(store).root,"compaction.lock")),context).toBe(false);
  expect(segmentsOf(store,"x"),context).toEqual([2]);
  expect(JSON.parse(readFileSync(join(storePathsV1(store).root,"compaction.json"),"utf8")).originId,context).toBe("a");
 }
 expect(killed).toBeGreaterThan(total);
});
