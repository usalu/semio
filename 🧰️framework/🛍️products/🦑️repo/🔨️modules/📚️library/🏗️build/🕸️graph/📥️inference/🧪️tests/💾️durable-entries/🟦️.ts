/** 💾️ Contract: a closure entry is a durable memo node. A second worker over the same store derives no entry, an edit to the input of one entry derives only that entry, a damaged record costs only its own derivation, and every roster equals the one an independent Python walker and esbuild find. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,readdirSync,rmSync,writeFileSync}from"node:fs";
import{createRequire}from"node:module";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";
import{privateRepositoryWorkspaceV1}from"../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference",scratch:"checkout"});
import{requireTestToolV1}from"../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
const pythonTool=requireTestToolV1("python");

const inference=await import("../../🟨️.mjs"),{createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any,storage=await import("../../../../📥️inputs/💾️storage/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Record<string,string|undefined>>)=>{for(const[path,text]of Object.entries(files)){if(text===undefined)continue;mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const python=(root:string,entry:string)=>{const result=Bun.spawnSync([pythonTool,"-I",fileURLToPath(new URL("../🔗️late-closures/🐍️.py",import.meta.url)),root,entry,JSON.stringify(fixture.question)]);return JSON.parse(result.stdout.toString()) as {closure:string[]};};
const esbuildClosure=async(root:string,entry:string)=>{const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});return Object.keys(result.metafile.inputs).map((path:string)=>path.replace(/^\.\//u,"")).sort();};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

test("closure entries are durable memo nodes vouched by their observed rows",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("durable-entries-"),stage=await repository.scratch("durable-stage-");roots.push(root,stage);write(root,fixture.files);
 const store=join(stage,"inference","ce","e");scriptInputs.resetClosureEntriesV1();
 await storage.withBuildGraphStagingV1({root:stage},async()=>{
 for(const step of fixture.steps){
  write(root,step.edit);
  if(step.forget)scriptInputs.resetClosureEntriesV1();
  if(step.damage){const id=readdirSync(store).filter((name:string)=>name.endsWith(".json.br"));expect(id.length).toBeGreaterThan(0);}
  if(step.damage)for(const name of readdirSync(store).slice(0,1))writeFileSync(join(store,name),"damaged");
  const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root);
  const rosters=await observation.observeInferenceSnapshotV1(async()=>{
   const lists=[];for(const entry of fixture.entries)lists.push(await api.closureOf([join(root,entry)],[],"test-discovery"));
   for(const list of lists)await api.some(list,fixture.question);
   const handles=lists.map((list:any)=>scriptInputs.closureHandleOfV1(list)),{values}=await api.resolve(handles.map((handle:any)=>[handle.key,handle.recipe]));
   return handles.map((handle:any)=>(values.get(handle.key) as string[]).map(path=>path.replace("{workspaceRoot}/","")).sort());
  });
  const counts=scriptInputs.closureEntryCountsV1();
  expect(counts.derived,step.name+" derived").toBe(step.derived);
  expect(counts.loaded,step.name+" loaded").toBe(step.loaded);
  expect(counts.stale,step.name+" stale").toBe(step.stale);
  expect(rosters,step.name).toEqual(step.closures);
  for(const[at,entry]of fixture.entries.entries()){expect(python(root,entry).closure.sort(),step.name).toEqual(step.closures[at]);expect(await esbuildClosure(root,entry),step.name).toEqual(step.closures[at]);}
 }
});
}));

test("a loaded entry gives exactly the support rows, walk facts and resolution of a freshly derived one",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("durable-differential-"),stage=await repository.scratch("durable-diff-stage-");roots.push(root,stage);
 const{entry,modules,extras}=fixture.differential,names=Array.from({length:modules},(_,at)=>"tools/m"+at+".mjs");
 write(root,{[entry]:names.map(name=>'import "./'+name.slice(6)+'";\n').join(""),...Object.fromEntries(names.map((name,at)=>[name,"export const m"+at+" = "+at+";\n"]))});
 await storage.withBuildGraphStagingV1({root:stage},async()=>{
  const image=async()=>{
   const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root);
   return observation.observeInferenceSnapshotV1(async()=>{
    const lists=[];for(const more of [[],extras])lists.push(await api.closureOf([join(root,entry)],more,"artifact-command"));
    const handles=lists.map((list:any)=>scriptInputs.closureHandleOfV1(list)),resolved=await api.resolve(handles.map((handle:any)=>[handle.key,handle.recipe]));
    return JSON.stringify({values:[...resolved.values].sort(),rows:resolved.rows,derived:resolved.derived.map((item:any)=>({row:item.row,support:item.support}))});
   });
  };
  scriptInputs.resetClosureEntriesV1();
  const derived=await image(),countsDerived=scriptInputs.closureEntryCountsV1();
  scriptInputs.resetClosureEntriesV1();
  const loaded=await image(),countsLoaded=scriptInputs.closureEntryCountsV1();
  expect(countsDerived.derived).toBe(2);expect(countsLoaded.derived).toBe(0);expect(countsLoaded.loaded).toBe(2);
  expect(JSON.parse(derived).derived.length).toBeGreaterThan(0);
  expect(loaded).toBe(derived);
 });
}));

test("resolving into a row log gives the rows, supports and values of the arrays for derived and for loaded entries, and the loaded entry reads its layers from the store",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("durable-log-"),stage=await repository.scratch("durable-log-stage-");roots.push(root,stage);
 const{entry,modules,extras}=fixture.differential,names=Array.from({length:modules},(_,at)=>"tools/m"+at+".mjs"),{createRowLogV1}=await import("../../../🕰️generations/🗃️row-log/🟨️.mjs") as any;
 write(root,{[entry]:names.map(name=>'import "./'+name.slice(6)+'";\n').join(""),...Object.fromEntries(names.map((name,at)=>[name,"export const m"+at+" = "+at+";\n"]))});
 await storage.withBuildGraphStagingV1({root:stage},async()=>{
  const image=async(log?:any)=>{
   const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root);
   return observation.observeInferenceSnapshotV1(async()=>{
    const lists=[];for(const more of [[],extras])lists.push(await api.closureOf([join(root,entry)],more,"artifact-command"));
    const handles=lists.map((list:any)=>scriptInputs.closureHandleOfV1(list)),resolved=await api.resolve(handles.map((handle:any)=>[handle.key,handle.recipe]),log);
    return JSON.stringify({values:[...resolved.values].sort(),rows:[...resolved.rows],derived:resolved.derived.map((item:any)=>({row:item.row,support:[...item.support]}))});
   });
  };
  const log=createRowLogV1({path:join(stage,"rl","durable.rows")});
  try{
   scriptInputs.resetClosureEntriesV1();
   const derivedArrays=await image();scriptInputs.resetClosureEntriesV1();
   const derivedLog=await image(log);
   expect(derivedLog).toBe(derivedArrays);
   scriptInputs.resetClosureEntriesV1();
   const loadedArrays=await image(),loadedCounts=scriptInputs.closureEntryCountsV1();scriptInputs.resetClosureEntriesV1();
   const loadedLog=await image(log);
   expect(loadedCounts.loaded).toBe(2);expect(loadedCounts.derived).toBe(0);
   expect(loadedLog).toBe(loadedArrays);expect(loadedArrays).toBe(derivedArrays);
   expect(log.stats().rows).toBeGreaterThan(0);
  }finally{log.close();}
 });
}));
