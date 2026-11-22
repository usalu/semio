/** 🧮️ Contract: a graph plan books the memory of its slice worker in the machine-wide gate before the worker starts and holds it until it is released, so a build that does not fit beside it queues behind it in the fair queue, starts when the plan ends, and a plan that waits can be cancelled without leaving a waiter behind. */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {Database} from "bun:sqlite";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {newHostCapacityIdV1,readHostCapacityPolicyV1,withHostCapacityV1,type HostCapacityWaitV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {graphPlanObserverV1,reserveGraphPlanCapacityV1} from "../../🟦️.ts";
import {createSlicePoolV1} from "../../../🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import custody from "./🧫️fixtures/🔣️custody.json" with {type:"json"};

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(realpathSync(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"plan-capacity-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=(over:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:30000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10,...over});
const fast=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const sample=async()=>({host:host(),swappedOutMegabytes:0});
const grants=(directory:string)=>existsSync(join(directory,"grants"))?readdirSync(join(directory,"grants")).filter(name=>name.endsWith(".json")):[];
const queue=(directory:string)=>existsSync(join(directory,"queue"))?readdirSync(join(directory,"queue")):[];
const until=async(test:()=>boolean,milliseconds=3000)=>{const started=Date.now();while(!test()&&Date.now()-started<milliseconds)await new Promise(done=>setTimeout(done,10));return test();};
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const build=(directory:string,megabytes:number,body:()=>Promise<void>)=>withHostCapacityV1({directory,policy:fast,request:{id:newHostCapacityIdV1(),operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1},control:control(),environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample},body);

test("with the gate off a plan reserves nothing and is never refused",async()=>{
 const directory=fresh(),reservation=await reserveGraphPlanCapacityV1({megabytes:6000,environment:{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY:"off"},directory,policy:fast,write:()=>undefined});
 reservation.attach(process.pid);reservation.release();reservation.release();expect(grants(directory)).toEqual([]);
},60000);

test("a plan books its megabytes, a build that does not fit beside it queues behind it and starts when the plan is released, and the booking records the worker it observes",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_DIRECTORY:directory},order:string[]=[];
 const reservation=await reserveGraphPlanCapacityV1({megabytes:20000,environment,directory,policy:fast,write:()=>undefined,sample});
 expect(grants(directory).length).toBe(1);
 const grant=JSON.parse(readFileSync(join(directory,"grants",grants(directory)[0]!),"utf8"));
 expect(grant.label).toBe("graph-plan");expect(grant.reservedMegabytes).toBe(20000);
 reservation.attach(process.pid);
 const queued=build(directory,20000,async()=>{order.push("build");});
 expect(await until(()=>queue(directory).length===1)).toBe(true);await new Promise(done=>setTimeout(done,200));expect(order).toEqual([]);
 order.push("release");reservation.release();await queued;
 expect(order).toEqual(["release","build"]);expect(await until(()=>grants(directory).length===0)).toBe(true);
},60000);

test("a build that fits beside the plan runs at once",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={LANG:"en_US.UTF-8"},reservation=await reserveGraphPlanCapacityV1({megabytes:6000,environment,directory,policy:fast,write:()=>undefined,sample});
 let ran=false;await build(directory,6000,async()=>{ran=true;expect(grants(directory).length).toBe(2);});expect(ran).toBe(true);reservation.release();
 expect(await until(()=>grants(directory).length===0)).toBe(true);
},60000);

test("a plan that waits for memory is cancelled by its signal, rejects with the reason and leaves no waiter behind",async()=>{
 const directory=fresh(),controller=new AbortController(),environment:NodeJS.ProcessEnv={LANG:"en_US.UTF-8"};
 const waiting=reserveGraphPlanCapacityV1({megabytes:6000,environment,directory,policy:fast,write:()=>undefined,signal:controller.signal,sample:async()=>({host:host({availableMegabytes:300,pressure:"critical"}),swappedOutMegabytes:0})}).then(()=>undefined,error=>error);
 expect(await until(()=>queue(directory).length===1)).toBe(true);controller.abort(new Error("plan cancelled"));
 expect((await waiting).message).toBe("plan cancelled");expect(queue(directory)).toEqual([]);expect(grants(directory)).toEqual([]);
},60000);


const deferred=<T>()=>{let resolve!:(value:T)=>void;const promise=new Promise<T>(done=>{resolve=done;});return {promise,resolve};};
const inertReaper={reap:async()=>0,killNow:()=>undefined};

test("a running cancelled slice keeps a conflicting native booking waiting until worker cleanup",async()=>{
 const directory=fresh(),controller=new AbortController(),started=deferred<void>(),terminating=deferred<void>(),reaped=deferred<void>(),db=new Database(":memory:");
 const reason=Error(custody.cancelled),environment={LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_DIRECTORY:directory};
 const fits=db.query("select ?1 + ?2 <= ?3 as possible").get(custody.planMegabytes,custody.competingMegabytes,custody.hostMegabytes) as {possible:number};
 const pool=createSlicePoolV1({
  workerUrl:new URL("file:///unused.mjs"),maximum:1,hostMegabytes:custody.hostMegabytes,unlearnedMegabytes:custody.planMegabytes,reaper:inertReaper,
  reserve:({megabytes,minimumMegabytes,signal}:Parameters<typeof reserveGraphPlanCapacityV1>[0])=>reserveGraphPlanCapacityV1({megabytes,minimumMegabytes,signal,environment,directory,policy:fast,write:()=>undefined,sample}),
  createWorker:()=>{queueMicrotask(()=>started.resolve());return{pid:process.pid,on:()=>undefined,terminate:async()=>{terminating.resolve();await reaped.promise;return 0;}};},
 });
 let competitor:Promise<void>|undefined,admitted=false;
 const run=pool.run({root:directory},{signal:controller.signal}).catch(error=>error);
 try{
  await started.promise;
  expect(grants(directory)).toHaveLength(1);
  competitor=build(directory,custody.competingMegabytes,async()=>{admitted=true;});
  expect(await until(()=>queue(directory).length===1)).toBe(true);
  controller.abort(reason);
  await terminating.promise;
  expect(await until(()=>admitted,custody.observeWhileReapingMilliseconds)).toBe(Boolean(fits.possible));
  expect(grants(directory)).toHaveLength(1);
  reaped.resolve();
  expect(await run).toBe(reason);
  await competitor;
  expect(admitted).toBe(true);
 }finally{
  controller.abort(reason);reaped.resolve();
  await run;await competitor;await pool.close();db.close();
 }
},60000);

test("cancellation during limiter creation releases both allocations before any worker starts",async()=>{
 const opened=deferred<void>(),limitReady=deferred<void>(),controller=new AbortController(),reason=Error(custody.cancelled);
 let starts=0,releases=0,limitReleases=0;
 const pool=createSlicePoolV1({
  workerUrl:new URL("file:///unused.mjs"),maximum:1,hostMegabytes:custody.hostMegabytes,unlearnedMegabytes:custody.planMegabytes,reaper:inertReaper,
  headroom:()=>({limitBytes:custody.hostMegabytes*1048576,currentBytes:0}),
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>{releases++;}}),
  limiter:{create:async()=>{opened.resolve();await limitReady.promise;return{attach:async()=>undefined,onBreach:()=>undefined,release:async()=>{limitReleases++;}};}},
  createWorker:()=>{starts++;return{pid:process.pid,on:(event:string,fn:(value:unknown)=>void)=>{if(event==="message")queueMicrotask(()=>fn({type:"done",graph:{}}));},terminate:async()=>0};},
 });
 const run=pool.run({root:"/tmp/plan-start-cancel"},{signal:controller.signal}).catch(error=>error);
 try{
  await opened.promise;
  controller.abort(reason);limitReady.resolve();
  expect(await run).toBe(reason);
  await pool.close();
  expect({starts,releases,limitReleases}).toEqual({starts:0,releases:1,limitReleases:1});
 }finally{controller.abort(reason);limitReady.resolve();await run;await pool.close();}
},60000);

test("a synchronous slice worker startup failure releases the untransferred allocations",async()=>{
 const reason=Error(custody.spawnFailure);
 let releases=0,limitReleases=0;
 const pool=createSlicePoolV1({
  workerUrl:new URL("file:///unused.mjs"),maximum:1,hostMegabytes:custody.hostMegabytes,unlearnedMegabytes:custody.planMegabytes,reaper:inertReaper,
  headroom:()=>({limitBytes:custody.hostMegabytes*1048576,currentBytes:0}),
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>{releases++;}}),
  limiter:{create:async()=>({attach:async()=>undefined,onBreach:()=>undefined,release:async()=>{limitReleases++;}})},
  createWorker:()=>{throw reason;},
 });
 try{
  await expect(pool.run({root:"/tmp/plan-start-error"})).rejects.toBe(reason);
  expect({releases,limitReleases}).toEqual({releases:1,limitReleases:1});
 }finally{await pool.close();}
},60000);

test("the actual graph reservation gate reports fair queue ordering even when a plan fits",async()=>{
 const directory=fresh(),messages:string[]=[],views:HostCapacityWaitV1[]=[],begin=new AbortController(),tail=new AbortController(),environment={LANG:"en_US.UTF-8"};
 const policy=readHostCapacityPolicyV1({...fast,wait:{...fast.wait,bypassMilliseconds:40}});
 const held=await reserveGraphPlanCapacityV1({megabytes:20000,environment,directory,policy,write:()=>undefined,sample});
 const front=reserveGraphPlanCapacityV1({megabytes:20000,environment,directory,policy,write:line=>messages.push(line),sample,signal:begin.signal}).then(grant=>{grant.release();},error=>error);
 try{
  expect(await until(()=>messages.filter(line=>line.includes("not enough free memory")).length>=4)).toBe(true);
  const last=reserveGraphPlanCapacityV1({megabytes:3000,minimumMegabytes:2048,environment,directory,policy,write:()=>undefined,sample,signal:tail.signal,onWait:view=>views.push(view)}).then(grant=>{grant.release();},error=>error);
  try{
   expect(await until(()=>views.length>0)).toBe(true);
   expect(views[0]).toMatchObject({reason:"queue-order",active:1,queued:2,minimumMegabytes:2048});
   expect(views[0]!.spareMegabytes).toBeGreaterThanOrEqual(3000);
  }finally{tail.abort(Error("cancel queued plan"));expect((await last)?.message).toBe("cancel queued plan");}
 }finally{begin.abort(Error("cancel first plan"));await front;held.release();}
 expect(await until(()=>queue(directory).length===0&&grants(directory).length===0)).toBe(true);
},60000);

test("the actual graph reservation gate supplies the measured memory shortage",async()=>{
 const directory=fresh(),controller=new AbortController(),environment={LANG:"en_US.UTF-8"},views:HostCapacityWaitV1[]=[];
 const program=reserveGraphPlanCapacityV1({megabytes:6000,minimumMegabytes:2048,environment,directory,policy:fast,write:()=>undefined,signal:controller.signal,onWait:view=>views.push(view),sample:async()=>({host:host({availableMegabytes:2500}),swappedOutMegabytes:0})}).then(grant=>{grant.release();},error=>error);
 try{
  expect(await until(()=>views.length>0)).toBe(true);
  expect(views[0]).toMatchObject({reason:"memory",active:0,queued:1,minimumMegabytes:2048});
  expect(views[0]!.spareMegabytes).toBeLessThan(2048);
 }finally{controller.abort(Error("cancel memory wait"));expect((await program)?.message).toBe("cancel memory wait");}
 expect(queue(directory)).toEqual([]);
},60000);

const table:readonly (readonly [number,number,number])[]=[[process.pid,1,300],[7001,process.pid,4000],[7002,7001,1000],[7003,7002,700],[7004,1,900]];
const oracle=(root:number)=>{const database=new Database(":memory:");database.run("create table process(pid integer primary key,parent integer,megabytes integer)");for(const row of table)database.run("insert into process values (?,?,?)",[...row]);
 const total=database.query("with recursive tree(pid) as (select ? union select process.pid from process join tree on process.parent=tree.pid) select coalesce(sum(megabytes),0) as total from process where pid in (select pid from tree)").get(root) as {total:number};database.close();return total.total;};
const tree=async(pid:number)=>oracle(pid);

test("the observer of a plan counts the attached worker tree once and nothing for the service that holds the grant",async()=>{
 let worker:number|undefined;const observe=graphPlanObserverV1(()=>worker,tree);
 expect(await observe(process.pid)).toBe(0);worker=7001;
 expect(await observe(7001)).toBe(5700);expect(await observe(process.pid)).toBe(0);expect(oracle(process.pid)).toBe(6000);expect(oracle(7001)).toBe(5700);
});

test("the booking of a plan shows the memory of its worker tree once, not the worker twice (the service is its parent), so no build waits behind a shortage that is not there",async()=>{
 const directory=fresh(),beating=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0,heartbeatMilliseconds:20}});
 const reservation=await reserveGraphPlanCapacityV1({megabytes:8000,environment:{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_DIRECTORY:directory},directory,policy:beating,write:()=>undefined,sample,observe:tree});
 const read=()=>{const name=grants(directory)[0];return name===undefined?undefined:JSON.parse(readFileSync(join(directory,"grants",name),"utf8")).observedMegabytes as number|undefined;};
 await new Promise(done=>setTimeout(done,200));expect(read()??0).toBe(0);
 reservation.attach(7001);
 expect(await until(()=>(read()??0)>0)).toBe(true);
 for(let beat=0;beat<5;beat++){await new Promise(done=>setTimeout(done,30));expect(read()).toBe(oracle(7001));}
 reservation.release();expect(await until(()=>grants(directory).length===0)).toBe(true);
},60000);

test("on a 16 GB machine the graph is computed with a grant of at least its minimum and at most what it asked, whatever else the host holds, and the minimum is the one the policy derives for the rebuild",async()=>{
 const minimum=buildPolicy.hostCapacity.estimates.rebuildMinimumMegabytes,graph=buildPolicy.hostCapacity.operations.find(row=>row.name==="graph")!;
 expect(minimum,"the rebuild minimum is the least the graph operation itself runs with").toBe(graph.minimumMegabytes);
 const small=(available:number)=>async()=>({host:host({totalMegabytes:16384,availableMegabytes:available}),swappedOutMegabytes:0});
 for(const available of [2048+minimum!,2048+4000,2048+9000]){
  const directory=fresh(),reservation=await reserveGraphPlanCapacityV1({megabytes:7168,minimumMegabytes:minimum,environment:{LANG:"en_US.UTF-8"},directory,policy:fast,write:()=>undefined,sample:small(available)});
  expect(reservation.megabytes,String(available)).toBe(Math.min(7168,available-2048));
  expect(reservation.megabytes).toBeGreaterThanOrEqual(minimum!);reservation.release();
  expect(await until(()=>grants(directory).length===0)).toBe(true);
 }
 const refused=await reserveGraphPlanCapacityV1({megabytes:7168,minimumMegabytes:minimum,environment:{LANG:"en_US.UTF-8"},directory:fresh(),policy:fast,write:()=>undefined,sample:async()=>({host:host({totalMegabytes:3072,availableMegabytes:3000}),swappedOutMegabytes:0})}).then(()=>"",error=>(error as Error).message);
 expect(refused,"a machine that can never give the minimum is refused at once, naming the numbers").toContain("3072");
},60000);
