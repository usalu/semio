import assert from "node:assert/strict";
import Ajv from "ajv";
import { sparse } from "mathjs";
import fixture from "../🧫️fixtures/🎼️pencil/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🎼️pencil/🔣️.json" with { type: "json" };
type SparseJson = { ptr: number[]; index: number[]; values: number[] };

export function verifySymmetricPencilOracle(): void {
  const validate = new Ajv({ strict: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  for (const row of fixture.cases) {
    const first = sparse(Array.from({ length: row.order }, (_, i) => Array.from({ length: row.order }, (_, j) => i < j ? 0 : i === j ? row.kDiagonal : i === j + 1 ? row.kOffDiagonal : 0))).toJSON() as unknown as SparseJson;
    const second = sparse(Array.from({ length: row.order }, (_, i) => Array.from({ length: row.order }, (_, j) => i === j ? i % 2 === 0 ? row.bEvenDiagonal : row.bOddDiagonal : Math.abs(i - j) === 1 ? row.bOffDiagonal : 0))).toJSON() as unknown as SparseJson;
    assert.deepEqual([first.ptr, first.index, first.values], [row.kColptr, row.kRows, row.kValues], row.id);
    assert.deepEqual([second.ptr, second.index, second.values], [row.bIndptr, row.bIndices, row.bValues], row.id);
  }
  console.log("[DEBUG] three signed symmetric pencils match independent mathjs compressed matrices, including zero diagonal and order65");
}
