import deletionSchema from "../🧫️fixtures/🏗️native/🧬️schema/🔣️.json";
import capacitySchema from "../../🎟️capacity/🧬️schema/🔣️.json";
import { compilerCapacityWorkerCountV1, acquireCompilerCapacityLeaseV1,phaseCompilerAdmissionControlV1 } from "../../🎟️capacity/🟦️.ts";
import { build } from "esbuild";
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { runOwnedCommand } from "../../../🎛️owned-execution/🟦️.ts";
import { readFileSync, mkdtempSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🏗️native/🔣️.json";
import schema from "../../../📦️artifacts/🏗️native-build/🧬️schema/🔣️.json";
import { buildCargoArtifacts, cargoArtifactArgumentsV1, cargoConfigurationArgumentsV1, readCargoArtifactBuildPolicyV1 } from "../../../📦️artifacts/🏗️native-build/🟦️.ts";
const root = resolve(import.meta.dir,"../../../../../..");
test("Native mode policy admission agrees with an independent schema oracle", () => {
 const validate = new Ajv({strict:true}).compile(schema);
 const base = { version:1,cwd:"/owner",buildDirectory:"/compiler",leaseDirectory:"/leases",captureDirectory:"/captures",budgetMs:0 };
 for (const mode of ["prototype","release","dev","ship",undefined]) {
  const policy = {...base,...(mode===undefined?{}:{mode})};
  const accepted = mode==="prototype" || mode==="release";
  expect(validate(policy)).toBe(accepted);
  const read = () => readCargoArtifactBuildPolicyV1({SEMIO_CARGO_ARTIFACT_POLICY:JSON.stringify(policy)},"/owner");
  if(accepted) expect(JSON.parse(JSON.stringify(read()))).toEqual(policy); else expect(read).toThrow();
  for(const configuration of ["/configuration.toml","relative.toml"]){const configured={...policy,compilerConfiguration:configuration};expect(validate(configured)).toBe(accepted&&configuration.startsWith("/"));if(accepted&&configuration.startsWith("/"))expect(JSON.parse(JSON.stringify(readCargoArtifactBuildPolicyV1({SEMIO_CARGO_ARTIFACT_POLICY:JSON.stringify(configured)},"/owner")))).toEqual(configured);}
 }
});




test("All-core capacity queues independent builders and admits inherited children", async () => {
 const directory = process.env.SEMIO_TEST_ARTIFACT_DIR!, signal = new AbortController().signal;
 const events:string[]=[];
 const first = await acquireCompilerCapacityLeaseV1({directory,control:phaseCompilerAdmissionControlV1(signal,30000,()=>undefined,()=>new Promise(done=>setImmediate(done))),environment:{}}); events.push("first-held");
 expect(new Ajv({strict:true}).compile(capacitySchema)({resource:first.resource,mode:first.mode,parallelism:first.parallelism})).toBe(true);
 for(const value of fixture.capacity.invalidWorkerBudgets)expect(()=>compilerCapacityWorkerCountV1({SEMIO_COMPILER_CAPACITY_HELD:"true",...(value===null?{}:{SEMIO_COMPILER_PARALLELISM:value})})).toThrow();
 const inherited = await acquireCompilerCapacityLeaseV1({directory,control:phaseCompilerAdmissionControlV1(signal,30000,()=>undefined,()=>new Promise(done=>setImmediate(done))),environment:{SEMIO_COMPILER_CAPACITY_HELD:"true",SEMIO_COMPILER_PARALLELISM:"1"}}); inherited.release();
 let observed!:()=>void; const waiting = new Promise<void>(accept=>{observed=accept;});
 const pending = acquireCompilerCapacityLeaseV1({directory,control:phaseCompilerAdmissionControlV1(signal,30000,()=>{if(!events.includes("second-waiting"))events.push("second-waiting");observed();},()=>new Promise(done=>setImmediate(done))),environment:{}});
 await waiting; events.push("first-released"); first.release();
 const second = await pending; events.push("second-held");
 expect(events).toEqual(fixture.capacity.events);
 const controller = new AbortController();
 const cancelled = acquireCompilerCapacityLeaseV1({directory,control:phaseCompilerAdmissionControlV1(controller.signal,30000,()=>controller.abort(),()=>new Promise(done=>setImmediate(done))),environment:{}});
 await expect(cancelled).rejects.toThrow(); second.release();
 console.log(`[DEBUG] native capacity events=${JSON.stringify(events)}`);
},120000);
test("CMake compiles both mode configurations and reuses warm Ninja outputs", async () => {
 const directory = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"cmake-"));
 writeFileSync(join(directory,"CMakeLists.txt"),'cmake_minimum_required(VERSION 3.28)\nproject(native_mode LANGUAGES CXX)\nadd_executable(native_mode main.cpp)\n');
 writeFileSync(join(directory,"main.cpp"),fixture.compiler.source);
 const signal = new AbortController().signal;
 for (const row of fixture.modes) {
  const output = join(directory,row.mode);
  await runOwnedCommand("cmake",["-S",directory,"-B",output,"-G","Ninja",`-DCMAKE_BUILD_TYPE=${row.configuration}`],directory,`native:${row.mode}:configure`,30000,{signal});
  await runOwnedCommand("cmake",["--build",output],directory,`native:${row.mode}:build`,30000,{signal});
  const binary = join(output,`native_mode${process.platform==="win32"?".exe":""}`);
  const runtime = spawnSync(binary,[],{encoding:"utf8"}); expect(runtime.status).toBe(0); expect(runtime.stdout).toBe(fixture.compiler.stdout);
  const warm = spawnSync("cmake",["--build",output],{encoding:"utf8"}); expect(warm.status).toBe(0); expect(warm.stdout).toContain("no work to do");
  console.log(`[DEBUG] CMake mode=${row.mode} runtime=${runtime.stdout} warm=${warm.stdout.trim()}`);
 }
},90000);



test("Cargo modes publish separate executable owners and reuse warm compiler intermediates", async () => {
 const directory = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"m-")), manifest = join(directory,"Cargo.toml");
 writeFileSync(manifest,`[workspace]\n[package]\nname="${fixture.cargo.name}"\nversion="0.1.0"\nedition="2021"\n[[bin]]\nname="${fixture.cargo.name}"\npath="main.rs"\n[profile.prototype]\ninherits="dev"\nincremental=true\n[profile.release]\nlto="thin"\ncodegen-units=1\n`);
 writeFileSync(join(directory,"main.rs"),fixture.cargo.source);
 expect(spawnSync("cargo",["generate-lockfile","--offline","--manifest-path",manifest],{cwd:directory,encoding:"utf8"}).status).toBe(0);
 for(const row of fixture.modes) {
  writeFileSync(join(directory,"main.rs"),fixture.cargo.source);
  const policy = {version:1 as const,mode:row.mode as "prototype"|"release",compilerConfiguration:fixtureConfigurationV1(directory),cwd:directory,buildDirectory:join(directory,"b"),leaseDirectory:process.env.SEMIO_TEST_ARTIFACT_DIR!,captureDirectory:join(directory,"c"),budgetMs:30000};
  let receipt = "";
  const times:number[]=[];
  for(const warm of [false,true]) {
   const started = performance.now();
   await buildCargoArtifacts(manifest,["--bin",fixture.cargo.name],policy,{control:fixtureAdmissionV1(),provenance:path=>{receipt=path;}});times.push(Math.round(performance.now()-started));
   const evidence=JSON.parse(readFileSync(receipt,"utf8")), unit=evidence.units.find((unit:any)=>unit.message.target.kind.includes("bin"));
   expect(unit.message.profile.opt_level).toBe(row.optLevel);expect(unit.message.fresh).toBe(warm);
   const binary=join(directory,"dist/build",row.mode,`${fixture.cargo.name}${process.platform==="win32"?".exe":""}`),runtime=spawnSync(binary,[],{encoding:"utf8"});expect(runtime.status).toBe(0);expect(runtime.stdout).toBe(fixture.cargo.stdout);
  }
  const oracle=spawnSync("cargo",["run","--offline","--quiet","--manifest-path",manifest,"--profile",row.nativeProfile,"--target-dir",join(directory,"oracle")],{cwd:directory,encoding:"utf8",env:{...process.env,CARGO_BUILD_BUILD_DIR:join(directory,"ob")},timeout:30000});expect(oracle.status).toBe(0);expect(oracle.stdout).toBe(fixture.cargo.stdout);
  writeFileSync(join(directory,"main.rs"),fixture.cargo.source.replace("runtime-ok","edited-runtime-ok"));
  const editedStarted = performance.now();await buildCargoArtifacts(manifest,["--bin",fixture.cargo.name],policy,{control:fixtureAdmissionV1(),provenance:path=>{receipt=path;}});
  const editedMs = Math.round(performance.now()-editedStarted), editedReceipt=JSON.parse(readFileSync(receipt,"utf8"));expect(editedReceipt.units.find((unit:any)=>unit.message.target.kind.includes("bin")).message.fresh).toBe(false);
  const edited=spawnSync(join(directory,"dist/build",row.mode,`${fixture.cargo.name}${process.platform==="win32"?".exe":""}`),[],{encoding:"utf8"});expect(edited.status).toBe(0);expect(edited.stdout).toBe(fixture.cargo.stdout.replace("runtime-ok","edited-runtime-ok"));
  console.log(`[DEBUG] Cargo mode=${row.mode} runtime=${oracle.stdout} coldMs=${times[0]} warmMs=${times[1]} editedMs=${editedMs} editedRuntime=${edited.stdout}`);
 }
},120000);




test("Reserved Cargo children consume their admitted compiler worker count",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR!,owner=mkdtempSync(join(output,"j-")),signal=new AbortController().signal,host=await acquireCompilerCapacityLeaseV1({directory:output,control:phaseCompilerAdmissionControlV1(signal,30000,()=>undefined,()=>new Promise(done=>setImmediate(done))),environment:{}});
 const count=fixture.capacity.reservedCargoWorkers;
 writeFileSync(join(owner,"Cargo.toml"),'[package]\nname="reserved_workers"\nversion="0.1.0"\nedition="2021"\nbuild="build.rs"\n[[bin]]\nname="reserved_workers"\npath="main.rs"\n[workspace]\n[profile.prototype]\ninherits="dev"\n');
 writeFileSync(join(owner,"build.rs"),'fn main(){ println!("cargo::rustc-env=OBSERVED_WORKERS={}",std::env::var("NUM_JOBS").unwrap()); }');writeFileSync(join(owner,"main.rs"),'fn main(){print!("{}",env!("OBSERVED_WORKERS"));}');
 const lock=spawnSync("cargo",["generate-lockfile","--offline"],{cwd:owner,encoding:"utf8"});expect(lock.status,lock.stderr).toBe(0);
 const environment:Record<string,string|undefined>={...process.env,SEMIO_BUILD_MODE:"prototype",SEMIO_COMPILER_CAPACITY_HELD:"true",SEMIO_COMPILER_PARALLELISM:String(count)};delete environment.MAKEFLAGS;delete environment.CARGO_MAKEFLAGS;delete environment.CARGO_BUILD_JOBS;
 try{
  await buildCargoArtifacts(join(owner,"Cargo.toml"),[],{version:1,mode:"prototype",cwd:owner,buildDirectory:join(owner,"b"),captureDirectory:output,leaseDirectory:output,budgetMs:120_000},{environment,control:fixtureAdmissionV1(signal)});
  const executable=join(owner,"dist/build/prototype/reserved_workers"+(process.platform==="win32"?".exe":""));expect(spawnSync(executable,[],{encoding:"utf8"}).stdout).toBe(String(count));
  const oracle=spawnSync("cargo",["run","--quiet","--profile","prototype","-j",String(count),"--target-dir",join(owner,"oracle")],{cwd:owner,env:environment,encoding:"utf8"});expect(oracle.status,oracle.stderr).toBe(0);expect(oracle.stdout).toBe(String(count));console.log(`[DEBUG] Reserved Cargo build-script NUM_JOBS=${count}; Cargo -j oracle agrees`);
 }finally{host.release();}
},120_000);

test("Central Cargo profile policy compiles independent workspaces without local profile tables",async()=>{
 const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"policy-")),manifest=join(directory,"Cargo.toml"),configuration=fixtureConfigurationV1(directory);
 writeFileSync(manifest,'[workspace]\n[package]\nname="central_profile_fixture"\nversion="0.0.0"\nedition="2021"\n[[bin]]\nname="central_profile_fixture"\npath="main.rs"\n');writeFileSync(join(directory,"main.rs"),'fn main(){print!("{}",cfg!(debug_assertions));}');writeFileSync(join(directory,"Cargo.lock"),'version=4\n[[package]]\nname="central_profile_fixture"\nversion="0.0.0"\n');
 for(const row of fixture.modes){
 const policy={version:1 as const,mode:row.mode as "prototype"|"release",compilerConfiguration:configuration,cwd:directory,buildDirectory:join(directory,"b"),leaseDirectory:process.env.SEMIO_TEST_ARTIFACT_DIR!,captureDirectory:directory,budgetMs:120000};
 await buildCargoArtifacts(manifest,["--bin","central_profile_fixture"],policy,{control:fixtureAdmissionV1(),environment:{...process.env,RUSTFLAGS:"-Zthreads=0"}});
 const binary=join(directory,"dist/build",row.mode,"central_profile_fixture"+(process.platform==="win32"?".exe":"")),result=spawnSync(binary,[],{encoding:"utf8"});expect(result.status).toBe(0);expect(result.stdout).toBe(row.mode==="prototype"?"true":"false");console.log(`[DEBUG] Independent Cargo workspace central profile=${row.mode} rustc-threads=0 runtime=${result.stdout}`);
 }
 const text=readFileSync(configuration,"utf8"),config=Bun.TOML.parse(text) as any,{parse}=await import("@iarna/toml");expect(config).toEqual(parse(text));expect(config.profile.prototype).toMatchObject(fixture.centralProfiles.prototype);expect(config.profile.release).toMatchObject(fixture.centralProfiles.release);
},120000);

test("Cargo capture and relay compiler arguments declare the selected mode and configuration",async()=>{for(const row of fixture.compilerArguments){expect(cargoArtifactArgumentsV1(row.input,{mode:row.mode as "prototype"|"release",compilerConfiguration:"/configuration.toml",buildDirectory:"/compiler"})).toEqual(row.expected);expect(()=>cargoArtifactArgumentsV1(["build","--profile",row.mode==="prototype"?"release":"prototype"],{mode:row.mode as "prototype"|"release",buildDirectory:"/compiler"})).toThrow();}const{parse}=await import("@iarna/toml");for(const row of fixture.compilerStorageArguments){const actual=cargoConfigurationArgumentsV1(row.input,row.configuration??undefined,row.buildDirectory);expect(actual).toEqual(row.expected);const delimiter=actual.indexOf("--"),head=delimiter<0?actual:actual.slice(0,delimiter),override=head.at(-1)!;expect(parse(override)).toEqual({build:{"build-dir":row.buildDirectory}});expect(Bun.TOML.parse(override)).toEqual(parse(override));}});
test("Cargo capture and relay compiler arguments have registered source and actual profile launch entries",async()=>{const{parse}=await import("jsonc-parser");for(const path of[".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){const errors:any[]=[],launch=parse(readFileSync(resolve(root,path),"utf8"),errors,{allowTrailingComma:path.endsWith("jsonc")});expect(errors).toEqual([]);for(const name of["🦑️Repo Actual Cargo Overlap Prototype Acceptance","🦑️Repo Actual Cargo Overlap Release Acceptance","🦑️Repo Cargo Compiler Storage Source Acceptance"])expect(launch.configurations.filter((row:any)=>row.name===name).length).toBe(1);}});

/** 🎛️ Owns this fixture compiler phase with its actual signal and fixed test deadline. */
function fixtureAdmissionV1(signal:AbortSignal=new AbortController().signal){return phaseCompilerAdmissionControlV1(signal,120000,()=>undefined,()=>new Promise(done=>setImmediate(done)));}

test("General native owners bundle when every product owner is refused",async()=>{
 expect(new Ajv({strict:true}).compile(deletionSchema)(fixture.deletion)).toBe(true);
 for(const entry of fixture.deletion.entries){
  const compiled=await build({absWorkingDir:root,entryPoints:[entry],bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,logLevel:"silent",plugins:[{name:"refuse-products",setup(builder){builder.onResolve({filter:/.*/},args=>{if(args.path.startsWith(".")&&resolve(args.resolveDir,args.path).includes(fixture.deletion.refusedOwner))throw Error("Deleted product owner reached "+args.path);return undefined;});}}]});
  expect(Object.keys(compiled.metafile!.inputs).some(input=>input.includes(fixture.deletion.refusedOwner))).toBe(false);
 }
 console.log("[DEBUG] General native scripts and native laws survive refused product owners");
});

/** 📋️ Authors the compiler policy inside the independent General specimen. */
function fixtureConfigurationV1(directory:string):string{const path=join(directory,"compiler.toml");writeFileSync(path,fixture.deletion.compilerConfiguration);return path;}
