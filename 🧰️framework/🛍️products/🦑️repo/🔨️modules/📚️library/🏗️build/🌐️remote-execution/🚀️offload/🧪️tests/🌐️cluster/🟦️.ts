import {afterAll,beforeAll,expect,test} from "bun:test";
import {spawn,spawnSync,type ChildProcess} from "node:child_process";
import {chmodSync,copyFileSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../../../🌡️host-capacity/🎛️control/🟦️.ts";
import {wrapRustcV1} from "../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";
import {startClusterV1,eventuallyV1,type ClusterV1} from "../../../🧪️tests/🧰️cluster/🟦️.ts";
import {createTestCheckoutV1,externHash,testHostProbeV1,testRustEnvironmentV1,type TestCheckoutV1} from "../🧰️workspace/🟦️.ts";

const here=fileURLToPath(new URL(".",import.meta.url)),entry=fileURLToPath(new URL("../../../🏃️entry/🟦️.ts",import.meta.url)),wrapper=fileURLToPath(new URL("../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts",import.meta.url));
const scratch=realpathSync(mkdtempSync(join(tmpdir(),"offload-e2e-")));
process.env.SEMIO_CAS_ROOT=join(scratch,"cas");
const children=new Set<ChildProcess>();
let cluster:ClusterV1;
const workers=new Map<string,{child:ChildProcess;lines:string[]}>();

const idleHost=join(scratch,"idle-host.json");
writeFileSync(idleHost,JSON.stringify({host:{version:1,atMs:Date.now()+86400000,platform:process.platform==="darwin"?"darwin":"linux",cores:16,totalMegabytes:65536,availableMegabytes:48000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0.5,openFiles:null,openFilesMaximum:null,attachedImages:null},grants:[],waiters:[]}));

function startWorker(name:string,directory:string):{child:ChildProcess;lines:string[]}{
 mkdirSync(directory,{recursive:true});
 const lines:string[]=[],child=spawn(process.execPath,[entry,"--join",cluster.server.workerUrl,"--name",name,"--origin",name,"--work-directory",join(directory,"work"),"--concurrency","2"],{cwd:directory,env:testRustEnvironmentV1({SEMIO_CAS_ROOT:undefined,SEMIO_REMOTE_EXECUTION_HOST_VIEW:idleHost}),stdio:["ignore","pipe","pipe"]});
 const collect=(chunk:Buffer)=>{lines.push(...chunk.toString().split("\n").filter(Boolean));};
 child.stdout!.on("data",collect);child.stderr!.on("data",collect);children.add(child);child.on("close",()=>children.delete(child));
 const row={child,lines};workers.set(name,row);return row;
}

beforeAll(async()=>{
 cluster=await startClusterV1({workers:0,root:scratch,scheduler:{leaseSeconds:5,lingerSeconds:1,unmatchedSeconds:30}});
 for(const name of ["wA","wB"])startWorker(name,join(scratch,"worker-"+name));
 await eventuallyV1(async()=>{const status=await (await fetch(cluster.server.workerUrl+"/status")).json() as {workers?:unknown[]};return(status.workers?.length??0)>=2;},60000,"both worker processes join");
 await eventuallyV1(async()=>{const response=await fetch(cluster.server.workerUrl+"/v1/fleet/load");if(!response.ok)return false;const table=await response.json() as {machines?:{origin:string;headroom?:boolean}[]};return["wA","wB"].every(name=>table.machines?.some(row=>row.origin===name&&row.headroom===true)===true);},60000,"both machines report their load");
},120000);
const orphans=():number[]=>spawnSync("ps",["-ax","-o","pid=,command="],{encoding:"utf8"}).stdout.split("\n").filter(line=>line.includes(scratch)&&line.includes(".semio-offload/o")&&line.includes("--crate-name")).map(line=>Number(line.trim().split(/\s+/u)[0])).filter(Number.isSafeInteger);
const reap=():void=>{for(const pid of orphans())try{process.kill(pid,"SIGKILL");}catch{}};
afterAll(async()=>{
 reap();
 for(const child of children)child.kill("SIGKILL");
 await cluster?.stop().catch(()=>{});
 rmSync(scratch,{recursive:true,force:true});
});

const countingRustc=(directory:string):{path:string;log:string}=>{
 const path=join(directory,"counting-rustc.sh"),log=join(directory,"rustc-calls.log");
 writeFileSync(path,`#!/bin/sh\ncase " $* " in *" -vV "*) ;; *) echo call >> '${log}';; esac\nexec rustc "$@"\n`);chmodSync(path,0o755);
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
 const environment=environmentOf(checkout,"c",root,{SEMIO_OFFLOAD:"auto"}),outputs=checkout.arguments("c",["b"]);
 const previous=process.cwd();process.chdir(checkout.root);
 let status:number;try{status=await wrapRustcV1(["rustc",...outputs],environment,checkout.root,{admission:{probe}});}finally{process.chdir(previous);}
 expect(status,JSON.stringify(events(root))+progress(root)).toBe(0);
 expect(callsOf(join(root,"rustc-calls.log")),"the local machine spawned no rustc "+JSON.stringify(events(root))+progress(root)+JSON.stringify(cluster.events.slice(-6))).toBe(0);
 const artifacts=readdirSync(checkout.deps).filter(name=>name.includes(`-${externHash("c")}`)).sort();
 expect(artifacts).toEqual([`c-${externHash("c")}.d`,`libc-${externHash("c")}.rlib`,`libc-${externHash("c")}.rmeta`]);
 expect(progress(root)).toMatch(/offloaded to w[AB]/u);expect(progress(root)).toMatch(/auf w[AB] ausgelagert/u);
 expect(events(root).map(row=>row.kind)).toEqual(["offload-run"]);
 const moved=events(root)[0]!;expect(moved.bytesShipped as number,"the input files of the action are counted").toBeGreaterThan(100);expect(moved.bytesUploaded as number).toBeLessThanOrEqual(moved.bytesShipped as number+4096);expect(moved.bytesReturned as number,"the outputs that came back are counted").toBeGreaterThan(500);
 const dep=readFileSync(join(checkout.deps,`c-${externHash("c")}.d`),"utf8");
 expect(dep).toContain("pk/c/src/lib.rs");expect(dep).not.toContain(".semio-offload");
 const entries=readdirSync(join(root,"e")).flatMap(shard=>readdirSync(join(root,"e",shard)));
 expect(entries.length,"exactly one entry was stored").toBe(1);
 const local=spawnSync("rustc",["--crate-name","d","--edition=2021","--crate-type","lib","--emit=metadata","--extern",`c=${join(checkout.deps,`libc-${externHash("c")}.rlib`)}`,"-L",`dependency=${checkout.deps}`,"-","--out-dir",join(scratch,"probe")],{input:"pub fn d()->u32{c::c()}\n",encoding:"utf8",env:testRustEnvironmentV1()});
 expect(local.status,local.stderr).toBe(0);
},180000);

test("a second checkout at another absolute path asks for the same unit and gets it without any compiler running: from the fleet unit cache or the action cache, never from a worker again",async()=>{
 const first=checkoutWith("twin-a","pub fn c()->u32{b::b()+7}\n"),second=checkoutWith("twin-b","pub fn c()->u32{b::b()+7}\n"),rootA=join(scratch,"unit-twin-a"),rootB=join(scratch,"unit-twin-b");
 expect(first.root).not.toBe(second.root);
 for(const name of readdirSync(first.deps))copyFileSync(join(first.deps,name),join(second.deps,name));
 const left=await runWrapper(first,"c",["b"],rootA,{SEMIO_OFFLOAD:"always"}),assignedBefore=cluster.events.filter(event=>event.startsWith("assigned")).length;
 const right=await runWrapper(second,"c",["b"],rootB,{SEMIO_OFFLOAD:"always"});
 expect([left.status,right.status]).toEqual([0,0]);
 expect(callsOf(join(rootA,"rustc-calls.log"))+callsOf(join(rootB,"rustc-calls.log"))).toBe(0);
 expect(cluster.events.filter(event=>event.startsWith("assigned")).length).toBe(assignedBefore);
 const served=events(rootB)[0];if(served)expect(String(served.origin)).toMatch(/\(cache\)$/u);
 expect(String(events(rootA)[0]?.origin)).not.toMatch(/\(cache\)$/u);
 expect(readFileSync(join(second.deps,`c-${externHash("c")}.d`),"utf8")).toContain(`pk/c/src/lib.rs`);
 expect(neutral(left.stderr,first)).toBe(neutral(right.stderr,second));
},180000);

test("with a calm local host nothing leaves the machine in auto mode, and never stays local whatever the fleet offers",async()=>{
 const checkout=checkoutWith("calm","pub fn c()->u32{b::b()+2}\n"),root=join(scratch,"unit-calm");
 mkdirSync(root,{recursive:true});
 const probe=testHostProbeV1("quiet-no-swap",true);
 const policy={...BUILD_HOST_CAPACITY_POLICY_V1,load:{...BUILD_HOST_CAPACITY_POLICY_V1.load,perCoreCeiling:10000}};
 const previous=process.cwd();process.chdir(checkout.root);
 try{expect(await wrapRustcV1(["rustc",...checkout.arguments("c",["b"])],environmentOf(checkout,"c",root,{SEMIO_OFFLOAD:"auto"}),checkout.root,{admission:{probe,policy}})).toBe(0);}finally{process.chdir(previous);}
 expect(callsOf(join(root,"rustc-calls.log"))).toBe(1);expect(events(root)).toEqual([]);
 const never=join(scratch,"unit-never");mkdirSync(never,{recursive:true});
 const second=checkoutWith("never","pub fn c()->u32{b::b()+3}\n");
 const run=await runWrapper(second,"c",["b"],never,{SEMIO_OFFLOAD:"never"});
 expect(run.status).toBe(0);expect(callsOf(join(never,"rustc-calls.log"))).toBe(1);expect(events(never)).toEqual([]);
},180000);

test("diagnostics of a crate with warnings and of a crate with an error are byte-identical to a local build, with the compiler's exit status",async()=>{
 for(const [label,source,expected] of [["warn","pub fn c()->u32{let unused=1;let also=2;b::b()}\n",0],["error","pub fn c()->u32{\"text\"}\n",1]] as const){
  const remote=checkoutWith("diag-r-"+label,source),local=checkoutWith("diag-l-"+label,source),remoteRoot=join(scratch,"unit-diag-r-"+label),localRoot=join(scratch,"unit-diag-l-"+label);
  const left=await runWrapper(remote,"c",["b"],remoteRoot,{SEMIO_OFFLOAD:"always"}),right=await runWrapper(local,"c",["b"],localRoot,{SEMIO_OFFLOAD:"never"});
  expect(callsOf(join(remoteRoot,"rustc-calls.log")),label).toBe(0);expect(callsOf(join(localRoot,"rustc-calls.log")),label).toBe(1);
  expect(left.status,label).toBe(expected);expect(right.status,label).toBe(expected);
  expect(neutral(left.stderr,remote),label).toBe(neutral(right.stderr,local));
  expect(neutral(left.stdout,remote),label).toBe(neutral(right.stdout,local));
  expect(left.stderr.length,label).toBeGreaterThan(0);
  if(label==="warn")expect(left.stderr).toContain("unused variable");
  if(label==="error")expect(left.stderr).toContain("mismatched types");
  if(expected===1)expect(readdirSync(remote.deps).some(name=>name.includes(`-${externHash("c")}`)),"a failed unit leaves no artifacts").toBe(false);
 }
},240000);

test("a proc-macro crate links on another machine and the dylib it returns loads into a local compiler",async()=>{
 const checkout=createTestCheckoutV1("procmacro",scratch),root=join(scratch,"unit-pm");
 checkout.crate("pm","extern crate proc_macro;\nuse proc_macro::TokenStream;\n#[proc_macro]\npub fn answer(_input:TokenStream)->TokenStream{\"42u32\".parse().unwrap()}\n");
 const asMacro=(args:string[])=>[...args.map((value,index)=>args[index-1]==="--crate-type"?"proc-macro":value),"--extern","proc_macro"];
 mkdirSync(root,{recursive:true});
 const child=spawn(process.execPath,[wrapper,"rustc",...asMacro(checkout.arguments("pm"))],{cwd:checkout.root,env:environmentOf(checkout,"pm",root,{SEMIO_OFFLOAD:"always"}),stdio:["ignore","pipe","pipe"]});
 children.add(child);const err:Buffer[]=[];child.stderr!.on("data",chunk=>err.push(chunk));
 const status=await new Promise<number|null>(done=>child.on("close",done));
 expect(status,Buffer.concat(err).toString()+progress(root)+JSON.stringify(events(root))).toBe(0);
 expect(callsOf(join(root,"rustc-calls.log"))).toBe(0);
 const library=readdirSync(checkout.deps).find(name=>name.startsWith(`libpm-${externHash("pm")}`)&&/\.(dylib|so)$/u.test(name));
 expect(library,readdirSync(checkout.deps).join(",")).toBeDefined();
 mkdirSync(join(scratch,"pm-user"),{recursive:true});
 const again=spawnSync("rustc",["--edition=2021","--crate-type","lib","--emit=metadata","--extern",`pm=${join(checkout.deps,library!)}`,"-","--out-dir",join(scratch,"pm-user")],{input:"pub fn f()->u32{pm::answer!()}\n",encoding:"utf8",env:testRustEnvironmentV1(),cwd:scratch});
 expect(again.status,again.stderr).toBe(0);
},180000);

test("a result that embeds the other machine's path is never stored: the unit falls back to a local compile and the entry holds the requester's own path",async()=>{
 const checkout=checkoutWith("pathdep","pub const D:&str=env!(\"CARGO_MANIFEST_DIR\");\n"),root=join(scratch,"unit-pathdep");
 const run=await runWrapper(checkout,"c",["b"],root,{SEMIO_OFFLOAD:"always"});
 expect(run.status,run.stderr).toBe(0);
 expect(events(root).map(row=>row.reason)).toEqual(["path-dependent-output"]);
 expect(callsOf(join(root,"rustc-calls.log"))).toBe(1);
 const rlib=readFileSync(join(checkout.deps,`libc-${externHash("c")}.rlib`));
 expect(rlib.includes(Buffer.from(join(checkout.root,"pk","c")))).toBe(true);
 expect(rlib.includes(Buffer.from("/work/"))).toBe(false);
},180000);

const SLOW="#![allow(long_running_const_eval)]\nconst X:u64={let mut i=0u64;let mut s=0u64;while i<40_000_000{s=s.wrapping_add(i);i+=1;}s};\npub fn c()->u64{X}\n";

test("killing the worker that runs the unit mid-compilation still completes the unit, elsewhere or locally",async()=>{
 const checkout=checkoutWith("kill",SLOW),root=join(scratch,"unit-kill");
 const started=cluster.events.length,running=runWrapper(checkout,"c",["b"],root,{SEMIO_OFFLOAD:"always"});
 const assigned=()=>cluster.events.slice(started).map(event=>/-> (\S+)$/u.exec(event)?.[1]).find(Boolean);
 await eventuallyV1(()=>assigned()!==undefined,60000,"a worker is assigned the unit");
 const victim=assigned()!;
 workers.get(victim)!.child.kill("SIGKILL");
 const result=await running;
 expect(result.status,result.stderr).toBe(0);
 expect(readdirSync(checkout.deps).some(name=>name.startsWith(`libc-${externHash("c")}`)&&name.endsWith(".rlib"))).toBe(true);
 const text=progress(root),locally=callsOf(join(root,"rustc-calls.log"));
 expect(text.includes("offloaded to")||locally>0,text).toBe(true);
 if(text.includes("offloaded to"))expect(text).not.toContain(`offloaded to ${victim}`);
 expect(cluster.events.length).toBeGreaterThan(started);
 reap();
 startWorker(victim,join(scratch,"worker-"+victim+"-again"));
},240000);

test("cancelling the build (SIGTERM to the wrapper) cancels the remote action: the compiler on the other machine is killed",async()=>{
 await eventuallyV1(async()=>{const status=await (await fetch(cluster.server.workerUrl+"/status")).json() as {workers?:unknown[]};return(status.workers?.length??0)>=2;},60000,"both workers present");
 const checkout=checkoutWith("cancel",SLOW.replace("40_000_000","4_000_000_000"),"cx"),root=join(scratch,"unit-cancel");
 const running=runWrapper(checkout,"cx",["b"],root,{SEMIO_OFFLOAD:"always"}),child=lastWrapper!;
 const marker=`pk/cx/src/lib.rs`;
 const compilerRunning=():boolean=>spawnSync("ps",["-ax","-o","command="],{encoding:"utf8"}).stdout.split("\n").some(line=>line.includes("--crate-name cx")&&line.includes(marker)&&line.includes(".semio-offload/o"));
 await eventuallyV1(compilerRunning,90000,"the remote compiler runs");
  child.kill("SIGTERM");
 const result=await running;
 expect(result.status).toBe(143);
 await eventuallyV1(()=>!compilerRunning(),30000,"the remote compiler is killed");
},240000);
