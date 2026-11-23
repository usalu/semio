/** ☁️ A build script run on another machine of the fleet leaves the bytes of a local run (its `OUT_DIR` tree, its `cargo:` lines translated into the requester's paths); a script that is not hermetic or depends on the worker's directories stays here with its reason. The oracle is the same compiled script run locally; the rules are language-agnostic fixtures. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {symlinkSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,relative} from "node:path";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixtures from "./🔣️.json" with {type:"json"};
import {BUILD_OFFLOAD_POLICY_V1,buildScriptBinaryRefusalV1,buildScriptEnvironmentV1,buildScriptSourcePathV1,buildScriptSourceRefusalV1,buildScriptToolTextV1,manifestInputsV1,offloadBuildScriptRunV1,translateBuildStreamV1} from "../🟦️.ts";
import {runBuildScriptV1} from "../../🟦️.ts";
import {lightSlotsHeldV1} from "../../../🔒️lock/🟦️.ts";
import {eventuallyV1,startClusterV1,type ClusterV1} from "../../../../../../🏗️build/🌐️remote-execution/🧪️tests/🧰️cluster/🟦️.ts";
import {detectWorkerCommandsV1} from "../../../../../../🏗️build/🌐️remote-execution/🚀️offload/🧰️command/🛠️worker-tools/🟦️.ts";
import {remoteExecutionHostPropertiesV1} from "../../../../../../🏗️build/🌐️remote-execution/🟦️.ts";

const scratch=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"build-script-offload-"))),previous=process.env.SEMIO_CAS_ROOT;
const agentStop=new AbortController();let reporter:Promise<void>|undefined,cluster:ClusterV1,environment:Record<string,string>;

const SCRIPT=`use std::{env,fs,path::PathBuf};
fn main(){
 let out=PathBuf::from(env::var("OUT_DIR").unwrap());
 let target=env::var("TARGET").unwrap();
 let manifest=env::var("CARGO_MANIFEST_DIR").unwrap();
 let data=fs::read_to_string(PathBuf::from(&manifest).join("data/table.txt")).unwrap();
 fs::write(out.join("gen.rs"),format!("pub const T:&str={:?};pub const TARGET:&str={:?};\\n",data.trim(),target)).unwrap();
 fs::create_dir_all(out.join("lib")).unwrap();
 fs::write(out.join("lib/blob.bin"),[0u8,255,1,254]).unwrap();
 println!("cargo:rerun-if-changed=build.rs");
 println!("cargo:rerun-if-changed=data/table.txt");
 println!("cargo:rustc-link-search=native={}/lib",out.display());
 println!("cargo:rustc-env=GEN_DIR={}",out.display());
 println!("cargo:rustc-env=MANIFEST_FILE={}/data/table.txt",manifest);
 eprintln!("warning: generated in {}",out.display());
}
`;
const READS_OUTSIDE=`use std::{env,fs};
fn main(){
 let out=env::var("OUT_DIR").unwrap();
 let hosts=fs::read_to_string("/etc/hosts").unwrap_or_default();
 fs::write(format!("{}/hosts.txt",out),hosts.len().to_string()).unwrap();
}
`;
const LEAKS_ROOT=`use std::{env,fs};
fn main(){
 let out=env::var("OUT_DIR").unwrap();
 fs::write(format!("{}/where.txt",out),env::current_dir().unwrap().display().to_string()).unwrap();
}
`;

const SLEEPER=`use std::{env,fs,io::Write,time::{SystemTime,UNIX_EPOCH}};
const ID:&str="__ID__";
fn now()->u128{SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()}
fn main(){
 let out=env::var("OUT_DIR").unwrap();
 let log=env::var("SEMIO_TEST_LOG").unwrap();
 let mut file=fs::OpenOptions::new().create(true).append(true).open(&log).unwrap();
 writeln!(file,"{} start {}",ID,now()).unwrap();
 std::thread::sleep(std::time::Duration::from_millis(1500));
 writeln!(file,"{} end {}",ID,now()).unwrap();
 fs::write(format!("{}/{}.txt",out,ID),ID).unwrap();
}
`;

type Package=Readonly<{manifest:string;binary:string}>;
function packageOf(name:string,source:string):Package{
 const manifest=join(scratch,name,"pkg"),binary=join(scratch,name,"build-script");
 mkdirSync(join(manifest,"data"),{recursive:true});mkdirSync(join(manifest,"src"),{recursive:true});mkdirSync(join(manifest,"target"),{recursive:true});
 writeFileSync(join(manifest,"Cargo.toml"),`[package]\nname="${name}"\nversion="0.1.0"\nedition="2021"\n`);
 writeFileSync(join(manifest,"build.rs"),source);writeFileSync(join(manifest,"data/table.txt"),"one two three\n");writeFileSync(join(manifest,"src/lib.rs"),"//\n");writeFileSync(join(manifest,"target/ignored.bin"),"not shipped");
 const compiled=spawnSync("rustc",["--edition","2021","-C","opt-level=1",join(manifest,"build.rs"),"-o",binary],{encoding:"utf8"});
 if(compiled.status!==0)throw Error("rustc: "+compiled.stderr);
 return{manifest,binary};
}
const cargoEnvironment=(manifest:string,outDir:string,extra:Record<string,string>={})=>({PATH:process.env.PATH??"",HOME:scratch,OUT_DIR:outDir,CARGO_MANIFEST_DIR:manifest,CARGO_PKG_NAME:"tiny",CARGO_PKG_VERSION:"0.1.0",CARGO_PKG_DESCRIPTION:"a tiny\nscript",TARGET:"aarch64-apple-darwin",HOST:"aarch64-apple-darwin",PROFILE:"release",OPT_LEVEL:"3",DEBUG:"false",NUM_JOBS:"4",CARGO_CFG_TARGET_OS:"macos",CARGO_FEATURE_STD:"1",CARGO_ENCODED_RUSTFLAGS:"",RUSTC:"/x/rustc",...environment,...extra});
function tree(root:string,directory=root):Map<string,string>{
 const files=new Map<string,string>();
 for(const entry of readdirSync(directory).sort()){
  const path=join(directory,entry);
  if(statSync(path).isDirectory())for(const [name,hash] of tree(root,path))files.set(name,hash);
  else files.set(relative(root,path),new Bun.CryptoHasher("sha256").update(readFileSync(path)).digest("hex"));
 }
 return files;
}
function local(pack:Package,outDir:string,extra:Record<string,string>={}){
 mkdirSync(outDir,{recursive:true});
 const ran=spawnSync(pack.binary,[],{cwd:pack.manifest,env:cargoEnvironment(pack.manifest,outDir,extra) as NodeJS.ProcessEnv,encoding:"utf8"});
 return{status:ran.status,stdout:ran.stdout,stderr:ran.stderr};
}
const ready=()=>eventuallyV1(async()=>((await (await fetch(cluster.server.workerUrl+"/v1/fleet/load")).json()) as {machines?:{origin:string;headroom?:boolean}[]}).machines?.some(row=>row.origin==="wA"&&row.headroom===true)===true,120000,"the worker's machine reports headroom");
const events=(kind:string)=>{
 const file=join(scratch,"offload","x","offload.jsonl");
 return existsSync(file)?readFileSync(file,"utf8").split("\n").filter(Boolean).map(row=>JSON.parse(row) as {kind:string;crate:string;reason?:string;detail?:string}).filter(row=>row.kind===kind):[];
};

afterAll(async()=>{agentStop.abort();await reporter?.catch(()=>{});await cluster?.stop().catch(()=>{});if(previous===undefined)delete process.env.SEMIO_CAS_ROOT;else process.env.SEMIO_CAS_ROOT=previous;rmSync(scratch,{recursive:true,force:true});});
beforeAll(async()=>{
 process.env.SEMIO_CAS_ROOT=join(scratch,"cas");
 cluster=await startClusterV1({workers:0,root:scratch,scheduler:{leaseSeconds:5,lingerSeconds:1,unmatchedSeconds:6}});
  const detected=await detectWorkerCommandsV1(join(import.meta.dir,"../../../../../../../../../../.."),process.env);
 cluster.startWorker({name:"wA",actions:"rustc",unconfinedForTests:false,properties:[...remoteExecutionHostPropertiesV1(),...detected.properties],commands:detected.tools});
 const host={version:1,atMs:Date.now()+86_400_000,platform:"darwin",cores:8,totalMegabytes:65536,availableMegabytes:60000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0.5,openFiles:1000,openFilesMaximum:200000,attachedImages:0};
 const {buildRemoteExecutionLoadReportV1,readRemoteExecutionCountersV1,remoteExecutionPlatformIdentityV1,runRemoteExecutionReporterV1}=await import("../../../../../../🏗️build/🌐️remote-execution/🌡️load/🟦️.ts");
 const report=async()=>buildRemoteExecutionLoadReportV1({origin:"wA",identity:remoteExecutionPlatformIdentityV1("darwin","arm64","29a34002290ca2fc","aarch64-apple-darwin"),view:{host,grants:[],waiters:[]} as never,worker:{state:"idle",concurrency:2,running:0,sandbox:"darwin-sandbox",confinement:"full",network:"denied"},counters:readRemoteExecutionCountersV1(scratch),nowMs:Date.now()});
 reporter=runRemoteExecutionReporterV1({scheduler:cluster.server.workerUrl,token:undefined,fetch,intervalSeconds:1,signal:agentStop.signal,report});
 await eventuallyV1(async()=>((await (await fetch(cluster.server.workerUrl+"/status")).json()) as {workers?:unknown[]}).workers?.length!>=1,60000,"the worker joined");
 await eventuallyV1(async()=>((await (await fetch(cluster.server.workerUrl+"/v1/fleet/load")).json()) as {machines?:{origin:string;headroom?:boolean}[]}).machines?.some(row=>row.origin==="wA"&&row.headroom===true)===true,60000,"the worker's machine reports its load");
 environment={SEMIO_UNIT_CACHE_ROOT:join(scratch,"offload"),SEMIO_REMOTE_EXECUTION_ENDPOINT:cluster.server.grpcUrl,SEMIO_REMOTE_EXECUTION_SCHEDULER:cluster.server.workerUrl,SEMIO_REMOTE_CACHE_ENDPOINT:cluster.cache.url.replace("//0.0.0.0","//127.0.0.1"),SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"test-token-not-a-secret"};
},120000);

test("the policy validates against its closed schema and the stand-in tool is the one the digest names",()=>{
 const check=new Ajv({strict:false}).compile({$ref:"#/definitions/Policy",definitions:schema.definitions});
 expect(check(BUILD_OFFLOAD_POLICY_V1),JSON.stringify(check.errors)).toBe(true);
 expect(buildScriptToolTextV1().startsWith("#!/bin/sh\n")).toBe(true);
});

test("the language-agnostic fixture gives the verdicts on sources, binaries, environments, translations and manifests",()=>{
 for(const row of fixtures.sources)expect(buildScriptSourceRefusalV1(row.source),row.name).toBe(row.reason??undefined);
 for(const row of fixtures.binaries)expect(buildScriptBinaryRefusalV1(Buffer.from(row.text)),row.name).toBe(row.reason??undefined);
 for(const row of fixtures.environments){
  const answer=buildScriptEnvironmentV1(row.environment,row.source);
  if(row.refusal)expect(answer,row.name).toEqual({refusal:row.refusal});
  else expect([...(answer as {variables:[string,string][]}).variables].sort(),row.name).toEqual((row.variables as [string,string][]).sort());
 }
 for(const row of fixtures.translations)expect(translateBuildStreamV1(row.text,row.roots,{outDir:row.outDir,manifest:row.manifest}),row.name).toBe(row.answer??undefined);
 for(const row of fixtures.manifests)expect(buildScriptSourcePathV1(row.manifest)).toBe(row.path??undefined);
});

test("the files of a package are shipped by a bounded walk: ignored directories are skipped, a link or too many files refuse",()=>{
 const pack=packageOf("walk",SCRIPT),files=manifestInputsV1(pack.manifest,{maximumFiles:100,maximumBytes:1_000_000}) as Map<string,string>;
 expect([...files.keys()].sort()).toEqual(["in/manifest/Cargo.toml","in/manifest/build.rs","in/manifest/data/table.txt","in/manifest/src/lib.rs"]);
 expect(manifestInputsV1(pack.manifest,{maximumFiles:2,maximumBytes:1_000_000})).toEqual({refusal:"inputs-too-large"});
symlinkSync("/etc/hosts",join(pack.manifest,"src","link"));
 expect((manifestInputsV1(pack.manifest,{maximumFiles:100,maximumBytes:1_000_000}) as {refusal:string}).refusal.startsWith("symbolic-link:")).toBe(true);
});

test("a tiny build script run on another machine leaves the OUT_DIR tree and the translated cargo lines of a local run, byte for byte",async()=>{
 const pack=packageOf("tiny",SCRIPT),outDir=join(scratch,"tiny","out"),expected=local(pack,outDir);
 expect(expected.status).toBe(0);
 const wanted=tree(outDir);rmSync(outDir,{recursive:true,force:true});mkdirSync(outDir,{recursive:true});
 await ready();
 const answer=await offloadBuildScriptRunV1({real:pack.binary,args:[],environment:cargoEnvironment(pack.manifest,outDir),cwd:pack.manifest,root:join(scratch,"offload"),lane:"d",crate:"tiny",key:"k".repeat(64),outDir});
 expect(answer,JSON.stringify(answer)).toMatchObject({kind:"done"});
 const done=answer as {stdout:string;stderr:string};
 expect(done.stdout).toBe(expected.stdout);expect(done.stderr).toBe(expected.stderr);
 expect([...tree(outDir)]).toEqual([...wanted]);
 expect(existsSync(join(outDir,".semio-offload"))).toBe(false);
 expect(done.stdout).toContain(`cargo:rustc-link-search=native=${outDir}/lib`);
 expect(done.stdout).toContain(`cargo:rustc-env=MANIFEST_FILE=${pack.manifest}/data/table.txt`);
 expect(events("offload-run").some(row=>row.crate.startsWith("tiny"))).toBe(true);
},120000);

test("a script that reads a path outside its package stays here with its reason recorded, and nothing is written",async()=>{
 const pack=packageOf("outside",READS_OUTSIDE),outDir=join(scratch,"outside","out");mkdirSync(outDir,{recursive:true});
 const answer=await offloadBuildScriptRunV1({real:pack.binary,args:[],environment:cargoEnvironment(pack.manifest,outDir),cwd:pack.manifest,root:join(scratch,"offload"),lane:"d",crate:"outside",key:"o".repeat(64),outDir});
 expect(answer).toEqual({kind:"local",reason:"not-portable",detail:"reads-outside-manifest"});
 expect(readdirSync(outDir)).toEqual([]);
 expect(events("offload-declined").some(row=>row.crate==="outside"&&row.reason==="not-portable"&&row.detail==="build-script: reads-outside-manifest")).toBe(true);
},60000);

test("a script whose output names the worker's directory is withdrawn: the landed files are removed and it runs here",async()=>{
 const pack=packageOf("leaks",LEAKS_ROOT),outDir=join(scratch,"leaks","out");mkdirSync(outDir,{recursive:true});writeFileSync(join(outDir,"kept.txt"),"before");
 await ready();
 const answer=await offloadBuildScriptRunV1({real:pack.binary,args:[],environment:cargoEnvironment(pack.manifest,outDir),cwd:pack.manifest,root:join(scratch,"offload"),lane:"d",crate:"leaks",key:"l".repeat(64),outDir});
 expect(answer).toMatchObject({kind:"local",reason:"path-dependent-output"});
 expect(readdirSync(outDir)).toEqual(["kept.txt"]);
},120000);

test("offloading switched off and an environment the command cannot carry leave the run here untouched",async()=>{
 const pack=packageOf("never",SCRIPT),outDir=join(scratch,"never","out");mkdirSync(outDir,{recursive:true});
 const never=await offloadBuildScriptRunV1({real:pack.binary,args:[],environment:cargoEnvironment(pack.manifest,outDir,{SEMIO_OFFLOAD:"never"}),cwd:pack.manifest,root:join(scratch,"offload"),lane:"d",crate:"never",key:"n".repeat(64),outDir});
 expect(never).toMatchObject({kind:"local",reason:"never"});
 const links=await offloadBuildScriptRunV1({real:pack.binary,args:[],environment:cargoEnvironment(pack.manifest,outDir,{DEP_Z_ROOT:"/z"}),cwd:pack.manifest,root:join(scratch,"offload"),lane:"d",crate:"never",key:"n".repeat(64),outDir});
 expect(links).toEqual({kind:"local",reason:"not-portable",detail:"environment:DEP_Z_ROOT"});
 expect(readdirSync(outDir)).toEqual([]);
},60000);

test("the cache wrapper runs a missed build script on the fleet, records and replays it, and runs the next one here when offloading is off",async()=>{
 const pack=packageOf("wrapped",SCRIPT),outDir=join(scratch,"wrapped","out"),expected=local(pack,join(scratch,"wrapped","oracle"));
 const wanted=tree(join(scratch,"wrapped","oracle")),base={...cargoEnvironment(pack.manifest,outDir,{CARGO_PKG_NAME:"wrapped"}),SEMIO_UNIT_CACHE_BUILD_SCRIPTS:"on",SEMIO_UNIT_CACHE_INNER:JSON.stringify(["rustc"]),SEMIO_UNIT_CACHE_LANE:"w"};
 mkdirSync(outDir,{recursive:true});
 await ready();
 const first=await runBuildScriptV1(pack.binary,[],base,pack.manifest);
 expect(first).toBe(0);
 const kinds=readFileSync(join(scratch,"offload","x","offload.jsonl"),"utf8");expect(kinds).toContain("offload-run");
 expect(events("offload-run").some(row=>row.crate==="wrapped build script")).toBe(true);
 expect(tree(outDir)).toEqual(wanted);
 expect(readFileSync(join(outDir,"lib","blob.bin"))).toEqual(Buffer.from([0,255,1,254]));
 expect(readFileSync(join(outDir,"gen.rs"),"utf8")).toBe(readFileSync(join(scratch,"wrapped","oracle","gen.rs"),"utf8"));
 expect(expected.status).toBe(0);
},180000);

test("with more cargo jobs than the local grant, local build script runs queue for the real slots and run one at a time",async()=>{
 const log=join(scratch,"slots.log"),names=["s1","s2","s3"],packs=names.map(name=>packageOf(name,SLEEPER.replace("__ID__",name)));
 const runs=packs.map((pack,index)=>{
  const outDir=join(scratch,names[index]!,"out");mkdirSync(outDir,{recursive:true});
  const {SEMIO_REMOTE_EXECUTION_ENDPOINT:_a,SEMIO_REMOTE_EXECUTION_SCHEDULER:_b,SEMIO_REMOTE_CACHE_ENDPOINT:_c,SEMIO_REMOTE_CACHE_INSTANCE:_d,SEMIO_REMOTE_CACHE_ACCESS:_e,SEMIO_REMOTE_CACHE_TOKEN:_f,...rest}=cargoEnvironment(pack.manifest,outDir,{CARGO_PKG_NAME:names[index]!});
  return runBuildScriptV1(pack.binary,[],{...rest,SEMIO_OFFLOAD:"never",SEMIO_UNIT_CACHE_BUILD_SCRIPTS:"on",SEMIO_UNIT_CACHE_INNER:JSON.stringify(["rustc"]),SEMIO_UNIT_CACHE_LANE:"j",SEMIO_UNIT_CACHE_ROOT:join(scratch,"slots-root"),SEMIO_UNIT_CACHE_SLOTS:"1",SEMIO_TEST_LOG:log},pack.manifest);
 });
 expect(await Promise.all(runs)).toEqual([0,0,0]);
 const rows=readFileSync(log,"utf8").trim().split("\n").map(row=>row.split(" ")).map(([id,kind,at])=>({id:id!,kind:kind!,at:Number(at)})).sort((left,right)=>left.at-right.at);
 expect(rows.map(row=>row.kind)).toEqual(["start","end","start","end","start","end"]);
 expect(rows.every((row,index)=>index%2===0||row.id===rows[index-1]!.id)).toBe(true);
},300000);

test("a build script run offloaded to a worker holds only a light slot while it runs, never a local compile slot",async()=>{
 const pack=packageOf("light",SCRIPT),outDir=join(scratch,"light","out"),root=join(scratch,"offload");mkdirSync(outDir,{recursive:true});
 const base={...cargoEnvironment(pack.manifest,outDir,{CARGO_PKG_NAME:"light"}),SEMIO_UNIT_CACHE_BUILD_SCRIPTS:"on",SEMIO_UNIT_CACHE_INNER:JSON.stringify(["rustc"]),SEMIO_UNIT_CACHE_LANE:"l",SEMIO_UNIT_CACHE_SLOTS:"1",SEMIO_UNIT_CACHE_LIGHT_SLOTS:"3"};
 let peak=0,running=true;
 const watch=(async()=>{while(running){peak=Math.max(peak,lightSlotsHeldV1(root,3));await new Promise(done=>setTimeout(done,20));}})();
 await ready();
 const status=await runBuildScriptV1(pack.binary,[],base,pack.manifest);
 running=false;await watch;
 expect(status).toBe(0);expect(peak).toBeGreaterThanOrEqual(1);expect(lightSlotsHeldV1(root,3)).toBe(0);
 expect(events("offload-run").some(row=>row.crate==="light build script")).toBe(true);
},180000);
