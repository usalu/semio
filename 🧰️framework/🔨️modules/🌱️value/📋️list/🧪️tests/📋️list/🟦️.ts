import assert from "node:assert/strict";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import corpus from "../../🧫️fixtures/🔣️.json" with { type: "json" };

export function testPagedListOwnership(): void {
  const operations: Operation[] = Array.from({ length: corpus.ordered.count }, (_, value) => ({ op: "add", path: "/-", value }));
  const values = applyPatch<number[]>([], operations, true).newDocument;
  assert.deepEqual(values, Array.from({ length: corpus.capacity.maximum }, (_, value) => value));
  assert.equal(values.reduce((sum, value) => sum + value, 0), corpus.ordered.sum);
  assert.equal(corpus.capacity.rejected, values.length + 1);
  assert(corpus.oversized.elementBytes > corpus.oversized.grantBytes);
  for (const row of corpus.counter.signedLimits) {
    const limit = BigInt(row.limit);
    assert.equal(limit, (1n << BigInt(row.bits - 1)) - 1n);
    assert(BigInt(row.before) + BigInt(row.requested) <= limit);
    const retained = BigInt(row.before) + BigInt(row.actual);
    assert(retained > limit);
    assert(BigInt(row.actual) <= BigInt(row.grant));
    const result = applyPatch({ allocated: row.before, result: "admitted" }, [
      { op: "replace", path: "/allocated", value: retained.toString() },
      { op: "replace", path: "/result", value: "rejected-owner-retained" },
    ], true).newDocument;
    assert.deepEqual(result, { allocated: row.retained, result: row.result });
    assert.equal(retained - BigInt(row.actual), BigInt(row.before));
  }
}
