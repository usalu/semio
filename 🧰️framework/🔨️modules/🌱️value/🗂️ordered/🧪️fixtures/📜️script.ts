/** 🗂️ Language-neutral ordered-map fixtures, strict admission, and independent UTF-8 ordering oracle. */
import Ajv from "ajv";
import { strict as assert } from "node:assert";
import stableStringify from "fast-json-stable-stringify";

//#region 🧬️Contract
const fixture = await Bun.file(new URL("./🔣️ordered-map.json", import.meta.url)).json();
const schema = await Bun.file(new URL("./🔣️.schema.json", import.meta.url)).json();
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
assert(validate(fixture), JSON.stringify(validate.errors));
assert.equal(new Set(fixture.cases.map((row: any) => row.id)).size, fixture.cases.length);
const key = (value: any): string => value.prefix.repeat(value.repetitions) + value.suffix;
for (const row of fixture.cases) {
  const oracle = new Map<string, string>();
  for (const operation of row.operations) {
    if (operation.op === "set") oracle.set(key(operation.key), operation.value);
    else oracle.delete(key(operation.key));
  }
  const entries = [...oracle].sort(([a], [b]) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
  assert.deepEqual(entries, row.expected.map((entry: any) => [key(entry.key), entry.value]));
  assert.deepEqual(JSON.parse(JSON.stringify(Object.fromEntries(entries))), Object.fromEntries(entries));
  const thirdParty = stableStringify(Object.fromEntries(oracle), { cmp: (a, b) => Buffer.compare(Buffer.from(a.key), Buffer.from(b.key)) });
  assert.equal(thirdParty, `{${entries.map(([key, value]) => `${JSON.stringify(key)}:${JSON.stringify(value)}`).join(",")}}`);
}
for (const row of fixture.lookupCases) {
  const source = Buffer.from(key(row.sourceKey)); const query = Buffer.from(key(row.query));
  const oracle = new Map([[key(row.sourceKey), row.sourceValue]]);
  assert.equal(oracle.get(key(row.query)) ?? null, row.expected);
  let compared = 0;
  for (let index = 0; index < Math.min(source.length, query.length); index++) { compared += 2; if (source[index] !== query[index]) break; }
  assert.equal(compared, row.expectedComparedBytes);
  assert.equal(Buffer.compare(source, query) === 0, row.expected !== null);
}
for (const mutate of [
  (value: any) => { value.extra = true; },
  (value: any) => { value.grants = [16384]; },
  (value: any) => { value.cases[0].operations[0].key.unknown = true; },
  (value: any) => { value.cases[0].operations[0].key.repetitions = -1; },
  (value: any) => { value.cases[0].operations[0].op = "replace"; },
  (value: any) => { value.lookupCases[0].extra = true; },
  (value: any) => { value.ownership.liveDrop = "recursive-drop"; },
  (value: any) => { value.ownership.terminalOwners = 1; },
]) {
  const mutant = structuredClone(fixture); mutate(mutant); assert(!validate(mutant));
}
//#endregion 🧬️Contract
//#region 📤️SharedOwnership
const sharedFixture = await Bun.file(new URL("./📤️shared-owner.json", import.meta.url)).json();
const sharedSchema = await Bun.file(new URL("./📤️shared-owner.schema.json", import.meta.url)).json();
const validateShared = new Ajv({ strict: true, allErrors: true }).compile(sharedSchema);
assert(validateShared(sharedFixture), JSON.stringify(validateShared.errors));
const sharedKey = sharedFixture.key.text.repeat(sharedFixture.key.repetitions);
assert.equal(Buffer.byteLength(sharedKey), sharedFixture.expected.keyBytes);
assert.equal(sharedFixture.aliases - 1, sharedFixture.expected.sharedReleases);
assert.equal(stableStringify({ [sharedKey]: true }), JSON.stringify({ [sharedKey]: true }));
for (const mutant of [{ ...sharedFixture, extra: true }, { ...sharedFixture, expected: { ...sharedFixture.expected, finalHandoffs: 0 } }]) assert(!validateShared(mutant));
console.log("[DEBUG] Shared-owner source fixtures=1 hostileRejections=2 oracle=fast-json-stable-stringify runtimeClaims=0");
//#endregion 📤️SharedOwnership
//#region 🧺️SetContract
const setFixture = await Bun.file(new URL("../🧺️set/🧪️fixture/🔣️s.json", import.meta.url)).json();
const setSchema = await Bun.file(new URL("../🧺️set/🧬️schema/🔣️.schema.json", import.meta.url)).json();
const validateSet = new Ajv({ strict: true, allErrors: true }).compile(setSchema);
assert(validateSet(setFixture), JSON.stringify(validateSet.errors));
const orderedSet = [...new Set<string>(setFixture.values)].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
assert.deepEqual(orderedSet, setFixture.expectedValues);
assert.equal(stableStringify(orderedSet), JSON.stringify(setFixture.expectedValues));
for (const mutant of [{ ...setFixture, extra: true }, { ...setFixture, expected: { ...setFixture.expected, explicitRetirement: false } }]) assert(!validateSet(mutant));
console.log("[DEBUG] Ordered-set source fixtures=1 hostileRejections=2 oracle=fast-json-stable-stringify runtimeClaims=0");
//#endregion 🧺️SetContract
console.log("[DEBUG] Ordered-map source fixtures=3 lookupCases=2 hostileRejections=8 grants=1,64,4096 oracle=fast-json-stable-stringify runtimeClaims=0");
