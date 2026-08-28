#!/usr/bin/env bun
//#region 🧪️ResidentOracle
import { strict as assert } from "node:assert";
import Ajv from "ajv";
import { produce } from "immer";
import ts from "typescript";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "@semio-tech/repo-lib";
import { OwnedResidentLedger } from "./🟦️component.ts";
import * as resident from "./🟦️component.ts";
import fixture from "./🧪️fixture.json";
import fixtureSchema from "./🧪️schema.json";
import capacitySchema from "./🧬️schema.json";
import admissionContract from "./📨️admission/🧬️contract.json";
import admissionContractSchema from "./📨️admission/🧬️schema.json";
import admissionFixture from "./📨️admission/🧪️fixture.json";
import admissionFixtureSchema from "./📨️admission/🧪️schema.json";

class TestScript extends BundleScript {
  async run(): Promise<void> {
    const ajv = new Ajv({ strict: true, allErrors: true }); ajv.addSchema(capacitySchema);
    assert(ajv.compile(fixtureSchema)(fixture), JSON.stringify(ajv.errors));
    assert(ajv.compile(admissionContractSchema)(admissionContract), JSON.stringify(ajv.errors)); assert(ajv.compile(admissionFixtureSchema)(admissionFixture), JSON.stringify(ajv.errors));
    const bootstrap = new OwnedResidentLedger(fixture.capacity); const bootstrapConsumer = Object.freeze({ name: "original" }); const foreignConsumer = Object.freeze({ name: "foreign" }); const bootstrapGrant = admissionFixture.grants[2]!;
    assert.equal(typeof Reflect.get(bootstrap, "prepareAdmission"), "function", "Original admission cells must survive a wrapper throwing after preparation");
    for (const short of admissionFixture.grants.slice(0, 2)) { assert.equal(bootstrap.prepareAdmission(bootstrapConsumer, "data", short).kind, "blocked"); assert.deepEqual(bootstrap.usage.data, { bytes: 0, slots: 0, owners: 0 }); }
    const prepare = resident.OwnedResidentLedger.prototype.prepareAdmission; const originalFault = Object.freeze({ retained: Buffer.alloc(1024, admissionFixture.unrelatedByte) }); let caught: unknown;
    resident.OwnedResidentLedger.prototype.prepareAdmission = function (consumer, partition, grant) { const result = prepare.call(this, consumer, partition, grant); assert.equal(result.kind, "pending"); throw originalFault; };
    try { bootstrap.prepareAdmission(bootstrapConsumer, "data", bootstrapGrant); assert.fail("wrapper must throw"); } catch (error) { caught = error; } finally { resident.OwnedResidentLedger.prototype.prepareAdmission = prepare; }
    assert.equal(caught, originalFault); const heldAdmission = bootstrap.preparedAdmission(bootstrapConsumer); assert(heldAdmission); assert.equal(bootstrap.preparedAdmission(bootstrapConsumer), heldAdmission); assert.equal(bootstrap.preparedAdmission(foreignConsumer), null); assert.equal(bootstrap.prepareAdmission(foreignConsumer, "data", bootstrapGrant).kind, "blocked");
    const cellUsage = produce({ bytes: 0, slots: 0, owners: 0 }, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] += admissionContract.charge[axis]; }); assert.deepEqual(bootstrap.usage.data, cellUsage);
    assert.equal(heldAdmission.retainFailure(caught, { maxItems: 0, maxBytes: 4096 }).kind, "blocked"); assert.equal(heldAdmission.hasFailure, false); assert.equal(heldAdmission.retainFailure(caught, bootstrapGrant).kind, "pending"); assert.equal(heldAdmission.failure, originalFault); assert.equal(heldAdmission.retainFailure(originalFault, bootstrapGrant).kind, "ready"); assert.equal(heldAdmission.retainFailure(foreignConsumer, bootstrapGrant).kind, "rejected");
    assert.equal(bootstrap.claimAdmission(bootstrapConsumer, heldAdmission, bootstrapGrant).kind, "rejected"); heldAdmission.beginClose(); assert.equal(heldAdmission.closeStep(bootstrapGrant).kind, "pending"); assert.equal(bootstrap.preparedAdmission(bootstrapConsumer), null); assert.equal(heldAdmission.closeStep(bootstrapGrant).kind, "rejected"); assert.deepEqual(bootstrap.usage.data, cellUsage); assert.equal(heldAdmission.retirement, null); assert.equal(heldAdmission.failure, originalFault);
    const nextConsumer = Object.freeze({ name: "next" }); assert.equal(bootstrap.prepareAdmission(nextConsumer, "data", bootstrapGrant).kind, "pending"); const nextCell = bootstrap.preparedAdmission(nextConsumer); assert(nextCell); const claim = resident.OwnedResidentLedger.prototype.claimAdmission;
    resident.OwnedResidentLedger.prototype.claimAdmission = function (consumer, cell, grant) { const result = claim.call(this, consumer, cell, grant); assert.equal(result.kind, "ready"); throw originalFault; };
    try { bootstrap.claimAdmission(nextConsumer, nextCell, bootstrapGrant); assert.fail("claim wrapper must throw"); } catch (error) { assert.equal(error, originalFault); } finally { resident.OwnedResidentLedger.prototype.claimAdmission = claim; }
    assert(nextCell.claimed); assert.equal(bootstrap.preparedAdmission(nextConsumer), null); assert.equal(bootstrap.claimAdmission(nextConsumer, nextCell, bootstrapGrant).kind, "rejected"); assert.equal(nextCell.retainFailure(originalFault, bootstrapGrant).kind, "pending"); nextCell.beginClose(); assert.equal(nextCell.closeStep(bootstrapGrant).kind, "rejected");
    for (const kind of admissionFixture.faultKinds) {
      const pool = new OwnedResidentLedger(fixture.capacity); const consumer = Object.freeze({}); const fault = kind === "null" ? null : kind === "undefined" ? undefined : kind === "proxy" ? new Proxy({}, { get() { throw new Error("fault root must remain opaque"); } }) : Object.freeze({ bytes: Buffer.alloc(256, 73) });
      const freeze = Object.freeze; let original: unknown; Object.freeze = value => { if (value instanceof resident.OwnedResidentAdmission) { original = value; throw fault; } return freeze(value); };
      try { assert.equal(pool.prepareAdmission(consumer, "data", bootstrapGrant).kind, "rejected"); } finally { Object.freeze = freeze; }
      const cell = pool.preparedAdmission(consumer); assert(cell); assert.equal(cell, original); assert(cell.hasFailure); assert.equal(cell.failure, fault); assert.equal(pool.claimAdmission(consumer, cell, bootstrapGrant).kind, "rejected"); cell.beginClose(); cell.closeStep(bootstrapGrant); assert.equal(cell.closeStep(bootstrapGrant).kind, "rejected"); assert.deepEqual(pool.usage.data, cellUsage); assert.equal(cell.retirement, null);
    }
    const unusedCellPool = new OwnedResidentLedger(fixture.capacity); const unusedConsumer = Object.freeze({}); unusedCellPool.prepareAdmission(unusedConsumer, "data", bootstrapGrant); const unusedCell = unusedCellPool.preparedAdmission(unusedConsumer); assert(unusedCell); unusedCellPool.claimAdmission(unusedConsumer, unusedCell, bootstrapGrant); unusedCell.beginClose(); assert.equal(unusedCell.closeStep({ maxItems: 1, maxBytes: 295 }).kind, "blocked"); assert.equal(unusedCell.closeStep(bootstrapGrant).kind, "complete"); assert(resident.OwnedResidentRetirement.matches(unusedCell.retirement, unusedCell)); assert.deepEqual(unusedCellPool.usage.data, { bytes: 0, slots: 0, owners: 0 });
    for (const vector of fixture.invalidCapacities) assert.throws(() => new OwnedResidentLedger(vector.value), vector.name);
    const ledger = new OwnedResidentLedger(fixture.capacity); assert.deepEqual(ledger.capacity, fixture.capacity);
    const retainedGrant = { maxItems: 1, maxBytes: 4096 }; const unrelated = ledger.reserveRecord("data", fixture.domainRecord.envelope, retainedGrant).record; assert(unrelated); const unrelatedShell = Object.freeze({}); unrelated.install(unrelatedShell, retainedGrant); const unrelatedUsage = ledger.usage;
    const originalFreeze = Object.freeze; let failedRecord: ReturnType<typeof ledger.reserveRecord> | undefined;
    Object.freeze = value => { if (value instanceof resident.OwnedResidentRecord) throw new Error("record-return-finalization"); return originalFreeze(value); };
    try { assert.doesNotThrow(() => { failedRecord = ledger.reserveRecord("data", fixture.domainRecord.envelope, retainedGrant); }, "Admission must return the exact retained closing record instead of stranding it behind a throw"); } finally { Object.freeze = originalFreeze; }
    assert(failedRecord); assert.equal(failedRecord.step.kind, "rejected"); assert(failedRecord.record); assert.equal(failedRecord.record.install({}, retainedGrant).kind, "rejected"); assert.equal(failedRecord.record.closeStep(retainedGrant).kind, "complete"); assert.deepEqual(ledger.usage, unrelatedUsage); assert(unrelated.matchesShell(unrelatedShell)); unrelated.beginClose(); unrelated.detach(unrelatedShell, retainedGrant); unrelated.closeStep(retainedGrant);
    assert.equal(typeof Reflect.get(ledger, "reserveRecord"), "function", "Exact fixed domain record admission must exist");
    const recordGrant = { maxItems: 1, maxBytes: 4096 }; const recordResult = ledger.reserveRecord("data", fixture.domainRecord.envelope, recordGrant); assert(recordResult.record); const record = recordResult.record;
    const shell = Object.create(null); const foreign = Object.create(null); let propertiesRead = 0; Object.defineProperty(shell, "terminal", { get() { propertiesRead++; throw new Error("No structural terminal authority"); } });
    const recordUsage = produce({ bytes: 0, slots: 0, owners: 0 }, draft => { for (const axis of ["bytes", "slots", "owners"] as const) draft[axis] = fixture.domainRecord.envelope[axis] + fixture.domainRecord.intrinsic[axis]; }); assert.deepEqual(ledger.usage.data, recordUsage);
    assert.throws(() => { assert.equal(record.install(shell, recordGrant).kind, "ready"); throw new Error("after-install"); }, /after-install/); assert(record.matchesShell(shell)); assert.equal(record.install(foreign, recordGrant).kind, "rejected");
    record.beginClose(); assert.equal(record.closeStep(recordGrant).kind, "blocked"); assert.equal(record.detach(foreign, recordGrant).kind, "rejected"); assert.deepEqual(ledger.usage.data, recordUsage);
    assert.throws(() => { assert.equal(record.detach(shell, recordGrant).kind, "pending"); throw new Error("after-detach"); }, /after-detach/); const observation = record.detachment; assert(resident.OwnedResidentRecordDetachment.matches(observation, record, shell)); assert(!resident.OwnedResidentRecordDetachment.matches(observation, record, foreign)); assert.equal(record.detach(shell, recordGrant).kind, "rejected");
    assert.equal(record.closeStep({ maxItems: 0, maxBytes: 4096 }).kind, "blocked"); assert.equal(record.closeStep({ maxItems: 1, maxBytes: 255 }).kind, "blocked"); assert.deepEqual(ledger.usage.data, recordUsage); assert.equal(record.closeStep(recordGrant).kind, "complete"); assert(resident.OwnedResidentRetirement.matches(record.retirement, record)); assert(resident.OwnedResidentRecordDetachment.matches(record.detachment, record, shell)); assert.equal(propertiesRead, 0); assert.deepEqual(ledger.usage.data, { bytes: 0, slots: 0, owners: 0 });
    const unused = ledger.reserveRecord("data", fixture.domainRecord.envelope, recordGrant).record; assert(unused); unused.beginClose(); assert.equal(unused.closeStep(recordGrant).kind, "complete"); assert.equal(unused.detachment, null);
    for (const axis of ["bytes", "slots", "owners"] as const) { const envelope = { bytes: 0, slots: 0, owners: 0, [axis]: Number.MAX_SAFE_INTEGER }; assert.equal(ledger.reserveRecord("data", envelope, recordGrant).record, null); assert.deepEqual(ledger.usage.data, { bytes: 0, slots: 0, owners: 0 }); }
    for (const kind of ["record", "witness", "observation"] as const) {
      const pool = new OwnedResidentLedger(fixture.capacity); const freeze = Object.freeze; let captured: object | null = null;
      Object.freeze = value => { if (kind === "record" ? value instanceof resident.OwnedResidentRecord : kind === "witness" ? value instanceof resident.OwnedResidentRetirement : value instanceof resident.OwnedResidentRecordDetachment) { captured = value; throw new Error("record-finalization"); } return freeze(value); };
      try { const rejected = pool.reserveRecord("data", fixture.domainRecord.envelope, recordGrant); assert.equal(rejected.step.kind, "rejected"); assert(rejected.record); assert.equal(rejected.record.install({}, recordGrant).kind, "rejected"); } finally { Object.freeze = freeze; }
      assert(captured); pool.beginClose(); for (let turn = 0; turn < 8 && !pool.terminalIsEmpty(); turn++) pool.closeStep(recordGrant); assert(pool.terminalIsEmpty(), kind);
    }
    const installedPool = new OwnedResidentLedger(fixture.capacity); const installed = installedPool.reserveRecord("data", fixture.domainRecord.envelope, recordGrant).record; assert(installed); const installedShell = Object.freeze({}); installed.install(installedShell, recordGrant); installedPool.beginClose(); assert.equal(installedPool.closeStep(recordGrant).kind, "blocked"); assert(!installedPool.terminalIsEmpty()); installed.detach(installedShell, recordGrant); for (let turn = 0; turn < 4 && !installedPool.terminalIsEmpty(); turn++) installedPool.closeStep(recordGrant); assert(installedPool.terminalIsEmpty());
    const { capacity, used, request, accepted } = fixture.overflow; assert.equal(BigInt(request) <= BigInt(capacity) - BigInt(used), accepted);
    assert.equal(typeof Reflect.get(resident, "OwnedResidentRetirement"), "function", "Exact preadmitted retirement witness must exist");
    assert.equal(typeof Reflect.get(ledger, "beginOwner"), "function", "Strong retained owner admission must exist");
    const grant = { maxItems: 1, maxBytes: 4096 };
    const owner = ledger.beginOwner("data", grant).owner; assert(owner);
    const readerOwner = ledger.beginOwner("data", grant).owner; assert(readerOwner);
    const page = owner.reservePage(256, grant).page; assert(page);
    let expected = { bytes: 0, slots: 0, owners: 0 };
    for (const charge of [fixture.recordCharges.owner, fixture.recordCharges.witness, fixture.recordCharges.owner, fixture.recordCharges.witness, fixture.recordCharges.storage, fixture.recordCharges.witness, fixture.recordCharges.pageBacking]) expected = produce(expected, draft => { draft.bytes += charge.bytes; draft.slots += charge.slots; draft.owners += charge.owners; });
    assert.deepEqual(ledger.usage.data, expected);
    assert.equal(page.allocate({ maxItems: 0, maxBytes: 4096 }).kind, "blocked"); assert.equal(page.allocate(grant).kind, "ready");
    const bytes = Buffer.alloc(fixture.pattern.length); for (let i = 0; i < bytes.length; i++) { bytes[i] = (fixture.pattern.multiplier * i + fixture.pattern.addend) & 255; assert.equal(page.writeByte(bytes[i]!, grant).bytes, 1); }
    assert.equal(page.seal(grant).kind, "ready"); const reader = readerOwner.beginRead(page, grant).reader; assert(reader);
    owner.beginClose(); assert.equal(owner.closeStep(grant).kind, "blocked");
    for (let i = 0; i < bytes.length; i++) assert.equal(reader.byteAt(i), bytes[i]);
    readerOwner.beginClose();
    for (let turn = 0; turn < 20 && !readerOwner.terminalIsEmpty(); turn++) { const current = readerOwner.closeStep(grant); assert(current.items <= 1 && current.bytes <= grant.maxBytes); }
    assert(readerOwner.terminalIsEmpty()); assert.throws(() => reader.byteAt(0));
    for (let turn = 0; turn < 20 && !owner.terminalIsEmpty(); turn++) { const current = owner.closeStep(grant); assert(current.items <= 1 && current.bytes <= grant.maxBytes); }
    assert(owner.terminalIsEmpty()); assert.deepEqual(ledger.usage.data, { bytes: 0, slots: 0, owners: 0 });
    assert(resident.OwnedResidentRetirement.matches(owner.retirement, owner)); assert(!resident.OwnedResidentRetirement.matches(owner.retirement, readerOwner)); assert(!resident.OwnedResidentRetirement.matches({ root: owner }, owner));
    assert(resident.OwnedResidentRetirement.matches(page.retirement, page)); assert(resident.OwnedResidentRetirement.matches(reader.retirement, reader));
    for (const kind of ["owner", "page", "reader", "witness"] as const) {
      const lost = new OwnedResidentLedger(fixture.capacity); const parent = lost.beginOwner("data", grant).owner; assert(parent); const source = parent.reservePage(256, grant).page; assert(source); source.allocate(grant); for (let i = 0; i < 256; i++) source.writeByte(i, grant); source.seal(grant);
      const freeze = Object.freeze; let captured: object | null = null;
      Object.freeze = value => { const match = kind === "owner" ? value instanceof resident.OwnedResidentOwner : kind === "page" ? value instanceof resident.OwnedResidentPage : kind === "reader" ? value instanceof resident.OwnedResidentReader : value instanceof resident.OwnedResidentRetirement; if (match) { captured = value; throw new Error("retained-finalization"); } return freeze(value); };
      try { const rejected = kind === "owner" || kind === "witness" ? lost.beginOwner("data", grant) : kind === "page" ? parent.reservePage(256, grant) : parent.beginRead(source, grant); assert.equal(rejected.step.kind, "rejected"); assert("owner" in rejected ? rejected.owner : "page" in rejected ? rejected.page : rejected.reader); } finally { Object.freeze = freeze; }
      assert(captured); lost.beginClose(); for (let turn = 0; turn < 50 && !lost.terminalIsEmpty(); turn++) { const current = lost.closeStep(grant); assert.notEqual(current.kind, "rejected"); assert(current.bytes <= grant.maxBytes); } assert(lost.terminalIsEmpty(), kind);
    }
    for (const length of fixture.pageLengths) {
      const pool = new OwnedResidentLedger(fixture.capacity); const scope = pool.beginOwner("data", grant).owner; assert(scope); const storage = scope.reservePage(length, grant).page; assert(storage, `logical extent ${length}`); storage.allocate(grant);
      const expectedBytes = Buffer.alloc(length, 23); for (const byte of expectedBytes) storage.writeByte(byte, grant); assert.equal(storage.seal(grant).kind, "ready"); const view = scope.beginRead(storage, grant).reader; assert(view); assert.equal(view.length, length);
      for (let i = 0; i < length; i++) assert.equal(view.byteAt(i), expectedBytes[i]); assert.throws(() => view.byteAt(length));
      pool.beginClose(); for (let turn = 0; turn < 20 && !pool.terminalIsEmpty(); turn++) pool.closeStep(grant); assert(pool.terminalIsEmpty());
    }
    const shared = new OwnedResidentLedger(fixture.capacity); const rawOwner = shared.beginOwner("data", grant).owner; const uiOwner = shared.beginOwner("data", grant).owner; assert(rawOwner && uiOwner);
    assert.equal(typeof Reflect.get(rawOwner, "reserveExternalBacking"), "function", "Pre-Open raw backing reservation must exist");
    const [rawCase, uiCase, scratchCase] = fixture.storageCases; assert(rawCase && uiCase && scratchCase);
    const raw = rawOwner.reserveExternalBacking(rawCase.bytes, grant); assert(raw.slot); assert.equal(raw.step.kind, "ready");
    const destination = uiOwner.reservePage(uiCase.bytes, grant).page; const scratch = uiOwner.reservePage(scratchCase.bytes, grant).page; assert(destination && scratch);
    const combined = produce({ bytes: 0, slots: 0, owners: 0 }, draft => { draft.bytes = 2 * 192 + rawCase.bytes + 320 + 2 * 512; draft.slots = 2 * 2 + 4 + 2 * 3; draft.owners = 2 * 2 + 3 + 2 * 2; }); assert.deepEqual(shared.usage.data, combined);
    const pending = raw.slot; assert.equal(pending.beginReceive(grant).kind, "pending"); rawOwner.beginClose(); const before = shared.usage; assert.equal(rawOwner.closeStep(grant).kind, "blocked"); assert.deepEqual(shared.usage, before);
    const original = new ArrayBuffer(4161); const alias = new Uint8Array(original); for (let i = 0; i < alias.length; i++) alias[i] = i & 255;
    const adopted = pending.adoptTransferred(original, grant); assert.equal(adopted.step.kind, "pending"); assert.equal(original.byteLength, 0); assert.equal(alias.byteLength, 0); assert.equal(adopted.receipt, null, "Late custody during close is retirement-only");
    shared.beginClose(); for (let turn = 0; turn < 40 && !shared.terminalIsEmpty(); turn++) shared.closeStep(grant); assert(shared.terminalIsEmpty());
    const huge = new OwnedResidentLedger({ bytes: Number.MAX_SAFE_INTEGER, slots: 20, owners: 20, control: { bytes: 0, slots: 0, owners: 0 } }); const hugeOwner = huge.beginOwner("data", grant).owner; assert(hugeOwner);
    assert.equal(hugeOwner.reserveExternalBacking(Number.MAX_SAFE_INTEGER, grant).slot, null); assert.equal(hugeOwner.reserveExternalBacking(Number.MAX_SAFE_INTEGER - 320, grant).slot, null);
    const unsubmitted = hugeOwner.reserveExternalBacking(4161, grant); assert(unsubmitted.slot); huge.beginClose(); for (let turn = 0; turn < 20 && !huge.terminalIsEmpty(); turn++) huge.closeStep(grant); assert(huge.terminalIsEmpty());
    const transferFault = new OwnedResidentLedger(fixture.capacity); const faultOwner = transferFault.beginOwner("data", grant).owner; assert(faultOwner); const faultSlot = faultOwner.reserveExternalBacking(32, grant).slot; assert(faultSlot); faultSlot.beginReceive(grant); const faultBacking = new ArrayBuffer(32);
    const NativeBytes = globalThis.Uint8Array; globalThis.Uint8Array = new Proxy(NativeBytes, { construct(target, args, newTarget) { if (args[0] instanceof ArrayBuffer) throw new Error("after-transfer-view"); return Reflect.construct(target, args, newTarget); } });
    try { assert.equal(faultSlot.adoptTransferred(faultBacking, grant).step.kind, "rejected"); } finally { globalThis.Uint8Array = NativeBytes; }
    assert.equal(faultBacking.byteLength, 0); transferFault.beginClose(); for (let turn = 0; turn < 30 && !transferFault.terminalIsEmpty(); turn++) transferFault.closeStep(grant); assert(transferFault.terminalIsEmpty(), "Actual transferred backing survives view-constructor failure and retires");
    for (const vector of fixture.controlCases) {
      const pool = new OwnedResidentLedger({ ...fixture.capacity, control: vector.control }); const data = pool.beginOwner("data", grant).owner; assert(data); const maximum = fixture.capacity.bytes - vector.control.bytes - 192 - 320; assert(data.reserveExternalBacking(maximum, grant).slot);
      const before = pool.usage; assert.equal(data.reservePage(1, grant).page, null); assert.equal(pool.beginOwner("data", grant).owner, null); assert.deepEqual(pool.usage, before);
      const control = pool.beginOwner("control", grant).owner; assert(control); assert(control.reservePage(1, grant).page); assert.equal(pool.beginOwner("control", grant).owner, null, vector.name);
      pool.beginClose(); for (let turn = 0; turn < 30 && !pool.terminalIsEmpty(); turn++) pool.closeStep(grant); assert(pool.terminalIsEmpty());
    }
    for (const kind of ["custody", "external", "retirement"] as const) {
      const pool = new OwnedResidentLedger(fixture.capacity); const scope = pool.beginOwner("data", grant).owner; assert(scope); const freeze = Object.freeze; let captured: object | null = null;
      Object.freeze = value => { if (kind === "custody" ? value instanceof resident.OwnedResidentBackingCustody : kind === "external" ? value instanceof resident.OwnedResidentExternalBacking : value instanceof resident.OwnedResidentRetirement) { captured = value; throw new Error("external-finalization"); } return freeze(value); };
      try { const rejected = scope.reserveExternalBacking(32, grant); assert.equal(rejected.step.kind, "rejected"); assert(rejected.slot); assert.equal(rejected.slot.beginReceive(grant).kind, "rejected"); } finally { Object.freeze = freeze; }
      assert(captured); pool.beginClose(); for (let turn = 0; turn < 30 && !pool.terminalIsEmpty(); turn++) pool.closeStep(grant); assert(pool.terminalIsEmpty(), kind);
    }
    for (const failure of fixture.closeChildren) {
      const pool = new OwnedResidentLedger(fixture.capacity); const scope = pool.beginOwner("data", grant).owner; assert(scope); const storage = scope.reservePage(0, grant).page; assert(storage); scope.beginClose(); const actual = resident.OwnedResidentPage.prototype.closeStep;
      resident.OwnedResidentPage.prototype.closeStep = function (budget): resident.ResidentStep { if (failure === "throw") throw new Error("child-close"); if (failure === "full-grant-complete") { const current = actual.call(this, budget); assert.equal(current.kind, "complete"); return { ...current, bytes: budget.maxBytes }; } return { kind: failure === "blocked" ? "blocked" : "rejected", phase: failure, items: 1, bytes: failure === "over-grant" ? budget.maxBytes + 1 : 7 }; };
      let current: resident.ResidentStep; try { current = scope.closeStep(grant); } finally { resident.OwnedResidentPage.prototype.closeStep = actual; }
      assert.equal(current.kind, failure === "blocked" ? "blocked" : failure === "full-grant-complete" ? "pending" : "rejected"); assert.equal(current.bytes, failure === "throw" ? 0 : failure === "over-grant" ? 4097 : failure === "full-grant-complete" ? 4096 : 7);
      assert(!scope.terminalIsEmpty()); pool.beginClose(); for (let turn = 0; turn < 20 && !pool.terminalIsEmpty(); turn++) pool.closeStep(grant); assert(pool.terminalIsEmpty());
    }
    for (const kind of fixture.admissionFailures) {
      const shared = new OwnedResidentLedger(fixture.capacity); const consumer = shared.beginOwner("data", grant).owner; assert(consumer); const livePage = consumer.reservePage(1, grant).page; assert(livePage); livePage.allocate(grant); livePage.writeByte(73, grant); livePage.seal(grant); const baseline = shared.usage;
      const freeze = Object.freeze; let original: object | null = null;
      Object.freeze = value => { const match = kind === "owner" ? value instanceof resident.OwnedResidentOwner : kind === "record" ? value instanceof resident.OwnedResidentRecord : kind === "page" ? value instanceof resident.OwnedResidentPage : kind === "reader" ? value instanceof resident.OwnedResidentReader : value instanceof resident.OwnedResidentExternalBacking; if (match) { original = value; throw new Error("original-admission-finalization"); } return freeze(value); };
      let failed: resident.OwnedResidentOwner | resident.OwnedResidentRecord | resident.OwnedResidentPage | resident.OwnedResidentReader | resident.OwnedResidentExternalBacking | null = null;
      try {
        const rejected = kind === "owner" ? shared.beginOwner("data", grant) : kind === "record" ? shared.reserveRecord("data", fixture.domainRecord.envelope, grant) : kind === "page" ? consumer.reservePage(1, grant) : kind === "reader" ? consumer.beginRead(livePage, grant) : consumer.reserveExternalBacking(32, grant);
        assert.equal(rejected.step.kind, "rejected"); failed = "owner" in rejected ? rejected.owner : "record" in rejected ? rejected.record : "page" in rejected ? rejected.page : "reader" in rejected ? rejected.reader : rejected.slot;
      } finally { Object.freeze = freeze; }
      assert(failed); assert.equal(failed, original); const retained = shared.usage; assert.equal(failed.closeStep({ maxItems: 0, maxBytes: 4096 }).kind, "blocked"); assert.deepEqual(shared.usage, retained);
      for (let turn = 0; turn < 4 && !failed.terminalIsEmpty(); turn++) assert.notEqual(failed.closeStep(grant).kind, "rejected"); assert(failed.terminalIsEmpty(), kind); assert.deepEqual(shared.usage, baseline);
      const reader = consumer.beginRead(livePage, grant).reader; assert(reader); assert.equal(reader.byteAt(0), 73); reader.beginClose(); reader.closeStep(grant); assert.deepEqual(shared.usage, baseline); assert(!shared.terminalIsEmpty()); shared.beginClose(); for (let turn = 0; turn < 10 && !shared.terminalIsEmpty(); turn++) shared.closeStep(grant); assert(shared.terminalIsEmpty());
    }
    const custodyPool = new OwnedResidentLedger(fixture.capacity); const custodyOwner = custodyPool.beginOwner("data", grant).owner; const readOwner = custodyPool.beginOwner("data", grant).owner; assert(custodyOwner && readOwner); const custodySlot = custodyOwner.reserveExternalBacking(4, grant).slot; assert(custodySlot); custodySlot.beginReceive(grant);
    const contents = new ArrayBuffer(4); new Uint8Array(contents).set([1, 2, 3, 4]); const receipt = custodySlot.adoptTransferred(contents, grant).receipt; assert(resident.OwnedResidentBackingCustody.matches(receipt, custodySlot)); const retainedRead = readOwner.beginRead(custodySlot, grant).reader; assert(retainedRead);
    custodyOwner.beginClose(); assert(!resident.OwnedResidentBackingCustody.matches(receipt, custodySlot), "Closing invalidates current custody before scrub"); assert.equal(readOwner.beginRead(custodySlot, grant).reader, null); assert.equal(retainedRead.byteAt(3), 4); assert.equal(custodyOwner.closeStep(grant).kind, "blocked");
    readOwner.beginClose(); for (let turn = 0; turn < 10 && !readOwner.terminalIsEmpty(); turn++) readOwner.closeStep(grant);
    for (let turn = 0; turn < 12 && !custodyOwner.terminalIsEmpty(); turn++) { custodyOwner.closeStep(grant); assert(!resident.OwnedResidentBackingCustody.matches(receipt, custodySlot)); } assert(custodyOwner.terminalIsEmpty());
    const program = ts.createProgram([`${import.meta.dir}/🟦️component.ts`], { strict: true, noEmit: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, types: [], lib: ["lib.es2022.d.ts", "lib.dom.d.ts"] });
    const diagnostics = ts.getPreEmitDiagnostics(program); assert.equal(diagnostics.length, 0, ts.formatDiagnosticsWithColorAndContext(diagnostics, { getCanonicalFileName: name => name, getCurrentDirectory: () => import.meta.dir, getNewLine: () => "\n" }));
    console.log(`[DEBUG] Resident capacity=${fixture.invalidCapacities.length} actualOverflow=2 ownerReader=1 constructor=7 partialExtent=${fixture.pageLengths.length} simultaneousRawUiScratch=1 postedCancel=1 unsubmittedCancel=1 transferredViewFault=1 controlAxes=${fixture.controlCases.length} childClose=${fixture.closeChildren.length} domainRecord=15 recordOverflow=3 recordConstructor=3 admissionFailures=${fixture.admissionFailures.length} admissionBootstrap=${admissionFixture.bootstrap.length} firstFault=${admissionFixture.faultKinds.length} strictTS=0 oracle=Ajv+Immer+Buffer+BigInt`);
  }
}
//#endregion 🧪️ResidentOracle
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
