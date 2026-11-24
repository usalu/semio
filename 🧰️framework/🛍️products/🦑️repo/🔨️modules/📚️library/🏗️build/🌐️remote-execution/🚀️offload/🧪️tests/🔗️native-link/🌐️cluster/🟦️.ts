import{stretchedBudgetMillisecondsV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import {afterAll,beforeAll,expect,test} from "bun:test";
import {spawn,spawnSync,type ChildProcess} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {wrapRustcV1,PATH_NEUTRAL_FLAGS_V1} from "../../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";
import {linkRootArgumentsV1} from "../../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧭️remap/🟦️.ts";
import {readRemoteExecutionPolicyV1} from "../../../../🟦️.ts";
import {startClusterV1,eventuallyV1,type ClusterV1} from "../../../../🧪️tests/🧰️cluster/🟦️.ts";
import {testHostProbeV1,testRustEnvironmentV1} from "../../🧰️workspace/🟦️.ts";

const entry=fileURLToPath(new URL("../../../../🏃️entry/🟦️.ts",import.meta.url));
const scratch=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"native-e2e-")));
process.env.SEMIO_CAS_ROOT=join(scratch,"cas");
const children=new Set<ChildProcess>();
let cluster:ClusterV1;
const idleHost=join(scratch,"idle-host.json");
writeFileSync(idleHost,JSON.stringify({host:{version:1,atMs:Date.now()+86400000,platform:process.platform==="darwin"?"darwin":"linux",cores:16,totalMegabytes:65536,availableMegabytes:48000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0.5,openFiles:null,openFilesMaximum:null,attachedImages:null},grants:[],waiters:[]}));
const toolsPresent=spawnSync("cc",["--version"]).status===0&&spawnSync("rustc",["--version"],{env:testRustEnvironmentV1()}).status===0;

function startWorker(name:string):void{
 const directory=join(scratch,"worker-"+name);mkdirSync(directory,{recursive:true});
 const child=spawn(process.execPath,[entry,"--join",cluster.server.workerUrl,"--name",name,"--origin",name,"--work-directory",join(directory,"work"),"--concurrency","2"],{cwd:directory,env:testRustEnvironmentV1({SEMIO_CAS_ROOT:undefined,SEMIO_REMOTE_EXECUTION_HOST_VIEW:idleHost}),stdio:["ignore","pipe","pipe"]});
 child.stdout!.on("data",()=>{});child.stderr!.on("data",()=>{});children.add(child);child.on("close",()=>children.delete(child));
}
beforeAll(async()=>{
 if(!toolsPresent)return;
 cluster=await startClusterV1({workers:0,root:scratch,scheduler:{leaseSeconds:5,lingerSeconds:1,unmatchedSeconds:30}});
 for(const name of ["wA","wB"])startWorker(name);
 await eventuallyV1(async()=>{const status=await (await fetch(cluster.server.workerUrl+"/status")).json() as {workers?:unknown[]};return(status.workers?.length??0)>=2;},60000,"both worker processes join");
 await eventuallyV1(async()=>{const response=await fetch(cluster.server.workerUrl+"/v1/fleet/load");if(!response.ok)return false;const table=await response.json() as {machines?:{origin:string;headroom?:boolean}[]};return["wA","wB"].every(name=>table.machines?.some(row=>row.origin===name&&row.headroom===true)===true);},60000,"both machines report their load");
},stretchedBudgetMillisecondsV1(120000));
afterAll(async()=>{
 for(const child of children)child.kill("SIGKILL");
 await cluster?.stop().catch(()=>{});
 rmSync(scratch,{recursive:true,force:true});
});

/** 📁️ A requester checkout: a cdylib `t` that calls the C function of a static library `libnat.a` built beside it (what a build script's OUT_DIR holds), linked with `-l static=nat -L native=<dir>`. */
function checkout(label:string){
 const base=realpathSync(mkdtempSync(join(scratch,`${label}-`))),root=join(base,"ws✏️"),nat=join(root,"nat"),out=join(root,"out");
 for(const directory of [join(root,"t","src"),nat,out])mkdirSync(directory,{recursive:true});
 writeFileSync(join(root,"t","Cargo.toml"),'[package]\nname = "t"\n');
 writeFileSync(join(root,"t","src","lib.rs"),'#[link(name = "nat", kind = "static")]\nextern "C" { fn nat_add(a: u32, b: u32) -> u32; }\n#[no_mangle]\npub extern "C" fn answer(n: u32) -> u32 { unsafe { nat_add(n, 41) } }\n');
 writeFileSync(join(nat,"nat.c"),"unsigned nat_add(unsigned a, unsigned b) { return a + b; }\n");
 if(spawnSync("cc",["-c","-O2","-o",join(nat,"nat.o"),join(nat,"nat.c")]).status!==0||spawnSync("ar",["rcs",join(nat,"libnat.a"),join(nat,"nat.o")]).status!==0)throw Error("cannot build the static library");
 const args=["--crate-name","t","--edition=2021","t/src/lib.rs","--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type","cdylib","--emit=dep-info,metadata,link","-C","debuginfo=0","-C","opt-level=s","-C","strip=symbols",`--remap-path-prefix=${root}=.`,"-C","metadata=def456","-C","extra-filename=-def456","--out-dir",out,"-L",`dependency=${out}`,"-L",`native=${nat}`,"-L","native=/usr/lib","-l","static=nat","-Z","threads=0",...PATH_NEUTRAL_FLAGS_V1];
 const environment={CARGO_MANIFEST_DIR:join(root,"t"),CARGO_PKG_NAME:"t",CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:"t",CARGO_PRIMARY_PACKAGE:"1"};
 return{base,root,out,args,environment};
}
const events=(cacheRoot:string):Record<string,unknown>[]=>existsSync(join(cacheRoot,"x","offload.jsonl"))?readFileSync(join(cacheRoot,"x","offload.jsonl"),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line)):[];

test.skipIf(!toolsPresent)("a native cdylib with a static library of the checkout links on another machine: the worker accepts the action, answers its output, and the bytes equal the library that rustc links here at another path",async()=>{
 const remote=checkout("remote"),plain=checkout("plain"),cacheRoot=join(scratch,"unit-native");mkdirSync(cacheRoot,{recursive:true});
 const environment={...testRustEnvironmentV1(remote.environment),SEMIO_UNIT_CACHE_ROOT:cacheRoot,SEMIO_UNIT_CACHE_LANE:"t",SEMIO_UNIT_CACHE_SLOTS:"0",SEMIO_OFFLOAD:"distribute",SEMIO_OFFLOAD_PRESSURE:"door",SEMIO_REMOTE_EXECUTION_ENDPOINT:cluster.server.grpcUrl,SEMIO_REMOTE_CACHE_ENDPOINT:cluster.cache.url.replace("//0.0.0.0","//127.0.0.1"),SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_CAS_ROOT:join(scratch,"cas-native")};
 const previous=process.cwd();process.chdir(remote.root);
 let status:number;try{status=await wrapRustcV1(["rustc",...remote.args],environment,remote.root,{admission:{probe:testHostProbeV1("quiet-no-swap",true)}});}finally{process.chdir(previous);}
 expect(status,JSON.stringify(events(cacheRoot))).toBe(0);
 expect(events(cacheRoot).map(row=>row.kind),JSON.stringify(events(cacheRoot))).toEqual(["offload-run"]);
 const run=spawnSync("rustc",[...plain.args,...linkRootArgumentsV1(plain.args,plain.out)],{cwd:plain.root,encoding:"utf8",env:testRustEnvironmentV1(plain.environment)});
 expect(run.status,run.stderr).toBe(0);
 const name=readdirSync(remote.out).find(file=>/^libt-def456\.dylib$|^libt-def456\.so$/u.test(file))!;
 expect(name,"the linked library came back").toBeString();
 const offloaded=readFileSync(join(remote.out,name)),local=readFileSync(join(plain.out,name));
 expect(Buffer.compare(offloaded,local),"offloaded and local library have the same bytes").toBe(0);
 expect(offloaded.includes(Buffer.from(remote.base))).toBe(false);
},stretchedBudgetMillisecondsV1(240000));

/** 🧵️ Offloads one cdylib unit to the cluster and answers what the worker reported of its code generation threads (the `offload-run` event's detail), after the host view of the workers was set to `load`. */
async function offloadedThreads(label:string,load:number):Promise<number|undefined>{
 const view=JSON.parse(readFileSync(idleHost,"utf8")) as {host:{load:number}};view.host.load=load;writeFileSync(idleHost,JSON.stringify(view));
 await new Promise<void>(resolve=>{setTimeout(resolve,1500);});
 const remote=checkout(label),cacheRoot=join(scratch,"unit-"+label);mkdirSync(cacheRoot,{recursive:true});
 const environment={...testRustEnvironmentV1(remote.environment),SEMIO_UNIT_CACHE_ROOT:cacheRoot,SEMIO_UNIT_CACHE_LANE:"t",SEMIO_UNIT_CACHE_SLOTS:"0",SEMIO_OFFLOAD:"distribute",SEMIO_OFFLOAD_PRESSURE:"door",SEMIO_REMOTE_EXECUTION_ENDPOINT:cluster.server.grpcUrl,SEMIO_REMOTE_CACHE_ENDPOINT:cluster.cache.url.replace("//0.0.0.0","//127.0.0.1"),SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_CAS_ROOT:join(scratch,"cas-"+label)};
 const previous=process.cwd();process.chdir(remote.root);
 let status:number;try{status=await wrapRustcV1(["rustc",...remote.args],environment,remote.root,{admission:{probe:testHostProbeV1("quiet-no-swap",true)}});}finally{process.chdir(previous);}
 expect(status,JSON.stringify(events(cacheRoot))).toBe(0);
 const run=events(cacheRoot).find(row=>row.kind==="offload-run");expect(run,JSON.stringify(events(cacheRoot))).toBeDefined();
 const found=/(\d+) codegen threads/u.exec(String(run!.detail??""));
 return found===null?undefined:Number(found[1]);
}

test.skipIf(!toolsPresent)("a cdylib final that runs on a worker uses the cores of that worker: an idle 16-core machine grants it several code generation threads (the policy's cap), a saturated one grants the fair share (a busy machine's operating system divides its cores by threads), and the grant comes back in the result",async()=>{
 const idle=await offloadedThreads("idle",0.5);
 expect(idle,"threads granted by an idle worker").toBeGreaterThan(1);
 const saturated=await offloadedThreads("busy",30);
 expect(saturated,"threads granted by a saturated worker").toBe(readRemoteExecutionPolicyV1().worker.codegen.fairShareThreads);
 expect(saturated!,"fewer than an idle one").toBeLessThan(idle!);
},stretchedBudgetMillisecondsV1(240000));
