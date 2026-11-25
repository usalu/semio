import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {collectGarbageSingleFlightV1,reclaimUnitStorageV1} from "../../🟦️.ts";
import {collectRustImageV1} from "../../♻️auto-reclaim/🟦️.ts";
import {lanePathsV1,listLanesV1} from "../../../🧵️lane/🟦️.ts";
import {collectTemporariesV1,measureTemporariesV1,planTemporariesV1,temporariesHoldersV1,TEMPORARIES_POLICY_V1,type TemporariesInputV1} from "../🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),schemaPath=join(here,"..","🧬️schema","🔣️.json"),hour=3_600_000,grace=TEMPORARIES_POLICY_V1.graceMilliseconds,idle=TEMPORARIES_POLICY_V1.idleMilliseconds;
const oracle=(states:readonly TemporariesInputV1[]):unknown=>{const run=spawnSync("python3",["-I",join(here,"🐍️.py"),schemaPath],{input:JSON.stringify(states),encoding:"utf8",maxBuffer:1<<28});if(run.status!==0)throw Error("Python temporaries oracle failed: "+run.stderr);return JSON.parse(run.stdout);};
const dead=()=>false;

test("every vector validates against the schema and matches the planner and the independent Python oracle",()=>{
 for(const row of fixture.vectors)expect(validateJsonSchemaSubset({definitions:schema.definitions,$ref:"#/definitions/Vector"},row),row.name).toEqual([]);
 fixture.vectors.forEach(row=>expect<unknown>(planTemporariesV1(row.input as TemporariesInputV1),row.name).toEqual(row.remove));
 expect<unknown>(oracle(fixture.vectors.map(row=>row.input as TemporariesInputV1))).toEqual(fixture.vectors.map(row=>row.remove));
});

test("300 seeded random states give the same plan from the planner and the independent oracle",()=>{
 let seed=20261125;const next=(limit:number)=>{seed=(seed*1103515245+12345)&0x7fffffff;return (seed>>>8)%limit;},states:TemporariesInputV1[]=[];
 for(let round=0;round<300;round++)states.push({nowMs:next(4_000_000),lanes:Array.from({length:next(6)},(_,lane)=>({id:`l${lane}${next(3)}`,holders:Array.from({length:next(3)},()=>({startedMs:next(6)===0?null:next(2_000_000)})),files:Array.from({length:next(8)},(_,file)=>({path:`/z${lane}/f${file}-${next(4)}.o`,bytes:next(5000),mtimeMs:next(2_000_000)}))}))});
 const answers=oracle(states) as unknown[];let removals=0;
 states.forEach((state,index)=>{const plan=planTemporariesV1(state);removals+=plan.length;expect<unknown>(plan).toEqual(answers[index]);for(const row of plan){const lane=state.lanes.find(item=>item.id===row.lane)!;expect(lane.holders.every(holder=>holder.startedMs!==null)).toBe(true);}});
 expect(removals).toBeGreaterThan(300);
});

type Spec=Readonly<{id:string;files:readonly (readonly [string,number,number?])[];holders?:readonly string[]}>;
function build(specs:readonly Spec[],nowMs=Date.now()){
 const store=mkdtempSync(join(tmpdir(),"stmp-")),root=join(store,"u");
 for(const spec of specs){
  const paths=lanePathsV1(root,spec.id);mkdirSync(paths.live,{recursive:true});mkdirSync(paths.temporaries,{recursive:true});writeFileSync(paths.metadata,"{}");
  for(const [name,ageMs,bytes] of spec.files){const path=join(paths.temporaries,name);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,Buffer.alloc(bytes??10,1));const time=new Date(nowMs-ageMs);utimesSync(path,time,time);}
  (spec.holders??[]).forEach((text,index)=>writeFileSync(join(paths.live,`4242-${index}`),text));
 }
 return{store,root,names:(id:string)=>{const out:string[]=[],walk=(directory:string)=>{if(!existsSync(directory))return;for(const entry of readdirSync(directory,{withFileTypes:true}))entry.isDirectory()?walk(join(directory,entry.name)):out.push(entry.name);};walk(lanePathsV1(root,id).temporaries);return out.sort();}};
}
const holder=(startedMs:number|undefined)=>JSON.stringify({pid:4242,startMs:1,...startedMs===undefined?{}:{startedMs}});
const alive=()=>true;

test("an idle lane is collected, nested directories included, and nothing outside its temporaries directory is touched",async()=>{
 const{store,root,names}=build([{id:"a1",files:[["x.rcgu.o",5*hour],["sub/y.rcgu.o",hour,100],["sub/deep/z.o",idle+1000],["fresh.o",idle/2]],holders:[holder(Date.now()-hour)]},{id:"b2",files:[["keep.o",hour]],holders:[holder(undefined)]}]);
 try{
  const result=await collectTemporariesV1(root,{dryRun:false,alive:dead});
  expect(result.removedFiles).toBe(4);expect(names("a1")).toEqual(["fresh.o"]);expect(names("b2")).toEqual([]);
  expect(existsSync(lanePathsV1(root,"a1").temporaries)).toBe(true);expect(existsSync(join(lanePathsV1(root,"a1").temporaries,"sub"))).toBe(false);expect(existsSync(lanePathsV1(root,"a1").metadata)).toBe(true);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("an idle lane keeps every file younger than the idle grace, as a compile without a live marker needs them",async()=>{
 const now=Date.now(),{store,root,names}=build([{id:"f1",files:[["old.o",idle+1000],["edge.o",idle/2],["new.o",1000]]}],now);
 try{
  const result=await collectTemporariesV1(root,{dryRun:false,now:()=>now});
  expect(result.removedFiles).toBe(1);expect(names("f1")).toEqual(["edge.o","new.o"]);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("a live lane keeps every file from the grace before its oldest holder's start on, and the holder's own newer files",async()=>{
 const now=Date.now(),{store,root,names}=build([{id:"l1",files:[["old.o",3*hour],["near.o",hour+grace/2],["new.o",hour/2],["fresh.o",1000]],holders:[holder(now-hour)]}],now);
 try{
  const progress:number[]=[],result=await collectTemporariesV1(root,{dryRun:false,alive,progress:done=>progress.push(done)});
  expect(names("l1")).toEqual(["fresh.o","near.o","new.o"]);expect(result.removedFiles).toBe(1);expect(result.liveLanes).toBe(1);expect(progress.length).toBeGreaterThan(0);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("an unreadable or start-less holder record keeps everything of the lane, a dead holder's record is no holder",async()=>{
 const{store,root,names}=build([{id:"u1",files:[["a.o",9*hour]],holders:["{not json"]},{id:"u2",files:[["a.o",9*hour]],holders:[holder(undefined)]},{id:"u3",files:[["a.o",9*hour]],holders:[holder(Date.now()-hour)]}]);
 try{
  expect(temporariesHoldersV1(lanePathsV1(root,"u1").live,alive)).toEqual([{startedMs:null}]);expect(temporariesHoldersV1(lanePathsV1(root,"nowhere").live,alive)).toEqual([]);
  const lanes=await measureTemporariesV1(root,{alive:pid=>pid===4242&&false});expect(lanes.every(lane=>lane.holders.length===0||lane.id==="u1")).toBe(true);
  await collectTemporariesV1(root,{dryRun:false,alive:()=>true});
  expect(names("u1")).toEqual(["a.o"]);expect(names("u2")).toEqual(["a.o"]);expect(names("u3")).toEqual([]);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("liveness is read again before each batch: a holder that appeared since the measurement keeps its newer files; a vanished file is fine",async()=>{
 const now=Date.now(),{store,root,names}=build([{id:"r1",files:[["a.o",3*hour],["b.o",3*hour],["c.o",hour/2],["d.o",hour/4]],holders:[holder(now-2*hour)]}],now);
 try{
  let calls=0;
  const result=await collectTemporariesV1(root,{dryRun:false,alive:()=>{calls++;if(calls===2)rmSync(join(lanePathsV1(root,"r1").temporaries,"b.o"));return calls>=2;}});
  expect(names("r1")).toEqual(["c.o","d.o"]);expect(result.removedFiles).toBe(1);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("cancellation stops a collection between batches",async()=>{
 const controller=new AbortController(),{store,root,names}=build([{id:"c1",files:Array.from({length:600},(_,index)=>[`f${index}.o`,hour,1] as const)}]);
 try{
  await expect(collectTemporariesV1(root,{dryRun:false,alive:dead,signal:controller.signal,progress:(_done,_total,phase)=>{if(phase==="remove")controller.abort();}})).rejects.toBeDefined();
  expect(names("c1").length).toBe(600-TEMPORARIES_POLICY_V1.batchFiles);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("removal stops as soon as the need is satisfied, and a need of zero never stops it",async()=>{
 const{store,root,names}=build([{id:"n1",files:Array.from({length:50},(_,index)=>[`f${index}.o`,hour,1] as const)}]);
 try{
  const early=await collectTemporariesV1(root,{dryRun:false,alive:dead,batchFiles:10,needBytes:()=>1,satisfied:()=>true});
  expect(early.removedFiles).toBe(10);expect(names("n1").length).toBe(40);
  const all=await collectTemporariesV1(root,{dryRun:false,alive:dead,batchFiles:10,needBytes:()=>0,satisfied:()=>true});
  expect(all.removedFiles).toBe(40);expect(names("n1")).toEqual([]);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("a dry run removes nothing and reports the counts and bytes in English and German",async()=>{
 const{store,root,names}=build([{id:"d1",files:[["a.o",hour,2048],["b.o",hour,2048]]},{id:"d2",files:[["a.o",hour,10]],holders:[holder(undefined)]}]);
 try{
  const en:string[]=[],de:string[]=[];
  const result=await collectTemporariesV1(root,{dryRun:true,alive,report:{language:"en",write:line=>en.push(line)}});
  await collectTemporariesV1(root,{dryRun:true,alive,report:{language:"de",write:line=>de.push(line)}});
  expect(result.dryRun).toBe(true);expect(result.removedFiles).toBe(2);expect(result.files).toBe(3);expect(result.freedBytes).toBeGreaterThanOrEqual(4096);
  expect(names("d1").length).toBe(2);expect(names("d2").length).toBe(1);
  expect(en.some(line=>line.includes("would remove 2 of 3 files"))).toBe(true);expect(de.some(line=>line.includes("würde 2 von 3 Dateien entfernen"))).toBe(true);expect(en.some(line=>line.includes("lane d1"))).toBe(true);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("the lane measurement counts the temporaries and records them beside the size",()=>{
 const{store,root}=build([{id:"m1",files:[["a.o",hour,5000],["s/b.o",hour,3000]]}]);
 try{
  const[lane]=listLanesV1(root,Date.now());
  expect(lane!.temporariesBytes).toBe(8000);expect(lane!.bytes).toBeGreaterThanOrEqual(8000);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("reclaiming before a reservation refuses collects the temporaries first, in German on request",async()=>{
 const{store,root,names}=build([{id:"q1",files:[["a.o",hour,100],["b.o",hour,100]]}]);
 try{
  const lines:string[]=[];
  await reclaimUnitStorageV1(store,1,{report:{language:"de",write:line=>lines.push(line)}});
  expect(names("q1")).toEqual([]);expect(lines[0]).toContain("Compiler-Temporärdateien");
  void root;
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("the single-flight collector removes the temporaries only under need, and keeps them while space is plentiful",async()=>{
 const{store,root,names}=build([{id:"s1",files:[["a.o",hour,100]]}]);
 try{
  const plenty=await collectGarbageSingleFlightV1(root,1_000_000_000,{volume:()=>({volumeBytes:1_000_000_000,freeBytes:900_000_000})});
  expect(plenty.running).toBe(false);expect(names("s1")).toEqual(["a.o"]);if(!plenty.running)expect(plenty.applied.temporaries).toBe(0);
  const tight=await collectGarbageSingleFlightV1(root,1_000_000_000,{volume:()=>({volumeBytes:1_000_000_000,freeBytes:100_000_000})});
  expect(tight.running).toBe(false);expect(names("s1")).toEqual([]);if(!tight.running)expect(tight.applied.temporaries).toBe(1);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("the repository collector deletes nothing without need but a dry run still reports the temporaries",async()=>{
 const{store,names}=build([{id:"g1",files:[["a.o",hour,100]]}]);
 try{
  const run=async(dryRun:boolean,freeBytes:number)=>{const lines:string[]=[];await collectRustImageV1(store,{dryRun,language:"en",demandBytes:0,quotaBytes:0,unitBudgetBytes:1,measure:()=>({volumeBytes:1_000_000_000,freeBytes}),control:()=>({}),write:line=>lines.push(line)});return lines;};
  await run(false,900_000_000);expect(names("g1")).toEqual(["a.o"]);
  expect((await run(true,900_000_000)).some(line=>line.includes("compiler temporaries: would remove 1 of 1"))).toBe(true);expect(names("g1")).toEqual(["a.o"]);
  await run(false,100_000_000);expect(names("g1")).toEqual([]);
 }finally{rmSync(store,{recursive:true,force:true});}
});
