import assert from "node:assert/strict";
import Ajv from "ajv";
import { lusolve, sparse } from "mathjs";
import fixture from "../../🧫️fixtures/📚️paged-ldlt/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/📚️paged-ldlt/🔣️.json" with { type: "json" };

export function pagedLdltSolutionOracle(row: typeof fixture.cases[number]): number[] {
  const matrix = sparse(Array.from({ length: row.order }, (_, i) => Array.from({ length: row.order }, (_, j) => i === j ? row.diagonal : Math.abs(i - j) === 1 ? row.offDiagonal : 0)));
  return (lusolve(matrix, Array<number>(row.order).fill(1)).toArray() as number[][]).map(value => value[0]!);
}

export function verifyPagedLdltOracle(): void {
  const validate = new Ajv({ strict: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  for (const row of fixture.cases) {
    const solution = pagedLdltSolutionOracle(row);
    for (const sample of row.samples) assert(Math.abs(solution[sample.row]! - sample.solution) <= fixture.tolerance, `${row.id} original equation ${sample.row}`);
    for (let i = 0; i < row.order; i++) {
      const residual = row.diagonal * solution[i]! + (i > 0 ? row.offDiagonal * solution[i - 1]! : 0) + (i + 1 < row.order ? row.offDiagonal * solution[i + 1]! : 0) - 1;
      assert(Math.abs(residual) <= fixture.tolerance, `${row.id} sparse LU residual ${i}`);
    }
  }
  console.log("[DEBUG] independent mathjs sparse LU verifies orders1/41/513 and every original equation residual");
}
