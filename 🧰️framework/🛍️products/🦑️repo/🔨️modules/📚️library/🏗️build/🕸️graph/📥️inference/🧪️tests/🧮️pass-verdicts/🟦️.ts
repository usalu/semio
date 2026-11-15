/** 🧮️ Contract: inside one inference pass every row that the layers of many closure entries name is probed once, however many owners ask for those entries and however many layers repeat the row; outside a pass nothing is remembered, so the same questions probe again. The expected counts come from an independent Python model of the scenario. @see ../../🧭️script-inputs/🟨️.mjs */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{privateRepositoryWorkspaceV1}from"../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference",scratch:"checkout"});
import{requireTestToolV1}from"../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
const pythonTool=requireTestToolV1("python");

const inference=await import("../../🟨️.mjs") as any;
await inference.libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any,entries=await import("../../🧭️script-inputs/💾️entries/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

const DIRECTORIES=40,ENTRIES=12,LAYERS_PER_ENTRY=6;
const layerRows=(entry:number,layer:number)=>Array.from({length:DIRECTORIES-2*layer},(_,at)=>(at*7+entry*layer)%DIRECTORIES);
const scenario=()=>({entries:Array.from({length:ENTRIES},(_,entry)=>Array.from({length:LAYERS_PER_ENTRY},(_,layer)=>layerRows(entry,layer)))});

const prepare=async(prefix:string)=>{
 const root=await repository.scratch(prefix),store=await repository.scratch(prefix+"store-");roots.push(root,store);
 const rows:unknown[]=[];
 for(let at=0;at<DIRECTORIES;at++){
  const directory=join(root,"d"+at);mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"f"),String(at));
  const detached=await observation.observeDetachedV1(()=>observation.observedFs.readdirSync(directory));
  rows.push(detached.rows[0]);
 }
 const writer=entries.createClosureStoreV1({directory:store}),recipes:unknown[]=[];
 for(const[at,layers]of scenario().entries.entries()){
  const recipe={fn:"closure-of",entries:["e"+at],extras:[],role:"test-discovery"};recipes.push(recipe);
  await writer.save(root+"\0"+scriptInputs.closureRecipeKeyV1(recipe),{recipe,value:[],rows:[],layers:layers.map(layer=>layer.map(row=>rows[row]))});
 }
 return{root,store,recipes};
};

const run=async(prefix:string,inPass:boolean)=>{
 const{root,store,recipes}=await prepare(prefix);
 scriptInputs.resetClosureEntriesV1();scriptInputs.bindClosureStoreV1(root,store,false);
 const before=observation.observationProbeCountsV1().readdir,owners=Array.from({length:ENTRIES/2},(_,at)=>recipes.slice(at*2,at*2+2));
 const pass=async()=>{for(const owner of owners)await observation.withObservationRoundV1(()=>scriptInputs.primeSharedClosuresV1(root,{},{recipes:owner}));};
 if(inPass)await scriptInputs.withClosurePassV1(pass);else await pass();
 return{probes:observation.observationProbeCountsV1().readdir-before,loaded:scriptInputs.closureEntryCountsV1().loaded,stale:scriptInputs.closureEntryCountsV1().stale};
};

const model=()=>{
 const result=Bun.spawnSync([pythonTool,"-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{stdin:Buffer.from(JSON.stringify(scenario()))});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 return JSON.parse(result.stdout.toString()) as {distinctRows:number;rowReferences:number;distinctLayers:number;entries:number};
};

test("inside a pass the owners of many entries probe every distinct row once",async()=>repository.invoke(async()=>{
 const expected=model(),found=await run("pass-verdicts-pass-",true);
 expect(found.stale).toBe(0);expect(found.loaded).toBe(expected.entries);
 expect(found.probes).toBe(expected.distinctRows);
 expect(expected.rowReferences).toBeGreaterThan(3*expected.distinctRows);
}));

test("outside a pass every owner probes the rows of its layers again",async()=>repository.invoke(async()=>{
 const expected=model(),found=await run("pass-verdicts-plain-",false);
 expect(found.loaded).toBe(expected.entries);
 expect(found.probes).toBeGreaterThan(expected.distinctRows);
}));
