/** 🔄️ Contract: a host service whose implementation is superseded by a newer sealable one drains (refuses new plans, lets the plans in flight finish for a bounded time) after its replacement started, and ends with the reason `reload`; newer code that cannot be sealed, or a replacement that does not start, keeps the running service and is reported in its status. The decision is replayed by an independent Python model.
 * @see ../../🔄️reload/🟨️.mjs, ../../../🧭️invocation/🔄️reload/🟦️.ts */
import {afterAll,describe,expect,test} from "bun:test";
import {EventEmitter} from "node:events";
import {createHash} from "node:crypto";
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {HERE,sleep} from "../🟦️.ts";
import {createHostServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {createHostReloadV1,replaceHostServiceV1} from "../../🔄️reload/🟨️.mjs";
import {requestHostStatusV1,requestPlanV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {validatorV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {hostReloadIdentityV1} from "../../../🧭️invocation/🔄️reload/🟦️.ts";
import {sealedEntryOfV1} from "../../../🏛️sealed/🟦️.ts";
import {resolveSealedImplementationV1} from "../../../🏛️sealed/📤️promotion/🟦️.ts";
import {fixtureCaptureV1,fixtureEntryV1,fixtureLoaderV1,makeFixtureWorkspaceV1,testPolicyV1} from "../../../🏛️sealed/🧪️tests/🧫️fixtures/🟦️.ts";

const GRAPH=join(HERE,"🧫️fixtures/🛰️host-graph.json"),CASES=join(HERE,"🔄️reload/🧫️cases.json"),ORACLE=join(HERE,"🔄️reload/🐍️oracle.py");
const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","hr-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const digestOf=(name:string)=>createHash("sha256").update(name).digest("hex");
const environment=(name:string)=>{const dir=join(base,"run-"+name);mkdirSync(dir,{recursive:true});return{...process.env,XDG_RUNTIME_DIR:dir,TMPDIR:dir,LOCALAPPDATA:dir} as Record<string,string>;};
const workspace=(name:string)=>{const root=join(base,name);mkdirSync(root,{recursive:true});copyFileSync(GRAPH,join(root,"graph.json"));return realpathSync(root);};
const reloadPolicy={checkMilliseconds:50,quietMilliseconds:0,maximumDeferralMilliseconds:0,drainStallMilliseconds:400,retryMilliseconds:100000,replacementMilliseconds:1000};

describe("the reload rule equals an independent model",()=>{
 test("every case of the shared table gives the trace of the Python model",async()=>{
  const spec=JSON.parse(readFileSync(CASES,"utf8")),model=JSON.parse(spawnSync(process.env.SEMIO_PYTHON??"python3",["-I",ORACLE,CASES],{encoding:"utf8"}).stdout) as Record<string,unknown[]>;
  for(const entry of spec.cases){
   let clock=0,replaced=0,drained=false,found=entry.ticks[0].identify;
   const reload=createHostReloadV1({implementation:spec.implementation,identify:()=>found,replace:async()=>{replaced++;return entry.replace;},drain:()=>{drained=true;},policy:spec.policy,now:()=>clock}),trace:unknown[]=[];
   for(const tick of entry.ticks){clock=tick.at;found=tick.identify;await reload.tick();trace.push([reload.state().state,replaced,drained]);}
   expect(trace,entry.name).toEqual(model[entry.name]!);
  }
 });
});

describe("a running host service reloads onto a newer implementation",()=>{
 const harness=(name:string,reload:Record<string,unknown>,loadMs=0,gate:()=>void=()=>undefined)=>{
  const env=environment(name),implementation=digestOf("old-"+name),runtime=hostEndpointsV1(implementation,env),events:string[]=[],graph=JSON.parse(readFileSync(GRAPH,"utf8"));
  const planner=createGraphPlannerPortV1({identify:async()=>"k".repeat(64),loadGraph:async()=>{gate();await sleep(loadMs);return graph;}});
  const service=createHostServiceV1({runtime,implementation,ports:{planner,reload:reload as never},policy:{residentCheckMilliseconds:5000,reload:reloadPolicy},log:event=>events.push(event)});
  const ask=(root:string)=>requestPlanV1({root,implementation,targets:["app:test"],agent:"a",language:"en",environment:env,spawnService:async()=>{await service.start();}});
  return{env,implementation,service,events,ask};
 };
 test("drift is detected, the replacement starts first, the plan in flight completes, then the service drains and ends with the reason reload",async()=>{
  let wanted="",resolveLoading:()=>void=()=>undefined;const loading=new Promise<void>(resolve=>{resolveLoading=resolve;}),order:string[]=[];
  const h=harness("drain",{identify:async()=>({implementation:wanted||h.implementation}),replace:async({implementation}:{implementation:string})=>{order.push("replace:"+implementation);return{started:true};}},200,()=>resolveLoading());
  await h.service.start();
  const plan=h.ask(workspace("w-drain"));
  await loading;wanted=digestOf("new-drain");
  const reason=await h.service.closed,answer=await plan;
  expect(reason).toBe("reload");
  expect(answer.plan.tasks.length).toBeGreaterThan(0);
  expect(order).toEqual(["replace:"+digestOf("new-drain")]);
  expect(h.events.indexOf("reload-draining")).toBeGreaterThanOrEqual(0);
  expect(h.events.indexOf("reload-draining")).toBeLessThan(h.events.indexOf("shutdown"));
  expect(await requestHostStatusV1({implementation:h.implementation,environment:h.env})).toBeUndefined();
 },30000);
 test("a plan that needs far longer than the stall bound runs to completion on the draining host, is computed once and never re-issued on the new identity",async()=>{
  let wanted="",loads=0,replaced=0;const events:string[]=[],env=environment("long"),implementation=digestOf("old-long"),runtime=hostEndpointsV1(implementation,env),graph=JSON.parse(readFileSync(GRAPH,"utf8"));
  const planner=createGraphPlannerPortV1({identify:async()=>"k".repeat(64),loadGraph:async()=>graph,loadSlice:async({onProgress}:any)=>{loads++;wanted=digestOf("new-long");for(let at=1;at<=15;at++){await sleep(100);onProgress(at*100);}return{graph,generationPin:"g"};}});
  const service=createHostServiceV1({runtime,implementation,ports:{planner,reload:{identify:async()=>({implementation:wanted||implementation}),replace:async()=>{replaced++;return{started:true};}}},policy:{residentCheckMilliseconds:5000,reload:reloadPolicy},log:event=>events.push(event)});
  await service.start();
  const started=Date.now(),answer=await requestPlanV1({root:workspace("w-long"),implementation,targets:["app:test"],agent:"a",language:"en",environment:env,arguments:["run"],attempts:1,spawnService:async()=>{await service.start();}});
  expect(Date.now()-started).toBeGreaterThan(1400);
  expect(answer.plan.tasks.length).toBeGreaterThan(0);
  expect(await service.closed).toBe("reload");expect(loads).toBe(1);expect(replaced).toBe(1);
  expect(events.indexOf("reload-draining")).toBeLessThan(events.indexOf("shutdown"));
 },30000);
 test("a plan that makes no progress for the stall bound is cut by the draining host",async()=>{
  let wanted="";const env=environment("stall"),implementation=digestOf("old-stall"),runtime=hostEndpointsV1(implementation,env),graph=JSON.parse(readFileSync(GRAPH,"utf8"));
  const planner=createGraphPlannerPortV1({identify:async()=>"k".repeat(64),loadGraph:async()=>graph,loadSlice:async({signal}:any)=>{wanted=digestOf("new-stall");await sleep(4000,signal).catch(()=>undefined);signal?.throwIfAborted();return{graph};}});
  const service=createHostServiceV1({runtime,implementation,ports:{planner,reload:{identify:async()=>({implementation:wanted||implementation}),replace:async()=>({started:true})}},policy:{residentCheckMilliseconds:5000,reload:reloadPolicy}});
  await service.start();
  const plan=requestPlanV1({root:workspace("w-stall"),implementation,targets:["app:test"],agent:"a",language:"en",environment:env,arguments:["run"],attempts:1,spawnService:async()=>{await service.start();}}).then(()=>"answered",()=>"cut"),began=Date.now();
  expect(await service.closed).toBe("reload");expect(Date.now()-began).toBeLessThan(3500);
  expect(await plan).toBe("cut");
 },30000);
 test("resident drain renews from a new worker observation while replayed detail only keeps the client connected",async()=>{
  const spec=JSON.parse(readFileSync(CASES,"utf8")),model=JSON.parse(spawnSync(process.env.SEMIO_PYTHON??"python3",["-I",ORACLE,CASES],{encoding:"utf8"}).stdout).drain as Record<string,string>;
  for(const row of spec.drain.cases){
   let wanted="",loads=0,frames=0;const env=environment(row.name),implementation=digestOf("old-"+row.name),runtime=hostEndpointsV1(implementation,env),graph=JSON.parse(readFileSync(GRAPH,"utf8"));
   const pool=createSlicePoolV1({workerUrl:new URL(import.meta.url),heartbeatMilliseconds:row.beatMs,planLimitMegabytes:Infinity,reaper:{reap:async()=>0,killNow:()=>undefined},
    createWorker:(()=>{
     loads++;const worker=new EventEmitter(),emit=(done:number)=>worker.emit("message",{type:"progress",phase:"taken",done,total:row.durationMs});
     const started=performance.now(),updates=row.advances?setInterval(()=>emit(Math.round(performance.now()-started)),row.beatMs):undefined;
     const first=setTimeout(()=>emit(1),0),last=setTimeout(()=>worker.emit("message",{type:"done",graph}),row.durationMs);
     return Object.assign(worker,{terminate:()=>{clearTimeout(first);clearTimeout(last);clearInterval(updates);}});
    }) as never});
   const planner=createGraphPlannerPortV1({identify:async()=>"k".repeat(64),loadGraph:async()=>graph,loadSlice:async({root,signal,onProgress})=>pool.run({root},{signal,onProgress})});
   const service=createHostServiceV1({runtime,implementation,ports:{planner,reload:{identify:async()=>({implementation:wanted||implementation}),replace:async()=>({started:true})}},policy:{residentCheckMilliseconds:5000,reload:{...reloadPolicy,drainStallMilliseconds:spec.drain.stallMs}}});
   try{
    await service.start();
    const answer=await requestPlanV1({root:workspace("w-"+row.name),implementation,targets:["app:test"],agent:"a",language:"en",environment:env,arguments:["run"],attempts:1,spawnService:async()=>{await service.start();},
     onProgress:frame=>{if(frame.phase==="slice"){frames++;wanted=digestOf("new-"+row.name);}}}).then(()=>"answered",()=>"cut");
    expect(answer,row.name).toBe(model[row.name]);
    expect(await service.closed).toBe("reload");expect(loads).toBe(1);expect(frames).toBeGreaterThan(2);
   }finally{await service.stop("test");await pool.close();}
  }
 },30000);
 test("newer code that cannot be sealed keeps the service answering and is reported in its status",async()=>{
  let replaced=0;
  const h=harness("blocked",{identify:async()=>({implementation:digestOf("new-blocked"),blocked:"entry.ts:3:1 syntax error / entry.ts:3:1 Syntaxfehler"}),replace:async()=>{replaced++;return{started:true};}});
  await h.service.start();
  await sleep(400);
  const status=await requestHostStatusV1({implementation:h.implementation,environment:h.env}) as any;
  expect(status.reload).toMatchObject({state:"blocked",next:digestOf("new-blocked")});expect(status.reload.reason).toContain("Syntaxfehler");
  expect(validatorV1("ServiceStatus")(status)).toEqual([]);
  expect(replaced).toBe(0);
  expect((await h.ask(workspace("w-blocked"))).plan.tasks.length).toBeGreaterThan(0);
  await h.service.stop("test");
 },30000);
 test("a replacement that does not start keeps the service, is reported and is not tried again before the retry delay",async()=>{
  let replaced=0;
  const h=harness("failed",{identify:async()=>({implementation:digestOf("new-failed")}),replace:async()=>{replaced++;return{started:false,reason:"exit 1: boom"};}});
  await h.service.start();
  await sleep(500);
  const status=await requestHostStatusV1({implementation:h.implementation,environment:h.env}) as any;
  expect(status.reload).toMatchObject({state:"failed",reason:"exit 1: boom"});expect(validatorV1("ServiceStatus")(status)).toEqual([]);
  expect(replaced).toBe(1);
  await h.service.stop("test");
 },30000);
});

describe("the replacement process",()=>{
 const fake=(code:number|undefined,output="")=>()=>{const child=new EventEmitter() as any;child.stdout=new EventEmitter();child.stderr=new EventEmitter();child.killed=false;child.kill=()=>{child.killed=true;};setTimeout(()=>{child.stderr.emit("data",output);if(code!==undefined)child.emit("close",code);},5);return child;};
 const wanted=digestOf("wanted"),replace=(spawnProcess:()=>unknown,status:()=>unknown,timeoutMilliseconds=200)=>replaceHostServiceV1({command:["repo","service","host","start"],environment:{},cwd:".",timeoutMilliseconds,status:status as never,spawnProcess:spawnProcess as never});
 test("it counts only a process that ended with status 0 while a service answers on the wanted implementation",async()=>{
  expect(await replace(fake(0),()=>({implementation:wanted}))({implementation:wanted})).toEqual({started:true});
  expect(await replace(fake(0),()=>({implementation:digestOf("other")}))({implementation:wanted})).toMatchObject({started:false});
  expect(await replace(fake(0),()=>undefined)({implementation:wanted})).toMatchObject({started:false});
  expect(await replace(fake(1,"consent denied"),()=>({implementation:wanted}))({implementation:wanted})).toEqual({started:false,reason:"exit 1: consent denied"});
 });
 test("a process that outlives the time is killed and reported",async()=>{
  const spawned:any[]=[],spawnProcess=()=>{const child=fake(undefined,"still starting")();spawned.push(child);return child;};
  const outcome=await replace(spawnProcess,()=>({implementation:wanted}),60)({implementation:wanted});
  expect(outcome.started).toBe(false);expect(outcome.reason).toContain("timeout");expect(spawned[0].killed).toBe(true);
 });
});

describe("the identity the tree wants",()=>{
 const running=digestOf("running"),promoted=async(label="A")=>{
  const root=makeFixtureWorkspaceV1(label),store=sealedEntryOfV1(root,fixtureEntryV1,testPolicyV1);
  const result=await resolveSealedImplementationV1(root,store,{capture:fixtureCaptureV1(root,fixtureEntryV1),loader:fixtureLoaderV1,policy:testPolicyV1,environment:{},write:()=>undefined});
  return{root,store,digest:result.pointer.digest};
 };
 const sealed=(root:string)=>hostReloadIdentityV1(root,{SEMIO_SEALED_DOOR_DIGEST:running},running,{entry:fixtureEntryV1});
 test("a sealed host follows the pointer of the store only while the snapshot, its loader and its receipt are intact",async()=>{
  const world=await promoted();
  expect(await sealed(world.root)()).toEqual({implementation:world.digest});
  expect(await hostReloadIdentityV1(world.root,{SEMIO_SEALED_DOOR_DIGEST:world.digest},world.digest,{entry:fixtureEntryV1})()).toEqual({implementation:world.digest});
  const receipt=world.store.receipt(world.digest),kept=readFileSync(receipt);
  rmSync(receipt);
  expect(await sealed(world.root)()).toMatchObject({implementation:world.digest,blocked:expect.stringContaining("Siegelquittung")});
  writeFileSync(receipt,kept);rmSync(world.store.snapshot(world.digest));writeFileSync(world.store.snapshot(world.digest),"damaged");
  expect(await sealed(world.root)()).toMatchObject({implementation:world.digest,blocked:expect.stringContaining("beschädigt")});
 });
 test("a sealed host without a published snapshot keeps its own implementation and says so",async()=>{
  const root=makeFixtureWorkspaceV1("A");
  expect(await sealed(root)()).toMatchObject({implementation:running,blocked:expect.stringContaining("kein versiegelter")});
 });
 test("a live host accepts a changed revision only when every file of the service parses",async()=>{
  const live=(revision:string,rows:readonly unknown[])=>hostReloadIdentityV1("/r",{},hostRunning,{revision:()=>revision,syntaxRows:async()=>rows as never});
  const hostRunning=createHash("sha256").update("live\0/r\0\0same").digest("hex");
  expect(await live("same",[])()).toEqual({implementation:hostRunning});
  const changed=await live("next",[])();expect(changed.implementation).not.toBe(hostRunning);expect(changed.blocked).toBeUndefined();
  expect(await live("next",[{path:"a/b.mjs",line:4,column:2,message:"Unexpected token"}])()).toMatchObject({blocked:"a/b.mjs:4:2 Unexpected token"});
 });
});
