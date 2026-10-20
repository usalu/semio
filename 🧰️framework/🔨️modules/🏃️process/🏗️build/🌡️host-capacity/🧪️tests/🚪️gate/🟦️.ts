import {afterAll,expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {hostCapacityDirectoryV1,newHostCapacityIdV1,readHostCapacityPolicyV1,readHostCapacityStatusV1,withHostCapacityV1,type HostCapacityOptionsV1,type HostSampleV1} from "../../🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";

const base=readHostCapacityPolicyV1(decisionFixture.policy),policy=readHostCapacityPolicyV1({...decisionFixture.policy,load:{...decisionFixture.policy.load,overrideAfterMilliseconds:150},wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hc-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=(over:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10,...over});
const sampler=(current:()=>Partial<HostSampleV1>)=>async()=>({host:host({...current()}),swappedOutMegabytes:0});
const control=(signal=new AbortController().signal,remaining:()=>number|null=()=>null)=>{const published:unknown[]=[];return {published,value:{signal,remainingMilliseconds:remaining,publish:async(row:unknown)=>{published.push(row);},yieldContinuation:async()=>undefined}};};
const request=(over:object={})=>({id:newHostCapacityIdV1(),operation:"run",priority:"batch" as const,minimumJobs:1,preferredJobs:1,fixedMegabytes:8000,perJobMegabytes:0,cpuPerJob:1,...over});
const options=(directory:string,over:Partial<HostCapacityOptionsV1>={}):HostCapacityOptionsV1=>({directory,policy,request:request(),control:control().value,environment:{},write:()=>undefined,sample:sampler(()=>({})),...over});

test("Requests that do not fit together run one after the other in arrival order and the machine never holds more than its budget",async()=>{
 const directory=fresh(),order:string[]=[];let running=0,peak=0;
 const jobs=[0,1,2,3].map(async index=>{await sleep(index*15);return withHostCapacityV1(options(directory),async handle=>{running++;peak=Math.max(peak,running);order.push(handle.id);await sleep(40);running--;});});
 const ids=await Promise.all(jobs).then(()=>order);
 expect(peak).toBe(1);
 expect(ids.length).toBe(4);
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
 expect(readdirSync(join(directory,"queue"))).toEqual([]);
},30000);

test("Two requests that fit together run together and a third waits for one of them",async()=>{
 const directory=fresh();let running=0,peak=0;
 await Promise.all([0,1,2].map(async index=>{await sleep(index*15);return withHostCapacityV1(options(directory,{request:request({fixedMegabytes:5000})}),async()=>{running++;peak=Math.max(peak,running);await sleep(120);running--;});}));
 expect(peak).toBe(2);
},30000);

test("Waiting reports progress lines in English and German and publishes to the caller control",async()=>{
 const directory=fresh(),english:string[]=[],german:string[]=[],holder=control(),waiting=control();
 let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});
 const first=withHostCapacityV1(options(directory,{control:holder.value}),async()=>hold);
 await sleep(60);
 const second=withHostCapacityV1(options(directory,{control:waiting.value,write:line=>english.push(line),environment:{LANG:"en_US.UTF-8"}}),async()=>undefined);
 const third=withHostCapacityV1(options(directory,{write:line=>german.push(line),environment:{LANG:"de_DE.UTF-8"}}),async()=>undefined);
 await sleep(150);release();await Promise.all([first,second,third]);
 expect(english.some(line=>/^\[capacity\] waiting \d+s for run: (?:an earlier request is still waiting|not enough free memory)/u.test(line))).toBe(true);
 expect(english.at(-1)).toMatch(/^\[capacity\] admitted run: 1 job\(s\), 8000 MB reserved/u);
 expect(german.some(line=>/^\[capacity\] wartet seit \d+ s auf run: (?:eine frühere Anfrage wartet noch|nicht genug freier Speicher)/u.test(line))).toBe(true);
 expect(german.at(-1)).toMatch(/^\[capacity\] zugelassen run: 1 Job\(s\), 8000 MB reserviert/u);
 expect(waiting.published.length).toBeGreaterThan(0);
 expect(waiting.published[0]).toMatchObject({resource:"semio.host.capacity/v1",mode:"shared"});
},30000);

test("Cancelling a waiting request removes it from the queue, rejects with the caller's reason and leaves the holder alone",async()=>{
 const directory=fresh();let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});
 const first=withHostCapacityV1(options(directory),async()=>hold);
 await sleep(60);
 const controller=new AbortController(),waiting=withHostCapacityV1(options(directory,{control:control(controller.signal).value}),async()=>"never");
 await sleep(80);
 expect(readdirSync(join(directory,"queue")).length).toBe(1);
 controller.abort(Error("stop waiting"));
 await expect(waiting).rejects.toThrow("stop waiting");
 expect(readdirSync(join(directory,"queue")).length).toBe(0);
 expect(readdirSync(join(directory,"grants")).length).toBe(1);
 release();await first;
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
},30000);

test("An exhausted caller deadline stops the wait",async()=>{
 const directory=fresh();let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});
 const first=withHostCapacityV1(options(directory),async()=>hold);
 await sleep(60);
 const started=Date.now();
 await expect(withHostCapacityV1(options(directory,{control:control(new AbortController().signal,()=>150-(Date.now()-started)).value}),async()=>"never")).rejects.toThrow("deadline exhausted");
 release();await first;
});

test("A failing body still gives its reservation back",async()=>{
 const directory=fresh();
 await expect(withHostCapacityV1(options(directory),async()=>{throw Error("body failed");})).rejects.toThrow("body failed");
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
 await withHostCapacityV1(options(directory),async()=>undefined);
});

test("A grant of a dead process is reclaimed instead of blocking the machine",async()=>{
 const directory=fresh(),dead=spawnSync(process.execPath,["--eval","process.exit(0)"]),id=newHostCapacityIdV1();
 mkdirSync(join(directory,"grants"),{recursive:true});
 const bootAt=Math.round((Date.now()-require("node:os").uptime()*1000)/1000)*1000;
 writeFileSync(join(directory,"grants",`${id}.json`),JSON.stringify({id,operation:"run",jobs:1,cpu:1,reservedMegabytes:13000,admittedAtMs:Date.now(),observedMegabytes:null,pid:dead.pid,bootAtMs:bootAt}));
 const started=Date.now();
 await withHostCapacityV1(options(directory),async()=>undefined);
 expect(Date.now()-started).toBeLessThan(2000);
 expect(existsSync(join(directory,"grants",`${id}.json`))).toBe(false);
});

test("A grant from before a reboot is reclaimed even when its pid is alive again",async()=>{
 const directory=fresh(),id=newHostCapacityIdV1();
 mkdirSync(join(directory,"grants"),{recursive:true});
 writeFileSync(join(directory,"grants",`${id}.json`),JSON.stringify({id,operation:"run",jobs:1,cpu:1,reservedMegabytes:13000,admittedAtMs:1,observedMegabytes:null,pid:process.pid,bootAtMs:1000}));
 await withHostCapacityV1(options(directory),async()=>undefined);
 expect(existsSync(join(directory,"grants",`${id}.json`))).toBe(false);
});

test("A child inside a grant inherits it without queueing even when the machine is full",async()=>{
 const directory=fresh();let seen:unknown;
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:13000})}),async outer=>{
  expect(outer.environment.SEMIO_HOST_CAPACITY_GRANT).toBe(outer.id);
  const inner=await withHostCapacityV1(options(directory,{environment:outer.environment,request:request({fixedMegabytes:13000})}),async handle=>handle);
  seen=inner;expect(inner).toMatchObject({inherited:true,id:outer.id,jobs:outer.jobs});
 });
 expect(seen).toBeDefined();
 expect(readdirSync(join(directory,"queue"))).toEqual([]);
});

test("The governor can be switched off explicitly or by policy and says so",async()=>{
 const directory=fresh(),lines:string[]=[];
 await withHostCapacityV1(options(directory,{environment:{SEMIO_HOST_CAPACITY:"off"},write:line=>lines.push(line),sample:async()=>{throw Error("must not sample");}}),async handle=>expect(handle).toMatchObject({bypassed:true,reservedMegabytes:0}));
 expect(lines[0]).toContain("SEMIO_HOST_CAPACITY=off");
 await withHostCapacityV1(options(directory,{policy:{...policy,enabled:false}}),async handle=>expect(handle.bypassed).toBe(true));
});

test("The heartbeat records what the owner's process tree holds",async()=>{
 const directory=fresh();
 await withHostCapacityV1(options(directory,{observe:async()=>4321}),async handle=>{
  await sleep(120);
  const row=JSON.parse(readFileSync(join(directory,"grants",`${handle.id}.json`),"utf8"));
  expect(row.observedMegabytes).toBe(4321);
  const status=await readHostCapacityStatusV1({directory,policy,control:control().value,sample:sampler(()=>({}))});
  expect(status.grants.map(grant=>grant.observedMegabytes)).toEqual([4321]);
  expect(status.evaluation.unrealizedMegabytes).toBe(8000-4321);
 });
});

test("When nothing of ours runs a high load alone cannot starve a request forever, but memory pressure at the hard level does",async()=>{
 const directory=fresh(),lines:string[]=[];
 const started=Date.now();
 await withHostCapacityV1(options(directory,{sample:sampler(()=>({load:30})),write:line=>lines.push(line)}),async handle=>expect(handle.jobs).toBe(1));
 expect(Date.now()-started).toBeGreaterThanOrEqual(140);
 expect(lines.at(-1)).toMatch(/admitted run after \d+s although the load is above its ceiling/u);
 const controller=new AbortController();setTimeout(()=>controller.abort(Error("gave up")),250);
 await expect(withHostCapacityV1(options(directory,{control:control(controller.signal).value,sample:sampler(()=>({pressure:"critical"}))}),async()=>"never")).rejects.toThrow("gave up");
},30000);

test("Leaked disk images and an exhausted file table hold a request back, say why, and are overridden only when nothing of ours runs",async()=>{
 const directory=fresh(),lines:string[]=[];
 for(const [over,sentence] of [[{attachedImages:40},"too many disk images are attached"],[{openFiles:240000},"the machine has too many open files"]] as const){
  lines.length=0;
  const started=Date.now();
  await withHostCapacityV1(options(directory,{sample:sampler(()=>over),write:line=>lines.push(line)}),async handle=>expect(handle.jobs).toBe(1));
  expect(Date.now()-started).toBeGreaterThanOrEqual(140);
  expect(lines.some(line=>line.includes(`waiting`)&&line.includes(sentence)),sentence).toBe(true);
  expect(lines.at(-1),sentence).toContain(`although ${sentence}`);
 }
 let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});
 const first=withHostCapacityV1(options(directory,{request:request({fixedMegabytes:100})}),async()=>hold);
 await sleep(60);
 const controller=new AbortController();setTimeout(()=>controller.abort(Error("gave up")),300);
 await expect(withHostCapacityV1(options(directory,{control:control(controller.signal).value,sample:sampler(()=>({attachedImages:40})),request:request({fixedMegabytes:100})}),async()=>"never")).rejects.toThrow("gave up");
 release();await first;
},30000);

test("A waiter that cannot fit yet does not hold back a smaller one behind it, unless finishing our own grants would admit it and it has waited its patience",async()=>{
 const directory=fresh(),order:string[]=[];let release!:()=>void;const hold=new Promise<void>(resolve=>{release=resolve;});
 const holder=withHostCapacityV1(options(directory),async()=>hold);
 await sleep(60);
 const head=withHostCapacityV1(options(directory,{request:request({fixedMegabytes:13000})}),async()=>{order.push("head");});
 await sleep(40);
 await withHostCapacityV1(options(directory,{request:request({fixedMegabytes:1000})}),async()=>{order.push("small");});
 expect(order).toEqual(["small"]);
 expect(readdirSync(join(directory,"queue")).length).toBe(1);
 release();await holder;await head;
 expect(order).toEqual(["small","head"]);
 const strict=fresh(),patient={...policy,wait:{...policy.wait,bypassMilliseconds:0}};let free!:()=>void;const kept=new Promise<void>(resolve=>{free=resolve;});
 const first=withHostCapacityV1(options(strict,{policy:patient}),async()=>kept);
 await sleep(60);
 const big=withHostCapacityV1(options(strict,{policy:patient,request:request({fixedMegabytes:13000})}),async()=>"big");
 await sleep(40);
 const controller=new AbortController();setTimeout(()=>controller.abort(Error("gave up")),250);
 await expect(withHostCapacityV1(options(strict,{policy:patient,control:control(controller.signal).value,request:request({fixedMegabytes:1000})}),async()=>"never")).rejects.toThrow("gave up");
 free();await first;await big;
},30000);

test("The directory is per user and shared by every checkout, with an explicit override",()=>{
 expect(hostCapacityDirectoryV1({},"darwin","/Users/x")).toBe("/Users/x/Library/Caches/semio/host-capacity");
 expect(hostCapacityDirectoryV1({XDG_CACHE_HOME:"/c"},"linux","/home/x")).toBe("/c/semio/host-capacity");
 expect(hostCapacityDirectoryV1({},"linux","/home/x")).toBe("/home/x/.cache/semio/host-capacity");
 expect(hostCapacityDirectoryV1({SEMIO_HOST_CAPACITY_DIRECTORY:"/tmp/g"},"darwin","/Users/x")).toBe("/tmp/g");
 expect(hostCapacityDirectoryV1({LOCALAPPDATA:"C:\\L"},"win32","C:\\Users\\x")).toContain("semio");
});

test("Separate operating-system processes share one budget, a killed holder frees its reservation, and a killed holder or waiter leaves nothing behind",async()=>{
 const directory=fresh(),log=join(directory,"log.jsonl"),moduleUrl=new URL("../../🟦️.ts",import.meta.url).href,hostFile=join(directory,"host.json");
 writeFileSync(hostFile,JSON.stringify(host()));
 const script=join(directory,"child.ts");
 writeFileSync(script,`import {appendFileSync,readFileSync} from "node:fs";
import {setTimeout as sleep} from "node:timers/promises";
import {withHostCapacityV1} from ${JSON.stringify(moduleUrl)};
const [directory,logFile,hostFile,policyText,mode]=process.argv.slice(2);
const policy=JSON.parse(policyText);
const id=process.env.CHILD_ID;
await withHostCapacityV1({directory,policy,request:{id,operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:8000,perJobMegabytes:0,cpuPerJob:1},control:{signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined},environment:{},write:()=>undefined,sample:async()=>({host:{...JSON.parse(readFileSync(hostFile,"utf8")),atMs:Date.now()},swappedOutMegabytes:0})},async()=>{
 appendFileSync(logFile,JSON.stringify({id,event:"start",at:Date.now()})+"\\n");
 await sleep(mode==="hold"?600000:250);
 appendFileSync(logFile,JSON.stringify({id,event:"end",at:Date.now()})+"\\n");
});`);
 const child=(id:string,mode:string)=>spawn(process.execPath,[script,directory,log,hostFile,JSON.stringify(policy),mode],{env:{...process.env,CHILD_ID:id},stdio:"ignore"});
 const ids=[newHostCapacityIdV1(),newHostCapacityIdV1(),newHostCapacityIdV1()],children=ids.map((id,index)=>{const row=child(id,"short");return sleep(0).then(()=>row);});
 const spawned=await Promise.all(children);
 await Promise.all(spawned.map(row=>new Promise(resolve=>row.once("exit",resolve))));
 const events=readFileSync(log,"utf8").trim().split("\n").map(line=>JSON.parse(line)),intervals=ids.map(id=>({start:events.find(row=>row.id===id&&row.event==="start")!.at as number,end:events.find(row=>row.id===id&&row.event==="end")!.at as number})).sort((left,right)=>left.start-right.start);
 for(let index=1;index<intervals.length;index++)expect(intervals[index]!.start,`process ${index} starts after process ${index-1} ends`).toBeGreaterThanOrEqual(intervals[index-1]!.end);
 const holderId=newHostCapacityIdV1(),holder=child(holderId,"hold");
 for(let waited=0;waited<200&&!existsSync(join(directory,"grants",`${holderId}.json`));waited++)await sleep(25);
 expect(existsSync(join(directory,"grants",`${holderId}.json`))).toBe(true);
 const waiterId=newHostCapacityIdV1(),waiter=child(waiterId,"short");
 for(let waited=0;waited<400&&!readdirSync(join(directory,"queue")).some(name=>name.includes(waiterId));waited++)await sleep(25);
 expect(readdirSync(join(directory,"queue")).some(name=>name.includes(waiterId))).toBe(true);
 holder.kill("SIGKILL");
 await new Promise(resolve=>waiter.once("exit",resolve));
 const after=readFileSync(log,"utf8").trim().split("\n").map(line=>JSON.parse(line));
 expect(after.some(row=>row.id===waiterId&&row.event==="end")).toBe(true);
 const sleeper=child(newHostCapacityIdV1(),"hold");
 for(let waited=0;waited<400&&readdirSync(join(directory,"grants")).length<1;waited++)await sleep(25);
 const queued=child(newHostCapacityIdV1(),"short");
 for(let waited=0;waited<400&&readdirSync(join(directory,"queue")).length<1;waited++)await sleep(25);
 expect(readdirSync(join(directory,"queue")).length).toBe(1);
 queued.kill("SIGKILL");sleeper.kill("SIGKILL");
 await sleep(150);
 const status=await readHostCapacityStatusV1({directory,policy,control:control().value,sample:sampler(()=>({}))});
 expect(status.waiters).toEqual([]);
 expect(status.grants).toEqual([]);
},60000);
