/** 🧬 Independent JSON Schema and GlobalId oracle for the portable Villa Savoye teaching model. */
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const assert = require("node:assert/strict");
const Ajv = require("ajv/dist/2020").default;
const clip = path.resolve(__dirname, "..");
const read = file => JSON.parse(fs.readFileSync(path.join(clip, file), "utf8"));
const schema = read("🧱️model/🧬️schema/🔣️.schema.json");
const model = read("🧱️model/🔣️.json");
const cases = read("🧪️cases/🔣️.json");

const resolve = (root, steps) => steps.reduce((value, step) => {
  if (step !== null && typeof step === "object") {
    const found = value.find(item => Object.entries(step).every(([key, expected]) => item[key] === expected));
    assert.ok(found, "selector " + JSON.stringify(step) + " matches nothing");
    return found;
  }
  assert.ok(value !== undefined && value !== null && step in Object(value), "path step " + step + " does not exist");
  return value[step];
}, root);

const validate = new Ajv({strict: true, allErrors: true}).compile(schema);
assert.equal(validate(model), true, JSON.stringify(validate.errors));
for (const test of cases.invalid_models) {
  const candidate = structuredClone(model);
  const parent = resolve(candidate, test.path.slice(0, -1));
  const key = test.path.at(-1);
  if (test.remove) {
    assert.ok(key in Object(parent), test.id + ": removed key exists");
    if (Array.isArray(parent)) parent.splice(key, 1); else delete parent[key];
  } else {
    assert.ok(test.add ? !(key in Object(parent)) : key in Object(parent), test.id + ": mutated key " + (test.add ? "is new" : "exists"));
    parent[key] = test.value;
  }
  assert.equal(validate(candidate), false, test.id);
}
console.log("BIM schema: authored model accepted; " + cases.invalid_models.length + " invalid models rejected.");

const alphabet = cases.guid.alphabet;
const encode = number => {
  assert.ok(number >= 0n && number < 1n << 128n);
  let text = "";
  for (let shift = 0n; shift < 132n; shift += 6n) text = alphabet[Number((number >> shift) & 63n)] + text;
  return text;
};
const uuidInteger = text => BigInt("0x" + text.replaceAll("-", ""));
const uuid5 = name => {
  const hash = crypto.createHash("sha1").update(Buffer.from(cases.guid.namespace.replaceAll("-", ""), "hex")).update(name, "utf8").digest().subarray(0, 16);
  hash[6] = (hash[6] & 0x0f) | 0x50;
  hash[8] = (hash[8] & 0x3f) | 0x80;
  return BigInt("0x" + hash.toString("hex"));
};
for (const vector of cases.guid.vectors) assert.equal(encode(uuidInteger(vector.uuid)), vector.globalId, vector.uuid);
const tagged = [model.project, model.site, model.building, ...["storeys", "types", "materials", "elements", "spaces", "relations"].flatMap(group => model[group])];
assert.ok(tagged.length > 100);
for (const item of tagged) {
  assert.equal(item.guid, encode(uuid5(cases.guid.seed + item.tag)), item.tag);
  assert.match(item.guid, /^[0-3][0-9A-Za-z_$]{21}$/);
}
assert.equal(new Set(tagged.map(item => item.guid)).size, tagged.length);
console.log("BIM GlobalIds: " + cases.guid.vectors.length + " vectors and " + tagged.length + " stored GlobalIds reproduced with BigInt and SHA-1.");

const manifestSchema = JSON.parse(fs.readFileSync(path.resolve(clip, "../../⚡️energy/🏗️authoring/🎬️clip/🧬️schema/🔣️.schema.json"), "utf8"));
const validateManifest = new Ajv({strict: true, allErrors: true}).compile(manifestSchema);
assert.equal(validateManifest(cases.source_validation.manifest), true, JSON.stringify(validateManifest.errors));
for (const test of cases.source_validation.invalid_structure) {
  const candidate = structuredClone(cases.source_validation.manifest);
  const parent = test.path.slice(0, -1).reduce((value, key) => value[key], candidate);
  parent[test.path.at(-1)] = test.value;
  assert.equal(validateManifest(candidate), false, test.id);
}
console.log("BIM selection metadata: valid fixture accepted; " + cases.source_validation.invalid_structure.length + " invalid structures rejected.");
