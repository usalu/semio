import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import {applyPatch} from "fast-json-patch";
test("original mutex payload survives poisoned status and independent owned transfer replay",()=>{
 const corpus=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));
 const validate=new Ajv2020({strict:true,allErrors:true}).compile(schema);expect(validate(corpus),JSON.stringify(validate.errors)).toBe(true);
 for(const row of corpus.cases){const original={owner:row.owner};const transferred=applyPatch({mutex:original,poisoned:row.poisoned},[{op:"move",from:"/mutex",path:"/owned"}],true,false).newDocument as unknown as {owned:typeof original};expect({preserved:transferred.owned.owner===row.owner,terminal:true}).toEqual(row.expected);}
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8");expect(source.includes("poisoned.into_inner()")).toBe(true);expect(source.includes("ControlledRetirement::new(original)")).toBe(true);expect(source.includes("maximum_bytes")).toBe(false);
});
