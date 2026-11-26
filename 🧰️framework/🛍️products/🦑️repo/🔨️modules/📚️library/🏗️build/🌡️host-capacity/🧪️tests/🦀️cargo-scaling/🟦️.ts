/** 🦀️ Contract of cargo admission: a cargo run is sized by its tool alone, jobs times the footprint of one job, so it is granted the jobs the spare memory carries (at least one) and never waits behind the constant planning floor that graph runs learned.
 * @see ../🛰️planned-grant/🟦️.ts
 * @see ../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/cargo-admission-scaling.md */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {newHostCapacityIdV1,readHostCapacityPolicyV1,readLiveHostCapacityReservationV1,withHostCapacityV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {readCompileUnitsHostV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🧮️units/🟦️.ts";
import {recordHostOutcomeV1,updateHostEstimateStoreV1,emptyHostEstimateStoreV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📐️estimates/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {withBuildCapacityV1} from "../../🟦️.ts";
import {testHostProbeV1} from "../../../🌐️remote-execution/🚀️offload/🧪️tests/🧰️workspace/🟦️.ts";
import {extractOffloadFlagV1} from "../../../🌐️remote-execution/🚀️offload/📐️policy/🟦️.ts";
import {readRepositoryCargoLaunchV1} from "../../../../🏃️process/🦀️cargo/🚪️launcher/🟦️.ts";
import cases from "./🧫️fixtures/🔣️.json" with {type:"json"};

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"cs-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const policy=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:cases.host.cores,totalMegabytes:cases.host.totalMegabytes,availableMegabytes:cases.host.availableMegabytes,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:100,openFilesMaximum:1000000,attachedImages:null});
const sample=async()=>({host:host(),swappedOutMegabytes:0});
const strip=(args:readonly string[])=>args.filter(argument=>!argument.startsWith("--tool"));

for(const item of cases.runs)test(`cargo run: ${item.name}`,async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_OFFLOAD:"never",LANG:"en_US.UTF-8"},abort=new AbortController();
 await updateHostEstimateStoreV1(directory,policy,()=>Array.from({length:cases.learnedPlanRuns},(_,index)=>index).reduce((store,index)=>recordHostOutcomeV1(policy,store,["phase:plan"],{operation:"run",jobs:1,peakMegabytes:cases.learnedPlanRatioBasisPoints*policy.estimates.phaseReferenceMegabytes/10000,defaultMegabytes:policy.estimates.phaseReferenceMegabytes,activeMilliseconds:1000,peakAtMilliseconds:500,endedAtMs:Date.now()-index}),emptyHostEstimateStoreV1()));
 const timer=setTimeout(()=>abort.abort(),4000);
 let jobs=0,parallelism="";
 const ports={directory,policy,strip,sample,observe:async()=>45,write:()=>undefined,exitCode:()=>0,control:{signal:abort.signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined}};
 try{
  await withBuildCapacityV1(item.args,environment,async granted=>{
   jobs=granted.handle!.jobs;parallelism=granted.args.find(argument=>argument.startsWith("--parallelism="))!;
   expect(readLiveHostCapacityReservationV1(environment,policy)?.retainReservation).toBeUndefined();
   await withBuildCapacityV1(granted.args,environment,async nested=>{
    expect(nested.handle?.id).toBe(granted.handle?.id);
    expect(nested.handle?.inherited).toBe(true);
    expect(nested.handle!.plan.memoryMegabytes).toBeGreaterThan(0);
    expect(nested.args).toEqual(granted.args);
   },ports);
  },ports);
 }finally{clearTimeout(timer);}
 expect(jobs).toBeGreaterThanOrEqual(item.jobsAtLeast);
 expect(jobs).toBeLessThan(item.jobsBelow);
 expect(parallelism).toBe(`--parallelism=${jobs}`);
},30000);

const fleetWithHeadroom=(candidates:number)=>({fetch:(async()=>new Response(JSON.stringify({workers:candidates+2,compatible:candidates,headroom:candidates,candidates}))) as unknown as typeof fetch});
const smallHost=async()=>({host:{...host(),availableMegabytes:cases.fleetAvailableMegabytes},swappedOutMegabytes:0});

for (const item of cases.fleetRuns) test(`cargo with published fleet execution ${item.name}`, async () => {
 const directory = fresh();
 const environment: NodeJS.ProcessEnv = {
  LANG: "en_US.UTF-8",
  SEMIO_HOST_CAPACITY_DIRECTORY: directory,
  SEMIO_WORKSPACE_ROOT: directory,
  SEMIO_REMOTE_EXECUTION_ENDPOINT: "grpc://192.0.2.1:8980",
 };
 const args = ["cargo", "--store=shared", "--configuration=prototype", ...item.parallelismArgs, "--", "check", "-p", "x"];
 const ports = {
  policy, directory, strip, sample: smallHost, write: () => undefined,
  offloadAdmission: { probe: testHostProbeV1("critical-pressure-gigabyte-swap"), policy, fleet: fleetWithHeadroom(2) },
 };
 await withBuildCapacityV1(args, environment, async grant => {
  const reservation = readLiveHostCapacityReservationV1(environment, policy);
  expect(reservation).toMatchObject({reservedMegabytes:cases.fleetControlMegabytes,retainReservation:true});
  expect(grant.handle?.plan).toMatchObject({memoryMegabytes:0,fixedMegabytes:cases.fleetControlMegabytes});
  expect(grant.args).toEqual(args);
  expect<unknown>(readRepositoryCargoLaunchV1(extractOffloadFlagV1(grant.args).args.slice(1), environment).parallelism).toBe(item.localParallelism);
  expect(await readCompileUnitsHostV1({environment,probe:testHostProbeV1("quiet-no-swap",true)})).toMatchObject({grantedJobs:null,grantedMegabytes:null});
  await withBuildCapacityV1(grant.args,environment,async nested=>{
   expect(nested.handle?.id).toBe(reservation?.id);
   expect(nested.handle?.inherited).toBe(true);
   expect(nested.args).toEqual(args);
   expect(readdirSync(join(directory,"grants")).filter(path=>path.endsWith(".json"))).toEqual([`${reservation!.id}.json`]);
  },{...ports,offloadAdmission:{...ports.offloadAdmission,probe:testHostProbeV1("quiet-no-swap",true)}});
 }, ports);
 expect(environment.SEMIO_HOST_CAPACITY_GRANT).toBeUndefined();
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
});

for (const item of cases.queuedFleetRuns) test(`cargo with published fleet execution ${item.name}`, async () => {
 const directory = fresh(), queue = join(directory, "queue");
 const environment = {LANG: "en_US.UTF-8", SEMIO_WORKSPACE_ROOT: directory, SEMIO_REMOTE_EXECUTION_ENDPOINT: "grpc://192.0.2.1:8980", SEMIO_OFFLOAD: item.ambientOffloadMode};
 const offloadArgs = item.offloadMode === undefined ? [] : [`--offload=${item.offloadMode}`];
 const args = ["cargo", "--store=shared", "--configuration=prototype", ...offloadArgs, "--parallelism", "1", "--", "check", "-p", "x"];
 const quiet = testHostProbeV1("quiet-no-swap", true), pressured = testHostProbeV1("critical-pressure-gigabyte-swap");
 const controller = new AbortController(), cancelled = Error("cancelled while probing fleet eligibility");
 const timeout = setTimeout(() => controller.abort(Error("Cargo did not recheck fleet eligibility")), 1500);
 const waitersAtProbe: string[][] = [];
 let launches = 0, queued = false;
 const result = await withBuildCapacityV1(args, environment, async grant => {
  launches++;
  expect(readdirSync(queue)).toEqual([]);
  if (item.outcome === "local") expect(grant.handle?.jobs).toBe(1);
  else {
   expect(grant.handle?.plan).toMatchObject({memoryMegabytes:0,fixedMegabytes:cases.fleetControlMegabytes});
   expect(grant.args).toEqual(args);
  }
  expect(readRepositoryCargoLaunchV1(extractOffloadFlagV1(grant.args).args.slice(1), environment).parallelism).toBe(1);
  return "launched";
 }, {
  directory, policy, write: () => undefined,
  sample: async () => ({host: queued && item.outcome === "local" ? host() : {...host(), availableMegabytes: item.hostAvailableMegabytes}, swappedOutMegabytes: 0}),
  observe: async () => 0,
  offloadAdmission: {policy, fleet: item.outcome === "cancelled" ? {fetch: (async (_url: unknown, init?: RequestInit) => {if (queued) {controller.abort(cancelled); init?.signal?.throwIfAborted();} return fleetWithHeadroom(2).fetch(_url as string);}) as unknown as typeof fetch} : fleetWithHeadroom(2), probe: {...quiet, run: async (command, arguments_, signal) => {
   if (command === "vm_stat") {
    waitersAtProbe.push(existsSync(queue) ? readdirSync(queue) : []);
    if (queued) expect(signal).toBe(controller.signal);
    if (queued && item.outcome === "cancelled") {
     controller.abort(cancelled);
     signal?.throwIfAborted();
    }
   }
   return (queued ? pressured : quiet).run(command, arguments_);
  }}},
  control: {signal: controller.signal, remainingMilliseconds: () => null, publish: () => {queued = true;}, yieldContinuation: async () => undefined},
 }).catch(error => error).finally(() => clearTimeout(timeout));
 expect(result).toBe(item.outcome === "cancelled" ? cancelled : "launched");
 expect(queued).toBe(true);
 expect(waitersAtProbe.map(rows => rows.length)).toEqual(item.probeQueueLengths);
 expect(readdirSync(queue)).toEqual([]);
 expect(readdirSync(join(directory, "grants"))).toEqual([]);
 expect(launches).toBe(item.outcome === "cancelled" ? 0 : 1);
});

test("a machine the door would make wait starts cargo with a minimal local grant when the fleet has headroom, and keeps waiting when it has none or the scheduler does not answer",async()=>{
 const directory=fresh(),environment:Record<string,string|undefined>={LANG:"en_US.UTF-8",SEMIO_WORKSPACE_ROOT:directory,SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://192.0.2.1:8980"};
 const args=["cargo","--store=shared","--configuration=prototype","--","check","-p","x"],pressured=testHostProbeV1("critical-pressure-gigabyte-swap");
 const started=await withBuildCapacityV1(args,environment,async grant=>({reserved:grant.handle?.reservedMegabytes,slots:environment.SEMIO_UNIT_CACHE_SLOTS,pressure:environment.SEMIO_OFFLOAD_PRESSURE}),{policy,directory,strip,sample:smallHost,write:()=>undefined,offloadAdmission:{probe:pressured,policy,fleet:fleetWithHeadroom(1)}});
 expect(started).toEqual({reserved:cases.fleetControlMegabytes,slots:"1",pressure:"door"});
 expect(environment.SEMIO_UNIT_CACHE_SLOTS,"restored after the build").toBeUndefined();expect(environment.SEMIO_OFFLOAD_PRESSURE).toBeUndefined();
 for(const [label,fleet] of [["nobody has headroom",fleetWithHeadroom(0)],["the scheduler does not answer",{fetch:(async()=>{throw Error("unreachable");}) as unknown as typeof fetch}]] as const){
  const controller=new AbortController(),timer=setTimeout(()=>{controller.abort(Error("kept waiting"));},1200);let launched=false;
  const outcome=await withBuildCapacityV1(args,{...environment},async()=>{launched=true;return "launched";},{policy,directory:fresh(),strip,write:()=>undefined,sample:async()=>({host:{...host(),availableMegabytes:100},swappedOutMegabytes:0}),observe:async()=>0,offloadAdmission:{probe:pressured,policy,fleet},control:{signal:controller.signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:async()=>undefined}}).catch(error=>error).finally(()=>clearTimeout(timer));
  expect(launched,label).toBe(false);expect(String((outcome as Error).message),label).toContain("kept waiting");
 }
},30000);

const pinned=["cargo","--offload=pin:test-worker","--parallelism=3","--","check"];
const fleetEnvironment=(directory:string):NodeJS.ProcessEnv=>({LANG:"en",SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_WORKSPACE_ROOT:directory,SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://192.0.2.1:8980"});
const controlOf=(signal:AbortSignal,publish:()=>void=()=>undefined)=>({signal,remainingMilliseconds:()=>null,publish,yieldContinuation:async()=>undefined});

for(const row of cases.fleetRefusals)test(`fleet headroom does not bypass ${row.name} on local Cargo orchestration`,async()=>{
 const directory=fresh(),environment=fleetEnvironment(directory),controller=new AbortController(),cancelled=Error("cancelled safely"),reserve=async()=>({host:{...host(),pressure:row.pressure as HostSampleV1["pressure"],availableMegabytes:row.availableMegabytes,swapOutMegabytesPerSecond:row.swapOutMegabytesPerSecond},swappedOutMegabytes:0});
 let ran=false;
 const outcome=await withBuildCapacityV1(pinned,environment,async()=>{ran=true;},{policy,directory,strip,sample:reserve,write:()=>undefined,control:controlOf(controller.signal,()=>controller.abort(cancelled))}).catch(error=>error);
 expect(outcome).toBe(cancelled);
 expect(ran).toBe(false);
 expect(readdirSync(join(directory,"queue"))).toEqual([]);
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
});

test("fleet orchestration requires a sufficient retained parent grant",async()=>{
 const directory=fresh(),environment=fleetEnvironment(directory),control=controlOf(AbortSignal.timeout(3000)),ports={directory,policy,strip,sample,write:()=>undefined,control};
 for(const row of cases.fleetParentGrants){
  const request={id:newHostCapacityIdV1(),operation:"command",priority:"batch" as const,minimumJobs:1,preferredJobs:1,fixedMegabytes:row.megabytes,minimumMegabytes:row.megabytes,perJobMegabytes:0,cpuPerJob:1,...row.retained?{retainReservation:true}:{}};
  await withHostCapacityV1({...ports,environment,request},async parent=>{
   const scoped={...environment,...parent.environment};
   const nested=withBuildCapacityV1(pinned,scoped,async active=>{
    expect(active.handle?.id).toBe(parent.id);
    expect(active.handle?.inherited).toBe(true);
    expect(active.args).toEqual(pinned);
    expect(readdirSync(join(directory,"grants")).filter(path=>path.endsWith(".json"))).toEqual([`${parent.id}.json`]);
    return parent.id;
   },ports);
   if(row.admitted)expect(await nested).toBe(parent.id);
   else await expect(nested).rejects.toMatchObject({key:"capacity.cargo-orchestration-grant"});
   expect(readLiveHostCapacityReservationV1(scoped,policy)?.reservedMegabytes).toBe(row.megabytes);
  });
 }
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
});

test("fleet orchestration keeps its grant while the cancelled body drains",async()=>{
 const directory=fresh(),environment=fleetEnvironment(directory),controller=new AbortController(),reason=Error("cancelled while running");
 let release!:()=>void,ready!:()=>void;
 const drained=new Promise<void>(done=>{release=done;}),entered=new Promise<void>(done=>{ready=done;});
 const running=withBuildCapacityV1(pinned,environment,async()=>{
  ready();
  await drained;
  controller.signal.throwIfAborted();
 },{policy,directory,strip,sample:smallHost,control:controlOf(controller.signal),write:()=>undefined});
 try{
  await Promise.race([entered,running]);
  controller.abort(reason);
  expect(readLiveHostCapacityReservationV1(environment,policy)).toMatchObject({reservedMegabytes:cases.fleetControlMegabytes,retainReservation:true});
 }finally{release();}
 await expect(running).rejects.toBe(reason);
 expect(readLiveHostCapacityReservationV1(environment,policy)).toBeUndefined();
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
});
