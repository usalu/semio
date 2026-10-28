/** 🎚️ Contract: the compiler mode is one invocation axis; every Bazel task accepts prototype and release without declaring them, inherits the initiating mode, carries it in its identity and is refused when it disagrees with the caller's authority. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {Database} from "bun:sqlite";
import corpus from "./🔣️.json";
import schema from "./🧬️schema.json";
import {createNativeBuildCallerCapabilitiesV1} from "../../../../📥️caller/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {renderRepoErrorV1,type RepoErrorV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";

const executor=await import("../../🎬️executor/🟦️.ts"),api=await import("../../📜️script.ts"),modeOf=(task:{definition:{execution:{env?:Record<string,string>}}})=>task.definition.execution.env?.SEMIO_BUILD_MODE??null;

test("Bazel mode fixtures are closed",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 expect(new Set(corpus.cases.map(row=>row.id)).size).toBe(corpus.cases.length);
});

for(const row of corpus.cases)test(`Bazel plans ${row.id} with the mode as a global axis`,()=>{
 const graph=corpus.graphs[row.graph as "declared"|"undeclared"|"pinned"|"partial"] as any;
 if("refused" in row.expected){
  expect(()=>api.planBazelTasksV1(graph,row.request as any)).toThrow(row.expected.refused);
  const german=(corpus.refusals as Record<string,string>)[String(row.expected.refused)]??corpus.refusals["Conflicting build modes"];
  expect(()=>api.planBazelTasksV1(graph,row.request as any)).toThrow(german);
  return;
 }
 const plan=api.planBazelTasksV1(graph,row.request as any);
 expect(plan.selected).toEqual(row.expected.selected);
 expect<unknown>(Object.fromEntries(Object.entries(plan.tasks).map(([id,task])=>[id,modeOf(task as any)]))).toEqual(row.expected.tasks);
 for(const [id,task] of Object.entries(plan.tasks)){expect(task.id).toBe(id);expect(task.label).toMatch(/^t_[a-f0-9]{16}$/u);}
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE tasks(id TEXT PRIMARY KEY,mode TEXT);CREATE TABLE edges(source TEXT,target TEXT)");
  for(const task of Object.values(plan.tasks)){database.query("INSERT INTO tasks VALUES(?,?)").run(task.id,task.configuration??null);for(const dependency of task.dependencies)database.query("INSERT INTO edges VALUES(?,?)").run(task.id,dependency);}
  const mixed=(root:string,mode:string)=>database.query("WITH RECURSIVE reach(id) AS (SELECT ?1 UNION SELECT e.target FROM edges e JOIN reach r ON e.source=r.id) SELECT count(*) AS n FROM reach JOIN tasks t ON t.id=reach.id WHERE t.mode IS NOT ?2").get(root,mode) as {n:number};
  if("inheritedMode" in row.expected)for(const root of plan.selected)expect(mixed(root,row.expected.inheritedMode!).n,row.id).toBe(0);
  const ids=(database.query("SELECT id FROM tasks ORDER BY id").all() as {id:string}[]).map(value=>value.id);
  expect(ids).toEqual(Object.keys(row.expected.tasks).sort());
 }finally{database.close();}
});

test("Bazel plans of the two modes never share a task identity, label or workspace namespace, even without declared configurations",()=>{
 for(const name of ["declared","undeclared"] as const){
  const graph=corpus.graphs[name] as any,prototype=api.planBazelTasksV1(graph,{tasks:["site:publish"],authority:"prototype"}),release=api.planBazelTasksV1(graph,{tasks:["site:publish"],authority:"release"}),namespace=(plan:{selected:string[]})=>createHash("sha256").update(JSON.stringify({selected:plan.selected,args:undefined,configuration:undefined})).digest("hex");
  const ids=[...Object.keys(prototype.tasks),...Object.keys(release.tasks)],labels=[...Object.values(prototype.tasks),...Object.values(release.tasks)].map(task=>task.label);
  expect(new Set(ids).size).toBe(ids.length);expect(new Set(labels).size).toBe(labels.length);expect(namespace(prototype)).not.toBe(namespace(release));
 }
});

for(const row of corpus.authorized)test(`Bazel request ${row.id} takes the original caller's mode as authority`,async()=>{
 const root=process.cwd(),capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","fixture:publish"],"bazel",{SEMIO_BUILD_MODE:row.caller}),graph=corpus.graphs[row.graph as "declared"|"undeclared"] as any;
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:mode-authority",maximumElapsedMilliseconds:30000},capabilities,Date.now()),async invocation=>{
  const request=api.bazelAuthorizedRequestV1({tasks:row.tasks},invocation);
  expect<unknown>(request.authority).toBe(row.caller);expect(request.tasks).toEqual(row.tasks);
  expect(api.planBazelTasksV1(graph,request).selected).toEqual(row.selected);
 });
});

test("Bazel plans of the two modes never share a task identity or label",()=>{
 for(const name of ["declared","undeclared"] as const){
  const graph=corpus.graphs[name] as any,prototype=api.planBazelTasksV1(graph,{tasks:["site:publish:prototype"]}),release=api.planBazelTasksV1(graph,{tasks:["site:publish:release"]});
  const ids=[...Object.keys(prototype.tasks),...Object.keys(release.tasks)],labels=[...Object.values(prototype.tasks),...Object.values(release.tasks)].map(task=>task.label);
  expect(new Set(ids).size).toBe(ids.length);expect(new Set(labels).size).toBe(labels.length);
 }
});

for(const row of corpus.authority)test(`Bazel task environment ${row.id}: the caller's mode is the authority and a task of another mode is refused`,async()=>{
 const root=process.cwd(),capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","fixture:publish"],"bazel",{SEMIO_BUILD_MODE:row.caller}),task={id:"fixture:publish",project:"fixture",target:"publish",root:"fixture",label:"t_0000000000000000",dependencies:[],args:[],...(row.configuration===null?{}:{configuration:row.configuration}),definition:{execution:{kind:"command",command:"repo ./📜️script.ts publish"}}} as any;
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:mode-axis",maximumElapsedMilliseconds:30000},capabilities,Date.now()),async invocation=>{
  if(row.refused){
   const refusal=(()=>{try{executor.issueBazelTaskEnvironmentV1(root,invocation,task,{});}catch(error){return error as RepoErrorV1;}throw Error("A build mode refusal is required");})();
   expect(refusal.key).toBe("build.bazel.build-modes-conflict");
   expect(renderRepoErrorV1(refusal,"en")).toContain("Conflicting build modes");
   expect(renderRepoErrorV1(refusal,"de")).toContain(corpus.refusals["Conflicting build modes"]);
   return;
  }
  expect(executor.issueBazelTaskEnvironmentV1(root,invocation,task,{}).SEMIO_BUILD_MODE).toBe(row.caller);
 });
});
