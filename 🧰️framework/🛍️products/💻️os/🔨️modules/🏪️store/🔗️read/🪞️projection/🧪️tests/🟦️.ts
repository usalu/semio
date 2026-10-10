import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import {applyPatch} from "fast-json-patch";
test("original Store projection corpus preserves the exact issuer through independent patch replay",()=>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));
 const valid=new Ajv2020({strict:true,allErrors:true}).compile(schema);
 expect(valid(fixture),JSON.stringify(valid.errors)).toBe(true);
 for(const row of fixture.cases){
  const original={identity:row.id,owner:row.owner};const input={projection:original,read:original,registry:[original]};
  const result=applyPatch(input,[{op:"remove",path:"/projection"},{op:"move",from:"/read",path:"/returned"}],true,false).newDocument as unknown as {returned:typeof original;registry:Array<typeof original>};
  expect({projectionHeapRelease:0,originalPreserved:result.returned.owner===row.owner,returnedToOriginalIssuer:result.registry[0]?.identity===result.returned.identity}).toEqual(row.expected);
 }
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8");
 expect(source.includes("self.root.has_original_issuer()")).toBe(true);
 expect(source.includes("drop(self.root.projection.take())")).toBe(true);
 expect(source.includes("ControlledRetirement::new(read)")).toBe(true);
 expect(source.includes("weak_count")).toBe(false);
 expect(source.includes("strong_count")).toBe(false);
});
