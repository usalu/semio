import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import Ajv2020 from "ajv/dist/2020.js";
import fixture from "../🧫️fixtures/📚️paged-csr/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/📚️paged-csr/🔣️.json" with { type: "json" };

function sha256(values: readonly number[], kind: "u32" | "f64"): string {
  const bytes = Buffer.alloc(values.length * (kind === "u32" ? 4 : 8));
  values.forEach((value, index) => (kind === "u32" ? bytes.writeUInt32LE(value, index * 4) : bytes.writeDoubleLE(value, index * 8)));
  return createHash("sha256").update(bytes).digest("hex");
}

export function testFemPagedCsrOracle(): void {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  assert(!validate({ ...structuredClone(fixture), foreignOwner: true }), "strict fixture rejects a foreign owner field");
  const indptr = [0];
  const indices: number[] = [];
  const values: number[] = [];
  for (let row = 0; row < fixture.order; row++) {
    for (let column = Math.max(0, row - 1); column <= Math.min(fixture.order - 1, row + 1); column++) {
      indices.push(column);
      values.push(row === column ? (row === 0 || row + 1 === fixture.order ? 1 : 2) : -1);
    }
    indptr.push(indices.length);
  }
  assert.equal(indices.length, fixture.entryCount);
  assert.equal(indptr.at(-1), fixture.indptrTail);
  assert.equal(sha256(indices, "u32"), fixture.indicesSha256);
  assert.equal(sha256(values, "f64"), fixture.valuesSha256);
  const rule = fixture.vectorRule;
  const vector = Array.from({ length: fixture.order }, (_, index) => (((index * rule.multiplier) % rule.modulus) - rule.offset) / rule.divisor);
  const action = Array.from({ length: fixture.order }, (_, row) => {
    let value = 0;
    for (let entry = indptr[row]; entry < indptr[row + 1]; entry++) value += values[entry] * vector[indices[entry]];
    return value;
  });
  const tolerance = 1e-12;
  for (const sample of fixture.actionSamples) assert(Math.abs(action[sample.index] - sample.value) <= tolerance, `NumPy action sample ${sample.index}`);
  assert(Math.abs(action.reduce((sum, value) => sum + Math.abs(value), 0) - fixture.actionL1) <= 1e-10, "NumPy action L1");
  assert(indices.length * 4 > fixture.physicalPageBytes && values.length * 8 > fixture.physicalPageBytes, "both entry arrays cross one physical page");
  console.log(`[DEBUG] FEM paged CSR matches NumPy ${fixture.oracle.version} hashes and action across ${fixture.entryCount} entries`);
}

if (import.meta.main) testFemPagedCsrOracle();
