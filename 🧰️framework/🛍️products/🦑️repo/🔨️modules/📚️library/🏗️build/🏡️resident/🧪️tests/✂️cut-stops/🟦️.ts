/** ✂️ Contract: the other stops of a slice load are cut-and-continue too: a kernel/limit overrun (`memory-budget`, the Linux cgroup case) and a memory-pressure stall cut the load one level and start it again; a project too heavy for this machine (`build.graph.owner-too-heavy` from the worker) is handed to the fleet (the next load publishes the wanted marker and waits for the record) up to the policy's attempts; at the last level the old typed refusals stay.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs
 * @see ../../../🧠️memory/✂️cut/🧬️schema/🔣️.json */
import {expect,test} from "bun:test";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {GRAPH_CUT_POLICY_V1} from "../../../🧠️memory/✂️cut/🟨️.mjs";
import {HOST_POLICY_V1,renderBothV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number},guard=HOST_POLICY_V1.slicePlanGuard as {reserveMegabytes:number;ceilingBasisPoints:number;settleWaitMilliseconds:number},MiB=1048576;
const wait=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
type Fake={emit:(event:string,value:unknown)=>void;env:Record<string,string|undefined>};

const harness=(options:{hostMegabytes?:number;tripped?:(env:Record<string,string|undefined>)=>boolean;limitBytes?:(env:Record<string,string|undefined>)=>number}) => {
 const workers:Fake[]=[],decisions:{key:string;used:number;budget:number;kind?:string}[]=[];let current:Fake|undefined;
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,sampleMilliseconds:10,heartbeatMilliseconds:30,hostMegabytes:options.hostMegabytes??16384,elasticity,unlearnedMegabytes:2048,
  planGuard:{...guard,settleWaitMilliseconds:20},hostPressure:()=>"normal",report:()=>undefined,
  pressure:()=>({sample:()=>({tripped:options.tripped?.(current?.env??{})===true,kind:"memory",value:99,limit:20,seconds:30}),stop:()=>undefined}),
  headroom:()=>({limitBytes:options.limitBytes?.(current?.env??{})??Infinity,currentBytes:100*MiB,availableBytes:8000*MiB}),
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
  observe:async()=>1000,
  createWorker:(_url:unknown,_data:unknown,env:Record<string,string|undefined>)=>{const listeners:Record<string,(value:unknown)=>void>={},fake:Fake={emit:(event,value)=>listeners[event]?.(value),env};workers.push(fake);current=fake;
   return{pid:4242,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},send:()=>true,terminate:async()=>0};}});
 const start=(name:string,signal?:AbortSignal)=>pool.run({root:"/tmp/"+name},{signal,onProgress:(_ms:number,detail:any)=>{if(detail?.decision&&["cut","handoff"].includes(detail.decision.key))decisions.push(detail.decision);}});
 const next=async(count:number)=>{for(let tries=0;workers.length<count&&tries<300;tries++)await wait(10);return workers[count-1]!;};
 return{pool,workers,decisions,start,next};
};
const cap=(fake:Fake)=>fake.env.SEMIO_GRAPH_CUT_CONCURRENCY;
const heavy=(named:string)=>({type:"error",message:"graph.owner-too-heavy",stack:"",refusal:{code:"unserved",key:"build.graph.owner-too-heavy",values:{named,limit:3000}}});

test("a memory-pressure stall cuts the load one level and the second worker finishes; at the last level the stall is still the typed refusal",async()=>{
 const stalled=harness({tripped:env=>env.SEMIO_GRAPH_CUT_CONCURRENCY==="4"}),run=stalled.start("pressure");
 await stalled.next(1);const second=await stalled.next(2);expect(cap(second)).toBe("2");second.emit("message",{type:"done",graph:{}});await run;
 expect(stalled.decisions).toEqual([expect.objectContaining({key:"cut",used:2,budget:6})]);await stalled.pool.close();
 const always=harness({tripped:()=>true}),error:any=await always.start("always").then(()=>undefined,reason=>reason);
 expect(error?.code).toBe("memory-pressure");expect(always.workers.length).toBe(GRAPH_CUT_POLICY_V1.levels);await always.pool.close();
});

test("a limit overrun (the Linux cgroup case, memory-budget) cuts the load one level and continues; the old refusal stays at the last level",async()=>{
 const over=harness({limitBytes:env=>env.SEMIO_GRAPH_CUT_CONCURRENCY==="4"?50*MiB:Infinity}),run=over.start("budget");
 await over.next(1);const second=await over.next(2);expect(cap(second)).toBe("2");second.emit("message",{type:"done",graph:{}});await run;
 expect(over.decisions.map(row=>row.key)).toEqual(["cut"]);await over.pool.close();
 const hard=harness({limitBytes:()=>50*MiB}),error:any=await hard.start("hard").then(()=>undefined,reason=>reason);
 expect(error?.code).toBe("memory-budget");await hard.pool.close();
});

test("a too heavy project is handed to the fleet: the load starts again (the next one publishes the wanted marker and waits), three times, then the typed refusal",async()=>{
 const{pool,workers,decisions,start,next}=harness({}),run=start("heavy").then(()=>undefined,reason=>reason);
 for(let at=1;at<=GRAPH_CUT_POLICY_V1.heavyHandoffs;at++){const worker=await next(at);worker.emit("message",heavy("workspace (4096 MB)"));}
 const last=await next(GRAPH_CUT_POLICY_V1.heavyHandoffs+1);last.emit("message",heavy("workspace (4096 MB)"));
 const error:any=await run;await pool.close();
 expect(error?.key).toBe("build.graph.owner-too-heavy");expect(workers).toHaveLength(GRAPH_CUT_POLICY_V1.heavyHandoffs+1);
 expect(decisions.map(row=>row.key)).toEqual(["handoff","handoff","handoff"]);expect(decisions[0]!.kind).toBe("workspace (4096 MB)");
});

test("the fleet delivers after one hand-off: the second load finishes with the graph",async()=>{
 const{pool,start,next}=harness({}),run=start("delivered");
 (await next(1)).emit("message",heavy("workspace"));
 const second=await next(2);second.emit("message",{type:"done",graph:{ok:1}});expect(await run).toEqual({graph:{ok:1}});await pool.close();
});

test("the hand-off line exists in English and German with the owners, the attempt and the budget",()=>{
 const text=renderBothV1("host-slice-owners-handoff",{seconds:3,used:1,budget:3,kind:"workspace (4096 MB)"})!;
 for(const language of ["en","de"] as const)for(const part of ["workspace (4096 MB)","1","3"])expect(text[language]).toContain(part);
});
