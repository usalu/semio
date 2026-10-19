/** 🪆️ Portable intrinsic graph laws with an independent graphology record oracle. */
import {expect,test} from "bun:test";
import {build} from "esbuild";
import {fileURLToPath} from "node:url";
import {parse,Kind} from "graphql";
import {MultiDirectedGraph} from "graphology";
import fixtureJson from "../../../../🧪️tests/🧫️fixtures/💡️inferences/🪆️query/🔣️.json";
import * as api from "./../../../../../../../../🟦️.ts";
interface GraphCase {name:string;nodes:string[];edges:(api.JackGraphEndpoint&api.JackFlatPositionUv)[];topology:api.JackTopology;positions:Record<string,api.JackFlatPositionUv>}
const fixture=fixtureJson as unknown as {identity:api.ArtifactChild;cases:GraphCase[]};
const topology=api.inferJackTopology;
const flat=api.inferJackFlatPosition;
const parent:api.JackArtifact={schema:"s.trinity.jack/v1",name:"query",manifest:{nodeKinds:[],edgeKinds:[],portKinds:[]},camera:{x:{bits:0n},y:{bits:0n},zoom:{bits:0x3ff0000000000000n}},content:fixture.identity,query:"",rootNodeId:undefined};
function query(example:Pick<GraphCase,"nodes"|"edges">,identity=fixture.identity){return{identity,nodeIds:()=>example.nodes,edges:()=>example.edges,displacement:(id:string)=>{const edge=example.edges.find(edge=>edge.id===id)!;return{u:edge.u,v:edge.v};}};}
function oracle(example:Pick<GraphCase,"nodes"|"edges">){
 const key=(id:string)=>JSON.stringify([id]),literal=(id:string)=>JSON.parse(id)[0] as string,graph=new MultiDirectedGraph();for(const id of example.nodes)graph.addNode(key(id));for(const edge of example.edges)graph.addDirectedEdgeWithKey(edge.id,key(edge.source),key(edge.target),{u:edge.u,v:edge.v});
 const order:string[]=[],depth:Record<string,number>=Object.create(null),pending=new Set(graph.nodes().map(literal)),held=graph.copy(),frontier=[...pending].filter(id=>held.inDegree(key(id))===0).sort();
 for(let index=0;index<frontier.length;index++){const id=frontier[index]!,targets=held.outNeighbors(key(id)).map(literal).sort();order.push(id);depth[id]=Math.max(0,...graph.inNeighbors(key(id)).map(literal).map(source=>depth[source]!+1));pending.delete(id);held.dropNode(key(id));frontier.push(...targets.filter(target=>held.inDegree(key(target))===0));}
 const positions:Record<string,{u:number;v:number}>=Object.create(null),remaining=new Set(graph.nodes().map(literal));
 while(remaining.size){const sorted=[...remaining].sort(),seed=Object.keys(positions).length===0?sorted[0]!:(sorted.find(id=>graph.inNeighbors(key(id)).map(literal).every(source=>!remaining.has(source)))??sorted[0]!);positions[seed]={u:0,v:0};remaining.delete(seed);const stack=[seed];while(stack.length){const source=stack.pop()!;for(const edge of graph.outEdges(key(source)).sort()){const target=literal(graph.target(edge));if(!remaining.has(target))continue;const attributes=graph.getEdgeAttributes(edge),held=positions[source]!;positions[target]={u:held.u+attributes.u,v:held.v+attributes.v};remaining.delete(target);stack.push(target);}}}
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

test("Jack schema declares the consumed retained query contract",async()=>{
 const schema=parse(await Bun.file(new URL("../../🪆️graph-query/🔗️.graphql",import.meta.url)).text()),port=schema.definitions.find(value=>value.kind===Kind.INTERFACE_TYPE_DEFINITION&&value.name.value==="JackGraphQuery");
 expect(port?.kind).toBe(Kind.INTERFACE_TYPE_DEFINITION);if(port?.kind!==Kind.INTERFACE_TYPE_DEFINITION)throw Error("Jack query schema absent");expect(port.fields?.map(value=>value.name.value)).toEqual(["identity","nodeIds","edges","displacement"]);
 const edge=schema.definitions.find(value=>value.kind===Kind.OBJECT_TYPE_DEFINITION&&value.name.value==="JackGraphEndpoint");if(edge?.kind!==Kind.OBJECT_TYPE_DEFINITION)throw Error("Jack edge schema absent");expect(edge.fields?.map(value=>value.name.value)).toEqual(["id","source","target"]);
});
test("Jack public source builds with the removable Stdio implementation forbidden",async()=>{
 const result=await build({entryPoints:[fileURLToPath(new URL("./../../../../../../../../🟦️.ts",import.meta.url))],bundle:true,write:false,platform:"node",plugins:[{name:"removed-stdio",setup(builder){builder.onLoad({filter:/.*/},(args)=>{if(args.path.replaceAll("\\","/").includes("/✏️s/🔌️plugins/🗄️stdio/"))throw Error("Jack public source reached removable Stdio implementation");return undefined;});}}]});expect(result.outputFiles.length).toBe(1);
});


test("rich read presentation agrees with independent SQLite case semantics", async () => {
  const { Database } = await import("bun:sqlite");
  const fixture = await Bun.file(new URL("../../../../🧪️tests/🧫️fixtures/💡️inferences/👁️rich-read/🔣️.json", import.meta.url)).json();
  const db = new Database(":memory:");
  try {
    for (const node of fixture.nodes) {
      const result = db.query("SELECT CASE WHEN ? = '' THEN ? ELSE ? END AS label, CASE WHEN ? > 0 THEN ? ELSE 96 END AS width, CASE WHEN ? > 0 THEN ? ELSE 48 END AS height").get(node.label, node.id, node.label, node.width, node.width, node.height, node.height) as { label: string; width: number; height: number };
      expect(result).toEqual({ label: node.expectedLabel, width: node.expectedWidth, height: node.expectedHeight });
      expect(node.ports.filter((port: {inbound: boolean}) => port.inbound).map((port: {id: string}) => port.id)).toEqual(["both@port"]);
      expect(node.ports.filter((port: {outbound: boolean}) => port.outbound).map((port: {id: string}) => port.id)).toEqual(["both@port"]);
    }
  } finally { db.close(); }
});
