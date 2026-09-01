//#region 💾️ResidentAdmission
import type { ActorInstanceLifetime } from "../../../../🎭️actor/🚪️lifetime/🟦️component.ts";
import { ShardClient, type ShardActorActivationLease } from "../../../../🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts";
import type { NumericIndexGrant } from "../../../../🌱️value/🗂️ordered/🔢️numeric/🟦️component.ts";
import { OwnedResidentLedger, OwnedResidentRetirement, OwnedResidentRecordDetachment, type OwnedResidentAdmission, type OwnedResidentRecord, type ResidentResources } from "../../../../🌱️value/💾️resident/🟦️component.ts";
import type { RetainedUiWireStep } from "../📦️wire/🟦️component.ts";
import { OwnedUiInstance } from "../🏘️instance/🟦️component.ts";
import { OwnedUiOperationPayloadBuilder, type OwnedUiOperationInputCopied, type OwnedUiOperationInputCancelled } from "../🩹️operations/📥️wire/📄️pages/🟦️component.ts";
import { uiResidentMetadataEnvelope } from "./🪪️metadata/🟦️component.ts";
import { OwnedKernelReturnInputField, OwnedKernelReturnPayloadDetachment } from "../../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts";

export type OwnedUiResidentPoolAdmission = { readonly step: RetainedUiWireStep; readonly pool: OwnedUiResidentPool | null };
export type OwnedUiResidentInstanceAdmission = { readonly step: RetainedUiWireStep; readonly scope: OwnedUiResidentInstance | null };
export type OwnedUiResidentPayloadAdmission = { readonly step: RetainedUiWireStep; readonly payload: OwnedUiResidentPayload | null };
type SlotPhase = "empty" | "preparing" | "bootstrap-rejected" | "cell-held" | "claiming" | "claimed" | "record-admitting" | "record-held" | "shell-installed" | "roster-observing" | "closing-domain" | "handoff-observing" | "detaching" | "record-closing" | "record-observing" | "cell-closing" | "cell-observing" | "fault-held";
type Slot = { requestOwner: OwnedUiInstance | null; cell: OwnedResidentAdmission | null; record: OwnedResidentRecord | null; entry: Instance | null; phase: SlotPhase; failure: unknown; witness: InstanceWitness | null };
type Pool = { ledger: OwnedResidentLedger | null; composition: ShardClient | null; failure: unknown; phase: "open" | "closing" | "closed"; bindings: WeakMap<OwnedUiInstance, OwnedUiResidentInstance> | null; head: Instance | null; tail: Instance | null; pending: Slot | null; closing: boolean; closed: boolean; facade: OwnedUiResidentPool | null; witness: OwnedUiResidentPoolRetirement | null };
type Instance = { pool: Pool | null; owner: OwnedUiInstance | null; facade: OwnedUiResidentInstance | null; activation: ShardActorActivationLease | null; lifetime: ActorInstanceLifetime | null; previous: Instance | null; next: Instance | null; head: Payload | null; tail: Payload | null; children: number; closing: boolean; closed: boolean; record: OwnedResidentRecord | null; cell: OwnedResidentAdmission | null; phase: "constructing" | "live" | "domain-closed" | "retired"; witness: InstanceWitness | null; failure: unknown; slot: Slot | null; bindingInstalled: boolean; pending: PayloadSlot | null };
type PayloadPhase = "constructing" | "live" | "body-retired" | "source-detached" | "source-settled" | "domain-retired" | "registration-retired";
type BuilderEntry = { facade: OwnedUiOperationPayloadBuilder | null; record: OwnedResidentRecord | null; cell: OwnedResidentAdmission | null; phase: "constructing" | "live" | "closing" | "retired"; witness: BuilderWitness | null };
type BuilderSlot = { requestOwner: OwnedKernelReturnInputField | null; cell: OwnedResidentAdmission | null; record: OwnedResidentRecord | null; entry: BuilderEntry | null; phase: SlotPhase | "entry-held" | "constructing" | "source-installing" | "source-bound" | "witness-ready" | "finalized" | "binding-detaching" | "binding-settling" | "binding-settled"; failure: unknown; witness: BuilderWitness | null };
type InputEvidence = OwnedUiOperationInputCopied | OwnedUiOperationInputCancelled;
type EvidenceEntry = { facade: InputEvidence | null; record: OwnedResidentRecord | null; cell: OwnedResidentAdmission | null; phase: "constructing" | "live" | "closing" | "retired"; witness: EvidenceWitness | null };
type EvidenceSlot = { requestOwner: OwnedUiOperationPayloadBuilder | null; cell: OwnedResidentAdmission | null; record: OwnedResidentRecord | null; entry: EvidenceEntry | null; phase: SlotPhase | "entry-held" | "constructing" | "witness-ready" | "finalized"; failure: unknown; witness: EvidenceWitness | null };
export type OwnedUiResidentEvidenceAdmission = { readonly step: RetainedUiWireStep; readonly evidence: InputEvidence | null };
type PayloadSlot = { requestOwner: OwnedKernelReturnInputField | null; cell: OwnedResidentAdmission | null; record: OwnedResidentRecord | null; entry: Payload | null; phase: SlotPhase | "source-installing" | "source-bound" | "finalized" | "body-proving" | "source-never-installed" | "source-detaching" | "source-observing" | "source-clearing" | "source-settling" | "source-settle-observing" | "source-settled"; failure: unknown; witness: OwnedKernelReturnPayloadDetachment | OwnedUiResidentPayloadSourceRelease | null };
type PageSlot = { requestOwner: number | null; cell: OwnedResidentAdmission | null; record: OwnedResidentRecord | null; entry: Page | null; phase: SlotPhase | "owner-preparing" | "owner-cell-held" | "owner-claiming" | "owner-claimed" | "owner-admitting" | "page-state" | "page-shell" | "page-storage" | "page-binding" | "page-builder-installing" | "page-finalizing" | "page-unbound" | "finalized"; failure: unknown; witness: PageWitness | null };
export type OwnedUiResidentPageAdmission = { readonly step: RetainedUiWireStep; readonly page: OwnedUiResidentPage | null };
type Payload = { instance: Instance | null; facade: OwnedUiResidentPayload | null; previous: Payload | null; next: Payload | null; head: Page | null; tail: Page | null; cursor: Page | null; builder: BuilderEntry | null; storageCell: OwnedResidentAdmission | null; reader: Reader | null; evidence: EvidenceEntry | null; closing: boolean; closed: boolean; field: OwnedKernelReturnInputField | null; record: OwnedResidentRecord | null; cell: OwnedResidentAdmission | null; phase: PayloadPhase; witness: OwnedUiResidentPayloadSourceRelease | null; parentSlot: PayloadSlot | null; failure: unknown; pending: BuilderSlot | EvidenceSlot | PageSlot | ReaderSlot | null };
type Page = { payload: Payload | null; facade: OwnedUiResidentPage | null; previous: Page | null; next: Page | null; readonly length: number; cell: OwnedResidentAdmission | null; record: OwnedResidentRecord | null; storageCell: OwnedResidentAdmission | null; phase: "constructing" | "sealed" | "storage-empty" | "storage-preparing" | "storage-rejected" | "storage-cell-held" | "storage-claiming" | "storage-claimed" | "storage-admitting" | "live" | "closing" | "domain-retired" | "registration-retired"; witness: PageWitness | null; failure: unknown };
type ReaderSlot = { requestOwner: OwnedUiResidentPayload | null; cell: OwnedResidentAdmission | null; record: OwnedResidentRecord | null; entry: Reader | null; phase: SlotPhase | "reader-state" | "reader-shell" | "reader-witness" | "builder-installing" | "builder-installed" | "reader-finalized" | "binding-detaching" | "binding-settling" | "binding-settled"; failure: unknown; witness: OwnedUiResidentReaderRetirement | null };
type Reader = { payload: Payload | null; facade: OwnedUiResidentPayloadReader | null; cell: OwnedResidentAdmission | null; record: OwnedResidentRecord | null; page: Page | null; storageCell: OwnedResidentAdmission | null; offset: number; phase: "constructing" | "live" | "page-held" | "alias-preparing" | "alias-rejected" | "alias-held" | "alias-claiming" | "alias-claimed" | "alias-admitting" | "reading" | "alias-closing" | "page-detaching" | "page-retiring" | "page-observing" | "closing" | "body-retired" | "domain-retired" | "registration-retired"; witness: OwnedUiResidentReaderRetirement | null; failure: unknown; consumed: bigint };
export type OwnedUiResidentReaderAdmission = { readonly step: RetainedUiWireStep; readonly reader: OwnedUiResidentPayloadReader | null };
export type OwnedUiResidentReaderStep = RetainedUiWireStep | { readonly kind: "byte"; readonly value: number; readonly items: number; readonly bytes: number };
let readerState: (reader: unknown) => Reader | null;
let createReader: (state: Reader) => OwnedUiResidentPayloadReader;
let createReaderWitness: (state: Reader) => OwnedUiResidentReaderRetirement;
let moveReaderWitness: (witness: OwnedUiResidentReaderRetirement, phase: "body-retired" | "detached" | "settled" | "terminal") => void;
const MINT = Object.freeze({});
const NO_POOL_FAULT = Object.freeze({});
const PAGE_BYTES = 256;
const admitted = (grant: NumericIndexGrant, bytes: number): boolean => Number.isSafeInteger(grant.maxItems) && Number.isSafeInteger(grant.maxBytes) && grant.maxItems >= 1 && grant.maxBytes >= bytes;
const step = (kind: RetainedUiWireStep["kind"], phase: string, bytes = 0): RetainedUiWireStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
let instanceOwner: (state: Instance) => OwnedUiResidentInstance;
let payloadOwner: (state: Payload) => OwnedUiResidentPayload;
let pageOwner: (state: Page) => OwnedUiResidentPage;
let pageState: (value: unknown) => Page | null;
let createPageWitness: (page: Page) => PageWitness;
let markPageWitness: (witness: PageWitness, page: Page) => void;
let createBuilderWitness: (entry: BuilderEntry) => BuilderWitness;
let markBuilderWitness: (witness: BuilderWitness, builder: OwnedUiOperationPayloadBuilder) => void;
let moveBuilderWitness: (witness: BuilderWitness, builder: OwnedUiOperationPayloadBuilder, phase: "body-retired" | "source-detached" | "source-settled") => void;
let createEvidenceWitness: (entry: EvidenceEntry) => EvidenceWitness;
let markEvidenceWitness: (witness: EvidenceWitness, token: InputEvidence) => void;
let poolWitness: (state: Pool, pool: OwnedUiResidentPool) => OwnedUiResidentPoolRetirement;
let finishPoolWitness: (witness: OwnedUiResidentPoolRetirement, pool: OwnedUiResidentPool) => void;
let createInstanceWitness: (state: Instance) => InstanceWitness;
let markInstanceDomainClosed: (witness: InstanceWitness, state: Instance) => void;
let instanceDomainClosed: (witness: InstanceWitness, state: Instance) => boolean;
let closeInstance: (state: Instance, grant: NumericIndexGrant) => RetainedUiWireStep;
let scopeAvailable: (scope: OwnedUiResidentInstance) => boolean;
let payloadState: (payload: unknown) => Payload | null;
let payloadWitness: (state: Payload) => OwnedUiResidentPayloadSourceRelease;
let movePayloadWitness: (state: Payload, phase: Exclude<PayloadPhase, "constructing" | "live">) => void;
let payloadWitnessOriginal: (witness: OwnedUiResidentPayloadSourceRelease) => OwnedUiResidentPayload;
let closePayload: (state: Payload, grant: NumericIndexGrant) => RetainedUiWireStep;
function active(instance: Instance): boolean {
  if (instance.phase !== "live" || instance.failure !== NO_POOL_FAULT || instance.cell?.hasFailure || !instance.pool || instance.pool.closing || instance.closing || instance.closed || !instance.owner || !instance.activation || !instance.lifetime || !OwnedUiInstance.matches(instance.owner, instance.activation, instance.lifetime)) return false;
  try { instance.activation.assertActive(); return true; } catch { return false; }
}
function activePayload(payload: Payload): boolean { return payload.phase === "live" && payload.failure === NO_POOL_FAULT && !payload.cell?.hasFailure && !payload.closing && !payload.closed && payload.instance !== null && active(payload.instance); }
function payloadSlotEmpty(slot: PayloadSlot | BuilderSlot | EvidenceSlot | PageSlot | ReaderSlot): boolean { return slot.phase === "empty" && slot.requestOwner === null && !slot.cell && !slot.record && !slot.entry && !slot.witness && slot.failure === NO_POOL_FAULT; }
function evidenceSlot(slot: BuilderSlot | EvidenceSlot | PageSlot | ReaderSlot): slot is EvidenceSlot { return OwnedUiOperationPayloadBuilder.hasBrand(slot.requestOwner); }
function payloadBodyEmpty(state: Payload): boolean { return !state.head && !state.tail && !state.cursor && !state.builder && !state.storageCell && !state.reader && !state.evidence && (!state.pending || payloadSlotEmpty(state.pending)) && state.failure === NO_POOL_FAULT && !state.cell?.hasFailure; }
function childStep(current: RetainedUiWireStep, grant: NumericIndexGrant): RetainedUiWireStep {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes) return { ...current, kind: "rejected" };
  return current.kind === "complete" ? { ...current, kind: "pending" } : current;
}
function admissionStep(current: RetainedUiWireStep, grant: NumericIndexGrant): RetainedUiWireStep { const result = childStep(current, grant); return result.kind === "ready" ? { ...result, kind: "pending" } : result; }
function clearSlot(slot: Slot): void { slot.requestOwner = null; slot.cell = null; slot.record = null; slot.entry = null; slot.witness = null; slot.phase = "empty"; }
function slotFault(slot: Slot, error: unknown): void { if (slot.failure !== NO_POOL_FAULT && !Object.is(slot.failure, error)) throw error; slot.failure = error; }
function observeSlot(pool: Pool, slot: Slot): RetainedUiWireStep | null {
  if (slot.phase === "bootstrap-rejected") { const cell = pool.ledger!.preparedAdmission(slot); if (cell) { slot.cell = cell; slot.phase = "cell-held"; } else clearSlot(slot); return step("pending", "resident-scope-rejection-observation", 64); }
  if (slot.phase === "preparing") { const cell = pool.ledger!.preparedAdmission(slot); if (!cell) return step("blocked", "resident-scope-cell-handoff"); slot.cell = cell; slot.phase = "cell-held"; return step("pending", "resident-scope-cell-observation", 64); }
  if (slot.phase === "claiming") { if (!slot.cell?.claimed) return step("rejected", "resident-scope-claim"); slot.phase = "claimed"; return step("pending", "resident-scope-claim-observation", 64); }
  if (slot.phase === "record-admitting") { slot.record = slot.cell!.result?.record ?? null; slot.phase = slot.record && slot.cell!.result?.step.kind === "ready" && !slot.cell!.hasFailure ? "record-held" : "fault-held"; return step(slot.phase === "record-held" ? "pending" : "rejected", "resident-scope-record-observation", 64); }
  return null;
}
function closeSlot(pool: Pool | null, slot: Slot, grant: NumericIndexGrant): RetainedUiWireStep {
  if (!admitted(grant, 64)) return step("blocked", "resident-scope-slot-close");
  if (slot.failure !== NO_POOL_FAULT) {
    if (!slot.cell) { const cell = pool?.ledger?.preparedAdmission(slot); if (!cell) return step("rejected", "resident-scope-fault-held"); slot.cell = cell; return step("pending", "resident-scope-fault-cell-observation", 64); }
    if (!slot.cell.hasFailure) return childStep(slot.cell.retainFailure(slot.failure, grant), grant); if (!Object.is(slot.cell.failure, slot.failure)) return step("rejected", "resident-scope-distinct-fault");
    if (!slot.record && slot.cell.result?.record) { slot.record = slot.cell.result.record; return step("pending", "resident-scope-fault-record-observation", 64); }
    if (!slot.entry && slot.phase !== "record-closing" && slot.phase !== "record-observing" && slot.phase !== "cell-closing" && slot.phase !== "cell-observing") { if (slot.record) { slot.record.beginClose(); slot.phase = "record-closing"; } else { slot.cell.beginClose(); slot.phase = "cell-closing"; } return step("pending", "resident-scope-fault-intrinsic-begin", 64); }
  } else if (pool) { const observed = observeSlot(pool, slot); if (observed) return observed; }
  const entry = slot.entry;
  if (entry && entry.phase !== "domain-closed" && entry.phase !== "retired") return closeInstance(entry, grant);
  if (slot.phase === "handoff-observing") { if (!entry || !slot.witness || !instanceDomainClosed(slot.witness, entry)) return step("rejected", "resident-scope-domain-proof"); slot.record!.beginClose(); slot.phase = slot.record!.matchesShell(slot.witness.scope) ? "detaching" : "record-closing"; return step("pending", "resident-scope-record-begin", 64); }
  if (slot.phase === "detaching") { const record = slot.record!; if (OwnedResidentRecordDetachment.matches(record.detachment, record, slot.witness!.scope)) { slot.phase = "record-closing"; return step("pending", "resident-scope-detach-observation", 64); } return childStep(record.detach(slot.witness!.scope, grant), grant); }
  if (slot.phase === "record-closing") { const current = slot.record!.closeStep(grant); const result = childStep(current, grant); if (current.kind === "complete" && result.kind === "pending") slot.phase = "record-observing"; return result; }
  if (slot.phase === "record-observing") { if (!OwnedResidentRetirement.matches(slot.record!.retirement, slot.record)) return step("rejected", "resident-scope-record-proof"); slot.cell!.beginClose(); slot.phase = "cell-closing"; return step("pending", "resident-scope-cell-begin", 64); }
  if (slot.phase === "cell-closing") { const current = slot.cell!.closeStep(grant); const result = childStep(current, grant); if (current.kind === "complete" && result.kind === "pending") slot.phase = "cell-observing"; return result; }
  if (slot.phase === "cell-observing") { if (slot.failure !== NO_POOL_FAULT || !slot.cell!.terminalIsEmpty() || slot.record && !slot.record.terminalIsEmpty()) return step("rejected", "resident-scope-cell-proof"); if (entry) { entry.slot = null; entry.witness = null; entry.phase = "retired"; entry.closed = true; } clearSlot(slot); return step("complete", "resident-scope-slot-close", 64); }
  if (entry) return step("rejected", "resident-scope-slot-phase");
  if (slot.record) { slot.record.beginClose(); slot.phase = "record-closing"; return step("pending", "resident-scope-unused-record", 64); }
  if (slot.cell) { slot.cell.beginClose(); slot.phase = "cell-closing"; return step("pending", "resident-scope-unused-cell", 64); }
  if (slot.failure !== NO_POOL_FAULT) return step("rejected", "resident-scope-fault-held"); clearSlot(slot); return step("complete", "resident-scope-slot-close", 64);
}
//#endregion 💾️ResidentAdmission

//#region 🏦️SharedPool
/** 🏦️ The exact Shard composition owns this charged pool before any fallible finalization. */
export class OwnedUiResidentPool {
  readonly #state: Pool;
  private constructor(mint: object, client: ShardClient, ledger: OwnedResidentLedger, grant: NumericIndexGrant) {
    if (mint !== MINT) throw new Error("Invalid resident pool authority");
    const state: Pool = { ledger, composition: client, failure: NO_POOL_FAULT, phase: "open", bindings: null, head: null, tail: null, pending: null, closing: false, closed: false, facade: this, witness: null }; this.#state = state;
    try {
      const current = client.installUiResidentPool(this, grant);
      if (current.kind !== "ready" || current.items !== 1 || current.bytes !== 64 || !client.ownsUiResidentPool(this)) throw new Error("Resident pool installation refused");
      state.pending = { requestOwner: null, cell: null, record: null, entry: null, phase: "empty", failure: NO_POOL_FAULT, witness: null }; state.bindings = new WeakMap(); poolWitness(state, this); Object.freeze(this);
    } catch (error) { state.failure = error; state.closing = true; state.phase = "closing"; throw error; }
  }
  static begin(client: ShardClient, ledger: OwnedResidentLedger, grant: NumericIndexGrant): OwnedUiResidentPoolAdmission {
    if (!ShardClient.matchesResidentLedger(client, ledger)) return { step: step("rejected", "resident-pool-composition"), pool: null };
    if (!admitted(grant, 1)) return { step: step("blocked", "resident-pool-admission"), pool: null };
    const prepared = client.prepareUiResidentPool(ledger, grant); const current = childStep(prepared, grant);
    if (current.kind !== "ready" || current.items !== 0 || current.bytes !== 0) return { step: current.kind === "ready" ? { ...current, kind: "pending" } : current, pool: null };
    const bytes = uiResidentMetadataEnvelope("pool").bytes + 64; if (!admitted(grant, bytes)) return { step: step("blocked", "resident-pool-construction"), pool: null };
    try { return { step: step("ready", "resident-pool-construction", bytes), pool: new OwnedUiResidentPool(MINT, client, ledger, { maxItems: 1, maxBytes: 64 }) }; }
    catch (error) { client.captureUiResidentPoolFault(error); return { step: step("rejected", "resident-pool-construction", bytes), pool: null }; }
  }
  static matchesComposition(pool: unknown, client: ShardClient, ledger: OwnedResidentLedger): pool is OwnedUiResidentPool {
    return pool !== null && typeof pool === "object" && #state in pool && pool.#state.composition === client && pool.#state.ledger === ledger && ShardClient.matchesResidentLedger(client, ledger);
  }
  get usage(): ResidentResources { const state = this.#state; if (!state.ledger) throw new Error("Resident pool is retired"); return state.ledger.usage.data; }
  bindInstance(owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime, grant: NumericIndexGrant): OwnedUiResidentInstanceAdmission {
    const result = (current: RetainedUiWireStep): OwnedUiResidentInstanceAdmission => ({ step: current, scope: null });
    if (!admitted(grant, 64)) return result(step("blocked", "resident-scope-admission")); const pool = this.#state; const slot = pool.pending;
    if (pool.closing || !pool.bindings || !slot || !ShardClient.matchesActivation(pool.composition, activation) || !OwnedUiInstance.matches(owner, activation, lifetime)) return result(step("rejected", "resident-scope-authority"));
    try {
      activation.assertActive(); const previous = pool.bindings.get(owner); if (previous) { const usable = scopeAvailable(previous); return { step: step(usable ? "ready" : "rejected", "resident-scope-held"), scope: usable ? previous : null }; }
      if (slot.requestOwner && slot.requestOwner !== owner) return result(step("blocked", "resident-scope-slot-busy"));
      if (slot.failure !== NO_POOL_FAULT || slot.cell?.hasFailure) return result(step("rejected", "resident-scope-fault-held"));
      const observed = observeSlot(pool, slot); if (observed) return result(observed);
      if (slot.phase === "empty") { if (!admitted(grant, 296)) return result(step("blocked", "resident-scope-bootstrap")); slot.requestOwner = owner; slot.phase = "preparing"; const current = pool.ledger!.prepareAdmission(slot, "data", grant); if (current.kind === "blocked") clearSlot(slot); else if (current.kind === "rejected") slot.phase = "bootstrap-rejected"; return result(admissionStep(current, grant)); }
      if (slot.phase === "cell-held") { slot.phase = "claiming"; const current = pool.ledger!.claimAdmission(slot, slot.cell!, grant); if (current.kind === "blocked") slot.phase = "cell-held"; return result(admissionStep(current, grant)); }
      if (slot.phase === "claimed") { if (!admitted(grant, 264)) return result(step("blocked", "resident-scope-record")); slot.phase = "record-admitting"; const current = pool.ledger!.reserveRecord("data", uiResidentMetadataEnvelope("instance"), slot.cell!, grant); if (current.step.kind === "blocked") slot.phase = "claimed"; return result(admissionStep(current.step, grant)); }
      if (slot.phase === "record-held") {
        const bytes = uiResidentMetadataEnvelope("instance").bytes + 64; if (!admitted(grant, bytes)) return result(step("blocked", "resident-scope-construction"));
        const state: Instance = { pool, owner, facade: null, activation, lifetime: null, previous: pool.tail, next: null, head: null, tail: null, children: 0, closing: false, closed: false, record: slot.record, cell: slot.cell, phase: "constructing", witness: null, failure: NO_POOL_FAULT, slot, bindingInstalled: false, pending: null }; slot.entry = state;
        instanceOwner(state); state.lifetime = Object.freeze({ activationGeneration: lifetime.activationGeneration, instanceId: lifetime.instanceId, guestLifetime: lifetime.guestLifetime }); slot.phase = "shell-installed"; return result(step("pending", "resident-scope-construction", bytes));
      }
      if (slot.phase === "shell-installed") { const state = slot.entry!; if (!owner.attachResidentScope(state.facade!)) return result(step("rejected", "resident-scope-owner-refused")); slot.phase = "roster-observing"; return result(step("pending", "resident-scope-owner-attachment", 64)); }
      if (slot.phase === "roster-observing") { if (!admitted(grant, 128)) return result(step("blocked", "resident-scope-publish")); const state = slot.entry!; const scope = state.facade!; pool.bindings.set(owner, scope); state.bindingInstalled = true; state.slot = null; state.phase = "live"; clearSlot(slot); return { step: step("ready", "resident-scope-admission", 128), scope }; }
      return result(step("rejected", "resident-scope-phase"));
    } catch (error) { slotFault(slot, error); if (slot.entry) { slot.entry.failure = error; slot.entry.closing = true; } return result(step("rejected", "resident-scope-fault")); }
  }
  beginClose(): void { const state = this.#state; state.closing = true; if (state.phase === "open") state.phase = "closing"; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "resident-pool-close"); const state = this.#state;
    if (!state.closing) throw new Error("Resident pool close has not begun"); if (state.closed) return step("complete", "resident-pool-close");
    if (!state.witness) { try { poolWitness(state, this); return step("pending", "resident-pool-witness", 64); } catch (error) { if (state.failure !== NO_POOL_FAULT && !Object.is(state.failure, error)) throw error; state.failure = error; state.composition!.captureUiResidentPoolFault(error); return step("rejected", "resident-pool-witness"); } }
    if (state.pending && state.pending.phase !== "empty") { try { return childStep(closeSlot(state, state.pending, grant), grant); } catch (error) { slotFault(state.pending, error); return step("rejected", "resident-scope-slot-fault"); } }
    if (state.head) return childStep(closeInstance(state.head, grant), grant);
    if (state.tail) return step("blocked", "resident-pool-children");
    if (state.failure !== NO_POOL_FAULT) return step("rejected", "resident-pool-unretired-fault");
    state.pending = null; state.bindings = null; state.ledger = null; state.composition = null; state.facade = null; state.phase = "closed"; state.closed = true; finishPoolWitness(state.witness, this); return step("complete", "resident-pool-close", 64);
  }
  terminalIsEmpty(): boolean { const state = this.#state; return state.closed && !state.bindings && !state.head && !state.tail && !state.pending && !state.ledger && !state.composition && !state.facade; }
  get retirement(): OwnedUiResidentPoolRetirement | null { return this.#state.closed ? this.#state.witness : null; }
}
/** 🧾️ Preadmitted proof of this exact pool's domain emptiness, separate from its composition-held intrinsic record. */
export class OwnedUiResidentPoolRetirement {
  readonly #pool: OwnedUiResidentPool;
  #terminal = false;
  private constructor(mint: object, state: Pool, pool: OwnedUiResidentPool) { if (mint !== MINT) throw new Error("Invalid pool retirement authority"); this.#pool = pool; state.witness = this; Object.freeze(this); }
  static {
    poolWitness = (state, pool) => new OwnedUiResidentPoolRetirement(MINT, state, pool);
    finishPoolWitness = (witness, pool) => { if (witness.#pool !== pool) throw new Error("Pool retirement identity differs"); witness.#terminal = true; };
  }
  static matches(witness: unknown, pool: OwnedUiResidentPool, client: ShardClient, ledger: OwnedResidentLedger): witness is OwnedUiResidentPoolRetirement {
    return witness !== null && typeof witness === "object" && #pool in witness && witness.#pool === pool && witness.#terminal && ShardClient.matchesResidentLedger(client, ledger) && Reflect.apply(ShardClient.prototype.ownsUiResidentPool, client, [pool]);
  }
}
//#endregion 🏦️SharedPool

//#region 🪪️LifetimeScope
class InstanceWitness {
  readonly #scope: OwnedUiResidentInstance;
  #terminal = false;
  constructor(mint: object, state: Instance) { if (mint !== MINT || !state.facade) throw new Error("Invalid resident instance witness"); this.#scope = state.facade; state.witness = this; Object.freeze(this); }
  static { createInstanceWitness = state => new InstanceWitness(MINT, state); markInstanceDomainClosed = (witness, state) => { if (state.witness !== witness || state.phase !== "domain-closed") throw new Error("Invalid resident domain transition"); witness.#terminal = true; }; instanceDomainClosed = (witness, state) => state.witness === witness && witness.#terminal && state.phase === "domain-closed" && !state.pool && !state.owner && !state.activation && !state.lifetime && !state.head && !state.tail && !state.previous && !state.next && !state.record && !state.cell && !state.pending && state.children === 0 && state.failure === NO_POOL_FAULT; }
  get scope(): OwnedUiResidentInstance { return this.#scope; }
}
/** 🪪️ One exact native lifetime owns all payload scopes admitted through this host pool. */
export class OwnedUiResidentInstance {
  readonly #state: Instance;
  private constructor(mint: object, state: Instance) {
    if (mint !== MINT) throw new Error("Invalid resident instance authority"); this.#state = state; state.facade = this; const pool = state.pool!;
    if (pool.tail) pool.tail.next = state; else pool.head = state; pool.tail = state;
    const installed = state.record!.install(this, { maxItems: 1, maxBytes: 64 }); if (installed.kind !== "ready") throw new Error("Resident instance record refused installation"); state.pending = newPayloadSlot(); createInstanceWitness(state); Object.freeze(this);
  }
  static { instanceOwner = state => new OwnedUiResidentInstance(MINT, state); scopeAvailable = scope => active(scope.#state); closeInstance = (state, grant) => { state.closing = true; return state.facade ? state.facade.#close(grant) : closeSlot(null, state.slot!, grant); }; }
  static matches(scope: unknown, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): scope is OwnedUiResidentInstance {
    if (scope === null || typeof scope !== "object" || !(#state in scope)) return false; const state = scope.#state;
    return !state.closed && state.owner === owner && state.activation === activation && state.lifetime !== null && state.lifetime.activationGeneration === lifetime.activationGeneration && state.lifetime.instanceId === lifetime.instanceId && state.lifetime.guestLifetime === lifetime.guestLifetime;
  }
  beginPayload(field: OwnedKernelReturnInputField, grant: NumericIndexGrant): OwnedUiResidentPayloadAdmission { return admitPayload(this.#state, field, grant); }
  beginClose(): void { this.#state.closing = true; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep { return this.#close(grant); }
  #close(grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "resident-instance-close"); const state = this.#state;
    if (!state.closing) throw new Error("Resident instance close has not begun"); if (state.closed) return step("complete", "resident-instance-close");
    if (state.phase === "domain-closed") { try { return closeSlot(null, state.slot!, grant); } catch (error) { slotFault(state.slot!, error); return step("rejected", "resident-scope-slot-fault"); } }
    if (state.pending && state.pending.phase !== "empty") { try { return childStep(closePayloadSlot(state, state.pending, grant), grant); } catch (error) { payloadSlotFault(state.pending, error); return step("rejected", "resident-payload-slot-fault"); } }
    if (state.head) return childStep(closePayload(state.head, grant), grant);
    if (state.children || state.tail) return step("blocked", "resident-instance-children"); const pool = state.pool!; const slot = pool.pending!;
    if (slot.phase !== "empty" && slot.entry !== state) return step("blocked", "resident-scope-slot-busy");
    if (!state.slot) { slot.requestOwner = state.owner; slot.cell = state.cell; slot.record = state.record; slot.entry = state; slot.witness = state.witness; slot.phase = "closing-domain"; state.slot = slot; return step("pending", "resident-scope-retirement-capture", 64); }
    if (state.failure !== NO_POOL_FAULT) { slotFault(slot, state.failure); if (!slot.cell!.hasFailure) return childStep(slot.cell!.retainFailure(state.failure, grant), grant); if (!Object.is(slot.cell!.failure, state.failure)) return step("rejected", "resident-scope-distinct-fault"); }
    if (!state.witness) { try { createInstanceWitness(state); slot.witness = state.witness; return step("pending", "resident-scope-domain-witness", 64); } catch (error) { slotFault(slot, error); return step("rejected", "resident-scope-domain-witness"); } }
    if (!admitted(grant, 128)) return step("blocked", "resident-scope-domain-unlink"); slot.witness = state.witness;
    if (pool.bindings!.get(state.owner!) === state.facade) pool.bindings!.delete(state.owner!);
    if (state.previous) state.previous.next = state.next; else pool.head = state.next; if (state.next) state.next.previous = state.previous; else pool.tail = state.previous;
    state.record = null; state.cell = null; state.previous = null; state.next = null; state.facade = null; state.pool = null; state.owner = null; state.activation = null; state.lifetime = null; state.failure = NO_POOL_FAULT; state.bindingInstalled = false; state.pending = null; state.phase = "domain-closed"; slot.requestOwner = null; slot.phase = "handoff-observing"; markInstanceDomainClosed(state.witness, state); return step("pending", "resident-scope-domain-unlink", 128);
  }
  terminalIsEmpty(): boolean { const state = this.#state; return state.closed && !state.pool && !state.owner && !state.facade && !state.activation && !state.lifetime && !state.head && !state.tail && !state.previous && !state.next && !state.record && !state.cell && !state.slot && !state.witness && !state.pending && state.failure === NO_POOL_FAULT && state.children === 0; }
}
//#endregion 🪪️LifetimeScope

//#region 📦️PayloadScope
//#region 📨️OriginalParentSlot
function newPayloadSlot(): PayloadSlot { return { requestOwner: null, cell: null, record: null, entry: null, phase: "empty", failure: NO_POOL_FAULT, witness: null }; }
function clearPayloadSlot(slot: PayloadSlot): void { slot.requestOwner = null; slot.cell = null; slot.record = null; slot.entry = null; slot.witness = null; slot.phase = "empty"; }
function payloadSlotFault(slot: PayloadSlot, error: unknown): void { if (slot.failure !== NO_POOL_FAULT && !Object.is(slot.failure, error)) throw error; slot.failure = error; if (slot.entry) { slot.entry.failure = error; slot.entry.closing = true; } }
function observePayloadSlot(instance: Instance, slot: PayloadSlot): RetainedUiWireStep | null {
  if (slot.phase === "bootstrap-rejected") { const cell = instance.pool!.ledger!.preparedAdmission(slot); if (cell) { slot.cell = cell; slot.phase = "cell-held"; } else clearPayloadSlot(slot); return step("pending", "resident-payload-rejection-observation", 64); }
  if (slot.phase === "preparing") { const cell = instance.pool!.ledger!.preparedAdmission(slot); if (!cell) return step("blocked", "resident-payload-cell-handoff"); slot.cell = cell; slot.phase = "cell-held"; return step("pending", "resident-payload-cell-observation", 64); }
  if (slot.phase === "claiming") { if (!slot.cell?.claimed) return step("rejected", "resident-payload-claim"); slot.phase = "claimed"; return step("pending", "resident-payload-claim-observation", 64); }
  if (slot.phase === "record-admitting") { slot.record = slot.cell!.result?.record ?? null; slot.phase = slot.record && slot.cell!.result?.step.kind === "ready" && !slot.cell!.hasFailure ? "record-held" : "fault-held"; return step(slot.phase === "record-held" ? "pending" : "rejected", "resident-payload-record-observation", 64); }
  return null;
}
function admitPayload(instance: Instance, field: OwnedKernelReturnInputField, grant: NumericIndexGrant): OwnedUiResidentPayloadAdmission {
  const result = (current: RetainedUiWireStep): OwnedUiResidentPayloadAdmission => ({ step: current, payload: null });
  if (!admitted(grant, 64)) return result(step("blocked", "resident-payload-admission")); const slot = instance.pending;
  if (!active(instance) || !slot || !OwnedKernelReturnInputField.matchesOwner(field, instance.owner!, instance.activation!, instance.lifetime!)) return result(step("rejected", "resident-payload-authority"));
  try {
    if (OwnedKernelReturnPayloadDetachment.matchesOwner(field.residentPayloadDetachment, field)) return result(step("rejected", "resident-payload-source-retired"));
    const previous = field.residentPayload(instance.facade!); if (previous) { const state = payloadState(previous); if (state && activePayload(state)) return { step: step("ready", "resident-payload-held"), payload: previous }; if (slot.entry !== state) return result(step("rejected", "resident-payload-held")); }
    if (slot.requestOwner && slot.requestOwner !== field) return result(step("blocked", "resident-payload-slot-busy")); if (slot.failure !== NO_POOL_FAULT || slot.cell?.hasFailure) return result(step("rejected", "resident-payload-fault-held"));
    const observed = observePayloadSlot(instance, slot); if (observed) return result(observed); const ledger = instance.pool!.ledger!;
    if (slot.phase === "empty") { if (!admitted(grant, 296)) return result(step("blocked", "resident-payload-bootstrap")); slot.requestOwner = field; slot.phase = "preparing"; const current = ledger.prepareAdmission(slot, "data", grant); if (current.kind === "blocked") clearPayloadSlot(slot); else if (current.kind === "rejected") slot.phase = "bootstrap-rejected"; return result(admissionStep(current, grant)); }
    if (slot.phase === "cell-held") { slot.phase = "claiming"; const current = ledger.claimAdmission(slot, slot.cell!, grant); if (current.kind === "blocked") slot.phase = "cell-held"; return result(admissionStep(current, grant)); }
    if (slot.phase === "claimed") { if (!admitted(grant, 264)) return result(step("blocked", "resident-payload-record")); slot.phase = "record-admitting"; const current = ledger.reserveRecord("data", uiResidentMetadataEnvelope("payload"), slot.cell!, grant); if (current.step.kind === "blocked") slot.phase = "claimed"; return result(admissionStep(current.step, grant)); }
    if (slot.phase === "record-held") {
      if (!admitted(grant, 272)) return result(step("blocked", "resident-payload-shell")); const state: Payload = { instance, facade: null, previous: instance.tail, next: null, head: null, tail: null, cursor: null, builder: null, storageCell: null, reader: null, evidence: null, closing: false, closed: false, field, record: slot.record, cell: slot.cell, phase: "constructing", witness: null, parentSlot: slot, failure: NO_POOL_FAULT, pending: null }; slot.entry = state; slot.phase = "shell-installed"; payloadOwner(state); return result(step("pending", "resident-payload-shell", 272));
    }
    if (slot.phase === "shell-installed") { slot.phase = "source-installing"; const current = field.installResidentPayload(slot.entry!.facade!, grant); if (current.kind === "blocked") slot.phase = "shell-installed"; return result(admissionStep({ ...current, phase: "resident-payload-source-install" }, grant)); }
    if (slot.phase === "source-installing") { if (!OwnedKernelReturnInputField.matchesResidentPayload(field, slot.entry!.facade)) return result(step("rejected", "resident-payload-source-observation")); slot.phase = "source-bound"; return result(step("pending", "resident-payload-source-observation", 64)); }
    if (slot.phase === "source-bound") { if (!admitted(grant, 104)) return result(step("blocked", "resident-payload-finalization")); const state = slot.entry!; state.pending = newBuilderSlot(); payloadWitness(state); Object.freeze(state.facade); slot.phase = "finalized"; return result(step("pending", "resident-payload-finalization", 104)); }
    if (slot.phase === "finalized") { const state = slot.entry!; state.phase = "live"; state.parentSlot = null; const payload = state.facade; clearPayloadSlot(slot); return { step: step("ready", "resident-payload-publication", 64), payload }; }
    return result(step("rejected", "resident-payload-phase"));
  } catch (error) { payloadSlotFault(slot, error); return result(step("rejected", "resident-payload-fault")); }
}
function closePayloadSlot(instance: Instance | null, slot: PayloadSlot, grant: NumericIndexGrant): RetainedUiWireStep {
  if (!admitted(grant, 64)) return step("blocked", "resident-payload-slot-close");
  if (slot.failure !== NO_POOL_FAULT) {
    if (!slot.cell) { const cell = instance?.pool?.ledger?.preparedAdmission(slot); if (!cell) return step("rejected", "resident-payload-fault-held"); slot.cell = cell; return step("pending", "resident-payload-fault-cell-observation", 64); }
    if (!slot.cell.hasFailure) return childStep(slot.cell.retainFailure(slot.failure, grant), grant); if (!Object.is(slot.cell.failure, slot.failure)) return step("rejected", "resident-payload-distinct-fault");
    if (!slot.record && slot.cell.result?.record) { slot.record = slot.cell.result.record; return step("pending", "resident-payload-fault-record-observation", 64); }
    if (slot.entry) return step("rejected", "resident-payload-fault-held");
  } else if (instance) { const observed = observePayloadSlot(instance, slot); if (observed) return observed; }
  const entry = slot.entry; if (entry && entry.phase !== "domain-retired" && entry.phase !== "registration-retired") return closePayload(entry, grant);
  if (slot.phase === "handoff-observing") { if (!entry || !entry.witness || slot.witness !== entry.witness || !payloadDomainEmpty(entry)) return step("rejected", "resident-payload-domain-proof"); slot.record!.beginClose(); slot.phase = slot.record!.matchesShell(payloadWitnessOriginal(entry.witness)) ? "detaching" : "record-closing"; return step("pending", "resident-payload-record-begin", 64); }
  if (slot.phase === "detaching") { const original = payloadWitnessOriginal(entry!.witness!); if (OwnedResidentRecordDetachment.matches(slot.record!.detachment, slot.record!, original)) { slot.phase = "record-closing"; return step("pending", "resident-payload-detach-observation", 64); } return childStep(slot.record!.detach(original, grant), grant); }
  if (slot.phase === "record-closing") { const current = slot.record!.closeStep(grant); const forwarded = childStep(current, grant); if (current.kind === "complete" && forwarded.kind === "pending") slot.phase = "record-observing"; return forwarded; }
  if (slot.phase === "record-observing") { if (!OwnedResidentRetirement.matches(slot.record!.retirement, slot.record)) return step("rejected", "resident-payload-record-proof"); slot.cell!.beginClose(); slot.phase = "cell-closing"; return step("pending", "resident-payload-cell-begin", 64); }
  if (slot.phase === "cell-closing") { const current = slot.cell!.closeStep(grant); const forwarded = childStep(current, grant); if (current.kind === "complete" && forwarded.kind === "pending") slot.phase = "cell-observing"; return forwarded; }
  if (slot.phase === "cell-observing") { if (slot.failure !== NO_POOL_FAULT || !slot.cell!.terminalIsEmpty() || slot.record && !slot.record.terminalIsEmpty()) return step("rejected", "resident-payload-cell-proof"); if (entry) { movePayloadWitness(entry, "registration-retired"); entry.parentSlot = null; entry.witness = null; entry.closed = true; } clearPayloadSlot(slot); return step("complete", "resident-payload-slot-close", 64); }
  if (entry) return step("rejected", "resident-payload-slot-phase");
  if (slot.record) { slot.record.beginClose(); slot.phase = "record-closing"; return step("pending", "resident-payload-unused-record", 64); } if (slot.cell) { slot.cell.beginClose(); slot.phase = "cell-closing"; return step("pending", "resident-payload-unused-cell", 64); }
  if (slot.failure !== NO_POOL_FAULT) return step("rejected", "resident-payload-fault-held"); clearPayloadSlot(slot); return step("complete", "resident-payload-slot-close", 64);
}
function payloadDomainEmpty(state: Payload): boolean { return state.phase === "domain-retired" && payloadBodyEmpty(state) && !state.field && !state.instance && !state.facade && !state.previous && !state.next && !state.record && !state.cell && !state.pending; }
//#endregion 📨️OriginalParentSlot

//#region 🏗️BuilderRegistration
function newBuilderSlot(): BuilderSlot { return { requestOwner: null, cell: null, record: null, entry: null, phase: "empty", failure: NO_POOL_FAULT, witness: null }; }
class BuilderWitness {
  readonly #builder: OwnedUiOperationPayloadBuilder;
  #phase: "constructed" | "body-retired" | "source-detached" | "source-settled" | "terminal" = "constructed";
  constructor(mint: object, entry: BuilderEntry) { if (mint !== MINT || !entry.facade) throw new Error("Invalid builder witness"); this.#builder = entry.facade; entry.witness = this; Object.freeze(this); }
  static { createBuilderWitness = entry => new BuilderWitness(MINT, entry); markBuilderWitness = (witness, builder) => { if (witness.#builder !== builder || !OwnedUiOperationPayloadBuilder.empty(builder)) throw new Error("Invalid builder terminal proof"); witness.#phase = "terminal"; }; moveBuilderWitness = (witness, builder, phase) => { if (witness.#builder !== builder || !OwnedUiOperationPayloadBuilder.bodyEmpty(builder) || phase === "body-retired" && witness.#phase !== "constructed" || phase === "source-detached" && (witness.#phase !== "body-retired" || !OwnedUiOperationPayloadBuilder.sourceDetached(builder)) || phase === "source-settled" && witness.#phase !== "source-detached") throw new Error("Invalid builder binding phase"); witness.#phase = phase; }; }
  static matchesBody(proof: unknown, builder: unknown, field: unknown): proof is BuilderWitness { return proof !== null && typeof proof === "object" && #phase in proof && proof.#phase === "body-retired" && proof.#builder === builder && OwnedUiOperationPayloadBuilder.bodyEmpty(proof.#builder) && OwnedUiOperationPayloadBuilder.matchesRetirementOwner(proof.#builder, field, proof); }
  static matchesDetached(proof: unknown, field: unknown): proof is BuilderWitness { return proof !== null && typeof proof === "object" && #phase in proof && proof.#phase === "source-detached" && OwnedUiOperationPayloadBuilder.sourceDetached(proof.#builder) && OwnedUiOperationPayloadBuilder.matchesRetirementOwner(proof.#builder, field, proof); }
  static matchesSourceBinding(proof: unknown, field: unknown): proof is BuilderWitness { return proof !== null && typeof proof === "object" && #phase in proof && (proof.#phase === "source-detached" || proof.#phase === "source-settled") && OwnedUiOperationPayloadBuilder.sourceDetached(proof.#builder) && OwnedUiOperationPayloadBuilder.matchesRetirementOwner(proof.#builder, field, proof); }
  get builder(): OwnedUiOperationPayloadBuilder { return this.#builder; }
  get terminal(): boolean { return this.#phase === "terminal" && OwnedUiOperationPayloadBuilder.empty(this.#builder); }
}
export { BuilderWitness as OwnedUiResidentBuilderRetirement };
function clearBuilderSlot(slot: BuilderSlot | EvidenceSlot | PageSlot | ReaderSlot): void { slot.requestOwner = null; slot.cell = null; slot.record = null; slot.entry = null; slot.witness = null; slot.phase = "empty"; }
function builderFault(slot: BuilderSlot | EvidenceSlot | PageSlot | ReaderSlot, error: unknown): void { if (slot.failure !== NO_POOL_FAULT && !Object.is(slot.failure, error)) throw error; slot.failure = error; }
function observeBuilderSlot(state: Payload, slot: BuilderSlot | EvidenceSlot, grant: NumericIndexGrant): RetainedUiWireStep | null {
  if ((slot.phase === "bootstrap-rejected" || slot.phase === "preparing" || slot.phase === "claiming" || slot.phase === "record-admitting") && !admitted(grant, 64)) return step("blocked", "resident-builder-observation");
  const ledger = state.instance!.pool!.ledger!;
  if (slot.phase === "bootstrap-rejected") { const cell = ledger.preparedAdmission(slot); if (cell) { slot.cell = cell; slot.phase = "cell-held"; } else clearBuilderSlot(slot); return step("pending", "resident-builder-rejection-observation", 64); }
  if (slot.phase === "preparing") { const cell = ledger.preparedAdmission(slot); if (!cell) return step("blocked", "resident-builder-cell-handoff"); slot.cell = cell; slot.phase = "cell-held"; return step("pending", "resident-builder-cell-observation", 64); }
  if (slot.phase === "claiming") { if (!slot.cell?.claimed) return step("rejected", "resident-builder-claim"); slot.phase = "claimed"; return step("pending", "resident-builder-claim-observation", 64); }
  if (slot.phase === "record-admitting") { slot.record = slot.cell!.result?.record ?? null; slot.phase = slot.record && slot.cell!.result?.step.kind === "ready" && !slot.cell!.hasFailure ? "record-held" : "fault-held"; return step(slot.phase === "record-held" ? "pending" : "rejected", "resident-builder-record-observation", 64); }
  return null;
}
function admitBuilder(state: Payload, field: OwnedKernelReturnInputField, grant: NumericIndexGrant): import("../🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationPayloadAdmission {
  const result = (current: RetainedUiWireStep): import("../🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationPayloadAdmission => ({ step: current, builder: null });
  if (!admitted(grant, 32)) return result(step("blocked", "resident-builder-admission")); const slot = state.pending;
  if (!activePayload(state) || !slot || state.field !== field || !OwnedKernelReturnInputField.matchesResidentPayload(field, state.facade)) return result(step("rejected", "resident-builder-owner"));
  if (pageSlot(slot) || readerSlot(slot) || evidenceSlot(slot)) return result(step("blocked", "resident-builder-slot-busy"));
  try {
    if (state.builder?.phase === "live") { const builder = state.builder.facade!; return OwnedUiOperationPayloadBuilder.healthy(builder) && !state.builder.cell?.hasFailure ? { step: step("ready", "resident-builder-held"), builder } : result(step("rejected", "resident-builder-held")); }
    if (slot.requestOwner && slot.requestOwner !== field) return result(step("blocked", "resident-builder-slot-busy")); if (slot.failure !== NO_POOL_FAULT || slot.cell?.hasFailure) return result(step("rejected", "resident-builder-fault-held"));
    const observed = observeBuilderSlot(state, slot, grant); if (observed) return result(observed); const ledger = state.instance!.pool!.ledger!;
    if (slot.phase === "empty") { if (!admitted(grant, 296)) return result(step("blocked", "resident-builder-bootstrap")); slot.requestOwner = field; slot.phase = "preparing"; const current = ledger.prepareAdmission(slot, "data", grant); if (current.kind === "blocked") clearBuilderSlot(slot); else if (current.kind === "rejected") slot.phase = "bootstrap-rejected"; return result(admissionStep(current, grant)); }
    if (slot.phase === "cell-held") { slot.phase = "claiming"; const current = ledger.claimAdmission(slot, slot.cell!, grant); if (current.kind === "blocked") slot.phase = "cell-held"; return result(admissionStep(current, grant)); }
    if (slot.phase === "claimed") { if (!admitted(grant, 264)) return result(step("blocked", "resident-builder-record")); slot.phase = "record-admitting"; const current = ledger.reserveRecord("data", uiResidentMetadataEnvelope("builder"), slot.cell!, grant); if (current.step.kind === "blocked") slot.phase = "claimed"; return result(admissionStep(current.step, grant)); }
    if (slot.phase === "record-held") { if (!admitted(grant, 56)) return result(step("blocked", "resident-builder-entry")); const entry: BuilderEntry = { facade: null, record: slot.record, cell: slot.cell, phase: "constructing", witness: null }; slot.entry = entry; state.builder = entry; slot.phase = "entry-held"; return result(step("pending", "resident-builder-entry", 56)); }
    if (slot.phase === "entry-held") { if (!admitted(grant, 272)) return result(step("blocked", "resident-builder-shell")); slot.phase = "constructing"; OwnedUiOperationPayloadBuilder.construct(field, state.facade!, grant); slot.phase = "shell-installed"; return result(step("pending", "resident-builder-shell", 272)); }
    if (slot.phase === "shell-installed") { if (!admitted(grant, 64)) return result(step("blocked", "resident-builder-source-bind")); slot.phase = "source-installing"; return result(admissionStep(OwnedUiOperationPayloadBuilder.bindSource(slot.entry!.facade!, state.facade!, grant), grant)); }
    if (slot.phase === "source-installing") { if (!admitted(grant, 64)) return result(step("blocked", "resident-builder-source-observation")); if (!OwnedKernelReturnInputField.matchesBuilder(field, slot.entry!.facade)) return result(step("rejected", "resident-builder-source-observation")); slot.phase = "source-bound"; return result(step("pending", "resident-builder-source-observation", 64)); }
    if (slot.phase === "source-bound") { if (!admitted(grant, 32)) return result(step("blocked", "resident-builder-witness")); createBuilderWitness(slot.entry!); slot.phase = "witness-ready"; return result(step("pending", "resident-builder-witness", 32)); }
    if (slot.phase === "witness-ready") { const current = OwnedUiOperationPayloadBuilder.finalize(slot.entry!.facade!, state.facade!, grant); if (current.kind === "pending") slot.phase = "finalized"; return result(admissionStep(current, grant)); }
    if (slot.phase === "finalized") { if (!admitted(grant, 64)) return result(step("blocked", "resident-builder-publication")); const entry = slot.entry!; entry.phase = "live"; clearBuilderSlot(slot); return { step: step("ready", "resident-builder-publication", 64), builder: entry.facade }; }
    return result(step("rejected", "resident-builder-phase"));
  } catch (error) { builderFault(slot, error); return result(step("rejected", "resident-builder-fault")); }
}
function closeBuilderSlot(state: Payload, grant: NumericIndexGrant): RetainedUiWireStep {
  if (!admitted(grant, 64)) return step("blocked", "resident-builder-close"); const slot = state.pending!; const ledger = state.instance!.pool!.ledger!;
  if (readerSlot(slot) || state.reader && payloadSlotEmpty(slot)) return closeReaderSlot(state, grant);
  if (pageSlot(slot) || state.head && payloadSlotEmpty(slot)) return closePageSlot(state, grant);
  if (evidenceSlot(slot)) {
    if (slot.failure !== NO_POOL_FAULT) { if (!slot.cell) { const cell = ledger.preparedAdmission(slot); if (!cell) return step("rejected", "resident-evidence-fault-held"); slot.cell = cell; return step("pending", "resident-evidence-fault-observation", 64); } if (!slot.cell.hasFailure) return childStep(slot.cell.retainFailure(slot.failure, grant), grant); return step("rejected", "resident-evidence-fault-held"); }
    if (OwnedUiOperationPayloadBuilder.cancellationPrepared(slot.requestOwner!) && (!state.evidence || state.evidence.phase === "constructing")) return admissionStep(admitEvidence(state, slot.requestOwner!, grant).step, grant);
    return advanceEvidence(state, slot.requestOwner!, grant);
  }
  if (state.evidence) return state.builder?.facade ? advanceEvidence(state, state.builder.facade, grant) : step("rejected", "resident-evidence-builder-missing");
  if (payloadSlotEmpty(slot) && state.builder?.facade && state.builder.phase === "live") {
    const builder = state.builder.facade;
    if (OwnedUiOperationPayloadBuilder.activeInput(builder)) return childStep(OwnedUiOperationPayloadBuilder.prepareInputCancellation(builder, state.facade!, grant), grant);
    if (OwnedUiOperationPayloadBuilder.cancellationPrepared(builder)) return admissionStep(admitEvidence(state, builder, grant).step, grant);
  }
  if (slot.failure !== NO_POOL_FAULT) {
    if (!slot.cell) { const cell = ledger.preparedAdmission(slot); if (!cell) return step("rejected", "resident-builder-fault-held"); slot.cell = cell; return step("pending", "resident-builder-fault-observation", 64); }
    if (!slot.cell.hasFailure) return childStep(slot.cell.retainFailure(slot.failure, grant), grant); return step("rejected", "resident-builder-fault-held");
  }
  const observed = observeBuilderSlot(state, slot, grant); if (observed) return observed;
  if (state.builder && !slot.entry) { const entry = state.builder; slot.requestOwner = state.field; slot.cell = entry.cell; slot.record = entry.record; slot.entry = entry; slot.witness = entry.witness; slot.phase = "closing-domain"; return step("pending", "resident-builder-close-capture", 64); }
  const entry = slot.entry;
  if (entry) {
    const builder = entry.facade;
    if (builder && !OwnedUiOperationPayloadBuilder.empty(builder)) {
      builder.beginClose(); if (!OwnedUiOperationPayloadBuilder.bodyEmpty(builder)) return childStep(builder.closeStep(grant), grant);
      if (!entry.witness) { createBuilderWitness(entry); slot.witness = entry.witness; return step("pending", "resident-builder-close-witness", 32); }
      slot.witness = entry.witness; const field = slot.requestOwner; if (!field) return step("rejected", "resident-builder-binding-owner");
      try {
        if (slot.phase === "binding-detaching") { if (OwnedKernelReturnInputField.matchesBuilderDetached(field, entry.witness)) { if (!OwnedUiOperationPayloadBuilder.sourceDetached(builder)) return childStep(OwnedUiOperationPayloadBuilder.detachRetirementSource(builder, state.facade!, grant), grant); moveBuilderWitness(entry.witness, builder, "source-detached"); slot.phase = "binding-settling"; return step("pending", "resident-builder-source-detachment-observation", 64); } return childStep({ ...field.detachBuilder(builder, entry.witness, grant), phase: "resident-builder-source-detach" }, grant); }
        if (slot.phase === "binding-settling") { if (OwnedKernelReturnInputField.matchesBuilderSettled(field, entry.witness)) { moveBuilderWitness(entry.witness, builder, "source-settled"); slot.phase = "binding-settled"; return step("pending", "resident-builder-source-settlement-observation", 64); } return childStep({ ...field.settleBuilder(entry.witness, grant), phase: "resident-builder-source-settle" }, grant); }
        if (slot.phase === "binding-settled") return childStep(OwnedUiOperationPayloadBuilder.finishRetirement(builder, state.facade!, grant), grant);
        moveBuilderWitness(entry.witness, builder, "body-retired"); slot.phase = "binding-detaching"; return step("pending", "resident-builder-body-proof", 64);
      } catch (error) { builderFault(slot, error); return step("rejected", "resident-builder-binding-fault"); }
    }
    if (builder && !entry.witness) { createBuilderWitness(entry); return step("pending", "resident-builder-close-witness", 32); }
    if (builder) { markBuilderWitness(entry.witness!, builder); slot.witness = entry.witness; }
    entry.facade = null; entry.cell = null; entry.record = null; entry.witness = null; entry.phase = "retired"; state.builder = null; slot.entry = null; slot.phase = "handoff-observing"; return step("pending", "resident-builder-domain-unlink", 64);
  }
  if (slot.phase === "handoff-observing") { if (slot.witness && !slot.witness.terminal) return step("rejected", "resident-builder-proof"); slot.record!.beginClose(); slot.phase = slot.witness ? "detaching" : "record-closing"; return step("pending", "resident-builder-record-begin", 64); }
  if (slot.phase === "detaching") { if (OwnedResidentRecordDetachment.matches(slot.record!.detachment, slot.record!, slot.witness!.builder)) { slot.phase = "record-closing"; return step("pending", "resident-builder-detach-observation", 64); } return childStep(slot.record!.detach(slot.witness!.builder, grant), grant); }
  if (slot.phase === "record-closing") { const current = slot.record!.closeStep(grant); const forwarded = childStep(current, grant); if (current.kind === "complete" && forwarded.kind === "pending") slot.phase = "record-observing"; return forwarded; }
  if (slot.phase === "record-observing") { if (!OwnedResidentRetirement.matches(slot.record!.retirement, slot.record)) return step("rejected", "resident-builder-record-proof"); slot.cell!.beginClose(); slot.phase = "cell-closing"; return step("pending", "resident-builder-cell-begin", 64); }
  if (slot.phase === "cell-closing") { const current = slot.cell!.closeStep(grant); const forwarded = childStep(current, grant); if (current.kind === "complete" && forwarded.kind === "pending") slot.phase = "cell-observing"; return forwarded; }
  if (slot.phase === "cell-observing") { if (!slot.cell!.terminalIsEmpty() || slot.record && !slot.record.terminalIsEmpty()) return step("rejected", "resident-builder-cell-proof"); clearBuilderSlot(slot); return step("complete", "resident-builder-slot-close", 64); }
  if (slot.record) { slot.record.beginClose(); slot.phase = "record-closing"; return step("pending", "resident-builder-unused-record", 64); }
  if (slot.cell) { slot.cell.beginClose(); slot.phase = "cell-closing"; return step("pending", "resident-builder-unused-cell", 64); }
  clearBuilderSlot(slot); return step("complete", "resident-builder-slot-close", 64);
}
//#endregion 🏗️BuilderRegistration

//#region 🧾️EvidenceRegistration
class EvidenceWitness {
  readonly #evidence: InputEvidence;
  #terminal = false;
  private constructor(mint: object, entry: EvidenceEntry) { if (mint !== MINT || !entry.facade) throw new Error("Invalid evidence witness"); this.#evidence = entry.facade; entry.witness = this; Object.freeze(this); }
  static { createEvidenceWitness = entry => new EvidenceWitness(MINT, entry); markEvidenceWitness = (witness, token) => { if (witness.#evidence !== token || !OwnedUiOperationPayloadBuilder.evidenceEmpty(token)) throw new Error("Invalid evidence terminal proof"); witness.#terminal = true; }; }
  get evidence(): InputEvidence { return this.#evidence; }
  get terminal(): boolean { return this.#terminal && OwnedUiOperationPayloadBuilder.evidenceEmpty(this.#evidence); }
}
function admitEvidence(state: Payload, builder: OwnedUiOperationPayloadBuilder, grant: NumericIndexGrant): OwnedUiResidentEvidenceAdmission {
  const result = (current: RetainedUiWireStep): OwnedUiResidentEvidenceAdmission => ({ step: current, evidence: null }); const slot = state.pending;
  if (!admitted(grant, 32)) return result(step("blocked", "resident-evidence-admission"));
  if (!state.instance?.pool?.ledger || !state.facade || !slot || state.failure !== NO_POOL_FAULT || state.cell?.hasFailure || state.builder?.facade !== builder || state.builder.phase !== "live" || state.builder.cell?.hasFailure || !OwnedUiOperationPayloadBuilder.evidenceEligible(builder, state.facade)) return result(step("rejected", "resident-evidence-owner"));
  if (slot.failure !== NO_POOL_FAULT || slot.cell?.hasFailure) return result(step("rejected", "resident-evidence-fault-held"));
  if (state.evidence?.phase === "live") return !state.evidence.cell?.hasFailure && OwnedUiOperationPayloadBuilder.matchesEvidence(state.evidence.facade, builder) ? { step: step("ready", "resident-evidence-held"), evidence: state.evidence.facade } : result(step("rejected", "resident-evidence-held"));
  if (slot.requestOwner !== null && slot.requestOwner !== builder) return result(step("blocked", "resident-evidence-slot-busy"));
  if (slot.failure !== NO_POOL_FAULT || slot.cell?.hasFailure) return result(step("rejected", "resident-evidence-fault-held"));
  try {
    const ledger = state.instance.pool.ledger;
    if (slot.phase === "empty") {
      if (!admitted(grant, 296)) return result(step("blocked", "resident-evidence-bootstrap")); slot.requestOwner = builder; slot.phase = "preparing";
      const current = ledger.prepareAdmission(slot, "data", grant); if (current.kind === "blocked") clearBuilderSlot(slot); else if (current.kind === "rejected") slot.phase = "bootstrap-rejected"; return result(admissionStep(current, grant));
    }
    if (!evidenceSlot(slot)) return result(step("rejected", "resident-evidence-slot-owner")); const observed = observeBuilderSlot(state, slot, grant); if (observed) return result(observed);
    if (slot.phase === "cell-held") { if (!admitted(grant, 64)) return result(step("blocked", "resident-evidence-claim")); slot.phase = "claiming"; const current = ledger.claimAdmission(slot, slot.cell!, grant); if (current.kind === "blocked") slot.phase = "cell-held"; return result(admissionStep(current, grant)); }
    if (slot.phase === "claimed") { if (!admitted(grant, 264)) return result(step("blocked", "resident-evidence-record")); slot.phase = "record-admitting"; const current = ledger.reserveRecord("data", uiResidentMetadataEnvelope("evidence"), slot.cell!, grant); if (current.step.kind === "blocked") slot.phase = "claimed"; return result(admissionStep(current.step, grant)); }
    if (slot.phase === "record-held") { if (!admitted(grant, 56)) return result(step("blocked", "resident-evidence-entry")); const entry: EvidenceEntry = { facade: null, record: slot.record, cell: slot.cell, phase: "constructing", witness: null }; state.evidence = entry; slot.entry = entry; slot.phase = "entry-held"; return result(step("pending", "resident-evidence-entry", 56)); }
    if (slot.phase === "entry-held") { if (!admitted(grant, 168)) return result(step("blocked", "resident-evidence-shell")); slot.phase = "constructing"; OwnedUiOperationPayloadBuilder.constructEvidence(builder, state.facade, grant); slot.phase = "shell-installed"; return result(step("pending", "resident-evidence-shell", 168)); }
    if (slot.phase === "shell-installed") { if (!admitted(grant, 32)) return result(step("blocked", "resident-evidence-witness")); createEvidenceWitness(slot.entry!); slot.phase = "witness-ready"; return result(step("pending", "resident-evidence-witness", 32)); }
    if (slot.phase === "witness-ready") { const current = OwnedUiOperationPayloadBuilder.finalizeEvidence(slot.entry!.facade!, builder, state.facade, grant); if (current.kind === "pending") slot.phase = "finalized"; return result(admissionStep(current, grant)); }
    if (slot.phase === "finalized") { if (!admitted(grant, 64)) return result(step("blocked", "resident-evidence-publication")); const entry = slot.entry!; OwnedUiOperationPayloadBuilder.publishEvidence(entry.facade!, builder, state.facade); entry.phase = "live"; clearBuilderSlot(slot); return { step: step("ready", "resident-evidence-publication", 64), evidence: entry.facade }; }
    return result(step("rejected", "resident-evidence-phase"));
  } catch (error) { builderFault(slot, error); return result(step("rejected", "resident-evidence-fault")); }
}
function advanceEvidence(state: Payload, builder: OwnedUiOperationPayloadBuilder, grant: NumericIndexGrant): RetainedUiWireStep {
  if (!admitted(grant, 1)) return step("blocked", "resident-evidence-retirement"); const slot = state.pending;
  if (!slot || !state.facade || state.builder?.facade !== builder || state.failure !== NO_POOL_FAULT || state.cell?.hasFailure) return step("rejected", "resident-evidence-retirement-owner");
  if (!state.evidence && payloadSlotEmpty(slot)) return step("complete", "resident-evidence-retirement");
  if (slot.requestOwner !== null && slot.requestOwner !== builder) return step("blocked", "resident-evidence-slot-busy");
  try {
    if (slot.phase === "closing-domain" && evidenceSlot(slot) && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure && slot.entry?.facade && !OwnedUiOperationPayloadBuilder.evidenceEmpty(slot.entry.facade)) return childStep(OwnedUiOperationPayloadBuilder.advanceEvidence(slot.entry.facade, builder, state.facade, grant), grant);
    if (!admitted(grant, 64)) return step("blocked", "resident-evidence-retirement");
    if (slot.failure !== NO_POOL_FAULT) { if (slot.cell && !slot.cell.hasFailure) return childStep(slot.cell.retainFailure(slot.failure, grant), grant); return step("rejected", "resident-evidence-fault-held"); }
    if (slot.cell?.hasFailure) return step("rejected", "resident-evidence-fault-held");
    if (slot.phase === "empty") { const entry = state.evidence; if (!entry || entry.phase !== "live" || !entry.facade || !OwnedUiOperationPayloadBuilder.matchesEvidence(entry.facade, builder)) return step("rejected", "resident-evidence-capture-owner"); slot.requestOwner = builder; if (!evidenceSlot(slot)) throw new Error("Invalid evidence slot"); slot.entry = entry; slot.cell = entry.cell; slot.record = entry.record; slot.witness = entry.witness; slot.phase = "closing-domain"; entry.phase = "closing"; return step("pending", "resident-evidence-capture", 64); }
    if (!evidenceSlot(slot)) return step("rejected", "resident-evidence-slot-owner");
    if (slot.phase === "closing-domain") { const entry = slot.entry; const token = entry?.facade; if (!entry || !token || !entry.witness || slot.witness !== entry.witness) return step("rejected", "resident-evidence-entry-owner"); if (!OwnedUiOperationPayloadBuilder.evidenceEmpty(token)) return childStep(OwnedUiOperationPayloadBuilder.advanceEvidence(token, builder, state.facade, grant), grant); markEvidenceWitness(entry.witness, token); entry.facade = null; entry.record = null; entry.cell = null; entry.witness = null; entry.phase = "retired"; state.evidence = null; slot.entry = null; slot.phase = "handoff-observing"; return step("pending", "resident-evidence-domain-unlink", 64); }
    if (slot.phase === "handoff-observing") { if (!slot.witness?.terminal || !slot.record) return step("rejected", "resident-evidence-body-proof"); slot.record.beginClose(); slot.phase = "detaching"; return step("pending", "resident-evidence-record-begin", 64); }
    if (slot.phase === "detaching") { if (OwnedResidentRecordDetachment.matches(slot.record!.detachment, slot.record!, slot.witness!.evidence)) { slot.phase = "record-closing"; return step("pending", "resident-evidence-detach-observation", 64); } return childStep(slot.record!.detach(slot.witness!.evidence, grant), grant); }
    if (slot.phase === "record-closing") { const current = slot.record!.closeStep(grant); const forwarded = childStep(current, grant); if (current.kind === "complete" && forwarded.kind === "pending") slot.phase = "record-observing"; return forwarded; }
    if (slot.phase === "record-observing") { if (!OwnedResidentRetirement.matches(slot.record!.retirement, slot.record)) return step("rejected", "resident-evidence-record-proof"); slot.cell!.beginClose(); slot.phase = "cell-closing"; return step("pending", "resident-evidence-cell-begin", 64); }
    if (slot.phase === "cell-closing") { const current = slot.cell!.closeStep(grant); const forwarded = childStep(current, grant); if (current.kind === "complete" && forwarded.kind === "pending") slot.phase = "cell-observing"; return forwarded; }
    if (slot.phase === "cell-observing") { if (!slot.cell!.terminalIsEmpty() || slot.record && !slot.record.terminalIsEmpty()) return step("rejected", "resident-evidence-cell-proof"); clearBuilderSlot(slot); return step("complete", "resident-evidence-slot-close", 64); }
    const observed = observeBuilderSlot(state, slot, grant); if (observed) return observed;
    if (slot.phase === "shell-installed" || slot.phase === "witness-ready" || slot.phase === "finalized") return admissionStep(admitEvidence(state, builder, grant).step, grant);
    if (slot.phase === "entry-held") { const entry = slot.entry; if (!entry || entry.facade || entry.witness || entry.record !== slot.record || entry.cell !== slot.cell) return step("rejected", "resident-evidence-unused-entry"); entry.record = null; entry.cell = null; entry.phase = "retired"; state.evidence = null; slot.entry = null; slot.phase = "record-held"; return step("pending", "resident-evidence-unused-entry", 64); }
    if (slot.phase === "record-held") { if (!slot.record || slot.entry || slot.witness) return step("rejected", "resident-evidence-unused-record"); slot.record.beginClose(); slot.phase = "record-closing"; return step("pending", "resident-evidence-unused-record", 64); }
    if (slot.phase === "cell-held" || slot.phase === "claimed") { if (!slot.cell || slot.record || slot.entry || slot.witness) return step("rejected", "resident-evidence-unused-cell"); slot.cell.beginClose(); slot.phase = "cell-closing"; return step("pending", "resident-evidence-unused-cell", 64); }
    return step("blocked", "resident-evidence-construction-held");
  } catch (error) { builderFault(slot, error); return step("rejected", "resident-evidence-retirement-fault"); }
}
//#endregion 🧾️EvidenceRegistration

/** 📦️ Keeps page reservations and shared aliases charged until their final explicit retirement. */
export class OwnedUiResidentPayload {
  readonly #state: Payload;
  private constructor(mint: object, state: Payload) { if (mint !== MINT) throw new Error("Invalid resident payload authority"); this.#state = state; state.facade = this; const instance = state.instance!; if (instance.tail) instance.tail.next = state; else instance.head = state; instance.tail = state; instance.children++; const installed = state.record!.install(this, { maxItems: 1, maxBytes: 64 }); if (installed.kind !== "ready") throw new Error("Resident payload record refused installation"); }
  static {
    payloadOwner = state => new OwnedUiResidentPayload(MINT, state);
    payloadState = payload => payload !== null && typeof payload === "object" && #state in payload ? payload.#state : null;
    closePayload = (state, grant) => { state.closing = true; return state.facade ? state.facade.#close(grant) : closePayloadSlot(null, state.parentSlot!, grant); };
  }
  static matchesBuilderConstruction(payload: unknown, field: unknown): payload is OwnedUiResidentPayload { const state = payloadState(payload); return state !== null && activePayload(state) && state.field === field && state.pending?.phase === "constructing" && state.pending.entry === state.builder && state.builder !== null && state.builder.facade === null && state.pending.failure === NO_POOL_FAULT && !state.pending.cell?.hasFailure; }
  static matchesBuilderPhase(payload: unknown, builder: unknown, phase: "source-installing" | "witness-ready"): payload is OwnedUiResidentPayload { const state = payloadState(payload); return state !== null && activePayload(state) && state.builder?.facade === builder && state.pending?.entry === state.builder && state.pending.phase === phase && state.pending.failure === NO_POOL_FAULT && !state.pending.cell?.hasFailure; }
  static matchesBuilderLive(payload: unknown, builder: unknown): payload is OwnedUiResidentPayload { const state = payloadState(payload); const entry = state?.builder; return state !== null && activePayload(state) && entry !== null && entry !== undefined && entry.facade === builder && entry.phase === "live" && !entry.cell?.hasFailure; }
  static matchesBuilderRetirement(payload: unknown, builder: unknown, field: unknown, witness: unknown): boolean { const state = payloadState(payload); const slot = state?.pending; return state !== null && state.failure === NO_POOL_FAULT && !state.cell?.hasFailure && !state.builder?.cell?.hasFailure && slot !== null && slot !== undefined && !pageSlot(slot) && !readerSlot(slot) && !evidenceSlot(slot) && slot.requestOwner !== null && slot.requestOwner === field && slot.entry !== null && slot.entry === state.builder && slot.entry.facade === builder && slot.entry.witness === witness && slot.witness === witness && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure && !state.evidence; }
  static matchesBuilderRetirementPhase(payload: unknown, builder: unknown, phase: "binding-detaching" | "binding-settled"): boolean { const state = payloadState(payload); const slot = state?.pending; if (!state || state.failure !== NO_POOL_FAULT || state.cell?.hasFailure || !slot || pageSlot(slot) || evidenceSlot(slot) || slot.phase !== phase || !slot.requestOwner || !slot.witness || !OwnedUiResidentPayload.matchesBuilderRetirement(payload, builder, slot.requestOwner, slot.witness)) return false; return phase === "binding-detaching" ? OwnedKernelReturnInputField.matchesBuilderDetached(slot.requestOwner, slot.witness) : OwnedKernelReturnInputField.matchesBuilderSettled(slot.requestOwner, slot.witness); }
  installBuilder(builder: OwnedUiOperationPayloadBuilder, grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "resident-builder-install"); const state = this.#state; const slot = state.pending; const entry = state.builder;
    if (!slot || slot.phase !== "constructing" || slot.entry !== entry || !entry || entry.facade || !OwnedUiOperationPayloadBuilder.matchesResident(builder, this) || !OwnedUiOperationPayloadBuilder.matchesField(builder, state.field!)) return step("rejected", "resident-builder-install"); entry.facade = builder;
    return entry.record!.install(builder, grant);
  }
  beginBuilder(field: OwnedKernelReturnInputField, grant: NumericIndexGrant): import("../🩹️operations/📥️wire/📄️pages/🟦️component.ts").OwnedUiOperationPayloadAdmission { return admitBuilder(this.#state, field, grant); }
  beginEvidence(builder: OwnedUiOperationPayloadBuilder, grant: NumericIndexGrant): OwnedUiResidentEvidenceAdmission { return admitEvidence(this.#state, builder, grant); }
  static matchesInputCancellation(payload: unknown, builder: unknown): boolean { const state = payloadState(payload); return state !== null && state.closing && state.phase === "live" && state.failure === NO_POOL_FAULT && !state.cell?.hasFailure && state.builder !== null && state.builder.facade === builder && state.builder.phase === "live" && !state.builder.cell?.hasFailure && state.pending !== null && payloadSlotEmpty(state.pending) && !state.reader && !state.head && !state.evidence; }
  advanceEvidence(builder: OwnedUiOperationPayloadBuilder, grant: NumericIndexGrant): RetainedUiWireStep { return advanceEvidence(this.#state, builder, grant); }
  static matchesEvidenceRetirement(payload: unknown, builder: unknown, token: unknown): boolean { const state = payloadState(payload); const slot = state?.pending; return state !== null && state.failure === NO_POOL_FAULT && !state.cell?.hasFailure && !state.builder?.cell?.hasFailure && slot !== null && slot !== undefined && evidenceSlot(slot) && slot.requestOwner === builder && state.builder?.facade === builder && slot.phase === "closing-domain" && slot.entry !== null && slot.entry === state.evidence && slot.entry.facade === token && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure; }
  static matchesEvidenceCancellation(payload: unknown, builder: unknown, token: unknown): boolean { const state = payloadState(payload); return state !== null && state.closing && OwnedUiResidentPayload.matchesEvidenceRetirement(payload, builder, token); }
  static matchesEvidencePhase(payload: unknown, builder: unknown, phase: "constructing" | "witness-ready" | "finalized"): payload is OwnedUiResidentPayload { const state = payloadState(payload); const slot = state?.pending; return state !== null && state.failure === NO_POOL_FAULT && !state.cell?.hasFailure && !state.builder?.cell?.hasFailure && slot !== null && slot !== undefined && evidenceSlot(slot) && slot.requestOwner === builder && slot.phase === phase && slot.entry === state.evidence && state.evidence !== null && state.builder?.facade === builder && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure; }
  installEvidence(token: InputEvidence, builder: OwnedUiOperationPayloadBuilder, grant: NumericIndexGrant): RetainedUiWireStep {
    if (!admitted(grant, 64)) return step("blocked", "resident-evidence-install"); const state = this.#state; const slot = state.pending; const entry = state.evidence;
    if (!slot || !evidenceSlot(slot) || slot.requestOwner !== builder || slot.phase !== "constructing" || slot.entry !== entry || !entry || entry.facade || !OwnedUiOperationPayloadBuilder.matchesEvidenceConstruction(token, builder)) return step("rejected", "resident-evidence-install"); entry.facade = token; return entry.record!.install(token, grant);
  }
  static matchesField(payload: unknown, field: unknown): payload is OwnedUiResidentPayload { const state = payloadState(payload); return state !== null && state.field !== null && state.field === field; }
  static matchesScope(payload: unknown, scope: unknown): payload is OwnedUiResidentPayload { const state = payloadState(payload); return state !== null && state.instance !== null && state.instance.facade !== null && state.instance.facade === scope; }
  static matchesSourceDetachment(payload: unknown, observation: unknown): payload is OwnedUiResidentPayload {
    const state = payloadState(payload); return state !== null && state.field === null && state.parentSlot !== null && state.parentSlot.entry === state && state.parentSlot.witness !== null && state.parentSlot.witness === observation && (state.phase === "source-detached" || state.phase === "source-settled") && payloadBodyEmpty(state);
  }
  static matchesOwner(payload: unknown, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): payload is OwnedUiResidentPayload {
    if (payload === null || typeof payload !== "object" || !(#state in payload)) return false; const state = payload.#state; const instance = state.instance;
    return !state.closing && !state.closed && state.failure === NO_POOL_FAULT && !state.cell?.hasFailure && instance !== null && active(instance) && instance.lifetime !== null && instance.owner === owner && instance.activation === activation && instance.lifetime.activationGeneration === lifetime.activationGeneration && instance.lifetime.instanceId === lifetime.instanceId && instance.lifetime.guestLifetime === lifetime.guestLifetime;
  }
  beginReader(builder: OwnedUiOperationPayloadBuilder, grant: NumericIndexGrant): OwnedUiResidentReaderAdmission { return admitReader(this.#state, builder, grant); }
  closeReader(reader: OwnedUiResidentPayloadReader, grant: NumericIndexGrant): RetainedUiWireStep { const original = readerState(reader); const state = this.#state; if (!original || original.payload !== state && (!state.pending || !readerSlot(state.pending) || state.pending.witness?.original !== reader)) return step("rejected", "resident-reader-close-owner"); return closeReaderSlot(state, grant, original); }
  static matchesReaderConstruction(payload: unknown, reader: unknown): boolean { const state = payloadState(payload); const slot = state?.pending; return state !== null && state.failure === NO_POOL_FAULT && !state.cell?.hasFailure && slot !== null && slot !== undefined && readerSlot(slot) && slot.phase === "builder-installing" && slot.entry === state.reader && slot.entry?.facade === reader && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure; }
  static matchesReaderBinding(payload: unknown, reader: unknown, witness: unknown): boolean { const state = payloadState(payload); const slot = state?.pending; return state !== null && state.failure === NO_POOL_FAULT && !state.cell?.hasFailure && slot !== null && slot !== undefined && readerSlot(slot) && slot.entry === state.reader && slot.entry?.facade === reader && slot.witness === witness && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure; }
  static matchesPageConstruction(payload: unknown, page: unknown): boolean { const state = payloadState(payload); const original = pageState(page); const slot = state?.pending; return state !== null && activePayload(state) && original !== null && original.payload === state && slot !== null && slot !== undefined && pageSlot(slot) && slot.phase === "page-builder-installing" && slot.entry === original && original.facade === page && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure; }
  static matchesPageRetirement(payload: unknown, page: unknown, proof: unknown): boolean { const state = payloadState(payload); const original = pageState(page); const slot = state?.pending; return state !== null && state.failure === NO_POOL_FAULT && !state.cell?.hasFailure && original !== null && original.payload === state && original.failure === NO_POOL_FAULT && !original.cell?.hasFailure && state.reader?.page !== original && slot !== null && slot !== undefined && pageSlot(slot) && slot.phase === "closing-domain" && slot.entry === original && original.facade === page && slot.witness !== null && slot.witness === proof && original.witness === proof && slot.failure === NO_POOL_FAULT && !slot.cell?.hasFailure; }
  static pageLength(payload: unknown, builder: unknown, page: unknown): number | null { const state = payloadState(payload); const original = pageState(page); return state !== null && activePayload(state) && state.builder?.facade === builder && original !== null && original.payload === state && original.facade === page && (original.phase === "live" || original.phase === "sealed") && original.failure === NO_POOL_FAULT && !original.cell?.hasFailure ? original.length : null; }
  beginPage(builder: OwnedUiOperationPayloadBuilder, length: number, grant: NumericIndexGrant): OwnedUiResidentPageAdmission { return admitPage(this.#state, builder, length, grant); }
  closePage(page: OwnedUiResidentPage, grant: NumericIndexGrant): RetainedUiWireStep { if (!admitted(grant, 64)) return step("blocked", "resident-page-close"); const original = pageState(page); const state = this.#state; if (!original) return step("rejected", "resident-page-close-owner"); const slot = state.pending; if (original.payload !== state && (!slot || !pageSlot(slot) || slot.witness?.original !== page)) return step("rejected", "resident-page-close-owner"); return closePageSlot(state, grant, original); }

  beginClose(): void { const state = this.#state; if (state.closing) return; state.closing = true; state.cursor = state.head; }
  closeStep(grant: NumericIndexGrant): RetainedUiWireStep {
    try { return this.#close(grant); } catch (error) { const state = this.#state; if (state.parentSlot) payloadSlotFault(state.parentSlot, error); else { if (state.failure !== NO_POOL_FAULT && !Object.is(state.failure, error)) throw error; state.failure = error; } return step("rejected", "resident-payload-close-fault"); }
  }
  #close(grant: NumericIndexGrant): RetainedUiWireStep {
    const state = this.#state;
    if (admitted(grant, 32) && state.closing && state.phase === "live" && state.failure === NO_POOL_FAULT && !state.cell?.hasFailure && state.builder?.facade && state.pending && (payloadSlotEmpty(state.pending) || evidenceSlot(state.pending)) && !state.reader && !state.head && (!state.evidence || state.evidence.phase === "constructing") && OwnedUiOperationPayloadBuilder.cancellationPrepared(state.builder.facade)) return admissionStep(admitEvidence(state, state.builder.facade, grant).step, grant);
    if (!admitted(grant, 64)) return step("blocked", "resident-payload-close");
    if (!state.closing) throw new Error("Resident payload close has not begun"); if (state.closed) return step("complete", "resident-payload-close");
    if (state.phase === "domain-retired") return closePayloadSlot(null, state.parentSlot!, grant);
    if (state.failure !== NO_POOL_FAULT) { if (state.cell && !state.cell.hasFailure) return childStep(state.cell.retainFailure(state.failure, grant), grant); return step("rejected", "resident-payload-fault-held"); }
    if (state.cell?.hasFailure) return step("rejected", "resident-payload-fault-held");
    if (state.pending && state.pending.phase !== "empty" || state.reader || state.builder || state.head) return childStep(closeBuilderSlot(state, grant), grant);
    if (state.storageCell) return closeStorage(state, grant);
    if (state.reader || state.builder) return step("blocked", "resident-payload-child-registration");
    if (state.cursor) return childStep(closePageSlot(state, grant, state.cursor), grant);
    if (state.evidence || state.head || state.tail) return step("blocked", "resident-payload-readers"); const instance = state.instance!; const slot = instance.pending!;
    if (!state.parentSlot) { if (slot.phase !== "empty") return step("blocked", "resident-payload-slot-busy"); slot.requestOwner = state.field; slot.cell = state.cell; slot.record = state.record; slot.entry = state; slot.witness = state.witness; slot.phase = "body-proving"; state.parentSlot = slot; return step("pending", "resident-payload-retirement-capture", 64); }
    if (!state.witness) { payloadWitness(state); return step("pending", "resident-payload-witness", 32); }
    if (state.phase === "constructing" || state.phase === "live") { if (!payloadBodyEmpty(state)) return step("rejected", "resident-payload-body-proof"); movePayloadWitness(state, "body-retired"); slot.phase = slot.phase === "shell-installed" ? "source-never-installed" : "source-detaching"; return step("pending", "resident-payload-body-proof", 64); }
    const field = slot.requestOwner;
    if (slot.phase === "source-detaching" || slot.phase === "source-never-installed") {
      if (!field) return step("rejected", "resident-payload-source-owner"); const observation = field.residentPayloadDetachment;
      if (OwnedKernelReturnPayloadDetachment.matches(observation, field, this)) { slot.witness = observation; slot.phase = "source-clearing"; return step("pending", "resident-payload-source-observation", 64); }
      if (slot.phase === "source-never-installed" && !OwnedKernelReturnInputField.matchesResidentPayload(field, this)) { state.field = null; movePayloadWitness(state, "source-settled"); slot.phase = "source-settled"; return step("pending", "resident-payload-never-installed-source", 64); }
      const current = field.detachResidentPayload(this, state.witness, grant); const forwarded = childStep({ ...current, phase: "resident-payload-source-detach" }, grant); if (current.kind === "pending" && forwarded.kind === "pending") slot.phase = "source-observing"; return forwarded;
    }
    if (slot.phase === "source-observing") { const observation = field!.residentPayloadDetachment; if (!OwnedKernelReturnPayloadDetachment.matches(observation, field, this)) return step("rejected", "resident-payload-source-observation"); slot.witness = observation; slot.phase = "source-clearing"; return step("pending", "resident-payload-source-observation", 64); }
    if (slot.phase === "source-clearing") { if (!OwnedKernelReturnPayloadDetachment.matches(slot.witness, field, this)) return step("rejected", "resident-payload-source-proof"); state.field = null; movePayloadWitness(state, "source-detached"); slot.phase = "source-settling"; return step("pending", "resident-payload-ui-source-detach", 64); }
    if (slot.phase === "source-settling") {
      if (OwnedKernelReturnPayloadDetachment.matchesSettled(slot.witness, this)) { slot.phase = "source-settle-observing"; return step("pending", "resident-payload-settle-recovery", 64); }
      if (!OwnedKernelReturnPayloadDetachment.matchesOwner(slot.witness, field)) return step("rejected", "resident-payload-source-proof"); const current = field!.settleResidentPayload(slot.witness, state.witness, grant); const forwarded = childStep({ ...current, phase: "resident-payload-source-settle" }, grant); if (current.kind === "complete" && forwarded.kind === "pending") slot.phase = "source-settle-observing"; return forwarded;
    }
    if (slot.phase === "source-settle-observing") { if (!OwnedKernelReturnPayloadDetachment.matchesSettled(slot.witness, this)) return step("rejected", "resident-payload-settled-proof"); movePayloadWitness(state, "source-settled"); slot.phase = "source-settled"; return step("pending", "resident-payload-settle-observation", 64); }
    if (slot.phase === "source-settled") { slot.witness = state.witness; slot.requestOwner = null; slot.phase = "closing-domain"; return step("pending", "resident-payload-domain-proof-handoff", 64); }
    if (slot.phase !== "closing-domain" || !admitted(grant, 128)) return step("blocked", "resident-payload-domain-unlink");
    if (state.previous) state.previous.next = state.next; else instance.head = state.next; if (state.next) state.next.previous = state.previous; else instance.tail = state.previous;
    instance.children--; state.previous = null; state.next = null; state.facade = null; state.instance = null; state.record = null; state.cell = null; state.pending = null; movePayloadWitness(state, "domain-retired"); slot.phase = "handoff-observing"; return step("pending", "resident-payload-domain-unlink", 128);
  }
  terminalIsEmpty(): boolean { const state = this.#state; return state.closed && !state.instance && !state.facade && !state.head && !state.tail && !state.cursor && !state.previous && !state.next && !state.builder && !state.storageCell && !state.reader && !state.field && !state.record && !state.cell && !state.witness && !state.parentSlot && !state.pending && state.failure === NO_POOL_FAULT && !state.evidence; }
}
/** 🧾️ One privately preadmitted payload witness changes phase without retaining a permanent source-field backlink. */
export class OwnedUiResidentPayloadSourceRelease {
  readonly #payload: OwnedUiResidentPayload;
  #phase: "constructed" | "body-retired" | "source-detached" | "source-settled" | "domain-retired" | "registration-retired" = "constructed";
  private constructor(mint: object, state: Payload) { if (mint !== MINT || !state.facade) throw new Error("Invalid resident payload source authority"); this.#payload = state.facade; state.witness = this; Object.freeze(this); }
  static {
    payloadWitness = state => new OwnedUiResidentPayloadSourceRelease(MINT, state);
    payloadWitnessOriginal = witness => witness.#payload;
    movePayloadWitness = (state, phase) => { const witness = state.witness; if (!witness || payloadState(witness.#payload) !== state) throw new Error("Invalid payload witness transition"); state.phase = phase; witness.#phase = phase; };
  }
  static matches(proof: unknown, payload: unknown, field: unknown): proof is OwnedUiResidentPayloadSourceRelease {
    if (proof === null || typeof proof !== "object" || !(#payload in proof) || proof.#payload !== payload || proof.#phase !== "body-retired") return false;
    const state = payloadState(payload); return state !== null && state.witness === proof && state.phase === "body-retired" && state.field !== null && state.field === field && payloadBodyEmpty(state);
  }
  static matchesDetached(proof: unknown, payload: unknown): proof is OwnedUiResidentPayloadSourceRelease {
    if (proof === null || typeof proof !== "object" || !(#payload in proof) || proof.#payload !== payload || proof.#phase !== "source-detached") return false;
    const state = payloadState(payload); return state !== null && state.witness === proof && state.phase === "source-detached" && state.field === null && state.parentSlot !== null && state.parentSlot.entry === state && state.parentSlot.witness !== null && payloadBodyEmpty(state);
  }
}
//#endregion 📦️PayloadScope

//#region 📄️FixedPageOwner

function pageSlot(slot: BuilderSlot | EvidenceSlot | PageSlot | ReaderSlot): slot is PageSlot { return typeof slot.requestOwner === "number"; }
function capturePageSlot(slot: BuilderSlot | EvidenceSlot | PageSlot | ReaderSlot, page: Page): slot is PageSlot { if (!payloadSlotEmpty(slot)) return false; slot.requestOwner = page.length; return pageSlot(slot); }
function pageStorage(page: Page) { return page.storageCell?.result?.page ?? null; }
function storageOwner(state: Payload) { return state.storageCell?.result?.owner ?? null; }
function closeStorage(state: Payload, grant: NumericIndexGrant): RetainedUiWireStep {
  const cell = state.storageCell; if (!cell) return step("complete", "resident-storage-empty");
  if (cell.hasFailure) return step("rejected", "resident-storage-fault-held");
  if (!cell.terminalIsEmpty()) { cell.beginClose(); return childStep(cell.closeStep(grant), grant); }
  if (!admitted(grant, 64)) return step("blocked", "resident-storage-observation"); state.storageCell = null; return step("pending", "resident-storage-observation", 64);
}
function admitPage(state: Payload, builder: OwnedUiOperationPayloadBuilder, length: number, grant: NumericIndexGrant): OwnedUiResidentPageAdmission {
  const result = (current: RetainedUiWireStep): OwnedUiResidentPageAdmission => ({ step: current, page: null }); const slot = state.pending;
  if (!admitted(grant, 32)) return result(step("blocked", "resident-page-admission"));
  if (!slot || !activePayload(state) || !OwnedUiResidentPayload.matchesBuilderLive(state.facade, builder) || !Number.isInteger(length) || length < 0 || length > 256) return result(step("rejected", "resident-page-owner"));
  if (slot.requestOwner !== null && (!pageSlot(slot) || slot.requestOwner !== length)) return result(step("blocked", "resident-page-slot-busy"));
  if (slot.failure !== NO_POOL_FAULT || slot.cell?.hasFailure || state.storageCell?.hasFailure || state.head && (state.head.failure !== NO_POOL_FAULT || state.head.cell?.hasFailure || state.head.storageCell?.hasFailure)) return result(step("rejected", "resident-page-fault-held"));
  if (state.head && (state.head.phase === "live" || state.head.phase === "sealed") && state.head.length === length && !state.head.cell?.hasFailure && !state.head.storageCell?.hasFailure) return { step: step("ready", "resident-page-held"), page: state.head.facade };
  try {
    const ledger = state.instance!.pool!.ledger!;
    if (slot.phase === "empty") {
      if (state.head) return result(step("blocked", "resident-page-window"));
      if (!admitted(grant, 296)) return result(step("blocked", "resident-page-bootstrap")); slot.requestOwner = length; if (!pageSlot(slot)) throw new Error("Invalid page slot"); slot.phase = state.storageCell ? "preparing" : "owner-preparing";
      const current = ledger.prepareAdmission(slot, "data", grant); if (current.kind === "blocked") clearBuilderSlot(slot); else if (current.kind === "rejected") slot.phase = "bootstrap-rejected"; return result(admissionStep(current, grant));
    }
    if (!pageSlot(slot)) return result(step("rejected", "resident-page-slot"));
    if (slot.phase === "owner-preparing" || slot.phase === "preparing") { if (!admitted(grant, 64)) return result(step("blocked", "resident-page-cell-observation")); const cell = ledger.preparedAdmission(slot); if (!cell) return result(step("rejected", "resident-page-cell-handoff")); slot.cell = cell; slot.phase = slot.phase === "owner-preparing" ? "owner-cell-held" : "cell-held"; return result(step("pending", "resident-page-cell-observation", 64)); }
    if (slot.phase === "owner-cell-held" || slot.phase === "cell-held") { if (!admitted(grant, 64)) return result(step("blocked", "resident-page-claim")); const previous = slot.phase; slot.phase = previous === "owner-cell-held" ? "owner-claiming" : "claiming"; const current = ledger.claimAdmission(slot, slot.cell!, grant); if (current.kind === "blocked") slot.phase = previous; return result(admissionStep(current, grant)); }
    if (slot.phase === "owner-claiming" || slot.phase === "claiming") { if (!admitted(grant, 64)) return result(step("blocked", "resident-page-claim-observation")); if (!slot.cell?.claimed) return result(step("rejected", "resident-page-claim")); slot.phase = slot.phase === "owner-claiming" ? "owner-claimed" : "claimed"; return result(step("pending", "resident-page-claim-observation", 64)); }
    if (slot.phase === "owner-claimed") { if (!admitted(grant, 200)) return result(step("blocked", "resident-storage-admission")); slot.phase = "owner-admitting"; const current = ledger.beginOwner("data", slot.cell!, grant); if (current.step.kind === "blocked") slot.phase = "owner-claimed"; return result(admissionStep(current.step, grant)); }
    if (slot.phase === "owner-admitting") { if (!admitted(grant, 64)) return result(step("blocked", "resident-storage-observation")); if (!slot.cell?.result?.owner || slot.cell.hasFailure || slot.cell.result.step.kind !== "ready") return result(step("rejected", "resident-storage-result")); state.storageCell = slot.cell; clearBuilderSlot(slot); return result(step("pending", "resident-storage-observation", 64)); }
    if (slot.phase === "claimed") { if (!admitted(grant, 264)) return result(step("blocked", "resident-page-record")); slot.phase = "record-admitting"; const current = ledger.reserveRecord("data", uiResidentMetadataEnvelope("page"), slot.cell!, grant); if (current.step.kind === "blocked") slot.phase = "claimed"; return result(admissionStep(current.step, grant)); }
    if (slot.phase === "record-admitting") { if (!admitted(grant, 64)) return result(step("blocked", "resident-page-record-observation")); slot.record = slot.cell!.result?.record ?? null; if (!slot.record || slot.cell!.hasFailure || slot.cell!.result?.step.kind !== "ready") return result(step("rejected", "resident-page-record-result")); slot.phase = "record-held"; return result(step("pending", "resident-page-record-observation", 64)); }
    if (slot.phase === "record-held") { if (!admitted(grant, 104)) return result(step("blocked", "resident-page-state")); slot.entry = { payload: state, facade: null, previous: null, next: null, length, cell: slot.cell, record: slot.record, storageCell: null, phase: "constructing", witness: null, failure: NO_POOL_FAULT }; slot.phase = "page-state"; return result(step("pending", "resident-page-state", 104)); }
    if (slot.phase === "page-state") { if (!admitted(grant, 88)) return result(step("blocked", "resident-page-shell")); pageOwner(slot.entry!); slot.phase = "page-shell"; return result(step("pending", "resident-page-shell", 88)); }
    const page = slot.entry;
    if (!page?.facade) return result(step("rejected", "resident-page-entry"));
    if (slot.phase === "page-shell") { if (!admitted(grant, 32)) return result(step("blocked", "resident-page-witness")); createPageWitness(page); slot.phase = "page-storage"; page.phase = "storage-empty"; return result(step("pending", "resident-page-witness", 32)); }
    if (slot.phase === "page-storage") {
      if (page.phase === "storage-empty") { if (!admitted(grant, 296)) return result(step("blocked", "resident-page-storage-bootstrap")); page.phase = "storage-preparing"; const current = ledger.prepareAdmission(page.facade, "data", grant); if (current.kind === "blocked") page.phase = "storage-empty"; else if (current.kind === "rejected") page.phase = "storage-rejected"; return result(admissionStep(current, grant)); }
      if (page.phase === "storage-preparing") { if (!admitted(grant, 64)) return result(step("blocked", "resident-page-storage-observation")); const cell = ledger.preparedAdmission(page.facade); if (!cell) return result(step("rejected", "resident-page-storage-handoff")); page.storageCell = cell; page.phase = "storage-cell-held"; return result(step("pending", "resident-page-storage-observation", 64)); }
      if (page.phase === "storage-cell-held") { if (!admitted(grant, 64)) return result(step("blocked", "resident-page-storage-claim")); page.phase = "storage-claiming"; const current = ledger.claimAdmission(page.facade, page.storageCell!, grant); if (current.kind === "blocked") page.phase = "storage-cell-held"; return result(admissionStep(current, grant)); }
      if (page.phase === "storage-claiming") { if (!admitted(grant, 64)) return result(step("blocked", "resident-page-storage-claim-observation")); if (!page.storageCell?.claimed) return result(step("rejected", "resident-page-storage-claim")); page.phase = "storage-claimed"; return result(step("pending", "resident-page-storage-claim-observation", 64)); }
      if (page.phase === "storage-claimed") { if (!admitted(grant, 264)) return result(step("blocked", "resident-page-storage")); const owner = storageOwner(state); if (!owner) return result(step("rejected", "resident-storage-owner")); page.phase = "storage-admitting"; const current = owner.reservePage(length, page.storageCell!, grant); if (current.step.kind === "blocked") page.phase = "storage-claimed"; return result(admissionStep(current.step, grant)); }
      if (page.phase === "storage-admitting") { if (!admitted(grant, 64)) return result(step("blocked", "resident-page-storage-result")); if (!pageStorage(page) || page.storageCell!.hasFailure || page.storageCell!.result?.step.kind !== "ready") return result(step("rejected", "resident-page-storage-result")); slot.phase = "page-binding"; return result(step("pending", "resident-page-storage-result", 64)); }
      return result(step("rejected", "resident-page-storage-phase"));
    }
    if (slot.phase === "page-binding") { if (!admitted(grant, 64)) return result(step("blocked", "resident-page-builder-install")); slot.phase = "page-builder-installing"; const current = admissionStep(OwnedUiOperationPayloadBuilder.installPage(builder, page.facade, state.facade!, grant), grant); if (current.kind === "blocked" && !OwnedUiOperationPayloadBuilder.matchesPage(builder, page.facade, state.facade!)) slot.phase = "page-binding"; return result(current); }
    if (slot.phase === "page-builder-installing") { if (!admitted(grant, 64)) return result(step("blocked", "resident-page-builder-observation")); if (!OwnedUiOperationPayloadBuilder.matchesPage(builder, page.facade, state.facade!)) return result(step("rejected", "resident-page-builder-owner")); slot.phase = "page-finalizing"; return result(step("pending", "resident-page-builder-observation", 64)); }
    if (slot.phase === "page-finalizing") { if (!admitted(grant, 64)) return result(step("blocked", "resident-page-finalize")); Object.freeze(page.facade); slot.phase = "finalized"; return result(step("pending", "resident-page-finalize", 64)); }
    if (slot.phase === "finalized") { if (!admitted(grant, 64)) return result(step("blocked", "resident-page-publication")); page.phase = "live"; clearBuilderSlot(slot); return { step: step("ready", "resident-page-publication", 64), page: page.facade }; }
    return result(step("rejected", "resident-page-phase"));
  } catch (error) { builderFault(slot, error); return result(step("rejected", "resident-page-admission-fault")); }
}

function pageDomainEmpty(page: Page): boolean { return (page.phase === "domain-retired" || page.phase === "registration-retired") && !page.payload && !page.facade && !page.previous && !page.next && !page.cell && !page.record && !page.storageCell && !page.witness && page.failure === NO_POOL_FAULT; }
class PageWitness {
  readonly #original: OwnedUiResidentPage;
  #phase: "constructed" | "terminal" = "constructed";
  private constructor(mint: object, page: Page) { if (mint !== MINT || !page.facade) throw new Error("Invalid page witness"); this.#original = page.facade; page.witness = this; Object.freeze(this); }
  static { createPageWitness = page => new PageWitness(MINT, page); markPageWitness = (witness, page) => { if (pageState(witness.#original) !== page || !pageDomainEmpty(page)) throw new Error("Invalid page terminal proof"); witness.#phase = "terminal"; }; }
  get original(): OwnedUiResidentPage { return this.#original; }
  get terminal(): boolean { const page = pageState(this.#original); return this.#phase === "terminal" && page !== null && pageDomainEmpty(page); }
}
function closePageSlot(state: Payload, grant: NumericIndexGrant, requested: Page | null = null): RetainedUiWireStep {
  if (!admitted(grant, 64)) return step("blocked", "resident-page-close"); if (state.failure !== NO_POOL_FAULT || state.cell?.hasFailure) return step("rejected", "resident-page-parent-fault"); const slot = state.pending; if (!slot) return step("rejected", "resident-page-parent-slot"); const ledger = state.instance!.pool!.ledger!;
  if (!pageSlot(slot)) {
    if (!payloadSlotEmpty(slot)) return step("blocked", "resident-page-slot-busy"); const page = requested ?? state.head; if (!page) return step("complete", "resident-page-empty");
    if (!capturePageSlot(state.pending!, page)) throw new Error("Invalid page retirement slot"); state.pending!.entry = page; state.pending!.cell = page.cell; state.pending!.record = page.record; state.pending!.witness = page.witness; state.pending!.phase = "closing-domain"; if (page.phase === "live" || page.phase === "sealed") page.phase = "closing"; return step("pending", "resident-page-close-capture", 64);
  }
  if (requested && slot.entry && slot.entry !== requested) return step("blocked", "resident-page-other-owner");
  try {
    if (slot.failure !== NO_POOL_FAULT) { if (!slot.cell) { const cell = ledger.preparedAdmission(slot); if (!cell) return step("rejected", "resident-page-fault-held"); slot.cell = cell; return step("pending", "resident-page-fault-observation", 64); } if (!slot.cell.hasFailure) return childStep(slot.cell.retainFailure(slot.failure, grant), grant); return step("rejected", "resident-page-fault-held"); }
    if (slot.cell?.hasFailure) return step("rejected", "resident-page-fault-held");
    if (slot.phase === "owner-preparing" || slot.phase === "preparing" || slot.phase === "bootstrap-rejected") { const cell = ledger.preparedAdmission(slot); if (!cell) { if (slot.phase !== "bootstrap-rejected") return step("blocked", "resident-page-admission-handoff"); clearBuilderSlot(slot); return step("pending", "resident-page-no-cell-observation", 64); } slot.cell = cell; slot.phase = "cell-held"; return step("pending", "resident-page-cell-observation", 64); }
    if (slot.phase === "record-admitting") { slot.record = slot.cell!.result?.record ?? null; slot.phase = "record-held"; return step("pending", "resident-page-record-observation", 64); }
    const page = slot.entry;
    if (page) {
      if (state.reader?.page === page) return step("blocked", "resident-page-reader-alias");
      if (page.failure !== NO_POOL_FAULT) { builderFault(slot, page.failure); return step("rejected", "resident-page-body-fault"); }
      if (!page.facade) { if (page.witness || page.storageCell) return step("rejected", "resident-page-unconstructed-body"); page.payload = null; page.record = null; page.cell = null; page.phase = "domain-retired"; slot.entry = null; slot.phase = "record-held"; return step("pending", "resident-page-unused-state", 64); }
      if (!page.witness) { createPageWitness(page); slot.witness = page.witness; return step("pending", "resident-page-close-witness", 32); }
      if (slot.phase !== "page-unbound") {
        slot.witness = page.witness; if (slot.phase !== "closing-domain") { slot.phase = "closing-domain"; return step("pending", "resident-page-binding-close", 64); }
        const builder = state.builder?.facade; if (!builder) return step("rejected", "resident-page-builder");
        if (OwnedUiOperationPayloadBuilder.matchesPageDetached(builder, page.facade, page.witness, state.facade!)) { slot.phase = "page-unbound"; return step("pending", "resident-page-builder-detach-observation", 64); }
        return childStep(OwnedUiOperationPayloadBuilder.detachPage(builder, page.facade, page.witness, state.facade!, grant), grant);
      }
      if (page.phase === "storage-preparing" || page.phase === "storage-rejected") { const cell = ledger.preparedAdmission(page.facade); if (!cell && page.phase !== "storage-rejected") return step("blocked", "resident-page-storage-handoff"); page.storageCell = cell; page.phase = "closing"; return step("pending", "resident-page-storage-close-observation", 64); }
      if (page.storageCell) { if (page.storageCell.hasFailure) return step("rejected", "resident-page-storage-fault-held"); if (!page.storageCell.terminalIsEmpty()) { page.storageCell.beginClose(); return childStep(page.storageCell.closeStep(grant), grant); } page.storageCell = null; page.phase = "closing"; return step("pending", "resident-page-storage-release-observation", 64); }
      slot.witness = page.witness; const witness = page.witness;
      if (page.previous) page.previous.next = page.next; else state.head = page.next; if (page.next) page.next.previous = page.previous; else state.tail = page.previous; if (state.cursor === page) state.cursor = page.next;
      page.payload = null; page.facade = null; page.previous = null; page.next = null; page.record = null; page.cell = null; page.witness = null; page.phase = "domain-retired"; markPageWitness(witness, page); slot.entry = null; slot.phase = "handoff-observing"; return step("pending", "resident-page-domain-unlink", 64);
    }
    if (slot.phase === "handoff-observing") { if (!slot.witness?.terminal) return step("rejected", "resident-page-domain-proof"); slot.record!.beginClose(); slot.phase = "detaching"; return step("pending", "resident-page-record-begin", 64); }
    if (slot.phase === "detaching") { if (OwnedResidentRecordDetachment.matches(slot.record!.detachment, slot.record!, slot.witness!.original)) { slot.phase = "record-closing"; return step("pending", "resident-page-detachment-observation", 64); } return childStep(slot.record!.detach(slot.witness!.original, grant), grant); }
    if (slot.phase === "record-closing") { const current = slot.record!.closeStep(grant); const result = childStep(current, grant); if (current.kind === "complete" && result.kind === "pending") slot.phase = "record-observing"; return result; }
    if (slot.phase === "record-observing") { if (!OwnedResidentRetirement.matches(slot.record!.retirement, slot.record)) return step("rejected", "resident-page-record-proof"); slot.cell!.beginClose(); slot.phase = "cell-closing"; return step("pending", "resident-page-cell-begin", 64); }
    if (slot.phase === "cell-closing") { const current = slot.cell!.closeStep(grant); const result = childStep(current, grant); if (current.kind === "complete" && result.kind === "pending") slot.phase = "cell-observing"; return result; }
    if (slot.phase === "cell-observing") { if (!slot.cell!.terminalIsEmpty() || slot.record && !slot.record.terminalIsEmpty()) return step("rejected", "resident-page-cell-proof"); if (slot.witness) { const original = pageState(slot.witness.original); if (!original || !slot.witness.terminal) return step("rejected", "resident-page-original-proof"); original.phase = "registration-retired"; } clearBuilderSlot(slot); return step("complete", "resident-page-close", 64); }
    if (slot.record) { slot.record.beginClose(); slot.phase = "record-closing"; return step("pending", "resident-page-unused-record", 64); }
    if (slot.cell) { slot.cell.beginClose(); slot.phase = "cell-closing"; return step("pending", "resident-page-unused-cell", 64); }
    return step("rejected", "resident-page-close-phase");
  } catch (error) { builderFault(slot, error); return step("rejected", "resident-page-close-fault"); }
}
/** 📄️ Original domain page; neutral storage and read capabilities remain under the privately registered payload. */
export class OwnedUiResidentPage {
  readonly #state: Page;
  private constructor(mint: object, state: Page) {
    if (mint !== MINT) throw new Error("Invalid resident page authority"); this.#state = state; state.facade = this; const payload = state.payload!; state.previous = payload.tail; if (payload.tail) payload.tail.next = state; else payload.head = state; payload.tail = state;
    const current = state.record!.install(this, { maxItems: 1, maxBytes: 64 }); if (current.kind !== "ready" || current.items !== 1 || current.bytes !== 64) throw new Error("Resident page record refused installation");
  }
  static { pageOwner = state => new OwnedUiResidentPage(MINT, state); pageState = value => value !== null && typeof value === "object" && #state in value ? value.#state : null; }
  allocate(grant: NumericIndexGrant): RetainedUiWireStep { const state = this.#state; if (!admitted(grant, 256)) return step("blocked", "resident-page-allocate"); if (state.phase !== "live" || !state.payload || !activePayload(state.payload) || state.failure !== NO_POOL_FAULT || state.cell?.hasFailure) return step("rejected", "resident-page-allocate"); try { const page = pageStorage(state); return page ? childStep(page.allocate(grant), grant) : step("rejected", "resident-page-storage"); } catch (error) { this.#retain(error); return step("rejected", "resident-page-allocation-fault"); } }
  writeByte(value: number, grant: NumericIndexGrant): RetainedUiWireStep { const state = this.#state; if (!admitted(grant, 1)) return step("blocked", "resident-page-write"); if (state.phase !== "live" || !state.payload || !activePayload(state.payload) || state.failure !== NO_POOL_FAULT || state.cell?.hasFailure) return step("rejected", "resident-page-write"); try { const page = pageStorage(state); return page ? childStep(page.writeByte(value, grant), grant) : step("rejected", "resident-page-storage"); } catch (error) { this.#retain(error); return step("rejected", "resident-page-write-fault"); } }
  seal(grant: NumericIndexGrant): RetainedUiWireStep { const state = this.#state; if (!admitted(grant, 64)) return step("blocked", "resident-page-seal"); if (state.phase !== "live" && state.phase !== "sealed" || !state.payload || !activePayload(state.payload) || state.failure !== NO_POOL_FAULT || state.cell?.hasFailure) return step("rejected", "resident-page-seal"); try { const page = pageStorage(state); if (!page) return step("rejected", "resident-page-storage"); const result = childStep(page.seal(grant), grant); if (result.kind === "ready") state.phase = "sealed"; return result; } catch (error) { this.#retain(error); return step("rejected", "resident-page-seal-fault"); } }
  beginClose(): void { if (this.#state.phase === "live" || this.#state.phase === "sealed") this.#state.phase = "closing"; }
  terminalIsEmpty(): boolean { return this.#state.phase === "registration-retired" && pageDomainEmpty(this.#state); }
  #retain(error: unknown): void { const state = this.#state; if (state.failure !== NO_POOL_FAULT && !Object.is(state.failure, error)) throw error; state.failure = error; }
}
//#endregion 📄️FixedPageOwner

//#region 📖️RegisteredStreamingReader
function readerSlot(slot: BuilderSlot | EvidenceSlot | PageSlot | ReaderSlot): slot is ReaderSlot { return payloadState(slot.requestOwner) !== null; }
function readerSlotStart(slot: BuilderSlot | EvidenceSlot | PageSlot | ReaderSlot, state: Payload): slot is ReaderSlot { if (!payloadSlotEmpty(slot)) return false; slot.requestOwner = state.facade; return readerSlot(slot); }
function readerHealthy(state: Reader): boolean { return state.failure === NO_POOL_FAULT && !state.cell?.hasFailure && state.payload !== null && state.payload.failure === NO_POOL_FAULT && !state.payload.cell?.hasFailure; }
function readerPublished(state: Reader): boolean { const slot = state.payload?.pending; return state.phase !== "constructing" && state.phase !== "alias-rejected" && state.phase !== "closing" && state.phase !== "body-retired" && state.phase !== "domain-retired" && state.phase !== "registration-retired" && (!slot || !readerSlot(slot)); }
function readerBodyEmpty(state: Reader): boolean { return !state.page && !state.storageCell && state.failure === NO_POOL_FAULT && !state.cell?.hasFailure && (state.phase === "closing" || state.phase === "body-retired"); }
function readerDomainEmpty(state: Reader): boolean { return !state.payload && !state.facade && !state.cell && !state.record && !state.page && !state.storageCell && !state.witness && state.failure === NO_POOL_FAULT && (state.phase === "domain-retired" || state.phase === "registration-retired"); }
function readerFault(state: Reader, error: unknown): void { if (state.failure !== NO_POOL_FAULT && !Object.is(state.failure, error)) throw error; state.failure = error; }
function admitReader(state: Payload, builder: OwnedUiOperationPayloadBuilder, grant: NumericIndexGrant): OwnedUiResidentReaderAdmission {
  const result = (current: RetainedUiWireStep): OwnedUiResidentReaderAdmission => ({ step: current, reader: null }); const slot = state.pending;
  if (!admitted(grant, 32)) return result(step("blocked", "resident-reader-admission"));
  if (!slot || !activePayload(state) || !OwnedUiResidentPayload.matchesBuilderLive(state.facade, builder)) return result(step("rejected", "resident-reader-owner"));
  if (slot.requestOwner !== null && (!readerSlot(slot) || slot.requestOwner !== state.facade)) return result(step("blocked", "resident-reader-slot-busy"));
  if (slot.failure !== NO_POOL_FAULT || slot.cell?.hasFailure || state.reader && !readerHealthy(state.reader)) return result(step("rejected", "resident-reader-fault-held"));
  if (state.reader && readerPublished(state.reader)) return { step: step("ready", "resident-reader-held"), reader: state.reader.facade };
  try {
    const ledger = state.instance!.pool!.ledger!;
    if (slot.phase === "empty") { if (state.reader || !OwnedUiOperationPayloadBuilder.readerAvailable(builder, state.facade!)) return result(step("rejected", "resident-reader-consumed")); if (!admitted(grant, 296)) return result(step("blocked", "resident-reader-bootstrap")); if (!readerSlotStart(slot, state)) return result(step("blocked", "resident-reader-slot-busy")); slot.phase = "preparing"; const current = ledger.prepareAdmission(slot, "data", grant); if (current.kind === "blocked") clearBuilderSlot(slot); else if (current.kind === "rejected") slot.phase = "bootstrap-rejected"; return result(admissionStep(current, grant)); }
    if (!readerSlot(slot)) return result(step("rejected", "resident-reader-slot"));
    if (slot.phase === "preparing") { if (!admitted(grant, 64)) return result(step("blocked", "resident-reader-cell-observation")); const cell = ledger.preparedAdmission(slot); if (!cell) return result(step("rejected", "resident-reader-cell-handoff")); slot.cell = cell; slot.phase = "cell-held"; return result(step("pending", "resident-reader-cell-observation", 64)); }
    if (slot.phase === "cell-held") { if (!admitted(grant, 64)) return result(step("blocked", "resident-reader-claim")); slot.phase = "claiming"; const current = ledger.claimAdmission(slot, slot.cell!, grant); if (current.kind === "blocked") slot.phase = "cell-held"; return result(admissionStep(current, grant)); }
    if (slot.phase === "claiming") { if (!admitted(grant, 64)) return result(step("blocked", "resident-reader-claim-observation")); if (!slot.cell?.claimed) return result(step("rejected", "resident-reader-claim")); slot.phase = "claimed"; return result(step("pending", "resident-reader-claim-observation", 64)); }
    if (slot.phase === "claimed") { if (!admitted(grant, 264)) return result(step("blocked", "resident-reader-record")); slot.phase = "record-admitting"; const current = ledger.reserveRecord("data", uiResidentMetadataEnvelope("reader"), slot.cell!, grant); if (current.step.kind === "blocked") slot.phase = "claimed"; return result(admissionStep(current.step, grant)); }
    if (slot.phase === "record-admitting") { if (!admitted(grant, 64)) return result(step("blocked", "resident-reader-record-observation")); slot.record = slot.cell!.result?.record ?? null; if (!slot.record || slot.cell!.hasFailure || slot.cell!.result?.step.kind !== "ready") return result(step("rejected", "resident-reader-record-result")); slot.phase = "record-held"; return result(step("pending", "resident-reader-record-observation", 64)); }
    if (slot.phase === "record-held") { if (!admitted(grant, 104)) return result(step("blocked", "resident-reader-state")); const reader: Reader = { payload: state, facade: null, cell: slot.cell, record: slot.record, page: null, storageCell: null, offset: 0, phase: "constructing", witness: null, failure: NO_POOL_FAULT, consumed: 0n }; slot.entry = reader; state.reader = reader; slot.phase = "reader-state"; return result(step("pending", "resident-reader-state", 104)); }
    const reader = slot.entry; if (!reader) return result(step("rejected", "resident-reader-entry"));
    if (slot.phase === "reader-state") { if (!admitted(grant, 88)) return result(step("blocked", "resident-reader-shell")); createReader(reader); slot.phase = "reader-shell"; return result(step("pending", "resident-reader-shell", 88)); }
    if (slot.phase === "reader-shell") { if (!admitted(grant, 32)) return result(step("blocked", "resident-reader-witness")); createReaderWitness(reader); slot.phase = "reader-witness"; return result(step("pending", "resident-reader-witness", 32)); }
    if (slot.phase === "reader-witness") { if (!admitted(grant, 64)) return result(step("blocked", "resident-reader-builder-install")); slot.phase = "builder-installing"; return result(admissionStep(OwnedUiOperationPayloadBuilder.installReader(builder, reader.facade!, state.facade!, grant), grant)); }
    if (slot.phase === "builder-installing") { if (!admitted(grant, 64)) return result(step("blocked", "resident-reader-builder-observation")); if (!OwnedUiOperationPayloadBuilder.matchesReader(builder, reader.facade!, state.facade!)) return result(step("rejected", "resident-reader-builder-owner")); slot.phase = "builder-installed"; return result(step("pending", "resident-reader-builder-observation", 64)); }
    if (slot.phase === "builder-installed") { if (!admitted(grant, 64)) return result(step("blocked", "resident-reader-finalize")); Object.freeze(reader.facade); slot.phase = "reader-finalized"; return result(step("pending", "resident-reader-finalize", 64)); }
    if (slot.phase === "reader-finalized") { if (!admitted(grant, 64)) return result(step("blocked", "resident-reader-publication")); reader.phase = "live"; clearBuilderSlot(slot); return { step: step("ready", "resident-reader-publication", 64), reader: reader.facade }; }
    return result(step("rejected", "resident-reader-phase"));
  } catch (error) { builderFault(slot, error); return result(step("rejected", "resident-reader-admission-fault")); }
}
function closeReaderAlias(state: Reader, grant: NumericIndexGrant): RetainedUiWireStep {
  const cell = state.storageCell; if (!cell) return step("complete", "resident-reader-alias-empty");
  if (cell.hasFailure) return step("rejected", "resident-reader-alias-fault-held");
  if (!cell.terminalIsEmpty()) { cell.beginClose(); return childStep(cell.closeStep(grant), grant); }
  if (!admitted(grant, 64)) return step("blocked", "resident-reader-alias-observation"); state.storageCell = null; return step("pending", "resident-reader-alias-observation", 64);
}
function advanceReader(state: Reader, grant: NumericIndexGrant): OwnedUiResidentReaderStep {
  if (!admitted(grant, 1)) return step("blocked", "resident-reader");
  if (!readerHealthy(state) || !readerPublished(state) || state.payload!.closing) return step("rejected", "resident-reader");
  const payload = state.payload!; const builder = payload.builder?.facade; if (!builder) return step("rejected", "resident-reader-builder");
  try {
    const ledger = payload.instance!.pool!.ledger!;
    if (state.phase === "live") {
      const page = payload.head;
      if (!page) return step(OwnedUiOperationPayloadBuilder.readerEof(builder, payload.facade!, state.consumed) ? "complete" : "blocked", "resident-reader-await-page");
      if (page.phase !== "sealed") return step("blocked", "resident-reader-await-seal"); if (!admitted(grant, 64)) return step("blocked", "resident-reader-page-capture"); state.page = page; state.offset = 0; state.phase = "page-held"; return step("pending", "resident-reader-page-capture", 64);
    }
    if (state.phase === "page-held") { if (!admitted(grant, 296)) return step("blocked", "resident-reader-alias-bootstrap"); state.phase = "alias-preparing"; const current = ledger.prepareAdmission(state.facade!, "data", grant); if (current.kind === "blocked") state.phase = "page-held"; else if (current.kind === "rejected") state.phase = "alias-rejected"; return admissionStep(current, grant); }
    if (state.phase === "alias-preparing") { if (!admitted(grant, 64)) return step("blocked", "resident-reader-alias-observation"); const cell = ledger.preparedAdmission(state.facade!); if (!cell) return step("rejected", "resident-reader-alias-handoff"); state.storageCell = cell; state.phase = "alias-held"; return step("pending", "resident-reader-alias-observation", 64); }
    if (state.phase === "alias-held") { if (!admitted(grant, 64)) return step("blocked", "resident-reader-alias-claim"); state.phase = "alias-claiming"; const current = ledger.claimAdmission(state.facade!, state.storageCell!, grant); if (current.kind === "blocked") state.phase = "alias-held"; return admissionStep(current, grant); }
    if (state.phase === "alias-claiming") { if (!admitted(grant, 64)) return step("blocked", "resident-reader-alias-claim-observation"); if (!state.storageCell?.claimed) return step("rejected", "resident-reader-alias-claim"); state.phase = "alias-claimed"; return step("pending", "resident-reader-alias-claim-observation", 64); }
    if (state.phase === "alias-claimed") { if (!admitted(grant, 136)) return step("blocked", "resident-reader-alias-admission"); const owner = storageOwner(payload); const page = state.page && pageStorage(state.page); if (!owner || !page) return step("rejected", "resident-reader-alias-source"); state.phase = "alias-admitting"; const current = owner.beginRead(page, state.storageCell!, grant); if (current.step.kind === "blocked") state.phase = "alias-claimed"; return admissionStep(current.step, grant); }
    if (state.phase === "alias-admitting") { if (!admitted(grant, 64)) return step("blocked", "resident-reader-alias-result"); if (!state.storageCell?.result?.reader || state.storageCell.hasFailure || state.storageCell.result.step.kind !== "ready") return step("rejected", "resident-reader-alias-result"); state.phase = "reading"; return step("pending", "resident-reader-alias-result", 64); }
    if (state.phase === "reading") { if (state.offset === state.page!.length) { if (!admitted(grant, 64)) return step("blocked", "resident-reader-page-end"); state.phase = "alias-closing"; return step("pending", "resident-reader-page-end", 64); } const value = state.storageCell!.result!.reader!.byteAt(state.offset); state.offset++; state.consumed++; return { kind: "byte", value, items: 1, bytes: 1 }; }
    if (state.phase === "alias-closing") { if (state.storageCell) return closeReaderAlias(state, grant); if (!admitted(grant, 64)) return step("blocked", "resident-reader-page-detach"); state.page = null; state.phase = "page-retiring"; return step("pending", "resident-reader-page-detach", 64); }
    if (state.phase === "page-retiring") { const current = closePageSlot(payload, grant); const forwarded = childStep(current, grant); if (current.kind === "complete" && forwarded.kind === "pending") state.phase = "page-observing"; return forwarded; }
    if (state.phase === "page-observing") { if (!admitted(grant, 64)) return step("blocked", "resident-reader-page-observation"); if (payload.head || !payload.pending || !payloadSlotEmpty(payload.pending)) return step("rejected", "resident-reader-page-observation"); state.phase = "live"; return step("pending", "resident-reader-page-observation", 64); }
    return step("rejected", "resident-reader-phase");
  } catch (error) { readerFault(state, error); return step("rejected", "resident-reader-fault"); }
}
/** 📖️ One preadmitted parser reads the original page window without escaping a byte backing. */
export class OwnedUiResidentPayloadReader {
  readonly #state: Reader;
  private constructor(mint: object, state: Reader) { if (mint !== MINT) throw new Error("Invalid resident reader authority"); this.#state = state; state.facade = this; const current = state.record!.install(this, { maxItems: 1, maxBytes: 64 }); if (current.kind !== "ready" || current.items !== 1 || current.bytes !== 64) throw new Error("Reader record installation refused"); }
  static { createReader = state => new OwnedUiResidentPayloadReader(MINT, state); readerState = value => value !== null && typeof value === "object" && #state in value ? value.#state : null; }
  advance(grant: NumericIndexGrant): OwnedUiResidentReaderStep { return advanceReader(this.#state, grant); }
  terminalIsEmpty(): boolean { return this.#state.phase === "registration-retired" && readerDomainEmpty(this.#state); }
}
/** 🧾️ The same original witness traverses body, builder-binding and domain retirement. */
export class OwnedUiResidentReaderRetirement {
  readonly #original: OwnedUiResidentPayloadReader;
  #phase: "constructed" | "body-retired" | "detached" | "settled" | "terminal" = "constructed";
  private constructor(mint: object, state: Reader) { if (mint !== MINT || !state.facade) throw new Error("Invalid reader witness"); this.#original = state.facade; state.witness = this; Object.freeze(this); }
  static { createReaderWitness = state => new OwnedUiResidentReaderRetirement(MINT, state); moveReaderWitness = (witness, phase) => { witness.#phase = phase; }; }
  static matchesBody(proof: unknown, reader: unknown, payload: OwnedUiResidentPayload): boolean { return proof !== null && typeof proof === "object" && #original in proof && proof.#original === reader && proof.#phase === "body-retired" && OwnedUiResidentPayload.matchesReaderBinding(payload, reader, proof) && readerBodyEmpty(readerState(reader)!); }
  static matchesDetached(proof: unknown, reader: unknown, payload: OwnedUiResidentPayload): boolean { return proof !== null && typeof proof === "object" && #original in proof && proof.#original === reader && proof.#phase === "detached" && OwnedUiResidentPayload.matchesReaderBinding(payload, reader, proof); }
  get original(): OwnedUiResidentPayloadReader { return this.#original; }
  get terminal(): boolean { const state = readerState(this.#original); return this.#phase === "terminal" && state !== null && readerDomainEmpty(state); }
}
function closeReaderSlot(state: Payload, grant: NumericIndexGrant, requested: Reader | null = null): RetainedUiWireStep {
  if (!admitted(grant, 64)) return step("blocked", "resident-reader-close"); const slot = state.pending; if (!slot || state.failure !== NO_POOL_FAULT || state.cell?.hasFailure) return step("rejected", "resident-reader-parent");
  const ledger = state.instance!.pool!.ledger!;
  if (!readerSlot(slot)) { const reader = requested ?? state.reader; if (!reader || !readerSlotStart(state.pending!, state)) return step("blocked", "resident-reader-slot-busy"); const held = state.pending!; held.entry = reader; held.cell = reader.cell; held.record = reader.record; held.witness = reader.witness; held.phase = "closing-domain"; return step("pending", "resident-reader-close-capture", 64); }
  if (requested && slot.entry && requested !== slot.entry) return step("blocked", "resident-reader-other-owner");
  try {
    if (slot.failure !== NO_POOL_FAULT) { if (!slot.cell) { const cell = ledger.preparedAdmission(slot); if (!cell) return step("rejected", "resident-reader-fault-held"); slot.cell = cell; return step("pending", "resident-reader-fault-observation", 64); } if (!slot.cell.hasFailure) return childStep(slot.cell.retainFailure(slot.failure, grant), grant); return step("rejected", "resident-reader-fault-held"); }
    if (slot.cell?.hasFailure) return step("rejected", "resident-reader-fault-held");
    if (slot.phase === "preparing" || slot.phase === "bootstrap-rejected") { const cell = ledger.preparedAdmission(slot); if (!cell) { if (slot.phase !== "bootstrap-rejected") return step("blocked", "resident-reader-cell-handoff"); clearBuilderSlot(slot); return step("pending", "resident-reader-no-cell-observation", 64); } slot.cell = cell; slot.phase = "cell-held"; return step("pending", "resident-reader-cell-observation", 64); }
    if (slot.phase === "record-admitting") { slot.record = slot.cell!.result?.record ?? null; slot.phase = "record-held"; return step("pending", "resident-reader-record-observation", 64); }
    const reader = slot.entry;
    if (reader) {
      if (reader.failure !== NO_POOL_FAULT) { builderFault(slot, reader.failure); return step("rejected", "resident-reader-body-fault"); }
      if (!reader.facade) { reader.payload = null; reader.cell = null; reader.record = null; reader.phase = "domain-retired"; state.reader = null; slot.entry = null; slot.phase = "record-held"; return step("pending", "resident-reader-unused-state", 64); }
      if (reader.phase === "alias-preparing" || reader.phase === "alias-rejected") { const cell = ledger.preparedAdmission(reader.facade); if (!cell && reader.phase !== "alias-rejected") return step("blocked", "resident-reader-alias-handoff"); reader.storageCell = cell; reader.phase = "closing"; return step("pending", "resident-reader-alias-close-observation", 64); }
      if (reader.storageCell) return closeReaderAlias(reader, grant);
      if (reader.page) { reader.page = null; reader.phase = "closing"; return step("pending", "resident-reader-page-detach", 64); }
      if (!reader.witness) { createReaderWitness(reader); slot.witness = reader.witness; return step("pending", "resident-reader-close-witness", 32); }
      slot.witness = reader.witness; const builder = state.builder?.facade; if (!builder) return step("rejected", "resident-reader-builder");
      if (slot.phase === "binding-detaching") { if (OwnedUiOperationPayloadBuilder.matchesReaderDetached(builder, reader.facade, reader.witness, state.facade!)) { moveReaderWitness(reader.witness, "detached"); slot.phase = "binding-settling"; return step("pending", "resident-reader-binding-detach-observation", 64); } return childStep(OwnedUiOperationPayloadBuilder.detachReader(builder, reader.facade, reader.witness, state.facade!, grant), grant); }
      if (slot.phase === "binding-settling") { if (OwnedUiOperationPayloadBuilder.matchesReaderSettled(builder, reader.facade, reader.witness, state.facade!)) { moveReaderWitness(reader.witness, "settled"); slot.phase = "binding-settled"; return step("pending", "resident-reader-binding-settle-observation", 64); } return childStep(OwnedUiOperationPayloadBuilder.settleReader(builder, reader.facade, reader.witness, state.facade!, grant), grant); }
      if (slot.phase !== "binding-settled") { reader.phase = "body-retired"; moveReaderWitness(reader.witness, "body-retired"); slot.phase = "binding-detaching"; return step("pending", "resident-reader-body-proof", 64); }
      const witness = reader.witness; reader.payload = null; reader.facade = null; reader.cell = null; reader.record = null; reader.witness = null; reader.phase = "domain-retired"; state.reader = null; slot.entry = null; moveReaderWitness(witness, "terminal"); slot.phase = "handoff-observing"; return step("pending", "resident-reader-domain-unlink", 64);
    }
    if (slot.phase === "handoff-observing") { if (!slot.witness?.terminal) return step("rejected", "resident-reader-domain-proof"); slot.record!.beginClose(); slot.phase = "detaching"; return step("pending", "resident-reader-record-begin", 64); }
    if (slot.phase === "detaching") { if (OwnedResidentRecordDetachment.matches(slot.record!.detachment, slot.record!, slot.witness!.original)) { slot.phase = "record-closing"; return step("pending", "resident-reader-detach-observation", 64); } return childStep(slot.record!.detach(slot.witness!.original, grant), grant); }
    if (slot.phase === "record-closing") { const current = slot.record!.closeStep(grant); const result = childStep(current, grant); if (current.kind === "complete" && result.kind === "pending") slot.phase = "record-observing"; return result; }
    if (slot.phase === "record-observing") { if (!OwnedResidentRetirement.matches(slot.record!.retirement, slot.record)) return step("rejected", "resident-reader-record-proof"); slot.cell!.beginClose(); slot.phase = "cell-closing"; return step("pending", "resident-reader-cell-begin", 64); }
    if (slot.phase === "cell-closing") { const current = slot.cell!.closeStep(grant); const result = childStep(current, grant); if (current.kind === "complete" && result.kind === "pending") slot.phase = "cell-observing"; return result; }
    if (slot.phase === "cell-observing") { if (!slot.cell!.terminalIsEmpty() || slot.record && !slot.record.terminalIsEmpty()) return step("rejected", "resident-reader-cell-proof"); if (slot.witness) { if (!slot.witness.terminal) return step("rejected", "resident-reader-original-proof"); readerState(slot.witness.original)!.phase = "registration-retired"; } clearBuilderSlot(slot); return step("complete", "resident-reader-close", 64); }
    if (slot.record) { slot.record.beginClose(); slot.phase = "record-closing"; return step("pending", "resident-reader-unused-record", 64); }
    if (slot.cell) { slot.cell.beginClose(); slot.phase = "cell-closing"; return step("pending", "resident-reader-unused-cell", 64); }
    return step("rejected", "resident-reader-close-phase");
  } catch (error) { builderFault(slot, error); return step("rejected", "resident-reader-close-fault"); }
}
//#endregion 📖️RegisteredStreamingReader
