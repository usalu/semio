import {afterAll,describe,expect,test} from "bun:test";
import {copyFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawn} from "node:child_process";
import {createHash} from "node:crypto";
import {HERE,sleep,waitFor} from "../🟦️.ts";
import {createHostServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {createReaperV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🧹️reap/🟨️.mjs";
import {createPressureWatchV1,cgroupMeasuresV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🌡️pressure/🟨️.mjs";
import {HOST_POLICY_V1,renderBothV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {createCgroupLimiterV1} from "../../../🧱️job-limits/🐧️cgroup/🟦️.ts";
import {hostAvailableBytesV1,sliceLimitPolicyV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs";
import {PlanRefusedErrorV1,requestPlanV1} from "../../🚪️client/🛰️plan/🟨️.mjs";

const FIXTURE=JSON.parse(readFileSync(join(HERE,"🧫️fixtures/🛑️cancel.json"),"utf8")),BOUND=FIXTURE.boundMilliseconds as number;
const GRAPH=join(HERE,"🧫️fixtures/🛰️host-graph.json"),CLIENT=join(HERE,"🛑️cancel/🛰️client.mjs");
const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","cx-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const alive=(pid:number)=>{try{process.kill(pid,0);}catch(error:any){return error?.code==="EPERM";}try{return !/^\d+ \(.*\) Z/u.test(readFileSync("/proc/"+pid+"/stat","utf8"));}catch{return true;}};
const digestOf=(name:string)=>createHash("sha256").update(name).digest("hex");
const workspace=(name:string)=>{const root=join(base,name);mkdirSync(root,{recursive:true});copyFileSync(GRAPH,join(root,"graph.json"));return realpathSync(root);};
const environment=(name:string)=>{const dir=join(base,"run-"+name);mkdirSync(dir,{recursive:true});return{...process.env,XDG_RUNTIME_DIR:dir,TMPDIR:dir,LOCALAPPDATA:dir} as Record<string,string>;};

describe("the host service cancels the job of a client that is gone",()=>{
 const harness=(name:string,bound=false)=>{
  const env=environment(name),implementation=digestOf("impl-"+name),runtime=hostEndpointsV1(implementation,env),root=workspace("w-"+name);
  const real=createGraphPlannerPortV1({identify:async()=>digestOf(name),loadGraph:async()=>JSON.parse(readFileSync(GRAPH,"utf8"))});
  const calls:{signal:AbortSignal;targets:string[]}[]=[];let release:()=>void=()=>{};const gate=new Promise<void>(resolve=>{release=resolve;});
  const digest=(path:string)=>digestOf(readFileSync(join(path,"graph.json"),"utf8")+(existsSync(join(path,"edit.txt"))?readFileSync(join(path,"edit.txt"),"utf8"):""));
  const binding=bound?{contentDigest:async({root}:any)=>digest(root),bind:async({root}:any)=>({pin:{version:1,revision:digestOf("rev"),holder:"h-"+digestOf(root).slice(0,16),expiresMs:1}})}:{};
  const planner={...real,...binding,plan:async(request:any)=>{
   calls.push({signal:request.signal,targets:request.targets});await request.progress?.(1);
   await new Promise<void>((resolve,reject)=>{gate.then(resolve);request.signal.addEventListener("abort",()=>reject(request.signal.reason),{once:true});});
   return real.plan(request);
  }};
  const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50}});
  const ask=(extra:Record<string,unknown>={})=>{
   const controller=new AbortController(),progress:any[]=[];
   const promise=requestPlanV1({root,implementation,targets:["app:test"],agent:"agent-"+Math.random(),language:"en",environment:env,spawnService:async()=>{await service.start();},signal:controller.signal,onProgress:(frame:any)=>progress.push(frame),...extra});
   return{controller,progress,promise};
  };
  return{env,implementation,root,service,calls,release,ask};
 };
 for(const row of FIXTURE.client as {name:string;signal:"SIGTERM"|"SIGKILL"}[])test(row.name,async()=>{
  const h=harness("c-"+row.signal);await h.service.start();
  const child=spawn(process.execPath,[CLIENT,h.root,h.implementation],{env:h.env,stdio:["ignore","pipe","inherit"]});
  const exited=new Promise<void>(resolve=>child.once("exit",()=>resolve()));
  expect(await waitFor(()=>h.calls.length===1,15000)).toBe(true);
  const started=performance.now();child.kill(row.signal);await exited;
  expect(await waitFor(()=>h.calls[0]!.signal.aborted,BOUND)).toBe(true);expect(performance.now()-started).toBeLessThan(BOUND);
  expect(await waitFor(()=>(h.service.status() as any).inFlight===0,BOUND)).toBe(true);
  await h.service.stop("test");
 },60000);

 test("two clients asking for the same plan share one job and every client gets progress and the result",async()=>{
  const h=harness("same");await h.service.start();const[a,b]=[h.ask(),h.ask()];
  expect(await waitFor(()=>h.calls.length>=1&&h.service.counters.coalesced===1,15000)).toBe(true);
  await sleep(200);expect(h.calls.length).toBe(FIXTURE.coalescing.identical.jobs);expect(h.service.counters.jobs).toBe(1);
  h.release();const[ra,rb]=await Promise.all([a.promise,b.promise]);
  expect(ra.reply.revision).toBe(rb.reply.revision);expect(ra.plan.tasks.length).toBeGreaterThan(0);
  expect(a.progress.some(f=>f.phase==="plan")).toBe(true);expect(b.progress.some(f=>f.phase==="plan")).toBe(true);
  await h.service.stop("test");
 },30000);

 test("different requests are different jobs",async()=>{
  const h=harness("diff");await h.service.start();const[first,second]=FIXTURE.coalescing.different.targets as string[][];
  const a=h.ask({targets:first}),b=h.ask({targets:second});
  expect(await waitFor(()=>h.calls.length===FIXTURE.coalescing.different.jobs,15000)).toBe(true);expect(h.service.counters.coalesced).toBe(0);
  h.release();await Promise.all([a.promise,b.promise]);await h.service.stop("test");
 },30000);

 test("two workspaces never share a job unless the planner binds each reply to its own workspace",async()=>{
  const h=harness("ws");await h.service.start();const other=workspace("w-ws-other");
  const a=h.ask(),b=h.ask({root:other});
  expect(await waitFor(()=>h.calls.length===FIXTURE.coalescing.otherWorkspace.jobs,15000)).toBe(true);expect(h.service.counters.coalesced).toBe(0);
  h.release();await Promise.all([a.promise,b.promise]);await h.service.stop("test");
 },30000);

 test("two workspaces with identical captured inputs share one job and each gets its own pin back",async()=>{
  const h=harness("bound",true);await h.service.start();const other=workspace("w-bound-other");
  const a=h.ask(),b=h.ask({root:other});
  expect(await waitFor(()=>h.service.counters.coalesced===FIXTURE.coalescing.boundContent.clients-1,15000)).toBe(true);
  await sleep(200);expect(h.calls.length).toBe(FIXTURE.coalescing.boundContent.jobs);
  h.release();const[ra,rb]=await Promise.all([a.promise,b.promise]);
  expect(ra.reply.pin.holder).toBe("h-"+digestOf(h.root).slice(0,16));expect(rb.reply.pin.holder).toBe("h-"+digestOf(other).slice(0,16));expect(ra.reply.pin.holder).not.toBe(rb.reply.pin.holder);
  expect(ra.plan.tasks).toEqual(rb.plan.tasks);await h.service.stop("test");
 },30000);

 test("two workspaces with different uncommitted edits get two jobs",async()=>{
  const h=harness("edits",true);await h.service.start();const other=workspace("w-edits-other");
  writeFileSync(join(other,"edit.txt"),"uncommitted\n");
  const a=h.ask(),b=h.ask({root:other});
  expect(await waitFor(()=>h.calls.length===FIXTURE.coalescing.differentEdits.jobs,15000)).toBe(true);expect(h.service.counters.coalesced).toBe(0);
  h.release();await Promise.all([a.promise,b.promise]);await h.service.stop("test");
 },30000);

 test("the same workspace at a different revision is another job",async()=>{
  const h=harness("rev");await h.service.start();
  const a=h.ask(),b=h.ask({revision:"b".repeat(64)});
  expect(await waitFor(()=>h.calls.length===FIXTURE.coalescing.otherRevision.jobs,15000)).toBe(true);expect(h.service.counters.coalesced).toBe(0);
  h.release();await Promise.allSettled([a.promise,b.promise]);await h.service.stop("test");
 },30000);

 test("cancelling one of two clients leaves the other and the job running",async()=>{
  const h=harness("first");await h.service.start();const[a,b]=[h.ask(),h.ask()];
  expect(await waitFor(()=>h.service.counters.coalesced===1,15000)).toBe(true);
  a.controller.abort(new Error("gone"));await expect(a.promise).rejects.toThrow("gone");
  await sleep(300);expect(h.calls[0]!.signal.aborted).toBe(FIXTURE.coalescing.firstCancelled.jobCancelled);
  h.release();expect((await b.promise).plan.tasks.length).toBeGreaterThan(0);
  await h.service.stop("test");
 },30000);

 test("the job is cancelled when both clients are gone, and the next identical request starts a fresh job",async()=>{
  const h=harness("both");await h.service.start();const[a,b]=[h.ask(),h.ask()];
  expect(await waitFor(()=>h.service.counters.coalesced===1,15000)).toBe(true);
  const started=performance.now();a.controller.abort(new Error("gone"));b.controller.abort(new Error("gone"));
  await Promise.allSettled([a.promise,b.promise]);
  expect(await waitFor(()=>h.calls[0]!.signal.aborted,BOUND)).toBe(FIXTURE.coalescing.bothCancelled.jobCancelled);expect(performance.now()-started).toBeLessThan(BOUND);
  expect(await waitFor(()=>(h.service.status() as any).inFlight===0,BOUND)).toBe(true);
  const c=h.ask();expect(await waitFor(()=>h.calls.length===2,15000)).toBe(true);h.release();await c.promise;
  await h.service.stop("test");
 },30000);
});

describe("processes a slice worker thread starts are reaped with the load",()=>{
 const worker=new URL("./🧒️spawning-worker.mjs",import.meta.url);
 for(const row of FIXTURE.slice as {name:string;mode:"abort"|"done"|"reported"|"busy"|"dead-service"}[])test(row.name,async()=>{
  if(row.mode==="dead-service"){
   const dead=spawn(process.execPath,["-e","0"]),deadPid=dead.pid!;await new Promise(resolve=>dead.once("exit",resolve));
   const orphan=spawn(process.execPath,["-e","setInterval(()=>{},1000)"],{stdio:"ignore",env:{...process.env,SEMIO_SLICE_JOB:deadPid+".1.orphan"}});
   expect(alive(orphan.pid!)).toBe(true);
   const pool=createSlicePoolV1({workerUrl:worker,maximum:1,graceMilliseconds:500});
   expect(await waitFor(()=>!alive(orphan.pid!),BOUND)).toBe(true);await pool.close();return;
  }
  const pidFile=join(base,"grand-"+row.mode+".pid");let child=0,job=0;const pool=createSlicePoolV1({workerUrl:worker,maximum:1,graceMilliseconds:500,heartbeatMilliseconds:100}),controller=new AbortController();
  const running=pool.run({root:base,mode:row.mode,pidFile},{signal:controller.signal,onProgress:(_ms:number,detail:any)=>{if(detail?.phase==="child"){child=detail.done;job=detail.total;}}}).catch(()=>undefined);
  expect(await waitFor(()=>child>0,15000)).toBe(true);expect(alive(child)).toBe(true);
  if(row.mode==="reported"){expect(await waitFor(()=>existsSync(pidFile)&&readFileSync(pidFile,"utf8")!=="",15000)).toBe(true);}
  const grand=row.mode==="reported"?Number(readFileSync(pidFile,"utf8")):0;
  const started=performance.now();if(row.mode!=="done")controller.abort(new Error("cancelled"));await running;
  expect(await waitFor(()=>!alive(child)&&!alive(job)&&(grand===0||!alive(grand)),BOUND)).toBe(true);expect(performance.now()-started).toBeLessThan(BOUND);
  await pool.close();
 },30000);
});

describe("the reaper over a modelled process table",()=>{
 test("a reported pid and its descendants get SIGINT then SIGKILL, unrelated processes are never touched",async()=>{
  const table=[{pid:10,ppid:1},{pid:11,ppid:10},{pid:12,ppid:11},{pid:20,ppid:1},{pid:30,ppid:1,token:"1.1.abc"}],sent:string[]=[];
  const reaper=createReaperV1({list:()=>table,send:(pid:number,signal:string)=>sent.push(pid+":"+signal),graceMilliseconds:100,alive:()=>true});
  expect(await reaper.reap({pids:[10],match:row=>row.token==="1.1.abc"})).toBe(4);
  expect(sent.filter(row=>row.endsWith("SIGINT")).map(row=>row.split(":")[0]).sort()).toEqual(["10","11","12","30"]);
  expect(sent.filter(row=>row.endsWith("SIGKILL")).map(row=>row.split(":")[0]).sort()).toEqual(["10","11","12","30"]);
  expect(sent.some(row=>row.startsWith("20:"))).toBe(false);
 });
});

describe("sustained memory throttling cancels the load instead of crawling",()=>{
 for(const row of FIXTURE.pressure.sequences as {name:string;samples:any[];tripsAtSecond:number|null}[])test(row.name,()=>{
  let clock=0,at=0;const watch=createPressureWatchV1({measures:()=>row.samples[at]??row.samples.at(-1),policy:FIXTURE.pressure.policy,now:()=>clock});
  let tripped:number|null=null;
  for(;at<row.samples.length;at++){clock=at*1000;if(watch.sample().tripped){tripped=at;break;}}
  expect(tripped).toBe(row.tripsAtSecond);
 });
 test("the schema declares the thresholds and the refusals in both languages",()=>{
  expect(Object.keys(HOST_POLICY_V1.memoryPressure).sort()).toEqual(Object.keys(FIXTURE.pressure.policy).sort());
  for(const kind of["psi","events","free"]){const both=renderBothV1("host-memory-pressure-"+kind,{value:1,limit:2,seconds:3})!;expect(both.en).toMatch(/cancelled/u);expect(both.de).toMatch(/abgebrochen/u);}
 });
 test("the cgroup reader takes PSI and the high counter from the service cgroup and falls back to free memory without one",()=>{
  const files:Record<string,string>={"/c/svc/memory.events":"low 0\nhigh 77\nmax 0\n","/c/svc/memory.pressure":"some avg10=3.00 avg60=1 avg300=1 total=9\nfull avg10=12.50 avg60=1 avg300=1 total=9\n"};
  const read=(path:string)=>{if(path in files)return files[path]!;throw new Error("missing");};
  expect(cgroupMeasuresV1({root:"/c",read,own:()=>"/svc",free:()=>5})()).toEqual({full:12.5,high:77});
  delete files["/c/svc/memory.pressure"];expect(cgroupMeasuresV1({root:"/c",read,own:()=>"/svc",free:()=>5})()).toEqual({high:77});
  expect(cgroupMeasuresV1({root:"/nowhere",read:()=>{throw new Error("none");},own:()=>"",free:()=>5})()).toEqual({freeBytes:5});
 });
 test("a load under sustained pressure is cancelled, its processes are reaped and the host refuses with memory and retry later in both languages",async()=>{
  const worker=new URL("./🧒️spawning-worker.mjs",import.meta.url);let clock=0,child=0;
  const pool=createSlicePoolV1({workerUrl:worker,maximum:1,graceMilliseconds:500,heartbeatMilliseconds:100,sampleMilliseconds:50,pressure:()=>createPressureWatchV1({measures:()=>({full:90}),policy:{...FIXTURE.pressure.policy,sustainedSeconds:1},now:()=>(clock+=100)})});
  const env=environment("pressure"),implementation=digestOf("impl-pressure"),runtime=hostEndpointsV1(implementation,env),root=workspace("w-pressure");
  const real=createGraphPlannerPortV1({identify:async()=>digestOf("pressure"),loadGraph:async()=>JSON.parse(readFileSync(GRAPH,"utf8"))});
  const planner={...real,plan:async(request:any)=>{await pool.run({root:base,mode:"abort"},{signal:request.signal,onProgress:(_ms:number,detail:any)=>{if(detail?.phase==="child"){child=detail.done;}}});return real.plan(request);}};
  const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50}});await service.start();
  const failure=await requestPlanV1({root,implementation,targets:["app:test"],agent:"a",language:"en",environment:env}).then(()=>undefined,error=>error);
  expect(failure).toBeInstanceOf(PlanRefusedErrorV1);expect(failure.code).toBe("memory");expect(failure.retry).toBe("later");
  expect(failure.message_.en).toMatch(/memory pressure/u);expect(failure.message_.de).toMatch(/Speicherdruck/u);
  expect(child).toBeGreaterThan(0);expect(await waitFor(()=>!alive(child),BOUND)).toBe(true);
  await pool.close();await service.stop("test");
 },30000);
});

describe("the allocator purge setting of the policy reaches every process the service starts",()=>{
 test("the schema declares it in both languages and a slice worker's environment carries it next to the token",async()=>{
  const purge=HOST_POLICY_V1.memoryPurge;
  expect(purge.environmentVariable).toBe("MIMALLOC_PURGE_DELAY");expect(purge.delayMilliseconds).toBe(0);expect(purge.description.en.length).toBeGreaterThan(20);expect(purge.description.de.length).toBeGreaterThan(20);expect(purge.description.de).not.toBe(purge.description.en);
  let seen:Record<string,string>|undefined;
  const listeners:Record<string,(value:unknown)=>void>={},fake={on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0};
  const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,createWorker:(_url:URL,_data:unknown,env:Record<string,string>)=>{seen=env;queueMicrotask(()=>listeners.message?.({type:"done",graph:{}}));return fake;}});
  await pool.run({root:base});
  expect(seen![purge.environmentVariable]).toBe(String(purge.delayMilliseconds));expect(seen!.SEMIO_SLICE_JOB).toMatch(/^[0-9]+\.1\.[a-z0-9]+$/u);expect(seen!.PATH).toBe(process.env.PATH);
  await pool.close();
 });
});

describe("a large slice result crosses the process boundary as a file",()=>{
 test("a result of several megabytes arrives intact and its spool directory is removed",async()=>{
  const{readdirSync}=await import("node:fs"),{scratchHomeV1}=await import("../../../../../../../../🔨️modules/🏃️process/🧹️scratch/🟦️.ts"),spools=()=>{try{return readdirSync(scratchHomeV1()).filter(name=>/^[0-9]+-slice-/u.test(name)).length;}catch{return 0;}},before=spools();
  const pool=createSlicePoolV1({workerUrl:new URL("./🧒️spawning-worker.mjs",import.meta.url),maximum:1,graceMilliseconds:200});
  const result=await pool.run({root:base,mode:"big"});
  expect(Object.keys(result.graph.projects).length).toBe(60000);expect(result.graph.projects.p59999.root).toBe("libs/p59999");expect(result.generationPin).toBe("big");
  expect(await waitFor(()=>spools()<=before,5000)).toBe(true);await pool.close();
 },30000);
});

describe("every slice load gets a hard memory limit of its own where cgroups are delegated",()=>{
 const MiB=1048576,L=FIXTURE.limit;
 const modelled=()=>{
  const files=new Map<string,string>(),removed:string[]=[];
  const ports={root:"/cg",read:(path:string)=>{if(!files.has(path))throw Object.assign(new Error("ENOENT"),{code:"ENOENT"});return files.get(path)!;},write:(path:string,text:string)=>{files.set(path,text);},mkdir:(_path:string)=>undefined,rmdir:(path:string)=>{removed.push(path);},list:()=>[],writable:()=>true,own:()=>"/svc",sleep:async()=>undefined,pid:7};
  return{files,removed,ports};
 };
 const headroom=()=>({limitBytes:L.headroom.limitMegabytes*MiB,currentBytes:L.headroom.currentMegabytes*MiB});
 test("the policy is the service limit minus its usage minus the margin, a hard kill at the limit (no memory.high throttle), no swap, never below the floor, none without a limit",()=>{
  const policy=sliceLimitPolicyV1(headroom(),L.marginMegabytes)!;
  expect(policy.memoryMegabytes).toBe(L.expect.memoryMaxMegabytes);expect(policy.kind).toBe("job");expect(policy.swapMegabytes).toBe(0);
  expect(sliceLimitPolicyV1({limitBytes:4096*MiB,currentBytes:4090*MiB},100)!.memoryMegabytes).toBe(L.floorMegabytes);
  expect(sliceLimitPolicyV1({limitBytes:Infinity,currentBytes:1})).toBeUndefined();
 });
 test("a host with memory to spare lends the load a bounded share above the service limit, and never less than the limit leaves",()=>{
  const elasticity={maximumMegabytes:8192,hostShareBasisPoints:7500,hostFloorMegabytes:1536},at=(availableMegabytes:number|undefined)=>sliceLimitPolicyV1({limitBytes:4096*MiB,currentBytes:300*MiB,availableBytes:availableMegabytes===undefined?undefined:availableMegabytes*MiB},256,elasticity)!.memoryMegabytes;
  expect(at(undefined)).toBe(3540);expect(at(2000)).toBe(3540);expect(at(12000)).toBe(7464);expect(at(20000)).toBe(8192);
  expect(sliceLimitPolicyV1({limitBytes:4096*MiB,currentBytes:300*MiB,availableBytes:20000*MiB},256)!.memoryMegabytes).toBe(3540);
 });
 test("a load is never reserved less than the peak earlier loads needed, however little the host has available at its start, and never more than the elasticity maximum",()=>{
  const elasticity={maximumMegabytes:8192,hostShareBasisPoints:7500,hostFloorMegabytes:1536},at=(availableMegabytes:number,learned:number)=>sliceLimitPolicyV1({limitBytes:4096*MiB,currentBytes:300*MiB,availableBytes:availableMegabytes*MiB},256,elasticity,learned)!.memoryMegabytes;
  expect(at(2000,0)).toBe(3540);expect(at(2000,6000)).toBe(6000);expect(at(12000,6000)).toBe(7464);expect(at(2000,20000)).toBe(8192);
 });
 test("a plan reserves its limit through the capacity gate before its worker starts, attaches the worker to the reservation and gives it back when the load ends, even when the load fails",async()=>{
  const log:string[]=[],listeners:Record<string,(value:unknown)=>void>={},fake={pid:4242,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0},limiter={create:async()=>({attach:async()=>undefined,onBreach:()=>undefined,release:async()=>undefined})};
  const harness=(answer:"done"|"error")=>createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,headroom,marginMegabytes:L.marginMegabytes,limiter:limiter as never,learned:{read:()=>0,write:()=>undefined},reserve:async({megabytes}:{megabytes:number})=>{log.push("reserve "+megabytes);return{attach:(pid:number)=>{log.push("attach "+pid);},release:()=>{log.push("release");}};},createWorker:()=>{log.push("worker");queueMicrotask(()=>listeners.message?.(answer==="done"?{type:"done",graph:{}}:{type:"error",message:"broken"}));return fake;}});
  const pool=harness("done");await pool.run({root:base});await pool.close();await waitFor(()=>log.at(-1)==="release",5000);
  expect(log).toEqual(["reserve "+L.expect.memoryMaxMegabytes,"worker","attach 4242","release"]);
  log.length=0;const failing=harness("error");await expect(failing.run({root:base})).rejects.toThrow("broken");await failing.close();await waitFor(()=>log.at(-1)==="release",5000);
  expect(log).toEqual(["reserve "+L.expect.memoryMaxMegabytes,"worker","attach 4242","release"]);
 });
 test("a plan that waits for its reservation reports the wait, starts no worker while it waits, and a cancellation during the wait ends it without a worker",async()=>{
  const log:string[]=[],progress:any[]=[],controller=new AbortController();let started=0;
  const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,heartbeatMilliseconds:20,headroom:()=>({limitBytes:L.headroom.limitMegabytes*MiB,currentBytes:L.headroom.currentMegabytes*MiB,availableBytes:1024*MiB}),marginMegabytes:L.marginMegabytes,limiter:{create:async()=>undefined} as never,reserve:({signal}:{signal:AbortSignal})=>new Promise((_resolve,reject)=>{signal.addEventListener("abort",()=>reject(signal.reason),{once:true});}),createWorker:()=>{started++;throw new Error("no worker may start");}});
  const running=pool.run({root:base},{signal:controller.signal,onProgress:(_ms:number,detail:any)=>progress.push(detail)}).then(()=>undefined,error=>error);
  expect(await waitFor(()=>progress.length>=2,5000)).toBe(true);
  expect(progress[0]).toMatchObject({phase:"reservation",done:0,total:L.expect.memoryMaxMegabytes,decision:{key:"waiting",used:L.expect.memoryMaxMegabytes,budget:1024}});
  controller.abort(new Error("stop waiting"));expect((await running).message).toBe("stop waiting");expect(started).toBe(0);expect(log).toEqual([]);await pool.close();
 });
 test("the host availability reads MemAvailable in kilobytes",()=>{expect(hostAvailableBytesV1(()=>"MemTotal: 1 kB\nMemAvailable:    2048 kB\n")).toBe(2048*1024);expect(hostAvailableBytesV1(()=>{throw Error("none");})).toBeGreaterThan(0);});
 test("a load is placed in its own cgroup with the computed limits and the group is killed and removed when the load ends",async()=>{
  const m=modelled(),pool=createSlicePoolV1({workerUrl:new URL("./🧒️spawning-worker.mjs",import.meta.url),maximum:1,graceMilliseconds:200,headroom,marginMegabytes:L.marginMegabytes,limiter:createCgroupLimiterV1("/cg/semio-jobs",m.ports)});
  await pool.run({root:base,mode:"done"});
  const dir=[...m.files.keys()].find(path=>path.endsWith("/memory.max"))!.replace("/memory.max","");
  expect(Number(m.files.get(dir+"/memory.max"))).toBe(L.expect.memoryMaxMegabytes*MiB);expect(m.files.has(dir+"/memory.high")).toBe(false);expect(m.files.get(dir+"/memory.swap.max")).toBe("0");expect(m.files.get(dir+"/memory.oom.group")).toBe("1");
  expect(Number(m.files.get(dir+"/cgroup.procs"))).toBeGreaterThan(0);
  expect(await waitFor(()=>m.files.get(dir+"/cgroup.kill")==="1"&&m.removed.includes(dir),BOUND)).toBe(true);await pool.close();
 },30000);
 test("an overrun kills only that load: typed memory-budget error, process gone, group killed",async()=>{
  const m=modelled();let child=0,job=0;
  const pool=createSlicePoolV1({workerUrl:new URL("./🧒️spawning-worker.mjs",import.meta.url),maximum:1,graceMilliseconds:200,heartbeatMilliseconds:100,headroom,marginMegabytes:L.marginMegabytes,limiter:createCgroupLimiterV1("/cg/semio-jobs",m.ports)});
  const running=pool.run({root:base,mode:"abort"},{onProgress:(_ms:number,detail:any)=>{if(detail?.phase==="child"){child=detail.done;job=detail.total;}}}).then(()=>undefined,error=>error);
  expect(await waitFor(()=>child>0,15000)).toBe(true);
  const dir=[...m.files.keys()].find(path=>path.endsWith("/memory.max"))!.replace("/memory.max","");
  m.files.set(dir+"/memory.events","low 0\nhigh 0\nmax 3\noom 1\noom_kill 1\n");m.files.set(dir+"/memory.current",String(L.expect.memoryMaxMegabytes*MiB));
  const failure=await running;
  expect(failure.code).toBe("memory-budget");expect(failure.limitMegabytes).toBe(L.expect.memoryMaxMegabytes);
  expect(await waitFor(()=>!alive(job)&&!alive(child)&&m.files.get(dir+"/cgroup.kill")==="1",BOUND)).toBe(true);await pool.close();
 },30000);
});
