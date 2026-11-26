/** 🧊️ Contract: the policy of the Nx engine digest is closed by its schema and refuses forged tables; the declared surface files exist and are part of the engine closure; the digest of each family equals an independent Python oracle on the real closure; an edit of any closure file outside the declared surface (graph reading, planning, capacity, leases, readiness, the fleet pool, cache custody, the process authority) leaves both digests unchanged, an edit of a surface file or of the declared task semantics revision moves them, and a Cargo argument builder moves only the native family. See `../🔣️.json` and `./🔖️task-semantics`. */
import {afterAll,expect,test} from "bun:test";
import {createRequire} from "node:module";
import {cpSync,mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import Ajv from "ajv";
import {nxBuildEngineClosureV1,nxBuildEngineDigestV1,nxBuildEngineFilesDigestV1,nxBuildEngineInputsV1,projectBuildGraphForNxV1,projectBuildTargetForNxV1} from "../../../../../🟨️.mjs";
import {nxEngineNamedInputV1,nxEngineNativeNamedInputV1} from "../../🔏️implementation/🟨️.mjs";
import {buildSchedulingExclusionPatternsV1,buildSchedulingExclusionsInputV1,buildSchedulingPrefixesV1} from "../../../../../🏗️build/📥️source/🔏️closure/🗓️scheduling/🟦️.ts";
import {nxEngineFamilyV1,nxEngineIdentityV1,nxEnginePolicyV1,nxEngineSpawnOnlyV1,nxEngineSurfaceV1,type NxEngineFamilyV1,type NxEnginePolicyV1} from "../🟦️.ts";
import {nxBuildEngineReferencesV1} from "../../../../../🟨️.mjs";
import declared from "../🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

const checkout=resolve(import.meta.dir,corpus.checkoutFromTests),closure=nxBuildEngineClosureV1(),families=corpus.families as NxEngineFamilyV1[],library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const copy=():NxEnginePolicyV1=>JSON.parse(JSON.stringify(declared));
const refusal=(table:unknown):unknown=>{try{nxEnginePolicyV1(table);}catch(error){return (error as {key?:string}).key;}return undefined;};
const oracle=(root:string,family:string,files:readonly string[],policy:unknown=declared)=>{
 const run=Bun.spawnSync(["python3","-I",join(import.meta.dir,"🐍️oracle/🐍️.py")],{stdin:Buffer.from(JSON.stringify({root,family,files,enginePolicy:policy}))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);return JSON.parse(run.stdout.toString()) as {covered:string[];digest:string};
};
const digestOf=(root:string,family:NxEngineFamilyV1)=>nxBuildEngineFilesDigestV1(root,nxEngineSurfaceV1(closure.files,family),nxEngineIdentityV1(family));
const pick=(select:{path?:string;prefix?:string}):string=>{
 const found=select.path!==undefined?closure.files.find(file=>file===library+select.path):closure.files.find(file=>file.startsWith(select.prefix!.startsWith("🧰️")?select.prefix!:library+select.prefix!)&&!file.includes("🧪️"));
 expect(found,JSON.stringify(select)).toBeDefined();return found!;
};

test("the corpus and the policy are closed by their schemas",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
 expect(ajv.validate(schema,declared),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(schema,{...declared,extra:1})).toBe(false);
 expect(ajv.validate(schema,{...declared,semantics:{...declared.semantics,surface:[]}})).toBe(false);
 expect(ajv.validate(schema,{...declared,semantics:{...declared.semantics,revision:0}})).toBe(false);
 expect(declared.semantics.contract.en.length).toBeGreaterThan(0);expect(declared.semantics.contract.de.length).toBeGreaterThan(0);
});

test("every forged policy is refused and the declared one is admitted once",()=>{
 expect(nxEnginePolicyV1()).toBe(nxEnginePolicyV1());
 const forged:Record<string,()=>unknown>={
  "extra-key":()=>({...copy(),extra:true}),
  "old-version":()=>({...copy(),version:1}),
  "no-surface":()=>{const table=copy() as any;table.semantics.surface=[];return table;},
  "duplicate-surface-file":()=>{const table=copy() as any;table.semantics.surface=[table.semantics.surface[0],table.semantics.surface[0]];return table;},
  "zero-revision":()=>{const table=copy() as any;table.semantics.revision=0;return table;},
  "fractional-revision":()=>{const table=copy() as any;table.semantics.revision=1.5;return table;},
  "parent-segment":()=>{const table=copy() as any;table.semantics.surface=["a/../b.ts"];return table;},
  "nested-prefix":()=>{const table=copy() as any;table.semantics.nativeSurface=[table.semantics.nativeSurface[0],table.semantics.nativeSurface[0]+"inner/"];return table;},
  "surface-file-below-native-prefix":()=>{const table=copy() as any;table.semantics.surface=[table.semantics.nativeSurface[0]+"a.ts"];return table;},
  "prefix-without-slash":()=>{const table=copy() as any;table.semantics.nativeSurface=["a/b"];return table;},
  "missing-german-contract":()=>{const table=copy() as any;table.semantics.contract={en:"x"};return table;},
  "spawn-extra-key":()=>{const table=copy() as any;table.spawnOnly=[{...declared.spawnOnly[0]!,extra:1}];return table;},
  "spawn-absolute-reference":()=>{const table=copy() as any;table.spawnOnly=[{...declared.spawnOnly[0]!,reference:"/etc/passwd"}];return table;},
  "spawn-missing-german-contract":()=>{const table=copy() as any;table.spawnOnly=[{...declared.spawnOnly[0]!,contract:{en:"x"}}];return table;}
 };
 expect(Object.keys(forged).sort()).toEqual([...corpus.forgedPolicies].sort());
 for(const [name,make] of Object.entries(forged))expect(refusal(make()),name).toBe("build.nx-engine.table-required");
 expect(()=>nxEngineFamilyV1("other")).toThrow();expect(nxEngineFamilyV1(undefined)).toBe("generic");
});

test("every surface file exists, every native prefix is a real directory, and no test file is covered",()=>{
 const {surface,nativeSurface}=nxEnginePolicyV1().semantics;
 expect(surface).toEqual([...surface].sort());expect(nativeSurface).toEqual([...nativeSurface].sort());
 for(const file of surface){expect(statSync(join(checkout,file)).isFile(),file).toBe(true);expect(file.split("/").includes("🧪️tests"),file).toBe(false);}
 for(const prefix of nativeSurface)expect(statSync(join(checkout,prefix)).isDirectory(),prefix).toBe(true);
 for(const family of families){const covered=nxEngineSurfaceV1(closure.files,family);expect(covered).toEqual([...covered].sort());expect(covered.some(file=>file.split("/").some(part=>nxEnginePolicyV1().semantics.excludedSegments.includes(part))),family).toBe(false);}
 expect(nxEngineSurfaceV1(closure.files,"generic").length).toBe(surface.length);
 expect(nxEngineSurfaceV1(closure.files,"native").length).toBeGreaterThan(surface.length);
 expect(nxEngineSurfaceV1(closure.files,"native").length).toBeLessThan(closure.files.length/4);
});

let copied:string|undefined;
const snapshot=():string=>{
 if(copied)return copied;const base=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();copied=mkdtempSync(join(base,"nx-engine-snapshot-"));
 for(const file of [...new Set([...closure.files,...nxEnginePolicyV1().semantics.surface])]){const target=join(copied,file);mkdirSync(dirname(target),{recursive:true});cpSync(join(checkout,file),target);}
 return copied;
};
afterAll(()=>{if(copied)rmSync(copied,{recursive:true,force:true});});
for(const family of families)test(`the ${family} engine surface and digest of the real closure equal the independent Python oracle`,()=>{
 const copy=snapshot(),covered=nxEngineSurfaceV1(closure.files,family),expected=oracle(copy,family,closure.files);
 expect(covered).toEqual(expected.covered);
 expect(digestOf(copy,family)).toBe(expected.digest);
 expect(covered.length).toBeLessThan(closure.files.length/10);
 for(const entry of ["🏃️execution/🟨️.mjs","📤️arguments/🟨️.mjs","🏗️build/📤️arguments/🟨️.mjs"])expect(covered.some(file=>file.endsWith(entry)),entry).toBe(true);
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


test("an edit outside the declared surface leaves both digests, an edit of a surface file or of the revision moves them, a Cargo argument builder moves only the native family, equal to the oracle on an edited copy",()=>{
 const root=snapshot(),before=Object.fromEntries(families.map(family=>[family,digestOf(root,family)]));
 for(const row of corpus.edits){
  const path=pick(row.select as {path?:string;prefix?:string}),target=join(root,path),original=readFileSync(target);
  writeFileSync(target,Buffer.concat([original,Buffer.from("\n// probe\n")]));
  try{for(const family of families){
   const after=digestOf(root,family),expected=row[family as "generic"|"native"];
   expect(after===before[family],`${row.role} ${family} ${path}`).toBe(expected==="unchanged");
   expect(after,`${row.role} ${family} oracle`).toBe(oracle(root,family,closure.files).digest);
  }}finally{writeFileSync(target,original);}
 }
 for(const family of families)expect(digestOf(root,family)).toBe(before[family]);
 const bumped:NxEnginePolicyV1={...declared,semantics:{...declared.semantics,revision:declared.semantics.revision+1}} as NxEnginePolicyV1;
 for(const family of families){
  const moved=nxBuildEngineFilesDigestV1(root,nxEngineSurfaceV1(closure.files,family,bumped),nxEngineIdentityV1(family,bumped));
  expect(moved,`revision ${family}`).not.toBe(before[family]);expect(moved).toBe(oracle(root,family,closure.files,bumped).digest);
 }
},120000);

test("the revision and the family re-key the digest even when no file byte changes",()=>{
 const files=nxEngineSurfaceV1(closure.files,"generic"),identity=nxEngineIdentityV1("generic"),base=nxBuildEngineFilesDigestV1(checkout,files,identity);
 expect(nxBuildEngineFilesDigestV1(checkout,files,["semantics 2","family generic"])).not.toBe(base);
 expect(nxBuildEngineFilesDigestV1(checkout,files,nxEngineIdentityV1("native"))).not.toBe(base);
 expect(nxBuildEngineFilesDigestV1(checkout,files)).not.toBe(base);
 expect(nxBuildEngineFilesDigestV1(checkout,files,identity)).toBe(base);
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
 expect(full.size-cut.size).toBeGreaterThanOrEqual(corpus.spawnOnly.mustLeave.length);
 for(const name of corpus.spawnOnly.mustLeave){expect(full.has(library+name),"full "+name).toBe(true);expect(cut.has(library+name),"cut "+name).toBe(false);}
 for(const name of corpus.spawnOnly.mustStay)expect(cut.has(library+name),name).toBe(true);
},60000);

test("the engine closure answered from the in-process memo equals a fresh computation in another process and a caller cannot change the memo",()=>{
 const first=nxBuildEngineClosureV1(),second=nxBuildEngineClosureV1();
 expect(second).toEqual(first);
 Reflect.set(first.files,"length",0);Reflect.set(first.packages,"length",0);
 const third=nxBuildEngineClosureV1();expect(third).toEqual(second);expect(third.files.length).toBeGreaterThan(0);
 const library=resolve(import.meta.dir,"../../../../../🟨️.mjs"),run=Bun.spawnSync([process.execPath,"-e",`const m=await import(${JSON.stringify(library)});console.log(JSON.stringify(m.nxBuildEngineClosureV1()));`],{env:{...process.env,BUN_OPTIONS:process.env.BUN_OPTIONS??""}});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 expect(JSON.parse(run.stdout.toString().trim().split("\n").at(-1)!)).toEqual(third);
},120000);
