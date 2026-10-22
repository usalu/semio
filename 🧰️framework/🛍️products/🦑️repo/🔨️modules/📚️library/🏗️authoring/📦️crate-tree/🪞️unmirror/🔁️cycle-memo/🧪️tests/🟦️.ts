/** 🔁️ Contract: the cycle memo answers a lookup that reaches a computation still in progress with the fallback, keeps results that never saw such a cut final and reuses the others only while the computation they cut against runs, so every query order over a cyclic graph yields the true reachable set; the corpus is language-agnostic and fast-check (seeded random graphs and query orders) plus a brute-force DFS are the independent oracle. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import fc from "fast-check";
import {CycleMemo} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
type Edges=Record<string,string[]|undefined>;
const reach=(memo:CycleMemo,edges:Edges,counts?:Record<string,number>)=>{
 const visit=(node:string):Set<string>=>memo.compute<Set<string>>(node,()=>new Set(),()=>{if(counts)counts[node]=(counts[node]??0)+1;const out=new Set([node]);for(const next of edges[node]??[])for(const name of visit(next))out.add(name);return out;});
 return visit;
};
const brute=(edges:Edges,start:string):string[]=>{const seen=new Set<string>(),stack=[start];while(stack.length){const node=stack.pop()!;if(seen.has(node))continue;seen.add(node);stack.push(...(edges[node]??[]));}return[...seen].sort();};
for(const row of corpus.graphs)test("reachability through the memo: "+row.name,()=>{
 for(const order of row.orders){const memo=new CycleMemo(),visit=reach(memo,row.edges);for(const node of order)expect([...visit(node)].sort(),row.name+" "+node+" in order "+order.join()).toEqual((row.reach as Edges)[node]!);}
 for(const node of Object.keys(row.reach)){const visit=reach(new CycleMemo(),row.edges);expect([...visit(node)].sort(),row.name+" fresh "+node).toEqual((row.reach as Edges)[node]!);}
 expect<unknown>(Object.fromEntries(Object.keys(row.reach).map(node=>[node,brute(row.edges,node)]))).toEqual(row.reach);
});
test("an acyclic computation runs each node once and answers repeats from the memo",()=>{
 const edges=corpus.graphs.find(row=>row.name==="a diamond")!.edges,counts:Record<string,number>={},visit=reach(new CycleMemo(),edges,counts);visit("a");visit("a");visit("b");expect(counts).toEqual({a:1,b:1,c:1,d:1});
});
test("a node inside a cycle is final once its cycle head has finished only when it never saw a cut",()=>{
 const edges=corpus.graphs.find(row=>row.name==="a cycle with a tail")!.edges,counts:Record<string,number>={},visit=reach(new CycleMemo(),edges,counts);visit("t");expect(counts["z"]).toBe(1);expect(counts["t"]).toBe(1);visit("t");expect(counts["t"]).toBe(1);
});
test("a failing computation does not poison the memo",()=>{
 const memo=new CycleMemo();let attempts=0;
 expect(()=>memo.compute("k",()=>"cut",()=>{attempts+=1;throw new Error("boom");})).toThrow("boom");expect(memo.compute("k",()=>"cut",()=>{attempts+=1;return "ok";})).toBe("ok");expect(attempts).toBe(2);expect(memo.compute("k",()=>"cut",()=>"again")).toBe("ok");
});
test("random graphs: every query order answers the true reachable set (fast-check)",()=>{
 const names=["a","b","c","d","e","f"],edgeList=fc.array(fc.tuple(fc.constantFrom(...names),fc.constantFrom(...names)),{maxLength:14});
 fc.assert(fc.property(edgeList,fc.shuffledSubarray(names,{minLength:1,maxLength:6}),fc.shuffledSubarray(names,{minLength:1,maxLength:6}),(pairs,first,second)=>{
  const edges:Record<string,string[]>={};for(const[from,to]of pairs)(edges[from]??=[]).push(to);
  const memo=new CycleMemo(),visit=reach(memo,edges);
  for(const node of[...first,...second])expect([...visit(node)].sort(),node).toEqual(brute(edges,node));
  for(const node of names)expect([...reach(new CycleMemo(),edges)(node)].sort(),"fresh "+node).toEqual(brute(edges,node));
 }),{numRuns:corpus.property.runs,seed:corpus.property.seed});
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
