import {expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,readdirSync} from "node:fs";
import {dirname,isAbsolute,join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {dependencyClosureV1,walkFilesV1} from "../🌳️inputs/🟦️.ts";
import {climbAboveV1,includedSourcesV1,offloadRealTextV1,planOffloadV1,semioEnvironmentReadV1,type OffloadPlanV1} from "../🗺️paths/🟦️.ts";
import {parseDepInfoV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";
import {OFFLOAD_POLICY_V1,extractOffloadFlagV1,offloadLinksNativelyV1,offloadModeV1,offloadPinV1,offloadSettingV1,offloadTextV1} from "../📐️policy/🟦️.ts";
import {offloadRustcCommandV1,offloadPlatformV1,parseRustcIdentityV1} from "../📜️action/🟦️.ts";
import {placementIdleCountV1,RemoteExecutionLoadTableV1,type PlacementWorkerV1} from "../../🗓️scheduler/⚖️placement/🟦️.ts";
import type {RemoteExecutionLoadReportV1} from "../../🌡️load/🟦️.ts";
import {remoteExecutionMatchPropertiesV1,remoteExecutionPlatformMatchesV1} from "../../🟦️.ts";
import {createTestCheckoutV1,externHash,testHostProbeV1,testRustEnvironmentV1} from "./🧰️workspace/🟦️.ts";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {availableParallelism,tmpdir} from "node:os";
import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../../🌡️host-capacity/🎛️control/🟦️.ts";
import {fleetHeadroomV1,localUnitAdmissionV1,offloadFanOutJobsV1,readUnitCostV1,workspaceRootFromV1,worthARoundTripV1,writeUnitCostV1,readOffloadMemoV1,slotQueueMillisecondsV1,writeOffloadMemoV1} from "../🚦️admission/🟦️.ts";
import {declineOffloadV1,detectWorkerRustcV1} from "../🟦️.ts";
import {offloadSdkIdentityV1} from "../🧰️toolchain/🟦️.ts";
import {releaseImplicitTokenV1} from "../🎟️token/🟦️.ts";
import {bumpOffloadCountersV1,readOffloadCountersV1,readOffloadReasonCountersV1} from "../📊️counters/🟦️.ts";
import {tryClaimV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts";

const ajv=new Ajv({strict:false});ajv.addSchema(schema);

function chain(label:string){
 const checkout=createTestCheckoutV1(label);
 checkout.compile("a","pub fn a()->u32{1}\n");
 checkout.compile("b","pub fn b()->u32{a::a()+1}\n",["a"]);
 checkout.crate("c","pub fn c()->u32{b::b()+1}\n");
 return checkout;
}
const plan=(checkout:ReturnType<typeof chain>):OffloadPlanV1=>{
 const result=planOffloadV1({args:checkout.arguments("c",["b"]),environment:{...checkout.environment("c"),HOME:"/Users/someone",PATH:"/x",SEMIO_UNIT_CACHE_ROOT:"/c",CARGO_MAKEFLAGS:"-j2"},cwd:checkout.root});
 if("declined" in result)throw Error(result.declined+" "+result.detail);
 return result;
};

test("the policy and the messages validate against the closed schema, in both validators, and both languages carry the same texts",()=>{
 const ours=validateJsonSchemaSubset({$ref:"#/definitions/Policy"},schema.policy,schema as never);
 expect(ours).toEqual([]);
 expect(ajv.validate({$ref:`${schema.$id}#/definitions/Policy`},schema.policy),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate({$ref:`${schema.$id}#/definitions/Policy`},{...schema.policy,extra:1})).toBe(false);
 expect(Object.keys(schema.messages.en).sort()).toEqual(Object.keys(schema.messages.de).sort());
 for(const key of ["no-idle-machine","queue-timeout","infrastructure","path-dependent-output"])expect(offloadTextV1("de",`reason.${key}`)).not.toBe(offloadTextV1("en",`reason.${key}`));
 expect(ajv.validate({$ref:`${schema.$id}#/definitions/Event`},{version:1,atMs:1,pid:2,kind:"offload-run",crate:"a",key:null,milliseconds:3,waitedMs:0,origin:"w"})).toBe(true);
});

test("the setting reads never, distribute and pin:<origin>, and every other value, including the removed always and auto, is the default distribute",()=>{
 expect(offloadSettingV1({SEMIO_OFFLOAD:"never"})).toEqual({mode:"never"});expect(offloadSettingV1({SEMIO_OFFLOAD:"distribute"})).toEqual({mode:"distribute"});
 expect(offloadSettingV1({SEMIO_OFFLOAD:"pin:g1-m06"})).toEqual({mode:"pin",pin:"g1-m06"});expect(offloadPinV1({SEMIO_OFFLOAD:"pin:g1-m06"})).toBe("g1-m06");expect(offloadPinV1({SEMIO_OFFLOAD:"distribute"})).toBeUndefined();
 for(const value of [undefined,"","sometimes","always","auto","pin:","pin:-x","pin:a b","pin:"+"a".repeat(65)])expect(offloadModeV1({SEMIO_OFFLOAD:value}),String(value)).toBe("distribute");
 expect(OFFLOAD_POLICY_V1.mode.default).toBe("distribute");
});

test("a unit that the host linker links requires a worker with the same SDK: workers offer semio-sdk=<SDK directory name> by themselves, the requester asks for its own, and WebAssembly units and plain libraries ask for none",()=>{
 expect(offloadLinksNativelyV1(["bin"],undefined)).toBe(true);expect(offloadLinksNativelyV1(["proc-macro"],undefined)).toBe(true);expect(offloadLinksNativelyV1(["lib","cdylib"],"aarch64-apple-darwin")).toBe(true);
 expect(offloadLinksNativelyV1(["lib"],undefined)).toBe(false);expect(offloadLinksNativelyV1(["rlib"],undefined)).toBe(false);expect(offloadLinksNativelyV1(["cdylib"],"wasm32-wasip2")).toBe(false);expect(offloadLinksNativelyV1(["bin"],"wasm32-unknown-unknown")).toBe(false);
 expect(offloadSdkIdentityV1({SDKROOT:"/Library/Developer/CommandLineTools/SDKs/MacOSX15.4.sdk"},"darwin")).toBe("macosx15.4");
 expect(offloadSdkIdentityV1({SDKROOT:"/opt/x/MacOSX15.4.sdk"},"linux")).toBeUndefined();
 const identity={commit:"0".repeat(40),host:"aarch64-apple-darwin",digest:"a".repeat(16),targets:[]},host=[{name:"OSFamily",value:"macos"},{name:"Arch",value:"arm64"}];
 const required=remoteExecutionMatchPropertiesV1(offloadPlatformV1(host,identity,undefined,"g1-m03",0,{sdk:"MacOSX15.4.sdk"})),offered=(sdk:string|undefined)=>[...host,{name:"semio-target",value:identity.host},{name:"semio-toolchain",value:identity.digest},{name:"semio-offload-format",value:"argv-1"},...sdk===undefined?[]:[{name:"semio-sdk",value:sdk}]];
 expect(remoteExecutionPlatformMatchesV1(required,offered("MacOSX15.4.sdk"))).toBe(true);expect(remoteExecutionPlatformMatchesV1(required,offered("MacOSX14.5.sdk"))).toBe(false);expect(remoteExecutionPlatformMatchesV1(required,offered(undefined))).toBe(false);
 expect(remoteExecutionPlatformMatchesV1(remoteExecutionMatchPropertiesV1(offloadPlatformV1(host,identity,undefined,"g1-m03")),offered("MacOSX14.5.sdk")),"a plain library asks for no SDK").toBe(true);
 if(process.platform==="darwin"){const offer=detectWorkerRustcV1(process.cwd(),testRustEnvironmentV1());if(offer)expect(offer.properties.find(row=>row.name==="semio-sdk")?.value).toBe(offloadSdkIdentityV1(testRustEnvironmentV1()));}
});

test("a unit that is pinned asks for the queue and for the one machine that offers its origin; an unpinned unit that waits only asks for the queue; every other unit is declined when nobody has room",()=>{
 const identity={commit:"0".repeat(40),host:"aarch64-apple-darwin",digest:"a".repeat(16),targets:[]},host=[{name:"OSFamily",value:"macos"},{name:"Arch",value:"arm64"}],value=(rows:readonly {name:string;value:string}[],name:string)=>rows.find(row=>row.name===name)?.value;
 const idle=offloadPlatformV1(host,identity,undefined,"g1-m03"),queued=offloadPlatformV1(host,identity,undefined,"g1-m03",0,{queue:true}),pinned=offloadPlatformV1(host,identity,undefined,"g1-m03",0,{queue:true,pin:"g1-m06"});
 expect(value(idle,"semio-offload")).toBe("if-idle");expect(value(idle,"semio-pin")).toBeUndefined();
 expect(value(queued,"semio-offload")).toBe("queue");expect(value(queued,"semio-pin")).toBeUndefined();
 expect(value(pinned,"semio-offload")).toBe("queue");expect(value(pinned,"semio-pin")).toBe("g1-m06");
 expect(pinned.map(row=>row.name)).toEqual([...pinned.map(row=>row.name)].sort());
 const required=remoteExecutionMatchPropertiesV1(pinned);
 expect(required.some(row=>row.name==="semio-pin"&&row.value==="g1-m06")).toBe(true);expect(required.some(row=>row.name==="semio-offload")).toBe(false);
 expect(remoteExecutionPlatformMatchesV1(required,[...host,{name:"semio-target",value:identity.host},{name:"semio-toolchain",value:identity.digest},{name:"semio-offload-format",value:"argv-1"},{name:"semio-pin",value:"g1-m06"}])).toBe(true);
 expect(remoteExecutionPlatformMatchesV1(required,[...host,{name:"semio-target",value:identity.host},{name:"semio-toolchain",value:identity.digest},{name:"semio-offload-format",value:"argv-1"},{name:"semio-pin",value:"g1-m08"}])).toBe(false);
 expect(remoteExecutionPlatformMatchesV1(required,[...host,{name:"semio-target",value:identity.host},{name:"semio-toolchain",value:identity.digest},{name:"semio-offload-format",value:"argv-1"}])).toBe(false);
});

test("the compiler identity is the commit, the host and the first 16 hex of the sha-256 of the exact text; platform properties are sorted and carry the scheduler's control properties",()=>{
 const text=spawnSync("rustc",["-vV"],{encoding:"utf8",env:testRustEnvironmentV1()}).stdout,identity=parseRustcIdentityV1(text)!;
 expect(identity.commit).toMatch(/^[0-9a-f]{40}$/u);expect(identity.host).toMatch(/-/u);
 const python=spawnSync("python3",["-I","-c","import hashlib,sys;print(hashlib.sha256(sys.stdin.buffer.read()).hexdigest()[:16])"],{input:text,encoding:"utf8"}).stdout.trim();
 expect(identity.digest).toBe(python);
 const rows=offloadPlatformV1([{name:"OSFamily",value:"macos"},{name:"Arch",value:"arm64"},{name:"semio-target",value:"stale"}],{...identity,targets:[]},"wasm32-wasip2","g1-m03");
 expect(rows.map(row=>row.name)).toEqual([...rows.map(row=>row.name)].sort());
 expect(Object.fromEntries(rows.map(row=>[row.name,row.value]))).toEqual({Arch:"arm64",OSFamily:"macos","semio-offload":"if-idle","semio-offload-format":"argv-1","semio-origin":"g1-m03","semio-rustc-target-wasm32-wasip2":"1","semio-target":identity.host,"semio-toolchain":identity.digest});
 expect(parseRustcIdentityV1("rustc nothing")).toBeUndefined();
});

test("the dependency closure follows the extra-filename each artifact records, so a direct extern brings its transitive dependencies and the rmeta beside an rlib",()=>{
 const checkout=chain("closure");
 const closure=dependencyClosureV1([checkout.deps],[join(checkout.deps,`libb-${externHash("b")}.rlib`)])!.get(checkout.deps)!;
 expect(closure).toContain(`liba-${externHash("a")}.rlib`);expect(closure).toContain(`liba-${externHash("a")}.rmeta`);
 expect(closure).toContain(`libb-${externHash("b")}.rmeta`);
 expect(dependencyClosureV1([checkout.deps],[join(checkout.deps,`liba-${externHash("a")}.rlib`)])?.get(checkout.deps)).not.toContain(`libb-${externHash("b")}.rlib`);
 expect(dependencyClosureV1([join(checkout.base,"missing")],[])).toEqual(new Map());
 expect(dependencyClosureV1([join(checkout.base,"missing"),checkout.deps],[join(checkout.deps,`libb-${externHash("b")}.rlib`)])?.get(checkout.deps)).toContain(`liba-${externHash("a")}.rmeta`);
 const library=dependencyClosureV1([checkout.deps],[join(checkout.deps,`libb-${externHash("b")}.rmeta`)],undefined,false)!.get(checkout.deps)!;
 expect(library).toContain(`liba-${externHash("a")}.rmeta`);expect(library).not.toContain(`liba-${externHash("a")}.rlib`);expect(library).not.toContain(`libb-${externHash("b")}.rlib`);
});

test("a search directory full of unrelated artifacts ships only the artifacts the unit loads: the direct externs and what their metadata names, an unreadable or missing directory is skipped, a library unit leaves the rlibs behind",()=>{
 const checkout=chain("closure-unrelated"),crowd=join(checkout.base,"crowd");
 try{
  mkdirSync(crowd,{recursive:true});
  const hex=(index:number)=>index.toString(16).padStart(16,"0"),artifact=(name:string,index:number,extension:string)=>`lib${name}-${hex(index)}.${extension}`;
  for(let index=1;index<=300;index++)for(const extension of ["rlib","rmeta"])writeFileSync(join(crowd,artifact("unrelated"+index,index,extension)),extension==="rmeta"?`names ${hex(index+1)} only`:"x".repeat(2048));
  writeFileSync(join(crowd,artifact("root",1000,"rmeta")),`depends on ${hex(1001)}`);writeFileSync(join(crowd,artifact("root",1000,"rlib")),`depends on ${hex(1001)} and ${hex(1002)}`);
  writeFileSync(join(crowd,artifact("leaf",1001,"rmeta")),"leaf");writeFileSync(join(crowd,artifact("leaf",1001,"rlib")),"leaf");
  const direct=[join(crowd,artifact("root",1000,"rmeta"))],linked=dependencyClosureV1([crowd,join(checkout.base,"gone")],direct)!.get(crowd)!,library=dependencyClosureV1([crowd,join(checkout.base,"gone")],direct,undefined,false)!.get(crowd)!;
  expect(linked).toEqual([artifact("leaf",1001,"rlib"),artifact("leaf",1001,"rmeta"),artifact("root",1000,"rlib"),artifact("root",1000,"rmeta")]);
  expect(library).toEqual([artifact("leaf",1001,"rmeta"),artifact("root",1000,"rmeta")]);
 }finally{rmSync(checkout.base,{recursive:true,force:true});}
});

test("offload preserves each Cargo search root and ships only metadata-reached artifacts across them",()=>{
 const checkout=chain("split-dependency-roots");
 try{
  checkout.compile("unused","pub fn unused()->u32{0}\n");
  const transitive=join(checkout.target,"transitive","out");mkdirSync(transitive,{recursive:true});
  const artifact=(name:string,extension:string)=>`lib${name}-${externHash(name)}.${extension}`;
  for(const name of ["a","unused"])for(const extension of ["rlib","rmeta"]){
   const file=artifact(name,extension);renameSync(join(checkout.deps,file),join(transitive,file));
  }
  const args=checkout.arguments("c",["b"],["-L",`dependency=${transitive}`]).map(argument=>
   argument.startsWith("b=")?argument.replace(/\.rlib$/u,".rmeta"):argument.startsWith("--emit=")?"--emit=metadata":argument,
  );
  const result=planOffloadV1({args,environment:checkout.environment("c"),cwd:checkout.root});
  if("declined" in result)throw Error(result.declined+" "+result.detail);
  const primary=OFFLOAD_POLICY_V1.virtual.deps,secondary=primary+"1";
  expect(result.arguments.filter(argument=>argument.startsWith("dependency="))).toEqual([`dependency=${primary}`,`dependency=${secondary}`]);
  expect(result.files.get(`${primary}/${artifact("b","rmeta")}`)).toBe(join(checkout.deps,artifact("b","rmeta")));
  expect(result.files.get(`${secondary}/${artifact("a","rmeta")}`)).toBe(join(transitive,artifact("a","rmeta")));
  for(const extension of ["rlib","rmeta"])expect(result.files.has(`${secondary}/${artifact("unused",extension)}`)).toBe(false);
  expect(result.files.has(`${secondary}/${artifact("a","rlib")}`)).toBe(false);
 }finally{rmSync(checkout.base,{recursive:true,force:true});}
});

test("the transitive extern closure is complete: rustc itself compiles the planned unit from the shipped files alone, with the dependency of a dependency in a second search root, and a dependency that only one file of an artifact names is followed too",()=>{
 const checkout=chain("closure-oracle"),scratchRoot=mkdtempSync(join(checkout.base,"worker-"));
 try{
  const transitive=join(checkout.target,"transitive","out");mkdirSync(transitive,{recursive:true});
  for(const extension of ["rlib","rmeta"]){const file=`liba-${externHash("a")}.${extension}`;renameSync(join(checkout.deps,file),join(transitive,file));}
  const args=checkout.arguments("c",["b"],["-L",`dependency=${transitive}`]);
  const result=planOffloadV1({args,environment:checkout.environment("c"),cwd:checkout.root});
  if("declined" in result)throw Error(result.declined+" "+result.detail);
  for(const [path,real] of result.files){const target=join(scratchRoot,path);mkdirSync(join(target,".."),{recursive:true});writeFileSync(target,readFileSync(real));}
  mkdirSync(join(scratchRoot,OFFLOAD_POLICY_V1.virtual.out),{recursive:true});
  const run=spawnSync("rustc",result.arguments as string[],{cwd:scratchRoot,encoding:"utf8",env:testRustEnvironmentV1({...checkout.environment("c"),CARGO_MANIFEST_DIR:join(scratchRoot,"pk","c")})});
  expect(run.status,run.stderr.slice(0,600)).toBe(0);
  expect(readdirSync(join(scratchRoot,OFFLOAD_POLICY_V1.virtual.out)).some(name=>name===`libc-${externHash("c")}.rmeta`)).toBe(true);
  const stub=join(checkout.base,"stub");mkdirSync(stub,{recursive:true});
  writeFileSync(join(stub,"libx-0123456789abcdef.rmeta"),"refers to -fedcba9876543210 here");writeFileSync(join(stub,"libx-0123456789abcdef.rlib"),"nothing");writeFileSync(join(stub,"liby-fedcba9876543210.rmeta"),"leaf");
  expect(dependencyClosureV1([stub],[join(stub,"libx-0123456789abcdef.rlib")])!.get(stub)).toEqual(["libx-0123456789abcdef.rlib","libx-0123456789abcdef.rmeta","liby-fedcba9876543210.rmeta"]);
 }finally{rmSync(checkout.base,{recursive:true,force:true});}
});

test("a unit whose sources read a SEMIO_ variable that the requester has set stays here instead of compiling differently on a machine that never receives it; one that reads an unset variable, or none, is planned",()=>{
 const checkout=chain("semio-read");
 try{
  const source=join(checkout.root,"pk","c","src","lib.rs");
  writeFileSync(source,'pub fn c()->Option<&\'static str>{option_env!("SEMIO_BUILD_FLAVOUR")}\n');
  const plan_=(extra:Record<string,string>)=>planOffloadV1({args:checkout.arguments("c",["b"]),environment:{...checkout.environment("c"),...extra},cwd:checkout.root});
  const refused=plan_({SEMIO_BUILD_FLAVOUR:"x"});
  expect(refused).toMatchObject({declined:"not-portable"});expect((refused as {detail:string}).detail).toContain("SEMIO_BUILD_FLAVOUR");
  expect("declined" in plan_({}),"unset: the compile sees the same on both machines").toBe(false);
  expect("declined" in plan_({SEMIO_OTHER:"x"}),"a variable the sources never read is not carried and not a reason").toBe(false);
  expect(semioEnvironmentReadV1([source],{SEMIO_BUILD_FLAVOUR:"x"})).toBe("SEMIO_BUILD_FLAVOUR");expect(semioEnvironmentReadV1([source],{})).toBeUndefined();
  expect(semioEnvironmentReadV1(["x.txt"],{SEMIO_BUILD_FLAVOUR:"x"},()=>'env!("SEMIO_BUILD_FLAVOUR")'),"only Rust files are read").toBeUndefined();
  expect(semioEnvironmentReadV1(["x.rs"],{SEMIO_A:"1"},()=>'const A:&str=env!("SEMIO_A");')).toBe("SEMIO_A");
 }finally{rmSync(checkout.base,{recursive:true,force:true});}
});

test("walking a package is ordered, skips ignored directories and refuses beyond its bounds",()=>{
 const checkout=chain("walk"),files=walkFilesV1(join(checkout.root,"pk","c"),["target"])!;
 expect([...files.keys()]).toEqual(["Cargo.toml","src/lib.rs"]);
 expect(walkFilesV1(join(checkout.root,"pk","c"),[],1)).toBeUndefined();
 expect(walkFilesV1(join(checkout.root,"pk","c"),[],10,5)).toBeUndefined();
});

test("a plan is path-independent: two checkouts at different absolute paths yield the same arguments, environment and virtual files, and no requester path remains",()=>{
 const left=chain("left"),right=chain("right"),a=plan(left),b=plan(right);
 expect(left.root).not.toBe(right.root);
 expect(a.arguments).toEqual(b.arguments);expect(a.environment).toEqual(b.environment);expect([...a.files.keys()]).toEqual([...b.files.keys()]);
 for(const text of [...a.arguments,...Object.values(a.environment)]){expect(text.includes(left.base),text).toBe(false);}
 expect(a.arguments).toContain(OFFLOAD_POLICY_V1.virtual.out);
 expect(a.arguments.some(argument=>argument.startsWith("dependency="+OFFLOAD_POLICY_V1.virtual.deps))||a.arguments.includes("dependency="+OFFLOAD_POLICY_V1.virtual.deps)).toBe(true);
 expect(a.arguments.filter(argument=>isAbsolute(argument))).toEqual([]);
 expect(a.arguments.join(" ")).not.toContain("-C incremental");
 expect(a.environment.PATH).toBe(OFFLOAD_POLICY_V1.environment.path);expect(a.environment.SEMIO_TOOL_RUSTC).toBeUndefined();
 expect(a.environment.SEMIO_ABSOLUTE).toBe("CARGO_MANIFEST_DIR");expect(a.environment.SEMIO_REL_CARGO_MANIFEST_DIR).toBe("pk/c");
 for(const denied of ["HOME","SEMIO_UNIT_CACHE_ROOT","CARGO_MAKEFLAGS"])expect(a.environment[denied],denied).toBeUndefined();
 expect(a.environment.CARGO_PKG_NAME).toBe("c");expect(a.environment.RUST_MIN_STACK).toBe("8388608");
 const files=[...a.files.keys()];
 expect(files).toContain("pk/c/src/lib.rs");expect(files).toContain(`${OFFLOAD_POLICY_V1.virtual.deps}/libb-${externHash("b")}.rlib`);expect(files).toContain(`${OFFLOAD_POLICY_V1.virtual.deps}/liba-${externHash("a")}.rmeta`);
 for(const real of a.files.values())expect(existsSync(real),real).toBe(true);
});

test("units that cannot be made path-independent are declined with a reason instead of being half-mapped",()=>{
 const checkout=chain("decline"),base={environment:checkout.environment("c"),cwd:checkout.root};
 const declined=(args:readonly string[],environment=base.environment)=>{const result=planOffloadV1({args,environment,cwd:checkout.root});return "declined" in result?result.declined:"planned";};
 expect(declined([...checkout.arguments("c",["b"]),"-l","dylib=foo"])).toBe("not-portable");
 expect(declined([...checkout.arguments("c",["b"]),"-L","native=/opt/homebrew/lib"])).toBe("not-portable");
 expect(declined([...checkout.arguments("c",["b"]),"-L","crate=/x"])).toBe("not-portable");
 expect(declined([...checkout.arguments("c",["b"]),"-o","/x"])).toBe("unmapped-path");
 expect(declined([...checkout.arguments("c",["b"]),"-C","linker=/Users/x/ld"])).toBe("unmapped-path");
 expect(declined([...checkout.arguments("c",["b"]),"-C",`link-arg=${checkout.root}/x`])).toBe("not-portable");
 expect(declined(checkout.arguments("c",["b"]),{...base.environment,CARGO_SOMETHING:`${checkout.root}/x`})).toBe("unmapped-path");
 expect(declined(checkout.arguments("c",["b"]),{...base.environment,CARGO_MANIFEST_DIR:undefined as never})).toBe("unmapped-path");
 expect(declined(checkout.arguments("c",["missing"]))).toBe("unreadable-input");
});

test("text from the other machine is translated back in one pass, longest names first, with its root and the remapped root",()=>{
 const mappings=[{real:"/r/target/deps",virtual:".semio-offload/d"},{real:"/r/other",virtual:".semio-offload/d1"},{real:"target/deps",virtual:".semio-offload/o"}];
 const text=offloadRealTextV1('{"artifact":".semio-offload/o/liba.rmeta"} .semio-offload/d1/x .semio-offload/d/y /w/o/root/.semio-offload/d/z /semio/root/pk/c .semio-offload/dx',mappings,"/w/o/root","/r");
 expect(text).toBe('{"artifact":"target/deps/liba.rmeta"} /r/other/x /r/target/deps/y /r/target/deps/z /r/pk/c .semio-offload/dx');
 expect(offloadRealTextV1("plain",[],undefined,"/r")).toBe("plain");
 expect(readdirSync(".").length).toBeGreaterThan(0);
});

const probeOf=(name:"critical-pressure-gigabyte-swap"|"quiet-no-swap")=>testHostProbeV1(name,name==="quiet-no-swap");

test("the machine's single admission decision says wait under critical pressure and admit on a calm host, through the injected probe",async()=>{
 const roomy={...BUILD_HOST_CAPACITY_POLICY_V1,load:{...BUILD_HOST_CAPACITY_POLICY_V1.load,perCoreCeiling:100000}};
 const pressured=await localUnitAdmissionV1({probe:probeOf("critical-pressure-gigabyte-swap"),policy:roomy});
 expect(pressured.verdict).toBe("wait");expect(["pressure-critical","swap","swap-activity"]).toContain(pressured.reason);
 const calm=await localUnitAdmissionV1({probe:probeOf("quiet-no-swap"),policy:roomy});
 expect(calm.verdict).toBe("admit");
 const loaded=await localUnitAdmissionV1({probe:probeOf("quiet-no-swap"),policy:{...BUILD_HOST_CAPACITY_POLICY_V1,load:{...BUILD_HOST_CAPACITY_POLICY_V1.load,perCoreCeiling:0}}});
 expect(loaded.verdict).toBe("wait");expect(loaded.reason).toBe("load");
});

test("verdicts are shared within their window and expire after it",()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-memo-")));
 writeOffloadMemoV1(root,"declined",{atMs:1000});
 expect(readOffloadMemoV1(root,"declined",1500,3000)).toEqual({atMs:1000});
 expect(readOffloadMemoV1(root,"declined",4001,3000)).toBeUndefined();expect(readOffloadMemoV1(root,"declined",500,3000)).toBeUndefined();expect(readOffloadMemoV1(root,"other",1500,3000)).toBeUndefined();
 rmSync(root,{recursive:true,force:true});
});

test("a unit queues here when every slot is held: the estimate is the age of the oldest live waiter, and zero while a slot is free",()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-slots-"))),slots=join(root,"s"),tickets=join(root,"q");
 mkdirSync(tickets,{recursive:true});
 expect(slotQueueMillisecondsV1(root,2,5000)).toBe(0);expect(slotQueueMillisecondsV1(root,0,5000)).toBe(0);
 const first=tryClaimV1(slots,"0")!,second=tryClaimV1(slots,"1")!;
 expect(slotQueueMillisecondsV1(root,2,5000)).toBe(1);
 writeFileSync(join(tickets,`${String(2000).padStart(15,"0")}-abcd1234-${process.pid}`),"{}");
 writeFileSync(join(tickets,`${String(1000).padStart(15,"0")}-abcd1234-999999999`),"{}");
 expect(slotQueueMillisecondsV1(root,2,5000)).toBe(3000);
 first.release();expect(slotQueueMillisecondsV1(root,2,5000)).toBe(0);
 second.release();rmSync(root,{recursive:true,force:true});
});

test("the job token goes back to the build's named pool while the unit is away and is taken back afterwards, and pools that cannot be served keep the token",async()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-token-"))),fifo=join(root,"pool");
 expect(spawnSync("mkfifo",[fifo]).status).toBe(0);
 const opened=await import("node:fs").then(fs=>fs.openSync(fifo,fs.constants.O_RDWR|fs.constants.O_NONBLOCK));
 const {writeSync,readSync,closeSync}=await import("node:fs");
 writeSync(opened,"++");
 const count=()=>{const buffer=Buffer.alloc(16);let total=0;const taken:Buffer[]=[];try{for(;;){const n=readSync(opened,buffer,0,1,null);if(n<=0)break;total+=n;taken.push(Buffer.from("+"));}}catch{}if(total>0)writeSync(opened,"+".repeat(total));return total;};
 expect(count()).toBe(2);
 const lease=releaseImplicitTokenV1({CARGO_MAKEFLAGS:`-j3 --jobserver-fds=fifo:${fifo} --jobserver-auth=fifo:${fifo}`},"darwin")!;
 expect(lease).toBeDefined();expect(count()).toBe(3);
 expect(await lease.reacquire(1000)).toBe(true);expect(count()).toBe(2);
 expect(await lease.reacquire(1000)).toBe(true);expect(count()).toBe(2);
 lease.close();
 const empty=join(root,"empty");expect(spawnSync("mkfifo",[empty]).status).toBe(0);
 const idle=releaseImplicitTokenV1({MAKEFLAGS:`--jobserver-auth=fifo:${empty}`},"darwin")!;
 const started=Date.now();expect(await idle.reacquire(150)).toBe(true);expect(Date.now()-started).toBeLessThan(140);
 const none=releaseImplicitTokenV1({MAKEFLAGS:`--jobserver-auth=fifo:${empty}`},"darwin")!;
 expect(await none.reacquire(100)).toBe(true);
 expect(releaseImplicitTokenV1({},"darwin")).toBeUndefined();expect(releaseImplicitTokenV1({MAKEFLAGS:"--jobserver-auth=3,4"},"darwin")).toBeUndefined();expect(releaseImplicitTokenV1({MAKEFLAGS:"--jobserver-auth=fifo:/nope"},"darwin")).toBeUndefined();expect(releaseImplicitTokenV1({MAKEFLAGS:`--jobserver-auth=fifo:${fifo}`},"win32")).toBeUndefined();
 closeSync(opened);rmSync(root,{recursive:true,force:true});
});

test("every decline and fall-back is counted under its reason beside the three totals, so the refusals can be told apart by cause",()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-reasons-")));
 for(const [add,reason,times] of [[{refused:1},"no-idle-machine",5],[{refused:1},"queue-timeout",2],[{fellBack:1},"worker-policy",3],[{fellBack:1},"incomplete-inputs",4],[{},"unmapped-path",6]] as const)for(let index=0;index<times;index++)bumpOffloadCountersV1(root,add,reason);
 bumpOffloadCountersV1(root,{out:1});bumpOffloadCountersV1(root,{refused:1},"../escape");bumpOffloadCountersV1(root,{},"Bad Reason");
 expect(readOffloadReasonCountersV1(root)).toEqual({"no-idle-machine":5,"queue-timeout":2,"worker-policy":3,"incomplete-inputs":4,"unmapped-path":6});
 expect(readOffloadCountersV1(root)).toEqual({out:1,refused:8,fellBack:7});
 const counted=readOffloadReasonCountersV1(root);
 expect(counted["no-idle-machine"]!+counted["queue-timeout"]!,"the reasons of the refused total").toBe(7);expect(readOffloadCountersV1(root).refused-1,"one refused event had an unusable reason name and is counted in the total only").toBe(7);
 expect(readOffloadReasonCountersV1(join(root,"nothing"))).toEqual({});
 rmSync(root,{recursive:true,force:true});
});

test("an eligibility refusal is one offload-declined event with its reason and detail and one counter, in English and German; a unit that is only cheaper than a round trip is counted without an event",()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-declines-"))),cache=join(root,"cache");
 const environment={SEMIO_UNIT_CACHE_WORKSPACE_ROOT:root};
 declineOffloadV1(cache,environment,{lane:"t",crate:"k",key:"abc",reason:"crate-type",detail:"staticlib"});
 declineOffloadV1(cache,environment,{crate:"s",key:null,reason:"source-kind",detail:"-"});
 declineOffloadV1(cache,environment,{crate:"s",key:null,reason:"cheap",silent:true});declineOffloadV1(cache,environment,{crate:"s",key:null,reason:"cheap",silent:true});
 const rows=readFileSync(join(cache,"x","offload.jsonl"),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line));
 expect(rows.map(row=>[row.kind,row.reason,row.detail,row.crate])).toEqual([["offload-declined","crate-type","staticlib","k"],["offload-declined","source-kind","-","s"]]);
 for(const row of rows)expect(ajv.validate({$ref:`${schema.$id}#/definitions/Event`},row),JSON.stringify(ajv.errors)).toBe(true);
 const lines=readFileSync(join(cache,"x","offload.progress"),"utf8").split("\n").filter(Boolean);
 expect(lines).toEqual(["Unit k stays here: its crate type is linked here.","Einheit k bleibt hier: sein Crate-Typ wird hier gelinkt.","Unit s stays here: its source is not a Rust file.","Einheit s bleibt hier: seine Quelle ist keine Rust-Datei."]);
 expect(readOffloadReasonCountersV1(root)).toEqual({"crate-type":1,"source-kind":1,cheap:2});expect(readOffloadCountersV1(root)).toEqual({out:0,refused:0,fellBack:0});
 rmSync(root,{recursive:true,force:true});
});

test("counters add up concurrent events exactly and keep the unit cache fields of the shared file",async()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-counters-"))),file=join(root,".cache","f","build-counters.json");
 mkdirSync(join(root,".cache","f"),{recursive:true});writeFileSync(file,JSON.stringify({version:1,wallMs:5,unit:{localHit:1,remoteHit:2,miss:3,stored:4},offload:{out:0,refused:0,fellBack:0}}));
 await Promise.all(Array.from({length:40},(_,index)=>Promise.resolve().then(()=>bumpOffloadCountersV1(root,index%4===0?{refused:1}:index%4===1?{fellBack:1}:{out:1}))));
 bumpOffloadCountersV1(root,{out:1});
 const value=JSON.parse(readFileSync(file,"utf8"));
 expect(value.unit).toEqual({localHit:1,remoteHit:2,miss:3,stored:4});expect(value.wallMs).toBe(5);expect(value.offload).toEqual({out:21,refused:10,fellBack:10});
 expect(readOffloadCountersV1(root)).toEqual({out:21,refused:10,fellBack:10});
 rmSync(root,{recursive:true,force:true});
});

test("the --offload flag is taken out before the delimiter, in both spellings, and refused when unknown or repeated",()=>{
 expect(extractOffloadFlagV1(["run","x:build","--offload=distribute","--","--offload=never"])).toEqual({args:["run","x:build","--","--offload=never"],mode:"distribute"});
 expect(extractOffloadFlagV1(["run","--offload=pin:g1-m06","y"])).toEqual({args:["run","y"],mode:"pin:g1-m06"});
 expect(extractOffloadFlagV1(["run","--offload","never","y"])).toEqual({args:["run","y"],mode:"never"});
 expect(extractOffloadFlagV1(["run","y"])).toEqual({args:["run","y"]});
 for(const bad of [["--offload=sometimes"],["--offload"],["--offload=always"],["--offload=auto"],["--offload=pin:"],["--offload=distribute","--offload=never"]])expect(()=>extractOffloadFlagV1(bad),bad.join(" ")).toThrow(/--offload takes never, distribute or pin:<origin>, once\.\n--offload nimmt never, distribute oder pin:<Ursprung>, genau einmal\./u);
});

test("repo run and repo cargo take --offload out of their arguments and hand the mode to the build environment",async()=>{
 const {resolveBuildModeV1}=await import("../../../🧭️invocation/🟦️.ts");
 const resolved=resolveBuildModeV1({args:["cargo","--store=shared","--offload=pin:g1-m06","--configuration=prototype","--","check","-p","x","--offload=never"],env:{}},{});
 expect(resolved.env.SEMIO_OFFLOAD).toBe("pin:g1-m06");expect(resolved.args).toEqual(["cargo","--store=shared","--configuration=prototype","--","check","-p","x","--offload=never"]);
 const plain=resolveBuildModeV1({args:["cargo","--store=shared","--configuration=prototype","--","check"],env:{}},{});
 expect(plain.env.SEMIO_OFFLOAD).toBeUndefined();
 expect(()=>resolveBuildModeV1({args:["run","a:build","--offload=maybe"],env:{}},{})).toThrow(/--offload nimmt/u);
});

test("Cargo's CARGO_MANIFEST_PATH is mapped like the manifest directory, and a wrong one is declined",()=>{
 const left=chain("manifest-left"),right=chain("manifest-right");
 const planned=(checkout:ReturnType<typeof chain>,path=join(checkout.root,"pk","c","Cargo.toml"))=>planOffloadV1({args:checkout.arguments("c",["b"]),environment:{...checkout.environment("c"),CARGO_MANIFEST_PATH:path},cwd:checkout.root});
 const a=planned(left),b=planned(right);
 if("declined" in a||"declined" in b)throw Error("planned");
 expect(a.environment.SEMIO_ABSOLUTE).toBe("CARGO_MANIFEST_DIR CARGO_MANIFEST_PATH");expect(a.environment.SEMIO_REL_CARGO_MANIFEST_PATH).toBe("pk/c/Cargo.toml");
 expect(a.environment).toEqual(b.environment);expect(a.environment.CARGO_MANIFEST_PATH).toBeUndefined();
 const wrong=planned(left,join(left.root,"elsewhere","Cargo.toml"));expect("declined" in wrong&&wrong.declined).toBe("unmapped-path");
});

test("the repository's own layouts are mapped: an absolute source in the unit's own directory (registry crates) and a manifest below the directory that holds the source (module layout), both path-independent",()=>{
 const planned=(checkout:ReturnType<typeof chain>,mode:"registry"|"module")=>{
  const base=join(checkout.root,"pk"),manifest=mode==="registry"?join(base,"c"):join(base,"m","packages","rust");
  mkdirSync(join(manifest,"src"),{recursive:true});writeFileSync(join(manifest,"Cargo.toml"),"[package]\nname = \"c\"\n");
  const source=mode==="registry"?join(manifest,"src","lib.rs"):join(base,"m","lib.rs");writeFileSync(source,"pub fn c()->u32{b::b()+1}\n");
  const cwd=mode==="registry"?manifest:checkout.root,args=checkout.arguments("c",["b"]).map(argument=>argument==="pk/c/src/lib.rs"?(mode==="registry"?source:"pk/m/lib.rs"):argument);
  return{cwd,result:planOffloadV1({args,environment:{...checkout.environment("c"),CARGO_MANIFEST_DIR:manifest},cwd})};
 };
 for(const mode of ["registry","module"] as const){
  const left=chain("layout-left-"+mode),right=chain("layout-right-"+mode),a=planned(left,mode),b=planned(right,mode);
  if("declined" in a.result||"declined" in b.result)throw Error(mode+" declined: "+JSON.stringify(a.result)+JSON.stringify(b.result));
  expect(a.result.arguments,mode).toEqual(b.result.arguments);expect(a.result.environment,mode).toEqual(b.result.environment);expect([...a.result.files.keys()],mode).toEqual([...b.result.files.keys()]);
  expect(a.result.arguments.some(argument=>argument.includes(left.base)),mode).toBe(false);
  if(mode==="registry"){expect(a.result.environment.SEMIO_SOURCE,"an absolute source stays absolute on the other machine, rooted at its work directory").toBe("src/lib.rs");expect(a.result.arguments).not.toContain("src/lib.rs");}
  else{expect([...a.result.files.keys()]).toContain("pk/m/lib.rs");expect([...a.result.files.keys()]).toContain("pk/m/packages/rust/Cargo.toml");expect(a.result.environment.SEMIO_REL_CARGO_MANIFEST_DIR).toBe("pk/m/packages/rust");}
 }
 const checkout=chain("layout-outside"),manifest=join(checkout.root,"pk","c"),result=planOffloadV1({args:checkout.arguments("c",["b"]).map(argument=>argument==="pk/c/src/lib.rs"?"/tmp/outside/lib.rs":argument),environment:{...checkout.environment("c"),CARGO_MANIFEST_DIR:manifest},cwd:checkout.root});
 expect("declined" in result&&result.declined).toBe("unmapped-path");
});

test("a one-line shim that includes the crate's code from the module directory above its manifest ships that directory (and the files that its files include in turn)",()=>{
 const planned=(checkout:ReturnType<typeof chain>)=>{
  const base=join(checkout.root,"pk"),manifest=join(base,"m","packages","rust");
  mkdirSync(manifest,{recursive:true});writeFileSync(join(manifest,"Cargo.toml"),"[package]\nname = \"c\"\n");
  writeFileSync(join(manifest,"shim.rs"),"#[path = \"../../lib.rs\"]\nmod component;\npub use component::*;\n");
  writeFileSync(join(base,"m","lib.rs"),"mod part;\npub fn c()->u32{b::b()+include_str!(\"data/v.txt\").len() as u32}\n");writeFileSync(join(base,"m","part.rs"),"\n");
  mkdirSync(join(base,"m","data"),{recursive:true});writeFileSync(join(base,"m","data","v.txt"),"1");
  const args=checkout.arguments("c",["b"]).map(argument=>argument==="pk/c/src/lib.rs"?"pk/m/packages/rust/shim.rs":argument);
  return planOffloadV1({args,environment:{...checkout.environment("c"),CARGO_MANIFEST_DIR:manifest},cwd:checkout.root});
 };
 const left=planned(chain("shim-left")),right=planned(chain("shim-right"));
 if("declined" in left||"declined" in right)throw Error(JSON.stringify(left)+JSON.stringify(right));
 const names=[...left.files.keys()];
 for(const wanted of["pk/m/packages/rust/shim.rs","pk/m/lib.rs","pk/m/part.rs","pk/m/data/v.txt","pk/m/packages/rust/Cargo.toml"])expect(names).toContain(wanted);
 expect(names).toEqual([...right.files.keys()]);expect(left.arguments).toEqual(right.arguments);
 expect(includedSourcesV1("/nonexistent/entry.rs",()=>undefined)).toEqual([]);
 const chainRead=(path:string)=>path==="/x/a.rs"?'#[path = "b.rs"] mod b; include!("c.rs");':path==="/x/b.rs"?'#[path = "a.rs"] mod a;':undefined;
 expect(includedSourcesV1("/x/a.rs",chainRead)).toEqual([]);
});

test("a unit Cargo compiles in its package directory (absolute source, working directory = manifest directory) whose shim includes the module directory above the manifest is planned too: the module directory travels as the virtual package, the plan is the same in two checkouts, and an ancestor that is no module directory is still declined",()=>{
 const planned=(checkout:ReturnType<typeof chain>,depth:string)=>{
  const base=join(checkout.root,depth),manifest=join(base,"m","packages","rust");
  mkdirSync(manifest,{recursive:true});writeFileSync(join(manifest,"Cargo.toml"),"[package]\nname = \"c\"\n");
  writeFileSync(join(manifest,"shim.rs"),"#[path = \"../../lib.rs\"]\nmod component;\npub use component::*;\n");
  writeFileSync(join(base,"m","lib.rs"),"mod part;\npub fn c()->u32{b::b()+include_str!(\"data/v.txt\").len() as u32}\n");writeFileSync(join(base,"m","part.rs"),"\n");
  mkdirSync(join(base,"m","data"),{recursive:true});writeFileSync(join(base,"m","data","v.txt"),"1");
  const args=checkout.arguments("c",["b"]).map(argument=>argument==="pk/c/src/lib.rs"?join(manifest,"shim.rs"):argument).filter((argument,index,all)=>!argument.startsWith("--remap-path-prefix")&&all[index-1]!=="--remap-path-prefix");
  return{manifest,base,result:planOffloadV1({args:[...args,`--remap-path-prefix=${checkout.root}=.`],environment:{...checkout.environment("c"),CARGO_MANIFEST_DIR:manifest},cwd:manifest})};
 };
 const left=planned(chain("package-cwd-left"),"pk"),right=planned(chain("package-cwd-right"),"pk");
 if("declined" in left.result||"declined" in right.result)throw Error(JSON.stringify(left.result)+JSON.stringify(right.result));
 expect(left.result.arguments).toEqual(right.result.arguments);expect(left.result.environment).toEqual(right.result.environment);expect([...left.result.files.keys()]).toEqual([...right.result.files.keys()]);
 const names=[...left.result.files.keys()];
 for(const wanted of["lib.rs","part.rs","data/v.txt","packages/rust/shim.rs","packages/rust/Cargo.toml"])expect(names.map(name=>name.replace(/^\.semio-offload\/p\//u,"")),wanted).toContain(wanted);
 expect(left.result.environment.SEMIO_REL_CARGO_MANIFEST_DIR.endsWith("/packages/rust")).toBe(true);
 expect(left.result.arguments.some(argument=>argument.includes(left.base)||argument.includes(left.manifest))).toBe(false);
 expect(left.result.mappings.some(row=>row.real===left.base+"/m")).toBe(true);
 const narrow=chain("package-cwd-narrow"),narrowed=planned(narrow,"pk");
 expect("declined" in narrowed.result,"planned with the root remap").toBe(false);
 const lonely=chain("package-cwd-lonely"),base=join(lonely.root,"pk"),home=join(base,"m","packages","rust");
 mkdirSync(home,{recursive:true});writeFileSync(join(home,"Cargo.toml"),"[package]\nname = \"c\"\n");writeFileSync(join(home,"shim.rs"),"#[path = \"../../lib.rs\"]\nmod component;\n");writeFileSync(join(base,"m","lib.rs"),"pub fn c()->u32{1}\n");
 const uncovered=planOffloadV1({args:[...lonely.arguments("c",["b"]).filter((argument,index,all)=>!argument.startsWith("--remap-path-prefix")&&all[index-1]!=="--remap-path-prefix").map(argument=>argument==="pk/c/src/lib.rs"?join(home,"shim.rs"):argument),`--remap-path-prefix=${home}=.`],environment:{...lonely.environment("c"),CARGO_MANIFEST_DIR:home},cwd:home});
 expect("declined" in uncovered&&uncovered.declined,"a module directory the remaps do not cover would be recorded with this machine's path").toBe("unmapped-path");
 const shallow=chain("package-cwd-shallow"),manifest=join(shallow.root,"pk","c");
 const declined=planOffloadV1({args:shallow.arguments("c",["b"]).map(argument=>argument==="pk/c/src/lib.rs"?"/tmp/outside/lib.rs":argument),environment:{...shallow.environment("c"),CARGO_MANIFEST_DIR:manifest},cwd:manifest});
 expect("declined" in declined&&declined.declined).toBe("unmapped-path");
 expect("declined" in declined&&declined.detail).toContain("source outside CARGO_MANIFEST_DIR");
});

test("a shim whose #[path] climbs out of the module directory and back into it by name (intrinsic-size: [lib] path in packages/rust, #[path = \"../../../intrinsic-size/lib.rs\"]) keeps that directory name in the virtual layout: the text resolves on the other machine and rustc compiles the shipped files",()=>{
 const planned=(label:string,depth:string)=>{
  const checkout=chain(label),base=join(checkout.root,depth,"modules"),module=join(base,"📏️intrinsic-size"),manifest=join(module,"packages","rust");
  mkdirSync(manifest,{recursive:true});writeFileSync(join(manifest,"Cargo.toml"),"[package]\nname = \"c\"\n");
  writeFileSync(join(manifest,"lib.rs"),"#[path = \"../../../📏️intrinsic-size/lib.rs\"]\nmod component;\npub use component::*;\n");
  writeFileSync(join(module,"lib.rs"),"pub fn c()->u32{1}\n");
  const args=checkout.arguments("c",[]).map(argument=>argument==="pk/c/src/lib.rs"?join(manifest,"lib.rs"):argument).filter((argument,index,all)=>!argument.startsWith("--remap-path-prefix")&&all[index-1]!=="--remap-path-prefix");
  return{manifest,module,result:planOffloadV1({args:[...args,`--remap-path-prefix=${checkout.root}=.`,`--remap-path-prefix=${manifest}=c-1.0.0`],environment:{...checkout.environment("c"),CARGO_MANIFEST_DIR:manifest},cwd:manifest})};
 };
 const left=planned("climb-left","pk"),right=planned("climb-right","pk");
 if("declined" in left.result||"declined" in right.result)throw Error(JSON.stringify(left.result)+JSON.stringify(right.result));
 expect(left.result.arguments).toEqual(right.result.arguments);expect([...left.result.files.keys()]).toEqual([...right.result.files.keys()]);
 const names=[...left.result.files.keys()],source=left.result.environment.SEMIO_SOURCE??"";
 expect(source.endsWith("/📏️intrinsic-size/packages/rust/lib.rs")).toBe(true);
 const target=join(dirname(source),"../../../📏️intrinsic-size/lib.rs").split("\\").join("/");
 expect(names,"the file that the #[path] text names, resolved in the virtual layout").toContain(target);
 expect(climbAboveV1(join(left.manifest,"lib.rs"),[join(left.module,"lib.rs")],left.module,left.manifest)).toBe(1);
 expect(climbAboveV1("/m/a.rs",[],"/m",undefined,()=>"#[path = \"b/c.rs\"] mod x;")).toBe(0);
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-climb-")));
 try{
  for(const [name,real] of left.result.files){mkdirSync(dirname(join(root,name)),{recursive:true});writeFileSync(join(root,name),readFileSync(real));}
  const done=spawnSync("rustc",["--edition=2021","--crate-type=lib","--crate-name=c","--emit=metadata","--out-dir",root,join(root,source)],{encoding:"utf8",cwd:root,env:testRustEnvironmentV1()});
  expect(done.stderr,"rustc over the shipped virtual layout").not.toContain("couldn't find file");expect(done.status).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("environment variables the worker policy cannot carry (a name with a hyphen, a value with a line break or beyond the bound) stay on the requester instead of failing the whole action there",()=>{
 const checkout=chain("names"),result=planOffloadV1({args:checkout.arguments("c",["b"]),environment:{...checkout.environment("c"),"SENTRY-TRACE":"00-abc-def-01",CARGO_BROKEN_LINES:"a\nb",CARGO_HUGE:"x".repeat(20000),CARGO_KEPT_NAME:"fine"},cwd:checkout.root});
 if("declined" in result)throw Error(JSON.stringify(result));
 for(const name of ["SENTRY-TRACE","CARGO_BROKEN_LINES","CARGO_HUGE"])expect(result.environment[name],name).toBeUndefined();
 expect(result.environment.CARGO_KEPT_NAME).toBe("fine");expect(result.environment.CARGO_PKG_NAME).toBe("c");
});

test("the requester's own temporary directories and path remaps are left behind, not carried to the other machine",()=>{
 const checkout=chain("remap"),remap=`--remap-path-prefix=${checkout.root}/.cache/cargo=/cargo`,foreign=`--remap-path-prefix=/usr/lib=/lib`;
 const result=planOffloadV1({args:[...checkout.arguments("c",["b"]),remap,"--remap-path-prefix",`${checkout.root}/t=/t`,foreign],environment:{...checkout.environment("c"),TEMP:`${checkout.root}/.tmp`,TMP:`${checkout.root}/.tmp`},cwd:checkout.root});
 if("declined" in result)throw Error(JSON.stringify(result));
 expect(result.arguments.some(argument=>argument.includes(checkout.root))).toBe(false);
 expect(result.arguments).toContain(foreign);expect(result.environment.TEMP).toBeUndefined();expect(result.environment.TMP).toBeUndefined();
});

test("an unreachable cache endpoint is a quick, silent decline and is remembered, never a hang",async()=>{
 const {offloadRustcUnitV1}=await import("../🟦️.ts");
 const checkout=chain("unreachable"),root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-unreachable-")));
 const environment={...checkout.environment("c"),...testRustEnvironmentV1(),SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://127.0.0.1:1",SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:1",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const started=Date.now(),first=await offloadRustcUnitV1({args:checkout.arguments("c",["b"]),environment,cwd:checkout.root,root,crate:"c",key:"a".repeat(64),compiler:["rustc"]});
 expect(first).toMatchObject({kind:"local",reason:"infrastructure"});expect(Date.now()-started).toBeLessThan(5000);
 const again=Date.now(),second=await offloadRustcUnitV1({args:checkout.arguments("c",["b"]),environment,cwd:checkout.root,root,crate:"c",key:"a".repeat(64),compiler:["rustc"]});
 expect(second,"the remembered decline keeps its own reason instead of posing as a full fleet").toMatchObject({kind:"local",reason:"infrastructure",detail:"memo cache endpoint unreachable"});expect(Date.now()-again).toBeLessThan(1000);
 const missing=await offloadRustcUnitV1({args:checkout.arguments("c",["b"]),environment:{...environment,SEMIO_REMOTE_EXECUTION_ENDPOINT:"",SEMIO_REMOTE_CACHE_ENDPOINT:""},cwd:checkout.root,root:join(root,"other"),crate:"c",key:"a".repeat(64),compiler:["rustc"]});
 expect(missing).toMatchObject({kind:"local",reason:"no-endpoint"});
 rmSync(root,{recursive:true,force:true});
});

test("a cache endpoint that answered is remembered for a while: a build of many units probes it about once, not once per decline window",async()=>{
 const {offloadRustcUnitV1}=await import("../🟦️.ts");
 let probes=0;
 const cache=Bun.serve({port:0,hostname:"127.0.0.1",fetch(request){if(new URL(request.url).pathname==="/healthz"){probes++;return new Response("{\"status\":\"ok\"}");}return new Response("no",{status:404});}});
 try{
  const checkout=chain("reachable"),root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-reachable-")));
  const environment={...checkout.environment("c"),...testRustEnvironmentV1(),SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://127.0.0.1:1",SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${cache.port}`,SEMIO_REMOTE_CACHE_INSTANCE:"main"};
  for(let index=0;index<4;index++){
   await offloadRustcUnitV1({args:checkout.arguments("c",["b"]),environment,cwd:checkout.root,root,crate:"c",key:String(index).repeat(64),compiler:["rustc"]});
   rmSync(join(root,"offload-declined.json"),{force:true});
  }
  expect(probes).toBe(1);
  rmSync(root,{recursive:true,force:true});
 }finally{cache.stop(true);}
});

const TOOLCHAIN="aaaaaaaaaaaaaaaa",host=process.platform==="darwin"?"macos":process.platform==="win32"?"windows":process.platform,cpu=process.arch==="x64"?"amd64":process.arch;
const loadReport=(origin:string,toolchain:string,headroom=true,pressure:"normal"|"warn"|"critical"="normal"):RemoteExecutionLoadReportV1=>({version:1,origin,platform:{key:`${host}-${cpu}-${toolchain}`,os:host,arch:cpu,toolchain,target:"aarch64-apple-darwin"},capacity:{cores:8,memoryMegabytes:16384},running:0,queued:0,pressure,freeMegabytes:8000,load:1,headroom,wallMs:1_000_000,worker:{state:"idle",concurrency:4,running:0,sandbox:null,confinement:null,network:null},counters:{unit:{localHit:0,remoteHit:0,miss:0,stored:0},offload:{out:0,refused:0,fellBack:0}}});
const peer=(extra:Partial<{origin:string;toolchain:string;running:number;draining:boolean;headroom:boolean;pressure:"normal"|"warn"|"critical";os:string;format:boolean;refusedUntilMs:number;silent:boolean}>={})=>({origin:"peer",toolchain:TOOLCHAIN,running:0,draining:false,headroom:true,pressure:"normal" as const,os:host,format:true,refusedUntilMs:0,silent:false,...extra});
const schedulerOf=(rows:readonly ReturnType<typeof peer>[])=>({fetch:(async(url:string)=>{
 const table=new RemoteExecutionLoadTableV1(60_000,()=>1_000_000),origin=new URL(url).searchParams.get("origin")!;
 table.accept(loadReport(origin,TOOLCHAIN));
 const workers:PlacementWorkerV1[]=rows.map((row,index)=>{if(!row.silent)table.accept(loadReport(row.origin,row.toolchain,row.headroom,row.pressure));return{id:`w${index}`,name:`n${index}`,origin:row.origin,properties:[{name:"Arch",value:cpu},{name:"OSFamily",value:row.os},{name:"semio-toolchain",value:row.toolchain},{name:"semio-target",value:"aarch64-apple-darwin"},...row.format?[{name:OFFLOAD_POLICY_V1.property.format,value:OFFLOAD_POLICY_V1.property.formatValue}]:[]],concurrency:4,running:row.running,waiting:true,draining:row.draining,refusedUntilMs:row.refusedUntilMs};});
 const property=OFFLOAD_POLICY_V1.property;
 return new Response(JSON.stringify(placementIdleCountV1({id:"probe",platform:[{name:property.format,value:property.formatValue}],tried:new Set<string>(),requester:origin},workers,table,1_000_000)));
}) as unknown as typeof fetch});
const tableOf=(rows:readonly ReturnType<typeof peer>[])=>schedulerOf(rows);
test("the fleet fanout ceiling is available with offload enabled, a published scheduler with headroom, and a busy host or a door that refused",async()=>{
 const busy={probe:testHostProbeV1("critical-pressure-gigabyte-swap"),fleet:tableOf([peer()])},quiet={probe:testHostProbeV1("quiet-no-swap",true)},scheduler={SEMIO_WORKSPACE_ROOT:realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"fanout-"))),SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://192.0.2.1:8980"};
 expect(await offloadFanOutJobsV1(scheduler,busy)).toBe(Math.min(OFFLOAD_POLICY_V1.limits.fanOutJobs,availableParallelism()));
 expect(await offloadFanOutJobsV1(scheduler,quiet),"a machine that admits one more unit queues normally").toBeUndefined();
 expect(await offloadFanOutJobsV1(scheduler,{...quiet,doorRefused:true,fleet:busy.fleet}),"the door refused the whole build: that verdict counts, whatever one unit would do").toBeGreaterThan(0);
 for(const [label,workers] of [["nobody",[]],["a draining machine",[peer({draining:true})]],["a full machine",[peer({running:4})]],["a machine without headroom",[peer({headroom:false})]],["a machine under critical pressure",[peer({pressure:"critical"})]],["a machine in its refusal cool-down",[peer({refusedUntilMs:2_000_000})]],["another operating system",[peer({os:"plan9"})]],["another toolchain build",[peer({toolchain:"bbbbbbbbbbbbbbbb"})]],["an older action format",[peer({format:false})]]] as const)expect(await offloadFanOutJobsV1(scheduler,{...busy,fleet:tableOf(workers)}),label+" is no headroom").toBeUndefined();
 expect(await offloadFanOutJobsV1(scheduler,{...busy,fleet:{fetch:(async()=>{throw Error("down");}) as unknown as typeof fetch}}),"an unreachable scheduler keeps the caller waiting").toBeUndefined();
 expect(await offloadFanOutJobsV1({...scheduler,SEMIO_OFFLOAD:"never"},busy)).toBeUndefined();
 expect(await offloadFanOutJobsV1({...scheduler,SEMIO_REMOTE_EXECUTION_ENDPOINT:""},busy),"no scheduler published").toBeUndefined();
 expect(await offloadFanOutJobsV1({...scheduler,SEMIO_OFFLOAD:"distribute"},busy)).toBeGreaterThan(1);
 expect(await offloadFanOutJobsV1({...scheduler,SEMIO_OFFLOAD:"pin:g1-m06"},quiet),"a pinned build queues on one machine whatever this one is doing").toBe(Math.min(OFFLOAD_POLICY_V1.limits.fanOutJobs,availableParallelism()));
});

test("a unit whose recorded local cost is below a round trip is not worth a round trip; costly units go, unmeasured ones compile here first; the workspace root is found from any directory below it",()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"cost-")));
 expect(readUnitCostV1(root,"tiny")).toBeUndefined();expect(worthARoundTripV1(root,"tiny"),"no record: the first compile teaches the cost").toBe("unmeasured");
 writeUnitCostV1(root,"tiny",300);expect(readUnitCostV1(root,"tiny")).toBe(300);expect(worthARoundTripV1(root,"tiny")).toBe("cheap");
 writeUnitCostV1(root,"tiny",9300);expect(readUnitCostV1(root,"tiny"),"halfway to the new measurement").toBe(4800);expect(worthARoundTripV1(root,"tiny")).toBe("cheap");writeUnitCostV1(root,"tiny",14800);expect(readUnitCostV1(root,"tiny")).toBe(9800);expect(worthARoundTripV1(root,"tiny")).toBe("worth");
 writeUnitCostV1(root,"a/b c",OFFLOAD_POLICY_V1.limits.roundTripMilliseconds);expect(worthARoundTripV1(root,"a/b c")).toBe("worth");
 const checkout=join(root,"ws"),below=join(checkout,"x","y");mkdirSync(join(checkout,".cache","f"),{recursive:true});mkdirSync(below,{recursive:true});writeFileSync(join(checkout,".cache","f","machine.json"),"{}");
 expect(workspaceRootFromV1(below)).toBe(checkout);expect(workspaceRootFromV1(root)).toBeUndefined();
 rmSync(root,{recursive:true,force:true});
});

test("an extern given as an rlib ships its rmeta beside it (with embed-metadata=no the rlib alone is a stub: 'can't find crate' on the other machine), and the other way round",()=>{
 const checkout=chain("sibling"),stem=`libb-${externHash("b")}`;
 for(const extension of["rlib","rmeta"]){
  const args=checkout.arguments("c",["b"]).map(argument=>argument.startsWith("b=")?`b=${checkout.deps}/${stem}.${extension}`:argument);
  const planned=planOffloadV1({args,environment:checkout.environment("c"),cwd:checkout.root});
  if("declined" in planned)throw Error(JSON.stringify(planned));
  const names=[...planned.files.keys()];
  expect(names.some(name=>name.endsWith(`${stem}.rlib`))&&names.some(name=>name.endsWith(`${stem}.rmeta`)),extension).toBe(true);
 }
});

test("a worker that runs an older action format is no candidate: the action asks for the format and only a worker that offers it matches",()=>{
 const identity=parseRustcIdentityV1("commit-hash: "+"a".repeat(40)+"\nhost: aarch64-apple-darwin\n")!,host=[{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"}];
 const required=remoteExecutionMatchPropertiesV1(offloadPlatformV1(host,{...identity,targets:[]},undefined,"g1-m03")),base=[...host,{name:"semio-toolchain",value:identity.digest},{name:"semio-target",value:identity.host}];
 expect(required.some(row=>row.name===OFFLOAD_POLICY_V1.property.format&&row.value===OFFLOAD_POLICY_V1.property.formatValue)).toBe(true);
 expect(remoteExecutionPlatformMatchesV1(required,base),"an older worker offers no format").toBe(false);
 expect(remoteExecutionPlatformMatchesV1(required,[...base,{name:OFFLOAD_POLICY_V1.property.format,value:OFFLOAD_POLICY_V1.property.formatValue}])).toBe(true);
});

test("the door and the submission check share one truth: the same machines are counted by the scheduler's own idle counting whether it is asked by the door or at submission, including a requester that is never its own candidate and a machine that has not reported yet",async()=>{
 const environment={SEMIO_WORKSPACE_ROOT:realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"truth-"))),SEMIO_REMOTE_EXECUTION_ENDPOINT:"grpc://192.0.2.1:8980"};
 const rows=[peer({origin:"a"}),peer({origin:"b",headroom:false}),peer({origin:"c",silent:true}),peer({origin:"d",toolchain:"bbbbbbbbbbbbbbbb"})];
 expect(await fleetHeadroomV1(environment,{fleet:schedulerOf(rows)}),"a, and c (no report counts as headroom); b has none, d has another toolchain").toBe(2);
 expect(await fleetHeadroomV1(environment,{fleet:schedulerOf(rows.map(row=>({...row,origin:row.origin==="a"?"requester-is-a-worker":row.origin})))})).toBe(2);
});

test("the environment of an offload test never names the checkout that runs it: its fleet facts would supersede the test cluster's endpoint and the test would write the live fleet's cache and counters",()=>{
 const named=testRustEnvironmentV1({SEMIO_WORKSPACE_ROOT:"/elsewhere"});
 expect(named.SEMIO_WORKSPACE_ROOT).toBe("/elsewhere");
 const previous=process.env.SEMIO_WORKSPACE_ROOT,previousUnit=process.env.SEMIO_UNIT_CACHE_WORKSPACE_ROOT;
 process.env.SEMIO_WORKSPACE_ROOT="/checkout";process.env.SEMIO_UNIT_CACHE_WORKSPACE_ROOT="/checkout";
 try{const environment=testRustEnvironmentV1();expect(environment.SEMIO_WORKSPACE_ROOT).toBeUndefined();expect(environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT).toBeUndefined();}
 finally{if(previous===undefined)delete process.env.SEMIO_WORKSPACE_ROOT;else process.env.SEMIO_WORKSPACE_ROOT=previous;if(previousUnit===undefined)delete process.env.SEMIO_UNIT_CACHE_WORKSPACE_ROOT;else process.env.SEMIO_UNIT_CACHE_WORKSPACE_ROOT=previousUnit;}
});

test("a module file named by a cfg_attr path (mio, errno, memmap2) is shipped: the files that rustc itself reads for the unit are all reached",()=>{
 const root=mkdtempSync(join(tmpdir(),"offload-cfg-attr-"));
 try{
  mkdirSync(join(root,"sel"));
  writeFileSync(join(root,"lib.rs"),'#[cfg_attr(\n    any(target_os = "macos", target_os = "linux", target_os = "windows"),\n    path = "sel/native.rs"\n)]\n#[cfg_attr(not(any(target_os = "macos", target_os = "linux", target_os = "windows")), path = "sel/other.rs")]\nmod selector;\npub fn f() -> u32 { selector::v() }\n');
  for(const name of ["native","other"])writeFileSync(join(root,"sel",`${name}.rs`),"pub fn v() -> u32 { 1 }\n");
  const info=join(root,"lib.d"),done=spawnSync("rustc",["--crate-type=lib","--crate-name=lib",`--emit=dep-info=${info}`,join(root,"lib.rs")],{encoding:"utf8",env:testRustEnvironmentV1()});
  expect(done.status,done.stderr).toBe(0);
  const read=readFileSync(info,"utf8").split("\n")[0]!.split(": ")[1]!.split(" ").filter(path=>path!==""&&path!==join(root,"lib.rs"));
  expect(read.length).toBe(1);
  const reached=includedSourcesV1(join(root,"lib.rs"));
  for(const path of read)expect(reached).toContain(path);
  expect(reached).toContain(join(root,"sel","native.rs"));
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("an environment variable a build script points into OUT_DIR (cranelift's ISLE_DIR) travels as the virtual generated path, not as the requester's path, and the worker command spells it once",()=>{
 const checkout=chain("generated-env"),generated=join(checkout.base,"gen"),virtual=OFFLOAD_POLICY_V1.virtual.generated;
 mkdirSync(join(generated,"sub"),{recursive:true});writeFileSync(join(generated,"isle.rs"),"//\n");writeFileSync(join(generated,"sub","more.rs"),"//\n");
 const planned=(extra:Record<string,string>)=>planOffloadV1({args:checkout.arguments("c",["b"]),environment:{...checkout.environment("c"),OUT_DIR:generated,...extra},cwd:checkout.root});
 const result=planned({ISLE_DIR:generated,ISLE_FILE:join(generated,"sub","more.rs")});
 if("declined" in result)throw Error(result.declined+" "+result.detail);
 expect(result.environment.SEMIO_ABSOLUTE.split(" ")).toEqual(expect.arrayContaining(["ISLE_DIR","ISLE_FILE","OUT_DIR"]));
 expect(result.environment.SEMIO_REL_ISLE_DIR).toBe(virtual);expect(result.environment.SEMIO_REL_ISLE_FILE).toBe(`${virtual}/sub/more.rs`);
 expect([...result.files.keys()]).toEqual(expect.arrayContaining([`${virtual}/isle.rs`,`${virtual}/sub/more.rs`]));
 const command=offloadRustcCommandV1(result,"aarch64-apple-darwin");
 for(const name of ["ISLE_DIR","ISLE_FILE","OUT_DIR"]){
  const rows=command.environment.filter(([key])=>key===name);expect(rows.length,name).toBe(1);expect(rows[0]![1].includes(checkout.base),name).toBe(false);expect(rows[0]![1].startsWith("@root/")||rows[0]![1]==="@root","@root placeholder "+name).toBe(true);
 }
 const unmapped=planned({STRAY:generated+"-sibling"});
 expect("declined" in unmapped?unmapped.declined:"planned").toBe("unmapped-path");
});

test("include_* (plain, raw, concat, include_wgsl) reaches every file rustc reads, and planOffloadV1 of that registry crate ships blit.wgsl plus a non-Rust src file a custom shader_file! macro reads",()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-wgsl-")));
 try{
  mkdirSync(join(root,"src"));
  writeFileSync(join(root,"Cargo.toml"),"[package]\nname = \"wgpu\"\nversion = \"0.1.0\"\n");
  const lib=(extra:string)=>'macro_rules! include_wgsl { ($p:literal) => { include_str!($p) }; }\nmacro_rules! shader_file { ($p:literal) => { include_str!($p) }; }\nmod blit;\npub const RAW: &str = include_str!(r#"raw.txt"#);\npub const HASHED: &str = include_str!(r##"hashed.wgsl"##);\npub const PACKED: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/packed.bin"));\n'+extra;
  writeFileSync(join(root,"src","lib.rs"),lib(""));
  writeFileSync(join(root,"src","blit.rs"),'pub const SHADER: &str = include_wgsl!("blit.wgsl");\n');
  writeFileSync(join(root,"src","blit.wgsl"),"@compute fn blit() {}\n");
  writeFileSync(join(root,"src","raw.txt"),"raw\n");
  writeFileSync(join(root,"src","hashed.wgsl"),"@fragment fn hashed() {}\n");
  writeFileSync(join(root,"packed.bin"),"pk");
  const entry=join(root,"src","lib.rs"),info=join(root,"lib.d"),environment=testRustEnvironmentV1({CARGO_MANIFEST_DIR:root});
  const compile=()=>spawnSync("rustc",["--crate-type=lib","--crate-name=lib",`--emit=dep-info=${info}`,entry],{encoding:"utf8",cwd:root,env:environment});
  const dependencies=()=>parseDepInfoV1(readFileSync(info,"utf8").replace(/\\\r?\n/gu," ")).dependencies.map(path=>isAbsolute(path)?path:join(root,path));
  const oracle=compile();
  expect(oracle.status,oracle.stderr).toBe(0);
  const reached=includedSourcesV1(entry,undefined,undefined,root);
  for(const path of dependencies())if(path!==entry)expect(reached,path).toContain(path);
  expect(reached).toContain(join(root,"src","blit.wgsl"));
  writeFileSync(join(root,"src","extra.wgsl"),"@compute fn extra() {}\n");
  writeFileSync(entry,lib('pub const EXTRA: &str = shader_file!("extra.wgsl");\n'));
  const again=compile();
  expect(again.status,again.stderr).toBe(0);
  expect(dependencies()).toContain(join(root,"src","extra.wgsl"));
  const planned=planOffloadV1({args:["--crate-name","wgpu","--edition=2021","src/lib.rs","--crate-type","lib","--emit=metadata","--out-dir","out","--remap-path-prefix",`${root}=.`],environment:{CARGO_MANIFEST_DIR:root,CARGO_PKG_NAME:"wgpu",CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:"wgpu"},cwd:root});
  if("declined" in planned)throw Error(planned.declined+" "+planned.detail);
  const shipped=[...planned.files.values()];
  expect(shipped).toContain(join(root,"src","blit.wgsl"));
  expect(shipped).toContain(join(root,"src","extra.wgsl"));
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a unit whose extern is semio_os_kernel_dsl_derive stays local: planOffloadV1 declines it unmapped-path",()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-derive-")));
 try{
  mkdirSync(join(root,"src"));
  writeFileSync(join(root,"src","lib.rs"),"pub fn f() {}\n");
  writeFileSync(join(root,"Cargo.toml"),"[package]\nname = \"caller\"\nversion = \"0.1.0\"\n");
  const environment={CARGO_MANIFEST_DIR:root,CARGO_PKG_NAME:"caller",CARGO_PKG_VERSION:"0.1.0",CARGO_CRATE_NAME:"caller"};
  const base=["--crate-name","caller","--edition=2021","src/lib.rs","--crate-type","lib","--emit=metadata","--out-dir","out","--remap-path-prefix",`${root}=.`];
  const declined=(externSpec:string)=>{const result=planOffloadV1({args:[...base,"--extern",externSpec],environment,cwd:root});if(!("declined" in result))return "planned";expect(result.detail).toContain("proc-macro source authority");return result.declined;};
  expect(declined(`semio_os_kernel_dsl_derive=${join(root,"libsemio_os_kernel_dsl_derive.dylib")}`)).toBe("unmapped-path");
  expect(declined(`other=${join(root,"deps","libsemio_os_kernel_dsl_derive-hash.dylib")}`)).toBe("unmapped-path");
 }finally{rmSync(root,{recursive:true,force:true});}
});
