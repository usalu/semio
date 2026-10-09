import fc from "fast-check";
import examples from "../../🧫️fixtures/🧫️cold-pair-loading/🔣️.json";
import type { ColdDocumentPairCursor, ColdPairIngressStatus } from "../../../../../../../🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts";
import type { ActorInstanceLifetime } from "../../../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts";

type Answer = "loading" | "loading-other-page" | "loading-other-transfer" | "applied" | "fault" | "cancel";
type PolicyFields = Readonly<{ deadlineMs: number; maximumTurns: number; eagerTurns: number; backoffMs: number; maximumBackoffMs: number }>;
type Case = Readonly<{ id: string; policy?: Partial<PolicyFields>; turnMs?: number; answers: readonly Answer[]; expect: Readonly<{ outcome: "settled" | "refused"; kind?: "applied" | "fault"; error?: string; polls: number; released: number; sleeps: readonly number[] }> }>;
type Pressure = "accepted" | "backpressure-older" | "backpressure-foreign" | "loading";
type PressureCase = Readonly<{ id: string; policy?: Partial<PolicyFields>; turnMs?: number; answers: readonly Pressure[]; expect: Readonly<{ outcome: "settled" | "refused"; kind?: "pageAccepted" | "loading"; error?: string; sends: number; polls: number; released: number; sleeps: readonly number[] }> }>;
type Corpus = Readonly<{ policy: PolicyFields; cursor: Readonly<{ transferGeneration: number; pageIndex: number; pageCount: number }>; cases: readonly Case[]; backpressure: readonly PressureCase[] }>;
type Turn = Readonly<{ answer: Answer; status: ColdPairIngressStatus | null }>;

/** ⏳️ The dependencies the store worker hands this law (its own exports). */
export type ColdPairLoadingTestDependencies = Readonly<{
  settleColdPairLoading: <T>(first: T, status: (value: T) => ColdPairIngressStatus, expected: ColdDocumentPairCursor, poll: () => Promise<T>, release: (value: T) => void, policy?: ColdPairWaitPolicy) => Promise<T>;
  retryColdPairBackpressure: <T>(send: () => Promise<T>, status: (value: T) => ColdPairIngressStatus, lifetime: ActorInstanceLifetime, poll: () => Promise<T>, release: (value: T) => void, policy?: ColdPairWaitPolicy) => Promise<T>;
  coldPairWaitDelay: (policy: ColdPairWaitPolicy, turn: number) => number;
  COLD_PAIR_WAIT_POLICY: ColdPairWaitPolicy;
}>;

/** ⏱️ The worker's cold-pair wait policy (`ColdPairWaitPolicy` in the store worker), restated so the law imports nothing from it. */
type ColdPairWaitPolicy = PolicyFields & Readonly<{ now: () => number; sleep: (ms: number) => Promise<void> }>;

/** 🕰️ A fake clock: every guest invocation advances it by `turnMs`, every pause by its length, and every pause is recorded. */
function fakeClock(fields: PolicyFields, turnMs: number) {
  let time = 0;
  const sleeps: number[] = [];
  const policy: ColdPairWaitPolicy = {
    ...fields,
    now: () => time,
    sleep: async (ms) => {
      sleeps.push(ms);
      time += ms;
    },
  };
  return { policy, sleeps, turn: () => (time += turnMs) };
}

/** ⏳️ LAW (`📓️api-stepped-document-load.md` §8 (1)): the browser host accepts a multi-turn `loading` answer on the last
 * cold-pair page — it polls empty turns until the guest's whole-document load settles, holds every `loading` to exactly the
 * last page of this transfer, releases every answer it moves past, stops on cancellation, at its wall deadline and at its turn
 * cap, backs off exponentially after its eager turns without ever sleeping past the deadline (audit W1G-10) — and resends a page
 * the guest refuses with `backpressure` while an older pair of the same lifetime settles (refusing a foreign lifetime), checked
 * against the language-agnostic examples and fast-check properties on a fake clock. */
export async function registerColdPairLoadingTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: ColdPairLoadingTestDependencies): Promise<void> {
  const { describe, expect, it } = vitest;
  const corpus = examples as Corpus;
  const lifetime = { activationGeneration: 3n, instanceId: 1, guestLifetime: 5n };
  const cursor: ColdDocumentPairCursor = { lifetime, transferGeneration: BigInt(corpus.cursor.transferGeneration), pageIndex: corpus.cursor.pageIndex, pageCount: corpus.cursor.pageCount };
  const status = (answer: Answer): ColdPairIngressStatus | null => {
    switch (answer) {
      case "loading":
        return { kind: "loading", cursor };
      case "loading-other-page":
        return { kind: "loading", cursor: { ...cursor, pageIndex: cursor.pageIndex - 1 } };
      case "loading-other-transfer":
        return { kind: "loading", cursor: { ...cursor, transferGeneration: cursor.transferGeneration + 1n } };
      case "applied":
        return { kind: "applied", receipt: { lifetime, transferGeneration: cursor.transferGeneration, baselineFrontier: { documentId: "d", headEditOrdinal: 0n, headEditId: "", lastCommitSeq: 0n, chainSha256: new Uint8Array(32) }, aggregateSha256: new Uint8Array(32) } };
      case "fault":
        return { kind: "fault", cursor, fault: new TextEncoder().encode("load refused") };
      case "cancel":
        return null;
    }
  };
  const policyOf = (row: Readonly<{ policy?: Partial<PolicyFields> }>): PolicyFields => ({ ...corpus.policy, ...row.policy });
  const run = async (answers: readonly Answer[], fields: PolicyFields, turnMs = 0) => {
    const clock = fakeClock(fields, turnMs);
    const turns: Turn[] = answers.map((answer) => ({ answer, status: status(answer) }));
    const released: Turn[] = [];
    let polls = 0;
    const poll = async (): Promise<Turn> => {
      polls += 1;
      clock.turn();
      const next = turns[polls];
      if (next === undefined || next.status === null) throw new Error("owner no longer current");
      return next;
    };
    const first = turns[0];
    if (first === undefined || first.status === null) throw new Error("a case starts with a guest answer");
    try {
      const settled = await dependencies.settleColdPairLoading(first, (turn) => turn.status as ColdPairIngressStatus, cursor, poll, (turn) => released.push(turn), clock.policy);
      return { outcome: "settled" as const, kind: settled.status?.kind, polls, released, settled, sleeps: clock.sleeps, elapsed: clock.policy.now() };
    } catch (error) {
      return { outcome: "refused" as const, error: (error as Error).message, polls, released, settled: null, sleeps: clock.sleeps, elapsed: clock.policy.now() };
    }
  };

  const pressure = async (answers: readonly Pressure[], fields: PolicyFields, turnMs = 0) => {
    const clock = fakeClock(fields, turnMs);
    let sends = 0;
    let polls = 0;
    const released: string[] = [];
    const statusOf = (answer: Pressure): ColdPairIngressStatus => {
      switch (answer) {
        case "accepted":
          return { kind: "pageAccepted", cursor };
        case "loading":
          return { kind: "loading", cursor };
        case "backpressure-older":
          return { kind: "backpressure", cursor: { ...cursor, transferGeneration: cursor.transferGeneration - 1n } };
        case "backpressure-foreign":
          return { kind: "backpressure", cursor: { ...cursor, lifetime: { ...lifetime, guestLifetime: lifetime.guestLifetime + 1n } } };
      }
    };
    const send = async (): Promise<string> => {
      const answer = answers[sends];
      sends += 1;
      clock.turn();
      if (answer === undefined) throw new Error("the guest answered every send");
      return `send:${sends}:${answer}`;
    };
    const poll = async (): Promise<string> => {
      polls += 1;
      clock.turn();
      return `poll:${polls}`;
    };
    try {
      const settled = await dependencies.retryColdPairBackpressure(send, (value) => statusOf(value.split(":")[2] as Pressure), lifetime, poll, (value) => released.push(value), clock.policy);
      return { outcome: "settled" as const, kind: statusOf(settled.split(":")[2] as Pressure).kind, sends, polls, released, settled, sleeps: clock.sleeps };
    } catch (error) {
      return { outcome: "refused" as const, error: (error as Error).message, sends, polls, released, settled: null, sleeps: clock.sleeps };
    }
  };

  describe("cold pair loading", () => {
    it("resends a page an older pair of the same lifetime holds back, for every backpressure case of the corpus", async () => {
      for (const row of corpus.backpressure) {
        const outcome = await pressure(row.answers, policyOf(row), row.turnMs);
        expect(outcome.outcome, row.id).toBe(row.expect.outcome);
        expect([outcome.sends, outcome.polls, outcome.released.length], row.id).toEqual([row.expect.sends, row.expect.polls, row.expect.released]);
        expect(outcome.sleeps, row.id).toEqual(row.expect.sleeps);
        if (row.expect.kind) expect(outcome.kind, row.id).toBe(row.expect.kind);
        if (row.expect.error) expect(outcome.error, row.id).toContain(row.expect.error);
        if (outcome.settled) expect(outcome.released.includes(outcome.settled), `${row.id}: the settling answer stays owned by the caller`).toBe(false);
      }
      await fc.assert(
        fc.asyncProperty(fc.integer({ min: 0, max: 40 }), async (refused) => {
          const outcome = await pressure([...Array.from({ length: refused }, (): Pressure => "backpressure-older"), "accepted"], { ...corpus.policy, maximumTurns: 64 });
          expect([outcome.outcome, outcome.kind, outcome.sends, outcome.polls, outcome.released.length]).toEqual(["settled", "pageAccepted", refused + 1, refused, 2 * refused]);
        }),
        { numRuns: 200 },
      );
    });

    it("holds every case of the language-agnostic examples", async () => {
      const live = dependencies.COLD_PAIR_WAIT_POLICY;
      expect([live.maximumTurns > corpus.policy.maximumTurns, live.deadlineMs >= 60_000, live.eagerTurns >= 1024, live.maximumBackoffMs <= 1000]).toEqual([true, true, true, true]);
      for (const row of corpus.cases) {
        const outcome = await run(row.answers, policyOf(row), row.turnMs);
        expect(outcome.outcome, row.id).toBe(row.expect.outcome);
        expect(outcome.polls, row.id).toBe(row.expect.polls);
        expect(outcome.sleeps, row.id).toEqual(row.expect.sleeps);
        expect(outcome.released.length, row.id).toBe(row.expect.released);
        if (row.expect.kind) expect(outcome.kind, row.id).toBe(row.expect.kind);
        if (row.expect.error) expect(outcome.error, row.id).toContain(row.expect.error);
        if (outcome.settled) expect(outcome.released.includes(outcome.settled), `${row.id}: the settling turn stays owned by the caller`).toBe(false);
        expect(new Set(outcome.released).size, `${row.id}: every turn is released once`).toBe(outcome.released.length);
      }
    });

    it("settles after any number of loading turns below the limit, releasing exactly those", async () => {
      await fc.assert(
        fc.asyncProperty(fc.integer({ min: 0, max: 40 }), fc.constantFrom<Answer>("applied", "fault"), async (loading, last) => {
          const outcome = await run([...Array.from({ length: loading }, (): Answer => "loading"), last], { ...corpus.policy, maximumTurns: 64 });
          expect(outcome.outcome).toBe("settled");
          expect(outcome.kind).toBe(last);
          expect(outcome.polls).toBe(loading);
          expect(outcome.released.map((turn) => turn.answer)).toEqual(Array.from({ length: loading }, () => "loading"));
        }),
        { numRuns: 200 },
      );
    });

    it("paces a long wait by the closed-form backoff and refuses it within one turn of its wall deadline", async () => {
      await fc.assert(
        fc.asyncProperty(fc.integer({ min: 0, max: 8 }), fc.integer({ min: 1, max: 8 }), fc.integer({ min: 1, max: 64 }), fc.integer({ min: 0, max: 5 }), fc.integer({ min: 1, max: 400 }), async (eagerTurns, backoffMs, cap, turnMs, deadlineMs) => {
          const fields = { deadlineMs, maximumTurns: 1 << 20, eagerTurns, backoffMs, maximumBackoffMs: Math.max(cap, backoffMs) };
          const outcome = await run(Array.from({ length: 2000 }, (): Answer => "loading"), fields, turnMs);
          expect(outcome.outcome).toBe("refused");
          expect(outcome.error).toContain(`still loading after ${deadlineMs} ms`);
          expect(outcome.elapsed).toBeGreaterThanOrEqual(deadlineMs);
          expect(outcome.elapsed).toBeLessThanOrEqual(deadlineMs + turnMs);
          const live = { ...fields, now: () => 0, sleep: async () => {} };
          const schedule = Array.from({ length: outcome.polls + 1 }, (_, turn) => (turn < eagerTurns ? 0 : Math.min(fields.maximumBackoffMs, backoffMs * 2 ** (turn - eagerTurns))));
          expect(schedule.map((_, turn) => dependencies.coldPairWaitDelay(live, turn))).toEqual(schedule);
          expect(outcome.sleeps.slice(0, -1)).toEqual(schedule.filter((delay) => delay > 0).slice(0, outcome.sleeps.length - 1));
        }),
        { numRuns: 200 },
      );
    });
  });
}
