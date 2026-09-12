import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020.js";
import { applyPatch, type Operation } from "fast-json-patch";
import fixture from "../🧫️fixtures/🔀️merge-refusal/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔀️merge-refusal/🔣️.json" with { type: "json" };

type Triplet = { sequence: number; row: number; column: number; value: number };
type State = { sourceCursor: number; candidate: Triplet | null; destination: Triplet[] };

function attempt(state: State, capacity: number): State {
  const next = structuredClone(state);
  if (next.candidate === null || next.destination.length >= capacity) return next;
  next.destination.push(next.candidate);
  next.candidate = null;
  next.sourceCursor += 1;
  return next;
}

export function testFemAssemblyMergeRefusalOracle(): void {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  assert(!validate({ ...structuredClone(fixture), foreignOwner: true }), "strict fixture rejects a foreign owner field");
  for (const row of fixture.cases) {
    const initial: State = { sourceCursor: row.sourceCursor, candidate: row.candidate, destination: row.destination };
    const refused = attempt(initial, row.destinationCapacity);
    assert.deepEqual(refused, row.refused, `${row.lane} refusal retains cursor, candidate, and destination`);
    const retirement = [{ op: "remove", path: `/destination/${refused.destination.length - 1}` }] as Operation[];
    const available = applyPatch(structuredClone(refused), retirement, true, false).newDocument as State;
    assert.deepEqual(attempt(available, row.destinationCapacity), row.retried, `${row.lane} retry publishes once and advances once`);
  }
  console.log(`[DEBUG] FEM assembly merge refusal retains and retries both exact candidates under fast-json-patch ${fixture.oracle.version}`);
}

if (import.meta.main) testFemAssemblyMergeRefusalOracle();
