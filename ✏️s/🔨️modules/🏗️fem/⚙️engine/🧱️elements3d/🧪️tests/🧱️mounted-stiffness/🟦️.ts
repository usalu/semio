import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020.js";
import { applyPatch, type Operation } from "fast-json-patch";
import fixture from "../🧫️fixtures/🧱️mounted-stiffness/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🧱️mounted-stiffness/🔣️.json" with { type: "json" };

export function testFem3dMountedStiffnessOracle(): void {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  assert(!validate({ ...structuredClone(fixture), foreignOwner: true }), "strict fixture must reject a foreign owner field");
  assert.equal(fixture.oracle.package, "numpy");
  for (const row of fixture.cases) {
    const side = Math.sqrt(row.matrix.length);
    assert(Number.isInteger(side), `${row.kind} square matrix`);
    const patch = row.matrix.map((value, index) => ({ op: "replace", path: `/${index}`, value })) as Operation[];
    const reconstructed = applyPatch(new Array<number>(row.matrix.length).fill(0), patch, true, false).newDocument;
    assert.deepEqual(reconstructed, row.matrix, `${row.kind} cell reconstruction`);
    const scale = Math.max(1, ...row.matrix.map(Math.abs));
    for (let rowIndex = 0; rowIndex < side; rowIndex++) {
      for (let column = 0; column < side; column++) {
        assert(Math.abs(row.matrix[rowIndex * side + column] - row.matrix[column * side + rowIndex]) <= fixture.tolerance * scale, `${row.kind} symmetry (${rowIndex}, ${column})`);
      }
    }
    const dofs = row.kind === "frame3" ? 6 : 3;
    for (let rowIndex = 0; rowIndex < side; rowIndex++) {
      for (let axis = 0; axis < 3; axis++) {
        let rigidTranslation = 0;
        for (let node = 0; node < row.nodeIds.length; node++) rigidTranslation += row.matrix[rowIndex * side + node * dofs + axis];
        assert(Math.abs(rigidTranslation) <= fixture.tolerance * scale * side, `${row.kind} rigid translation row ${rowIndex} axis ${axis}`);
      }
    }
  }
  console.log(`[DEBUG] FEM3D mounted node/cell fixture validates and reconstructs three NumPy ${fixture.oracle.version} reference matrices`);
}

if (import.meta.main) testFem3dMountedStiffnessOracle();
