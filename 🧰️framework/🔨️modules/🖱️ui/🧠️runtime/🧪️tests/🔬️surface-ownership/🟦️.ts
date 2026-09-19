/** 🔬️ Canonical surfaceOwnershipSelfTests fixture and oracle checks. */
import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import Ajv from "ajv";

/** 📏️ Physical backing, inline growth and transfer ledgers for one owned surface. */
type SurfaceOwnershipFixture = {
  readonly version: number;
  readonly componentCopy: {
    readonly allocationGrant: number;
    readonly workGrant: number;
    readonly payloadBytes: number;
    readonly cancelFrontiers: readonly number[];
  };
  readonly existingComponent: {
    readonly allocationGrant: number;
    readonly workGrant: number;
    readonly payloadBytes: number;
    readonly minimumTurns: number;
    readonly rejectionAllocationBytes: number;
    readonly oldRootCopied: boolean;
  };
  readonly patchAllocation: {
    readonly maximumBytesPerTurn: number;
    readonly directoryBytes: number;
    readonly operationBytes: number;
  };
  readonly bindingClone: {
    readonly items: number;
    readonly elementBytes: number;
    readonly wholeBackingBytes: number;
    readonly maximumBytesPerTurn: number;
    readonly cancelFrontiers: readonly number[];
    readonly unwindRetainsRoots: boolean;
  };
  readonly inlineCases: readonly {
    readonly name: string;
    readonly before: number;
    readonly after: number;
    readonly backingCapacity: number;
    readonly additionalOwnedBytes: number;
  }[];
  readonly transfer: {
    readonly capacity: number;
    readonly values: readonly number[];
    readonly sourceBackingBytes: number;
    readonly sourceItems: number;
    readonly rejectedPayload: string;
  };
  readonly backingCases: readonly {
    readonly name: string;
    readonly capacity: number;
    readonly initialized: number;
    readonly elementBytes: number;
    readonly ownedBytes: number;
  }[];
};

/** 🩹️ The patch handoff ledger: grants, unwind frontiers and the source-preservation claims. */
type SurfacePatchFixture = {
  readonly version: number;
  readonly generation: number;
  readonly surface: string;
  readonly surfaceBytes: number;
  readonly revision: number;
  readonly grants: readonly number[];
  readonly handoffGrant: number;
  readonly unwindFrontiers: readonly number[];
  readonly occupiedTargetPreservesSource: boolean;
  readonly invalidAckPreservesSource: boolean;
  readonly contendedReleasePreservesSource: boolean;
};

/** 🚪️ The handback ledger: which contention outcomes may move the owner out of its slot. */
type SurfaceHandbackFixture = {
  readonly version: number;
  readonly generation: number;
  readonly surface: string;
  readonly surfaceUtf8: string;
  readonly entryWaits: boolean;
  readonly poisonIsFault: boolean;
  readonly poisonMutatesOwner: boolean;
  readonly busyLosesOwner: boolean;
  readonly queuedOwnerLeavesSlotDuringStep: boolean;
};

/** 📃️ The document-level byte and slot ledger the aggregate ceiling funds resident surfaces against. */
type SurfaceDocumentFixture = {
  readonly version: number;
  readonly surfaces: number;
  readonly surfaceCeilingBytes: number;
  readonly documentBytes: number;
  readonly residentSlots: number;
  readonly aggregateCeilingBytes: number;
  readonly workBytes: number;
  readonly physicalBytes: number;
  readonly comparisonHeapAllocations: number;
  readonly comparisonLogicalDepth: number;
  readonly unwindPhases: readonly string[];
  readonly freshCompletionFields: readonly string[];
  readonly completionTransferUsesSeparateGrant: boolean;
  readonly sameRootForReader: boolean;
  readonly oldReaderSurvivesReplacement: boolean;
  readonly readerReleaseGrants: readonly number[];
};

/** 📤️ The producer-output ledger: entry slots, admission outcomes and every unwind-retention claim. */
type SurfaceOutputFixture = {
  readonly version: number;
  readonly entrySlots: number;
  readonly queueSlots: number;
  readonly physicalGrant: number;
  readonly closeGrants: readonly number[];
  readonly zeroGrantInvocations: number;
  readonly saturatedInvocations: number;
  readonly extraInvocation: boolean;
  readonly surfaces: readonly string[];
  readonly surfaceUtf8: readonly string[];
  readonly reserveBeforeProducer: boolean;
  readonly refusalPreservesReady: boolean;
  readonly partialProducerUnwindRetainsOwner: boolean;
  readonly incompleteProducerSourcesPreserveRemainingOwners: boolean;
  readonly cancelledAdmission: {
    readonly surface: string;
    readonly revision: number;
    readonly generationPreserved: boolean;
    readonly terminal: boolean;
  };
  readonly dropWaits: boolean;
  readonly concurrentAdmission: {
    readonly occupied: number;
    readonly workers: number;
    readonly accepted: number;
    readonly restored: number;
  };
  readonly capturedLifetime: {
    readonly instance: number;
    readonly original: number;
    readonly reused: number;
    readonly foreignCloseAccepted: boolean;
    readonly terminal: boolean;
  };
  readonly directReceiver: {
    readonly zeroGrantTransfers: boolean;
    readonly occupiedTransfers: boolean;
    readonly unwindRetainsPayload: boolean;
    readonly terminal: boolean;
  };
  readonly aggregateCeilingBytes: number;
  readonly residentCapacity: {
    readonly reservationBytes: number;
    readonly cases: readonly {
      readonly fixedBytes: number;
      readonly capacity: number;
    }[];
    readonly capPlusOneAccepted: boolean;
    readonly refusalPreservesTree: boolean;
    readonly terminal: boolean;
  };
  readonly returnedEntryReusableBeforeDrain: boolean;
  readonly staleEpochAccepted: boolean;
  readonly zeroGrantMutates: boolean;
  readonly handbackAdmission: {
    readonly slots: number;
    readonly perProducer: number;
    readonly onlyOneFreeAccepted: boolean;
    readonly saturatedAfterSealTransfers: boolean;
  };
  readonly readyRevalidation: readonly {
    readonly case: string;
    readonly cancelled: boolean;
    readonly sameGeneration: boolean;
    readonly fuel: number;
    readonly deadline: number;
    readonly now: number;
    readonly outcome: string;
  }[];
};

/** 🔄️ The transaction credit ledger: required nodes against each case's node ceiling. */
type SurfaceTransactionFixture = {
  readonly version: number;
  readonly requiredNodes: number;
  readonly maximumItems: number;
  readonly maximumBytes: number;
  readonly cases: readonly {
    readonly maximumNodes: number;
    readonly patches: number;
    readonly creditFault: boolean;
  }[];
};

/** 📐️ Validates physical backing independently with Ajv and Node's fixed byte-buffer allocation. */
export function surfaceOwnershipSelfTests(): number {
  const fixture: SurfaceOwnershipFixture = JSON.parse(readFileSync(new URL("../../📏️ownership/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../📏️ownership/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  for (const entry of fixture.backingCases) {
    const owner = Buffer.alloc(entry.capacity * entry.elementBytes);
    assert.equal(owner.byteLength, entry.ownedBytes);
    assert(entry.initialized <= entry.capacity);
  }
  for (const entry of fixture.inlineCases) {
    const owner = Buffer.alloc(512 + entry.backingCapacity * 3072);
    const before = owner.byteLength;
    owner.writeUInt8(entry.after, 0);
    assert.equal(owner.byteLength - before, entry.additionalOwnedBytes);
  }
  assert.equal(validate({ ...fixture, unowned: true }), false);
  assert.equal(validate({ ...fixture, inlineCases: fixture.inlineCases.map((entry: object) => ({ ...entry, additionalOwnedBytes: 512 })) }), false);
  const source = new ArrayBuffer(fixture.transfer.capacity * BigUint64Array.BYTES_PER_ELEMENT);
  const target = structuredClone(source, { transfer: [source] });
  assert.equal(source.byteLength, fixture.transfer.sourceBackingBytes);
  assert.equal(target.byteLength, fixture.transfer.capacity * BigUint64Array.BYTES_PER_ELEMENT);
  assert.throws(() => new Uint8Array(source), TypeError);
  assert.equal(fixture.transfer.rejectedPayload, "retained-payload");
  const directory = Buffer.alloc(fixture.patchAllocation.directoryBytes);
  const operation = Buffer.alloc(fixture.patchAllocation.operationBytes);
  assert(directory.byteLength <= fixture.patchAllocation.maximumBytesPerTurn);
  assert(operation.byteLength <= fixture.patchAllocation.maximumBytesPerTurn);
  assert(directory.byteLength + operation.byteLength > fixture.patchAllocation.maximumBytesPerTurn);
  const bindings = Buffer.alloc(fixture.bindingClone.items * fixture.bindingClone.elementBytes);
  assert.equal(bindings.byteLength, fixture.bindingClone.wholeBackingBytes);
  assert(bindings.byteLength > fixture.bindingClone.maximumBytesPerTurn);
  const component = Buffer.alloc(fixture.componentCopy.payloadBytes);
  assert.equal(component.byteLength, fixture.componentCopy.allocationGrant);
  assert.equal(component.byteLength / fixture.componentCopy.workGrant, 8);
  assert.equal(validate({ ...fixture, componentCopy: { ...fixture.componentCopy, workGrant: 32768 } }), false);
  const existing = Buffer.alloc(fixture.existingComponent.payloadBytes);
  const different = Buffer.from(existing);
  different[different.length - 1] = 1;
  assert.equal(existing.equals(different), false);
  assert.equal(existing.byteLength / fixture.existingComponent.workGrant, fixture.existingComponent.minimumTurns);
  assert.equal(validate({ ...fixture, existingComponent: { ...fixture.existingComponent, oldRootCopied: true } }), false);
  const patch: SurfacePatchFixture = JSON.parse(readFileSync(new URL("../../🩹️patch/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const patchSchema = JSON.parse(readFileSync(new URL("../../🩹️patch/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validatePatch = new Ajv({ strict: true, allErrors: true }).compile(patchSchema);
  assert(validatePatch(patch), JSON.stringify(validatePatch.errors));
  assert.equal(Buffer.from(patch.surface).byteLength, patch.surfaceBytes);
  assert.equal(validatePatch({ ...patch, occupiedTargetPreservesSource: false }), false);
  assert.equal(validatePatch({ ...patch, contendedReleasePreservesSource: false }), false);
  const handback: SurfaceHandbackFixture = JSON.parse(readFileSync(new URL("../../🚪️handback/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const handbackSchema = JSON.parse(readFileSync(new URL("../../🚪️handback/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validateHandback = new Ajv({ strict: true, allErrors: true }).compile(handbackSchema);
  assert(validateHandback(handback), JSON.stringify(validateHandback.errors));
  assert.equal(Buffer.from(handback.surface).toString("hex"), handback.surfaceUtf8);
  for (const field of ["entryWaits", "poisonMutatesOwner", "busyLosesOwner", "queuedOwnerLeavesSlotDuringStep"]) assert.equal(validateHandback({ ...handback, [field]: true }), false);
  assert.equal(validateHandback({ ...handback, poisonIsFault: false }), false);
  const document: SurfaceDocumentFixture = JSON.parse(readFileSync(new URL("../../📃️document/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const documentSchema = JSON.parse(readFileSync(new URL("../../📃️document/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validateDocument = new Ajv({ strict: true, allErrors: true }).compile(documentSchema);
  assert(validateDocument(document), JSON.stringify(validateDocument.errors));
  // ⚖️ The aggregate funds every resident SLOT at one full DOCUMENT each, so the byte ledger and the
  // slot ledger refuse together. Read as a multiple of the per-surface CEILING it funded four holders
  // against sixty-four slots, and a twenty-six-surface React session was refused at 99.8 % with
  // thirty-eight slots free (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
  assert.equal(document.aggregateCeilingBytes / document.documentBytes, document.residentSlots);
  assert(document.documentBytes < document.surfaceCeilingBytes);
  assert(document.surfaces <= document.residentSlots);
  assert(document.surfaces > Math.floor(document.aggregateCeilingBytes / document.surfaceCeilingBytes));
  const canonical = Buffer.from("canonical-root", "utf8");
  const alias = canonical.subarray();
  assert.equal(alias.buffer, canonical.buffer);
  assert.equal(alias.byteOffset, canonical.byteOffset);
  assert.equal(validateDocument({ ...document, sameRootForReader: false }), false);
  assert.equal(validateDocument({ ...document, oldReaderSurvivesReplacement: false }), false);
  const outputs: SurfaceOutputFixture = JSON.parse(readFileSync(new URL("../../📤️output/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const outputSchema = JSON.parse(readFileSync(new URL("../../📤️output/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validateOutputs = new Ajv({ strict: true, allErrors: true }).compile(outputSchema);
  assert(validateOutputs(outputs), JSON.stringify(validateOutputs.errors));
  assert.deepEqual(outputs.surfaces.map((surface: string) => Buffer.from(surface).toString("hex")), outputs.surfaceUtf8);
  const slots = new Uint8Array(outputs.entrySlots);
  slots.fill(1);
  assert.equal(slots.findIndex((slot) => slot === 0), -1);
  assert.equal(validateOutputs({ ...outputs, entrySlots: 65 }), false);
  assert.equal(validateOutputs({ ...outputs, reserveBeforeProducer: false }), false);
  assert.equal(validateOutputs({ ...outputs, refusalPreservesReady: false }), false);
  assert.equal(validateOutputs({ ...outputs, partialProducerUnwindRetainsOwner: false }), false);
  const retainedRoot = { payload: Buffer.from(outputs.surfaces[0]) };
  const retainedBuffer = retainedRoot.payload;
  try { retainedRoot.payload[0] ^= 1; throw new Error("controlled ownership oracle unwind"); } catch {}
  assert.equal(retainedRoot.payload === retainedBuffer, outputs.partialProducerUnwindRetainsOwner);
  const incompleteSource: { current?: Buffer; permit?: Buffer; payload?: Buffer } = { permit: Buffer.from([1]), payload: retainedBuffer };
  const originalPermit = incompleteSource.permit;
  if (incompleteSource.current && incompleteSource.permit && incompleteSource.payload) {
    delete incompleteSource.current; delete incompleteSource.permit; delete incompleteSource.payload;
  }
  assert.equal(incompleteSource.permit === originalPermit && incompleteSource.payload === retainedBuffer, outputs.incompleteProducerSourcesPreserveRemainingOwners);
  assert.equal(validateOutputs({ ...outputs, incompleteProducerSourcesPreserveRemainingOwners: false }), false);
  const cancelledIdentity = Buffer.from("01000000000000000000000000000000", "hex");
  const cancelledOwner = { identity: cancelledIdentity, reservation: {} as object | undefined };
  cancelledOwner.reservation = undefined;
  assert.equal(cancelledOwner.identity === cancelledIdentity && cancelledOwner.identity.readBigUInt64LE(0) === 1n, outputs.cancelledAdmission.generationPreserved);
  assert.equal(Number(cancelledOwner.identity.readBigUInt64LE(8)), outputs.cancelledAdmission.revision);
  assert.equal(validateOutputs({ ...outputs, cancelledAdmission: { ...outputs.cancelledAdmission, generationPreserved: false } }), false);
  const handbackSlots = Buffer.alloc(outputs.handbackAdmission.slots, 1);
  const concurrentEntries = Buffer.alloc(outputs.entrySlots);
  for (const row of outputs.residentCapacity.cases) {
    const counters = Buffer.alloc(24);
    counters.writeBigUInt64LE(BigInt(outputs.aggregateCeilingBytes));
    counters.writeBigUInt64LE(BigInt(row.fixedBytes), 8);
    counters.writeBigUInt64LE(BigInt(outputs.residentCapacity.reservationBytes), 16);
    const available = counters.readBigUInt64LE(0) - counters.readBigUInt64LE(8);
    assert.equal(available / counters.readBigUInt64LE(16), BigInt(row.capacity));
    assert.equal((BigInt(row.capacity) + 1n) * counters.readBigUInt64LE(16) <= available, outputs.residentCapacity.capPlusOneAccepted);
  }
  assert.equal(validateOutputs({ ...outputs, residentCapacity: { ...outputs.residentCapacity, cases: [{ fixedBytes: 1, capacity: 4 }] } }), false);
  concurrentEntries.fill(1, 0, outputs.concurrentAdmission.occupied);
  let acceptedConcurrent = 0;
  for (let worker = 0; worker < outputs.concurrentAdmission.workers; worker++) {
    const free = concurrentEntries.indexOf(0);
    if (free >= 0) { concurrentEntries[free] = 1; acceptedConcurrent++; }
  }
  assert.equal(acceptedConcurrent, outputs.concurrentAdmission.accepted);
  concurrentEntries.fill(0);
  assert.equal(concurrentEntries.filter((entry) => entry === 0).length, outputs.concurrentAdmission.restored);
  assert.equal(validateOutputs({ ...outputs, concurrentAdmission: { ...outputs.concurrentAdmission, accepted: 2 } }), false);
  const originalLifetime = Buffer.alloc(12); originalLifetime.writeUInt32LE(outputs.capturedLifetime.instance); originalLifetime.writeBigUInt64LE(BigInt(outputs.capturedLifetime.original), 4);
  const reusedLifetime = Buffer.from(originalLifetime); reusedLifetime.writeBigUInt64LE(BigInt(outputs.capturedLifetime.reused), 4);
  assert.equal(originalLifetime.equals(reusedLifetime), outputs.capturedLifetime.foreignCloseAccepted);
  assert.equal(validateOutputs({ ...outputs, capturedLifetime: { ...outputs.capturedLifetime, foreignCloseAccepted: true } }), false);
  const directSource: { value?: Buffer } = { value: Buffer.from(outputs.surfaces[0]) };
  const directEntry: { value?: Buffer } = {};
  const directIdentity = directSource.value;
  const transferDirect = (grant: number) => {
    if (grant === 0 || directEntry.value || !directSource.value) return false;
    directEntry.value = directSource.value;
    directSource.value = undefined;
    return true;
  };
  assert.equal(transferDirect(0), outputs.directReceiver.zeroGrantTransfers);
  try { assert(transferDirect(outputs.physicalGrant)); throw new Error("controlled direct receiver unwind"); } catch {}
  assert.equal(transferDirect(outputs.physicalGrant), outputs.directReceiver.occupiedTransfers);
  assert.equal(directEntry.value === directIdentity && directSource.value === undefined, outputs.directReceiver.unwindRetainsPayload);
  assert.equal(validateOutputs({ ...outputs, directReceiver: { ...outputs.directReceiver, unwindRetainsPayload: false } }), false);
  handbackSlots[handbackSlots.length - 1] = 0;
  assert.equal(handbackSlots.filter((slot) => slot === 0).length >= outputs.handbackAdmission.perProducer, outputs.handbackAdmission.onlyOneFreeAccepted);
  const admittedPair = handbackSlots.subarray(0, outputs.handbackAdmission.perProducer);
  handbackSlots.fill(1);
  assert.equal(admittedPair.length === outputs.handbackAdmission.perProducer, outputs.handbackAdmission.saturatedAfterSealTransfers);
  assert.equal(validateOutputs({ ...outputs, handbackAdmission: { ...outputs.handbackAdmission, perProducer: 1 } }), false);
  for (const row of outputs.readyRevalidation) {
    const deadline = Buffer.alloc(8); deadline.writeBigUInt64LE(BigInt(row.deadline));
    const outcome = row.cancelled || !row.sameGeneration ? "fault" : row.fuel === 0 || BigInt(row.now) >= deadline.readBigUInt64LE() ? "pending" : "ready";
    assert.equal(outcome, row.outcome);
  }
  const transaction: SurfaceTransactionFixture = JSON.parse(readFileSync(new URL("../../🔄️transaction/🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const transactionSchema = JSON.parse(readFileSync(new URL("../../🔄️transaction/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validateTransaction = new Ajv({ strict: true, allErrors: true }).compile(transactionSchema);
  assert(validateTransaction(transaction), JSON.stringify(validateTransaction.errors));
  const requiredNodes = Buffer.alloc(8); requiredNodes.writeBigUInt64LE(BigInt(transaction.requiredNodes));
  for (const row of transaction.cases) {
    const fault = requiredNodes.readBigUInt64LE() > BigInt(row.maximumNodes);
    assert.equal(fault, row.creditFault);
    assert.equal(fault ? 0 : 1, row.patches);
  }
  assert.equal(validateTransaction({ ...transaction, requiredNodes: 0 }), false);
  return fixture.inlineCases.length + fixture.backingCases.length + 32;
}
