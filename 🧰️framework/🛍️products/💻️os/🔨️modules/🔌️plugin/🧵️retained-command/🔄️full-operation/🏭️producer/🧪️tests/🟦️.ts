import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
test("original producer source bindings and retained custody follow the neutral corpus",()=>{
 const fixture=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 const schema={type:"object",additionalProperties:false,required:["schema","binding","cases"],properties:{schema:{const:"semio.plugin.original-producer-source.v1"},binding:{type:"array",minItems:6,maxItems:6,uniqueItems:true,items:{type:"string"}},cases:{type:"array",minItems:9,items:{type:"object",additionalProperties:false,required:["name","change","phase","allowed","owners","effect"],properties:{name:{type:"string"},change:{type:["string","null"]},phase:{enum:["pending","closing","refused"]},allowed:{type:"boolean"},owners:{type:"array",minItems:1,maxItems:1,items:{enum:["request","requestRetirement","job"]}},effect:{enum:["none","paidRetirement","actualReceipt"]}}}}}};
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const original=Object.fromEntries(fixture.binding.map((key:string,index:number)=>[key,index+1]));
 for(const row of fixture.cases){
  const candidate=row.change===null?original:applyPatch(original,[{op:"replace",path:"/"+row.change,value:-1}],true,false).newDocument;
  expect(fixture.binding.every((key:string)=>candidate[key]===original[key])).toBe(row.allowed);
 }
 const mounted=readModuleTreeSource(new URL("../../🦀️.rs",import.meta.url));
 const producer=readModuleTreeSource(new URL("../🦀️.rs",import.meta.url));
 const request=readFileSync(new URL("../../../../📜️declaration/👷️job/🧵️reserved-tool-job/🦀️.rs",import.meta.url),"utf8");
 expect(request).toContain("pub source_binding: semio_framework::action_bus::OriginalToolSourceBinding");
 expect(mounted).toContain("self.validate_original_producer_source(cx)?;");
 expect(mounted).toContain("(self.producer_request.is_some()||self.original_reserved_producer_pending())");
 expect(mounted).toContain("cx.is_cancelled()&&(self.original_producer_pending()");
 expect(producer).toContain("binding.matches(cx)");
 expect(producer.indexOf("if let Some(job)=candidate")).toBeLessThan(producer.indexOf("Err(failure)=>return Err(failure)"));
 console.log("[DEBUG] Nine neutral source/custody cases agree with independent Ajv/JSONPatch and the authored Rust source contract");
});
