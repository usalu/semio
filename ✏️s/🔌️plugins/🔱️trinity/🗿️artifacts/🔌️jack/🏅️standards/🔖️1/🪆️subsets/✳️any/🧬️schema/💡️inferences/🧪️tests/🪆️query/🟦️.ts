/** 🪆️ Portable intrinsic graph laws with an independent graphology record oracle. */
import {expect,test} from "bun:test";
import {MultiDirectedGraph} from "graphology";
import fixture from "../../🧫️fixtures/🪆️query/🔣️.json";
import * as api from "./../../../../../../../../🟦️.ts";
const topology=(api as unknown as {inferJackTopology:Function}).inferJackTopology;
const flat=(api as unknown as {inferJackFlatPosition:Function}).inferJackFlatPosition;
const parent={content:fixture.identity,rootNodeId:undefined};
function query(example:typeof fixture.cases[number],identity=fixture.identity){return{identity,nodeIds:()=>example.nodes,edges:()=>example.edges,displacement:(id:string)=>{const edge=example.edges.find(edge=>edge.id===id)!;return{u:edge.u,v:edge.v};}};}
function oracle(example:typeof fixture.cases[number]){
 const graph=new MultiDirectedGraph();for(const id of example.nodes)graph.addNode(id);for(const edge of example.edges)graph.addDirectedEdgeWithKey(edge.id,edge.source,edge.target,{u:edge.u,v:edge.v});
 const order:string[]=[],depth:Record<string,number>=Object.create(null),pending=new Set(graph.nodes());
 while(pending.size){const frontier=[...pending].filter(id=>graph.inNeighbors(id).every(source=>!pending.has(source))).sort();if(!frontier.length)break;for(const id of frontier){order.push(id);depth[id]=Math.max(0,...graph.inNeighbors(id).map(source=>depth[source]!+1));pending.delete(id);}}
 const positions:Record<string,{u:number;v:number}>=Object.create(null),remaining=new Set(graph.nodes());
 while(remaining.size){const sorted=[...remaining].sort(),seed=Object.keys(positions).length===0?sorted[0]!:(sorted.find(id=>graph.inNeighbors(id).every(source=>!remaining.has(source)))??sorted[0]!);positions[seed]={u:0,v:0};remaining.delete(seed);const stack=[seed];while(stack.length){const source=stack.pop()!;for(const edge of graph.outEdges(source).sort()){const target=graph.target(edge);if(!remaining.has(target))continue;const attributes=graph.getEdgeAttributes(edge),held=positions[source]!;positions[target]={u:held.u+attributes.u,v:held.v+attributes.v};remaining.delete(target);stack.push(target);}}}
 return{topology:{topoOrder:order,depth,cycleFree:pending.size===0,nodeCount:graph.order},positions};
}
for(const example of fixture.cases)test("Jack intrinsic retained graph query "+example.name,()=>{
 expect(typeof topology).toBe("function");expect(typeof flat).toBe("function");const before=JSON.stringify(example),thirdParty=oracle(example);expect(thirdParty).toEqual({topology:example.topology,positions:example.positions});
 for(const current of[example,{...example,nodes:[...example.nodes].reverse(),edges:[...example.edges].reverse()}]){const children=new Map([[fixture.identity.childId,query(current)]]);expect(topology(parent,children)).toEqual(example.topology);expect(flat(parent,children)).toEqual({positions:example.positions});}expect(JSON.stringify(example)).toBe(before);
});
test("Jack query refuses absent and mismatched retained identities",()=>{
 const child=query(fixture.cases[1]!);expect(()=>topology(parent,new Map())).toThrow();
 for(const identity of[{...fixture.identity,childId:"other"},{...fixture.identity,target:{...fixture.identity.target,artifactId:"other"}},...Object.keys(fixture.identity.target.dialect).map(key=>({...fixture.identity,target:{...fixture.identity.target,dialect:{...fixture.identity.target.dialect,[key]:"other"}}}))])expect(()=>topology(parent,new Map([[fixture.identity.childId,{...child,identity}]]))).toThrow();
});
test("Jack query refuses duplicated identities, lost endpoints and foreign root",()=>{
 const example=fixture.cases[1]!;for(const current of[{...example,nodes:[...example.nodes,example.nodes[0]!]},{...example,edges:[...example.edges,example.edges[0]!]},{...example,edges:[{...example.edges[0]!,source:"absent"}]},{...example,edges:[{...example.edges[0]!,target:"absent"}]}])expect(()=>topology(parent,new Map([[fixture.identity.childId,query(current)]]))).toThrow();
 expect(()=>flat({...parent,rootNodeId:"absent"},new Map([[fixture.identity.childId,query(example)]]))).toThrow();
 const children=new Map([[fixture.identity.childId,query(example)]]);expect(flat({...parent,rootNodeId:"b"},children).positions.b).toEqual({u:0,v:0});
});
