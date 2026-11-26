/** 📚️ Contract: the catalog contributions and the native manifest candidates of the providers are computed once per owner registry and shared by every owner that consumes the catalog and by every later revision of one owner (its targets or inputs grew); the shared answer equals the answer of a computation of its own (a registry object that never saw the memo), an edit of a manifest content is visible in the rows of every owner without a new computation, and a change of what the candidates looked at (a manifest that became a link) is computed again.
 * @see ../../🟨️.mjs `sharedCatalogV1` */
import{expect,test}from"bun:test";
import{mkdir,mkdtemp,rm,symlink,writeFile}from"node:fs/promises";
import{dirname,join,resolve}from"node:path";
import{readBuildOwnerRegistryV1}from"../../../../🚪️admission/🟦️.ts";
import{captureObservedSourceInputsV1}from"../../../../../👁️observation/🟨️.mjs";

const crateCount=12,crate=(at:number)=>({name:"crate"+at,root:"catalog/plugins/c"+at,manifest:"catalog/plugins/c"+at+"/Cargo.toml",packages:[] as string[]});
const app=(name:string)=>({name,root:"apps/"+name,manifest:"apps/"+name+"/📋️project.json",packages:[] as string[]});
const spec={version:1,owners:[app("one"),app("two"),{name:"catalog-owner",root:"catalog",manifest:"catalog/📋️project.json",packages:[]},...Array.from({length:crateCount},(_,at)=>crate(at))],catalogProviders:[{ownerRoot:"catalog",sourcePath:"catalog/port.ts",consumerRoots:["apps"]}]},registry=()=>readBuildOwnerRegistryV1(structuredClone(spec));
const contribution={version:1,publication:{version:1,ownerRoot:"catalog/cat",outputRoot:"catalog/cat/generated",contributions:[{ownerRoot:"catalog",packageRoots:["catalog/plugins"]}]},projectRoot:"catalog",project:"catalog-owner",generateTarget:"generate",inputs:["catalog/request.json"]};
const project=(name:string)=>JSON.stringify({name,targets:{build:{executor:"repo:script",cache:false,inputs:[],dependsOn:[],options:{command:"repo ./📜️script.ts build",cwd:"apps/"+name},metadata:{semio:{execution:{version:1,kind:"source"}}}},check:{executor:"repo:script",cache:false,inputs:[],dependsOn:[],options:{command:"repo ./📜️script.ts check",cwd:"apps/"+name},metadata:{semio:{execution:{version:1,kind:"source"}}}}}});
const tree:Record<string,string>={"📋️build-owners.json":JSON.stringify(spec),"apps/one/📋️project.json":project("one"),"apps/one/📜️script.ts":"export const current = 1;\n","apps/two/📋️project.json":project("two"),"apps/two/📜️script.ts":"export const current = 2;\n","catalog/📋️project.json":JSON.stringify({name:"catalog-owner",targets:{generate:{executor:"repo:noop"}}}),"catalog/request.json":"{}","catalog/port.ts":"export function catalogBuildContributionV1(){return "+JSON.stringify(contribution)+";}",...Object.fromEntries(Array.from({length:crateCount},(_,at)=>["catalog/plugins/c"+at+"/Cargo.toml","[package]\nname='crate"+at+"'\nversion='0.0.0'\n"]))};
const selection={kind:"owners",projects:["one","two"],exclude:[],targets:["build"]};

test("the catalog is computed once per registry, shared by owners and revisions, equal to a computation of its own, and recomputed after a change it looked at",async()=>{
 const producer=await import("../../../../📥️inference/🟨️.mjs"),owners=await import("../../🟨️.mjs"),root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??process.env.TMPDIR??"/tmp"),"shared-catalog-"));
 try{
  await producer.libraryBootstrap;
  for(const[path,bytes]of Object.entries(tree)){await mkdir(dirname(join(root,path)),{recursive:true});await writeFile(join(root,path),bytes);}
  const shared=registry(),discover=(reg:ReturnType<typeof registry>,name:string,targets:string[])=>captureObservedSourceInputsV1(()=>producer.discoverBuildOwnerAdmissionFactsV1(reg.owners.find(owner=>owner.name===name)!,targets,reg,{workspaceRoot:root,selection},[]),undefined,false);
  const stats=()=>owners.catalogSharedStatsV1(),before=stats();
  const steps:[string,string[]][]=[["one",["build"]],["one",["build","check"]],["two",["build"]],["two",["build","check"]]];
  const answers=[];for(const[name,targets]of steps){answers.push(await discover(shared,name,targets));}
  const after=stats();
  expect(after.computed-before.computed).toBe(1);expect(after.reused-before.reused).toBe(steps.length-1);
  for(const[index,[name,targets]]of steps.entries()){const own=await discover(registry(),name,targets);expect(answers[index]!.value).toEqual(own.value);{const facts=answers[index]!.value as {files:string[];observedFiles?:string[]};expect([...facts.files,...facts.observedFiles??[]]).toContain("catalog/plugins/c3/Cargo.toml");}}
  const rows=(captured:{observations:Iterable<{path:string}>},path:string)=>JSON.stringify([...captured.observations].filter(row=>String(row.path).endsWith(path)));
  const first=rows(answers[0]!,"catalog/plugins/c3/Cargo.toml");expect(first).not.toBe("[]");
  await writeFile(join(root,"catalog/plugins/c3/Cargo.toml"),"[package]\nname='crate3'\nversion='0.0.1'\n");
  const mark=stats().computed,again=await discover(shared,"one",["build"]);
  expect(stats().computed-mark).toBe(0);expect(rows(again,"catalog/plugins/c3/Cargo.toml")).not.toBe(first);
  await rm(join(root,"catalog/plugins/c3/Cargo.toml"));await symlink(join(root,"catalog/plugins/c4/Cargo.toml"),join(root,"catalog/plugins/c3/Cargo.toml"));
  await expect(discover(shared,"one",["build"])).rejects.toThrow("Catalog native manifest must be a real file");
  expect(stats().computed-mark).toBe(1);
 }finally{await rm(root,{recursive:true,force:true});}
},180_000);
