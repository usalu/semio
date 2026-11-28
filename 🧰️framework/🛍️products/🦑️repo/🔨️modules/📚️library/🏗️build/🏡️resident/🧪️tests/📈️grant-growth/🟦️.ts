/** 📈️ Contract: a slice worker whose process tree passes the memory it was granted is not killed and started again in a loop: the pool books the missing memory beside the grant, holding the worker stopped while the gate answers, and resumes it with the larger grant; a load that was cut `cutsBeforeReadmit` times asks the gate for its measured peak instead of the reduced share it was first admitted with; a worker killed from outside names the grant it had.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs
 * @see ../../🧬️schema/🔣️.json `sliceGrantGrowth` */
import {expect,test} from "bun:test";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number;leanBasisPoints:number},guard=HOST_POLICY_V1.slicePlanGuard as {reserveMegabytes:number;ceilingBasisPoints:number;settleWaitMilliseconds:number},growth=HOST_POLICY_V1.sliceGrantGrowth as {standingBasisPoints:number;minimumMegabytes:number;headroomBasisPoints:number;triggerBasisPoints:number;maximum:number;cutsBeforeReadmit:number},MiB=1048576;
const wait=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
type Fake={emit:(event:string,...values:unknown[])=>void;env:Record<string,string|undefined>;calls:string[]};
type Ask={signal?:AbortSignal;megabytes:number;minimumMegabytes?:number;floorMegabytes?:number;queuedSinceMs?:number};

const harness=(options:{hostMegabytes:number;footprint:(index:number)=>number;grant?:number;extra?:(ask:Ask,index:number)=>Promise<void>;refuseFull?:boolean;growth?:Record<string,number>;learned?:{read(scope?:string):number;write(megabytes:number,scope?:string):void}}) => {
 const workers:Fake[]=[],asks:Ask[]=[],released:number[]=[],claimed:{pid:number;grantMegabytes?:number;stopped:boolean;owner?:string}[]=[],freed:number[]=[];
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,sampleMilliseconds:10,heartbeatMilliseconds:30,hostMegabytes:options.hostMegabytes,elasticity,unlearnedMegabytes:2048,
  planGuard:{...guard,settleWaitMilliseconds:20},hostPressure:()=>"normal",report:()=>undefined,...options.learned===undefined?{}:{learned:options.learned},...options.growth===undefined?{}:{growth:options.growth},claims:{claim:(pid:number,facts:{grantMegabytes?:number;stopped:boolean;owner?:string})=>{claimed.push({pid,...facts});},release:(pid:number)=>{freed.push(pid);}},
  headroom:()=>({limitBytes:Infinity,currentBytes:0,availableBytes:8000*MiB}),
  reserve:async(ask:Ask)=>{asks.push(ask);const index=asks.length;if(options.refuseFull&&index===1)throw new Error("the minimum never fits");return{megabytes:index===1&&options.grant!==undefined?options.grant:ask.megabytes,attach:()=>undefined,grow:async(more:Ask)=>{asks.push(more);await options.extra?.(more,asks.length);return more.megabytes;},release:()=>{released.push(index);}};},
  observe:async()=>options.footprint(workers.length),
  createWorker:(_url:unknown,_data:unknown,env:Record<string,string|undefined>)=>{const listeners:Record<string,(...values:unknown[])=>void>={},calls:string[]=[],fake:Fake={emit:(event,...values)=>listeners[event]?.(...values),env,calls};workers.push(fake);
   return{pid:4242,on:(event:string,listener:(...values:unknown[])=>void)=>{listeners[event]=listener;},send:(message:{type:string})=>{if(message.type==="settle")queueMicrotask(()=>fake.emit("message",{type:"settled"}));return true;},suspend:()=>{calls.push("suspend");},resume:()=>{calls.push("resume");},terminate:async()=>{calls.push("terminate");return 0;}};}});
 const next=async(count:number)=>{for(let tries=0;workers.length<count&&tries<300;tries++)await wait(10);return workers[count-1]!;};
 return{pool,workers,asks,released,claimed,freed,next};
};

test("a worker over its grant books the missing memory beside it, runs on the same process with the larger grant, and is not killed or started again",async()=>{
 const{pool,workers,asks,released,claimed,freed,next}=harness({hostMegabytes:65536,grant:2048,footprint:()=>3500}),run=pool.run({root:"/tmp/grow"},{}),first=await next(1);
 for(let tries=0;asks.length<2&&tries<300;tries++)await wait(10);
 expect(asks.length).toBeGreaterThanOrEqual(2);expect(asks[1]!.megabytes).toBeGreaterThanOrEqual(Math.ceil(3500*growth.headroomBasisPoints/10000)-2048);expect(asks[1]!.megabytes).toBeGreaterThanOrEqual(growth.minimumMegabytes);
 for(let tries=0;!first.calls.includes("resume")&&tries<300;tries++)await wait(10);
 expect(first.calls.slice(0,2)).toEqual(["suspend","resume"]);expect(first.calls).not.toContain("terminate");
 first.emit("message",{type:"done",graph:{ok:true}});expect(await run).toEqual({graph:{ok:true}});await pool.close();
 expect(workers).toHaveLength(1);expect(pool.stats().overBudget).toBe(0);expect(pool.stats().grown).toBeGreaterThanOrEqual(1);
 for(let tries=0;!released.includes(1)&&tries<300;tries++)await wait(10);
 expect(released,"the growth rides on the one grant: only it is released").toEqual([1]);
 expect(claimed[0],"the warden is told the grant of the worker at its start").toEqual({pid:4242,grantMegabytes:2048,stopped:false,owner:"slice-pool"});
 expect(claimed[1],"and that the pool itself stops it while it waits for more").toMatchObject({stopped:true,grantMegabytes:2048});
 expect(claimed[2]).toMatchObject({stopped:false});expect(claimed[2]!.grantMegabytes!).toBeGreaterThan(2048);
 expect(freed,"the claim ends with the worker").toEqual([4242]);
});

test("while the gate has no room the worker waits stopped in admission: it is neither killed nor resumed before the grant arrives",async()=>{
 let open!:()=>void;const gate=new Promise<void>(resolve=>{open=resolve;});
 const{pool,workers,asks,next}=harness({hostMegabytes:65536,grant:2048,footprint:()=>3500,extra:()=>gate}),run=pool.run({root:"/tmp/wait"},{}),first=await next(1);
 for(let tries=0;asks.length<2&&tries<300;tries++)await wait(10);
 await wait(200);
 expect(first.calls).toEqual(["suspend"]);expect(workers).toHaveLength(1);
 open();for(let tries=0;!first.calls.includes("resume")&&tries<300;tries++)await wait(10);
 expect(first.calls).toEqual(["suspend","resume"]);
 first.emit("message",{type:"done",graph:{}});await run;await pool.close();expect(workers).toHaveLength(1);
});

test("growths are bounded: a worker that keeps outgrowing every grant is judged by the plan guard after the declared maximum, not grown for ever",async()=>{
 const{pool,asks,next}=harness({hostMegabytes:65536,grant:1024,footprint:()=>2048*1.6**Math.min(9,asks.length)}),run=pool.run({root:"/tmp/bounded"},{}),first=await next(1);
 for(let tries=0;asks.length<growth.maximum+1&&tries<400;tries++)await wait(10);
 await wait(300);
 expect(asks.length).toBeLessThanOrEqual(growth.maximum+1);
 first.emit("message",{type:"done",graph:{}});await run;await pool.close();
});

test("a load cut twice asks the gate for its measured peak, not for the reduced share it was admitted with the first time",async()=>{
 const{pool,workers,asks,next}=harness({hostMegabytes:16384,grant:2048,footprint:index=>index<=growth.cutsBeforeReadmit?10500:3000}),run=pool.run({root:"/tmp/readmit"},{});
 const last=await next(growth.cutsBeforeReadmit+1);last.emit("message",{type:"done",graph:{}});await run;await pool.close();
 const booked=asks.filter(ask=>ask.minimumMegabytes!==undefined);
 const final=asks[asks.length-1]!;
 expect(final.minimumMegabytes!,"the booking is already the full share up front, a re-admission never asks for less").toBeGreaterThanOrEqual(asks[0]!.minimumMegabytes??0);
 expect(final.minimumMegabytes!).toBeGreaterThanOrEqual(Math.min(Math.ceil(10500*growth.headroomBasisPoints/10000),Math.floor(16384*elasticity.hostCapBasisPoints/10000))-1);
 expect(final.floorMegabytes).toBe(Math.min(elasticity.minimumMegabytes,final.megabytes));expect(final.floorMegabytes!).toBeLessThan(final.minimumMegabytes!);expect(final.queuedSinceMs).toBeDefined();
 expect(workers.length).toBeLessThanOrEqual(growth.cutsBeforeReadmit+2);expect(booked.length).toBeGreaterThan(0);
});

test("a worker killed from outside names the grant it had instead of a limit of zero, and the next worker is admitted at its measured peak",async()=>{
 const{pool,workers,asks,next}=harness({hostMegabytes:65536,grant:2048,footprint:()=>1500}),cuts:{kind?:string;limit?:number}[]=[],run=pool.run({root:"/tmp/killed"},{onProgress:(_ms:number,detail:any)=>{if(["cut","killed"].includes(detail?.decision?.key))cuts.push(detail.decision);}}),first=await next(1);
 await wait(100);first.emit("exit",null,"SIGKILL");
 const second=await next(2);second.emit("message",{type:"done",graph:{}});await run;await pool.close();
 expect(workers.length).toBe(2);expect(cuts.length).toBeGreaterThan(0);expect(cuts[0]!.limit).toBe(2048);expect(asks.length).toBeGreaterThanOrEqual(2);
});

test("a scope with a measured peak asks for that peak times 1.15 and accepts a lean start derived from it; a scope never measured asks for the plan ceiling but accepts a reduced start",async()=>{
 const reads:(string|undefined)[]=[],{pool,asks,next}=harness({hostMegabytes:65536,footprint:()=>1000,learned:{read:scope=>{reads.push(scope);return 3000;},write:()=>undefined}}),run=pool.run({root:"/tmp/scope-a"},{}),first=await next(1);
 expect(asks[0]!.minimumMegabytes,"the minimum follows the measured booking, never below the floor").toBe(Math.max(elasticity.minimumMegabytes,Math.ceil(asks[0]!.megabytes*elasticity.leanBasisPoints/10000)));expect(asks[0]!.minimumMegabytes!).toBeLessThan(asks[0]!.megabytes);
 expect(asks[0]!.megabytes,"the measured peak of the scope times 1.15").toBe(Math.ceil(3000*11500/10000));
 expect(reads.every(scope=>scope==="/tmp/scope-a"),"the peak is read for the scope of the load").toBe(true);
 first.emit("message",{type:"done",graph:{}});await run;await pool.close();
 const fresh=harness({hostMegabytes:65536,footprint:()=>1000}),second=fresh.pool.run({root:"/tmp/never-measured"},{}),again=await fresh.next(1);
 expect(fresh.asks[0]!.megabytes).toBe(elasticity.maximumMegabytes);
 expect(fresh.asks[0]!.minimumMegabytes!,"the start may be reduced and then grows").toBeLessThan(fresh.asks[0]!.megabytes);
 again.emit("message",{type:"done",graph:{}});await second;await fresh.pool.close();
});

test("the peak a load reached is written for its own scope",async()=>{
 const writes:[number,string|undefined][]=[],{pool,next}=harness({hostMegabytes:65536,footprint:()=>1500,learned:{read:()=>0,write:(megabytes,scope)=>{writes.push([megabytes,scope]);}}}),run=pool.run({root:"/tmp/scope-b"},{}),first=await next(1);
 await wait(100);first.emit("message",{type:"done",graph:{}});await run;await pool.close();
 expect(writes.length).toBeGreaterThanOrEqual(1);expect(writes.every(([megabytes,scope])=>megabytes>0&&scope==="/tmp/scope-b")).toBe(true);
});

test("when the gate refuses the full booking the load falls back to the smaller booking and the grant growth keeps working",async()=>{
 const{pool,asks,next,workers}=harness({hostMegabytes:65536,footprint:()=>1000,refuseFull:true,learned:{read:()=>3000,write:()=>undefined}}),run=pool.run({root:"/tmp/fallback"},{}),first=await next(1);
 expect(asks[0]!.minimumMegabytes!).toBeLessThan(asks[0]!.megabytes);
 expect(asks[1]!.minimumMegabytes!,"the second ask accepts a reduced share").toBeLessThan(asks[1]!.megabytes);
 expect(pool.stats().bookedSmaller).toBe(1);
 first.emit("message",{type:"done",graph:{}});await run;await pool.close();expect(workers).toHaveLength(1);
});

test("the grant of a read stands at its measured peak: the first growth asks the gate once for the learned farm peak instead of one small step per outgrown grant, and a read with no history grows by the headroom step as before",async()=>{
 const peak=6000,learned={read:()=>peak,write:()=>undefined};
 const known=harness({hostMegabytes:65536,grant:2048,footprint:()=>3500,learned}),run=known.pool.run({root:"/tmp/standing"},{}),first=await known.next(1);
 for(let tries=0;known.asks.length<2&&tries<300;tries++)await wait(10);
 expect(known.asks[1]!.megabytes).toBeGreaterThanOrEqual(Math.ceil(peak*growth.standingBasisPoints/10000)-2048);
 expect(known.asks[1]!.megabytes).toBeGreaterThan(Math.ceil(3500*growth.headroomBasisPoints/10000)-2048);
 first.emit("message",{type:"done",graph:{}});await run;await known.pool.close();
 const fresh=harness({hostMegabytes:65536,grant:2048,footprint:()=>3500}),again=fresh.pool.run({root:"/tmp/standing-fresh"},{}),second=await fresh.next(1);
 for(let tries=0;fresh.asks.length<2&&tries<300;tries++)await wait(10);
 expect(fresh.asks[1]!.megabytes).toBe(Math.ceil(3500*growth.headroomBasisPoints/10000)-2048);
 second.emit("message",{type:"done",graph:{}});await again;await fresh.pool.close();
});

test("a growth that the gate cannot answer does not hold the worker stopped for ever: after the declared wait the worker runs on, the wait is abandoned and the pool does not ask again within the cooldown (two planners that each wait for the memory of the other cannot deadlock)",async()=>{
 const{pool,asks,next}=harness({hostMegabytes:65536,grant:2048,footprint:()=>3500,growth:{...growth,waitMilliseconds:150,cooldownMilliseconds:60000},extra:ask=>new Promise((_resolve,reject)=>{ask.signal?.addEventListener("abort",()=>reject(ask.signal?.reason),{once:true});})}),run=pool.run({root:"/tmp/deadlock"},{}),first=await next(1);
 for(let tries=0;!first.calls.includes("resume")&&tries<300;tries++)await wait(10);
 expect(first.calls.slice(0,2)).toEqual(["suspend","resume"]);expect(first.calls).not.toContain("terminate");
 const asked=asks.length;await wait(300);expect(asks.length).toBe(asked);
 first.emit("message",{type:"done",graph:{ok:1}});expect(await run).toEqual({graph:{ok:1}});await pool.close();
});
