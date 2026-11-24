import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {keyEnvironmentV1,neutralRustflagsV1,normalizeRustcArgumentsV1,sdkIdentityV1,sha256V1,unitKeyV1,UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import {machineNeutralRemapV1} from "../../🧭️remap/🟦️.ts";
import {localKeyV1,localTextV1,portableKeyV1,portableRootsV1,portableTextV1,remoteKeyV1,toolRootsV1,type PlatformV1} from "../../🛰️fleet/🪪️identity/🟦️.ts";
import {buildRunKeyInputV1,buildScriptIdentityV1,markBuildScriptIdentityV1} from "../../🏗️buildscript/🔑️run-key/🟦️.ts";
import {packRunV1,unpackRunV1,runFileNameV1,runFilePathV1} from "../../🏗️buildscript/📦️run-wire/🟦️.ts";
import {compilerWrittenV1,keyGapV1,unitSourcesV1} from "../../🔏️key/🔗️sources/🟦️.ts";
import {manifestFilesV1,manifestOfV1,observedManifestV1,readManifestV1,writeManifestV1} from "../../🛰️fleet/📋️manifest/🟦️.ts";
import type {BuildEntryV1} from "../../💾️store/🟦️.ts";
import {buildKeyEnvironmentV1} from "../../🔏️key/🟦️.ts";
import {countSkipV1,countUnitV1,readBuildCountersV1,readSkipReasonsV1,skipNameV1,writeBuildCountersV1} from "../../🛰️fleet/📊️counters/🟦️.ts";
import {unsharedReasonV1,type FleetUnitV1} from "../../🛰️fleet/🟦️.ts";
import {churnReportV1,traceDifferencesV1,type UnitLookupV1} from "../../💾️store/🟦️.ts";
import {readRemoteExecutionCountersV1} from "../../../../../🏗️build/🌐️remote-execution/🌡️load/🟦️.ts";

/** 🧳️ Key portability: one real unit, compiled on two simulated machines with other home, checkout, target and SDK locations, gives one fleet key; every normalised argument has a table row; every unit compiled without sharing is counted under a reason name that adds up to the skipped count. */
const scratch:string[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),"pt-"+label+"-")));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const platform=fixture.platform as PlatformV1;

type Machine=Readonly<{root:string;home:string;sdk:string}>;
function keyOn(machine:Machine,mutate:(args:string[])=>string[]=args=>args):{remote:string;unit:string;normalized:string[];environment:Array<[string,string]>}{
 const lane=machine.root+"/.cache/lane-"+sha256V1(machine.home).slice(0,6)+"/t-"+sha256V1(machine.root).slice(0,8);
 const raw=mutate(fixture.arguments.map(argument=>argument.replaceAll("⟨cargo-build-dir⟩",lane).replaceAll("⟨rust-src⟩",machine.home+"/.rustup/toolchains/nightly/lib/rustlib/src/rust").replaceAll(fixture.root,machine.root)));
 const environment=keyEnvironmentV1({...Object.fromEntries(fixture.environment.map(([name,value])=>[name,value!.replaceAll(fixture.root,machine.root)])),SDKROOT:machine.sdk,PWD:machine.root,HOME:machine.home,CARGO_HOME:machine.home+"/.cargo",TMPDIR:machine.home+"/tmp"},undefined);
 const normalized=normalizeRustcArgumentsV1(raw,new Map(),undefined),input={toolchain:fixture.toolchain,normalized,sourceSha256:fixture.sourceSha256,externs:[],environment};
 const roots=portableRootsV1({cwd:machine.root,workspace:machine.root,hints:[fixture.manifest.replaceAll(fixture.root,machine.root),machine.root]});
 return {remote:remoteKeyV1(platform,input,roots),unit:unitKeyV1(input),normalized,environment};
}
function sdkAt(label:string,layout:readonly string[]):string{
 const base=directory(label),real=join(base,...layout.slice(0,-1),"MacOSX15.4.sdk");mkdirSync(real,{recursive:true});
 const link=join(base,...layout.slice(0,-1),layout.at(-1)!);if(link!==real)symlinkSync(real,link);return link;
}
const machineA=():Machine=>({root:directory("a")+"/semio",home:"/Users/ueli",sdk:sdkAt("sa",["Applications","Xcode.app","Contents","Developer","SDKs","MacOSX.sdk"])});
const machineB=():Machine=>({root:directory("b")+"/work/semio-checkout",home:"/home/deck",sdk:sdkAt("sb",["Library","Developer","CommandLineTools","SDKs","MacOSX15.4.sdk"])});

test("one real unit gives the same fleet key on two machines with other home, checkout, target directory and SDK locations",()=>{
 const one=keyOn(machineA()),two=keyOn(machineB());
 expect(one.remote).toBe(two.remote);expect(one.unit,"the raw local key is machine-bound, the fleet key is not").not.toBe(two.unit);
 expect(JSON.stringify(one.normalized)).not.toContain("/Users/ueli/.rustup");
 expect(JSON.stringify(two.normalized)).not.toContain("/home/deck");
});

test("what is an input still moves the key: the profile, a feature, the compiler flags and the SDK itself",()=>{
 const a=machineA(),base=keyOn(a).remote;
 expect(keyOn(a,args=>args.map(argument=>argument==="opt-level=3"?"opt-level=0":argument)).remote).not.toBe(base);
 expect(keyOn(a,args=>[...args,"--cfg",'feature="x"']).remote).not.toBe(base);
 const other=directory("sdk-other"),older=join(other,"MacOSX14.5.sdk");mkdirSync(older);
 expect(keyOn({...a,sdk:older}).remote).not.toBe(base);
});

test("the key equals an independent recomputation by python's hashlib over the same inputs",()=>{
 const key=keyOn(machineB()),run=spawnSync("python3",["-I","-c","import hashlib,json,sys;d=json.load(sys.stdin);print(hashlib.sha256(json.dumps([d['namespace'],d['toolchain'],d['normalized'],d['source'],[],d['environment']],separators=(',',':'),ensure_ascii=False).encode()).hexdigest())"],{input:JSON.stringify({namespace:UNIT_CACHE_POLICY_V1.namespace,toolchain:fixture.toolchain,normalized:key.normalized,source:fixture.sourceSha256,environment:key.environment}),encoding:"utf8"});
 expect(run.status).toBe(0);expect(run.stdout.trim()).toBe(key.unit);
});

const REGISTRY="⟨cargo-registry-src⟩",GIT="⟨cargo-git-checkouts⟩",RUST="⟨rust-src⟩",BUILD="⟨cargo-build-dir⟩";
const remapRows:ReadonlyArray<readonly [string,string,string]>=[
 ["registry source cache on macOS","--remap-path-prefix=/Users/ueli/.cargo/registry/src=","--remap-path-prefix="+REGISTRY+"="],
 ["registry source cache on Linux","--remap-path-prefix=/home/deck/.cargo/registry/src=","--remap-path-prefix="+REGISTRY+"="],
 ["registry source cache on Windows","--remap-path-prefix=C:\\Users\\deck\\.cargo\\registry\\src=","--remap-path-prefix="+REGISTRY+"="],
 ["git checkouts","--remap-path-prefix=/Users/ueli/.cargo/git/checkouts=/cargo/git","--remap-path-prefix="+GIT+"=/cargo/git"],
 ["the compiler's source copy","--remap-path-prefix=/Users/ueli/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust=/rustc/abc","--remap-path-prefix="+RUST+"=/rustc/abc"],
 ["Cargo's build directory in a lane","--remap-path-prefix=/Users/ueli/semio/.cache/b/r/m/u/lane/t-1234=/cargo/build-dir","--remap-path-prefix="+BUILD+"=/cargo/build-dir"],
 ["Cargo's build directory elsewhere","--remap-path-prefix=/tmp/target=/cargo/build-dir","--remap-path-prefix="+BUILD+"=/cargo/build-dir"],
 ["a remapping of an unknown root stays as it is","--remap-path-prefix=/opt/vendor=/vendor","--remap-path-prefix=/opt/vendor=/vendor"],
 ["a flag that is no remapping stays as it is","--cfg=feature=\"x\"","--cfg=feature=\"x\""],
];
test.each(remapRows)("normalised argument: %s",(_,given,expected)=>{expect(machineNeutralRemapV1(given)).toBe(expected);});

const flagRows:ReadonlyArray<readonly [string,string,string]>=[
 ["joined remapping in the encoded flags","-Zthreads=0\u001f--remap-path-prefix=/home/deck/.cargo/registry/src=","-Zthreads=0\u001f--remap-path-prefix="+REGISTRY+"="],
 ["two-argument remapping in the encoded flags","--remap-path-prefix\u001f/Users/ueli/.cargo/git/checkouts=/g","--remap-path-prefix\u001f"+GIT+"=/g"],
 ["flags without a path stay as they are","-Z\u001fthreads=0","-Z\u001fthreads=0"],
];
test.each(flagRows)("normalised environment value: %s",(_,given,expected)=>{expect(neutralRustflagsV1(given)).toBe(expected);});

test("an SDK is its resolved name, wherever it is installed, and the key environment carries that name",()=>{
 const xcode=sdkAt("x",["Applications","Xcode.app","Contents","Developer","Platforms","MacOSX.platform","Developer","SDKs","MacOSX.sdk"]),tools=sdkAt("t",["Library","Developer","CommandLineTools","SDKs","MacOSX15.4.sdk"]);
 expect(sdkIdentityV1(xcode)).toBe("macosx15.4");expect(sdkIdentityV1(tools)).toBe("macosx15.4");
 expect(sdkIdentityV1("/missing/MacOSX99.sdk/")).toBe("macosx99");
 expect(keyEnvironmentV1({SDKROOT:xcode},undefined)).toEqual(keyEnvironmentV1({SDKROOT:tools},undefined));
 expect(keyEnvironmentV1({SDKROOT:xcode,HOME:"/h",PWD:"/p",TMPDIR:"/t"},undefined).map(([name])=>name),"HOME, PWD and TMPDIR never enter the key").toEqual(["SDKROOT"]);
});

test("an SDK is its declared version: Xcode's unversioned MacOSX.sdk behind a versioned link equals the same SDK of the command line tools and differs from another version",()=>{
 const settings=(dir:string,name:string)=>writeFileSync(join(dir,"SDKSettings.json"),JSON.stringify({CanonicalName:name,Version:name.slice(6)}));
 const xcodeDirectory=directory("xs"),xcodeSdks=join(xcodeDirectory,"Xcode.app","SDKs"),unversioned=join(xcodeSdks,"MacOSX.sdk");mkdirSync(unversioned,{recursive:true});settings(unversioned,"macosx27.2");
 const link=join(xcodeSdks,"MacOSX27.2.sdk");symlinkSync(unversioned,link);
 const toolsDirectory=directory("ts"),toolsSdk=join(toolsDirectory,"CommandLineTools","SDKs","MacOSX27.2.sdk");mkdirSync(toolsSdk,{recursive:true});settings(toolsSdk,"macosx27.2");
 const toolsLink=join(toolsDirectory,"CommandLineTools","SDKs","MacOSX.sdk");symlinkSync(toolsSdk,toolsLink);
 const olderDirectory=directory("os"),older=join(olderDirectory,"MacOSX.sdk");mkdirSync(older,{recursive:true});settings(older,"macosx26.5");
 for(const path of [unversioned,link,toolsSdk,toolsLink])expect(sdkIdentityV1(path)).toBe("macosx27.2");
 expect(sdkIdentityV1(older)).toBe("macosx26.5");
 expect(keyEnvironmentV1({SDKROOT:link},undefined)).toEqual(keyEnvironmentV1({SDKROOT:toolsLink},undefined));
 expect(keyEnvironmentV1({SDKROOT:link},undefined)).not.toEqual(keyEnvironmentV1({SDKROOT:older},undefined));
});

test.each([["breaker: denied","breaker"],["incomplete: a.rlib,b.rmeta","incomplete"],["no-artifact-messages","no-artifact-messages"],["Native Link!","native-link"],["","other"],["crate-type:cdylib","crate-type"]])("skip reason name of %p is %p",(text,name)=>{expect(skipNameV1(text)).toBe(name);});

test("skipped units are counted under their reason, the reasons add up to the skipped count, and they reach the fleet's closed counter shape",()=>{
 const workspace=directory("skip");
 for(const reason of["breaker-denied","breaker-denied","breaker-down","read-only","path-dependent","upload-dropped","upload-dropped","Not A Reason"])countSkipV1(workspace,reason);
 countSkipV1(undefined,"read-only");
 expect(readSkipReasonsV1(workspace)).toEqual({"breaker-denied":2,"breaker-down":1,"read-only":1,"path-dependent":1,"upload-dropped":2,other:1});
 const counters=readBuildCountersV1(workspace);
 expect(counters.unit.skipped).toBe(8);expect(Object.values(counters.skipReasons).reduce((sum,count)=>sum+count,0)).toBe(counters.unit.skipped);
 writeBuildCountersV1(workspace);
 const reported=readRemoteExecutionCountersV1(workspace);
 expect(reported.unit.skipped).toBe(8);expect(reported.skipReasons).toEqual(counters.skipReasons);
});

test("a unit that was compiled but not shared says why: read-only or a full spool; the breaker is no reason, the upload waits",()=>{
 const unit=(upload:boolean)=>({client:{settings:{upload}}} as unknown as FleetUnitV1);
 expect(unsharedReasonV1(unit(false))).toBe("read-only");
 expect(unsharedReasonV1(unit(true))).toBe("spool-full");
});

test("the skipped count is the sum of the reason files, so no skip is unrecorded; the sum equals an independent python reading of the files",()=>{
 const workspace=mkdtempSync(join(tmpdir(),"skip-unrecorded-"));
 try{
  for(const reason of["read-only","read-only","spool-full","unreadable-source","publish-error"])countSkipV1(workspace,reason);
  const counters=readBuildCountersV1(workspace);
  expect(counters.unit.skipped).toBe(5);expect(counters.skipReasons).toEqual({"read-only":2,"spool-full":1,"unreadable-source":1,"publish-error":1});expect("unrecorded" in counters.skipReasons).toBe(false);
  const python=spawnSync("python3",["-c","import os,sys\nd=os.path.join(sys.argv[1],'.cache','f','unit-counters','skip')\nprint(sum(os.path.getsize(os.path.join(d,n)) for n in os.listdir(d)))",workspace],{encoding:"utf8"});
  expect(python.status).toBe(0);expect(Number(python.stdout)).toBe(counters.unit.skipped);
 }finally{rmSync(workspace,{recursive:true,force:true});}
});

test("the traced keys of one crate name the argument and the environment entry that differ between them",()=>{
 const row=(remote:string,args:string[],environment:Array<[string,string]>,platform?:string):UnitLookupV1=>({version:1,atMs:1,lane:"d",pid:1,crate:"dep",key:remote,remote,outcome:"miss",components:{toolchain:"a",source:remote.slice(0,16).padEnd(16,"0"),"args.codegen":remote.slice(0,16).padEnd(16,"0")},trace:{args,environment,...platform===undefined?{}:{platform}}});
 const rows=[row("1".repeat(64),["--crate-name","dep","-C","metadata=aa"],[["SDKROOT","⟨sdk⟩MacOSX15.4.sdk"]],"p1"),row("2".repeat(64),["--crate-name","dep","-C","metadata=bb"],[["SDKROOT","⟨sdk⟩MacOSX14.5.sdk"]],"p2")];
 const difference=traceDifferencesV1(rows);
 expect(difference.keys).toBe(2);
 expect(difference.arguments).toEqual([{text:"metadata=aa",in:1},{text:"metadata=bb",in:1}]);
 expect(difference.environment).toEqual([{text:"SDKROOT=⟨sdk⟩MacOSX14.5.sdk",in:1},{text:"SDKROOT=⟨sdk⟩MacOSX15.4.sdk",in:1}]);
 expect(difference.platform).toEqual(["p1","p2"]);
 const report=churnReportV1(rows);
 expect(report.perCrate[0]!.trace?.arguments).toHaveLength(2);
 expect(churnReportV1(rows.map(({trace:_,...rest})=>rest)).perCrate[0]!.trace,"without a trace there is nothing to show").toBeUndefined();
 expect(traceDifferencesV1([rows[0]!,rows[0]!])).toEqual({keys:2,arguments:[],environment:[],platform:["p1"]});
});

test("the rendering of the compiler's messages (colour, width, ansi or short text) is no part of a unit's key, the kind of messages is",()=>{
 const base=["--crate-name","a","--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--color=always","--diagnostic-width=120","src/lib.rs"];
 const normalize=(args:readonly string[])=>normalizeRustcArgumentsV1(args,new Map(),undefined);
 const plain=["--crate-name","a","--error-format=json","--json","artifacts,future-incompat","--color","never","src/lib.rs"],short=["--crate-name","a","--error-format=json","--json=diagnostic-short,artifacts,future-incompat","src/lib.rs"];
 expect(normalize(base)).toEqual(normalize(plain));expect(normalize(base)).toEqual(normalize(short));
 expect(normalize(base)).not.toEqual(normalize(["--crate-name","a","--error-format=json","--json=diagnostic-rendered-ansi,artifacts","src/lib.rs"]));
});

type Home=Readonly<{root:string;home:string;cargo:string;rustup:string;tools:string}>;
const homeA=():Home=>{const root=directory("ha")+"/Users/ueli/Documents/semio";return{root,home:"/Users/ueli",cargo:root+"/.cache/b/r/m/cargo/home",rustup:"/Users/ueli/.rustup",tools:"/opt/a"};};
const homeB=():Home=>({root:directory("hb")+"/Users/Shared/semio",home:"/home/deck",cargo:"/home/deck/.cargo",rustup:"/srv/rust/rustup",tools:"/opt/b"});
const homeEnvironment=(machine:Home)=>({HOME:machine.home,CARGO_HOME:machine.cargo,RUSTUP_HOME:machine.rustup});

function registryKey(machine:Home,mutate:(roots:ReturnType<typeof portableRootsV1>)=>ReturnType<typeof portableRootsV1>=roots=>roots){
 const directoryOfCrate=machine.cargo+"/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.229",lane=machine.root+"/.cache/lane-x/t-"+sha256V1(machine.root).slice(0,8);
 const raw=["--crate-name","serde","--edition=2021","src/lib.rs","--crate-type","lib","--emit=dep-info,metadata,link","-C","metadata=0a1b2c","--cfg",'feature="std"',"--out-dir",lane+"/deps","--remap-path-prefix="+machine.cargo+"/registry/src=","--sysroot",machine.rustup+"/toolchains/nightly-aarch64-apple-darwin","-C","link-arg=-L"+machine.cargo+"/lib","-L","dependency="+lane+"/deps"];
 const environment=keyEnvironmentV1({CARGO_MANIFEST_DIR:directoryOfCrate,CARGO_PKG_NAME:"serde",CARGO_PKG_VERSION:"1.0.229",CARGO_CRATE_NAME:"serde",CARGO_ENCODED_RUSTFLAGS:"--remap-path-prefix="+machine.cargo+"/registry/src=\u001f-Zthreads=0",...homeEnvironment(machine)},undefined);
 const normalized=normalizeRustcArgumentsV1(raw,new Map(),undefined),input={toolchain:fixture.toolchain,normalized,sourceSha256:sha256V1("serde source"),externs:[],environment};
 const roots=mutate(portableRootsV1({cwd:directoryOfCrate,workspace:machine.root,hints:[directoryOfCrate],...toolRootsV1(homeEnvironment(machine))}));
 return{remote:remoteKeyV1(platform,input,roots),normalized,environment};
}

function workspaceKey(machine:Home,tools:boolean){
 const manifest=machine.root+"/🧰️framework/crates/demo",lane=machine.root+"/.cache/lane-y/t-"+sha256V1(machine.root).slice(0,8);
 const raw=["--crate-name","demo","--edition=2024","🧰️framework/crates/demo/src/lib.rs","--crate-type","lib","-C","metadata=ff00ee","--out-dir",lane+"/deps","-C","link-arg=-L"+machine.cargo+"/lib","--sysroot",machine.rustup+"/toolchains/nightly-aarch64-apple-darwin","--extern","dep=#abc"];
 const environment=keyEnvironmentV1({CARGO_MANIFEST_DIR:manifest,CARGO_PKG_NAME:"demo",CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:"demo",...homeEnvironment(machine)},undefined);
 const input={toolchain:fixture.toolchain,normalized:normalizeRustcArgumentsV1(raw.slice(0,-2),new Map(),undefined),sourceSha256:sha256V1("demo source"),externs:[],environment};
 const roots=portableRootsV1({cwd:machine.root,workspace:machine.root,hints:[manifest,machine.root],...tools?toolRootsV1(homeEnvironment(machine)):{}});
 return remoteKeyV1(platform,input,roots);
}

test("a registry crate and a workspace crate give one fleet key on two machines with other home, Cargo home, rustup home and checkout root (one machine keeps its Cargo home inside the checkout, the other checks out below Users/Shared)",()=>{
 const a=homeA(),b=homeB();
 expect(registryKey(a).remote).toBe(registryKey(b).remote);
 expect(workspaceKey(a,true)).toBe(workspaceKey(b,true));
 expect(workspaceKey(a,false),"without the tool roots the Cargo home and the rustup home split the key").not.toBe(workspaceKey(b,false));
 expect(registryKey(a,roots=>roots.filter(root=>!root.token.includes("cargo-home"))).remote,"without the Cargo home root the registry crate splits").not.toBe(registryKey(b,roots=>roots.filter(root=>!root.token.includes("cargo-home"))).remote);
});

test("what is an input still moves the key of a registry crate: its version, and a toolchain at another place in the rustup home",()=>{
 const a=homeA(),base=registryKey(a).remote;
 expect(registryKey({...a,rustup:"/Users/ueli/.rustup"}).remote).toBe(base);
 expect(registryKey(a,roots=>roots.filter(root=>root.token!=="@@semio-rustup-home@@")).remote).not.toBe(base);
});

test("the portable key of the registry crate equals an independent recomputation by python's hashlib over its tokenised inputs",()=>{
 const a=homeA(),b=homeB(),roots=(machine:Home)=>portableRootsV1({cwd:machine.cargo+"/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.229",workspace:machine.root,...toolRootsV1(homeEnvironment(machine))});
 const compute=(machine:Home)=>{const key=registryKey(machine),text=(value:string)=>portableTextV1(value,roots(machine));const input={toolchain:fixture.toolchain,normalized:key.normalized,sourceSha256:sha256V1("serde source"),externs:[],environment:key.environment};return{ours:portableKeyV1(input,roots(machine)),normalized:key.normalized.map(text),environment:key.environment.map(([name,value])=>[name,text(value)])};};
 const one=compute(a),two=compute(b);expect(one.ours).toBe(two.ours);
 const run=spawnSync("python3",["-I","-c","import hashlib,json,sys;d=json.load(sys.stdin);print(hashlib.sha256(json.dumps([d['namespace'],d['toolchain'],d['normalized'],d['source'],[],d['environment']],separators=(',',':'),ensure_ascii=False).encode()).hexdigest())"],{input:JSON.stringify({namespace:UNIT_CACHE_POLICY_V1.namespace,toolchain:fixture.toolchain,normalized:two.normalized,source:sha256V1("serde source"),environment:two.environment}),encoding:"utf8"});
 expect(run.status).toBe(0);expect(run.stdout.trim()).toBe(two.ours);
});

const runEnvironment=(machine:Home,directory:string)=>({CARGO_PKG_NAME:"ring",CARGO_PKG_VERSION:"0.17.0",CARGO_MANIFEST_DIR:directory,TARGET:"aarch64-apple-darwin",HOST:"aarch64-apple-darwin",PROFILE:"release",RUSTC:machine.rustup+"/toolchains/nightly/bin/rustc",RUSTC_WRAPPER:machine.root+"/bin/semio-unit-cache",RUSTC_LINKER:machine.tools+"/bin/cc",CARGO_ENCODED_RUSTFLAGS:"--remap-path-prefix="+machine.cargo+"/registry/src=\u001f-Zthreads=0",DEP_Z_ROOT:directory+"/out",CARGO_FEATURE_STD:"1",NUM_JOBS:"8",...homeEnvironment(machine)});

function runKey(machine:Home,binary:string){
 const directory=machine.cargo+"/registry/src/index.crates.io-1949cf8c6b5b557f/ring-0.17.0",environment=runEnvironment(machine,directory);
 const roots=portableRootsV1({cwd:directory,workspace:machine.root,hints:[directory],...toolRootsV1(environment)}),input=buildRunKeyInputV1({toolchain:fixture.toolchain,identity:binary,args:[],environment,laneRoot:undefined});
 return{local:localKeyV1(input,roots),remote:remoteKeyV1(platform,input,roots),input,tokenised:portableTextV1(JSON.stringify(input.environment),roots)};
}

test("a build script run gives one key on two machines with other compiler, wrapper and linker paths, flags remapped to other homes and Cargo homes, other dependency output directories and other checkout roots; NUM_JOBS and the home directories are no inputs",()=>{
 const a=runKey(homeA(),"unit:"+"1".repeat(64)),b=runKey(homeB(),"unit:"+"1".repeat(64));
 expect(a.local).toBe(b.local);expect(a.remote).toBe(b.remote);
 expect(a.tokenised).not.toContain("/Users/ueli");expect(a.tokenised).toContain("@@semio-cwd@@");
 expect(b.tokenised).not.toContain("/home/deck");expect(b.tokenised).toBe(a.tokenised);
 expect(a.input.environment.map(([name])=>name)).not.toContain("NUM_JOBS");
 expect(runKey(homeA(),"unit:"+"2".repeat(64)).remote,"another executable is another run").not.toBe(a.remote);
 const changed=buildRunKeyInputV1({toolchain:fixture.toolchain,identity:"unit:"+"1".repeat(64),args:[],environment:{...runEnvironment(homeA(),"/x/ring"),CARGO_FEATURE_STD:"0"},laneRoot:undefined});
 expect(changed.environment).not.toEqual(a.input.environment);
 expect(buildKeyEnvironmentV1({RUSTC:"/a/b/rustc",RUSTC_WRAPPER:"C:\\tools\\wrap.exe"},undefined)).toEqual([["RUSTC","⟨tool⟩rustc"],["RUSTC_WRAPPER","⟨tool⟩wrap"]]);
});

test("a build script executable is keyed by the unit it was compiled from while its identity file names its bytes, else by its bytes",()=>{
 const base=directory("bin"),real=join(base,"build_script_build.real");writeFileSync(real,"binary one");
 const sha=sha256V1("binary one");
 expect(buildScriptIdentityV1(real,sha)).toBe("bytes:"+sha);
 markBuildScriptIdentityV1(real,"a".repeat(64),["/work"],sha);
 const named=buildScriptIdentityV1(real,sha);expect(named).toMatch(/^unit:[0-9a-f]{64}$/u);
 writeFileSync(real,"binary two, other bytes of the same unit on another machine");
 const other=sha256V1("binary two, other bytes of the same unit on another machine");
 expect(buildScriptIdentityV1(real,other),"an identity file never vouches for other bytes").toBe("bytes:"+other);
 markBuildScriptIdentityV1(real,"a".repeat(64),["/work"],other);expect(buildScriptIdentityV1(real,other)).toBe(named);
 markBuildScriptIdentityV1(real,"a".repeat(64),["/elsewhere"],other);expect(buildScriptIdentityV1(real,other),"a unit that embeds other paths is another identity").not.toBe(named);
});

function runEntry(machine:Home,blobs:Map<string,Uint8Array>,text:string):BuildEntryV1{
 const put=(value:string|Uint8Array)=>{const bytes=typeof value==="string"?Buffer.from(value):value,sha=sha256V1(bytes);blobs.set(sha,bytes);return sha;};
 const header=put("cfg"),generated=put("pub const NAME: &str = \"ring\";\n"+text),binary=put(new Uint8Array([0,1,2,3,255]));
 return{version:1,key:"k".repeat(64).replace(/k/gu,"1"),package:"ring",facts:[{path:"build.rs",kind:"file",sha256:header},{path:".",kind:"package",sha256:sha256V1("tree")}],environment:[{name:"CC",value:null}],outputs:[{path:"nested/dir/gen.rs",sha256:generated,bytes:10,executable:false,template:true},{path:"table.bin",sha256:binary,bytes:5,executable:false,template:false}],stdout:put("cargo:rerun-if-changed=build.rs\ncargo:rustc-link-search="+machine.cargo+"/registry/src/x/lib\n"),stderr:null,createdMs:1};
}

test("a build script run travels between two machines: roots become tokens on the way out and the other machine's roots on the way in; output names with slashes survive",()=>{
 const a=homeA(),b=homeB(),blobs=new Map<string,Uint8Array>(),entry=runEntry(a,blobs,"// from "+a.cargo+"/registry/src/y\n");
 const header={key:"2".repeat(64),localKey:"3".repeat(64),slot:"4".repeat(64),platform,costMs:7};
 const rootsA=portableRootsV1({cwd:a.root,workspace:a.root,...toolRootsV1(homeEnvironment(a))}),rootsB=portableRootsV1({cwd:b.root,workspace:b.root,...toolRootsV1(homeEnvironment(b))});
 const packed=packRunV1(entry,header,rootsA,sha=>blobs.get(sha),[a.home]);
 if(typeof packed==="string")throw Error(packed);
 expect(packed.unit.outputs.map(output=>output.name).sort()).toEqual(["f:nested%2Fdir%2Fgen.rs","f:table.bin","run.json"]);
 const wire=new Map(packed.contents),texts=[...wire.values()].map(bytes=>Buffer.from(bytes).toString("latin1")).join("\n");
 expect(texts).not.toContain(a.cargo);expect(texts).toContain("@@semio-cargo-home@@");
 const stored=new Map<string,Uint8Array>(),back=unpackRunV1({localKey:"5".repeat(64),roots:rootsB,manifest:packed.unit,blobs:wire,put:bytes=>{const sha=sha256V1(bytes);stored.set(sha,bytes);return sha;}});
 expect(back.outputs.map(output=>output.path).sort()).toEqual(["nested/dir/gen.rs","table.bin"]);
 const text=(sha:string|null)=>Buffer.from(stored.get(sha!)!).toString("utf8");
 expect(text(back.stdout)).toContain("cargo:rustc-link-search="+b.cargo+"/registry/src/x/lib");expect(text(back.stdout)).not.toContain(a.cargo);
 expect(text(back.outputs.find(output=>output.path==="nested/dir/gen.rs")!.sha256)).toContain(b.cargo+"/registry/src/y");
 expect([...stored.get(back.outputs.find(output=>output.path==="table.bin")!.sha256)!]).toEqual([0,1,2,3,255]);
 expect(back.facts.find(fact=>fact.kind==="package")!.sha256).toBe(sha256V1("tree"));expect(back.key).toBe("5".repeat(64));
 expect(runFilePathV1(runFileNameV1("a%2Fb/c%d"))).toBe("a%2Fb/c%d");
});

test("a build script run that still names a directory of its machine after its roots became tokens stays local, with its reason; so does one that does not fit a manifest",()=>{
 const a=homeA(),blobs=new Map<string,Uint8Array>(),rootsA=portableRootsV1({cwd:a.root,workspace:a.root,...toolRootsV1(homeEnvironment(a))}),header={key:"2".repeat(64),localKey:"3".repeat(64),slot:"4".repeat(64),platform,costMs:7},read=(sha:string)=>blobs.get(sha);
 const home=packRunV1(runEntry(a,blobs,"// "+a.home+"/.local/share\n"),header,rootsA,read,[a.home]);
 expect(typeof home==="string"&&home.startsWith("path-dependent")).toBe(true);
 const leaking=runEntry(a,blobs,"x"),binary=new Uint8Array([0,...Buffer.from(a.cargo+"/lib"),0]),sha=sha256V1(binary);blobs.set(sha,binary);
 expect(String(packRunV1({...leaking,outputs:[...leaking.outputs,{path:"lib.a",sha256:sha,bytes:binary.length,executable:false,template:false}]},header,rootsA,read))).toMatch(/^path-dependent: lib\.a holds/u);
 expect(packRunV1({...leaking,outputs:[{...leaking.outputs[0]!,path:"d/".repeat(120)+"x"}]},header,rootsA,read)).toBe("name-too-long");
 expect(packRunV1({...leaking,outputs:Array.from({length:70},(_,index)=>({...leaking.outputs[1]!,path:"f"+index}))},header,rootsA,read)).toBe("too-many-files");
});

test("the files a finished compilation read although the scan missed them are kept in the unit slot's input manifest and keyed by the next compilation; the observation file a proc-macro writes for the receipt is no input",()=>{
 const machine=homeA(),root=directory("gap"),base=directory("gap-tree"),entry=join(base,"lib.rs"),extra=join(base,"files","data.json"),roots=portableRootsV1({cwd:base,workspace:base});
 mkdirSync(join(base,"files"));writeFileSync(entry,"fn main(){}");writeFileSync(extra,"{\"a\":1}");
 const slot="6".repeat(64),digest=(path:string)=>path.endsWith("data.json")?sha256V1("one"):sha256V1("entry");
 const observation=base+"/semio-compiler-resources/1795376000000-4242-0/observation.json",dep="lib.rs: lib.rs files/data.json "+observation+"\n";
 const gap=keyGapV1({cwd:base,covered:new Set([entry]),keyedNames:new Set(),depInfo:dep});
 expect(gap).toEqual(["files/data.json"]);
 expect(compilerWrittenV1("/x/target/semio-compiler-resources/17-9-3/observation.json")).toBe(true);expect(compilerWrittenV1("/x/semio-compiler-resources/17-9/observation.json")).toBe(false);
 const id="7".repeat(64);expect(readManifestV1(root,id)).toBeUndefined();
 writeManifestV1(root,id,observedManifestV1(gap,{cwd:base,roots}),false);
 const recorded=manifestFilesV1(readManifestV1(root,id)!.manifest,roots);expect(recorded).toEqual([{name:"@@semio-cwd@@/files/data.json",path:extra}]);
 return unitSourcesV1(entry,undefined,undefined,digest,recorded).then(async keyed=>{
  const plain=await unitSourcesV1(entry,undefined,undefined,digest);
  expect(keyed!.sha256).not.toBe(plain!.sha256);expect(keyed!.rows.map(row=>row[0])).toContain("⟨gap⟩/@@semio-cwd@@/files/data.json");
  const other=await unitSourcesV1(entry,undefined,undefined,path=>path.endsWith("data.json")?sha256V1("two"):sha256V1("entry"),recorded);
  expect(other!.sha256,"a changed file moves the key").not.toBe(keyed!.sha256);
  expect(keyGapV1({cwd:base,covered:new Set(keyed!.files.map(file=>resolve(file))),keyedNames:new Set(),depInfo:dep}),"the recorded file is covered now").toEqual([]);
  const moved=directory("gap-moved");expect(manifestFilesV1(readManifestV1(root,id)!.manifest,portableRootsV1({cwd:moved,workspace:moved}))[0]!.path).toBe(join(moved,"files","data.json"));
 });
});
