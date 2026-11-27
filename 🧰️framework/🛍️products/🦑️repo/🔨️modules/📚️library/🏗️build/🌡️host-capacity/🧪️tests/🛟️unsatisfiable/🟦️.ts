/** 🛟 Contract: a request that waits for memory although it would not fit even if every grant of ours ended can never be served by waiting (no release of ours will make room), so after `wait.bypassMilliseconds` the gate admits it with the room the host has, reduced below its estimate and told in a bilingual progress line: a 16 GB host with 5.6 GB available, a 2 GB floor and a fixed reservation of 8 or 12 GB runs slower, never never. A host with room, a request that fits once the live grants of ours end and a host with nothing to give keep their rules. */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {evaluateHostCapacityV1,hostRequestNeverFitsV1,hostWaitDeadlineMillisecondsV1,newHostCapacityIdV1,readHostCapacityPolicyV1,unsatisfiableHostRequestV1,withHostCapacityV1,withUnmanagedHostV1,type HostGrantV1,type HostRequestV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"unsatisfiable-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const policy=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,bypassMilliseconds:200,sampleMaximumAgeMilliseconds:0}});
const host=(availableMegabytes:number):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:16384,availableMegabytes,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const control=(signal=new AbortController().signal)=>({signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const request=(fixedMegabytes:number,operation="graph-plan"):HostRequestV1=>({id:newHostCapacityIdV1(),operation,priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes,perJobMegabytes:0,cpuPerJob:1} as HostRequestV1);
const python=Bun.which("python3")??Bun.which("python");
const independent=(available:number,floor:number,asked:number)=>Number(Bun.spawnSync([python!,"-I","-c","import sys\na,f,r=map(int,sys.argv[1:])\nprint(min(r,a-f))",String(available),String(floor),String(asked)]).stdout.toString());
const admit=async(directory:string,available:number,asked:HostRequestV1,language="en_US.UTF-8")=>{
 const lines:string[]=[],started=Date.now();
 const handle=await withHostCapacityV1({directory,policy,request:asked,control:control(),environment:{LANG:language},write:line=>lines.push(line),sample:async()=>({host:host(available),swappedOutMegabytes:0})},async granted=>granted);
 return {handle,lines,seconds:(Date.now()-started)/1000};
};

test("a lone 8 GB graph plan on a 16 GB host with 5.6 GB available is admitted after the bypass time with what the host has, and says so",async()=>{
 const {handle,lines}=await admit(fresh(),5660,request(8192));
 expect(handle.reservedMegabytes).toBe(independent(5660,2048,8192));expect(handle.reservedMegabytes).toBeGreaterThan(0);expect(handle.reservedMegabytes).toBeLessThan(8192);
 expect(lines.some(line=>line.includes("[capacity] admitted graph-plan")&&line.includes("with "+handle.reservedMegabytes+" MB of the 8192 MB it asked for"))).toBe(true);
});

test("the message exists in German too",async()=>{
 const {lines}=await admit(fresh(),5660,request(8192),"de_DE.UTF-8");
 expect(lines.some(line=>line.includes("der angefragten 8192 MB zugelassen"))).toBe(true);
});

test("the fixed class reservation of the course build is clamped the same way",async()=>{
 const ceiling=buildPolicy.hostCapacity.classCeilings.find(row=>row.name==="course-build")!.megabytes,{handle}=await admit(fresh(),5660,request(ceiling,"run"));
 expect(handle.reservedMegabytes).toBe(independent(5660,2048,ceiling));
});

test("a host with plenty of memory is unchanged: the full request is admitted at once without the message",async()=>{
 const {handle,lines,seconds}=await admit(fresh(),20000,request(8192));
 expect(handle.reservedMegabytes).toBe(8192);expect(lines.some(line=>line.includes("it asked for"))).toBe(false);expect(seconds).toBeLessThan(policy.wait.bypassMilliseconds/1000+5);
});

test("a host with nothing to give above the floor keeps the request waiting",async()=>{
 const controller=new AbortController();
 const waiting=withHostCapacityV1({directory:fresh(),policy,request:request(8192),control:control(controller.signal),environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample:async()=>({host:host(2048+policy.estimates.minimumMegabytes-1),swappedOutMegabytes:0})},async granted=>granted);
 setTimeout(()=>controller.abort(new Error("stop")),policy.wait.bypassMilliseconds*3);
 await expect(waiting).rejects.toThrow();
});

test("live grants of ours that hold little do not keep an unsatisfiable request waiting, but a request that fits once they end waits for them",async()=>{
 const directory=fresh();let released:()=>void=()=>undefined;const held=new Promise<void>(done=>{released=done;});
 const first=withHostCapacityV1({directory,policy,request:request(1536,"run"),control:control(),environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample:async()=>({host:host(5660),swappedOutMegabytes:0})},async()=>{await held;});
 await new Promise(done=>setTimeout(done,200));
 const {handle}=await admit(directory,5660,request(8192));
 expect(handle.reservedMegabytes).toBeGreaterThan(0);expect(handle.reservedMegabytes).toBeLessThan(8192);
 const stopper=new AbortController(),fits=withHostCapacityV1({directory,policy,request:request(5000),control:control(stopper.signal),environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample:async()=>({host:host(5660),swappedOutMegabytes:0})},async granted=>granted);
 setTimeout(()=>stopper.abort(new Error("stop")),policy.wait.bypassMilliseconds*3);
 await expect(fits).rejects.toThrow();
 released();await first;
});

test("the pure admission rule needs that releases of ours cannot make room, the bypass time and the minimum room",()=>{
 const base={waitedMilliseconds:policy.wait.bypassMilliseconds,roomMegabytes:3612,drainedMegabytes:3612,needMegabytes:8192,admitted:0,replaced:false,quiet:false,elastic:false,pause:undefined};
 expect(unsatisfiableHostRequestV1(policy,base)).toBe(true);
 for(const change of [{waitedMilliseconds:policy.wait.bypassMilliseconds-1},{roomMegabytes:policy.estimates.minimumMegabytes-1},{drainedMegabytes:8192},{admitted:1},{replaced:true},{quiet:true},{elastic:true},{pause:"pressure-warn"}])expect(unsatisfiableHostRequestV1(policy,{...base,...change})).toBe(false);
});


const real=readHostCapacityPolicyV1(buildPolicy.hostCapacity),batchDeadline=real.wait.deadlineBatchMilliseconds!,least=real.estimates.rebuildMinimumMegabytes!;
const machine=(total:number,available:number,over:Partial<HostSampleV1>={}):HostSampleV1=>({...host(available),totalMegabytes:total,...over});
const own=(id:string,reserved:number,observed:number|null):HostGrantV1=>({id,operation:"show",jobs:1,cpu:2,reservedMegabytes:reserved,admittedAtMs:0,observedMegabytes:observed,pid:process.pid,bootAtMs:0,rebuildScope:"scope0001"} as HostGrantV1);
const rebuild=(id:string,fixed:number,minimum:number|undefined):HostRequestV1=>({id,operation:"show",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:fixed,perJobMegabytes:0,cpuPerJob:2,...minimum===undefined?{}:{minimumMegabytes:minimum},label:"projects"} as HostRequestV1);
const upgrade=(total:number,available:number,reserved:number,observed:number|null,fixed:number,minimum:number|undefined)=>{
 const id="aaaaaaaa1",request=rebuild(id,fixed,minimum),sample=machine(total,available),held=[own(id,reserved,observed)];
 return evaluateHostCapacityV1(real,withUnmanagedHostV1(sample,held),held,[{request,arrivedAtMs:0,pid:process.pid,upgrade:true,rebuild:"scope0001"}],3_600_000).decisions[0]!;
};
const expectedBooking=(held:number,observed:number,desired:number)=>Number(Bun.spawnSync([python!,"-I","-c","import sys\nheld,observed,desired=map(int,sys.argv[1:])\nprint(min(desired,held+observed))",String(held),String(observed),String(desired)]).stdout.toString());

test("a rebuild booking on a 16 GB machine is admitted with the largest amount that fits now, never under its minimum, instead of waiting for the full block",()=>{
 const room=(available:number)=>available-2048,full=7168;
 for(const available of [4000,5000,8000]){
  const row=upgrade(16384,available,2747,2747,full,least);
  expect(row.verdict,String(available)).toBe("admit");
  expect(row.reservedMegabytes,String(available)).toBe(Math.max(2747,expectedBooking(2747,room(available),full)));
  expect(row.reservedMegabytes).toBeGreaterThanOrEqual(least);expect(row.reservedMegabytes).toBeLessThanOrEqual(full);
 }
 expect(upgrade(16384,8000,2747,2747,full,least)).toMatchObject({verdict:"admit",reason:"fits",reservedMegabytes:full});
 expect(upgrade(16384,5000,2747,2747,full,least)).toMatchObject({verdict:"admit",reason:"shrunk"});
});

test("an upgrade never waits on itself: its own grant counts toward the booking, and a holder that already holds its minimum keeps going when the host has nothing more to give",()=>{
 const starved=upgrade(16384,1000,2747,2747,7168,least);
 expect(starved).toMatchObject({verdict:"admit",reason:"shrunk",reservedMegabytes:2747});
 expect(upgrade(16384,1000,2747,2747,7168,undefined),"without a minimum the same upgrade waits for the full block").toMatchObject({verdict:"wait",reason:"memory"});
 expect(upgrade(16384,3000,2747,null,7168,least).reservedMegabytes,"a grant nobody observed yet is not credited with memory the host still shows free").toBeGreaterThanOrEqual(2747);
});

test("a rebuild whose minimum can never fit this machine is refused at once with the typed reason, on every machine below the minimum",()=>{
 expect(upgrade(3072,3000,1024,1024,2304,least)).toMatchObject({verdict:"wait",reason:"never"});
 expect(upgrade(4096,3500,1024,1024,2560,least).reason).not.toBe("never");
 expect(hostRequestNeverFitsV1(real,{totalMegabytes:16384},request(15000))).toBe(true);
 expect(hostRequestNeverFitsV1(real,{totalMegabytes:16384},request(14336))).toBe(false);
 expect(hostRequestNeverFitsV1(real,{totalMegabytes:16384},{...request(15000),quiet:true} as HostRequestV1)).toBe(false);
});

test("a request that can never fit is refused at the door with an actionable bilingual error instead of waiting",async()=>{
 for(const [language,text] of [["en_US.UTF-8","cannot run here"],["de_DE.UTF-8","kann hier nicht laufen"]] as const){
  const started=Date.now(),failure=await withHostCapacityV1({directory:fresh(),policy,request:request(15000),control:control(),environment:{LANG:language},write:()=>undefined,sample:async()=>({host:host(3000),swappedOutMegabytes:0})},async granted=>granted).then(()=>"", (error:Error)=>error.message);
  expect(failure.toLowerCase(),language).toContain(text);expect(Date.now()-started).toBeLessThan(5000);
 }
});

test("the priority deadline can admit a shrinkable minimum under a soft stop, while hard stops wait or expire",()=>{
 expect(hostWaitDeadlineMillisecondsV1(real,"interactive")).toBeLessThan(hostWaitDeadlineMillisecondsV1(real,"batch"));
 const item={...request(6627),minimumMegabytes:2048,label:"graph-plan"} as HostRequestV1,busy:HostGrantV1[]=[{id:"bbbbbbbb2",operation:"run",jobs:1,cpu:1,reservedMegabytes:1000,admittedAtMs:0,observedMegabytes:1000,pid:process.pid,bootAtMs:0} as HostGrantV1],ask=(sample:HostSampleV1,waited:number)=>evaluateHostCapacityV1(real,withUnmanagedHostV1(sample,busy),busy,[{request:item,arrivedAtMs:1000,pid:process.pid}],1000+waited).decisions[0]!;
 const critical=machine(16384,5000,{pressure:"critical"}),warn=machine(16384,5000,{pressure:"warn"});
 expect(ask(critical,batchDeadline-1)).toMatchObject({verdict:"wait",reason:"pressure-critical"});
 expect(ask(critical,batchDeadline)).toMatchObject({verdict:"wait",reason:"pressure-critical",reservedMegabytes:0});
 expect(ask(warn,batchDeadline-1).verdict).toBe("wait");
 expect(ask(warn,batchDeadline)).toMatchObject({verdict:"admit",reason:"shrunk"});
 expect(ask(machine(16384,2048+1000,{pressure:"critical"}),batchDeadline),"below the minimum the stop keeps holding").toMatchObject({verdict:"wait",reason:"pressure-critical"});
 const swapping=machine(32768,7000,{swapUsedMegabytes:9000,swapOutMegabytesPerSecond:103});
 expect(ask(swapping,batchDeadline)).toMatchObject({verdict:"wait",reason:"swap",reservedMegabytes:0});
 const saturated=machine(16384,5000,{swapUsedMegabytes:0,swapOutMegabytesPerSecond:103});
 expect(ask(saturated,batchDeadline)).toMatchObject({verdict:"wait",reason:"swap-activity",reservedMegabytes:0});
 expect(ask(critical,real.wait.starvedMilliseconds!)).toMatchObject({verdict:"wait",reason:"pressure-critical",reservedMegabytes:0});
});

test("a request past its deadline under a stop says so in both languages, naming the stop, the limit and what it waited",async()=>{
 const fastDeadline=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0,deadlineBatchMilliseconds:1000,deadlineInteractiveMilliseconds:1000}});
 for(const [language,text] of [["en_US.UTF-8","after waiting"],["de_DE.UTF-8","Wartezeit"]] as const){
  const lines:string[]=[],item={...request(6627),minimumMegabytes:2048,label:"graph-plan"} as HostRequestV1;
  const pastLimit=language.startsWith("de")?"über der Wartegrenze von 1s":"past the wait limit of 1s";
  let recovered=false;
  const handle=await withHostCapacityV1({directory:fresh(),policy:fastDeadline,request:item,control:control(),environment:{LANG:language},write:line=>{lines.push(line);if(line.includes(pastLimit))recovered=true;},sample:async()=>({host:{...host(5000),pressure:recovered?"warn" as const:"critical" as const},swappedOutMegabytes:0})},async granted=>granted);
  expect(handle.reservedMegabytes).toBe(5000-2048);
  expect(recovered).toBe(true);
  expect(lines.some(line=>line.includes(text)),language+": "+lines.join(" | ")).toBe(true);
  const stuck:string[]=[],controller=new AbortController();
  setTimeout(()=>controller.abort(new Error("stop")),2500);
  await withHostCapacityV1({directory:fresh(),policy:fastDeadline,request:item,control:control(controller.signal),environment:{LANG:language},write:line=>stuck.push(line),sample:async()=>({host:{...host(2048+1000),pressure:"critical" as const},swappedOutMegabytes:0})},async granted=>granted).catch(()=>undefined);
  expect(stuck.some(line=>line.includes(language.startsWith("de")?"über der Wartegrenze von 1s":"past the wait limit of 1s")),language+": "+stuck.slice(-2).join(" | ")).toBe(true);
 }
},60000);
