import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020.js";
import { applyPatch, type Operation } from "fast-json-patch";
import corpus from "../../../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../../../🧬️schema/🔣️.json" with { type: "json" };

export function testRetainedPackPhysicalOwnership(): void {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  assert(validate(corpus), JSON.stringify(validate.errors));
  assert(corpus.physicalOwnership.pageItemMinimumBytes > corpus.admission.pageBytes);
  assert.equal(corpus.physicalOwnership.zeroGrant.mutates, false);
  const events = [
    { event: "begin", kind: "chunk" },
    ...corpus.multiByteChunk.bytes.map((value, index) => ({ event: "raw-byte", kind: "chunk", index, value })),
    { event: "complete", kind: "chunk" },
  ];
  const observations = events.filter(({ event, kind }) => event === corpus.multiByteChunk.observationEvent && kind === "chunk");
  assert.equal(observations.length, corpus.multiByteChunk.observations);
  assert.equal(events.filter(({ event }) => event === "raw-byte").length, corpus.multiByteChunk.rawByteEvents);
  assert.equal(corpus.outerCancellation.subexactGrant.mutates, false);
  assert.equal(corpus.outerCancellation.workFuelPerCloseOpportunity, 1);
  assert.deepEqual(corpus.outerCancellation.demandDelegation, ["field-decoder", "vcs", "snapshot", "mounted-source"]);
  const release: Operation[] = [
    { op: "replace", path: "/length", value: 0 },
    { op: "replace", path: "/capacity", value: 0 },
    { op: "replace", path: "/allocatedBytes", value: 0 },
    { op: "replace", path: "/rootCapacity", value: 0 },
  ];
  const terminal = applyPatch({ rootCapacity: 1, length: 1, capacity: 1, allocatedBytes: corpus.physicalOwnership.pageItemMinimumBytes }, release, true).newDocument;
  assert.deepEqual(terminal, corpus.physicalOwnership.terminal);
  console.log("[DEBUG] Retained Pack physical source fixture agrees with Ajv 2020 and fast-json-patch; chunk observations=1 raw-byte-events=5 terminal-ledger=zero");
}
