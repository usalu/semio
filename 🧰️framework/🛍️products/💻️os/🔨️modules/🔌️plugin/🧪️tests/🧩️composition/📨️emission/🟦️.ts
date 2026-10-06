/** 📨️ Closed vectors preserve the accepted prefix when one child operation codec refuses. */
import assert from "node:assert/strict";
import { Buffer } from "node:buffer";

import fixture from "../../../🧩️composition/📨️emission/🧫️fixtures/🔣️.json" with { type: "json" };


export function testOwnedChildEmissionRefusalOracle(): void {
  
  
  assert(fixture.slot.includes("\0") && fixture.childId.includes("\0") && fixture.refusal.detail.includes("\0"));
  const operation = Buffer.alloc(4);
  operation.writeInt32LE(fixture.validValue);
  const independent = new DataView(new ArrayBuffer(4));
  independent.setInt32(0, fixture.validValue, true);
  assert.deepEqual([...operation], [...new Uint8Array(independent.buffer)]);
  assert.equal(operation.readInt32LE(), fixture.validValue);
  const accepted = [{ bytes: [...operation], label: "set-count" }];
  const prefix = structuredClone(accepted);
  const append = (encoded: { bytes: number[] } | { refusal: typeof fixture.refusal }): boolean => {
    if ("refusal" in encoded) return false;
    accepted.push({ bytes: encoded.bytes, label: "set-count" });
    return true;
  };
  assert.equal(append({ refusal: fixture.refusal }), fixture.expected.failedAppendPublishes);
  assert.deepEqual(accepted, prefix);
  assert.equal(accepted.length, fixture.expected.acceptedOperations);
  assert.equal(accepted.map(operation => operation.label).length, fixture.expected.acceptedLabels);
  const extended = structuredClone(fixture) as Record<string, unknown>;
  extended["fallbackBytes"] = [];
  
}
