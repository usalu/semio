/** 👻️ Contract: a client run whose parent process or standard output consumer is gone gives its grant back through the heartbeat of the gate.
 * @see ../../🎫️gate/👻️orphan/🟦️.ts */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {evaluateHostCapacityV1,newHostCapacityIdV1,readHostCapacityPolicyV1,readHostCapacityStatusV1,requestHostCapacityUpgradeV1,withHostCapacityV1,type HostCapacityHandleV1,type HostCapacityOptionsV1,type HostRequestV1,type HostRunV1,type HostSampleV1} from "../../🟦️.ts";
import {createHostOrphanWatchV1} from "../../🎫️gate/👻️orphan/🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";
const policy=readHostCapacityPolicyV1({...decisionFixture.policy,load:{...decisionFixture.policy.load,overrideAfterMilliseconds:600000},wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"q16-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const control=(signal=new AbortController().signal)=>({signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const sized=(id:string,fixed:number,over:object={}):HostRequestV1=>({id,operation:"run",priority:"batch",minimumJobs:1,preferredJobs:3,fixedMegabytes:fixed,perJobMegabytes:100,cpuPerJob:1,...over});
const options=(directory:string,request:HostRequestV1,over:Partial<HostCapacityOptionsV1>={}):HostCapacityOptionsV1=>({directory,policy,request,control:control(),environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample:async()=>({host:host(),swappedOutMegabytes:0}),...over});

test("The orphan watch is lost only when the output consumer hangs up: a reparented process with a live output keeps its grant",()=>{
 let closed=false,reparented=false;
 const watch=createHostOrphanWatchV1({outputClosed:()=>closed,stop:()=>undefined});
 expect(watch.lost()).toBe(false);reparented=true;expect(watch.lost(),"reparented ("+reparented+") with a live output").toBe(false);
 closed=true;expect(watch.lost(),"broken pipe").toBe(true);
});

test("The orphan watch of a process notes a broken pipe on its standard output and nothing else",async()=>{
 const {spawn}=await import("node:child_process"),source=fileURLToPath(new URL("../../🎫️gate/👻️orphan/🟦️.ts",import.meta.url));
 const script=`import {currentHostOrphanWatchV1} from ${JSON.stringify(source)};const watch=currentHostOrphanWatchV1();const before=watch.lost();process.stdout.write("x\\n".repeat(100000),()=>{});setTimeout(()=>{console.error(JSON.stringify({before,after:watch.lost()}));process.exit(0);},400);`;
 const file=join(fresh(),"watch.ts");writeFileSync(file,script);
 const run=(mode:"pipe-closed"|"file")=>new Promise<string>(resolve=>{
  const child=spawn(process.execPath,[file],{stdio:["ignore",mode==="file"?"ignore":"pipe","pipe"]});let err="";child.stderr!.on("data",chunk=>{err+=String(chunk);});
  if(mode==="pipe-closed")child.stdout!.destroy();
  child.on("close",()=>resolve(err));
 });
 expect(JSON.parse((await run("file")).trim().split("\n").pop()!),"stdout is the null device").toEqual({before:false,after:false});
 expect(JSON.parse((await run("pipe-closed")).trim().split("\n").pop()!),"reader hung up").toEqual({before:false,after:true});
});

test("A held grant whose client lost its output consumer is stopped by the heartbeat and its reservation is released when the run ends",async()=>{
 const directory=fresh();let orphaned=false,stops=0,release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});
 const done=withHostCapacityV1(options(directory,sized(newHostCapacityIdV1(),2048,{preferredJobs:1,perJobMegabytes:0}),{orphan:{lost:()=>orphaned,stop:()=>{stops++;release();}}}),async()=>hold);
 await sleep(60);expect(stops,"a run with a live parent is left alone").toBe(0);
 expect((await readHostCapacityStatusV1({directory,policy,control:control(),sample:async()=>({host:host(),swappedOutMegabytes:0})})).grants.length).toBe(1);
 orphaned=true;await sleep(120);await done;
 expect(stops).toBeGreaterThan(0);
 expect((await readHostCapacityStatusV1({directory,policy,control:control(),sample:async()=>({host:host(),swappedOutMegabytes:0})})).grants.length,"the grant is given back").toBe(0);
});
