import nxAdmissionFixture from "../../../⚡️caching/🧪️tests/🧫️fixtures/nx-contract/🔣️.json";
import nxAdmissionPolicy from "../../../⚡️caching/🔣️policy.json";
import {boundedAdmissionWalkV1,collectionManifestsFromInventory} from "../../../⚡️caching/🚀️bootstrap/📥️admission/🟦️.ts";
import admissionSchema from "../../../⚡️caching/🚀️bootstrap/📥️admission/🧬️schema/🔣️.json";
import {withRepositoryBuildGraphArtifactV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts";
import {repositoryBuildGraphDataDirectoryV1} from "../../📥️inputs/💾️storage/🟨️.mjs";
import {mkdir,mkdtemp,writeFile,readFile,rm,access,symlink} from "node:fs/promises";
import * as rawPromises from "node:fs/promises";
import {dirname,join,resolve,posix,win32} from "node:path";
import {ensureBuildEnvironmentV1} from "../../🚀️bootstrap/📜️script.ts";
import {projectBuildGraphForNxV1,projectBuildTargetForNxV1,nxBuildEngineInputsV1,nxBuildEngineReferencesV1,nxBuildEngineSourceFileV1} from "../../../🟨️.mjs";
import {expect,test,spyOn,beforeAll} from "bun:test";
import {buildInputPathMatchesV1} from "../../../🏗️build/📥️inputs/🟨️.mjs";
import {fileURLToPath} from "node:url";
import {createRequire} from "node:module";
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {build} from "esbuild";
import fixture from "../🧫️fixtures/🕸️graph/🔣️.json";
import pluginSchema from "../../🧬️schema/📥️plugin.json";
import engineSchema from "../../🧬️schema/📥️engine.json";
import callerSchema from "../../🧬️schema/📥️caller.json";
import initialAuthoringSchema from "../../🧬️schema/📥️authoring.json";
import dependencyInvocationSchema from "../../🧬️schema/📥️dependencies.json";
import {readBuildAuthoringCapabilitiesV1,createNativeBuildCallerCapabilitiesV1,issueBuildTaskEnvironmentV1} from "../../📥️caller/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import {buildGraphPathV1,buildGraphMembershipV1,buildGraphSourceFactsHoldV1,workspaceBuildFilesV1,loadBuildGraphV1,readBuildGraphV1,neutralBuildTargetV1,resolveBuildTargetV1} from "../../🕸️graph/🟦️.ts";
import {parseBuildInvocationV1,nxBuildArgumentsV1,resolveBuildInvocationV1,readBuildDependencyInvocationV1} from "../../🧭️invocation/🟦️.ts";
import {watchBuildInvocationV1,selectBuildTasksV1,selectBuildProjectsV1} from "../../🧭️selection/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,receiveScriptProcessInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020").default;
const nxOracleRequireV1=createRequire(resolve(process.cwd(),".nx/installation/package.json")),independentGlobV1=Object.assign((patterns:string|string[],options:Record<string,unknown>)=>nxOracleRequireV1("glob").glob(patterns,{...options,nodir:true}),{sync:(patterns:string|string[],options:Record<string,unknown>)=>nxOracleRequireV1("glob").globSync(patterns,{...options,nodir:true})});
const graph=()=>readBuildGraphV1(structuredClone(fixture.graph));
const withGraphTestInvocationV1=(body:(invocation:any)=>Promise<void>)=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"graph-fixture-publisher",maximumElapsedMilliseconds:20000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["show","projects"],"nx",process.env),Date.now()),body);

beforeAll(async()=>{const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"graph-init-"));try{await require("../../👁️observation/🟨️.mjs").observeInference({cachePath:join(root,"init.json"),key:"owned-test-initialization"},()=>null);}finally{await rm(root,{recursive:true,force:true});}});

test("portable graph fixture agrees with the independent JSON Schema implementation",()=>{
  const validate=new Ajv({strict:false}).compile(schema);
  expect(validate(fixture.graph)).toBe(true);
  expect(graph().projects.app.name).toBe("app");
  for(const value of [{...fixture.graph,version:2},{...fixture.graph,projects:{bad:{name:"bad",root:"bad",targets:{build:{}}}}}]){
    expect(validate(value)).toBe(false);expect(()=>readBuildGraphV1(value)).toThrow();
  }
});

test("portable configuration and Nx projection preserve command semantics",()=>{
  const target={executor:"repo:script",options:{command:"bun ./📜️script.ts build",cwd:"app",env:{BASE:"yes",OVERRIDE:"base"}},configurations:{release:{env:{MODE:"release",OVERRIDE:"release"}}},outputs:["dist"],dependsOn:["^build"]};
  const neutral=neutralBuildTargetV1(target),resolved=resolveBuildTargetV1(neutral,"release"),nx=projectBuildTargetForNxV1(neutral);
  expect(neutral.execution.kind).toBe("command");expect(neutral).not.toHaveProperty("executor");
  expect(resolved.execution.env).toEqual({BASE:"yes",OVERRIDE:"release",MODE:"release"});expect(resolved.outputs).toEqual(["dist"]);expect(resolved.dependsOn).toEqual(["^build"]);expect(nx).toEqual({...target,executor:"@semio-tech/repo-lib:command",inputs:["default","^default",...nxBuildEngineInputsV1()],configurations:{release:{...target.configurations.release,env:{BASE:"yes",OVERRIDE:"release",MODE:"release"}}}});
  const sdk=createRequire(resolve(process.cwd(),".nx/installation/package.json")),oracle=sdk("nx/src/utils/params").combineOptionsForExecutor({},"release",structuredClone(nx),sdk("nx/src/executors/run-commands/schema.json"),null,null);expect(oracle.env).toEqual(resolved.execution.env);
  expect(()=>resolveBuildTargetV1(neutral,"unknown")).toThrow();
});

test("portable CLI keeps backend selection separate from literal task arguments",()=>{
  expect(parseBuildInvocationV1(["--tool=bazel","run","app:build","--","--tool=nx"],{})).toEqual({tool:"bazel",args:["run","app:build","--","--tool=nx"]});
  expect(parseBuildInvocationV1(["show","projects"],{SEMIO_BUILD_TOOL:"bazel"}).tool).toBe("bazel");
  expect(()=>parseBuildInvocationV1(["--tool=unknown"],{})).toThrow();
  expect(nxBuildArgumentsV1(["run","app:build","--skip-cache","--exclude-dependencies","--output=stream","--","--skip-cache"])).toEqual(["run","app:build","--skip-nx-cache","--excludeTaskDependencies","--output-style=stream","--","--skip-cache"]);
  expect(resolveBuildInvocationV1(process.cwd(),["run","app:build:release"],{}).env.SEMIO_BUILD_MODE).toBe("release");
});

test("language-neutral task selections preserve configurations, tags and exclusions",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"build-graph-test",maximumElapsedMilliseconds:0},{},Date.now()),async invocation=>{
  for(const row of fixture.selectionCases){const result=await selectBuildTasksV1(process.cwd(),graph(),row.args,invocation);expect(result.tasks).toEqual(row.tasks);expect(result.args).toEqual(row.forwarded);}
  expect(selectBuildProjectsV1(graph(),["tag:kind:app"])).toEqual(["app"]);
  const skipped=await selectBuildTasksV1(process.cwd(),graph(),fixture.selectionCases[2].args,invocation);expect(skipped.skipCache).toBe(true);expect(skipped.excludeDependencies).toBe(true);
}));

test("portable models compile under the independent bundler without Nx runtime",async()=>{
  const result=await build({entryPoints:[fileURLToPath(new URL("../../🕸️graph/🟦️.ts",import.meta.url))],bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,logLevel:"silent"});
  expect(Object.keys(result.metafile!.inputs).some(path=>path.includes("node_modules/nx/"))).toBe(false);
});


test("portable native caller preserves exact original resources and literal task arguments",async()=>{
  const root=process.cwd(),capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","app:build"],"bazel",{SEMIO_BUILD_MODE:"release"}),validate=new (require("ajv").default)({strict:false}).compile(callerSchema);
  expect(validate(capabilities)).toBe(true);
  const envelope=createScriptProcessEnvelope({version:1,owner:"build-native-test",maximumElapsedMilliseconds:0},capabilities,Date.now());
  await withScriptProcessEnvelope(envelope,async invocation=>{
    const execution={kind:"command" as const,ownerArguments:true,nativeOwnerCommand:{kind:"native-owner-command",manifest:"app/Cargo.toml",workingDirectory:"app",program:"bun",arguments:["./📜️script.ts","build"]}};
    const environment=issueBuildTaskEnvironmentV1(root,invocation,execution,["literal space","$literal"],{SEMIO_BUILD_MODE:"release"}),issued=JSON.parse(environment.SEMIO_SCRIPT_PROCESS_INVOCATION!);
    expect(issued.policy).toEqual(envelope.policy);expect(issued.deadlineEpochMilliseconds).toBeNull();expect(issued.capabilities.command.arguments).toEqual(["./📜️script.ts","build","literal space","$literal"]);
    expect(issued.capabilities.transport).toEqual(capabilities.native.transport);expect(environment.SEMIO_BUILD_TOOL).toBe("bazel");expect(JSON.parse(environment.SEMIO_OWNER_ARGUMENTS!)).toEqual({version:1,arguments:["literal space","$literal"]});
    expect(invocation.capabilities).toBe(capabilities);
  });
});


test("portable source glob matching agrees with the installed Nx implementation",()=>{
  const {paths,patterns}=fixture.globCases;
  for(const pattern of patterns)expect(paths.map(path=>buildInputPathMatchesV1(path,pattern))).toEqual(require("nx/src/native").matchGlobPaths([pattern],paths));
});


test("portable selection refuses original cancellation before graph execution",async()=>{
  const controller=new AbortController();controller.abort(Error("cancelled-build"));
  const invocation={policy:{version:1 as const,owner:"cancelled-build",maximumElapsedMilliseconds:0},capabilities:{},control:{signal:controller.signal,remainingMilliseconds:()=>null,publish:()=>{},yieldContinuation:async()=>{}}};
  await expect(selectBuildTasksV1(process.cwd(),graph(),["run","app:build"],invocation)).rejects.toThrow("cancelled-build");
});


test("Nx projects exactly the common graph into its independent task graph implementation",()=>{
  const portable=graph(),projection=projectBuildGraphForNxV1(portable),nodes=Object.fromEntries(Object.entries(projection.projects).map(([name,data])=>[name,{name,type:"lib",data}])),dependencies=Object.fromEntries(Object.keys(nodes).map(name=>[name,portable.dependencies.filter(edge=>edge.source===name)]));
  const {createTaskGraph}=require("nx/src/tasks-runner/create-task-graph"),tasks=createTaskGraph({nodes,dependencies}, {}, ["app"],["test"],undefined,{});
  expect(Object.keys(projection.projects).sort()).toEqual(Object.keys(portable.projects).sort());
  expect(Object.keys(tasks.tasks).sort()).toEqual(["app:test","lib:build"]);
  expect(tasks.dependencies["app:test"]).toEqual(["lib:build"]);
  expect(projection.projects.app.targets.build.executor).toBe("@semio-tech/repo-lib:command");
});

test("policy facets retain six physical owners and ordinary lint scheduling in both backends",async()=>{
  const owners=Object.fromEntries(fixture.policyOwnership.map((row,index)=>[row.name,{...row,targets:index%2?{}:{lint:{executor:"repo:script",options:{command:"bun --version"},dependsOn:["prepare"]},prepare:{executor:"repo:noop"}}}])),files=fixture.policyOwnership.map(row=>join(row.root,"📜️script.ts")),policy=require("../../🕸️graph/📥️inference/🛡️policy/🟨️.mjs").default;
  const contributions=await policy.createNodesV2[1](files,{}, {workspaceRoot:process.cwd(),projects:owners}),projects=structuredClone(owners);
  for(const[,result]of contributions)for(const[name,project]of Object.entries(result.projects)as[string,any][])projects[name]={...projects[name],...project,targets:{...projects[name]?.targets,...project.targets}};
  expect(Object.keys(projects).sort()).toEqual(fixture.policyOwnership.map(row=>row.name).sort());
  const edges=fixture.policyOwnership.map((row,index)=>({source:row.name,target:fixture.policyOwnership[(index+1)%fixture.policyOwnership.length].name,type:"implicit"})),portable=readBuildGraphV1({version:1,projects:Object.fromEntries(Object.entries(projects).map(([name,project]:[string,any])=>[name,{...project,targets:Object.fromEntries(Object.entries(project.targets).map(([name,target])=>[name,neutralBuildTargetV1(target as any)]))}])),dependencies:edges}),projection=projectBuildGraphForNxV1(portable),errors:unknown[]=[],merged=require("nx/src/project-graph/utils/project-configuration-utils").mergeCreateNodesResults([[["owned-policy","📋️project.json",projection,0]]],[],{},process.cwd(),errors);
  expect(new Ajv({strict:true}).compile(schema)(portable)).toBe(true);
  expect(errors).toEqual([]);expect(Object.values(merged.projectRootMap).map((project:any)=>project.name).sort()).toEqual(Object.keys(portable.projects).sort());
  const {planBazelTasksV1}=await import("../../🔧️tools/🏗️bazel/📜️script.ts"),nodes=Object.fromEntries(Object.entries(projection.projects).map(([name,data])=>[name,{name,type:"lib",data}])),dependencies=Object.fromEntries(Object.keys(nodes).map(name=>[name,[]]));
  const builder=new(require("nx/src/project-graph/project-graph-builder").ProjectGraphBuilder)({nodes,dependencies,externalNodes:{}});for(const edge of portable.dependencies)builder.addDependency(edge.source,edge.target,edge.type);expect(Object.values(builder.getUpdatedProjectGraph().dependencies).flat()).toEqual(edges);
  for(const[rowIndex,row]of fixture.policyOwnership.entries()){
    const target=portable.projects[row.name].targets.lint,plan=planBazelTasksV1(portable,{tasks:[row.name+":lint"]}),oracle=require("nx/src/tasks-runner/create-task-graph").createTaskGraph({nodes,dependencies},{},[row.name],["lint"],undefined,{});
    expect(Object.keys(plan.tasks).sort()).toEqual(Object.keys(oracle.tasks).sort());expect(plan.tasks[row.name+":lint"].dependencies.sort()).toEqual(oracle.dependencies[row.name+":lint"].sort());expect(target.dependsOn).toContain("lint-policy");expect(portable.projects[row.name].targets["lint-policy"].execution.command).toContain(files[rowIndex]);
    if(rowIndex%2===0){expect(target.execution.command).toBe("bun --version");expect(target.dependsOn).toContain("prepare");}
  }
  expect(()=>policy.createNodesV2[1]([files[0]],{}, {workspaceRoot:process.cwd(),projects:{[fixture.policyOwnership[0].name]:{...owners[fixture.policyOwnership[0].name],targets:{"lint-policy":{executor:"repo:noop"}}}}})).toThrow("Policy target already declared");
  const standalone=policy.createNodesV2[1]([files[0]],{}, {workspaceRoot:process.cwd(),projects:{}})[0][1].projects,facet=Object.values(standalone)[0]as any;expect(facet.name.startsWith("breach-")).toBe(true);expect(facet.targets.lint).toEqual({executor:"repo:noop",dependsOn:["lint-policy"]});
});

test("canonical graph refuses competing identities at one physical root",()=>{
  const duplicate=structuredClone(fixture.graph);duplicate.projects.alias={...duplicate.projects.app,name:"alias"};
  expect(new Ajv({strict:true}).compile({type:"array",items:{type:"string"},uniqueItems:true})(Object.values(duplicate.projects).map(project=>project.root))).toBe(false);
  expect(()=>readBuildGraphV1(duplicate)).toThrow("Duplicate build project root");
});

test("private Nx source bridge binds complete literals to immutable bytes and original control",async()=>{
 const {executeLiteralBuildArgumentsV1}=await import("../../../🔌️nx-plugin/📤️arguments/🟨️.mjs"),{runNxCommandSourceV1}=await import("../../../🔌️nx-plugin/📤️arguments/📜️script.ts"),{createHash}=await import("node:crypto"),row=fixture.nxCommandBridge,previous=process.env.SEMIO_SCRIPT_PROCESS_INVOCATION,wire=JSON.stringify(createScriptProcessEnvelope({version:1,owner:"nx-source-test",maximumElapsedMilliseconds:0},{exact:"original"},Date.now())),context={root:process.cwd(),projectName:"app",targetName:"probe"},sourceSchema=JSON.parse(await readFile(new URL("../../../🔌️nx-plugin/📤️arguments/🧬️schema/📥️source.json",import.meta.url),"utf8")),admit=new(require("ajv").default)({strict:true}).compile(sourceSchema);
 process.env.SEMIO_SCRIPT_PROCESS_INVOCATION=wire;
 try{
  const plan=await executeLiteralBuildArgumentsV1({command:row.command,args:row.args,__unparsed__:row.forwarded},context,async(plan:any)=>plan),digest=plan.command.split(" ").at(-1),path=join(plan.env.SEMIO_NX_COMMAND_DIRECTORY,digest+".json"),image=await readFile(path,"utf8"),source=JSON.parse(image),environment={...plan.env};
  expect(admit(source)).toBe(true);expect(new Bun.CryptoHasher("sha256").update(image).digest("hex")).toBe(digest);expect(source.invocation).toBe(new Bun.CryptoHasher("sha256").update(wire).digest("hex"));expect(plan.args).toBeUndefined();expect(plan.__unparsed__).toEqual([]);expect(plan.forwardAllArgs).toBe(false);expect(plan.command.length).toBeLessThan(128);expect(plan.env.SEMIO_SCRIPT_PROCESS_INVOCATION).toBe(wire);
  await expect(runNxCommandSourceV1(["../bad"],environment)).rejects.toThrow("Private Nx command source required");
  await writeFile(path,image+" ");await expect(runNxCommandSourceV1([digest],environment)).rejects.toThrow("source digest differs");await writeFile(path,image);
  await expect(runNxCommandSourceV1([digest],{...environment,SEMIO_SCRIPT_PROCESS_INVOCATION:JSON.stringify({...JSON.parse(wire),capabilities:{foreign:true}})})).rejects.toThrow("invocation differs");
  const invalid={...source,version:2},invalidImage=JSON.stringify(invalid),invalidDigest=createHash("sha256").update(invalidImage).digest("hex");await writeFile(join(plan.env.SEMIO_NX_COMMAND_DIRECTORY,invalidDigest+".json"),invalidImage);expect(admit(invalid)).toBe(false);await expect(runNxCommandSourceV1([invalidDigest],environment)).rejects.toThrow("source refused");
  const expired={...JSON.parse(wire),policy:{version:1,owner:"nx-source-expired",maximumElapsedMilliseconds:1000},deadlineEpochMilliseconds:Date.now()-1};process.env.SEMIO_SCRIPT_PROCESS_INVOCATION=JSON.stringify(expired);const expiredPlan=await executeLiteralBuildArgumentsV1({command:"echo refused"},context,async(plan:any)=>plan);await expect(runNxCommandSourceV1([expiredPlan.command.split(" ").at(-1)],expiredPlan.env)).rejects.toThrow("deadline refused");
  process.env.SEMIO_SCRIPT_PROCESS_INVOCATION=wire;const list=await executeLiteralBuildArgumentsV1({commands:[{command:"first",forwardAllArgs:false},{command:"second",forwardAllArgs:true}],args:row.args,__unparsed__:row.forwarded,parallel:false},context,async(plan:any)=>plan);const sources=[];for(const command of list.commands)sources.push(JSON.parse(await readFile(join(list.env.SEMIO_NX_COMMAND_DIRECTORY,command.command.split(" ").at(-1)+".json"),"utf8")).command);expect(sources[0]).toBe("first");expect(sources[1]).toContain("'two words' '' 'literal$()'");expect(list.parallel).toBe(false);expect(list.commands.every((command:any)=>command.forwardAllArgs===false)).toBe(true);
  const controlWire=JSON.stringify(createScriptProcessEnvelope({version:1,owner:"nx-source-control",maximumElapsedMilliseconds:2000},{exact:"original"},Date.now())),marker=join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"nx-source-control.json");process.env.SEMIO_SCRIPT_PROCESS_INVOCATION=controlWire;const code='require("fs").writeFileSync(process.argv.at(-1),process.env.SEMIO_SCRIPT_PROCESS_INVOCATION);setTimeout(()=>{},30000)',controlled=await executeLiteralBuildArgumentsV1({command:"bun -e "+JSON.stringify(code),args:[marker]},context,async(plan:any)=>plan),started=performance.now();await expect(runNxCommandSourceV1([controlled.command.split(" ").at(-1)],controlled.env)).rejects.toThrow();expect(performance.now()-started).toBeLessThan(5000);expect(await readFile(marker,"utf8")).toBe(controlWire);
  console.log("[DEBUG] Nx source bridge rejects corrupt/malformed/foreign/expired authority; actual child keeps original capabilities and is cancelled by its unchanged deadline");
 }finally{if(previous===undefined)delete process.env.SEMIO_SCRIPT_PROCESS_INVOCATION;else process.env.SEMIO_SCRIPT_PROCESS_INVOCATION=previous;}
});


test("isolated bootstrap acquires absent node_modules before inference and refreshes changed locks",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"isolated-build-bootstrap",maximumElapsedMilliseconds:0},{},Date.now()),async invocation=>{
  const artifacts=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??join(process.cwd(),".🧬semio/🦑️repo/⚡️cache/tests/build-bootstrap"));await mkdir(artifacts,{recursive:true});const root=await mkdtemp(join(artifacts,"environment-")),events:string[]=[];
  try{
    await writeFile(join(root,"package.json"),JSON.stringify(fixture.bootstrap.manifest));await writeFile(join(root,"bun.lock"),fixture.bootstrap.lock);
    await expect(access(join(root,"node_modules"))).rejects.toThrow();
    const install=async(directory:string,original:typeof invocation)=>{expect(original).toBe(invocation);events.push("install");const modules=join(directory,".cache/b/d/m/node_modules");await mkdir(modules,{recursive:true});if(!await access(join(directory,"node_modules")).then(()=>true,()=>false))await symlink(modules,join(directory,"node_modules"),process.platform==="win32"?"junction":"dir");for(const dependency of fixture.bootstrap.dependencies){const path=join(modules,dependency);await mkdir(path,{recursive:true});await writeFile(join(path,"package.json"),JSON.stringify({name:dependency,main:"./index.js",version:fixture.bootstrap.manifest.dependencies[dependency as keyof typeof fixture.bootstrap.manifest.dependencies]}));await writeFile(join(path,"index.js"),"module.exports={};");}};
    const sources=()=>{events.push("sources");};
    await ensureBuildEnvironmentV1(root,invocation,install,sources);await ensureBuildEnvironmentV1(root,invocation,install,sources);
    expect(events).toEqual(["sources","install","sources","install"]);
    await writeFile(join(root,"bun.lock"),fixture.bootstrap.changedLock);await ensureBuildEnvironmentV1(root,invocation,install,sources);
    expect(events.filter(event=>event==="install").length).toBe(3);
    await rm(join(root,"node_modules/typescript"),{recursive:true,force:true});await ensureBuildEnvironmentV1(root,invocation,install,sources);
    expect(events.filter(event=>event==="install").length).toBe(fixture.bootstrap.expectedInstalls);
  }finally{await rm(root,{recursive:true,force:true});}
}));


test("nested consumers reuse the exact pinned authority and reject a changed publication",async()=>{
  const directory=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"pin-")),previous={path:process.env.SEMIO_BUILD_GRAPH_PATH,digest:process.env.SEMIO_BUILD_GRAPH_DIGEST};
  try{
    const portable=graph();await withGraphTestInvocationV1(original=>withRepositoryBuildGraphArtifactV1(process.cwd(),portable,original,async pin=>{process.env.SEMIO_BUILD_GRAPH_PATH=pin.path;process.env.SEMIO_BUILD_GRAPH_DIGEST=pin.digest;
    expect(await loadBuildGraphV1(directory,undefined,{reuse:true})).toEqual(portable);
    const digest=new Bun.CryptoHasher("sha256").update(await readFile(pin.path)).digest("hex");expect(digest).toBe(pin.digest);
    const changed=join(directory,"changed.json");await writeFile(changed,JSON.stringify({...portable,version:2}));process.env.SEMIO_BUILD_GRAPH_PATH=changed;await expect(loadBuildGraphV1(directory,undefined,{reuse:true})).rejects.toThrow("Pinned build graph digest");}));
  }finally{if(previous.path===undefined)delete process.env.SEMIO_BUILD_GRAPH_PATH;else process.env.SEMIO_BUILD_GRAPH_PATH=previous.path;if(previous.digest===undefined)delete process.env.SEMIO_BUILD_GRAPH_DIGEST;else process.env.SEMIO_BUILD_GRAPH_DIGEST=previous.digest;await rm(directory,{recursive:true,force:true});}
});

test("source watch filters owned projects, debounces events and returns on original cancellation",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"build-watch-test",maximumElapsedMilliseconds:10000},{},Date.now()),async original=>{
  const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"watch-")),controller=new AbortController(),invocation={...original,control:{...original.control,signal:AbortSignal.any([original.control.signal,controller.signal])}},receipt=join(root,".🧬semio","watch.json");
  await mkdir(join(root,graph().projects.app.root),{recursive:true});await mkdir(join(root,graph().projects.lib.root),{recursive:true});await mkdir(join(root,".🧬semio"));
  const command=[process.execPath,"-e",'require("node:fs").writeFileSync(process.argv[1],JSON.stringify({files:JSON.parse(process.env.SEMIO_BUILD_CHANGED_FILES),projects:JSON.parse(process.env.SEMIO_BUILD_CHANGED_PROJECTS)}))',receipt],watching=watchBuildInvocationV1(root,graph(),["watch","--projects="+fixture.watch.selected,"--",...command],invocation);
  try{
    await writeFile(join(root,fixture.watch.excluded),"unselected");await Bun.sleep(300);await expect(access(receipt)).rejects.toThrow();
    await writeFile(join(root,fixture.watch.included),"selected");
    for(let attempt=0;attempt<100;attempt++){try{await access(receipt);break;}catch{await Bun.sleep(20);}}
    const result=JSON.parse(await readFile(receipt,"utf8"));expect(result.projects).toEqual(fixture.watch.expectedChangedProjects);expect(result.files).toEqual([fixture.watch.included]);
    console.log("[DEBUG] portable watcher actualChild=true projectFilter=true debounce=true cancellation=true");
  }finally{controller.abort(Error("watch-test-complete"));await watching;await rm(root,{recursive:true,force:true});}
}));

test("a pinned Nx publication is the only plugin authority",()=>{
  const path=require.resolve("nx/src/project-graph/plugins/get-plugins"),source=require("node:fs").readFileSync(path,"utf8"),start=source.indexOf("function getDefaultPlugins(root)"),end=source.indexOf("\n}",start)+2,owned=new Function("process",source.slice(start,end)+";return getDefaultPlugins('.')");
  expect(owned({env:{SEMIO_BUILD_GRAPH_PATH:"original-authority.json"}})).toEqual([]);
});

test("private Nx nodes and dependencies require exact immutable publication pins",async()=>{
 const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"pin-")),path=join(root,"original.json"),source=JSON.stringify(fixture.graph),digest=createHash("sha256").update(source).digest("hex"),plugin=await import("../../../🟨️.mjs"),previous={SEMIO_BUILD_GRAPH_PATH:process.env.SEMIO_BUILD_GRAPH_PATH,SEMIO_BUILD_GRAPH_DIGEST:process.env.SEMIO_BUILD_GRAPH_DIGEST};
 try{
  await writeFile(path,source);const context={workspaceRoot:root};
  for(const row of fixture.pinnedPublication.invalid){
   const value=row as {path?:string;digest?:string},environment={SEMIO_BUILD_GRAPH_PATH:value.path==="absolute"?path:value.path==="relative"?"original.json":undefined,SEMIO_BUILD_GRAPH_DIGEST:value.digest==="matching"?digest:value.digest==="different"?"0".repeat(64):value.digest};
   for(const[key,value]of Object.entries(environment))if(value===undefined)delete process.env[key];else process.env[key]=value;
   expect(()=>plugin.createDependencies({},context)).toThrow();await expect(plugin.default.createNodesV2[1](["📋️project.json"],{},context)).rejects.toThrow();await expect(loadBuildGraphV1(root,undefined,{reuse:true})).rejects.toThrow();
  }
  process.env.SEMIO_BUILD_GRAPH_PATH=path;process.env.SEMIO_BUILD_GRAPH_DIGEST=digest;
  const admitted=new Ajv({strict:true}).compile(JSON.parse(await readFile(new URL("../../🧬️schema/📥️pin.json",import.meta.url),"utf8")));expect(admitted({path,digest})).toBe(true);for(const value of [{path},{digest},{path,digest:"malformed"},{path,digest,extra:true}])expect(admitted(value)).toBe(false);expect(new Bun.CryptoHasher("sha256").update(source).digest("hex")).toBe(digest);
  expect(plugin.createDependencies({},context)).toEqual(fixture.graph.dependencies);expect(await loadBuildGraphV1(root,undefined,{reuse:true})).toEqual(fixture.graph);const nodes=await plugin.default.createNodesV2[1](["📋️project.json"],{},context);expect(Object.keys(nodes[0][1].projects)).toEqual(Object.keys(fixture.graph.projects));expect(createHash("sha256").update(await readFile(path)).digest("hex")).toBe(digest);
 }finally{for(const[key,value]of Object.entries(previous))if(value===undefined)delete process.env[key];else process.env[key]=value;await rm(root,{recursive:true,force:true});}
});

test("test owner inference names only the current declared fact namespace without storage admission",async()=>{
 const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"ti-")),plugin=await import("../../../../🧪️test/🟨️.mjs"),declaration=JSON.parse(await readFile(new URL("../../📥️inputs/💾️storage/🧬️schema/🔣️.json",import.meta.url),"utf8")),admitted=new Ajv({strict:true}).compile(declaration);
 try{expect(admitted(fixture.testInferenceLocation.namespace)).toBe(true);expect(plugin.testBuildInferenceDirectoryV1(root,{})).toBe(join(root,fixture.testInferenceLocation.relative));expect(plugin.testBuildInferenceDirectoryV1(root,{SEMIO_BUILD_INFERENCE_DIRECTORY:join(root,fixture.testInferenceLocation.override)})).toBe(join(root,fixture.testInferenceLocation.relative));expect(await access(join(root,".cache")).then(()=>true,()=>false)).toBe(false);}finally{await rm(root,{recursive:true,force:true});}
});

test("loaded graph implementation and publication source changes refuse current authority",async()=>{
 const {requireBuildGraphModuleAuthorityV1,buildGraphProducerAuthorityV1}=await import("../../🕸️graph/🟦️.ts"),row=fixture.graphModuleAuthority,hash=(value:string)=>createHash("sha256").update(value).digest("hex"),authority=(source:{implementation:string;publication:string})=>({implementation:hash(source.implementation),publication:hash(source.publication)}),loaded=authority(row),schema=JSON.parse(await readFile(new URL("../../🧬️schema/📥️authority.json",import.meta.url),"utf8")),validator=new Ajv({strict:true}).compile(schema.properties.module),admit=new Ajv({strict:false}).compile(schema),receipt={version:3,module:loaded,graph:"a".repeat(64),membership:"b".repeat(64),observations:[]};
 expect(validator(loaded)).toBe(true);expect(loaded.implementation).toBe(new Bun.CryptoHasher("sha256").update(row.implementation).digest("hex"));expect(loaded.publication).toBe(new Bun.CryptoHasher("sha256").update(row.publication).digest("hex"));expect(()=>requireBuildGraphModuleAuthorityV1(loaded,authority(row))).not.toThrow();
 for(const change of row.changes){const current=authority({...row,...change});expect(validator(current)).toBe(true);expect(()=>requireBuildGraphModuleAuthorityV1(loaded,current)).toThrow("Loaded build graph source changed");}
 for(const invalid of [{},{...loaded,implementation:"invalid"},{...loaded,unexpected:true}]){expect(validator(invalid)).toBe(false);expect(()=>requireBuildGraphModuleAuthorityV1(loaded,invalid as any)).toThrow();}
 expect(admit(receipt)).toBe(true);expect(buildGraphProducerAuthorityV1(receipt,loaded)).toBe(true);const {module,...unbound}=receipt;for(const invalid of [{...unbound,version:2},unbound,{...receipt,module:{...loaded,implementation:"0".repeat(64)}},{...receipt,module:{...loaded,publication:"0".repeat(64)}}])expect(buildGraphProducerAuthorityV1(invalid,loaded)).toBe(false);expect(admit({...unbound,version:2})).toBe(false);expect(admit(unbound)).toBe(false);
});


test("each pinned publication receives a distinct Nx cache and never forces an older graph",async()=>{
  const {nxFiniteEnvironment,nxBuildEnvironmentV1}=await import("../../../⚡️caching/🚀️bootstrap/📜️script.ts"),tool="a".repeat(64),root=process.cwd(),base={SEMIO_BUILD_REUSE_GRAPH:"true",NX_WORKSPACE_DATA_DIRECTORY:"/owned/cache"},first=nxFiniteEnvironment(root,{...base,SEMIO_BUILD_GRAPH_DIGEST:"b".repeat(64)},tool),second=nxFiniteEnvironment(root,{...base,SEMIO_BUILD_GRAPH_DIGEST:"c".repeat(64)},tool);
  expect(first.NX_WORKSPACE_DATA_DIRECTORY).not.toBe(second.NX_WORKSPACE_DATA_DIRECTORY);expect(first.NX_SOCKET_DIR).not.toBe(second.NX_SOCKET_DIR);expect(nxBuildEnvironmentV1(root,first).NX_FORCE_REUSE_CACHED_GRAPH).toBe("false");
});

test("Cargo edges resolve inherited paths against their actual nested workspace",async()=>{
  const directory=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"cargo-"));
  try{
    for(const[path,source]of Object.entries(fixture.cargoWorkspace.files)){const file=join(directory,path);await mkdir(dirname(file),{recursive:true});await writeFile(file,source);}
    const owner=require("../../🕸️graph/📥️inference/🟨️.mjs");await owner.libraryBootstrap;
    const projects={owner:{name:"owner",root:"nested/owner",targets:{}},dependency:{name:"dependency",root:"nested/dependency",targets:{}}},fileMap={projectFileMap:{owner:[{file:"nested/owner/Cargo.toml"}],dependency:[{file:"nested/dependency/Cargo.toml"}]}};
    const oracle=require("@iarna/toml").parse(fixture.cargoWorkspace.files["nested/Cargo.toml"]);expect(resolve(directory,"nested",oracle.workspace.dependencies.local.path)).toBe(join(directory,projects.dependency.root));
    expect(await owner.cacheInternals.createDependenciesImplementation({}, {workspaceRoot:directory,projects,fileMap})).toEqual(fixture.cargoWorkspace.expected);
    const names=new Map(Object.entries(projects).map(([name,project])=>[project.root,name])),before=owner.cacheInternals.projectAssemblyAuthorityV1(directory,projects.owner.root,names);
    names.set("unrelated/new-crate","unrelated");expect(owner.cacheInternals.projectAssemblyAuthorityV1(directory,projects.owner.root,names)).toBe(before);
    names.set(projects.dependency.root,"renamed-dependency");expect(owner.cacheInternals.projectAssemblyAuthorityV1(directory,projects.owner.root,names)).not.toBe(before);
  }finally{await rm(directory,{recursive:true,force:true});}
});


test("portable source discovery excludes generated dependency sources before graph admission",async()=>{
  const artifacts=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??join(process.cwd(),".🧬semio/🦑️repo/⚡️cache/tests/build-membership"));await mkdir(artifacts,{recursive:true});const root=await mkdtemp(join(artifacts,"membership-"));
  try{
    for(const[path,source]of Object.entries(fixture.membership.files)){const destination=join(root,path);await mkdir(dirname(destination),{recursive:true});await writeFile(destination,source);}
    const original=Object.keys(fixture.membership.files).sort(),excluded:boolean[]=require("nx/src/native").matchGlobPaths(fixture.membership.excludedPatterns,original),independent=original.filter((_,index)=>!excluded[index]);
    expect(independent).toEqual(fixture.membership.expected);
    expect(await workspaceBuildFilesV1(root)).toEqual(independent);
  }finally{await rm(root,{recursive:true,force:true});}
});


test("explicit editor grants retain their original native resource bounds before the CLI envelope",async()=>{
  const {createBuildBootstrapCapabilitiesV1}=await import("../../📜️script.ts"),root=process.cwd(),args=["run","app:build"],declared=createNativeBuildCallerCapabilitiesV1(root,args,"nx",{SEMIO_BUILD_MODE:"release"}),{tool,...authored}=declared;
  const custom={...authored,native:{...authored.native,transport:{maximumBytes:12345,maximumLines:123},child:{maximumElapsedMilliseconds:123456}}};
  const admitted=readBuildAuthoringCapabilitiesV1(root,args,"nx",custom),schema=(await import("../../🧬️schema/📥️authoring.json")).default;
  expect(new (require("ajv").default)({strict:false}).compile(schema)(custom)).toBe(true);expect(admitted.native).toEqual(custom.native);expect(admitted.tool).toBe("nx");
  const actual=await createBuildBootstrapCapabilitiesV1(root,["repo",...args],{SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(custom)});expect(actual).toEqual(admitted);
  expect(()=>readBuildAuthoringCapabilitiesV1(root,["run","other:build"],"nx",custom)).toThrow("authored build request");
});

test("relative initial declarations preserve exact GUI resources through original orchestration and native task issuance",async()=>{
 const {createBuildBootstrapCapabilitiesV1,runOwnedBuildInvocationV1}=await import("../../📜️script.ts"),authoringSchema=(await import("../../🧬️schema/📥️authoring.json")).default,validateAuthoring=new(require("ajv").default)({strict:true}).compile(authoringSchema),validateCaller=new(require("ajv").default)({strict:true}).compile(callerSchema),root=process.cwd();
 expect(require("@iarna/toml").parse(await readFile(join(root,".cargo/config.toml"),"utf8")).build["target-dir"]).toBe(fixture.initialAuthoring.cases[0]!.native.artifactDirectory);
 for(const row of fixture.initialAuthoring.cases){
  const args=[...row.arguments,"--help"],request=["repo",...args],authored={workspaceRoot:row.workspaceRoot,tool:row.tool,arguments:request,selection:request,launches:[request],buildMode:row.buildMode,native:row.native};
  expect(validateAuthoring(authored),row.name).toBe(true);
  const admitted=readBuildAuthoringCapabilitiesV1(root,args,"nx",authored),initial=await createBuildBootstrapCapabilitiesV1(root,request,{SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(authored)});
  expect(initial).toEqual(admitted);expect(validateCaller(initial),row.name).toBe(true);expect(admitted.workspaceRoot).toBe(root);expect(admitted.native).toEqual({...row.native,artifactDirectory:resolve(root,row.native.artifactDirectory)});
  const refusal=fixture.initialAuthoring.missingLaunches,missing=JSON.parse(JSON.stringify(authored));delete missing[refusal.omit];expect(validateAuthoring(missing)).toBe(refusal.valid);expect(()=>readBuildAuthoringCapabilitiesV1(root,args,"nx",missing)).toThrow(refusal.error);
  const missingCaller=JSON.parse(JSON.stringify(admitted));delete missingCaller[refusal.omit];expect(validateCaller(missingCaller)).toBe(refusal.valid);
  expect(()=>readBuildAuthoringCapabilitiesV1(root,args,"nx",{...authored,workspaceRoot:".."})).toThrow("authored build request");
  const envelope=createScriptProcessEnvelope({...fixture.initialAuthoring.parent,version:1},initial,Date.now());
  await withScriptProcessEnvelope(envelope,async invocation=>{
   let handed:any;
   const output=spyOn(console,"log").mockImplementation(()=>{const wire=process.env.SEMIO_SCRIPT_PROCESS_INVOCATION;if(wire)handed=JSON.parse(wire);});
   try{await runOwnedBuildInvocationV1(root,args,invocation);}finally{output.mockRestore();}
   expect(handed.capabilities).toEqual(admitted);expect(handed.policy).toEqual(envelope.policy);expect(handed.deadlineEpochMilliseconds).toBeLessThanOrEqual(envelope.deadlineEpochMilliseconds!);expect(handed.deadlineEpochMilliseconds).toBeGreaterThan(Date.now());
   const environment=issueBuildTaskEnvironmentV1(root,{...invocation,capabilities:handed.capabilities},{...fixture.initialAuthoring.task,kind:"command"},fixture.initialAuthoring.taskArguments,{}),native=JSON.parse(environment.SEMIO_SCRIPT_PROCESS_INVOCATION!);
   expect(native.policy).toEqual(envelope.policy);expect(native.deadlineEpochMilliseconds).toBeLessThanOrEqual(envelope.deadlineEpochMilliseconds!);expect(native.deadlineEpochMilliseconds).toBeGreaterThan(Date.now());expect(native.capabilities.child).toEqual(row.native.child);expect(native.capabilities.transport).toEqual(row.native.transport);expect(native.capabilities.network).toEqual(row.native.network);expect(native.capabilities.buildMode).toBe(row.buildMode);expect(native.capabilities.artifactDirectory).toBe(admitted.native.artifactDirectory);expect(native.capabilities.repositoryRoot).toBe(root);expect(native.capabilities.command.arguments).toEqual([...fixture.initialAuthoring.task.nativeOwnerCommand.arguments,...fixture.initialAuthoring.taskArguments]);
  });
 }
 console.log("[DEBUG] relative initial GUI authority preserved exact declared native resources through original handoff");
});

test("portable relative GUI declarations survive Windows and POSIX JSON path serialization",()=>{
 const validate=new(require("ajv").default)({strict:true}).compile(initialAuthoringSchema);
 for(const row of fixture.initialAuthoring.paths){
  const path=row.platform==="win32"?win32:posix,source=fixture.initialAuthoring.cases[0]!,request=["repo",...source.arguments],declared={workspaceRoot:source.workspaceRoot,tool:source.tool,arguments:request,selection:request,launches:[request],buildMode:source.buildMode,native:source.native},serialized=JSON.stringify({SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(declared),CARGO_TARGET_DIR:row.artifact}),environment=JSON.parse(serialized),parsed=JSON.parse(environment.SEMIO_SCRIPT_CAPABILITIES);
  expect(validate(parsed)).toBe(true);
  expect(parsed.workspaceRoot).toBe(".");expect(path.resolve(row.root,parsed.workspaceRoot)).toBe(row.root);expect(path.resolve(row.root,parsed.native.artifactDirectory)).toBe(row.artifact);expect(environment.CARGO_TARGET_DIR).toBe(row.artifact);expect(environment.SEMIO_SCRIPT_CAPABILITIES).not.toContain("${workspaceFolder}");
 }
});

test("the three registered GUI rows match the portable initial authority corpus",()=>{
 const validate=new(require("ajv").default)({strict:true}).compile(initialAuthoringSchema),parser=require("typescript");
 for(const file of [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){
  const result=parser.parseConfigFileTextToJson(file,readFileSync(resolve(process.cwd(),file),"utf8"));expect(result.error).toBeUndefined();
  for(const row of fixture.initialAuthoring.cases){
   const actual=result.config.configurations.find((entry:any)=>entry.name===row.launchName),request=["repo",...row.arguments];expect(actual).toBeDefined();
   const declared=JSON.parse(actual.env.SEMIO_SCRIPT_CAPABILITIES);
   expect(validate(declared),row.name).toBe(true);expect(declared).toEqual({workspaceRoot:row.workspaceRoot,tool:row.tool,arguments:request,selection:request,launches:[request],buildMode:row.buildMode,native:row.native});expect(actual.env.CARGO_TARGET_DIR).toBe("${workspaceFolder}/"+row.native.artifactDirectory);expect(actual.env.SEMIO_SCRIPT_CAPABILITIES).not.toContain("${workspaceFolder}");
  }
 }
});

test("the defining graph fact module exposes its declared writer port under original custody",async()=>{
 const source=new URL("../../📥️inputs/💾️storage/🟨️.mjs",import.meta.url),storage=await import(source.href),parser=require("typescript"),tree=parser.createSourceFile(fileURLToPath(source),readFileSync(source,"utf8"),parser.ScriptTarget.Latest,true),exports=tree.statements.filter((node:any)=>parser.isFunctionDeclaration(node)&&node.modifiers?.some((modifier:any)=>modifier.kind===parser.SyntaxKind.ExportKeyword)).map((node:any)=>node.name.text);
 expect(exports).toEqual(fixture.initialAuthoring.storagePorts);
 for(const name of fixture.initialAuthoring.storagePorts)expect(typeof storage[name],name).toBe("function");
 expect(()=>storage.repositoryBuildGraphWriterDirectoryV1(process.cwd(),"inference")).toThrow("Original bounded graph writer custody required");
});


test("observed authority ignores unread body edits and identical rewrites while retaining consumed bytes and physical membership",async()=>{
  const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"facts-")),observation=require("../../👁️observation/🟨️.mjs");
  try{
    for(const[path,source]of Object.entries(fixture.authority.files))await writeFile(join(root,path),source);
    const captured=await observation.captureObservedSourceInputsV1(async()=>observation.observedFs.readFileSync(join(root,fixture.authority.consumed),"utf8")),facts=captured.observations.filter((row:any)=>row.path.startsWith(root)),before=await buildGraphMembershipV1(root,await workspaceBuildFilesV1(root));
    const contract={version:3,module:{implementation:"a".repeat(64),publication:"b".repeat(64)},membership:before,graph:"a".repeat(64),observations:facts};expect(new Ajv({strict:false}).compile((await import("../../🧬️schema/📥️authority.json")).default)(contract)).toBe(true);
    for(const row of fixture.authority.cases){if(row.kind==="remove")await rm(join(root,row.path));else await writeFile(join(root,row.path),row.content!);expect(await buildGraphMembershipV1(root,await workspaceBuildFilesV1(root))===before,row.name).toBe(row.membership);expect(await buildGraphSourceFactsHoldV1(facts),row.name).toBe(row.facts);}
  }finally{await rm(root,{recursive:true,force:true});}
});


test("failed reads retain precise facts for transient retry while stable malformed sources remain stable",async()=>{
  const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"failed-facts-")),observation=require("../../👁️observation/🟨️.mjs"),source=join(root,fixture.authority.consumed);let facts:unknown[]=[];
  try{
    await writeFile(source,"{");await expect(observation.captureObservedSourceInputsV1(()=>JSON.parse(observation.observedFs.readFileSync(source,"utf8")),(rows:any[])=>{facts=rows.filter(row=>row.path===source);})).rejects.toThrow();
    expect(facts.length).toBe(1);expect(await buildGraphSourceFactsHoldV1(facts)).toBe(true);
    await writeFile(source,fixture.authority.files[fixture.authority.consumed as keyof typeof fixture.authority.files]);expect(await buildGraphSourceFactsHoldV1(facts)).toBe(false);
  }finally{await rm(root,{recursive:true,force:true});}
});

test("public pinned graph reuse preserves ordinary nested metadata and execution arrays",async()=>{
 const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"shape-")),previous={path:process.env.SEMIO_BUILD_GRAPH_PATH,digest:process.env.SEMIO_BUILD_GRAPH_DIGEST},portable=graph(),patch=require("fast-json-patch");
 try{expect(patch.applyPatch(portable,fixture.compactExpansion.ordinaryPatch,true).newDocument).toBe(portable);await withGraphTestInvocationV1(original=>withRepositoryBuildGraphArtifactV1(process.cwd(),portable,original,async pin=>{process.env.SEMIO_BUILD_GRAPH_PATH=pin.path;process.env.SEMIO_BUILD_GRAPH_DIGEST=pin.digest;const result=await loadBuildGraphV1(root,undefined,{reuse:true});expect(new Ajv({strict:false}).compile(schema)(result)).toBe(true);expect(result).toEqual(portable);expect(new Bun.CryptoHasher("sha256").update(JSON.stringify(result)).digest("hex")).toBe(pin.digest);console.log("[DEBUG] pinned ordinary graph nested arrays retain exact JSON digest");}));}finally{if(previous.path===undefined)delete process.env.SEMIO_BUILD_GRAPH_PATH;else process.env.SEMIO_BUILD_GRAPH_PATH=previous.path;if(previous.digest===undefined)delete process.env.SEMIO_BUILD_GRAPH_DIGEST;else process.env.SEMIO_BUILD_GRAPH_DIGEST=previous.digest;await rm(root,{recursive:true,force:true});}
});

test("public pinned graph expands only compact tokens with shared bounded content reads",async()=>{
  const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"expand-")),previous={path:process.env.SEMIO_BUILD_GRAPH_PATH,digest:process.env.SEMIO_BUILD_GRAPH_DIGEST},tokens:Record<string,string>={},cache=dirname(buildGraphPathV1(root)),paths:string[]=[],patch=require("fast-json-patch"),replace=(value:unknown):any=>JSON.parse(JSON.stringify(value).replaceAll('"$source"',JSON.stringify(tokens.source)).replaceAll('"$lock"',JSON.stringify(tokens.lock))),original=rawPromises.readFile;let active=0,maximum=0,reads=0;
 const mocked=spyOn(rawPromises,"readFile").mockImplementation((async(path:any,options:any)=>{const tracked=String(path).startsWith(cache)&&String(path).includes("-closures/");if(tracked){reads++;active++;maximum=Math.max(maximum,active);await Bun.sleep(10);}try{return await original(path,options);}finally{if(tracked)active--;}}) as typeof rawPromises.readFile);
 try{
  for(const[kind,contract]of Object.entries(fixture.compactExpansion.contracts)){const contractSchema=kind==="source"?require("../../📥️inputs/🧮️digest/🔣️schema.json"):require("../../📥️inputs/🔒️lock/🔣️schema.json").$defs.CargoLockInventory;expect(new Ajv({strict:false}).compile(contractSchema)(contract)).toBe(true);const source=JSON.stringify(contract),digest=new Bun.CryptoHasher("sha256").update(source).digest("hex"),directory=join(cache,kind==="source"?"build-input-closures":"build-lock-closures"),path=join(directory,digest+".json");await mkdir(directory,{recursive:true});await writeFile(path,source);paths.push(path);tokens[kind]=(kind==="source"?"SEMIO_SOURCE_CLOSURE_":"SEMIO_CARGO_LOCK_")+digest;}
  const portable=graph(),expected=graph();expect(patch.applyPatch(portable,replace(fixture.compactExpansion.inputPatch),true).newDocument).toBe(portable);expect(patch.applyPatch(expected,replace(fixture.compactExpansion.expectedPatch),true).newDocument).toBe(expected);await withGraphTestInvocationV1(original=>withRepositoryBuildGraphArtifactV1(process.cwd(),portable,original,async pin=>{process.env.SEMIO_BUILD_GRAPH_PATH=pin.path;process.env.SEMIO_BUILD_GRAPH_DIGEST=pin.digest;
  expect(await loadBuildGraphV1(root,undefined,{reuse:true})).toEqual(expected);expect(reads).toBe(2);expect(new Ajv({strict:false}).compile(schema)(expected)).toBe(true);
  const batch=graph(),input:any[]=[],expanded:string[]=[];for(let index=0;index<fixture.compactExpansion.batch.count;index++){const file=fixture.compactExpansion.batch.filePrefix+index+".ts",source=JSON.stringify({version:1,files:[file]}),digest=new Bun.CryptoHasher("sha256").update(source).digest("hex");await writeFile(join(cache,"build-input-closures",digest+".json"),source);input.push({env:"SEMIO_SOURCE_CLOSURE_"+digest});expanded.push("{workspaceRoot}/"+file);}batch.projects.app!.targets.build!.inputs=input;await withRepositoryBuildGraphArtifactV1(process.cwd(),batch,original,async batchPin=>{process.env.SEMIO_BUILD_GRAPH_PATH=batchPin.path;process.env.SEMIO_BUILD_GRAPH_DIGEST=batchPin.digest;reads=0;maximum=0;expect((await loadBuildGraphV1(root,undefined,{reuse:true})).projects.app!.targets.build!.inputs).toEqual(expanded);expect(reads).toBe(fixture.compactExpansion.batch.count);expect(maximum).toBeGreaterThan(1);expect(maximum).toBeLessThanOrEqual(fixture.compactExpansion.batch.maximumReads);});
  process.env.SEMIO_BUILD_GRAPH_PATH=pin.path;process.env.SEMIO_BUILD_GRAPH_DIGEST=pin.digest;await writeFile(paths[0]!,"{}");await expect(loadBuildGraphV1(root,undefined,{reuse:true})).rejects.toThrow(fixture.compactExpansion.digestError);console.log("[DEBUG] compact graph repeated tokens read once; bounded workers="+maximum+"; content digest refusal verified");}));
 }finally{mocked.mockRestore();if(previous.path===undefined)delete process.env.SEMIO_BUILD_GRAPH_PATH;else process.env.SEMIO_BUILD_GRAPH_PATH=previous.path;if(previous.digest===undefined)delete process.env.SEMIO_BUILD_GRAPH_DIGEST;else process.env.SEMIO_BUILD_GRAPH_DIGEST=previous.digest;await rm(root,{recursive:true,force:true});}
},20000);



test("a captured descendant whose ancestor becomes a file provides membership retry evidence",async()=>{
  const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"replaced-ancestor-")),row=fixture.authority.replacedAncestor,oracle=independentGlobV1;
  try{
    await mkdir(join(root,row.directory));await writeFile(join(root,row.file),row.content);const files=await workspaceBuildFilesV1(root);
    expect(files).toEqual(oracle.sync("**/*",{cwd:root,onlyFiles:true}));await rm(join(root,row.directory),{recursive:true});await writeFile(join(root,row.directory),row.content);
    expect(oracle.sync(row.file,{cwd:root,onlyFiles:true})).toEqual([]);await expect(rawPromises.lstat(join(root,row.file))).rejects.toMatchObject({code:"ENOTDIR"});await expect(buildGraphMembershipV1(root,files)).rejects.toMatchObject({code:"BUILD_MEMBERSHIP_CHANGED"});
  }finally{await rm(root,{recursive:true,force:true});}
});


test("bounded manifest discovery delegates original progress, continuation and cancellation",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"inference-control-test",maximumElapsedMilliseconds:10000},{},Date.now()),async original=>{
  const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"inference-control-")),controller=new AbortController(),phases:string[]=[],files:string[]=[];let continuations=0;
  const invocation={...original,control:{...original.control,signal:AbortSignal.any([original.control.signal,controller.signal]),publish:(event:any)=>{phases.push(event.command);return original.control.publish(event);},yieldContinuation:async()=>{continuations++;controller.abort(Error("fixture-inference-cancelled"));await original.control.yieldContinuation();}}};
  try{
    for(let index=0;index<fixture.inferenceControl.owners;index++){const directory="owner-"+index,file=directory+"/Cargo.toml",manifest=fixture.inferenceControl.manifest.replaceAll("{{index}}",String(index));expect(require("@iarna/toml").parse(manifest).package.name).toBe(directory);await mkdir(join(root,directory));await writeFile(join(root,file),manifest);files.push(file);}
    const owner=require("../../🕸️graph/📥️inference/🟨️.mjs");await owner.libraryBootstrap;
    const {checkScriptInvocation,advanceScriptInvocation}=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
    await expect(owner.cacheInternals.walkCargoTomlControlledV1(root,{check:()=>checkScriptInvocation(invocation),advance:(phase:string)=>advanceScriptInvocation(invocation,"build:"+phase,"running")})).rejects.toThrow("fixture-inference-cancelled");
    expect(phases).toEqual(["build:manifest-discovery"]);expect(continuations).toBe(fixture.inferenceControl.continuations);expect(invocation.control.remainingMilliseconds).toBe(original.control.remainingMilliseconds);
  }finally{await rm(root,{recursive:true,force:true});}
}));

test("primary native owner discovery retains its original phase cancellation before any fact write",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"owner-control-test",maximumElapsedMilliseconds:10000},{},Date.now()),async original=>{
 const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"oc-")),controller=new AbortController(),phases:string[]=[],files:string[]=[];let continuations=0;
 const invocation={...original,control:{...original.control,signal:AbortSignal.any([original.control.signal,controller.signal]),publish:(event:any)=>{phases.push(event.command);return original.control.publish(event);},yieldContinuation:async()=>{continuations++;controller.abort(Error("fixture-owner-cancelled"));await original.control.yieldContinuation();}}};
 try{
  for(let index=0;index<fixture.inferenceControl.owners;index++){const directory="owner-"+index,file=directory+"/Cargo.toml",manifest=fixture.inferenceControl.manifest.replaceAll("{{index}}",String(index));expect(require("@iarna/toml").parse(manifest).package.name).toBe(directory);await mkdir(join(root,directory));await writeFile(join(root,file),manifest);files.push(file);}
  const owner=await import("../../🕸️graph/📥️inference/🟨️.mjs");await owner.libraryBootstrap;await expect(owner.discoverNativeBuildOwnersV1(files,root,{workspaceRoot:root,invocation})).rejects.toThrow("fixture-owner-cancelled");expect(phases).toEqual([fixture.inferenceControl.phase]);expect(continuations).toBe(fixture.inferenceControl.continuations);expect(invocation.control.remainingMilliseconds).toBe(original.control.remainingMilliseconds);await expect(access(join(root,".cache"))).rejects.toThrow();
 }finally{await rm(root,{recursive:true,force:true});}
}));


test("configuration options preserve task identity and command form under both schema oracles",()=>{
 const authored=require("../../🧬️schema/📋️project/🔣️.json"),authoring=new(require("ajv").default)({strict:true}).compile(authored),portable=new Ajv({strict:true}).compile(schema);
 for(const row of fixture.configurationRefusals){const target={executor:"repo:script",options:{command:"one"},configurations:{release:row.configuration}},project={name:"app",targets:{build:target}};expect(authoring(project),row.name).toBe(false);expect(()=>neutralBuildTargetV1(target),row.name).toThrow();const value=graph();value.projects.app!.targets.build!.configurations={release:row.name==="commands"?{execution:row.configuration}:row.name==="kind"||row.name==="ownerArguments"||row.name==="nativeOwnerCommand"?{execution:row.configuration}:row.configuration};expect(portable(value),row.name).toBe(false);expect(()=>readBuildGraphV1(value),row.name).toThrow();}
});


test("script build requirements admit body changes while retaining compiler selection",async()=>{
 const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"requirements-")),owner=require("../../🕸️graph/📥️inference/🟨️.mjs"),observation=require("../../👁️observation/🟨️.mjs"),ts=require("typescript");
 try{await owner.libraryBootstrap;const provider=owner.prepareSourceAnalysesV1().script,file=join(root,"📜️script.ts"),validate=new Ajv({strict:true}).compile(schema.$defs.ScriptBuildRequirements);let original:any;
  for(const[index,row]of fixture.scriptRequirements.entries()){await writeFile(file,row.source);const captured=await observation.captureObservedSourceInputsV1(()=>observation.observedSourceAnalysisV1(provider,file));expect(validate(captured.value)).toBe(true);expect(captured.value).toEqual({go:row.go,wasm:row.wasm});const calls:string[]=[];const visit=(node:any)=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression))calls.push(node.expression.text);ts.forEachChild(node,visit);};visit(ts.createSourceFile(file,row.source,ts.ScriptTarget.Latest,false));expect(captured.value).toEqual({go:calls.some(name=>name.startsWith("runGo")),wasm:calls.some(name=>["buildWasmWebV1","buildRepositoryWasmWebV1"].includes(name))});if(index===0)original=captured.observations.filter((row:any)=>row.path===file);else expect(await observation.observationsHold(original)).toBe(index===1);}
 }finally{await rm(root,{recursive:true,force:true});}
});

test("catalog provider imports bind actual source revisions across repeated admission",async()=>{
 const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"provider-revision-")),owner=require("../../🕸️graph/📥️inference/🟨️.mjs"),row=fixture.catalogReload,file=join(root,row.provider.sourcePath),project=join(root,row.provider.ownerRoot,"📋️project.json");
 try{await owner.libraryBootstrap;await mkdir(dirname(file),{recursive:true});await writeFile(join(root,row.contribution.inputs[0]!),"{}");const put=async(contribution:typeof row.contribution)=>{await writeFile(file,row.source.replace("@@CONTRIBUTION@@",JSON.stringify(contribution)));await writeFile(project,JSON.stringify({name:contribution.project,targets:{generate:{}}}));};await put(row.contribution);const url=await import("node:url"),canonical=await import(url.pathToFileURL(file).href);expect(await canonical.catalogBuildContributionV1()).toEqual(row.contribution);expect(await owner.cacheInternals.catalogBuildContributionsV1(root,[row.provider])).toEqual([row.contribution]);const next={...row.contribution,project:row.nextProject};await put(next);expect(await(await import(url.pathToFileURL(file).href)).catalogBuildContributionV1()).toEqual(row.contribution);expect(await owner.cacheInternals.catalogBuildContributionsV1(root,[row.provider])).toEqual([next]);expect(new Bun.CryptoHasher("sha256").update(await readFile(file)).digest("hex")).toBe(require("node:crypto").createHash("sha256").update(await readFile(file)).digest("hex"));
 }finally{await rm(root,{recursive:true,force:true});}
});

test("authored command shorthand normalizes to the complete portable execution",()=>{
  const source=fixture.topCommand,author=require("../../🧬️schema/📋️project/🔣️.json"),Draft7=require("ajv").default,validate=new Draft7({strict:true}).compile(author),neutral=neutralBuildTargetV1(source.target),resolved=resolveBuildTargetV1(neutral,"release"),sdk=createRequire(resolve(process.cwd(),".nx/installation/package.json")),projection=projectBuildTargetForNxV1(neutral),oracle=sdk("nx/src/utils/params").combineOptionsForExecutor({},"release",projection,sdk("nx/src/executors/run-commands/schema.json"),null,null);
  const inferred=require("../../🕸️graph/📥️inference/🟨️.mjs").cacheInternals.targetWithDefaults(source.target,"app",true);expect(inferred).not.toHaveProperty("command");expect(inferred.options.command).toBe(source.target.command);
  expect(validate({name:"app",targets:{build:source.target}})).toBe(true);expect(neutral).not.toHaveProperty("command");expect(neutral.execution.command).toBe(source.target.command);expect(resolved.execution.command).toBe(source.target.configurations.release.command);expect(oracle.command).toBe(resolved.execution.command);expect(oracle.env).toEqual(resolved.execution.env);
  for(const target of source.invalid){expect(validate({name:"app",targets:{build:target}})).toBe(false);expect(()=>neutralBuildTargetV1(target)).toThrow();}
});

test("selected Rust aggregate scheduling stays invariant across selectable modes",async()=>{
  const source=fixture.rustAggregate,owner=await import("../../🕸️graph/🦀️rust/🟦️.ts"),aggregate=owner.selectedRustAggregateTargetV1(source.original as any,source.manifest,source.mode,source.roots),portable=graph();portable.projects.app.targets.build=aggregate;portable.projects.producer={name:"producer",root:"producer",targets:{compile:{execution:{kind:"noop"},configurations:{release:{execution:{}}}}}};readBuildGraphV1(portable);expect(new Ajv({strict:true}).compile(schema)(portable)).toBe(true);
  expect(aggregate.execution).toEqual({kind:"noop"});expect(aggregate.cache).toBe(false);expect(aggregate.inputs).toEqual([]);expect(aggregate.outputs).toEqual([]);expect(aggregate.dependsOn).toEqual(["lib:build","producer:compile"]);expect(resolveBuildTargetV1(aggregate,"release").execution).toEqual({kind:"noop",env:{}});expect(resolveBuildTargetV1(aggregate,"prototype").dependsOn).toEqual(aggregate.dependsOn);
  const {planBazelTasksV1}=await import("../../🔧️tools/🏗️bazel/📜️script.ts"),plan=planBazelTasksV1(portable,{tasks:["app:build:release"]}),projection=projectBuildGraphForNxV1(portable),nodes=Object.fromEntries(Object.entries(projection.projects).map(([name,data])=>[name,{name,type:"lib",data}])),dependencies=Object.fromEntries(Object.keys(nodes).map(name=>[name,portable.dependencies.filter(edge=>edge.source===name)])),tasks=require("nx/src/tasks-runner/create-task-graph").createTaskGraph({nodes,dependencies},{},["app"],["build"],"release",{});expect(Object.keys(plan.tasks).sort()).toEqual(source.tasks);expect(Object.keys(tasks.tasks).sort()).toEqual(source.tasks);expect(tasks.dependencies["app:build:release"].sort()).toEqual(plan.tasks["app:build:release"].dependencies.sort());
});

test("configured command sources preserve every recipe through read-only production source inference and Nx inputs",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"configured-source-inference",maximumElapsedMilliseconds:30000},{},Date.now()),async invocation=>{
 const row=fixture.configuredCommandSources,workspace=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"cc-")),sdk=createRequire(resolve(process.cwd(),".nx/installation/package.json")),hasher=sdk("nx/src/hasher/task-hasher"),minimatch=sdk("minimatch").minimatch,compiler=require("typescript"),validate=new(require("ajv").default)({strict:true}).compile(require("../../🧬️schema/📋️project/🔣️.json")),{inferReadOnlyBuildProjectSourcesV1,cacheInternals}=await import("../../🕸️graph/📥️inference/🟨️.mjs");
 const fallback=["{workspaceRoot}/fallback-marker.ts"],paths=Object.keys(row.files);
 try{
  for(const[path,bytes]of Object.entries(row.files)){await mkdir(dirname(join(workspace,path)),{recursive:true});await writeFile(join(workspace,path),bytes);}
  for(const entry of row.cases){
   const target={executor:"repo:script",cache:true,outputs:["{projectRoot}/dist"],options:entry.options,configurations:entry.configurations},declared={name:"configured-inputs",targets:{build:target}};
   expect(validate(declared),entry.name+JSON.stringify(validate.errors)).toBe(true);await writeFile(join(workspace,"owner/📋️project.json"),JSON.stringify(declared));
   const portable=graph();portable.projects.app.targets.build=neutralBuildTargetV1(target);expect(readBuildGraphV1(portable).projects.app.targets.build.execution.args).toEqual(entry.options.args);
   const project=await inferReadOnlyBuildProjectSourcesV1(declared,"owner",workspace,invocation),expected=[...("expectedSources"in entry?entry.expectedSources:row.expectedSources)].sort(),generic=cacheInternals.genericTargetCommandInputs(target,workspace,fallback,undefined,"owner"),native=cacheInternals.nativeTargetCommandInputs(target,workspace,fallback,undefined,"owner");
   const direct=generic.filter((input:unknown)=>typeof input==="string"&&paths.some(path=>input==="{workspaceRoot}/"+path)).map((input:string)=>input.slice("{workspaceRoot}/".length)).sort();expect(direct,entry.name).toEqual(expected);for(const path of expected)expect(native).toContain("{workspaceRoot}/"+path);expect(generic.includes(fallback[0]),entry.name).toBe("fallback"in entry&&entry.fallback);
   if(expected.length){const oracle=await build({entryPoints:expected.filter(path=>path.endsWith("📜️script.ts")).map(path=>join(workspace,path)),absWorkingDir:workspace,outdir:join(workspace,"oracle"),bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,logLevel:"silent"});expect(Object.keys(oracle.metafile!.inputs).sort(),entry.name).toEqual(expected);}
   const program=compiler.createProgram(expected.filter(path=>path.endsWith("📜️script.ts")).map(path=>join(workspace,path)),{types:[],noLib:true,noResolve:false,allowImportingTsExtensions:true,moduleResolution:compiler.ModuleResolutionKind.Bundler,module:compiler.ModuleKind.ESNext});expect(program.getSourceFiles().map((source:any)=>source.fileName.slice(workspace.length+1).replaceAll("\\","/")).sort(),entry.name).toEqual(expected);
   const groups=project.namedInputs,patterns=hasher.getTargetInputs({}, {data:project},"build").selfInputs.map((pattern:string)=>pattern.replaceAll("{projectRoot}",project.root).replaceAll("{workspaceRoot}/","")),expanded=project.targets.build.inputs.filter((input:any)=>typeof input==="string"&&!input.startsWith("^")).flatMap((input:string)=>groups[input]?hasher.expandNamedInput(input,groups):[{fileset:input}]),inventory=new Set<string>();
   for(const input of expanded)if(input.env?.startsWith("SEMIO_SOURCE_CLOSURE_")){const record=JSON.parse(await readFile(join(dirname(buildGraphPathV1(workspace)),"build-input-closures",input.env.slice(21)+".json"),"utf8"));for(const path of record.files)inventory.add(path);}
   const covered=(path:string)=>inventory.has(path)||patterns.some((pattern:string)=>!pattern.startsWith("!")&&minimatch(path,pattern,{dot:true}))&&!patterns.some((pattern:string)=>pattern.startsWith("!")&&minimatch(path,pattern.slice(1),{dot:true}));for(const path of expected)expect(covered(path),entry.name+":"+path).toBe(true);expect(covered("unrelated/📜️script.ts"),entry.name).toBe(false);
   expect(project.targets.build.outputs).toEqual(target.outputs);expect(project.targets.build.cache).toBe(true);for(const configuration of Object.values(project.targets.build.configurations??{}))expect(configuration).not.toHaveProperty("inputs");
   if(entry.name==="commandAndCwd"){
    const selected=sdk("nx/src/utils/params").combineOptionsForExecutor({},"release",project.targets.build,sdk("nx/src/executors/run-commands/schema.json"),null,null),run=()=>{const child=Bun.spawnSync([process.execPath,join(workspace,selected.cwd,"📜️script.ts")],{cwd:join(workspace,selected.cwd),stdout:"pipe",stderr:"pipe"});expect(child.exitCode).toBe(0);return new TextDecoder().decode(child.stdout).trim();},hash=()=>{const node=createHash("sha256"),bun=new Bun.CryptoHasher("sha256");for(const path of paths.filter(covered).sort()){const bytes=readFileSync(join(workspace,path));node.update(path+"\0").update(bytes).update("\0");bun.update(path+"\0").update(bytes).update("\0");}const digest=node.digest("hex");expect(digest).toBe(bun.digest("hex"));return digest;},before={output:run(),hash:hash()};
    await writeFile(join(workspace,row.mutation.path),row.mutation.bytes);expect(run()).not.toBe(before.output);expect(hash()).not.toBe(before.hash);await writeFile(join(workspace,row.mutation.path),row.files[row.mutation.path]);console.log("[DEBUG] Configured external script execution and admitted byte hash both change");
   }
  }
  expect(cacheInternals.nativeLockInputs(row.configuredLocks)).toEqual(row.expectedLocks);expect(cacheInternals.nativeLockInputs({options:{commands:[{command:row.configuredLocks.configurations.release.command}]}})).toEqual(row.expectedLocks);
  await expect(access(join(workspace,".cache"))).rejects.toThrow();await expect(access(join(workspace,".🧬semio"))).rejects.toThrow();
  const outside=workspace+"-outside.ts";await writeFile(outside,"export const outside=1;\n");try{await mkdir(join(workspace,"bad"));await writeFile(join(workspace,"bad/📜️script.ts"),`import ${JSON.stringify("../../"+outside.slice(dirname(workspace).length+1))};\n`);expect(()=>cacheInternals.targetScriptClosure({options:{command:"bun bad/📜️script.ts"}},workspace)).toThrow(/escapes workspace/);}finally{await rm(outside,{force:true});}
  console.log(`[DEBUG] ${row.cases.length} configured command/cwd/list/args/fallback fixtures agree with strict schema, Nx, TypeScript and esbuild`);
 }finally{await rm(workspace,{recursive:true,force:true});}
}),30000);

test("neutral arguments admit literal arrays and refuse scalar shell fragments before execution",()=>{
 const validate=new(require("ajv").default)({strict:true}).compile(require("../../🧬️schema/📋️project/🔣️.json")),portable=new Ajv({strict:true}).compile(schema),row=fixture.literalArguments;
 for(const args of row.rejected){
  const base={executor:"repo:script",options:{command:"bun ./📜️script.ts",args}},configured={executor:"repo:script",options:{command:"bun ./📜️script.ts"},configurations:{release:{args}}};
  for(const target of [base,configured]){expect(validate({name:"app",targets:{build:target}})).toBe(false);expect(()=>neutralBuildTargetV1(target,"app:build")).toThrow(/arguments.*app:build/);}
  const baseGraph=graph();baseGraph.projects.app.targets.build.execution.args=args as any;expect(portable(baseGraph)).toBe(false);expect(()=>readBuildGraphV1(baseGraph)).toThrow();const configuredGraph=graph();configuredGraph.projects.app.targets.build.configurations={release:{execution:{args}}};expect(portable(configuredGraph)).toBe(false);expect(()=>readBuildGraphV1(configuredGraph)).toThrow();
 }
 for(const converted of row.converted){const project=JSON.parse(readFileSync(resolve(process.cwd(),converted.path),"utf8")),target=project.targets[converted.target],neutral=neutralBuildTargetV1(target,project.name+":"+converted.target),selected=resolveBuildTargetV1(neutral,converted.configuration);expect(validate(project)).toBe(true);expect(selected.execution.args).toEqual(converted.args);expect(require("../../📥️caller/🟦️.ts").buildExecutionArgumentsV1(selected.execution,row.boundaryArguments)).toEqual([...converted.args,...row.boundaryArguments]);}
 console.log(`[DEBUG] ${row.converted.length} current authored argv arrays admit; scalar/dynamic/non-string argv are refused before execution`);
});

test("private Nx engine bytes affect real native task hashes while invocation staging does not",()=>{
 const row=fixture.nxEngineIdentity,sdk=createRequire(resolve(process.cwd(),".nx/installation/package.json")),native=sdk("nx/src/native"),transform=sdk("nx/src/native/transform-objects").transformProjectGraphForRust,target=projectBuildTargetForNxV1({execution:{kind:"command",command:"bun ./📜️script.ts",cwd:"fixture"},cache:true,inputs:["{projectRoot}/📜️script.ts"]}),task={id:"fixture:build",target:{project:"fixture",target:"build"},overrides:{},outputs:[],projectRoot:"fixture",cache:true,parallelism:true},taskGraph={roots:[task.id],tasks:{[task.id]:task},dependencies:{[task.id]:[]},continuousDependencies:{[task.id]:[]}};
 const projectedInputs=target.inputs??[],files=new Set([row.engine,...projectedInputs.filter((input:unknown):input is string=>typeof input==="string"&&input.startsWith("{workspaceRoot}/")&&!input.includes("*")).map((input:string)=>input.slice("{workspaceRoot}/".length))]);
 const hash=(engine:string,wire:string,nx:string)=>{
  const graph={nodes:{fixture:{name:"fixture",type:"lib",data:{root:"fixture",targets:{build:target},namedInputs:{default:["{projectRoot}/**/*"]}}}},externalNodes:{"npm:nx":{name:"npm:nx",type:"npm",data:{packageName:"nx",version:"23.2.0",hash:nx}},"npm:typescript":{name:"npm:typescript",type:"npm",data:{packageName:"typescript",version:"5.9.3",hash:"typescript-fixed"}}},dependencies:{fixture:[]}},graphRef=native.transferProjectGraph(transform(graph)),filemap=native.testOnlyTransferFileMap({fixture:[{file:row.script,hash:"script-fixed"}]},[...[...files].map(file=>({file,hash:file===row.engine?engine:"engine-fixed"})),{file:row.wire,hash:wire}]),planner=new native.HashPlanner({},graphRef),plans=planner.getPlansReference([task.id],taskGraph),hasher=new native.TaskHasher(process.cwd(),graphRef,filemap.projectFiles,filemap.allWorkspaceFiles,Buffer.from("{}"),{},undefined,{selectivelyHashTsConfig:false});
  return {value:hasher.hashPlans(plans,{[task.id]:{}},process.cwd(),true)[task.id].value,instructions:planner.getPlans([task.id],taskGraph)[task.id]};
 };
 const baseline=hash("engine-before","wire-before","nx-before"),changed=hash("engine-after","wire-before","nx-before"),restaged=hash("engine-before","wire-after","nx-before"),upgraded=hash("engine-before","wire-before","nx-after");console.log("[DEBUG] Independent native Nx engine hash oracle "+JSON.stringify({baseline,changed,restaged,upgraded}));
 expect(changed.value).not.toBe(baseline.value);expect(restaged.value).toBe(baseline.value);expect(upgraded.value).not.toBe(baseline.value);
 const implicit=projectBuildTargetForNxV1({execution:{kind:"command",command:"bun ./📜️script.ts"},cache:true});expect(implicit.inputs).toContain("default");expect(implicit.inputs).toContain("^default");
});

test("neutral graph reset selects owned publication files and preserves storage receipts and pinned facts",async()=>{
 const owner=await import("../../🕸️graph/🟦️.ts"),row=fixture.graphReset,contract=require("../../🧬️schema/🗑️reset.json"),validator=new Ajv({strict:false}).compile(contract),root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"rs-")),oracle=independentGlobV1;
 try{
  expect(buildGraphPathV1(root)).toBe(join(dirname(buildGraphPathV1(root)),"canonical.json"));
  for(const path of row.files){await mkdir(dirname(join(root,path)),{recursive:true});await writeFile(join(root,path),path);}
  const device=(await rawPromises.lstat(root)).dev,entries=await Promise.all((await rawPromises.readdir(root)).map(async name=>{const status=await rawPromises.lstat(join(root,name));return {name,kind:status.isFile()?"file":status.isDirectory()?"directory":"other",links:status.nlink,bytes:status.size,device:status.dev};})),inventory={device,entries};expect(validator(inventory)).toBe(true);
  const expected=oracle.sync(["canonical.json","authority.json","canonical.json.????????-????-????-????-????????????","authority.json.????????-????-????-????-????????????"],{cwd:root,onlyFiles:true}).sort();expect(expected).toEqual(row.removed);expect(owner.planBuildGraphInvalidationV1(inventory)).toEqual(expected);
  const valid={name:"canonical.json",kind:"file",links:1,bytes:0,device:1};for(const invalid of row.invalid)expect(()=>owner.planBuildGraphInvalidationV1({device:1,entries:[{...valid,...invalid}]})).toThrow();
  const controller=new AbortController(),reason=Error("reset-original-cancelled");await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"reset-control-test",maximumElapsedMilliseconds:10000},{},Date.now()),async original=>{controller.abort(reason);const invocation={...original,control:{...original.control,signal:AbortSignal.any([original.control.signal,controller.signal])}};await expect(owner.invalidateBuildGraphV1(root,invocation)).rejects.toThrow(reason.message);expect(invocation.control.remainingMilliseconds).toBe(original.control.remainingMilliseconds);});
  for(const path of row.files)expect(await readFile(join(root,path),"utf8")).toBe(path);
  const source=await readFile(new URL("../../📜️script.ts",import.meta.url),"utf8");expect(source).toContain("invalidateBuildGraphV1(root,invocation)");expect(source).not.toContain('rm(join(root,".🧬semio/🦑️repo/⚡️cache/build")');console.log("[DEBUG] Real filesystem reset inventory agrees with glob; original cancellation preserves every file before storage admission");
 }finally{await rm(root,{recursive:true,force:true});}
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
  expect(validate(actual)).toBe(true);expect(actual).toEqual(row.references);expect(actual).toEqual(oracle);
 }
 console.log("[DEBUG] Private engine source references schema=Ajv oracle=Babel cases="+fixture.engineReferences.length);
});


test("private engine references keep actual directory-index imports and missing-file refusals",async()=>{
 const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"engine-files-")),owner=join(root,"engine.mjs"),index=join(root,"dir/index.js");
 try{await mkdir(dirname(index),{recursive:true});await writeFile(owner,"");await writeFile(index,"module.exports = {};\n");
  expect(nxBuildEngineSourceFileV1(owner,"./dir/")).toBe(createRequire(owner).resolve("./dir/"));
  expect(()=>nxBuildEngineSourceFileV1(owner,"./missing.json")).toThrow("Private Nx engine import missing");
  expect(()=>nxBuildEngineSourceFileV1(owner,"./missing/")).toThrow("Private Nx engine import missing");
  console.log("[DEBUG] Actual engine directory index agrees with Node resolution; missing file and module refused");
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
  console.log("[DEBUG] Actual frozen Nx explicit plugin paths agree with Node resolution; missing file refuses and bare package preserves discovery");
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
  console.log("[DEBUG] Bounded concurrent physical admission matches Ajv+glob, all active readers finish before failure release");
 }finally{await rm(root,{recursive:true,force:true});}
});


test("private engine references retain nested structural Source owners and newly admitted ignored leaves",async()=>{
 const row=nxAdmissionFixture.collectionAdmission,root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"structural-walk-"));
 try{for(const[path,bytes]of Object.entries(row.files)){await mkdir(dirname(join(root,path)),{recursive:true});await writeFile(join(root,path),bytes);}
  const inventory=[row.indexed.join("\0"),row.untracked.join("\0")],actual=await collectionManifestsFromInventory(root,inventory,nxAdmissionPolicy,row.collections),oracle=await independentGlobV1(["**/📋️project.json","**/package.json","**/Cargo.toml"],{cwd:root,dot:true,ignore:["**/dist/**","**/owner/🧫️examples/**"]});expect(actual).toEqual(row.expected);expect(actual).toEqual(oracle.sort());
  await mkdir(dirname(join(root,row.additional.path)),{recursive:true});await writeFile(join(root,row.additional.path),row.additional.content);expect(await collectionManifestsFromInventory(root,inventory,nxAdmissionPolicy,row.collections)).toEqual([...row.expected,row.additional.path].sort());
  console.log("[DEBUG] Actual concurrent structural admission preserves neutral nested owners and fresh ignored leaf membership, oracle=glob");
 }finally{await rm(root,{recursive:true,force:true});}
});


test("build bootstrap receives the defining dependency module while its executable preserves the original header",async()=>{
  const row=fixture.bootstrap.moduleReceiving,library=resolve(import.meta.dir,"../../.."),owner=join(library,row.owner),router=join(library,row.router),parser=require("typescript"),body=readFileSync(owner,"utf8"),commands=readFileSync(router,"utf8"),syntax=parser.createSourceFile(owner,body,parser.ScriptTarget.Latest,true),commandSyntax=parser.createSourceFile(router,commands,parser.ScriptTarget.Latest,true),functions=(source:any)=>source.statements.filter((node:any)=>parser.isFunctionDeclaration(node)&&node.modifiers?.some((modifier:any)=>modifier.kind===parser.SyntaxKind.ExportKeyword)).map((node:any)=>node.name.text);
  expect(commands.split("\n")[0]).toBe(row.scriptHeader);expect(body.startsWith("#!")).toBe(false);expect(syntax.parseDiagnostics).toEqual([]);expect(commandSyntax.parseDiagnostics).toEqual([]);expect(functions(syntax)).toEqual(row.operationExports);expect(functions(commandSyntax)).toEqual([]);expect(commandSyntax.statements.filter((node:any)=>parser.isClassDeclaration(node)&&node.modifiers?.some((modifier:any)=>modifier.kind===parser.SyntaxKind.ExportKeyword)).map((node:any)=>node.name.text)).toEqual(row.commands);
  const bootstrap=readFileSync(resolve(import.meta.dir,"../../🚀️bootstrap/📜️script.ts"),"utf8"),imports=parser.preProcessFile(bootstrap,true,true).importedFiles.map((value:any)=>value.fileName);expect(imports).toContain("../../"+row.owner);expect(imports).not.toContain("../../"+row.router);
  const oracle=await build({stdin:{contents:body,sourcefile:owner,resolveDir:dirname(owner),loader:"ts"},bundle:false,write:false,format:"esm",platform:"node",logLevel:"silent"});expect(oracle.errors).toEqual([]);expect(oracle.outputFiles![0]!.text.startsWith("#!")).toBe(false);expect(new Bun.Transpiler({loader:"ts"}).transformSync(body).startsWith("#!")).toBe(false);
  const received=await import(owner);for(const name of row.operationExports)expect(typeof received[name]).toBe("function");for(const operation of ["sync","lock"] as const)expect(received.javascriptDependencyArgumentsV1(operation)).toEqual(["install",operation==="sync"?"--frozen-lockfile":"--lockfile-only"]);
  console.log("[DEBUG] Defining dependency module received directly; original executable header preserved; TypeScript and esbuild agree on the closed operation and command owners");
});


test("original direct dependency CLI admits explicit whole or scoped selections and refuses installer overrides",()=>{
  const oracle=new(require("ajv"))({strict:true}).compile(dependencyInvocationSchema);
  for(const row of fixture.dependencyInvocations.cases){const selected=readBuildDependencyInvocationV1(row.args);expect(selected).toEqual(row.value);expect(oracle(selected),row.name).toBe(true);expect(selected.selection.kind==="all"?["--all"]:["--scope",selected.selection.owner]).toEqual(row.forwarded);}
  for(const args of fixture.dependencyInvocations.refused)expect(()=>readBuildDependencyInvocationV1(args)).toThrow("Expected repo dependencies sync|lock --all|--scope <owner>");
  for(const value of [{mode:"unknown",selection:{kind:"all"}},{mode:"sync",selection:{kind:"all",owner:"."}},{mode:"lock",selection:{kind:"scope"}},{mode:"sync",selection:{kind:"scope",owner:""}},{mode:"sync",selection:{kind:"scope",owner:"\0"}}])expect(oracle(value)).toBe(false);
  console.log("[DEBUG] Direct dependency whole/scoped selections agree with portable closed schema and independent Ajv; installer/resource override arguments refuse");
});

test("registered JavaScript dependency initial callers enter direct finite online preparation before graph bootstrap",async()=>{
  const parser=require("typescript"),workspace=process.cwd(),source=readFileSync(resolve(import.meta.dir,"../../📜️script.ts"),"utf8"),syntax=parser.createSourceFile("build.ts",source,parser.ScriptTarget.Latest,true),definition=syntax.statements.find((node:any)=>parser.isFunctionDeclaration(node)&&node.name?.text==="runOwnedBuildInLeasesV1"),body=definition.getText(syntax);expect(body.indexOf('command==="dependencies"')).toBeLessThan(body.indexOf("await ensureBuildEnvironmentV1"));expect(body).toContain("readBuildDependencyInvocationV1(resolved.args)");
  for(const path of [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){const launch=parser.parseConfigFileTextToJson(path,readFileSync(join(workspace,path),"utf8"));expect(launch.error).toBeUndefined();for(const expected of fixture.dependencyInvocations.launches){const rows=launch.config.configurations.filter((row:any)=>row.name===expected.name);expect(rows).toHaveLength(1);const row=rows[0];expect(row.command).toBe(expected.command);expect(JSON.parse(row.env.SEMIO_SCRIPT_POLICY)).toEqual(expected.policy);expect(row.env.SEMIO_SCRIPT_CAPABILITIES).toBeUndefined();expect(row.env.CARGO_NET_OFFLINE).toBeUndefined();const args=expected.command.split(" ").slice(2),capabilities=createNativeBuildCallerCapabilitiesV1(workspace,args,"nx",{});expect(capabilities.native.network.offline).toBe(false);expect(capabilities.arguments).toEqual(["repo",...args]);const envelope=createScriptProcessEnvelope(expected.policy,capabilities,Date.now());expect(envelope.deadlineEpochMilliseconds).not.toBeNull();expect(envelope.capabilities).toEqual(capabilities);}}
  console.log("[DEBUG] Both authored JavaScript GUI rows select direct dependency commands; original finite600000 initial factory remains online before first process envelope");
});
