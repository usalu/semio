import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🧭️ Exercises event-to-history-to-ancestry resolution and validates its result with Ajv. */
export async function testCiResolution(): Promise<void> {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧭️baseline-resolution/🔣️.json"), "utf8"));
  const input = JSON.parse(readFileSync(join(import.meta.dir, "../../../🌿️environment/🧫️fixtures/🌿️workflow-context/🔣️.json"), "utf8"));
  const github = JSON.parse(readFileSync(join(import.meta.dir, "../../../../🐙️github/🧫️fixtures/🐙️workflow-history/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../../🧬️schema/🔣️.json"), "utf8")), valid = new (createRequire(import.meta.url)("ajv"))().compile({ ...schema, $ref: "#/definitions/selection" });
  const { resolveCiValidation } = await import("../../🟦️.ts");
  for (const row of fixture.cases) {
    let calls = 0;
    const result = await resolveCiValidation({ environment: input.environment, event: input.event, head: fixture.head, full: row.full }, async path => {
      calls++; return { status: row.status, body: path.includes("/actions/runs/") ? github.current : { total_count: 1, workflow_runs: [{ ...github.run, head_sha: fixture.base, head_branch: "⛳️integration" }] } };
    }, async (base, head) => { assert.equal(base, fixture.base); assert.equal(head, fixture.head); return row.ancestor; }, new AbortController().signal);
    assert.equal(valid(result.baseline), true, row.name); assert.equal(result.baseline.mode, row.mode, row.name); assert.equal(calls, row.requests, row.name);
    assert.equal(result.baseline.base, row.mode === "affected" ? fixture.base : null);
    assert.equal(result.historyIssue, row.status === 403 ? "history-unavailable" : null);
  }
  console.log("[DEBUG] CI environment, paginated provider and ancestor selector compose into a schema-valid fail-closed baseline PASS");
}
