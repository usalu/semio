/** 🧷️ Contract: a plugin that consumes the catalog depends on the catalog producer through the provider source and the crate manifests only; the producer's `production` tree is never asked for by the consumer's `^production`, so an unrelated file below the producer does not invalidate the consumer while a manifest does. A real import edge to the producer still asks for it. */
import{expect,test}from"bun:test";
import{realpathSync}from"node:fs";
import{mkdir,mkdtemp,rm,writeFile}from"node:fs/promises";
import{dirname,join,resolve}from"node:path";
import{closeBuildOwnerAdmissionV1,readBuildOwnerRegistryV1,readBuildGraphSelectionV1}from"../../../../🕸️graph/🚪️admission/🟦️.ts";

const crateCount=12,crate=(at:number)=>({name:"crate"+at,root:"catalog/plugins/c"+at,manifest:"catalog/plugins/c"+at+"/Cargo.toml",packages:[] as string[]});
const registry=()=>readBuildOwnerRegistryV1({version:1,owners:[{name:"app",root:"apps/app",manifest:"apps/app/📋️project.json",packages:["@t/app"]},{name:"catalog-owner",root:"catalog",manifest:"catalog/📋️project.json",packages:["@t/catalog"]},...Array.from({length:crateCount},(_,at)=>crate(at))],catalogProviders:[{ownerRoot:"catalog",sourcePath:"catalog/port.ts",consumerRoots:["apps"]}]});
const contribution={version:1,publication:{version:1,ownerRoot:"catalog/cat",outputRoot:"catalog/cat/generated",contributions:[{ownerRoot:"catalog",packageRoots:["catalog/plugins"]}]},projectRoot:"catalog",project:"catalog-owner",generateTarget:"generate",inputs:["catalog/request.json"]};
const files=(script:string)=>({"apps/app/📋️project.json":JSON.stringify({name:"app",targets:{build:{executor:"repo:script",cache:false,inputs:["^production"],dependsOn:[],options:{command:"repo ./📜️script.ts build",cwd:"apps/app"},metadata:{semio:{execution:{version:1,kind:"source"}}}}}}),"apps/app/📜️script.ts":script,"catalog/📋️project.json":JSON.stringify({name:"catalog-owner",namedInputs:{production:["{workspaceRoot}/catalog/**/*"]},targets:{generate:{executor:"repo:noop"}}}),"catalog/request.json":"{}","catalog/port.ts":"export function catalogBuildContributionV1(){return "+JSON.stringify(contribution)+";}","catalog/unrelated/a.txt":"a","catalog/unrelated/b.txt":"b","catalog/lib.ts":"export const lib = 1;\n",...Object.fromEntries(Array.from({length:crateCount},(_,at)=>["catalog/plugins/c"+at+"/Cargo.toml","[package]\nname='crate"+at+"'\nversion='0.0.0'\n"]))});

const world=async(script:string)=>{
 const owner=await import("../../../../🕸️graph/📥️inference/🟨️.mjs"),root=await mkdtemp(join(realpathSync(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??process.env.TMPDIR??"/tmp")),"catalog-producer-")),reg=registry(),selection=readBuildGraphSelectionV1({kind:"owners",projects:["app"],exclude:[],targets:["build"]});
 await owner.libraryBootstrap;
 for(const[path,bytes]of Object.entries(files(script))){await mkdir(dirname(join(root,path)),{recursive:true});await writeFile(join(root,path),bytes);}
 const observation=await import("../../../../👁️observation/🟨️.mjs"),asked:Record<string,string[]>={},run=()=>observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(()=>closeBuildOwnerAdmissionV1(selection,reg,async(value,targets,inputs)=>{const facts=await owner.discoverBuildOwnerAdmissionFactsV1(value,targets,reg,{workspaceRoot:root,selection},inputs);asked[value.name]=[...inputs];return facts;})));
 return{root,run,asked};
};
const rows=(captured:any,path:string)=>JSON.stringify(captured.observations.filter((fact:any)=>String(fact.path).endsWith(path)));

test("the consumer never asks the catalog producer for its production tree: unrelated files stay unobserved and unrelated edits do not invalidate, a manifest edit does",async()=>{
 const{root,run,asked}=await world("export const current = 1;\n");
 try{
  const first=await run(),admission=first.value as {owners:string[];files:string[]};
  expect(admission.owners).toEqual(["app","catalog-owner"]);
  expect(asked["catalog-owner"]?.includes("production")).toBe(false);
  const members=[...admission.files,...(first.value as {observedFiles?:string[]}).observedFiles??[]];expect(members).not.toContain("catalog/unrelated/a.txt");expect(members).not.toContain("catalog/lib.ts");const snapshot=JSON.stringify(first.observations);
  const manifest=rows(first,"catalog/plugins/c3/Cargo.toml"),port=rows(first,"catalog/port.ts");expect(manifest).not.toBe("[]");expect(port).not.toBe("[]");
  await writeFile(join(root,"catalog/unrelated/a.txt"),"changed");await writeFile(join(root,"catalog/lib.ts"),"export const lib = 2;\n");
  const second=await run();expect(JSON.stringify(second.observations)).toBe(snapshot);expect(rows(second,"catalog/plugins/c3/Cargo.toml")).toBe(manifest);expect(rows(second,"catalog/port.ts")).toBe(port);expect(JSON.stringify((second.value as {files:string[]}).files)).toBe(JSON.stringify(admission.files));
  await writeFile(join(root,"catalog/plugins/c3/Cargo.toml"),"[package]\nname='crate3'\nversion='0.0.1'\n");
  const third=await run();expect(rows(third,"catalog/plugins/c3/Cargo.toml")).not.toBe(manifest);
 }finally{await rm(root,{recursive:true,force:true});}
},120_000);

test("a real import edge to the catalog producer still asks for its production tree",async()=>{
 const{root,run,asked}=await world("import {lib} from \"../../catalog/lib.ts\";\nexport const current = lib;\n");
 try{
  const first=await run();
  expect(asked["catalog-owner"]?.includes("production")).toBe(true);
  expect([...(first.value as {files:string[]}).files,...(first.value as {observedFiles?:string[]}).observedFiles??[]]).toContain("catalog/unrelated/a.txt");
 }finally{await rm(root,{recursive:true,force:true});}
},120_000);
