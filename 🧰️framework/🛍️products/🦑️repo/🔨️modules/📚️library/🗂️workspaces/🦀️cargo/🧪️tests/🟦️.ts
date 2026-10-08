import { fileURLToPath } from "node:url";
import { test, expect, spyOn } from "bun:test";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import glob from "fast-glob";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { join, relative } from "node:path";
import { acquireResourceLease } from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import { cargoRepositoryPackageSelections, publishCargoWorkspaceMemberships, parseCargoWorkspaceContribution, discoverCargoWorkspaces, cargoWorkspaceMembers, cargoWorkspaceForManifest, selectedCargoArguments, prepareCargoWorkspaceInvocation, cargoCommandRequiresOwnerPreparationV1, publishCargoWorkspaceMembership, parseCargoPreparation, prepareCargoOwners } from "../🟦️.ts";
const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required for workspace native proofs");
mkdirSync(artifactRoot, { recursive: true });
const put = (root: string, path: string, source: string): void => { mkdirSync(join(root, path, ".."), { recursive: true }); writeFileSync(join(root, path), source); };
const workspace = (members: string[], dependencies = ""): string => `[workspace]\nresolver="2"\nmembers=${JSON.stringify(members)}\n[workspace.metadata.semio.repository]\nschema-version=1\nexclude-patterns=[]\nowner-manifests=["*/Cargo.toml"]\nmember-manifests=${JSON.stringify(members.map(m=>m+"/Cargo.toml"))}\n[workspace.package]\nedition="2021"\n${dependencies}`;
const pkg = (name: string, deps = ""): string => `[package]\nname="${name}"\nversion="0.1.0"\nedition.workspace=true\n[lib]\npath="🦀️.rs"\n${deps}`;
const cargo = (root: string, path: string): { status: number; text: string } => { const r=Bun.spawnSync(["cargo","metadata","--offline","--no-deps","--format-version","1","--manifest-path",join(root,path)],{cwd:root,stdout:"pipe",stderr:"pipe"}); return {status:r.exitCode,text:r.stdout.toString()+r.stderr.toString()}; };
test("portable admitted workspace contributions match independent JSON schema", () => {
 for(const v of fixture.accepted) { expect(validate(v)).toBe(true); expect(parseCargoWorkspaceContribution(v)).toEqual(v); }
 for(const {value} of fixture.rejected) { expect(validate(value)).toBe(false); expect(()=>parseCargoWorkspaceContribution(value)).toThrow(); }
});
test("present owner scopes isolate selected native metadata without concealing broken retained consumers", () => {
 for(const row of fixture.selection) {
  const root=mkdtempSync(join(artifactRoot!,"workspace-owner-"));
  put(root,"Cargo.toml",workspace(["framework/*"])); put(root,"framework/kernel/Cargo.toml",pkg("neutral-kernel")); put(root,"framework/kernel/🦀️.rs","pub fn kernel() {}\n");
  if(!row.removed.includes("specific")) {put(root,"specific/Cargo.toml",workspace(["*"]));put(root,"specific/model/Cargo.toml",pkg("specific-model"));put(root,"specific/model/🦀️.rs","pub fn model() {}\n");}
  if(!row.removed.includes("product")) {put(root,"product/Cargo.toml",workspace(["*"]));put(root,"product/app/Cargo.toml",pkg("product-app",'[dependencies]\nspecific-model={path="../../specific/model"}\n'));put(root,"product/app/🦀️.rs","pub fn app() {}\n");}
  const scopes=discoverCargoWorkspaces(root), members=scopes.flatMap(s=>cargoWorkspaceMembers(root,s));
  expect(members.map(m=>m.directory).sort()).toEqual([...row.selected].sort());
  for(const scope of scopes) { const own=Bun.TOML.parse(readFileSync(join(root,scope.manifest),"utf8"));expect(own).toEqual(TOML.parse(readFileSync(join(root,scope.manifest),"utf8"))); const actual=cargo(root,scope.manifest); if(scope.directory==="product" && row.productValid===false) expect(actual.status).not.toBe(0); else {expect(actual.status).toBe(0); const native=JSON.parse(actual.text);expect(native.workspace_members.length).toBe(members.filter(m=>m.workspace===scope.directory).length);}}
  const independent=glob.sync("framework/*/Cargo.toml",{cwd:root,onlyFiles:true,followSymbolicLinks:false}).map(p=>p.replace(/\/Cargo.toml$/,"")); expect(members.filter(m=>m.workspace===".").map(m=>m.directory)).toEqual(independent);
  expect(cargoWorkspaceForManifest(root,"framework/kernel/Cargo.toml").directory).toBe(".");
 }
});
test("Cargo-native unmatched members refuse while unused inherited path declarations are inert",()=>{
 for(const row of fixture.nativeSemantics) { const root=mkdtempSync(join(artifactRoot!,"workspace-native-"));put(root,"Cargo.toml",workspace(row.members,row.unusedPath?'[workspace.dependencies]\nunused={path="'+row.unusedPath+'"}\n':''));put(root,"framework/kernel/Cargo.toml",pkg("neutral-kernel"));put(root,"framework/kernel/🦀️.rs","pub fn kernel() {}\n"); expect(cargo(root,"Cargo.toml").status===0).toBe(row.valid); }
});

test("inherited path consumers are physically bound to their independently authored workspace", () => {
 for(const row of fixture.inheritance) {
  const root=mkdtempSync(join(artifactRoot!,"workspace-inheritance-"));
  put(root,"Cargo.toml",workspace(["framework/*"]));put(root,"framework/kernel/Cargo.toml",pkg("neutral-kernel").replace('[package]\n','[package]\nworkspace="../.."\n'));put(root,"framework/kernel/🦀️.rs","pub fn kernel() {}\n");
  put(root,"specific/Cargo.toml",workspace(["*"],'[workspace.dependencies]\nshared-key={package="neutral-kernel",path="../framework/kernel"}\n'));
  put(root,"specific/model/Cargo.toml",pkg("specific-model",'[dependencies]\nshared-key={workspace=true}\n').replace('[package]\n',row.explicit?'[package]\nworkspace=".."\n':'[package]\n'));put(root,"specific/model/🦀️.rs","pub fn model() {}\n");
  put(root,"product/Cargo.toml",workspace(["*"]));put(root,"product/app/Cargo.toml",pkg("product-app",'[dependencies]\nspecific-model={path="../../specific/model"}\n'));put(root,"product/app/🦀️.rs","pub fn app() {}\n");
  expect(cargo(root,"product/Cargo.toml").status===0,row.id).toBe(row.valid);
 }
});
test("package command selection binds one current owner and refuses cross-scope or removed selectors",()=>{
 const root=mkdtempSync(join(artifactRoot!,"workspace-selection-"));put(root,"Cargo.toml",workspace(["framework/*"]));put(root,"framework/kernel/Cargo.toml",pkg("neutral-kernel"));put(root,"specific/Cargo.toml",workspace(["*"]));put(root,"specific/model/Cargo.toml",pkg("specific-model"));
 for(const row of fixture.commandSelection) { if(row.error) expect(()=>selectedCargoArguments(root,row.args)).toThrow();else expect(selectedCargoArguments(root,row.args)).toEqual([row.args[0],"--manifest-path",join(root,row.manifest),...row.args.slice(1)]); }
});

test("closed discovery admission requires authored regular-file recipes and portable relative paths", () => {
 const validator = new Ajv({strict:true}).compile(JSON.parse(readFileSync(new URL("../🧬️schema/📇️discovery/🔣️.json",import.meta.url),"utf8")));
 for(const admission of [{"schema-version":1},{"schema-version":1,"member-manifests":["C:/Cargo.toml"]},{"schema-version":1,"member-manifests":["framework/*/Cargo.toml"],"owner-manifests":["../Cargo.toml"]}]) {
  const root=mkdtempSync(join(artifactRoot!,"workspace-admission-"));
  put(root,"Cargo.toml",`[workspace]\nmembers=["framework/*"]\n[workspace.metadata.semio.repository]\n${Object.entries(admission).map(([k,v])=>k+"="+JSON.stringify(v)).join("\n")}\n`);
  expect(validator(admission)).toBe(false); expect(()=>discoverCargoWorkspaces(root)).toThrow();
 }
 expect(validate({schemaVersion:1,members:["C:/owned"],exclude:[]})).toBe(false);
});
test("selected Cargo refresh removes deleted artifact and plugin manifests while ghost directories remain", () => {
 for(const deleted of ["artifact","plugin"]) {
  const root=mkdtempSync(join(artifactRoot!,"workspace-ghost-"));
  put(root,"Cargo.toml",workspace(["framework/kernel","specific/artifact","specific/plugin"]).replace('member-manifests=["framework/kernel/Cargo.toml","specific/artifact/Cargo.toml","specific/plugin/Cargo.toml"]','member-manifests=["*/*/Cargo.toml"]'));
  for(const name of ["kernel","artifact","plugin"]) { const directory=name==="kernel"?"framework/kernel":"specific/"+name;put(root,directory+"/Cargo.toml",pkg("owned-"+name));put(root,directory+"/🦀️.rs","pub fn law() {}\n"); }
  expect(cargo(root,"Cargo.toml").status).toBe(0);
  unlinkSync(join(root,"specific",deleted,"Cargo.toml"));put(root,"specific/"+deleted+"/target/retained","source absence is explicit");
  expect(cargo(root,"Cargo.toml").status).not.toBe(0);
  expect(()=>publishCargoWorkspaceMembership(root,discoverCargoWorkspaces(root)[0]!,"check")).toThrow();
  prepareCargoWorkspaceInvocation(root,["metadata","--manifest-path","Cargo.toml"],root);
  const native=cargo(root,"Cargo.toml");expect(native.status).toBe(0);expect(JSON.parse(native.text).workspace_members.length).toBe(2);
  expect(Bun.TOML.parse(readFileSync(join(root,"Cargo.toml"),"utf8"))).toEqual(TOML.parse(readFileSync(join(root,"Cargo.toml"),"utf8")));
  expect(publishCargoWorkspaceMembership(root,discoverCargoWorkspaces(root)[0]!,"check")).toBe(false);
 }
});

test("portable preparation recipes match the independent closed schema",()=>{
 const validator=new Ajv({strict:true}).compile(JSON.parse(readFileSync(new URL("../🧬️schema/🛠️preparation/🔣️.json",import.meta.url),"utf8")));
 for(const row of fixture.preparation.accepted){expect(validator(row)).toBe(true);expect(parseCargoPreparation(row)).toEqual(row);}
 for(const row of fixture.preparation.rejected){expect(validator(row)).toBe(false);expect(()=>parseCargoPreparation(row)).toThrow();}
});
test("preparation follows actual selected dependency scopes and deduplicates shared owner argv",()=>{
 const root=mkdtempSync(join(artifactRoot!,"workspace-preparation-"));
 put(root,"Cargo.toml",workspace(["framework/*"]));put(root,"framework/kernel/Cargo.toml",pkg("neutral-kernel",'[package.metadata.semio.preparation]\nscript="../../📜️script.ts"\ncommand=["general"]\n'));
 put(root,"specific/Cargo.toml",workspace(["models/*"]));
 for(const name of ["a","b"])put(root,"specific/models/"+name+"/Cargo.toml",pkg("model-"+name,'[package.metadata.semio.preparation]\nscript="../../📜️script.ts"\ncommand=["specific"]\n'));
 put(root,"product/Cargo.toml",workspace(["apps/*"]));put(root,"product/apps/app/Cargo.toml",pkg("product-app",'[dependencies]\nmodel-a={path="../../../specific/models/a"}\nneutral-kernel={path="../../../framework/kernel"}\n'));
 const source='import {appendFileSync} from "node:fs";import {join} from "node:path";appendFileSync(join(process.env.NX_WORKSPACE_ROOT!,"events.jsonl"),JSON.stringify(process.argv.slice(2))+"\\n");';
 put(root,"📜️script.ts",source);put(root,"specific/📜️script.ts",source);
 prepareCargoOwners(root,cargoWorkspaceForManifest(root,"Cargo.toml"));expect(readFileSync(join(root,"events.jsonl"),"utf8")).toBe('["general"]\n');writeFileSync(join(root,"events.jsonl"),"");
 prepareCargoOwners(root,cargoWorkspaceForManifest(root,"product/Cargo.toml"));expect(readFileSync(join(root,"events.jsonl"),"utf8").trim().split("\n").sort()).toEqual(['["general"]','["specific"]']);
 const path=join(root,"framework/kernel/Cargo.toml");writeFileSync(path,readFileSync(path,"utf8").replace('../../📜️script.ts','../../../📜️script.ts'));expect(()=>prepareCargoOwners(root,cargoWorkspaceForManifest(root,"Cargo.toml"))).toThrow();
});

test("every compiling native command prepares owners and keeps diagnostics off machine stdout", () => {
 const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🏃️invocation/🔣️.json",import.meta.url),"utf8")),corpus=JSON.parse(readFileSync(new URL("../🧫️fixtures/🏃️invocation/🔣️.json",import.meta.url),"utf8"));
 const oracle=new Ajv({strict:true}).compile(schema);
 for(const vector of corpus.cases){expect(oracle(vector)).toBe(true);expect(cargoCommandRequiresOwnerPreparationV1(vector.command)).toBe(vector.requiresOwnerPreparation);expect(oracle({...vector,requiresOwnerPreparation:!vector.requiresOwnerPreparation})).toBe(false);}
 const root=mkdtempSync(join(artifactRoot!,"cargo-machine-output-"));
 put(root,"Cargo.toml",workspace(["framework/*"]));
 put(root,"framework/kernel/Cargo.toml",pkg("neutral-kernel",'[package.metadata.semio.preparation]\nscript="../../📜️script.ts"\ncommand=["publish"]\n'));
 put(root,"📜️script.ts",'console.log("[preparation] owned input refreshed");');
 const api=fileURLToPath(new URL("../🟦️.ts",import.meta.url));
 put(root,"proof/📜️script.ts",`import {prepareCargoWorkspaceInvocation} from ${JSON.stringify(api)};prepareCargoWorkspaceInvocation(${JSON.stringify(root)},["run","--manifest-path","Cargo.toml"],${JSON.stringify(root)});console.log(JSON.stringify({machine:"retained"}));`);
 const child=Bun.spawnSync([process.execPath,join(root,"proof/📜️script.ts")],{cwd:root,env:{...process.env,NX_WORKSPACE_ROOT:root},stdout:"pipe",stderr:"pipe"});
 if(child.exitCode!==0)throw new Error(Buffer.from(child.stderr).toString());
 expect(child.exitCode).toBe(0);expect(Buffer.from(child.stdout).toString()).toBe('{"machine":"retained"}\n');expect(Buffer.from(child.stderr).toString()).toContain("owned input refreshed");
});


test("one membership invocation parses each current manifest once and observes later saves", () => {
  const law = JSON.parse(readFileSync(new URL("../🧫️fixtures/📇️invocation-inventory/🔣️.json", import.meta.url), "utf8"));
  const validator = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(new URL("../🧬️schema/📇️invocation-inventory/🔣️.json", import.meta.url), "utf8")));
  expect(validator(law)).toBe(true);
  const root = mkdtempSync(join(artifactRoot!, "cargo-invocation-inventory-"));
  put(root, "Cargo.toml", workspace(["packages/*"]));
  const names = Array.from({ length: law.members }, (_, index) => `member-${String(index).padStart(3, "0")}`);
  const manifest = (index: number): string => `packages/${String(index).padStart(3, "0")}/Cargo.toml`;
  for (let index = 0; index < law.members; index++) put(root, manifest(index), pkg(names[index]!));
  const owner = cargoWorkspaceForManifest(root, "Cargo.toml");
  const original = Bun.TOML.parse;
  const calls = new Map<string, number>();
  const documents = new Map<string, unknown>();
  const parser = spyOn(Bun.TOML, "parse").mockImplementation((source: string) => {
    calls.set(source, (calls.get(source) ?? 0) + 1);
    const result = original(source);
    documents.set(source, result);
    return result;
  });
  const check = (expected: readonly string[]): void => {
    calls.clear(); documents.clear();
    const rows = cargoWorkspaceMembers(root, owner);
    expect(rows.map(row => row.name)).toEqual(expected);
    for (const [source, document] of documents) expect(document).toEqual(TOML.parse(source));
    expect(Math.max(...calls.values())).toBeLessThanOrEqual(law.maxParsesPerManifest);
  };
  try {
    check(names);
    names[law.rename.index] = law.rename.name;
    put(root, manifest(law.rename.index), pkg(law.rename.name));
    check(names);
    unlinkSync(join(root, manifest(law.removeIndex)));
    check(names.filter((_, index) => index !== law.removeIndex));
  } finally { parser.mockRestore(); }
});


test("batch package selection discovers one fresh repository inventory", () => {
  const law = JSON.parse(readFileSync(new URL("../🧫️fixtures/📇️invocation-inventory/🔣️.json", import.meta.url), "utf8"));
  const root = mkdtempSync(join(artifactRoot!, "cargo-batch-inventory-"));
  const rootSource = workspace(["packages/*"]);
  put(root, "Cargo.toml", rootSource);
  const names = Array.from({ length: law.members }, (_, index) => `member-${String(index).padStart(3, "0")}`);
  for (let index = 0; index < law.members; index++) put(root, `packages/${String(index).padStart(3, "0")}/Cargo.toml`, pkg(names[index]!));
  const original = Bun.TOML.parse;
  let rootParses = 0;
  const parser = spyOn(Bun.TOML, "parse").mockImplementation((source: string) => { if (source.includes('owner-manifests=["*/Cargo.toml"]') && source.includes('member-manifests=["packages/*/Cargo.toml"]')) rootParses++; return original(source); });
  try {
    expect(cargoRepositoryPackageSelections(root, law.selectedIndices.map((index: number) => names[index])) .map(row => row.name)).toEqual(law.selectedIndices.map((index: number) => names[index]));
    expect(rootParses).toBeLessThanOrEqual(law.maxRepositoryParses);
  } finally { parser.mockRestore(); }
});

test("batch owner publication discovers one fresh repository inventory", () => {
  const law = JSON.parse(readFileSync(new URL("../🧫️fixtures/📇️invocation-inventory/🔣️.json", import.meta.url), "utf8"));
  const root = mkdtempSync(join(artifactRoot!, "cargo-batch-inventory-"));
  const rootSource = workspace(["packages/*"]);
  put(root, "Cargo.toml", rootSource);
  const names = Array.from({ length: law.members }, (_, index) => `member-${String(index).padStart(3, "0")}`);
  for (let index = 0; index < law.members; index++) put(root, `packages/${String(index).padStart(3, "0")}/Cargo.toml`, pkg(names[index]!));
  const original = Bun.TOML.parse;
  let rootParses = 0;
  const parser = spyOn(Bun.TOML, "parse").mockImplementation((source: string) => { if (source.includes('owner-manifests=["*/Cargo.toml"]') && source.includes('member-manifests=["packages/*/Cargo.toml"]')) rootParses++; return original(source); });
  try {
    for (let index = 0; index < law.publicationOwners; index++) {
      put(root, `owner-${index}/Cargo.toml`, workspace(["members/*"]));
      put(root, `owner-${index}/members/a/Cargo.toml`, pkg(`child-${index}`));
    }
    rootParses = 0;
    const published = publishCargoWorkspaceMemberships(root, "write");
    expect(published).toHaveLength(law.publicationOwners + 1);
    expect(rootParses).toBeLessThanOrEqual(law.maxPublicationParses);
    const after = readFileSync(join(root, "Cargo.toml"), "utf8");
    expect(Bun.TOML.parse(after)).toEqual(TOML.parse(after));
    expect((TOML.parse(after).workspace as { exclude: string[] }).exclude).toEqual(Array.from({ length: law.publicationOwners }, (_, index) => `owner-${index}`));
    unlinkSync(join(root, "owner-0/Cargo.toml"));
    expect(publishCargoWorkspaceMemberships(root, "write")).toHaveLength(law.publicationOwners);
    expect((TOML.parse(readFileSync(join(root, "Cargo.toml"), "utf8")).workspace as { exclude: string[] }).exclude).not.toContain("owner-0");
  } finally { parser.mockRestore(); }
});


test("selected preparation development closure matches Cargo tree root ownership",()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🎯️selected-preparation/🔣️.json",import.meta.url),"utf8")).developmentClosure;
 for(const row of law.cases){
  const root=mkdtempSync(join(artifactRoot!,"cargo-development-closure-")),names=new Set<string>(law.packages.map((value:any)=>value.name));
  put(root,"Cargo.toml",workspace(["packages/*"]));
  for(const value of law.packages){
   const edges=["dependencies","build-dependencies","dev-dependencies"].flatMap(kind=>value[kind]?.length?[`[${kind}]\n${value[kind].map((name:string)=>`${name}={path="../${name}"}`).join("\n")}\n`]:[]).join("");
   const path=`packages/${value.name}/Cargo.toml`,source=pkg(value.name,'[package.metadata.semio.preparation]\nscript="../../📜️script.ts"\ncommand=["'+value.name+'"]\n'+edges);
   put(root,path,source);put(root,`packages/${value.name}/🦀️.rs`,"pub fn law() {}\n");expect(Bun.TOML.parse(source)).toEqual(TOML.parse(source));
  }
  expect(glob.sync("packages/*/Cargo.toml",{cwd:root,onlyFiles:true}).length).toBe(names.size);
  const tree=Bun.spawnSync(["cargo","tree","--offline","--prefix","none","--no-dedupe","--format","{p}","--edges","normal,build,dev","--manifest-path",join(root,"Cargo.toml"),...(row.roots.length?row.roots.flatMap((name:string)=>["-p",name]):["--workspace"])],{cwd:root,stdout:"pipe",stderr:"pipe",timeout:10_000});
  expect(tree.exitCode,tree.stderr.toString()).toBe(0);
  const reached=[...new Set(tree.stdout.toString().split("\n").map(line=>line.split(" ")[0]!).filter(name=>names.has(name)))].sort();expect(reached).toEqual(row.expected);
  put(root,"📜️script.ts",'import {appendFileSync} from "node:fs";import {join} from "node:path";appendFileSync(join(process.env.NX_WORKSPACE_ROOT!,"events.jsonl"),process.argv[2]+"\\n");');
  prepareCargoOwners(root,cargoWorkspaceForManifest(root,"Cargo.toml"),row.roots);
  expect(readFileSync(join(root,"events.jsonl"),"utf8").trim().split("\n").sort()).toEqual(reached);
  console.log(`[DEBUG] Cargo preparation development closure roots=${row.roots.join(",")||"workspace"} exactPrepared=${reached.length}`);
 }
});

test("Cargo preparation diagnostics observe only opted-in exact phases",()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🛠️preparation-diagnostics/🔣️.json",import.meta.url),"utf8")),packages=JSON.parse(readFileSync(new URL("../🧫️fixtures/🎯️selected-preparation/🔣️.json",import.meta.url),"utf8")).developmentClosure.packages;
 const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(new URL("../🧬️schema/🛠️preparation/📊️diagnostic/🔣️.json",import.meta.url),"utf8"))),root=mkdtempSync(join(artifactRoot!,"cargo-preparation-diagnostics-"));
 put(root,"Cargo.toml",workspace(["packages/*"]));
 for(const value of packages){
  const edges=["dependencies","build-dependencies","dev-dependencies"].flatMap(kind=>value[kind]?.length?[`[${kind}]\n${value[kind].map((name:string)=>`${name}={path="../${name}"}`).join("\n")}\n`]:[]).join("");
  const source=pkg(value.name,'[package.metadata.semio.preparation]\nscript="../../📜️script.ts"\ncommand=["'+value.name+'"]\n'+edges);
  put(root,`packages/${value.name}/Cargo.toml`,source);put(root,`packages/${value.name}/🦀️.rs`,"pub fn law() {}\n");expect(Bun.TOML.parse(source)).toEqual(TOML.parse(source));
 }
 const tree=Bun.spawnSync(["cargo","tree","--offline","--prefix","none","--no-dedupe","--format","{p}","--edges","normal,build,dev","--manifest-path",join(root,"Cargo.toml"),...law.roots.flatMap((name:string)=>["-p",name])],{cwd:root,stdout:"pipe",stderr:"pipe"});
 expect(tree.exitCode,tree.stderr.toString()).toBe(0);
 const reached=[...new Set(tree.stdout.toString().trim().split("\n").map(line=>line.split(" ")[0]))].sort();expect(reached).toEqual(law.expected);
 put(root,"📜️script.ts",'import {appendFileSync} from "node:fs";appendFileSync(process.env.NX_WORKSPACE_ROOT+"/events.jsonl",process.argv[2]+"\\n");');
 const old=process.env.SEMIO_CARGO_PREPARATION_TIMING,output:string[]=[];
 expect(law.stream).toBe("stderr");
 const logger=spyOn(console,"error").mockImplementation((line:unknown)=>{output.push(String(line));});
 try{
  process.env.SEMIO_CARGO_PREPARATION_TIMING=law.disabled;prepareCargoOwners(root,cargoWorkspaceForManifest(root,"Cargo.toml"),law.roots);
  expect(output.filter(line=>line.startsWith("[DEBUG] cargo-preparation "))).toHaveLength(0);output.length=0;
  process.env.SEMIO_CARGO_PREPARATION_TIMING=law.enabled;prepareCargoOwners(root,cargoWorkspaceForManifest(root,"Cargo.toml"),law.roots);
  const records=output.filter(line=>line.startsWith("[DEBUG] cargo-preparation ")).map(line=>JSON.parse(line.slice("[DEBUG] cargo-preparation ".length)));
  expect([...new Set(records.map(row=>row.phase))].sort()).toEqual([...law.phases].sort());
  for(const record of records)expect(validate(record),JSON.stringify(validate.errors)).toBe(true);
  expect(records.filter(row=>row.phase==="recipe").map(row=>TOML.parse(readFileSync(join(root,row.owner),"utf8")).package!.name).sort()).toEqual(reached);
  expect(records.filter(row=>row.phase==="inventory")).toHaveLength(1);
  expect(records.filter(row=>row.phase==="finished").map(row=>row.items)).toEqual([reached.length]);
 }finally{logger.mockRestore();if(old===undefined)delete process.env.SEMIO_CARGO_PREPARATION_TIMING;else process.env.SEMIO_CARGO_PREPARATION_TIMING=old;}
 const script=fileURLToPath(new URL("../🛠️preparation/📜️script.ts",import.meta.url));
 const child=Bun.spawnSync([process.execPath,script,"prepare","--manifest","Cargo.toml",...law.roots.flatMap((name:string)=>["--package",name])],{cwd:root,env:{...process.env,NX_WORKSPACE_ROOT:root,SEMIO_CARGO_PREPARATION_TIMING:law.enabled},stdout:"pipe",stderr:"pipe",timeout:10_000});
 expect(child.exitCode,child.stderr.toString()).toBe(0);
 const records=child.stderr.toString().split("\n").filter(line=>line.startsWith("[DEBUG] cargo-preparation ")).map(line=>JSON.parse(line.slice("[DEBUG] cargo-preparation ".length)));
 expect(records.filter(row=>law.scriptPhases.includes(row.phase)).map(row=>row.phase)).toEqual(law.scriptPhases);
 for(const record of records)expect(validate(record),JSON.stringify(validate.errors)).toBe(true);
 expect((TOML.parse(readFileSync(join(root,"Cargo.toml"),"utf8")).workspace as {members:string[]}).members.sort()).toEqual(glob.sync("packages/*/Cargo.toml",{cwd:root,onlyFiles:true}).map(path=>path.slice(0,-"/Cargo.toml".length)).sort());
 console.log(`[DEBUG] independent Cargo/TOML preparation diagnostic phases=${law.phases.length} exactRecipes=${reached.length} defaultQuiet=true`);
});

test("queued Cargo preparation exposes opted-in waiting before the protected operation completes",async()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🛠️preparation-diagnostics/🔣️.json",import.meta.url),"utf8")),root=mkdtempSync(join(artifactRoot!,"cargo-live-wait-"));
 put(root,"Cargo.toml",workspace(["packages/*"]));put(root,"packages/a/Cargo.toml",pkg("a"));put(root,"packages/a/🦀️.rs","pub fn law() {}\n");
 const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(new URL("../🧬️schema/🛠️preparation/📊️diagnostic/🔣️.json",import.meta.url),"utf8")));
 expect(validate({phase:law.waiting.phase,owner:law.waiting.owner,elapsedMs:0,items:law.waiting.items})).toBe(true);
 const lease=await acquireResourceLease({directory:join(root,".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"),resource:`cargo-preparation:${root}`,mode:"exclusive",signal:new AbortController().signal});
 const script=fileURLToPath(new URL("../🛠️preparation/📜️script.ts",import.meta.url));
 const child=Bun.spawn([process.execPath,script,"prepare","--manifest",law.waiting.owner],{cwd:root,env:{...process.env,NX_WORKSPACE_ROOT:root,SEMIO_CARGO_PREPARATION_TIMING:law.enabled},stdout:"pipe",stderr:"pipe"});
 const reader=child.stderr.getReader();let timeout:ReturnType<typeof setTimeout>|undefined;
 try{
  const observation=async():Promise<string>=>{const decoder=new TextDecoder();let buffered="";for(;;){const next=await reader.read();if(next.done)throw Error("Preparation closed before waiting observation");buffered+=decoder.decode(next.value,{stream:true});const lines=buffered.split("\n");buffered=lines.pop()!;const line=lines.find(line=>line.startsWith("[DEBUG] cargo-preparation "));if(line)return line;}};
  const observed=await Promise.race([observation(),new Promise<never>((_,reject)=>{timeout=setTimeout(()=>reject(Error("No live stderr lease-wait observation")),law.waiting.maximumObservationMs);})]);
  const record=JSON.parse(observed.slice("[DEBUG] cargo-preparation ".length));
  expect(validate(record),JSON.stringify(validate.errors)).toBe(true);expect(record.phase).toBe(law.waiting.phase);expect(record.owner).toBe(law.waiting.owner);expect(record.items).toBe(law.waiting.items);expect(child.exitCode).toBe(null);
  console.log(`[DEBUG] queued Cargo preparation live stderr phase=${record.phase} protectedOperationIncomplete=true`);
 }finally{if(timeout)clearTimeout(timeout);child.kill("SIGTERM");await child.exited;reader.releaseLock();lease.release();}
},10000);

test("selected package preparation follows the Cargo resolved local closure without preparing siblings",()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🎯️selected-preparation/🔣️.json",import.meta.url),"utf8"));
 for(const row of law.cases){
  const root=mkdtempSync(join(artifactRoot!,"cargo-selected-preparation-"));
  put(root,"Cargo.toml",workspace(["framework/*"]).replace('resolver="2"','resolver="2"\nexclude=["specific"]'));
  put(root,"specific/Cargo.toml",workspace(["models/*"],'[workspace.dependencies]\nbridge={path="../../framework/bridge"}\n'.replace("../../framework","../framework")));
  const recipe=(name:string)=>'[package.metadata.semio.preparation]\nscript="../../📜️script.ts"\ncommand=["'+name+'"]\n';
  put(root,"framework/kernel/Cargo.toml",pkg("neutral-kernel",recipe("neutral-kernel")));
  put(root,"framework/bridge/Cargo.toml",pkg("bridge",recipe("bridge")+'[dependencies]\nneutral-kernel={path="../kernel"}\n'));
  put(root,"specific/models/a/Cargo.toml",pkg("model-a",recipe("model-a")+'[dependencies]\nbridge={workspace=true}\n'));
  put(root,"specific/models/b/Cargo.toml",pkg("model-b",recipe("model-b")));
  for(const path of ["framework/kernel","framework/bridge","specific/models/a","specific/models/b"])put(root,path+"/🦀️.rs","pub fn law() {}\n");
  const source='import {appendFileSync} from "node:fs";import {join} from "node:path";appendFileSync(join(process.env.NX_WORKSPACE_ROOT!,"events.jsonl"),process.argv[2]+"\\n");';
  put(root,"📜️script.ts",source);put(root,"specific/📜️script.ts",source);
  const metadata=Bun.spawnSync(["cargo","metadata","--offline","--format-version","1","--manifest-path",join(root,row.manifest)],{cwd:root,stdout:"pipe",stderr:"pipe"});
  expect(metadata.exitCode,metadata.stderr.toString()).toBe(0);
  const oracle=JSON.parse(metadata.stdout.toString()),packages=new Map(oracle.packages.map((value:any)=>[value.id,value])),nodes=new Map(oracle.resolve.nodes.map((value:any)=>[value.id,value]));
  const seeds=row.packages.length?oracle.packages.filter((value:any)=>row.packages.includes(value.name)).map((value:any)=>value.id):row.manifest.endsWith("models/a/Cargo.toml")?[oracle.resolve.root]:oracle.workspace_members;
  const reached=new Set<string>(seeds);for(const id of reached)for(const dependency of (nodes.get(id) as any).dependencies)reached.add(dependency);
  expect([...reached].map(id=>(packages.get(id) as any).name).sort()).toEqual(row.expected);
  const api=fileURLToPath(new URL("../🟦️.ts",import.meta.url));
  const args=["test","--manifest-path",row.manifest,...row.packages.flatMap((name:string)=>["-p",name])];
  put(root,"proof/📜️script.ts",`import {prepareCargoWorkspaceInvocation} from ${JSON.stringify(api)};prepareCargoWorkspaceInvocation(${JSON.stringify(root)},${JSON.stringify(args)},${JSON.stringify(root)});`);
  const child=Bun.spawnSync([process.execPath,join(root,"proof/📜️script.ts")],{cwd:root,env:{...process.env,NX_WORKSPACE_ROOT:root},stdout:"pipe",stderr:"pipe"});
  expect(child.exitCode,child.stderr.toString()).toBe(0);
  expect(readFileSync(join(root,"events.jsonl"),"utf8").trim().split("\n").sort()).toEqual(row.expected);
  for(const path of ["Cargo.toml","specific/Cargo.toml"])expect(Bun.TOML.parse(readFileSync(join(root,path),"utf8"))).toEqual(TOML.parse(readFileSync(join(root,path),"utf8")));
 }
});


test("membership discovery compiles bounded matchers independent of unrelated leaf count", () => {
 const law=fixture.discoveryWork,root=mkdtempSync(join(artifactRoot!,"cargo-discovery-work-"));
 put(root,"Cargo.toml",workspace(law.members).replace("exclude-patterns=[]","exclude-patterns="+JSON.stringify(law.exclude)));
 put(root,"packages/owned/Cargo.toml",pkg("owned-member"));
 put(root,"packages/ignored/nested/Cargo.toml","excluded authority must not parse");
 for(let index=0;index<law.noiseFiles;index++)put(root,`packages/noise/${index}.data`,"unrelated leaf");
 const owner=cargoWorkspaceForManifest(root,"Cargo.toml"),original=Bun.Glob,descriptor=Object.getOwnPropertyDescriptor(Bun,"Glob")!;
 let matchers=0;
 Object.defineProperty(Bun,"Glob",{configurable:descriptor.configurable,enumerable:descriptor.enumerable,writable:true,value:class extends original {constructor(pattern:string){super(pattern);matchers++;}}});
 try {
  const actual=cargoWorkspaceMembers(root,owner);
  const independent=glob.sync(owner.memberManifests,{cwd:root,onlyFiles:true,followSymbolicLinks:false,ignore:law.exclude}).sort();
  expect(actual.map(row=>row.manifest)).toEqual(independent);
  expect(actual.map(row=>row.name)).toEqual(law.expectedNames);
  expect(matchers).toBeLessThanOrEqual(law.maximumMatchers);
  console.log(`[DEBUG] Cargo membership discovery unrelatedLeaves=${law.noiseFiles} compiledMatchers=${matchers} exactManifests=${actual.length}`);
 } finally {Object.defineProperty(Bun,"Glob",descriptor);}
});


test("explicit root-owned packages remain discoverable beneath a different native workspace",()=>{
 const row=fixture.overlappingOwners,root=mkdtempSync(join(artifactRoot!,"workspace-overlap-"));put(root,"Cargo.toml",workspace(row.rootMembers).replace('member-manifests='+JSON.stringify(row.rootMembers.map((member:string)=>member+"/Cargo.toml")),'member-manifests='+JSON.stringify(row.rootAdmission)).replace('members='+JSON.stringify(row.rootMembers),'members='+JSON.stringify(row.rootMembers)+'\nexclude=["framework"]'));put(root,"framework/Cargo.toml",workspace(row.frameworkMembers).replace('members='+JSON.stringify(row.frameworkMembers),'members='+JSON.stringify(row.frameworkMembers)+'\nexclude=["product"]'));put(root,"framework/product/Cargo.toml",pkg(row.rootPackage).replace('[package]\n','[package]\nworkspace='+JSON.stringify(row.rootPackageWorkspace)+'\n'));put(root,"framework/product/🦀️.rs","pub fn product(){}\n");put(root,"framework/general/Cargo.toml",pkg(row.frameworkPackage));put(root,"framework/general/🦀️.rs","pub fn general(){}\n");
 const scopes=discoverCargoWorkspaces(root),packages=scopes.flatMap(scope=>cargoWorkspaceMembers(root,scope));expect(packages.map(row=>[row.name,row.workspace]).sort()).toEqual([[row.frameworkPackage,"framework"],[row.rootPackage,"."]].sort());for(const scope of scopes){const native=cargo(root,scope.manifest);expect(native.status,native.text).toBe(0);const metadata=JSON.parse(native.text);expect(metadata.workspace_members).toHaveLength(1);}
 expect(selectedCargoArguments(root,["check","-p",row.rootPackage])).toEqual(["check","--manifest-path",join(root,"framework/product/Cargo.toml"),"-p",row.rootPackage]);expect(selectedCargoArguments(root,["check","-p",row.frameworkPackage])).toEqual(["check","--manifest-path",join(root,"framework/general/Cargo.toml"),"-p",row.frameworkPackage]);
});


test("current physical workspace admissions and selected nextest profiles match independent native documents",async()=>{
 const root=process.cwd(),{repositoryCargoTestPolicyV1}=await import("../../../🟦️.ts"),scopes=discoverCargoWorkspaces(root),packages=scopes.flatMap(scope=>{const members=cargoWorkspaceMembers(root,scope),document=TOML.parse(readFileSync(join(root,scope.manifest),"utf8")) as any;
  expect(members.map(row=>relative(join(root,scope.directory),join(root,row.directory)).replaceAll("\\","/")||".").sort()).toEqual([...document.workspace.members].sort());return members;});
 for(const target of fixture.policyPackages){const selected=packages.find(row=>row.name===target.name);expect(selected?.workspace).toBe(target.workspace);const configPath=join(root,target.workspace,".config/nextest.toml"),source=readFileSync(configPath,"utf8"),config=TOML.parse(source) as any;expect(Bun.TOML.parse(source)).toEqual(config);
  for(const [level,period]of Object.entries(fixture.profilePeriods)){expect(config.profile[level]["slow-timeout"]).toEqual({period,"terminate-after":1});const policy=repositoryCargoTestPolicyV1(join(root,selected!.manifest),root,{...process.env,SEMIO_TEST_LEVEL:level});expect(policy.configPath).toBe(configPath);expect(policy.level).toBe(level);}
 }
},30000);


test("all-root dependency preparation honors explicit package workspaces beneath native child owners",()=>{
 const row=fixture.overlappingOwners,root=mkdtempSync(join(artifactRoot!,"workspace-prepare-nested-"));
 put(root,"Cargo.toml",workspace(row.rootMembers,'[workspace.dependencies]\ngeneral={package="'+row.frameworkPackage+'",path="framework/general"}\n').replace('member-manifests='+JSON.stringify(row.rootMembers.map((m:string)=>m+"/Cargo.toml")),'member-manifests='+JSON.stringify(row.rootAdmission)).replace('[workspace]\n','[workspace]\nexclude=["framework"]\n'));
 put(root,"framework/Cargo.toml",workspace(row.frameworkMembers).replace('[workspace]\n','[workspace]\nexclude=["product"]\n'));put(root,"framework/product/Cargo.toml",pkg(row.rootPackage,'[dependencies]\ngeneral={workspace=true}\n').replace('[package]\n','[package]\nworkspace="'+row.rootPackageWorkspace+'"\n'));put(root,"framework/product/🦀️.rs","pub fn product() {}\n");put(root,"framework/general/Cargo.toml",pkg(row.frameworkPackage));put(root,"framework/general/🦀️.rs","pub fn general() {}\n");
 const native=cargo(root,"Cargo.toml");expect(native.status,native.text).toBe(0);expect(JSON.parse(native.text).workspace_members).toHaveLength(1);const tree=Bun.spawnSync(["cargo","tree","--offline","--manifest-path",join(root,"Cargo.toml"),"-p",row.frameworkPackage],{cwd:root,stdout:"pipe",stderr:"pipe"});expect(tree.exitCode,tree.stderr.toString()).toBe(0);expect(tree.stdout.toString()).toContain(row.frameworkPackage);
 expect(()=>prepareCargoOwners(root,discoverCargoWorkspaces(root).find(scope=>scope.directory===".")!)).not.toThrow();
});


test("named dependency preparation agrees with actual Cargo root graph reachability",()=>{
 const row=fixture.overlappingOwners,root=mkdtempSync(join(artifactRoot!,"workspace-prepare-reachable-"));
 put(root,"Cargo.toml",workspace(row.rootMembers,'[workspace.dependencies]\ngeneral={package="'+row.frameworkPackage+'",path="framework/general"}\n').replace('member-manifests='+JSON.stringify(row.rootMembers.map((m:string)=>m+"/Cargo.toml")),'member-manifests='+JSON.stringify(row.rootAdmission)).replace('[workspace]\n','[workspace]\nexclude=["framework"]\n'));
 put(root,"framework/Cargo.toml",workspace([...row.frameworkMembers,"orphan"]).replace('[workspace]\n','[workspace]\nexclude=["product"]\n'));put(root,"framework/product/Cargo.toml",pkg(row.rootPackage,'[dependencies]\ngeneral={workspace=true}\n').replace('[package]\n','[package]\nworkspace="'+row.rootPackageWorkspace+'"\n'));put(root,"framework/product/🦀️.rs","pub fn product() {}\n");put(root,"framework/general/Cargo.toml",pkg(row.frameworkPackage));put(root,"framework/general/🦀️.rs","pub fn general() {}\n");put(root,"framework/orphan/Cargo.toml",pkg("unreachable-module"));put(root,"framework/orphan/🦀️.rs","pub fn orphan() {}\n");
 const selected=discoverCargoWorkspaces(root).find(scope=>scope.directory===".")!;
 for(const selection of fixture.reachablePreparation){const tree=Bun.spawnSync(["cargo","tree","--offline","--manifest-path",join(root,"Cargo.toml"),...selection.names.flatMap((name:string)=>["-p",name])],{cwd:root,stdout:"pipe",stderr:"pipe"});expect(tree.exitCode===0,tree.stderr.toString()).toBe(selection.valid);if(selection.valid)expect(()=>prepareCargoOwners(root,selected,selection.names)).not.toThrow();else expect(()=>prepareCargoOwners(root,selected,selection.names)).toThrow();}
});
