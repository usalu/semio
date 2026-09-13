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
    { op: "replace", path: "/catalogCursorClosed", value: true },
  ];
  const catalogTerminal = applyPatch({ pending: true, partialSymbolBytes: 2, partialSymbolScalars: 1, logicalItems: 3, allocatedBytes: 8192, catalogCursorClosed: false }, catalogRelease, true).newDocument;
  assert.deepEqual(catalogTerminal, corpus.retainedCatalog.terminal);
  assert.equal(corpus.retainedValue.constructionAllocates, false);
  assert.equal(corpus.retainedValue.diagnostic.storage, "inline");
  assert.equal(corpus.retainedValue.diagnostic.heapBytes, 0);
  assert.equal(corpus.retainedValue.diagnostic.publicStepAllocates, false);
  assert.equal(corpus.retainedValue.diagnostic.sticky, true);
  assert.equal(corpus.retainedValue.stack.maximumFrames, corpus.retainedValue.stack.maxDepth * corpus.retainedValue.stack.framesPerDepth);
  assert(corpus.retainedValue.stack.initialFrames <= corpus.retainedValue.stack.maximumFrames);
  assert.equal(corpus.retainedValue.stack.allocationBeforeInput, true);
  assert.equal(corpus.retainedValue.stack.zeroOrSubexactMutates, false);
  for (const symbol of corpus.retainedValue.recordBody.symbols) {
    assert.deepEqual([...encoder.encode(symbol.text)], symbol.utf8);
    assert.deepEqual([...symbol.text].map((scalar) => scalar.codePointAt(0)), symbol.scalars);
    assert.equal(decoder.decode(Uint8Array.from(symbol.utf8)), symbol.text);
  }
  assert(corpus.retainedValue.recordBody.multiLeaf.symbols > 1);
  assert.equal(corpus.retainedValue.recordBody.multiLeaf.positiveDemandBeyondFirstLeaf, true);
  assert.equal(corpus.retainedValue.recordBody.coordinateModel.firstUnrepresentableSymbol, 2 ** corpus.retainedValue.recordBody.coordinateModel.pointerBits);
  assert.equal(corpus.retainedValue.recordBody.coordinateModel.checkedConversion, true);
  assert.equal(corpus.retainedValue.recordBody.eventHandoff.finalScalarAndCatalogSameByte, true);
  assert.equal(corpus.retainedValue.recordBody.eventHandoff.preserveCatalogCompleteAcrossAllocation, true);
  assert.equal(corpus.retainedValue.recordBody.eventHandoff.blockIngressUntilObserved, true);
  assert.equal(corpus.retainedValue.recordBody.unopenedCancellation.zeroItemZeroByteMutates, false);
  assert.equal(corpus.retainedValue.recordBody.unopenedCancellation.bytesOnlyMetadataTransition, true);
  const valueRelease: Operation[] = [
    { op: "replace", path: "/pending", value: false },
    { op: "replace", path: "/stackFrames", value: 0 },
    { op: "replace", path: "/symbols", value: 0 },
    { op: "replace", path: "/scalars", value: 0 },
    { op: "replace", path: "/allocatedBytes", value: 0 },
    { op: "replace", path: "/closed", value: true },
  ];
  const valueTerminal = applyPatch({ pending: true, stackFrames: 2, symbols: 3, scalars: 7, allocatedBytes: 32768, closed: false }, valueRelease, true).newDocument;
  assert.deepEqual(valueTerminal, corpus.retainedValue.terminal);
  console.log("[DEBUG] Retained Pack physical source, catalog and value fixtures agree with Ajv 2020, fast-json-patch and platform UTF-8; value consumers=4 allocation-before-input=true terminal-ledgers=zero");
}
