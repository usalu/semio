/** 🧹️ Strict cross-language nested-value retirement laws; source oracle only. */
import { strict as assert } from "node:assert";
import stableStringify from "fast-json-stable-stringify";



//#region 🔣️DomainFixture
const fixture = await Bun.file(new URL("../../🧫️fixtures/🔣️value-retirement.json", import.meta.url)).json();


const bytes = (value: any): number => typeof value === "string" ? Buffer.byteLength(value) : value && typeof value === "object" ? Object.entries(value).reduce((sum, [key, child]) => sum + Buffer.byteLength(key) + bytes(child), 0) : 0;
for (const row of fixture.cases) {
  const value = JSON.parse(row.json.replaceAll("$text", row.expandedText.text.repeat(row.expandedText.repetitions)));
  assert.equal(bytes(value), row.expectedBytes);
  assert.deepEqual(JSON.parse(stableStringify(value)), value);
  for (const grant of fixture.grants) {
    let remaining = row.expectedBytes; let released = 0;
    while (remaining) { const step = Math.min(grant, remaining); remaining -= step; released += step; }
    assert.equal(released, row.expectedBytes);
  }
}
//#endregion 🔣️DomainFixture

//#region 🧠️CacheFixture
const cacheFixture = await Bun.file(new URL("../../🧫️fixtures/🗃️cache-retirement/🔣️.json", import.meta.url)).json();


const cache = new Map<number, Record<string, string>>(); const pending: Record<string, string>[] = [];
let finalBytes = 0;
for (const operation of cacheFixture.operations) {
  if (operation.op === "seed") {
    const old = cache.get(cacheFixture.key); if (old) pending.push(old);
    cache.set(cacheFixture.key, { [operation.field]: operation.text.repeat(operation.repeat) });
  } else if (operation.op === "release-shared") {
    assert.equal(cache.size, cacheFixture.expected.liveEntriesBeforeFinal);
    assert.equal(finalBytes, cacheFixture.expected.sharedReleasedBytes);
    const current = cache.get(cacheFixture.key)!;
    const expected = cacheFixture.expected.finalJson.replace("$text", cacheFixture.operations[1].text.repeat(cacheFixture.operations[1].repeat));
    assert.equal(stableStringify(current), expected);
  } else {
    finalBytes = [...pending, ...cache.values()].reduce((sum, value) => sum + bytes(value), 0);
    pending.length = 0; cache.clear();
  }
}
assert.equal(finalBytes, cacheFixture.expected.finalReleasedBytes); assert.equal(cache.size + pending.length, cacheFixture.expected.terminalOwners);

//#endregion 🧠️CacheFixture

//#region 📸️EvaluationOwnership
const evaluation = await Bun.file(new URL("../../🧫️fixtures/🧮️evaluation-owners/🔣️.json", import.meta.url)).json();


const node = evaluation.node.text.repeat(evaluation.node.repeat); const payload = evaluation.payload.text.repeat(evaluation.payload.repeat);
assert.equal(Buffer.byteLength(node) + 2 * Buffer.byteLength(payload) + Buffer.byteLength("seednodelabel"), evaluation.expectedBytes);
assert.equal(stableStringify({ node: { label: payload } }), JSON.stringify({ node: { label: payload } }));

//#endregion 📸️EvaluationOwnership
