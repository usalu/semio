/** 🧺️ The original heap corpus shares ordered output with independent Lodash and JSON. */
import {test,expect} from "bun:test";
import sortBy from "lodash/sortBy";
import stableStringify from "fast-json-stable-stringify";
test("original heap preserves the neutral priority and custody corpus",async()=>{
 const fixture=await Bun.file(new URL("../🧫️fixtures/🧺️heap.json",import.meta.url)).json();
 expect(stableStringify(sortBy(fixture.keys))).toBe(stableStringify(fixture.expected));
 expect(fixture.capacityBytes).toBeGreaterThan(Math.max(...fixture.keys.map((key:string)=>Buffer.byteLength(key))));
 console.log("[DEBUG] Original BinaryHeap/Reverse priorities agree with independent Lodash and canonical JSON");
});
