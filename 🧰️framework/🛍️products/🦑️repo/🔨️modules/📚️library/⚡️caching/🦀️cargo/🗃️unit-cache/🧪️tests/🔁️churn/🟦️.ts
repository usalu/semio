import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {cargoScratchBaseV1} from "../🏖️scratch/🟦️.ts";
import {differingComponentsV1,keyEnvironmentV1,normalizeRustcArgumentsV1,unitKeyComponentsV1,unitKeyV1} from "../../🔏️key/🟦️.ts";
import {churnReportV1,digestFileV1,emitUnitEventV1,readEvictionsV1,readLookupsV1,recordEvictionV1,recordLookupV1} from "../../💾️store/🟦️.ts";
import {applyGcV1} from "../../🧹️gc/🟦️.ts";
import {enterLaneV1,listLanesV1} from "../../🧵️lane/🟦️.ts";
import {portableKeyV1,portableRootsV1} from "../../🛰️fleet/🪪️identity/🟦️.ts";
import {externIdentityFileV1,externIdentityV1} from "../../🪪️extern/🟦️.ts";
import {manageV1} from "../../📜️script.ts";

/** 🔁️ Churn laws: what must not split a unit (the member selection, the terminal, the primary package, the build directory, the checkout, the lane) does not, what really is another unit (features, profile, emit) is named by the report, and a dependency that is rebuilt to the same artifact leaves its dependants' keys alone. */
const here=fileURLToPath(new URL(".",import.meta.url)),script=resolve(here,"../../📜️script.ts"),base=cargoScratchBaseV1(),cargo=process.env.SEMIO_TEST_CARGO??"cargo";
const scratch:string[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(base,"ch-"+label+"-")));scratch.push(path);return path;};
afterAll(()=>{if(!process.env.SEMIO_TEST_KEEP)for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const cargoArguments=(extra:readonly string[])=>["--crate-name","dep","--edition=2021","src/lib.rs","--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat",...extra,"--crate-type","lib","--emit=dep-info,metadata,link","-C","embed-bitcode=no","-C","metadata=abc","-C","extra-filename=-abc","--out-dir","/lane/t/debug/deps","-L","dependency=/lane/t/debug/deps","--extern","core=/lane/t/debug/deps/libcore-1.rmeta"];
const keyOf=(args:readonly string[],environment:Record<string,string|undefined>,laneRoot="/lane",digest="d1")=>unitKeyV1({toolchain:"t",normalized:normalizeRustcArgumentsV1(args,new Map([["/lane/t/debug/deps/libcore-1.rmeta",digest],["/other/lane/t/debug/deps/libcore-1.rmeta",digest]]),laneRoot),sourceSha256:"a".repeat(64),externs:[{name:"core",sha256:digest}],environment:keyEnvironmentV1(environment,laneRoot)});

test("the terminal width, the primary package and the lane do not split a unit",()=>{
 const plain=keyOf(cargoArguments([]),{CARGO_PKG_NAME:"dep"});
 for(const width of[["--diagnostic-width=80"],["--diagnostic-width=211"],["--diagnostic-width","97"]])expect(keyOf(cargoArguments(width),{CARGO_PKG_NAME:"dep"}),width.join(" ")).toBe(plain);
 expect(keyOf(cargoArguments([]),{CARGO_PKG_NAME:"dep",CARGO_PRIMARY_PACKAGE:"1"})).toBe(plain);
 expect(keyOf(cargoArguments([]).map(argument=>argument.replace("/lane/","/other/lane/")),{CARGO_PKG_NAME:"dep"},"/other/lane")).toBe(plain);
 expect(keyOf(cargoArguments([]),{CARGO_PKG_NAME:"dep",CARGO_ENCODED_RUSTFLAGS:"-Zthreads=4"}),"real rustflags are still inputs").not.toBe(plain);
 expect(keyOf(cargoArguments(["--cfg","feature=\"a\""]),{CARGO_PKG_NAME:"dep"}),"features are still inputs").not.toBe(plain);
 expect(keyOf(cargoArguments(["--color=always"]),{CARGO_PKG_NAME:"dep"}),"colour changes the replayed text").not.toBe(plain);
});

test("two machines with different roots, target directories and homes give one portable key",()=>{
 const machine=(root:string,target:string,home:string)=>{
  const args=cargoArguments([`--remap-path-prefix=${root}=.`,`--remap-path-prefix=${target}=/cargo/build-dir`,`--remap-path-prefix=${home}/.cargo/registry/src=`]).map(argument=>argument.replace("/lane/t",target));
  const roots=portableRootsV1({cwd:root,workspace:root,hints:[root]});
  return portableKeyV1({toolchain:"t",normalized:normalizeRustcArgumentsV1(args,new Map([[target+"/debug/deps/libcore-1.rmeta","d1"]]),undefined),sourceSha256:"a".repeat(64),externs:[{name:"core",sha256:"d1"}],environment:keyEnvironmentV1({CARGO_PKG_NAME:"dep",CARGO_MANIFEST_DIR:root+"/dep",CARGO_PRIMARY_PACKAGE:"1"},undefined)},roots);
 };
 const one=machine("/Users/ueli/Documents/semio","/Users/ueli/Documents/semio/.cache/lane-a/t-ce555540","/Users/ueli"),two=machine("/home/deck/semio","/tmp/other-target","/home/deck");
 expect(one).toBe(two);
});

test("a dependency rebuilt to the same artifact leaves its dependants' keys alone (early cutoff)",()=>{
 const root=directory("cutoff"),a=join(root,"a","deps"),b=join(root,"b","deps");mkdirSync(a,{recursive:true});mkdirSync(b,{recursive:true});
 const rmeta=Buffer.from("rmeta-bytes-v1"),changed=Buffer.from("rmeta-bytes-v2");
 writeFileSync(join(a,"libdep-1.rmeta"),rmeta);writeFileSync(join(b,"libdep-9.rmeta"),rmeta);
 const first=digestFileV1(join(a,"libdep-1.rmeta"),undefined),second=digestFileV1(join(b,"libdep-9.rmeta"),undefined);
 expect(first).toBe(second);
 const dependency=(source:string)=>unitKeyV1({toolchain:"t",normalized:["--crate-name","dep"],sourceSha256:source.repeat(64),externs:[],environment:[]});
 expect(dependency("a"),"a body-only change moves the dependency's own key").not.toBe(dependency("b"));
 const dependant=(path:string,digest:string)=>unitKeyV1({toolchain:"t",normalized:normalizeRustcArgumentsV1(["--crate-name","user","src/lib.rs","--extern","dep="+path],new Map([[path,digest]]),undefined),sourceSha256:"c".repeat(64),externs:[{name:"dep",sha256:digest}],environment:[]});
 expect(dependant(join(a,"libdep-1.rmeta"),first!),"the dependant sees the artifact's bytes, not the dependency's key or path").toBe(dependant(join(b,"libdep-9.rmeta"),second!));
 writeFileSync(join(b,"libdep-9.rmeta"),changed);
 expect(dependant(join(b,"libdep-9.rmeta"),digestFileV1(join(b,"libdep-9.rmeta"),undefined)!)).not.toBe(dependant(join(a,"libdep-1.rmeta"),first!));
});

test("a proc-macro library is identified by the inputs of the unit that built it, so a relinked copy with other bytes is the same dependency",()=>{
 const root=directory("dylib"),path=join(root,"libmac-1.dylib");
 const stamp=(bytes:string,identity:string)=>{writeFileSync(path,bytes);const sha=digestFileV1(path,undefined)!;writeFileSync(externIdentityFileV1(path),JSON.stringify({version:1,sha256:sha,identity}));return externIdentityV1(path,sha,undefined);};
 const identity="e".repeat(64);
 expect(stamp("linked-by-machine-one",identity)).toBe(stamp("linked-by-machine-two",identity));
 expect(stamp("linked-by-machine-one",identity)).not.toBe(stamp("linked-by-machine-one","f".repeat(64)));
 writeFileSync(path,"edited after the stamp");
 expect(externIdentityV1(path,digestFileV1(path,undefined)!,undefined),"a stamp is trusted only while it names the library's current bytes").toBeUndefined();
});

test("the churn report names the components that differ between the keys of one crate",()=>{
 const input=(extra:Partial<Parameters<typeof unitKeyV1>[0]>={})=>({toolchain:"t",normalized:["--crate-name","dep","--cfg","feature=\"a\"","-C","metadata=1"],sourceSha256:"a".repeat(64),externs:[{name:"core",sha256:"b".repeat(64)}],environment:[["CARGO_PKG_NAME","dep"]] as Array<[string,string]>,...extra});
 const rows=[input(),input({normalized:["--crate-name","dep","--cfg","feature=\"a\"","--cfg","feature=\"b\"","-C","metadata=2"]}),input({externs:[{name:"core",sha256:"c".repeat(64)}]})].map((value,index)=>{const made=unitKeyComponentsV1(value);return{version:1 as const,atMs:index,lane:"d",pid:1,crate:"dep",key:unitKeyV1(value),outcome:"miss" as const,components:made.components,values:made.values};});
 const report=churnReportV1([...rows,{...rows[0]!,crate:"steady"}]);
 expect(report.multiKey).toBe(1);expect(report.perCrate[0]!.crate).toBe("dep");
 expect(report.perCrate[0]!.differs).toEqual(["args.features","args.metadata","extern:core"]);
 expect(report.causes).toEqual([{components:["args.features","args.metadata","extern:core"],crates:1,examples:["dep"]}]);
 expect(report.perCrate[0]!.samples["args.features"]!.length).toBe(2);
 expect(differingComponentsV1([rows[0]!.components,rows[0]!.components])).toEqual([]);
});

test("the lookup log rotates at its bound and never throws",()=>{
 const root=directory("log"),made=unitKeyComponentsV1({toolchain:"t",normalized:["--crate-name","x"],sourceSha256:"a".repeat(64),externs:[],environment:[]});
 for(let index=0;index<5;index++)recordLookupV1(root,{lane:"d",crate:"x"+index,key:"a".repeat(64),outcome:"miss",components:made.components});
 expect(readLookupsV1(root).map(row=>row.crate)).toEqual(["x0","x1","x2","x3","x4"]);
 expect(readLookupsV1(root,Date.now()+60000)).toEqual([]);
 expect(()=>recordLookupV1("/nonexistent/\0bad",{lane:"d",crate:"x",key:"a".repeat(64),outcome:"miss",components:made.components})).not.toThrow();
});

test("evictions are counted by kind and shown by stats beside the not-shared reasons",async()=>{
 const root=directory("evict");
 expect(readEvictionsV1(root)).toEqual({records:0,totals:{},firstMs:null,lastMs:null});
 recordEvictionV1(root,{lanes:0});expect(readEvictionsV1(root).records,"all-zero counts are not written").toBe(0);
 recordEvictionV1(root,{lanes:2,superseded:5});recordEvictionV1(root,{lanes:1,entries:4});
 const counted=readEvictionsV1(root);expect(counted.totals).toEqual({lanes:3,superseded:5,entries:4});expect(counted.records).toBe(2);
 const lane=enterLaneV1(root,"gone1");lane.release();
 const before=readEvictionsV1(root).totals.lanes??0;
 applyGcV1(root,{removeLanes:listLanesV1(root,Date.now()).map(row=>row.id),removeIncremental:[]} as never,Date.now()+86400000);
 expect(readEvictionsV1(root).totals.lanes??0,"a collection that removed a lane records it").toBe(before+1);
 const stats=await manageV1(["stats","--root",root]) as {evictions:ReturnType<typeof readEvictionsV1>;notShared:Record<string,number>;lookups:Record<string,number>};
 expect(stats.evictions.totals.lanes).toBe(before+1);expect(stats.notShared).toEqual({});
 emitUnitEventV1(root,{lane:"d",kind:"bypass",crate:"x",key:null,milliseconds:1,waitedMs:0,reason:"native-link"});emitUnitEventV1(root,{lane:"d",kind:"remote-skip",crate:"y",key:"a".repeat(64),milliseconds:0,waitedMs:0,reason:"path-dependent: the compiled bytes hold /x"});
 expect((await manageV1(["stats","--root",root]) as typeof stats).notShared).toEqual({"bypass: native-link":1,"remote-skip: path-dependent":1});
});

function crate(root:string,name:string,manifest:string,source:string,extra:Record<string,string>={}){const dir=join(root,name);mkdirSync(join(dir,"src"),{recursive:true});writeFileSync(join(dir,"Cargo.toml"),manifest);writeFileSync(join(dir,"src",source.includes("fn main()")?"main.rs":"lib.rs"),source);for(const [file,text] of Object.entries(extra))writeFileSync(join(dir,file),text);}
function workspace(root:string,features:{leaf1:string[];leaf2:string[]}):void{
 mkdirSync(root,{recursive:true});
 writeFileSync(join(root,"Cargo.toml"),`cargo-features = ["trim-paths"]\n[workspace]\nresolver = "2"\nmembers = ["core","mac","leaf1","leaf2","app"]\n[profile.dev]\ndebug = 0\ntrim-paths = "object"\n`);
 const manifest=(name:string,dependencies:string,extra="")=>`[package]\nname = "${name}"\nversion = "0.1.0"\nedition = "2021"\n${extra}[features]\nfa = []\nfb = []\n[dependencies]\n${dependencies}`,list=(names:string[])=>"["+names.map(name=>`"${name}"`).join(",")+"]";
 crate(root,"core",manifest("core",""),"pub fn id() -> u32 { 1 }\n");
 crate(root,"mac",manifest("mac","","[lib]\nproc-macro = true\n"),"use proc_macro::TokenStream;\n#[proc_macro]\npub fn five(_: TokenStream) -> TokenStream { \"5\".parse().unwrap() }\n");
 crate(root,"leaf1",manifest("leaf1",`core = { path = "../core", features = ${list(features.leaf1)} }\nmac = { path = "../mac" }\n`,"build = \"build.rs\"\n"),"pub fn f() -> u32 { core::id() + mac::five!() }\n",{"build.rs":"fn main() { println!(\"cargo:rustc-env=X=1\"); }\n"});
 crate(root,"leaf2",manifest("leaf2",`core = { path = "../core", features = ${list(features.leaf2)} }\nmac = { path = "../mac" }\n`),"pub fn f() -> u32 { core::id() + mac::five!() }\n");
 crate(root,"app",manifest("app","leaf1 = { path = \"../leaf1\" }\nleaf2 = { path = \"../leaf2\" }\n"),"fn main() { println!(\"{}\", leaf1::f() + leaf2::f()); }\n");
}
type Run=Readonly<{lane:string;checkout:string;arguments:readonly string[];targetDirectory?:string;terminalColumns?:number}>;
function build(cache:string,cas:string,run:Run):void{
 const laneRoot=join(cache,"l",run.lane);mkdirSync(laneRoot,{recursive:true});
 const env:Record<string,string|undefined>={...process.env,RUSTC_WRAPPER:process.execPath,RUSTC:script,SEMIO_UNIT_CACHE_ROOT:cache,SEMIO_UNIT_CACHE_LANE:run.lane,SEMIO_UNIT_CACHE_LANE_ROOT:laneRoot,SEMIO_UNIT_CACHE_SLOTS:"4",SEMIO_UNIT_CACHE_TOOLCHAIN:"test-toolchain",SEMIO_UNIT_CACHE_INNER:JSON.stringify(["rustc"]),SEMIO_CAS_ROOT:cas,CARGO_TARGET_DIR:run.targetDirectory??join(laneRoot,"t"),CARGO_TERM_COLOR:"never",CARGO_INCREMENTAL:"0",RUSTUP_TOOLCHAIN:"nightly-2026-07-20"};
 for(const key of Object.keys(env))if(key.startsWith("SEMIO_REMOTE_CACHE_")||key.startsWith("SEMIO_FLEET_")||key==="SEMIO_WORKSPACE_ROOT"||key==="SEMIO_UNIT_CACHE_WORKSPACE_ROOT"||["CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS","RUSTC_WORKSPACE_WRAPPER","CARGO_BUILD_RUSTC_WRAPPER"].includes(key))delete env[key];
 const command=[cargo,"build","--manifest-path",join(run.checkout,"Cargo.toml"),...run.arguments];
 const tty=run.terminalColumns!==undefined&&(process.platform==="darwin"||process.platform==="linux");
 const shell="stty cols "+run.terminalColumns+"; exec "+command.map(part=>JSON.stringify(part)).join(" ");
 const result=tty?(process.platform==="darwin"?spawnSync("script",["-q","/dev/null","sh","-c",shell],{cwd:run.checkout,env:env as NodeJS.ProcessEnv,encoding:"utf8"}):spawnSync("script",["-qec",shell,"/dev/null"],{cwd:run.checkout,env:env as NodeJS.ProcessEnv,encoding:"utf8"})):spawnSync(command[0]!,command.slice(1),{cwd:run.checkout,env:env as NodeJS.ProcessEnv,encoding:"utf8"});
 if(result.status!==0)throw Error("cargo failed in lane "+run.lane+": "+(result.stderr||result.stdout).slice(-600));
}

test("the same crate built through different member selections, terminals, target directories and checkouts is compiled once",async()=>{
 const root=directory("same"),cache=join(root,"cache"),cas=join(root,"cas"),one=join(root,"a","ws"),two=join(root,"deeper","b","ws");
 workspace(one,{leaf1:[],leaf2:[]});workspace(two,{leaf1:[],leaf2:[]});mkdirSync(cache,{recursive:true});
 const runs:Run[]=[{lane:"l1",checkout:one,arguments:["-p","leaf1"]},{lane:"l2",checkout:one,arguments:["-p","leaf2"]},{lane:"l3",checkout:one,arguments:["--workspace"]},{lane:"l4",checkout:one,arguments:["-p","app"],targetDirectory:join(root,"odd","target")},{lane:"l5",checkout:one,arguments:["-p","leaf1"],terminalColumns:97},{lane:"l6",checkout:one,arguments:["-p","leaf2"],terminalColumns:211}];
 for(const run of runs)build(cache,cas,run);
 const rows=readLookupsV1(cache),report=churnReportV1(rows,200);
 const keys=new Map<string,Set<string>>();for(const row of rows)keys.set(row.crate,(keys.get(row.crate)??new Set()).add(row.key));
 for(const name of["core","mac","leaf1","leaf2","app"])expect([name,keys.get(name)?.size],"one key per crate").toEqual([name,1]);
 expect(report.multiKey,JSON.stringify(report.perCrate)).toBe(0);
 for(const name of["core","mac","leaf1","leaf2"])expect(rows.filter(row=>row.crate===name&&row.outcome==="miss").length,name+" compiled once").toBe(1);
 expect(rows.filter(row=>row.outcome==="miss").length).toBe(new Set(rows.filter(row=>row.outcome==="miss").map(row=>row.key)).size);
},300000);

test("members that ask for different features are different units and the report says which component differs",async()=>{
 const root=directory("features"),cache=join(root,"cache"),cas=join(root,"cas"),checkout=join(root,"ws");
 workspace(checkout,{leaf1:["fa"],leaf2:["fb"]});mkdirSync(cache,{recursive:true});
 build(cache,cas,{lane:"l1",checkout,arguments:["-p","leaf1"]});build(cache,cas,{lane:"l2",checkout,arguments:["-p","leaf2"]});
 const report=await manageV1(["churn","--root",cache]) as ReturnType<typeof churnReportV1>;
 const core=report.perCrate.find(row=>row.crate==="core")!;
 expect(core.keys).toBe(2);expect(core.differs).toEqual(["args.features","args.metadata"]);
 expect(core.samples["args.features"]!.sort()).toEqual(['--cfg=feature="fa"','--cfg=feature="fb"']);
 expect(report.causes.some(cause=>cause.components.join()==="args.features,args.metadata"&&cause.examples.includes("core"))).toBe(true);
 expect(report.perCrate.find(row=>row.crate==="mac"),"the proc-macro is not split").toBeUndefined();
 const only=await manageV1(["churn","--root",cache,"--crate","core"]) as ReturnType<typeof churnReportV1>;
 expect(only.lookups).toBeLessThan(report.lookups);
},300000);
