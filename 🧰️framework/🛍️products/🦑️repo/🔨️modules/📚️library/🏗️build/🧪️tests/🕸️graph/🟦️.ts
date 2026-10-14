import {projectBuildGraphForNxV1} from "../../../🟨️.mjs";
import {expect,test} from "bun:test";
import {buildInputPathMatchesV1} from "../../../🔌️nx-plugin/📥️inputs/🟨️.mjs";
import {fileURLToPath} from "node:url";
import {createRequire} from "node:module";
import {build} from "esbuild";
import fixture from "../🧫️fixtures/🕸️graph/🔣️.json";
import callerSchema from "../../🧬️schema/📥️caller.json";
import {createNativeBuildCallerCapabilitiesV1,issueBuildTaskEnvironmentV1} from "../../📥️caller/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import {readBuildGraphV1,neutralBuildTargetV1,nxBuildTargetV1,resolveBuildTargetV1} from "../../🕸️graph/🟦️.ts";
import {parseBuildInvocationV1,nxBuildArgumentsV1,resolveBuildInvocationV1} from "../../🧭️invocation/🟦️.ts";
import {selectBuildTasksV1,selectBuildProjectsV1} from "../../🧭️selection/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020").default;
const graph=()=>readBuildGraphV1(structuredClone(fixture.graph));

test("portable graph fixture agrees with the independent JSON Schema implementation",()=>{
  const validate=new Ajv({strict:false}).compile(schema);
  expect(validate(fixture.graph)).toBe(true);
  expect(graph().projects.app.name).toBe("app");
  for(const value of [{...fixture.graph,version:2},{...fixture.graph,projects:{bad:{name:"bad",root:"bad",targets:{build:{}}}}}]){
    expect(validate(value)).toBe(false);expect(()=>readBuildGraphV1(value)).toThrow();
  }
});

test("portable configuration and Nx projection preserve command semantics",()=>{
  const target={executor:"repo:script",options:{command:"bun ./📜️script.ts build",cwd:"app",env:{BASE:"yes"}},configurations:{release:{env:{MODE:"release"},outputs:["dist/release"],dependsOn:["prepare-release"],cache:false}},outputs:["dist"],dependsOn:["^build"]};
  const neutral=neutralBuildTargetV1(target),resolved=resolveBuildTargetV1(neutral,"release"),nx=nxBuildTargetV1(neutral);
  expect(neutral.execution.kind).toBe("command");expect(neutral).not.toHaveProperty("executor");
  expect(resolved.execution.env).toEqual({BASE:"yes",MODE:"release"});expect(resolved.outputs).toEqual(["dist/release"]);expect(resolved.dependsOn).toEqual(["prepare-release"]);expect(resolved.cache).toBe(false);expect(nx).toEqual({...target,executor:"nx:run-commands"});
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
  expect(projection.projects.app.targets.build.executor).toBe("nx:run-commands");
});
