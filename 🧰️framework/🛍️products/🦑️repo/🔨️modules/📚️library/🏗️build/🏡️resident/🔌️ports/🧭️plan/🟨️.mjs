/** 🧭 The planner port of the host service: opens a fact index once per content key and answers plan requests from it. Pure over a neutral build graph; the graph loader is injected. */
import {createHash} from "node:crypto";
import {existsSync} from "node:fs";
import {join} from "node:path";
import {canonicalJsonV1} from "../../📨️protocol/🔑️capability/🟨️.mjs";
import {stateConfigurationFromBuildGraphV1} from "../🗺️graph/🟨️.mjs";
import {taskKeysV1} from "../../../🕸️graph/🧮️derived-nodes/🎯️task-state/🟨️.mjs";

/** 🚨 A requested target does not exist in the graph. */
export class UnknownTargetErrorV1 extends Error{constructor(target){super("Unknown target "+target);this.name="UnknownTargetErrorV1";this.code="unknown-target";this.target=target;}}

/** 🔢 Digest of the planner-relevant content of a graph. */
export function graphDigestV1(graph){return createHash("sha256").update(canonicalJsonV1(stateConfigurationFromBuildGraphV1(graph))).digest("hex");}

/** 🎯 The task reference that selects every task of the loaded slice (roster commands carry no literal task). */
export const ALL_TARGETS_V1="*:*";

/** 📏 The resident size of a loaded graph, estimated from at most 64 evenly spread project records and doubled for the interned copy: serialising the whole graph for its size allocated a string as large as the graph (1.2 GB of JSON for 4748 projects) on every load. */
export function estimatedGraphBytesV1(graph){
 const names=Object.keys(graph.projects??{});if(!names.length)return 0;
 const step=Math.max(1,Math.floor(names.length/64));let bytes=0,count=0;
 for(let at=0;at<names.length;at+=step){bytes+=Buffer.byteLength(JSON.stringify(graph.projects[names[at]]));count++;}
 return Math.round(bytes/count*names.length)*2;
}

/** 🧭 Builds the port. `loadGraph({root,signal})` returns the neutral graph; `identify` may return a cheap content key, else the loaded graph's digest is the key. `taskState({root,workspace})` answers `{graph,recorder}` (the workspace graph and the task recorder): the keys of the planned tasks are computed on the graph and a task whose recorded key matches is not stale, the reply carries every key. `capture({root,workspace,agent,parent,dirty,cursor,signal}, body)` runs the slice load of one plan with its reads captured and answers `{result,pinned}`: the pinned revision is the plan's pin and replaces the `pin` hook. With `loadSlice({root,workspace,arguments,tool,graphPolicy,signal})` the index is only the workspace key and each plan is computed from, and returns, the graph slice the invocation selects (`{graph,generationPin}`): the loader is the single owner of the fact index of the host. */
export function createGraphPlannerPortV1({loadGraph,loadSlice,identify,bytesOf=estimatedGraphBytesV1,stale=()=>true,pin,capture,taskState}){
 const indexOf=graph=>{
  const configuration=stateConfigurationFromBuildGraphV1(graph),tasks=new Map();
  for(const owner of configuration.owners)for(const task of owner.tasks)tasks.set(owner.name+":"+task.name,{id:owner.name+":"+task.name,dependsOn:task.dependsOn,cacheable:task.cacheable!==false,outputs:task.outputs??[],root:owner.root,target:task.name,declaration:task.command[0]});
  return tasks;
 };
 return{
  async identify({root,signal}){return identify?identify({root,signal}):undefined;},
  async load({root,workspace,signal}){
   if(loadSlice)return{key:createHash("sha256").update("workspace\0"+workspace).digest("hex"),value:{tasks:new Map()},bytes:0};
   const graph=await loadGraph({root,signal});signal?.throwIfAborted();
   const tasks=indexOf(graph);
   return{key:graphDigestV1(graph),value:{tasks},bytes:bytesOf(graph)};
  },
  async plan({index,targets,revision,parent,dirty,cursor,agent,signal,workspace,root,arguments:invocation,tool,graphPolicy,progress}){
   const sliceLoad=()=>loadSlice({root,workspace,arguments:invocation,tool,graphPolicy,signal,onProgress:progress}),captured=loadSlice&&invocation&&capture?await capture({root,workspace,agent,parent,dirty,cursor,signal},sliceLoad):undefined,slice=captured?captured.result:loadSlice&&invocation?await sliceLoad():undefined,tasks=slice?indexOf(slice.graph):index.value.tasks,order=[],seen=new Set(),visiting=new Set();
   if(slice&&targets.includes(ALL_TARGETS_V1))targets=[...tasks.keys()];
   for(const target of targets)if(!tasks.has(target))throw new UnknownTargetErrorV1(target);
   const visit=id=>{
    if(seen.has(id)||visiting.has(id))return;
    visiting.add(id);
    for(const dependency of [...tasks.get(id).dependsOn].sort())if(tasks.has(dependency))visit(dependency);
    visiting.delete(id);seen.add(id);order.push(id);
   };
   for(const target of [...targets].sort())visit(target);
   signal?.throwIfAborted();
   const pinned=captured?{revision:captured.pinned.revision.id,manifest:captured.pinned.manifest.digest,pin:captured.pinned.pin}:pin?await pin({workspace,root,agent,targets,order,parent,dirty,cursor,signal}):undefined;
   revision=pinned?.revision??revision;
   let keys,staleSet;
   if(taskState){
    const state=await taskState({root,workspace});await state.graph.verify?.({signal});keys=await taskKeysV1({graph:state.graph,tasks,order,signal});
    const fresh=await state.recorder.upToDate(workspace,new Map([...keys].filter(([id])=>tasks.get(id).cacheable)));
    staleSet=new Set(order.filter(id=>!fresh.has(id)||!tasks.get(id).outputs.every(output=>existsSync(join(root,output)))));
   }else{
    const stales=await stale({workspace,ids:order,revision,signal});
    staleSet=new Set(stales===true?order:stales||[]);
   }
   const loadStats=slice?.stats,sliceDocument=slice===undefined?undefined:(({stats,...rest})=>rest)(slice);
   return{stats:{captureFiles:captured?.pinned.manifest.entries.length??0,captureProbes:captured?.probes??0,unstaged:captured?.unstaged.length??0,...loadStats===undefined?{}:{load:loadStats}},...pinned?.manifest?{manifest:pinned.manifest}:{},...pinned?.pin?{pin:pinned.pin}:{},...sliceDocument?{slice:sliceDocument}:{},revision:revision??createHash("sha256").update(canonicalJsonV1({workspace,key:index.key,generation:slice?.generationPin??null})).digest("hex"),tasks:order.map(id=>{const task=tasks.get(id);return{id,dependsOn:task.dependsOn.filter(dep=>tasks.has(dep)).sort(),cacheable:task.cacheable,outputs:task.outputs,stale:staleSet.has(id),...keys?{key:keys.get(id)??null}:{}};})};
  },
 };
}
