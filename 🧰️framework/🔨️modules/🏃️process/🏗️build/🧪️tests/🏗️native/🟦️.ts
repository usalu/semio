import { build } from "esbuild";
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { acquireNativeCapacityLeaseV1 } from "../../../📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import { runOwnedCommand } from "../../../🎛️owned-execution/🟦️.ts";
import { readFileSync, mkdtempSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🏗️native/🔣️.json";
import schema from "../../../📦️artifacts/🏗️native-build/🧬️schema/🔣️.json";
import { buildCargoArtifacts, readCargoArtifactBuildPolicyV1 } from "../../../📦️artifacts/🏗️native-build/🟦️.ts";
const root = resolve(import.meta.dir,"../../../../../..");
test("Native mode policy admission agrees with an independent schema oracle", () => {
 const validate = new Ajv({strict:true}).compile(schema);
 const base = { version:1,cwd:"/owner",buildDirectory:"/compiler",leaseDirectory:"/leases",captureDirectory:"/captures",budgetMs:0 };
 for (const mode of ["prototype","release","dev","ship",undefined]) {
  const policy = {...base,...(mode===undefined?{}:{mode})};
  const accepted = mode==="prototype" || mode==="release";
  expect(validate(policy)).toBe(accepted);
  const read = () => readCargoArtifactBuildPolicyV1({SEMIO_CARGO_ARTIFACT_POLICY:JSON.stringify(policy)},"/owner");
  if(accepted) expect(read()).toEqual(policy); else expect(read).toThrow();
 }
});
test("All Cargo workspaces explicitly own native and WASM prototype and release profiles", () => {
 for(const manifest of fixture.workspaces) {
  const parsed = Bun.TOML.parse(readFileSync(resolve(root,manifest),"utf8")) as any;
  for(const row of fixture.modes) expect(parsed.profile[row.nativeProfile]).toBeDefined();
  expect(parsed.profile.prototype).toMatchObject({inherits:"dev",incremental:true,lto:false});
  expect(parsed.profile["wasm-prototype"]).toMatchObject({incremental:true,lto:false});
  expect(parsed.profile.release).toMatchObject({incremental:false,lto:"thin"});
 }
});
test("CMake presets have explicit portable modes and matching configurations", () => {
 const parsed = JSON.parse(readFileSync(resolve(root,"CMakePresets.json"),"utf8"));
 for(const platform of fixture.platforms) for(const row of fixture.modes) {
  const name = `${platform}-${row.mode}`;
  expect(parsed.configurePresets.find((preset:any)=>preset.name===name)).toBeDefined();
  expect(parsed.buildPresets.find((preset:any)=>preset.name===name)).toMatchObject({configuration:row.configuration});
  expect(parsed.testPresets.find((preset:any)=>preset.name===name)).toMatchObject({configuration:row.configuration});
 }
});

test("All-core capacity queues independent builders and admits inherited children", async () => {
 const directory = process.env.SEMIO_TEST_ARTIFACT_DIR!, signal = new AbortController().signal;
 const events:string[]=[];
 const first = await acquireNativeCapacityLeaseV1({directory,signal,environment:{}}); events.push("first-held");
 const inherited = await acquireNativeCapacityLeaseV1({directory,signal,environment:{SEMIO_NATIVE_CAPACITY_HELD:"true"}}); inherited.release();
 let observed!:()=>void; const waiting = new Promise<void>(accept=>{observed=accept;});
 const pending = acquireNativeCapacityLeaseV1({directory,signal,environment:{},onWait:()=>{if(!events.includes("second-waiting"))events.push("second-waiting");observed();}});
 await waiting; events.push("first-released"); first.release();
 const second = await pending; events.push("second-held");
 expect(events).toEqual(fixture.capacity.events);
 const controller = new AbortController();
 const cancelled = acquireNativeCapacityLeaseV1({directory,signal:controller.signal,environment:{},onWait:()=>controller.abort()});
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

test("CMake owner loads independently of root audit code and owns direct Nx commands", async () => {
 const compiled = await build({absWorkingDir:root,entryPoints:[fixture.cmakeEntry],bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,logLevel:"silent"});
 expect(Object.keys(compiled.metafile!.inputs)).not.toContain("📜️script.ts");
 expect(Object.keys(compiled.metafile!.inputs).some(input=>input.includes("/🔍️discovery/"))).toBe(false);
 const project = JSON.parse(readFileSync(resolve(root,"📋️project.json"),"utf8"));
 for(const target of fixture.cppTargets) expect(project.targets[target].options.command).toBe(`bun ./${fixture.cmakeEntry} cpp ${target.slice(4)}`);
});

test("Cargo modes publish separate executable owners and reuse warm compiler intermediates", async () => {
 const directory = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"m-")), manifest = join(directory,"Cargo.toml");
 writeFileSync(manifest,`[workspace]\n[package]\nname="${fixture.cargo.name}"\nversion="0.1.0"\nedition="2021"\n[[bin]]\nname="${fixture.cargo.name}"\npath="main.rs"\n[profile.prototype]\ninherits="dev"\nincremental=true\n[profile.release]\nlto="thin"\ncodegen-units=1\n`);
 writeFileSync(join(directory,"main.rs"),fixture.cargo.source);
 expect(spawnSync("cargo",["generate-lockfile","--offline","--manifest-path",manifest],{cwd:directory,encoding:"utf8"}).status).toBe(0);
 for(const row of fixture.modes) {
  writeFileSync(join(directory,"main.rs"),fixture.cargo.source);
  const policy = {version:1 as const,mode:row.mode as "prototype"|"release",cwd:directory,buildDirectory:join(directory,"b"),leaseDirectory:process.env.SEMIO_TEST_ARTIFACT_DIR!,captureDirectory:join(directory,"c"),budgetMs:30000};
  let receipt = "";
  const times:number[]=[];
  for(const warm of [false,true]) {
   const started = performance.now();
   await buildCargoArtifacts(manifest,["--bin",fixture.cargo.name],policy,{provenance:path=>{receipt=path;}});times.push(Math.round(performance.now()-started));
   const evidence=JSON.parse(readFileSync(receipt,"utf8")), unit=evidence.units.find((unit:any)=>unit.message.target.kind.includes("bin"));
   expect(unit.message.profile.opt_level).toBe(row.optLevel);expect(unit.message.fresh).toBe(warm);
   const binary=join(directory,"dist/build",row.mode,`${fixture.cargo.name}${process.platform==="win32"?".exe":""}`),runtime=spawnSync(binary,[],{encoding:"utf8"});expect(runtime.status).toBe(0);expect(runtime.stdout).toBe(fixture.cargo.stdout);
  }
  const oracle=spawnSync("cargo",["run","--offline","--quiet","--manifest-path",manifest,"--profile",row.nativeProfile,"--target-dir",join(directory,"oracle")],{cwd:directory,encoding:"utf8",env:{...process.env,CARGO_BUILD_BUILD_DIR:join(directory,"ob")},timeout:30000});expect(oracle.status).toBe(0);expect(oracle.stdout).toBe(fixture.cargo.stdout);
  writeFileSync(join(directory,"main.rs"),fixture.cargo.source.replace("runtime-ok","edited-runtime-ok"));
  const editedStarted = performance.now();await buildCargoArtifacts(manifest,["--bin",fixture.cargo.name],policy,{provenance:path=>{receipt=path;}});
  const editedMs = Math.round(performance.now()-editedStarted), editedReceipt=JSON.parse(readFileSync(receipt,"utf8"));expect(editedReceipt.units.find((unit:any)=>unit.message.target.kind.includes("bin")).message.fresh).toBe(false);
  const edited=spawnSync(join(directory,"dist/build",row.mode,`${fixture.cargo.name}${process.platform==="win32"?".exe":""}`),[],{encoding:"utf8"});expect(edited.status).toBe(0);expect(edited.stdout).toBe(fixture.cargo.stdout.replace("runtime-ok","edited-runtime-ok"));
  console.log(`[DEBUG] Cargo mode=${row.mode} runtime=${oracle.stdout} coldMs=${times[0]} warmMs=${times[1]} editedMs=${editedMs} editedRuntime=${edited.stdout}`);
 }
},120000);
