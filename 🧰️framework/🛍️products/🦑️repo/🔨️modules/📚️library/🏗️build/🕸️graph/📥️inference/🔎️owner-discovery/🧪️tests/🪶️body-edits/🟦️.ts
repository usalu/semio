/** 🪶️ Contract: an owner admission remembers which modules its commands import, never the bytes of those modules; the body of an imported module (the build tool's own code is one) is edited all day, so an edit that leaves the imports as they were keeps every remembered row of the owner valid and the discovery answers the same, while an edit that adds an import makes the rows stale and changes the answer.
 * @see ../../🟨️.mjs `discoverBuildOwnerAdmissionFactsV1` */
import{expect,test}from"bun:test";
import{mkdir,mkdtemp,rm,writeFile}from"node:fs/promises";
import{dirname,join,resolve}from"node:path";
import{readBuildOwnerRegistryV1}from"../../../../🚪️admission/🟦️.ts";
import{captureObservedSourceInputsV1,observationsHold}from"../../../../../👁️observation/🟨️.mjs";

const spec={version:1,owners:[{name:"app",root:"apps/app",manifest:"apps/app/📋️project.json",packages:[] as string[]}],catalogProviders:[]},selection={kind:"owners",projects:["app"],exclude:[],targets:["build"]};
const command=(script:string)=>({executor:"repo:script",cache:false,inputs:[],dependsOn:[],options:{command:"repo ./"+script+" build",cwd:"apps/app"},metadata:{semio:{execution:{version:1,kind:"source"}}}});
const tree:Record<string,string>={"📋️build-owners.json":JSON.stringify(spec),"apps/app/📋️project.json":JSON.stringify({name:"app",targets:{build:command("📜️script.ts"),compiled:{...command("📜️script.ts"),inputs:["{projectRoot}/compiled/**/*.ts"],metadata:{semio:{execution:{version:1,kind:"source"},sourceInputObservation:"compile"}}}}}),"apps/app/📜️script.ts":"import {helper} from \"./helper.ts\";\nexport const run = () => helper();\n","apps/app/helper.ts":"import {deep} from \"./deep.ts\";\nexport const helper = () => deep() + 1;\n","apps/app/deep.ts":"export const deep = () => 1;\n","apps/app/extra.ts":"export const extra = () => 2;\n","apps/app/compiled/entry.ts":"import {leaf} from \"./leaf.ts\";\nexport const entry = () => leaf();\n","apps/app/compiled/leaf.ts":"export const leaf = () => 3;\n","apps/app/compiled/data.json":"{\"a\":1}\n"};

test("a body edit of a command module keeps the remembered rows of the owner valid, an edit that adds an import does not",async()=>{
 const producer=await import("../../../../📥️inference/🟨️.mjs"),root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??process.env.TMPDIR??"/tmp"),"body-edits-"));
 try{
  await producer.libraryBootstrap;
  for(const[path,bytes]of Object.entries(tree)){await mkdir(dirname(join(root,path)),{recursive:true});await writeFile(join(root,path),bytes);}
  const discover=()=>{const registry=readBuildOwnerRegistryV1(structuredClone(spec));return captureObservedSourceInputsV1(()=>producer.discoverBuildOwnerAdmissionFactsV1(registry.owners[0]!,["build"],registry,{workspaceRoot:root,selection},[]),undefined,false);};
  const base=await discover(),facts=base.value as {files:string[]};
  expect(facts.files).toEqual(expect.arrayContaining(["apps/app/helper.ts","apps/app/deep.ts"]));
  expect(await observationsHold(base.observations)).toBe(true);
  const bodies=["export const deep = () => 41; // edited body\n","export const deep = () => {\n  const value = 7;\n  return value;\n};\n"];
  for(const body of bodies){
   await writeFile(join(root,"apps/app/deep.ts"),body);
   expect(await observationsHold(base.observations),body).toBe(true);
   const again=await discover();expect(again.value).toEqual(base.value);
  }
  await writeFile(join(root,"apps/app/deep.ts"),"import {extra} from \"./extra.ts\";\nexport const deep = () => extra();\n");
  expect(await observationsHold(base.observations)).toBe(false);
  const grown=(await discover()).value as {files:string[]};expect(grown.files).toContain("apps/app/extra.ts");expect(grown.files).not.toEqual(facts.files);
 }finally{await rm(root,{recursive:true,force:true});}
},180_000);

test("a body edit of a compile-role source keeps the remembered rows of the owner valid, an edit that adds an import does not",async()=>{
 const producer=await import("../../../../📥️inference/🟨️.mjs"),root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??process.env.TMPDIR??"/tmp"),"body-edits-compile-"));
 try{
  await producer.libraryBootstrap;
  for(const[path,bytes]of Object.entries(tree)){await mkdir(dirname(join(root,path)),{recursive:true});await writeFile(join(root,path),bytes);}
  const compiled={...selection,targets:["compiled"]},discover=()=>{const registry=readBuildOwnerRegistryV1(structuredClone(spec));return captureObservedSourceInputsV1(()=>producer.discoverBuildOwnerAdmissionFactsV1(registry.owners[0]!,["compiled"],registry,{workspaceRoot:root,selection:compiled},[]),undefined,false);};
  const base=await discover(),facts=base.value as {files:string[]};
  expect(facts.files).toEqual(expect.arrayContaining(["apps/app/compiled/entry.ts","apps/app/compiled/leaf.ts"]));
  expect(await observationsHold(base.observations)).toBe(true);
  await writeFile(join(root,"apps/app/compiled/leaf.ts"),"export const leaf = () => 99; // edited body\n");
  expect(await observationsHold(base.observations)).toBe(true);
  expect((await discover()).value).toEqual(base.value);
  await writeFile(join(root,"apps/app/compiled/leaf.ts"),"import {extra} from \"../extra.ts\";\nexport const leaf = () => extra();\n");
  expect(await observationsHold(base.observations)).toBe(false);
  expect(((await discover()).value as {files:string[]}).files).toContain("apps/app/extra.ts");
 }finally{await rm(root,{recursive:true,force:true});}
},180_000);
