import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {createGraphPhaseClockV1} from "../../../../../⏱️phases/🟨️.mjs";
import {runWarmLanesV1,warmLineV1} from "../../🟨️.mjs";
const source=await Bun.file(new URL("../../../🟨️.mjs",import.meta.url)).text();
import fixture from "./🔣️.json" with {type:"json"};

test("warm attempts and preparations stay outside completed inference",async()=>{
 const db=new Database(":memory:"),resources:string[]=[],completed:number[]=[];
 const phases=createGraphPhaseClockV1({publish:(resource:string)=>{resources.push(resource);},minimumMilliseconds:0});
 await phases.begin("inference",fixture.total);
 const counted=source.indexOf("  let staleTold="),afterCounted=source.indexOf("  const resolveOwners",counted);
 const reporter=source.indexOf("  let warmState="),afterReporter=source.indexOf("  const warmOwners=",reporter);
 const warm=source.indexOf("tell:fields=>{",afterReporter)+5,afterWarm=source.indexOf("});try{phases?.running",warm);
 expect(Math.min(counted,afterCounted,reporter,afterReporter,warm,afterWarm)).toBeGreaterThan(0);
 const make=new Function("phases","options","jobs","takeSay","warmLineV1","warmStartedMs","finished",
  source.slice(counted,afterCounted)+source.slice(reporter,afterReporter)+"return {counted,reportProgress,tell:"+source.slice(warm,afterWarm)+"}");
 const work=make(phases,{onOwnerProgress:(count:number)=>completed.push(count)},Array(fixture.total),()=>undefined,warmLineV1,performance.now(),fixture.skipped) as {
  counted(name:string):void; reportProgress(previousCount:number):void;
  tell:NonNullable<Parameters<typeof runWarmLanesV1>[0]["tell"]>;
 };
 try{
  const counters=await runWarmLanesV1({
   lanes:[fixture.warm.map(row=>row.name)],inspect:()=>({key:"fixture",stamp:"unchanged"}),
   index:{has:(name:string)=>fixture.warm.some(row=>row.name===name&&row.outcome==="durable")},
   warm:async(name:string)=>fixture.warm.some(row=>row.name===name&&row.outcome==="settled")?{peakMegabytes:1}:undefined,
   tell:work.tell,progressMilliseconds:0,now:()=>0,
  });
  const sql=db.query("select count(*) attempts,sum(json_extract(value,'$.outcome') in ('settled','durable')) successes from json_each(?)").get(JSON.stringify(fixture.warm)) as {attempts:number;successes:number};
  expect(counters.done).toBe(sql.attempts);
  expect(completed).toEqual([]);
  const warm=resources.filter(resource=>resource.includes(":warm|warm|"));
  expect(Number(warm.at(-1)?.split("|").at(-1))).toBe(sql.successes);
  expect(warm.map(resource=>Number(resource.split(":")[3]))).toEqual(warm.map(()=>0));
  phases.running("inference",[]);
  work.reportProgress(fixture.skipped);
  for(const [index,row] of fixture.completed.entries()){
   work.counted(row.name);work.reportProgress(fixture.skipped+index+1);
  }
  const observed=db.query("select count(distinct json_extract(value,'$.name')) count from json_each(?)").get(JSON.stringify(fixture.completed)) as {count:number};
  expect(completed).toEqual(Array.from({length:observed.count},(_,index)=>index+1));
  expect((await phases.end("inference")).done).toBe(observed.count);
 }finally{db.close();}
});

test("the final warm count is emitted once when the last owner settles at a progress boundary",async()=>{
 const db=new Database(":memory:"),rows:number[]=[];
 try{
  const expected=db.query("select 1 as done where length(?1)>0").all(fixture.lastWarmOwner) as {done:number}[];
  await runWarmLanesV1({
   lanes:[[fixture.lastWarmOwner]],
   inspect:name=>({key:name}),
   warm:async()=>({peakMegabytes:1}),
   progressMilliseconds:0,
   tell:fields=>rows.push(fields.done),
  });
  expect(rows).toEqual(expected.map(row=>row.done));
 }finally{db.close();}
});
