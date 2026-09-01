/** 🪪️ Validates concrete runtime-owner fixtures without claiming native execution. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";
import _ from "lodash";

//#region 🪪️RuntimeIdentityOracle
export function testDirectoryRuntimeIdentityFixture(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8")));
  assert(validate(fixture), JSON.stringify(validate.errors));
  const owners: Record<string, object> = { original: { workers: 2 }, "foreign-equal-workers": { workers: 2 } };
  for (const row of fixture.identity) {
    assert.equal(Object.is(owners[row.left], owners[row.right]), row.same);
    assert.equal(_.eq(owners[row.left], owners[row.right]), row.same);
  }
  assert(_.isEqual(owners.original, owners["foreign-equal-workers"]));
  assert.equal(_.eq(owners.original, owners["foreign-equal-workers"]), false);
  assert.deepEqual(fixture.cases.map((row: { workers: number }) => row.workers), [1, 2, 3]);
  for (const hostile of [{ ...fixture, provider: "semio_framework_async::TokioHostRuntime" }, { ...fixture, cases: [] }, { ...fixture, invariants: { ...fixture.invariants, originalRuntime: false } }, { ...fixture, extra: true }]) assert.equal(validate(hostile), false);
  console.log("[DEBUG] Directory runtime schema/Lodash oracle: 3 pool sizes, 2 reference-identity cases, 4 hostiles; native constructor identity unexecuted by this command");
}
//#endregion 🪪️RuntimeIdentityOracle
