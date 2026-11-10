import type {BuildGraphV1} from "../📋️model/🟦️.ts";
import {refuseBuildV1} from "../../📖️texts/🟦️.ts";
import {resolve} from "node:path";
import {GRAPH_MEMORY_POLICY_V1,graphStringInternerV1,createMemoryPortsV1} from "../../🧠️memory/🟨️.mjs";


export const inventoryDirectoriesV1=new WeakMap<string[],string[]>();


/** 🧊️ The expanded input lists of closure inventories this process already read, by repository and content address: a content addressed leaf expands to the same list in every chunk, so a chunked expansion reads each distinct closure once. Bounded by the declared entry count; over it the table starts again. */
export const expandedClosuresV1=new Map<string,string[]>();
let expandedClosureEntriesV1=0;

export function rememberExpandedClosureV1(key:string,list:string[]):void{
  if(expandedClosureEntriesV1+list.length>GRAPH_MEMORY_POLICY_V1.maximumCachedClosureEntries){expandedClosuresV1.clear();expandedClosureEntriesV1=0;}
  expandedClosuresV1.set(key,list);expandedClosureEntriesV1+=list.length;
}


const releasedBuildGraphsV1=new WeakSet<object>(),buildGraphContentKeysV1=["projects","dependencies","namedInputs","externalNodes","unavailable","admission"] as const;


/** 🕊️ Drops the content of graphs whose planning is done and the caches built to expand them, then collects once the event loop turned (a collection inside the releasing call still sees the caller's stack), so a long build phase holds only what it plans from: every later read of a released graph throws instead of silently seeing nothing. Callers hold the shell, never the content; a consumer that needs the graph again reads the pinned snapshot. */
export async function releaseBuildGraphV1(...graphs:readonly BuildGraphV1[]):Promise<void>{
  for(const graph of graphs){
    if(releasedBuildGraphsV1.has(graph))continue;
    releasedBuildGraphsV1.add(graph);
    for(const key of buildGraphContentKeysV1)Object.defineProperty(graph,key,{configurable:true,enumerable:false,get(){throw refuseBuildV1("stale","graph.released-read");}});
  }
  expandedClosuresV1.clear();expandedClosureEntriesV1=0;graphStringInternerV1.clear();
  const memory=createMemoryPortsV1();await new Promise<void>(resolve=>setTimeout(resolve,0));memory.collect();
}


/** 🔨️ Refuses a graph whose content was released. */
export function requireLiveBuildGraphV1(graph:BuildGraphV1):BuildGraphV1{
  if(releasedBuildGraphsV1.has(graph))throw refuseBuildV1("stale","graph.released-read");
  return graph;
}
