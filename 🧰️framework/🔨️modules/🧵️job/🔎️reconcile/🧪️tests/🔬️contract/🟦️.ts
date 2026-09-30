import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import Ajv from "ajv";
import { parseJobReconcileRequestV1, parseJobReconcileResultV1 } from "../../🧬️schema/🟦️.ts";

/** 🧪️ Shared neutral vectors agree with the independent draft-07 validator. */
export function proveJobReconcileContract(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true }).addKeyword("x-semio-formats");
  const validators = {
    request: ajv.compile({ ...schema, $id: `${schema.$id}/request`, $ref: "#/$defs/JobReconcileRequestV1" }),
    result: ajv.compile({ ...schema, $id: `${schema.$id}/result`, $ref: "#/$defs/JobReconcileResultV1" }),
  };
  assert.equal(fixture.schema, "semio.framework.job-reconcile-fixture/v1");
  for (const row of fixture.cases) {
    let accepted = false;
    try {
      const decoded = row.kind === "request" ? parseJobReconcileRequestV1(row.value) : parseJobReconcileResultV1(row.value, (job) => job);
      assert.deepEqual(decoded, row.value);
      accepted = true;
    } catch {}
    assert.equal(accepted, row.accepted, row.name);
    assert.equal(validators[row.kind as keyof typeof validators](row.value), row.accepted, `${row.name}: Ajv`);
  }
  let calls = 0;
  const requestId = "1".repeat(32);
  const absent = parseJobReconcileResultV1({ schema: "semio.framework.job-reconcile-result/v1", version: 1, requestId, found: false, job: null }, () => { calls++; return {}; });
  assert.equal(absent.job, null);
  assert.equal(calls, 0);
  assert.throws(() => parseJobReconcileResultV1({ schema: "semio.framework.job-reconcile-result/v1", version: 1, requestId, found: true, job: {} }, () => { throw new Error("payload refused"); }), /payload refused/);
  console.log(`job-reconcile-contract: vectors=${fixture.cases.length} ajv+typescript=1 opaque-payload=1`);
  return fixture.cases.length;
}
