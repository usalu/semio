/** 🛟️ Contract: a whole read does not die on the stable failure of one owner: the failure becomes the withheld-source failure of that owner (named, English and German), the observations of the failed inference (declaration and both route tables) stay with it, and editing any of them makes them stale so the owner is inferred again. Failures another layer follows (system errors, cancellation, restart codes) are never withheld. */
import{afterAll,expect,test}from"bun:test";
import{existsSync,mkdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{canonicalTestAggregatePolicyV1 as aggregate,deriveCanonicalTestAggregateV1,requireCanonicalTestAggregateV1}from"../../../../../../⚡️caching/🗂️suites/🧮️aggregate/🟦️.ts";
import{privateRepositoryWorkspaceV1}from"../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference-projects",scratch:"checkout"});

const inference=await import("../../../🟨️.mjs") as any;
await inference.libraryBootstrap;
const observation=await import("../../../../../👁️observation/🟨️.mjs") as any,nodes=await import("../../🟨️.mjs") as any;
const workspace=(()=>{for(let directory=import.meta.dirname;directory!==dirname(directory);directory=dirname(directory))if(existsSync(join(directory,aggregate.manifest)))return directory;throw Error("workspace not found");})();
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

const setup=async()=>{
 const root=await repository.scratch("owner-withheld-");roots.push(root);
 const put=(path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
 for(const path of [aggregate.cacheTable,aggregate.buildTable,aggregate.manifest])put(path,readFileSync(join(workspace,path),"utf8"));
 const build=JSON.parse(readFileSync(join(root,aggregate.buildTable),"utf8")),manifest=JSON.parse(readFileSync(join(root,aggregate.manifest),"utf8")),id="zz-whole-read",donor=build.suites.find((row:{id:string})=>row.id==="build-test-file")??build.suites[0];
 const original=readFileSync(join(root,aggregate.buildTable),"utf8"),originalManifest=readFileSync(join(root,aggregate.manifest),"utf8");
 const editTable=()=>put(aggregate.buildTable,JSON.stringify({...build,suites:[...build.suites,{...donor,id}]}));
 const revertTable=()=>put(aggregate.buildTable,original);
 const editManifest=()=>{
  const target=structuredClone(manifest.targets["test-"+donor.id]);target.options.command=String(target.options.command).replace(donor.id,id);
  const next={...manifest,targets:{...manifest.targets,["test-"+id]:target}};
  next.targets[aggregate.target]=deriveCanonicalTestAggregateV1({...next,root:aggregate.root},path=>readFileSync(join(root,path),"utf8"));
  put(aggregate.manifest,JSON.stringify(next,null,2)+"\n");
 };
 const cachePath=join(root,"np","owner.json");
 const infer=()=>observation.observeInferenceSnapshotV1(()=>observation.observeProjectInferenceV1({cachePath,key:"owner",owner:"owner"},async()=>{
  const read=(path:string)=>observation.observedFs.readFileSync(join(root,path),"utf8");
  try{requireCanonicalTestAggregateV1(JSON.parse(read(aggregate.manifest)),aggregate.root,read);return{withheld:false} as any;}
  catch(raised){
   const error=nodes.ownerFailureAsWithheldV1(raised,{workspaceRoot:root,root:aggregate.root,name:"repo",aborted:false});
   if(error===undefined)throw raised;
   observation.observedFs.readFileSync(error.source);
   return{withheld:true,message:error.message,source:error.source,code:error.code,cause:String(error.cause?.message)} as any;
  }
 }));
 const rows=()=>(observation.projectInferenceObservationsV1(cachePath)??[]) as {kind:string;path:string}[];
 return{root,infer,rows,editTable,editManifest,revertTable,put,originalManifest};
};

test("a stable owner failure of a whole read is withheld by name, bilingual, with the table rows it read",async()=>repository.invoke(async()=>{
 const state=await setup();
 expect((await state.infer()).withheld).toBe(false);
 state.editTable();
 await new Promise(done=>setTimeout(done,2200));
 const value=await state.infer();
 expect(value.withheld).toBe(true);
 expect(value.code).toBe("BUILD_SOURCE_UNPARSEABLE");
 expect(value.source).toBe(join(state.root,aggregate.manifest));
 expect(value.message).toContain("repo");
 expect(value.message).toContain(" / ");
 expect(value.message).toContain("kann nicht abgeleitet werden");
 expect(value.cause).toContain("route-targets");
 const paths=new Set(state.rows().filter(row=>row.kind==="file").map(row=>row.path));
 for(const path of [aggregate.manifest,aggregate.cacheTable,aggregate.buildTable])expect(paths.has(join(state.root,path)),path).toBe(true);
 expect(await observation.observationsHold(state.rows())).toBe(true);
}),60_000);

test("editing the table input of a withheld owner, or its declaration, makes the withholding stale and the owner is inferred again",async()=>repository.invoke(async()=>{
 const state=await setup();
 state.editTable();
 await new Promise(done=>setTimeout(done,2200));
 expect((await state.infer()).withheld).toBe(true);
 const withheldRows=state.rows();
 state.revertTable();
 expect(await observation.observationsHold(withheldRows),"the table row went stale").toBe(false);
 expect((await state.infer()).withheld).toBe(false);
 state.editTable();
 await new Promise(done=>setTimeout(done,2200));
 expect((await state.infer()).withheld).toBe(true);
 const again=state.rows();
 state.editManifest();
 expect(await observation.observationsHold(again),"the declaration row went stale").toBe(false);
 expect((await state.infer()).withheld).toBe(false);
}),60_000);

test("failures another layer follows are never withheld",async()=>repository.invoke(async()=>{
 const state=await setup(),scope={workspaceRoot:state.root,root:aggregate.root,name:"repo",aborted:false};
 expect(nodes.ownerFailureAsWithheldV1(Error("boom"),scope)).toBeDefined();
 expect(nodes.ownerFailureAsWithheldV1(Error("boom"),{...scope,aborted:true})).toBeUndefined();
 expect(nodes.ownerFailureAsWithheldV1(Object.assign(Error("disk"),{code:"EMFILE"}),scope)).toBeUndefined();
 expect(nodes.ownerFailureAsWithheldV1(Object.assign(Error("restart"),{code:"BUILD_AUTHORITY_CHANGED"}),scope)).toBeUndefined();
 expect(nodes.ownerFailureAsWithheldV1(Object.assign(Error("gone"),{name:"AbortError"}),scope)).toBeUndefined();
 expect(nodes.ownerFailureAsWithheldV1(Error("boom"),{...scope,root:"missing-owner"})).toBeUndefined();
}),60_000);

test("the late closure-reference stage withholds the one owner whose references refuse (the aggregate route table), and leaves the failures of other layers alone",async()=>repository.invoke(async()=>{
 const state=await setup(),scope={workspaceRoot:state.root,root:aggregate.root,name:"repo",aborted:false};
 state.editTable();
 const raised=(()=>{try{requireCanonicalTestAggregateV1(JSON.parse(readFileSync(join(state.root,aggregate.manifest),"utf8")),aggregate.root,path=>readFileSync(join(state.root,path),"utf8"));}catch(error){return error;}throw Error("the edited table must refuse");})();
 const withheld=nodes.ownerRefsFailureV1(raised,scope);
 expect(withheld.code).toBe("BUILD_SOURCE_UNPARSEABLE");
 expect(withheld.source).toBe(join(state.root,aggregate.manifest));
 expect(withheld.message).toContain("kann nicht abgeleitet werden");
 for(const code of ["graph.farm-job-failed","BUILD_OWNER_DEFERRED","BUILD_STORE_CHANGED"])expect(nodes.ownerRefsFailureV1(Object.assign(Error("layer"),{code}),scope),code).toBeUndefined();
 expect(nodes.ownerRefsFailureV1(Object.assign(Error("layer"),{storeChanged:true}),scope)).toBeUndefined();
 expect(nodes.ownerRefsFailureV1(Object.assign(Error("restart"),{code:"BUILD_AUTHORITY_CHANGED"}),scope)).toBeUndefined();
}),60_000);
