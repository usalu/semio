import {test,expect} from "bun:test";
import {build} from "esbuild";
import fixture from "./🧫️fixtures/🔣️.json";
import * as parser from "../../../📥️inference/🟨️.mjs";

test("complete module facts retain command test and typed imports while production scheduling is scoped",async()=>{
 for(const row of fixture.cases){
  expect([...parser.moduleSourceImports(row.file,row.source)].sort()).toEqual([...row.all].sort());
  expect([...parser.productionModuleSourceImports(row.file,row.source)].sort()).toEqual([...row.production].sort());
  if(row.file.endsWith(".d.ts")||row.file.includes("📜️script")||row.file.includes("🧪️tests"))continue;
  const runtime=new Set<string>();
  await build({stdin:{contents:row.source,sourcefile:row.file,loader:"ts"},write:false,bundle:true,format:"esm",define:{"import.meta.vitest":"false"},logLevel:"silent",plugins:[{name:"production-scheduling-oracle",setup(builder){builder.onResolve({filter:/^\./},args=>{runtime.add(args.path);return{path:args.path,external:true}})}}]});
  expect([...runtime].sort()).toEqual(row.production.filter(path=>!path.includes("types.ts")).sort());
 }
 console.log("[DEBUG] Production DAG retains typed runtime edges and complete command/test source facts; esbuild runtime oracle agrees");
});

import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json";
import {mkdtempSync,mkdirSync,writeFileSync,readFileSync,rmSync} from "node:fs";
import {join,dirname} from "node:path";
import {createHash} from "node:crypto";

test("original SDK full roster supplies complete immutable metadata through full and incremental scheduling",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Original ticket artifact directory required");
 const root=mkdtempSync(join(output,"dag-"));
 try{
  for(const row of fixture.cases){const path=join(root,row.file);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,row.source);}
  for(const file of ["process/types.ts","process/core.ts","validator/core.ts","test/helper.ts"]){const path=join(root,file);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,"export const value=1;");}
  const projects=Object.fromEntries(["app","process","validator","test"].map(name=>[name,{root:name,targets:{build:{}}}]));
  const files=Object.fromEntries(Object.keys(projects).map(name=>[name,fixture.cases.filter(row=>row.file.startsWith(name+"/")).map(row=>({file:row.file,hash:createHash("sha256").update(row.source).digest("hex")}))]));
  writeFileSync(join(root,"📜️script.ts"),"import './process/core.ts';");
  projects.workspace={root:".",targets:{build:{}}};files.workspace=[{file:"📜️script.ts",hash:"original-sdk-root-hash"}];
  const {cacheInternals}=await import("../../../../🟨️.mjs");
  const context={workspaceRoot:root,projects,fileMap:{projectFileMap:files},filesToProcess:{projectFileMap:files}};
  const edges=await cacheInternals.createDependenciesImplementation({},context);
  expect(edges.map(edge=>[edge.source,edge.target,edge.sourceFile]).sort()).toEqual([["app","process","app/core.ts"],["app","process","app/types.d.ts"],["app","validator","app/core.ts"]].sort());
  const graph={nodes:Object.fromEntries(Object.entries(projects).map(([name,data])=>[name,{type:"lib",data:{...data,metadata:{semio:{authored:true}}}}]))};
  const metadata=cacheInternals.createMetadataImplementation(graph,{},context),validate=new Ajv().compile(schema);
  for(const [name,entry]of Object.entries(metadata)){expect(validate(entry.metadata.semio.moduleGraph)).toBe(true);expect(entry.metadata.semio.authored).toBe(true);expect(entry.metadata.semio.moduleGraph.files.length).toBe(files[name].length);for(const row of entry.metadata.semio.moduleGraph.files)expect(row.trackedHash).toBe(files[name].find(file=>file.file===row.file).hash);}
  expect(metadata.workspace.metadata.semio.moduleGraph.files[0].imports).toEqual(["./process/core.ts"]);
  expect(metadata.workspace.metadata.semio.moduleGraph.files[0].productionImports).toEqual([]);
  expect(metadata.validator.metadata.semio.moduleGraph.files[0].imports).toEqual(["../process/core.ts"]);
  expect(metadata.validator.metadata.semio.moduleGraph.files[0].productionImports).toEqual([]);
  metadata.validator.metadata.semio.moduleGraph.files[0].imports.push("./forged.ts");
  expect(cacheInternals.createMetadataImplementation(graph,{},context).validator.metadata.semio.moduleGraph.files[0].imports).toEqual(["../process/core.ts"]);
  const incremental=await cacheInternals.createDependenciesImplementation({}, {...context,filesToProcess:{projectFileMap:{app:[files.app[0]]}}});
  expect(incremental.length).toBe(2);
  const full=cacheInternals.createMetadataImplementation(graph,{},context);
  expect(full.app.metadata.semio.moduleGraph.files.length).toBe(files.app.length);
  const snapshot=full.app.metadata.semio.moduleGraph.files.find(row=>row.file==="app/core.ts");
  writeFileSync(join(root,"app/core.ts"),"export const changed=1;");
  expect(cacheInternals.createMetadataImplementation(graph,{},context).app.metadata.semio.moduleGraph.files.find(row=>row.file==="app/core.ts")).toEqual(snapshot);
  expect(snapshot.hash).not.toBe(createHash("sha256").update(readFileSync(join(root,"app/core.ts"))).digest("hex"));
  console.log("[DEBUG] Original full/incremental SDK facts preserve command/test metadata and captured byte identities; production DAG has no command cycles");
 }finally{rmSync(root,{recursive:true,force:true});}
});
