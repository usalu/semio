/** 🎯️ The Input Causality Ledger as laws over the pure module alone — no React, no shell, no fixture; the
 * same separation `🔀️surface-switch` and `🎚️continuous-gesture-lane` keep for their own units.
 *
 * 🏁️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER, design §1/§2 A/E/D. Four laws, stated so they can fail:
 *   L1 admit or refuse, never drop — `issue` mints a monotonic `inputSeq`, `settle` closes an entry
 *      exactly once (first outcome wins), `settled()` answers immediately for a closed entry, later for
 *      an open one, and a typed `dispatch-failed` refusal for an entry the ledger never heard of — never
 *      `undefined`;
 *   L2 causal closure — `compareCausalV1` sorts a guest follow-up (`causedBy = N`) after input `N` itself
 *      and before every later root input, a total order stable under any shuffle of the same set;
 *   L3 versioned registers — `createVersionedRegisterV1` is a CAS cell: a stale `expectedGeneration` is
 *      refused and leaves the register untouched, `null` is the unconditional write, and
 *      `resolveUtilityActivationV1` is the toggle-off/toggle-on truth table the picker interceptor uses;
 *   L4 batched samples — `createGestureSampleLaneV1` never drops an offered sample, keeps at most one
 *      send in flight, and never lets a later gesture's `begin` overtake an earlier `end` nor an `end`
 *      overtake samples still owed ahead of it.
 * Refusal surfacing (design §G) rides along: every refusal reason has a retryability, a plain console
 * line, a notify decision (guest/tick never toast) and bilingual notice text, throttled per reason. */

import { describe, expect, it } from "vitest";
import {
  causalOrderKeyV1,
  compareCausalV1,
  createGestureSampleLaneV1,
  createInputLedgerV1,
  createRefusalNoticeThrottleV1,
  createVersionedRegisterV1,
  expectedGenerationFromArgsV1,
  inputAppliedV1,
  inputRefusalNoticeTextV1,
  inputRefusalNotifiesV1,
  inputRefusalTextV1,
  inputRefusedV1,
  inputSupersededV1,
  resolveUtilityActivationV1,
  INPUT_REFUSAL_LABELS_V1,
  INPUT_REFUSAL_NOTIFIED_V1,
  INPUT_REFUSAL_RETRYABLE_V1,
  type GestureSendV1,
  type InputOutcomeV1,
  type InputProvenanceV1,
  type InputRefusalReasonV1,
} from "../../🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts";

const REFUSAL_REASONS = Object.keys(INPUT_REFUSAL_RETRYABLE_V1) as InputRefusalReasonV1[];

//#region 📒️L1 ledger
describe("input ledger", () => {
  it("mints a monotonic inputSeq and defaults provenance to a user-origin, windowless, causeless input", () => {
    const ledger = createInputLedgerV1();
    const first = ledger.issue({ controllerId: "c", action: "a" });
    const second = ledger.issue({ controllerId: "c", action: "a" });
    expect(first.provenance).toEqual({ windowId: null, inputSeq: 1, causedBy: null, origin: "user" });
    expect(second.provenance.inputSeq).toBe(2);
  });

  it("preserves given provenance except inputSeq, which the ledger always mints fresh", () => {
    const ledger = createInputLedgerV1();
    const given: InputProvenanceV1 = { windowId: "w1", inputSeq: 9999, causedBy: 5, origin: "guest" };
    const entry = ledger.issue({ controllerId: "c", action: "a", provenance: given });
    expect(entry.provenance).toEqual({ windowId: "w1", inputSeq: 1, causedBy: 5, origin: "guest" });
  });

  it("falls back to caller-supplied defaults only where the descriptor carries no provenance field", () => {
    const ledger = createInputLedgerV1();
    const entry = ledger.issue({ controllerId: "c", action: "a" }, { windowId: "w9", origin: "tick", causedBy: 3 });
    expect(entry.provenance).toEqual({ windowId: "w9", inputSeq: 1, causedBy: 3, origin: "tick" });
  });

  it("settles an entry exactly once — a second settle is refused and the first outcome wins", () => {
    const ledger = createInputLedgerV1();
    const entry = ledger.issue({ controllerId: "c", action: "a" });
    const seq = entry.provenance.inputSeq;
    expect(ledger.settle(inputAppliedV1(seq))).toBe(true);
    expect(ledger.outcome(seq)).toEqual({ kind: "applied", inputSeq: seq });
    expect(ledger.settle(inputSupersededV1(seq, seq + 1))).toBe(false);
    expect(ledger.outcome(seq)).toEqual({ kind: "applied", inputSeq: seq });
  });

  it("settle() on an inputSeq the ledger never issued returns false without side effects", () => {
    const ledger = createInputLedgerV1();
    expect(ledger.settle(inputAppliedV1(4242))).toBe(false);
    expect(ledger.outcome(4242)).toBeNull();
  });

  it("settled() resolves immediately for a closed entry and only after settle() for an open one", async () => {
    const ledger = createInputLedgerV1();
    const closedEntry = ledger.issue({ controllerId: "c", action: "a" });
    const closedSeq = closedEntry.provenance.inputSeq;
    ledger.settle(inputAppliedV1(closedSeq));
    await expect(ledger.settled(closedSeq)).resolves.toEqual({ kind: "applied", inputSeq: closedSeq });

    const openEntry = ledger.issue({ controllerId: "c", action: "a" });
    const openSeq = openEntry.provenance.inputSeq;
    let resolved = false;
    const waiting = ledger.settled(openSeq).then((outcome) => {
      resolved = true;
      return outcome;
    });
    await Promise.resolve();
    await Promise.resolve();
    expect(resolved).toBe(false);
    ledger.settle(inputAppliedV1(openSeq));
    await expect(waiting).resolves.toEqual({ kind: "applied", inputSeq: openSeq });
    expect(resolved).toBe(true);
  });

  it("settled() on an unknown inputSeq resolves to a dispatch-failed refusal, never undefined", async () => {
    const ledger = createInputLedgerV1();
    await expect(ledger.settled(777)).resolves.toEqual({
      kind: "refused",
      inputSeq: 777,
      reason: "dispatch-failed",
      retryable: true,
      detail: "unknown input",
    });
  });

  it("caps concurrent settled() waiters — the waiter past waiterSlots resolves immediately as dispatch-failed", async () => {
    const ledger = createInputLedgerV1({ waiterSlots: 2 });
    const entry = ledger.issue({ controllerId: "c", action: "a" });
    const seq = entry.provenance.inputSeq;
    const first = ledger.settled(seq);
    const second = ledger.settled(seq);
    const third = ledger.settled(seq);
    await expect(third).resolves.toEqual({
      kind: "refused",
      inputSeq: seq,
      reason: "dispatch-failed",
      retryable: true,
      detail: "waiter slots exhausted",
    });
    ledger.settle(inputAppliedV1(seq));
    await expect(first).resolves.toEqual({ kind: "applied", inputSeq: seq });
    await expect(second).resolves.toEqual({ kind: "applied", inputSeq: seq });
  });

  it("census() counts issued, applied, superseded, refused-per-reason and pending", () => {
    const ledger = createInputLedgerV1();
    const a = ledger.issue({ controllerId: "c", action: "a" });
    const b = ledger.issue({ controllerId: "c", action: "a" });
    const c = ledger.issue({ controllerId: "c", action: "a" });
    ledger.issue({ controllerId: "c", action: "a" }); // left pending
    ledger.settle(inputAppliedV1(a.provenance.inputSeq));
    ledger.settle(inputSupersededV1(b.provenance.inputSeq, c.provenance.inputSeq));
    ledger.settle(inputRefusedV1(c.provenance.inputSeq, "queue-full"));
    const census = ledger.census();
    expect(census.issued).toBe(4);
    expect(census.applied).toBe(1);
    expect(census.superseded).toBe(1);
    expect(census.refused["queue-full"]).toBe(1);
    expect(census.refused["stale-generation"]).toBe(0);
    expect(census.pending).toBe(1);
    expect(ledger.pending()).toBe(1);
  });

  it("forgets the oldest closed outcome once history exceeds historySlots", () => {
    const ledger = createInputLedgerV1({ historySlots: 2 });
    const seqs: number[] = [];
    for (let round = 0; round < 3; round += 1) {
      const entry = ledger.issue({ controllerId: "c", action: "a" });
      seqs.push(entry.provenance.inputSeq);
      ledger.settle(inputAppliedV1(entry.provenance.inputSeq));
    }
    expect(ledger.outcome(seqs[0])).toBeNull();
    expect(ledger.outcome(seqs[1])).not.toBeNull();
    expect(ledger.outcome(seqs[2])).not.toBeNull();
  });
});
//#endregion 📒️L1 ledger

//#region 🔗️L2 causal order
describe("causal order", () => {
  it("causalOrderKeyV1 is causedBy when present, else inputSeq", () => {
    expect(causalOrderKeyV1({ inputSeq: 5, causedBy: null })).toBe(5);
    expect(causalOrderKeyV1({ inputSeq: 5, causedBy: 2 })).toBe(2);
  });

  it("compareCausalV1 sorts a follow-up right after its cause and before any later root input", () => {
    const cause = { inputSeq: 10, causedBy: null };
    const followUp = { inputSeq: 15, causedBy: 10 };
    const laterRoot = { inputSeq: 11, causedBy: null };
    expect(compareCausalV1(cause, followUp)).toBeLessThan(0);
    expect(compareCausalV1(followUp, laterRoot)).toBeLessThan(0);
    expect([laterRoot, followUp, cause].sort(compareCausalV1).map((item) => item.inputSeq)).toEqual([10, 15, 11]);
  });

  it("sorts a shuffled array deterministically, every follow-up landing after its cause and before any later root (a few hundred random arrays)", () => {
    const rand = mulberry32(0x494e5055); // "INPU"
    for (let trial = 0; trial < 300; trial += 1) {
      const items = buildCausalItems(rand, 10 + (trial % 12));
      const canonical = [...items].sort(compareCausalV1).map((item) => item.inputSeq);
      for (let shuffle = 0; shuffle < 4; shuffle += 1) {
        const sorted = shuffleWith(items, rand).sort(compareCausalV1).map((item) => item.inputSeq);
        expect(sorted).toEqual(canonical);
      }
      const positionOf = new Map(canonical.map((seq, index) => [seq, index]));
      for (const item of items) {
        if (item.causedBy === null) continue;
        expect(positionOf.get(item.inputSeq)!).toBeGreaterThan(positionOf.get(item.causedBy)!);
        for (const other of items) {
          if (other.causedBy === null && other.inputSeq > item.causedBy) {
            expect(positionOf.get(item.inputSeq)!).toBeLessThan(positionOf.get(other.inputSeq)!);
          }
        }
      }
    }
  });
});

/** 🎲️ A tiny deterministic PRNG so the property test above is reproducible across runs. */
function mulberry32(seed: number): () => number {
  let state = seed | 0;
  return () => {
    state = (state + 0x6d2b79f5) | 0;
    let t = Math.imul(state ^ (state >>> 15), 1 | state);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

/** 🎲️ A random causal set: each input is a root, or (with roots already minted) a follow-up of a
 * randomly chosen earlier root — the shape `causedBy` is defined over in the design. */
function buildCausalItems(rand: () => number, count: number): { inputSeq: number; causedBy: number | null }[] {
  const roots: number[] = [];
  const items: { inputSeq: number; causedBy: number | null }[] = [];
  for (let seq = 1; seq <= count; seq += 1) {
    const causedBy = roots.length > 0 && rand() < 0.4 ? roots[Math.floor(rand() * roots.length)] : null;
    items.push({ inputSeq: seq, causedBy });
    if (causedBy === null) roots.push(seq);
  }
  return items;
}

function shuffleWith<T>(items: readonly T[], rand: () => number): T[] {
  const copy = items.slice();
  for (let index = copy.length - 1; index > 0; index -= 1) {
    const swapWith = Math.floor(rand() * (index + 1));
    [copy[index], copy[swapWith]] = [copy[swapWith], copy[index]];
  }
  return copy;
}
//#endregion 🔗️L2 causal order

//#region 🔢️L3 versioned register
describe("versioned register", () => {
  it("applies a write whose expectedGeneration matches, and bumps the generation", () => {
    const cell = createVersionedRegisterV1<string | null>(null);
    expect(cell.read()).toEqual({ value: null, generation: 0 });
    const result = cell.write("a", 0);
    expect(result).toEqual({ kind: "applied", register: { value: "a", generation: 1 }, changed: true });
    expect(cell.read()).toEqual({ value: "a", generation: 1 });
  });

  it("refuses a stale expectedGeneration with stale-generation and leaves the register untouched", () => {
    const cell = createVersionedRegisterV1("a");
    cell.write("b", 0);
    const before = cell.read();
    const result = cell.write("c", 0);
    expect(result).toEqual({ kind: "refused", reason: "stale-generation", register: before, expected: 0 });
    expect(cell.read()).toEqual(before);
  });

  it("treats a null expectedGeneration as the unconditional write", () => {
    const cell = createVersionedRegisterV1("a");
    cell.write("b", 0);
    const result = cell.write("z", null);
    expect(result.kind).toBe("applied");
    expect(cell.read()).toEqual({ value: "z", generation: 2 });
  });

  it("reports changed:false when the written value equals the current one under Object.is", () => {
    const cell = createVersionedRegisterV1("a");
    const result = cell.write("a", 0);
    expect(result).toEqual({ kind: "applied", register: { value: "a", generation: 0 }, changed: false });
  });

  it("respects a custom equals function instead of Object.is", () => {
    const cell = createVersionedRegisterV1({ id: 1 }, (a, b) => a.id === b.id);
    const result = cell.write({ id: 1 }, 0);
    expect(result.kind).toBe("applied");
    expect(result.kind === "applied" && result.changed).toBe(false);
  });

  it("resolveUtilityActivationV1 matches the truth table: empty request or re-request of the active one deactivates, else activates", () => {
    expect(resolveUtilityActivationV1(null, "")).toBeNull();
    expect(resolveUtilityActivationV1("x", "")).toBeNull();
    expect(resolveUtilityActivationV1("x", "x")).toBeNull();
    expect(resolveUtilityActivationV1(undefined, "x")).toBe("x");
    expect(resolveUtilityActivationV1(null, "y")).toBe("y");
    expect(resolveUtilityActivationV1("x", "y")).toBe("y");
  });

  it("expectedGenerationFromArgsV1 accepts only finite, non-negative, safe integers", () => {
    expect(expectedGenerationFromArgsV1({ expectedGeneration: 3 })).toBe(3);
    expect(expectedGenerationFromArgsV1({ expectedGeneration: 0 })).toBe(0);
    expect(expectedGenerationFromArgsV1({ expectedGeneration: -1 })).toBeNull();
    expect(expectedGenerationFromArgsV1({ expectedGeneration: 1.5 })).toBeNull();
    expect(expectedGenerationFromArgsV1({ expectedGeneration: Number.NaN })).toBeNull();
    expect(expectedGenerationFromArgsV1({ expectedGeneration: Number.POSITIVE_INFINITY })).toBeNull();
    expect(expectedGenerationFromArgsV1({ expectedGeneration: "3" })).toBeNull();
    expect(expectedGenerationFromArgsV1({})).toBeNull();
    expect(expectedGenerationFromArgsV1(null)).toBeNull();
    expect(expectedGenerationFromArgsV1(undefined)).toBeNull();
    expect(expectedGenerationFromArgsV1("not an object")).toBeNull();
  });
});
//#endregion 🔢️L3 versioned register

//#region 🔔️Refusal surfacing
describe("refusal surfacing", () => {
  it("inputRefusedV1 fills retryable from INPUT_REFUSAL_RETRYABLE_V1 for every reason", () => {
    for (const reason of REFUSAL_REASONS) {
      expect(inputRefusedV1(7, reason)).toEqual({ kind: "refused", inputSeq: 7, reason, retryable: INPUT_REFUSAL_RETRYABLE_V1[reason] });
    }
  });

  it("carries an optional detail without disturbing retryable", () => {
    expect(inputRefusedV1(7, "queue-full", "backlog")).toEqual({
      kind: "refused",
      inputSeq: 7,
      reason: "queue-full",
      retryable: true,
      detail: "backlog",
    });
  });

  it("inputRefusalTextV1 names the seq, action, reason, origin, window, cause and detail", () => {
    const provenance: InputProvenanceV1 = { windowId: "w7", inputSeq: 12, causedBy: 5, origin: "guest" };
    const outcome = inputRefusedV1(12, "stale-generation", "observed generation 2") as Extract<InputOutcomeV1, { kind: "refused" }>;
    const text = inputRefusalTextV1("setActiveUtility", outcome, provenance);
    for (const fragment of ["#12", "setActiveUtility", "stale-generation", "guest", "window=w7", "causedBy=#5", "observed generation 2"]) {
      expect(text).toContain(fragment);
    }
  });

  it("never notifies a guest- or tick-origin refusal, whatever the reason", () => {
    for (const origin of ["guest", "tick"] as const) {
      for (const reason of REFUSAL_REASONS) {
        const outcome = inputRefusedV1(1, reason) as Extract<InputOutcomeV1, { kind: "refused" }>;
        const provenance: InputProvenanceV1 = { windowId: null, inputSeq: 1, causedBy: null, origin };
        expect(inputRefusalNotifiesV1(outcome, provenance)).toBe(false);
      }
    }
  });

  it("follows INPUT_REFUSAL_NOTIFIED_V1 for a user-origin refusal", () => {
    for (const reason of REFUSAL_REASONS) {
      const outcome = inputRefusedV1(1, reason) as Extract<InputOutcomeV1, { kind: "refused" }>;
      const provenance: InputProvenanceV1 = { windowId: null, inputSeq: 1, causedBy: null, origin: "user" };
      expect(inputRefusalNotifiesV1(outcome, provenance)).toBe(INPUT_REFUSAL_NOTIFIED_V1[reason]);
    }
  });

  it("inputRefusalNoticeTextV1 answers de for locale \"de\" and en for anything else", () => {
    expect(inputRefusalNoticeTextV1("queue-full", "de")).toBe(INPUT_REFUSAL_LABELS_V1["queue-full"].de);
    for (const locale of ["en", "fr", "", "DE"]) {
      expect(inputRefusalNoticeTextV1("queue-full", locale)).toBe(INPUT_REFUSAL_LABELS_V1["queue-full"].en);
    }
  });

  it("gives every refusal reason a non-empty English and German label", () => {
    for (const reason of REFUSAL_REASONS) {
      expect(INPUT_REFUSAL_LABELS_V1[reason].en.length).toBeGreaterThan(0);
      expect(INPUT_REFUSAL_LABELS_V1[reason].de.length).toBeGreaterThan(0);
    }
  });

  it("createRefusalNoticeThrottleV1 admits once per window per reason, independently per reason", () => {
    const throttle = createRefusalNoticeThrottleV1(1000);
    expect(throttle.admit("queue-full", 0)).toBe(true);
    expect(throttle.admit("queue-full", 500)).toBe(false);
    expect(throttle.admit("queue-full", 999)).toBe(false);
    expect(throttle.admit("queue-full", 1000)).toBe(true);
    expect(throttle.admit("stale-generation", 500)).toBe(true);
  });
});
//#endregion 🔔️Refusal surfacing

//#region 🖱️L4 gesture sample lane
/** 🔌️ A receiver whose round trips are resolved by the test one at a time — copied from
 * `🎚️continuous-gesture-lane`'s `manualReceiver`, adapted to the lane's discriminated send item. */
function manualLaneReceiver<Sample, Extra>() {
  const sent: GestureSendV1<Sample, Extra>[] = [];
  const pending: { resolve: () => void; reject: (error: unknown) => void }[] = [];
  return {
    sent,
    pending,
    send(item: GestureSendV1<Sample, Extra>) {
      sent.push(item);
      return new Promise<void>((resolve, reject) => pending.push({ resolve, reject }));
    },
    async settleOne() {
      const next = pending.shift();
      next?.resolve();
      await Promise.resolve();
      await Promise.resolve();
    },
    async failOne(error: unknown) {
      const next = pending.shift();
      next?.reject(error);
      await Promise.resolve();
      await Promise.resolve();
    },
    async drain() {
      while (pending.length > 0) {
        const next = pending.shift();
        next?.resolve();
        await Promise.resolve();
        await Promise.resolve();
      }
    },
  };
}

describe("gesture sample lane", () => {
  it("merges every offer made while a send is in flight into exactly one live send, samples in order", async () => {
    const receiver = manualLaneReceiver<number, void>();
    const lane = createGestureSampleLaneV1<number, void>({ send: receiver.send.bind(receiver) });
    lane.offer(1);
    for (let sample = 2; sample <= 10; sample += 1) lane.offer(sample);
    expect(receiver.sent).toEqual([{ phase: "live", gestureId: null, samples: [1] }]);
    expect(lane.owed()).toBe(9);
    await receiver.settleOne();
    expect(receiver.sent).toEqual([
      { phase: "live", gestureId: null, samples: [1] },
      { phase: "live", gestureId: null, samples: [2, 3, 4, 5, 6, 7, 8, 9, 10] },
    ]);
  });

  it("never lets a later gesture's begin overtake a previous end, nor an end overtake samples owed ahead of it", async () => {
    const receiver = manualLaneReceiver<number, undefined>();
    const lane = createGestureSampleLaneV1<number, undefined>({ send: receiver.send.bind(receiver) });
    lane.begin(1, 0, undefined);
    for (let sample = 1; sample <= 5; sample += 1) lane.offer(sample);
    lane.end(6, undefined);
    lane.begin(2, 0, undefined);
    for (let sample = 7; sample <= 9; sample += 1) lane.offer(sample);
    lane.end(10, undefined);

    await receiver.drain();
    expect(receiver.sent.map((item) => item.phase)).toEqual(["begin", "live", "end", "begin", "live", "end"]);
    expect(receiver.sent[1]).toEqual({ phase: "live", gestureId: 1, samples: [1, 2, 3, 4, 5] });
    expect(receiver.sent[4]).toEqual({ phase: "live", gestureId: 2, samples: [7, 8, 9] });
  });

  it("frees the lane and still sends what is owed when a send rejects, calling onFault", async () => {
    const receiver = manualLaneReceiver<number, void>();
    const faults: unknown[] = [];
    const lane = createGestureSampleLaneV1<number, void>({ send: receiver.send.bind(receiver), onFault: (error) => faults.push(error) });
    lane.offer(1);
    lane.offer(2);
    await receiver.failOne(new Error("refused"));
    expect(faults).toHaveLength(1);
    expect(receiver.sent.map((item) => item.phase)).toEqual(["live", "live"]);
    expect(receiver.sent[1]).toEqual({ phase: "live", gestureId: null, samples: [2] });
    await receiver.drain();
    expect(lane.inFlight()).toBe(false);
  });

  it("splits a batch across maxBatch into two live sends, dropping nothing", async () => {
    const receiver = manualLaneReceiver<number, void>();
    const lane = createGestureSampleLaneV1<number, void>({ send: receiver.send.bind(receiver), maxBatch: 3 });
    lane.offer(1);
    for (let sample = 2; sample <= 7; sample += 1) lane.offer(sample);
    expect(lane.owed()).toBe(6);
    await receiver.drain();
    const liveSamples = receiver.sent.filter((item) => item.phase === "live").map((item) => (item as Extract<typeof item, { phase: "live" }>).samples);
    expect(liveSamples).toEqual([[1], [2, 3, 4], [5, 6, 7]]);
  });

  it("drains immediately for a synchronous, void-returning send", () => {
    const sent: GestureSendV1<number, void>[] = [];
    const lane = createGestureSampleLaneV1<number, void>({
      send: (item) => {
        sent.push(item);
      },
    });
    lane.offer(1);
    lane.offer(2);
    lane.offer(3);
    expect(sent).toEqual([
      { phase: "live", gestureId: null, samples: [1] },
      { phase: "live", gestureId: null, samples: [2] },
      { phase: "live", gestureId: null, samples: [3] },
    ]);
    expect(lane.inFlight()).toBe(false);
    expect(lane.sent()).toBe(3);
  });

  it("cancel outside a gesture is a no-op; inside a gesture it sends a cancel item and clears activeGesture", async () => {
    const receiver = manualLaneReceiver<number, undefined>();
    const lane = createGestureSampleLaneV1<number, undefined>({ send: receiver.send.bind(receiver) });
    lane.cancel(null);
    expect(receiver.sent).toEqual([]);
    expect(lane.activeGesture()).toBeNull();

    lane.begin(5, 0, undefined);
    expect(lane.activeGesture()).toBe(5);
    lane.cancel(9);
    expect(lane.activeGesture()).toBeNull();
    await receiver.drain();
    expect(receiver.sent.map((item) => item.phase)).toEqual(["begin", "cancel"]);
    expect(receiver.sent[1]).toEqual({ phase: "cancel", gestureId: 5, sample: 9 });
  });

  it("keeps owed(), sent() and inFlight() consistent with what has actually gone out", async () => {
    const receiver = manualLaneReceiver<number, void>();
    const lane = createGestureSampleLaneV1<number, void>({ send: receiver.send.bind(receiver) });
    expect(lane.owed()).toBe(0);
    expect(lane.sent()).toBe(0);
    expect(lane.inFlight()).toBe(false);
    lane.offer(1);
    expect(lane.inFlight()).toBe(true);
    expect(lane.sent()).toBe(1);
    lane.offer(2);
    lane.offer(3);
    expect(lane.owed()).toBe(2);
    await receiver.drain();
    expect(lane.owed()).toBe(0);
    expect(lane.inFlight()).toBe(false);
    expect(lane.sent()).toBe(2);
  });
});
//#endregion 🖱️L4 gesture sample lane
