import {expect,test} from "bun:test";
import Ajv from "ajv";
import {isPlatformRowV1,platformClassV1,PLATFORM_CLASS_POLICY_V1,splitNxPlatformInputsV1,type PlatformProjectV1} from "../🟦️.ts";
import {projectBuildGraphForNxV1} from "../../../../🟨️.mjs";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const PLATFORM={runtime:PLATFORM_CLASS_POLICY_V1.canonicalRuntime},OLD={runtime:'node -p "process.platform.concat(process.arch)"'};
const project=(targets:Record<string,Record<string,unknown>>,extra:Record<string,readonly unknown[]>={}):PlatformProjectV1=>({namedInputs:{default:["{projectRoot}/**/*","sharedGlobals"],sharedGlobals:[PLATFORM,{runtime:"repo tools exec --light bun -- --version"},{env:"SEMIO_BUILD_MODE"}],nativeSources:["{projectRoot}/src/**/*.rs"],...extra},targets:targets as PlatformProjectV1["targets"]});
const reaches=(projects:Record<string,PlatformProjectV1>,owner:string,name:string)=>platformClassV1(name,projects[owner]!.targets![name]!,owner,projects).reachesPlatform;

test("the policy and a verdict are closed by the schema (Ajv as the independent validator)",()=>{
 const ajv=new Ajv({strict:false}),validate=ajv.compile({$ref:"#/definitions/Verdict",definitions:schema.definitions});
 expect(validate({class:"independent",reason:"kind build"})).toBe(true);expect(validate({class:"both",reason:"x"})).toBe(false);expect(validate({class:"platform"})).toBe(false);
 const {reachesPlatform:_reached,...verdict}=platformClassV1("build",{inputs:["default"]},"a",{a:project({})});expect(validate(verdict)).toBe(true);
 expect(PLATFORM_CLASS_POLICY_V1.platformRuntimes).toContain(PLATFORM_CLASS_POLICY_V1.canonicalRuntime);
 expect(isPlatformRowV1(PLATFORM)).toBe(true);expect(isPlatformRowV1(OLD)).toBe(true);expect(isPlatformRowV1({runtime:"bun --version"})).toBe(false);expect(isPlatformRowV1("default")).toBe(false);
});

test("a native-free TypeScript build, lint and generator are independent; a test, a native target and a toolchain target keep the platform",()=>{
 const options={command:"repo run.ts build"},projects={a:project({build:{inputs:["default","^default"],options},lint:{inputs:["default"],options},"🏭️generator":{inputs:["default"],options},test:{inputs:["default"],options},native:{inputs:["default"],options:{command:"x",nativeOwnerCommand:"y"}},cargoish:{inputs:["default"],options:{command:"repo native cargo build"}},check:{inputs:["default","nativeSources"],options}})};
 const verdicts=Object.fromEntries(Object.keys(projects.a.targets!).map(name=>[name,platformClassV1(name,projects.a.targets![name]!,"a",projects).class]));
 expect(verdicts).toEqual({build:"independent",lint:"independent","🏭️generator":"independent",test:"platform",native:"platform",cargoish:"platform",check:"platform"});
});

test("after the split an independent target reaches no platform row anywhere and a platform target keeps exactly one, at its own inputs",()=>{
 const options={command:"repo run.ts"},before={a:project({build:{inputs:["default","^default"],options},test:{inputs:["default"],options},"🔌️wasm":{inputs:["default"],options}})};
 const after=splitNxPlatformInputsV1(before);
 expect(reaches(before,"a","build")).toBe(true);expect(reaches(after,"a","build")).toBe(false);
 expect(after.a.targets!.build!.inputs).toEqual(["default","^default"]);
 expect(after.a.targets!.test!.inputs).toEqual(["default",PLATFORM]);expect(reaches(after,"a","test")).toBe(true);
 expect(JSON.stringify(after.a.namedInputs)).not.toContain("process.platform");
 expect(JSON.stringify(after.a.targets!.build)).not.toContain("process.platform");
 expect(after.a.targets!["🔌️wasm"]!.inputs).toEqual(["default",PLATFORM]);
});

test("the old spelling of the platform row is taken out of an explicit input list as well and re-added to a platform target as the canonical row",()=>{
 const options={command:"repo run.ts"},before={a:project({build:{inputs:["{projectRoot}/**/*",OLD],options},package:{inputs:["{projectRoot}/**/*",OLD],options}})},after=splitNxPlatformInputsV1(before);
 expect(after.a.targets!.build!.inputs).toEqual(["{projectRoot}/**/*"]);expect(after.a.targets!.package!.inputs).toEqual(["{projectRoot}/**/*",PLATFORM]);
});

test("a toolchain runtime, a native input reached through a nested group or another project, a native command and a store-bound kind keep the platform",()=>{
 const options={command:"repo run.ts"},nested=project({build:{inputs:["viaGroup"],options},cross:{inputs:[{input:"toolchain",projects:["b"]}],options},rt:{inputs:["default",{runtime:"rustc -vV"}],options},env:{inputs:["default",{env:"CARGO_HOME"}],options},cmd:{inputs:["default"],options:{command:"cargo fetch"}},comp:{inputs:["default"],options,metadata:{semio:{componentStoreV1:{version:1}}}}},{viaGroup:["default","inner"],inner:["nativeTestSources"]});
 const projects={a:nested,b:project({},{toolchain:["cargoToolchain"]})};
 for(const name of["build","cross","rt","env","cmd","comp"])expect(platformClassV1(name,projects.a.targets![name]!,"a",projects).class,name).toBe("platform");
 expect(platformClassV1("build",projects.a.targets!.build!,"a",projects).reason).toContain("nativeTestSources");
 expect(platformClassV1("rt",projects.a.targets!.rt!,"a",projects).reason).toContain("rustc");
});

test("a declaration in the target metadata overrides the kind list for an independent kind and always wins for a bound one",()=>{
 const options={command:"repo run.ts"},projects={a:project({weird:{inputs:["default"],options,metadata:{semio:{platform:"independent"}}},build:{inputs:["default"],options,metadata:{semio:{platform:"bound"}}},"declared-native":{inputs:["default"],options:{command:"cargo build"},metadata:{semio:{platform:"independent"}}}})};
 expect(platformClassV1("weird",projects.a.targets!.weird!,"a",projects).class).toBe("independent");
 expect(platformClassV1("build",projects.a.targets!.build!,"a",projects).class).toBe("platform");
 expect(platformClassV1("declared-native",projects.a.targets!["declared-native"]!,"a",projects).class,"a declaration never overrides a toolchain command").toBe("platform");
});

test("the split is idempotent, keeps untouched projects and targets by identity and never adds a platform row to a target that had none",()=>{
 const options={command:"repo run.ts"},quiet={namedInputs:{default:["{projectRoot}/**/*"]},targets:{noop:{options}}} as PlatformProjectV1,projects={a:project({build:{inputs:["default"],options},test:{inputs:["default"],options}}),q:quiet};
 const once=splitNxPlatformInputsV1(projects),twice=splitNxPlatformInputsV1(once);
 expect(twice).toEqual(once);expect(once.q!.targets!.noop).toBe(quiet.targets!.noop);expect(once.q!.targets!.noop!.inputs).toBeUndefined();
});

test("an independently written resolver agrees with the class on 300 seeded random projects (a second implementation)",()=>{
 let seed=20261115;const next=(n:number)=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed%n;};
 const atoms=["{projectRoot}/**/*","sharedGlobals","nativeSources","cargoToolchain","extra",{runtime:"bun --version"},{runtime:"rustc -vV"},{env:"SEMIO_BUILD_MODE"},{env:"RUSTFLAGS"},PLATFORM,OLD];
 const names=["build","lint","test","check","package","wasm","typecheck"];
 const naive=(projects:Record<string,PlatformProjectV1>,owner:string,list:readonly unknown[],seen=new Set<string>()):{bound:boolean;platform:boolean}=>{
  let bound=false,platform=false;
  for(const item of list){
   if(typeof item==="string"){const n=item.replace(/^[!^]+/u,"");if(["nativeSources","nativeTestSources","cargoToolchain","go","uv","dotnet"].includes(n))bound=true;else if(projects[owner]!.namedInputs![n]&&!seen.has(n)){const r=naive(projects,owner,projects[owner]!.namedInputs![n]!,new Set([...seen,n]));bound||=r.bound;platform||=r.platform;}}
   else if(JSON.stringify(item)===JSON.stringify(PLATFORM)||JSON.stringify(item)===JSON.stringify(OLD))platform=true;
   else if((item as {runtime?:string}).runtime==="rustc -vV"||(item as {env?:string}).env==="RUSTFLAGS")bound=true;
  }
  return {bound,platform};
 };
 for(let round=0;round<300;round++){
  const pick=()=>Array.from({length:1+next(4)},()=>atoms[next(atoms.length)]!),targets:Record<string,Record<string,unknown>>={};
  for(let i=0;i<1+next(4);i++)targets[names[next(names.length)]!]={inputs:next(3)===0?undefined:["default",...pick().filter((_,at)=>at<2)],options:{command:"repo run.ts"}};
  const one={a:{namedInputs:{default:["{projectRoot}/**/*","sharedGlobals"],sharedGlobals:[PLATFORM,{runtime:"bun --version"}],extra:pick()},targets:targets as PlatformProjectV1["targets"]}} as Record<string,PlatformProjectV1>;
  for(const [name,target] of Object.entries(one.a!.targets!)){
   const verdict=platformClassV1(name,target,"a",one),r=naive(one,"a",target.inputs??["default","^default"]),listed=["build","check","typecheck","lint"].includes(name);
   expect(verdict.class,JSON.stringify([name,target.inputs])).toBe(!r.bound&&listed?"independent":"platform");expect(verdict.reachesPlatform).toBe(r.platform);
  }
  const after=splitNxPlatformInputsV1(one);
  for(const [name,target] of Object.entries(after.a!.targets!)){const verdict=platformClassV1(name,target,"a",after);if(verdict.class==="independent")expect(verdict.reachesPlatform).toBe(false);}
 }
});

test("the Nx projection of a portable graph drops the platform from independent kinds and keeps it for the rest",()=>{
 const target=(extra:Record<string,unknown>={})=>({execution:{kind:"command",command:"repo run.ts"},cache:true,outputs:["{workspaceRoot}/.cache/b/p/m/x"],inputs:["default","^default"],...extra});
 const graph={namedInputs:{default:["{projectRoot}/**/*","sharedGlobals"],sharedGlobals:[PLATFORM,{runtime:"repo tools exec --light bun -- --version"}]},projects:{a:{root:"libs/a",targets:{build:target(),test:target(),lint:target()}}},externalNodes:{}};
 const nx=projectBuildGraphForNxV1(graph as never) as {projects:Record<string,PlatformProjectV1>};
 expect(JSON.stringify(nx.projects.a!.namedInputs)).not.toContain("process.platform");
 const text=(name:string)=>JSON.stringify(nx.projects.a!.targets![name]!.inputs);
 expect(text("build")).not.toContain("process.platform");expect(text("lint")).not.toContain("process.platform");expect(text("test")).toContain("process.platform");
});
