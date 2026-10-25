import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
test("original tool request retirement covers every language-neutral owned source",()=>{
 const fixture=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 const schema={type:"object",required:["schema","owned","sharedLeases","wire","cancelled","pending","terminalDropBytes"],additionalProperties:false,properties:{schema:{const:"semio.plugin.original-tool-request-retirement.v1"},owned:{type:"array",minItems:19,maxItems:19,uniqueItems:true,items:{type:"string"}},sharedLeases:{type:"array",minItems:5,maxItems:5,uniqueItems:true,items:{type:"string"}},wire:{type:"array",minItems:2,maxItems:2,uniqueItems:true,items:{type:"string"}},cancelled:{const:"sameOriginalSources"},pending:{const:"sameCallerSlot"},terminalDropBytes:{const:0}}};
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 for(const field of fixture.owned)expect(source).toContain("deferred("+field+")");
 for(const field of fixture.sharedLeases)expect(source).toContain("OriginalRequestLease("+field+")");
 for(const field of fixture.wire)expect(source).toContain(field+".map(OriginalRequestWire)");
 const input=Object.fromEntries(fixture.owned.map((field:string)=>[field,{original:true}]));
 expect(applyPatch(input,fixture.owned.map((field:string)=>({op:"remove"as const,path:"/"+field})),true,false).newDocument).toEqual({});
});
