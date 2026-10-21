import{test,expect}from"bun:test";
import Ajv from"ajv";
import{readCargoCompilationGraphV1}from"../../🟦️.ts";
import graphSchema from"../../🧬️schema/🔣️.json"with{type:"json"};
import graphCorpus from"../🧫️fixtures/🔣️.json"with{type:"json"};

/** 🔁️ Cargo's unit graph lists an edge once per dependency table, so a crate that is both a normal and a dev dependency (or reached through two tables under one name) appears twice with identical facts; the closed graph keeps one edge, and distinct edges (another extern name, visibility or prelude flag) stay distinct. */
const raw=graphCorpus.cases[graphCorpus.identicalUnits.case]!.units[graphCorpus.identicalUnits.unit]!;
const edge=(index:number,name:string,extra:Record<string,unknown>={})=>({index,extern_crate_name:name,public:false,noprelude:false,...extra});
function graphOf(dependencies:readonly object[]){
 const units=[{...structuredClone(raw),dependencies:[...dependencies]},{...structuredClone(raw),dependencies:[]}];
 return readCargoCompilationGraphV1({version:1,units,roots:[0,1]},graphCorpus.metadata,"/repo");
}

test("identical dependency edges collapse to one, distinct edges stay, and the graph validates",()=>{
 const validate=new Ajv({strict:false}).compile(graphSchema);
 for(const [name,dependencies,expected] of[["twice",[edge(1,"dep"),edge(1,"dep")],1],["thrice",[edge(1,"dep"),edge(1,"dep"),edge(1,"dep")],1],["two names",[edge(1,"dep"),edge(1,"other")],2],["visibility",[edge(1,"dep"),edge(1,"dep",{public:true})],2],["prelude",[edge(1,"dep"),edge(1,"dep",{noprelude:true})],2],["none",[],0]] as const){
  const graph=graphOf(dependencies);const owner=graph.units.find(unit=>unit.dependencies.length>0||expected===0)!;
  const counts=graph.units.map(unit=>unit.dependencies.length);
  expect(Math.max(...counts),name).toBe(expected);expect(validate(graph),name+" "+JSON.stringify(validate.errors)).toBe(true);void owner;
 }
});
