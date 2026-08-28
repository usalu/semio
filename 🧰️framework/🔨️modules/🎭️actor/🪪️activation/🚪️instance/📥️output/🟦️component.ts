//#region 🧬️OutputReservation
export type OwnedActorTurnOutputState = { readonly capacity: number; readonly sequence: string; readonly phase: "reserved" | "pending" | "returned" | "cancelled"; readonly retained: boolean };
export type OwnedActorTurnOutputOutcome = { readonly kind: "returned" | "refused"; readonly value: unknown };
type Slot = { readonly owner: object; readonly capacity: number; readonly sequence: bigint; queue: OwnedActorTurnOutputs | null; handle: OwnedActorTurnOutput | null; phase: OwnedActorTurnOutputState["phase"]; response: object | null; outcome: OwnedActorTurnOutputOutcome | null; previous: Slot | null; next: Slot | null };
const MINT = Object.freeze({});
const MAX_SEQUENCE = 0xffffffffffffffffn;
let createOutput: (slot: Slot) => OwnedActorTurnOutput;
let cancelEmpty: (slot: Slot) => boolean;
let canRun: (slot: Slot) => boolean;

/** 📥️ One pre-admitted response slot retains success or failure before an external continuation runs. */
export class OwnedActorTurnOutput {
  readonly #slot: Slot;
  private constructor(mint: object, slot: Slot) { if (mint !== MINT) throw new Error("actor-output.private-mint"); this.#slot = slot; slot.handle = this; Object.freeze(this); }
  static { createOutput = slot => new OwnedActorTurnOutput(MINT, slot); }
  static matches(output: unknown, owner: object): output is OwnedActorTurnOutput { return output !== null && typeof output === "object" && #slot in output && output.#slot.owner === owner; }
  get state(): OwnedActorTurnOutputState { return Object.freeze({ capacity: this.#slot.capacity, sequence: this.#slot.sequence.toString(), phase: this.#slot.phase, retained: this.#slot.outcome !== null || this.#slot.response !== null }); }
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
    if (slot.phase !== "reserved") throw new Error("actor-output.already-submitted");
    if (!canRun(slot)) throw new Error("actor-output.closed");
    slot.phase = "pending";
    try {
      const value = await submit();
      slot.outcome = Object.freeze({ kind: "returned", value }); slot.phase = "returned";
      return value;
    } catch (value) {
      slot.outcome = Object.freeze({ kind: "refused", value }); slot.phase = "returned";
      throw value;
    }
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
  constructor(owner: object, capacity: number, sequence = 0n) {
    if (!owner || typeof owner !== "object" || !Number.isSafeInteger(capacity) || capacity < 1 || capacity > 0xffffffff || typeof sequence !== "bigint" || sequence < 0n || sequence > MAX_SEQUENCE) throw new Error("actor-output.invalid-admission");
    this.#owner = owner; this.#capacity = capacity; this.#sequence = sequence; Object.freeze(this);
  }
  static {
    canRun = slot => slot.queue !== null && !slot.queue.#closed;
    cancelEmpty = slot => {
      const queue = slot.queue;
      if (!queue || slot.phase !== "reserved" || slot.outcome !== null) return false;
      if (slot.previous) slot.previous.next = slot.next; else queue.#head = slot.next;
      if (slot.next) slot.next.previous = slot.previous; else queue.#tail = slot.previous;
      queue.#pending--; slot.previous = null; slot.next = null; slot.queue = null; slot.phase = "cancelled";
      return true;
    };
  }
  get pending(): number { return this.#pending; }
  peek(): OwnedActorTurnOutput | null { return this.#head?.handle ?? null; }
  reserve(): OwnedActorTurnOutput | null {
    if (this.#closed || this.#pending >= this.#capacity || this.#sequence === MAX_SEQUENCE) return null;
    const slot: Slot = { owner: this.#owner, capacity: this.#capacity, sequence: this.#sequence + 1n, queue: this, handle: null, phase: "reserved", response: null, outcome: null, previous: this.#tail, next: null };
    if (this.#tail) this.#tail.next = slot; else this.#head = slot;
    this.#tail = slot; this.#sequence = slot.sequence; this.#pending++;
    return createOutput(slot);
  }
  beginClose(): void { this.#closed = true; }
}
//#endregion 🧬️OutputReservation

//#region 🧪️OutputReservationTests
if (import.meta.vitest) {
  const { describe, expect, it, vi } = import.meta.vitest;
  describe("OwnedActorTurnOutput", () => {
    it("retains the exact constructed shell before a finalizer can throw", async () => {
      const { default: fixture } = await import("./🧪️fixture.json"); const { produce } = await import("immer");
      for (const boundary of fixture.construction.faults) {
        const owner = {}; const queue = new OwnedActorTurnOutputs(owner, fixture.capacity); const original = Object.freeze; const failure = new Error(boundary); const captured: OwnedActorTurnOutput[] = [];
        const finalizer = vi.spyOn(Object, "freeze").mockImplementation(value => {
          if (value instanceof OwnedActorTurnOutput) { captured.push(value); if (boundary === "after-finalize") original(value); throw failure; }
          return original(value);
        });
        try { expect(() => queue.reserve()).toThrow(failure); } finally { finalizer.mockRestore(); }
        expect(captured).toHaveLength(1); const output = captured[0]!;
        expect(queue.pending).toBe(fixture.construction.retainedSlotsAfterFault); expect(queue.peek()).toBe(output); expect(OwnedActorTurnOutput.matches(output, owner)).toBe(true);
        expect(Object.keys(output)).toEqual(fixture.construction.publicCapabilityKeys); expect(output.state).toEqual(fixture.trace[0]);
        const second = queue.reserve()!; expect(second.state.sequence).toBe(fixture.construction.nextSequence); expect(queue.reserve()).toBeNull(); expect(queue.peek()).toBe(output);
        expect(output.cancelEmpty()).toBe(fixture.construction.cancelUnused); expect(output.cancelEmpty()).toBe(fixture.construction.cancelReplay); expect(queue.peek()).toBe(second);
        expect(output.state).toEqual(produce(fixture.trace[0]!, state => { state.phase = "cancelled"; })); expect(second.cancelEmpty()).toBe(true); expect(queue.pending).toBe(0);
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
      const queue = new OwnedActorTurnOutputs(owner, fixture.capacity);
      const output = queue.reserve()!;
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
      queue.beginClose(); expect(queue.reserve()).toBeNull(); expect(output.cancelEmpty()).toBe(false); expect(queue.peek()).toBe(output);
    });

    it("refuses full admission and sequence exhaustion without discarding retained outputs", async () => {
      const { default: fixture } = await import("./🧪️fixture.json");
      const queue = new OwnedActorTurnOutputs({}, fixture.capacity);
      const first = queue.reserve()!; const second = queue.reserve()!;
      expect(queue.reserve()).toBeNull(); expect(queue.pending).toBe(fixture.capacity);
      expect(second.cancelEmpty()).toBe(true); expect(second.cancelEmpty()).toBe(false); expect(queue.pending).toBe(1);
      const replacement = queue.reserve()!; expect(replacement.state.sequence).toBe("3"); expect(queue.peek()).toBe(first);
      const refused = new Error("post refused");
      await expect(first.run(() => { throw refused; })).rejects.toBe(refused);
      expect(first.outcome?.value).toBe(refused); expect(first.outcome?.kind).toBe("refused"); expect(first.cancelEmpty()).toBe(false);
      let calls = 0; await expect(first.run(async () => { calls++; return {}; })).rejects.toThrow("actor-output.already-submitted"); expect(calls).toBe(0);
      const exhausted = new OwnedActorTurnOutputs({}, fixture.capacity, BigInt(fixture.maximumSequence) - 1n);
      const last = exhausted.reserve()!; expect(last.state.sequence).toBe(fixture.maximumSequence); expect(exhausted.reserve()).toBeNull(); expect(exhausted.peek()).toBe(last);
      expect(replacement.cancelEmpty()).toBe(true); expect(queue.peek()).toBe(first);
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
        const owner = Object.freeze({}); const queue = new OwnedActorTurnOutputs(owner, fixture.capacity); const output = queue.reserve()!;
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
