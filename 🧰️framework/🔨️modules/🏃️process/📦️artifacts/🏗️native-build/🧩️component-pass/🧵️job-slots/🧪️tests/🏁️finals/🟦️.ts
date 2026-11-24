import {afterAll,expect,test}from"bun:test";
import {mkdtempSync,rmSync}from"node:fs";
import {tmpdir}from"node:os";
import {join}from"node:path";
import {awaitFleetFitV1,offloadGateV1,readUnitExpenseV1,writeUnitCostV1,writeUnitPeakV1}from"../../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts";
import{OFFLOAD_POLICY_V1}from"../../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/📐️policy/🟦️.ts";
import{planComponentJobSlotsV1,planComponentJobsV1,runPassesInJobSlotsV1}from"../../🟦️.ts";

const roots:string[]=[],fresh=():string=>{const root=mkdtempSync(join(tmpdir(),"semio-finals-jobs-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const pause=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));

class Semaphore{
 held=0;maximum=0;waiting:(()=>void)[]=[];
 constructor(readonly size:number){}
 async take():Promise<void>{if(this.held<this.size){this.held++;this.maximum=Math.max(this.maximum,this.held);return;}await new Promise<void>(resolve=>this.waiting.push(resolve));this.held++;this.maximum=Math.max(this.maximum,this.held);}
 give():void{this.held--;this.waiting.shift()?.();}
}

test("N finals run on workers while the local jobs run and the cheap units stay here: the finals-first gate, the fleet job plan and the local job plan compose without double counting or starvation",async()=>{
 const workers=3,peak=7938,spare=11000,root=fresh(),environment={SEMIO_WORKSPACE_ROOT:fresh(),SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://192.0.2.1:8980"};
 const remote=new Semaphore(workers),localFinals=new Semaphore(2),localSlots=new Semaphore(4);
 let remoteRunning=0,askedByCheap=0,fleetCalls=0,clock=0,remoteDuringCheap=0,cheapLocal=0,remoteFinals=0,localFinalsDone=0;
 const slotsAtPeak=()=>workers===0?0:Math.max(0,Math.floor(spare/peak)*workers-remote.held);
 const fleet=(async()=>{fleetCalls++;return new Response(JSON.stringify({slots:slotsAtPeak(),candidates:workers}),{status:200});})as typeof fetch;
 for(let index=0;index<6;index++){writeUnitPeakV1(root,`semio_hub_${index}_component`,6000);writeUnitCostV1(root,`semio_hub_${index}_component`,900000);}
 writeUnitCostV1(root,"semio_cheap_unit",800);
 const expense=(crate:string)=>readUnitExpenseV1(root,crate,crate.endsWith("_component")?["cdylib"]:["lib"],crate.endsWith("_component")?"wasm32-wasip2":undefined);
 const unit=async(crate:string):Promise<"remote"|"local">=>{
  const spend=expense(crate);
  const gate=await offloadGateV1({mode:"distribute",door:false,expensive:()=>spend.expense.expensive,
   workerFits:waitMilliseconds=>{if(!spend.expense.expensive)askedByCheap++;return awaitFleetFitV1(environment,{fleet:{fetch:fleet},peakMegabytes:spend.peak,waitMilliseconds,pollMilliseconds:OFFLOAD_POLICY_V1.finals.pollMilliseconds,now:()=>clock,sleep:async milliseconds=>{clock+=milliseconds;await pause(2);}});},
   localSlot:async()=>localSlots.held<localSlots.size,worth:()=>spend.expense.milliseconds!==undefined&&spend.expense.milliseconds<OFFLOAD_POLICY_V1.limits.roundTripMilliseconds?"cheap":"worth",slotQueueMilliseconds:()=>0,host:async()=>"admit",broker:async()=>"admit"});
  if(gate.offload){
   await remote.take();remoteRunning++;await pause(40);remoteRunning--;remote.give();remoteFinals++;return"remote";
  }
  if(spend.expense.expensive){await localFinals.take();await pause(60);localFinals.give();localFinalsDone++;return"local";}
  await localSlots.take();cheapLocal++;if(remoteRunning>0)remoteDuringCheap++;await pause(5);localSlots.give();return"local";
 };
 const plan=planComponentJobsV1({local:1,fleetSlots:slotsAtPeak(),availableMegabytes:20000,pressure:"normal",unitMegabytes:1024});
 expect(plan.remote).toBe(workers);
 const slots=planComponentJobSlotsV1({fleetJobs:plan.jobs,tokens:4,wanted:3});
 expect(slots.light,"the light jobs are the worker slots less the first job: counted once").toBe(workers);
 expect(slots.jobs).toBe(slots.light+slots.local);
 const passes=Array.from({length:8},(_,index)=>({id:"hub_"+(index%6),target:"wasm32-wasip2"}));
 let running=0,maximumRunning=0,local=0,maximumLocal=0,reservedMaximum=0,reserved=0;
 await runPassesInJobSlotsV1(passes,{jobs:slots.jobs,check:()=>undefined,
  local:{maximum:slots.local,floorMegabytes:4000,peak:()=>peak,sample:async()=>({availableMegabytes:20000,ownedMegabytes:0,pressure:"normal"}),pollMilliseconds:5,report:(_pass,event)=>{if(event.kind==="started"){reserved+=peak;reservedMaximum=Math.max(reservedMaximum,reserved);}}},
  run:async(pass,slot)=>{
   running++;maximumRunning=Math.max(maximumRunning,running);if(slot==="local"){local++;maximumLocal=Math.max(maximumLocal,local);}
   for(let index=0;index<3;index++)expect(await unit("semio_cheap_unit")).toBe("local");
   await unit(`semio_${pass.id}_component`);
   running--;if(slot==="local")local--;
  }});
 expect(maximumRunning,"never more concurrent jobs than planned").toBeLessThanOrEqual(slots.jobs);
 expect(maximumLocal,"never more local jobs than the local plan").toBeLessThanOrEqual(slots.local);
 expect(remote.maximum,"several finals at once on workers, never more than the workers carry").toBe(workers);
 expect(remoteFinals+localFinalsDone,"every final ran: nothing starved").toBe(passes.length);
 expect(remoteFinals,"the workers took the finals first").toBeGreaterThanOrEqual(workers);
 expect(localFinals.maximum,"the fallen back finals never exceed the local grant").toBeLessThanOrEqual(localFinals.size);
 expect(askedByCheap,"a cheap unit never asks a worker").toBe(0);
 expect(cheapLocal,"every cheap unit compiled here").toBe(passes.length*3);
 expect(remoteDuringCheap,"the local jobs kept compiling while finals ran on workers").toBeGreaterThan(0);
 expect(fleetCalls,"only the finals asked the scheduler").toBeGreaterThanOrEqual(passes.length);
 expect(reservedMaximum,"only the extra local jobs reserved memory here, none for a light job").toBeLessThanOrEqual(Math.max(0,slots.local-1)*peak);
},60000);

test("with every worker busy the finals wait their short wait and compile here under the local grant, with no worker at all they do not wait",async()=>{
 const root=fresh(),environment={SEMIO_WORKSPACE_ROOT:fresh(),SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://192.0.2.1:8980"};
 writeUnitCostV1(root,"semio_hub_x_component",900000);
 const spend=readUnitExpenseV1(root,"semio_hub_x_component",["cdylib"],"wasm32-wasip2");
 let clock=0,calls=0;
 const busy=(async()=>{calls++;return new Response(JSON.stringify({slots:0,candidates:3}),{status:200});})as typeof fetch;
 const gate=await offloadGateV1({mode:"distribute",door:false,expensive:()=>spend.expense.expensive,workerFits:waitMilliseconds=>awaitFleetFitV1(environment,{fleet:{fetch:busy},peakMegabytes:spend.peak,waitMilliseconds,pollMilliseconds:OFFLOAD_POLICY_V1.finals.pollMilliseconds,now:()=>clock,sleep:async milliseconds=>{clock+=milliseconds;}}),localSlot:async()=>true,worth:()=>"worth",slotQueueMilliseconds:()=>0,host:async()=>"admit",broker:async()=>"admit"});
 expect(gate).toEqual({offload:false,queue:false,reason:"final-wait"});
 expect(clock).toBeGreaterThanOrEqual(OFFLOAD_POLICY_V1.finals.waitMilliseconds-OFFLOAD_POLICY_V1.finals.pollMilliseconds);
 expect(calls).toBeGreaterThan(10);
 let none=0;
 const absent=await awaitFleetFitV1({SEMIO_WORKSPACE_ROOT:fresh()},{waitMilliseconds:30000,pollMilliseconds:2000,sleep:async()=>{none++;}});
 expect(absent).toBe(false);expect(none).toBe(0);
});
