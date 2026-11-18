/** ⏳️ Contract (loads of other slices of one checkout; loads of the same slice are one load, see `../🤝️slice-share`): a slice load that waits for the load of the same checkout ahead of it (they share one fact store) or for a free worker is not silent: it reports a queue decision on every heartbeat, in English and German, starts only when the load ahead has ended, and a queued load whose client left never starts a worker nor lets a later load overtake the running one.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs */
import {expect,test} from "bun:test";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {HOST_POLICY_V1,renderTextV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number};
const harness=(maximum:number)=>{
 const started:string[]=[],finish=new Map<string,()=>void>();
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum,heartbeatMilliseconds:20,graceMilliseconds:50,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
  createWorker:(_url:URL,data:{root:string;name:string})=>{const listeners:Record<string,(value:unknown)=>void>={};started.push(data.name);finish.set(data.name,()=>listeners.message?.({type:"done",graph:{}}));return{pid:undefined,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0};}});
 return{pool,started,finish};
};
const until=async(check:()=>boolean)=>{for(let wait=0;!check()&&wait<200;wait++)await new Promise(done=>setTimeout(done,10));expect(check()).toBe(true);};

test("a load behind a load of the same checkout reports the queue every heartbeat and starts when the first has ended",async()=>{
 const{pool,started,finish}=harness(2),queue:any[]=[];
 const first=pool.run({root:"/tmp/queue",name:"first",arguments:["1"]}),second=pool.run({root:"/tmp/queue",name:"second",arguments:["2"]},{onProgress:(_ms:number,detail:any)=>{if(detail?.phase==="queue")queue.push(detail.decision);}});
 await until(()=>queue.length>=3);
 expect(started).toEqual(["first"]);expect(queue.every(decision=>decision.key==="queued-root"&&decision.used===1&&decision.budget===2)).toBe(true);
 finish.get("first")!();await first;await until(()=>started.length===2);finish.get("second")!();await second;await pool.close();
 const text=renderTextV1("host-slice-owners-queued-root",{seconds:5,phase:"queue",done:0,total:0,owners:"",used:1,budget:2,kind:"",limit:0},["en","de"])!;
 for(const language of["en","de"] as const){expect(text[language]).not.toMatch(/\{[a-z]+\}/u);expect(text[language]).toContain("5");}
 expect(text.en).toContain("waiting for the load of this checkout");expect(text.de).toContain("warte auf die vorausgehende Ladung");
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

test("a queued load whose client left starts nothing and a later load still waits for the running one",async()=>{
 const{pool,started,finish}=harness(3),controller=new AbortController();
 const first=pool.run({root:"/tmp/leave",name:"first",arguments:["1"]}),leaving=pool.run({root:"/tmp/leave",name:"leaving",arguments:["2"]},{signal:controller.signal}),third=pool.run({root:"/tmp/leave",name:"third",arguments:["3"]});
 await until(()=>started.length===1);controller.abort(new Error("gone"));
 const error:any=await leaving.then(()=>undefined,reason=>reason);expect(error.message).toBe("gone");
 await new Promise(done=>setTimeout(done,60));expect(started).toEqual(["first"]);
 finish.get("first")!();await first;await until(()=>started.length===2);expect(started).toEqual(["first","third"]);
 finish.get("third")!();await third;await pool.close();
});
