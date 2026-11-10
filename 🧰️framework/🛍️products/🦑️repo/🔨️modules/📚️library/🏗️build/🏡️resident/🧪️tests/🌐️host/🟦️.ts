import {afterAll,describe,expect,test} from "bun:test";
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {spawnSync} from "node:child_process";
import {HERE,ORACLE,sleep} from "../🟦️.ts";
import {createIndexPoolV1} from "../../🏛️service/🌐️host/🧊️pool/🟨️.mjs";
import {createHostServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {spawnHostServiceV1} from "../../🏗️composition/🌐️host/🟨️.mjs";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {GRAPH_MEMORY_POLICY_V1} from "../../../🧠️memory/🟨️.mjs";
import {PlanRefusedErrorV1,connectHostV1,requestHostStatusV1,requestPlanV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {HOST_POLICY_V1,validatorV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {writeFileSync} from "node:fs";
import {createCasStoreV1} from "../../../🗄️cas/🗃️store/🟦️.ts";
import {observedFs} from "../../../👁️observation/🟨️.mjs";
import {createDerivedGraphV1} from "../../../🕸️graph/🧮️derived-nodes/🟨️.mjs";
import {KIND_RULES_V1,verifyObservedV1} from "../../../🕸️graph/🧮️derived-nodes/🧩️kinds/🟨️.mjs";
import {createTaskRecorderV1} from "../../../🕸️graph/🧮️derived-nodes/🎯️task-state/🟨️.mjs";
import {createRevisionCaptureHookV1,createRevisionPinHookV1,loadManifestV1,openPinnedWorldV1,releasePinV1} from "../../../🧊️revision/🟦️.ts";

const GRAPH=join(HERE,"🧫️fixtures/🛰️host-graph.json"),MAIN=join(HERE,"🌐️host/🖥️host-main.mjs");
const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","hs-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const oracle=(targets:string[])=>JSON.parse(spawnSync(process.env.SEMIO_PYTHON??"python3",["-I",ORACLE,"plan",GRAPH,...targets],{encoding:"utf8"}).stdout) as Record<string,string[]>;
const workspace=(name:string,graph=GRAPH)=>{const root=join(base,name);mkdirSync(root,{recursive:true});copyFileSync(graph,join(root,"graph.json"));return realpathSync(root);};
const environment=(name:string)=>{const dir=join(base,"run-"+name);mkdirSync(dir,{recursive:true});return{...process.env,XDG_RUNTIME_DIR:dir,TMPDIR:dir,LOCALAPPDATA:dir} as Record<string,string>;};
const digestOf=(name:string)=>createHash("sha256").update(name).digest("hex");
const checkPlan=(tasks:any[],targets:string[])=>{
 const expected=oracle(targets),position=new Map(tasks.map((task,at)=>[task.id,at]));
 expect(Object.fromEntries(tasks.map(task=>[task.id,task.dependsOn]))).toEqual(expected);
 for(const task of tasks)for(const dependency of task.dependsOn)expect(position.get(dependency)!).toBeLessThan(position.get(task.id)!);
};

describe("index pool",()=>{
 test("single flight, reference counting, least recently used eviction within the bound",async()=>{
  let clock=0,loads=0;const evicted:string[]=[];
  const pool=createIndexPoolV1({maximumBytes:100,maximumEntries:3,now:()=>clock++,onEvict:key=>evicted.push(key)});
  const load=(bytes:number)=>async()=>{loads++;await sleep(5);return{value:{bytes},bytes};};
  const [a1,a2]=await Promise.all([pool.acquire("a",load(60)),pool.acquire("a",load(60))]);
  expect(loads).toBe(1);expect(pool.stats().bytes).toBe(60);
  a1.release();a1.release();a2.release();
  const b=await pool.acquire("b",load(60));
  expect(evicted).toEqual(["a"]);expect(pool.stats().bytes).toBe(60);
  const c=await pool.acquire("c",load(30));
  expect(pool.stats().bytes).toBe(90);b.release();c.release();
  const d=await pool.acquire("d",load(50));
  expect(pool.stats().bytes).toBeLessThanOrEqual(100);d.release();
 });
 test("a referenced index is never evicted and a load nobody waits for is aborted",async()=>{
  const pool=createIndexPoolV1({maximumBytes:10,maximumEntries:1});
  const held=await pool.acquire("x",async()=>({value:1,bytes:50}));
  const other=await pool.acquire("y",async()=>({value:2,bytes:50}));
  expect(pool.keys().sort()).toEqual(["x","y"]);held.release();other.release();
  expect(pool.stats().bytes).toBeLessThanOrEqual(50);
  const controller=new AbortController();let aborted=false;
  const pending=pool.acquire("z",signal=>new Promise((_,reject)=>{signal.addEventListener("abort",()=>{aborted=true;reject(signal.reason);});}),controller.signal);
  controller.abort(new Error("gone"));
  await expect(pending).rejects.toThrow("gone");expect(aborted).toBe(true);expect(pool.keys()).not.toContain("z");
 });
});

describe("planner port against the Python oracle",()=>{
 test("closure and dependency order equal the oracle for every target set",async()=>{
  const graph=JSON.parse(readFileSync(GRAPH,"utf8")),port=createGraphPlannerPortV1({loadGraph:async()=>graph});
  const index=await port.load({root:"/x"});
  for(const targets of[["app:test"],["core:build"],["tool:build","docs:test"],["app:build","util:test","app:test"]]){
   const answer=await port.plan({index,targets,workspace:"0123456789ab"});
   checkPlan(answer.tasks,targets);
  }
  await expect(port.plan({index,targets:["nope:build"]})).rejects.toThrow("Unknown target");
 });
});

function inProcess(name:string,extra:Record<string,unknown>={},loadMs=0){
 const env=environment(name),implementation=digestOf("impl-"+name),runtime=hostEndpointsV1(implementation,env);
 const loads={count:0};
 const planner=createGraphPlannerPortV1({identify:async({root}:any)=>createHash("sha256").update(readFileSync(join(root,"graph.json"))).digest("hex"),loadGraph:async({root,signal}:any)=>{loads.count++;if(loadMs)await sleep(loadMs);signal?.throwIfAborted();return JSON.parse(readFileSync(join(root,"graph.json"),"utf8"));}});
 const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50,...extra}});
 const spawnService=async()=>{await service.start();};
 return{env,implementation,runtime,service,loads,spawnService};
}

describe("host service pins revisions",()=>{
 test("plan requests carry parent and dirty set, replies return the pinned revision, manifest and pin; edits after the reply never change the pinned content",async()=>{
  const env=environment("rev"),implementation=digestOf("impl-rev"),runtime=hostEndpointsV1(implementation,env),root=workspace("wrev"),cas=createCasStoreV1({root:join(base,"cas-rev")});
  writeFileSync(join(root,"src.txt"),"one\n");
  const pin=createRevisionPinHookV1({cas,inputsOf:async()=>["src.txt","graph.json"]});
  const planner=createGraphPlannerPortV1({identify:async()=>"k".repeat(64),loadGraph:async()=>JSON.parse(readFileSync(GRAPH,"utf8")),pin});
  const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50}});
  const ask=(extra:Record<string,unknown>)=>requestPlanV1({root,implementation,targets:["app:test"],agent:"agent-1",language:"en",environment:env,spawnService:async()=>{await service.start();},...extra});
  const first=await ask({});
  expect(validatorV1("PlanReply")(first.reply)).toEqual([]);
  expect(first.reply.revision).toMatch(/^[0-9a-f]{64}$/u);expect(first.reply.pin!.revision).toBe(first.reply.revision);
  const world=await openPinnedWorldV1(cas,first.reply.revision);
  expect(Buffer.from((await world.read("src.txt"))!).toString()).toBe("one\n");
  writeFileSync(join(root,"src.txt"),"two\n");
  expect(Buffer.from((await (await openPinnedWorldV1(cas,first.reply.revision)).read("src.txt"))!).toString()).toBe("one\n");
  const second=await ask({parent:first.reply.revision,dirty:{paths:["src.txt"],cursor:"c2"}});
  expect(validatorV1("PlanRequest")({t:"plan",id:1,workspace:"0123456789ab",root,targets:["a:b"],agent:"a",priority:1,language:"en",parent:first.reply.revision,dirty:{paths:["src.txt"],cursor:"c2"}})).toEqual([]);
  expect(second.reply.manifest).not.toBe(first.reply.manifest);
  const loaded=await loadManifestV1(cas,second.reply.revision);
  expect(loaded.revision.parent).toBe(first.reply.revision);expect(loaded.revision.journalCursor).toBe("c2");
  expect(Buffer.from((await (await openPinnedWorldV1(cas,second.reply.revision)).read("src.txt"))!).toString()).toBe("two\n");
  await releasePinV1(cas,first.reply.pin!);await releasePinV1(cas,second.reply.pin!);
  await service.stop("test");
 });
});

describe("slice worker pool",()=>{
 const slow=new URL("./🐌️slow-slice-worker.mjs",import.meta.url),graphText=readFileSync(GRAPH,"utf8");
 test("a slow synchronous slice load on a worker never blocks the service: status answers within a second while the load runs, and its heartbeat reports progress",async()=>{
  let notes=0;const pool=createSlicePoolV1({workerUrl:slow,maximum:2,heartbeatMilliseconds:200,note:()=>{notes++;}}),beats:number[]=[];
  const h=inProcess("pool-latency",{},0),root=workspace("w-pool");await h.service.start();
  const load=pool.run({root,graphText,busyMilliseconds:2500},{onProgress:ms=>beats.push(ms)});
  const latencies:number[]=[];
  for(let at=0;at<6;at++){const started=performance.now();const status=await requestHostStatusV1({implementation:h.implementation,environment:h.env});latencies.push(performance.now()-started);expect(status).toBeDefined();await sleep(300);}
  expect(Math.max(...latencies)).toBeLessThan(1000);
  await sleep(100);const samples=pool.memory();expect(samples.length).toBe(1);expect(samples[0]).toMatchObject({index:1,heapMb:12.5,extraMb:3.5,objectCount:1000});expect(samples[0]!.ageMilliseconds).toBeLessThan(3000);expect(validatorV1("MemoryStatus")({rssMb:1,heapMb:1,extraMb:1,objectCount:1,objectTypes:[],caches:[],workers:samples})).toEqual([]);
  const loaded=await load;expect(Object.keys((loaded.graph as any).projects??loaded.graph).length).toBeGreaterThan(0);expect(loaded.generationPin).toBe("pin-"+root.length);
  expect(beats.length).toBeGreaterThan(2);expect(notes).toBeGreaterThanOrEqual(beats.length);expect(pool.stats().completed).toBe(1);expect(pool.memory().every(row=>row.running===false)).toBe(true);
  await h.service.stop("test");
 },30000);
 test("the phase counters a worker reports reach the heartbeat of its pool as detail",async()=>{
  const pool=createSlicePoolV1({workerUrl:slow,maximum:1,heartbeatMilliseconds:100}),details:any[]=[],root=workspace("w-detail");
  await pool.run({root,graphText,busyMilliseconds:600,reportPhase:"verification"},{onProgress:(_ms:number,detail:any)=>{if(detail)details.push(detail);}});
  expect(details.length).toBeGreaterThan(0);expect(details[0]).toEqual({phase:"verification",done:7,total:9});
 });
 test("at most the maximum run at once, one root runs one after the other, a queued load can be cancelled and a failing worker rejects",async()=>{
  const pool=createSlicePoolV1({workerUrl:slow,maximum:2,heartbeatMilliseconds:1000}),a=workspace("w-pa"),b=workspace("w-pb"),c=workspace("w-pc");
  const started=performance.now(),runs=[pool.run({root:a,graphText,busyMilliseconds:600}),pool.run({root:a,graphText,busyMilliseconds:600}),pool.run({root:b,graphText,busyMilliseconds:600})];
  await sleep(100);expect(pool.stats().active).toBeLessThanOrEqual(2);
  await Promise.all(runs);const elapsed=performance.now()-started;
  expect(elapsed).toBeGreaterThan(1100);
  const controller=new AbortController(),slowOne=pool.run({root:c,graphText,busyMilliseconds:5000},{signal:controller.signal});
  await sleep(200);controller.abort(new Error("cancelled by test"));await expect(slowOne).rejects.toThrow("cancelled by test");expect(pool.stats().cancelled).toBe(1);
  await expect(pool.run({root:c,graphText,busyMilliseconds:0,fail:true})).rejects.toThrow("fixture failure");expect(pool.stats().failed).toBeGreaterThanOrEqual(1);
 },30000);
});

describe("slice memory guard",()=>{
 test("the host policy declares a finite safety margin and the pool refuses a missing or non-finite margin instead of disabling its guard",()=>{
  expect(Number.isFinite(HOST_POLICY_V1.sliceSafetyMarginMegabytes)&&HOST_POLICY_V1.sliceSafetyMarginMegabytes>0).toBe(true);
  const workerUrl=new URL("./🐌️slow-slice-worker.mjs",import.meta.url);
  for(const marginMegabytes of [Number(undefined),-1,Infinity])expect(()=>createSlicePoolV1({workerUrl,maximum:1,marginMegabytes})).toThrow("Finite memory margin and budget required");
 });
 const slow=new URL("./🐌️slow-slice-worker.mjs",import.meta.url),graphText=readFileSync(GRAPH,"utf8"),MiB=1048576;
 /** 🧪️ A modelled process: its resident bytes are a fixed base plus the growth of the running slice loads, read by the pool's guard and by the service's own sweep alike, so the outcome never depends on the allocator or on timing. */
 const serve=async(name:string,limitMegabytes:number,margin:number,growthMegabytes:number,residentMegabytes?:number)=>{
  const base=512*MiB;let busy:()=>number=()=>0;const rss=()=>base+(busy()>0?growthMegabytes*MiB:0);
  const pool=createSlicePoolV1({workerUrl:slow,maximum:2,marginMegabytes:margin,sampleMilliseconds:50,headroom:()=>({limitBytes:base+limitMegabytes*MiB,currentBytes:rss()})});busy=()=>pool.memory().filter(row=>row.running).length;
  const env=environment(name),implementation=digestOf("impl-"+name),runtime=hostEndpointsV1(implementation,env),root=workspace("w-"+name);
  const planner=createGraphPlannerPortV1({identify:async()=>digestOf(name+"-key"),loadGraph:async()=>JSON.parse(graphText),loadSlice:async(request:any)=>pool.run({root:request.root,graphText,busyMilliseconds:1500},{signal:request.signal,onProgress:request.onProgress})});
  const service=createHostServiceV1({runtime,implementation,ports:{planner,workers:{memory:()=>pool.memory(),active:()=>pool.busy()},resident:{rss}},policy:{residentCheckMilliseconds:50,...residentMegabytes===undefined?{}:{maximumResidentBytes:residentMegabytes*MiB}}});await service.start();
  return{pool,service,env,implementation,root};
 };
 test("a plan that would cross the memory limit is refused in both languages with the current and limit figures, the worker is gone and the service keeps answering",async()=>{
  const h=await serve("guard",150,20,400);
  const failure=await requestPlanV1({root:h.root,implementation:h.implementation,targets:["app:test"],agent:"a",language:"en",environment:h.env,arguments:["run","app:test"]}).then(()=>undefined,error=>error);
  expect(failure).toBeInstanceOf(PlanRefusedErrorV1);expect(failure.code).toBe("memory");expect(failure.retry).toBe("later");
  expect(failure.message_.en).toMatch(/holds 912 MB of its 662 MB memory limit \(safety margin 20 MB\)/u);expect(failure.message_.de).toMatch(/belegt 912 MB seines Speicherlimits von 662 MB/u);expect(failure.message_.en).not.toBe(failure.message_.de);
  const status=await requestHostStatusV1({implementation:h.implementation,environment:h.env});
  expect(status!.plans!.budgetRefusals).toBe(1);expect(validatorV1("ServiceStatus")(status)).toEqual([]);expect(h.pool.stats().overBudget).toBe(1);expect(h.pool.stats().active).toBe(0);
  await h.service.stop("test");
 },30000);
 test("a worker growing past the service's own restart threshold costs only its plan: refused, service alive, status answering, worker samples kept",async()=>{
  const h=await serve("restart",250,20,900,512+60),exits:string[]=[];h.service.closed.then(reason=>exits.push(reason));
  const failure=await requestPlanV1({root:h.root,implementation:h.implementation,targets:["app:test"],agent:"a",language:"en",environment:h.env,arguments:["run","app:test"]}).then(()=>undefined,error=>error);
  expect(failure).toBeInstanceOf(PlanRefusedErrorV1);expect(failure.code).toBe("memory");
  await sleep(300);expect(exits).toEqual([]);
  const status=await requestHostStatusV1({implementation:h.implementation,environment:h.env});
  expect(status).toBeDefined();expect(validatorV1("ServiceStatus")(status)).toEqual([]);expect(status!.plans!.budgetRefusals).toBe(1);
  expect(status!.memory!.workers.length).toBe(1);expect(status!.memory!.workers[0]).toMatchObject({running:false,heapMb:12.5});
  await h.service.stop("test");
 },30000);
 test("a plan that fits under the limit is never refused, however large its growth",async()=>{
  const h=await serve("fits",4000,20,400);
  const result=await requestPlanV1({root:h.root,implementation:h.implementation,targets:["app:test"],agent:"a",language:"en",environment:h.env,arguments:["run","app:test"]});
  expect(result.reply.revision).toBeDefined();expect(h.pool.stats().overBudget).toBe(0);expect(h.pool.stats().completed).toBe(1);
  const status=await requestHostStatusV1({implementation:h.implementation,environment:h.env});expect(status!.plans!.budgetRefusals).toBe(0);
  await h.service.stop("test");
 },30000);
});

describe("plan load statistics",()=>{
 test("the statistics a slice load returns reach the status plans block and stay out of the slice document",async()=>{
  const graphText=readFileSync(GRAPH,"utf8"),env=environment("stats"),implementation=digestOf("impl-stats"),runtime=hostEndpointsV1(implementation,env),root=workspace("w-stats");
  const stats={staleFacts:12,closuresVerified:30,ownersReinferred:3,ownersCutOff:7,verifiedMilliseconds:250};
  const planner=createGraphPlannerPortV1({identify:async()=>digestOf("stats-key"),loadGraph:async()=>JSON.parse(graphText),loadSlice:async()=>({graph:JSON.parse(graphText),stats})});
  const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50}});await service.start();
  const result=await requestPlanV1({root,implementation,targets:["app:test"],agent:"a",language:"en",environment:env,arguments:["run","app:test"]});
  expect(JSON.parse(readFileSync(join(result.runtime.directory,"plans-"+result.runtime.tag,result.reply.slice.hash+".json"),"utf8")).stats).toBeUndefined();
  const status=await requestHostStatusV1({implementation,environment:env});
  expect(status!.plans!.load).toEqual(stats);expect(status!.plans!.computed).toBe(1);expect(validatorV1("ServiceStatus")(status)).toEqual([]);
  await service.stop("test");
 },30000);
});

describe("host service in process",()=>{
 test("status answers with the memory report while a plan is still loading, never queued behind it",async()=>{
  const h=inProcess("status-lane",{},1500),root=workspace("w-lane");await h.service.start();
  const pending=requestPlanV1({root,implementation:h.implementation,targets:["app:test"],agent:"agent-1",language:"en",environment:h.env,spawnService:h.spawnService});
  await sleep(300);
  const started=performance.now(),status=await requestHostStatusV1({implementation:h.implementation,environment:h.env});
  expect(performance.now()-started).toBeLessThan(700);
  expect(status!.inFlight).toBeGreaterThanOrEqual(1);expect(validatorV1("ServiceStatus")(status)).toEqual([]);expect(status!.memory!.rssMb).toBeGreaterThan(0);
  checkPlan((await pending).plan.tasks,["app:test"]);
  await h.service.stop("test");
 });
 test("auto start, plan equals oracle, second client and second workspace share one index, status counts it once",async()=>{
  const h=inProcess("a"),first=workspace("w1"),second=workspace("w2"),phases:string[]=[];
  const run=(root:string,agent:string,targets:string[])=>requestPlanV1({root,implementation:h.implementation,targets,agent,language:"de",environment:h.env,spawnService:h.spawnService,onProgress:frame=>phases.push(frame.phase)});
  const one=await run(first,"agent-1",["app:test"]);
  expect(one.reply.cacheHits).toBe(0);checkPlan(one.plan.tasks,["app:test"]);
  expect(phases).toContain("start");
  const again=await run(first,"agent-2",["app:test"]);
  expect(again.reply.cacheHits).toBe(1);expect(again.reply.revision).toBe(one.reply.revision);
  const other=await run(second,"agent-3",["tool:build"]);checkPlan(other.plan.tasks,["tool:build"]);
  const status=await requestHostStatusV1({implementation:h.implementation,environment:h.env});
  expect(status!.workspaces.length).toBe(2);expect(status!.indexes).toBe(1);expect(h.loads.count).toBe(1);
  expect(validatorV1("ServiceStatus")(status)).toEqual([]);
  expect(validatorV1("PlanReply")(one.reply)).toEqual([]);
  expect(status!.memory!.rssMb).toBeGreaterThan(0);expect(status!.memory!.workers).toEqual([]);expect(status!.memory!.heapMb).toBeGreaterThan(0);expect(status!.memory!.objectTypes.length).toBeGreaterThan(0);expect(Array.isArray(status!.memory!.caches)).toBe(true);
  expect(status!.workspaces.every((row:any)=>row.indexBytes>0)).toBe(true);
  await h.service.stop("test");
  expect(readFileSync(GRAPH).length).toBeGreaterThan(0);
 });
 test("workspace identity, capability and target errors are refusals, not crashes",async()=>{
  const h=inProcess("b"),root=workspace("w3");await h.service.start();
  await expect(requestPlanV1({root,implementation:h.implementation,targets:["zzz:build"],agent:"a",language:"en",environment:h.env})).rejects.toBeInstanceOf(PlanRefusedErrorV1);
  const{connectHostV1:_}=await import("../../🚪️client/🛰️plan/🟨️.mjs");
  const{socket}=await connectHostV1({implementation:h.implementation,environment:h.env});
  const replies:any[]=[];
  await new Promise<void>(resolve=>{
   socket.on("data",chunk=>{for(const line of String(chunk).split("\n"))if(line)replies.push(JSON.parse(line));if(replies.some(r=>r.t==="refused"))resolve();});
   socket.write(JSON.stringify({t:"hello",v:1,client:{name:"x",pid:1,lane:"l"},languages:["en"]})+"\n"+JSON.stringify({t:"plan",id:1,workspace:"0123456789ab",root,targets:["app:build"],agent:"a",priority:1,language:"en",cap:"v1.abcd."+"A".repeat(43)})+"\n");
  });
  socket.destroy();
  expect(replies.find(r=>r.t==="refused").code).toBe("capability");
  await h.service.stop("test");
 });
 test("cancelling the request aborts the load nobody waits for and frees the slot",async()=>{
  const h=inProcess("c",{},2000),root=workspace("w4"),controller=new AbortController();
  const pending=requestPlanV1({root,implementation:h.implementation,targets:["app:build"],agent:"a",language:"en",environment:h.env,spawnService:h.spawnService,signal:controller.signal,onProgress:frame=>{if(frame.phase==="load")controller.abort(new Error("cancelled"));}});
  await expect(pending).rejects.toThrow("cancelled");
  await sleep(200);
  const status=await requestHostStatusV1({implementation:h.implementation,environment:h.env});
  expect(status!.inFlight).toBe(0);expect(status!.indexes).toBe(0);
  await h.service.stop("test");
 });
 test("large plans spill to a content addressed reference that verifies",async()=>{
  const h=inProcess("d",{maximumInlinePlanTasks:2}),root=workspace("w5");
  const result=await requestPlanV1({root,implementation:h.implementation,targets:["app:test"],agent:"a",language:"en",environment:h.env,spawnService:h.spawnService});
  expect("ref" in result.reply.plan).toBe(true);checkPlan(result.plan.tasks,["app:test"]);
  await h.service.stop("test");
 });
 test("bounded: more workspaces than the bound close the least recently used one",async()=>{
  const h=inProcess("e",{maximumWorkspaces:2}),roots=["x1","x2","x3"].map(n=>workspace(n));
  for(const root of roots)await requestPlanV1({root,implementation:h.implementation,targets:["core:build"],agent:"a",language:"en",environment:h.env,spawnService:h.spawnService});
  const status=await requestHostStatusV1({implementation:h.implementation,environment:h.env});
  expect(status!.workspaces.length).toBe(2);expect(status!.workspaces.map((w:any)=>w.root)).not.toContain(roots[0]);
  await h.service.stop("test");
 });
});

describe("bounded memory",()=>{
 test("exceeding the resident bound drops indexes and finally retires the service for a restart",async()=>{
  const h=inProcess("m",{maximumResidentBytes:1}),root=workspace("mw");
  await h.service.start();
  const exits:string[]=[];h.service.closed.then(reason=>exits.push(reason));
  await requestPlanV1({root,implementation:h.implementation,targets:["core:build"],agent:"a",language:"en",environment:h.env});
  for(let at=0;at<100&&exits.length===0;at++)await sleep(50);
  expect(exits).toEqual(["memory"]);expect(h.service.pool.stats().bytes).toBe(0);
 });
 test("a stalled slice load reaches the client as the typed refusal: code stalled, retry later, rendered in both languages, never internal",async()=>{
  const env=environment("stalled"),implementation=digestOf("impl-stalled"),runtime=hostEndpointsV1(implementation,env),root=workspace("stalled-w");
  const stalled=Object.assign(new Error("stalled process.routing.stalled-worker"),{name:"RepoError",code:"stalled",key:"process.routing.stalled-worker",values:{minutes:10,retry:"later"}});
  const planner=createGraphPlannerPortV1({identify:async()=>"id",loadGraph:async()=>{throw stalled;}});
  const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50}});
  await service.start();
  try{
   const error=await requestPlanV1({root,implementation,targets:["core:build"],agent:"a",language:"en",environment:env}).then(()=>undefined,reason=>reason as any);
   expect(error?.name).toBe("PlanRefusedErrorV1");expect(error.code).toBe("stalled");expect(error.retry).toBe("later");
   expect(error.message_.en).toContain("10 minutes");expect(error.message_.de).toContain("10 Minuten");expect(error.message_.en).not.toContain("process.routing");
  }finally{await service.stop("test");}
 });
 test("an unexpected failure reaches the client localized and retryable, never as the raw error text",async()=>{
  const env=environment("internal"),implementation=digestOf("impl-internal"),runtime=hostEndpointsV1(implementation,env),root=workspace("internal-w");
  const planner=createGraphPlannerPortV1({identify:async()=>"id",loadGraph:async()=>{throw new Error("secret implementation detail");}});
  const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50}});
  await service.start();
  try{
   const error=await requestPlanV1({root,implementation,targets:["core:build"],agent:"a",language:"de",environment:env}).then(()=>undefined,reason=>reason as any);
   expect(error?.name).toBe("PlanRefusedErrorV1");expect(error.code).toBe("internal");expect(error.retry).toBe("later");
   expect(JSON.stringify(error.message_)).not.toContain("secret implementation detail");expect(error.message_.de).toContain("unerwarteten");expect(error.message_.en).toContain("unexpected");expect(error.message_.en).toMatch(/reference [0-9a-f]{8}/u);
  }finally{await service.stop("test");}
 });
 test("a second service on a live endpoint adopts it: it exits quietly, leaves no crash row and the first keeps serving",async()=>{
  const h=inProcess("adopt"),root=workspace("adopt-w");
  await h.service.start();
  const{runHostForegroundV1}=await import("../../🏗️composition/🌐️host/🟨️.mjs"),{readServiceExitRowsV1}=await import("../../🧾️exit-log/🟨️.mjs");
  const code=await runHostForegroundV1({implementation:h.implementation,loadGraph:async()=>({}),identify:async()=>"x",environment:h.env});
  expect(code).toBe(0);
  const rows=readServiceExitRowsV1(h.runtime.directory);
  expect(rows.some((row:any)=>row.reason==="adopted")).toBe(true);expect(rows.some((row:any)=>row.reason==="crash")).toBe(false);
  const plan=await requestPlanV1({root,implementation:h.implementation,targets:["core:build"],agent:"a",language:"en",environment:h.env});
  expect(plan).toBeDefined();
  await h.service.stop("test");
 });
});

describe("host service as a separate process",()=>{
 test("a service started by a client does not inherit the client's capacity grant or plan child and gets the declared collector target",async()=>{
  const{mkdtempSync,readFileSync,rmSync}=await import("node:fs"),{tmpdir}=await import("node:os"),{join}=await import("node:path"),directory=mkdtempSync(join(tmpdir(),"host-env-")),out=join(directory,"env.json");
  try{
   const start=spawnHostServiceV1({command:[process.execPath,"-e","require('node:fs').writeFileSync(process.argv[1],JSON.stringify(process.env))",out],environment:{PATH:process.env.PATH??"",SEMIO_HOST_CAPACITY_GRANT:"grant",SEMIO_HOST_CAPACITY_PLAN:"{}",SEMIO_BUILD_GRAPH_PLAN_OUT:"/x",SEMIO_LANGUAGE:"en"}});
   start({runtime:{directory,log:join(directory,"log")}});
   for(let at=0;at<100&&!require("node:fs").existsSync(out);at++)await sleep(50);
   const seen=JSON.parse(readFileSync(out,"utf8"));
   expect(seen.SEMIO_HOST_CAPACITY_GRANT).toBeUndefined();expect(seen.SEMIO_HOST_CAPACITY_PLAN).toBeUndefined();expect(seen.SEMIO_BUILD_GRAPH_PLAN_OUT).toBeUndefined();
   expect(seen.SEMIO_LANGUAGE).toBe("en");expect(seen.BUN_JSC_forceRAMSize).toBe(String(GRAPH_MEMORY_POLICY_V1.serviceHeapMegabytes*1048576));
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
 const launch=(implementation:string,env:Record<string,string>,policy:Record<string,unknown>={})=>spawnHostServiceV1({command:[process.execPath,MAIN,implementation,GRAPH,JSON.stringify({residentCheckMilliseconds:50,...policy})],environment:env});
 const ask=(root:string,implementation:string,env:Record<string,string>,agent:string,targets=["app:test"])=>requestPlanV1({root,implementation,targets,agent,language:"en",environment:env,spawnService:launch(implementation,env)});
 test("16 concurrent clients start exactly one daemon; kill -9 mid-life is recovered by the next request",async()=>{
  const env=environment("p"),implementation=digestOf("proc-1"),roots=Array.from({length:4},(_,at)=>workspace("pw"+at));
  const results=await Promise.all(Array.from({length:16},(_,at)=>ask(roots[at%4],implementation,env,"agent-"+at)));
  for(const result of results)checkPlan(result.plan.tasks,["app:test"]);
  const status=await requestHostStatusV1({implementation,environment:env});
  expect(new Set(results.map(r=>r.welcome.service.pid)).size).toBe(1);
  expect(status!.pid).toBe(results[0].welcome.service.pid);expect(status!.indexes).toBe(1);expect(status!.workspaces.length).toBe(4);
  expect(status!.rssMb).toBeLessThan(HOST_POLICY_V1.maximumResidentBytes/1048576);
  process.kill(status!.pid,"SIGKILL");
  await sleep(100);
  const after=await ask(roots[0],implementation,env,"agent-after");
  checkPlan(after.plan.tasks,["app:test"]);expect(after.welcome.service.pid).not.toBe(status!.pid);
  const stopped=await requestHostStatusV1({implementation,environment:env});
  process.kill(stopped!.pid,"SIGTERM");
  for(let at=0;at<100&&await requestHostStatusV1({implementation,environment:env});at++)await sleep(50);
  expect(await requestHostStatusV1({implementation,environment:env})).toBeUndefined();
 },60000);
 test("a client of another implementation digest starts a new daemon and the old one retires",async()=>{
  const env=environment("q"),root=workspace("qw"),old=digestOf("proc-old"),fresh=digestOf("proc-new"),policy={supersededGraceMilliseconds:100};
  const first=await requestPlanV1({root,implementation:old,targets:["core:build"],agent:"a",language:"en",environment:env,spawnService:launch(old,env,policy)});
  await sleep(20);
  const second=await requestPlanV1({root,implementation:fresh,targets:["core:build"],agent:"a",language:"en",environment:env,spawnService:launch(fresh,env,policy)});
  expect(second.welcome.service.pid).not.toBe(first.welcome.service.pid);
  let gone=false;
  for(let at=0;at<100&&!gone;at++){gone=(await requestHostStatusV1({implementation:old,environment:env}))===undefined;if(!gone)await sleep(50);}
  expect(gone).toBe(true);
  const left=await requestHostStatusV1({implementation:fresh,environment:env});process.kill(left!.pid,"SIGTERM");
 },60000);
});

describe("host service captures each plan",()=>{
 test("a slice plan reads one snapshot while the file changes, and its reply pins exactly the files the plan read",async()=>{
  const root=workspace("wcapture"),cas=createCasStoreV1({root:join(base,"cas-capture")});
  writeFileSync(join(root,"src.txt"),"one\n");writeFileSync(join(root,"other.txt"),"never read\n");
  const seen:string[]=[];
  const loadSlice=async()=>{
   seen.push(String(observedFs.readFileSync(join(root,"src.txt"),"utf8")));
   writeFileSync(join(root,"src.txt"),"two\n");
   seen.push(String(observedFs.readFileSync(join(root,"src.txt"),"utf8")));
   return{graph:JSON.parse(readFileSync(GRAPH,"utf8")),generationPin:"g1"};
  };
  const planner=createGraphPlannerPortV1({loadSlice,identify:async()=>"k".repeat(64),capture:createRevisionCaptureHookV1({cas})});
  const index=await planner.load({root,workspace:"wcapture"});
  const reply=await planner.plan({index:index as never,targets:["*:*"],root,workspace:"wcapture",agent:"agent-1",arguments:["run"],cursor:"c1"} as never);
  expect(seen).toEqual(["one\n","one\n"]);
  const world=await openPinnedWorldV1(cas,reply.revision);
  expect(world.manifest.entries.map(entry=>entry.path)).toEqual(["src.txt"]);
  expect(Buffer.from((await world.read("src.txt"))!).toString()).toBe("one\n");
  expect(reply.manifest).toBe(world.manifest.digest);expect(reply.pin!.revision).toBe(reply.revision);
 });
});

describe("host service reports recorded tasks up to date",()=>{
 test("a plan keys every task, a recorded key makes the task not stale, an edit of its inputs makes it and the tasks above it stale again",async()=>{
  const root=workspace("wkeys"),cas=createCasStoreV1({root:join(base,"cas-keys")}),recorder=createTaskRecorderV1({directory:join(base,"task-keys"),cas});
  const graph=JSON.parse(readFileSync(GRAPH,"utf8"));
  const files=new Map<string,string>();
  const derived=createDerivedGraphV1({rules:KIND_RULES_V1,world:{read:async(path:string)=>files.has(path)?Buffer.from(files.get(path)!):undefined,files:async()=>[...files.keys()]},environment:{},verifyObserved:verifyObservedV1});
  const planner=createGraphPlannerPortV1({identify:async()=>"k".repeat(64),loadGraph:async()=>graph,taskState:async()=>({graph:derived,recorder})});
  const index=await planner.load({root,workspace:"wkeys"}),ask=()=>planner.plan({index:index as never,targets:[...(index as {value:{tasks:Map<string,unknown>}}).value.tasks.keys()],root,workspace:"wkeys",agent:"a",revision:"r"} as never) as Promise<{tasks:{id:string;stale:boolean;key?:string|null;outputs:string[]}[]}>;
  const planned=await ask();
  expect(planned.tasks.length).toBeGreaterThan(0);expect(planned.tasks.every(task=>task.stale)).toBe(true);expect(planned.tasks.every(task=>/^[0-9a-f]{64}$/u.test(task.key??""))).toBe(true);
  for(const task of planned.tasks)for(const output of task.outputs){mkdirSync(dirname(join(root,output)),{recursive:true});writeFileSync(join(root,output),"built\n");}
  const missing=await ask();expect(missing.tasks.every(task=>task.stale),"nothing is recorded yet").toBe(true);
  await recorder.record("wkeys",planned.tasks.map(task=>({id:task.id,key:task.key!})));
  const again=await ask();expect(again.tasks.filter(task=>task.stale).map(task=>task.id)).toEqual([]);expect(again.tasks.map(task=>task.key)).toEqual(planned.tasks.map(task=>task.key));
  const lost=planned.tasks.find(task=>task.outputs.length>0);
  if(lost){rmSync(join(root,lost.outputs[0]!),{recursive:true,force:true});expect((await ask()).tasks.find(task=>task.id===lost.id)!.stale,"a task whose output is gone is stale although its key is recorded").toBe(true);}
 });
});
