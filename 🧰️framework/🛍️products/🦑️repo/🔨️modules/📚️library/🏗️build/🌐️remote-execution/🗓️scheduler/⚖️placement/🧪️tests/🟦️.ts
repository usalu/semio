import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import {planDispatchV1,placementAnyCandidateV1,placementIdleCountV1,placementSlotsV1,placementIdleElsewhereV1,placementKeyV1,placementRequiredV1,RemoteExecutionLoadTableV1,type PlacementOperationV1,type PlacementWorkerV1,type RemoteExecutionLoadEntryV1} from "../🟦️.ts";
import {remoteExecutionPeakMegabytesV1,remoteExecutionPlatformMatchesV1,type RemoteExecutionPropertyV1} from "../../../🟦️.ts";
import type {RemoteExecutionLoadReportV1} from "../../../🌡️load/🟦️.ts";

const here=import.meta.dir;

function random(seed:number):()=>number{let state=seed>>>0;return ()=>{state=(state*1664525+1013904223)>>>0;return state/4294967296;};}

type Scenario={now:number;workers:PlacementWorkerV1[];queue:{id:string;platform:RemoteExecutionPropertyV1[];tried:string[];requester:string|null}[];table:Record<string,RemoteExecutionLoadEntryV1>};

const TOOLCHAINS=["aaaaaaaaaaaaaaaa","bbbbbbbbbbbbbbbb"],ORIGINS=Array.from({length:9},(_,index)=>`m${index}`);
function report(origin:string,rand:()=>number,toolchain:string):RemoteExecutionLoadReportV1{
 const pressure=(["normal","normal","warn","critical"] as const)[Math.floor(rand()*4)]!,running=Math.floor(rand()*6),own=Math.min(running,Math.floor(rand()*3));
 return {version:1,origin,platform:{key:`macos-arm64-${toolchain}`,os:"macos",arch:"arm64",toolchain,target:"aarch64-apple-darwin"},capacity:{cores:2+Math.floor(rand()*10),memoryMegabytes:16384},running,queued:Math.floor(rand()*4),pressure,freeMegabytes:Math.floor(rand()*16000),load:rand()<0.2?null:Math.round(rand()*20*100)/100,headroom:pressure!=="critical"&&rand()<0.8,wallMs:0,worker:{state:"idle",concurrency:4,running:own,sandbox:"darwin-sandbox",confinement:"full",network:"denied"},counters:{unit:{localHit:0,remoteHit:0,miss:0,stored:0},offload:{out:0,refused:0,fellBack:0}}};
}
function scenario(seed:number):Scenario{
 const rand=random(seed),extra=random(seed*31+7),now=1_000_000,workers:PlacementWorkerV1[]=[],table:Record<string,RemoteExecutionLoadEntryV1>={};
 const count=1+Math.floor(rand()*8);
 for(let index=0;index<count;index++){
  const origin=ORIGINS[index]!,toolchain=TOOLCHAINS[Math.floor(rand()*2)]!,concurrency=1+Math.floor(rand()*4);
  workers.push({id:`w${index}`,name:`name-${index}`,origin,properties:[{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"},{name:"semio-toolchain",value:toolchain},{name:"semio-target",value:"aarch64-apple-darwin"}],concurrency,running:Math.floor(rand()*(concurrency+1)),waiting:rand()<0.75,draining:rand()<0.05,refusedUntilMs:rand()<0.15?now+5000:0,committedMegabytes:extra()<0.3?Math.floor(extra()*9000):0});
  if(rand()<0.85)table[origin]={report:report(origin,rand,toolchain),receivedAtMs:now,offloadedIn:0,offloadedOut:0,refused:0};
 }
 const queue=Array.from({length:Math.floor(rand()*10)},(_,index)=>{
  const platform:RemoteExecutionPropertyV1[]=[];
  if(rand()<0.6)platform.push({name:"OSFamily",value:"macos"});
  if(rand()<0.4)platform.push({name:"semio-toolchain",value:TOOLCHAINS[Math.floor(rand()*2)]!});
  if(rand()<0.3)platform.push({name:"semio-offload",value:"if-idle"});
  if(extra()<0.3)platform.push({name:"semio-peak-mb",value:String(1024+Math.floor(extra()*8000))});
  const requester=rand()<0.6?ORIGINS[Math.floor(rand()*9)]!:null;
  if(requester!==null)platform.push({name:"semio-origin",value:requester});
  return {id:`op${index}`,platform,tried:ORIGINS.filter(()=>rand()<0.1),requester};
 });
 return {now,workers,queue,table};
}
const tableOf=(value:Scenario):Pick<RemoteExecutionLoadTableV1,"get">=>({get:origin=>value.table[origin]});
const operationsOf=(value:Scenario):PlacementOperationV1[]=>value.queue.map(row=>({id:row.id,platform:row.platform,tried:new Set(row.tried),requester:row.requester??undefined,peakMegabytes:remoteExecutionPeakMegabytesV1(row.platform)}));

test("property: a dispatch round never gives an action to a worker without headroom, to an incompatible platform or toolchain build, to a worker that failed it or to its requester, and gives a worker at most one action",()=>{
 for(let seed=1;seed<=2000;seed++){
  const value=scenario(seed),table=tableOf(value),plan=planDispatchV1(operationsOf(value),value.workers,table,value.now),taken=new Set<string>();
  for(const step of plan){
   const worker=value.workers.find(row=>row.id===step.worker)!,operation=value.queue.find(row=>row.id===step.operation)!,entry=value.table[worker.origin];
   expect(worker.waiting,`seed ${seed}`).toBe(true);expect(taken.has(worker.id),`seed ${seed} twice`).toBe(false);taken.add(worker.id);
   expect(worker.draining||worker.running>=worker.concurrency||worker.refusedUntilMs>value.now,`seed ${seed} no slot`).toBe(false);
   if(entry){expect(entry.report.pressure,`seed ${seed} pressure`).not.toBe("critical");expect(entry.report.headroom,`seed ${seed} headroom`).toBe(true);}
   const peak=remoteExecutionPeakMegabytesV1(operation.platform);
   if(peak>0){expect(entry,`seed ${seed} an unreported machine never gets a sized action`).toBeDefined();expect(entry!.report.freeMegabytes-(worker.committedMegabytes??0),`seed ${seed} memory`).toBeGreaterThanOrEqual(peak);}
   const required=placementRequiredV1(operation.platform,operation.requester===null?undefined:value.table[operation.requester]);
   expect(remoteExecutionPlatformMatchesV1(required,worker.properties),`seed ${seed} platform`).toBe(true);
   expect(operation.tried.includes(placementKeyV1(worker)),`seed ${seed} tried`).toBe(false);expect(placementKeyV1(worker)===operation.requester,`seed ${seed} requester`).toBe(false);
   const toolchain=worker.properties.find(row=>row.name==="semio-toolchain")!.value,named=operation.platform.find(row=>row.name==="semio-toolchain")?.value,requested=operation.requester===null?undefined:value.table[operation.requester]?.report.platform.toolchain;
   if(named!==undefined)expect(toolchain,`seed ${seed} named toolchain`).toBe(named);else if(requested)expect(toolchain,`seed ${seed} requester toolchain`).toBe(requested);
  }
 }
});

test("property: a dispatch round is work conserving and first come first served: an action left in the queue has no waiting worker that could take it, and no earlier action that a worker could take is passed over",()=>{
 for(let seed=1;seed<=2000;seed++){
  const value=scenario(seed),table=tableOf(value),operations=operationsOf(value),plan=planDispatchV1(operations,value.workers,table,value.now),assigned=new Set(plan.map(step=>step.operation)),used=new Set(plan.map(step=>step.worker));
  for(const operation of operations){
   if(assigned.has(operation.id))continue;
   const free=value.workers.filter(worker=>worker.waiting&&!used.has(worker.id)),alone=planDispatchV1([operation],free,table,value.now);
   expect(alone,`seed ${seed} op ${operation.id} could still be served`).toEqual([]);
  }
 }
});

test("property: no action starves: with workers finishing at random, every action that a healthy compatible worker exists for is assigned within a bounded number of rounds, in arrival order per worker",()=>{
 for(let seed=1;seed<=300;seed++){
  const rand=random(seed*7919),value=scenario(seed);
  const workers=value.workers.map(worker=>({...worker,draining:false,refusedUntilMs:0,waiting:true,running:0,committedMegabytes:0}));
  for(const origin of Object.keys(value.table))value.table[origin]!.report={...value.table[origin]!.report,pressure:"normal",headroom:true,freeMegabytes:1_000_000};
  const queue=operationsOf(value).map(row=>({...row,tried:new Set<string>()})),servable=queue.filter(operation=>{
   const required=placementRequiredV1(operation.platform,operation.requester===undefined?undefined:value.table[operation.requester]);
   return workers.some(worker=>remoteExecutionPlatformMatchesV1(required,worker.properties)&&placementKeyV1(worker)!==operation.requester&&(remoteExecutionPeakMegabytesV1(operation.platform)===0||value.table[worker.origin]!==undefined));
  });
  const pending=[...queue],order:string[]=[],busy=new Map<string,number>();
  for(let round=0;round<200&&pending.length>0;round++){
   const view=workers.map(worker=>({...worker,running:busy.get(worker.id)??0,waiting:(busy.get(worker.id)??0)<worker.concurrency}));
   const plan=planDispatchV1(pending,view,tableOf(value),value.now);
   for(const step of plan){busy.set(step.worker,(busy.get(step.worker)??0)+1);order.push(step.operation);pending.splice(pending.findIndex(row=>row.id===step.operation),1);}
   for(const [id,count] of busy)if(count>0&&rand()<0.7)busy.set(id,count-1);
  }
  for(const operation of servable)expect(order.includes(operation.id),`seed ${seed} ${operation.id} starved`).toBe(true);
 }
});

test("the least loaded compatible worker is chosen, a worker without a report ranks last, and idle-elsewhere is false when only the requester or workers without headroom remain",()=>{
 const now=1000,load=(origin:string,running:number,cores:number,extra:Partial<RemoteExecutionLoadReportV1>={}):RemoteExecutionLoadEntryV1=>({report:{...report(origin,random(1),"aaaaaaaaaaaaaaaa"),running,queued:0,capacity:{cores,memoryMegabytes:16384},pressure:"normal",headroom:true,load:null,worker:{state:"idle",concurrency:4,running:0,sandbox:"darwin-sandbox",confinement:"full",network:"denied"},...extra},receivedAtMs:now,offloadedIn:0,offloadedOut:0,refused:0});
 const properties=[{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"},{name:"semio-toolchain",value:"aaaaaaaaaaaaaaaa"},{name:"semio-target",value:"aarch64-apple-darwin"}];
 const worker=(origin:string,over:Partial<PlacementWorkerV1>={}):PlacementWorkerV1=>({id:origin,name:origin,origin,properties,concurrency:4,running:0,waiting:true,draining:false,refusedUntilMs:0,...over});
 const table={a:load("a",4,8),b:load("b",1,8),c:load("c",0,8,{pressure:"critical"})} as Record<string,RemoteExecutionLoadEntryV1>,view={get:(origin:string)=>table[origin]};
 const operation:PlacementOperationV1={id:"x",platform:[],tried:new Set(),requester:undefined};
 expect(planDispatchV1([operation],[worker("a"),worker("b"),worker("c"),worker("d")],view,now)).toEqual([{operation:"x",worker:"b"}]);
 expect(planDispatchV1([operation],[worker("d")],view,now)).toEqual([{operation:"x",worker:"d"}]);
 expect(placementIdleElsewhereV1({...operation,requester:"b"},[worker("b")],view,now)).toBe(false);
 expect(placementIdleElsewhereV1({...operation,requester:"b"},[worker("b"),worker("c")],view,now)).toBe(false);
 expect(placementIdleElsewhereV1({...operation,requester:"b"},[worker("b"),worker("c"),worker("a",{running:4})],view,now)).toBe(false);
 expect(placementIdleElsewhereV1({...operation,requester:"b"},[worker("b"),worker("c"),worker("a")],view,now)).toBe(true);
 expect(placementIdleElsewhereV1({...operation,tried:new Set(["a"]),requester:"b"},[worker("b"),worker("a")],view,now)).toBe(false);
 const other=worker("a",{properties:properties.map(row=>row.name==="semio-toolchain"?{name:row.name,value:"bbbbbbbbbbbbbbbb"}:row)});
 expect(placementIdleElsewhereV1({...operation,requester:"b"},[worker("b"),other],view,now),"another toolchain build never takes a unit of the requester").toBe(false);
});

test("an action with a declared peak goes only to a machine whose reported free memory, less what the scheduler already placed on it, covers the peak; a machine that has not reported never gets one",()=>{
 const now=1000,properties=[{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"},{name:"semio-toolchain",value:"aaaaaaaaaaaaaaaa"},{name:"semio-target",value:"aarch64-apple-darwin"}];
 const load=(origin:string,freeMegabytes:number):RemoteExecutionLoadEntryV1=>({report:{...report(origin,random(1),"aaaaaaaaaaaaaaaa"),running:0,queued:0,capacity:{cores:8,memoryMegabytes:32768},pressure:"normal",headroom:true,load:null,freeMegabytes,worker:{state:"idle",concurrency:4,running:0,sandbox:"darwin-sandbox",confinement:"full",network:"denied"}},receivedAtMs:now,offloadedIn:0,offloadedOut:0,refused:0});
 const worker=(origin:string,over:Partial<PlacementWorkerV1>={}):PlacementWorkerV1=>({id:origin,name:origin,origin,properties,concurrency:4,running:0,waiting:true,draining:false,refusedUntilMs:0,...over});
 const table={small:load("small",3000),full:load("full",9000),big:load("big",20000)} as Record<string,RemoteExecutionLoadEntryV1>,view={get:(origin:string)=>table[origin]};
 const operation=(peakMegabytes:number):PlacementOperationV1=>({id:"final",platform:[{name:"semio-peak-mb",value:String(peakMegabytes)}],tried:new Set(),requester:undefined,peakMegabytes});
 const workers=[worker("small"),worker("full",{committedMegabytes:5000}),worker("big"),worker("silent")];
 expect(planDispatchV1([operation(4813)],workers,view,now),"3000 free is too little, 9000 less 5000 placed is too little, a silent machine is unknown").toEqual([{operation:"final",worker:"big"}]);
 expect(planDispatchV1([operation(4813)],[workers[0]!,workers[1]!,workers[3]!],view,now)).toEqual([]);
 expect(planDispatchV1([operation(2500)],[workers[0]!,workers[3]!],view,now),"a smaller peak fits the small machine").toEqual([{operation:"final",worker:"small"}]);
 expect(planDispatchV1([operation(0)],[workers[3]!],view,now),"an action without a peak keeps the old rule: a silent machine has headroom").toEqual([{operation:"final",worker:"silent"}]);
 expect(placementIdleElsewhereV1(operation(4813),[workers[0]!,workers[3]!],view,now)).toBe(false);
 expect(placementIdleElsewhereV1(operation(4813),workers,view,now)).toBe(true);
 expect(remoteExecutionPeakMegabytesV1([{name:"semio-peak-mb",value:"4813"}]),"the declared peak").toBe(4813);
 for(const bad of ["","x","-5","1.5","12345678901"])expect(remoteExecutionPeakMegabytesV1([{name:"semio-peak-mb",value:bad}]),bad).toBe(0);
});

test("the load table keeps the last report per machine, expires silent machines after the bound and keeps their counters for their return",()=>{
 let clock=0;const table=new RemoteExecutionLoadTableV1(15_000,()=>clock),first=report("m1",random(3),"aaaaaaaaaaaaaaaa");
 table.accept(first);table.accept(report("m2",random(4),"aaaaaaaaaaaaaaaa"));table.count("m1","offloadedIn");table.count("m1","offloadedIn");
 expect(table.all().map(row=>[row.report.origin,row.offloadedIn])).toEqual([["m1",2],["m2",0]]);
 clock=10_000;table.accept(first);clock=20_000;
 expect(table.all().map(row=>row.report.origin)).toEqual(["m1"]);clock=40_000;expect(table.all()).toEqual([]);
 table.accept(first);expect(table.get("m1")?.offloadedIn).toBe(2);
});

test("an independent python implementation of the policy makes the same decisions on 400 random scenarios",()=>{
 const scenarios=Array.from({length:400},(_,index)=>scenario(10_000+index)),input=JSON.stringify(scenarios.map(value=>({now:value.now,workers:value.workers,queue:value.queue,table:value.table})));
 const run=spawnSync("python3",["-I",join(here,"🐍️oracle","🐍️.py")],{input,encoding:"utf8",maxBuffer:1<<26});
 expect(run.status,run.stderr).toBe(0);
 const expected=JSON.parse(run.stdout) as {operation:string;worker:string}[][];
 scenarios.forEach((value,index)=>{expect(planDispatchV1(operationsOf(value),value.workers,tableOf(value),value.now),`scenario ${index}`).toEqual(expected[index]!);});
});

test("a Windows worker is a normal worker: units of its own platform key reach it, units of a macOS requester find no compatible worker as an ordinary placement result",()=>{
 const windows=(origin:string,id:string):PlacementWorkerV1=>({id,name:origin,origin,properties:[{name:"Arch",value:"x86_64"},{name:"OSFamily",value:"windows"},{name:"semio-toolchain",value:"cccccccccccccccc"},{name:"semio-target",value:"x86_64-pc-windows-msvc"}],concurrency:2,running:0,waiting:true,draining:false,refusedUntilMs:0});
 const none={get:()=>undefined},workers=[windows("g1-m05","w1")];
 const macos:PlacementOperationV1={id:"a",platform:[{name:"OSFamily",value:"macos"},{name:"semio-toolchain",value:"aaaaaaaaaaaaaaaa"}],tried:new Set(),requester:"g1-m01"};
 const same:PlacementOperationV1={id:"b",platform:[{name:"OSFamily",value:"windows"},{name:"semio-toolchain",value:"cccccccccccccccc"}],tried:new Set(),requester:"g1-m06"};
 expect(planDispatchV1([macos],workers,none,1_000_000)).toEqual([]);
 expect(placementIdleElsewhereV1(macos,workers,none,1_000_000)).toBe(false);
 expect(placementAnyCandidateV1(macos,workers,none)).toBe(false);
 expect(planDispatchV1([same],workers,none,1_000_000)).toEqual([{operation:"b",worker:"w1"}]);
 expect(placementIdleElsewhereV1(same,workers,none,1_000_000)).toBe(true);
 expect(planDispatchV1([{...same,requester:"g1-m05"}],workers,none,1_000_000),"never its own requester").toEqual([]);
});

test("the idle count reports the free action slots of the candidates, the width a build sizes its jobs to: never the requester, a full worker or a draining one",()=>{
 const none={get:()=>undefined},worker=(origin:string,id:string,concurrency:number,running:number,extra:Partial<PlacementWorkerV1>={}):PlacementWorkerV1=>({id,name:origin,origin,properties:[{name:"OSFamily",value:"macos"}],concurrency,running,waiting:true,draining:false,refusedUntilMs:0,...extra});
 const operation:PlacementOperationV1={id:"p",platform:[{name:"OSFamily",value:"macos"}],tried:new Set(),requester:"r"};
 const workers=[worker("r","w0",4,0),worker("a","w1",4,1),worker("b","w2",2,2),worker("c","w3",3,0),worker("d","w4",8,0,{draining:true})];
 expect(placementIdleCountV1(operation,workers,none,1_000_000)).toEqual({workers:5,compatible:4,headroom:3,candidates:2,slots:6});
 expect(placementIdleCountV1(operation,[],none,1_000_000)).toEqual({workers:0,compatible:0,headroom:0,candidates:0,slots:0});
});

test("the idle count of a sized action counts whole multiples of its peak the free memory of each machine covers, so a 16 GB worker takes one or two wasm finals, never more than its free action slots and never less than one",()=>{
 const worker=(origin:string,id:string,concurrency:number,running:number,extra:Partial<PlacementWorkerV1>={}):PlacementWorkerV1=>({id,name:origin,origin,properties:[{name:"OSFamily",value:"macos"}],concurrency,running,waiting:true,draining:false,refusedUntilMs:0,...extra});
 const entry=(origin:string,freeMegabytes:number):RemoteExecutionLoadEntryV1=>({report:{...report(origin,random(3),TOOLCHAINS[0]!),origin,pressure:"normal",headroom:true,freeMegabytes},receivedAtMs:1_000_000,offloadedIn:0,offloadedOut:0,refused:0});
 const table:Record<string,RemoteExecutionLoadEntryV1>={a:entry("a",7000),b:entry("b",13000),c:entry("c",30000),d:entry("d",3000)},get={get:(origin:string)=>table[origin]};
 const workers=[worker("a","w1",4,0),worker("b","w2",4,0),worker("c","w3",2,0),worker("d","w4",4,0),worker("e","w5",3,0)];
 const operation:PlacementOperationV1={id:"p",platform:[{name:"OSFamily",value:"macos"}],tried:new Set(),requester:"r"};
 expect(placementIdleCountV1(operation,workers,get,1_000_000).slots,"no peak: free action slots").toBe(4+4+2+4+3);
 const sized=placementIdleCountV1({...operation,peakMegabytes:6144},workers,get,1_000_000);
 expect(sized.candidates,"d has 3000 MB, e never reported: neither takes a sized action").toBe(3);
 expect(sized.slots).toBe(1+2+2);
 expect(placementSlotsV1(workers[0]!,table.a,6144)).toBe(1);expect(placementSlotsV1(workers[1]!,table.b,6144)).toBe(2);expect(placementSlotsV1(workers[2]!,table.c,6144),"bounded by its two action slots").toBe(2);
 expect(placementSlotsV1(workers[0]!,table.a,0)).toBe(4);expect(placementSlotsV1(workers[4]!,undefined,6144)).toBe(3);
 expect(placementSlotsV1({...workers[1]!,committedMegabytes:7000},table.b,6144),"what the scheduler placed already is not free").toBe(1);
});
