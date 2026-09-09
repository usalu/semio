import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🌿️ Validates GitHub event identity and checkout selection independently with Ajv and lodash. */
export async function testCiEnvironment(): Promise<void> {
  const require = createRequire(import.meta.url), lodash = require("lodash"), ajv = new (require("ajv"))();
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🌿️workflow-context/🔣️.json"), "utf8")), valid = ajv.compile(JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🔣️.json"), "utf8")));
  const { ciBaselineEnvironment } = await import("../../🟦️.ts");
  for (const row of fixture.cases) {
    const result = ciBaselineEnvironment({ ...fixture.environment, ...row.change }, fixture.event, fixture.head, false);
    assert.equal(valid(result), true, row.name); assert.equal(result.full, row.full, row.name); assert.equal(result.head, fixture.head);
    const oracleBranch = row.full ? null : row.change.GITHUB_EVENT_NAME === "pull_request" ? lodash.get(fixture.event, "pull_request.base.ref") : lodash.replace(fixture.environment.GITHUB_REF, "refs/heads/", "");
    assert.equal(oracleBranch, row.branch); assert.equal(result.history?.branch ?? null, row.branch);
    if (result.history) assert.deepEqual(result.history, { repository: "semio/fixture", repositoryId: 41, runId: 500, branch: row.branch });
  }
  assert.deepEqual(ciBaselineEnvironment({}, null, fixture.head, false), { head: fixture.head, full: true, history: null });
  assert.deepEqual(ciBaselineEnvironment(fixture.environment, fixture.event, fixture.head, true), { head: fixture.head, full: true, history: null });
  for (const row of fixture.invalid) assert.throws(() => ciBaselineEnvironment({ ...fixture.environment, ...row }, fixture.event, fixture.head, false), /Invalid|Unsupported/);
  assert.throws(() => ciBaselineEnvironment({ ...fixture.environment, GITHUB_EVENT_NAME: "pull_request" }, { ...fixture.event, pull_request: { base: { ref: "foreign", repo: { id: 99 } } } }, fixture.head, false), /Invalid/);
  assert.throws(() => ciBaselineEnvironment({}, null, "--HEAD", true), /Invalid/);
  console.log("[DEBUG] CI checkout/event identity, PR destination branch, schedule/tag/local full modes and Ajv/lodash contract PASS");
}
