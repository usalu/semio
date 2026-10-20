/** 🧪️ Bazel operations: neutral parity of affected, dry-run plans, exec and cache status|clear against Nx's own code and Bazel's own binary. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {spawn,spawnSync} from "node:child_process";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1,type ScriptInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {bazelBuildCacheToolV1,bazelBuildEventsFileV1,bazelBuildMetricsV1,bazelDryRunPlanV1,bazelEmptyPlanV1,clearBazelDiskCacheV1,clearBazelServerBasesV1,measureBazelDiskCacheV1,readBazelLedgerV1,recordBazelBuildMetricsV1} from "../🟦️.ts";
import {bazelCacheLayoutV1} from "../../💾️storage/🟦️.ts";
import {bazelTaskBaseEnvironmentV1,planBazelTasksV1} from "../../📜️script.ts";
import type {BazelTaskV1} from "../../📋️task-plan/🟦️.ts";
import {affectedBuildProjectsV1,selectBuildTasksV1} from "../../../../🧭️selection/🟦️.ts";
import {buildExecPlanV1} from "../../../../⚙️operations/🟦️.ts";
import {buildExecEnvironmentV1} from "../../../../⚙️operations/🌱️exec-environment/🟦️.ts";
import type {BuildGraphV1} from "../../../../🕸️graph/🟦️.ts";
import neutral from "../../../../⚙️operations/🧬️schema/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

const root=join(import.meta.dir,"../../../../../../../../../..").replace(/[\\/]+$/u,""),installed=createRequire(join(root,"package.json")),posix=process.platform!=="win32";
const ajv=new Ajv({strict:true,allowUnionTypes:true}),compile=(source:{$schema:string;definitions:object},definition:string)=>ajv.compile({$schema:source.$schema,definitions:source.definitions,$ref:`#/definitions/${definition}`});
const dryRun=compile(neutral,"DryRunPlan"),cacheStatus=compile(neutral,"CacheStatus"),cacheClear=compile(neutral,"CacheClear"),execEnvironment=compile(neutral,"ExecEnvironment");

async function withControl<T>(body:(control:ResourceLeaseControl,original:ScriptInvocation)=>Promise<T>):Promise<T>{
 return withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel-operations-model",maximumElapsedMilliseconds:600000},{},Date.now()),original=>body(createIssuedResourceControlV1(original,"test:bazel-operations",0),original));
}
const temporary=()=>realpathSync(mkdtempSync(join(tmpdir(),"bzl-ops-")));
const lay=(directory:string,files:readonly {path:string;bytes:number}[])=>{for(const file of files){mkdirSync(dirname(join(directory,file.path)),{recursive:true});writeFileSync(join(directory,file.path),"x".repeat(file.bytes));}};

type Case=(typeof fixture.cases)[number];
const toGraph=(row:Case):BuildGraphV1=>({version:1,namedInputs:(row as {namedInputs?:Record<string,unknown[]>}).namedInputs,projects:Object.fromEntries(Object.entries(row.projects).map(([name,project])=>[name,{name,root:project.root,targets:Object.fromEntries(Object.entries(project.targets as Record<string,{dependsOn?:string[];inputs?:string[];continuous?:boolean}>).map(([target,definition])=>[target,{execution:{kind:"noop" as const},...definition}]))}])),dependencies:row.edges.map(([source,target])=>({source:source!,target:target!,type:"static"}))}) as BuildGraphV1;
const normalized=(graph:{roots:readonly string[];dependencies:Readonly<Record<string,readonly string[]>>;continuousDependencies?:Readonly<Record<string,readonly string[]>>;tasks:Readonly<Record<string,unknown>>})=>{
 const sort=(rows:Readonly<Record<string,readonly string[]>>|undefined)=>Object.fromEntries(Object.keys(graph.tasks).sort().map(id=>[id,[...(rows?.[id]??[])].sort()]));
 return {roots:[...graph.roots].sort(),dependencies:sort(graph.dependencies),continuousDependencies:sort(graph.continuousDependencies)};
};
const expectedGraph=(row:Case):Pick<ReturnType<typeof normalized>,"roots"|"dependencies"|"continuousDependencies">=>({roots:row.expected.roots,dependencies:row.expected.dependencies as Record<string,string[]>,continuousDependencies:row.expected.continuousDependencies as Record<string,string[]>});

/** 🌳️ The installed Nx answers every fixture case with its own affected locators and task graph creation, in a clean workspace. */
function nxOracle():Record<string,{affected:string[];graph:ReturnType<typeof normalized>&{tasks:Record<string,{target:{project:string;target:string}}>}}>{
 const directory=temporary();
 try{
  writeFileSync(join(directory,"nx.json"),"{}\n");
  const source=`
const {filterAffected}=require(process.env.NX_AFFECTED),{WholeFileChange}=require(process.env.NX_FILES),{createTaskGraph}=require(process.env.NX_PLAN);
(async()=>{
 const input=JSON.parse(process.env.NX_CASES),result={};
 for(const row of input){
  const nodes=Object.fromEntries(Object.entries(row.projects).map(([name,project])=>[name,{name,type:"lib",data:{name,root:project.root,targets:Object.fromEntries(Object.entries(project.targets).map(([target,definition])=>[target,{executor:"nx:noop",...definition}]))}}])),
   dependencies=Object.fromEntries(Object.keys(nodes).map(name=>[name,row.edges.filter(([source])=>source===name).map(([source,target])=>({source,target,type:"static"}))])),
   graph={nodes,dependencies,externalNodes:{}},
   touched=row.files.map(file=>({file,hash:"",getChanges:()=>[new WholeFileChange()]})),
   affected=Object.keys((await filterAffected(graph,touched,{namedInputs:row.namedInputs??{}},{name:"workspace"})).nodes).sort(),
   tasks=createTaskGraph(graph,{},affected,row.targets,undefined,{},row.excludeDependencies===true);
  result[row.name]={affected,graph:{roots:tasks.roots,dependencies:tasks.dependencies,continuousDependencies:tasks.continuousDependencies,tasks:tasks.tasks}};
 }
 process.stdout.write(JSON.stringify(result));
})().catch(error=>{process.stderr.write(String(error&&error.stack||error));process.exit(1);});`;
  const run=spawnSync("node",["-e",source],{cwd:directory,encoding:"utf8",maxBuffer:64*1024*1024,env:{...process.env,NX_AFFECTED:installed.resolve("nx/src/project-graph/affected/affected-project-graph"),NX_FILES:installed.resolve("nx/src/project-graph/file-utils"),NX_PLAN:installed.resolve("nx/src/tasks-runner/create-task-graph"),NX_CASES:JSON.stringify(fixture.cases),NX_WORKSPACE_ROOT_PATH:directory,NX_DAEMON:"false"} as NodeJS.ProcessEnv});
  if(run.status!==0)throw Error("nx oracle: "+run.stderr);
  return JSON.parse(run.stdout);
 }finally{rmSync(directory,{recursive:true,force:true});}
}

test("the Bazel operations schema agrees with Ajv on the policy, the ledger and the neutral cache results",()=>{
 const ledger=compile(schema,"Ledger"),metrics=compile(schema,"BuildMetrics");
 expect(ajv.validateSchema(schema)).toBe(true);
 for(const value of [fixture.ledger.expected,...fixture.refusedLedgers])expect(validateJsonSchemaSubset(schema.definitions.Ledger,value,schema).length===0,JSON.stringify(value)).toBe(ledger(value));
 expect(ledger(fixture.ledger.expected)).toBe(true);
 expect(fixture.refusedLedgers.every(value=>!ledger(value))).toBe(true);
 for(const row of fixture.metrics)expect(metrics(row.expected),row.name).toBe(true);
 const activity=fixture.ledger.expected,status={version:1,tool:"bazel",available:true,directory:"/cache/a",entries:8,bytes:75,activity:{builds:activity.builds,actions:activity.actions,executed:activity.executed,cacheHits:activity.cacheHits}};
 expect(cacheStatus(status),JSON.stringify(cacheStatus.errors)).toBe(true);
 expect(cacheStatus({...status,activity:{builds:1}})).toBe(false);
 expect(cacheStatus({...status,extra:1})).toBe(false);
 expect(cacheClear({version:1,tool:"bazel",cleared:true,removedEntries:8,removedBytes:75})).toBe(true);
 expect(schema.definitions.Policy.const.diskCache.directories).toEqual(["ac","cas"]);
});

test("build metrics read from a Bazel build event stream agree with an SQLite reading of the same runner counts",()=>{
 const database=new Database(":memory:");
 try{
  for(const row of fixture.metrics){
   const lines=[...fixture.ignoredLines,JSON.stringify(row.event)].join("\n"),actual=bazelBuildMetricsV1(lines),oracle=database.query(`select
    coalesce(sum(case when json_extract(value,'$.name')='total' then json_extract(value,'$.count') end),0) actions,
    coalesce(sum(case when json_extract(value,'$.name') not in ('total','internal') and json_extract(value,'$.name') not like '% cache hit' then json_extract(value,'$.count') end),0) executed,
    coalesce(sum(case when json_extract(value,'$.name') like '% cache hit' then json_extract(value,'$.count') end),0) cacheHits
    from json_each(?1,'$.buildMetrics.actionSummary.runnerCount')`).get(JSON.stringify(row.event)) as {actions:number;executed:number;cacheHits:number};
   expect(actual,row.name).toEqual(row.expected);
   expect(oracle,row.name).toEqual(row.expected);
  }
  expect(bazelBuildMetricsV1(fixture.ignoredLines.join("\n"))).toBeUndefined();
  expect(bazelBuildMetricsV1("")).toBeUndefined();
 }finally{database.close();}
});

test("the disk cache census counts the ac and cas files and bytes, never tmp or gc, and publishes progress",async()=>{
 const directory=temporary();
 try{
  lay(directory,fixture.census.files);
  await withControl(async control=>{
   const calls:string[]=[],census=await measureBazelDiskCacheV1(directory,{phaseControl:control,progress:async stage=>{calls.push(stage);}});
   expect(census).toEqual({available:true,...fixture.census.expected});
   expect(await measureBazelDiskCacheV1(join(directory,"absent"),{phaseControl:control,progress:async()=>{}})).toEqual({available:false,entries:0,bytes:0});
   if(posix){const listed=spawnSync("find",["ac","cas","-type","f"],{cwd:directory,encoding:"utf8"}).stdout.split("\n").filter(Boolean);expect(listed.length).toBe(fixture.census.expected.entries);}
   const wide=join(directory,"wide");
   lay(wide,Array.from({length:300},(_,index)=>({path:`cas/${index.toString(16).padStart(4,"0")}/entry`,bytes:2})));
   const large=await measureBazelDiskCacheV1(wide,{phaseControl:control,progress:async stage=>{calls.push(stage);}});
   expect(large).toEqual({available:true,entries:300,bytes:600});
   expect(calls.length).toBeGreaterThanOrEqual(2);
   expect(new Set(calls)).toEqual(new Set(["measuring"]));
   let seen=0;
   await expect(measureBazelDiskCacheV1(wide,{phaseControl:control,progress:async()=>{},check:()=>{if(++seen>4)throw Error("cancelled");}})).rejects.toThrow("cancelled");
  });
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("clearing the disk cache empties ac and cas, keeps tmp and gc, unlinks links without following them and stops when cancelled",async()=>{
 const directory=temporary(),outside=temporary();
 try{
  lay(directory,fixture.census.files);
  lay(directory,Array.from({length:300},(_,index)=>({path:`cas/${index.toString(16).padStart(4,"0")}/entry`,bytes:2})));
  writeFileSync(join(outside,"sentinel"),"keep");
  if(posix){symlinkSync(join(outside,"sentinel"),join(directory,"ac/5c/link"));symlinkSync(outside,join(directory,"cas/folder-link"));chmodSync(join(directory,"ac/5c/5ce4a9"),0o444);chmodSync(join(directory,"ac/5c"),0o555);}
  await withControl(async control=>{
   const stages:string[]=[];
   let seen=0;
   await expect(clearBazelDiskCacheV1(directory,{phaseControl:control,progress:async stage=>{stages.push(stage);},check:()=>{if(++seen>40)throw Error("cancelled");}})).rejects.toThrow("cancelled");
   const partial=await measureBazelDiskCacheV1(directory,{phaseControl:control,progress:async()=>{}});
   expect(partial.entries).toBeGreaterThan(0);
   expect(partial.entries).toBeLessThan(fixture.census.expected.entries+300+(posix?2:0));
   const removed=await clearBazelDiskCacheV1(directory,{phaseControl:control,progress:async stage=>{stages.push(stage);}});
   expect(removed).toBeGreaterThan(0);
   expect(new Set(stages)).toEqual(new Set(["clearing"]));
   expect(await measureBazelDiskCacheV1(directory,{phaseControl:control,progress:async()=>{}})).toEqual({available:true,entries:0,bytes:0});
   expect(readdirSync(join(directory,"ac"))).toEqual([]);
   expect(readdirSync(join(directory,"cas"))).toEqual([]);
   expect(existsSync(join(directory,"tmp/pending"))&&existsSync(join(directory,"gc/lock"))).toBe(true);
   expect(readFileSync(join(outside,"sentinel"),"utf8")).toBe("keep");
  });
 }finally{if(posix)try{chmodSync(join(directory,"ac/5c"),0o755);}catch{}rmSync(directory,{recursive:true,force:true});rmSync(outside,{recursive:true,force:true});}
});

test("clearing the server bases stops a server that exits when its pid file vanishes, then removes only output bases, the ledger and event streams",async()=>{
 const directory=temporary(),outside=temporary(),base=join(directory,"0123456789abcdef");
 try{
  lay(directory,[{path:"0123456789abcdef/execroot/_main/bazel-out/x",bytes:3},{path:"0123456789abcdef/action_cache",bytes:3},{path:"0123456789abcdef/server/server.pid.txt",bytes:0},{path:"fedcba9876543210/server/other",bytes:1},{path:"not-a-base/kept",bytes:1},{path:"metrics.json",bytes:2},{path:"0123456789abcdef.events.json",bytes:2}]);
  writeFileSync(join(outside,"sentinel"),"keep");
  const watcher=spawn(process.execPath,["-e","const f=process.argv[1];setInterval(()=>{if(!require('fs').existsSync(f))process.exit(0)},50)",join(base,"server/server.pid.txt")],{stdio:"ignore"});
  const exited=new Promise<number|null>(resolve=>watcher.on("exit",code=>resolve(code)));
  writeFileSync(join(base,"server/server.pid.txt"),String(watcher.pid));
  if(posix){symlinkSync(outside,join(base,"execroot/_main/s"));chmodSync(join(base,"execroot/_main/bazel-out/x"),0o444);chmodSync(join(base,"execroot/_main/bazel-out"),0o555);chmodSync(join(base,"execroot/_main"),0o555);}
  await withControl(async control=>{
   const removed=await clearBazelServerBasesV1(directory,{phaseControl:control,progress:async()=>{}});
   expect(removed).toBeGreaterThan(5);
   expect(await Promise.race([exited,new Promise(resolve=>setTimeout(()=>resolve("running"),5000))])).toBe(0);
   expect(readdirSync(directory).sort()).toEqual(["not-a-base"]);
   expect(readFileSync(join(outside,"sentinel"),"utf8")).toBe("keep");
   expect(await clearBazelServerBasesV1(join(directory,"absent"),{phaseControl:control,progress:async()=>{}})).toBe(0);
  });
  watcher.kill();
 }finally{if(posix){for(const path of ["execroot/_main","execroot/_main/bazel-out"])try{chmodSync(join(base,path),0o755);}catch{}}rmSync(directory,{recursive:true,force:true});rmSync(outside,{recursive:true,force:true});}
});

test("recorded build metrics add up in the ledger, consume their event stream and survive a malformed ledger",async()=>{
 const directory=temporary(),workspace=join(directory,"workspace"),layout=bazelCacheLayoutV1({tools:join(directory,"t"),dependencies:join(directory,"d"),products:join(directory,"p")});
 try{
  mkdirSync(layout.server,{recursive:true});
  const events=bazelBuildEventsFileV1(layout,workspace),event=(name:string)=>JSON.stringify(fixture.metrics.find(row=>row.name===name)!.event)+"\n";
  expect(events.length).toBeLessThanOrEqual(256);
  expect(dirname(events)).toBe(layout.server);
  await withControl(async control=>{
   expect(await readBazelLedgerV1(layout)).toEqual({builds:0,actions:0,executed:0,cacheHits:0});
   writeFileSync(events,event("cold"));
   await recordBazelBuildMetricsV1(directory,layout,workspace,control);
   expect(existsSync(events)).toBe(false);
   writeFileSync(events,event("disk-cache-restore"));
   await recordBazelBuildMetricsV1(directory,layout,workspace,control);
   const {version,...totals}=fixture.ledger.expected;
   expect(version).toBe(1);
   expect(await readBazelLedgerV1(layout)).toEqual(totals);
   writeFileSync(events,"{\"id\":{\"started\":{}}}\n");
   await recordBazelBuildMetricsV1(directory,layout,workspace,control);
   expect(await readBazelLedgerV1(layout)).toEqual(totals);
   writeFileSync(join(layout.server,"metrics.json"),"{");
   expect(await readBazelLedgerV1(layout)).toEqual({builds:0,actions:0,executed:0,cacheHits:0});
   writeFileSync(events,event("up-to-date"));
   await recordBazelBuildMetricsV1(directory,layout,workspace,control);
   expect(await readBazelLedgerV1(layout)).toEqual({builds:1,actions:1,executed:0,cacheHits:0});
  });
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("Bazel's affected projects and dry-run graphs equal Nx's own for every fixture case, and equal the hand-authored expectation",async()=>{
 const oracle=nxOracle();
 await withControl(async(_control,original)=>{
  for(const row of fixture.cases){
   const graph=toGraph(row),nx=oracle[row.name]!,affected=affectedBuildProjectsV1(graph,row.files);
   expect(affected,row.name).toEqual(row.expected.affected);
   expect(nx.affected,"nx "+row.name).toEqual(row.expected.affected);
   const args=["affected",...row.targets.map(target=>"--targets="+target),...(row.files.length?["--files="+row.files.join(",")]:[]),...("excludeDependencies"in row&&row.excludeDependencies?["--exclude-task-dependencies"]:[])];
   const request=row.files.length?await selectBuildTasksV1(root,graph,args,original):{tasks:affected.flatMap(project=>row.targets.filter(target=>graph.projects[project]!.targets[target]).map(target=>project+":"+target)),excludeDependencies:"excludeDependencies"in row&&row.excludeDependencies===true};
   expect([...new Set(request.tasks.map(task=>task.split(":")[0]!))].sort(),row.name).toEqual(affected.filter(project=>row.targets.some(target=>graph.projects[project]!.targets[target])));
   const plan=bazelDryRunPlanV1(request.tasks.length?planBazelTasksV1(graph,{tasks:request.tasks,excludeDependencies:request.excludeDependencies}):bazelEmptyPlanV1),printed=JSON.parse(JSON.stringify(plan));
   expect(dryRun(printed),row.name+JSON.stringify(dryRun.errors)).toBe(true);
   expect(normalized(plan.tasks),row.name).toEqual(expectedGraph(row));
   expect(normalized(nx.graph),"nx "+row.name).toEqual(expectedGraph(row));
   expect(Object.keys(plan.tasks.tasks).sort(),row.name).toEqual(Object.keys(nx.graph.tasks).sort());
   for(const [id,task] of Object.entries(nx.graph.tasks))expect(plan.tasks.tasks[id]!.target,id).toEqual({project:task.target.project,target:task.target.target});
   expect(dryRun({tasks:nx.graph}),"nx "+row.name).toBe(true);
  }
 });
},120_000);

test("exec plans one repo-exec task per project exactly like the neutral plan, exports the neutral project environment and runs in the project root",async()=>{
 const graph:BuildGraphV1={version:1,projects:{one:{name:"one",root:"packages/one",targets:{}},two:{name:"two",root:"packages/two",targets:{}},three:{name:"three",root:"other/three",targets:{}}},dependencies:[]};
 await withControl(async(_control,original)=>{
  const request=await selectBuildTasksV1(root,graph,["exec","--projects=one,two","--","echo","hello world"],original),plan=bazelDryRunPlanV1(planBazelTasksV1(graph,request));
  expect(request.tasks).toEqual(["one:repo-exec","two:repo-exec"]);
  expect(plan.tasks.tasks).toEqual(buildExecPlanV1(["one","two"]).tasks.tasks);
  expect(normalized(plan.tasks)).toEqual(normalized(buildExecPlanV1(["one","two"]).tasks));
  expect(dryRun(JSON.parse(JSON.stringify(plan)))).toBe(true);
  const task=planBazelTasksV1(graph,request).tasks["two:repo-exec"] as BazelTaskV1,environment=bazelTaskBaseEnvironmentV1("/workspace",task,{KEEP:"1"});
  expect(task.definition.execution).toMatchObject({program:"echo",args:["hello world"],cwd:"packages/two"});
  expect(task.definition.cache).toBe(false);
  expect(environment).toMatchObject({KEEP:"1",SEMIO_PROJECT_NAME:"two",SEMIO_PROJECT_ROOT:"packages/two"});
  expect(execEnvironment(buildExecEnvironmentV1("two","packages/two"))).toBe(true);
  expect(execEnvironment({SEMIO_PROJECT_NAME:"two"})).toBe(false);
  const build=planBazelTasksV1({...graph,projects:{...graph.projects,one:{...graph.projects.one!,targets:{build:{execution:{kind:"noop"}}}}}},{tasks:["one:build"]}).tasks["one:build"] as BazelTaskV1;
  expect(Object.keys(bazelTaskBaseEnvironmentV1("/workspace",build,{}))).not.toContain("SEMIO_PROJECT_NAME");
 });
 const source=readFileSync(installed.resolve("nx/src/command-line/exec/exec.js"),"utf8");
 expect(source).toMatch(/NX_PROJECT_NAME:[^,]*\.name/u);
 expect(source).toMatch(/NX_PROJECT_ROOT_PATH:[^,]*\.data\?\.root/u);
});

test("affected refuses --projects for every tool and selects nothing without succeeding differently",async()=>{
 const {validateBuildTaskArgumentsV1}=await import("../../../../🧭️selection/🟦️.ts");
 for(const args of [["affected","--targets=build","--projects=app"],["affected","--targets=build","-p","app"],["affected","--targets=build","-p=app"]])expect(()=>validateBuildTaskArgumentsV1(args),args.join(" ")).toThrow("Affected cannot be narrowed with --projects");
 expect(()=>validateBuildTaskArgumentsV1(["affected","--targets=build","--files=a","--","--projects=literal"])).not.toThrow();
 expect(()=>validateBuildTaskArgumentsV1(["run-many","--targets=build","--projects=app"])).not.toThrow();
 expect(bazelEmptyPlanV1.selected).toEqual([]);
 expect(Object.keys(bazelDryRunPlanV1(bazelEmptyPlanV1).tasks.tasks)).toEqual([]);
});

test("the Bazel tool registers itself in the neutral catalogue with a status and a clear",()=>{
 const tool=bazelBuildCacheToolV1();
 expect(tool.tool).toBe("bazel");
 expect(typeof tool.status).toBe("function");
 expect(typeof tool.clear).toBe("function");
});

const binary=process.env.SEMIO_TEST_BAZEL_BINARY;

/** 🏗️ Runs the pinned Bazel binary in a scratch workspace whose genrules form a chain, so the cache numbers come from the real thing. */
function bazel(workspace:string,state:string,args:readonly string[]){
 const run=spawnSync(binary!,["--nosystem_rc","--nohome_rc",`--output_user_root=${join(state,"user")}`,`--install_base=${join(state,"install")}`,`--output_base=${join(state,"0123456789abcdef")}`,"--max_idle_secs=120",...args],{cwd:workspace,encoding:"utf8",maxBuffer:64*1024*1024,timeout:540_000});
 if(run.status!==0)throw Error(`bazel ${args.join(" ")} failed: ${run.stdout}${run.stderr}`);
 return run.stdout+run.stderr;
}

test.skipIf(binary===undefined)("real Bazel: the census, the build metrics and the clear follow a cold build, a disk cache restore and a fresh miss",async()=>{
 const scratch=temporary(),workspace=join(scratch,"ws"),state=join(scratch,"state"),actions=join(scratch,"actions");
 try{
  mkdirSync(workspace,{recursive:true});mkdirSync(state,{recursive:true});
  writeFileSync(join(workspace,"MODULE.bazel"),'module(name = "scratch", version = "0.0.0")\n');
  writeFileSync(join(workspace,"BUILD"),'genrule(name="a", outs=["a.txt"], cmd="echo a > $@")\ngenrule(name="b", srcs=[":a"], outs=["b.txt"], cmd="cat $(SRCS) > $@; echo b >> $@")\ngenrule(name="c", srcs=[":b"], outs=["c.txt"], cmd="cat $(SRCS) > $@; echo c >> $@")\n');
  const events=join(scratch,"events.json"),build=()=>bazel(workspace,state,["build","--lockfile_mode=off",`--disk_cache=${actions}`,`--build_event_json_file=${events}`,"//:c"]);
  await withControl(async control=>{
   const walk={phaseControl:control,progress:async()=>{}},metrics=()=>bazelBuildMetricsV1(readFileSync(events,"utf8"))!;
   build();
   const cold=metrics(),census=await measureBazelDiskCacheV1(actions,walk);
   expect(cold.executed).toBe(3);expect(cold.cacheHits).toBe(0);
   expect(census.entries).toBeGreaterThan(0);
   if(posix){const listed=spawnSync("find",["ac","cas","-type","f"],{cwd:actions,encoding:"utf8"}).stdout.split("\n").filter(Boolean);expect(census.entries).toBe(listed.length);}
   bazel(workspace,state,["clean"]);build();
   const restored=metrics();
   expect(restored.executed).toBe(0);expect(restored.cacheHits).toBe(3);
   expect(await measureBazelDiskCacheV1(actions,walk)).toEqual(census);
   const pid=Number(readFileSync(join(state,"0123456789abcdef/server/server.pid.txt"),"utf8"));
   expect(()=>process.kill(pid,0)).not.toThrow();
   await clearBazelServerBasesV1(state,walk);
   expect(()=>process.kill(pid,0)).toThrow();
   expect(existsSync(join(state,"0123456789abcdef"))).toBe(false);
   expect(existsSync(join(state,"install"))).toBe(true);
   await clearBazelDiskCacheV1(actions,walk);
   expect(await measureBazelDiskCacheV1(actions,walk)).toEqual({available:true,entries:0,bytes:0});
   build();
   const missed=metrics();
   expect(missed.executed).toBe(3);expect(missed.cacheHits).toBe(0);
   const regrown=await measureBazelDiskCacheV1(actions,walk);
   expect(regrown.entries).toBe(census.entries);
   expect(regrown.bytes).toBeGreaterThan(0);
  });
 }finally{rmSync(scratch,{recursive:true,force:true});}
},600_000);

test.skipIf(binary===undefined)("real Bazel: rdeps over a package per project returns the affected projects of every graph-shaped fixture case",()=>{
 const scratch=temporary(),state=join(scratch,"state");
 try{
  for(const row of fixture.cases.filter(row=>row.bazelQuery&&row.files.length)){
   const workspace=join(scratch,"w");
   for(const name of existsSync(workspace)?readdirSync(workspace):[])rmSync(join(workspace,name),{recursive:true,force:true});
   mkdirSync(workspace,{recursive:true});mkdirSync(state,{recursive:true});
   writeFileSync(join(workspace,"MODULE.bazel"),'module(name = "scratch", version = "0.0.0")\n');
   const roots=Object.entries(row.projects).map(([name,project])=>({name,root:project.root})),label=(root:string,name:string)=>`//${root==="."?"":root}:${name}`,found=new Map<string,string>(roots.map(project=>[label(project.root,project.name),project.name]));
   writeFileSync(join(workspace,"BUILD"),'exports_files(glob(["**/*"]))\n');
   for(const project of roots){
    const data=row.edges.filter(([source])=>source===project.name).map(([,target])=>label(Object.values(row.projects).find((_,index)=>Object.keys(row.projects)[index]===target)!.root,target!));
    const directory=project.root==="."?workspace:join(workspace,project.root);mkdirSync(directory,{recursive:true});
    writeFileSync(join(directory,"BUILD"),`filegroup(name = ${JSON.stringify(project.name)}, srcs = glob(["**/*"]), data = ${JSON.stringify(data)}, visibility = ["//visibility:public"])\n`);
   }
   const ownerRoot=(file:string)=>roots.map(project=>project.root).filter(root=>root!=="."&&file.startsWith(root+"/")).sort((a,b)=>b.length-a.length)[0]??"",targets=row.files.map(file=>{const owner=ownerRoot(file),path=join(workspace,file);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,"x");return `//${owner}:${owner?file.slice(owner.length+1):file}`;});
   const output=bazel(workspace,state,["query","--lockfile_mode=off",`rdeps(//..., set(${targets.join(" ")}))`,"--output=label"]),affected=output.split("\n").map(line=>found.get(line.trim())).filter((name):name is string=>name!==undefined).sort();
   expect(affected,row.name).toEqual(row.expected.affected);
  }
 }finally{rmSync(scratch,{recursive:true,force:true});}
},600_000);
