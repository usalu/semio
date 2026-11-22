import {afterEach,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {join} from "node:path";
import {testScratchRootV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {ensureFleetIdentityV1} from "../../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {admitRemoteExecutionActionV1,buildRemoteExecutionLoadReportV1,type RemoteExecutionHostViewV1,type RemoteExecutionLoadReportV1} from "../../🌡️load/🟦️.ts";
import {remoteExecutionWorkerWhyOfMessageV1,remoteExecutionWorkerWhyV1,type RemoteExecutionWorkerWhyCodeV1} from "../../🌡️load/🔎️why/🟦️.ts";
import {RemoteExecutionSchedulerV1} from "../../🗓️scheduler/🟦️.ts";
import {runRemoteExecutionAgentV1} from "../../🛰️agent/🟦️.ts";
import {checkRemoteExecutionAgentReadinessV1} from "../../🛰️agent/🩺️readiness/🟦️.ts";
import {readRemoteExecutionPolicyV1,requireRemoteExecutionShapeV1} from "../../🟦️.ts";

const scratch:string[]=[],servers:{stop:(force?:boolean)=>void}[]=[];
afterEach(()=>{for(const server of servers.splice(0))server.stop(true);for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const temporary=():string=>{const path=mkdtempSync(join(testScratchRootV1("rexec-why"),"scratch-"));scratch.push(path);return path;};
const CODES:readonly RemoteExecutionWorkerWhyCodeV1[]=["starting","no-scheduler","scheduler-unreachable","identity-unregistered","no-toolchain","provisioning","sandbox-unavailable","disk-full","policy-off","not-registered","no-headroom","failed"];
const view:RemoteExecutionHostViewV1={host:{version:1,atMs:Date.now()+86_400_000,platform:"darwin",cores:8,totalMegabytes:65536,availableMegabytes:60000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0.5,openFiles:1000,openFilesMaximum:200000,attachedImages:0},grants:[],waiters:[]} as never;
const toolchain="rustc 1.0.0-nightly (abcdef 2026-07-20)\nhost: aarch64-apple-darwin\n";

async function fakeScheduler(workerStatus:number){
 const reports:RemoteExecutionLoadReportV1[]=[],workerRequests:string[]=[];
 const server=Bun.serve({port:0,hostname:"127.0.0.1",async fetch(request){
  const path=new URL(request.url).pathname;
  if(path==="/v1/fleet/load"){reports.push(await request.json() as RemoteExecutionLoadReportV1);return new Response("{}",{status:200});}
   workerRequests.push(path);
  return new Response("Forbidden: this worker belongs to another origin",{status:workerStatus});
 }});
 servers.push(server);return {url:`http://127.0.0.1:${server.port}`,reports,workerRequests};
}
const agentOptions=(root:string,url:string,signal:AbortSignal)=>({root,environment:{SEMIO_UNIT_CACHE_TOOLCHAIN:toolchain,SEMIO_FLEET_ORIGIN:"g1-test"},origin:"g1-test",scheduler:url,concurrency:1,directory:join(root,"work"),sandbox:"isolated-root" as const,materialize:"copy" as const,maximumBlobBytes:1048576,inlineOutputBytes:0,loadIntervalSeconds:1,signal,view:async()=>view});
async function until<T>(read:()=>T|undefined,milliseconds=15000):Promise<T>{const end=Date.now()+milliseconds;for(;;){const value=read();if(value!==undefined)return value;if(Date.now()>end)throw Error("not reached");await Bun.sleep(50);}}

test("every reason has an English and a German sentence without leftovers, a detail is bounded and kept to one line, and the closed shape accepts it",()=>{
 for(const code of CODES){
  const why=remoteExecutionWorkerWhyV1(code,"line one\nline two "+"x".repeat(400));
  expect(why.code).toBe(code);expect(why.en.length).toBeGreaterThan(8);expect(why.de.length).toBeGreaterThan(8);
  expect(why.en+why.de).not.toMatch(/\{detail\}|\n/u);expect(why.en.length).toBeLessThanOrEqual(400);expect(why.de.length).toBeLessThanOrEqual(400);
  const report=buildRemoteExecutionLoadReportV1({origin:"g1-test",identity:{key:"macos-arm64-none",os:"macos",arch:"arm64",toolchain:null,target:null},view,worker:{state:"absent",concurrency:1,running:0,sandbox:null,confinement:null,network:null,why},counters:{unit:{localHit:0,remoteHit:0,miss:0,stored:0},offload:{out:0,refused:0,fellBack:0}},nowMs:Date.now()});
  expect(report.worker.why).toEqual(why);
 }
 expect(remoteExecutionWorkerWhyV1("policy-off").en).toBe("the worker policy runs no actions (worker.actions is none)");
 expect(()=>requireRemoteExecutionShapeV1("WorkerWhy",{code:"bogus",en:"a",de:"b"},"closed")).toThrow();
});

test("what a worker says when it cannot join is classified: a refused signature, a missing confinement, a failed connection, anything else",()=>{
 const code=(message:string)=>remoteExecutionWorkerWhyOfMessageV1(message).code;
 expect(code("Scheduler answered 403: Forbidden: this worker belongs to another origin")).toBe("identity-unregistered");
 expect(code("Scheduler answered 401: unauthorized")).toBe("identity-unregistered");
 expect(code("Confinement required but this host has no confining sandbox (only isolated-root)")).toBe("sandbox-unavailable");
 expect(code("Unable to connect. Is the computer able to access the url?")).toBe("scheduler-unreachable");
 expect(code("The socket connection was closed unexpectedly.")).toBe("scheduler-unreachable");
 expect(code("Scheduler answered 500: boom")).toBe("failed");
});

test("readiness names the first thing that is wrong: policy, worker key, confinement; free disk is no longer a reason to stay out of the scheduler",async()=>{
 const root=temporary(),policy={...readRemoteExecutionPolicyV1().worker,sandbox:"isolated-root" as const,confinement:"best-effort" as const};
 const check=(over:object={})=>checkRemoteExecutionAgentReadinessV1({root,policy:{...policy,...over} as never})?.code;
 expect(check({actions:"none"})).toBe("policy-off");
 expect(check()).toBe("identity-unregistered");
 await ensureFleetIdentityV1({root,originId:"g1-test",group:"g1"});
 expect(check()).toBeUndefined();
 expect(check({confinement:"required"})).toBe("sandbox-unavailable");
});

test("no platform is refused by its name: the readiness of a worker never depends on the operating system, so a Windows machine reports like any other and the platform key alone decides who may use it",()=>{
 const root=temporary(),policy={...readRemoteExecutionPolicyV1().worker,actions:"rustc" as const,sandbox:"isolated-root" as const,confinement:"best-effort" as const};
 return ensureFleetIdentityV1({root,originId:"g1-test",group:"g1"}).then(()=>{
  expect(checkRemoteExecutionAgentReadinessV1({root,policy})).toBeUndefined();
  expect(CODES).not.toContain("action-unsupported" as never);
  expect(()=>remoteExecutionWorkerWhyV1("action-unsupported" as never)).toThrow();
 });
});

test("a worker the scheduler refuses reports itself absent with the reason, in every report, and the agent keeps reporting its load",async()=>{
 const root=temporary(),fake=await fakeScheduler(403),controller=new AbortController();
 const agent=runRemoteExecutionAgentV1(agentOptions(root,fake.url,controller.signal));
 const reports=fake.reports;
 const refused=await until(()=>reports.find(row=>row.worker.why?.code==="identity-unregistered"));
 expect(refused.worker.state).toBe("absent");expect(refused.worker.why!.en).toContain("403");expect(refused.worker.why!.de).toContain("403");
 const first=reports[0]!;expect(first.worker.state).toBe("absent");expect(first.worker.why?.code).toBe("starting");
 for(const row of reports)requireRemoteExecutionShapeV1("LoadReport",row,"closed");
 const count=reports.length;await until(()=>reports.length>count+1?true:undefined);
 controller.abort();await agent.catch(()=>undefined);
});

test("an agent started without a worker reports the reason it was given; with a retry a worker that cannot start leaves the agent reporting instead of ending it",async()=>{
 const root=temporary(),fake=await fakeScheduler(200),controller=new AbortController();
 const off=runRemoteExecutionAgentV1({...agentOptions(root,fake.url,controller.signal),worker:false,why:remoteExecutionWorkerWhyV1("disk-full","1.0 GiB free, floor 20 GiB")});
 await until(()=>fake.reports.length>=2?true:undefined);
 for(const row of fake.reports)expect(row).toMatchObject({worker:{state:"absent",why:{code:"disk-full"}}});
 controller.abort();await off;
 const second=await fakeScheduler(200),again=new AbortController();
 const retrying=runRemoteExecutionAgentV1({...agentOptions(root,second.url,again.signal),confinement:"required",retryWorkerSeconds:30});
 const row=await until(()=>second.reports.find(report=>report.worker.why?.code==="sandbox-unavailable"));
 expect(row.worker.state).toBe("absent");
 const count=second.reports.length;await until(()=>second.reports.length>count?true:undefined);
 again.abort();await retrying;
 const plain=new AbortController();
 await expect(runRemoteExecutionAgentV1({...agentOptions(root,second.url,plain.signal),confinement:"required"})).rejects.toThrow(/Confinement required/u);
});

test("the scheduler row of a machine whose worker is not registered says why: the machine's own reason, else not-registered; a registered worker drops the reason",()=>{
 let now=1_000_000;
 const scheduler=new RemoteExecutionSchedulerV1({content:{} as never,unmatchedSeconds:30,lingerSeconds:0.05,leaseSeconds:10,maximumAttempts:3,defaultActionSeconds:60,maximumActionSeconds:600,maximumQueued:100,heartbeatSeconds:2,pollSeconds:5,loadExpireSeconds:15,refusalCooldownSeconds:2,cache:{url:"http://c",relative:false},language:"none",sweepMilliseconds:0,now:()=>now});
 const report=(origin:string,worker:object):RemoteExecutionLoadReportV1=>buildRemoteExecutionLoadReportV1({origin,identity:{key:"macos-arm64-none",os:"macos",arch:"arm64",toolchain:null,target:null},view,worker:{state:"idle",concurrency:2,running:0,sandbox:"isolated-root",confinement:"none",network:"unenforced",...worker} as never,counters:{unit:{localHit:0,remoteHit:0,miss:0,stored:0},offload:{out:0,refused:0,fellBack:0}},nowMs:now});
 scheduler.acceptLoad(report("a",{}));
 scheduler.acceptLoad(report("b",{state:"absent",why:remoteExecutionWorkerWhyV1("disk-full","1 GiB")}));
 scheduler.acceptLoad(report("c",{state:"absent",why:remoteExecutionWorkerWhyV1("starting")}));
 scheduler.acceptLoad(report("d",{state:"refusing",why:remoteExecutionWorkerWhyV1("no-headroom","memory")}));
 scheduler.register({version:1,name:"c",host:"h",platform:"darwin-arm64",properties:[{name:"OSFamily",value:"macos"}],concurrency:1,sandbox:"isolated-root",runtime:"t",origin:"c",capacity:{cores:8,memoryMegabytes:16384},confinement:"none",network:"unenforced"});
 const table=scheduler.fleetLoad() as {machines:{origin:string;worker:{state:string;why?:{code:string}}}[]};
 requireRemoteExecutionShapeV1("FleetLoad",table,"closed");
 const row=(origin:string)=>table.machines.find(machine=>machine.origin===origin)!.worker;
 expect(row("a")).toMatchObject({state:"absent",why:{code:"not-registered"}});
 expect(row("d"),"a refusing machine whose worker the scheduler does not know is not-registered, not no-headroom").toMatchObject({state:"absent",why:{code:"not-registered"}});
 expect(row("b")).toMatchObject({state:"absent",why:{code:"disk-full"}});
 expect(row("c").state).toBe("idle");expect(row("c").why).toBeUndefined();
});

test("a machine that is over its own admission reports its worker as refusing with the admission's reason, and a registered worker keeps that reason in the scheduler row",()=>{
 let now=1_000_000;
 const scheduler=new RemoteExecutionSchedulerV1({content:{} as never,unmatchedSeconds:30,lingerSeconds:0.05,leaseSeconds:10,maximumAttempts:3,defaultActionSeconds:60,maximumActionSeconds:600,maximumQueued:100,heartbeatSeconds:2,pollSeconds:5,loadExpireSeconds:15,refusalCooldownSeconds:2,cache:{url:"http://c",relative:false},language:"none",sweepMilliseconds:0,now:()=>now});
 const tight={...view,host:{...view.host,availableMegabytes:1000}} as RemoteExecutionHostViewV1;
 const report=buildRemoteExecutionLoadReportV1({origin:"a",identity:{key:"macos-arm64-none",os:"macos",arch:"arm64",toolchain:null,target:null},view:tight,worker:{state:"idle",concurrency:2,running:0,sandbox:"isolated-root",confinement:"none",network:"unenforced"},counters:{unit:{localHit:0,remoteHit:0,miss:0,stored:0},offload:{out:0,refused:0,fellBack:0}},nowMs:now});
 expect(report.headroom).toBe(false);expect(report.worker).toMatchObject({state:"refusing",why:{code:"no-headroom"}});expect(report.worker.why!.en).toMatch(/\((memory|pressure-warn|pressure-critical|load)\)/u);
 scheduler.acceptLoad(report);
 scheduler.register({version:1,name:"a",host:"h",platform:"darwin-arm64",properties:[{name:"OSFamily",value:"macos"}],concurrency:2,sandbox:"isolated-root",runtime:"t",origin:"a",capacity:{cores:8,memoryMegabytes:16384},confinement:"none",network:"unenforced"});
 const row=(scheduler.fleetLoad() as {machines:{origin:string;worker:{state:string;why?:{code:string}}}[]}).machines[0]!.worker;
 expect(row).toMatchObject({state:"refusing",why:{code:"no-headroom"}});
});

test("a machine lends nothing while a request of its own waits, and lends again when nothing waits",()=>{
 const nowMs=Date.now(),local={request:{id:"localaaaa1",operation:"run",priority:"batch" as const,minimumJobs:1,preferredJobs:1,fixedMegabytes:65000,perJobMegabytes:0,cpuPerJob:1},arrivedAtMs:nowMs-1000,pid:1};
 expect(admitRemoteExecutionActionV1({view,running:0,concurrency:4,nowMs})).toEqual({admit:true});
 expect(admitRemoteExecutionActionV1({view:{...view,waiters:[local]} as never,running:0,concurrency:4,nowMs})).toEqual({admit:false,reason:"queue-order"});
 expect(admitRemoteExecutionActionV1({view:{...view,waiters:[]} as never,running:0,concurrency:4,nowMs})).toEqual({admit:true});
});

test("a 16 GB machine with free memory keeps lending beside its running remote actions: what they hold is in the sample already and counts once, and a machine whose room is really used up still refuses",()=>{
 const nowMs=Date.now(),sixteen=(availableMegabytes:number):RemoteExecutionHostViewV1=>({...view,host:{...view.host,cores:10,totalMegabytes:16384,availableMegabytes}}) as RemoteExecutionHostViewV1;
 expect(admitRemoteExecutionActionV1({view:sixteen(6000),running:0,concurrency:10,nowMs})).toEqual({admit:true});
 expect(admitRemoteExecutionActionV1({view:sixteen(6000),running:1,concurrency:10,nowMs}),"one running action beside 6000 MB free").toEqual({admit:true});
 expect(admitRemoteExecutionActionV1({view:sixteen(6000),running:2,concurrency:10,nowMs}),"two running actions beside 6000 MB free").toEqual({admit:true});
 expect(admitRemoteExecutionActionV1({view:sixteen(3000),running:1,concurrency:10,nowMs})).toEqual({admit:false,reason:"memory"});
 expect(admitRemoteExecutionActionV1({view:sixteen(6000),running:10,concurrency:10,nowMs})).toEqual({admit:false,reason:"slots"});
});

test("an agent whose offered toolchain is gone refuses every action, reports no headroom and says why; once ready it reports headroom again",async()=>{
 const root=temporary(),fake=await fakeScheduler(403),controller=new AbortController();let ready=false;
 const agent=runRemoteExecutionAgentV1({...agentOptions(root,fake.url,controller.signal),ready:()=>ready});
 const waiting=await until(()=>fake.reports.find(row=>row.worker.why?.code==="no-toolchain"));
 expect(waiting.headroom).toBe(false);expect(waiting.worker.why!.en).toContain("toolchain");expect(waiting.worker.why!.de).toContain("Toolchain");
 for(const row of fake.reports)requireRemoteExecutionShapeV1("LoadReport",row,"closed");
 ready=true;
 const lending=await until(()=>fake.reports.filter(row=>row.worker.why?.code!=="no-toolchain").pop());
 expect(lending.headroom).toBe(true);
 controller.abort();await agent.catch(()=>undefined);
});
