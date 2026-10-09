import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Graph,alg} from "graphlib";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";

test("original budgeted topology agrees with independent Graphlib levels",()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,unknown:true})).toBe(false);
 for(const row of fixture.cases){
  const graph=new Graph({directed:true});for(const id of row.nodes)graph.setNode(id);for(const [from,to] of row.edges)graph.setEdge(from,to);
  const levels=new Map<string,number>();for(const id of alg.topsort(graph))levels.set(id,Math.max(0,...(graph.predecessors(id)??[]).map(source=>(levels.get(source)??0)+1)));
  const order=[...row.nodes].sort((left,right)=>(levels.get(left)??0)-(levels.get(right)??0)||(left<right?-1:left>right?1:0));expect(order).toEqual(row.order);
  for(const [from,to] of row.edges)expect(order.indexOf(from)).toBeLessThan(order.indexOf(to));
 }
 console.log("[DEBUG] Original retained topology strictAjv=true Graphlib=3 sourceConservation=unqualified");
});
