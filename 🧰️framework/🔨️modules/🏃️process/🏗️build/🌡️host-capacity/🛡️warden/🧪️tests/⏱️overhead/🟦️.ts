/** ⏱️ Measurement: the overhead of the host warden on the real machine. 60 real ticks run in observe mode against this machine with a private directory (the signal port throws, nothing can be signalled): the test prints the median and 95th percentile of the tick, the CPU time one tick costs (the whole process tree of the warden: the loop and the `ps`, `vm_stat` and `sysctl` it starts, from the difference of a 65-tick and a 5-tick run under `time -p`) and its share at the calm interval, and the resident megabytes of the warden. Run as a plain script with `WARDEN_OVERHEAD_TICKS=<n>` it is the driver the test measures.
 * @see ../../🔁️loop/🟦️.ts */
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import buildPolicy from "../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json" with {type:"json"};
import {readHostCapacityPolicyV1} from "../../../📏️decision/🟦️.ts";
import {HOST_WARDEN_POLICY_V1,type HostWardenStateV1} from "../../🟦️.ts";
import {defaultHostWardenPortsV1,runHostWardenLoopV1} from "../../🔁️loop/🟦️.ts";

const ticks=Number(process.env.WARDEN_OVERHEAD_TICKS??0),directory=process.env.WARDEN_OVERHEAD_DIRECTORY??"";
if(ticks>0){
 const hostPolicy=readHostCapacityPolicyV1((buildPolicy as unknown as {hostCapacity:unknown}).hostCapacity),real=defaultHostWardenPortsV1({hostPolicy,directory,observe:true});
 await runHostWardenLoopV1({ports:{...real,write:()=>undefined,sleep:milliseconds=>Bun.sleep(Math.min(milliseconds,100)),signal:()=>{throw Error("the overhead measurement never signals");}},maxTicks:ticks,code:"overhead",policy:{...HOST_WARDEN_POLICY_V1,idleMilliseconds:3600000}});
 process.exit(0);
}else{
 const {expect,test}=await import("bun:test");
 const cpuOf=(count:number,directoryOfRun:string):{cpuSeconds:number;state:HostWardenStateV1}=>{
  const done=Bun.spawnSync(["/usr/bin/time","-p",process.execPath,fileURLToPath(import.meta.url)],{env:{...process.env,WARDEN_OVERHEAD_TICKS:String(count),WARDEN_OVERHEAD_DIRECTORY:directoryOfRun},stderr:"pipe",stdout:"pipe"}),text=new TextDecoder().decode(done.stderr);
  expect(done.exitCode,text).toBe(0);
  const read=(name:string)=>Number(new RegExp(`^${name}\\s+([0-9.]+)`,"mu").exec(text)?.[1]??NaN);
  return{cpuSeconds:read("user")+read("sys"),state:JSON.parse(readFileSync(join(directoryOfRun,"warden","state.json"),"utf8")) as HostWardenStateV1};
 };
 test.skipIf(process.platform==="win32")("60 real observe ticks: p50, p95, CPU per tick and resident megabytes of the warden on this machine",()=>{
  const root=mkdtempSync(join(tmpdir(),"warden-overhead-"));
  try{
   const base=cpuOf(5,join(root,"a")),run=cpuOf(65,join(root,"b")),perTickMilliseconds=Math.max(0,(run.cpuSeconds-base.cpuSeconds)/60*1000),calm=HOST_WARDEN_POLICY_V1.calmMilliseconds,state=run.state;
   const report={ticks:state.counters.ticks,tickMillisecondsP50:state.overhead.tickMillisecondsP50,tickMillisecondsP95:state.overhead.tickMillisecondsP95,cpuMillisecondsPerTickWholeTree:Math.round(perTickMilliseconds*10)/10,cpuPercentAtCalmInterval:Math.round(perTickMilliseconds/calm*10000)/100,cpuPercentAtTightInterval:Math.round(perTickMilliseconds/HOST_WARDEN_POLICY_V1.tightMilliseconds*10000)/100,residentMegabytes:state.overhead.residentMegabytes,subjectsOnThisMachine:state.subjects.count,subjectMegabytes:state.subjects.megabytes,mode:state.mode,level:state.level,startupCpuSeconds:base.cpuSeconds};
   console.log("[overhead] "+JSON.stringify(report));
   expect(state.counters.ticks).toBe(65);
   expect(state.mode).toBe("observe");
   expect(state.counters.suspended+state.counters.cancelled+state.counters.resumed).toBe(0);
   expect(state.overhead.tickMillisecondsP95).toBeLessThan(2000);
   expect(state.overhead.residentMegabytes).toBeLessThan(400);
  }finally{rmSync(root,{recursive:true,force:true});}
 },180000);
}
