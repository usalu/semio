import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Graph,alg} from "graphlib";
import {applyPatch,type Operation} from "fast-json-patch";
import engineContract from "../../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";

test("original budgeted evaluation boundary corpus agrees with Graphlib and JSONPatch",()=>{
 const domain=new Ajv({strict:true,allErrors:true});domain.addSchema(engineContract);const validateValue=domain.compile({$ref:engineContract.$id+"#/$defs/Value"});
 const merge=(base:unknown,overlay:Record<string,unknown>)=>applyPatch(structuredClone(base),Object.entries(overlay).map(([key,value])=>({op:"add",path:`/${key.replaceAll("~","~0").replaceAll("/","~1")}`,value})) as Operation[],true,false).newDocument;
 for(const row of fixture.cases){for(const value of [row.seeds,...row.nodes.map(node=>node.params)])expect(validateValue(value)).toBe(true);const graph=new Graph({directed:true});for(const node of row.nodes)graph.setNode(node.id);for(const [from,to] of row.edges)graph.setEdge(from,to);const outputs:Record<string,Record<string,unknown>>={},inputs:Record<string,unknown>={};for(const id of alg.topsort(graph)){let input:unknown={};for(const edge of row.edges){const [from,to,fromPort="",toPort=""]=edge;if(to!==id)continue;const output=outputs[from];let value:unknown=output;if(fromPort){value=output[fromPort]??(fromPort==="out"?(Object.keys(output).length===1?Object.values(output)[0]:output):{error:`missing channel ${fromPort}`});}if(toPort){input=merge(input,{[toPort]:value});}else if(value!==null&&typeof value==="object"&&!Array.isArray(value)){input=merge(input,value as Record<string,unknown>);}}inputs[id]=input;const node=row.nodes.find(node=>node.id===id)!;const params=Object.fromEntries(Object.entries(node.params).filter(([key])=>!row.edges.some(edge=>edge[1]===id&&edge[3]&&edge[3]===key)));outputs[id]=(row.seeds as Record<string,Record<string,unknown>>)[id]??merge(input,params);}expect(inputs).toEqual(row.expectedInputs);expect(outputs).toEqual(row.expectedOutputs);expect(validateValue(inputs)).toBe(true);expect(validateValue(outputs)).toBe(true);}
 console.log("[DEBUG] Original boundary evaluation domainValueAjv=true wholeCorpusAuthority=false Graphlib=7 JSONPatch=7 nativeConservation=unqualified");
});
