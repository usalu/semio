/** 🧭️ Topology reads retained graph queries and propagates intrinsic owner refusal. */
import type {JackArtifact} from "../../🟦️.ts";
import {retainedJackGraph,type JackGraphQuery} from "../🪆️graph-query/🟦️.ts";
import {ValueError} from "@semio-tech/framework/value/refusal";
import type {JackTopology} from "../🟦️.ts";
export type {JackTopology};
/** 🪆️ Child ownership and endpoint validity precede Kahn traversal. */
export function inferJackTopology(parent:JackArtifact,children:ReadonlyMap<string,JackGraphQuery>):JackTopology {
 const child=retainedJackGraph(parent,children);
 const nodes=new Map<string,{degree:number;children:string[]}>();
 for(const node of child.nodeIds()){if(nodes.has(node))throw new ValueError("invalidValue","Jack inference requires unique node identities");nodes.set(node,{degree:0,children:[]});}
 const edgeIds=new Set<string>();
 for(const edge of child.edges()){if(edgeIds.has(edge.id))throw new ValueError("invalidValue","Jack inference requires unique edge identities");edgeIds.add(edge.id);const source=nodes.get(edge.source),target=nodes.get(edge.target);if(!source||!target)throw new ValueError("invalidValue","Jack inference edge requires retained endpoints");source.children.push(edge.target);target.degree++;}
 const queue=[...nodes.keys()].filter(key=>nodes.get(key)!.degree===0).sort(),depth:Record<string,number>=Object.create(null),topoOrder:string[]=[];
 for(const key of queue)depth[key]=0;
 for(let index=0;index<queue.length;index++){const key=queue[index]!,node=nodes.get(key)!;topoOrder.push(key);const newlyZero:string[]=[];for(const child of node.children){const target=nodes.get(child)!;depth[child]=Math.max(depth[child]??0,depth[key]!+1);target.degree--;if(target.degree===0)newlyZero.push(child);}newlyZero.sort();queue.push(...newlyZero);}
 const visited=new Set(topoOrder);for(const key of Object.keys(depth))if(!visited.has(key))delete depth[key];
 return{topoOrder,depth,cycleFree:topoOrder.length===nodes.size,nodeCount:nodes.size};
}
