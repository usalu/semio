/** 🔄️ Contract: a pass that starts again (a boundary between two passes, a registry follow-up) reaches the run output at once as a line in both languages, and a restart-type decision stays in the progress lines that follow it, so a read that resets its counters says why. Independent model: the schema texts rendered by hand.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs
 * @see ../../../🕸️graph/📥️load/♻️pass-reuse/🟦️.ts */
import{expect,test}from"bun:test";
import{createSlicePoolV1}from"../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import{HOST_POLICY_V1,renderTextV1}from"../../📨️protocol/🎚️policy/🟨️.mjs";
import{createPassReuseV1,emitGraphPassNoticeV1,onGraphPassNoticeV1}from"../../../🕸️graph/📥️load/♻️pass-reuse/🟦️.ts";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number};
const fields={seconds:5565,used:2,budget:19,kind:"3",limit:0,cause:"",owners:"",minimum:0,floor:0};

test("the round text exists in both languages and carries the numbers",()=>{
 const text=renderTextV1("host-slice-owners-round",fields,["en","de"]) as Record<string,string>;
 expect(text.en).toBe("Reading the project graph slice (5565 s): a new pass over the owners starts (round 2): 3 path(s) changed under the read, 19 owner(s) forgotten, live heap 0 MB (0 when not measured).");
 expect(text.de).toContain("Runde 2");expect(text.de).toContain("19 Eigentümer vergessen");
});

test("the second pass over the owners announces itself, the first does not",()=>{
 const seen:any[]=[],off=onGraphPassNoticeV1(notice=>seen.push(notice)),lines:string[]=[];
 const reuse=createPassReuseV1({say:line=>lines.push(line),environment:{}});
 reuse.boundary({round:1,moved:[]});reuse.boundary({round:2,moved:["/a","/b"]});
 off();emitGraphPassNoticeV1({round:9,moved:0,evicted:0,heap:0});
 expect(seen).toHaveLength(1);expect(seen[0]).toMatchObject({round:2,moved:2});expect(lines).toHaveLength(1);
});

test("a round message of the worker reaches the progress callback at once, with its decision",async()=>{
 const progress:any[]=[];
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
  createWorker:()=>{const listeners:Record<string,(value:unknown)=>void>={};queueMicrotask(()=>{listeners.message?.({type:"progress",phase:"round",done:2,total:2,running:[],decision:{key:"round",used:2,budget:19,kind:"3",limit:0}});listeners.message?.({type:"done",graph:{}});});return{pid:undefined,stderrTail:()=>"",on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0};}});
 await pool.run({root:"/x"},{onProgress:(_ms:number,detail:any)=>progress.push(detail)});
 const row=progress.find(detail=>detail?.decision?.key==="round");
 expect(row).toMatchObject({phase:"round",running:[],decision:{key:"round",used:2,budget:19,kind:"3"}});
});
