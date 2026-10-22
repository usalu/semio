import assert from "node:assert/strict";
import { add, inv, matrix, multiply } from "mathjs";
import Ajv from "ajv/dist/2020";
import corpus from "../🧫️fixtures/🔄️shift-operator/🔣️.json";
import schema from "../🧫️fixtures/🔄️shift-operator/🧬️schema.json";

type Dense = number[][];

/** 🧭️ The shifted operator image `y = K⁻¹Bx + σx` and its stiffness image `Ky = Bx + σKx` agree with the NumPy values and the K-equation. */
export function verifyShiftOperatorOracle(): void {
  const validate = new Ajv({ strict: false }).compile(schema);
  assert(validate(corpus), JSON.stringify(validate.errors));
  for (const row of corpus.cases) {
    const k = matrix(row.stiffness), b = matrix(row.right), x = matrix(row.x);
    const bx = multiply(b, x);
    const rhs = add(bx, multiply(row.shift, multiply(k, x))).valueOf() as Dense;
    const solved = add(multiply(inv(k), bx), multiply(row.shift, x)).valueOf() as Dense;
    const close = (actual: Dense, expected: Dense, label: string) => actual.forEach((line, i) => line.forEach((value, j) => assert(Math.abs(value - expected[i]![j]!) <= corpus.relativeTolerance * Math.max(1, Math.abs(expected[i]![j]!)), `${row.name} ${label}[${i}][${j}]`)));
    close(solved, row.solved, "solved");
    close(rhs, row.rhs, "rhs");
    close(multiply(k, matrix(solved)).valueOf() as Dense, row.rhs, "K y");
  }
  console.log(`[DEBUG] ${corpus.cases.length} shift-operator states match independent mathjs and NumPy operator images`);
}
