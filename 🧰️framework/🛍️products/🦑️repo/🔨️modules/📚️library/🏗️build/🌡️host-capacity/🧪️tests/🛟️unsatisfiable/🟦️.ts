/** 🛟 Contract: a request that waits for memory although it would not fit even if every grant of ours ended can never be served by waiting (no release of ours will make room), so after `wait.bypassMilliseconds` the gate admits it with the room the host has, reduced below its estimate and told in a bilingual progress line: a 16 GB host with 5.6 GB available, a 2 GB floor and a fixed reservation of 8 or 12 GB runs slower, never never. A host with room, a request that fits once the live grants of ours end and a host with nothing to give keep their rules. */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {newHostCapacityIdV1,readHostCapacityPolicyV1,unsatisfiableHostRequestV1,withHostCapacityV1,type HostRequestV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
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
