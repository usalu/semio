/** 🤝️ Contract: loads of one slice are one load. A client that asks for the slice of a running or queued load joins it, sees its counters and gets its answer without a second computation; a client that leaves leaves alone until the last one is gone; a load of another slice of the same checkout runs alongside it, bound only by the worker maximum; a stopped load leaves no process and no slot. Real processes: a worker with a detached grandchild.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,existsSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createSlicePoolV1,shareKeyV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {waitFor} from "../🟦️.ts";

const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","ss-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const WORKER=new URL("./👷️worker.mjs",import.meta.url);
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}};
const counted=(file:string)=>existsSync(file)?readFileSync(file,"utf8").split("\n").filter(Boolean):[];
const setup=(name:string)=>{
 const counter=join(base,name+".count"),pool=createSlicePoolV1({workerUrl:WORKER,maximum:2,graceMilliseconds:300,heartbeatMilliseconds:50});
 const request=(who:string,extra:Record<string,unknown>={})=>({root:join(base,name),name:who,counter,total:12,stepMilliseconds:100,pidFile:join(base,name+"-"+who+".pid"),...extra});
 return{counter,pool,request};
};
const grandOf=(file:string)=>Number(readFileSync(file,"utf8"));

test("the share key is the slice: root, arguments, tool and graph policy, nothing of the client",()=>{
 const a=shareKeyV1({root:"/r",arguments:["x"],tool:"nx",graphPolicy:"current",name:"a",pidFile:"p"});
 expect(shareKeyV1({root:"/r",arguments:["x"],tool:"nx",graphPolicy:"current",name:"b"})).toBe(a);
 for(const other of[{root:"/s"},{arguments:["y"]},{tool:"bazel"},{graphPolicy:"fresh"}])expect(shareKeyV1({root:"/r",arguments:["x"],tool:"nx",graphPolicy:"current",...other})).not.toBe(a);
});

test("a live leader: the follower of the same slice finishes with the leader, nothing is computed twice, and its log carries the leader's counters",async()=>{
 const{counter,pool,request}=setup("live"),seen:any[]=[];
 const leader=pool.run(request("leader",{arguments:["s"]}));
 expect(await waitFor(()=>counted(counter).length===1,15000)).toBe(true);
 await new Promise(done=>setTimeout(done,350));
 const follower=pool.run(request("follower",{arguments:["s"]}),{onProgress:(_ms:number,detail:any)=>{seen.push(detail);}});
 const[a,b]=await Promise.all([leader,follower]);
 expect(counted(counter)).toEqual(["leader"]);
 expect(b).toBe(a);expect((b as any).graph.by).toBe("leader");
 const taken=seen.filter(detail=>detail?.phase==="taken");
 expect(taken.length).toBeGreaterThan(2);expect(taken.every(detail=>detail.total===12&&detail.done>=1)).toBe(true);
 expect(taken[0].done).toBeGreaterThan(1);
 await pool.close();
},60000);

test("a leader whose client is killed: the load goes on for the follower that joined, without a restart",async()=>{
 const{counter,pool,request}=setup("handover"),leaving=new AbortController();
 const first=pool.run(request("first",{arguments:["s"]}),{signal:leaving.signal}).catch((error:Error)=>error);
 expect(await waitFor(()=>counted(counter).length===1,15000)).toBe(true);
 const second=pool.run(request("second",{arguments:["s"]}));
 await new Promise(done=>setTimeout(done,300));
 const began=performance.now();leaving.abort(new Error("killed"));
 expect(((await first) as Error).message).toBe("killed");expect(performance.now()-began).toBeLessThan(2000);
 const answer:any=await second;
 expect(answer.graph.by).toBe("first");expect(counted(counter)).toEqual(["first"]);
 await pool.close();
},60000);

test("a killed load of another slice of the same checkout: the next one runs alongside it from the start, and nothing of the killed one holds the slot",async()=>{
 const{counter,pool,request}=setup("dead"),killed=new AbortController();
 const first=pool.run(request("first",{arguments:["one"]}),{signal:killed.signal}).catch((error:Error)=>error);
 expect(await waitFor(()=>existsSync(join(base,"dead-first.pid"))&&readFileSync(join(base,"dead-first.pid"),"utf8")!=="",15000)).toBe(true);
 const grand=grandOf(join(base,"dead-first.pid"));expect(alive(grand)).toBe(true);
 const second=pool.run(request("second",{arguments:["two"],total:3}));
 expect(await waitFor(()=>counted(counter).includes("second"),15000)).toBe(true);
 expect(counted(counter)).toEqual(["first","second"]);expect(alive(grand)).toBe(true);
 killed.abort(new Error("stopped"));
 await first;
 expect(alive(grand)).toBe(false);
 const answer:any=await second;
 expect(answer.graph.by).toBe("second");
 expect(pool.busy()).toBe(0);
 await pool.close();
},60000);

test("a stopped stage leaves no process and no slot: the last client leaving ends the whole load, the detached grandchild too",async()=>{
 const{counter,pool,request}=setup("stop"),one=new AbortController(),two=new AbortController();
 const a=pool.run(request("a",{arguments:["s"]}),{signal:one.signal}).catch((error:Error)=>error);
 expect(await waitFor(()=>existsSync(join(base,"stop-a.pid"))&&readFileSync(join(base,"stop-a.pid"),"utf8")!=="",15000)).toBe(true);
 const grand=grandOf(join(base,"stop-a.pid")),b=pool.run(request("b",{arguments:["s"]}),{signal:two.signal}).catch((error:Error)=>error);
 await new Promise(done=>setTimeout(done,200));
 one.abort(new Error("one left"));expect(((await a) as Error).message).toBe("one left");
 expect(alive(grand)).toBe(true);expect(pool.busy()).toBe(1);
 two.abort(new Error("two left"));expect(((await b) as Error).message).toBe("two left");
 expect(alive(grand)).toBe(false);expect(pool.busy()).toBe(0);expect(pool.stats().active).toBe(0);
 const again=await pool.run(request("again",{arguments:["s"],total:2}));
 expect((again as any).graph.by).toBe("again");expect(counted(counter)).toEqual(["a","again"]);
 await pool.close();
},60000);
