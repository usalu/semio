import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {requireTestToolV1} from "../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {silentLatencyV1} from "../../🎣️tiers/🟦️.ts";
import {writeAttestationV1,writeRetirementV1,type FleetAttestationV1} from "../../🩺️attest/🟦️.ts";
import {appendEventsV1,blobPathV1,formatHlcV1,readHeadV1,storeBlobV1,storePathsV1,type FleetCursorV1,type FleetEventV1,type FleetManifestEntryV1} from "../../📒️journal/🟦️.ts";
import {autoCompactV1,closeReplicaV1,fleetCompactV1,followOnceV1,foldStoreV1,joinV1,openReplicaV1,publishOnceV1,seedV1,snapshotV1,statusV1,type ReplicaEventV1,type ReplicaOptionsV1} from "../../🔁️replica/🟦️.ts";
import {runReplicaCommandV1} from "../../🔁️replica/⌨️command/🟦️.ts";
import {readTopologyV1} from "../../🗺️topology/🟦️.ts";
import {compactionDefaultsV1,estimateEventsBehindV1,listSnapshotsV1,planRetirementV1,reclaimBlobsV1,reclaimLitterV1,readCompactionRecordV1,readSnapshotCursorsV1,recoverTrashV1,requireCompactionShapeV1,retireSegmentsV1,scanSnapshotV1,storeReportV1,withCompactionLockV1,type CompactionSettingsV1,type SnapshotRefV1} from "../🟦️.ts";

setDefaultTimeout(280_000);
const scratch:string[]=[],python=requireTestToolV1("python"),oraclePath=join(import.meta.dir,"🐍️.py"),hour=3_600_000,day=24*hour;
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`compaction-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const settings:CompactionSettingsV1={automatic:true,snapshotEvents:1000,safetyHours:24,staleHours:12,catchUpEvents:1000};
const hlc=(ms:number,counter:number,origin:string):string=>formatHlcV1({ms,counter},origin);
const put=(origin:string,ms:number,counter:number,path:string,sha:string):FleetEventV1=>({version:1,hlc:hlc(ms,counter,origin),operation:"put",path,sha256:sha,bytes:3,executable:false,base:null});
const del=(origin:string,ms:number,counter:number,path:string):FleetEventV1=>({version:1,hlc:hlc(ms,counter,origin),operation:"delete",path,base:null});
const link=(origin:string,ms:number,counter:number,path:string,target:string="../keep.txt"):FleetEventV1=>({version:1,hlc:hlc(ms,counter,origin),operation:"link",path,target,base:null});
const entryOf=(event:FleetEventV1):FleetManifestEntryV1=>event.operation==="put"?{hlc:event.hlc,operation:"put",sha256:event.sha256!,bytes:event.bytes!,executable:event.executable!}:event.operation==="link"?{hlc:event.hlc,operation:"link",target:event.target!}:{hlc:event.hlc,operation:"delete"};
const canonical=(value:unknown):string=>JSON.stringify(value,(_key,item:unknown)=>item!==null&&typeof item==="object"&&!Array.isArray(item)?Object.fromEntries(Object.entries(item as object).sort(([left],[right])=>left<right?-1:left>right?1:0)):item);
const bytewise=(left:string,right:string):number=>Buffer.compare(Buffer.from(left),Buffer.from(right));
const digestOf=(manifest:ReadonlyMap<string,FleetManifestEntryV1>):string=>{
 const hash=createHash("sha256");
 for(const path of [...manifest.keys()].sort(bytewise))hash.update(`${path}\t${canonical(manifest.get(path))}\n`);
 return hash.digest("hex");
};
const oracle=(events:readonly FleetEventV1[]):Map<string,FleetManifestEntryV1>=>{
 const found=new Map<string,FleetManifestEntryV1>();
 for(const event of events){const current=found.get(event.path);if(current===undefined||event.hlc>current.hlc)found.set(event.path,entryOf(event));}
 return found;
};
async function folded(store:string):Promise<Map<string,FleetManifestEntryV1>>{
 const fold=await foldStoreV1(store),found=new Map<string,FleetManifestEntryV1>();
 for(const path of fold.manifest.keys())found.set(path,fold.manifest.get(path)!);
 return found;
}
const pythonFold=(store:string,snapshot:boolean):Readonly<{entries:number;digest:string}>&{failed:boolean}=>{
 const result=spawnSync(python,[oraclePath,"fold",store,...snapshot?[]:["--no-snapshot"]],{encoding:"utf8"});
 return result.status===0?{...JSON.parse(result.stdout) as {entries:number;digest:string},failed:false}:{entries:0,digest:"",failed:true};
};
const segmentsOf=(store:string,origin:string):number[]=>readdirSync(join(storePathsV1(store).origins,origin,"segments")).filter(name=>/^[0-9]{12}\.jsonl$/u.test(name)).map(name=>Number(name.slice(0,12))).sort((left,right)=>left-right);
const future=(milliseconds:number):(()=>number)=>()=>Date.now()+milliseconds;
const attestationOf=(origin:string,wallMs:number,cursors:Record<string,FleetCursorV1>,sync:boolean=true):FleetAttestationV1=>({version:1,originId:origin,groupId:"g1",platform:"darwin",wallMs,hlc:hlc(wallMs,0,origin),sync,code:"0".repeat(64),cursors,files:0,manifest:"0".repeat(64),head:null,indexTree:null,view:null,guards:null,latency:silentLatencyV1(),queue:{incoming:0}});

let seedValue=1;
const random=():number=>{seedValue=(seedValue+0x6d2b79f5)>>>0;let t=seedValue;t=Math.imul(t^(t>>>15),t|1);t^=t+Math.imul(t^(t>>>7),t|61);return((t^(t>>>14))>>>0)/4294967296;};
const pick=<T>(items:readonly T[]):T=>items[Math.floor(random()*items.length)]!;

const names=["a.txt","b/c.txt","b/d/e.txt","Ä🙂/f.txt","ａ/g.txt","h","i/j","k.rs","l/m/n.rs","o.json","p/q","r"," s.txt"].flatMap(path=>[path,`${path}2`]);
const shas=Array.from({length:30},(_,index)=>createHash("sha256").update(`blob${index}`).digest("hex"));

function randomEvents(origins:readonly string[],count:number,lag:Readonly<Record<string,number>>,counters:Map<string,number>,base:number):FleetEventV1[]{
 const events:FleetEventV1[]=[];
 for(let index=0;index<count;index++){
  const origin=pick(origins),counter=(counters.get(origin)??0)+1;
  counters.set(origin,counter);
  const ms=base+Math.floor(index/pick([1,2,5,40]))-(lag[origin]??0),path=pick(names),kind=random();
  events.push(kind<0.6?put(origin,ms,counter,path,pick(shas)):kind<0.8?del(origin,ms,counter,path):link(origin,ms,counter,path,pick(["x","y/z"])));
 }
 return events;
}

async function append(store:string,events:readonly FleetEventV1[]):Promise<void>{
 const by=new Map<string,FleetEventV1[]>();
 for(const event of events){const origin=event.hlc.slice(21);by.set(origin,[...by.get(origin)??[],event]);}
 for(const [origin,list] of by)await appendEventsV1(store,origin,[...list].sort((left,right)=>left.hlc<right.hlc?-1:left.hlc>right.hlc?1:0));
}

test("the settings, the record and the store report have closed shapes and the committed topology carries the defaults",()=>{
 requireCompactionShapeV1("Settings",compactionDefaultsV1,"x");
 expect(compactionDefaultsV1.automatic).toBe(false);
 expect(readTopologyV1().replica.compaction).toEqual(compactionDefaultsV1);
 expect(()=>requireCompactionShapeV1("Settings",{...compactionDefaultsV1,safetyHours:0},"x")).toThrow();
 expect(()=>requireCompactionShapeV1("Settings",{...compactionDefaultsV1,extra:1},"x")).toThrow();
 expect(()=>requireCompactionShapeV1("Record",{version:1},"x")).toThrow();
});

test("the cursors of a snapshot are read from its tail and agree with the streamed reading for any layout of valid JSON",async()=>{
 const store=temp("tail");
 await append(store,[put("a",1_700_000_000_000,1,"x.txt",shas[0]!),link("b",1_700_000_000_001,1,"y","z")]);
 const snapshot=await snapshotV1(store,"a");
 expect(snapshot).toBeDefined();
 const cursors=await readSnapshotCursorsV1(snapshot!.file);
 expect(cursors).toEqual({a:{segment:0,bytes:statSync(join(storePathsV1(store).origins,"a","segments","000000000000.jsonl")).size},b:{segment:0,bytes:statSync(join(storePathsV1(store).origins,"b","segments","000000000000.jsonl")).size}});
 const document=JSON.parse(readFileSync(snapshot!.file,"utf8")) as {hlc:string;manifest:object;cursors:object};
 const spread=join(temp("tail-layout"),"pretty.json");
 writeFileSync(spread,JSON.stringify({cursors:document.cursors,manifest:document.manifest,hlc:document.hlc},null,2));
 expect(await readSnapshotCursorsV1(spread)).toEqual(cursors);
 writeFileSync(spread,"{\"hlc\":");
 expect(await readSnapshotCursorsV1(spread)).toBeUndefined();
 expect(await scanSnapshotV1(snapshot!.file,snapshot!.hlc)).toBe(2);
 await expect(scanSnapshotV1(snapshot!.file,hlc(1,0,"a"))).rejects.toThrow();
});

test("a snapshot plus the journal after its cursors folds to exactly the manifest of the full journal, through random histories with deletes, links, ties, a lagging clock and a retired origin, also after segments were retired (property test against an independent fold and the Python oracle)",async()=>{
 let retiredSomething=0;
 for(let seed=1;seed<=24;seed++){
  seedValue=seed*7919;
  let retiredHere=0;
  const store=temp(`prop${seed}`),origins=["a","b","c","d"],all:FleetEventV1[]=[],counters=new Map<string,number>(),big=seed%6===0;
  await writeRetirementV1(store,{version:1,originId:"c",by:"a",hlc:hlc(1_700_000_000_000,0,"a"),wallMs:1_700_000_000_000,reason:"t",retired:true});
  let base=1_700_000_000_000;
  for(let round=0;round<1+seed%3;round++){
   const events=randomEvents(origins,big?18000:60+Math.floor(random()*200),{d:5000},counters,base);
   base+=100_000;
   all.push(...events);
   await append(store,events);
   const before=pythonFold(store,false);
   if(retiredHere===0){expect(before.failed).toBe(false);expect(before.digest).toBe(digestOf(oracle(all)));}
   else expect(before.failed).toBe(true);
   if(round%2===0){
    const result=await fleetCompactV1(store,"a",settings,"apply",undefined,()=>{},future(3*day));
    expect(result.skipped).toBeNull();
    for(const origin of origins){const kept=segmentsOf(store,origin);expect(kept).toEqual(Array.from({length:kept.length},(_,index)=>kept[0]!+index));}
    retiredSomething+=result.segments;retiredHere+=result.segments;
   }
   expect(digestOf(await folded(store))).toBe(digestOf(oracle(all)));
   const viaSnapshot=pythonFold(store,true);
   expect(viaSnapshot.failed).toBe(false);
   expect(viaSnapshot.digest).toBe(digestOf(oracle(all)));
   expect(viaSnapshot.entries).toBe(oracle(all).size);
  }
  expect(canonical([...(await folded(store))].sort(([left],[right])=>bytewise(left,right)))).toBe(canonical([...oracle(all)].sort(([left],[right])=>bytewise(left,right))));
 }
 expect(retiredSomething).toBeGreaterThan(0);
});

test("the retirement plan removes only segments that the snapshot covers, that are below the head and below every fresh synchronising origin's attested cursor, and that are old enough, always as a prefix; stale, off, retired and unattested origins do not hold it back (checked against the Python oracle)",async()=>{
 const store=temp("plan");
 await append(store,Array.from({length:9000},(_,index)=>link("x",1_700_000_000_000+index,index,`bulk/${index}`)));
 await append(store,Array.from({length:10},(_,index)=>link("y",1_700_000_000_000+index,index,`other/${index}`)));
 const made=await snapshotV1(store,"a"),ref=(await listSnapshotsV1(store))[0]!;
 expect(made!.created).toBe(true);
 expect(ref.cursors.x!.segment).toBe(2);
 const now=Date.now()+2*day,plan=async(cursors:Record<string,FleetCursorV1>,at:number=now,staleHours:number=12)=>planRetirementV1(store,{...ref,cursors},{now:at,safetyMilliseconds:day,staleMilliseconds:staleHours*hour});
 expect((await plan(ref.cursors)).origins.x!.segments).toEqual([0,1]);
 expect((await plan(ref.cursors)).origins.y!.segments).toEqual([]);
 expect((await plan(ref.cursors,Date.now())).segments).toBe(0);
 expect((await plan({...ref.cursors,x:{segment:1,bytes:0}})).origins.x!.segments).toEqual([0]);
 expect((await plan({})).segments).toBe(0);
 const verify=async(label:string):Promise<void>=>{
  const expected=JSON.parse(spawnSync(python,[oraclePath,"retire",store,String(now),"24","12",ref.origin,ref.hlc],{encoding:"utf8"}).stdout) as Record<string,number[]>,actual=await plan(ref.cursors);
  for(const origin of Object.keys(expected))expect(actual.origins[origin]!.segments,`${label} ${origin}`).toEqual(expected[origin]!);
 };
 await verify("no attestations");
 expect((await plan(ref.cursors)).ignored).toEqual({x:"unattested",y:"unattested"});
 await writeAttestationV1(store,attestationOf("x",now,{x:{segment:2,bytes:0},y:{segment:0,bytes:0}}));
 await writeAttestationV1(store,attestationOf("y",now,{x:{segment:1,bytes:5},y:{segment:0,bytes:0}}));
 let held=await plan(ref.cursors);
 expect(held.origins.x).toMatchObject({segments:[0],holder:"y",limit:1});
 await verify("holder y at segment 1");
 await writeAttestationV1(store,attestationOf("y",now,{x:{segment:1,bytes:5}},false));
 held=await plan(ref.cursors);
 expect(held.ignored.y).toBe("off");expect(held.origins.x!.segments).toEqual([0,1]);
 await verify("y off");
 await writeAttestationV1(store,attestationOf("y",now-13*hour,{x:{segment:0,bytes:0}}));
 held=await plan(ref.cursors);
 expect(held.ignored.y).toBe("stale");expect(held.origins.x!.segments).toEqual([0,1]);
 expect((await plan(ref.cursors,now,24)).origins.x!.segments).toEqual([]);
 await verify("y stale");
 await writeAttestationV1(store,attestationOf("y",now,{x:{segment:0,bytes:0}}));
 expect((await plan(ref.cursors)).origins.x!.segments).toEqual([]);
 await writeRetirementV1(store,{version:1,originId:"y",by:"x",hlc:hlc(now,0,"x"),wallMs:now,reason:"t",retired:true});
 held=await plan(ref.cursors);
 expect(held.ignored.y).toBe("retired");expect(held.origins.x!.segments).toEqual([0,1]);
 await verify("y retired");
 await writeAttestationV1(store,attestationOf("y",now,{y:{segment:0,bytes:0}}));
 await writeRetirementV1(store,{version:1,originId:"y",by:"x",hlc:hlc(now+1,0,"x"),wallMs:now,reason:"t",retired:false});
 expect((await plan(ref.cursors)).origins.x).toMatchObject({segments:[],holder:"y"});
 const folder=join(storePathsV1(store).origins,"x","segments"),stamp=new Date(),old=new Date(Date.now()-3*day);
 utimesSync(join(folder,"000000000000.jsonl"),old,old);utimesSync(join(folder,"000000000001.jsonl"),stamp,stamp);
 await writeAttestationV1(store,attestationOf("y",Date.now()+hour,{x:{segment:2,bytes:0}}));
 const young=await planRetirementV1(store,{...ref},{now:Date.now()+hour,safetyMilliseconds:day,staleMilliseconds:12*hour});
 expect(young.origins.x!.segments).toEqual([0]);
});

test("an attestation that lacks fields of the current schema still holds the retirement back with its cursors, one that cannot be read at all holds it back with none until it is stale, and a missing one does not",async()=>{
 const store=temp("drift");
 await append(store,Array.from({length:9000},(_,index)=>link("x",1_700_000_000_000+index,index,`bulk/${index}`)));
 await append(store,[link("m",1_700_000_000_000,1,"m/1")]);
 await snapshotV1(store,"a");
 const ref=(await listSnapshotsV1(store))[0]!,now=Date.now()+2*day,plan=()=>planRetirementV1(store,ref,{now,safetyMilliseconds:day,staleMilliseconds:12*hour});
 const file=join(storePathsV1(store).origins,"m","attest.json");
 writeFileSync(file,JSON.stringify({version:1,originId:"m",wallMs:now,sync:true,cursors:{x:{segment:1,bytes:0}}}));
 expect((await plan()).origins.x).toMatchObject({segments:[0],holder:"m"});
 writeFileSync(file,"{\"wallMs\":\"now\"");
 utimesSync(file,new Date(now),new Date(now));
 expect((await plan()).origins.x).toMatchObject({segments:[],holder:"m"});
 const old=new Date(now-13*hour);
 utimesSync(file,old,old);
 const stale=await plan();
 expect(stale.ignored.m).toBe("stale");expect(stale.origins.x!.segments).toEqual([0,1]);
 rmSync(file);
 expect((await plan()).ignored.m).toBe("unattested");
});

test("retiring removes a prefix of segments oldest first and refuses without the snapshot that justifies it",async()=>{
 const store=temp("retire");
 await append(store,Array.from({length:9000},(_,index)=>link("x",1_700_000_000_000+index,index,`bulk/${index}`)));
 await snapshotV1(store,"a");
 const ref=(await listSnapshotsV1(store))[0]!,plan=await planRetirementV1(store,ref,{now:Date.now()+2*day,safetyMilliseconds:day,staleMilliseconds:12*hour}),steps:number[]=[];
 const removed=await retireSegmentsV1(store,plan,{progress:done=>steps.push(done)});
 expect(removed.segments).toBe(2);expect(steps).toEqual([1,2]);expect(segmentsOf(store,"x")).toEqual([2]);
 rmSync(ref.file);
 await expect(retireSegmentsV1(store,plan)).rejects.toThrow();
});

test("a replica whose cursor points into a removed segment resumes from the snapshot instead of failing, whether it attested long ago, never, or is the fresh origin that kept a cursor behind (then nothing is removed)",async()=>{
 const store=temp("resume"),a={name:"a",options:{worktree:temp("a"),storeRoot:store,originId:"a",groupId:"g1",pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0,restartJitterMilliseconds:0,minimumUptimeMilliseconds:0,logFiles:[] as string[]} satisfies ReplicaOptionsV1};
 mkdirSync(a.options.worktree,{recursive:true});writeFileSync(join(a.options.worktree,"keep.txt"),"keep\n");
 await seedV1(a.options);
 const b:ReplicaOptionsV1={...a.options,worktree:temp("b"),originId:"b"};
 await joinV1(b);
 const events:ReplicaEventV1[]=[],replica=await openReplicaV1(b,event=>events.push(event));
 try{
  await append(store,Array.from({length:9000},(_,index)=>link("x",1_700_000_000_000+index,index,`bulk/${index}`)));
  const when=Date.now()+2*day;
  await writeAttestationV1(store,attestationOf("b",when,{}));
  const guarded=await fleetCompactV1(store,"a",settings,"apply",undefined,()=>{},()=>when);
  expect(guarded.segments).toBe(0);
  expect(segmentsOf(store,"x")).toEqual([0,1,2]);
  await writeAttestationV1(store,attestationOf("b",when-13*hour,{}));
  const released=await fleetCompactV1(store,"a",settings,"apply",undefined,()=>{},()=>when);
  expect(released.ignored.b).toBe("stale");expect(released.segments).toBeGreaterThanOrEqual(2);
  expect(segmentsOf(store,"x")).toEqual([2]);
  for(let round=0;round<400;round++){const result=await followOnceV1(replica);if(result.events===0&&replica.queue.size===0)break;}
  expect(events.some(event=>event.event==="rejoin-required")).toBe(false);
  expect(events.some(event=>event.event==="progress"&&event.stage==="rejoin")).toBe(true);
  expect(readdirSync(join(b.worktree,"bulk")).length).toBe(9000);
  const head=(await readHeadV1(store,"x"))!;
  expect(replica.cursors.x).toEqual({segment:head.segment,bytes:head.bytes});
 }finally{await closeReplicaV1(replica);}
});

test("a replica that is far behind jumps to the newest snapshot and reads only the journal after it, a replica with the setting off reads everything, and a replica that joins a compacted store needs no segment that was removed",async()=>{
 const store=temp("jump"),options=(name:string,automatic:boolean):ReplicaOptionsV1=>({worktree:temp(name),storeRoot:store,originId:name,groupId:"g1",pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0,restartJitterMilliseconds:0,minimumUptimeMilliseconds:0,logFiles:[] as string[],compaction:{...settings,automatic}});
 const a=options("a",true);
 mkdirSync(a.worktree,{recursive:true});writeFileSync(join(a.worktree,"keep.txt"),"keep\n");
 await seedV1(a);
 const b=options("b",true),c=options("c",false);
 await joinV1(b);await joinV1(c);
 const eventsB:ReplicaEventV1[]=[],replicaB=await openReplicaV1(b,event=>eventsB.push(event)),replicaC=await openReplicaV1(c);
 try{
  await append(store,Array.from({length:9000},(_,index)=>link("x",1_700_000_000_000+index,index,`bulk/${index}`)));
  await snapshotV1(store,"a");
  await append(store,Array.from({length:50},(_,index)=>link("x",1_700_000_100_000+index,9000+index,`tail/${index}`)));
  const jumped=await followOnceV1(replicaB);
  expect(eventsB.some(event=>event.event==="progress"&&event.stage==="jump")).toBe(true);
  expect(jumped.events).toBeLessThan(200);
  let total=0,result=await followOnceV1(replicaC);
  total+=result.events;
  while(result.events>0){result=await followOnceV1(replicaC);total+=result.events;}
  expect(total).toBe(9050);
  for(let round=0;round<40&&replicaB.queue.size+replicaC.queue.size>0;round++){await followOnceV1(replicaB);await followOnceV1(replicaC);}
  for(const who of [b,c]){
   expect(readdirSync(join(who.worktree,"bulk")).length).toBe(9000);
   expect(readdirSync(join(who.worktree,"tail")).length).toBe(50);
  }
  await fleetCompactV1(store,"a",settings,"apply",undefined,()=>{},future(2*day));
  expect(segmentsOf(store,"x")[0]).toBeGreaterThan(0);
  const d=options("d",true);
  await joinV1(d);
  expect(readdirSync(join(d.worktree,"bulk")).length).toBe(9000);
 }finally{await closeReplicaV1(replicaB);await closeReplicaV1(replicaC);}
});

async function blobOf(store:string,text:string,ageMilliseconds:number):Promise<string>{
 const source=join(temp("source"),"file");
 writeFileSync(source,text);
 const stored=await storeBlobV1(store,"a",source),stamp=new Date(Date.now()-ageMilliseconds);
 utimesSync(blobPathV1(store,stored.sha256),stamp,stamp);
 return stored.sha256;
}
const present=(store:string,sha:string):boolean=>existsSync(blobPathV1(store,sha));
const trashOf=(store:string):string[]=>existsSync(join(storePathsV1(store).root,"trash"))?readdirSync(join(storePathsV1(store).root,"trash")):[];

async function blobWorld():Promise<{store:string;ref:SnapshotRefV1;shas:Record<string,string>}>{
 const store=temp("blobs"),shas:Record<string,string>={};
 for(const [name,age] of Object.entries({b1:5*day,b2:5*day,b3:0,b4:5*day,b5:5*day,b6:5*day}))shas[name]=await blobOf(store,name,age);
 await append(store,[put("a",1_700_000_000_000,1,"p1",shas.b1!),put("a",1_700_000_000_000,2,"p5",shas.b5!),del("a",1_700_000_000_001,3,"p5")]);
 await snapshotV1(store,"a");
 await append(store,[put("a",1_700_000_000_002,4,"p4",shas.b4!)]);
 return {store,ref:(await listSnapshotsV1(store))[0]!,shas};
}

test("a blob goes when neither the snapshot nor a later event names it and it is old enough; younger ones stay and are counted; a plan removes nothing",async()=>{
 const {store,ref,shas}=await blobWorld(),base={now:Date.now(),safetyMilliseconds:day};
 const plan=await reclaimBlobsV1(store,ref,{...base,apply:false});
 expect(plan).toMatchObject({blobs:3,pending:1,aborted:false});
 for(const sha of Object.values(shas))expect(present(store,sha)).toBe(true);
 const done=await reclaimBlobsV1(store,ref,{...base,apply:true});
 expect(done).toMatchObject({blobs:3,pending:1,restored:0,aborted:false});
 expect(done.bytes).toBe(plan.bytes);
 expect(["b1","b3","b4"].map(name=>present(store,shas[name]!))).toEqual([true,true,true]);
 expect(["b2","b5","b6"].map(name=>present(store,shas[name]!))).toEqual([false,false,false]);
 expect(trashOf(store)).toEqual([]);
 expect(await reclaimBlobsV1(store,ref,{...base,apply:true})).toMatchObject({blobs:0,pending:1});
});

test("a publish that races with a reclaim never ends with an event whose blob is gone: the event appended while the blob is set aside, the republish of a blob that was set aside, a touch between scan and removal, a segment that vanishes, and the leftovers of a reclaim that died",async()=>{
 const base=(): {now:number;safetyMilliseconds:number}=>({now:Date.now(),safetyMilliseconds:day});
 {
  const {store,ref,shas}=await blobWorld();
  const result=await reclaimBlobsV1(store,ref,{...base(),apply:true,between:async held=>{if(held.includes(shas.b2!))await append(store,[put("a",1_700_000_000_010,5,"late",shas.b2!)]);}});
  expect(result.restored).toBeGreaterThanOrEqual(1);expect(present(store,shas.b2!)).toBe(true);expect(present(store,shas.b6!)).toBe(false);expect(trashOf(store)).toEqual([]);
 }
 {
  const {store,ref,shas}=await blobWorld(),source=join(temp("again"),"file");
  writeFileSync(source,"b2");
  let created:boolean|undefined;
  const result=await reclaimBlobsV1(store,ref,{...base(),apply:true,between:async held=>{if(!held.includes(shas.b2!))return;created=(await storeBlobV1(store,"a",source,{known:{sha256:shas.b2!,bytes:2}})).created;await append(store,[put("a",1_700_000_000_010,5,"again",shas.b2!)]);}});
  expect(created).toBe(true);expect(present(store,shas.b2!)).toBe(true);expect(result.blobs).toBe(2);expect(readFileSync(blobPathV1(store,shas.b2!),"utf8")).toBe("b2");expect(trashOf(store)).toEqual([]);
 }
 {
  const {store,ref,shas}=await blobWorld(),stamp=new Date();
  const result=await reclaimBlobsV1(store,ref,{...base(),apply:true,batch:1,scanned:async sha=>{if(sha===shas.b6)utimesSync(blobPathV1(store,sha),stamp,stamp);}});
  expect(present(store,shas.b6!)).toBe(true);expect(result.restored).toBe(1);expect(result.blobs).toBe(2);expect(trashOf(store)).toEqual([]);
 }
 {
  const {store,ref,shas}=await blobWorld(),source=join(temp("touch"),"file");
  writeFileSync(source,"b6");
  const result=await reclaimBlobsV1(store,ref,{...base(),apply:true,scanned:async sha=>{if(sha===shas.b6)expect((await storeBlobV1(store,"a",source,{known:{sha256:shas.b6!,bytes:2}})).created).toBe(false);}});
  expect(present(store,shas.b6!)).toBe(true);expect(result.blobs).toBe(2);
 }
 {
  const {store,ref,shas}=await blobWorld();
  await append(store,Array.from({length:4200},(_,index)=>link("z",1_700_000_200_000+index,index,`z/${index}`)));
  rmSync(join(storePathsV1(store).origins,"z","segments","000000000000.jsonl"));
  const result=await reclaimBlobsV1(store,ref,{...base(),apply:true});
  expect(result).toMatchObject({aborted:true,blobs:0});
  for(const sha of Object.values(shas))expect(present(store,sha)).toBe(true);
 }
 {
  const {store,ref,shas}=await blobWorld();
  await append(store,Array.from({length:4200},(_,index)=>link("z",1_700_000_200_000+index,index,`z/${index}`)));
  const result=await reclaimBlobsV1(store,ref,{...base(),apply:true,between:async()=>{
   await append(store,Array.from({length:4100},(_,index)=>link("z",1_700_000_300_000+index,10_000+index,`z/late${index}`)));
   rmSync(join(storePathsV1(store).origins,"z","segments","000000000001.jsonl"),{force:true});
  }});
  expect(result.aborted).toBe(true);expect(result.blobs).toBe(0);
  for(const sha of Object.values(shas))expect(present(store,sha)).toBe(true);
  expect(trashOf(store)).toEqual([]);
 }
 {
  const {store,ref,shas}=await blobWorld(),trash=join(storePathsV1(store).root,"trash");
  mkdirSync(trash,{recursive:true});
  const left=await blobOf(store,"left",5*day);
  await append(store,[put("a",1_700_000_000_020,6,"left",left)]);
  const moved=join(trash,`dead-${left}`);
  writeFileSync(moved,readFileSync(blobPathV1(store,left)));rmSync(blobPathV1(store,left));
  expect(await recoverTrashV1(store)).toBe(1);
  expect(present(store,left)).toBe(true);expect(trashOf(store)).toEqual([]);
  expect((await reclaimBlobsV1(store,ref,{...base(),apply:true})).blobs).toBeGreaterThanOrEqual(2);
  expect(present(store,shas.b1!)).toBe(true);
 }
});

test("a file that went back to the content of a blob that was reclaimed publishes that blob again and reaches the other replica",async()=>{
 const store=temp("revert"),options=(name:string):ReplicaOptionsV1=>({worktree:temp(name),storeRoot:store,originId:name,groupId:"g1",pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0,restartJitterMilliseconds:0,minimumUptimeMilliseconds:0,logFiles:[] as string[],compaction:settings}),a=options("a"),b=options("b");
 mkdirSync(a.worktree,{recursive:true});writeFileSync(join(a.worktree,"keep.txt"),"one\n");
 await seedV1(a);await joinV1(b);
 const replicaA=await openReplicaV1(a),replicaB=await openReplicaV1(b);
 try{
  writeFileSync(join(a.worktree,"keep.txt"),"two\n");utimesSync(join(a.worktree,"keep.txt"),new Date(Date.now()+2000),new Date(Date.now()+2000));
  await publishOnceV1(replicaA);await followOnceV1(replicaB);
  expect(readFileSync(join(b.worktree,"keep.txt"),"utf8")).toBe("two\n");
  const old=createHash("sha256").update("one\n").digest("hex");
  expect(present(store,old)).toBe(true);
  const stamp=new Date(Date.now()-5*day);utimesSync(blobPathV1(store,old),stamp,stamp);
  const result=await fleetCompactV1(store,"a",settings,"apply");
  expect(result.blobs).toBeGreaterThanOrEqual(1);expect(present(store,old)).toBe(false);
  writeFileSync(join(a.worktree,"keep.txt"),"one\n");utimesSync(join(a.worktree,"keep.txt"),new Date(Date.now()+4000),new Date(Date.now()+4000));
  await publishOnceV1(replicaA);await followOnceV1(replicaB);
  expect(present(store,old)).toBe(true);
  expect(readFileSync(join(b.worktree,"keep.txt"),"utf8")).toBe("one\n");
 }finally{await closeReplicaV1(replicaA);await closeReplicaV1(replicaB);}
});

test("two machines compacting at once are safe: the second finds the lock held, a lock nobody refreshed is taken over, and two snapshots written at once are both complete folds",async()=>{
 const store=temp("lock");
 await append(store,Array.from({length:30},(_,index)=>link("x",1_700_000_000_000+index,index,`l/${index}`)));
 let inner:unknown="unset";
 const first=await withCompactionLockV1(store,"a",async()=>{inner=await withCompactionLockV1(store,"b",async()=>"never");return "ran";});
 expect(first).toEqual({value:"ran"});expect(inner).toBeUndefined();
 expect(existsSync(join(storePathsV1(store).root,"compaction.lock"))).toBe(false);
 mkdirSync(join(storePathsV1(store).root,"compaction.lock"));
 expect(await withCompactionLockV1(store,"b",async()=>"x")).toBeUndefined();
 expect(await withCompactionLockV1(store,"b",async()=>"x",()=>Date.now()+3*hour)).toEqual({value:"x"});
 const [one,two]=await Promise.all([snapshotV1(store,"a"),snapshotV1(store,"b")]);
 expect(one!.hlc).toBe(two!.hlc);
 const listed=await listSnapshotsV1(store);
 expect(listed.length).toBe(2);
 for(const item of listed)expect(await scanSnapshotV1(item.file,item.hlc)).toBe(30);
 const [first2,second2]=await Promise.all([fleetCompactV1(store,"a",settings,"apply"),fleetCompactV1(store,"b",settings,"apply")]);
 expect([first2.skipped,second2.skipped].filter(item=>item==="locked").length).toBeLessThanOrEqual(1);
 expect(digestOf(await folded(store))).toBe(digestOf(oracle(Array.from({length:30},(_,index)=>link("x",1_700_000_000_000+index,index,`l/${index}`)))));
});

test("the command plans without writing and applies on request, in both languages, and status shows the journal, the snapshot and what could be reclaimed",async()=>{
 const store=temp("command"),root=temp("root");
 mkdirSync(root,{recursive:true});writeFileSync(join(root,"keep.txt"),"keep\n");
 const options:ReplicaOptionsV1={worktree:root,storeRoot:store,originId:"g1-m01",groupId:"g1",pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0};
 await seedV1(options);
 await append(store,Array.from({length:9000},(_,index)=>link("x",1_700_000_000_000+index,index,`bulk/${index}`)));
 const run=async(args:readonly string[]):Promise<{exitCode:number;lines:string[]}>=>{
  const lines:string[]=[];
  const result=await runReplicaCommandV1([...args,"--store",store],{root,environment:{},signal:new AbortController().signal,write:line=>lines.push(line),notice:line=>lines.push(line),readStdin:async()=>""});
  return {exitCode:result.exitCode,lines};
 };
 await expect(run(["compaction","--lang","en"])).rejects.toThrow("exactly one of --plan or --apply");
 await expect(run(["compaction","--plan","--apply","--lang","en"])).rejects.toThrow("--plan");
 expect((await run(["compaction","--plan","--lang","en"])).lines.join("\n")).toContain("No snapshot exists yet");
 expect(await listSnapshotsV1(store)).toEqual([]);
 await snapshotV1(store,"g1-m01");
 const planned=await run(["compaction","--plan","--lang","de"]);
 expect(planned.exitCode).toBe(0);expect(planned.lines.join("\n")).toContain("Es wurde nichts geändert");
 expect(segmentsOf(store,"x")).toEqual([0,1,2]);
 const status=await statusV1(options);
 expect(status.store).not.toBeNull();expect(status.store!.journal.segments).toBeGreaterThanOrEqual(4);expect(status.store!.snapshot).not.toBeNull();
 requireCompactionShapeV1("Store",status.store,"x");
 const text=(await run(["status","--lang","en"])).lines.join("\n"),german=(await run(["status","--lang","de"])).lines.join("\n");
 expect(text).toContain("Journal:");expect(text).toContain("Newest snapshot by g1-m01");expect(text).toContain("Reclaimable now:");
 expect(german).toContain("Neuester Schnappschuss von g1-m01");expect(german).toContain("Jetzt freigebbar:");
 expect(text).not.toContain("Reclaimable blobs");
 expect((await statusV1(options)).store!.reclaimable.blobs).toBeNull();
 expect((await statusV1(options,true)).store!.reclaimable.blobs).toEqual({count:0,bytes:0,pending:0,pendingBytes:0});
 expect((await run(["status","--deep","--lang","en"])).lines.join("\n")).toContain("Reclaimable blobs (measured by --deep): 0");
 const stamp=new Date(Date.now()-3*day);
 for(const number of segmentsOf(store,"x")){utimesSync(join(storePathsV1(store).origins,"x","segments",`${String(number).padStart(12,"0")}.jsonl`),stamp,stamp);}
 const applied=await run(["compaction","--apply","--lang","en"]);
 expect(applied.exitCode).toBe(0);expect(applied.lines.join("\n")).toContain("Compacted on a snapshot");
 expect(segmentsOf(store,"x")).toEqual([2]);
 const record=await readCompactionRecordV1(store);
 expect(record).toMatchObject({originId:"g1-m01",segments:{removed:2}});
 const after=await statusV1(options);
 expect(after.store!.last).toMatchObject({blobs:0});
 expect((await run(["status","--lang","de"])).lines.join("\n")).toContain("Letzte Verdichtung vor");
 expect((await storeReportV1(store,compactionDefaultsV1,Date.now())).journal.segments).toBeLessThan(status.store!.journal.segments);
});

test("the automatic compaction runs only on the group server with the setting on, only when a snapshot is due, and never twice within ten minutes",async()=>{
 const store=temp("auto"),options=(origin:string,automatic:boolean):ReplicaOptionsV1=>({worktree:temp(origin),storeRoot:store,originId:origin,groupId:"g1",pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0,compaction:{...settings,automatic,snapshotEvents:20}});
 const server=options("g1-server",true);
 mkdirSync(server.worktree,{recursive:true});
 for(let index=0;index<30;index++)writeFileSync(join(server.worktree,`f${index}.txt`),`${index}\n`);
 await seedV1(server);
 const workerOptions=options("g1-m09",true);
 await joinV1(workerOptions);
 const off=await openReplicaV1({...server,compaction:{...settings,automatic:false}}),worker=await openReplicaV1(workerOptions);
 try{
  expect(autoCompactV1(off)).toBeUndefined();expect(autoCompactV1(worker)).toBeUndefined();
  expect(await listSnapshotsV1(store)).toEqual([]);
 }finally{await closeReplicaV1(off);await closeReplicaV1(worker);}
 const on=await openReplicaV1(server);
 try{
  const running=autoCompactV1(on);
  expect(running).toBeDefined();await running;
  expect((await listSnapshotsV1(store)).length).toBe(1);
  expect(await readCompactionRecordV1(store)).toMatchObject({originId:"g1-server"});
  expect(autoCompactV1(on)).toBeUndefined();
 }finally{await closeReplicaV1(on);}
});

test("the number of events behind a cursor is estimated from the heads without reading the journal",()=>{
 const heads=new Map([["x",{version:1 as const,segment:3,bytes:1000,events:100,hlc:hlc(1,0,"x")}],["y",undefined],["self",{version:1 as const,segment:9,bytes:1,events:1,hlc:hlc(1,0,"self")}]]);
 expect(estimateEventsBehindV1({},heads,"self")).toBe(3*4096+100);
 expect(estimateEventsBehindV1({x:{segment:3,bytes:500}},heads,"self")).toBe(50);
 expect(estimateEventsBehindV1({x:{segment:1,bytes:0}},heads,"self")).toBe(2*4096+100);
 expect(estimateEventsBehindV1({x:{segment:3,bytes:1000}},heads,"self")).toBe(0);
});

test("AppleDouble sidecars below fleet and bootstrap at any depth and next to the root directories are swept when the file they belong to exists, a lone ._ file there is kept",async()=>{
 const store=temp("litter-deep"),paths=storePathsV1(store),day2=2*day,fleet=join(paths.root,"fleet","install","blobs","e0"),code=join(paths.root,"bootstrap","code","a".repeat(64),"files","mod");
 mkdirSync(fleet,{recursive:true});mkdirSync(code,{recursive:true});mkdirSync(paths.heads,{recursive:true});
 const paired=[join(fleet,"._blob"),join(code,"._🟦️.ts"),join(paths.root,"fleet","._cache.json"),join(paths.root,"._heads")],real=[join(fleet,"blob"),join(code,"🟦️.ts"),join(paths.root,"fleet","cache.json")],lone=join(code,"._lone");
 for(const file of [...real,...paired.slice(0,3)])writeFileSync(file,"x".repeat(file.includes("._")?4096:3));
 writeFileSync(paired[3]!,"x".repeat(4096));writeFileSync(lone,"keep");
 const before=Date.now();
 expect(await reclaimLitterV1(store,{olderThanMilliseconds:day,now:before})).toEqual({files:0,bytes:0});
 const swept=await reclaimLitterV1(store,{olderThanMilliseconds:day,now:before+day2});
 expect(swept).toEqual({files:4,bytes:4*4096});
 for(const file of paired)expect(existsSync(file),file).toBe(false);
 for(const file of [...real,lone])expect(existsSync(file),file).toBe(true);
});

test("AppleDouble sidecars and .DS_Store files that macOS clients left in the store are removed once they are old enough, nothing else is touched, and a compaction apply sweeps them",async()=>{
 const store=temp("litter"),paths=storePathsV1(store),sha=createHash("sha256").update("abc").digest("hex");
 await append(store,[put("a",1_700_000_000_000,0,"x.txt",sha)]);
 const real=blobPathV1(store,sha);
 mkdirSync(join(real,".."),{recursive:true});writeFileSync(real,"abc");
 const old=[join(paths.heads,"._h"),join(paths.snapshots,"._s"),join(paths.snapshots,".DS_Store"),join(paths.origins,"._o"),join(paths.origins,"a","._a"),join(paths.origins,"a","segments","._000000000000.jsonl"),join(real,"..","._"+sha),join(paths.tmp,"._t")];
 mkdirSync(paths.heads,{recursive:true});mkdirSync(paths.snapshots,{recursive:true});mkdirSync(paths.tmp,{recursive:true});mkdirSync(join(paths.origins,"._dir"),{recursive:true});
 for(const file of old)writeFileSync(file,"x".repeat(4096));
 const young=join(paths.heads,"._young");writeFileSync(young,"x");
 const keep=[real,...readdirSync(paths.origins).filter(name=>!name.startsWith(".")).map(name=>join(paths.origins,name))];
 const before=Date.now();
 const none=await reclaimLitterV1(store,{olderThanMilliseconds:day,now:before});
 expect(none).toEqual({files:0,bytes:0});
 const swept=await reclaimLitterV1(store,{olderThanMilliseconds:day,now:before+2*day});
 expect(swept.files).toBe(old.length+2);
 expect(swept.bytes).toBe(old.length*4096+1);
 for(const file of [...old,young,join(paths.origins,"._dir")])expect(existsSync(file),file).toBe(false);
 for(const file of keep)expect(existsSync(file),file).toBe(true);
 for(const file of old.slice(0,3))writeFileSync(file,"y");
 const result=await fleetCompactV1(store,"a",settings,"apply",undefined,()=>{},()=>Date.now()+3*day);
 expect(result.skipped).toBeNull();
 for(const file of old.slice(0,3))expect(existsSync(file),file).toBe(false);
});
