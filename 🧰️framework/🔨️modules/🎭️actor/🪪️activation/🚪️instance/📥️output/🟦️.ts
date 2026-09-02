import { OwnedResidentLedger, type OwnedResidentAdmission, type OwnedResidentRecord, type ResidentGrant, type ResidentStep } from "../../../../🌱️value/💾️resident/🟦️.ts";

//#region 🧬️OutputReservation
export type OwnedActorTurnOutputState = { readonly capacity: number; readonly sequence: string; readonly phase: "reserved" | "pending" | "returned" | "cancelled"; readonly retained: boolean };
export type OwnedActorTurnOutputOutcome = { readonly kind: "returned" | "refused"; readonly value: unknown };
export type OwnedActorTurnOutputAdmission = { readonly step: ResidentStep; readonly output: OwnedActorTurnOutput | null };
type Slot = { readonly owner: object; readonly capacity: number; readonly sequence: bigint; queue: OwnedActorTurnOutputs | null; handle: OwnedActorTurnOutput | null; phase: OwnedActorTurnOutputState["phase"]; response: object | null; outcome: OwnedActorTurnOutputOutcome | null; fault: unknown; previous: Slot | null; next: Slot | null; readonly cell: OwnedResidentAdmission; readonly record: OwnedResidentRecord };
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
  readonly #owner: object;
  readonly #capacity: number;
  #sequence: bigint;
  #head: Slot | null = null;
  #tail: Slot | null = null;
  #pending = 0;
  #closed = false;
  readonly #ledger: OwnedResidentLedger;
  #admissionCell: OwnedResidentAdmission | null = null;
  #admissionRecord: OwnedResidentRecord | null = null;
  #admissionPhase: AdmissionPhase = "idle";
  #admissionFault: unknown = NO_OUTPUT_FAULT;
  constructor(owner: object, capacity: number, ledger: OwnedResidentLedger, sequence = 0n) {
    if (!owner || typeof owner !== "object" || !Number.isSafeInteger(capacity) || capacity < 1 || capacity > 0xffffffff || !(ledger instanceof OwnedResidentLedger) || typeof sequence !== "bigint" || sequence < 0n || sequence > MAX_SEQUENCE) throw new Error("actor-output.invalid-admission");
    this.#owner = owner; this.#capacity = capacity; this.#sequence = sequence; this.#ledger = ledger;
  }
  static {
    canRun = slot => slot.queue !== null && !slot.queue.#closed && slot.queue.#admissionFault === NO_OUTPUT_FAULT && (slot.queue.#admissionCell !== slot.cell || slot.queue.#admissionPhase === "published") && slot.record.matchesLiveShell(slot.queue);
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
}
//#endregion 🧬️OutputReservation

//#region 🧪️OutputReservationTests
if (import.meta.vitest) {
  const { describe, expect, it, vi } = import.meta.vitest;
  const fixtureLedger = () => new OwnedResidentLedger({ bytes: 65536, slots: 256, owners: 256, control: { bytes: 0, slots: 0, owners: 0 } });
  async function fixtureOutput(queue: OwnedActorTurnOutputs): Promise<OwnedActorTurnOutput | null> {
    const { default: fixture } = await import("./🏘️admission/🧪️fixture.json");
    for (let turn = 0; turn < fixture.phases.length + 1; turn++) { const current = queue.reserve({ maxItems: 1, maxBytes: 4096 }); if (current.step.kind === "ready") return current.output; if (current.step.kind === "blocked" || current.step.kind === "rejected") return null; }
    throw new Error("Response admission exceeded declared transitions");
  }
  describe("OwnedActorTurnOutput", () => {
    it("ActorResponseAdmission declares conserved metadata and separate grants without receiver or refund authority", async () => {
      const { default: contract } = await import("./🏘️admission/🧬️contract.json"); const { default: schema } = await import("./🏘️admission/🧬️schema.json"); const { default: fixture } = await import("./🏘️admission/🧪️fixture.json"); const { default: fixtureSchema } = await import("./🏘️admission/🧪️schema.json"); const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
      const ajv = new Ajv({ strict: true }); expect(ajv.validate(schema, contract)).toBe(true); expect(ajv.validate(fixtureSchema, fixture)).toBe(true);
      const domain = [contract.slotFields, contract.facadeFields, contract.outcomeFields].reduce((total, fields) => produce(total, value => { value.bytes += contract.model.recordBytes + fields.length * contract.model.fieldBytes; value.slots++; value.owners++; }), { bytes: 0, slots: 0, owners: 0 }); expect(domain).toEqual(contract.domain);
      const retained = [domain, contract.intrinsicRecord, contract.admissionCell].reduce((total, charge) => produce(total, value => { value.bytes += charge.bytes; value.slots += charge.slots; value.owners += charge.owners; }), { bytes: 0, slots: 0, owners: 0 }); expect(retained).toEqual(contract.retained);
      expect(contract.model.recordBytes + contract.rosterFields.length * contract.model.fieldBytes).toBe(contract.parentRoster.bytes); expect(fixture.phases.map(row => row.phase)).toEqual(contract.phases); expect(fixture.phases.map(row => row.grant)).toEqual(contract.grants);
      let usage = { bytes: 0, slots: 0, owners: 0 };
      for (const [index, row] of fixture.phases.entries()) { if (index === 0) usage = produce(usage, value => Object.assign(value, contract.admissionCell)); if (index === 4) usage = produce(usage, value => Object.assign(value, retained)); expect(row.resident).toEqual(usage); expect(row.output).toBe(index === fixture.phases.length - 1); }
      expect(fixture.dispatch).toEqual({ beforeReady: false, withoutReceiver: false }); expect(fixture.retainedCancellation).toEqual({ unlink: false, refund: false, capacityRecovered: false });
    });

    it("ActorResponseAdmission binds its declared fields to the actual output source", async () => {
      const { default: contract } = await import("./🏘️admission/🧬️contract.json"); const ts = await import("typescript"); const { readFile } = await import("node:fs/promises"); const source = ts.createSourceFile("output.ts", await readFile(new URL("./🟦️.ts", import.meta.url), "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
      const fields = (name: string): string[] => { const declaration = source.statements.find(value => (ts.isClassDeclaration(value) || ts.isTypeAliasDeclaration(value)) && value.name?.text === name); if (!declaration) throw new Error(`Missing ${name}`); if (ts.isClassDeclaration(declaration)) return declaration.members.filter(ts.isPropertyDeclaration).map(value => value.name.getText(source).replace(/^#/, "")); if (ts.isTypeAliasDeclaration(declaration) && ts.isTypeLiteralNode(declaration.type)) return declaration.type.members.filter(ts.isPropertySignature).map(value => value.name.getText(source)); throw new Error(`Invalid ${name}`); };
      expect(fields("OwnedActorTurnOutputs")).toEqual(contract.rosterFields); expect(fields("Slot")).toEqual(contract.slotFields); expect(fields("OwnedActorTurnOutput")).toEqual(contract.facadeFields); expect(fields("OwnedActorTurnOutputOutcome")).toEqual(contract.outcomeFields);
    });

    it("retains the exact constructed shell before a finalizer can throw", async () => {
      const { default: fixture } = await import("./🧪️fixture.json"); const { produce } = await import("immer");
      for (const boundary of fixture.construction.faults) {
        const owner = {}; const queue = new OwnedActorTurnOutputs(owner, fixture.capacity, fixtureLedger()); const original = Object.freeze; const failure = new Error(boundary); const captured: OwnedActorTurnOutput[] = [];
        const finalizer = vi.spyOn(Object, "freeze").mockImplementation(value => {
          if (value instanceof OwnedActorTurnOutput) { captured.push(value); if (boundary === "after-finalize") original(value); throw failure; }
          return original(value);
        });
        try { await expect(fixtureOutput(queue)).rejects.toBe(failure); } finally { finalizer.mockRestore(); }
        expect(captured).toHaveLength(1); const output = captured[0]!;
        expect(queue.pending).toBe(fixture.construction.retainedSlotsAfterFault); expect(queue.peek()).toBe(output); expect(OwnedActorTurnOutput.matches(output, owner)).toBe(true);
        expect(Object.keys(output)).toEqual(fixture.construction.publicCapabilityKeys); const retained = produce(fixture.trace[0]!, state => { state.retained = true; }); expect(output.state).toEqual(retained);
        expect(await fixtureOutput(queue)).toBeNull(); expect(queue.peek()).toBe(output);
        expect(output.cancelEmpty()).toBe(fixture.construction.cancelUnused); expect(output.cancelEmpty()).toBe(fixture.construction.cancelReplay); expect(queue.peek()).toBe(output);
        expect(output.state).toEqual(retained); expect(queue.pending).toBe(1);
      }
    });
    it("ActorOutputFault retains exact constructor failures and rejects empty cancellation or dispatch", async () => {
      const { default: fixture } = await import("./🧯️fault/🧪️fixture.json"); const { default: schema } = await import("./🧯️fault/🧬️schema.json"); const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
      expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true); const matches = Reflect.get(OwnedActorTurnOutput, "matchesFault"); expect(typeof matches).toBe("function");
      for (const boundary of fixture.boundaries) for (const kind of fixture.values) {
        const queue = new OwnedActorTurnOutputs({}, 1, fixtureLedger()); let reads = 0; const fault = kind === "null" ? null : kind === "undefined" ? undefined : kind === "false" ? false : kind === "zero" ? 0 : { payload: new Uint8Array(fixture.unknownBytes), get message() { reads++; throw new Error("Foreign constructor fault getter"); } };
        const original = Object.freeze; const spy = vi.spyOn(Object, "freeze").mockImplementation(value => { if (value === queue.peek() && value !== null) { if (boundary === "after-finalize") original(value); throw fault; } return original(value); }); let threw = false;
        try { await fixtureOutput(queue); } catch (error) { threw = true; expect(Object.is(error, fault)).toBe(true); } finally { spy.mockRestore(); }
        const output = queue.peek(); if (!output) throw new Error("Original faulted shell lost"); expect(threw).toBe(true); expect(Reflect.apply(matches, OwnedActorTurnOutput, [output, fault])).toBe(fixture.constructor.originalFaultRetained);
        expect(output.state).toEqual(produce({ capacity: 1, sequence: "1", phase: "reserved", retained: false }, value => { value.retained = true; })); expect(output.cancelEmpty()).toBe(fixture.constructor.cancelEmpty);
        let called = false; await expect(output.run(async () => { called = true; return null; })).rejects.toThrow("actor-output.faulted"); expect(called).toBe(fixture.constructor.runAllowed); expect(queue.peek()).toBe(output); expect(queue.pending).toBe(fixture.constructor.retainedSlots); expect(reads).toBe(0);
      }
    });
    it("ActorOutputFault installs returned and refused outcomes before finalization without replacing either root", async () => {
      const { default: fixture } = await import("./🧯️fault/🧪️fixture.json"); const matches = Reflect.get(OwnedActorTurnOutput, "matchesFault");
      for (const outcome of fixture.outcomes) for (const boundary of fixture.boundaries) for (const kind of fixture.values) {
        const queue = new OwnedActorTurnOutputs({}, 1, fixtureLedger()); const output = (await fixtureOutput(queue))!; const raw = { payload: new Uint8Array(fixture.unknownBytes), outcome }; let reads = 0;
        const fault = kind === "null" ? null : kind === "undefined" ? undefined : kind === "false" ? false : kind === "zero" ? 0 : { payload: new Uint8Array(fixture.unknownBytes), get message() { reads++; throw new Error("Foreign outcome fault getter"); } };
        const original = Object.freeze; let observed = false; const spy = vi.spyOn(Object, "freeze").mockImplementation(value => { if (value !== null && typeof value === "object" && Object.getOwnPropertyDescriptor(value, "value")?.value === raw && Object.getOwnPropertyDescriptor(value, "kind")?.value === outcome) { observed = true; expect(output.outcome === value).toBe(true); if (boundary === "after-finalize") original(value); throw fault; } return original(value); });
        try { await expect(output.run(async () => { if (outcome === "refused") throw raw; return raw; })).rejects.toBe(fault); } finally { spy.mockRestore(); }
        expect(observed).toBe(fixture.outcome.installBeforeFinalizer); expect(output.outcome?.kind).toBe(outcome); expect(output.outcome?.value === raw).toBe(fixture.outcome.originalOutcomeRetained); expect(Reflect.apply(matches, OwnedActorTurnOutput, [output, fault])).toBe(fixture.outcome.originalFaultRetained);
        expect(output.cancelEmpty()).toBe(fixture.outcome.cancelEmpty); expect(queue.peek()).toBe(output); expect(queue.pending).toBe(1); expect(reads).toBe(0);
      }
    });
    it("pre-admits a strong exact output owner before dispatch and retains it across caller faults", async () => {
      const { default: fixture } = await import("./🧪️fixture.json");
      const { default: schema } = await import("./🧬️schema.json");
      const { default: lifetimeSchema } = await import("../../../🚪️lifetime/🧬️schema.json");
      const { default: Ajv } = await import("ajv");
      const { produce } = await import("immer");
      const validate = new Ajv({ strict: true }).addSchema(lifetimeSchema).compile(schema);
      const owner = Object.freeze({});
      const queue = new OwnedActorTurnOutputs(owner, fixture.capacity, fixtureLedger());
      const output = (await fixtureOutput(queue))!;
      const trace = [output.state];
      expect(OwnedActorTurnOutput.matches(output, owner)).toBe(true);
      expect(OwnedActorTurnOutput.matches(output, {})).toBe(false);
      let resolve!: (value: object) => void;
      const raw = { unknown: { marker: fixture.mutablePayload.before }, uiPatches: [{ ops: [new Uint8Array(8192)] }] };
      const pending = output.run(() => new Promise<object>(done => { resolve = done; }));
      trace.push(output.state);
      const observer = pending.then(() => { throw new Error("caller publication fault"); });
      const observed = expect(observer).rejects.toThrow("caller publication fault");
      resolve(raw); await observed; trace.push(output.state);
      expect(trace).toEqual(fixture.trace); expect(trace.every(state => validate(state))).toBe(true);
      const oracle = produce(fixture.trace[0]!, state => { state.phase = "returned"; state.retained = true; });
      expect(output.state).toEqual(oracle);
      expect(queue.peek()).toBe(output); expect(output.outcome).toEqual({ kind: "returned", value: raw });
      expect(output.outcome?.value).toBe(raw); expect(queue.pending).toBe(1);
      raw.unknown.marker = fixture.mutablePayload.after; expect((output.outcome?.value as typeof raw).unknown.marker).toBe(fixture.mutablePayload.after);
      queue.beginClose(); expect(await fixtureOutput(queue)).toBeNull(); expect(output.cancelEmpty()).toBe(false); expect(queue.peek()).toBe(output);
    });

    it("refuses full admission and sequence exhaustion without discarding retained outputs", async () => {
      const { default: fixture } = await import("./🧪️fixture.json");
      const ledger = fixtureLedger(); const queue = new OwnedActorTurnOutputs({}, fixture.capacity, ledger);
      const first = (await fixtureOutput(queue))!;
      const refused = new Error("post refused");
      await expect(first.run(() => { throw refused; })).rejects.toBe(refused);
      expect(first.outcome?.value).toBe(refused); expect(first.outcome?.kind).toBe("refused"); expect(first.cancelEmpty()).toBe(false);
      const second = (await fixtureOutput(queue))!; expect(second.state.sequence).toBe("2"); const retained = ledger.usage;
      expect(second.cancelEmpty()).toBe(true); expect(second.cancelEmpty()).toBe(false); expect(queue.pending).toBe(fixture.capacity); expect(await fixtureOutput(queue)).toBeNull(); expect(ledger.usage).toEqual(retained); expect(queue.peek()).toBe(first);
      let calls = 0; await expect(first.run(async () => { calls++; return {}; })).rejects.toThrow("actor-output.already-submitted"); expect(calls).toBe(0);
      const exhausted = new OwnedActorTurnOutputs({}, fixture.capacity, fixtureLedger(), BigInt(fixture.maximumSequence) - 1n);
      const last = (await fixtureOutput(exhausted))!; expect(last.state.sequence).toBe(fixture.maximumSequence); expect(last.cancelEmpty()).toBe(true); expect(await fixtureOutput(exhausted)).toBeNull(); expect(exhausted.peek()).toBe(last); expect(queue.peek()).toBe(first);
    });

    it("rejects fabricated reservations without consulting public getters", () => {
      let reads = 0;
      const forged = { get owner() { reads++; throw new Error("Unowned getter"); } };
      expect(OwnedActorTurnOutput.matches(forged, {})).toBe(false);
      expect(OwnedActorTurnOutput.matches(Object.create(OwnedActorTurnOutput.prototype), {})).toBe(false);
      expect(() => Reflect.construct(OwnedActorTurnOutput, [forged])).toThrow("actor-output.private-mint"); expect(reads).toBe(0);
    });
    it("captures the original response envelope before settlement or failure extraction can throw", async () => {
      const { default: fixture } = await import("./🧪️fixture.json");
      const { default: schema } = await import("./🧬️schema.json");
      const { default: lifetimeSchema } = await import("../../../🚪️lifetime/🧬️schema.json");
      const { default: Ajv } = await import("ajv");
      const { produce } = await import("immer");
      const validate = new Ajv({ strict: true }).addSchema(lifetimeSchema).compile(schema);
      for (const kind of fixture.responseSettlement.outcomes) {
        const owner = Object.freeze({}); const queue = new OwnedActorTurnOutputs(owner, fixture.capacity, fixtureLedger()); const output = (await fixtureOutput(queue))!;
        const raw = { kind: "result", ok: kind === "success", value: { uiPatches: [] }, framesBytes: new Uint8Array(fixture.responseSettlement.unknownPayloadBytes), unknown: { retained: true } };
        const observed: string[] = [];
        const pending = output.run(async () => {
          expect(output.captureResponse(raw)).toBe(true); observed.push(fixture.responseSettlement.phases[0]!);
          expect(output.responseEnvelope).toBe(raw); expect(output.state.phase).toBe("returned"); expect(validate(output.state)).toBe(true);
          expect(output.captureResponse({ ...raw })).toBe(fixture.responseSettlement.replaceCapturedResponse);
          for (const phase of fixture.responseSettlement.phases.slice(1)) {
            observed.push(phase); expect(queue.peek()?.responseEnvelope).toBe(raw);
            if (kind === "graft-fault" && phase === "error-graft") throw new Error("graft fixture fault");
          }
          return raw.value;
        });
        if (kind === "graft-fault") await expect(pending).rejects.toThrow("graft fixture fault");
        else if (kind === "callback-fault") await expect(pending.then(() => { throw new Error("caller fixture fault"); })).rejects.toThrow("caller fixture fault");
        else expect(await pending).toBe(raw.value);
        expect(output.responseEnvelope).toBe(raw); expect(queue.peek()).toBe(output); expect(raw.framesBytes.byteLength).toBe(fixture.responseSettlement.unknownPayloadBytes);
        expect(observed).toEqual(kind === "graft-fault" ? fixture.responseSettlement.phases.slice(0, 4) : fixture.responseSettlement.phases);
        expect(output.state).toEqual(produce(fixture.trace[0]!, state => { state.phase = "returned"; state.retained = true; }));
        expect(output.cancelEmpty()).toBe(false); queue.beginClose(); expect(output.responseEnvelope).toBe(raw);
      }
    });
  });
}
//#endregion 🧪️OutputReservationTests
