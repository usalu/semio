import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import {readFileSync} from "node:fs";

test("original encoder issuer fixture is accepted by independent schema validation",()=>{
 const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));
 const fixture=read("../../🧪️tests/🧫️fixtures/🎟️issuer.json"),schema=read("../🧬️schema/🎟️issuer/🔣️.json");
 expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);
 expect(fixture.allowance).toBe(1);
});

test("original post recovery fixture preserves each callback boundary",()=>{
 const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));
 const fixture=read("🧫️settlement.json"),schema=read("../🧬️schema/🧾️settlement/🔣️.json");
 expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);
 expect(fixture.postRefusals.work).not.toBe(fixture.postRefusals.physical);
 expect(new Set(fixture.admissionCases)).toEqual(new Set(["quote","work"]));
 for(const row of fixture.cases){expect(row.workCalls).toBe(1);expect(row.physicalCalls).toBe(1);expect(row.workBefore).toBe(row.panic==="work"?"started":"completed");expect(row.physicalBefore).toBe(row.panic==="physical"?"started":"waiting");}
});
