import { expect, test } from "bun:test";
import Ajv from "ajv";
import { createRequire } from "node:module";
import policy from "../🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import examples from "../🧫️fixtures/🔣️.json";
import { browserActorVisibleSurfacesV1, type BrowserActorVisibleTurnV1 } from "../🟦️.ts";

const require = createRequire(import.meta.url);
test("visible surface policy is admitted independently and contains no examples", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  const oracle = require("jsonschema");
  for (const value of [policy, { ...policy, cases: examples.cases }, { ...policy, turns: {} }, { ...policy, sections: [] }]) expect(validate(value)).toBe(oracle.validate(value, schema).valid);
  expect(validate(policy)).toBe(true);
  expect(validate({ ...policy, cases: examples.cases })).toBe(false);
});
for (const row of examples.cases) test(row.name, () => {
  const actual = browserActorVisibleSurfacesV1(row.surfaces, row.turn as BrowserActorVisibleTurnV1);
  expect(require("fast-deep-equal")(actual, row.visible)).toBe(true);
  expect(actual).toEqual(row.visible);
});
