const assert = require("node:assert/strict");
const Ajv = require("ajv/dist/2020");
const fixture = require("./🔣️.json");
const schema = require("./🧬️schema/🔣️.json");
const validate = new Ajv().compile(schema);
assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
for (const test of fixture.cases) {
  const valid = /^\+?[0-9]+$/.test(test.text) && BigInt(test.text) <= 18446744073709551615n;
  assert.equal(valid, !test.refused, test.text);
  if (valid) assert.equal(BigInt(test.text).toString(), test.value, test.text);
}
