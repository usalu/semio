import {expect,test} from "bun:test";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../../../../🗿️artifacts/🌊️flow/🧵️retained/🧬️schema/🔣️.json";
import grant from "../../../../../../../../🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json";

test("original builtin seed custody agrees with independent JSON Patch and UTF8",()=>{
 const ajv=new Ajv({strict:true,allErrors:true}).addSchema(grant).addSchema(schema),validate=ajv.compile({$ref:schema.$id+"#/$defs/Tree"});
 for(const row of fixture.cases){expect(validate(row.tree)).toBe(true);expect(validate({...row.tree,neurons:null})).toBe(false);}
 for(const row of fixture.cases){let output:Record<string,unknown>={};for(const neuron of row.tree.neurons){const kind=({"core.number":"number","core.text":"text","core.image":"image"}as Record<string,string>)[neuron.kind];if(!kind)continue;const key=kind==="image"?"dataUrl":"value";const source=(neuron.params as Record<string,unknown>)[key];const value={schema:kind,[key]:source};output=applyPatch(output,[{op:"add",path:"/"+neuron.id.replaceAll("~","~0").replaceAll("/","~1"),value:{[kind]:value}}],true,false).newDocument;expect(Buffer.from(neuron.id,"utf8").toString("utf8")).toBe(neuron.id);}expect(output).toEqual(row.expected);}
 console.log("[DEBUG] Original builtin seed canonicalTreeAjv=true plainOriginalCases=true independentJSONPatch=true UTF8=true cases="+fixture.cases.length);
});
