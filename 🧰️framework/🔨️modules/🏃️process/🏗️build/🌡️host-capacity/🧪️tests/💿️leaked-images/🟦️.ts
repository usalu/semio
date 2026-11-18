import {afterAll,expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {existsSync,mkdirSync,statSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {enterScriptProcessWaitV1,liveChildPidsV1} from "../../../../🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import {attachHostCapacityProcessV1,bindHostCapacityWaitMarkerV1,hostCapacityDirectoryV1,hostTreeWaitsV1,processChildrenTableV1,processStartV1,readHostCapacityWaitMarkersV1,newHostCapacityIdV1,readHostCapacityPolicyV1,readHostCapacityStatusV1,takeHostLockV1,withHostCapacityV1,type HostCapacityOptionsV1,type HostSampleV1} from "../../🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";

const base=readHostCapacityPolicyV1(decisionFixture.policy),policy=readHostCapacityPolicyV1({...decisionFixture.policy,load:{...decisionFixture.policy.load,overrideAfterMilliseconds:150},wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hc-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=(over:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10,...over});
const sampler=(current:()=>Partial<HostSampleV1>)=>async()=>({host:host({...current()}),swappedOutMegabytes:0});
const control=(signal=new AbortController().signal,remaining:()=>number|null=()=>null)=>{const published:unknown[]=[];return {published,value:{signal,remainingMilliseconds:remaining,publish:async(row:unknown)=>{published.push(row);},yieldContinuation:async()=>undefined}};};
const request=(over:object={})=>({id:newHostCapacityIdV1(),operation:"run",priority:"batch" as const,minimumJobs:1,preferredJobs:1,fixedMegabytes:8000,perJobMegabytes:0,cpuPerJob:1,...over});
const options=(directory:string,over:Partial<HostCapacityOptionsV1>={}):HostCapacityOptionsV1=>({directory,policy,request:request(),control:control().value,environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample:sampler(()=>({})),...over});

const none=async()=>({total:0,ours:0,leaked:0,foreign:0,detached:0});
test("A leaked-image count is swept before the gate refuses: images of dead sandboxes are detached, the request is admitted at once and the line says so in both languages",async()=>{
 for(const [language,sentence] of [["en","detached 12 leaked disk image(s) of dead test sandboxes"],["de","12 verwaiste Disk-Image(s) toter Test-Sandboxes ausgehängt"]] as const){
  const directory=fresh(),lines:string[]=[];let swept=0;
  const started=Date.now();
  await withHostCapacityV1(options(directory,{environment:{LANG:language==="de"?"de_CH.UTF-8":"en_US.UTF-8",SEMIO_LANGUAGE:language},sample:sampler(()=>({attachedImages:40})),write:line=>lines.push(line),sweepImages:async()=>{swept++;return {total:40,ours:20,leaked:12,foreign:8,detached:12};}}),async handle=>expect(handle.jobs).toBe(1));
  expect(swept).toBe(1);
  expect(Date.now()-started).toBeLessThan(140);
  expect(lines.some(line=>line.includes(sentence)),language).toBe(true);
  expect(lines.some(line=>line.includes("waiting")||line.includes("wartet")),language).toBe(false);
 }
},30000);

test("When nothing can be swept the waiting line names how many images are ours, leaked and foreign in English and German",async()=>{
 for(const [language,sentence] of [["en","40 disk images are attached: 20 are ours, 12 belong to dead test sandboxes and 8 are foreign"],["de","40 Disk-Images sind eingehängt: 20 gehören uns, 12 gehören toten Test-Sandboxes und 8 sind fremd"]] as const){
  const directory=fresh(),lines:string[]=[];
  await withHostCapacityV1(options(directory,{environment:{LANG:language==="de"?"de_CH.UTF-8":"en_US.UTF-8",SEMIO_LANGUAGE:language},sample:sampler(()=>({attachedImages:40})),write:line=>lines.push(line),sweepImages:async()=>({total:40,ours:20,leaked:12,foreign:8,detached:0})}),async()=>undefined);
  expect(lines.some(line=>(line.includes("waiting")||line.includes("wartet"))&&line.includes(sentence)),language+lines.join("|")).toBe(true);
 }
},30000);

test("A sweep that throws never stops the gate",async()=>{
 const directory=fresh();
 await withHostCapacityV1(options(directory,{sample:sampler(()=>({attachedImages:40})),sweepImages:async()=>{throw Error("no hdiutil");}}),async handle=>expect(handle.jobs).toBe(1));
},30000);
