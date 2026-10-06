import { fileURLToPath } from "node:url";
import { test, expect } from "bun:test";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import glob from "fast-glob";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { join } from "node:path";
import { parseCargoWorkspaceContribution, discoverCargoWorkspaces, cargoWorkspaceMembers, cargoWorkspaceForManifest, selectedCargoArguments, prepareCargoWorkspaceInvocation, cargoCommandRequiresOwnerPreparationV1, publishCargoWorkspaceMembership, parseCargoPreparation, prepareCargoOwners } from "../🟦️.ts";
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
