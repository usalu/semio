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
  assert.deepEqual(corpus.retainedPipelineDiagnostic.stages, ["anchor", "segment", "retained-varint", "retained-deflate"]);
  assert.equal(corpus.retainedPipelineDiagnostic.hotVariant, "retained-malformed-static");
  assert.equal(corpus.retainedPipelineDiagnostic.storage, "inline");
  assert.equal(corpus.retainedPipelineDiagnostic.heapBytes, 0);
  assert.equal(corpus.retainedPipelineDiagnostic.publicStepAllocates, false);
  assert.equal(corpus.retainedPipelineDiagnostic.firstFaultSticky, true);
  assert.equal(corpus.retainedPipelineDiagnostic.laterIngress, "rejected");
  assert.equal(corpus.retainedPipelineDiagnostic.close, "bounded-logical");
  assert.equal(corpus.retainedPipelineDiagnostic.physicalScope, "existing-lower-owners-only");
  assert.equal(corpus.retainedInflater.constructionAllocates, false);
  assert.equal(corpus.retainedInflater.historyBytes, 32 * 1024);
  assert.equal(corpus.retainedInflater.dynamicLengths, 286 + 32);
  assert.equal(corpus.retainedInflater.huffmanSymbols, 288);
  assert.deepEqual(corpus.retainedInflater.blocks, ["stored", "fixed", "dynamic", "window-wrap"]);
  assert.deepEqual(corpus.retainedInflater.distances, [1, 32768]);
  assert.deepEqual(corpus.retainedInflater.dynamicRepeatCodes, [16, 17, 18]);
  assert.equal(corpus.retainedInflater.allocationBeforeInput, true);
  assert.equal(corpus.retainedInflater.zeroOrSubexactMutates, false);
  assert.equal(corpus.retainedInflater.actualBackingAccounted, true);
  assert.equal(corpus.retainedInflater.reuseAcrossCompressedSegments, true);
  assert.equal(corpus.retainedInflater.identityAllocationBytes, 0);
  assert.deepEqual(corpus.retainedInflater.closeOrder, ["pending-input", "history-logical", "decoder-logical", "history-physical"]);
  assert.deepEqual(corpus.retainedInflater.consumers, ["generation2-snapshot", "generation3-snapshot"]);
  const inflaterRelease: Operation[] = [
    { op: "replace", path: "/pending", value: false },
    { op: "replace", path: "/historyLength", value: 0 },
    { op: "replace", path: "/historyCapacity", value: 0 },
    { op: "replace", path: "/allocatedBytes", value: 0 },
    { op: "replace", path: "/inflaterPresent", value: false },
    { op: "replace", path: "/closed", value: true },
  ];
  const inflaterTerminal = applyPatch({ pending: true, historyLength: 32768, historyCapacity: 32768, allocatedBytes: 32768, inflaterPresent: true, closed: false }, inflaterRelease, true).newDocument;
  assert.deepEqual(inflaterTerminal, corpus.retainedInflater.terminal);
  assert.equal(corpus.retainedTypedPersistence.status, "next-slice-contract");
  assert.deepEqual(corpus.retainedTypedPersistence.implementationOrder, ["typed-snapshot-candidate", "typed-mutation-builder", "spr-history"]);
  assert.equal(corpus.retainedTypedPersistence.constructionAllocates, false);
  assert.equal(corpus.retainedTypedPersistence.allocationBeforeIngress, true);
  assert(corpus.retainedTypedPersistence.limits.includes("actual-allocation-bytes"));
  assert.deepEqual(corpus.retainedTypedPersistence.snapshotConsumers, ["generation2-snapshot", "generation3-snapshot"]);
  assert.deepEqual(corpus.retainedTypedPersistence.mutationConsumers, ["generation2-mutation", "generation3-mutation"]);
  assert.deepEqual(corpus.retainedTypedPersistence.handoff, { kind: "owner-and-ledger-atomic", plainValue: false, retirementOwnerTransferred: true });
  assert.equal(corpus.retainedTypedPersistence.sprHistory.recordAtomic, true);
  assert.equal(corpus.retainedTypedPersistence.sprHistory.batchDecode, false);
  assert.deepEqual(corpus.retainedTypedPersistence.sprHistory.order, ["verify", "admit-record", "decode-record", "validate", "typed-replay", "retire-history"]);
  assert.equal(corpus.retainedTypedPersistence.zeroOrSubexactMutates, false);
  const typedRelease: Operation[] = [
    { op: "replace", path: "/pending", value: false },
    { op: "replace", path: "/typedItems", value: 0 },
    { op: "replace", path: "/historyRecords", value: 0 },
    { op: "replace", path: "/allocatedBytes", value: 0 },
    { op: "replace", path: "/ownerTransferred", value: false },
    { op: "replace", path: "/closed", value: true },
  ];
  const typedTerminal = applyPatch({ pending: true, typedItems: 9, historyRecords: 3, allocatedBytes: 65536, ownerTransferred: true, closed: false }, typedRelease, true).newDocument;
  assert.deepEqual(typedTerminal, corpus.retainedTypedPersistence.terminal);
  console.log("[DEBUG] Retained Pack physical source, catalog, value, diagnostic and inflater fixtures agree with Ajv 2020, fast-json-patch and platform UTF-8; history=32768 dynamic=318 distances=1,32768 terminal-ledgers=zero; typed-persistence=planned-owner-ledger-handoff");
}
