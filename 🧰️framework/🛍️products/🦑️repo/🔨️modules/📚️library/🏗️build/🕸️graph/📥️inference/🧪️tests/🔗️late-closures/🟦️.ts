/** 🔗️ Contract: a project record names the command closure it used instead of carrying its rows. A new import in the closure moves the roster but leaves the record valid; the roster is the one the independent Python walker, esbuild and Bun's bundler find; a question whose answer moves makes the record stale; a process that starts later, without entries, finds the same record valid. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,writeFileSync}from"node:fs";
import{createRequire}from"node:module";
import{dirname,join,relative}from"node:path";
import{fileURLToPath}from"node:url";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

import{privateRepositoryWorkspaceV1}from"../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference"});
import{requireTestToolV1}from"../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
const pythonTool=requireTestToolV1("python");

const inference=await import("../../🟨️.mjs"),{cacheInternals,createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Record<string,string|undefined>>)=>{for(const[path,text]of Object.entries(files)){if(text===undefined)continue;mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const python=(root:string,entry:string)=>{const result=Bun.spawnSync([pythonTool,"-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),root,entry,JSON.stringify(fixture.question)]);return JSON.parse(result.stdout.toString()) as {closure:string[];answer:boolean};};
const esbuildClosure=async(root:string,entry:string)=>{const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});return Object.keys(result.metafile.inputs).map((path:string)=>path.replace(/^\.\//u,"")).sort();};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

test("a record names its closure; the closure moves without staling it unless the answer it was given moves",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("late-closure-");roots.push(root);
 write(root,fixture.files);
 const cachePath=join(root,"np","project.json"),entry=join(root,fixture.entry);
 let computations=0,rowsOfRecord=0;
 for(const step of fixture.steps){
  write(root,step.edit);if(step.forget)scriptInputs.resetClosureEntriesV1();
  const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),before=observation.projectInferenceCountsV1().project??0;
  const value=await observation.observeInferenceSnapshotV1(()=>observation.observeProjectInferenceV1({cachePath,key:"project",owner:"project"},async()=>{
   const list=await api.closureOf([entry],[],"test-discovery"),answer=await api.some(list,fixture.question);
   return{inputs:[...list],answer,...api.table([...list].map((item:any)=>item.closureRef))};
  }));
  computations=observation.projectInferenceCountsV1().project??0;
  expect(computations-before===0,step.name).toBe(step.reused);
  rowsOfRecord=(observation.projectInferenceObservationsV1(cachePath)??[]).length;
  expect(rowsOfRecord,step.name).toBeLessThan(40);
  const{values,rows}=await api.resolve(Object.entries(value.closureRecipes)),roster=(values.get(value.inputs[0].closureRef) as string[]).map(path=>path.replace("{workspaceRoot}/","")).sort();
  expect(roster,step.name).toEqual(step.closure);
  expect(rows.length,step.name).toBeGreaterThan(rowsOfRecord);
  const independent=python(root,fixture.entry);
  expect(roster,step.name).toEqual(independent.closure);
  expect(value.answer,step.name).toBe(step.answer);expect(independent.answer,step.name).toBe(step.answer);
  expect(await esbuildClosure(root,fixture.entry),step.name).toEqual(roster);
 }
}));

test("groups made from different recipes with the same files are merged and numbered as the direct inference numbers them",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("late-groups-");roots.push(root);write(root,fixture.files);
 const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),entry=join(root,fixture.entry);
 const first=await api.closureOf([entry],[],"test-discovery"),second=await api.closureOf([entry,join(root,"tools/a.mjs")],[],"test-discovery"),third=await api.closureOf([join(root,"tools/c.mjs")],[],"test-discovery");
 const marker=(list:any)=>scriptInputs.closureHandleOfV1(list);
 expect(marker(first).key).not.toBe(marker(second).key);
 const recipes=Object.fromEntries([first,second,third].map(list=>[marker(list).key,marker(list).recipe]));
 const project={name:"p",root:"p",namedInputs:{default:["{projectRoot}/**/*"],commandSources0:[marker(first).marker],commandSources1:[marker(second).marker],commandSources2:[marker(third).marker]},targets:{build:{inputs:["default","commandSources0","commandSources1","commandSources2"]}},closureRecipes:recipes,symbolicGroups:[["commandSources0",false],["commandSources1",false],["commandSources2",false]]};
 const assembly=await import("../../🧱️project-assembly/🟨️.mjs") as any,{project:bound,rows}=await assembly.resolveClosureRefsV1(project,root,scripts,"source");
 const direct=(await cacheInternals.relativeScriptInputs([entry],root,createScriptInputCache(),false)),other=(await cacheInternals.relativeScriptInputs([join(root,"tools/c.mjs")],root,createScriptInputCache(),false));
 expect(Object.keys(bound.namedInputs)).toEqual(["default","commandSources0","commandSources1"]);
 expect(bound.namedInputs.commandSources0).toEqual([...direct].sort());expect(bound.namedInputs.commandSources1).toEqual([...other].sort());
 expect(bound.targets.build.inputs).toEqual(["default","commandSources0","commandSources0","commandSources1"]);
 expect(rows.length).toBeGreaterThan(0);expect("closureRecipes"in bound||"symbolicGroups"in bound).toBe(false);
}));

test("the source groups of a TypeScript artifact package name their command closure too, and the files they resolve to are those of the direct inference",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("late-artifact-");roots.push(root);
 write(root,{"pkg/📦️packages/t/📋️project.json":"{}","pkg/📦️packages/t/📜️script.ts":"import \"./a.mjs\";\n","pkg/🟦️.ts":"import \"./b.mjs\";\n","pkg/a.mjs":"export const a=1;\n","pkg/b.mjs":"export const b=2;\n","package.json":"{}"});
 const json={name:"artifact",root:"pkg/📦️packages/t",tags:["role:artifact","language:typescript"],targets:{}},assembly=await import("../../🧱️project-assembly/🟨️.mjs") as any;
 const late=createScriptInputCache({workspaceRoot:root,lateClosures:true}),direct=createScriptInputCache({workspaceRoot:root});
 const symbolic=await assembly.projectInputs(json,"pkg/📦️packages/t",root,new Map(),late,"source"),plain=await assembly.projectInputs(json,"pkg/📦️packages/t",root,new Map(),direct,"source");
 expect(JSON.stringify(symbolic.artifactSources)).toContain("closureRef");expect(JSON.stringify(plain.artifactSources)).not.toContain("closureRef");
 const keys=[...new Set(JSON.stringify(symbolic).match(/[a-f0-9]{64}/gu)??[])].filter(key=>{try{scriptInputs.closureRecipeByKeyV1(key);return true;}catch{return false;}});
 const{values}=await scriptInputs.closureApiV1(late,root).resolve(keys.map((key:string)=>[key,scriptInputs.closureRecipeByKeyV1(key)]));
 const flat=(list:any[])=>list.flatMap(item=>item&&typeof item==="object"&&item.closureRef?values.get(item.closureRef):[item]);
 expect(flat(symbolic.artifactSources)).toEqual(plain.artifactSources);
 const expand=(list:any[]):string[]=>[...new Set(list.flatMap(item=>typeof item==="string"&&Array.isArray(plain[item])?expand(plain[item]):[JSON.stringify(item)]))].sort();
 expect(expand(flat(symbolic.artifactCommandSources))).toEqual(expand(plain.artifactCommandSources));
}));
