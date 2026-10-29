/** 🧪️ Actual input owners must reject every neutral admission omission. */
import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import jsonpatch from "fast-json-patch";
import schema from "../📐️.json";
import fixture from "../🔣️.json";
import {toolJobOriginalInputEvidence,toolJobOriginalInputAdmissionExact} from "../🟦️.ts";
import {toolJobPluginInstanceSource} from "../../../📤️publication/🟦️.ts";
import {toolJobProductionSource,toolJobRustBlock} from "../../../📋️inventory/🟦️.ts";
const source=toolJobProductionSource(toolJobPluginInstanceSource(process.cwd()));
const evidence=toolJobOriginalInputEvidence(source);
const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
test("actual original input admission satisfies every required gate",()=>{
 expect(evidence).toEqual(Object.fromEntries(schema.required.map(key=>[key,true])));
 expect(validate(evidence)).toBe(true);
 expect(toolJobOriginalInputAdmissionExact(source)).toBe(true);
 expect(toolJobOriginalInputAdmissionExact("")).toBe(false);
});
for(const row of fixture.cases)test(row.name,()=>{
 const pattern=(needle:string)=>new RegExp([...needle.replace(/\s+/g,"")].map(character=>character.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")).join("\\s*"));
 const match=pattern(row.owner).exec(source);
 expect(match).not.toBeNull();
 const open=source.indexOf("{",match!.index),block=toolJobRustBlock(source,open);
 expect(block).toBeDefined();
 const needle=pattern(row.before);
 expect(needle.test(block!.body)).toBe(true);
 const matches=[...block!.body.matchAll(new RegExp(needle.source,"g"))];
 const target=row.occurrence==="last"?matches.at(-1)!:matches[0]!;
 const changed=block!.body.slice(0,target.index!)+row.after+block!.body.slice(target.index!+target[0].length);
 const candidate=source.slice(0,open+1)+changed+source.slice(open+1+block!.body.length);
 const actual=toolJobOriginalInputEvidence(candidate);
 const reference=jsonpatch.applyPatch(evidence,[{op:"replace",path:"/"+row.gate,value:false}],true,false).newDocument;
 expect(validate(reference)).toBe(false);
 expect(actual[row.gate]).toBe(false);
 expect(validate(actual)).toBe(false);
 expect(toolJobOriginalInputAdmissionExact(candidate)).toBe(false);
});
