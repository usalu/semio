import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import {relieveUnitCachePressureV1,retryWhenFullV1,startUnitCacheCollectorV1} from "../../🚪️launch/🟦️.ts";
import {tryClaimV1} from "../../🔒️lock/🟦️.ts";
import {applyGcYieldingV1,collectGarbageSingleFlightV1,gcNeedBytesV1,gcOutOfSpaceV1,gcPressureV1,planGcV1,type GcPressureV1} from "../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

const rules=UNIT_CACHE_POLICY_V1.gc,GIB=1024**3,base=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(base,{recursive:true});
const scratch=()=>mkdtempSync(join(base,"ucp-"));
const ajv=new Ajv({strict:true});ajv.addSchema(schema);
const validator=(name:string)=>ajv.getSchema(`${schema.$id}#/definitions/${name}`)!;

function pythonOracle(program:string,input:unknown):unknown{
 const run=spawnSync(testPythonV1()??"python3",["-I","-c",program],{input:JSON.stringify(input),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);
}

const pythonPressure=`
import json,sys
v=json.load(sys.stdin);r=v["rules"]
def need(volume,free):
    used=max(0,volume-free)
    return used-int(volume*r["lowWaterShare"]) if used>int(volume*r["highWaterShare"]) else 0
def decide(i):
    age=float("inf") if i["lastLaunchMs"] is None else i["nowMs"]-i["lastLaunchMs"]
    if i["volumeBytes"]-i["freeBytes"]<=int(i["volumeBytes"]*r["highWaterShare"]):
        return "maintain" if age>=r["maintenanceIntervalMilliseconds"] else "enough"
    if i["freeBytes"]<i["emergencyBytes"]: return "collect-now"
    return "throttled" if age<r["collectorIntervalMilliseconds"] else "launch"
print(json.dumps([{"need":need(i["volumeBytes"],i["freeBytes"]),"decision":decide(i)} for i in v["inputs"]]))
`;

function sqliteDecision(input:typeof fixture.pressureVectors[number]["input"]):{need:number;decision:GcPressureV1}{
 const db=new Database(":memory:");
 try{
  const row=db.query(`
   with i as (select ?1 as volume,?2 as free,?3 as emergency,?4 as now,?5 as last,?6+0.0 as high,?7+0.0 as low,?8 as collector,?9 as maintenance),
   u as (select *,max(0,volume-free) as used,cast(volume*high as integer) as highBytes,cast(volume*low as integer) as lowBytes,case when last is null then null else now-last end as age from i)
   select case when used>highBytes then used-lowBytes else 0 end as need,
          case when used<=highBytes then case when age is null or age>=maintenance then 'maintain' else 'enough' end
               when free<emergency then 'collect-now'
               when age is not null and age<collector then 'throttled' else 'launch' end as decision from u`).get(input.volumeBytes,input.freeBytes,input.emergencyBytes,input.nowMs,input.lastLaunchMs,rules.highWaterShare,rules.lowWaterShare,rules.collectorIntervalMilliseconds,rules.maintenanceIntervalMilliseconds) as {need:number;decision:GcPressureV1};
  return row;
 }finally{db.close();}
}

function seeded(seed:number){let state=seed>>>0;return()=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return state/4294967296;};}

test("the water marks are ordered, leave one compile of room above the high-water mark, and the pressure vectors validate against the schema",()=>{
 expect(rules.lowWaterShare).toBeLessThan(rules.highWaterShare);expect(rules.highWaterShare).toBeLessThan(1);expect(rules.lowWaterShare).toBeGreaterThan(0);
 expect(rules.collectorIntervalMilliseconds).toBeLessThanOrEqual(rules.maintenanceIntervalMilliseconds);
 for(const row of fixture.pressureVectors)expect(validator("PressureVector")(row),row.name).toBe(true);
 for(const row of fixture.outOfSpaceVectors)expect(validator("OutOfSpaceVector")(row)).toBe(true);
});

test("pressure decisions equal the vectors, a Python oracle and a SQLite oracle on seeded random states",()=>{
 const random=seeded(20261022),inputs=fixture.pressureVectors.map(row=>row.input);
 for(let round=0;round<400;round++){
  const volumeBytes=1000+Math.floor(random()*(60*GIB)),freeBytes=Math.floor(random()*volumeBytes*(random()<0.5?0.3:1)),now=2_000_000_000;
  inputs.push({volumeBytes,freeBytes,emergencyBytes:Math.floor(random()*8*GIB),nowMs:now,lastLaunchMs:random()<0.2?null:now-Math.floor(random()*1_500_000)});
 }
 const python=pythonOracle(pythonPressure,{rules,inputs}) as Array<{need:number;decision:GcPressureV1}>;
 inputs.forEach((input,index)=>{
  const decision=gcPressureV1(input),need=gcNeedBytesV1(input.volumeBytes,input.freeBytes);
  expect({need,decision},"python "+index).toEqual(python[index]!);
  expect({need,decision},"sqlite "+index).toEqual(sqliteDecision(input));
  if(index<fixture.pressureVectors.length)expect(decision,fixture.pressureVectors[index]!.name).toBe(fixture.pressureVectors[index]!.decision as GcPressureV1);
 });
});

test("a collection frees exactly down to the low-water mark when enough is collectable, and never when the image is at or below the high-water mark",()=>{
 const volume=1000,lanes=Array.from({length:10},(_,index)=>({id:"l"+index,bytes:20,incrementalBytes:0,lastUsedMs:1000+index,live:false})),objects=Array.from({length:10},(_,index)=>({digest:index.toString(16).padStart(64,"0"),bytes:20,lastUsedMs:1000+index}));
 const input=(free:number)=>({capacityBytes:100000,volumeBytes:volume,freeBytes:free,nowMs:2000,lanes,objects});
 expect(planGcV1(input(200))).toEqual({removeLanes:[],removeIncremental:[],removeObjects:[]});
 for(const free of[199,150,100,50]){
  const plan=planGcV1(input(free)),freed=plan.removeLanes.length*20+plan.removeObjects.length*20,need=gcNeedBytesV1(volume,free);
  expect(freed,"free "+free).toBeGreaterThanOrEqual(need);expect(freed-need,"free "+free).toBeLessThan(40);
 }
});

test("compiler output is classified as out of space by the operating system's error number or message, never by an unrelated error",()=>{
 const python=pythonOracle(`
import json,re,sys
v=json.load(sys.stdin)
print(json.dumps([any(re.search(p,t) for p in v["patterns"]) for t in v["texts"]]))
`,{patterns:rules.outOfSpacePatterns,texts:fixture.outOfSpaceVectors.map(row=>row.text)}) as boolean[];
 fixture.outOfSpaceVectors.forEach((row,index)=>{expect(gcOutOfSpaceV1(row.text),row.text).toBe(row.outOfSpace);expect(python[index],"python "+row.text).toBe(row.outOfSpace);});
 expect(rules.outOfSpaceRetries).toBe(1);
});

test("a second collector finds the claim taken and reports running; a free claim is released after the collection",async()=>{
 const root=scratch();
 try{
  const held=tryClaimV1(join(root,UNIT_CACHE_POLICY_V1.layout.locks),rules.collectorClaim)!;
  expect(await collectGarbageSingleFlightV1(root,1000)).toEqual({running:true});
  held.release();
  const result=await collectGarbageSingleFlightV1(root,1000);expect(result.running).toBe(false);
  const again=tryClaimV1(join(root,UNIT_CACHE_POLICY_V1.layout.locks),rules.collectorClaim);expect(again).toBeDefined();again!.release();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a long collection reports progress, can be cancelled between items, and keeps the items it did not reach",async()=>{
 const root=scratch(),digests=Array.from({length:200},(_,index)=>index.toString(16).padStart(64,"0"));
 try{
  mkdirSync(join(root,UNIT_CACHE_POLICY_V1.layout.objects),{recursive:true});
  const seen:number[]=[],controller=new AbortController();
  await expect(applyGcYieldingV1(root,{removeLanes:[],removeIncremental:[],removeObjects:digests},Date.now(),{signal:controller.signal,progress:(done,total)=>{seen.push(done);expect(total).toBe(Math.ceil(200/64));if(done===2)controller.abort(new Error("cancelled"));}})).rejects.toThrow("cancelled");
  expect(seen).toEqual([1,2]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("pressure relief launches nothing below the high-water mark, one background collector above it, throttles the next one, and collects inline below one compile's headroom",async()=>{
 const root=scratch(),reports:string[]=[],started:string[]=[],volume=48*GIB;
 try{
  const relieve=(free:number,language:"en"|"de",nowMs:number)=>relieveUnitCachePressureV1({root,capacityBytes:44*GIB,emergencyBytes:4*GIB,environment:{},language,report:line=>reports.push(line),nowMs,measure:()=>({volumeBytes:volume,freeBytes:free}),start:options=>{started.push(options.root);writeFileSync(join(options.root,"gc.stamp"),String(nowMs));return{state:"started"};}});
  const now=Date.now();
  expect(await relieve(30*GIB,"en",now)).toBe("maintain");expect(started.length).toBe(1);expect(reports).toEqual([]);
  expect(await relieve(30*GIB,"en",now+1000)).toBe("enough");expect(started.length).toBe(1);
  expect(await relieve(8*GIB,"en",now+2000)).toBe("throttled");expect(started.length).toBe(1);
  expect(await relieve(8*GIB,"de",now+rules.collectorIntervalMilliseconds+2000)).toBe("launch");expect(started.length).toBe(2);
  expect(reports.length).toBe(1);expect(reports[0]).toContain("Hochwassermarke");expect(reports[0]).toContain("83 %");
  expect(await relieve(2*GIB,"en",now+rules.collectorIntervalMilliseconds+3000)).toBe("collect-now");expect(started.length).toBe(2);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the real background collector starts detached, takes the claim, collects and logs its result",async()=>{
 const root=scratch();
 try{
  mkdirSync(join(root,UNIT_CACHE_POLICY_V1.layout.locks),{recursive:true});
  const result=startUnitCacheCollectorV1({root,capacityBytes:1000,environment:process.env});expect(result).toEqual({state:"started"});
  expect(existsSync(join(root,"gc.stamp"))).toBe(true);
  const log=join(root,"gc.log");let text="";
  for(let attempt=0;attempt<200&&!text.includes("\"running\"");attempt++){await new Promise(done=>setTimeout(done,50));try{text=readFileSync(log,"utf8");}catch{}}
  expect(JSON.parse(text.trim().split("\n").pop()!)).toMatchObject({running:false,applied:{lanes:0,incremental:0,objects:0}});
  expect(startUnitCacheCollectorV1({root:join(root,"missing","\0bad"),capacityBytes:1000,environment:{}})).toMatchObject({state:"failed"});
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a command that ran out of space is repeated exactly once after the cleanup, and only then is the image reported as still full",async()=>{
 for(const[name,outcomes,runs,collected,reported]of[["succeeds",[false],1,0,0],["recovers",[true,false],2,1,0],["stays full",[true,true],2,1,1],["fails otherwise",[false],1,0,0]] as const){
  let run=0,collect=0,full=0;
  const last=await retryWhenFullV1(async()=>({full:outcomes[Math.min(run++,outcomes.length-1)]!,run}),{collect:async()=>{collect++;},stillFull:()=>{full++;}});
  expect({name,runs:run,collected:collect,reported:full,full:last.full}).toEqual({name,runs,collected,reported,full:outcomes[outcomes.length-1]});
 }
 let calls=0;await retryWhenFullV1(async()=>({full:++calls>0}),{collect:async()=>{},stillFull:()=>{}},3);expect(calls).toBe(4);
});
