/** 🛰️ Contract: an invocation is a client of the per-host graph service. Language-agnostic JSON fixtures decide what a reply means as a scoped plan and which commands ask the service; an independent Python oracle recomputes the plans; Ajv (third party) validates every scoped plan against the published schema; a real service over a socket answers a real client, whose handed-over slice is verified by digest. */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import scopeSchema from "../../../🧭️selection/🎯️scope/🧬️schema/🔣️.json" with {type:"json"};
import {createHostServiceV1} from "../../../🏡️resident/🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../../🏡️resident/🔌️ports/🧭️plan/🟨️.mjs";
import {connectHostV1,requestPlanV1} from "../../../🏡️resident/🚪️client/🛰️plan/🟨️.mjs";
import {hostEndpointsV1} from "../../../🏡️resident/📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {HOST_POLICY_V1} from "../../../🏡️resident/📨️protocol/🎚️policy/🟨️.mjs";
import {appendFileSync} from "node:fs";
import {recordPlanSuccessV1} from "../../🏃️run/🟦️.ts";
import {createCasStoreV1} from "../../../🗄️cas/🗃️store/🟦️.ts";
import {createTaskRecorderV1} from "../../../🕸️graph/🧮️derived-nodes/🎯️task-state/🟨️.mjs";
import {workspaceIdV1} from "../../../🏡️resident/📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {PLAN_HANDOFF_V1,invocationPlanRequestV1,planHandoffEnvironmentV1,readSliceV1,requestInvocationPlanV1,scopedPlanOfReplyV1,takePlannedGraphV1} from "../../🛰️plan/🟦️.ts";

const here=fileURLToPath(new URL(".",import.meta.url)),root=realpathSync(fileURLToPath(new URL("../../../../../../../../../",import.meta.url))),fixture=JSON.parse(readFileSync(join(here,"🔣️.json"),"utf8"));
const base=mkdtempSync(join(tmpdir(),"semio-client-plan-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const pair=(id:string)=>{const split=id.lastIndexOf(":");return{project:id.slice(0,split),target:id.slice(split+1)};};

test("a reply means the scoped plan the fixture and the independent Python oracle state",()=>{
 const oracle=JSON.parse(spawnSync(process.env.SEMIO_PYTHON??"python3",["-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")],{encoding:"utf8"}).stdout) as unknown[];
 const ajv=new Ajv({strict:false}),validate=ajv.compile({...scopeSchema,$ref:"#/definitions/ScopedPlan"});
 fixture.scoped.forEach((row:any,index:number)=>{
  expect(oracle[index],row.name).toEqual(row.expected);
  const request={arguments:["run"],tool:"nx",targets:row.targets};
  if(row.expected===null){expect(()=>scopedPlanOfReplyV1(request,row.tasks),row.name).toThrow("outside the plan");return;}
  const plan=scopedPlanOfReplyV1(request,row.tasks);
  expect(validate(plan),JSON.stringify(validate.errors)).toBe(true);
  expect(plan.projects).toEqual(row.expected.projects);
  expect(plan.tasks.map((task:any)=>task.project+":"+task.target)).toEqual(row.expected.tasks);
  expect(plan.edges.map((edge:any)=>[edge.from.project+":"+edge.from.target,edge.to.project+":"+edge.to.target])).toEqual(row.expected.edges);
  if(row.targets.includes("*:*"))expect(plan.scope.targets).toEqual([pair(row.expected.tasks[0])]);else expect(plan.scope.targets).toEqual(row.targets.map(pair));
 });
});

test("the commands that plan a task graph ask the service, nothing else does",async()=>{
 for(const row of fixture.requests){
  const request=await invocationPlanRequestV1(root,row.args,{SEMIO_BUILD_TOOL:"nx"});
  if(row.expected===null)expect(request,row.args.join(" ")).toBeUndefined();
  else expect(JSON.parse(JSON.stringify(request)),row.args.join(" ")).toMatchObject(row.expected);
 }
 expect(await invocationPlanRequestV1(root,["run","repo-task-scope:test"],{SEMIO_BUILD_REUSE_GRAPH:"true"})).toBeUndefined();
 expect(await invocationPlanRequestV1(root,["run","repo-task-scope:test"],{[PLAN_HANDOFF_V1]:"{}"})).toBeUndefined();
});

const graphOf=(names:readonly string[])=>({version:1,projects:Object.fromEntries(names.map(name=>[name,{name,root:"p/"+name,targets:{build:{execution:{kind:"noop"},dependsOn:names.indexOf(name)>0?["^build"]:[]}}}])),dependencies:names.slice(1).map((name,index)=>({source:name,target:names[index]!}))});

test("a real client gets its plan and verified graph slice from a real service, and a corrupted slice is refused",async()=>{
 const runtimeDirectory=join(base,"run");mkdirSync(runtimeDirectory,{recursive:true});
 const environment={...process.env,XDG_RUNTIME_DIR:runtimeDirectory,TMPDIR:runtimeDirectory,LOCALAPPDATA:runtimeDirectory,SEMIO_BUILD_TOOL:"nx"} as Record<string,string>,implementation="a".repeat(64),runtime=hostEndpointsV1(implementation,environment);
 const loads:string[][]=[];
 const planner=createGraphPlannerPortV1({loadSlice:async({arguments:invocation})=>{loads.push([...invocation]);return{graph:graphOf(["core","util","app"]),generationPin:"generation-1"};}});
 const service=createHostServiceV1({runtime,implementation,ports:{planner}});
 await service.start();
 try{
  const progress:string[]=[];
  const plan=await requestInvocationPlanV1({root,request:{arguments:["run","app:build"],tool:"nx",targets:["app:build"]},environment,agent:"agent-1",write:line=>progress.push(line),ports:{requestPlanV1:options=>requestPlanV1({...options,implementation})}});
  expect(plan.scoped.tasks.map(task=>task.project)).toEqual(["core","util","app"]);
  expect(plan.scoped.edges).toHaveLength(2);
  expect(plan.stale).toEqual(["core:build","util:build","app:build"]);
  expect(loads).toEqual([["run","app:build"]]);
  const graph=await takePlannedGraphVia(plan);
  expect(Object.keys(graph.projects)).toEqual(["core","util","app"]);
  const document=readSliceV1(plan.handoff);expect(document.generationPin).toBe("generation-1");
  const other=await requestInvocationPlanV1({root,request:{arguments:["run","app:build"],tool:"nx",targets:["app:build"]},environment,agent:"agent-2",write:()=>undefined,ports:{requestPlanV1:options=>requestPlanV1({...options,implementation})}});
  expect(other.generationKey).toBe(plan.generationKey);expect(other.handoff.hash).toBe(plan.handoff.hash);
  writeFileSync(plan.handoff.path,readFileSync(plan.handoff.path,"utf8").replace("core","c0re"));
  expect(()=>readSliceV1(plan.handoff)).toThrow("does not match its digest");
 }finally{await service.stop("test");}
});

async function takePlannedGraphVia(plan:Awaited<ReturnType<typeof requestInvocationPlanV1>>){return takePlannedGraphV1({...planHandoffEnvironmentV1(plan)});}

test("without a handed-over plan the executing process refuses, in both languages",async()=>{
 await expect(takePlannedGraphV1({})).rejects.toThrow("Ein Plan des Host-Dienstes");
});

test("the start timeout applies only to absent progress: a service that keeps writing its log is waited for, a silent one times out",async()=>{
 const directory=join(base,"slow");mkdirSync(directory,{recursive:true});
 const environment={...process.env,XDG_RUNTIME_DIR:directory,TMPDIR:directory,LOCALAPPDATA:directory} as Record<string,string>,implementation="c".repeat(64),runtime=hostEndpointsV1(implementation,environment),policy={...HOST_POLICY_V1,startTimeoutMilliseconds:200,startPollMilliseconds:20};
 const service=createHostServiceV1({runtime,implementation,ports:{planner:createGraphPlannerPortV1({loadSlice:async()=>({graph:graphOf(["a"])})})}});
 try{
  const{socket}=await connectHostV1({implementation,environment,policy,spawnService:()=>{const timer=setInterval(()=>appendFileSync(runtime.log,"booting\n"),50);setTimeout(()=>{clearInterval(timer);void service.start();},700);}});
  socket.destroy();
 }finally{await service.stop("test");}
 const silent={...environment,XDG_RUNTIME_DIR:join(directory,"silent")};mkdirSync(silent.XDG_RUNTIME_DIR!,{recursive:true});
 await expect(connectHostV1({implementation:"d".repeat(64),environment:silent,policy,spawnService:()=>undefined})).rejects.toMatchObject({reason:"timeout"});
});

test("a successful run records the keys of its plan, a dry run, a failed command class and an empty plan record nothing",async()=>{
 const workspace=mkdtempSync(join(base,"ws-")),cas=createCasStoreV1({root:join(base,"cas-record")}),keys={"a:build":"a".repeat(64),"b:build":"b".repeat(64)},recorder=createTaskRecorderV1({directory:join(workspace,".cache/b/g/m/task-keys"),cas}),ask=()=>recorder.upToDate(workspaceIdV1(workspace),new Map(Object.entries(keys)));
 expect(await recordPlanSuccessV1({root:workspace,args:["run","a:build","--dry-run"],keys,cas})).toBe(0);expect(await recordPlanSuccessV1({root:workspace,args:["exec","a"],keys,cas})).toBe(0);expect(await recordPlanSuccessV1({root:workspace,args:["run","a:build"],keys:{},cas})).toBe(0);
 expect([...await ask()]).toEqual([]);
 expect(await recordPlanSuccessV1({root:workspace,args:["run","a:build"],keys,cas})).toBe(2);
 expect([...await ask()].sort()).toEqual(["a:build","b:build"]);
});
