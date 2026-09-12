import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020.js";
import { applyPatch, type Operation } from "fast-json-patch";
import fixture from "../🧫️fixtures/🔢️scalar-owners/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔢️scalar-owners/🔣️.json" with { type: "json" };

export function testFemScalarOwnerOracle(): void {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  assert(!validate({ ...fixture, foreignOwner: true }));
  const { lower, diagonal, matrix, rhs, expected } = fixture.factor;
  for (let row = 0; row < 3; row++) for (let column = 0; column < 3; column++) {
    assert.equal(lower[row].reduce((sum, value, index) => sum + value * diagonal[index] * lower[column][index], 0), matrix[row][column]);
    assert.equal(matrix[row].reduce((sum, value, index) => sum + value * expected[index][column], 0), rhs[row][column]);
  }
  let direction = [0, 0, 0];
  fixture.precondition.steps.forEach((step, index) => {
    const value = fixture.precondition.residual[index] / fixture.precondition.diagonal[index];
    const patch: Operation[] = [{ op: "replace", path: `/${index}`, value }];
    direction = applyPatch(direction, patch, true, false).newDocument;
    assert.deepEqual(direction, step);
  });
  const publication = fixture.publication;
  assert.deepEqual(publication.eigenvalues, [...publication.diagonal].sort((left, right) => left - right));
  for (const modes of publication.requestedModes) {
    let values = applyPatch(publication.previous, [{ op: "replace", path: "", value: [] }], true, false).newDocument;
    publication.steps.forEach((step, index) => {
      values = applyPatch(values, [{ op: "add", path: "/-", value: publication.eigenvalues[index] }], true, false).newDocument;
      assert.deepEqual(values, step);
    });
    assert.deepEqual(values.slice(0, modes), publication.eigenvalues.slice(0, modes));
  }
  console.log(`[DEBUG] FEM scalar owners match NumPy ${fixture.oracle.version}: three non-diagonal RHS columns, three retained precondition steps and two complete modal publication prefixes`);
}
