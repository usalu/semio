/** 🧊️ Contract: the policy of Nx engine code that never computes task bytes is closed by its schema and refuses forged tables; every prefix is a real directory; the keyed part of each engine family and its digest equal an independent Python oracle on the real closure; an edit of a file under a scheduling or engine-domain prefix leaves the digest of the family unchanged, an edit of output-producing code changes it, the Cargo-argument prefixes key only the `native` family, and a moved prefix re-keys. The cases live in `./🔣️.json`. */
import {afterAll,expect,test} from "bun:test";
import {createRequire} from "node:module";
import {cpSync,mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import Ajv from "ajv";
import {nxBuildEngineClosureV1,nxBuildEngineDigestV1,nxBuildEngineFilesDigestV1,nxBuildEngineInputsV1,projectBuildGraphForNxV1,projectBuildTargetForNxV1} from "../../../../../🟨️.mjs";
import {nxEngineNamedInputV1,nxEngineNativeNamedInputV1} from "../../🔏️implementation/🟨️.mjs";
import {buildSchedulingDomainOfV1,buildSchedulingExclusionPatternsV1,buildSchedulingExclusionsInputV1,buildSchedulingPrefixesV1,buildSchedulingV1} from "../../../../../🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts";
import schedulingPolicy from "../../../../../🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json";
import {nxEngineFamilyV1,nxEnginePolicyV1,nxEngineSpawnOnlyV1,nxEngineUnkeyedPrefixesV1,splitNxEngineFilesV1,type NxEngineFamilyV1} from "../🟦️.ts";
import {nxBuildEngineReferencesV1} from "../../../../../🟨️.mjs";
import declared from "../🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

const checkout=resolve(import.meta.dir,corpus.checkoutFromTests),closure=nxBuildEngineClosureV1(),families=corpus.families as NxEngineFamilyV1[];
type Draft={version:1;domains:{id:string;contract:{en:string;de:string};prefixes:string[]}[]};
const copy=():Draft=>JSON.parse(JSON.stringify(declared));
const refusal=(table:unknown):unknown=>{try{nxEnginePolicyV1(table);}catch(error){return (error as {key?:string}).key;}return undefined;};
const oracle=(root:string,family:string,files:readonly string[])=>{
 const run=Bun.spawnSync(["python3","-I",join(import.meta.dir,"🐍️oracle/🐍️.py")],{stdin:Buffer.from(JSON.stringify({root,family,files,schedulingPolicy,enginePolicy:declared}))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);return JSON.parse(run.stdout.toString()) as {keyed:string[];unkeyed:string[];digest:string};
};
const digestOf=(root:string,family:NxEngineFamilyV1)=>nxBuildEngineFilesDigestV1(root,splitNxEngineFilesV1(closure.files,family).keyed,nxEngineUnkeyedPrefixesV1(family));
const nativeKeyedPrefixes=schedulingPolicy.domains.flatMap(domain=>"nativeKeyed" in domain?(domain as {nativeKeyed:string[]}).nativeKeyed:[]);
const pick=(select:Record<string,unknown>,optional=false):string|undefined=>{
 const found=select.path!==undefined?closure.files.find(file=>file===select.path)
  :select.schedulingDomain!==undefined?closure.files.find(file=>buildSchedulingDomainOfV1(file)===select.schedulingDomain&&!nativeKeyedPrefixes.some(prefix=>file.startsWith(prefix)))
  :select.engineDomain!==undefined?closure.files.find(file=>nxEnginePolicyV1().domains.find(domain=>domain.id===select.engineDomain)!.prefixes.some(prefix=>file.startsWith(prefix)))
  :closure.files.find(file=>nativeKeyedPrefixes.some(prefix=>file.startsWith(prefix)));
 if(found===undefined&&optional)return undefined;expect(found,JSON.stringify(select)).toBeDefined();return found!;
};

test("the corpus and the policy are closed by their schemas",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
 expect(ajv.validate(schema,declared),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(schema,{...declared,extra:1})).toBe(false);
 expect(ajv.validate(schema,{...declared,domains:[]})).toBe(false);
 for(const domain of declared.domains){expect(domain.contract.en.length,domain.id).toBeGreaterThan(0);expect(domain.contract.de.length,domain.id).toBeGreaterThan(0);}
});

test("every forged policy is refused and the declared one is admitted once",()=>{
 expect(nxEnginePolicyV1()).toBe(nxEnginePolicyV1());
 const forged:Record<string,()=>unknown>={
  "duplicate-domain-id":()=>{const table=copy();table.domains=[table.domains[0]!,{...table.domains[1]!,id:table.domains[0]!.id}];return table;},
  "nested-prefix":()=>{const table=copy();const first=table.domains[0]!.prefixes[0]!;table.domains=[table.domains[0]!,{...table.domains[1]!,prefixes:[first+"inner/"]}];return table;},
  "duplicate-prefix":()=>{const table=copy();const first=table.domains[0]!.prefixes[0]!;table.domains=[table.domains[0]!,{...table.domains[1]!,prefixes:[first]}];return table;},
  "parent-segment":()=>{const table=copy();table.domains=[{...table.domains[0]!,prefixes:["a/../b/"]}];return table;},
  "extra-key":()=>({...copy(),extra:true}),
  "no-domains":()=>({...copy(),domains:[]}),
  "bad-id":()=>{const table=copy();table.domains=[{...table.domains[0]!,id:"Bad Id"}];return table;},
  "missing-german-contract":()=>{const table=copy();table.domains=[{...table.domains[0]!,contract:{en:"x"} as {en:string;de:string}}];return table;},
  "empty-prefixes":()=>{const table=copy();table.domains=[{...table.domains[0]!,prefixes:[]}];return table;},
  "spawn-extra-key":()=>{const table=copy() as Draft&{spawnOnly:Record<string,unknown>[]};table.spawnOnly=[{...declared.spawnOnly[0]!,extra:1}];return table;},
  "spawn-absolute-reference":()=>{const table=copy() as Draft&{spawnOnly:Record<string,unknown>[]};table.spawnOnly=[{...declared.spawnOnly[0]!,reference:"/etc/passwd"}];return table;},
  "spawn-missing-german-contract":()=>{const table=copy() as Draft&{spawnOnly:Record<string,unknown>[]};table.spawnOnly=[{...declared.spawnOnly[0]!,contract:{en:"x"}}];return table;}
 };
 expect(Object.keys(forged).sort()).toEqual([...corpus.forgedPolicies].sort());
 for(const [name,make] of Object.entries(forged))expect(refusal(make()),name).toBe("build.nx-engine.table-required");
 expect(()=>nxEngineFamilyV1("other")).toThrow();expect(nxEngineFamilyV1(undefined)).toBe("generic");
});

test("every engine prefix is a real directory and no scheduling prefix is declared twice",()=>{
 for(const domain of nxEnginePolicyV1().domains)for(const prefix of domain.prefixes)expect(statSync(join(checkout,prefix)).isDirectory(),prefix).toBe(true);
 const engine=new Set(nxEnginePolicyV1().domains.flatMap(domain=>domain.prefixes)),scheduling=buildSchedulingV1().domains.flatMap(domain=>domain.prefixes);
 for(const prefix of scheduling)expect(engine.has(prefix),prefix).toBe(false);
 for(const family of families){const prefixes=nxEngineUnkeyedPrefixesV1(family);expect(prefixes).toEqual([...prefixes].sort());expect(new Set(prefixes).size).toBe(prefixes.length);}
 expect(nxEngineUnkeyedPrefixesV1("native").length).toBeLessThan(nxEngineUnkeyedPrefixesV1("generic").length);
});

let copied:string|undefined;
const snapshot=():string=>{
 if(copied)return copied;const base=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();copied=mkdtempSync(join(base,"nx-engine-snapshot-"));
 for(const file of closure.files){const target=join(copied,file);mkdirSync(dirname(target),{recursive:true});cpSync(join(checkout,file),target);}
 return copied;
};
afterAll(()=>{if(copied)rmSync(copied,{recursive:true,force:true});});
for(const family of families)test(`the ${family} engine partition and digest of the real closure equal the independent Python oracle`,()=>{
 const copy=snapshot(),split=splitNxEngineFilesV1(closure.files,family),expected=oracle(copy,family,closure.files);
 expect(split.keyed).toEqual(expected.keyed);expect(split.unkeyed).toEqual(expected.unkeyed);
 expect(digestOf(copy,family)).toBe(expected.digest);
 expect(split.keyed.length+split.unkeyed.length).toBe(closure.files.length);
 expect(split.keyed.length).toBeLessThan(closure.files.length);expect(split.unkeyed.length).toBeGreaterThan(0);
 for(const entry of ["🧮️engine-digest/🟨️.mjs","🏃️execution/🟨️.mjs","📤️arguments/🟨️.mjs","🏗️build/📤️arguments/🟨️.mjs"])expect(split.keyed.some(file=>file.endsWith(entry)),entry).toBe(true);
 const script=join(checkout,closure.files.find(file=>file.endsWith("🧮️engine-digest/🟨️.mjs"))!);let stable=false;
 for(let attempt=0;attempt<3&&!stable;attempt++){
  const before=nxBuildEngineDigestV1(family),run=Bun.spawnSync([process.execPath,script,...(family==="generic"?[]:[family])],{cwd:checkout});
  expect(run.exitCode,run.stderr.toString()).toBe(0);
  if(nxBuildEngineDigestV1(family)===before){expect(run.stdout.toString().trim()).toBe(before);stable=true;}
 }
},120000);

test("the two families are two runtime inputs that differ only in the command argument",()=>{
 const generic=nxBuildEngineInputsV1("generic")[0] as {runtime:string},native=nxBuildEngineInputsV1("native")[0] as {runtime:string};
 expect(native.runtime).toBe(generic.runtime+" native");expect(nxBuildEngineDigestV1("native")).not.toBe(nxBuildEngineDigestV1("generic"));
 expect(JSON.stringify(nxBuildEngineInputsV1("native"))).toBe(JSON.stringify([native,...nxBuildEngineInputsV1("generic").slice(1)]).replace(JSON.stringify(generic),JSON.stringify(native)));
},60000);

test("an edit of scheduling or engine-domain code leaves the digest, an edit of output-producing code moves it, equal to the oracle on an edited copy",()=>{
 const root=snapshot();
 {
  const before=Object.fromEntries(families.map(family=>[family,digestOf(root,family)]));
  for(const row of corpus.edits){
   const path=pick(row.select,"optional" in row);if(path===undefined)continue;const target=join(root,path),original=readFileSync(target);
   writeFileSync(target,Buffer.concat([original,Buffer.from("\n// probe\n")]));
   try{for(const family of families){
    const after=digestOf(root,family),expected=row[family as "generic"|"native"];
    expect(after===before[family],`${row.role} ${family} ${path}`).toBe(expected==="unchanged");
    expect(after,`${row.role} ${family} oracle`).toBe(oracle(root,family,closure.files).digest);
   }}finally{writeFileSync(target,original);}
  }
  for(const family of families)expect(digestOf(root,family)).toBe(before[family]);
 }
},120000);

test("a prefix that moves in or out of the unkeyed set re-keys even when no file byte changes",()=>{
 const keyed=splitNxEngineFilesV1(closure.files,"generic").keyed,prefixes=nxEngineUnkeyedPrefixesV1("generic");
 const base=nxBuildEngineFilesDigestV1(checkout,keyed,prefixes);
 expect(nxBuildEngineFilesDigestV1(checkout,keyed,prefixes.slice(1))).not.toBe(base);
 expect(nxBuildEngineFilesDigestV1(checkout,keyed,[...prefixes,"zz/"])).not.toBe(base);
 expect(nxBuildEngineFilesDigestV1(checkout,keyed)).not.toBe(base);
 expect(nxBuildEngineFilesDigestV1(checkout,keyed,prefixes)).toBe(base);
},60000);

test("a command that compiles Rust (native owner) keys the native engine, every other command (also a process owner) the generic one, and both named inputs travel whole in every project",()=>{
 const generic=projectBuildTargetForNxV1({execution:{kind:"command",command:"bun x"},cache:true} as never),native=projectBuildTargetForNxV1({execution:{kind:"command",command:"repo x",ownerArguments:true,nativeOwnerCommand:"cargo"},cache:true} as never),processOwner=projectBuildTargetForNxV1({execution:{kind:"command",command:"repo y",ownerArguments:true},cache:true} as never),noop=projectBuildTargetForNxV1({execution:{kind:"noop"}} as never);
 expect(generic.inputs).toContain(nxEngineNamedInputV1);expect(generic.inputs).not.toContain(nxEngineNativeNamedInputV1);
 expect(native.inputs).toContain(nxEngineNativeNamedInputV1);expect(native.inputs).not.toContain(nxEngineNamedInputV1);
 expect(processOwner.inputs).toContain(nxEngineNamedInputV1);expect(processOwner.inputs).not.toContain(nxEngineNativeNamedInputV1);expect(noop.inputs).toBeUndefined();expect(nxEngineNamedInputV1).not.toBe(nxEngineNativeNamedInputV1);
 const portable={version:1,projects:{a:{name:"a",root:"a",targets:{build:{execution:{kind:"command",command:"bun x"},cache:true},cargo:{execution:{kind:"command",command:"repo x",ownerArguments:true,nativeOwnerCommand:"cargo"},cache:true}}}},dependencies:[],namedInputs:{}} as never,projected=projectBuildGraphForNxV1(portable).projects.a!;
 expect(projected.namedInputs[nxEngineNamedInputV1]).toEqual([...nxBuildEngineInputsV1()]);expect(projected.namedInputs[nxEngineNativeNamedInputV1]).toEqual([...nxBuildEngineInputsV1("native")]);
 for(const name of [nxEngineNamedInputV1,nxEngineNativeNamedInputV1])expect(()=>projectBuildGraphForNxV1({...(portable as object),namedInputs:{[name]:[]}} as never),name).toThrow("reserved");
});

test("a scheduling file among a target's inputs moves its real Nx task hash unless the target names schedulingSourceExclusions, an output-producing file always does (independent oracle: the Nx native hasher)",()=>{
 const sdk=createRequire(resolve(process.cwd(),".nx/installation/package.json")),native=sdk("nx/src/native"),transform=sdk("nx/src/native/transform-objects").transformProjectGraphForRust;
 const scheduling=buildSchedulingPrefixesV1()[0]+"probe.ts",producing="fixture/produce.ts",task={id:"fixture:build",target:{project:"fixture",target:"build"},overrides:{},outputs:[],projectRoot:"fixture",cache:true,parallelism:true},taskGraph={roots:[task.id],tasks:{[task.id]:task},dependencies:{[task.id]:[]},continuousDependencies:{[task.id]:[]}};
 const hash=(withExclusion:boolean,schedulingHash:string,producingHash:string)=>{
  const target=projectBuildTargetForNxV1({execution:{kind:"command",command:"repo ./📜️script.ts"},cache:true,inputs:[`{workspaceRoot}/${scheduling}`,`{workspaceRoot}/${producing}`,...(withExclusion?[buildSchedulingExclusionsInputV1]:[])]} as never);
  const graph={nodes:{fixture:{name:"fixture",type:"lib",data:{root:"fixture",targets:{build:target},namedInputs:{default:["{projectRoot}/**/*"],[buildSchedulingExclusionsInputV1]:[...buildSchedulingExclusionPatternsV1()],[nxEngineNamedInputV1]:[{runtime:"node -p \"'engine-fixed'\""}]}}}},externalNodes:{"npm:nx":{name:"npm:nx",type:"npm",data:{packageName:"nx",version:"23.2.0",hash:"nx-fixed"}},"npm:typescript":{name:"npm:typescript",type:"npm",data:{packageName:"typescript",version:"5.9.3",hash:"typescript-fixed"}}},dependencies:{fixture:[]}},graphRef=native.transferProjectGraph(transform(graph)),filemap=native.testOnlyTransferFileMap({fixture:[]},[{file:scheduling,hash:schedulingHash},{file:producing,hash:producingHash}]),planner=new native.HashPlanner({},graphRef),plans=planner.getPlansReference([task.id],taskGraph),hasher=new native.TaskHasher(process.cwd(),graphRef,filemap.projectFiles,filemap.allWorkspaceFiles,Buffer.from("{}"),{},undefined,{selectivelyHashTsConfig:false});
  return hasher.hashPlans(plans,{[task.id]:{}},process.cwd(),true)[task.id].value as string;
 };
 for(const withExclusion of [false,true]){
  const base=hash(withExclusion,"s1","p1"),schedulingEdit=hash(withExclusion,"s2","p1"),producingEdit=hash(withExclusion,"s1","p2");
  expect(hash(withExclusion,"s1","p1"),`stable ${withExclusion}`).toBe(base);
  expect(schedulingEdit===base,`scheduling edit unchanged with exclusion=${withExclusion}`).toBe(withExclusion);
  expect(producingEdit,`producing edit moves with exclusion=${withExclusion}`).not.toBe(base);
 }
});

test("a spawn-only reference is a real file reference of its owner, and cutting it removes the CLI from the engine closure while the executor, the plugin and the launcher itself stay",()=>{
 const library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
 for(const row of declared.spawnOnly){
  const source=readFileSync(join(checkout,row.owner),"utf8");
  expect(nxBuildEngineReferencesV1(source).some(reference=>reference.kind==="file-url"&&reference.specifier===row.reference),row.owner).toBe(true);
  expect(source.includes(row.reference),row.owner).toBe(true);expect(statSync(resolve(dirname(join(checkout,row.owner)),row.reference)).isFile()).toBe(true);
  expect(nxEngineSpawnOnlyV1(row.owner,row.reference)).toBe(true);expect(nxEngineSpawnOnlyV1(row.owner,row.reference+"x")).toBe(false);expect(nxEngineSpawnOnlyV1("other/"+row.owner,row.reference)).toBe(false);
  expect(row.contract.en.length).toBeGreaterThan(0);expect(row.contract.de.length).toBeGreaterThan(0);
 }
 const cut=new Set(closure.files),full=new Set(nxBuildEngineClosureV1({...declared,spawnOnly:[]} as never).files);
 for(const file of cut)expect(full.has(file),file).toBe(true);
 expect(full.size).toBeGreaterThan(cut.size*1.4);
 for(const name of corpus.spawnOnly.mustLeave){expect(full.has(library+name),"full "+name).toBe(true);expect(cut.has(library+name),"cut "+name).toBe(false);}
 for(const name of corpus.spawnOnly.mustStay)expect(cut.has(library+name),name).toBe(true);
},60000);
