import {existsSync as testingSchemaExists} from "node:fs";
import {expect,test} from "bun:test";
import {Graph,alg} from "graphlib";
import fixture from "../🧫️fixtures/🔣️.json";

test("original budgeted topology agrees with independent Graphlib levels",()=>{
expect(testingSchemaExists(new URL("../🧬️schema/🔣️.json",import.meta.url))).toBe(false);
 for(const row of fixture.cases){
  const graph=new Graph({directed:true});for(const id of row.nodes)graph.setNode(id);for(const [from,to] of row.edges)graph.setEdge(from,to);
  const levels=new Map<string,number>();for(const id of alg.topsort(graph))levels.set(id,Math.max(0,...(graph.predecessors(id)??[]).map(source=>(levels.get(source)??0)+1)));
  const order=[...row.nodes].sort((left,right)=>(levels.get(left)??0)-(levels.get(right)??0)||(left<right?-1:left>right?1:0));expect(order).toEqual(row.order);
  for(const [from,to] of row.edges)expect(order.indexOf(from)).toBeLessThan(order.indexOf(to));
 }
 console.log("[DEBUG] Original retained topology schemaAuthority=absent Graphlib=3 sourceConservation=unqualified");
});
