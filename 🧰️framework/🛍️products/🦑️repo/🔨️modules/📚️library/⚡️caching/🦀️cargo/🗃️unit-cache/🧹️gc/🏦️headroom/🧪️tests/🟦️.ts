import {expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json";
import componentSchema from "../🧬️schema/🔣️.json";
import {UNIT_CACHE_POLICY_V1} from "../../../🔏️key/🟦️.ts";
import {planGcV1,type GcInputV1} from "../../🟦️.ts";
import {HEADROOM_POLICY_V1 as policy,admitImageHeadroomV1,decideHeadroomV1,headroomEstimateV1,headroomFloorBytesV1,headroomOutstandingV1,headroomQuantileV1,headroomTextV1,readHeadroomHistoryV1,readHeadroomLedgerV1,recordHeadroomSampleV1,type HeadroomHistoryV1,type HeadroomInputV1,type HeadroomRowV1} from "../🟦️.ts";

const GIB=1024**3,volume=48*GIB,layout=UNIT_CACHE_POLICY_V1.layout;
const ajv=new Ajv({strict:true});ajv.addSchema(componentSchema,"headroom.json");
const python=(rows:readonly unknown[]):unknown[]=>{const run=Bun.spawnSync(["python3","-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"../🧬️schema/🔣️.json")],{stdin:new TextEncoder().encode(JSON.stringify(rows))});expect(run.exitCode,new TextDecoder().decode(run.stderr)).toBe(0);return JSON.parse(new TextDecoder().decode(run.stdout));};
const scratch=():string=>mkdtempSync(join(realpathSync(tmpdir()),"headroom-"));
const noSleep=(milliseconds:number,signal?:AbortSignal):Promise<void>=>new Promise((done,fail)=>{if(signal?.aborted){fail(signal.reason);return;}setTimeout(done,Math.min(milliseconds,4));});

test("the fixture is closed and every vector equals the independent Python oracle",()=>{
 const validate=ajv.compile({$ref:"headroom.json#/definitions/Vectors"});expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 for(const vector of fixture.floors)expect(headroomFloorBytesV1(vector.volumeBytes),vector.name).toBe(vector.expected);
 for(const vector of fixture.quantiles)expect(headroomQuantileV1(vector.values,vector.basisPoints),vector.name).toBe(vector.expected);
 for(const vector of fixture.estimates)expect(headroomEstimateV1(vector.input.history as HeadroomHistoryV1,vector.input.classes,vector.input.prior,vector.input.volumeBytes) as unknown,vector.name).toEqual(vector.expected);
 for(const vector of fixture.outstanding)expect(headroomOutstandingV1(vector.input.row as HeadroomRowV1,vector.input.nowMs),vector.name).toBe(vector.expected);
 for(const vector of fixture.decisions)expect(decideHeadroomV1(vector.input as HeadroomInputV1) as unknown,vector.name).toEqual(vector.expected);
 expect(python(fixture.floors.map(vector=>({kind:"floor",input:vector.volumeBytes})))).toEqual(fixture.floors.map(vector=>vector.expected));
 expect(python(fixture.quantiles.map(vector=>({kind:"quantile",input:{values:vector.values,basisPoints:vector.basisPoints}})))).toEqual(fixture.quantiles.map(vector=>vector.expected));
 expect(python(fixture.estimates.map(vector=>({kind:"estimate",input:vector.input})))).toEqual(fixture.estimates.map(vector=>vector.expected));
 expect(python(fixture.outstanding.map(vector=>({kind:"outstanding",input:vector.input})))).toEqual(fixture.outstanding.map(vector=>vector.expected));
 expect(python(fixture.decisions.map(vector=>({kind:"decision",input:vector.input})))).toEqual(fixture.decisions.map(vector=>vector.expected));
});

test("seeded random ledgers, histories and images give the same decisions, estimates and holds in TypeScript and in the Python oracle, and a fitting verdict never exceeds the free space",()=>{
 let seed=20261029;const next=(limit:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%limit;};
 const decisions:HeadroomInputV1[]=[],estimates:{volumeBytes:number;classes:string[];prior:{bytes:number;rampMilliseconds:number};history:HeadroomHistoryV1}[]=[],holds:{row:HeadroomRowV1;nowMs:number}[]=[];
 for(let index=0;index<400;index++){
  const rows:HeadroomRowV1[]=Array.from({length:next(7)},(_,at)=>{const waiting=next(3)===0,reserved=next(40)*GIB/4+next(1000),ramp=30000+next(1700000),admitted=1_000_000-next(2_000_000);return{id:"r"+at,state:waiting?"waiting":"running",reservedBytes:reserved,rampMilliseconds:ramp,arrivedMs:next(10)*500,admittedMs:waiting?null:Math.max(0,admitted)};});
  decisions.push({freeBytes:next(48*4)*GIB/4+next(5),volumeBytes:[volume,16*GIB,160*GIB,1000][next(4)]!,nowMs:1_000_000,request:{id:"q"+next(3),reservedBytes:next(30)*GIB/4,arrivedMs:next(10)*500},rows,reclaimed:next(2)===1});
  for(const row of rows)holds.push({row,nowMs:next(3_000_000)});
  const sample=()=>({bytes:next(30)*GIB/4+next(1000),wallMs:next(2_000_000),atMs:next(1000)}),classes:Record<string,{bytes:number;wallMs:number;atMs:number}[]>={};
  for(const name of ["a#cold","b#cold","c#cold"])if(next(3))classes[name]=Array.from({length:next(40)},sample);
  estimates.push({volumeBytes:[volume,16*GIB,1000][next(3)]!,classes:["a#cold","b#cold","c#cold"],prior:{bytes:next(40)*GIB/4,rampMilliseconds:1+next(3_000_000)},history:{version:1,classes}});
 }
 const expectedDecisions=python(decisions.map(input=>({kind:"decision",input})));
 decisions.forEach((input,at)=>{const decision=decideHeadroomV1(input);expect(decision as unknown,JSON.stringify(input).slice(0,200)).toEqual(expectedDecisions[at]);if(decision.reason==="fits")expect(input.request.reservedBytes+decision.floorBytes+decision.heldBytes+decision.aheadBytes).toBeLessThanOrEqual(input.freeBytes);});
 const expectedEstimates=python(estimates.map(input=>({kind:"estimate",input})));
 estimates.forEach((input,at)=>expect(headroomEstimateV1(input.history,input.classes,input.prior,input.volumeBytes) as unknown,JSON.stringify(input).slice(0,200)).toEqual(expectedEstimates[at]));
 const expectedHolds=python(holds.map(input=>({kind:"outstanding",input})));
 holds.forEach((input,at)=>{const held=headroomOutstandingV1(input.row,input.nowMs);expect(held).toBe(expectedHolds[at] as number);expect(held).toBeLessThanOrEqual(input.row.reservedBytes);});
});

test("a command is admitted at once when the image has room, holds a record while it runs, and learns what it wrote so that the next command of its class reserves that",async()=>{
 const root=scratch();
 try{
  const reports:string[]=[],free=20*GIB,measure=()=>({volumeBytes:volume,freeBytes:free}),base={root,capacityBytes:volume,classes:["cargo/check","cargo/check/host","cargo/check/host/abc"],temperature:"cold" as const,label:"abc",language:"en" as const,report:(line:string)=>{reports.push(line);},measure,sleep:noSleep};
  const first=await admitImageHeadroomV1(base);
  expect(first.reservedBytes).toBe(policy.prior.cold.bytes);expect(reports).toEqual([]);
  const ledger=readHeadroomLedgerV1(root);expect(ledger.map(row=>[row.id,row.state,row.reservedBytes])).toEqual([[first.id,"running",policy.prior.cold.bytes]]);
  await first.release(()=>2*GIB);expect(readHeadroomLedgerV1(root)).toEqual([]);
  const second=await admitImageHeadroomV1(base);await second.release(()=>2*GIB);
  const third=await admitImageHeadroomV1(base);
  const learned=headroomEstimateV1(readHeadroomHistoryV1(join(root,layout.headroom)),base.classes.map(name=>name+"#cold"),policy.prior.cold,volume);
  expect(learned.samples).toBe(2);expect(third.reservedBytes).toBe(Math.floor(2*GIB*(10000+policy.estimate.marginBasisPoints)/10000));expect(third.reservedBytes).toBe(learned.reservedBytes);
  await third.release();
  const warm=await admitImageHeadroomV1({...base,temperature:"warm"});expect(warm.reservedBytes).toBe(policy.prior.warm.bytes);await warm.release();
  const history=readHeadroomHistoryV1(join(root,layout.headroom));expect(Object.keys(history.classes).sort()).toEqual(base.classes.map(name=>name+"#cold").sort());expect(history.classes["cargo/check#cold"]!.map(row=>row.bytes)).toEqual([2*GIB,2*GIB]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a command that does not fit waits for the running one, is told so in both languages, and starts the moment the reservation ends",async()=>{
 const root=scratch();
 try{
  const prior=policy.prior.cold.bytes,floor=headroomFloorBytesV1(volume),free=floor+prior+prior/2;const reports:string[]=[],measure=()=>({volumeBytes:volume,freeBytes:free}),reclaims:number[]=[];
  const base={root,capacityBytes:volume,classes:["cargo/check"],temperature:"cold" as const,language:"en" as const,report:(line:string)=>{reports.push(line);},measure,sleep:noSleep,reclaim:async(demand:number)=>{reclaims.push(demand);return true;}};
  const running=await admitImageHeadroomV1({...base,label:"first",reclaim:async()=>false});
  expect(running.reservedBytes).toBe(prior);
  let admitted=false;const waiter=admitImageHeadroomV1({...base,label:"second"}).then(lease=>{admitted=true;return lease;});
  await Bun.sleep(80);expect(admitted).toBe(false);
  const rows=readHeadroomLedgerV1(root);expect(rows.map(row=>row.state).sort()).toEqual(["running","waiting"]);
  expect(reports[0]).toContain("waiting 0s for compiler image headroom");expect(reports[0]).toContain((prior/GIB).toFixed(1)+" GiB");expect(headroomTextV1("waiting",{seconds:1,need:1,free:1,held:1,running:1,ahead:1,floor:1},"de")).toContain("warte seit 1s");
  expect(reclaims.length).toBeGreaterThan(0);expect(reclaims[0]!).toBeLessThanOrEqual(prior/2);expect(reclaims[0]!).toBeGreaterThan(prior/2-8*1024*1024);
  await running.release();
  const lease=await waiter;expect(admitted).toBe(true);expect(reports.some(line=>line.includes("admitted after"))).toBe(true);
  expect(readHeadroomLedgerV1(root).map(row=>row.state)).toEqual(["running"]);await lease.release();expect(readdirSync(join(root,layout.headroom)).filter(name=>name.endsWith(".json"))).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("waiting commands are served in arrival order and a cancellation withdraws a waiting command without touching the others",async()=>{
 const root=scratch();
 try{
  const prior=policy.prior.cold.bytes,free=headroomFloorBytesV1(volume)+prior*3/2,measure=()=>({volumeBytes:volume,freeBytes:free}),reports:string[]=[];
  let clock=1_000_000;const now=()=>clock++;
  const base={root,capacityBytes:volume,classes:["cargo/check"],temperature:"cold" as const,language:"en" as const,report:(line:string)=>{reports.push(line);},measure,sleep:noSleep,reclaim:async()=>false,now};
  const holder=await admitImageHeadroomV1({...base,label:"holder",id:"holder"});
  const order:string[]=[];
  const first=admitImageHeadroomV1({...base,label:"b",id:"bb"}).then(lease=>{order.push("bb");return lease;});await Bun.sleep(30);
  const controller=new AbortController(),second=admitImageHeadroomV1({...base,label:"c",id:"cc",signal:controller.signal}).then(lease=>{order.push("cc");return lease;},error=>{order.push("cancelled");throw error;});await Bun.sleep(30);
  const third=admitImageHeadroomV1({...base,label:"d",id:"dd"}).then(lease=>{order.push("dd");return lease;});await Bun.sleep(30);
  expect(readHeadroomLedgerV1(root).filter(row=>row.state==="waiting").map(row=>row.id).sort()).toEqual(["bb","cc","dd"]);
  controller.abort(new Error("cancelled by the caller"));await expect(second).rejects.toThrow("cancelled by the caller");
  expect(existsSync(join(root,layout.headroom,"cc.json"))).toBe(false);expect(readHeadroomLedgerV1(root).filter(row=>row.state==="waiting").map(row=>row.id).sort()).toEqual(["bb","dd"]);
  await holder.release();const firstLease=await first;expect(order).toEqual(["cancelled","bb"]);
  await firstLease.release();const thirdLease=await third;expect(order).toEqual(["cancelled","bb","dd"]);await thirdLease.release();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the only command left goes on after one completed cleanup found nothing more, and says so",async()=>{
 const root=scratch();
 try{
  const reports:string[]=[],demands:number[]=[];
  const lease=await admitImageHeadroomV1({root,capacityBytes:volume,classes:["cargo/check"],temperature:"cold",label:"alone",language:"de",report:line=>{reports.push(line);},measure:()=>({volumeBytes:volume,freeBytes:1*GIB}),sleep:noSleep,reclaim:async(demand)=>{demands.push(demand);return true;}});
  expect(demands).toEqual([policy.prior.cold.bytes+headroomFloorBytesV1(volume)-1*GIB]);expect(reports.some(line=>line.includes("kein anderer Befehl läuft"))).toBe(true);await lease.release();
  const refused=admitImageHeadroomV1({root,capacityBytes:volume,classes:["cargo/check"],temperature:"cold",label:"blocked",language:"en",report:()=>{},measure:()=>({volumeBytes:volume,freeBytes:1*GIB}),sleep:noSleep,reclaim:async()=>false,signal:AbortSignal.timeout(150)});
  expect(await refused.then(()=>"admitted",()=>"refused")).toBe("refused");expect(readHeadroomLedgerV1(root)).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a silent record, a record of a dead process and an unmeasurable image never hold anyone back",async()=>{
 const root=scratch(),directory=join(root,layout.headroom);mkdirSync(directory,{recursive:true});
 try{
  const record=(id:string,pid:number)=>JSON.stringify({version:1,pid,label:id,id,state:"running",reservedBytes:40*GIB,rampMilliseconds:1800000,arrivedMs:1,admittedMs:Date.now()});
  writeFileSync(join(directory,"silent.json"),record("silent",process.pid));const old=new Date(Date.now()-policy.staleMilliseconds-5000);utimesSync(join(directory,"silent.json"),old,old);
  const dead=Bun.spawnSync(["true"]).pid;writeFileSync(join(directory,"dead.json"),record("dead",dead));
  writeFileSync(join(directory,"broken.json"),"{");writeFileSync(join(directory,"live.json"),record("live",process.pid));
  expect(readHeadroomLedgerV1(root).map(row=>row.id)).toEqual(["live"]);
  const ancient=new Date(Date.now()-policy.abandonedMilliseconds-5000);utimesSync(join(directory,"silent.json"),ancient,ancient);readHeadroomLedgerV1(root);expect(existsSync(join(directory,"silent.json"))).toBe(false);
  rmSync(join(directory,"live.json"));
  const lease=await admitImageHeadroomV1({root,capacityBytes:volume,classes:["x"],temperature:"cold",label:"x",language:"en",report:()=>{},measure:()=>({volumeBytes:volume,freeBytes:volume,unmeasured:true}),sleep:noSleep});
  expect(lease.id).toBe("-");expect(lease.reservedBytes).toBe(0);await lease.release(()=>1);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("history keeps the newest samples per class, drops the least recently measured classes beyond the maximum, and a damaged file is an empty history",()=>{
 const root=scratch(),directory=join(root,layout.headroom);mkdirSync(directory,{recursive:true});
 try{
  expect(readHeadroomHistoryV1(directory).classes).toEqual({});
  writeFileSync(join(directory,policy.historyFile),"nonsense");expect(readHeadroomHistoryV1(directory).classes).toEqual({});
  writeFileSync(join(directory,policy.historyFile),JSON.stringify({version:1,classes:{good:[{bytes:1,wallMs:2,atMs:3},{bytes:"x"},null],bad:"no"}}));expect(readHeadroomHistoryV1(directory).classes).toEqual({good:[{bytes:1,wallMs:2,atMs:3}]});
  let history:HeadroomHistoryV1={version:1,classes:{}};
  for(let index=0;index<policy.estimate.maximumSamples+5;index++)history=recordHeadroomSampleV1(history,["a","b"],{bytes:index,wallMs:index,atMs:index});
  expect(history.classes.a!.length).toBe(policy.estimate.maximumSamples);expect(history.classes.a![0]!.bytes).toBe(5);expect(history.classes.b!.at(-1)!.bytes).toBe(policy.estimate.maximumSamples+4);
  const names=Array.from({length:policy.estimate.maximumClasses+3},(_,index)=>"class"+String(index).padStart(4,"0"));let wide:HeadroomHistoryV1={version:1,classes:{}};
  names.forEach((name,index)=>{wide=recordHeadroomSampleV1(wide,[name],{bytes:1,wallMs:1,atMs:100+index});});
  expect(Object.keys(wide.classes).length).toBe(policy.estimate.maximumClasses);expect(wide.classes[names[0]!]).toBeUndefined();expect(wide.classes[names.at(-1)!]).toBeDefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a running command frees on demand within the monitor interval once the image falls below the floor, and stops watching when it ends",async()=>{
 const root=scratch();
 try{
  let free=20*GIB;const demands:number[]=[];
  const lease=await admitImageHeadroomV1({root,capacityBytes:volume,classes:["cargo/check"],temperature:"cold",label:"monitored",language:"en",report:()=>{},measure:()=>({volumeBytes:volume,freeBytes:free}),sleep:noSleep,reclaim:async(demand)=>{demands.push(demand);return true;}});
  await Bun.sleep(policy.monitorMilliseconds+400);expect(demands).toEqual([]);
  free=1*GIB;await Bun.sleep(policy.monitorMilliseconds+400);expect(demands.length).toBeGreaterThan(0);expect(demands[0]).toBe(2*headroomFloorBytesV1(volume)-1*GIB);
  await lease.release();const seen=demands.length;await Bun.sleep(policy.monitorMilliseconds+400);expect(demands.length).toBe(seen);
 }finally{rmSync(root,{recursive:true,force:true});}
},30000);

test("a demand frees the oldest idle lanes and cold objects beyond the water marks, never a live lane and never the protected newest objects",()=>{
 const lane=(id:string,bytes:number,lastUsedMs:number,live=false)=>({id,bytes,incrementalBytes:0,lastUsedMs,live}),relaxed={laneIdleMilliseconds:10**9,laneShare:1,incrementalShare:1,casShare:1};
 const input:GcInputV1={capacityBytes:100,volumeBytes:100,freeBytes:60,nowMs:1_000_000,lanes:[lane("p1",10,100_000),lane("p2",10,200_000),lane("p3",10,300_000),lane("live",30,50_000,true),lane("p4",10,900_000)],objects:[{digest:"d1",bytes:5,lastUsedMs:1},{digest:"d2",bytes:5,lastUsedMs:2}]};
 expect(planGcV1(input,relaxed)).toEqual({removeLanes:[],removeIncremental:[],removeObjects:[]});
 expect(planGcV1({...input,demandBytes:25},relaxed)).toEqual({removeLanes:["p1","p2","p3"],removeIncremental:[],removeObjects:[]});
 expect(planGcV1({...input,demandBytes:70},{...relaxed,casProtectedShare:0})).toEqual({removeLanes:["p1","p2","p3","p4"],removeIncremental:[],removeObjects:["d1","d2"]});
 expect(planGcV1({...input,demandBytes:70},{...relaxed,casProtectedShare:0.05}).removeObjects).toEqual(["d1"]);
 expect(planGcV1({...input,demandBytes:70},{...relaxed,casProtectedShare:0.1}).removeObjects).toEqual([]);
 expect(planGcV1({...input,demandBytes:10**6},relaxed).removeLanes).not.toContain("live");
});
