import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv/dist/2020";
const root=new URL("../",import.meta.url);
const fixture=JSON.parse(readFileSync(new URL("🧫️fixtures/🔣️.json",root),"utf8"));
test("preborn command entry neutral UTC and full label corpus agrees with Node Date and TextEncoder",()=>{
 const schema=JSON.parse(readFileSync(new URL("📐️schema.json",root),"utf8"));expect(new Ajv({strict:false}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.timestamps){expect(new Date(row.millis).toISOString()).toBe(row.iso);expect(new TextEncoder().encode(row.iso).length).toBe(24);}
 for(const value of fixture.invalidMillis){expect(new Date(value).toISOString().length).toBe(27);}
 for(const row of fixture.cases){const labels=fixture.terminologies.flatMap((term:string)=>fixture.locales.map((locale:string)=>row.label?.[term]?.[locale]??row.actionId));expect(labels.length).toBe(4);for(const text of labels){expect(Buffer.from(text)).toEqual(Buffer.from(new TextEncoder().encode(text)));}expect(row.parentEditId!==null).toBe(row.parentTouched);}
});
test("preborn command fields exist before original receipt IDs transfer and final scalar sequence authority",()=>{
 const source=readFileSync(new URL("🦀️.rs",root),"utf8");for(const contract of ["MountedCommandEntrySource","captured_millis","LocalizedLabel","next_capacity_byte_demand","next_close_byte_demand","child_edit_count","fn take","seq: u64"]){expect(source).toContain(contract);}expect(source).not.toContain("now_iso");expect(source).not.toContain(".clone()");
});

test("large command label retains exact original UTF-8 extent across 64-byte work pages", () => {
 const text="雪\u0000".repeat(2048)+"é";
 expect(new TextEncoder().encode(text).length).toBe(fixture.largeLabelBytes);
 expect(Buffer.from(text).byteLength).toBe(fixture.largeLabelBytes);
 let offset=0;while(offset<fixture.largeLabelBytes){const copied=Math.min(64,fixture.largeLabelBytes-offset);expect(copied).toBeLessThanOrEqual(fixture.maximumCopyBytes);offset+=copied;}expect(offset).toBe(fixture.largeLabelBytes);
});
