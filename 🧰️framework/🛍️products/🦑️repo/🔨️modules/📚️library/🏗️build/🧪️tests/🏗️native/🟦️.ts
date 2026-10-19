import {repositoryCargoPreparationStorageV1} from "../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import { build } from "esbuild";
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { runOwnedCommand } from "@semio-tech/process/execution";
import { readFileSync, mkdtempSync, writeFileSync, mkdirSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "@semio-tech/process/artifacts/native-build/schema.json";
const root=resolve(import.meta.dir,"../../../../../../../..");

test("All Cargo workspaces consume selected inline profiles without authored manifest tables", async () => {
 const {renderCargoProfileConfigurationV1}=await import("../../🎚️mode/🦀️profile/🟦️.ts"),{parse}=await import("@iarna/toml");
 expect(Bun.TOML.parse(readFileSync(resolve(root,".cargo/config.toml"),"utf8"))).not.toHaveProperty("profile");
 for(const manifest of fixture.workspaces) {
  expect(Bun.TOML.parse(readFileSync(resolve(root,manifest),"utf8"))).not.toHaveProperty("profile");
  for(const row of fixture.modes)for(const target of["x86_64-unknown-linux-gnu","wasm32-wasip2"]){const rendered=renderCargoProfileConfigurationV1({version:1,mode:row.mode,producer:"artifact",operation:"build",target},manifest,"repository"),text=rendered.selectors.join("\n"),actual=Bun.TOML.parse(text);expect(actual).toEqual(parse(text));expect(actual).toHaveProperty("profile."+row.mode);expect(actual).not.toHaveProperty("profile."+(row.mode==="prototype"?"release":"prototype"));}
 }
});

test("Repository Cargo policy consumes the explicit child mode and compiler storage",async()=>{
 const {repositoryCargoArtifactBuildPolicyV1}=await import("../../../⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts");
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR!,compiler=join(output,"compiler-child"),capture=join(output,"capture-child");
 for(const row of fixture.modes){const policy=repositoryCargoArtifactBuildPolicyV1(output,root,{SEMIO_BUILD_MODE:row.mode,CARGO_BUILD_BUILD_DIR:compiler,SEMIO_TEST_ARTIFACT_DIR:capture});expect(policy).toMatchObject({mode:row.mode,buildDirectory:compiler,captureDirectory:capture});expect(new Ajv({strict:true}).compile(schema)(policy)).toBe(true);}
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

test("CMake owner loads independently of root audit code and owns direct Nx commands", async () => {
 const compiled = await build({absWorkingDir:root,entryPoints:[fixture.cmakeEntry],bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,logLevel:"silent"});
 expect(Object.keys(compiled.metafile!.inputs)).not.toContain("📜️script.ts");
 expect(Object.keys(compiled.metafile!.inputs).some(input=>input.includes("/🔍️discovery/"))).toBe(false);
 const project = JSON.parse(readFileSync(resolve(root,"📋️project.json"),"utf8"));
 for(const target of fixture.cppTargets) expect(project.targets[target].options.command).toBe(`bun ./${fixture.cmakeEntry} cpp ${target.slice(4)}`);
});

test("Cargo synchronization reuses content-bound admission and observes manifest and lock changes",async()=>{
 const {withPreparedCargoDependencyPairV1}=await import("../../../🗂️workspaces/🦀️cargo/🛠️preparation/🟦️.ts");
 const synchronizationSchema=(await import("../../../🗂️workspaces/🦀️cargo/🛠️preparation/🧾️synchronization/🧬️schema/🔣️.json")).default;
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR!,owner=mkdtempSync(join(output,"s-"));mkdirSync(join(owner,"kernel"));
 writeFileSync(join(owner,"Cargo.toml"),'[workspace]\nresolver="2"\nmembers=["kernel"]\n[workspace.metadata.semio.repository]\nschema-version=1\nowner-manifests=[]\nmember-manifests=["kernel/Cargo.toml"]\nexclude-patterns=[]\n');
 const packagePath=join(owner,"kernel/Cargo.toml");writeFileSync(packagePath,'[package]\nname="synchronized_kernel"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="lib.rs"\n');writeFileSync(join(owner,"kernel/lib.rs"),"pub fn ready() {}\n");
 const commands:string[][]=[];
 const started=performance.now(),progress:string[]=[],controlFor=(signal:AbortSignal)=>({signal,remainingMilliseconds:()=>fixture.originalPreparationControl.maximumElapsedMilliseconds-(performance.now()-started),async publish(event:unknown){progress.push(JSON.stringify(event));await new Promise(done=>setImmediate(done));},async yieldContinuation(){progress.push("yield");await new Promise(done=>setImmediate(done));}});
 const identityFor=(control:ReturnType<typeof controlFor>)=>async(cwd:string,signal:AbortSignal)=>{expect(signal).toBe(control.signal);let output="";await runOwnedCommand("cargo",["--version"],cwd,"original-paired-cargo-identity",control.remainingMilliseconds(),{signal,output:fixture.originalPreparationControl.transport,stdout:"stderr",onLine:line=>{output+=line+"\n";},onProgress:async()=>{await control.publish({resource:"cargo-preparation:"+owner,mode:"exclusive",elapsedMs:performance.now()-started});await control.yieldContinuation();}});return output;};
 const preparationFor=(control:ReturnType<typeof controlFor>)=>({leaseControl:control,signal:control.signal,async advance(event:unknown){await control.publish(event);await control.yieldContinuation();},childBudget:(requested:number)=>Math.min(requested,control.remainingMilliseconds()),childOptions:(environment:Readonly<Record<string,string|undefined>>)=>({signal:control.signal,env:environment,output:fixture.originalPreparationControl.transport,onProgress:async()=>{await control.publish({resource:"cargo-preparation:"+owner,mode:"exclusive",elapsedMs:performance.now()-started});await control.yieldContinuation();}})});
 const run=async(argv:string[],cwd:string):Promise<void>=>{commands.push(argv);const result=spawnSync("cargo",argv,{cwd,encoding:"utf8"});expect(result.status,result.stderr).toBe(0);};
 const schemaOracle=new Ajv({strict:true}).compile(synchronizationSchema);
 for(let index=0;index<fixture.synchronization.actions.length;index++){
  const action=fixture.synchronization.actions[index];if(action==="manifest-changed")writeFileSync(packagePath,readFileSync(packagePath,"utf8").replace('version="0.1.0"','version="0.2.0"'));
  if(action==="lock-changed")writeFileSync(join(owner,"Cargo.lock"),readFileSync(join(owner,"Cargo.lock"),"utf8")+"\n# changed lock authority\n");
  if(action==="unrelated-owner"){mkdirSync(join(owner,"unrelated"));writeFileSync(join(owner,"unrelated/Cargo.toml"),'[workspace]\n[workspace.metadata.semio.repository]\nschema-version=99\n');writeFileSync(join(owner,"Cargo.toml"),readFileSync(join(owner,"Cargo.toml"),"utf8").replace("owner-manifests=[]","owner-manifests=[\"unrelated/Cargo.toml\"]"));}
  const before=commands.length,control=controlFor(new AbortController().signal);await withPreparedCargoDependencyPairV1({version:1,directory:output,leaseDirectory:repositoryCargoPreparationStorageV1(process.cwd()).leaseDirectory,program:repositoryCargoPreparationStorageV1(process.cwd()).program},owner,"Cargo.toml",control,run,identityFor(control),preparationFor(control),[],["synchronized_kernel"]);expect(commands.length-before).toBe(fixture.synchronization.commands[index]);
 }
 const receipts=readdirSync(output).filter(name=>name.endsWith(".sync.json"));expect(receipts.length).toBeGreaterThan(0);for(const name of receipts)expect(schemaOracle(JSON.parse(readFileSync(join(output,name),"utf8")))).toBe(true);
 expect(progress.length).toBeGreaterThan(0);expect(progress).toContain("yield");
 const oracle=spawnSync("cargo",["metadata","--locked","--no-deps","--format-version=1"],{cwd:owner,encoding:"utf8"});expect(oracle.status,oracle.stderr).toBe(0);expect(JSON.parse(oracle.stdout).packages[0].version).toBe("0.2.0");
},120_000);

