/** 🔌️ The Nx projection and private engine references agree with the independent Nx implementation. */
import assert from "node:assert/strict";
import {testValueV1} from "../../../../⚡️caching/🧪️tests/📋️values/🟦️.ts";
import {readBuildInferenceDatasetV1} from "../../../📥️inputs/💾️storage/🟨️.mjs";
import nxAdmissionFixture from "../../../../⚡️caching/🧪️tests/🧫️fixtures/nx-contract/🔣️.json";
import nxAdmissionPolicy from "../../../../⚡️caching/🔣️policy.json";
import {boundedAdmissionWalkV1,collectionManifestsFromInventory} from "../../../../⚡️caching/🚀️bootstrap/📥️admission/🟦️.ts";
import admissionSchema from "../../../../⚡️caching/🚀️bootstrap/📥️admission/🧬️schema/🔣️.json";
import {mkdir,mkdtemp,writeFile,rm} from "node:fs/promises";
import * as rawPromises from "node:fs/promises";
import {dirname,join,resolve} from "node:path";
import {projectBuildGraphForNxV1,projectBuildTargetForNxV1,nxBuildEngineInputsV1,nxBuildEngineReferencesV1,nxBuildEngineSourceFileV1,nxBuildEngineWorkspacePackageFileV1,nxBuildEngineClosureV1,nxBuildEngineFilesDigestV1,nxBuildEngineDigestV1,nxOutputsCacheableV1,shareNxProjectInputsV1} from "../../../../🟨️.mjs";
import {expect,test} from "bun:test";
import {buildInputPathMatchesV1} from "../../../📥️inputs/🟨️.mjs";
import {fileURLToPath} from "node:url";
import {createRequire} from "node:module";
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import fixture from "../../🧫️fixtures/🕸️graph/🔣️.json";
import pluginSchema from "../../../🧬️schema/📥️plugin.json";
import {nxEngineUnkeyedPrefixesV1,splitNxEngineFilesV1} from "../../../../⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🟦️.ts";
import {classifyNxOutputV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🟦️.ts";
import budgetPolicy from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json";
import engineSchema from "../../../🧬️schema/📥️engine.json";
import schema from "../../../🧬️schema/🔣️.json";
import {readBuildGraphV1,neutralBuildTargetV1} from "../../../🕸️graph/🟦️.ts";
import {datasetSchema,nxOracleRequireV1,independentGlobV1,graph} from "../🧰️support/🟦️.ts";

const require=createRequire(import.meta.url),Ajv=require("ajv").default;
test("portable source glob matching agrees with the installed Nx implementation",()=>{
  const {paths,patterns}=fixture.globCases;
  for(const pattern of patterns)expect(paths.map(path=>buildInputPathMatchesV1(path,pattern))).toEqual(require("nx/src/native").matchGlobPaths([pattern],paths));
});
test("Nx projects exactly the common graph into its independent task graph implementation",()=>{
  const portable=graph(),projection=projectBuildGraphForNxV1(portable),nodes=Object.fromEntries(Object.entries(projection.projects).map(([name,data])=>[name,{name,type:"lib",data}])),dependencies=Object.fromEntries(Object.keys(nodes).map(name=>[name,portable.dependencies.filter(edge=>edge.source===name)]));
  const {createTaskGraph}=require("nx/src/tasks-runner/create-task-graph"),tasks=createTaskGraph({nodes,dependencies}, {}, ["app"],["test"],undefined,{});
  expect(Object.keys(projection.projects).sort()).toEqual(Object.keys(portable.projects).sort());
  expect(Object.keys(tasks.tasks).sort()).toEqual(["app:test","lib:build"]);
  expect(tasks.dependencies["app:test"]).toEqual(["lib:build"]);
  expect(testValueV1(testValueV1(projection.projects.app).targets.build).executor).toBe("@semio-tech/repo-lib:command");
});
test("policy facets retain six physical owners and ordinary lint scheduling in both backends",async()=>{
  const workspace=await mkdtemp(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"policy-")),policyText="export const policy=true;\n";
  for(const row of fixture.policyOwnership){await mkdir(join(workspace,row.root),{recursive:true});await writeFile(join(workspace,row.root,"📜️script.ts"),policyText);}
  try{
  const owners=Object.fromEntries(fixture.policyOwnership.map((row,index)=>[row.name,{...row,targets:index%2?{}:{lint:{executor:"repo:script",options:{command:"bun --version"},dependsOn:["prepare"]},prepare:{executor:"repo:noop"}}}])),files=fixture.policyOwnership.map(row=>join(row.root,"📜️script.ts")),policy=require("../../../🕸️graph/📥️inference/🛡️policy/🟨️.mjs").default;
  const contributions=await policy.createNodesV2[1](files,{}, {workspaceRoot:workspace,projects:owners}),projects=structuredClone(owners);
  for(const[,result]of contributions)for(const[name,project]of Object.entries(result.projects)as[string,any][])projects[name]={...projects[name],...project,targets:{...projects[name]?.targets,...project.targets}};
  expect(Object.keys(projects).sort()).toEqual(fixture.policyOwnership.map(row=>row.name).sort());
  const edges=fixture.policyOwnership.map((row,index)=>({source:row.name,target:testValueV1(fixture.policyOwnership[(index+1)%fixture.policyOwnership.length]).name,type:"implicit"})),portable=readBuildGraphV1({version:1,projects:Object.fromEntries(Object.entries(projects).map(([name,project]:[string,any])=>[name,{...project,targets:Object.fromEntries(Object.entries(project.targets).map(([name,target])=>[name,neutralBuildTargetV1(target as any)]))}])),dependencies:edges}),projection=projectBuildGraphForNxV1(portable),errors:unknown[]=[],merged=require("nx/src/project-graph/utils/project-configuration-utils").mergeCreateNodesResults([[["owned-policy","📋️project.json",projection,0]]],[],{},process.cwd(),errors);
  expect(new Ajv({strict:true}).compile(schema)(portable)).toBe(true);
  expect(errors).toEqual([]);expect(Object.values(merged.projectRootMap).map((project:any)=>project.name).sort()).toEqual(Object.keys(portable.projects).sort());
  const {planBazelTasksV1}=await import("../../../🔧️tools/🏗️bazel/📜️script.ts"),nodes=Object.fromEntries(Object.entries(projection.projects).map(([name,data])=>[name,{name,type:"lib",data}])),dependencies=Object.fromEntries(Object.keys(nodes).map(name=>[name,[]]));
  const builder=new(require("nx/src/project-graph/project-graph-builder").ProjectGraphBuilder)({nodes,dependencies,externalNodes:{}});for(const edge of portable.dependencies)builder.addDependency(edge.source,edge.target,edge.type);const byEdge=(rows:readonly object[])=>rows.map(row=>JSON.stringify(row)).sort();expect(byEdge(Object.values(builder.getUpdatedProjectGraph().dependencies as Record<string,object[]>).flat())).toEqual(byEdge(edges));
  for(const[rowIndex,row]of fixture.policyOwnership.entries()){
    const target=testValueV1(portable.projects[row.name]).targets.lint,plan=planBazelTasksV1(portable,{tasks:[row.name+":lint"]}),oracle=require("nx/src/tasks-runner/create-task-graph").createTaskGraph({nodes,dependencies},{},[row.name],["lint"],undefined,{});
    expect(Object.keys(plan.tasks).sort()).toEqual(Object.keys(oracle.tasks).sort());expect(testValueV1(plan.tasks[row.name+":lint"]).dependencies.sort()).toEqual(oracle.dependencies[row.name+":lint"].sort());expect(testValueV1(target).dependsOn).toContain("lint-policy");expect(testValueV1(testValueV1(portable.projects[row.name]).targets["lint-policy"]).execution.command).toContain(files[rowIndex]);
    if(rowIndex%2===0){expect(testValueV1(target).execution.command).toBe("bun --version");expect(testValueV1(target).dependsOn).toContain("prepare");}
  }
  expect(()=>policy.createNodesV2[1]([files[0]],{}, {workspaceRoot:workspace,projects:{[testValueV1(fixture.policyOwnership[0]).name]:{...owners[testValueV1(fixture.policyOwnership[0]).name],targets:{"lint-policy":{executor:"repo:noop"}}}}})).toThrow("Policy target already declared");
  const standalone=policy.createNodesV2[1]([files[0]],{}, {workspaceRoot:workspace,projects:{}})[0][1].projects,facet=Object.values(standalone)[0]as any;expect(facet.name.startsWith("breach-")).toBe(true);expect(facet.targets.lint).toEqual({executor:"repo:noop",dependsOn:["lint-policy"]});
  }finally{await rm(workspace,{recursive:true,force:true});}
});
test("private Nx engine digest affects real native task hashes while invocation staging does not",()=>{
 const row=fixture.nxEngineIdentity,sdk=createRequire(resolve(process.cwd(),".nx/installation/package.json")),native=sdk("nx/src/native"),transform=sdk("nx/src/native/transform-objects").transformProjectGraphForRust,target=projectBuildTargetForNxV1({execution:{kind:"command",command:"repo ./📜️script.ts",cwd:"fixture"},cache:true,inputs:["{projectRoot}/📜️script.ts"]}),task={id:"fixture:build",target:{project:"fixture",target:"build"},overrides:{},outputs:[],projectRoot:"fixture",cache:true,parallelism:true},taskGraph={roots:[task.id],tasks:{[task.id]:task},dependencies:{[task.id]:[]},continuousDependencies:{[task.id]:[]}};
 const engineWith=(digest:string|undefined)=>digest===undefined?nxBuildEngineInputsV1():[{runtime:`node -p "'${digest}'"`},...nxBuildEngineInputsV1().slice(1)];
 const hash=(engine:string|undefined,wire:string,nx:string)=>{
  const graph={nodes:{fixture:{name:"fixture",type:"lib",data:{root:"fixture",targets:{build:target},namedInputs:{default:["{projectRoot}/**/*"],[fixture.nxEngineNamedInput.name]:engineWith(engine)}}}},externalNodes:{"npm:nx":{name:"npm:nx",type:"npm",data:{packageName:"nx",version:"23.2.0",hash:nx}},"npm:typescript":{name:"npm:typescript",type:"npm",data:{packageName:"typescript",version:"5.9.3",hash:"typescript-fixed"}}},dependencies:{fixture:[]}},graphRef=native.transferProjectGraph(transform(graph)),filemap=native.testOnlyTransferFileMap({fixture:[{file:row.script,hash:"script-fixed"}]},[{file:row.engine,hash:"engine-fixed"},{file:row.wire,hash:wire}]),planner=new native.HashPlanner({},graphRef),plans=planner.getPlansReference([task.id],taskGraph),hasher=new native.TaskHasher(process.cwd(),graphRef,filemap.projectFiles,filemap.allWorkspaceFiles,Buffer.from("{}"),{},undefined,{selectivelyHashTsConfig:false});
  return {value:hasher.hashPlans(plans,{[task.id]:{}},process.cwd(),true)[task.id].value,instructions:planner.getPlans([task.id],taskGraph)[task.id]};
 };
 const baseline=hash("engine-before","wire-before","nx-before"),changed=hash("engine-after","wire-before","nx-before"),restaged=hash("engine-before","wire-after","nx-before"),upgraded=hash("engine-before","wire-before","nx-after");
 expect(changed.value).not.toBe(baseline.value);expect(restaged.value).toBe(baseline.value);expect(upgraded.value).not.toBe(baseline.value);
 const real=hash(undefined,"wire-before","nx-before"),again=hash(undefined,"wire-before","nx-before");
 expect(real.value).toBe(again.value);expect(real.value).not.toBe(baseline.value);expect(JSON.stringify(real.instructions).length).toBeLessThan(4096);
 const implicit=projectBuildTargetForNxV1({execution:{kind:"command",command:"repo ./📜️script.ts"},cache:true});expect(implicit.inputs).toContain("default");expect(implicit.inputs).toContain("^default");
},60000);
test("the private Nx engine is one runtime digest of its closure equal to an independent Python digest of the same files",()=>{
 const root=process.cwd(),inputs=nxBuildEngineInputsV1(),closure=nxBuildEngineClosureV1(),python=(files:readonly string[],base:string,prefixes:readonly string[]=[])=>{const run=Bun.spawnSync(["python3","-I","-c","import hashlib,json,os,sys\nbase,files,prefixes=sys.argv[1],json.loads(sys.argv[2]),json.loads(sys.argv[3])\nd=hashlib.sha256()\nfor f in files:\n d.update(f.encode()+b'\\0'+hashlib.sha256(open(os.path.join(base,f),'rb').read()).hexdigest().encode()+b'\\n')\nif prefixes:\n d.update(b'\\0unkeyed\\0'+hashlib.sha256('\\n'.join(prefixes).encode()).hexdigest().encode()+b'\\n')\nprint(d.hexdigest())",base,JSON.stringify(files),JSON.stringify(prefixes)]);expect(run.exitCode,run.stderr.toString()).toBe(0);return run.stdout.toString().trim();};
 expect(inputs.some(value=>typeof value==="string")).toBe(false);expect(JSON.stringify(inputs).length).toBeLessThan(2048);
 const command=(inputs[0] as {runtime:string}).runtime,entry=closure.files.find(file=>file.endsWith("🧮️engine-digest/🟨️.mjs"));
 expect(entry).toBeDefined();expect(command).toBe(`bun "./${entry}"`);
 expect(closure.files).toEqual([...closure.files].sort());expect(new Set(closure.files).size).toBe(closure.files.length);
 const workspace=nxBuildEngineWorkspacePackageFileV1(root,"@semio-tech/process/routing")!;expect(closure.files).toContain(workspace.slice(root.length+1));
 const digest=nxBuildEngineDigestV1(),keyed=splitNxEngineFilesV1(closure.files).keyed;expect(keyed.length).toBeLessThan(closure.files.length);expect(digest).toBe(python(keyed,root,nxEngineUnkeyedPrefixesV1()));
 const run=Bun.spawnSync([process.execPath,join(root,entry!)],{cwd:root});expect(run.exitCode,run.stderr.toString()).toBe(0);expect(run.stdout.toString().trim()).toBe(digest);
});
test("the engine files digest follows bytes, names and order against the independent Python digest",async()=>{
 const base=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"engine-digest-")),python=(files:readonly string[])=>{const run=Bun.spawnSync(["python3","-I","-c","import hashlib,json,os,sys\nbase,files=sys.argv[1],json.loads(sys.argv[2])\nd=hashlib.sha256()\nfor f in files:\n d.update(f.encode()+b'\\0'+hashlib.sha256(open(os.path.join(base,f),'rb').read()).hexdigest().encode()+b'\\n')\nprint(d.hexdigest())",base,JSON.stringify(files)]);expect(run.exitCode,run.stderr.toString()).toBe(0);return run.stdout.toString().trim();};
 try{await mkdir(join(base,"🧪️x"),{recursive:true});await writeFile(join(base,"a.mjs"),"export {};\n");await writeFile(join(base,"🧪️x/b.json"),"{}\n");
  const files=["a.mjs","🧪️x/b.json"],first=nxBuildEngineFilesDigestV1(base,files);expect(first).toBe(python(files));expect(nxBuildEngineFilesDigestV1(base,files)).toBe(first);
  await writeFile(join(base,"a.mjs"),"export {};\n//\n");const edited=nxBuildEngineFilesDigestV1(base,files);expect(edited).not.toBe(first);expect(edited).toBe(python(files));
  expect(nxBuildEngineFilesDigestV1(base,[...files].reverse())).not.toBe(edited);expect(nxBuildEngineFilesDigestV1(base,["a.mjs"])).not.toBe(edited);
  await writeFile(join(base,"c.mjs"),"export {};\n//\n");expect(nxBuildEngineFilesDigestV1(base,["c.mjs","🧪️x/b.json"])).not.toBe(edited);
  expect(()=>nxBuildEngineFilesDigestV1(base,["missing.mjs"])).toThrow();
 }finally{await rm(base,{recursive:true,force:true});}
});
test("private engine references distinguish directory URLs from real module imports",()=>{
 const validate=new (require("ajv").default)({strict:false}).compile(engineSchema),parse=nxOracleRequireV1("@babel/parser").parse;
 for(const row of fixture.engineReferences){
  const actual=nxBuildEngineReferencesV1(row.source),oracle:Array<{kind:string;specifier:string}>=[],add=(kind:string,specifier:string)=>{if(!oracle.some(value=>value.kind===kind&&value.specifier===specifier))oracle.push({kind,specifier});},urls:Array<{kind:string;specifier:string}>=[];
  const visit=(node:any)=>{
   if(["ImportDeclaration","ExportAllDeclaration","ExportNamedDeclaration"].includes(node.type)&&node.source)add("module",node.source.value);
   if(node.type==="CallExpression"&&((node.callee?.type==="Identifier"&&node.callee.name==="require")||node.callee?.type==="Import")&&node.arguments?.[0]?.type==="StringLiteral")add("module",node.arguments[0].value);
   if(node.type==="NewExpression"&&node.callee?.name==="URL"&&node.arguments?.length===2&&node.arguments[0]?.type==="StringLiteral"&&node.arguments[0].value.startsWith(".")&&node.arguments[1]?.type==="MemberExpression"&&node.arguments[1].object?.type==="MetaProperty"&&node.arguments[1].property?.name==="url"){const specifier=node.arguments[0].value;urls.push({kind:new URL(specifier,"file:///fixture/engine.mjs").pathname.endsWith("/")?"directory-url":"file-url",specifier});}
   for(const value of Object.values(node))if(Array.isArray(value)){for(const child of value)if(child&&typeof child==="object"&&typeof child.type==="string")visit(child);}else if(value&&typeof value==="object"&&"type" in value)visit(value);
  };
  visit(parse(row.source,{sourceType:"module"}));for(const value of urls)add(value.kind,value.specifier);
  expect(validate(actual)).toBe(true);assert.deepEqual(actual,row.references);assert.deepEqual(actual,oracle);
 }
});
test("private engine references keep actual directory-index imports and missing-file refusals",async()=>{
 const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"engine-files-")),owner=join(root,"engine.mjs"),index=join(root,"dir/index.js");
 try{await mkdir(dirname(index),{recursive:true});await writeFile(owner,"");await writeFile(index,"module.exports = {};\n");
  expect(nxBuildEngineSourceFileV1(owner,"./dir/")).toBe(createRequire(owner).resolve("./dir/"));
  expect(()=>nxBuildEngineSourceFileV1(owner,"./missing.json")).toThrow("Private Nx engine import missing");
  expect(()=>nxBuildEngineSourceFileV1(owner,"./missing/")).toThrow("Private Nx engine import missing");
 }finally{await rm(root,{recursive:true,force:true});}
});
test("private engine references load explicit plugin files without full workspace package inference",async()=>{
 const validate=new(require("ajv").default)({strict:true}).compile(pluginSchema);expect(validate(fixture.pluginReferences)).toBe(true);
 const directory=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"plugin-paths-")),root=join(directory,"workspace"),engine=join(directory,"engine"),sdk=createRequire(resolve(process.cwd(),".nx/installation/package.json")),retrieve=sdk("nx/src/project-graph/utils/retrieve-workspace-files"),resolver=sdk("nx/src/project-graph/plugins/resolve-plugin"),original=retrieve.retrieveProjectConfigurationsWithoutPluginInference;
 try{await mkdir(root,{recursive:true});await mkdir(engine);await writeFile(join(root,"plugin.mjs"),"export default {};\n");await writeFile(join(engine,"plugin.mjs"),"export default {};\n");
  for(const row of fixture.pluginReferences){let lookups=0;resolver.resetResolvePluginCache();retrieve.retrieveProjectConfigurationsWithoutPluginInference=async()=>{lookups++;return {};};
   const oracle=()=>createRequire(join(root,"package.json")).resolve(row.reference),actual=()=>resolver.resolveNxPlugin(row.reference,root,[root]);
   if(row.reference.includes("missing")||row.lookup){expect(oracle).toThrow();await expect(actual()).rejects.toThrow();}else{const resolved=await actual();expect(resolved.pluginPath).toBe(oracle());}
   expect(lookups).toBe(row.lookup?1:0);
  }
 }finally{retrieve.retrieveProjectConfigurationsWithoutPluginInference=original;resolver.resetResolvePluginCache();await rm(directory,{recursive:true,force:true});}
});
test("private engine references admit complete Source membership through bounded concurrent physical reads",async()=>{
 const row=fixture.admissionWalk,validate=new(require("ajv").default)({strict:true}).compile(admissionSchema),root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"admission-walk-"));let active=0,maximum=0;const found:string[]=[];
 try{for(const[path,bytes]of Object.entries(row.files)){await mkdir(dirname(join(root,path)),{recursive:true});await writeFile(join(root,path),bytes);}
  expect(validate({maximumConcurrentReads:row.maximumConcurrentReads})).toBe(true);
  await boundedAdmissionWalkV1([""],row.maximumConcurrentReads,async(directory,enqueue)=>{active++;maximum=Math.max(maximum,active);try{for(const entry of await rawPromises.readdir(join(root,directory),{withFileTypes:true})){const path=directory?directory+"/"+entry.name:entry.name;if(entry.isDirectory())enqueue(path);else if(entry.isFile())found.push(path);}}finally{active--;}});
  const oracle=await independentGlobV1("**/*.json",{cwd:root,dot:true});expect(found.sort()).toEqual(row.expected);expect(found).toEqual(oracle.sort());expect(maximum).toBe(row.maximumConcurrentReads);expect(active).toBe(0);
  for(const limit of row.rejected){let dispatched=0;expect(validate({maximumConcurrentReads:limit})).toBe(false);await expect(boundedAdmissionWalkV1([""],limit,async()=>{dispatched++;})).rejects.toThrow("Bounded admission concurrency");expect(dispatched).toBe(0);}
  let completed=false;const failure=boundedAdmissionWalkV1(["bad","waiting"],2,async(value)=>{if(value==="bad")throw Error("admission-stop");await new Promise(accept=>setTimeout(accept,20));completed=true;});await expect(failure).rejects.toThrow("admission-stop");expect(completed).toBe(true);
 }finally{await rm(root,{recursive:true,force:true});}
});
test("private engine references retain nested structural Source owners and newly admitted ignored leaves",async()=>{
 const row=nxAdmissionFixture.collectionAdmission,root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"structural-walk-"));
 try{for(const[path,bytes]of Object.entries(row.files)){await mkdir(dirname(join(root,path)),{recursive:true});await writeFile(join(root,path),bytes);}
  const inventory=[row.indexed.join("\0"),row.untracked.join("\0")],actual=await collectionManifestsFromInventory(root,inventory,nxAdmissionPolicy,row.collections),oracle=await independentGlobV1(["**/📋️project.json","**/package.json","**/Cargo.toml"],{cwd:root,dot:true,ignore:["**/dist/**","**/owner/🧫️examples/**"]});expect(actual).toEqual(row.expected);expect(actual).toEqual(oracle.sort());
  await mkdir(dirname(join(root,row.additional.path)),{recursive:true});await writeFile(join(root,row.additional.path),row.additional.content);expect(await collectionManifestsFromInventory(root,inventory,nxAdmissionPolicy,row.collections)).toEqual([...row.expected,row.additional.path].sort());
 }finally{await rm(root,{recursive:true,force:true});}
});
test("private engine references admit only closed original inference dataset identities",()=>{
 const validate=new(require("ajv").default)({strict:true}).compile(datasetSchema);expect(validate(fixture.inferenceDataset.valid)).toBe(true);assert.deepEqual(readBuildInferenceDatasetV1(fixture.inferenceDataset.valid),fixture.inferenceDataset.valid);
 for(const value of fixture.inferenceDataset.refused){expect(validate(value)).toBe(false);expect(()=>readBuildInferenceDatasetV1(value)).toThrow("Closed original inference dataset");}
});
test("private Nx dependency projection preserves portable authority with indexed provenance",async()=>{
 const rows=fixture.nxDependencyProjection,{ProjectGraphBuilder}=nxOracleRequireV1("nx/src/project-graph/project-graph-builder.js"),api=await import("../../../../🟨️.mjs"),portable=graph(),externals={"npm:a":{type:"npm",name:"npm:a",data:{packageName:"a",version:"1.0.0"}},"npm:b":{type:"npm",name:"npm:b",data:{packageName:"b",version:"1.0.0"}}},context={workspaceRoot:process.cwd(),projects:portable.projects,externalNodes:externals,fileMap:{projectFileMap:structuredClone(rows.projectFileMap),nonProjectFiles:structuredClone(rows.nonProjectFiles)}};
 const builder=()=>new ProjectGraphBuilder({nodes:Object.fromEntries(Object.entries(portable.projects).map(([name,data])=>[name,{name,type:"lib",data}])),externalNodes:externals,dependencies:{}},structuredClone(rows.projectFileMap),structuredClone(rows.nonProjectFiles)),add=(target:any,edge:any)=>target.addDependency(edge.source,edge.target,edge.type,edge.sourceFile),unindexed=rows.cases.find(row=>row.edge.sourceFile?.includes("generated"))!.edge;
 expect(()=>add(builder(),unindexed)).toThrow("does not exist in the workspace");expect(()=>add(builder(),rows.cases.find(row=>row.edge.source==="app"&&row.edge.target==="npm:a")!.edge)).toThrow("Source project file is required");expect(typeof api.projectBuildDependenciesForNxV1).toBe("function");
 const candidate={...portable,externalNodes:externals,dependencies:rows.cases.map(row=>row.edge)},before=JSON.stringify(candidate),digest=createHash("sha256").update(before).digest("hex"),projected=api.projectBuildDependenciesForNxV1(candidate,context),actual=builder();expect(projected).toEqual(rows.cases.map(row=>row.expected));for(const edge of projected)add(actual,edge);expect(projected.map((edge:any)=>[edge.source,edge.target])).toEqual(candidate.dependencies.map(edge=>[edge.source,edge.target]));expect(createHash("sha256").update(JSON.stringify(candidate)).digest("hex")).toBe(digest);expect(candidate.projects).toBe(portable.projects);expect(JSON.stringify(candidate)).toBe(before);
 expect(()=>api.projectBuildDependenciesForNxV1({...candidate,dependencies:[rows.otherOwner]},context)).toThrow();for(const edge of[{source:"missing",target:"lib",type:"implicit"},{source:"app",target:"missing",type:"implicit"}])expect(()=>{for(const projected of api.projectBuildDependenciesForNxV1({...candidate,dependencies:[edge]},context))add(builder(),projected);}).toThrow();
 const validate=new Ajv({strict:false}).compile(schema);expect(validate(candidate)).toBe(true);expect(readBuildGraphV1(candidate).dependencies).toEqual(candidate.dependencies);
});
test("private Nx implementation authority retains evaluated closure identity and current Source guards",async()=>{
 const row=fixture.nxImplementationAuthority,library=resolve(import.meta.dir,"../../../.."),api=await import("../../../../🟨️.mjs"),arguments_=await import("../../../📤️arguments/🟨️.mjs"),hash=(value:string)=>createHash("sha256").update(value).digest("hex"),file=(path:string)=>createHash("sha256").update(readFileSync(join(library,path))).digest("hex"),argumentIdentity=hash(JSON.stringify(row.arguments.map(file))),expected=hash(JSON.stringify([...row.projection.map(file),argumentIdentity,...row.privateTasks.map(file)])),identity=api.nxBuildDependencyImplementationV1();
 expect(arguments_.ownerArgumentsSourceIdentityV1()).toBe(argumentIdentity);expect(identity).toBe(expected);expect(api.nxBuildDependencyImplementationV1()).toBe(identity);
 const oracle=Bun.spawn(["node","--input-type=module","-e",'import{createHash}from"node:crypto";import{readFileSync}from"node:fs";import{join}from"node:path";const [root,row]=JSON.parse(process.argv[1]),hash=value=>createHash("sha256").update(value).digest("hex"),file=path=>hash(readFileSync(join(root,path)));console.log(hash(JSON.stringify([...row.projection.map(file),hash(JSON.stringify(row.arguments.map(file))),...row.privateTasks.map(file)])));',JSON.stringify([library,row])],{stdout:"pipe",stderr:"pipe"}),[output,errors,status]=await Promise.all([new Response(oracle.stdout).text(),new Response(oracle.stderr).text(),oracle.exited]);expect(status,errors).toBe(0);expect(output.trim()).toBe(identity);
 const scratch=await mkdtemp(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"guard-")),guard=join(import.meta.dir,"🔬️source-guard"),sources=[...row.arguments,...row.projection,...row.privateTasks],child=Bun.spawn(["node","--import",join(guard,"📜️preload","🟨️.mjs"),join(guard,"📜️child","🟨️.mjs"),JSON.stringify([library,scratch,sources.map(source=>({source,mutation:row.mutation}))])],{stdout:"pipe",stderr:"pipe"}),[stdout,stderr,code]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
 expect(code,stderr).toBe(0);const results=JSON.parse(stdout.trim().split("\n").at(-1)!) as {source:string;refusal:string|null;restored:boolean}[];expect(results.map(result=>result.source)).toEqual(sources);for(const result of results){expect(result.refusal,result.source).toMatch(/Source changed|descriptor changed|concurrent/iu);expect(result.restored,result.source).toBe(true);}await rm(scratch,{recursive:true,force:true});
 const directory=await mkdtemp(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"ia-")),path=join(directory,"graph.json"),bytes=JSON.stringify(fixture.graph),previous={SEMIO_BUILD_GRAPH_PATH:process.env.SEMIO_BUILD_GRAPH_PATH,SEMIO_BUILD_GRAPH_DIGEST:process.env.SEMIO_BUILD_GRAPH_DIGEST};try{await writeFile(path,bytes);process.env.SEMIO_BUILD_GRAPH_PATH=path;process.env.SEMIO_BUILD_GRAPH_DIGEST=hash(bytes);const owners=Object.entries(graph().projects).map(([name,project]):readonly[string,string]=>[name,project.root]).sort(([a],[b])=>a.localeCompare(b)),ordinary=readBuildGraphV1(JSON.parse(readFileSync(path,"utf8"))),authority=api.projectBuildDependencyAuthorityV1(process.cwd(),ordinary,hash(bytes),owners,identity);expect(authority).toBe(hash(JSON.stringify([process.cwd(),hash(bytes),owners,identity])));expect(api.projectBuildDependencyAuthorityV1(process.cwd(),ordinary,hash(bytes),owners,identity)).toBe(authority);}finally{for(const[key,value]of Object.entries(previous))if(value===undefined)delete process.env[key];else process.env[key]=value;await rm(directory,{recursive:true,force:true});}
});
test("private Nx engine named inputs preserve real native hash plans and reduce projection bytes",async()=>{
 const row=fixture.nxEngineNamedInput,engine=nxBuildEngineInputsV1(),sdk=nxOracleRequireV1,portable={version:1,projects:{app:{name:"app",root:"app",targets:Object.fromEntries(row.cases.map(entry=>[entry.name,{execution:{kind:"command",command:"bun app.ts",cwd:"app"},cache:true,...("inputs"in entry?{inputs:entry.inputs}:{}),...("configurations"in entry?{configurations:entry.configurations}:{})}]))}},dependencies:[]},projected=projectBuildGraphForNxV1(readBuildGraphV1(portable)).projects.app;
 assert.deepEqual(testValueV1(projected).namedInputs[row.name],engine);const native=sdk("nx/src/native"),transform=sdk("nx/src/native/transform-objects").transformProjectGraphForRust,inputs=sdk("nx/src/hasher/task-hasher");for(const entry of row.cases){expect(testValueV1(testValueV1(projected).targets[entry.name]).inputs).toEqual([...(entry.inputs??["default","^default"]),row.name]);const baseline={...projected,namedInputs:{...testValueV1(projected).namedInputs},targets:{...testValueV1(projected).targets,[entry.name]:{...testValueV1(projected).targets[entry.name],inputs:[...(entry.inputs??["default","^default"]),...engine]}}};delete baseline.namedInputs[row.name];const expanded=(project:any)=>inputs.getInputs({target:{project:"app",target:entry.name}},{nodes:{app:{name:"app",type:"lib",data:project}},dependencies:{app:[]},externalNodes:{}},{});expect(expanded(projected)).toEqual(expanded(baseline));}
 const external=engine.flatMap((value:any)=>value?.externalDependencies??[]),externalNodes=Object.fromEntries(external.map((name:string)=>["npm:"+name,{name:"npm:"+name,type:"npm",data:{packageName:name,version:"1.0.0",hash:"fixed-"+name}}])),swap=(value:any,digest:string):any=>Array.isArray(value)?value.map(item=>swap(item,digest)):value&&typeof value==="object"?value.runtime===(engine[0] as any).runtime?{runtime:`node -p "'${digest}'"`}:Object.fromEntries(Object.entries(value).map(([key,item])=>[key,swap(item,digest)])):value,hash=(source:any,target:string,configuration:string|undefined,changed:boolean)=>{const project=swap(source,changed?"engine-after":"engine-before"),task={id:"app:"+target+(configuration?":"+configuration:""),target:{project:"app",target,...(configuration?{configuration}:{})},overrides:{},outputs:[],projectRoot:"app",cache:true,parallelism:true},graph={nodes:{app:{name:"app",type:"lib",data:project}},externalNodes,dependencies:{app:[]}},taskGraph={roots:[task.id],tasks:{[task.id]:task},dependencies:{[task.id]:[]},continuousDependencies:{[task.id]:[]}},graphRef=native.transferProjectGraph(transform(graph)),filemap=native.testOnlyTransferFileMap({app:[{file:"app/index.ts",hash:"app-fixed"},{file:"app/release.ts",hash:"release-fixed"}]},[]),planner=new native.HashPlanner({},graphRef),plans=planner.getPlansReference([task.id],taskGraph),hasher=new native.TaskHasher(process.cwd(),graphRef,filemap.projectFiles,filemap.allWorkspaceFiles,Buffer.from("{}"),{},undefined,{selectivelyHashTsConfig:false});const result=hasher.hashPlans(plans,{[task.id]:{}},process.cwd(),true)[task.id];return {value:result.value,details:result.details,instructions:planner.getPlans([task.id],taskGraph)[task.id]};};
 for(const entry of row.cases){const baseline={...projected,namedInputs:{...testValueV1(projected).namedInputs},targets:Object.fromEntries(Object.entries(testValueV1(projected).targets).map(([name,target]:[string,any])=>[name,{...target,inputs:target.inputs.flatMap((input:unknown)=>input===row.name?engine:[input])}]))};delete baseline.namedInputs[row.name];for(const configuration of[undefined,...Object.keys(entry.configurations??{})]){const current=hash(projected,entry.name,configuration,false),expanded=hash(baseline,entry.name,configuration,false);const semantic=(value:Record<string,string>)=>Object.fromEntries(Object.entries(value).filter(([key])=>key!=="app:ProjectConfiguration"));expect(current.instructions).toEqual(expanded.instructions);expect(semantic(current.details)).toEqual(semantic(expanded.details));expect(current.details["app:ProjectConfiguration"]).not.toBe(expanded.details["app:ProjectConfiguration"]);expect(hash(projected,entry.name,configuration,false).value).toBe(current.value);expect(hash(projected,entry.name,configuration,true).value).not.toBe(current.value);}}
 for(const location of["graph","project"]){const invalid=structuredClone(portable)as any;if(location==="graph")invalid.namedInputs={[row.name]:engine};else invalid.projects.app.namedInputs={[row.name]:engine};expect(()=>projectBuildGraphForNxV1(invalid)).toThrow("reserved");}
 const expanded={...projected,namedInputs:{...testValueV1(projected).namedInputs},targets:Object.fromEntries(Object.entries(testValueV1(projected).targets).map(([name,target]:[string,any])=>[name,{...target,inputs:target.inputs.flatMap((input:unknown)=>input===row.name?engine:[input])}]))};delete expanded.namedInputs[row.name];const compactBytes=Buffer.byteLength(JSON.stringify(projected)),expandedBytes=Buffer.byteLength(JSON.stringify(expanded));expect(compactBytes).toBeLessThan(expandedBytes);
});

test("the private Nx engine names only real npm packages as external dependencies and follows workspace packages into their Source files (independent realpath oracle)",()=>{
 const root=process.cwd(),external=nxBuildEngineInputsV1().flatMap((value:any)=>value?.externalDependencies??[]) as string[];
 const oracle=(name:string)=>{const run=Bun.spawnSync(["python3","-I","-c","import os,sys;root,name=sys.argv[1:3];real=os.path.realpath(os.path.join(root,'node_modules',name));print('workspace' if real.startswith(root+os.sep) and 'node_modules' not in os.path.relpath(real,root).split(os.sep) else 'npm')",root,name]);expect(run.exitCode).toBe(0);return run.stdout.toString().trim();};
 expect(external.length).toBeGreaterThan(0);for(const name of external)expect(oracle(name),name).toBe("npm");
 expect(oracle("@semio-tech/process")).toBe("workspace");
 const file=nxBuildEngineWorkspacePackageFileV1(root,"@semio-tech/process/routing");expect(file).toBeDefined();expect(file!.startsWith(root)).toBe(true);expect(file!.split("/")).not.toContain("node_modules");
 expect(nxBuildEngineWorkspacePackageFileV1(root,"typescript")).toBeUndefined();
});

test("an Nx target caches only when every output lies in a partition the cache hook can hold, equal to the actual output classifier",()=>{
 const root=process.cwd(),rust=join(root,budgetPolicy.partitions.rust.directory,"m"),products=join(root,budgetPolicy.partitions.products.directory,"m"),roots={rust,products},rows=[
  {outputs:undefined,cache:true},{outputs:[],cache:true},
  {outputs:["{workspaceRoot}/"+budgetPolicy.partitions.rust.directory+"/m/u/l/d1/b/out"],cache:true},
  {outputs:["{workspaceRoot}/"+budgetPolicy.partitions.products.directory+"/m/artifact/out"],cache:true},
  {outputs:["{workspaceRoot}/🧰️framework/🤖️generated/🟦️.ts"],cache:false},
  {outputs:["{projectRoot}/dist"],cache:false},
  {outputs:["{workspaceRoot}/"+budgetPolicy.partitions.products.directory+"/m/artifact/out","{projectRoot}/dist"],cache:false},
  {outputs:["{workspaceRoot}/"+budgetPolicy.partitions.rust.directory+"/m/u/x","!{projectRoot}/dist"],cache:true},
  {outputs:["{workspaceRoot}/"+budgetPolicy.partitions.products.directory+"/x/out"],cache:false},
 ];
 for(const row of rows){
  const target={execution:{kind:"command",command:"repo ./📜️script.ts"},cache:true,...(row.outputs===undefined?{}:{outputs:row.outputs})},projected=projectBuildTargetForNxV1(target as any);
  expect(projected.cache,JSON.stringify(row.outputs)).toBe(row.cache);expect(nxOutputsCacheableV1(row.outputs??[])).toBe(row.cache);
  for(const output of (row.outputs??[]).filter(value=>!value.startsWith("!")&&value.startsWith("{workspaceRoot}/"))){
   const absolute=join(root,output.slice("{workspaceRoot}/".length));let admitted=true;
   try{classifyNxOutputV1(roots,absolute);}catch(error){if(!String((error as Error).message).includes("escapes declared cache partitions"))throw error;admitted=false;}
   if(output.includes("/m/")&&!output.includes("/x/out"))expect(admitted,output).toBe(true);else if(!output.includes("/m/"))expect(admitted,output).toBe(false);
  }
 }
 const explicit=projectBuildTargetForNxV1({execution:{kind:"command",command:"repo ./📜️script.ts"},cache:false,outputs:["{workspaceRoot}/"+budgetPolicy.partitions.rust.directory+"/m/u/x"]} as any);expect(explicit.cache).toBe(false);
 expect(projectBuildTargetForNxV1({execution:{kind:"noop"},cache:true} as any).cache).toBe(true);
});

test("a shared named input group is held once while the real native hasher reacts to the same files in every project",()=>{
 const sdk=createRequire(resolve(process.cwd(),".nx/installation/package.json")),native=sdk("nx/src/native"),transform=sdk("nx/src/native/transform-objects").transformProjectGraphForRust;
 const files=Array.from({length:60},(_,index)=>"{workspaceRoot}/shared/"+String(index).padStart(2,"0")+"-"+"x".repeat(40)+".ts"),extra="{workspaceRoot}/shared/extra.ts",target={executor:"@semio-tech/repo-lib:command",options:{command:"x"},inputs:["big"],cache:true};
 const make=(root:string,list:string[])=>({name:root,root,namedInputs:{big:list},targets:{build:structuredClone(target)}}),original:Record<string,any>={a:make("a",files),b:make("b",files),c:make("c",[...files,extra])},shared=shareNxProjectInputsV1(structuredClone(original));
 expect(shared.a.namedInputs.big).toEqual(files);expect("big" in shared.b.namedInputs).toBe(false);expect(shared.b.targets.build.inputs).toEqual([{input:"big",projects:["a"]}]);expect(shared.a.targets.build.inputs).toEqual(["big"]);expect(shared.c.namedInputs.big).toEqual(original.c.namedInputs.big);
 expect(Buffer.byteLength(JSON.stringify(shared))).toBeLessThan(Buffer.byteLength(JSON.stringify(original))*0.75);
 const small={a:{...original.a,namedInputs:{big:["{workspaceRoot}/s.ts"]}},b:{...original.b,namedInputs:{big:["{workspaceRoot}/s.ts"]}}};expect(shareNxProjectInputsV1(structuredClone(small)).b.namedInputs.big).toEqual(["{workspaceRoot}/s.ts"]);
 const relative={a:{...original.a,namedInputs:{big:[...files,"default"]}},b:{...original.b,namedInputs:{big:[...files,"default"]}}};expect(shareNxProjectInputsV1(structuredClone(relative)).b.namedInputs.big).toEqual([...files,"default"]);
 const nested={a:{...original.a},b:{...original.b,namedInputs:{big:files,production:["big"]}}};expect(shareNxProjectInputsV1(structuredClone(nested)).b.namedInputs.big).toEqual(files);
 const hash=(projects:Record<string,any>,name:string,changed:string|undefined)=>{
  const task={id:name+":build",target:{project:name,target:"build"},overrides:{},outputs:[],projectRoot:name,cache:true,parallelism:true},graph={nodes:Object.fromEntries(Object.entries(projects).map(([key,data])=>[key,{name:key,type:"lib",data}])),externalNodes:{},dependencies:Object.fromEntries(Object.keys(projects).map(key=>[key,[]]))},taskGraph={roots:[task.id],tasks:{[task.id]:task},dependencies:{[task.id]:[]},continuousDependencies:{[task.id]:[]}},graphRef=native.transferProjectGraph(transform(graph));
  const workspace=[...files,extra,"{workspaceRoot}/unrelated.ts"].map(value=>value.slice("{workspaceRoot}/".length)).map(file=>({file,hash:file===changed?"changed":"fixed"})),filemap=native.testOnlyTransferFileMap(Object.fromEntries(Object.keys(projects).map(key=>[key,[{file:key+"/index.ts",hash:"index"}]])),workspace),planner=new native.HashPlanner({},graphRef),plans=planner.getPlansReference([task.id],taskGraph),hasher=new native.TaskHasher(process.cwd(),graphRef,filemap.projectFiles,filemap.allWorkspaceFiles,Buffer.from("{}"),{},undefined,{selectivelyHashTsConfig:false});
  return hasher.hashPlans(plans,{[task.id]:{}},process.cwd(),true)[task.id].value;
 };
 const probes=[files[0],files[59],extra,"{workspaceRoot}/unrelated.ts"].map(value=>value.slice("{workspaceRoot}/".length)),sensitivity=(projects:Record<string,any>,name:string)=>probes.map(probe=>hash(projects,name,probe)!==hash(projects,name,undefined));
 for(const name of ["a","b","c"]){const before=sensitivity(original,name),after=sensitivity(shared,name);expect(after,name).toEqual(before);}
 expect(sensitivity(shared,"b")).toEqual([true,true,false,false]);expect(sensitivity(shared,"c")).toEqual([true,true,true,false]);
});

test("project implicit dependencies outside the published slice are dropped, equal to Nx's own workspace validity check",()=>{
 const sdk=createRequire(resolve(process.cwd(),".nx/installation/package.json")),{assertWorkspaceValidity}=sdk("nx/src/utils/assert-workspace-validity"),make=(name:string,implicit?:unknown)=>({name,root:name,targets:{},...(implicit===undefined?{}:{implicitDependencies:implicit})});
 const portable=(projects:Record<string,any>)=>readBuildGraphV1({version:1,projects,dependencies:[]} as any),run=(projects:Record<string,any>)=>projectBuildGraphForNxV1(portable(projects)).projects as Record<string,any>;
 const dangling=run({a:make("a",["b","missing"]),b:make("b",["a"])});
 expect(()=>assertWorkspaceValidity(dangling,{})).not.toThrow();expect(dangling.a.implicitDependencies).toEqual(["b"]);expect(dangling.b.implicitDependencies).toEqual(["a"]);
 expect(run({a:make("a",["b","missing","tag:x","packages/*","!c"]),b:make("b")}).a.implicitDependencies).toEqual(["b","tag:x","packages/*","!c"]);
 const raw={a:{...make("a",["missing"])},b:make("b")};expect(()=>assertWorkspaceValidity(structuredClone(raw),{})).toThrow("non-existent project");
 expect(run({a:make("a"),b:make("b",[])}).b.implicitDependencies).toEqual([]);expect("implicitDependencies" in run({a:make("a")}).a).toBe(false);
});
