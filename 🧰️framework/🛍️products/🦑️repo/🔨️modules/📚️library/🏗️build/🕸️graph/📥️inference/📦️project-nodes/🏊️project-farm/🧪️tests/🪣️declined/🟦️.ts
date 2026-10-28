/** 🪣️ Contract: once the graph cache declines the leaves a worker produced (its optional quota is full: gate run g46 declined 142 of 142 finished projects), the farm sends no further project to a worker and closes its pools at once, so no project is inferred twice (by a worker, then again by the parent) and the workers' memory and grants are free for the rest of the construction. Oracle: the jobs the scripted workers received and the processes that were told to shut down. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {EventEmitter} from "node:events";
import {openProjectFarmV1} from "../../🟨️.mjs";
import {inferenceRecordIdentityV1} from "../../../../../../👁️observation/🟨️.mjs";
import {graphFarmIdentityV1} from "../../🪆️captured/🟨️.mjs";
import {GRAPH_FARM_POLICY_V1} from "../../🏊️pool/🟨️.mjs";

/** 🧑‍🏭️ A scripted worker of this implementation that answers every job with one staged leaf and records what it was sent. */
class Worker extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;runs=0;shutdowns=0;
 send(message:{type:string;id?:number}){
  if(message.type==="init")setImmediate(()=>this.emit("message",{type:"ready",pid:1,identity:graphFarmIdentityV1(inferenceRecordIdentityV1())}));
  else if(message.type==="run"){this.runs++;setImmediate(()=>this.emit("message",{type:"result",id:message.id,reply:{names:["inference/np/000000000000000000000000.json"],cpuMilliseconds:1},rssMegabytes:1}));}
  else if(message.type==="shutdown"){this.shutdowns++;this.exitCode=0;setImmediate(()=>this.emit("exit"));}
  return true;
 }
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}

test("after the first declined adoption no project reaches a worker and the pools are closed before the construction ends",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"graph-farm-declined-")),workers:Worker[]=[],lines:string[]=[],adoptions:string[]=[],released:string[]=[];
 try{
  const farm=openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_PLAN:JSON.stringify({version:1,id:"x",jobs:1,cpu:1,memoryMegabytes:2048,fixedMegabytes:0})},processed:()=>24,cores:10,write:line=>lines.push(line),
   spawnProcess:(()=>{const worker=new Worker();workers.push(worker);return worker;}) as never,
   request:(()=>({granted:Promise.resolve(undefined),release:()=>{released.push("host");}})) as never,
   adopt:((_root:string,_staging:string,names:readonly string[])=>{adoptions.push(names.join(","));return {published:0,present:0,declined:true};}) as never})!;
  const job=(name:string)=>({name,root:name,json:{},native:false,pipeline:false,nativeRoot:name,selectedInputs:[],head:[],cacheKey:"0".repeat(24)});
  for(let index=0;index<GRAPH_FARM_POLICY_V1.minimumProjects;index++)await farm.prepare(job("miss"+index));
  await new Promise<number>(done=>farm.onReady(done));
  expect(await farm.prepare(job("first"))).toBe(false);
  for(let index=0;index<5;index++)expect(await farm.prepare(job("later"+index))).toBe(false);
  expect(workers.reduce((sum,worker)=>sum+worker.runs,0)).toBe(1);
  expect(adoptions).toEqual(["inference/np/000000000000000000000000.json"]);
  expect(lines.some(line=>line.includes("declined the leaves of project first"))).toBe(true);
  expect(lines.some(line=>line.includes("no more worker processes"))).toBe(true);
  for(let wait=0;wait<100&&workers.some(worker=>worker.exitCode===null);wait++)await Bun.sleep(10);
  expect(workers.every(worker=>worker.exitCode!==null)).toBe(true);
  await farm.close();
 }finally{rmSync(root,{recursive:true,force:true});}
});
