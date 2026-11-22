import {registeredToolExecutableV1} from "../../../../../../../🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts";
import {cargoWorkspaceForManifestControlledV1} from "../🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import {captureCargoFixtureV1} from "./🧰️runtime/🟦️.ts";
import {privateRepositoryWorkspaceV1,testCheckoutRootV1} from "../../../🧪️testing/🧫️private-repository/🟦️.ts";
import {fileURLToPath} from "node:url";
import {test,expect,spyOn,setDefaultTimeout} from "bun:test";
setDefaultTimeout(180000);
import Ajv from "ajv";
import TOML from "@iarna/toml";
import glob from "fast-glob";
import {mkdtempSync,mkdirSync,readFileSync,writeFileSync,unlinkSync} from "node:fs";
import {join,relative} from "node:path";
import {cargoRepositoryPackageSelections,publishCargoWorkspaceMemberships,parseCargoWorkspaceContribution,discoverCargoWorkspaces,cargoWorkspaceMembers,cargoWorkspaceForManifest,selectedCargoArguments,parseCargoPreparation,prepareCargoOwners} from "../🟦️.ts";

/** 📜️ Admits the one TOML input shape these tests feed: source text. */
const tomlText = (input: Parameters<typeof Bun.TOML.parse>[0]): string => {
  if (typeof input !== "string") throw new Error("TOML source text required");
  return input;
};
const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
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
test("Cargo-native unmatched members refuse while unused inherited path declarations are inert",async()=>{
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
test("package command selection binds one current owner and refuses cross-scope or removed selectors",async()=>{
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

test("portable preparation recipes match the independent closed schema",async()=>{
 const validator=new Ajv({strict:true}).compile(JSON.parse(readFileSync(new URL("../🧬️schema/🛠️preparation/🔣️.json",import.meta.url),"utf8")));
 for(const row of fixture.preparation.accepted){expect(validator(row)).toBe(true);expect(parseCargoPreparation(row)).toEqual(row);}
 for(const row of fixture.preparation.rejected){expect(validator(row)).toBe(false);expect(()=>parseCargoPreparation(row)).toThrow();}
});
test("preparation follows actual selected dependency scopes and deduplicates shared owner argv",async()=>{
 const root=mkdtempSync(join(artifactRoot!,"workspace-preparation-"));
 put(root,"Cargo.toml",workspace(["framework/*"]));put(root,"framework/kernel/Cargo.toml",pkg("neutral-kernel",'[package.metadata.semio.preparation]\nscript="../../📜️script.ts"\ncommand=["general"]\n'));
 put(root,"specific/Cargo.toml",workspace(["models/*"]));
 for(const name of ["a","b"])put(root,"specific/models/"+name+"/Cargo.toml",pkg("model-"+name,'[package.metadata.semio.preparation]\nscript="../../📜️script.ts"\ncommand=["specific"]\n'));
 put(root,"product/Cargo.toml",workspace(["apps/*"]));put(root,"product/apps/app/Cargo.toml",pkg("product-app",'[dependencies]\nmodel-a={path="../../../specific/models/a"}\nneutral-kernel={path="../../../framework/kernel"}\n'));
 const source='import {appendFileSync} from "node:fs";import {join} from "node:path";appendFileSync(join(process.env.SEMIO_WORKSPACE_ROOT!,"events.jsonl"),JSON.stringify(process.argv.slice(2))+"\\n");';
 put(root,"📜️script.ts",source);put(root,"specific/📜️script.ts",source);
 prepareCargoOwners(root,cargoWorkspaceForManifest(root,"Cargo.toml"));expect(readFileSync(join(root,"events.jsonl"),"utf8")).toBe('["general"]\n');writeFileSync(join(root,"events.jsonl"),"");
 prepareCargoOwners(root,cargoWorkspaceForManifest(root,"product/Cargo.toml"));expect(readFileSync(join(root,"events.jsonl"),"utf8").trim().split("\n").sort()).toEqual(['["general"]','["specific"]']);
 const path=join(root,"framework/kernel/Cargo.toml");writeFileSync(path,readFileSync(path,"utf8").replace('../../📜️script.ts','../../../📜️script.ts'));expect(()=>prepareCargoOwners(root,cargoWorkspaceForManifest(root,"Cargo.toml"))).toThrow();
});



test("one membership invocation parses each current manifest once and observes later saves", () => {
  const law = JSON.parse(readFileSync(new URL("./🧫️fixtures/📇️invocation-inventory/🔣️.json", import.meta.url), "utf8"));
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
  const parser = spyOn(Bun.TOML, "parse").mockImplementation((input) => { const source = tomlText(input);
    calls.set(source, (calls.get(source) ?? 0) + 1);
    const result = original(source);
    documents.set(source, result);
    return result;
  });
  const check = (expected: readonly string[]): void => {
    calls.clear(); documents.clear();
    const rows = cargoWorkspaceMembers(root, owner);
    expect(rows.map(row => row.name)).toEqual([...expected]);
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
  const law = JSON.parse(readFileSync(new URL("./🧫️fixtures/📇️invocation-inventory/🔣️.json", import.meta.url), "utf8"));
  const root = mkdtempSync(join(artifactRoot!, "cargo-batch-inventory-"));
  const rootSource = workspace(["packages/*"]);
  put(root, "Cargo.toml", rootSource);
  const names = Array.from({ length: law.members }, (_, index) => `member-${String(index).padStart(3, "0")}`);
  for (let index = 0; index < law.members; index++) put(root, `packages/${String(index).padStart(3, "0")}/Cargo.toml`, pkg(names[index]!));
  const original = Bun.TOML.parse;
  let rootParses = 0;
  const parser = spyOn(Bun.TOML, "parse").mockImplementation((input) => { const source = tomlText(input); if (source.includes('owner-manifests=["*/Cargo.toml"]') && source.includes('member-manifests=["packages/*/Cargo.toml"]')) rootParses++; return original(source); });
  try {
    expect(cargoRepositoryPackageSelections(root, law.selectedIndices.map((index: number) => names[index])) .map(row => row.name)).toEqual(law.selectedIndices.map((index: number) => names[index]));
    expect(rootParses).toBeLessThanOrEqual(law.maxRepositoryParses);
  } finally { parser.mockRestore(); }
});

test("batch owner publication discovers one fresh repository inventory", () => {
  const law = JSON.parse(readFileSync(new URL("./🧫️fixtures/📇️invocation-inventory/🔣️.json", import.meta.url), "utf8"));
  const root = mkdtempSync(join(artifactRoot!, "cargo-batch-inventory-"));
  const rootSource = workspace(["packages/*"]);
  put(root, "Cargo.toml", rootSource);
  const names = Array.from({ length: law.members }, (_, index) => `member-${String(index).padStart(3, "0")}`);
  for (let index = 0; index < law.members; index++) put(root, `packages/${String(index).padStart(3, "0")}/Cargo.toml`, pkg(names[index]!));
  const original = Bun.TOML.parse;
  let rootParses = 0;
  const parser = spyOn(Bun.TOML, "parse").mockImplementation((input) => { const source = tomlText(input); if (source.includes('owner-manifests=["*/Cargo.toml"]') && source.includes('member-manifests=["packages/*/Cargo.toml"]')) rootParses++; return original(source); });
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
  const independent=glob.sync([...owner.memberManifests],{cwd:root,onlyFiles:true,followSymbolicLinks:false,ignore:law.exclude}).sort();
  expect(actual.map(row=>row.manifest)).toEqual(independent);
  expect(actual.map(row=>row.name)).toEqual(law.expectedNames);
  expect(matchers).toBeLessThanOrEqual(law.maximumMatchers);
 } finally {Object.defineProperty(Bun,"Glob",descriptor);}
});


test("membership discovery meters physical custody per member linearly in directory depth", () => {
 const law=fixture.custodyWork,base=fileURLToPath(new URL("./📏️custody-work/",import.meta.url)),perMember:number[]=[];
 for(const depth of law.depths){
  const root=mkdtempSync(join(artifactRoot!,"cargo-custody-work-")),chain=Array.from({length:depth},(_,index)=>"d"+index).join("/");
  put(root,"Cargo.toml",workspace(law.members));
  for(let index=0;index<law.breadth;index++)put(root,`packages/${chain}/leaf${index}/Cargo.toml`,pkg("leaf"+index));
  const child=Bun.spawnSync([process.execPath,"--preload",join(base,"🪝️preload.ts"),join(base,"📜️script.ts"),root,"walk"],{cwd:root,stdout:"pipe",stderr:"pipe"});
  expect(child.exitCode,child.stderr.toString()).toBe(0);
  const actual=JSON.parse(child.stdout.toString()) as {members:string[];lstat:number},independent=glob.sync(law.members.map((member:string)=>member+"/Cargo.toml"),{cwd:root,onlyFiles:true,followSymbolicLinks:false}).sort();
  expect(actual.members).toEqual(independent);
  perMember.push(actual.lstat/law.breadth);
 }
 expect(perMember[1]!/perMember[0]!).toBeLessThan(law.maximumGrowth);
},120_000);

const meteredWalk=(root:string,mode:"within"|"replay",within?:string):any=>{
 const base=fileURLToPath(new URL("./📏️custody-work/",import.meta.url)),child=Bun.spawnSync([process.execPath,"--preload",join(base,"🪝️preload.ts"),join(base,"📜️script.ts"),root,mode,...(within?[within]:[])],{cwd:root,stdout:"pipe",stderr:"pipe"});
 expect(child.exitCode,child.stderr.toString()).toBe(0);return JSON.parse(child.stdout.toString());
};
test("scoped membership discovery does not grow with unrelated siblings and equals the full walk within the scope", () => {
 const law=fixture.scopeWork,calls:number[]=[];
 for(const noise of law.noise){
  const root=mkdtempSync(join(artifactRoot!,"cargo-scope-work-"));put(root,"Cargo.toml",workspace(law.members));
  for(let index=0;index<law.targets;index++)put(root,`packages/focus/leaf${index}/Cargo.toml`,pkg("focus"+index));
  for(let index=0;index<noise;index++)put(root,`packages/noise/n${index}/Cargo.toml`,pkg("noise"+index));
  const actual=meteredWalk(root,"within","packages/focus"),independent=glob.sync(["packages/focus/*/Cargo.toml"],{cwd:root,onlyFiles:true,followSymbolicLinks:false}).sort();
  expect(actual.members).toEqual(independent);expect(actual.full).toEqual(independent);calls.push(actual.lstat+actual.readdir);
 }
 expect(calls[1]!/calls[0]!).toBeLessThan(law.maximumGrowth);
},120_000);

test("a seeded pass answers settled listings without reading directories and agrees with the cold walk", () => {
 const law=fixture.replayWork,root=mkdtempSync(join(artifactRoot!,"cargo-replay-work-"));put(root,"Cargo.toml",workspace(law.members));
 for(let index=0;index<law.leaves;index++)put(root,`packages/group${index%law.groups}/leaf${index}/Cargo.toml`,pkg("leaf"+index));
 const actual=meteredWalk(root,"replay"),independent=glob.sync(law.members.map((member:string)=>member+"/Cargo.toml"),{cwd:root,onlyFiles:true,followSymbolicLinks:false}).sort();
 expect(actual.cold).toEqual(independent);expect(actual.warm).toEqual(independent);
 expect(actual.coldReaddir).toBeGreaterThan(law.groups);expect(actual.warmReaddir).toBeLessThanOrEqual(law.maximumReplayedReaddirs);
},120_000);

test("explicit root-owned packages remain discoverable beneath a different native workspace",async()=>{
 const row=fixture.overlappingOwners,root=mkdtempSync(join(artifactRoot!,"workspace-overlap-"));put(root,"Cargo.toml",workspace(row.rootMembers).replace('member-manifests='+JSON.stringify(row.rootMembers.map((member:string)=>member+"/Cargo.toml")),'member-manifests='+JSON.stringify(row.rootAdmission)).replace('members='+JSON.stringify(row.rootMembers),'members='+JSON.stringify(row.rootMembers)+'\nexclude=["framework"]'));put(root,"framework/Cargo.toml",workspace(row.frameworkMembers).replace('members='+JSON.stringify(row.frameworkMembers),'members='+JSON.stringify(row.frameworkMembers)+'\nexclude=["product"]'));put(root,"framework/product/Cargo.toml",pkg(row.rootPackage).replace('[package]\n','[package]\nworkspace='+JSON.stringify(row.rootPackageWorkspace)+'\n'));put(root,"framework/product/🦀️.rs","pub fn product(){}\n");put(root,"framework/general/Cargo.toml",pkg(row.frameworkPackage));put(root,"framework/general/🦀️.rs","pub fn general(){}\n");
 const scopes=discoverCargoWorkspaces(root),packages=scopes.flatMap(scope=>cargoWorkspaceMembers(root,scope));expect(packages.map(row=>[row.name,row.workspace]).sort()).toEqual([[row.frameworkPackage,"framework"],[row.rootPackage,"."]].sort());for(const scope of scopes){const native=cargo(root,scope.manifest);expect(native.status,native.text).toBe(0);const metadata=JSON.parse(native.text);expect(metadata.workspace_members).toHaveLength(1);}
 expect(selectedCargoArguments(root,["check","-p",row.rootPackage])).toEqual(["check","--manifest-path",join(root,"framework/product/Cargo.toml"),"-p",row.rootPackage]);expect(selectedCargoArguments(root,["check","-p",row.frameworkPackage])).toEqual(["check","--manifest-path",join(root,"framework/general/Cargo.toml"),"-p",row.frameworkPackage]);
});


test("current physical workspace admissions and selected nextest profiles match independent native documents",async()=>{
 const nativeNextest=Bun.spawnSync([registeredToolExecutableV1("cargo"),"nextest","--version"],{stdout:"pipe",stderr:"pipe",timeout:10000}).exitCode===0;
 const root=process.cwd(),{repositoryCargoTestPolicyV1}=await import("../../../🏃️process/🦀️cargo/🔣️policy/🟦️.ts"),scopes=discoverCargoWorkspaces(root),packages=scopes.flatMap(scope=>{const members=cargoWorkspaceMembers(root,scope),document=TOML.parse(readFileSync(join(root,scope.manifest),"utf8")) as any;
  expect(members.map(row=>relative(join(root,scope.directory),join(root,row.directory)).replaceAll("\\","/")||".").sort()).toEqual([...document.workspace.members].sort());return members;});
 for(const target of fixture.policyPackages){const selected=packages.find(row=>row.name===target.name);expect(selected?.workspace).toBe(target.workspace);const configPath=join(root,target.workspace,".config/nextest.toml"),source=readFileSync(configPath,"utf8"),config=TOML.parse(source) as any;expect(Bun.TOML.parse(source)).toEqual(config);
  for(const [level,period]of Object.entries(fixture.profilePeriods)){expect(config.profile[level]["slow-timeout"]).toEqual({period,"terminate-after":1});const policy=repositoryCargoTestPolicyV1(join(root,selected!.manifest),root,{...process.env,SEMIO_TEST_LEVEL:level});expect(policy.configPath).toBe(configPath);expect<typeof level>(policy.level).toBe(level);expect(policy.nextest).toBe(nativeNextest);}
 }
},30000);


test("all-root dependency preparation honors explicit package workspaces beneath native child owners",async()=>{
 const row=fixture.overlappingOwners,root=mkdtempSync(join(artifactRoot!,"workspace-prepare-nested-"));
 put(root,"Cargo.toml",workspace(row.rootMembers,'[workspace.dependencies]\ngeneral={package="'+row.frameworkPackage+'",path="framework/general"}\n').replace('member-manifests='+JSON.stringify(row.rootMembers.map((m:string)=>m+"/Cargo.toml")),'member-manifests='+JSON.stringify(row.rootAdmission)).replace('[workspace]\n','[workspace]\nexclude=["framework"]\n'));
 put(root,"framework/Cargo.toml",workspace(row.frameworkMembers).replace('[workspace]\n','[workspace]\nexclude=["product"]\n'));put(root,"framework/product/Cargo.toml",pkg(row.rootPackage,'[dependencies]\ngeneral={workspace=true}\n').replace('[package]\n','[package]\nworkspace="'+row.rootPackageWorkspace+'"\n'));put(root,"framework/product/🦀️.rs","pub fn product() {}\n");put(root,"framework/general/Cargo.toml",pkg(row.frameworkPackage));put(root,"framework/general/🦀️.rs","pub fn general() {}\n");
 const native=cargo(root,"Cargo.toml");expect(native.status,native.text).toBe(0);expect(JSON.parse(native.text).workspace_members).toHaveLength(1);const tree=await captureCargoFixtureV1(root,["tree","--offline","--manifest-path",join(root,"Cargo.toml"),"-p",row.frameworkPackage]);expect(tree).toContain(row.frameworkPackage);
 expect(()=>prepareCargoOwners(root,discoverCargoWorkspaces(root).find(scope=>scope.directory===".")!)).not.toThrow();
});




test("reachable inherited development aliases match Cargo metadata without expanding production preparation",()=>{
 const law=JSON.parse(readFileSync(new URL("./🧫️fixtures/🎯️selected-preparation/🔣️.json",import.meta.url),"utf8")).inheritedAliases;
 for(const row of law.cases){
  const root=mkdtempSync(join(artifactRoot!,"cargo-alias-"));
  put(root,"Cargo.toml",workspace(["kernel"]));put(root,"kernel/Cargo.toml",pkg("neutral-kernel"));put(root,"kernel/🦀️.rs","");put(root,"application/Cargo.toml",workspace(["app"]));put(root,"application/app/Cargo.toml",pkg("selected-app",'[dependencies]\nowned-dependency={path="../../dependency/member"}\n'));put(root,"application/app/🦀️.rs","");
  put(root,"dependency/Cargo.toml",workspace(["member"],row.declared?'[workspace.dependencies]\n'+law.alias+'={path="../observer/member"}\n':""));put(root,"dependency/member/Cargo.toml",pkg("owned-dependency",'['+row.section+']\n'+law.alias+'={workspace=true}\n'));put(root,"dependency/member/🦀️.rs","");
  put(root,"observer/Cargo.toml",workspace(["member"]));put(root,"observer/member/Cargo.toml",pkg(law.alias));put(root,"observer/member/🦀️.rs","");
  const manifests=["Cargo.toml","application/Cargo.toml","dependency/Cargo.toml","observer/Cargo.toml"],before=manifests.map(path=>readFileSync(join(root,path),"utf8")),native=cargo(root,"application/Cargo.toml");expect(native.status===0,row.id+"\n"+native.text).toBe(row.valid);
  if(row.valid){const prepared=prepareCargoOwners(root,cargoWorkspaceForManifest(root,"application/Cargo.toml"),["selected-app"]);expect([...prepared.owners].sort()).toEqual(law.productionOwners);expect(JSON.parse(native.text).workspace_members).toHaveLength(1);}else expect(()=>prepareCargoOwners(root,cargoWorkspaceForManifest(root,"application/Cargo.toml"),["selected-app"])).toThrow("Missing inherited Cargo dependency");
  expect(manifests.map(path=>readFileSync(join(root,path),"utf8"))).toEqual(before);
 }
});


 test("controlled native owner resolution preserves original cancellation and independent Cargo ownership",async()=>{
 for(const row of fixture.controlledSelection){const root=mkdtempSync(join(artifactRoot!,"ctrl-"));put(root,"Cargo.toml",workspace(["framework/*"]));put(root,"framework/kernel/Cargo.toml",pkg("neutral-kernel"));put(root,"framework/kernel/🦀️.rs","pub fn kernel() {}\n");const controller=new AbortController(),reason=Error("neutral original cancellation");let completed=0;if(row.cancelAfter===0)controller.abort(reason);const control={signal:controller.signal,async advance(){completed++;if(row.cancelAfter===completed)controller.abort(reason);}};if(row.expectedOwner===null){await expect(cargoWorkspaceForManifestControlledV1(root,row.manifest,control)).rejects.toBe(reason);expect(completed).toBe(row.cancelAfter);}else{const selected=await cargoWorkspaceForManifestControlledV1(root,row.manifest,control);expect(selected.directory).toBe(row.expectedOwner);const independent=cargo(root,selected.manifest);expect(independent.status,independent.text).toBe(0);expect(JSON.parse(independent.text).workspace_root.replaceAll("\\","/")).toBe(root.replaceAll("\\","/"));expect(completed).toBeGreaterThan(0);}}
});
