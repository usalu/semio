/** 🔤️ Independent schema and byte expectations for the native BMP source contract. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

export function runBmpSourceHexFixtureChecks(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔤️source-hex/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔤️source-hex/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  assert.equal(new Set(fixture.cases.map((row: { id: string }) => row.id)).size, fixture.cases.length);
  let checks = 2;
  for (const row of fixture.cases) {
    const compact = row.source.replace(/\s/gu, "");
    assert.equal(/^(?:[0-9a-fA-F]{2})+$/u.test(compact), row.valid, row.id);
    checks++;
    if (row.valid) {
      assert.deepEqual([...Buffer.from(compact, "hex")], fixture.bytes, row.id);
      checks++;
    }
  }
  console.log(`[DEBUG] BMP source-hex fixture: ${fixture.cases.length} cases, Ajv and independent bytes checked`);
  return checks;
}
