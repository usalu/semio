import {createRequire} from "node:module";
import {dirname} from "node:path";
import type {SelectedTaskPlannerV1} from "../../../🕸️graph/🔏️seal/🎯️selection/📥️scope/🟦️.ts";
import policy from "./🔣️policy.json" with {type:"json"};

const admitted:ReadonlySet<string>=new Set(policy.modules);

/** 🚪️ The only typed doors into the pinned Nx SDK; repository code outside the Nx implementation calls these instead of loading Nx modules. */
export interface NxSdkV1 {
 readonly packageDirectory:()=>string;
 readonly cli:()=>string;
 readonly workspaceRoot:()=>string;
 readonly readCachedProjectGraph:()=>any;
 readonly readSourceMapsCache:()=>any;
 readonly authorizers:()=>Promise<any>;
 readonly daemonClient:()=>any;
 readonly daemonEnvironment:(environment:Readonly<Record<string,string|undefined>>)=>Record<string,string|undefined>;
 readonly native:()=>any;
 readonly socketTransport:()=>any;
 readonly socketFraming:()=>any;
 readonly taskPlanning:()=>Readonly<{createTaskGraph:(...arguments_:any[])=>any;getInputs:(...arguments_:any[])=>any}>;
}

/** 🔌️ Resolves one closed SDK module catalogue relative to the file or package that pins the actual Nx installation. */
export function openNxSdkV1(base:string):NxSdkV1{
 const sdk=createRequire(base),load=(name:string):any=>{if(!admitted.has(name))throw Error("Closed Nx SDK module required: "+name);return sdk(name);},locate=(name:string):string=>{if(!admitted.has(name))throw Error("Closed Nx SDK module required: "+name);return sdk.resolve(name);};
 return Object.freeze({
  packageDirectory:()=>dirname(locate("nx/package.json")),
  cli:()=>locate("nx/bin/nx.js"),
  workspaceRoot:()=>load("nx/src/utils/workspace-root").workspaceRoot,
  readCachedProjectGraph:()=>load("@nx/devkit").readCachedProjectGraph(),
  readSourceMapsCache:()=>load("nx/src/project-graph/nx-deps-cache").readSourceMapsCache(),
  authorizers:()=>load("nx/src/utils/workspace-context").loadSemioGraphAuthorizersV1(),
  daemonClient:()=>load("nx/src/daemon/client/client").daemonClient,
  daemonEnvironment:(environment:Readonly<Record<string,string|undefined>>)=>load("nx/src/daemon/client/daemon-environment").getDaemonEnv(environment),
  native:()=>load("nx/src/native"),
  socketTransport:()=>load("nx/src/utils/consume-messages-from-socket"),
  socketFraming:()=>load("nx/src/daemon/socket-utils"),
  taskPlanning:()=>Object.freeze({createTaskGraph:load("nx/src/tasks-runner/create-task-graph").createTaskGraph,getInputs:load("nx/src/hasher/task-hasher").getInputs})
 });
}

/** 🎯️ Restricts actual pinned SDK values to owned task and input contracts. */
export function nativeSelectedTaskPlannerV1(implementationFile:string):SelectedTaskPlannerV1{
 const {createTaskGraph,getInputs}=openNxSdkV1(implementationFile).taskPlanning();
 return {
  tasks(graph:any,selection,nx:any){const planned=createTaskGraph(graph,nx.targetDefaults??{},[selection.project],[selection.target],selection.configuration,{},selection.excludeTaskDependencies);return Object.values(planned.tasks).map((task:any)=>({id:task.id,project:task.target.project,target:task.target.target}));},
  inputs(graph:any,project,target,nx,input){let current=graph,name=target;if(input){name="__semio_selected_named_input__";const node=graph.nodes[project];current={...graph,nodes:{...graph.nodes,[project]:{...node,data:{...node.data,targets:{...node.data.targets,[name]:{inputs:[input]}}}}}};}const expanded=getInputs({target:{project,target:name}},current,nx);return {self:expanded.selfInputs,dependencies:expanded.depsInputs.map((row:any)=>row.input),projectInputs:expanded.projectInputs.length,dependencyFilesets:expanded.depsFilesets.map((row:any)=>row.fileset)};}
 };
}
