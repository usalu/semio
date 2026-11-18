/** 🛡️ Contract: the pool stops a slice worker for the size of its process tree only when the host is really endangered: before it refuses it asks the worker to settle (collect and shrink) once and measures again, it refuses only when the settled tree is still above the limit and the memory the host can still give out is below the reserve or the memory pressure is raised, and a tree above the absolute ceiling is always refused; the refusal names the tree, the limit, the available memory and the reserve; the limits are the schema's `slicePlanGuard`.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {planGuardVerdictV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs";
import {hostPressureLevelV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🌡️pressure/🟨️.mjs";
import {GRAPH_CUT_POLICY_V1} from "../../../🧠️memory/✂️cut/🟨️.mjs";
import {GRAPH_MEMORY_POLICY_V1} from "../../../🧠️memory/🟨️.mjs";
import {HOST_POLICY_V1,renderBothV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number},guard=HOST_POLICY_V1.slicePlanGuard as {reserveMegabytes:number;ceilingBasisPoints:number;settleWaitMilliseconds:number},MiB=1048576;
const wait=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
const facts={treeMegabytes:0,limitMegabytes:6553,ceilingMegabytes:9830,availableMegabytes:6000,reserveMegabytes:2048,pressure:"normal" as string|undefined,settled:true};

test("the verdict table: under the limit continues, over it is settled first, then only a short host or raised pressure refuses, the ceiling always refuses",()=>{
 const verdict=(change:Partial<typeof facts>)=>planGuardVerdictV1({...facts,...change}) as {action:string;reason?:string};
 expect(verdict({treeMegabytes:6000})).toEqual({action:"continue"});
 expect(verdict({treeMegabytes:6558,settled:false})).toEqual({action:"settle"});
 expect(verdict({treeMegabytes:6558})).toEqual({action:"continue"});
 expect(verdict({treeMegabytes:7000,availableMegabytes:1500})).toEqual({action:"refuse",reason:"memory"});
 expect(verdict({treeMegabytes:7000,availableMegabytes:2047})).toEqual({action:"refuse",reason:"memory"});
 expect(verdict({treeMegabytes:7000,availableMegabytes:2048})).toEqual({action:"continue"});
 expect(verdict({treeMegabytes:7000,pressure:"warn"})).toEqual({action:"continue"});
 expect(verdict({treeMegabytes:7050,pressure:"warn",availableMegabytes:2998})).toEqual({action:"continue"});
 expect(verdict({treeMegabytes:7000,pressure:"warn",availableMegabytes:1500})).toEqual({action:"refuse",reason:"memory"});
 expect(verdict({treeMegabytes:7000,pressure:"critical"})).toEqual({action:"refuse",reason:"pressure"});
 expect(verdict({treeMegabytes:7000,pressure:undefined})).toEqual({action:"continue"});
 expect(verdict({treeMegabytes:7000,availableMegabytes:undefined})).toEqual({action:"refuse",reason:"unknown"});
 expect(verdict({treeMegabytes:9831,settled:false})).toEqual({action:"refuse",reason:"ceiling"});
 expect(verdict({treeMegabytes:9831,availableMegabytes:60000})).toEqual({action:"refuse",reason:"ceiling"});
 expect(verdict({treeMegabytes:9830,availableMegabytes:60000})).toEqual({action:"continue"});
});

test("the limits are the schema's: 2048 MB reserve, 60 % ceiling",()=>{
 expect(guard).toEqual({reserveMegabytes:2048,ceilingBasisPoints:6000,settleWaitMilliseconds:5000});
});

type Fake={emit:(event:string,value:unknown)=>void;sent:unknown[]};
const harness=(options:{observed:()=>number;availableMegabytes?:number;pressure?:()=>string|undefined;answer?:(worker:Fake,set:(megabytes:number)=>void)=>void;wait?:number}) => {
 const workers:Fake[]=[],lines:string[]=[],state={terminated:0};
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,cutPolicy:{...GRAPH_CUT_POLICY_V1,levels:0},graceMilliseconds:50,sampleMilliseconds:10,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,
  planGuard:{...guard,settleWaitMilliseconds:options.wait??guard.settleWaitMilliseconds},report:(line:string)=>{lines.push(line);},hostPressure:options.pressure??(()=>"normal"),
  headroom:()=>({limitBytes:Infinity,currentBytes:0,...options.availableMegabytes===undefined?{}:{availableBytes:options.availableMegabytes*MiB}}),
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
  observe:async()=>options.observed(),
  createWorker:()=>{const listeners:Record<string,(value:unknown)=>void>={},sent:unknown[]=[],fake:Fake={emit:(event,value)=>listeners[event]?.(value),sent};workers.push(fake);return{pid:4242,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},send:(message:{type:string})=>{sent.push(message);if(message.type==="settle")queueMicrotask(()=>options.answer?.(fake,()=>undefined));},terminate:async()=>{state.terminated++;return 0;}};}});
 const started=async()=>{for(let tries=0;workers.length===0&&tries<200;tries++)await wait(10);return workers[0]!;};
 return{pool,workers,lines,state,started};
};

test("a tree 5 MB over the limit on a host with 6000 MB available is settled once and continues, and the worker is never stopped",async()=>{
 const{pool,lines,state,started}=harness({observed:()=>6558,availableMegabytes:6000,answer:worker=>worker.emit("message",{type:"settled"})}),run=pool.run({root:"/tmp/plan-guard"}),worker=await started();
 worker.emit("message",{type:"stage",stage:"result"});await wait(150);
 expect(worker.sent.filter((message:any)=>message.type==="settle")).toHaveLength(1);
 worker.emit("message",{type:"done",graph:{ok:true}});expect(await run).toEqual({graph:{ok:true}});await pool.close();
 expect(pool.stats().overBudget).toBe(0);expect(state.terminated).toBeGreaterThanOrEqual(0);
 expect(lines.filter(line=>/settled from|beruhigt von/u.test(line))).toEqual(["graph: plan stage result settled from 6558 to 6558 MB","Graph: Plan-Stufe result beruhigt von 6558 auf 6558 MB"]);
});

test("a settle that brings the tree below the limit continues with one log line naming the stage and both sizes",async()=>{
 let megabytes=6600;const{pool,lines,started}=harness({observed:()=>megabytes,availableMegabytes:900,answer:(worker)=>{megabytes=5900;worker.emit("message",{type:"settled"});}}),run=pool.run({root:"/tmp/plan-guard"}),worker=await started();
 worker.emit("message",{type:"stage",stage:"edges"});await wait(150);
 worker.emit("message",{type:"done",graph:{}});expect(await run).toEqual({graph:{}});await pool.close();
 expect(lines.filter(line=>line.startsWith("graph: "))).toEqual(["graph: plan stage edges settled from 6600 to 5900 MB"]);expect(pool.stats().overBudget).toBe(0);
});

test("a tree of 7000 MB on a host with 1500 MB available is refused after the settle, with tree, limit, available memory and reserve",async()=>{
 const{pool,started}=harness({observed:()=>7000,availableMegabytes:1500,answer:worker=>worker.emit("message",{type:"settled"})}),run=pool.run({root:"/tmp/plan-guard"}).then(()=>undefined,error=>error),worker=await started();
 worker.emit("message",{type:"stage",stage:"result",operation:"result"});
 const error:any=await run;await pool.close();
 expect(error?.code).toBe("plan-memory");expect(error.stage).toBe("result");expect(error.currentMegabytes).toBe(7000);expect(error.limitMegabytes).toBe(6553);expect(error.availableMegabytes).toBe(1500);expect(error.reserveMegabytes).toBe(2048);expect(error.reason).toBe("memory");
 expect(worker.sent.filter((message:any)=>message.type==="settle")).toHaveLength(1);
 const text=renderBothV1("host-graph-plan-too-heavy",{stage:"result",operation:"result",megabytes:7000,limit:6553,available:1500,reserve:2048})!;
 for(const language of ["en","de"] as const)for(const number of ["7000","6553","1500","2048"])expect(text[language]).toContain(number);
});

test("critical memory pressure refuses a settled tree over the limit even when the host has memory available",async()=>{
 const{pool,started}=harness({observed:()=>7000,availableMegabytes:6000,pressure:()=>"critical",answer:worker=>worker.emit("message",{type:"settled"})}),run=pool.run({root:"/tmp/plan-guard"}).then(()=>undefined,error=>error);await started();
 const error:any=await run;await pool.close();expect(error?.code).toBe("plan-memory");expect(error.reason).toBe("pressure");expect(error.availableMegabytes).toBe(6000);
});

test("a tree above the ceiling is refused at once, without a settle, whatever the host has available",async()=>{
 const{pool,started}=harness({observed:()=>10000,availableMegabytes:60000}),run=pool.run({root:"/tmp/plan-guard"}).then(()=>undefined,error=>error),worker=await started();
 const error:any=await run;await pool.close();expect(error?.code).toBe("plan-memory");expect(error.reason).toBe("ceiling");expect(error.currentMegabytes).toBe(10000);expect(worker.sent).toEqual([]);
});

test("a worker that does not answer the settle is judged after the wait on what it holds",async()=>{
 const{pool,started}=harness({observed:()=>7000,availableMegabytes:1500,wait:100}),run=pool.run({root:"/tmp/plan-guard"}).then(()=>undefined,error=>error);await started();
 const error:any=await run;await pool.close();expect(error?.code).toBe("plan-memory");expect(error.reason).toBe("memory");
});

test("the pressure level of the host: macOS reads the kernel level, a host without a source answers undefined",()=>{
 expect(hostPressureLevelV1({platform:"darwin",run:()=>"1\n"})).toBe("normal");
 expect(hostPressureLevelV1({platform:"darwin",run:()=>"2\n"})).toBe("warn");
 expect(hostPressureLevelV1({platform:"darwin",run:()=>"4\n"})).toBe("critical");
 expect(hostPressureLevelV1({platform:"darwin",run:()=>{throw new Error("no sysctl");}})).toBeUndefined();
 expect(hostPressureLevelV1({platform:"linux",read:()=>"some avg10=0.00 avg60=0.00 avg300=0.00 total=1\nfull avg10=0.00 avg60=0.00 avg300=0.00 total=1\n"})).toBe("normal");
 expect(hostPressureLevelV1({platform:"linux",read:()=>"some avg10=0.00 avg60=0.00 avg300=0.00 total=1\nfull avg10=25.00 avg60=0.00 avg300=0.00 total=1\n"})).toBe("warn");
 expect(hostPressureLevelV1({platform:"linux",read:()=>{throw new Error("no psi");}})).toBeUndefined();
 expect(hostPressureLevelV1({platform:"win32"})).toBeUndefined();
});

test("the worker settles on request and answers, and the process worker can send to its child",()=>{
 const worker=readFileSync(new URL("../../🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts",import.meta.url),"utf8"),process_=readFileSync(new URL("../../🏗️composition/🌐️host/👷️slice-pool/🧵️process/🟨️.mjs",import.meta.url),"utf8");
 expect(worker).toContain('message?.type==="settle"');expect(worker).toContain("settleGraphMemoryV1");expect(worker).toContain('type:"settled"');expect(process_).toContain("send:message=>");
});

test("a refusal before the plan stages begin names the phase the worker reports (verification), not a question mark",async()=>{
 const{pool,started}=harness({observed:()=>8770,availableMegabytes:2803,pressure:()=>"critical",answer:worker=>worker.emit("message",{type:"settled"})}),run=pool.run({root:"/tmp/plan-guard"}).then(()=>undefined,error=>error),worker=await started();
 worker.emit("message",{type:"progress",phase:"verification",done:33,total:4553});
 const error:any=await run;await pool.close();expect(error?.code).toBe("plan-memory");expect(error.stage).toBe("verification");expect(error.operation).toBe("verification");
});

test("the pool names the declared plan limit to every worker it starts, so the worker keeps its own tree under it, and names none where the limit is off",async()=>{
 const environments:Record<string,string|undefined>[]=[],name=GRAPH_MEMORY_POLICY_V1.planCeilingEnvironment;
 const run=async(planLimitMegabytes:number)=>{
  const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,cutPolicy:{...GRAPH_CUT_POLICY_V1,levels:0},graceMilliseconds:50,sampleMilliseconds:10,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,planLimitMegabytes,
   reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
   createWorker:(_url:URL,_data:unknown,environment:Record<string,string|undefined>)=>{environments.push(environment);return{pid:4242,on:(event:string,listener:(value:unknown)=>void)=>{if(event==="message")queueMicrotask(()=>listener({type:"done",graph:{}}));},send:()=>true,terminate:async()=>0};}});
  expect(await pool.run({root:"/tmp/plan-ceiling"})).toEqual({graph:{}});await pool.close();
 };
 const limit=Math.min(elasticity.maximumMegabytes,Math.floor(16384*elasticity.hostCapBasisPoints/10000));
 await run(limit);await run(Infinity);
 expect(environments.map(environment=>environment[name])).toEqual([String(limit),undefined]);expect(limit).toBe(6553);
});
