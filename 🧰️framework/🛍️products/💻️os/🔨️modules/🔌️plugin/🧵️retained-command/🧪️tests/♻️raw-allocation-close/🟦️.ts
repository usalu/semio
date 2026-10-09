import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
test("raw retirement corpus separates copied content from original backing release",()=>{
 const root=join(import.meta.dir,"../..");
 const fixture=JSON.parse(readFileSync(join(root,"🧫️fixtures/🚪️raw-allocation-close.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(root,"🧬️schema/🚪️raw-allocation-close/🔣️.json"),"utf8"));
 const validate=new Ajv2020({strict:true,allErrors:true}).compile(schema);
 expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 for(const row of fixture.cases){expect(row.initializedBytes).toBeLessThanOrEqual(row.capacity);expect(row.expectedCopiedBytes).toBe(row.initializedBytes);expect(row.expectedReleasedBytes).toBe(row.capacity);}
 expect(fixture.cases.some((row:{capacity:number;initializedBytes:number})=>row.capacity>fixture.maximumCopyBytes&&row.initializedBytes===0)).toBe(true);
 expect(validate({...fixture,maximumCopyBytes:-1})).toBe(false);
});
