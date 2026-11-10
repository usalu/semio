/** 🧰️ Contract: the shared closure entries are derived once before the owners start, as durable memo nodes keyed by their recipe; every owner then only reads them, so the second owner derives no shared work, and every roster equals the one an independent Python walker and esbuild find. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,writeFileSync}from"node:fs";
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
const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const python=(root:string,entry:string)=>{const result=Bun.spawnSync([pythonTool,"-I",fileURLToPath(new URL("../🔗️late-closures/🐍️.py",import.meta.url)),root,entry,JSON.stringify(fixture.question)]);return JSON.parse(result.stdout.toString()) as {closure:string[]};};
const esbuildClosure=async(root:string,entry:string)=>{const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});return Object.keys(result.metafile.inputs).map((path:string)=>path.replace(/^\.\//u,"")).sort();};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

test("the prime derives the shared entries once and no owner derives shared work again",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("shared-prime-"),stage=await repository.scratch("shared-prime-stage-");roots.push(root,stage);write(root,fixture.files);
 scriptInputs.resetClosureEntriesV1();
 await storage.withBuildGraphStagingV1({root:stage},async()=>{
  const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),recipes=fixture.shared.map(entry=>({fn:"closure-of",entries:[entry],extras:[],role:"test-discovery"})),steps:string[]=[];
  const primed=await observation.observeInferenceSnapshotV1(()=>scriptInputs.primeSharedClosuresV1(root,scripts,{recipes,progress:({done,total}:{done:number;total:number})=>{steps.push(done+"/"+total);}}));
  expect(primed.recipes).toBe(fixture.primed.recipes);expect(primed.derived).toBe(fixture.primed.derived);expect(steps).toEqual(["0/"+recipes.length,...recipes.map((_,at)=>(at+1)+"/"+recipes.length)]);
  for(const[at,owner]of fixture.owners.entries()){
   const before=scriptInputs.closureEntryCountsV1().derived,ownerScripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(ownerScripts,root);
   const rosters=await observation.observeInferenceSnapshotV1(async()=>{
    const lists=[];for(const entry of owner.entries)lists.push(await api.closureOf([join(root,entry)],[],"test-discovery"));
    const handles=lists.map((list:any)=>scriptInputs.closureHandleOfV1(list)),{values}=await api.resolve(handles.map((handle:any)=>[handle.key,handle.recipe]));
    return handles.map((handle:any)=>(values.get(handle.key) as string[]).map(path=>path.replace("{workspaceRoot}/","")).sort());
   });
   expect(scriptInputs.closureEntryCountsV1().derived-before,owner.name+" derived").toBe(fixture.ownerDerived[at]);
   expect(rosters,owner.name).toEqual(owner.closures);
   for(const[index,entry]of owner.entries.entries()){expect(python(root,entry).closure.sort(),owner.name).toEqual(owner.closures[index]);expect(await esbuildClosure(root,entry),owner.name).toEqual(owner.closures[index]);}
  }
  const aborted=new AbortController();aborted.abort(Error("stopped"));
  await expect(scriptInputs.primeSharedClosuresV1(root,scripts,{recipes,signal:aborted.signal})).rejects.toThrow("stopped");
 });
}));
