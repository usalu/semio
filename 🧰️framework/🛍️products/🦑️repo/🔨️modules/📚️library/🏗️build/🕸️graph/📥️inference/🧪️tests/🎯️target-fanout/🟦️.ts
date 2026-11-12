/** 🎯️ Contract: one owner that declares many targets whose commands differ only in their arguments (the playground variants of the development package) stores the closure of their script once, every target gets the roster an independent Python walker finds, and the managed heap after a collection stays flat across the targets once each target releases what its owner left behind. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,readdirSync,rmSync,writeFileSync}from"node:fs";
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
const python=(root:string,entry:string)=>{const result=Bun.spawnSync([pythonTool,"-I",fileURLToPath(new URL("../🔗️late-closures/🐍️.py",import.meta.url)),root,entry,"[]"]);return JSON.parse(result.stdout.toString()) as {closure:string[]};};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

test("targets that differ only in their arguments share one stored closure value and leave a flat heap",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("fan-out-"),stage=await repository.scratch("fan-out-stage-");roots.push(root,stage);
 const names=Array.from({length:fixture.modules},(_,at)=>"m"+at+".mjs");
 mkdirSync(join(root,"owner"),{recursive:true});
 writeFileSync(join(root,"owner","📜️script.ts"),names.map(name=>'import "./'+name+'";\n').join(""));
 for(const[at,name]of names.entries())writeFileSync(join(root,"owner",name),"export const v"+at+" = \""+("x"+at+"-").repeat(Math.ceil(fixture.bytesPerModule/(String(at).length+2))).slice(0,fixture.bytesPerModule)+"\";\n");
 scriptInputs.resetClosureEntriesV1();
 const heaps:number[]=[];let stored=-1,rosterOf:string[]|undefined;
 await storage.withBuildGraphStagingV1({root:stage},async()=>{
  const shared=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(shared,root);
  for(let target=0;target<fixture.targets;target++){
   const spec={executor:"repo:script",options:{command:"repo owner/📜️script.ts publish variant"+target+" prototype"}};
   const handle=scriptInputs.closureHandleOfV1(await api.genericTargetCommandInputs(spec,"owner"));
   const roster=await observation.observeInferenceSnapshotV1(async()=>{const{values}=await api.resolve([[handle.key,handle.recipe]]);return(values.get(handle.key) as unknown[]).filter((path):path is string=>typeof path==="string"&&path.startsWith("{workspaceRoot}/owner/")).map(path=>path.replace("{workspaceRoot}/","")).sort();});
   if(rosterOf===undefined)rosterOf=roster;else expect(roster).toEqual(rosterOf);
   scriptInputs.releaseOwnerStateV1(shared);
   heaps.push(managed());
  }
  expect([shared.files,shared.closures,shared.commandDependencies,shared.declared,shared.paths].map(memo=>memo.size)).toEqual([0,0,0,0,0]);
  stored=readdirSync(join(scriptInputs.closureStoreDirectoryV1(root),"v")).length;
 });
 expect(rosterOf!).toEqual(python(root,"owner/📜️script.ts").closure.sort());
 expect(rosterOf!.length).toBe(fixture.modules+1);
 expect(stored).toBeLessThanOrEqual(fixture.maximumStoredValues);
 const tail=heaps.slice(fixture.warmup),growth=(tail[tail.length-1]!-tail[0]!)/MEGABYTE,mean=(values:number[])=>values.reduce((sum,value)=>sum+value,0)/values.length,xs=tail.map((_,at)=>at),mx=mean(xs),my=mean(tail),slope=xs.reduce((sum,x,at)=>sum+(x-mx)*(tail[at]!-my),0)/xs.reduce((sum,x)=>sum+(x-mx)**2,0)/1024;
 console.log("[DATA] target fan-out "+JSON.stringify({targets:fixture.targets,stored,growthMegabytes:Math.round(growth*10)/10,slopeKilobytesPerTarget:Math.round(slope),last:Math.round(heaps[heaps.length-1]!/MEGABYTE)}));
 expect(growth).toBeLessThan(fixture.maximumGrowthMegabytes);expect(slope).toBeLessThan(fixture.maximumSlopeKilobytesPerTarget);
}),600000);
