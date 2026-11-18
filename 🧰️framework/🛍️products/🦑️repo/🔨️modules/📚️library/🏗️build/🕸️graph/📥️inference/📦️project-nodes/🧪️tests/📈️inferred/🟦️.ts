/** 📈️ Contract: a wave of the graph read counts the projects it had to infer: a project whose stored record holds is a hit and calls neither the farm preparation nor the inference, a project whose record does not hold calls them, and the count reaches the build as `onInferred` so that the build can let a wave that inferred nothing grow. */
import{afterAll,expect,test}from"bun:test";
import{readFileSync,rmSync}from"node:fs";
import{join}from"node:path";
import{privateRepositoryWorkspaceV1}from"../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference-projects",scratch:"checkout"});
const inference=await import("../../../🟨️.mjs"),{libraryBootstrap}=inference as any;
await libraryBootstrap;
const observation=await import("../../../../../👁️observation/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

test("a hit calls neither the preparation nor the inference, a changed key calls both",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("inferred-");roots.push(root);
 const calls={prepare:0,compute:0},cachePath=join(root,"np","project.json");
 const observe=(key:string)=>observation.observeInferenceSnapshotV1(()=>observation.observeProjectInferenceV1({cachePath,key,owner:"project",prepare:()=>{calls.prepare++;return false;}},async()=>{calls.compute++;return{name:"project",key};}));
 await observe("one");expect(calls).toEqual({prepare:1,compute:1});
 await observe("one");expect(calls).toEqual({prepare:1,compute:1});
 await observe("two");expect(calls).toEqual({prepare:2,compute:2});
}),120000);

test("the project loop counts a project once, in the preparation and in the inference, and tells the build",()=>{
 const text=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8");
 expect(text).toContain("const missed=new Set();");
 expect(text).toContain("prepare:farm?()=>{missed.add(name);return stagedPrepareV1(name,away,()=>farm.prepare(");
 expect(text).toContain('()=>{missed.add(name);if(farm?.exists?.())throw farmJobRefusedV1(name,root,"the owner (the farm gave no record)");return inferProjectBodyV1(projectEnvironment,');
 expect(text).toContain("options?.onInferred?.(missed.size);");
 expect(text).toContain("options?.onPhases?.({setup:Math.round(loopStartedMs-enteredMs),projects:Math.round(performance.now()-loopStartedMs)});");
});

test("the discovery hands the count and the stages of the part to the build",()=>{
 const text=readFileSync(new URL("../../../../🧭️discovery/🟦️.ts",import.meta.url),"utf8");
 expect(text).toContain("onInferred:(count:number)=>{inferred=count;}");
 expect(text).toContain("onPhases:(times:Record<string,number>)=>{Object.assign(phases,times);}");
 expect(text).toContain("...(inferred===undefined?{}:{inferred}),phases:{...phases,tail:Math.round(performance.now()-tailStartedMs)},");
});
