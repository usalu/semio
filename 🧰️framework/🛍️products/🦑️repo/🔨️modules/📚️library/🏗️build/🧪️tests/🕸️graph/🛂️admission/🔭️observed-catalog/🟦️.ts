/** 🔭️ Contract: a plugin that consumes the catalog observes every catalog crate manifest as a foreign file fact (member, content observed, invalidating) and depends only on the catalog producer; the selected closure holds the plugin and the producer, never the crates; input globs that reach other owners' files are the same kind of fact. */
import{expect,test}from"bun:test";
import{mkdir,mkdtemp,rm,writeFile}from"node:fs/promises";
import{dirname,join,resolve}from"node:path";
import{closeBuildOwnerAdmissionV1,readBuildOwnerRegistryV1,readBuildGraphSelectionV1}from"../../../../🕸️graph/🚪️admission/🟦️.ts";
import{createRequire}from"node:module";
import{createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const require=createRequire(import.meta.url),crateCount=30;
const crate=(at:number)=>({name:"crate"+at,root:"catalog/plugins/c"+at,manifest:"catalog/plugins/c"+at+"/Cargo.toml",packages:[] as string[]});
const registry=()=>readBuildOwnerRegistryV1({version:1,owners:[{name:"app",root:"apps/app",manifest:"apps/app/📋️project.json",packages:[]},{name:"catalog-owner",root:"catalog",manifest:"catalog/📋️project.json",packages:[]},{name:"wide",root:"wide",manifest:"wide/📋️project.json",packages:[]},...Array.from({length:crateCount},(_,at)=>crate(at))],catalogProviders:[{ownerRoot:"catalog",sourcePath:"catalog/port.ts",consumerRoots:["apps"]}]});
const contribution={version:1,publication:{version:1,ownerRoot:"catalog/cat",outputRoot:"catalog/cat/generated",contributions:[{ownerRoot:"catalog",packageRoots:["catalog/plugins"]}]},projectRoot:"catalog",project:"catalog-owner",generateTarget:"generate",inputs:["catalog/request.json"]};
const files=(inputs:string[])=>({"apps/app/📋️project.json":JSON.stringify({name:"app",targets:{build:{executor:"repo:script",cache:false,inputs,dependsOn:[],options:{command:"repo ./📜️script.ts build",cwd:"apps/app"},metadata:{semio:{execution:{version:1,kind:"source"}}}}}}),"apps/app/📜️script.ts":"export const current = 1;\n","catalog/📋️project.json":JSON.stringify({name:"catalog-owner",targets:{generate:{executor:"repo:noop"}}}),"catalog/request.json":"{}","catalog/port.ts":"export function catalogBuildContributionV1(){return "+JSON.stringify(contribution)+";}","wide/📋️project.json":JSON.stringify({name:"wide",targets:{}}),"wide/a.txt":"a","wide/b.txt":"b",...Object.fromEntries(Array.from({length:crateCount},(_,at)=>["catalog/plugins/c"+at+"/Cargo.toml","[package]\nname='crate"+at+"'\nversion='0.0.0'\n"]))});

test("30 catalog crates and a foreign input glob are file facts of the plugin: members, never owners",async()=>{
 const owner=await import("../../../../🕸️graph/📥️inference/🟨️.mjs"),root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??process.env.TMPDIR??"/tmp"),"observed-catalog-")),selection=readBuildGraphSelectionV1({kind:"owners",projects:["app"],exclude:[],targets:["build"]}),reg=registry();
 try{
  await owner.libraryBootstrap;
  for(const[path,bytes]of Object.entries(files(["{workspaceRoot}/wide/**/*"]))){await mkdir(dirname(join(root,path)),{recursive:true});await writeFile(join(root,path),bytes);}
  const observation=await import("../../../../👁️observation/🟨️.mjs"),run=()=>observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(()=>closeBuildOwnerAdmissionV1(selection,reg,(value,targets,inputs)=>owner.discoverBuildOwnerAdmissionFactsV1(value,targets,reg,{workspaceRoot:root,selection},inputs))));
  const first=await run(),admission=first.value as {owners:string[];files:string[];observedFiles?:string[]};
  expect(admission.owners).toEqual(["app","catalog-owner"]);
  const manifests=Array.from({length:crateCount},(_,at)=>"catalog/plugins/c"+at+"/Cargo.toml");
  for(const path of[...manifests,"wide/a.txt","wide/b.txt"]){expect(admission.files).toContain(path);expect(admission.observedFiles).toContain(path);}
  expect(admission.observedFiles).toContain("wide/📋️project.json");expect(admission.observedFiles).not.toContain("apps/app/📋️project.json");expect(admission.owners).not.toContain("wide");
  const rows=(captured:any,path:string)=>JSON.stringify(captured.observations.filter((fact:any)=>String(fact.path).endsWith(path)));
  const before=rows(first,"catalog/plugins/c7/Cargo.toml");expect(before).not.toBe("[]");
  await writeFile(join(root,"catalog/plugins/c7/Cargo.toml"),"[package]\nname='crate7'\nversion='0.0.1'\n");
  const second=await run();expect(rows(second,"catalog/plugins/c7/Cargo.toml")).not.toBe(before);expect((second.value as {owners:string[]}).owners).toEqual(["app","catalog-owner"]);
 }finally{await rm(root,{recursive:true,force:true});}
},120_000);

test("120 seeded splits: the catalog and the component roots derived from admitted plus observed manifests equal those derived from every manifest",async()=>{
 const owner=await import("../../../../🕸️graph/📥️inference/🟨️.mjs"),internals=owner.cacheInternals as any,root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??process.env.TMPDIR??"/tmp"),"observed-split-"));
 try{
  await owner.libraryBootstrap;
  const reg=registry(),contributions=[{...contribution,inputs:[]}] as never[];
  const manifests=Array.from({length:crateCount},(_,at)=>"catalog/plugins/c"+at+"/Cargo.toml");
  for(const[at,path]of manifests.entries()){await mkdir(dirname(join(root,path)),{recursive:true});await writeFile(join(root,path),at%3===0?"[package]\nname='crate"+at+"'\nversion='0.0.0'\n[package.metadata.component]\npackage='component:c"+at+"'\n[package.metadata.semio]\ncomponent-kind='plugin'\nplayground-variant='v"+at+"'\n":"[package]\nname='crate"+at+"'\nversion='0.0.0'\n");}
  let seed=26111701;const random=()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};
  const reference=JSON.stringify(internals.collectPlaygroundCatalog(manifests,root,contributions)),roots=JSON.stringify(internals.componentCrateRoots(manifests,root));let nonEmpty=0;
  for(let world=0;world<120;world++){
   const admitted=manifests.filter(()=>random()<0.4),observed=manifests.filter(path=>!admitted.includes(path)),combined=[...admitted,...observed.reverse()];
   expect(JSON.stringify(internals.collectPlaygroundCatalog(combined,root,contributions)),"world "+world).toBe(reference);expect(JSON.stringify(internals.componentCrateRoots(combined,root))).toBe(roots);
   if(internals.componentCrateRoots(admitted,root).length<internals.componentCrateRoots(manifests,root).length)nonEmpty++;
  }
  expect(nonEmpty).toBeGreaterThan(100);expect(JSON.parse(roots).length).toBe(10);void reg;
 }finally{await rm(root,{recursive:true,force:true});}
},120_000);
