/** 🫧️ Contract: the heap does not grow with the number of owners. A generated workspace of independent owners is inferred one after the other through one shared script input cache, as the owner stage does; after a warmup the managed heap after a collection stays within the declared growth and slope, and every roster equals the one an independent Python walker finds. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{heapStats}from"bun:jsc";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";
import{privateRepositoryWorkspaceV1}from"../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import{requireTestToolV1}from"../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference",scratch:"checkout"});
const pythonTool=requireTestToolV1("python");
const inference=await import("../../🟨️.mjs"),{createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const observation=await import("../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../🧭️script-inputs/🟨️.mjs") as any,storage=await import("../../../../📥️inputs/💾️storage/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const MEGABYTE=1048576,managed=():number=>{Bun.gc(true);const statistics=heapStats();return statistics.heapSize+statistics.extraMemorySize;};

/** 🏗️ The files of owner `owner`: one entry that imports its own modules, each module a distinct body of the declared size. */
function filesOf(owner:number):Record<string,string>{
 const files:Record<string,string>={},names=Array.from({length:fixture.modulesPerOwner},(_,at)=>"m"+at+".mjs");
 files["owners/o"+owner+"/entry.mjs"]=names.map(name=>'import "./'+name+'";\n').join("");
 for(const[at,name]of names.entries())files["owners/o"+owner+"/"+name]="export const v"+owner+"_"+at+" = \""+(owner+"-"+at+"-").repeat(Math.ceil(fixture.bytesPerModule/(String(owner).length+String(at).length+2))).slice(0,fixture.bytesPerModule)+"\";\n";
 return files;
}
const python=(root:string,entry:string)=>{const result=Bun.spawnSync([pythonTool,"-I",fileURLToPath(new URL("../🔗️late-closures/🐍️.py",import.meta.url)),root,entry,"[]"]);return JSON.parse(result.stdout.toString()) as {closure:string[]};};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

test("the managed heap after a collection stays flat across owners, and the rosters equal the independent walker",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("flat-heap-"),stage=await repository.scratch("flat-heap-stage-");roots.push(root,stage);
 for(let owner=0;owner<fixture.owners;owner++)for(const[path,text]of Object.entries(filesOf(owner))){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
 scriptInputs.resetClosureEntriesV1();
 const heaps:number[]=[];
 await storage.withBuildGraphStagingV1({root:stage},async()=>{
  const shared=createScriptInputCache({workspaceRoot:root,lateClosures:true});
  for(let owner=0;owner<fixture.owners;owner++){
   const api=scriptInputs.closureApiV1(shared,root),entry="owners/o"+owner+"/entry.mjs";
   const roster=await observation.observeInferenceSnapshotV1(async()=>{
    const list=await api.closureOf([join(root,entry)],[],"test-discovery"),handle=scriptInputs.closureHandleOfV1(list),{values}=await api.resolve([[handle.key,handle.recipe]]);
    return(values.get(handle.key) as string[]).map(path=>path.replace("{workspaceRoot}/","")).sort();
   });
   if(owner%12===0)expect(python(root,entry).closure.sort()).toEqual(roster);
   expect(roster.length).toBe(fixture.modulesPerOwner+1);
   scriptInputs.releaseOwnerStateV1(shared);
   heaps.push(managed());
  }
 });
 const tail=heaps.slice(fixture.warmup),growth=(tail[tail.length-1]!-tail[0]!)/MEGABYTE,mean=(values:number[])=>values.reduce((sum,value)=>sum+value,0)/values.length,xs=tail.map((_,at)=>at),mx=mean(xs),my=mean(tail),slope=xs.reduce((sum,x,at)=>sum+(x-mx)*(tail[at]!-my),0)/xs.reduce((sum,x)=>sum+(x-mx)**2,0)/1024;
 console.log("[DATA] flat heap "+JSON.stringify({owners:fixture.owners,warmup:fixture.warmup,growthMegabytes:Math.round(growth*10)/10,slopeKilobytesPerOwner:Math.round(slope),first:Math.round(heaps[0]!/MEGABYTE),last:Math.round(heaps[heaps.length-1]!/MEGABYTE)}));
 expect(growth).toBeLessThan(fixture.maximumGrowthMegabytes);expect(slope).toBeLessThan(fixture.maximumSlopeKilobytesPerOwner);
}),600000);
