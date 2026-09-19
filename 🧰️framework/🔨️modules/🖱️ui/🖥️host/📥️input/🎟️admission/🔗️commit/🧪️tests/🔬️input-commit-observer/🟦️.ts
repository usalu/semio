/** 🔬️ Canonical testInputCommitObserverFixture fixture and oracle checks. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { testSingleEnqueuePublicationFixture } from "../../📥️enqueue/🧪️tests/🔬️single-enqueue-publication/🟦️.ts";

/** 🔗️ The commit-observer ledger: what each observed commit publishes and what it retires. */
type InputCommitObserverFixture = {
  version: number;
  scope: string;
  fields: string[];
  old: {
    frameGeneration: string;
    acceptedInputGeneration: string;
    surfaceMetricsGeneration: string;
    completionNextRevision: string;
    completionReadyCount: string;
    sceneRevision: string;
    observedBuildInputGeneration: string;
    leHex: string;
  };
  half: {
    frameGeneration: string;
    acceptedInputGeneration: string;
    surfaceMetricsGeneration: string;
    completionNextRevision: string;
    completionReadyCount: string;
    sceneRevision: string;
    observedBuildInputGeneration: string;
    leHex: string;
  };
  committed: {
    frameGeneration: string;
    acceptedInputGeneration: string;
    surfaceMetricsGeneration: string;
    completionNextRevision: string;
    completionReadyCount: string;
    sceneRevision: string;
    observedBuildInputGeneration: string;
    leHex: string;
  };
  observations: {
    phase: string;
    outcome: string;
    pair: string;
  }[];
  independentUpdates: {
    old: {
      sceneRevision: string;
      inputGeneration: string;
    };
    half: {
      sceneRevision: string;
      inputGeneration: string;
    };
    committed: {
      sceneRevision: string;
      inputGeneration: string;
    };
    halfIsLegitimate: boolean;
  };
  invariants: {
    halfOfOneOperationAccepted: boolean;
    observedBuildInputChangesOnMetrics: boolean;
    readerRetries: number;
    readerBlocks: boolean;
    sourceFundingProven: boolean;
  };
};

export function testInputCommitObserverFixture(): void {
  testSingleEnqueuePublicationFixture();
  const fixture: InputCommitObserverFixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const tuple = (value: Record<string, string>): Buffer => {
    const bytes = Buffer.alloc(fixture.fields.length * 8);
    for (const [index, field] of fixture.fields.entries()) bytes.writeBigUInt64LE(BigInt(value[field]), index * 8);
    return bytes;
  };
  for (const name of ["old", "half", "committed"] as const) assert.equal(tuple(fixture[name]).toString("hex"), fixture[name].leHex);
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
  for (const patch of [{ halfOfOneOperationAccepted: true }, { readerRetries: 1 }, { readerBlocks: true }, { sourceFundingProven: true }] satisfies Partial<InputCommitObserverFixture["invariants"]>[]) {
    const candidate = structuredClone(fixture);
    Object.assign(candidate.invariants, patch);
    assert.equal(validate(candidate), false);
  }
  console.log("[DEBUG] input commit observer format oracle: 3 exact 56-byte tuples, 3 declared phases, 5 schema hostiles; independent updates remain legitimate; single-operation native interlock and funding unexecuted");
}
