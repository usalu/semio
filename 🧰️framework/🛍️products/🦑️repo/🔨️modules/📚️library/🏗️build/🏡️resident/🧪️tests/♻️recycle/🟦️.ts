/** ♻️ Contract: a slice worker that says it recycles at a pass boundary is replaced by a fresh worker that gets the threshold and the carried counters in its environment, the client sees one load that finishes with the graph (never a failure or a counted restart), the handover is announced in both languages, and the number of handovers is bounded by the declared maximum.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs
 * @see ../../../🧠️memory/♻️recycle/🧬️schema/🔣️.json */
import {expect,test} from "bun:test";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {SLICE_RECYCLE_POLICY_V1,sliceRecycleCarriedV1} from "../../../🧠️memory/♻️recycle/🟨️.mjs";
import {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import table from "../../🧬️schema/🔣️.json";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number},guard=HOST_POLICY_V1.slicePlanGuard as {reserveMegabytes:number;ceilingBasisPoints:number;settleWaitMilliseconds:number},MiB=1048576;
const wait=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
type Fake={emit:(event:string,value:unknown)=>void;env:Record<string,string|undefined>;data:unknown};

const harness=(recycle?:null)=>{
 const workers:Fake[]=[],decisions:{key:string;used:number;budget:number;kind?:string;limit?:number}[]=[];
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,sampleMilliseconds:10,heartbeatMilliseconds:30,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,
  planGuard:{...guard,settleWaitMilliseconds:20},hostPressure:()=>"normal",report:()=>undefined,
  pressure:()=>({sample:()=>({tripped:false,kind:"memory",value:0,limit:20,seconds:30}),stop:()=>undefined}),
  headroom:()=>({limitBytes:Infinity,currentBytes:100*MiB,availableBytes:8000*MiB}),
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
  observe:async()=>1000,...recycle===null?{recycle:null}:{},
  createWorker:(_url:unknown,data:unknown,env:Record<string,string|undefined>)=>{const listeners:Record<string,(value:unknown)=>void>={},fake:Fake={emit:(event,value)=>listeners[event]?.(value),env,data};workers.push(fake);
   return{pid:4242,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},send:()=>true,terminate:async()=>0};}});
 const start=(name:string)=>pool.run({root:"/tmp/"+name},{onProgress:(_ms:number,detail:any)=>{if(detail?.decision?.key==="recycled")decisions.push(detail.decision);}});
 const next=async(count:number)=>{for(let tries=0;workers.length<count&&tries<300;tries++)await wait(10);return workers[count-1]!;};
 return{pool,workers,decisions,start,next};
};
const state=(recycles:number)=>({recycles,round:recycles,passes:recycles,owners:1310,heapMegabytes:2600});

test("a worker that recycles is replaced by a fresh one that carries the counters, the load finishes with the graph and no restart is counted",async()=>{
 const{pool,workers,decisions,start,next}=harness(),run=start("recycled");
 const first=await next(1);expect(first.env[SLICE_RECYCLE_POLICY_V1.heapEnvironment]).toBe(String(SLICE_RECYCLE_POLICY_V1.heapMegabytes));expect(sliceRecycleCarriedV1(first.env).recycles).toBe(0);
 first.emit("message",{type:"recycle",state:state(1)});
 const second=await next(2);expect(sliceRecycleCarriedV1(second.env)).toEqual(state(1));expect(second.data).toEqual(first.data);
 second.emit("message",{type:"done",graph:{ok:1}});
 expect(await run).toEqual({graph:{ok:1}});await pool.close();
 expect(workers).toHaveLength(2);expect(decisions).toEqual([{key:"recycled",used:1,budget:SLICE_RECYCLE_POLICY_V1.maximumRecycles,kind:"2600",limit:SLICE_RECYCLE_POLICY_V1.heapMegabytes}]);
 expect(pool.stats().failed).toBe(0);
});

test("the handovers of one load are bounded, and a pool without a policy never asks for one",async()=>{
 const{pool,workers,start,next}=harness(),run=start("bounded").then(()=>undefined,reason=>reason);
 for(let at=1;at<=SLICE_RECYCLE_POLICY_V1.maximumRecycles;at++)(await next(at)).emit("message",{type:"recycle",state:state(at)});
 const last=await next(SLICE_RECYCLE_POLICY_V1.maximumRecycles+1);expect(sliceRecycleCarriedV1(last.env).recycles).toBe(SLICE_RECYCLE_POLICY_V1.maximumRecycles);
 last.emit("message",{type:"recycle",state:state(SLICE_RECYCLE_POLICY_V1.maximumRecycles+1)});
 const error:any=await run;await pool.close();
 expect(error?.code).toBe("slice-recycle");expect(workers).toHaveLength(SLICE_RECYCLE_POLICY_V1.maximumRecycles+1);
 const off=harness(null),offRun=off.start("off");expect((await off.next(1)).env[SLICE_RECYCLE_POLICY_V1.heapEnvironment]).toBeUndefined();(await off.next(1)).emit("message",{type:"done",graph:{}});await offRun;await off.pool.close();
});

test("the handover is announced in both languages with the same placeholders",()=>{
 const find=(node:any):any=>node&&typeof node==="object"?(node.en&&node.de&&node.en["host-slice-owners-recycled"]?node:Object.values(node).map(find).find(Boolean)):undefined,texts=find(table),holes=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(row=>row[1]).sort();
 expect(holes(texts.en["host-slice-owners-recycled"])).toEqual(holes(texts.de["host-slice-owners-recycled"]));
 expect(holes(texts.en["host-slice-owners-recycled"])).toEqual(["budget","kind","limit","seconds","used"]);
});
