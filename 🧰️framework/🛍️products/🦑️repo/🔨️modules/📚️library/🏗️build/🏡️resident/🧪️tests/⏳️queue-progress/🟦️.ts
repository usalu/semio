/** ⏳️ Contract (loads of other slices of one checkout run side by side, bound only by the worker maximum; loads of the same slice are one load, see `../🤝️slice-share`): a slice load that waits for a free worker is not silent: it reports a queue decision on every heartbeat, in English and German, starts when a worker is free, and a queued load whose client left never starts a worker.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {shrinkBoundedCachesV1} from "../../../🧊️bounded/🟨️.mjs";
import checkout from "./🔖️checkout.json" with {type:"json"};
import checkoutSchema from "./🧬️schema/🔖️checkout.json" with {type:"json"};
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {HOST_POLICY_V1,renderTextV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number};
const harness=(maximum:number)=>{
 const started:string[]=[],finish=new Map<string,()=>void>();
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum,heartbeatMilliseconds:20,graceMilliseconds:50,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,
  reaper:{reap:async()=>0,killNow:()=>undefined},
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
  createWorker:(_url:URL,data)=>{const name=String(data.name);const listeners:Record<string,(value:unknown)=>void>={};started.push(name);finish.set(name,()=>listeners.message?.({type:"done",graph:{}}));return{pid:undefined,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0};}});
 return{pool,started,finish};
};
const until=async(check:()=>boolean)=>{for(let wait=0;!check()&&wait<200;wait++)await new Promise(done=>setTimeout(done,10));expect(check()).toBe(true);};

test("loads of different slices of one checkout run side by side up to the worker maximum, the next one waits for a worker",async()=>{
 const{pool,started,finish}=harness(2),queue:any[]=[];
 const first=pool.run({root:"/tmp/queue",name:"first",arguments:["exec","--projects=p1"]}),second=pool.run({root:"/tmp/queue",name:"second",arguments:["exec","--projects=p2"]}),third=pool.run({root:"/tmp/queue",name:"third",arguments:["exec","--projects=p3"]},{onProgress:(_ms:number,detail:any)=>{if(detail?.phase==="queue")queue.push(detail.decision);}});
 await until(()=>started.length===2&&queue.length>=2);
 expect(started).toEqual(["first","second"]);expect(queue.every(decision=>decision.key==="queued-pool"&&decision.used===2&&decision.budget===2&&decision.limit===1)).toBe(true);
 finish.get("first")!();await first;await until(()=>started.length===3);expect(started).toEqual(["first","second","third"]);
 finish.get("second")!();finish.get("third")!();await Promise.all([second,third]);await pool.close();
});

test("a load of another checkout waits for a free worker and says so in both languages",async()=>{
 const{pool,started,finish}=harness(1),queue:any[]=[];
 const first=pool.run({root:"/tmp/one",name:"one"}),second=pool.run({root:"/tmp/two",name:"two"},{onProgress:(_ms:number,detail:any)=>{if(detail?.phase==="queue")queue.push(detail.decision);}});
 await until(()=>queue.length>=2);
 expect(started).toEqual(["one"]);expect(queue.every(decision=>decision.key==="queued-pool"&&decision.used===1&&decision.budget===1)).toBe(true);
 finish.get("one")!();await first;await until(()=>started.length===2);finish.get("two")!();await second;await pool.close();
 const text=renderTextV1("host-slice-owners-queued-pool",{seconds:9,phase:"queue",done:0,total:0,owners:"",used:1,budget:1,kind:"",limit:0},["en","de"])!;
 for(const language of["en","de"] as const)expect(text[language]).not.toMatch(/\{[a-z]+\}/u);
 expect(text.en).toContain("free slice worker");expect(text.de).toContain("freien Slice-Worker");
});

test("a queued load whose client left starts nothing and a later load takes the freed worker",async()=>{
 const{pool,started,finish}=harness(1),controller=new AbortController();
 const first=pool.run({root:"/tmp/leave",name:"first",arguments:["exec","--projects=p1"]}),leaving=pool.run({root:"/tmp/leave",name:"leaving",arguments:["exec","--projects=p2"]},{signal:controller.signal}),third=pool.run({root:"/tmp/leave",name:"third",arguments:["exec","--projects=p3"]});
 await until(()=>started.length===1);controller.abort(new Error("gone"));
 const error:any=await leaving.then(()=>undefined,reason=>reason);expect(error.message).toBe("gone");
 await new Promise(done=>setTimeout(done,60));expect(started).toEqual(["first"]);
 finish.get("first")!();await first;await until(()=>started.length===2);expect(started).toEqual(["first","third"]);
 finish.get("third")!();await third;await pool.close();
});

for(const pressure of ["cache trimming","unrelated checkout churn"])test(pressure+" leaves live checkout loads side by side, bound by the worker maximum",async()=>{
 expect(new Ajv({strict:true}).validate(checkoutSchema,checkout)).toBe(true);
 const {pool,started,finish}=harness(checkout.maximumWorkers),controller=new AbortController(),pending:Promise<unknown>[]=[],db=new Database(":memory:");
 const [leader,follower,independent]=checkout.requests;
 const start=(request:typeof leader)=>{
  const result=pool.run(request,{signal:controller.signal});
  pending.push(result);
  void result.catch(()=>undefined);
  return result;
 };
 try{
  const first=start(leader!);
  await until(()=>started.length===1);
  if(pressure==="cache trimming")shrinkBoundedCachesV1(0);
  else{
   for(let index=0;index<checkout.unrelatedLoads;index++){
    const name="transient-"+index,visit=start({...independent!,root:independent!.root+"/"+index,name});
    await until(()=>finish.has(name));
    finish.get(name)!();await visit;
   }
  }
  const baseline=started.length,second=start(follower!),other=start(independent!),running=db.query<{count:number},[number,number]>("select min(?1,?2) count").get(checkout.maximumWorkers,2)!.count;
  await until(()=>started.length===baseline+running-1);
  await new Promise(done=>setTimeout(done,60));
  expect(started.slice(baseline)).toEqual([follower!.name]);
  finish.get(leader!.name)!();await first;
  await until(()=>started.includes(independent!.name));
  finish.get(follower!.name)!();finish.get(independent!.name)!();
  await Promise.all([second,other]);
  expect(pool.busy()).toBe(0);
 }finally{controller.abort(new Error("checkout test ended"));await Promise.allSettled(pending);await pool.close();db.close();}
});
