import assert from "node:assert/strict";
import Ajv from "ajv";
import { sparse } from "mathjs";
import fixture from "../🧫️fixtures/🧮️triplet-csr/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🧮️triplet-csr/🔣️.json" with { type: "json" };

export function verifyTripletCsrOracle(): void {
  const validate = new Ajv({ strict: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  for (const row of fixture.cases) {
    const dense = Array.from({ length: row.order }, () => Array<number>(row.order).fill(0));
    for (const entry of row.entries.toSorted((a, b) => a.sequence - b.sequence)) dense[entry.row]![entry.column]! += entry.value;
    assert.deepEqual(dense.flat(), row.matrix, row.id);
    if (row.order > 0) {
      const matrix = sparse(dense);
      assert.deepEqual(matrix.toArray().flat(), row.matrix, `${row.id} mathjs sparse representation`);
      for (let i = 0; i < row.order; i++) for (let j = 0; j < row.order; j++) assert.equal(matrix.get([i, j]), row.matrix[i * row.order + j]);
    }
  }
  console.log(`[DEBUG] ${fixture.cases.length} portable triplet CSR states match independent mathjs sparse matrices and original sequence accumulation`);
}
