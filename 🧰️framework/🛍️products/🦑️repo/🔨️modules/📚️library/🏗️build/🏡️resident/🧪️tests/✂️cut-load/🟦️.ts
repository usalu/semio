/** ✂️ Contract: a slice load whose process tree passes what a plan may take is not refused: the pool starts it again with half the projects at once and half the wave (progress line `host-slice-owners-cut`), the finished records are kept so nothing is repeated, a machine starts at the level its plan limit asks for and remembers the level that worked; only a load already at the last level (one project at a time) is refused with the typed `plan-memory`.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs
 * @see ../../../🧠️memory/✂️cut/🧬️schema/🔣️.json */
import {expect,test} from "bun:test";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {GRAPH_CUT_POLICY_V1} from "../../../🧠️memory/✂️cut/🟨️.mjs";
import {HOST_POLICY_V1,renderBothV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number},guard=HOST_POLICY_V1.slicePlanGuard as {reserveMegabytes:number;ceilingBasisPoints:number;settleWaitMilliseconds:number},MiB=1048576;
const wait=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
type Fake={emit:(event:string,value:unknown)=>void;env:Record<string,string|undefined>};

const harness=(options:{hostMegabytes:number;footprint:(env:Record<string,string|undefined>)=>number;availableMegabytes?:number;complete?:(worker:Fake)=>void}) => {
 const workers:Fake[]=[],decisions:{key:string;used:number;budget:number;kind?:string;limit?:number}[]=[];let current:Fake|undefined;
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,sampleMilliseconds:10,heartbeatMilliseconds:30,hostMegabytes:options.hostMegabytes,elasticity,unlearnedMegabytes:2048,
  planGuard:{...guard,settleWaitMilliseconds:20},hostPressure:()=>"normal",report:()=>undefined,
  headroom:()=>({limitBytes:Infinity,currentBytes:0,availableBytes:(options.availableMegabytes??8000)*MiB}),
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
  observe:async()=>options.footprint(current?.env??{}),
  createWorker:(_url:unknown,_data:unknown,env:Record<string,string|undefined>)=>{const listeners:Record<string,(value:unknown)=>void>={},fake:Fake={emit:(event,value)=>listeners[event]?.(value),env};workers.push(fake);current=fake;
   return{pid:4242,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},send:(message:{type:string})=>{if(message.type==="settle")queueMicrotask(()=>fake.emit("message",{type:"settled"}));return true;},terminate:async()=>0};}});
 const start=(name:string)=>pool.run({root:"/tmp/"+name},{onProgress:(_ms:number,detail:any)=>{if(detail?.decision?.key==="cut")decisions.push(detail.decision);}});
 const next=async(count:number)=>{for(let tries=0;workers.length<count&&tries<300;tries++)await wait(10);return workers[count-1]!;};
 return{pool,workers,decisions,start,next};
};
const wave=(fake:Fake)=>Number(fake.env.SEMIO_SLICE_WAVE_BASIS_POINTS),cap=(fake:Fake)=>fake.env.SEMIO_GRAPH_CUT_CONCURRENCY;
const footprint=(env:Record<string,string|undefined>)=>400+1600*Number(env.SEMIO_GRAPH_CUT_CONCURRENCY??8)-(env.SEMIO_GRAPH_CUT_CONCURRENCY==="2"?400:0);

test("a 16 GB worker (limit 6553 MB, start level 1) whose tree grows to 10500 MB is cut to level 2 and finishes; no third worker is needed",async()=>{
 const{pool,workers,decisions,start,next}=harness({hostMegabytes:16384,footprint:env=>env.SEMIO_GRAPH_CUT_CONCURRENCY==="4"?10500:3600}),run=start("sixteen");
 const first=await next(1);expect(cap(first)).toBe("4");expect(wave(first)).toBe(5000);
 const second=await next(2);expect(cap(second)).toBe("2");expect(wave(second)).toBe(2500);
 second.emit("message",{type:"done",graph:{ok:true}});expect(await run).toEqual({graph:{ok:true}});await pool.close();
 expect(workers).toHaveLength(2);expect(pool.stats().overBudget).toBe(1);
 expect(decisions).toEqual([expect.objectContaining({key:"cut",used:2,budget:6,kind:"10500",limit:6553})]);
});

test("a machine of 8 GB starts at level 2 from its plan limit, so its first load already runs two projects at once",async()=>{
 const{pool,start,next}=harness({hostMegabytes:8192,footprint}),run=start("eight"),first=await next(1);
 expect(cap(first)).toBe("2");expect(wave(first)).toBe(2500);
 first.emit("message",{type:"done",graph:{}});await run;await pool.close();expect(pool.stats().overBudget).toBe(0);
});

test("a serving machine without a plan limit starts uncut: no cap in the environment, the full wave",async()=>{
 const{pool,start,next}=harness({hostMegabytes:65536,footprint:()=>1000}),run=start("server"),first=await next(1);
 expect(cap(first)).toBeUndefined();expect(wave(first)).toBe(10000);
 first.emit("message",{type:"done",graph:{}});await run;await pool.close();
});

test("a load that grew to 9966 MB against 6553 MB is cut one level, a load 2.5 times over is cut two",async()=>{
 const near=harness({hostMegabytes:16384,footprint:env=>env.SEMIO_GRAPH_CUT_CONCURRENCY==="4"?9966:1000}),nearRun=near.start("near");
 await near.next(1);const second=await near.next(2);expect(cap(second)).toBe("2");expect(wave(second)).toBe(2500);second.emit("message",{type:"done",graph:{}});await nearRun;await near.pool.close();
 const far=harness({hostMegabytes:16384,footprint:env=>env.SEMIO_GRAPH_CUT_CONCURRENCY==="4"?16500:1000}),farRun=far.start("far");
 await far.next(1);const third=await far.next(2);expect(cap(third)).toBe("1");expect(wave(third)).toBe(1250);third.emit("message",{type:"done",graph:{}});await farRun;await far.pool.close();
});

test("the level that worked is remembered by the next load while its peak is near the limit, and relaxed by one level when the peak was under half the limit",async()=>{
 const busy=harness({hostMegabytes:16384,footprint:env=>env.SEMIO_GRAPH_CUT_CONCURRENCY==="4"?10000:5000});
 const first=busy.start("first");await busy.next(1);const second=await busy.next(2);expect(cap(second)).toBe("2");await wait(80);second.emit("message",{type:"done",graph:{}});await first;
 const again=busy.start("second"),third=await busy.next(3);expect(cap(third)).toBe("2");await wait(80);third.emit("message",{type:"done",graph:{}});await again;await busy.pool.close();
 const low=harness({hostMegabytes:16384,footprint:()=>1000}),run=low.start("low"),one=await low.next(1);expect(cap(one)).toBe("4");await wait(80);one.emit("message",{type:"done",graph:{}});await run;
 const after=low.start("after"),two=await low.next(2);expect(cap(two)).toBeUndefined();expect(wave(two)).toBe(10000);two.emit("message",{type:"done",graph:{}});await after;await low.pool.close();
});

test("a worker that reports the in-process plan refusal (graph.plan-too-heavy) is cut like one stopped from outside",async()=>{
 const{pool,workers,start,next}=harness({hostMegabytes:16384,footprint:()=>1000}),run=start("inprocess"),first=await next(1);
 first.emit("message",{type:"error",message:"Build refusal graph.plan-too-heavy {\"stage\":\"edges\",\"operation\":\"edges\",\"megabytes\":5200,\"limit\":4915}",stack:"",refusal:{code:"unserved",key:"build.graph.plan-too-heavy",values:{stage:"edges",operation:"edges",megabytes:5200,limit:4915}}});
 const second=await next(2);expect(cap(second)).toBe("2");second.emit("message",{type:"done",graph:{}});await run;await pool.close();expect(workers).toHaveLength(2);
});

test("only a load already at the last level is refused, with the typed plan-memory naming the tree, the limit and the level; every earlier level was tried first",async()=>{
 const{pool,workers,decisions,start,next}=harness({hostMegabytes:16384,footprint:()=>9900}),run=start("floor").then(()=>undefined,error=>error);
 const error:any=await run;await pool.close();
 expect(error?.code).toBe("plan-memory");expect(error.reason).toBe("ceiling");expect(error.currentMegabytes).toBe(9900);expect(error.limitMegabytes).toBe(6553);expect(error.cutLevel).toBe(GRAPH_CUT_POLICY_V1.levels);
 expect(workers).toHaveLength(GRAPH_CUT_POLICY_V1.levels);expect(workers.map(worker=>cap(worker))).toEqual(["4","2","1","1","1","1"]);expect(workers.map(wave)).toEqual([5000,2500,1250,625,312,156]);
 expect(decisions.map(decision=>decision.used)).toEqual([2,3,4,5,6]);
 const text=renderBothV1("host-graph-plan-too-heavy",{stage:"result",operation:"result",megabytes:9900,limit:6553,available:8000,reserve:2048})!;expect(text.en).toContain("9900");
 void next;
});

test("cancelling a cut load ends it with the abort and starts no further worker",async()=>{
 const controller=new AbortController(),{pool,workers,next}=harness({hostMegabytes:16384,footprint:()=>20000}),run=pool.run({root:"/tmp/abort"},{signal:controller.signal}).then(()=>undefined,error=>error);
 await next(1);controller.abort(new Error("stop"));const error:any=await run;await pool.close();expect(error?.message).toBe("stop");expect(workers.length).toBeLessThanOrEqual(2);
});

test("the cut progress line exists in English and German with the same placeholders and names the level, the tree and the limit",()=>{
 const text=renderBothV1("host-slice-owners-cut",{seconds:12,phase:"inference",done:3,total:78,owners:"",used:2,budget:6,kind:"9966",minimum:0,floor:0,limit:6553})!;
 for(const language of ["en","de"] as const)for(const number of ["12","2","6","9966","6553"])expect(text[language]).toContain(number);
});
