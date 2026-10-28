/** 🚧️ Contract: a project withheld because a source of its command closure does not parse holds the facts that withhold it: the bytes of the failing source and everything the failed walk read. Repairing the source, or creating the module it could not find, makes those facts stale, so the next run sees the repair; unrelated files leave them valid. The independent oracles are esbuild (the closure resolves or not) and Python's `hashlib` (the bytes the project holds). */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{createRequire}from"node:module";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

const inference=await import("../../🟨️.mjs"),{createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Partial<Record<string,string>>>)=>{for(const[path,text]of Object.entries(files)){if(text===undefined)continue;mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const sha=(path:string)=>Bun.spawnSync(["python3","-I","-c","import hashlib,sys;print(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",path]).stdout.toString().trim();
const bundles=async(root:string)=>{try{await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,"proj/📜️script.ts")],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",logLevel:"silent",outfile:"out.js"});return true;}catch{return false;}};

const withhold=async(root:string)=>{
 scriptInputs.resetClosureEntriesV1();
 const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root),cachePath=join(root,"np","project.json");
 const value=await observation.observeInferenceSnapshotV1(()=>observation.observeProjectInferenceV1({cachePath,key:"project",owner:"project"},async()=>{
  try{await api.targetClosureSome(fixture.target,"proj",["x"]);return{withheld:false};}
  catch(error:any){if(error.code!=="BUILD_SOURCE_UNPARSEABLE")throw error;return{withheld:true,source:error.source,sha256:error.sha256};}
 }));
 return{value,rows:(observation.projectInferenceObservationsV1(cachePath)??[]) as {kind:string;path:string;fact:any}[]};
};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

for(const scenario of fixture.scenarios)test("a withheld project "+scenario.name+(scenario.holds?" and its facts still hold":" and its facts go stale"),async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"withheld-rows-"));roots.push(root);write(root,fixture.files);
 const failing=join(root,fixture.failing);
 expect(await bundles(root),"esbuild cannot bundle the broken closure").toBe(false);
 await new Promise(done=>setTimeout(done,2200));
 const{value,rows}=await withhold(root);
 expect(value.withheld).toBe(true);expect(value.source).toBe(failing);
 const own=rows.filter(row=>row.kind==="file"&&row.path===failing);
 expect(own.length,"the project holds the bytes of the failing source").toBeGreaterThan(0);
 expect(own.every(row=>row.fact.sha256===sha(failing)&&row.fact.sha256===value.sha256)).toBe(true);
 expect(rows.some(row=>row.kind==="file"&&row.path===join(root,"proj/📜️script.ts")),"and the files the failed walk read before it").toBe(true);
 expect(await observation.observationsHold(rows)).toBe(true);
 write(root,scenario.edit);
 expect(await observation.observationsHold(rows),scenario.name).toBe(scenario.holds);
 expect(await bundles(root),"esbuild agrees on whether the closure resolves").toBe(scenario.resolves);
},60_000);

test("a stored failure is never reused, a stored node is: the next inference after a repair is computed again",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"withheld-reuse-"));roots.push(root);
 const nodes=await import("../../📦️project-nodes/🟨️.mjs") as any;
 const run=async(name:string,value:unknown)=>{
  const cachePath=join(root,"np",name+".json"),count=()=>observation.projectInferenceCountsV1()[name]??0;
  for(let round=0;round<2;round++)await observation.observeInferenceSnapshotV1(()=>observation.observeProjectInferenceV1({cachePath,key:name,owner:name,reusable:nodes.projectRecordReusableV1},async()=>value));
  return count();
 };
 expect(await run("failure",{unavailable:{root:"x",source:"x/y.ts",sha256:"0",reason:"does not parse"}})).toBe(2);
 expect(await run("node",{name:"node",root:"x",targets:{}})).toBe(1);
});
