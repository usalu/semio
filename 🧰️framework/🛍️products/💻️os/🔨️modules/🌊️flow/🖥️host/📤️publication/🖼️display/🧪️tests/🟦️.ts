import{expect,test}from"bun:test";
import Ajv from"ajv";
import{applyPatch}from"fast-json-patch";
import fixture from"../🧫️fixtures/🔣️.json";
import domain from"../../../../🗿️artifacts/🌊️flow/🧵️retained/🧬️schema/🔣️.json";
import grant from"../../../../../../../../🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json";
test("original display projection agrees with independently applied JSON Patch",()=>{
 const ajv=new Ajv({strict:true,allErrors:true}).addSchema(grant).addSchema(domain),widget=ajv.compile({$ref:domain.$id+"#/$defs/Widget"}),dictionary=ajv.compile({$ref:domain.$id+"#/$defs/FlowDictionary"});for(const row of fixture.cases){for(const value of row.widgets){expect(widget(value)).toBe(true);expect(widget({...value,id:null})).toBe(false);}for(const channels of [row.inputs,row.outputs])for(const value of Object.values(channels))expect(dictionary(value)).toBe(true);for(const value of Object.values(row.expected)){expect(dictionary(value.in)).toBe(true);expect(dictionary(value.out)).toBe(true);}}
 for(const row of fixture.cases){let expected={};for(const widget of row.widgets){const input=applyPatch(structuredClone(row.inputs[widget.id]??{}),Object.entries(widget.params??{}).map(([key,value])=>({op:"add"as const,path:"/"+key.replaceAll("~","~0").replaceAll("/","~1"),value})),true,false).newDocument;const info=row.kindInfos[widget.neuronKind];let visible=input;if(info){visible={...(info.variadicSlot?input[info.variadicSlot]??{}:{})};for(const key of info.inputs)if(key!=="*"&&key in input)visible[key]=input[key];}const out=row.outputs[widget.id]??{};expected=applyPatch(expected,[{op:"add",path:"/"+widget.id.replaceAll("~","~0").replaceAll("/","~1"),value:{in:visible,out,...(typeof out.error==="string"?{error:out.error}:{})}}],true,false).newDocument;}expect(expected).toEqual(row.expected);}
 console.log("[DEBUG] Original display canonicalPayloadAjv=true independentJSONPatch=true retainedNative=unqualified");
});
