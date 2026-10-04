/** 🧭️ Topology reads the actual retained Semio child and propagates intrinsic owner refusal. */
import type {JackArtifact} from "../../🟦️.ts";
import type {SemioGraphSnapshot} from "../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🟦️.ts";
import {ValueError} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/⚠️refusal/🟦️.ts";
export interface JackTopology {topoOrder:string[];depth:Record<string,number>;cycleFree:boolean;nodeCount:number}
/** 🪆️ Child ownership and endpoint validity precede Kahn traversal. */
export function inferJackTopology(parent:JackArtifact,children:ReadonlyMap<string,SemioGraphSnapshot>):JackTopology {
 const target=parent.content.target,dialect=target.dialect;
 if(dialect.artifactKind!=="s.stdio.semio"||dialect.standard!=="v1"||dialect.subset!=="graph")throw new ValueError("invalidValue","Jack inference requires exact Semio graph child dialect");
 const child=children.get(parent.content.childId);
 if(!child)throw new ValueError("invalidValue","Jack inference requires retained owned child");
 const nodes=new Map<string,{degree:number;children:string[]}>();
 for(const node of child.nodes){if(nodes.has(node.id.value))throw new ValueError("invalidValue","Jack inference requires unique node identities");nodes.set(node.id.value,{degree:0,children:[]});}
 const edgeIds=new Set<string>();
 for(const edge of child.edges){if(edgeIds.has(edge.id.value))throw new ValueError("invalidValue","Jack inference requires unique edge identities");edgeIds.add(edge.id.value);const source=nodes.get(edge.source.value),target=nodes.get(edge.target.value);if(!source||!target)throw new ValueError("invalidValue","Jack inference edge requires retained endpoints");source.children.push(edge.target.value);target.degree++;}
 const queue=[...nodes.keys()].filter(key=>nodes.get(key)!.degree===0).sort(),depth:Record<string,number>=Object.create(null),topoOrder:string[]=[];
 for(const key of queue)depth[key]=0;
 for(let index=0;index<queue.length;index++){const key=queue[index]!,node=nodes.get(key)!;topoOrder.push(key);const newlyZero:string[]=[];for(const child of node.children){const target=nodes.get(child)!;depth[child]=Math.max(depth[child]??0,depth[key]!+1);target.degree--;if(target.degree===0)newlyZero.push(child);}newlyZero.sort();queue.push(...newlyZero);}
 for(const key of Object.keys(depth))if(!topoOrder.includes(key))delete depth[key];
 return{topoOrder,depth,cycleFree:topoOrder.length===nodes.size,nodeCount:nodes.size};
}
