/** 🧪️ Task sets of scoped plans agree with the actual Nx task graph planner. */
import {test,expect} from "bun:test";
import {join} from "node:path";
import {withRepositoryNxStorageV1} from "../../../../../⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🟦️.ts";
import {captureOwnedCommandV1} from "@semio-tech/process/execution";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import {createRequestedIssuedResourceControlV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import {privateRepositoryWorkspaceV1,testNxToolkitDirectoryV1} from "../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import {planScopedTargetsV1,type ScopeNodeV1} from "../../🟦️.ts";
import fixture from "../🔣️.json" with {type:"json"};
const projects=fixture.projects as unknown as Record<string,ScopeNodeV1>,cases=fixture.cases as {name:string;nx?:boolean;scope:unknown}[],ids=(plan:{tasks:readonly {project:string;target:string}[]})=>plan.tasks.map(row=>row.project+":"+row.target);
const counting={select:(patterns:readonly string[])=>patterns.filter(name=>name in projects),node:async(name:string)=>projects[name]};
const repository=await privateRepositoryWorkspaceV1();

test("task sets agree with actual Nx",async()=>withBuildTestInvocationV1(async original=>{
 const native={nodes:Object.fromEntries(Object.entries(projects).map(([name,node])=>[name,{name,type:"lib",data:{root:name,targets:Object.fromEntries(Object.entries(node.targets).map(([target,row])=>[target,{executor:"nx:noop",options:{},dependsOn:row.dependsOn??[]}]))}}])),dependencies:Object.fromEntries(Object.entries(projects).map(([name,node])=>[name,node.dependencies.map(target=>({source:name,target,type:"static"}))])),externalNodes:{}},rows=cases.filter(row=>row.nx);
 await withRepositoryNxStorageV1(repository.root,original,scriptCompilerAdmissionControlV1(original,"nx:fixture",0),async(bind,nxPhase)=>{
  const identity=process.env.SEMIO_NX_TOOLING_IDENTITY;if(!identity)throw Error("Original Nx SDK required");
  const namespace=await bind(identity,null,(await import("../../../../../🟨️.mjs")).nxBuildDependencyImplementationV1()),source="const sdk=require('node:module').createRequire(process.env.SDK),{createTaskGraph}=sdk('nx/src/tasks-runner/create-task-graph'),graph=JSON.parse(process.env.GRAPH),out={};for(const row of JSON.parse(process.env.CASES)){const t=row.scope.targets[0];out[row.name]=Object.keys(createTaskGraph(graph,{},[t.project],[t.target],undefined,{},false).tasks).sort();}console.log('[TRACE] nx-plan '+JSON.stringify(out));",
  output=await captureOwnedCommandV1("node",["-e",source],process.cwd(),{control:createRequestedIssuedResourceControlV1(nxPhase,30000),env:{...process.env,...namespace.environment,SDK:join(testNxToolkitDirectoryV1(identity),"package.json"),GRAPH:JSON.stringify(native),CASES:JSON.stringify(rows)}}),line=output.split("\n").find(row=>row.startsWith("[TRACE] nx-plan "));
  if(!line)throw Error("Original actual Nx task plan required");const actual=JSON.parse(line.slice("[TRACE] nx-plan ".length));
  for(const row of rows)expect(ids(await planScopedTargetsV1(counting,row.scope)).sort(),row.name).toEqual(actual[row.name]);
 });
}));
