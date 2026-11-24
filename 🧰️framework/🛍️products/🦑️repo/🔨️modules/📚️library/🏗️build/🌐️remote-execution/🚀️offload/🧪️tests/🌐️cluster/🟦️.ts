import{stretchedBudgetMillisecondsV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import {afterAll,beforeAll,expect,test} from "bun:test";
import {spawn,spawnSync,type ChildProcess} from "node:child_process";
import {chmodSync,constants,copyFileSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,readlinkSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../../../🌡️host-capacity/🎛️control/🟦️.ts";
import {wrapRustcV1} from "../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";
import {startClusterV1,eventuallyV1,type ClusterV1} from "../../../🧪️tests/🧰️cluster/🟦️.ts";
import {readOffloadOverheadV1,writeUnitCostV1} from "../../🚦️admission/🟦️.ts";
import {OFFLOAD_POLICY_V1} from "../../📐️policy/🟦️.ts";
import {createTestCheckoutV1,externHash,testHostProbeV1,testRustEnvironmentV1,type TestCheckoutV1} from "../🧰️workspace/🟦️.ts";

const here=fileURLToPath(new URL(".",import.meta.url)),entry=fileURLToPath(new URL("../../../🏃️entry/🟦️.ts",import.meta.url)),wrapper=fileURLToPath(new URL("../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts",import.meta.url));
const scratch=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-e2e-")));
process.env.SEMIO_CAS_ROOT=join(scratch,"cas");
const children=new Set<ChildProcess>();
let cluster:ClusterV1;
const workers=new Map<string,{child:ChildProcess;lines:string[];directory:string}>();

/** 📍️ A sysroot of the test toolchain cloned below `scratch` (own inodes: through a hard link the loader may resolve the executable to its original path, whose libraries the sandbox hides) whose `bin/rustc` records its pid in `compiler-<crate>.pid` in the action root before it becomes the real compiler, so a test sees the remote compiler start and die without scanning the process table; `--print sysroot` names it, so the worker offers it exactly like the real one (same identity, targets and linker). */
const signalingSysroot=((sysroot:string,root:string):string=>{
 const mirror=(from:string,to:string):void=>{mkdirSync(to,{recursive:true});for(const entry of readdirSync(from,{withFileTypes:true})){const source=join(from,entry.name),target=join(to,entry.name);if(entry.isDirectory()){if(!(entry.name==="src"&&from.endsWith("rustlib")))mirror(source,target);}else if(entry.isSymbolicLink())symlinkSync(readlinkSync(source),target);else copyFileSync(source,target,constants.COPYFILE_FICLONE);}};
 mirror(join(sysroot,"lib"),join(root,"lib"));mkdirSync(join(root,"libexec"));copyFileSync(join(sysroot,"bin","rustc"),join(root,"libexec","rustc"),constants.COPYFILE_FICLONE);mkdirSync(join(root,"bin"));
 writeFileSync(join(root,"bin","rustc"),`#!/bin/sh\n[ "$*" = "--print sysroot" ] && { echo '${root}'; exit 0; }\np=\nfor a;do [ "$p" = --crate-name ]&&echo $$ >".c$$"&&/bin/mv ".c$$" "compiler-$a.pid";p=$a;done\nexec '${root}/libexec/rustc' "$@"\n`);chmodSync(join(root,"bin","rustc"),0o755);
 return join(root,"bin");
})(spawnSync("rustc",["--print","sysroot"],{encoding:"utf8",env:testRustEnvironmentV1()}).stdout.trim(),join(scratch,"sysroot"));
const compilerPids=(directory:string,crate:string):number[]=>{try{return(readdirSync(directory,{recursive:true}) as string[]).filter(path=>path.endsWith(`compiler-${crate}.pid`)).map(path=>Number(readFileSync(join(directory,path),"utf8").trim())).filter(Number.isSafeInteger);}catch{return[];}};
const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch{return false;}};

const idleHost=join(scratch,"idle-host.json");
writeFileSync(idleHost,JSON.stringify({host:{version:1,atMs:Date.now()+86400000,platform:process.platform==="darwin"?"darwin":"linux",cores:16,totalMegabytes:65536,availableMegabytes:48000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0.5,openFiles:null,openFilesMaximum:null,attachedImages:null},grants:[],waiters:[]}));

function startWorker(name:string,directory:string):{child:ChildProcess;lines:string[];directory:string}{
 mkdirSync(directory,{recursive:true});
 const lines:string[]=[],child=spawn(process.execPath,[entry,"--join",cluster.server.workerUrl,"--name",name,"--origin",name,"--work-directory",join(directory,"work"),"--concurrency","2"],{cwd:directory,env:testRustEnvironmentV1({SEMIO_CAS_ROOT:undefined,SEMIO_REMOTE_EXECUTION_HOST_VIEW:idleHost,PATH:`${signalingSysroot}:${process.env.PATH}`}),stdio:["ignore","pipe","pipe"]});
 const collect=(chunk:Buffer)=>{lines.push(...chunk.toString().split("\n").filter(Boolean));};
 child.stdout!.on("data",collect);child.stderr!.on("data",collect);children.add(child);child.on("close",()=>children.delete(child));
 const row={child,lines,directory};workers.set(name,row);return row;
}

const fleetReady=async(label:string):Promise<void>=>{try{
 await eventuallyV1(async()=>{const status=await (await fetch(cluster.server.workerUrl+"/status")).json() as {workers?:unknown[]};return(status.workers?.length??0)>=2;},stretchedBudgetMillisecondsV1(60000),label+": both workers present");
 await eventuallyV1(async()=>{const response=await fetch(cluster.server.workerUrl+"/v1/fleet/load");if(!response.ok)return false;const table=await response.json() as {machines?:{origin:string;headroom?:boolean}[]};return["wA","wB"].every(name=>table.machines?.some(row=>row.origin===name&&row.headroom===true)===true);},stretchedBudgetMillisecondsV1(60000),label+": both machines report their load");
}catch(error){throw Error(`${(error as Error).message}; workers: ${JSON.stringify([...workers].map(([name,row])=>({name,exit:row.child.exitCode,signal:row.child.signalCode,lines:row.lines.slice(-12)})))}; scheduler: ${JSON.stringify(cluster.events.slice(-12))}`);}};
beforeAll(async()=>{
 cluster=await startClusterV1({workers:0,root:scratch,scheduler:{leaseSeconds:5,lingerSeconds:1,unmatchedSeconds:30}});
 for(const name of ["wA","wB"])startWorker(name,join(scratch,"worker-"+name));
 await eventuallyV1(async()=>{const status=await (await fetch(cluster.server.workerUrl+"/status")).json() as {workers?:unknown[]};return(status.workers?.length??0)>=2;},60000,"both worker processes join");
 await eventuallyV1(async()=>{const response=await fetch(cluster.server.workerUrl+"/v1/fleet/load");if(!response.ok)return false;const table=await response.json() as {machines?:{origin:string;headroom?:boolean}[]};return["wA","wB"].every(name=>table.machines?.some(row=>row.origin===name&&row.headroom===true)===true);},60000,"both machines report their load");
},stretchedBudgetMillisecondsV1(120000));
const reap=():void=>{for(const directory of readdirSync(scratch).filter(name=>name.startsWith("worker-")))for(const crate of["c","cx"])for(const pid of compilerPids(join(scratch,directory),crate))if(spawnSync("ps",["-p",String(pid),"-o","comm="],{encoding:"utf8"}).stdout.trim().endsWith("rustc"))try{process.kill(pid,"SIGKILL");}catch{}};
afterAll(async()=>{
 reap();
 for(const child of children)child.kill("SIGKILL");
 await cluster?.stop().catch(()=>{});
 rmSync(scratch,{recursive:true,force:true});
});

const countingRustc=(directory:string):{path:string;log:string}=>{
 const path=join(directory,"counting-rustc.sh"),log=join(directory,"rustc-calls.log");
 writeFileSync(path,`#!/bin/sh\necho "$*" >> '${log}.all'\ncase " $* " in *" --crate-name "*) echo call >> '${log}';; esac\nexec rustc "$@"\n`);chmodSync(path,0o755);
 return{path,log};
};
const callsOf=(log:string):number=>existsSync(log)?readFileSync(log,"utf8").split("\n").filter(Boolean).length:0;

function environmentOf(checkout:TestCheckoutV1,name:string,cacheRoot:string,extra:Record<string,string>={}):Record<string,string>{
 const counter=countingRustc(cacheRoot);
 return{...testRustEnvironmentV1(checkout.environment(name)),SEMIO_UNIT_CACHE_ROOT:cacheRoot,SEMIO_UNIT_CACHE_LANE:"t",SEMIO_UNIT_CACHE_INNER:JSON.stringify([counter.path]),SEMIO_UNIT_CACHE_SLOTS:"0",
  SEMIO_REMOTE_EXECUTION_ENDPOINT:cluster.server.grpcUrl,SEMIO_REMOTE_CACHE_ENDPOINT:cluster.cache.url.replace("//0.0.0.0","//127.0.0.1"),SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_CAS_ROOT:join(scratch,"cas-"+cacheRoot.split("/").at(-1)),...extra};
}

type Run={status:number|null;stdout:string;stderr:string};
let lastWrapper:ChildProcess|undefined;
function runWrapper(checkout:TestCheckoutV1,name:string,externs:readonly string[],cacheRoot:string,extra:Record<string,string>={}):Promise<Run&{child:ChildProcess}>{
 mkdirSync(cacheRoot,{recursive:true});
 const child=spawn(process.execPath,[wrapper,"rustc",...checkout.arguments(name,externs)],{cwd:checkout.root,env:environmentOf(checkout,name,cacheRoot,extra),stdio:["ignore","pipe","pipe"]});
 children.add(child);lastWrapper=child;const out:Buffer[]=[],err:Buffer[]=[];child.stdout!.on("data",chunk=>out.push(chunk));child.stderr!.on("data",chunk=>err.push(chunk));
 return new Promise(done=>child.on("close",status=>{children.delete(child);done({status,stdout:Buffer.concat(out).toString(),stderr:Buffer.concat(err).toString(),child});}));
}
const neutral=(text:string,checkout:TestCheckoutV1)=>text.split(checkout.base).join("<checkout>");
const progress=(cacheRoot:string):string=>existsSync(join(cacheRoot,"x","offload.progress"))?readFileSync(join(cacheRoot,"x","offload.progress"),"utf8"):"";
const events=(cacheRoot:string):Record<string,unknown>[]=>existsSync(join(cacheRoot,"x","offload.jsonl"))?readFileSync(join(cacheRoot,"x","offload.jsonl"),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line)):[];

function checkoutWith(label:string,crateSource:string,name="c"):TestCheckoutV1{
 const checkout=createTestCheckoutV1(label,scratch);
 checkout.compile("a","pub fn a()->u32{1}\n");checkout.compile("b","pub fn b()->u32{a::a()+1}\n",["a"]);
 checkout.crate(name,crateSource);
 return checkout;
}

test("with the local host forced to wait, a crate compiles entirely on another machine, the local machine spawns no rustc, and the result is an ordinary unit-cache entry under the requester's key",async()=>{
 const checkout=checkoutWith("remote","pub fn c()->u32{b::b()+1}\n"),root=join(scratch,"unit-remote");
 mkdirSync(root,{recursive:true});
 const probe=testHostProbeV1("critical-pressure-gigabyte-swap");
 const environment=environmentOf(checkout,"c",root,{SEMIO_OFFLOAD:"distribute"}),outputs=checkout.arguments("c",["b"]);
 const previous=process.cwd();process.chdir(checkout.root);
 let status:number;try{status=await wrapRustcV1(["rustc",...outputs],environment,checkout.root,{admission:{probe}});}finally{process.chdir(previous);}
 expect(status,JSON.stringify(events(root))+progress(root)).toBe(0);
 expect(callsOf(join(root,"rustc-calls.log")),"the local machine spawned no rustc "+JSON.stringify(events(root))+progress(root)+JSON.stringify(cluster.events.slice(-6))).toBe(0);
 const artifacts=readdirSync(checkout.deps).filter(name=>name.includes(`-${externHash("c")}`)&&!name.endsWith(".semio-key")).sort();
 expect(artifacts).toEqual([`c-${externHash("c")}.d`,`libc-${externHash("c")}.rlib`,`libc-${externHash("c")}.rmeta`,`libc-${externHash("c")}.rmeta.semio-deps`]);
 expect(progress(root)).toMatch(/offloaded to w[AB]/u);expect(progress(root)).toMatch(/auf w[AB] ausgelagert/u);
 expect(events(root).map(row=>row.kind)).toEqual(["offload-run"]);
 const moved=events(root)[0]!;expect(moved.bytesShipped as number,"the input files of the action are counted").toBeGreaterThan(100);expect(moved.bytesUploaded as number).toBeLessThanOrEqual(moved.bytesShipped as number+4096);expect(moved.bytesReturned as number,"the outputs that came back are counted").toBeGreaterThan(500);
 const dep=readFileSync(join(checkout.deps,`c-${externHash("c")}.d`),"utf8");
 expect(dep).toContain("pk/c/src/lib.rs");expect(dep).not.toContain(".semio-offload");
 const entries=readdirSync(join(root,"e")).flatMap(shard=>readdirSync(join(root,"e",shard)));
 expect(entries.length,"exactly one entry was stored").toBe(1);
 const local=spawnSync("rustc",["--crate-name","d","--edition=2021","--crate-type","lib","--emit=metadata","--extern",`c=${join(checkout.deps,`libc-${externHash("c")}.rlib`)}`,"-L",`dependency=${checkout.deps}`,"-","--out-dir",join(scratch,"probe")],{input:"pub fn d()->u32{c::c()}\n",encoding:"utf8",env:testRustEnvironmentV1()});
 expect(local.status,local.stderr).toBe(0);
},stretchedBudgetMillisecondsV1(180000));

test("the verdict of the door (the whole build was refused, SEMIO_OFFLOAD_PRESSURE=door) sends the unit away and queues it there although this unit alone would be admitted, and a unit cheaper than a round trip stays on a calm machine without that verdict",async()=>{
 const calm=testHostProbeV1("quiet-no-swap",true);
 const door=checkoutWith("door","pub fn c()->u32{b::b()+11}\n"),rootDoor=join(scratch,"unit-door"),outputs=door.arguments("c",["b"]);mkdirSync(rootDoor,{recursive:true});
 const previous=process.cwd();process.chdir(door.root);
 try{
  const status=await wrapRustcV1(["rustc",...outputs],environmentOf(door,"c",rootDoor,{SEMIO_OFFLOAD:"distribute",SEMIO_OFFLOAD_PRESSURE:"door"}),door.root,{admission:{probe:calm}});
  expect(status,JSON.stringify(events(rootDoor))+progress(rootDoor)).toBe(0);
  expect(callsOf(join(rootDoor,"rustc-calls.log")),"no local compiler "+JSON.stringify(events(rootDoor))).toBe(0);
  expect(events(rootDoor).map(row=>row.kind)).toEqual(["offload-run"]);
 }finally{process.chdir(previous);}
 const quiet=checkoutWith("calm-door","pub fn c()->u32{b::b()+12}\n"),rootQuiet=join(scratch,"unit-calm-door");mkdirSync(rootQuiet,{recursive:true});writeUnitCostV1(rootQuiet,"c",10);
 process.chdir(quiet.root);
 try{
  const status=await wrapRustcV1(["rustc",...quiet.arguments("c",["b"])],environmentOf(quiet,"c",rootQuiet,{SEMIO_OFFLOAD:"distribute"}),quiet.root,{admission:{probe:calm}});
  expect(status).toBe(0);expect(events(rootQuiet)).toEqual([]);expect(callsOf(join(rootQuiet,"rustc-calls.log"))).toBeGreaterThan(0);
 }finally{process.chdir(previous);}
},stretchedBudgetMillisecondsV1(180000));

test("a second checkout at another absolute path asks for the same unit and gets it without any compiler running: from the fleet unit cache or the action cache, never from a worker again",async()=>{
 const first=checkoutWith("twin-a","pub fn c()->u32{b::b()+7}\n"),second=checkoutWith("twin-b","pub fn c()->u32{b::b()+7}\n"),rootA=join(scratch,"unit-twin-a"),rootB=join(scratch,"unit-twin-b");
 expect(first.root).not.toBe(second.root);
 for(const name of readdirSync(first.deps))copyFileSync(join(first.deps,name),join(second.deps,name));
 const left=await runWrapper(first,"c",["b"],rootA,{SEMIO_OFFLOAD:"distribute"}),assignedBefore=cluster.events.filter(event=>event.startsWith("assigned")).length;
 const right=await runWrapper(second,"c",["b"],rootB,{SEMIO_OFFLOAD:"distribute"});
 expect([left.status,right.status]).toEqual([0,0]);
 expect(callsOf(join(rootA,"rustc-calls.log"))+callsOf(join(rootB,"rustc-calls.log"))).toBe(0);
 expect(cluster.events.filter(event=>event.startsWith("assigned")).length).toBe(assignedBefore);
 const served=events(rootB)[0];if(served)expect(String(served.origin)).toMatch(/\(cache\)$/u);
 expect(String(events(rootA)[0]?.origin)).not.toMatch(/\(cache\)$/u);
 expect(readFileSync(join(second.deps,`c-${externHash("c")}.d`),"utf8")).toContain(`pk/c/src/lib.rs`);
 expect(neutral(left.stderr,first)).toBe(neutral(right.stderr,second));
},stretchedBudgetMillisecondsV1(180000));

test("with a calm local host a unit worth a round trip still goes to the fleet when another machine has headroom (the local slots are one more worker), a unit cheaper than a round trip stays, and never stays local whatever the fleet offers",async()=>{
 const checkout=checkoutWith("calm","pub fn c()->u32{b::b()+2}\n"),root=join(scratch,"unit-calm");
 mkdirSync(root,{recursive:true});
 const probe=testHostProbeV1("quiet-no-swap",true);
 const policy={...BUILD_HOST_CAPACITY_POLICY_V1,load:{...BUILD_HOST_CAPACITY_POLICY_V1.load,perCoreCeiling:10000}};
 const previous=process.cwd();process.chdir(checkout.root);
 try{expect(await wrapRustcV1(["rustc",...checkout.arguments("c",["b"])],environmentOf(checkout,"c",root,{SEMIO_OFFLOAD:"distribute"}),checkout.root,{admission:{probe,policy}})).toBe(0);}finally{process.chdir(previous);}
 expect(callsOf(join(root,"rustc-calls.log")),JSON.stringify(events(root))).toBe(0);expect(events(root).map(row=>row.kind)).toEqual(["offload-run"]);
 const cheap=join(scratch,"unit-cheap");mkdirSync(cheap,{recursive:true});writeUnitCostV1(cheap,"c",10);
 const small=checkoutWith("cheap","pub fn c()->u32{b::b()+4}\n");
 const stayed=await runWrapper(small,"c",["b"],cheap,{SEMIO_OFFLOAD:"distribute"});
 expect(stayed.status).toBe(0);expect(callsOf(join(cheap,"rustc-calls.log"))).toBe(1);expect(events(cheap)).toEqual([]);
 const never=join(scratch,"unit-never");mkdirSync(never,{recursive:true});
 const second=checkoutWith("never","pub fn c()->u32{b::b()+3}\n");
 const run=await runWrapper(second,"c",["b"],never,{SEMIO_OFFLOAD:"never"});
 expect(run.status).toBe(0);expect(callsOf(join(never,"rustc-calls.log"))).toBe(1);expect(events(never)).toEqual([]);
},stretchedBudgetMillisecondsV1(180000));

test("local first: a unit takes a free local slot at once, only a unit worth a round trip that would wait for a slot goes to a worker, a cheap one waits here, so a local and a remote unit are in flight at the same time",async()=>{
 const checkout=checkoutWith("first","pub fn c()->u32{b::b()+20}\n");checkout.crate("d","pub fn d()->u32{b::b()+21}\n");checkout.crate("e","pub fn e()->u32{b::b()+22}\n");
 const root=join(scratch,"unit-first");mkdirSync(root,{recursive:true});
 const slow=join(root,"slow-rustc.sh"),log=join(root,"rustc-calls.log");
 writeFileSync(slow,`#!/bin/sh\ncase " $* " in *" --crate-name "*) echo "$*" | sed 's/.*--crate-name \\([a-z]*\\).*/\\1/' >> '${log}';; esac\ncase " $* " in *" --crate-name c "*) sleep 8;; esac\nexec rustc "$@"\n`);chmodSync(slow,0o755);
 writeUnitCostV1(root,"c",60000);writeUnitCostV1(root,"d",60000);writeUnitCostV1(root,"e",10);
 const extra={SEMIO_OFFLOAD:"distribute",SEMIO_UNIT_CACHE_INNER:JSON.stringify([slow]),SEMIO_UNIT_CACHE_SLOTS:"1",SEMIO_UNIT_CACHE_LIGHT_SLOTS:"4"};
 const local=runWrapper(checkout,"c",["b"],root,extra),localStarted=Date.now();
 await eventuallyV1(async()=>existsSync(log)&&readFileSync(log,"utf8").includes("c"),30000,"unit c holds the only local slot");
 const remote=await runWrapper(checkout,"d",["b"],root,extra),remoteEnded=Date.now();
 expect(remote.status,remote.stderr+JSON.stringify(events(root))).toBe(0);
 expect(events(root).filter(row=>row.kind==="offload-run").map(row=>row.crate),"the unit that would have waited went to a worker "+JSON.stringify(events(root))).toEqual(["d"]);
 const moved=events(root).find(row=>row.kind==="offload-run")!;
 expect(String(moved.detail),"the round trip reports what the worker did and what it did not").toMatch(/worker fetch \d+ ms, run \d+ ms, upload \d+ ms, away from it \d+ ms of which token wait \d+ ms/u);
 expect(readOffloadOverheadV1(root),"the overhead of the round trip is learned").not.toBe(OFFLOAD_POLICY_V1.limits.roundTripMilliseconds);
 const cheap=runWrapper(checkout,"e",["b"],root,extra);
 const first=await local;expect(first.status,first.stderr).toBe(0);
 expect(remoteEnded,"the remote unit finished while the local one still compiled").toBeLessThan(localStarted+8000);
 const last=await cheap;expect(last.status,last.stderr).toBe(0);
 expect(readFileSync(log,"utf8").split("\n").filter(Boolean),"c and the cheap e compiled here, d did not").toEqual(["c","e"]);
 expect(events(root).filter(row=>row.kind==="offload-run").map(row=>row.crate)).toEqual(["d"]);
},stretchedBudgetMillisecondsV1(240000));

test("an expensive unit goes to a worker that has room although a local slot is free, and a cheap unit takes the free slot and compiles here",async()=>{
 const checkout=checkoutWith("expensive","pub fn c()->u32{b::b()+30}\n");checkout.crate("d","pub fn d()->u32{b::b()+31}\n");
 const root=join(scratch,"unit-expensive");mkdirSync(root,{recursive:true});
 writeUnitCostV1(root,"c",OFFLOAD_POLICY_V1.finals.minimumMilliseconds);writeUnitCostV1(root,"d",10);
 const extra={SEMIO_OFFLOAD:"distribute",SEMIO_UNIT_CACHE_SLOTS:"4",SEMIO_UNIT_CACHE_LIGHT_SLOTS:"4",SEMIO_REMOTE_EXECUTION_SCHEDULER:cluster.server.workerUrl,SEMIO_WORKSPACE_ROOT:root,SEMIO_FLEET_ORIGIN:"requester"};
 const expensive=await runWrapper(checkout,"c",["b"],root,extra);
 expect(expensive.status,expensive.stderr+JSON.stringify(events(root))).toBe(0);
 expect(events(root).filter(row=>row.kind==="offload-run").map(row=>row.crate),"the expensive unit went to a worker with every local slot free "+JSON.stringify(events(root))).toEqual(["c"]);
 const cheap=await runWrapper(checkout,"d",["b"],root,extra);
 expect(cheap.status,cheap.stderr).toBe(0);
 expect(events(root).filter(row=>row.kind==="offload-run").map(row=>row.crate),"the cheap unit stayed here").toEqual(["c"]);
 expect(callsOf(join(root,"rustc-calls.log")),"the expensive unit spawned no local rustc, the cheap one exactly one").toBe(1);
},stretchedBudgetMillisecondsV1(240000));

test("a pinned build compiles every unit on the pinned worker and on no other, reports a pin nobody offers as a fall-back with its reason, and still builds the unit here",async()=>{
 for(const target of ["wA","wB"]){
  const checkout=checkoutWith("pin-"+target,"pub fn c()->u32{b::b()+5}\n"),root=join(scratch,"unit-pin-"+target);
  const run=await runWrapper(checkout,"c",["b"],root,{SEMIO_OFFLOAD:"pin:"+target});
  expect(run.status,run.stderr+JSON.stringify(events(root))).toBe(0);expect(callsOf(join(root,"rustc-calls.log"))).toBe(0);
  const rows=events(root);expect(rows.map(row=>row.kind)).toEqual(["offload-run"]);expect(rows[0]!.origin,target).toBe(target);
 }
 const lost=checkoutWith("pin-none","pub fn c()->u32{b::b()+6}\n"),root=join(scratch,"unit-pin-none");
 const run=await runWrapper(lost,"c",["b"],root,{SEMIO_OFFLOAD:"pin:wZ"});
 expect(run.status,run.stderr).toBe(0);expect(callsOf(join(root,"rustc-calls.log")),"the build is never failed: the unit compiles here").toBe(1);
 const rows=events(root);expect(rows.map(row=>row.kind)).toEqual(["offload-local-fallback"]);expect(rows[0]).toMatchObject({reason:"pin-failed"});expect(String(rows[0]!.detail)).toContain("wZ");
 expect(progress(root)).toMatch(/the pinned machine could not take or finish it/u);expect(progress(root)).toMatch(/die festgelegte Maschine konnte sie nicht annehmen/u);
},stretchedBudgetMillisecondsV1(240000));

test("every eligibility refusal is an offload-declined event with its reason, a counter and English and German lines: a crate type the fleet cannot take (a static library for WebAssembly) and a source that is not a Rust file",async()=>{
 const checkout=checkoutWith("refused","pub fn c()->u32{b::b()+7}\n"),root=join(scratch,"unit-refused");
 const run=await runWrapper(checkout,"c",["b"],root,{SEMIO_OFFLOAD:"distribute",SEMIO_UNIT_CACHE_WORKSPACE_ROOT:checkout.base});
 expect(run.status).toBe(0);
 const linked=checkoutWith("refused-cdylib","pub fn c()->u32{b::b()+8}\n"),linkedRoot=join(scratch,"unit-refused-cdylib");mkdirSync(linkedRoot,{recursive:true});
 const child=spawn(process.execPath,[wrapper,"rustc",...linked.arguments("c",["b"],["--target","wasm32-unknown-unknown","--crate-type","staticlib"])],{cwd:linked.root,env:environmentOf(linked,"c",linkedRoot,{SEMIO_OFFLOAD:"distribute"}),stdio:["ignore","pipe","pipe"]});
 children.add(child);await new Promise<void>(done=>child.on("close",()=>{children.delete(child);done();}));
 const rows=events(linkedRoot).filter(row=>row.kind==="offload-declined");
 expect(rows.map(row=>row.reason),JSON.stringify(events(linkedRoot))).toContain("crate-type");
 expect(String(rows.find(row=>row.reason==="crate-type")!.detail)).toContain("staticlib");
 expect(progress(linkedRoot)).toMatch(/its crate type is linked here/u);expect(progress(linkedRoot)).toMatch(/sein Crate-Typ wird hier gelinkt/u);
},stretchedBudgetMillisecondsV1(180000));

test("diagnostics of a crate with warnings and of a crate with an error are byte-identical to a local build, with the compiler's exit status",async()=>{
 for(const [label,source,expected] of [["warn","pub fn c()->u32{let unused=1;let also=2;b::b()}\n",0],["error","pub fn c()->u32{\"text\"}\n",1]] as const){
  const remote=checkoutWith("diag-r-"+label,source),local=checkoutWith("diag-l-"+label,source),remoteRoot=join(scratch,"unit-diag-r-"+label),localRoot=join(scratch,"unit-diag-l-"+label);
  const left=await runWrapper(remote,"c",["b"],remoteRoot,{SEMIO_OFFLOAD:"distribute"}),right=await runWrapper(local,"c",["b"],localRoot,{SEMIO_OFFLOAD:"never"});
  expect(callsOf(join(remoteRoot,"rustc-calls.log")),label).toBe(0);expect(callsOf(join(localRoot,"rustc-calls.log")),label).toBe(1);
  expect(left.status,label).toBe(expected);expect(right.status,label).toBe(expected);
  expect(neutral(left.stderr,remote),label).toBe(neutral(right.stderr,local));
  expect(neutral(left.stdout,remote),label).toBe(neutral(right.stdout,local));
  expect(left.stderr.length,label).toBeGreaterThan(0);
  if(label==="warn")expect(left.stderr).toContain("unused variable");
  if(label==="error")expect(left.stderr).toContain("mismatched types");
  if(expected===1)expect(readdirSync(remote.deps).some(name=>name.includes(`-${externHash("c")}`)),"a failed unit leaves no artifacts").toBe(false);
 }
},stretchedBudgetMillisecondsV1(240000));

test("a proc-macro crate links on another machine and the dylib it returns loads into a local compiler",async()=>{
 const checkout=createTestCheckoutV1("procmacro",scratch),root=join(scratch,"unit-pm");
 checkout.crate("pm","extern crate proc_macro;\nuse proc_macro::TokenStream;\n#[proc_macro]\npub fn answer(_input:TokenStream)->TokenStream{\"42u32\".parse().unwrap()}\n");
 const asMacro=(args:string[])=>[...args.map((value,index)=>args[index-1]==="--crate-type"?"proc-macro":value),"--extern","proc_macro"];
 mkdirSync(root,{recursive:true});
 const child=spawn(process.execPath,[wrapper,"rustc",...asMacro(checkout.arguments("pm"))],{cwd:checkout.root,env:environmentOf(checkout,"pm",root,{SEMIO_OFFLOAD:"distribute"}),stdio:["ignore","pipe","pipe"]});
 children.add(child);const err:Buffer[]=[];child.stderr!.on("data",chunk=>err.push(chunk));
 const status=await new Promise<number|null>(done=>child.on("close",done));
 expect(status,Buffer.concat(err).toString()+progress(root)+JSON.stringify(events(root))).toBe(0);
 expect(callsOf(join(root,"rustc-calls.log"))).toBe(0);
 const library=readdirSync(checkout.deps).find(name=>name.startsWith(`libpm-${externHash("pm")}`)&&/\.(dylib|so)$/u.test(name));
 expect(library,readdirSync(checkout.deps).join(",")).toBeDefined();
 mkdirSync(join(scratch,"pm-user"),{recursive:true});
 const again=spawnSync("rustc",["--edition=2021","--crate-type","lib","--emit=metadata","--extern",`pm=${join(checkout.deps,library!)}`,"-","--out-dir",join(scratch,"pm-user")],{input:"pub fn f()->u32{pm::answer!()}\n",encoding:"utf8",env:testRustEnvironmentV1(),cwd:scratch});
 expect(again.status,again.stderr).toBe(0);
},stretchedBudgetMillisecondsV1(180000));

test("a unit whose proc-macro cannot find its inputs in the private root of the other machine (the derive answers E0463 there) is compiled locally and produces the artifact",async()=>{
 const checkout=createTestCheckoutV1("incomplete",scratch),macroRoot=join(scratch,"unit-inc-pm"),root=join(scratch,"unit-inc");
 checkout.crate("pm","extern crate proc_macro;\nuse proc_macro::TokenStream;\n#[proc_macro]\npub fn answer(_input:TokenStream)->TokenStream{if std::env::var_os(\"SEMIO_OFFLOAD_INPUT_ROOT\").is_some(){\"extern crate semio_offload_inputs_incomplete;\"}else{\"pub const V:u32=42;\"}.parse().unwrap()}\n");
 mkdirSync(macroRoot,{recursive:true});
 const built=await new Promise<number|null>(done=>{const child=spawn(process.execPath,[wrapper,"rustc",...checkout.arguments("pm").map((value,index,all)=>all[index-1]==="--crate-type"?"proc-macro":value),"--extern","proc_macro"],{cwd:checkout.root,env:environmentOf(checkout,"pm",macroRoot,{SEMIO_OFFLOAD:"never"}),stdio:"ignore"});children.add(child);child.on("close",done);});
 expect(built).toBe(0);
 const library=readdirSync(checkout.deps).find(name=>name.startsWith(`libpm-${externHash("pm")}`)&&/\.(dylib|so)$/u.test(name))!;
 checkout.crate("c","pm::answer!();\npub fn c()->u32{V}\n");
 mkdirSync(root,{recursive:true});
 const child=spawn(process.execPath,[wrapper,"rustc",...checkout.arguments("c",[],["--extern",`pm=${join(checkout.deps,library)}`])],{cwd:checkout.root,env:environmentOf(checkout,"c",root,{SEMIO_OFFLOAD:"distribute"}),stdio:["ignore","pipe","pipe"]});
 children.add(child);const err:Buffer[]=[];child.stderr!.on("data",chunk=>err.push(chunk));
 const status=await new Promise<number|null>(done=>child.on("close",done));
 expect(status,Buffer.concat(err).toString()+progress(root)+JSON.stringify(events(root))).toBe(0);
 expect(events(root).map(row=>row.reason)).toEqual(["incomplete-inputs"]);
 expect(callsOf(join(root,"rustc-calls.log"))).toBe(1);
 expect(readdirSync(checkout.deps).some(name=>name.startsWith(`libc-${externHash("c")}`)&&name.endsWith(".rlib"))).toBe(true);
},stretchedBudgetMillisecondsV1(180000));

test("a result that embeds the other machine's path is never stored: the unit falls back to a local compile and the entry holds the requester's own path",async()=>{
 const checkout=checkoutWith("pathdep","pub const D:&str=env!(\"CARGO_MANIFEST_DIR\");\n"),root=join(scratch,"unit-pathdep");
 const run=await runWrapper(checkout,"c",["b"],root,{SEMIO_OFFLOAD:"distribute"});
 expect(run.status,run.stderr).toBe(0);
 expect(events(root).map(row=>row.reason)).toEqual(["path-dependent-output"]);
 expect(callsOf(join(root,"rustc-calls.log"))).toBe(1);
 const rlib=readFileSync(join(checkout.deps,`libc-${externHash("c")}.rlib`));
 expect(rlib.includes(Buffer.from(join(checkout.root,"pk","c")))).toBe(true);
 expect(rlib.includes(Buffer.from("/work/"))).toBe(false);
},stretchedBudgetMillisecondsV1(180000));

test("a unit Cargo compiles in its package directory whose #[path] module lies in another top-level tree (the module directory is the workspace root) runs on a worker with the module shipped, and its rlib is byte-identical to a local build at another checkout path",async()=>{
 const source='#[path = "../../../x/m.rs"]\nmod m;\npub fn c()->u32{b::b()+m::m()}\n';
 const make=(label:string)=>{const checkout=checkoutWith(label,source);mkdirSync(join(checkout.root,"x"),{recursive:true});writeFileSync(join(checkout.root,"x","m.rs"),"pub fn m()->u32{if false{panic!(\"{}\",file!())}9}\n");return checkout;};
 const run=(checkout:TestCheckoutV1,root:string,offload:string):Promise<Run>=>{
  mkdirSync(root,{recursive:true});const manifest=join(checkout.root,"pk","c");
  const args=checkout.arguments("c",["b"]).map(argument=>argument==="pk/c/src/lib.rs"?join(manifest,"src","lib.rs"):argument);
  const child=spawn(process.execPath,[wrapper,"rustc",...args],{cwd:manifest,env:environmentOf(checkout,"c",root,{SEMIO_OFFLOAD:offload,SEMIO_UNIT_CACHE_WORKSPACE_ROOT:checkout.root}),stdio:["ignore","pipe","pipe"]});
  children.add(child);const out:Buffer[]=[],err:Buffer[]=[];child.stdout!.on("data",chunk=>out.push(chunk));child.stderr!.on("data",chunk=>err.push(chunk));
  return new Promise(done=>child.on("close",status=>{children.delete(child);done({status,stdout:Buffer.concat(out).toString(),stderr:Buffer.concat(err).toString()});}));
 };
 const remote=make("modroot-r"),local=make("modroot-l"),remoteRoot=join(scratch,"unit-modroot-r"),localRoot=join(scratch,"unit-modroot-l");
 const left=await run(remote,remoteRoot,"distribute"),right=await run(local,localRoot,"never");
 expect(left.status,left.stderr+JSON.stringify(events(remoteRoot))).toBe(0);expect(right.status,right.stderr).toBe(0);
 expect(events(remoteRoot).map(row=>row.kind),JSON.stringify(events(remoteRoot))).toEqual(["offload-run"]);
 expect(callsOf(join(remoteRoot,"rustc-calls.log"))).toBe(0);expect(callsOf(join(localRoot,"rustc-calls.log"))).toBe(1);
 const name=`libc-${externHash("c")}.rlib`;
 expect(Buffer.compare(readFileSync(join(remote.deps,name)),readFileSync(join(local.deps,name))),"the rlib of the worker against the local one").toBe(0);
},stretchedBudgetMillisecondsV1(240000));

const SLOW="#![allow(long_running_const_eval)]\nconst X:u64={let mut i=0u64;let mut s=0u64;while i<4_000_000{s=s.wrapping_add(i);i+=1;}s};\npub fn c()->u64{X}\n";

test("killing the worker that runs the unit mid-compilation still completes the unit, elsewhere or locally",async()=>{
 await fleetReady("before kill");
 const checkout=checkoutWith("kill",SLOW),root=join(scratch,"unit-kill");
 const started=cluster.events.length,running=runWrapper(checkout,"c",["b"],root,{SEMIO_OFFLOAD:"distribute"});
 const assigned=()=>cluster.events.slice(started).map(event=>/-> (\S+)$/u.exec(event)?.[1]).find(Boolean);
 let settled=false;void running.then(()=>{settled=true;});
 await eventuallyV1(()=>assigned()!==undefined,stretchedBudgetMillisecondsV1(60000),"a worker is assigned the unit");
 const victim=assigned()!,directory=workers.get(victim)!.directory;
 await eventuallyV1(()=>settled||compilerPids(directory,"c").some(alive),stretchedBudgetMillisecondsV1(60000),"the remote compiler runs");
 expect(compilerPids(directory,"c").some(alive),"the compiler on the assigned worker runs while the unit is unfinished").toBe(true);
 workers.get(victim)!.child.kill("SIGKILL");
 const result=await running;
 expect(result.status,result.stderr).toBe(0);
 expect(readdirSync(checkout.deps).some(name=>name.startsWith(`libc-${externHash("c")}`)&&name.endsWith(".rlib"))).toBe(true);
 const text=progress(root),locally=callsOf(join(root,"rustc-calls.log"));
 expect(text.includes("offloaded to")||locally>0,text).toBe(true);
 if(text.includes("offloaded to"))expect(text).not.toContain(`offloaded to ${victim}`);
 expect(cluster.events.length).toBeGreaterThan(started);
 reap();
 workers.delete(victim);
 startWorker(victim,join(scratch,"worker-"+victim+"-again"));
 await fleetReady("after kill");
},stretchedBudgetMillisecondsV1(240000));

test("cancelling the build (SIGTERM to the wrapper) cancels the remote action: the compiler on the other machine is killed",async()=>{
 reap();
 await fleetReady("before cancel");
 const checkout=checkoutWith("cancel",SLOW.replace("4_000_000","4_000_000_000"),"cx"),root=join(scratch,"unit-cancel");
 const running=runWrapper(checkout,"cx",["b"],root,{SEMIO_OFFLOAD:"distribute"}),child=lastWrapper!;
 const started=():number[]=>[...workers.values()].flatMap(row=>compilerPids(row.directory,"cx")).filter(alive);
 await eventuallyV1(()=>started().length>0,stretchedBudgetMillisecondsV1(90000),"the remote compiler runs");
 const [compiler]=started();
 expect(alive(compiler!)).toBe(true);
 child.kill("SIGTERM");
 const result=await running;
 expect(result.status).toBe(143);
 await eventuallyV1(()=>!alive(compiler!),stretchedBudgetMillisecondsV1(30000),"the remote compiler is killed");
},stretchedBudgetMillisecondsV1(240000));
