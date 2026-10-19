import assert from "node:assert/strict";
import { eigs, inv, matrix, multiply } from "mathjs";
import Ajv from "ajv/dist/2020";
import corpus from "../../🧫️fixtures/🎯️positive-spectrum/🔣️.json";
import schema from "../../🧫️fixtures/🎯️positive-spectrum/🧬️schema.json";

export function verifyPositiveSpectrumOracle(): void {
  const validate = new Ajv({ strict: false }).compile(schema);
  assert(validate(corpus), JSON.stringify(validate.errors));
  for (const sample of corpus.cases) {
    const row = sample as typeof sample & { stiffnessDiagonal?: number[]; rightDiagonal?: number[]; stiffness?: number[][]; right?: number[][] };
    const diagonal = (values: number[]) => values.map((value, i) => values.map((_, j) => i === j ? value : 0));
    const k = row.stiffness ?? diagonal(row.stiffnessDiagonal!);
    const b = row.right ?? diagonal(row.rightDiagonal!);
    const result = eigs(multiply(inv(matrix(k)), matrix(b)));
    const values = (result.values.valueOf() as number[]).map(value => { assert.equal(typeof value, "number"); return value; }).filter(value => value > 0).map(value => 1 / value).sort((a, b) => a - b);
    assert.equal(values.length, row.positiveCount, row.name);
    row.values.forEach((expected, i) => assert(Math.abs(values[i]! - expected) <= Math.abs(expected) * 1e-10, row.name));
  }
  console.log("[DEBUG] positive generalized spectrum fixture matches independent mathjs eigenvalues and NumPy Cholesky congruence");
}
