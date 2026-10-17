import{expect,test}from"bun:test";
import Ajv from"ajv";
import{applyPatch}from"fast-json-patch";
import fixture from"./🧫️fixtures/🔣️.json";
import domain from"../../../../🗿️artifacts/🌊️flow/🧵️retained/🧬️schema/🔣️.json";
import { orderedRetirementSchema as grant } from "@semio-tech/value-ts/ordered/retirement/schema";
const isRecord=(value:unknown):value is Record<string,unknown>=>typeof value==="object"&&value!==null&&!Array.isArray(value);
const record=(value:unknown):Record<string,unknown>=>isRecord(value)?value:{};
const pointer=(key:string):string=>"/"+key.replaceAll("~","~0").replaceAll("/","~1");
test("original display projection agrees with independently applied JSON Patch",()=>{
 const ajv=new Ajv({strict:true,allErrors:true}).addSchema(grant).addSchema(domain),widget=ajv.compile({$ref:domain.$id+"#/$defs/Widget"}),dictionary=ajv.compile({$ref:domain.$id+"#/$defs/FlowDictionary"});for(const row of fixture.cases){for(const value of row.widgets){expect(widget(value)).toBe(true);expect(widget({...value,id:null})).toBe(false);}for(const channels of [row.inputs,row.outputs])for(const value of Object.values(channels))expect(dictionary(value)).toBe(true);for(const value of Object.values(row.expected)){expect(dictionary(value.in)).toBe(true);expect(dictionary(value.out)).toBe(true);}}
 for(const row of fixture.cases){let expected:Record<string,unknown>={};const inputs=record(row.inputs),outputs=record(row.outputs),kindInfos=record(row.kindInfos);for(const widget of row.widgets){const fields=record(widget),input=applyPatch<Record<string,unknown>>(structuredClone(record(inputs[widget.id])),Object.entries(record(fields.params)).map(([key,value])=>({op:"add"as const,path:pointer(key),value})),true,false).newDocument;const info=record(kindInfos[String(fields.neuronKind)]),slot=typeof info.variadicSlot==="string"?info.variadicSlot:null,names=Array.isArray(info.inputs)?info.inputs.filter((name):name is string=>typeof name==="string"):[];let visible:Record<string,unknown>=input;if(Object.keys(info).length>0){visible={...(slot!==null?record(input[slot]):{})};for(const key of names)if(key!=="*"&&key in input)visible[key]=input[key];}const out=record(outputs[widget.id]);expected=applyPatch<Record<string,unknown>>(expected,[{op:"add",path:pointer(widget.id),value:{in:visible,out,...(typeof out.error==="string"?{error:out.error}:{})}}],true,false).newDocument;}expect(expected).toEqual(row.expected);}
});
