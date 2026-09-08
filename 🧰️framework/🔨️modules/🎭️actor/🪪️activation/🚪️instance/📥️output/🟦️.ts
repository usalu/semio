import { OwnedResidentLedger, OwnedResidentRecordDetachment, OwnedResidentRetirement, type OwnedResidentAdmission, type OwnedResidentRecord, type ResidentGrant, type ResidentStep } from "../../../../🌱️value/💾️resident/🟦️.ts";

//#region 🧬️OutputReservation
export type OwnedActorTurnOutputState = { readonly capacity: number; readonly sequence: string; readonly phase: "reserved" | "pending" | "returned" | "cancelled"; readonly retained: boolean };
export type OwnedActorTurnOutputOutcome = { readonly kind: "returned" | "refused"; readonly value: unknown };
export type OwnedActorTurnOutputAdmission = { readonly step: ResidentStep; readonly output: OwnedActorTurnOutput | null };
type Slot = { owner: object | null; readonly capacity: number; readonly sequence: bigint; queue: OwnedActorTurnOutputs | null; handle: OwnedActorTurnOutput | null; phase: OwnedActorTurnOutputState["phase"]; response: object | null; outcome: OwnedActorTurnOutputOutcome | null; fault: unknown; previous: Slot | null; next: Slot | null; cell: OwnedResidentAdmission | null; record: OwnedResidentRecord | null };
type AdmissionPhase = "idle" | "preparing" | "cell-held" | "claiming" | "claimed" | "record-admitting" | "record-held" | "installing" | "installed" | "slot-held" | "facade-held" | "published";
const MINT = Object.freeze({});
const NO_OUTPUT_FAULT = Symbol("actor-output.no-fault");
const MAX_SEQUENCE = 0xffffffffffffffffn;
const OUTPUT_ENVELOPE = Object.freeze({ bytes: 448, slots: 3, owners: 3 });
let createOutput: (slot: Slot) => OwnedActorTurnOutput;
let cancelEmpty: (slot: Slot) => boolean;
let canRun: (slot: Slot) => boolean;
function granted(grant: ResidentGrant, bytes: number): boolean { return Number.isSafeInteger(grant.maxItems) && grant.maxItems >= 1 && Number.isSafeInteger(grant.maxBytes) && grant.maxBytes >= bytes; }
function admission(kind: ResidentStep["kind"], phase: string, bytes = 0, output: OwnedActorTurnOutput | null = null): OwnedActorTurnOutputAdmission { return { step: { kind, phase, items: bytes ? 1 : 0, bytes }, output }; }
function retainOutputFault(slot: Slot, error: unknown): void { if (slot.fault === NO_OUTPUT_FAULT) slot.fault = error; else if (!Object.is(slot.fault, error)) throw error; }
function settleOutput(slot: Slot, kind: OwnedActorTurnOutputOutcome["kind"], value: unknown): void {
  slot.outcome = { kind, value }; slot.phase = "returned";
  try { Object.freeze(slot.outcome); } catch (error) { retainOutputFault(slot, error); throw error; }
}

/** 📥️ One pre-admitted response slot retains success or failure before an external continuation runs. */
export class OwnedActorTurnOutput {
  readonly #slot: Slot;
  private constructor(mint: object, slot: Slot) { if (mint !== MINT) throw new Error("actor-output.private-mint"); this.#slot = slot; slot.handle = this; Object.freeze(this); }
  static { createOutput = slot => new OwnedActorTurnOutput(MINT, slot); }
  static matches(output: unknown, owner: object): output is OwnedActorTurnOutput { return output !== null && typeof output === "object" && #slot in output && output.#slot.owner === owner; }
  static reserved(output: unknown, owner: object): output is OwnedActorTurnOutput { return OwnedActorTurnOutput.matches(output, owner) && output.#slot.phase === "reserved" && output.#slot.fault === NO_OUTPUT_FAULT && canRun(output.#slot); }
  /** 🧯️ Compares the first retained raw fault without inspecting its arbitrary payload. */
  static matchesFault(output: unknown, fault: unknown): boolean { return output !== null && typeof output === "object" && #slot in output && output.#slot.fault !== NO_OUTPUT_FAULT && Object.is(output.#slot.fault, fault); }
  get state(): OwnedActorTurnOutputState { return Object.freeze({ capacity: this.#slot.capacity, sequence: this.#slot.sequence.toString(), phase: this.#slot.phase, retained: this.#slot.outcome !== null || this.#slot.response !== null || this.#slot.fault !== NO_OUTPUT_FAULT }); }
  /** 🧾️ This is the original mutable transport value, not an immutable or normalized content claim. */
  get outcome(): OwnedActorTurnOutputOutcome | null { return this.#slot.outcome; }
  /** 📨️ Preserves the exact response wrapper separately from normalized success or failure values. */
  get responseEnvelope(): object | null { return this.#slot.response; }
  captureResponse(response: object): boolean {
    const slot = this.#slot;
    if (slot.phase !== "pending" || slot.response !== null || response === null || typeof response !== "object") return false;
    slot.response = response; slot.phase = "returned";
    return true;
  }
  async run<T>(submit: () => Promise<T>): Promise<T> {
    const slot = this.#slot;
    if (slot.fault !== NO_OUTPUT_FAULT) throw new Error("actor-output.faulted");
    if (slot.phase !== "reserved") throw new Error("actor-output.already-submitted");
    if (!canRun(slot)) throw new Error("actor-output.closed");
    slot.phase = "pending";
    let value: T;
    try { value = await submit(); }
    catch (error) { settleOutput(slot, "refused", error); throw error; }
    settleOutput(slot, "returned", value); return value;
  }
  cancelEmpty(): boolean { return cancelEmpty(this.#slot); }
}

/** 🗃️ A bounded strong response roster; closing admission never discards returned roots. */
export class OwnedActorTurnOutputs {
  #owner: object | null;
  readonly #capacity: number;
  #sequence: bigint;
  #head: Slot | null = null;
  #tail: Slot | null = null;
  #pending = 0;
  #closed = false;
  #ledger: OwnedResidentLedger | null;
  #admissionCell: OwnedResidentAdmission | null = null;
  #admissionRecord: OwnedResidentRecord | null = null;
  #admissionPhase: AdmissionPhase = "idle";
  #admissionFault: unknown = NO_OUTPUT_FAULT;
  constructor(owner: object, capacity: number, ledger: OwnedResidentLedger, sequence = 0n) {
    if (!owner || typeof owner !== "object" || !Number.isSafeInteger(capacity) || capacity < 1 || capacity > 0xffffffff || !(ledger instanceof OwnedResidentLedger) || typeof sequence !== "bigint" || sequence < 0n || sequence > MAX_SEQUENCE) throw new Error("actor-output.invalid-admission");
    this.#owner = owner; this.#capacity = capacity; this.#sequence = sequence; this.#ledger = ledger;
  }
  static {
    canRun = slot => slot.queue !== null && !slot.queue.#closed && slot.queue.#admissionFault === NO_OUTPUT_FAULT && (slot.queue.#admissionCell !== slot.cell || slot.queue.#admissionPhase === "published") && slot.record?.matchesLiveShell(slot.queue) === true;
    cancelEmpty = slot => {
      const queue = slot.queue;
      if (!queue || slot.phase !== "reserved" || slot.outcome !== null || slot.fault !== NO_OUTPUT_FAULT) return false;
      slot.phase = "cancelled";
      return true;
    };
  }
  get pending(): number { return this.#pending; }
  peek(): OwnedActorTurnOutput | null { return this.#head?.handle ?? null; }
  reserve(grant: ResidentGrant): OwnedActorTurnOutputAdmission {
    if (!granted(grant, 64)) return admission("blocked", "actor-output.grant");
    if (this.#closed || !this.#owner || !this.#ledger) return admission("rejected", "actor-output.closed");
    try {
      const ledger = this.#ledger;
      if (this.#admissionPhase === "preparing") {
        const cell = ledger.preparedAdmission(this); if (!cell) return admission("rejected", "actor-output.missing-cell");
        this.#admissionCell = cell; this.#admissionPhase = "cell-held"; return admission("pending", "actor-output.cell-held", 64);
      }
      const cell = this.#admissionCell;
      if (this.#admissionPhase === "claiming" && cell?.claimed) { this.#admissionPhase = "claimed"; return admission("pending", "actor-output.claimed", 64); }
      if (this.#admissionPhase === "record-admitting") {
        const record = cell?.result?.record; if (!record) return admission("rejected", "actor-output.missing-record");
        this.#admissionRecord = record; this.#admissionPhase = "record-held"; return admission("pending", "actor-output.record-held", 64);
      }
      const record = this.#admissionRecord;
      if (this.#admissionPhase === "installing" && record?.matchesShell(this)) { this.#admissionPhase = "installed"; return admission("pending", "actor-output.installed", 64); }
      if (this.#admissionFault !== NO_OUTPUT_FAULT) {
        if (cell && !cell.hasFailure) { const current = cell.retainFailure(this.#admissionFault, grant); return { step: { ...current, kind: current.kind === "ready" ? "pending" : current.kind }, output: null }; }
        return admission("rejected", "actor-output.fault-held");
      }
      if (cell?.hasFailure || cell?.result?.step.kind === "rejected") return admission("rejected", "actor-output.admission-fault");
      if (this.#closed) return admission("rejected", "actor-output.closed");
      if (["installed", "slot-held", "facade-held", "published"].includes(this.#admissionPhase) && !record?.matchesLiveShell(this)) return admission("rejected", "actor-output.parent-not-live");
      if (this.#admissionPhase === "published") {
        const slot = this.#tail; if (slot?.phase === "reserved") return admission("ready", "actor-output.published", 0, slot.handle);
        this.#admissionCell = null; this.#admissionRecord = null; this.#admissionPhase = "idle"; return admission("pending", "actor-output.next-admission", 64);
      }
      if (this.#admissionPhase === "idle") {
        if (this.#pending >= this.#capacity || this.#sequence === MAX_SEQUENCE) return admission("blocked", "actor-output.capacity");
        if (!granted(grant, 296)) return admission("blocked", "actor-output.bootstrap");
        this.#admissionPhase = "preparing"; const current = ledger.prepareAdmission(this, "data", grant);
        if (current.kind === "blocked" || current.kind === "rejected" && current.bytes === 0) this.#admissionPhase = "idle";
        return { step: current, output: null };
      }
      if (this.#admissionPhase === "cell-held" && cell) {
        this.#admissionPhase = "claiming"; const current = ledger.claimAdmission(this, cell, grant);
        if (current.kind !== "ready") this.#admissionPhase = "cell-held";
        return { step: { ...current, kind: current.kind === "ready" ? "pending" : current.kind }, output: null };
      }
      if (this.#admissionPhase === "claimed" && cell) {
        if (!granted(grant, 264)) return admission("blocked", "actor-output.record");
        this.#admissionPhase = "record-admitting"; const result = ledger.reserveRecord("data", OUTPUT_ENVELOPE, cell, grant);
        if (result.step.kind === "blocked" || result.step.kind === "rejected" && result.step.bytes === 0) this.#admissionPhase = "claimed";
        return { step: { ...result.step, kind: result.step.kind === "ready" ? "pending" : result.step.kind }, output: null };
      }
      if (this.#admissionPhase === "record-held" && record) {
        this.#admissionPhase = "installing"; const current = record.install(this, grant);
        if (current.kind !== "ready") this.#admissionPhase = "record-held";
        return { step: { ...current, kind: current.kind === "ready" ? "pending" : current.kind }, output: null };
      }
      if (this.#admissionPhase === "installed" && cell && record) {
        if (!granted(grant, 272)) return admission("blocked", "actor-output.slot");
        const slot: Slot = { owner: this.#owner, capacity: this.#capacity, sequence: this.#sequence + 1n, queue: this, handle: null, phase: "reserved", response: null, outcome: null, fault: NO_OUTPUT_FAULT, previous: this.#tail, next: null, cell, record };
        if (this.#tail) this.#tail.next = slot; else this.#head = slot;
        this.#tail = slot; this.#sequence = slot.sequence; this.#pending++; this.#admissionPhase = "slot-held"; return admission("pending", "actor-output.slot-held", 272);
      }
      if (this.#admissionPhase === "slot-held" && this.#tail) {
        if (!granted(grant, 80)) return admission("blocked", "actor-output.facade");
        createOutput(this.#tail); this.#admissionPhase = "facade-held"; return admission("pending", "actor-output.facade-held", 80);
      }
      if (this.#admissionPhase === "facade-held" && this.#tail?.handle) { this.#admissionPhase = "published"; return admission("ready", "actor-output.published", 64, this.#tail.handle); }
      return admission("rejected", "actor-output.admission-phase");
    } catch (error) {
      if (this.#admissionFault !== NO_OUTPUT_FAULT && !Object.is(this.#admissionFault, error)) throw error;
      this.#admissionFault = error;
      if (this.#tail?.cell === this.#admissionCell) retainOutputFault(this.#tail, error);
      throw error;
    }
  }
  beginClose(): void { this.#closed = true; }
  /** 🚪️ Retires only unused reservations; returned, in-flight and faulted data require their original domain discharge. */
  closeStep(grant: ResidentGrant): ResidentStep {
    if (!granted(grant, 64)) return admission("blocked", "actor-output.close-grant").step;
    if (!this.#closed) return admission("rejected", "actor-output.not-closing").step;
    if (this.terminalIsEmpty()) return admission("complete", "actor-output.closed").step;
    if (this.#admissionFault !== NO_OUTPUT_FAULT) return admission("blocked", "actor-output.fault-held").step;
    const slot = this.#head;
    if (slot) {
      if (slot.phase === "pending" || slot.response !== null || slot.outcome !== null || slot.fault !== NO_OUTPUT_FAULT) return admission("blocked", "actor-output.domain-discharge-required").step;
      if (slot.phase === "reserved") { slot.phase = "cancelled"; return admission("pending", "actor-output.cancel-unused", 64).step; }
      if (slot.owner !== null || slot.queue !== null || slot.handle !== null) {
        if (!granted(grant, 128)) return admission("blocked", "actor-output.slot-detachment").step;
        slot.owner = null; slot.queue = null; slot.handle = null;
        return admission("pending", "actor-output.slot-detachment", 128).step;
      }
    }
    let cell = slot?.cell ?? this.#admissionCell;
    if (!cell && this.#admissionPhase === "preparing") {
      cell = this.#ledger?.preparedAdmission(this) ?? null;
      if (!cell) return admission("blocked", "actor-output.cell-handoff").step;
      this.#admissionCell = cell;
      return admission("pending", "actor-output.cell-observation", 64).step;
    }
    if (cell) {
      if (cell.hasFailure) return admission("blocked", "actor-output.admission-fault").step;
      const record = slot?.record ?? this.#admissionRecord ?? cell.result?.record ?? null;
      if (!slot && record !== null && this.#admissionRecord !== record) { this.#admissionRecord = record; return admission("pending", "actor-output.record-observation", 64).step; }
      if (record?.matchesShell(this)) { record.beginClose(); return record.detach(this, grant); }
      if (!cell.terminalIsEmpty()) {
        cell.beginClose();
        const current = cell.closeStep(grant);
        return { ...current, kind: current.kind === "complete" ? "pending" : current.kind };
      }
      if (!OwnedResidentRetirement.matches(cell.retirement, cell) || record && (!record.terminalIsEmpty() || !OwnedResidentRetirement.matches(record.retirement, record) || record.detachment !== null && !OwnedResidentRecordDetachment.matches(record.detachment, record, this))) return admission("blocked", "actor-output.retirement-proof").step;
      if (!granted(grant, 256)) return admission("blocked", "actor-output.unlink").step;
      if (this.#admissionCell === cell) { this.#admissionCell = null; this.#admissionRecord = null; this.#admissionPhase = "idle"; }
      if (slot) {
        this.#head = slot.next; if (this.#head) this.#head.previous = null; else this.#tail = null;
        slot.previous = null; slot.next = null; slot.cell = null; slot.record = null; this.#pending--;
      }
      return admission("pending", "actor-output.unlink", 256).step;
    }
    if (slot || this.#pending !== 0 || this.#tail !== null || this.#admissionRecord !== null || this.#admissionPhase !== "idle") return admission("blocked", "actor-output.retained-admission").step;
    this.#owner = null; this.#ledger = null;
    return admission("complete", "actor-output.closed", 64).step;
  }
  terminalIsEmpty(): boolean { return this.#closed && this.#owner === null && this.#ledger === null && this.#head === null && this.#tail === null && this.#pending === 0 && this.#admissionCell === null && this.#admissionRecord === null && this.#admissionPhase === "idle" && this.#admissionFault === NO_OUTPUT_FAULT; }
}
//#endregion 🧬️OutputReservation

//#region 🧪️OutputReservationTests
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️ownedactorturnoutput/🟦️.ts");
  await registerTests1(import.meta.vitest, { OwnedActorTurnOutput, OwnedActorTurnOutputs, OwnedResidentLedger, cancelEmpty }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️OutputReservationTests
