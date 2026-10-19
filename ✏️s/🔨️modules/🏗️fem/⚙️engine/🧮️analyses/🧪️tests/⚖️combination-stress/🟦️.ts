import assert from "node:assert/strict";
import Ajv from "ajv";
import { eigs } from "mathjs";
import schema from "../../🧬️schema/⚖️combination-stress/🔣️.json";
import fixture from "../🧫️fixtures/⚖️combination-stress/🔣️.json";

/** 🔮️ Independently checks equivalent stress through mathjs principal-stress eigenvalues. */
export function verifyCombinationStressOracle(): void {
  const validate = new Ajv({ strict: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  for (const vector of fixture.cases) {
    const scale = vector.kind === "solid" ? 6 : 10;
    const stress = vector.loads[0]!.map((value, slot) => scale * (vector.factors[0]! * value + vector.factors[1]! * vector.loads[1]![slot]!));
    assert.deepEqual(stress, vector.expectedStress);
    const values = eigs([[stress[0]!, 0, 0], [0, stress[1]!, 0], [0, 0, stress[2]!]]).values as number[];
    const equivalent = Math.hypot(values[0]! - values[1]!, values[1]! - values[2]!, values[2]! - values[0]!) / Math.SQRT2;
    assert(Math.abs(equivalent - vector.expectedVonMises) < 1e-12 * Math.max(1, equivalent), vector.id);
    console.log(`[DEBUG] ${vector.id}: mathjs principal-stress oracle = ${equivalent}`);
  }
}
