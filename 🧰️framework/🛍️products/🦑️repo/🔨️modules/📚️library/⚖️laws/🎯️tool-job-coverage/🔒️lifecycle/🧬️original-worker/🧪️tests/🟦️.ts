/** 🧪️ Actual input owners must reject every neutral admission omission. */
import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import jsonpatch from "fast-json-patch";
import schema from "../📐️.json";
import fixture from "../🔣️.json";
import {toolJobOriginalWorkerEvidence,toolJobOriginalWorkerExact} from "../🟦️.ts";
import {toolJobPluginInstanceSource} from "../../../📤️publication/🟦️.ts";
import {toolJobProductionSource,toolJobRustBlock} from "../../../📋️inventory/🟦️.ts";
const source=toolJobProductionSource(toolJobPluginInstanceSource(process.cwd()));
const evidence=toolJobOriginalWorkerEvidence(source);
const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
test("actual original worker admission satisfies every required gate",()=>{
 expect(evidence).toEqual(Object.fromEntries(schema.required.map(key=>[key,true])));
 expect(validate(evidence)).toBe(true);
 expect(toolJobOriginalWorkerExact(source)).toBe(true);
 expect(toolJobOriginalWorkerExact("")).toBe(false);
});
for(const row of fixture.cases)test(row.name,()=>{
 const pattern=(needle:string)=>new RegExp([...needle.replace(/\s+/g,"")].map(character=>character.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join("\\s*"));
 const match=pattern(row.owner).exec(source);
 expect(match).not.toBeNull();
 const open=source.indexOf("{",match!.index),block=toolJobRustBlock(source,open);
 expect(block).toBeDefined();
 const needle=pattern(row.before);
 expect(needle.test(block!.body)).toBe(true);
 let changed:string;
 if(row.wrapper){
  const start=needle.exec(block!.body)!.index,open=block!.body.indexOf("{",start),expression=toolJobRustBlock(block!.body,open);
  expect(expression).toBeDefined();
  const actual=block!.body.slice(start,expression!.end);
  changed=block!.body.slice(0,start)+row.wrapper.prefix+actual+row.wrapper.suffix+block!.body.slice(expression!.end);
 }else changed=block!.body.replace(needle,row.after);
 const candidate=source.slice(0,open+1)+changed+source.slice(open+1+block!.body.length);
 const actual=toolJobOriginalWorkerEvidence(candidate);
 const reference=jsonpatch.applyPatch(evidence,[{op:"replace",path:"/"+row.gate,value:false}],true,false).newDocument;
 expect(validate(reference)).toBe(false);
 expect(actual[row.gate]).toBe(false);
 expect(validate(actual)).toBe(false);
 expect(toolJobOriginalWorkerExact(candidate)).toBe(false);
});
