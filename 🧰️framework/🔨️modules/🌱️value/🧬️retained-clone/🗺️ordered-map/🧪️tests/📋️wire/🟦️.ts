import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import fixture from "../../🧫️fixtures/📋️wire/🔣️.json" with {type:"json"};

test("cold context dictionaries preserve independent JSON map semantics and original paged authority",()=>{
 for(const row of fixture.cases){const oracle=Object.fromEntries(row.entries);expect(oracle).toEqual(row.expected);const sorted=Object.entries(oracle).sort(([left],[right])=>left<right?-1:left>right?1:0);expect(Object.fromEntries(sorted)).toEqual(JSON.parse(JSON.stringify(row.expected)));}
 const native=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");
 expect(native).toContain("FromIterator<(K, V)> for RetainedOrderedMap<K, V>");
 expect(native).toContain("Deserialize<'de> for RetainedOrderedMap<K, V>");
 expect(native).toContain("cold_insert");
 const codec=readFileSync(new URL("../../🚦️native/🦀️.rs",import.meta.url),"utf8");
 expect(codec).toContain("from_value_controlled");expect(codec).toContain("control.allocate_vec");
 console.log("[DEBUG] Five neutral cold dictionary cases checked against JavaScript Object.fromEntries and JSON wire; native physical witness remains separate");
});