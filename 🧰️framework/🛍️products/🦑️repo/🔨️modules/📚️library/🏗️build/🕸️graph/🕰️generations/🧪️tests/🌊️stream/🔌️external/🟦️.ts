import generationSchema from"../../../🧬️schema/🔣️.json";
import{readBuildGraphV1,requireBuildGraphExternalInputsV1}from"../../../../🟦️.ts";
import{nxOracleRequireV1}from"../../../../../🧪️tests/🕸️graph/🧰️support/🟦️.ts";
import{expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{readFileSync}from"node:fs";
import{join,resolve}from"node:path";
import Ajv from"ajv";
import ts from"typescript";
import{Database}from"bun:sqlite";
import{createGenerationStoreV1,createFrontierV1,readGenerationExternalInputsV1}from"../../../🟨️.mjs";
import{loadGenerationGraphV1}from"../../../🔌️load/🟨️.mjs";
import{createMemoryFilePort}from"../../🏃️world/🟦️.ts";
import{createGraphPhaseClockV1}from"../../../../⏱️phases/🟨️.mjs";
import{bunLockGraph}from"../../../../📥️inference/🧶️bun-lock/🟨️.mjs";
import{projectBuildGraphForNxV1,nxBuildEngineInputsV1,nxBuildEngineExternalInputsV1}from"../../../../../../🟨️.mjs";
import fixture from"./🔣️.json";
import schema from"./📋️schema.json";

const sha=(text:string)=>createHash("sha256").update(text).digest("hex"),owner="@semio-tech/assets";
async function publish(externalNodes:Record<string,unknown>,edges:unknown[],input?:string,request:{project:string;target:string;prerequisite:string;dependencyProject?:string}={project:owner,target:"build",prerequisite:"build"}){
 const owner=request.project;
 const memory=createMemoryFilePort(),options={root:"/store",fs:memory.port as never,holds:async()=>true,resources:{facts:"facts"}},store=createGenerationStoreV1(options),identity=sha("external-input-identity"),implementation=sha("[]"),membership=async()=>({digest:sha("membership"),frontier:createFrontierV1({directories:[],members:[],capturedAt:0}),changes:[]});
 const build={execution:{kind:"command",command:"node assets.js"},cache:true,inputs:input?[{externalDependencies:[input]}]:[]},node={name:owner,root:"assets",targets:{build,...request.target==="build"?{}:{[request.target]:{...build,dependsOn:[(request.dependencyProject?request.dependencyProject+":":"")+request.prerequisite]}}}};
 const result=await store.refresh({identity,implementation,membership,compute:async()=>({owners:[{name:owner,root:"assets",kind:"project",node,edges:request.dependencyProject?[{source:owner,target:request.dependencyProject,type:"implicit"}]:[],targets:Object.keys(node.targets),rows:[]},...request.dependencyProject?[{name:request.dependencyProject,root:"dependency",kind:"project",node:{name:request.dependencyProject,root:"dependency",targets:{[request.prerequisite]:build}},edges:[],targets:[request.prerequisite],rows:[]}]:[]],env:{digest:sha("environment"),rows:[]},global:{version:1,externalNodes,edges,unavailable:[]}})});
 const host={workspaceRoot:"/workspace",namedInputs:{default:[]},registryDigest:sha("registry"),signal:undefined,identity:async()=>identity,implementationRows:async()=>[],withShared:async(body:Function)=>body({},{}),withBuilder:async()=>{throw Error("A sealed test reader does not build");},storeOptions:()=>options,membership,discoverPart:async()=>{throw Error("No discovery in a sealed view");},expand:async(value:unknown)=>value,readGraph:readBuildGraphV1,readLean:(value:unknown)=>value,clock:()=>createGraphPhaseClockV1({publish:()=>{}}),say:()=>{}};
 const selection={kind:"tasks",tasks:[{project:owner,target:request.target}]};
 return{store,current:result.current,selected:async(externalInputs:readonly string[])=>loadGenerationGraphV1(host as never,{policy:"current",selection,slice:{selection,roots:[owner]},externalInputs} as never) as Promise<any>};
}
test("external input references preserve canonical sealed nodes and transitive pruning",async()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const db=new Database(":memory:");
 try{db.exec("create table nodes(name text primary key);create table edges(source text,target text)");const names=[...new Set([...fixture.cases.map(row=>row.canonical),...fixture.edges.flatMap(row=>[row.source,row.target]),fixture.unrelated])],nodes=Object.fromEntries(names.map(name=>[name,{name,type:"npm",data:{packageName:name.replace(/^npm:/u,""),version:"1.0.0",hash:sha(name)}}]));
 for(const name of names)db.query("insert into nodes values(?)").run(name);for(const edge of fixture.edges)db.query("insert into edges values(?,?)").run(edge.source,edge.target);
 for(const row of fixture.cases){const canonical=(db.query("select name from nodes where name=?1 or name='npm:'||?1 order by case when name=?1 then 0 else 1 end limit 1").get(row.reference)as{name:string}).name,expected=db.query("with recursive closure(name) as(select ?1 union select edges.target from edges join closure on edges.source=closure.name) select name from closure order by name").all(canonical).map((row:any)=>row.name),state=await publish(nodes,fixture.edges,row.reference);
 const explicit=await state.store.assemble(state.current,{names:[owner]}),consumer=await state.store.assemble(state.current,{names:[owner],externalInputs:[row.reference]} as never);
 expect(Object.keys(explicit.externalNodes).sort(),row.reference).toEqual(expected);expect(Object.keys(consumer.externalNodes).sort(),row.reference).toEqual(expected);for(const key of expected)expect(consumer.externalNodes[key]).toEqual(nodes[key]);expect(consumer.externalNodes[fixture.unrelated]).toBeUndefined();await expect(state.store.assemble(state.current,{names:[owner],externalInputs:["missing:original"]} as never)).rejects.toMatchObject({code:"GENERATION_UNKNOWN_EXTERNAL"});}
 }finally{db.close();}
});
test("selected backend engine external roots reach the real native Nx HashPlanner",async()=>{
 const root=process.cwd(),lock=ts.parseConfigFileTextToJson("bun.lock",readFileSync(join(root,"bun.lock"),"utf8")).config,patches=Object.fromEntries(Object.values(lock.patchedDependencies??{}).map((path:any)=>[path,readFileSync(resolve(root,path))])),locked=bunLockGraph(lock,patches),unused="npm:semio-unrelated-fixture",nodes={...locked.externalNodes,[unused]:{type:"npm",name:unused,data:{packageName:"semio-unrelated-fixture",version:"1.0.0",hash:sha("unused")}}},state=await publish(nodes,locked.dependencies);
 const external=nxBuildEngineExternalInputsV1(),selected=await state.selected(external),projection=projectBuildGraphForNxV1(selected),nxGraph={nodes:Object.fromEntries(Object.entries(projection.projects).map(([name,data])=>[name,{name,type:"lib",data}])),externalNodes:projection.externalNodes,dependencies:Object.fromEntries([...Object.keys(projection.projects),...Object.keys(projection.externalNodes)].map(name=>[name,selected.dependencies.filter((edge:any)=>edge.source===name)]))};
 const native=nxOracleRequireV1("nx/src/native"),transform=nxOracleRequireV1("nx/src/native/transform-objects").transformProjectGraphForRust,taskGraph=nxOracleRequireV1("nx/src/tasks-runner/create-task-graph").createTaskGraph(nxGraph,{},[owner],["build"],undefined,{}),reference=native.transferProjectGraph(transform(nxGraph)),planner=new native.HashPlanner({},reference);
 const plans=planner.getPlans([owner+":build"],taskGraph);expect(JSON.stringify(plans)).toContain("npm:nx");
 for(const name of external)expect(projection.externalNodes["npm:"+name]).toEqual(locked.externalNodes["npm:"+name]);expect(projection.externalNodes[unused]).toBeUndefined();expect(Object.keys(projection.externalNodes).length).toBeLessThan(Object.keys(nodes).length);
 const without=await state.selected([]),canonical=await state.selected(external.map(name=>"npm:"+name));expect(selected.admission.closure).not.toBe(without.admission.closure);expect(selected.admission.closure).toBe(canonical.admission.closure);expect(()=>requireBuildGraphExternalInputsV1(selected,external)).not.toThrow();expect(()=>requireBuildGraphExternalInputsV1(without,external)).toThrow("lacks external input");expect(()=>requireBuildGraphExternalInputsV1(selected,external.slice(1))).toThrow("admission");expect(external).toEqual([...new Set([...nxBuildEngineInputsV1(),...nxBuildEngineInputsV1("native")].flatMap((row:any)=>row.externalDependencies??[]))].sort());
});

test("consumer external roots obey their closed bounded schema",()=>{const validate=new Ajv({strict:true}).compile(generationSchema.definitions.ExternalInputs);for(const value of[["nx","@scope/tool"],[],null,[""],["x".repeat(4097)],Array.from({length:65537},()=>"nx")]){const valid=validate(value);if(valid)expect(readGenerationExternalInputsV1(value)).toEqual([...new Set(value as string[])].sort());else expect(()=>readGenerationExternalInputsV1(value)).toThrow("Bounded external input");}});

test("outer invocation admits backend engine inputs before pinning both target closures",async()=>{
 const{loadBuildInvocationGraphV1}=await import("../../../../../🧭️invocation/🕸️graph/🟦️.ts"),{loadBuildBackendV1}=await import("../../../../../🔌️backends/🟦️.ts"),{withGraphTestInvocationV1}=await import("../../../../../🧪️tests/🕸️graph/🧰️support/🟦️.ts"),{createIssuedResourceControlV1}=await import("../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const root=process.cwd(),lock=ts.parseConfigFileTextToJson("bun.lock",readFileSync(join(root,"bun.lock"),"utf8")).config,patches=Object.fromEntries(Object.values(lock.patchedDependencies??{}).map((path:any)=>[path,readFileSync(resolve(root,path))])),locked=bunLockGraph(lock,patches),backend=await loadBuildBackendV1("nx"),external=await backend.graphExternalInputs!(),native=nxOracleRequireV1("nx/src/native"),transform=nxOracleRequireV1("nx/src/native/transform-objects").transformProjectGraphForRust;
 const db=new Database(":memory:");try{db.exec("create table dependencies(source text,target text)");for(const edge of locked.dependencies)db.query("insert into dependencies values(?,?)").run(edge.source,edge.target);
 for(const row of fixture.invocations){const state=await publish(locked.externalNodes,locked.dependencies,undefined,row);await withGraphTestInvocationV1(async original=>{
 const graph=await loadBuildInvocationGraphV1(root,["run",row.project+":"+row.target],original,original,createIssuedResourceControlV1(original,"graph:backend-external-law",0),false,async(_root,_invocation,_phase,options)=>state.selected(options.graph?.externalInputs??[]),undefined,false,external);
 expect(graph.admission?.selection).toEqual({kind:"tasks",tasks:[{project:row.project,target:row.target}]});requireBuildGraphExternalInputsV1(graph,external);
 const expected=new Set<string>();for(const key of external)for(const found of db.query("with recursive closure(name) as(select ?1 union select target from dependencies join closure on source=name) select name from closure").all("npm:"+key)as{name:string}[])expected.add(found.name);
 expect(Object.keys(graph.externalNodes??{}).sort()).toEqual([...expected].sort());for(const key of expected)expect(graph.externalNodes![key]).toEqual(locked.externalNodes[key]);
 const projection=projectBuildGraphForNxV1(graph),nxGraph={nodes:Object.fromEntries(Object.entries(projection.projects).map(([name,data])=>[name,{name,type:"lib",data}])),externalNodes:projection.externalNodes,dependencies:Object.fromEntries([...Object.keys(projection.projects),...Object.keys(projection.externalNodes)].map(name=>[name,graph.dependencies.filter(edge=>edge.source===name)]))},taskGraph=nxOracleRequireV1("nx/src/tasks-runner/create-task-graph").createTaskGraph(nxGraph,{},[row.project],[row.target],undefined,{}),planner=new native.HashPlanner({},native.transferProjectGraph(transform(nxGraph)));
 expect(Object.keys(taskGraph.tasks).sort()).toEqual([(row.dependencyProject??row.project)+":"+row.prerequisite,row.project+":"+row.target].sort());expect(JSON.stringify(planner.getPlans(Object.keys(taskGraph.tasks),taskGraph))).toContain("npm:nx");
 const omitted=await state.selected([]);expect(()=>requireBuildGraphExternalInputsV1(omitted,external)).toThrow("lacks external input");expect(graph.admission?.closure).not.toBe(omitted.admission?.closure);
 });}
 }finally{db.close();}
});
