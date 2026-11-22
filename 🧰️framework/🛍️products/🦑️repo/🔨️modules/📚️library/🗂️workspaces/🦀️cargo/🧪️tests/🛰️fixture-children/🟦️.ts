import {registeredToolExecutableV1} from "../../../../../../../../🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import {stripVTControlCharacters} from "node:util";
import {cargoPreparationRuntimeV1,observedCargoPreparationV1,cargoPreparationFixtureEnvironmentV1,prepareCargoFixtureV1,captureCargoFixtureV1} from "../🧰️runtime/🟦️.ts";
import {privateRepositoryWorkspaceV1,testCheckoutRootV1} from "../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import {fileURLToPath} from "node:url";
import {test,expect,spyOn,setDefaultTimeout} from "bun:test";
setDefaultTimeout(180000);
import Ajv from "ajv";
import TOML from "@iarna/toml";
import glob from "fast-glob";
import {mkdtempSync,mkdirSync,readFileSync,writeFileSync,unlinkSync} from "node:fs";
import {join} from "node:path";
import {acquireQueuedResourceLease} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {discoverCargoWorkspaces,cargoWorkspaceForManifest,cargoCommandRequiresOwnerPreparationV1,publishCargoWorkspaceMembership,prepareCargoOwners} from "../../🟦️.ts";

/** 📜️ Admits the one TOML input shape these tests feed: source text. */
const tomlText = (input: Parameters<typeof Bun.TOML.parse>[0]): string => {
  if (typeof input !== "string") throw new Error("TOML source text required");
  return input;
};
const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
const fixtureRepositoryV1=await privateRepositoryWorkspaceV1();process.env.SEMIO_CARGO_FIXTURE_REPOSITORY=fixtureRepositoryV1.root;await fixtureRepositoryV1.prepare("leases","rust","tools");
const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for workspace native proofs");
mkdirSync(artifactRoot, { recursive: true });
const storage=JSON.parse(JSON.stringify(repositoryCargoPreparationStorageV1(process.env.SEMIO_CARGO_FIXTURE_REPOSITORY,testCheckoutRootV1())));process.env.SEMIO_CARGO_PREPARATION_STORAGE=JSON.stringify(storage);
const put = (root: string, path: string, source: string): void => { mkdirSync(join(root, path, ".."), { recursive: true }); writeFileSync(join(root, path), source); };
const workspace = (members: string[], dependencies = ""): string => `[workspace]\nresolver="2"\nmembers=${JSON.stringify(members)}\n[workspace.metadata.semio.repository]\nschema-version=1\nexclude-patterns=[]\nowner-manifests=["*/Cargo.toml"]\nmember-manifests=${JSON.stringify(members.map(m=>m+"/Cargo.toml"))}\n[workspace.package]\nedition="2021"\n${dependencies}`;
const pkg = (name: string, deps = ""): string => `[package]\nname="${name}"\nversion="0.1.0"\nedition.workspace=true\n[lib]\npath="🦀️.rs"\n${deps}`;
const cargo = (root: string, path: string): { status: number; text: string } => { const r=Bun.spawnSync([registeredToolExecutableV1("cargo"),"metadata","--offline","--no-deps","--format-version","1","--manifest-path",join(root,path)],{cwd:root,stdout:"pipe",stderr:"pipe"}); return {status:r.exitCode,text:r.stdout.toString()+r.stderr.toString()}; };
test("selected Cargo refresh removes deleted artifact and plugin manifests while ghost directories remain", async () => {
 for(const deleted of ["artifact","plugin"]) {
  const root=mkdtempSync(join(artifactRoot!,"workspace-ghost-"));
  put(root,"Cargo.toml",workspace(["framework/kernel","specific/artifact","specific/plugin"]).replace('member-manifests=["framework/kernel/Cargo.toml","specific/artifact/Cargo.toml","specific/plugin/Cargo.toml"]','member-manifests=["*/*/Cargo.toml"]'));
  for(const name of ["kernel","artifact","plugin"]) { const directory=name==="kernel"?"framework/kernel":"specific/"+name;put(root,directory+"/Cargo.toml",pkg("owned-"+name));put(root,directory+"/🦀️.rs","pub fn law() {}\n"); }
  expect(cargo(root,"Cargo.toml").status).toBe(0);
  unlinkSync(join(root,"specific",deleted,"Cargo.toml"));put(root,"specific/"+deleted+"/target/retained","source absence is explicit");
  expect(cargo(root,"Cargo.toml").status).not.toBe(0);
  expect(()=>publishCargoWorkspaceMembership(root,discoverCargoWorkspaces(root)[0]!,"check")).toThrow();
  await prepareCargoFixtureV1(storage,root,["metadata","--manifest-path","Cargo.toml"],root,cargoPreparationFixtureEnvironmentV1(root));
  const native=cargo(root,"Cargo.toml");expect(native.status).toBe(0);expect(JSON.parse(native.text).workspace_members.length).toBe(2);
  expect(Bun.TOML.parse(readFileSync(join(root,"Cargo.toml"),"utf8"))).toEqual(TOML.parse(readFileSync(join(root,"Cargo.toml"),"utf8")));
  expect(publishCargoWorkspaceMembership(root,discoverCargoWorkspaces(root)[0]!,"check")).toBe(false);
 }
});
test("every compiling native command prepares owners and keeps diagnostics off machine stdout", () => {
 const schema=JSON.parse(readFileSync(new URL("../../🧬️schema/🏃️invocation/🔣️.json",import.meta.url),"utf8")),corpus=JSON.parse(readFileSync(new URL("../🧫️fixtures/🏃️invocation/🔣️.json",import.meta.url),"utf8"));
 const oracle=new Ajv({strict:true}).compile(schema);
 for(const vector of corpus.cases){expect(oracle(vector)).toBe(true);expect(cargoCommandRequiresOwnerPreparationV1(vector.command)).toBe(vector.requiresOwnerPreparation);expect(oracle({...vector,requiresOwnerPreparation:!vector.requiresOwnerPreparation})).toBe(false);}
 const root=mkdtempSync(join(artifactRoot!,"cargo-machine-output-"));
 put(root,"Cargo.toml",workspace(["framework/*"]));
 put(root,"framework/kernel/Cargo.toml",pkg("neutral-kernel",'[package.metadata.semio.preparation]\nscript="../../📜️script.ts"\ncommand=["publish"]\n'));
 put(root,"framework/kernel/🦀️.rs","");
 put(root,"📜️script.ts",observedCargoPreparationV1(root,'console.log("[preparation] owned input refreshed");'));
 const api=cargoPreparationRuntimeV1(root);
 put(root,"proof/📜️script.ts",`import {prepareCargoFixtureV1,captureCargoFixtureV1} from ${JSON.stringify(fileURLToPath(new URL("../🧰️runtime/🟦️.ts",import.meta.url)))};await prepareCargoFixtureV1(${JSON.stringify(storage)},${JSON.stringify(root)},["run","--manifest-path","Cargo.toml"],${JSON.stringify(root)},process.env);console.log(JSON.stringify({machine:"retained"}));`);
 const child=Bun.spawnSync([process.execPath,join(root,"proof/📜️script.ts")],{cwd:root,env:{...cargoPreparationFixtureEnvironmentV1(root),SEMIO_WORKSPACE_ROOT:root},stdout:"pipe",stderr:"pipe"});
 if(child.exitCode!==0)throw new Error(Buffer.from(child.stderr).toString());
 expect(child.exitCode).toBe(0);expect(Buffer.from(child.stdout).toString()).toBe('{"machine":"retained"}\n');expect(Buffer.from(child.stderr).toString()).toContain("owned input refreshed");
});
test("Cargo preparation diagnostics observe only opted-in exact phases",async()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🛠️preparation-diagnostics/🔣️.json",import.meta.url),"utf8")),packages=JSON.parse(readFileSync(new URL("../🧫️fixtures/🎯️selected-preparation/🔣️.json",import.meta.url),"utf8")).developmentClosure.packages;
 const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(new URL("../../🧬️schema/🛠️preparation/📊️diagnostic/🔣️.json",import.meta.url),"utf8"))),root=mkdtempSync(join(artifactRoot!,"cargo-preparation-diagnostics-"));
 put(root,"Cargo.toml",workspace(["packages/*"]));
 for(const value of packages){
  const edges=["dependencies","build-dependencies","dev-dependencies"].flatMap(kind=>value[kind]?.length?[`[${kind}]\n${value[kind].map((name:string)=>`${name}={path="../${name}"}`).join("\n")}\n`]:[]).join("");
  const source=pkg(value.name,'[package.metadata.semio.preparation]\nscript="../../📜️script.ts"\ncommand=["'+value.name+'"]\n'+edges);
  put(root,`packages/${value.name}/Cargo.toml`,source);put(root,`packages/${value.name}/🦀️.rs`,"pub fn law() {}\n");expect(Bun.TOML.parse(source)).toEqual(TOML.parse(source));
 }
 const tree=await captureCargoFixtureV1(root,["tree","--offline","--prefix","none","--no-dedupe","--format","{p}","--edges","normal,build,dev","--manifest-path",join(root,"Cargo.toml"),...law.roots.flatMap((name:string)=>["-p",name])]);
 const reached=[...new Set(tree.trim().split("\n").map(line=>line.split(" ")[0]))].sort();expect(reached).toEqual(law.expected);
 put(root,"📜️script.ts",observedCargoPreparationV1(root,'import {appendFileSync} from "node:fs";appendFileSync(process.env.SEMIO_WORKSPACE_ROOT+"/events.jsonl",process.argv[2]+"\\n");'));
 const old=process.env.SEMIO_CARGO_PREPARATION_TIMING,output:string[]=[];
 expect(law.stream).toBe("stderr");
 const logger=spyOn(console,"error").mockImplementation((line:unknown)=>{output.push(String(line));});
 try{
  process.env.SEMIO_CARGO_PREPARATION_TIMING=law.disabled;prepareCargoOwners(root,cargoWorkspaceForManifest(root,"Cargo.toml"),law.roots);
  expect(output.filter(line=>line.startsWith("[cargo-preparation-timing] "))).toHaveLength(0);output.length=0;
  process.env.SEMIO_CARGO_PREPARATION_TIMING=law.enabled;prepareCargoOwners(root,cargoWorkspaceForManifest(root,"Cargo.toml"),law.roots);
  const records=output.filter(line=>line.startsWith("[cargo-preparation-timing] ")).map(line=>JSON.parse(line.slice("[cargo-preparation-timing] ".length)));
  expect([...new Set(records.map(row=>row.phase))].sort()).toEqual([...law.phases].sort());
  for(const record of records)expect(validate(record),JSON.stringify(validate.errors)).toBe(true);
  expect(records.filter(row=>row.phase==="recipe").map(row=>(TOML.parse(readFileSync(join(root,row.owner),"utf8")).package as {name:string}).name).sort()).toEqual(reached);
  expect(records.filter(row=>row.phase==="inventory")).toHaveLength(law.initialInventories+law.inventoriesPerRecipe*reached.length);
  expect(records.filter(row=>row.phase==="finished").map(row=>row.items)).toEqual([reached.length]);
 }finally{logger.mockRestore();if(old===undefined)delete process.env.SEMIO_CARGO_PREPARATION_TIMING;else process.env.SEMIO_CARGO_PREPARATION_TIMING=old;}
 const script=fileURLToPath(new URL("../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/📜️script.ts",import.meta.url));
 const child=Bun.spawnSync([process.execPath,script,"prepare","--manifest","Cargo.toml",...law.roots.flatMap((name:string)=>["--package",name])],{cwd:root,env:{...cargoPreparationFixtureEnvironmentV1(root),SEMIO_WORKSPACE_ROOT:root,SEMIO_CARGO_PREPARATION_STORAGE:JSON.stringify(storage),SEMIO_CARGO_PREPARATION_TIMING:law.enabled},stdout:"pipe",stderr:"pipe",timeout:120_000});
 expect(child.exitCode,child.stderr.toString()).toBe(0);
 const records=stripVTControlCharacters(child.stderr.toString()).split("\n").filter(line=>line.startsWith("[cargo-preparation-timing] ")).map(line=>JSON.parse(line.slice("[cargo-preparation-timing] ".length)));
 expect(records.filter(row=>law.scriptPhases.includes(row.phase)).map(row=>row.phase)).toEqual(law.scriptPhases);
 for(const record of records)expect(validate(record),JSON.stringify(validate.errors)).toBe(true);
 expect((TOML.parse(readFileSync(join(root,"Cargo.toml"),"utf8")).workspace as {members:string[]}).members.sort()).toEqual(glob.sync("packages/*/Cargo.toml",{cwd:root,onlyFiles:true}).map(path=>path.slice(0,-"/Cargo.toml".length)).sort());
});
test("queued Cargo preparation exposes opted-in waiting before the protected operation completes",async()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🛠️preparation-diagnostics/🔣️.json",import.meta.url),"utf8")),root=mkdtempSync(join(artifactRoot!,"cargo-live-wait-"));
 put(root,"Cargo.toml",workspace(["packages/*"]));put(root,"packages/a/Cargo.toml",pkg("a"));put(root,"packages/a/🦀️.rs","pub fn law() {}\n");
 const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(new URL("../../🧬️schema/🛠️preparation/📊️diagnostic/🔣️.json",import.meta.url),"utf8")));
 expect(validate({phase:law.waiting.phase,owner:law.waiting.owner,elapsedMs:0,items:law.waiting.items})).toBe(true);
 const lease=await acquireQueuedResourceLease({owner:"independent-diagnostic-owner",directory:storage.leaseDirectory,resource:`cargo-preparation:${root}`,mode:"exclusive",signal:new AbortController().signal});
 const script=fileURLToPath(new URL("../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/📜️script.ts",import.meta.url));
 const child=Bun.spawn([process.execPath,script,"prepare","--manifest",law.waiting.owner],{cwd:root,env:{...cargoPreparationFixtureEnvironmentV1(root),SEMIO_WORKSPACE_ROOT:root,SEMIO_CARGO_PREPARATION_STORAGE:JSON.stringify(storage),SEMIO_CARGO_PREPARATION_TIMING:law.enabled},stdout:"pipe",stderr:"pipe"});
 const reader=child.stderr.getReader();let transcript="",timeout:ReturnType<typeof setTimeout>|undefined;
 try{
  const observation=async():Promise<string>=>{const decoder=new TextDecoder();let buffered="";for(;;){const next=await reader.read();if(next.done)throw Error("Preparation closed before waiting observation");const text=decoder.decode(next.value,{stream:true});transcript+=text;buffered+=text;const lines=buffered.split("\n");buffered=lines.pop()!;const line=lines.map(line=>stripVTControlCharacters(line)).find(line=>line.startsWith("[cargo-preparation-timing] "));if(line)return line;}};
  const observed=await Promise.race([observation(),new Promise<never>((_,reject)=>{timeout=setTimeout(()=>reject(Error("No live stderr lease-wait observation within the startup bound: "+stripVTControlCharacters(transcript).slice(-2000))),law.waiting.maximumObservationMs);})]);
  const record=JSON.parse(observed.slice("[cargo-preparation-timing] ".length));
  expect(validate(record),JSON.stringify(validate.errors)).toBe(true);expect(record.phase).toBe(law.waiting.phase);expect(record.owner).toBe(law.waiting.owner);expect(record.items).toBe(law.waiting.items);expect(child.exitCode).toBe(null);
 }finally{if(timeout)clearTimeout(timeout);child.kill("SIGTERM");await child.exited;reader.releaseLock();lease.release();}
},150000);
