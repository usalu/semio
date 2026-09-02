/** 🧹️ Strict cross-language nested-value retirement laws; source oracle only. */
import Ajv from "ajv";
import { strict as assert } from "node:assert";
import stableStringify from "fast-json-stable-stringify";

//#region 🔣️DomainFixture
const fixture = await Bun.file(new URL("./🔣️value-retirement.json", import.meta.url)).json();
const schema = await Bun.file(new URL("./🔣️.schema.json", import.meta.url)).json();
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
assert(validate(fixture), JSON.stringify(validate.errors));
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
for (const mutate of [(value: any) => { value.extra = true; }, (value: any) => { value.grants = [16384]; }, (value: any) => { value.ownership.finalDrop = "recursive-drop"; }, (value: any) => { value.cases[0].expandedText.repetitions = -1; }]) {
  const mutant = structuredClone(fixture); mutate(mutant); assert(!validate(mutant));
}
console.log("[DEBUG] Neural value-retirement source fixtures=2 hostileRejections=4 grants=1,64,4096 oracle=fast-json-stable-stringify runtimeClaims=0");
//#endregion 🔣️DomainFixture

//#region 🧠️CacheFixture
const cacheFixture = await Bun.file(new URL("./🧪️cache-retirement/🔣️.json", import.meta.url)).json();
const cacheSchema = await Bun.file(new URL("./🧪️cache-retirement/🔣️.schema.json", import.meta.url)).json();
const validateCache = new Ajv({ strict: true, allErrors: true }).compile(cacheSchema);
assert(validateCache(cacheFixture), JSON.stringify(validateCache.errors));
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
for (const mutant of [{ ...cacheFixture, extra: true }, { ...cacheFixture, expected: { ...cacheFixture.expected, sharedReleasedBytes: 1 } }, { ...cacheFixture, operations: [{ op: "erase" }, ...cacheFixture.operations.slice(1)] }]) assert(!validateCache(mutant));
console.log("[DEBUG] Neural cache-retirement source fixtures=1 hostileRejections=3 oracle=fast-json-stable-stringify runtimeClaims=0");
//#endregion 🧠️CacheFixture

//#region 📸️EvaluationOwnership
const evaluation = await Bun.file(new URL("./🧪️evaluation-owners/🔣️.json", import.meta.url)).json();
const evaluationSchema = await Bun.file(new URL("./🧪️evaluation-owners/🔣️.schema.json", import.meta.url)).json();
const validateEvaluation = new Ajv({ strict: true, allErrors: true }).compile(evaluationSchema);
assert(validateEvaluation(evaluation), JSON.stringify(validateEvaluation.errors));
const node = evaluation.node.text.repeat(evaluation.node.repeat); const payload = evaluation.payload.text.repeat(evaluation.payload.repeat);
assert.equal(Buffer.byteLength(node) + 2 * Buffer.byteLength(payload) + Buffer.byteLength("seednodelabel"), evaluation.expectedBytes);
assert.equal(stableStringify({ node: { label: payload } }), JSON.stringify({ node: { label: payload } }));
for (const mutant of [{ ...evaluation, extra: true }, { ...evaluation, expectedBytes: 0 }]) assert(!validateEvaluation(mutant));
console.log("[DEBUG] Neural evaluation-retirement source fixtures=1 hostileRejections=2 bytes=25997 oracle=fast-json-stable-stringify runtimeClaims=0");
//#endregion 📸️EvaluationOwnership
