/** 🔗️ Validates exact committed tuples with Node Buffer; this is not a native concurrency proof. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";
import { testSingleEnqueuePublicationFixture } from "./📥️enqueue/📜️script.ts";

//#region 🔗️ObserverOracle
export function testInputCommitObserverFixture(): void {
  testSingleEnqueuePublicationFixture();
  const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const tuple = (value: Record<string, string>): Buffer => {
    const bytes = Buffer.alloc(fixture.fields.length * 8);
    for (const [index, field] of fixture.fields.entries()) bytes.writeBigUInt64LE(BigInt(value[field]), index * 8);
    return bytes;
  };
  for (const name of ["old", "half", "committed"]) assert.equal(tuple(fixture[name]).toString("hex"), fixture[name].leHex);
  const half = tuple(fixture.half);
  assert.equal(half.equals(tuple(fixture.old)) || half.equals(tuple(fixture.committed)), fixture.invariants.halfOfOneOperationAccepted);
  assert.equal(fixture.old.observedBuildInputGeneration, fixture.committed.observedBuildInputGeneration);
  assert.equal(fixture.independentUpdates.halfIsLegitimate, true);
  for (const row of fixture.observations) {
    if (row.phase === "between-field-writes") { assert.equal(row.outcome, "busy"); assert.equal(row.pair, null); }
    else { assert.equal(row.outcome, "committed"); assert(["old", "committed"].includes(row.pair)); }
  }
  const hostile = structuredClone(fixture);
  hostile.observations[1] = { phase: "between-field-writes", outcome: "committed", pair: "half" };
  assert.equal(validate(hostile), false);
  for (const [key, value] of [["halfOfOneOperationAccepted", true], ["readerRetries", 1], ["readerBlocks", true], ["sourceFundingProven", true]] as const) {
    const candidate = structuredClone(fixture);
    candidate.invariants[key] = value;
    assert.equal(validate(candidate), false);
  }
  console.log("[DEBUG] input commit observer format oracle: 3 exact 56-byte tuples, 3 declared phases, 5 schema hostiles; independent updates remain legitimate; single-operation native interlock and funding unexecuted");
}
//#endregion 🔗️ObserverOracle
