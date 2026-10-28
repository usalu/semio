import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {hostFloorMegabytesV1,readHostCapacityPolicyV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {planCompileUnitsV1,readCompileUnitsHostV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {upgradeBuildCapacityV1,withBuildCapacityV1} from "../../🟦️.ts";

/** 🌱️ Contract (g31-course): a run admitted with 1 of 8 jobs because fleet memory was short grows to more jobs once memory frees up, the plan of its processes follows the grant, and a planned compile inside it takes the cores the grant's memory carries at the measured unit footprint instead of one core. */
const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const policy=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,memory:{...buildPolicy.hostCapacity.memory,rampMilliseconds:1000},estimates:{...buildPolicy.hostCapacity.estimates,defaultRampMilliseconds:1000},wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:40,sampleMaximumAgeMilliseconds:0,lendAfterMilliseconds:600000}});
const floor=hostFloorMegabytesV1(policy,{totalMegabytes:32768});
const host=(available:number):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:available,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const strip=(args:readonly string[])=>args.filter(argument=>!argument.startsWith("--tool"));
const measured=new Map(Array.from({length:16},(_,index)=>[`unit-${index}`,[{peakRssBytes:900*1048576}]] as const));

test("g31 replay: the course run admitted with 1 of 8 jobs grows once memory frees up, its plan follows, and its compile takes the cores the grant carries, in English and German",async()=>{
 for(const [language,reduced,grown] of [["en_US.UTF-8","[capacity] admitted run with 1 of 8 job(s): the host cannot carry more right now; the run grows to more jobs once memory frees up",/^\[capacity\] run: grown from 1 to 8 of 8 job\(s\)/u],["de_DE.UTF-8","[capacity] run mit 1 von 8 Job(s) zugelassen: der Rechner trägt gerade nicht mehr; der Lauf wächst auf mehr Jobs, sobald Speicher frei wird",/^\[capacity\] run: nach \d+ s von 1 auf 8 von 8 Job\(s\) gewachsen/u]] as const){
  const directory=mkdtempSync(join(tmpdir(),"hgl-"));roots.push(directory);
  const lines:string[]=[],environment:NodeJS.ProcessEnv={SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"32768",LANG:language};let free=2600;
  await withBuildCapacityV1(["run","@teaching/architecture-course:build","--parallel=8"],environment,async granted=>{
   expect(granted.args,"the tool is told the jobs it may grow into; its plan holds the granted job").toContain("--parallel=8");
   expect(JSON.parse(environment.SEMIO_HOST_CAPACITY_PLAN!).jobs).toBe(1);
   const before=planCompileUnitsV1(await readCompileUnitsHostV1({environment,cores:10,history:measured}));
   expect([before.preferred,before.reason,before.budgetMegabytes],"one granted job: its 2048 MB carry two compile units of the measured 900 MB, not one core").toEqual([2,"granted",2048]);
   free=30000;
   let jobs=1;const end=Date.now()+4000;
   while(jobs===1&&Date.now()<end){await upgradeBuildCapacityV1(new AbortController().signal,environment);jobs=JSON.parse(environment.SEMIO_HOST_CAPACITY_PLAN!).jobs;if(jobs===1)await sleep(30);}
   expect([jobs,environment.SEMIO_BUILD_PARALLEL],"at the next work point the plan of the process follows the grown grant").toEqual([8,"8"]);
   const after=planCompileUnitsV1(await readCompileUnitsHostV1({environment,cores:10,history:measured}));
   expect([after.preferred,after.reason],"the grown grant carries the share of the host").toEqual([5,"share"]);
  },{directory,policy,strip,tool:"bazel",sample:async()=>({host:host(floor+free),swappedOutMegabytes:0}),observe:async()=>3000,waited:()=>0,children:()=>[1],write:(line:string)=>lines.push(line),exitCode:()=>0});
  expect(lines,lines.join("\n")).toContain(reduced);
  expect(lines.some(line=>grown.test(line)),lines.join("\n")).toBe(true);
 }
},30000);
