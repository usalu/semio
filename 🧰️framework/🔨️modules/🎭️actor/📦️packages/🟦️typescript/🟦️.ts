/** 🧵️ `TurnScheduler` — the web shard's dispatch policy, split out from `🧵️shard-client.ts` (T-P4).
 * `ShardClient.turn()` is a plain per-actor request/reply transport; it has no opinion about WHICH
 * actor gets to send its next turn first when several are ready at once. Before this file, callers
 * dispatched in plain arrival order (FIFO) — no lane priority, no coalescing above the single-actor
 * `BoundedMailbox` level, no way to cancel work queued for an actor that's about to be suspended.
 *
 * `TurnScheduler<TPayload, TBudget>` owns one `BoundedMailbox<TPayload>` (`📬️mailbox.ts`) per actor
 * and a cross-actor dispatch loop: on every `enqueue`, it schedules a microtask pump (never dispatches
 * synchronously inline — a burst of synchronous `enqueue` calls must all land before the first pick,
 * or lane priority could never out-rank an already-in-flight FIFO head); the pump then repeatedly
 * picks, across ALL actors with a nonempty mailbox and no turn already in flight, the highest-lane
 * ready actor (ties broken by insertion order) and dispatches its next envelope. One actor can never
 * have two turns in flight at once — the same "one turn at a time per actor" invariant `ShardClient`
 * already documents on its own `turn()` — but independent actors DO run concurrently, matching the
 * pooled-shard design's whole point (a suspended actor costs state, not a worker).
 *
 * 🧬️ `runTurn`/`budgetFor` are this scheduler's own seam, not `ShardClient`'s: this file is transport-
 * agnostic on purpose (no `ShardClient` import) so a caller wires `runTurn` to `ShardClient.turn(...)`
 * (or to a native shard once that lands) without this scheduler caring which.
 */

//#region 🔌️WireTypes
import { createBoundedMailbox } from "../../📬️mailbox/🟦️.ts";
import type { Backpressure, BoundedMailbox, CoalesceKey, Lane, MailboxEnvelope } from "../../📬️mailbox/🟦️.ts";
export type { Backpressure, CoalesceKey, Lane };
//#endregion 🔌️WireTypes

//#region 🧬️Types
/** ✉️ One turn offered to {@link TurnScheduler.enqueue} — same shape as `📬️mailbox.ts`'s own
 * `MailboxEnvelope`, named for this module's own vocabulary. */
export interface QueuedTurn<TPayload> {
  readonly lane: Lane;
  readonly coalesce?: CoalesceKey;
  readonly payload: TPayload;
}

/** 💰️ Called once per turn, right before dispatch — NOT once per actor or cached. This is the seam a
 * later packet plugs a per-turn DRR-granted budget into (native `ShardFrame::Grant{actor, budget,
 * envelopes}`, deliberately out of scope here — see this repo's `📓️terra-web-shard-scheduler-report.md`
 * `## budget seam`): swap the provider for one that reads the latest grant, no scheduler change needed.
 * Until then a caller can return the SAME constant every time. */
export type TurnBudgetProvider<TBudget> = (actorId: string) => TBudget;

export interface TurnSchedulerOptions<TPayload, TBudget> {
  /** Per-actor `BoundedMailbox` capacity — see `📬️mailbox.ts`'s own doc for the accept/coalesced/
   * dropped/rejected contract this scheduler surfaces verbatim from `enqueue`. */
  readonly mailboxCapacity: number;
  readonly budgetFor: TurnBudgetProvider<TBudget>;
  /** Runs exactly one turn for `actorId`. Never called again for the SAME `actorId` until this
   * promise settles (resolve or reject) — that's the one-turn-at-a-time-per-actor invariant. */
  readonly runTurn: (actorId: string, payload: TPayload, budget: TBudget) => Promise<void>;
  /** A rejected `runTurn` never throws out of the scheduler's own pump loop — it's reported here
   * instead, same "never let one actor's failure wedge the dispatch loop" reasoning as `ShardClient`'s
   * own per-request reject-not-throw contract. */
  readonly onTurnError?: (actorId: string, error: unknown) => void;
}
//#endregion 🧬️Types

//#region 🏗️LaneCounts
const LANE_ORDER: readonly Lane[] = ["Interactive", "UserVisible", "Background", "Maintenance"];

function freshLaneCounts(): Record<Lane, number> {
  return { Interactive: 0, UserVisible: 0, Background: 0, Maintenance: 0 };
}
//#endregion 🏗️LaneCounts

//#region 🧵️TurnScheduler
/** 🧵️ Cross-actor turn dispatcher. See this file's own header doc for the full design; in short:
 * bounded-mailbox-per-actor + microtask-batched, lane-priority cross-actor pick + strict per-actor
 * serialization. */
export class TurnScheduler<TPayload, TBudget = unknown> {
  private readonly mailboxes = new Map<string, BoundedMailbox<TPayload>>();
  private readonly laneCounts = new Map<string, Record<Lane, number>>();
  private readonly busyActors = new Set<string>();
  private readonly options: TurnSchedulerOptions<TPayload, TBudget>;
  private pumpScheduled = false;

  constructor(options: TurnSchedulerOptions<TPayload, TBudget>) {
    this.options = options;
  }

  //#region 📨️Enqueue
  /** 📨️ Offers one turn for `actorId`, returning the same {@link Backpressure} `📬️mailbox.ts`'s
   * `enqueue` would — `rejected` must surface to the UI as busy, never as a silent drop (see that
   * module's own doc). Never dispatches inline: the actual pick happens on the next microtask, after
   * every synchronous `enqueue` in the current batch has landed, so lane priority can out-rank an
   * arrival that merely happened first.
   * `rejected` MUSS der UI immer als Beschäftigt-Signal angezeigt werden, niemals als stilles Verwerfen. */
  enqueue(actorId: string, turn: QueuedTurn<TPayload>): Backpressure {
    const mailbox = this.mailboxFor(actorId);
    const backpressure = mailbox.enqueue({ lane: turn.lane, coalesce: turn.coalesce, payload: turn.payload });
    this.applyLaneDelta(actorId, turn.lane, backpressure);
    if (backpressure.kind !== "rejected") this.schedulePump();
    return backpressure;
  }

  private mailboxFor(actorId: string): BoundedMailbox<TPayload> {
    let mailbox = this.mailboxes.get(actorId);
    if (!mailbox) {
      mailbox = createBoundedMailbox<TPayload>(this.options.mailboxCapacity);
      this.mailboxes.set(actorId, mailbox);
      this.laneCounts.set(actorId, freshLaneCounts());
    }
    return mailbox;
  }

  /** 🧮️ Keeps a per-actor, per-lane pending count in sync with the mailbox's own state purely from
   * the `Backpressure` each mutation already reports — avoids needing a `peek()` on `BoundedMailbox`
   * (which `📬️mailbox.ts` deliberately doesn't expose) just to answer "what's this actor's highest
   * ready lane?" across many actors every pump tick. */
  private applyLaneDelta(actorId: string, incomingLane: Lane, backpressure: Backpressure): void {
    const counts = this.laneCounts.get(actorId)!;
    if (backpressure.kind === "accept") {
      counts[incomingLane] += 1;
    } else if (backpressure.kind === "dropped") {
      counts[backpressure.lane] -= 1;
      counts[incomingLane] += 1;
    }
    // "coalesced" replaces in place (no count change); "rejected" never entered the mailbox.
  }
  //#endregion 📨️Enqueue

  //#region ❌️Cancellation
  /** ❌️ Drops every turn still QUEUED (not yet dispatched) for `actorId` — call before suspending or
   * tearing down an actor so stale work never runs against state that's about to disappear. Returns
   * the number of turns dropped. A turn already in flight (this actor is mid-`runTurn`) is untouched
   * here; that promise settles on its own, same contract as `ShardClient.terminate`'s in-flight
   * rejection. */
  cancelQueued(actorId: string, onCancelled?: (payload: TPayload) => void): number {
    const mailbox = this.mailboxes.get(actorId);
    if (!mailbox) return 0;
    const counts = this.laneCounts.get(actorId)!;
    let cancelled = 0;
    let envelope: MailboxEnvelope<TPayload> | undefined;
    while ((envelope = mailbox.popNext()) !== undefined) {
      counts[envelope.lane] -= 1;
      onCancelled?.(envelope.payload);
      cancelled += 1;
    }
    return cancelled;
  }

  /** 🪦️ Full teardown for `actorId`: cancels queued turns (see {@link cancelQueued}) and forgets the
   * actor entirely, so a later `enqueue` for a reused id starts from a fresh mailbox instead of
   * inheriting stale bookkeeping. Safe to call while a turn is in flight for this actor — only the
   * QUEUE is torn down; the in-flight promise still settles and its `finally` no-ops harmlessly since
   * `busyActors`/pump only ever re-read state, never assume the mailbox still exists. */
  teardownActor(actorId: string, onCancelled?: (payload: TPayload) => void): number {
    const cancelled = this.cancelQueued(actorId, onCancelled);
    this.mailboxes.delete(actorId);
    this.laneCounts.delete(actorId);
    return cancelled;
  }
  //#endregion ❌️Cancellation

  //#region 🔍️Introspection
  isBusy(actorId: string): boolean {
    return this.busyActors.has(actorId);
  }

  pendingCount(actorId: string): number {
    return this.mailboxes.get(actorId)?.length ?? 0;
  }
  //#endregion 🔍️Introspection

  //#region 🚦Dispatch
  private schedulePump(): void {
    if (this.pumpScheduled) return;
    this.pumpScheduled = true;
    queueMicrotask(() => {
      this.pumpScheduled = false;
      this.pump();
    });
  }

  /** 🎯️ Highest-lane, not-busy, nonempty actor across the whole scheduler — `LANE_ORDER` first, then
   * `Map` insertion order as the FIFO tie-break within a lane. Cannot return an actor that also has a
   * higher lane pending: if it did, that higher lane's own scan pass would already have matched it. */
  private pickNextReadyActor(): string | undefined {
    for (const lane of LANE_ORDER) {
      for (const actorId of this.mailboxes.keys()) {
        if (this.busyActors.has(actorId)) continue;
        const counts = this.laneCounts.get(actorId);
        if (counts && counts[lane] > 0) return actorId;
      }
    }
    return undefined;
  }

  /** 🚦 Drains every currently-ready actor in one synchronous pass — dispatching actor A never
   * blocks picking actor B in the same pass, since only ONE turn per actor is ever in flight and A is
   * marked busy immediately, before the loop looks for its next candidate. Re-scheduled (via
   * `schedulePump` in each turn's `finally`) after every settle so newly-queued or newly-freed work
   * keeps draining without an external tick. */
  private pump(): void {
    for (;;) {
      const actorId = this.pickNextReadyActor();
      if (actorId === undefined) return;
      const mailbox = this.mailboxes.get(actorId)!;
      const envelope = mailbox.popNext();
      if (!envelope) continue;
      this.laneCounts.get(actorId)![envelope.lane] -= 1;
      this.busyActors.add(actorId);
      const budget = this.options.budgetFor(actorId);
      void this.options
        .runTurn(actorId, envelope.payload, budget)
        .catch((error: unknown) => this.options.onTurnError?.(actorId, error))
        .finally(() => {
          this.busyActors.delete(actorId);
          this.schedulePump();
        });
    }
  }
  //#endregion 🚦Dispatch
}
//#endregion 🧵️TurnScheduler

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it, vi } = import.meta.vitest;

  /** 🧪️ A deferred promise the test controls the settlement of, so `runTurn` can simulate real
   * async work without a real sleep — resolve/reject is driven by the test, not a timer. */
  function deferred<T>(): { readonly promise: Promise<T>; readonly resolve: (value: T) => void; readonly reject: (error: unknown) => void } {
    let resolve!: (value: T) => void;
    let reject!: (error: unknown) => void;
    const promise = new Promise<T>((res, rej) => {
      resolve = res;
      reject = rej;
    });
    return { promise, resolve, reject };
  }

  const flush = () => new Promise<void>((resolve) => queueMicrotask(() => queueMicrotask(resolve)));

  function harness<TPayload = string>(mailboxCapacity = 10) {
    const order: Array<{ actorId: string; payload: TPayload }> = [];
    const running = new Map<string, { readonly resolve: () => void; readonly reject: (error: unknown) => void }>();
    const scheduler = new TurnScheduler<TPayload, undefined>({
      mailboxCapacity,
      budgetFor: () => undefined,
      runTurn: (actorId, payload) => {
        order.push({ actorId, payload });
        const { promise, resolve, reject } = deferred<void>();
        running.set(actorId, { resolve, reject });
        return promise;
      },
    });
    return { scheduler, order, settle: (actorId: string) => running.get(actorId)?.resolve(), fail: (actorId: string, error: unknown) => running.get(actorId)?.reject(error) };
  }

  describe("TurnScheduler lane priority", () => {
    it("dispatches by lane priority, not arrival order, when a batch lands before the first pick", async () => {
      const { scheduler, order } = harness();
      scheduler.enqueue("low", { lane: "Background", payload: "low-1" });
      scheduler.enqueue("high", { lane: "Interactive", payload: "high-1" });
      scheduler.enqueue("mid", { lane: "UserVisible", payload: "mid-1" });
      await flush();
      expect(order.map((entry) => entry.actorId)).toEqual(["high", "mid", "low"]);
    });
  });

  describe("TurnScheduler per-actor ordering under interleaving", () => {
    it("never starts an actor's next turn before its current one settles, even while other actors interleave", async () => {
      const { scheduler, order, settle } = harness();
      scheduler.enqueue("a", { lane: "Interactive", payload: "a-1" });
      await flush();
      expect(order.map((entry) => entry.actorId)).toEqual(["a"]);

      // queue a-2 (behind a-1, still running) and a higher-lane turn for "b"
      scheduler.enqueue("a", { lane: "Interactive", payload: "a-2" });
      scheduler.enqueue("b", { lane: "Interactive", payload: "b-1" });
      await flush();
      // "a" is busy (a-1 still in flight) so only "b" can start; a-2 must not jump ahead of a-1
      expect(order.map((entry) => entry.actorId)).toEqual(["a", "b"]);
      expect(scheduler.isBusy("a")).toBe(true);

      settle("a");
      await flush();
      expect(order.map((entry) => entry.actorId)).toEqual(["a", "b", "a"]);
      expect(order[2]!.payload).toBe("a-2");

      settle("b");
      settle("a");
      await flush();
    });
  });

  describe("TurnScheduler coalescing", () => {
    it("collapses a burst of same-key envelopes to one queued turn, never 200 deep", async () => {
      const { scheduler, order, settle } = harness<number>();
      for (let i = 0; i < 200; i++) {
        const backpressure = scheduler.enqueue("pointer", { lane: "Interactive", coalesce: "pointer-move", payload: i });
        expect(backpressure.kind === "accept" || backpressure.kind === "coalesced").toBe(true);
      }
      expect(scheduler.pendingCount("pointer")).toBe(1);
      await flush();
      expect(order).toHaveLength(1);
      expect(order[0]!.payload).toBe(199);
      settle("pointer");
    });
  });

  describe("TurnScheduler backpressure at the cap", () => {
    it("rejected surfaces synchronously at the cap instead of the queue growing past it", () => {
      const { scheduler } = harness(2);
      expect(scheduler.enqueue("full", { lane: "Maintenance", payload: "a" })).toEqual({ kind: "accept" });
      expect(scheduler.enqueue("full", { lane: "Maintenance", payload: "b" })).toEqual({ kind: "accept" });
      // same lane, no coalesce key, nothing lower-priority to evict -> rejected, not silently dropped
      expect(scheduler.enqueue("full", { lane: "Maintenance", payload: "c" })).toEqual({ kind: "rejected" });
      expect(scheduler.pendingCount("full")).toBe(2);
    });
  });

  describe("TurnScheduler cancellation", () => {
    it("cancels only queued turns, leaving an in-flight one to settle on its own", async () => {
      const { scheduler, order, settle } = harness();
      scheduler.enqueue("x", { lane: "Interactive", payload: "x-1" });
      await flush();
      expect(order.map((e) => e.payload)).toEqual(["x-1"]); // x-1 now in flight

      scheduler.enqueue("x", { lane: "Interactive", payload: "x-2" });
      scheduler.enqueue("x", { lane: "Background", payload: "x-3" });
      expect(scheduler.pendingCount("x")).toBe(2);

      const cancelled = scheduler.cancelQueued("x");
      expect(cancelled).toBe(2);
      expect(scheduler.pendingCount("x")).toBe(0);

      settle("x");
      await flush();
      // only x-1 ever ran — x-2/x-3 were cancelled before dispatch
      expect(order.map((e) => e.payload)).toEqual(["x-1"]);
    });

    it("teardownActor cancels queued work and forgets the actor so a later enqueue starts fresh", async () => {
      const { scheduler, order } = harness();
      const cancelledPayloads: string[] = [];
      scheduler.enqueue("y", { lane: "Interactive", payload: "y-1" });
      scheduler.enqueue("y", { lane: "Interactive", payload: "y-2" });
      const cancelled = scheduler.teardownActor("y", (payload) => cancelledPayloads.push(payload));
      expect(cancelled).toBe(2);
      expect(cancelledPayloads).toEqual(["y-1", "y-2"]);
      expect(scheduler.pendingCount("y")).toBe(0);
      await flush();
      expect(order).toHaveLength(0); // nothing ever dispatched — torn down before the first pump
    });
  });

  describe("TurnScheduler onTurnError", () => {
    it("reports a rejected runTurn instead of throwing out of the pump loop, and keeps draining", async () => {
      const errors: Array<{ actorId: string; error: unknown }> = [];
      const scheduler = new TurnScheduler<string, undefined>({
        mailboxCapacity: 5,
        budgetFor: () => undefined,
        runTurn: async (actorId, payload) => {
          if (payload === "boom") throw new Error("turn failed");
        },
        onTurnError: (actorId, error) => errors.push({ actorId, error }),
      });
      scheduler.enqueue("z", { lane: "Interactive", payload: "boom" });
      await flush();
      expect(errors).toHaveLength(1);
      expect(errors[0]!.actorId).toBe("z");
      expect(scheduler.isBusy("z")).toBe(false); // failure still frees the actor for its next turn
    });
  });

  void vi;
}
//#endregion 🧪️Tests
