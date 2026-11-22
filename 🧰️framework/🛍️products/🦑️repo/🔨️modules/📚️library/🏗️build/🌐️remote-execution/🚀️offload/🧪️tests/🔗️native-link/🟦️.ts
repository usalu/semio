import {afterEach,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {copyFileSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,sep} from "node:path";
import {expandRemoteExecutionRootV1,validateRemoteExecutionRustcActionV1} from "../../../🧱️actions/🟦️.ts";
import {linkRootArgumentsV1} from "../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧭️remap/🟦️.ts";
import {PATH_NEUTRAL_FLAGS_V1} from "../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";
import {offloadRustcCommandV1} from "../../📜️action/🟦️.ts";
import {nativeLibraryFilesV1,planOffloadV1,type OffloadPlanV1} from "../../🗺️paths/🟦️.ts";
import {OFFLOAD_POLICY_V1,offloadCrateTypeRefusalV1,offloadLinkLibraryRefusalV1,offloadSearchDirectoryRefusalV1,offloadSystemDirectoryV1} from "../../📐️policy/🟦️.ts";
import {testRustEnvironmentV1} from "../🧰️workspace/🟦️.ts";
import vectors from "./🔣️.json" with {type:"json"};

const policy=OFFLOAD_POLICY_V1,mark=policy.root.placeholder,scratch:string[]=[];
afterEach(()=>{for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});

const view=(args:readonly string[],environment:readonly {name:string;value:string}[]=[])=>({arguments:[`${policy.tool.placeholder}:${policy.tool.name}`,...args],environment_variables:[{name:"PATH",value:policy.environment.path},{name:"HOME",value:`${mark}/${policy.root.home}`},{name:policy.root.report,value:policy.virtual.root},{name:"CARGO_MANIFEST_DIR",value:`${mark}/.semio-offload/p`},{name:"OUT_DIR",value:`${mark}/.semio-offload/g`},{name:"CARGO_PKG_NAME",value:"t"},...environment],output_paths:[policy.virtual.out,policy.virtual.root],working_directory:""});
const base=(extra:readonly string[]):string[]=>["--crate-name","t","--edition=2021",".semio-offload/p/src/lib.rs","--crate-type","lib","--emit=dep-info,link","--out-dir",policy.virtual.out,...extra];
const features=(names:readonly string[]):string=>`cfg(feature, values(${names.map(name=>`"${name}"`).join(", ")}))`;

test("the policy states what a native unit may link: the linked crate types, the linker arguments, the kinds, the SDK libraries and the directories that every machine has", ()=>{
 expect(policy.link.crateTypes).toEqual(["cdylib","dylib","staticlib"]);
 expect(policy.link.libraryKinds).toEqual(["static","dylib","framework"]);
 expect(policy.link.searchKinds).toEqual(["native","all","framework"]);
 expect(policy.link.arguments).toEqual(["^-Wl,-undefined,dynamic_lookup$"]);
 for(const name of ["System","c","c++","iconv","objc","z"])expect(policy.link.systemLibraries,name).toContain(name);
 for(const name of ["CoreFoundation","Metal","Foundation","AppKit"])expect(policy.link.frameworks,name).toContain(name);
 expect(policy.virtual.native).toBe(".semio-offload/n");
 expect(policy.limits.maximumCheckConfigurationBytes).toBe(vectors.checkConfiguration.maximumBytes);
 for(const name of policy.link.systemLibraries)expect(offloadLinkLibraryRefusalV1(name),name).toBeUndefined();
 for(const name of policy.link.frameworks)expect(offloadLinkLibraryRefusalV1("framework="+name),name).toBeUndefined();
});

test("the requester offloads a native linked unit for every native target and only a policy WebAssembly target links a cdylib, following the neutral vectors",()=>{
 for(const row of vectors.requester)expect(offloadCrateTypeRefusalV1(row.crateTypes,row.target??undefined)===undefined,row.id).toBe(row.offload);
});

test("a -l value is linked on another machine when it is a static library or a name that the SDK of the worker resolves, following the neutral vectors",()=>{
 for(const row of vectors.libraries)expect(offloadLinkLibraryRefusalV1(row.spec)===undefined,JSON.stringify(row.spec)).toBe(row.allowed);
});

test("a -L value reaches another machine only as a native, all or framework directory shipped with the action; the directories that every machine has are system directories",()=>{
 for(const row of vectors.searchDirectories)expect(offloadSearchDirectoryRefusalV1(row.value)===undefined,row.value).toBe(row.allowed);
 for(const row of vectors.systemDirectories)expect(offloadSystemDirectoryV1(row.path),row.path).toBe(row.system);
});

test("the worker accepts exactly the native and wasm linked actions of the neutral vectors: command lines of the release build with native libraries, wasm link arguments, a build script, and the refusals of every other library, directory and linker argument",()=>{
 for(const row of vectors.worker){
  const refusal=validateRemoteExecutionRustcActionV1(view(row.arguments));
  if(row.refused===null)expect(refusal,row.id).toBeUndefined();else expect(refusal?.startsWith(row.refused),`${row.id}: ${refusal}`).toBe(true);
 }
});

test("--check-cfg of a crate with many features is accepted whatever its length up to the declared bound: the 105 features of objc2-metal 0.3.2 (the refusal of the release build) and a longer list; a bound overrun or a shell character is refused",()=>{
 const text=features(vectors.checkConfiguration.objc2Metal.features);
 expect(text.length,"the real text exceeds the old bound of 1024").toBeGreaterThan(2000);
 for(const spelled of [["--check-cfg",text],[`--check-cfg=${text}`]])expect(validateRemoteExecutionRustcActionV1(view(base(spelled))),spelled[0]!.slice(0,12)).toBeUndefined();
 const scaled=features(Array.from({length:vectors.checkConfiguration.scaled.count},(_,index)=>vectors.checkConfiguration.scaled.prefix+index));
 expect(scaled.length).toBeGreaterThan(30000);
 for(const spelled of [["--check-cfg",scaled],[`--check-cfg=${scaled}`]])expect(validateRemoteExecutionRustcActionV1(view(base(spelled)))).toBeUndefined();
 const over=`cfg(feature, values("${"a".repeat(vectors.checkConfiguration.maximumBytes)}"))`;
 expect(validateRemoteExecutionRustcActionV1(view(base(["--check-cfg",over])))).toBeString();
 expect(validateRemoteExecutionRustcActionV1(view(base([`--check-cfg=${over}`])))).toBeString();
 for(const row of vectors.checkConfiguration.refused)expect(validateRemoteExecutionRustcActionV1(view(base(["--check-cfg",row.text]))),row.id).toBeString();
 expect(validateRemoteExecutionRustcActionV1(view(base(["--cfg",`feature="${"a".repeat(2000)}"`]))),"--cfg keeps its bound").toBe("--cfg");
 expect(validateRemoteExecutionRustcActionV1(view(base(["-A",features(["a"])]))),"another flag does not get the long text").toBeString();
});

test("the CARGO_ variables that Cargo sets from the manifests of the release (descriptions with backticks, quotes, dollars, pipes, non-ASCII dashes and newlines, authors with colons) are accepted; a NUL and a loader name next to them are not",()=>{
 for(const row of vectors.environment.accepted)expect(validateRemoteExecutionRustcActionV1(view(base([]),[row])),`${row.name}=${JSON.stringify(row.value).slice(0,60)}`).toBeUndefined();
 for(const row of vectors.environment.refused){
  const rows=[row,...("also" in row?[row.also as {name:string;value:string}]:[])];
  expect(validateRemoteExecutionRustcActionV1(view(base([]),rows)),JSON.stringify(rows).slice(0,80)).toBeString();
 }
});

const rustc=spawnSync("rustc",["--print","sysroot"],{encoding:"utf8",env:testRustEnvironmentV1()}).stdout.trim()+"/bin/rustc";
const HOST=/^host: (\S+)$/mu.exec(spawnSync(rustc,["-vV"],{encoding:"utf8",env:testRustEnvironmentV1()}).stdout)?.[1]??"";
const toolsPresent=spawnSync("cc",["--version"]).status===0&&spawnSync("ar",["--version"]).status!==null&&existsSync(rustc);

/** 📁️ A requester checkout of the fixture: a cdylib `t` whose Rust code calls the C function `nat_add` of a static library `libnat.a` in a search directory of the checkout (the shape of a crate whose build script compiled C code into its OUT_DIR), and a directory of system libraries and one of objects that must not travel. */
function checkout(label:string){
 const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),`native-link-${label}-`)));scratch.push(base);
 const root=join(base,"ws✏️"),nat=join(root,"nat");
 mkdirSync(join(root,"t","src"),{recursive:true});mkdirSync(nat,{recursive:true});mkdirSync(join(root,"out"),{recursive:true});
 writeFileSync(join(root,"t","Cargo.toml"),'[package]\nname = "t"\n');
 writeFileSync(join(root,"t","src","lib.rs"),'#[link(name = "nat", kind = "static")]\nextern "C" { fn nat_add(a: u32, b: u32) -> u32; }\n#[no_mangle]\npub extern "C" fn answer(n: u32) -> u32 { unsafe { nat_add(n, 41) } }\n');
 writeFileSync(join(nat,"nat.c"),"unsigned nat_add(unsigned a, unsigned b) { return a + b; }\n");
 writeFileSync(join(nat,"notes.log"),"build log with "+nat+"\n");
 const compiled=spawnSync("cc",["-c","-O2","-o",join(nat,"nat.o"),join(nat,"nat.c")],{cwd:nat,encoding:"utf8"}),archived=spawnSync("ar",["rcs",join(nat,"libnat.a"),join(nat,"nat.o")],{cwd:nat,encoding:"utf8"});
 if(compiled.status!==0||archived.status!==0)throw Error(compiled.stderr+archived.stderr);
 return{base,root,nat};
}
const common=(root:string,nat:string):string[]=>["--crate-name","t","--edition=2021","t/src/lib.rs","--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type","cdylib","--emit=dep-info,metadata,link","-C","debuginfo=0","-C","opt-level=s","-C","strip=symbols",`--remap-path-prefix=${root}=.`,"-C","metadata=def456","-C","extra-filename=-def456","--out-dir",join(root,"out"),"-L",`dependency=${join(root,"out")}`,"-L",`native=${nat}`,"-L","native=/usr/lib","-L","framework=/System/Library/Frameworks","-l","static=nat","-Z","threads=0",...PATH_NEUTRAL_FLAGS_V1];
const environmentOf=(root:string)=>({CARGO_MANIFEST_DIR:join(root,"t"),CARGO_PKG_NAME:"t",CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:"t"});

function local(root:string,nat:string):string{
 const args=common(root,nat),result=spawnSync(rustc,[...args,...linkRootArgumentsV1(args,join(root,"out"))],{cwd:root,encoding:"utf8",env:testRustEnvironmentV1(environmentOf(root))});
 if(result.status!==0)throw Error(result.stderr);
 return join(root,"out");
}

function runAction(plan:OffloadPlanV1,root:string):string{
 const built=offloadRustcCommandV1(plan,HOST),refusal=validateRemoteExecutionRustcActionV1({arguments:[...built.arguments],environment_variables:built.environment.map(([name,value])=>({name,value})),output_paths:[policy.virtual.out,policy.virtual.root],working_directory:""});
 if(refusal!==undefined)throw Error("the worker policy refuses what the requester sends: "+refusal);
 mkdirSync(join(root,".tmp"),{recursive:true});for(const directory of policy.root.directories)mkdirSync(join(root,directory),{recursive:true});
 for(const [path,real] of plan.files){const target=join(root,path);mkdirSync(dirname(target),{recursive:true});copyFileSync(real,target);}
 const env:Record<string,string>={};
 for(const [name,value] of built.environment)if(name!==policy.root.report)env[name]=expandRemoteExecutionRootV1(value,root,{join,sep});
 const result=spawnSync(rustc,built.arguments.map(argument=>expandRemoteExecutionRootV1(argument,root,{join,sep})).slice(1),{cwd:root,encoding:"utf8",env:{...testRustEnvironmentV1(),...env}});
 if(result.status!==0)throw Error(result.stderr);
 return join(root,policy.virtual.out);
}

const sha=(bytes:Uint8Array):string=>createHash("sha256").update(bytes).digest("hex");
const library=(directory:string):string[]=>readdirSync(directory).filter(name=>/^libt-def456\.(?:dylib|so|dll)$/u.test(name));

test.skipIf(!toolsPresent)("the native directory travels as a virtual one with its libraries only: the system directories are dropped, the objects, sources and logs stay, the arguments name the virtual directory and the mapping translates it back",()=>{
 const machine=checkout("plan"),plan=planOffloadV1({args:common(machine.root,machine.nat),environment:environmentOf(machine.root),cwd:machine.root});
 if("declined" in plan)throw Error(JSON.stringify(plan));
 const virtual=`${policy.virtual.native}0`;
 expect([...plan.files.keys()].filter(path=>path.startsWith(virtual+"/"))).toEqual([`${virtual}/libnat.a`]);
 const arguments_=plan.arguments;
 expect(arguments_.join(" ")).toContain(`-L native=${virtual}`);
 expect(arguments_.join(" "),"the system directories are not sent").not.toContain("/usr/lib");
 expect(arguments_.join(" ")).not.toContain("/System/Library");
 expect(arguments_.filter(value=>value.includes("nat"))).toEqual(expect.arrayContaining(["static=nat",`native=${virtual}`]));
 expect(plan.mappings.find(row=>row.virtual===virtual)?.real).toBe(machine.nat);
 for(const value of arguments_)expect(value.includes(machine.base),value).toBe(false);
 expect(validateRemoteExecutionRustcActionV1(((built)=>({arguments:[...built.arguments],environment_variables:built.environment.map(([name,value])=>({name,value})),output_paths:[policy.virtual.out,policy.virtual.root],working_directory:""}))(offloadRustcCommandV1(plan,HOST)))).toBeUndefined();
 expect(nativeLibraryFilesV1(machine.nat)?.size).toBe(1);
});

test.skipIf(!toolsPresent)("a dropped, a missing and an unportable search directory: system directories vanish, a missing directory vanishes, a directory of the machine outside the SDK is declined, and a dynamic library of another name is declined",()=>{
 const machine=checkout("decline"),plan=(extra:readonly string[])=>planOffloadV1({args:[...common(machine.root,machine.nat),...extra],environment:environmentOf(machine.root),cwd:machine.root});
 expect("declined" in plan(["-L","native="+join(machine.root,"missing")])).toBe(false);
 const homebrew=plan(["-L","native=/opt/homebrew/lib"]);
 expect("declined" in homebrew&&homebrew.declined).toBe("not-portable");
 const jvm=plan(["-l","dylib=jvm"]);
 expect("declined" in jvm&&jvm.declined).toBe("not-portable");
 const bare=plan(["-L",join(machine.root,"missing")]);
 expect("declined" in bare,"a bare directory is an all directory").toBe(false);
 const arg=plan(["-C","link-arg=-Wl,-rpath,/tmp"]);
 expect("declined" in arg&&arg.declined).toBe("not-portable");
});

test.skipIf(!toolsPresent)(`the native cdylib with a static library of the checkout compiled here at two checkout paths and by a worker in two other input roots has identical bytes, and the worker's own filter accepts what the requester sends (host ${HOST})`,()=>{
 const a=checkout("a"),b=checkout("b"),first=local(a.root,a.nat),second=local(b.root,b.nat),plans=[a,b].map(machine=>{
  const plan=planOffloadV1({args:common(machine.root,machine.nat),environment:environmentOf(machine.root),cwd:machine.root});
  if("declined" in plan)throw Error(JSON.stringify(plan));
  return plan;
 });
 const remoteA=runAction(plans[0]!,join(a.base,"work","🧱️root")),remoteB=runAction(plans[1]!,join(b.base,"other","🧱️root")),names=library(first);
 expect(names.length,"the linked library").toBe(1);
 const reference=readFileSync(join(first,names[0]!));
 expect(reference.length).toBeGreaterThan(100);
 for(const [label,directory] of [["local at another path",second],["worker in the first root",remoteA],["worker in the second root",remoteB]] as const){
  expect(library(directory),label).toEqual(names);
  const bytes=readFileSync(join(directory,names[0]!));
  expect({hash:sha(bytes),size:bytes.length},label).toEqual({hash:sha(reference),size:reference.length});
  expect(bytes.includes(Buffer.from(a.base)),`${label} leaks the first checkout`).toBe(false);expect(bytes.includes(Buffer.from(b.base)),`${label} leaks the second checkout`).toBe(false);
 }
},600000);
