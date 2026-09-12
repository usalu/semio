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
  assert.deepEqual(corpus.outerCancellation.constructionCeilings, {
    singleDemand: "maximum-close-byte-demand",
    cumulativeRetained: "maximum-retained-close-bytes",
    oversizedNestedOwner: "returned-for-close",
  });
  const encoder = new TextEncoder();
  const decoder = new TextDecoder("utf-8", { fatal: true });
  for (const symbol of corpus.retainedCatalog.symbols) {
    assert.deepEqual([...encoder.encode(symbol.text)], symbol.utf8);
    assert.deepEqual([...symbol.text].map((scalar) => scalar.codePointAt(0)), symbol.scalars);
    assert.equal(decoder.decode(Uint8Array.from(symbol.utf8)), symbol.text);
  }
  const cumulativeBytes = corpus.retainedCatalog.cumulativeLimit.values.map((value) => encoder.encode(value).byteLength);
  assert(cumulativeBytes.every((bytes) => bytes <= corpus.retainedCatalog.cumulativeLimit.eachMaximumBytes));
  assert.equal(cumulativeBytes.reduce((sum, bytes) => sum + bytes, 0), corpus.retainedCatalog.cumulativeLimit.totalBytes);
  assert(cumulativeBytes.slice(0, corpus.retainedCatalog.cumulativeLimit.acceptedBeforeRefusal).reduce((sum, bytes) => sum + bytes, 0) <= corpus.retainedCatalog.cumulativeLimit.totalMaximumBytes);
  assert(cumulativeBytes.reduce((sum, bytes) => sum + bytes, 0) > corpus.retainedCatalog.cumulativeLimit.totalMaximumBytes);
  assert.equal(corpus.retainedCatalog.pageCrossing.leadByteIndex + 1, corpus.retainedCatalog.pageCrossing.pageBytes);
  assert.equal(decoder.decode(Uint8Array.from(corpus.retainedCatalog.pageCrossing.utf8)).codePointAt(0), corpus.retainedCatalog.pageCrossing.scalar);
  assert.throws(() => decoder.decode(Uint8Array.from(corpus.retainedCatalog.invalidUtf8.malformedAfterPrefix)));
  assert.throws(() => decoder.decode(Uint8Array.from(corpus.retainedCatalog.invalidUtf8.truncatedAfterPrefix)));
  const release: Operation[] = [
    { op: "replace", path: "/length", value: 0 },
    { op: "replace", path: "/capacity", value: 0 },
    { op: "replace", path: "/allocatedBytes", value: 0 },
    { op: "replace", path: "/rootCapacity", value: 0 },
  ];
  const terminal = applyPatch({ rootCapacity: 1, length: 1, capacity: 1, allocatedBytes: corpus.physicalOwnership.pageItemMinimumBytes }, release, true).newDocument;
  assert.deepEqual(terminal, corpus.physicalOwnership.terminal);
  const catalogRelease: Operation[] = [
    { op: "replace", path: "/pending", value: false },
    { op: "replace", path: "/partialSymbolBytes", value: 0 },
    { op: "replace", path: "/partialSymbolScalars", value: 0 },
    { op: "replace", path: "/logicalItems", value: 0 },
    { op: "replace", path: "/allocatedBytes", value: 0 },
    { op: "replace", path: "/receiptClosed", value: true },
  ];
  const catalogTerminal = applyPatch({ pending: true, partialSymbolBytes: 2, partialSymbolScalars: 1, logicalItems: 3, allocatedBytes: 8192, receiptClosed: false }, catalogRelease, true).newDocument;
  assert.deepEqual(catalogTerminal, corpus.retainedCatalog.terminal);
  console.log("[DEBUG] Retained Pack physical source and catalog fixtures agree with Ajv 2020, fast-json-patch and platform UTF-8; catalog symbols=3 pending-preserved=true terminal-ledgers=zero");
}
