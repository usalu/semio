/** 🧬 Portable podium/sign schema contracts validated through the existing AJV interface. */
const fs = require("node:fs");
const path = require("node:path");
const assert = require("node:assert/strict");
const Ajv = require("ajv/dist/2020").default;
const root = path.resolve(__dirname, "../..");
const read = file => JSON.parse(fs.readFileSync(path.join(root, file), "utf8"));
const model = read("🔣️.json");
const cases = read("🧪️cases/🏬️podium/🔣️.json");
const validate = new Ajv({strict: true, allErrors: true}).compile(read("🧬️schema/🔣️.schema.json"));
assert.equal(validate(model), true, JSON.stringify(validate.errors));
for (const test of cases.invalid) {
  const candidate = structuredClone(model);
  const parent = test.path.slice(0, -1).reduce((value, key) => value[key], candidate);
  if (test.remove) delete parent[test.path.at(-1)];
  else parent[test.path.at(-1)] = test.value;
  assert.equal(validate(candidate), false, test.id);
}
assert.deepEqual(model.podium.architecture.window_band.map(q => q.value), cases.window_band);
assert.equal(model.capital.signage.text, cases.sign.text);
assert.deepEqual(model.capital.signage.faces, cases.sign.faces);
console.log("Podium/sign schema accepted; " + cases.invalid.length + " invalid portable variants rejected.");
