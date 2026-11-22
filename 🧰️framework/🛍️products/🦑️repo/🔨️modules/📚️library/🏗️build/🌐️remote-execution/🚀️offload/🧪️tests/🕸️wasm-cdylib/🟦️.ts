import {afterEach,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {copyFileSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,sep} from "node:path";
import type {RemoteExecutionContentV1} from "../../../🗄️content/🟦️.ts";
import type {RemoteExecutionDigestV1} from "../../../🟦️.ts";
import {expandRemoteExecutionRootV1,validateRemoteExecutionRustcActionV1} from "../../../🧱️actions/🟦️.ts";
import {linkRootArgumentsV1} from "../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧭️remap/🟦️.ts";
import {PATH_NEUTRAL_FLAGS_V1} from "../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";
import {buildOffloadActionV1,offloadRustcCommandV1,offloadPlatformV1,offloadSerialArgumentsV1} from "../../📜️action/🟦️.ts";
import {planOffloadV1,type OffloadPlanV1} from "../../🗺️paths/🟦️.ts";
import {OFFLOAD_POLICY_V1,offloadActionSecondsV1,offloadCrateTypeRefusalV1,offloadPeakMegabytesV1} from "../../📐️policy/🟦️.ts";
import {testRustEnvironmentV1} from "../🧰️workspace/🟦️.ts";
import vectors from "./🔣️.json" with {type:"json"};

const policy=OFFLOAD_POLICY_V1,mark=policy.root.placeholder,scratch:string[]=[];
afterEach(()=>{for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});

const view=(args:readonly string[])=>({arguments:[`${policy.tool.placeholder}:${policy.tool.name}`,...args],environment_variables:[{name:"PATH",value:policy.environment.path},{name:"HOME",value:`${mark}/${policy.root.home}`},{name:policy.root.report,value:policy.virtual.root},{name:"CARGO_MANIFEST_DIR",value:`${mark}/.semio-offload/p`},{name:"CARGO_PKG_NAME",value:"t"}],output_paths:[policy.virtual.out,policy.virtual.root],working_directory:""});
const argumentsOf=(row:{crateType:string;target:string;linkArguments:readonly string[];flags:readonly string[]}):string[]=>["--crate-name","t","--edition=2021",".semio-offload/p/src/lib.rs","--error-format=json","--crate-type",row.crateType,"--emit=dep-info,link","--out-dir",policy.virtual.out,"--target",row.target,"-C","opt-level=s","-C","lto=thin","-C","codegen-units=16",...row.linkArguments.flatMap(value=>["-C",`link-arg=${value}`]),...row.flags.flatMap(value=>["-Z",value]),`--remap-path-prefix=${mark}=.`];

test("the policy states the release's wasm link arguments, memory and time, as the neutral vectors do",()=>{
 expect(policy.wasm.crateTypes).toEqual(["cdylib"]);
 expect(policy.wasm.peakMegabytes).toBe(vectors.release.peakMegabytes);
 expect(policy.wasm.actionSeconds).toBe(vectors.release.actionSeconds);
 expect(policy.wasm.actionSeconds,"a component final runs up to 31 minutes").toBeGreaterThan(31*60);
 expect(policy.wasm.peakMegabytes,"the cold bound of the broker covers the measured 4.7 GiB peak").toBeGreaterThan(4.7*1024);
});

test("the requester offloads a linked wasm unit only for a WebAssembly target and sizes it, following the neutral vectors",()=>{
 for(const row of vectors.requester){
  const target=row.target??undefined;
  expect(offloadCrateTypeRefusalV1(row.crateTypes,target)===undefined,row.id).toBe(row.offload);
  expect(offloadPeakMegabytesV1(row.crateTypes,target),`${row.id} peak`).toBe(row.peakMegabytes);
  expect(offloadActionSecondsV1(row.crateTypes,target),`${row.id} seconds`).toBe(row.actionSeconds);
 }
});

test("the worker accepts exactly the linked wasm actions and link arguments of the neutral vectors, and every release link argument",()=>{
 for(const row of vectors.worker){
  const refusal=validateRemoteExecutionRustcActionV1(view(argumentsOf(row)));
  if(row.refused===null)expect(refusal,row.id).toBeUndefined();else expect(refusal?.startsWith(row.refused),`${row.id}: ${refusal}`).toBe(true);
 }
 for(const value of vectors.release.linkArguments)expect(validateRemoteExecutionRustcActionV1(view(argumentsOf({crateType:"cdylib",target:"wasm32-wasip2",linkArguments:[value],flags:[]}))),value).toBeUndefined();
 expect(validateRemoteExecutionRustcActionV1(view([...argumentsOf({crateType:"cdylib",target:"wasm32-wasip2",linkArguments:[],flags:[]}),"-C","linker=/bin/sh"]))).toBe("-C linker");
 expect(validateRemoteExecutionRustcActionV1(view(argumentsOf({crateType:"cdylib",target:"wasm32-wasip2",linkArguments:[],flags:[]}).filter(value=>value!=="--target"&&value!=="wasm32-wasip2")))).toBeUndefined();
});

test("a cdylib final runs on one front-end thread and a serial back end on the worker, as the build scheduler runs it here; every other unit is unchanged",()=>{
 const final=["--crate-type","cdylib","-Z","threads=0","a.rs","-Zthreads=8"];
 expect(offloadSerialArgumentsV1(final)).toEqual(["--crate-type","cdylib","-Z","threads=1","a.rs","-Zthreads=1","-Z","no-parallel-backend"]);
 expect(offloadSerialArgumentsV1(["--crate-type=cdylib","-Z","threads=0"])).toEqual(["--crate-type=cdylib","-Z","threads=1","-Z","no-parallel-backend"]);
 const library=["--crate-type","lib","-Z","threads=0","a.rs"];
 expect(offloadSerialArgumentsV1(library)).toBe(library);
});

test("the object prefix of the macOS linker is never sent for a WebAssembly target",()=>{
 const plan=(target:string|undefined):OffloadPlanV1=>Object.freeze({arguments:["--crate-type","cdylib"],environment:{PATH:policy.environment.path,SEMIO_CWD_NAME:"."},files:new Map(),mappings:[],outDir:"out",crateName:"t",target,extraFilename:""});
 const oso=(target:string|undefined)=>offloadRustcCommandV1(plan(target),"aarch64-apple-darwin").arguments.some(value=>value.includes("oso_prefix"));
 expect(oso("wasm32-wasip2")).toBe(false);expect(oso("wasm32-unknown-unknown")).toBe(false);expect(oso(undefined)).toBe(true);expect(oso("aarch64-apple-darwin")).toBe(true);
});

test("the platform of a sized unit carries its peak memory as a control property, other units carry none",()=>{
 const toolchain={commit:"a".repeat(40),host:"aarch64-apple-darwin",digest:"b".repeat(16),targets:[]};
 expect(offloadPlatformV1([],toolchain,"wasm32-wasip2","m1",6144).find(row=>row.name===policy.property.peak)).toEqual({name:"semio-peak-mb",value:"6144"});
 expect(offloadPlatformV1([],toolchain,"wasm32-wasip2","m1").some(row=>row.name===policy.property.peak)).toBe(false);
});

const rustc=spawnSync("rustc",["--print","sysroot"],{encoding:"utf8",env:testRustEnvironmentV1()}).stdout.trim()+"/bin/rustc";
const installed=(target:string):boolean=>existsSync(rustc.replace("/bin/rustc",`/lib/rustlib/${target}`));
const HOST=/^host: (\S+)$/mu.exec(spawnSync(rustc,["-vV"],{encoding:"utf8",env:testRustEnvironmentV1()}).stdout)?.[1]??"";

/** 📁️ A requester checkout of the fixture: a library `d` and the cdylib `t` that uses it, includes a data file and a module, and names its own source file in a panic (a path the compiler records unless it is remapped). */
function checkout(label:string){
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),`wasm-cdylib-${label}-`)));scratch.push(base);
 const root=join(base,"ws✏️");
 for(const unit of["d","t"]){mkdirSync(join(root,unit,"src"),{recursive:true});writeFileSync(join(root,unit,"Cargo.toml"),`[package]\nname = "${unit}"\n`);}
 mkdirSync(join(root,"t","data"),{recursive:true});writeFileSync(join(root,"t","data","v.txt"),"vector");
 writeFileSync(join(root,"d","src","lib.rs"),'#[inline]\npub fn gen<T:Default>()->T{ if false {panic!("{}",file!())} T::default() }\n');
 writeFileSync(join(root,"t","src","lib.rs"),'mod part;\nconst D:&str=include_str!("../data/v.txt");\n#[no_mangle]\npub extern "C" fn answer(n:u32)->u32{ if n==41 {panic!("{}",file!())} n+part::q()+d::gen::<u32>()+D.len() as u32 }\n');
 writeFileSync(join(root,"t","src","part.rs"),"pub fn q()->u32{ if false {panic!(\"p\")} 2 }\n");
 return{base,root};
}
const common=(root:string,name:string,target:string,type:string,hash:string):string[]=>[...["--crate-name",name,"--edition=2021",`${name}/src/lib.rs`,"--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type",type,"--emit=dep-info,metadata,link","-C","debuginfo=2","-C","opt-level=s","-C","lto=thin","-C","codegen-units=16","-C","strip=symbols",`--remap-path-prefix=${root}=.`,"--check-cfg","cfg(docsrs,test)","-C",`metadata=${hash}`,"-C",`extra-filename=-${hash}`,"--out-dir",join(root,"out"),"-L",`dependency=${join(root,"out")}`,"--target",target,"-Z","threads=0","-Z","checksum-hash-algorithm=blake3"],...PATH_NEUTRAL_FLAGS_V1];
const environmentOf=(root:string,name:string)=>({CARGO_MANIFEST_DIR:join(root,name),CARGO_PKG_NAME:name,CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:name});
const releaseLink=(target:string):string[]=>["-C","link-arg=--max-memory=536870912","-C",`link-arg=-zstack-size=${target==="wasm32-unknown-unknown"?16777216:8388608}`];
const unitArguments=(root:string,target:string):string[]=>[...common(root,"t",target,"cdylib","def456"),"--extern",`d=${join("out","libd-abc123.rlib")}`,...releaseLink(target)];

function local(root:string,target:string):string{
 mkdirSync(join(root,"out"),{recursive:true});
 const dependency=spawnSync(rustc,common(root,"d",target,"lib","abc123"),{cwd:root,encoding:"utf8",env:testRustEnvironmentV1(environmentOf(root,"d"))});
 if(dependency.status!==0)throw Error(dependency.stderr);
 const args=unitArguments(root,target),result=spawnSync(rustc,[...args,...linkRootArgumentsV1(args,join(root,"out"))],{cwd:root,encoding:"utf8",env:testRustEnvironmentV1(environmentOf(root,"t"))});
 if(result.status!==0)throw Error(result.stderr);
 return join(root,"out");
}

/** 👷️ Runs one planned unit the way a worker does: the argv form is validated by the worker's own filter, its inputs are placed in a fresh input root, `@root` is expanded and the compiler is spawned without a shell. */
function runAction(plan:OffloadPlanV1,root:string):string{
 const built=offloadRustcCommandV1(plan,HOST),refusal=validateRemoteExecutionRustcActionV1({arguments:[...built.arguments],environment_variables:built.environment.map(([name,value])=>({name,value})),output_paths:[policy.virtual.out,policy.virtual.root],working_directory:""});
 if(refusal!==undefined)throw Error("the worker policy refuses what the requester sends: "+refusal);
 expect(built.arguments.some(value=>value==="no-parallel-backend"),"serial back end").toBe(true);
 expect(built.arguments.some(value=>/^-?Z?threads=0$/u.test(value)),"bounded front end").toBe(false);
 mkdirSync(join(root,".tmp"),{recursive:true});for(const directory of policy.root.directories)mkdirSync(join(root,directory),{recursive:true});
 for(const [path,real] of plan.files){const target=join(root,path);mkdirSync(dirname(target),{recursive:true});copyFileSync(real,target);}
 const env:Record<string,string>={};
 for(const [name,value] of built.environment)if(name!==policy.root.report)env[name]=expandRemoteExecutionRootV1(value,root,{join,sep});
 const result=spawnSync(rustc,built.arguments.map(argument=>expandRemoteExecutionRootV1(argument,root,{join,sep})).slice(1),{cwd:root,encoding:"utf8",env:{...testRustEnvironmentV1(),...env}});
 if(result.status!==0)throw Error(result.stderr);
 return join(root,policy.virtual.out);
}

const sha=(bytes:Uint8Array):string=>createHash("sha256").update(bytes).digest("hex");
const outputs=(directory:string):string[]=>readdirSync(directory).filter(name=>/^t-def456\.wasm$/u.test(name));
class Sink implements RemoteExecutionContentV1{
 async has(){return false;}
 async missing(_:string,digests:readonly RemoteExecutionDigestV1[]){return [...digests];}
 async read(){return undefined;}
 async stream(){return undefined;}
 async write(){}
 async readActionResult(){return undefined;}
 async writeActionResult(){}
}

for(const target of ["wasm32-unknown-unknown","wasm32-wasip2"]){
 test.skipIf(!installed(target))(`${target} cdylib: the linked component compiled here at two checkout paths and by a worker in two other input roots has identical bytes and digests, and the action is the same for both requesters`,async()=>{
  try{
   const a=checkout("a"),b=checkout("b"),first=local(a.root,target),second=local(b.root,target),plans=[a,b].map(machine=>{
    const plan=planOffloadV1({args:unitArguments(machine.root,target),environment:environmentOf(machine.root,"t"),cwd:machine.root});
    if("declined" in plan)throw Error(JSON.stringify(plan));
    return plan;
   });
   const remoteA=runAction(plans[0]!,join(a.base,"work","🧱️root")),remoteB=runAction(plans[1]!,join(b.base,"other","🧱️root"));
   const names=outputs(first);
   expect(names.length,"the linked wasm file").toBe(1);
   const reference=readFileSync(join(first,names[0]!));
   expect(reference.length).toBeGreaterThan(100);
   for(const [label,directory] of [["local at another path",second],["worker in the first root",remoteA],["worker in the second root",remoteB]] as const){
    expect(outputs(directory),label).toEqual(names);
    const bytes=readFileSync(join(directory,names[0]!));
    expect({hash:sha(bytes),size:bytes.length},`${target} ${label}`).toEqual({hash:sha(reference),size:reference.length});
    expect(Buffer.compare(bytes,reference),`${target} ${label}`).toBe(0);
    expect(bytes.includes(Buffer.from(a.base)),`${label} leaks the first checkout`).toBe(false);expect(bytes.includes(Buffer.from(b.base)),`${label} leaks the second checkout`).toBe(false);
   }
   const platform=offloadPlatformV1([],{commit:"a".repeat(40),host:HOST,digest:"c".repeat(16),targets:[]},target,"m1",policy.wasm.peakMegabytes),digests=await Promise.all(plans.map(async plan=>{const built=await buildOffloadActionV1({plan,content:new Sink(),instance:"main",platform});if("declined" in built)throw Error(built.detail);return built.digest;}));
   expect(digests[1],"two requesters at other paths send one action digest").toEqual(digests[0]);
  }finally{for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});}
 },600000);
}
