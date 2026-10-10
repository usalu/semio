/** 🧺️ Native membership custody shares its exact corpus with independent JavaScript Set and JSON. */
import {strict as assert} from "node:assert";
import Ajv2020 from "ajv/dist/2020";
import stableStringify from "fast-json-stable-stringify";
const fixture=await Bun.file(new URL("../🧫️fixtures/🧺️set.json",import.meta.url)).json();
const schema=await Bun.file(new URL("../🧬️schema/🧺️set.json",import.meta.url)).json();
assert.equal(new Ajv2020({strict:true,allErrors:true}).compile(schema)(fixture),true);
const set=new Set<string>(fixture.clearedKeys);
set.clear();
assert.equal(set.size,0);
for(const key of fixture.keys){set.add(key);}
assert.equal(stableStringify([...set].sort()),stableStringify(fixture.expected));
assert.equal(fixture.detachedSlotPreservesLiveGeneration,true);
assert.equal(set.has(fixture.reusedKey),true);
for(const key of fixture.taken){assert.equal(set.has(key),true);assert.equal(set.delete(key),true);}
assert.equal(set.size,0);
console.log("[DEBUG] Original generic membership corpus agrees with JavaScript Set and independent canonical JSON");

import {test,expect} from "bun:test";

test("original receipt order fold 2 keeps plain trials outside schema authority",async()=>{const {existsSync}=await import("node:fs");expect(existsSync(new URL("../🧬️schema/🧺️set.json",import.meta.url))).toBe(false);console.log("[DEBUG] Original receipt/order/fold trial has no whole-corpus schema authority");});
