import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Graph,alg} from "graphlib";
import {applyPatch} from "fast-json-patch";
import schema from "../../../🗿️artifacts/🌊️flow/🧵️retained/🧬️schema/🔣️.json";
import grant from "../../../../../../../🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import corpus from "../🧬️schema/🔣️.json";

test("original Host source duplicate cycle port and nested custody corpus agrees with independent authorities",()=>{
 const ajv=new Ajv({strict:true,allErrors:true}).addSchema(grant).addSchema(schema);
 const validate=ajv.compile(corpus);expect(validate(fixture)).toBe(true);expect(validate({...fixture,unknownOriginalOwner:true})).toBe(false);expect(validate({...fixture,cases:[{...fixture.cases[0],expectedPorts:[["a"]]}]})).toBe(false);
 const widget=ajv.compile({$ref:schema.$id+"#/$defs/Widget"}),synapse=ajv.compile({$ref:schema.$id+"#/$defs/Synapse"});
 for(const row of fixture.cases){for(const value of row.widgets){expect(widget(value)).toBe(true);expect(widget({...value,id:null})).toBe(false);}for(const value of row.synapses){expect(synapse(value)).toBe(true);expect(synapse({...value,from:null})).toBe(false);}}
 for(const row of fixture.cases){const seen=new Set<string>(),widgets=row.widgets.filter(widget=>{if(seen.has(widget.id))return false;seen.add(widget.id);return true;}),executables=widgets.filter(widget=>widget.kind!=="outputPreview");expect(executables.map(widget=>widget.id)).toEqual(row.expectedIds);const ids=new Set(executables.map(widget=>widget.id));const ports=row.synapses.filter(edge=>{const graph=new Graph({directed:true});for(const other of row.synapses)if(other.from!==edge.from||other.to!==edge.to)graph.setEdge(other.from,other.to);graph.setNode(edge.to);return !alg.preorder(graph,edge.to).includes(edge.from)&&edge.from!==edge.to;}).filter(edge=>ids.has(edge.from)&&ids.has(edge.to)).map(edge=>{const resolve=(id:string,port:string,side:"inputs"|"outputs")=>{const widget=widgets.find(widget=>widget.id===id)!;const candidates=widget.kind==="inputNote"?(side==="outputs"?["text"]:[]):((row.kindInfos as Record<string,{inputs:string[];outputs:string[]}>)[(widget as {neuronKind?:string}).neuronKind??""]?.[side]??[]);return candidates.includes(port)?port:((port===""||port===(side==="inputs"?"in":"out"))?candidates[0]??port:port);};return [edge.from,resolve(edge.from,edge.fromPort,"outputs"),edge.to,resolve(edge.to,edge.toPort,"inputs")];});expect(ports).toEqual(row.expectedPorts);const clusters=Object.fromEntries(executables.filter(widget=>widget.kind==="cluster").map(widget=>[widget.id,applyPatch({},[{op:"add",path:"/tree",value:(widget as {tree:unknown}).tree}],true,false).newDocument.tree]));expect(clusters).toEqual(row.expectedClusters);}
 console.log("[DEBUG] Original Host source canonicalDomainAjv=true wholeCorpusSchema=true wholeCorpusNegatives=2 perPayloadNegatives=true GraphlibCycleCases=4 JSONPatchNestedSource=1 nativeProducer=unqualified");
});
