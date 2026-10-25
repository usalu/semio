import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {createRequire} from "node:module";
import {join} from "node:path";
import buildPolicy from "../../../../../📦️packages/🟦️typescript/🏗️build/🔣️policy/🔣️json/🟦️.ts";
import {WORKER_ACTION_KINDS_V1,currentWorkerHostV1,planWorkersV1,readWorkerPolicyV1,workerContractFieldsV1} from "../../../../../📦️packages/🟦️typescript/🏗️build/👷️workers/🟦️.ts";
import executors from "../../../../../📦️packages/🟦️typescript/📋️executors.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {nxBatchCapableV1,nxWorkerSettingsV1} from "../🟦️.ts";

const root=join(import.meta.dir,"../../../../../../../../../..").replace(/[\\/]+$/u,""),installed=createRequire(join(root,".nx/installation/package.json"));

/** 🧬️ Applies dotted-path changes to a copy of the authored worker policy. */
function changed(changes:Readonly<Record<string,unknown>>){
 const policy=structuredClone(buildPolicy.workers) as Record<string,any>;
 for(const[path,value]of Object.entries(changes)){const parts=path.split(".");let at=policy;for(const part of parts.slice(0,-1))at=at[part];at[parts.at(-1)!]=value;}
 return readWorkerPolicyV1(policy);
}

test("the worker plan becomes the Nx job count and batch mode the installed Nx reads, and every contract field is reported once",()=>{
 for(const row of fixture.scenarios){
  const plan=planWorkersV1(changed(row.policy),row.host,row.overrides),settings=nxWorkerSettingsV1(plan,{batchCapable:row.batchCapable});
  expect<unknown>(settings.environment,row.name).toEqual(row.environment);
  expect(settings.mapping.map(entry=>entry.field).sort(),row.name).toEqual([...workerContractFieldsV1()].sort());
  expect(settings.mapping.find(entry=>entry.field==="localJobs")!.support,row.name).toBe("exact");
  for(const kind of WORKER_ACTION_KINDS_V1)expect(settings.mapping.find(entry=>entry.field===`kinds.${kind}.workers`)!.support,row.name).toBe("unsupported");
  const directory=realpathSync(mkdtempSync(join(tmpdir(),"nx-parallel-")));
  try{
   writeFileSync(join(directory,"nx.json"),"{}\n");
   var job=spawnSync("node",["-e","process.stdout.write(String(require(process.env.NX_OPTIONS).readParallelFromArgsAndEnv({})))"],{cwd:directory,encoding:"utf8",env:{...process.env,NX_OPTIONS:installed.resolve("nx/src/command-line/yargs-utils/shared-options"),NX_WORKSPACE_ROOT_PATH:directory,NX_DAEMON:"false",NX_PARALLEL:settings.environment.NX_PARALLEL}});
  }finally{rmSync(directory,{recursive:true,force:true});}
  expect(job.status,job.stderr).toBe(0);expect(Number(job.stdout),row.name).toBe(plan.localJobs);
 }
});

test("the batch variable is the one the installed Nx tests and an executor declares a batch implementation only by a string",()=>{
 const schedule=readFileSync(installed.resolve("nx/src/tasks-runner/tasks-schedule"),"utf8");
 expect(schedule.includes("process.env.NX_BATCH_MODE === 'true'")).toBe(true);
 for(const row of fixture.executors)expect(nxBatchCapableV1(row.value==="repo"?executors:row.value),row.name).toBe(row.batchCapable);
 expect(nxBatchCapableV1(undefined)).toBe(false);
 expect(JSON.stringify(executors).includes("batchImplementation")).toBe(false);
});

test("the Nx build environment takes an explicit job count first and the neutral plan otherwise, and the worker switch never changes it",async()=>{
 const{nxBuildEnvironmentV1}=await import("../../../../../📦️packages/🟦️typescript/💾️storage/🚀️bootstrap/📜️script/🟦️.ts"),planned=String(planWorkersV1(readWorkerPolicyV1(buildPolicy.workers),currentWorkerHostV1(),{}).localJobs);
 for(const[environment,expected]of [[{},planned],[{SEMIO_BUILD_PARALLEL:"5"},"5"],[{NX_PARALLEL:"3"},"3"],[{SEMIO_BUILD_PARALLEL:"5",NX_PARALLEL:"3"},"5"],[{SEMIO_BUILD_WORKERS:"off"},planned],[{SEMIO_BUILD_WORKERS:"on"},planned]] as const)expect(nxBuildEnvironmentV1(root,environment as NodeJS.ProcessEnv).NX_PARALLEL,JSON.stringify(environment)).toBe(expected);
 expect(nxBuildEnvironmentV1(root,{}).NX_BATCH_MODE).toBeUndefined();
});
