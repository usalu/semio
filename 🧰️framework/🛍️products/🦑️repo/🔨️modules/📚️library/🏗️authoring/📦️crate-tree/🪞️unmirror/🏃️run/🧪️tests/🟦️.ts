/** 🏃️ Contract: the unmirror runner recognizes the facade crates of a plugin artifact or registry tree (those with carved crates below them) and reports the dependency cycles of the carved crates as `a -> b -> a` chains; the corpus is language-agnostic and graphlib (Tarjan strongly connected components) is the independent third-party oracle for acyclicity, chain validity and cycle coverage. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {Graph,alg} from "graphlib";
import {dependencyCycles,facadeIdents} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
test("a facade is an artifact or registry crate with a carved crate named and placed below it",()=>{for(const row of corpus.facades)expect([...facadeIdents(row.crates)].sort(),row.name).toEqual(row.expected);});
const edgesOf=(row:{edges:object}):Map<string,string[]>=>new Map(Object.entries(row.edges).filter((entry):entry is[string,string[]]=>Array.isArray(entry[1])));
for(const row of corpus.graphs)test("dependency cycles: "+row.name,()=>{expect(dependencyCycles(edgesOf(row))).toEqual(row.expected);});
const chain=(text:string):string[]=>text.split(" -> ");
test("graphlib agrees: a graph is acyclic exactly when no cycle is reported, every chain is a real cycle and every strongly connected component with a cycle is covered",()=>{
 for(const row of corpus.graphs){
  const graph=new Graph();for(const[name,next]of edgesOf(row)){graph.setNode(name);for(const target of next)graph.setEdge(name,target);}
  const reported=dependencyCycles(edgesOf(row));expect(reported.length===0,row.name).toBe(alg.isAcyclic(graph));
  for(const text of reported){const nodes=chain(text);expect<unknown>(nodes[0],text).toBe(nodes.at(-1));for(let index=0;index+1<nodes.length;index+=1)expect(graph.hasEdge(nodes[index]!,nodes[index+1]!),text+" edge "+index).toBe(true);}
  for(const component of alg.tarjan(graph).filter(members=>members.length>1||graph.hasEdge(members[0]!,members[0]!)))expect(reported.some(text=>chain(text).every(node=>component.includes(node))),row.name+" "+component.join(",")).toBe(true);
 }
});
test("the reported cycles are deterministic and do not depend on a second call",()=>{for(const row of corpus.graphs)expect(dependencyCycles(edgesOf(row))).toEqual(dependencyCycles(edgesOf(row)));});
test("a graph of many nodes with many cycles terminates",()=>{
 const edges=new Map<string,string[]>();for(let index=0;index<300;index+=1)edges.set("n"+index,["n"+((index+1)%300),"n"+((index*7+3)%300)]);
 const found=dependencyCycles(edges);expect(found.length).toBeGreaterThan(0);const graph=new Graph();for(const[name,next]of edges)for(const target of next)graph.setEdge(name,target);expect(alg.isAcyclic(graph)).toBe(false);
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
