import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import {readFileSync} from "node:fs";
test("JSON recovery fixture agrees with independent JSON encoding",()=>{
 const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));const value=read("🔣️.json"),schema=read("📐️.json");
 expect(new Ajv2020({strict:true}).compile(schema)(value)).toBe(true);expect(JSON.stringify(value.source)).toBe(value.expected);
});
