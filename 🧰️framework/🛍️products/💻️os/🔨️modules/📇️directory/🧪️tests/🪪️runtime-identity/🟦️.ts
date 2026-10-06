import { fileURLToPath as testFileUrlToPath } from "node:url";
const testSourceUrl = new URL("../../🔌️client/🪪️runtime/📜️script.ts", import.meta.url);
/** 🪪️ Validates concrete runtime-owner fixtures without claiming native execution. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import _ from "lodash";

//#region 🪪️RuntimeIdentityOracle
export function testDirectoryRuntimeIdentityFixture(): void {
  const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", testSourceUrl.href), "utf8"));
  const owners: Record<string, object> = { original: { workers: 2 }, "foreign-equal-workers": { workers: 2 } };
  for (const row of fixture.identity) {
    assert.equal(Object.is(owners[row.left], owners[row.right]), row.same);
    assert.equal(_.eq(owners[row.left], owners[row.right]), row.same);
  }
  assert(_.isEqual(owners.original, owners["foreign-equal-workers"]));
  assert.equal(_.eq(owners.original, owners["foreign-equal-workers"]), false);
  assert.deepEqual(fixture.cases.map((row: { workers: number }) => row.workers), [1, 2, 3]);
  console.log(`[DEBUG] directory runtime identity independent-comparisons=${fixture.identity.length} workers=${fixture.cases.length}`);
}
//#endregion 🪪️RuntimeIdentityOracle
