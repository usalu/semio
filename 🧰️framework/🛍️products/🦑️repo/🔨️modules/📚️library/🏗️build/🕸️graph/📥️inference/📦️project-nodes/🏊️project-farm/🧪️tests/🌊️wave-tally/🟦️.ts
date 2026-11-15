/** 🌊️ Contract: a farm tells the wave meter what happened to the projects of its wave when it closes, also when no worker ever started, so a wave row shows whether the parent or the workers inferred the projects. */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{graphWaveMeterV1}from"../../../../../📈️wave-row/🟨️.mjs";
import{EventEmitter}from"node:events";
import{closeProjectFarmsV1,openProjectFarmV1}from"../../🟨️.mjs";
import{inferenceRecordIdentityV1}from"../../../../../../👁️observation/🟨️.mjs";
import{graphFarmIdentityV1}from"../../🪆️captured/🟨️.mjs";
import{farmGrantEnvironmentV1}from"../🧫️fixtures/🎟️grant/🟨️.mjs";

/** 💀️ A scripted worker that is ready at once and dies on the first job it is given. */
class Dying extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 constructor(){super();setImmediate(()=>this.emit("message",{type:"ready",pid:1,identity:graphFarmIdentityV1(inferenceRecordIdentityV1())}));}
 send(message:{type:string}){
  if(message.type==="run")setImmediate(()=>{this.exitCode=1;this.emit("exit");});
  else if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}
  return true;
 }
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}

const roots:string[]=[],scratch=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"farm-tally-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

test("a farm that never got a worker reports its wave to the meter as handled by the parent",async()=>{
 graphWaveMeterV1.take({generation:0});
 const root=scratch();
 let grant!:(value:{jobs:number;megabytes:number}|undefined)=>void;
 const farm=openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8"},processed:()=>24,cores:10,write:()=>undefined,
  spawnProcess:(()=>{throw new Error("no worker must start");}) as never,
  request:(()=>({granted:new Promise<{jobs:number;megabytes:number}|undefined>(done=>{grant=done;}),release:()=>undefined})) as never})!;
 const job={name:"p",root:"p",json:{},native:false,pipeline:false,nativeRoot:"p",selectedInputs:[],head:[],cacheKey:"0".repeat(24)};
 for(let index=0;index<24;index++)await farm.prepare(job);
 grant({jobs:0,megabytes:2048});
 await Bun.sleep(20);
 await farm.close();
 expect(graphWaveMeterV1.take({generation:1}).farm).toEqual({workers:0,projects:566,adopted:0,failed:0,lost:0,declined:0});
});

test("the close of the farm reports the lost jobs and not the lost processes, also after workers ran",async()=>{
 graphWaveMeterV1.take({generation:0});
 const root=scratch(),children:Dying[]=[];
 const farm=openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:farmGrantEnvironmentV1(root,"farm_tally_0001"),processed:()=>400,cores:10,write:()=>undefined,engage:true,
  spawnProcess:(()=>{const child=new Dying();children.push(child);return child;}) as never,
  request:(()=>({granted:Promise.resolve(undefined),release:()=>undefined})) as never})!;
 const job={name:"p",root:"p",json:{},native:false,pipeline:false,nativeRoot:"p",selectedInputs:[],head:[],cacheKey:"0".repeat(24)};
 expect(await farm.prepare(job),"a job every process dies on is left to the parent").toBe(false);
 await farm.close();await closeProjectFarmsV1();
 expect(children.length,"the job was tried on two processes").toBe(2);
 expect(graphWaveMeterV1.take({generation:1}).farm).toEqual({workers:1,projects:566,adopted:0,failed:0,lost:1,declined:0});
});
