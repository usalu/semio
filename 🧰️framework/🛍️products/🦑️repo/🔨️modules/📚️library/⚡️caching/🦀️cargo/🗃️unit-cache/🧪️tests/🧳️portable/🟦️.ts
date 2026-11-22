import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {keyEnvironmentV1,neutralRustflagsV1,normalizeRustcArgumentsV1,sdkIdentityV1,sha256V1,unitKeyV1,UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import {machineNeutralRemapV1} from "../../🧭️remap/🟦️.ts";
import {portableRootsV1,remoteKeyV1,type PlatformV1} from "../../🛰️fleet/🪪️identity/🟦️.ts";
import {accountedSkipsV1,countSkipV1,countUnitV1,readBuildCountersV1,readSkipReasonsV1,skipNameV1,writeBuildCountersV1} from "../../🛰️fleet/📊️counters/🟦️.ts";
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
 expect(sdkIdentityV1(xcode)).toBe("MacOSX15.4.sdk");expect(sdkIdentityV1(tools)).toBe("MacOSX15.4.sdk");
 expect(sdkIdentityV1("/missing/MacOSX99.sdk/")).toBe("MacOSX99.sdk");
 expect(keyEnvironmentV1({SDKROOT:xcode},undefined)).toEqual(keyEnvironmentV1({SDKROOT:tools},undefined));
 expect(keyEnvironmentV1({SDKROOT:xcode,HOME:"/h",PWD:"/p",TMPDIR:"/t"},undefined).map(([name])=>name),"HOME, PWD and TMPDIR never enter the key").toEqual(["SDKROOT"]);
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

test("the skip reasons always add up to the skipped count: units counted before their reason was recorded show as unrecorded",()=>{
 expect(accountedSkipsV1(10,{"read-only":4})).toEqual({"read-only":4,unrecorded:6});
 expect(accountedSkipsV1(4,{"read-only":4})).toEqual({"read-only":4});
 expect(accountedSkipsV1(0,{})).toEqual({});
 const workspace=mkdtempSync(join(tmpdir(),"skip-unrecorded-"));
 try{for(let count=0;count<3;count++)countUnitV1(workspace,"skipped");countSkipV1(workspace,"read-only");const counters=readBuildCountersV1(workspace);expect(counters.unit.skipped).toBe(4);expect(counters.skipReasons).toEqual({"read-only":1,unrecorded:3});}
 finally{rmSync(workspace,{recursive:true,force:true});}
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
