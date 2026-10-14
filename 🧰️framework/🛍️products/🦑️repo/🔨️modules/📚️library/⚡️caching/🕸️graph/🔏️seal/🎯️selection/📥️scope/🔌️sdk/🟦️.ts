import {createRequire} from "node:module";
import type {SelectedTaskPlannerV1} from "../🟦️.ts";

/** 🔌️ Restricts actual pinned SDK values to owned task and input contracts. */
export function nativeSelectedTaskPlannerV1(implementationFile:string):SelectedTaskPlannerV1{
 const require=createRequire(implementationFile),{createTaskGraph}=require("nx/src/tasks-runner/create-task-graph"),{getInputs}=require("nx/src/hasher/task-hasher");
 return {
  tasks(graph:any,selection,nx:any){const planned=createTaskGraph(graph,nx.targetDefaults??{},[selection.project],[selection.target],selection.configuration,{},selection.excludeTaskDependencies);return Object.values(planned.tasks).map((task:any)=>({id:task.id,project:task.target.project,target:task.target.target}));},
  inputs(graph:any,project,target,nx,input){let current=graph,name=target;if(input){name="__semio_selected_named_input__";const node=graph.nodes[project];current={...graph,nodes:{...graph.nodes,[project]:{...node,data:{...node.data,targets:{...node.data.targets,[name]:{inputs:[input]}}}}}};}const expanded=getInputs({target:{project,target:name}},current,nx);return {self:expanded.selfInputs,dependencies:expanded.depsInputs.map((row:any)=>row.input),projectInputs:expanded.projectInputs.length,dependencyFilesets:expanded.depsFilesets.map((row:any)=>row.fileset)};}
 };
}
