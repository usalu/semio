import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

/** 🎞️ Independent JSON Schema admission for supplied-owner media witnesses. */
export function mediaOwnerContextOracle(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎞️media-owner-context.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(new URL("../../🧪️testing/🎞️media-owner-context/🧬️schema/🔣️.json", import.meta.url), "utf8")));
  assert.equal(fixture.cases.length, 2);
  assert.deepEqual(fixture.refusals, ["foreign-owner", "closed-owner", "unknown-port"]);
  let assertions = 0;
  for (const row of fixture.cases) {
    assert.deepEqual(row.expected, { owner: row.owner, count: row.count });
    assert.equal(validate(row.expected), true);
    assertions++;
  }
  for (const row of fixture.invalid) { assert.equal(validate(row), false); assertions++; }
  assert.equal(assertions, 6);
  assert.notDeepEqual(fixture.cases[0].expected, fixture.cases[1].expected);
  return assertions;
}
