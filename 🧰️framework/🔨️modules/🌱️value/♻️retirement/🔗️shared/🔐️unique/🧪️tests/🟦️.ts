import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
test("unique Arc custody corpus includes original weak leases and bounded work grants",()=>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
 const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));
 const validate=new Ajv2020({strict:true,allErrors:true}).compile(schema);
 expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 expect(fixture.cases.map((row:{weakAliases:number})=>row.weakAliases)).toEqual([0,1,3]);
 expect(fixture.copyGrants).toEqual([1,7,4096]);
 expect(validate({...fixture,cases:[{id:"negative",weakAliases:-1,value:31}]})).toBe(false);
});
