/** 🌱️ Checks original lazy entry examples with independent canonical JSON. */
import {strict as assert} from "node:assert";
import stableStringify from "fast-json-stable-stringify";
const fixture=await Bun.file(new URL("../../🧫️fixtures/🌱️entry/🔣️.json",import.meta.url)).json();
const map=new Map<number,string>(fixture.sourceRows);
let occupied=0;let vacant=0;
if(!map.has(fixture.occupiedKey)){occupied++;map.set(fixture.occupiedKey,"unused");}
if(!map.has(fixture.vacantKey)){vacant++;map.set(fixture.vacantKey,"");}
map.set(fixture.modifiedKey,map.get(fixture.modifiedKey)+"!");
for(const [key,value] of map)if(value.length)map.set(key,value+"?");
assert.equal(occupied,fixture.expected.occupiedLazyCalls);
assert.equal(vacant,fixture.expected.vacantLazyCalls);
assert.equal(stableStringify(Object.fromEntries(map)),stableStringify(fixture.expected.map));
console.log("[DEBUG] original fold index lazyOccupied=0 lazyVacant=1 independentStableJson=true");
