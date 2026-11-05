import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import chunk from "lodash/chunk";
const read=(name:string)=>JSON.parse(readFileSync(new URL("./🧫️fixtures/"+name,import.meta.url),"utf8"));
const fixture=read("🔣️.json");
test("independent schema accepts the bounded absent-field corpus",()=>{expect(new Ajv({allErrors:true}).validate(read("📋️schema.json"),fixture)).toBe(true);});
for(const row of fixture.cases)test(row.name+" has independently bounded copy pages",()=>{const pages=chunk(new Uint8Array(row.extentBytes),row.pageBytes).map((page:number[])=>page.length);expect(pages).toEqual(row.chunks);expect(pages.reduce((sum:number,n:number)=>sum+n,0)).toBe(row.extentBytes);expect(row.extentBytes).toBe(row.payloadBytes+1);});
test("invalid ranges exceed the retained destination without wrapping",()=>{const extent=fixture.cases[0].extentBytes;for(const row of fixture.invalidRanges)expect(row.offset>extent||row.count>extent-row.offset).toBe(true);});
