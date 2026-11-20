/** 🦀️ Contract of cargo admission: a cargo run is sized by its tool alone, jobs times the footprint of one job, so it is granted the jobs the spare memory carries (at least one) and never waits behind the constant planning floor that graph runs learned.
 * @see ../🛰️planned-grant/🟦️.ts
 * @see ../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/cargo-admission-scaling.md */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readHostCapacityPolicyV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
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
 try{
  await withBuildCapacityV1(item.args,environment,async granted=>{jobs=granted.handle!.jobs;parallelism=granted.args.find(argument=>argument.startsWith("--parallelism="))!;},{directory,policy,strip,sample,observe:async()=>45,write:()=>undefined,exitCode:()=>0,control:{signal:abort.signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined}});
 }finally{clearTimeout(timer);}
 expect(jobs).toBeGreaterThanOrEqual(item.jobsAtLeast);
 expect(jobs).toBeLessThan(item.jobsBelow);
 expect(parallelism).toBe(`--parallelism=${jobs}`);
},30000);

for (const item of cases.fleetRuns) test(`cargo with published fleet execution ${item.name}`, async () => {
 const directory = fresh();
 const environment = {
  LANG: "en_US.UTF-8",
  SEMIO_WORKSPACE_ROOT: directory,
  SEMIO_REMOTE_EXECUTION_ENDPOINT: "grpc://192.0.2.1:8980",
 };
 const args = ["cargo", "--store=shared", "--configuration=prototype", ...item.parallelismArgs, "--", "check", "-p", "x"];
 const result = await withBuildCapacityV1(args, environment, async grant => ({
  args: grant.args,
  hostGranted: grant.handle !== undefined,
  localParallelism: readRepositoryCargoLaunchV1(grant.args.slice(1), environment).parallelism,
 }), {
  policy, directory, strip, write: () => undefined,
  offloadAdmission: { probe: testHostProbeV1("critical-pressure-gigabyte-swap"), policy },
 });
 expect(result).toEqual({ args, hostGranted: false, localParallelism: item.localParallelism });
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
   expect(grant.handle).toBeUndefined();
   expect(grant.args).toEqual(args);
  }
  expect(readRepositoryCargoLaunchV1(extractOffloadFlagV1(grant.args).args.slice(1), environment).parallelism).toBe(1);
  return "launched";
 }, {
  directory, policy, write: () => undefined,
  sample: async () => ({host: queued && item.outcome === "local" ? host() : {...host(), availableMegabytes: item.hostAvailableMegabytes}, swappedOutMegabytes: 0}),
  observe: async () => 0,
  offloadAdmission: {policy, probe: {...quiet, run: async (command, arguments_, signal) => {
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
