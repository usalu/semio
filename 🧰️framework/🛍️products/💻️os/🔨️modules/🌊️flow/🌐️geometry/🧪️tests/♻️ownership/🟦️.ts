/** 🌐️ Checks original geometry ownership examples with independent Unicode and JSON oracles. */
import {strict as assert} from "node:assert";
import stableStringify from "fast-json-stable-stringify";
const fixture=await Bun.file(new URL("../../🧫️fixtures/♻️ownership/🔣️.json",import.meta.url)).json();
assert.equal(Buffer.byteLength(fixture.source.text),fixture.expected.utf8Bytes);
assert.equal(new TextEncoder().encode(fixture.source.text).length,fixture.expected.utf8Bytes);
assert.deepEqual(JSON.parse(stableStringify({value:fixture.source.text})),{value:fixture.source.text});
assert(fixture.source.capacityBytes>fixture.expected.utf8Bytes);
console.log("[DEBUG] original GeometryPort utf8Bytes=7 stableJson=true");

const work=await Bun.file(new URL("../../🧫️fixtures/🎟️work.json",import.meta.url)).json();
assert.equal(stableStringify([...new Set<string>(work.handles)].sort()),stableStringify(work.expected));
console.log("[DEBUG] original geometry work handle examples agree with independent JavaScript Set");

assert(work.requestCapacityItems>work.handles.length);assert(Buffer.byteLength(work.refusedHandles[0])>work.refusedHandles[0].length);console.log("[DEBUG] original geometry retained request independent UTF8/source Set and request capacity agree");
