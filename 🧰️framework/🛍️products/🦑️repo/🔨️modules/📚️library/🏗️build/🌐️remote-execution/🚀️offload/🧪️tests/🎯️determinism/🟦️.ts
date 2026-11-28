import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {copyFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,sep} from "node:path";
import {linkRootArgumentsV1} from "../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧭️remap/🟦️.ts";
import {PATH_NEUTRAL_FLAGS_V1} from "../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";
import {OFFLOAD_POLICY_V1} from "../../📐️policy/🟦️.ts";
import {moduleRemapArgumentsV1,moduleRemapV1,planOffloadV1,type OffloadPlanV1} from "../../🗺️paths/🟦️.ts";
import {offloadRustcCommandV1} from "../../📜️action/🟦️.ts";
import {expandRemoteExecutionRootV1,validateRemoteExecutionRustcActionV1} from "../../../🧱️actions/🟦️.ts";
import {testRustEnvironmentV1} from "../🧰️workspace/🟦️.ts";

const policy=OFFLOAD_POLICY_V1,scratch:string[]=[];
const cleanup=():void=>{for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});};
const rustc=spawnSync("rustc",["--print","sysroot"],{encoding:"utf8",env:testRustEnvironmentV1()}).stdout.trim()+"/bin/rustc";
const wasmInstalled=existsSync(rustc.replace("/bin/rustc","/lib/rustlib/wasm32-unknown-unknown"));
const HOST=/^host: (\S+)$/mu.exec(spawnSync(rustc,["-vV"],{encoding:"utf8",env:testRustEnvironmentV1()}).stdout)?.[1]??"";

/** 👷️ Runs one planned unit the way a worker does: the argv form is validated, its inputs are placed in a fresh input root, `@root` is expanded and the compiler is spawned without a shell. */
function runAction(plan:OffloadPlanV1,root:string):string{
 const built=offloadRustcCommandV1(plan,HOST),refusal=validateRemoteExecutionRustcActionV1({arguments:[...built.arguments],environment_variables:built.environment.map(([name,value])=>({name,value})),output_paths:[policy.virtual.out,policy.virtual.root],working_directory:""});
 if(refusal!==undefined)throw Error("the worker policy refuses what the planner sends: "+refusal);
 mkdirSync(root,{recursive:true});mkdirSync(join(root,".tmp"),{recursive:true});for(const directory of policy.root.directories)mkdirSync(join(root,directory),{recursive:true});
 for(const [path,real] of plan.files){const target=join(root,path);mkdirSync(dirname(target),{recursive:true});copyFileSync(real,target);}
 const env:Record<string,string>={};
 for(const [name,value] of built.environment)if(name!==policy.root.report)env[name]=expandRemoteExecutionRootV1(value,root,{join,sep});
 const argv=built.arguments.map(argument=>expandRemoteExecutionRootV1(argument,root,{join,sep}));
 const result=spawnSync(rustc,argv.slice(1),{cwd:root,encoding:"utf8",env:{...testRustEnvironmentV1(),...env}});
 if(result.status!==0)throw Error(result.stderr);
 return join(root,policy.virtual.out);
}

type Kind=Readonly<{label:string;type:string;target?:string}>;

/** 📁️ A checkout in the module layout of the repository: a unit `d` (a generic inline function and a macro) and the unit `t` under test, whose one-line shim includes the code of the module directory above its manifest and which uses `d`. */
function checkout(label:string,kind:Kind){
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),`determinism-${label}-`)));scratch.push(base);
 const root=join(base,"ws✏️"),macro=kind.type==="proc-macro";
 for(const unit of["d","t"]){mkdirSync(join(root,unit,"📦️packages","🦀️rust"),{recursive:true});writeFileSync(join(root,unit,"📦️packages","🦀️rust","Cargo.toml"),`[package]\nname = "${unit}"\n`);}
 mkdirSync(join(root,"t","data"),{recursive:true});writeFileSync(join(root,"t","data","v.txt"),"v");writeFileSync(join(root,"t","data","w.txt"),"w");
 writeFileSync(join(root,"d","📦️packages","🦀️rust","🦀️.rs"),'#[path = "../../🦀️.rs"]\nmod component;\npub use component::*;\n');
 writeFileSync(join(root,"d","🦀️.rs"),'#[inline]\npub fn gen<T:Default>()->T{ if false {panic!("{}",file!())} T::default() }\n#[macro_export]\nmacro_rules! mm{()=>{1}}\n');
 writeFileSync(join(root,"t","📦️packages","🦀️rust","🦀️.rs"),macro?'#[path = "../../🦀️.rs"]\nmod component;\nextern crate proc_macro;\nuse proc_macro::TokenStream;\n#[proc_macro]\npub fn m(_:TokenStream)->TokenStream{ let _=component::D; "1".parse().unwrap() }\n':'#[path = "../../🦀️.rs"]\nmod component;\npub use component::*;\n');
 writeFileSync(join(root,"t","🦀️.rs"),'mod part;\npub const D:&str=include_str!("data/v.txt");\npub const E:&str=include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../data/w.txt"));\n'+(macro?"":'pub fn p()->u32{part::q()+d::gen::<u32>()+d::mm!()}\n'));
 writeFileSync(join(root,"t","part.rs"),"pub fn q()->u32{panic_here()}\n#[track_caller]\nfn panic_here()->u32{if false{panic!(\"p\")}2}\n");
 return{base,root};
}
const withNeutralScope=(args:string[]):string[]=>[...args.filter(argument=>!argument.startsWith("--remap-path-scope")),...PATH_NEUTRAL_FLAGS_V1];
const common=(root:string,kind:Kind,name:string,hash:string):string[]=>withNeutralScope(["--crate-name",name,"--edition=2021",`${name==="d"?"d":"t"}/📦️packages/🦀️rust/🦀️.rs`,"--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type",name==="d"?"lib":kind.type,"--emit=dep-info,metadata,link","-Z","embed-metadata=no","-C","embed-bitcode=no","-C","debuginfo=2","--remap-path-scope=object",`--remap-path-prefix=${root}=.`,"--check-cfg","cfg(docsrs,test)","-C",`metadata=${hash}`,"-C",`extra-filename=-${hash}`,"--out-dir",join(root,"out"),"-L",`dependency=${join(root,"out")}`,...kind.target?["--target",kind.target]:[],"-Z","threads=0","-Z","checksum-hash-algorithm=blake3"]);
const environmentOf=(root:string,name:string)=>({CARGO_MANIFEST_DIR:join(root,name,"📦️packages","🦀️rust"),CARGO_PKG_NAME:name,CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:name,RUST_MIN_STACK:"8388608"});
const unitArguments=(root:string,kind:Kind):string[]=>[...common(root,kind,"t","def456"),"--extern",`d=${join(root,"out","libd-abc123.rmeta")}`.replace(root+"/","")];
const names=(directory:string):string[]=>existsSync(directory)?readdirSync(directory).filter(name=>/^lib(?:t)-.*\.(?:rmeta|rlib|dylib|so)$|^t-.*\.d$/u.test(name)).sort():[];

function local(root:string,kind:Kind):string{
 mkdirSync(join(root,"out"),{recursive:true});
 const dependency=spawnSync(rustc,common(root,{label:"d",type:"lib",...kind.target?{target:kind.target}:{}},"d","abc123"),{cwd:root,encoding:"utf8",env:testRustEnvironmentV1(environmentOf(root,"d"))});
 if(dependency.status!==0)throw Error(dependency.stderr);
 const linked=unitArguments(root,kind),result=spawnSync(rustc,[...linked,...linkRootArgumentsV1(linked,join(root,"out"))],{cwd:root,encoding:"utf8",env:testRustEnvironmentV1(environmentOf(root,"t"))});
 if(result.status!==0)throw Error(result.stderr);
 return join(root,"out");
}

function worker(requester:string,kind:Kind,base:string):string{
 const plan=planOffloadV1({args:unitArguments(requester,kind),environment:environmentOf(requester,"t"),cwd:requester});
 if("declined" in plan)throw Error(JSON.stringify(plan));
 return runAction(plan,join(base,"work","🧱️root"));
}

const cases:Kind[]=[{label:"lib",type:"lib"},{label:"proc-macro",type:"proc-macro"},...(wasmInstalled?[{label:"wasm32",type:"lib",target:"wasm32-unknown-unknown"} as Kind]:[])];
for(const kind of cases){
 test(`${kind.label}: a unit compiled locally at one path, locally at another path and in a worker root gives the same bytes (rmeta, rlib, dep-info with the requester's paths)`,()=>{
  try{
   const a=checkout("a",kind),b=checkout("b",kind),base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"determinism-w-")));scratch.push(base);
   const here=local(a.root,kind),other=local(b.root,kind),remote=worker(a.root,kind,base);
   expect(names(here).length,"rmeta and dep-info, and the rlib of a library").toBeGreaterThan(1);
   for(const name of names(here)){
    const left=readFileSync(join(here,name)),right=readFileSync(join(other,name)),far=readFileSync(join(remote,name));
    if(name.endsWith(".d")){expect(far.toString().length>0).toBe(true);continue;}
    expect(Buffer.compare(left,right),`${kind.label} ${name}: two local checkouts`).toBe(0);
    expect(Buffer.compare(left,far),`${kind.label} ${name}: worker against local (${left.length} against ${far.length} bytes)`).toBe(0);
   }
  }finally{cleanup();}
 },180000);
}

const packageArguments=(root:string,kind:Kind,names:readonly string[]):string[]=>{
 const manifest=join(root,"t","📦️packages","🦀️rust"),out=join(root,"out");
 const kept=unitArguments(root,kind).filter(argument=>argument!==`--remap-path-prefix=${root}=.`).map(argument=>argument==="t/📦️packages/🦀️rust/🦀️.rs"?"🦀️.rs":argument.startsWith("d=")?`d=${join(out,"libd-abc123.rmeta")}`:argument);
 return [...kept,`--remap-path-prefix=${manifest}=${names[0]}`];
};
test("lib: a unit compiled in its package directory whose module directory above the manifest no remap of Cargo covers gets its own remap from the launcher and has the same bytes (crate hash included) locally and in a worker",()=>{
 try{
  const kind:Kind={label:"lib",type:"lib"},a=checkout("m",kind),manifest=join(a.root,"t","📦️packages","🦀️rust"),base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"determinism-m-")));scratch.push(base);
  mkdirSync(join(a.root,"out"),{recursive:true});
  const dependency=spawnSync(rustc,common(a.root,{label:"d",type:"lib"},"d","abc123"),{cwd:a.root,encoding:"utf8",env:testRustEnvironmentV1(environmentOf(a.root,"d"))});
  if(dependency.status!==0)throw Error(dependency.stderr);
  const cargoGave=packageArguments(a.root,kind,["."]),linked=[...moduleRemapArgumentsV1(cargoGave,environmentOf(a.root,"t"),manifest,a.root)??cargoGave],here=spawnSync(rustc,[...linked,...linkRootArgumentsV1(linked,join(a.root,"out"))],{cwd:manifest,encoding:"utf8",env:testRustEnvironmentV1(environmentOf(a.root,"t"))});
  if(here.status!==0)throw Error(here.stderr);
  expect(linked.length,"the launcher added the remap of the module directory").toBe(cargoGave.length+1);
  expect(linked.some(argument=>argument===`--remap-path-prefix=${join(a.root,"t")}=t`)).toBe(true);
  const plan=planOffloadV1({args:linked,environment:environmentOf(a.root,"t"),cwd:manifest});
  if("declined" in plan)throw Error(JSON.stringify(plan));
  const remote=runAction(plan,join(base,"work","🧱️root"));
  expect(names(join(a.root,"out")).length).toBeGreaterThan(1);
  for(const name of names(join(a.root,"out"))){
   if(name.endsWith(".d"))continue;
   expect(Buffer.compare(readFileSync(join(a.root,"out",name)),readFileSync(join(remote,name))),name).toBe(0);
  }
 }finally{cleanup();}
},180000);

test("lib: a unit whose #[path] modules live in another top-level tree, so that the module directory is the workspace root itself, gets the remap `workspace=.` from the launcher and has the same bytes locally at two checkout paths and in a worker",()=>{
 try{
  const kind:Kind={label:"lib",type:"lib"},prepare=(label:string)=>{
   const side=checkout(label,kind),manifest=join(side.root,"t","📦️packages","🦀️rust");
   mkdirSync(join(side.root,"x"),{recursive:true});writeFileSync(join(side.root,"x","🦀️.rs"),"pub fn o()->u32{if false{panic!(\"{}\",file!())}7}\n");
   writeFileSync(join(side.root,"t","🦀️.rs"),readFileSync(join(side.root,"t","🦀️.rs"),"utf8")+'#[path = "../x/🦀️.rs"]\nmod outside;\npub fn o()->u32{outside::o()}\n');
   mkdirSync(join(side.root,"out"),{recursive:true});
   const dependency=spawnSync(rustc,common(side.root,{label:"d",type:"lib"},"d","abc123"),{cwd:side.root,encoding:"utf8",env:testRustEnvironmentV1(environmentOf(side.root,"d"))});
   if(dependency.status!==0)throw Error(dependency.stderr);
   const cargoGave=packageArguments(side.root,kind,["."]),linked=[...moduleRemapArgumentsV1(cargoGave,environmentOf(side.root,"t"),manifest,side.root)??cargoGave],here=spawnSync(rustc,[...linked,...linkRootArgumentsV1(linked,join(side.root,"out"))],{cwd:manifest,encoding:"utf8",env:testRustEnvironmentV1(environmentOf(side.root,"t"))});
   if(here.status!==0)throw Error(here.stderr);
   return{side,manifest,linked,cargoGave};
  };
  const a=prepare("wa"),b=prepare("wb"),base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"determinism-w2-")));scratch.push(base);
  expect(a.linked.length,"the launcher added the remap of the workspace root").toBe(a.cargoGave.length+1);
  expect(a.linked.some(argument=>argument===`--remap-path-prefix=${a.side.root}=.`)).toBe(true);
  const plan=planOffloadV1({args:a.linked,environment:environmentOf(a.side.root,"t"),cwd:a.manifest});
  if("declined" in plan)throw Error(JSON.stringify(plan));
  expect([...plan.files.keys()].some(name=>name.endsWith("/x/🦀️.rs")),"the module of the other tree travels").toBe(true);
  const remote=runAction(plan,join(base,"work","🧱️root"));
  expect(names(join(a.side.root,"out")).length).toBeGreaterThan(1);
  for(const name of names(join(a.side.root,"out"))){
   if(name.endsWith(".d"))continue;
   const here=readFileSync(join(a.side.root,"out",name));
   expect(Buffer.compare(here,readFileSync(join(remote,name))),name+": local against worker").toBe(0);
   expect(Buffer.compare(here,readFileSync(join(b.side.root,"out",name))),name+": two checkout paths").toBe(0);
  }
 }finally{cleanup();}
},180000);

type Source=Readonly<{label:string;below:string;remap:string;crate:string}>;
const SOURCES:readonly Source[]=[
 {label:"registry crate",below:"registry/src/index.crates.io-1949cf8c6b5b557f/reg-1.0.0",remap:"registry/src",crate:"reg"},
 {label:"git crate",below:"git/checkouts/gitreg-0123456789abcdef/abcdef1/crates/reg",remap:"git/checkouts",crate:"reg"},
];
const COMMIT=/^commit-hash: ([0-9a-f]{40})$/mu.exec(spawnSync(rustc,["-vV"],{encoding:"utf8",env:testRustEnvironmentV1()}).stdout)?.[1]??"0".repeat(40);

/** 📁️ One machine: its own home, `CARGO_HOME` and checkout below a fresh directory, so two machines differ in every absolute path (user name, cargo home, rustup home, checkout, target directory); the package sits in the cargo source cache like a registry or git dependency, and a workspace unit with a build script output uses it. */
function machine(label:string,source:Source){
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),`determinism-${label}-`)));scratch.push(base);
 const home=join(base,`user-${label}`),cargoHome=join(home,".cargo"),rustup=join(home,".rustup","toolchains","t","lib","rustlib","src","rust"),checkout=join(base,"ws✏️"),target=join(checkout,"t"),deps=join(target,"debug","deps"),pkg=join(cargoHome,source.below);
 mkdirSync(join(pkg,"src"),{recursive:true});mkdirSync(deps,{recursive:true});mkdirSync(join(checkout,"w","src"),{recursive:true});
 writeFileSync(join(pkg,"Cargo.toml"),`[package]\nname = "reg"\nversion = "1.0.0"\n`);
 writeFileSync(join(pkg,"src","lib.rs"),'mod part;\npub fn gen<T:Default>()->T{ if false {panic!("{}",file!())} T::default() }\npub const N:&str=file!();\npub fn q()->u32{part::q()}\n#[macro_export]\nmacro_rules! rm{()=>{1}}\n');
 writeFileSync(join(pkg,"src","part.rs"),"pub fn q()->u32{if false{panic!(\"p\")}2}\n");
 writeFileSync(join(checkout,"w","Cargo.toml"),`[package]\nname = "w"\nversion = "0.1.0"\n`);
 const out=join(target,"debug","build","w-0123456789abcdef","out");mkdirSync(out,{recursive:true});writeFileSync(join(out,"gen.rs"),"pub const G:u32=3;\n");
 writeFileSync(join(checkout,"w","src","lib.rs"),'include!(concat!(env!("OUT_DIR"),"/gen.rs"));\npub fn p()->u32{reg::gen::<u32>()+reg::q()+reg::rm!()+G}\n');
 return{base,home,cargoHome,rustup,checkout,target,deps,pkg,out};
}
type Machine=ReturnType<typeof machine>;
const remapsOf=(m:Machine,source:Source):string[]=>["--remap-path-scope=object",`--remap-path-prefix=${join(m.cargoHome,source.remap)}=`,`--remap-path-prefix=${m.target}=/cargo/build-dir`,`--remap-path-prefix=${m.rustup}=/rustc/${COMMIT}`];
const registryArguments=(m:Machine,source:Source):string[]=>withNeutralScope(["--crate-name","reg","--edition=2021",join(m.pkg,"src","lib.rs"),"--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type","lib","--emit=dep-info,metadata,link","-Z","embed-metadata=no","-C","embed-bitcode=no","-C","debuginfo=2",...remapsOf(m,source),"--check-cfg","cfg(docsrs,test)","-C","metadata=aa11bb22","-C","extra-filename=-aa11bb22","--out-dir",m.deps,"-L",`dependency=${m.deps}`,"--cap-lints","allow","-Z","threads=0"]);
const dependantArguments=(m:Machine,source:Source):string[]=>withNeutralScope(["--crate-name","w","--edition=2021","w/src/lib.rs","--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type","lib","--emit=dep-info,metadata,link","-Z","embed-metadata=no","-C","embed-bitcode=no","-C","debuginfo=2",...remapsOf(m,source),`--remap-path-prefix=${m.checkout}=.`,"--check-cfg","cfg(docsrs,test)","-C","metadata=cc33dd44","-C","extra-filename=-cc33dd44","--out-dir",m.deps,"-L",`dependency=${m.deps}`,"--extern",`reg=${join(m.deps,"libreg-aa11bb22.rmeta")}`,"-Z","threads=0"]);
const registryEnvironment=(m:Machine)=>({CARGO_MANIFEST_DIR:m.pkg,CARGO_PKG_NAME:"reg",CARGO_PKG_VERSION:"1.0.0",CARGO_CRATE_NAME:"reg",HOME:m.home,CARGO_HOME:m.cargoHome,RUST_MIN_STACK:"8388608"});
const dependantEnvironment=(m:Machine)=>({CARGO_MANIFEST_DIR:join(m.checkout,"w"),CARGO_PKG_NAME:"w",CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:"w",CARGO_PRIMARY_PACKAGE:"1",OUT_DIR:m.out,HOME:m.home,CARGO_HOME:m.cargoHome,RUST_MIN_STACK:"8388608"});
const unitNames=(directory:string,crate:string):string[]=>existsSync(directory)?readdirSync(directory).filter(name=>new RegExp(`^lib${crate}-.*\\.(?:rmeta|rlib)$`,"u").test(name)).sort():[];

function compileHere(m:Machine,source:Source,which:"reg"|"w"):void{
 const registryUnit=which==="reg",result=spawnSync(rustc,registryUnit?registryArguments(m,source):dependantArguments(m,source),{cwd:registryUnit?m.pkg:m.checkout,encoding:"utf8",env:testRustEnvironmentV1(registryUnit?registryEnvironment(m):dependantEnvironment(m))});
 if(result.status!==0)throw Error(result.stderr);
}
function compileThere(requester:Machine,source:Source,which:"reg"|"w",base:string,label:string):string{
 const registryUnit=which==="reg",cwd=registryUnit?requester.pkg:requester.checkout,plan=planOffloadV1({args:registryUnit?registryArguments(requester,source):dependantArguments(requester,source),environment:registryUnit?registryEnvironment(requester):dependantEnvironment(requester),cwd});
 if("declined" in plan)throw Error(JSON.stringify(plan));
 return runAction(plan,join(base,"work",label,"🧱️root"));
}
for(const source of SOURCES){
 test(`${source.label}: compiled on two machines with different user, cargo home, rustup home and checkout, and through the planner in a worker root, gives the same rmeta and rlib; so does a workspace unit with a build script output that uses it`,()=>{
  try{
   const a=machine("a",source),b=machine("b",source),base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"determinism-w-")));scratch.push(base);
   compileHere(a,source,"reg");compileHere(b,source,"reg");
   const far=compileThere(a,source,"reg",base,"reg");
   expect(unitNames(a.deps,"reg").length).toBe(2);
   for(const name of unitNames(a.deps,"reg")){
    const left=readFileSync(join(a.deps,name)),right=readFileSync(join(b.deps,name)),worker=readFileSync(join(far,name));
    expect(Buffer.compare(left,right),`${source.label} ${name}: two machines (${left.length} against ${right.length} bytes)`).toBe(0);
    expect(Buffer.compare(left,worker),`${source.label} ${name}: worker against local (${left.length} against ${worker.length} bytes)`).toBe(0);
   }
   compileHere(a,source,"w");compileHere(b,source,"w");
   const dependant=compileThere(a,source,"w",base,"w");
   expect(unitNames(a.deps,"w").length).toBe(2);
   for(const name of unitNames(a.deps,"w")){
    const left=readFileSync(join(a.deps,name)),right=readFileSync(join(b.deps,name)),worker=readFileSync(join(dependant,name));
    expect(Buffer.compare(left,right),`${source.label} dependant ${name}: two machines (${left.length} against ${right.length} bytes)`).toBe(0);
    expect(Buffer.compare(left,worker),`${source.label} dependant ${name}: worker against local (${left.length} against ${worker.length} bytes)`).toBe(0);
   }
  }finally{cleanup();}
 },240000);
}

test("the module remap of a unit says what it came to: a remap of its directory, nothing needed, or the guard that declined it (a declined unit is never shared)",()=>{
 try{
  const {base,root}=checkout("remap",{label:"lib",type:"lib"}),manifest=join(root,"t","📦️packages","🦀️rust"),shipped=join(root,"t"),environment={CARGO_MANIFEST_DIR:manifest};
  const args=["--crate-name","t","--edition=2021","🦀️.rs","--crate-type","lib","--emit=metadata","--out-dir","/o"],remap=`--remap-path-prefix=${shipped}=t`;
  const made=moduleRemapV1(args,environment,manifest,root);
  expect("arguments" in made&&made.arguments.includes(remap)&&made.arguments.includes(join(manifest,"🦀️.rs"))).toBe(true);
  expect(moduleRemapArgumentsV1(args,environment,manifest,root)).toEqual("arguments" in made?made.arguments:undefined);
  expect(moduleRemapV1(args,environment,manifest,undefined)).toEqual({declined:"no-workspace"});
  expect(moduleRemapV1(args,environment,manifest,join(base,"elsewhere"))).toEqual({declined:"outside-workspace"});
  const link=join(base,"link");symlinkSync(root,link);
  const linked=moduleRemapV1(args,environment,manifest,link);
  expect("arguments" in linked&&linked.arguments.includes(remap),"a workspace spelled through a link names the directory like the manifest").toBe(true);
  expect(moduleRemapV1([...args,`--remap-path-prefix=${root}=⟨root⟩`],environment,manifest,root)).toEqual({needless:true});
  writeFileSync(join(manifest,"local.rs"),"pub fn f()->u32{1}\n");
  expect(moduleRemapV1(["--crate-name","t","local.rs"],environment,manifest,root)).toEqual({needless:true});
  expect(moduleRemapV1(args,{},manifest,root)).toEqual({needless:true});
 }finally{cleanup();}
});
