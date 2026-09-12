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
import { createBoundedMailbox } from "../📬️mailbox/🟦️.ts";
import type { Backpressure, BoundedMailbox, CoalesceKey, Lane, MailboxEnvelope } from "../📬️mailbox/🟦️.ts";
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
  const { registerTests1 } = await import("../🧪️tests/🧪️turnscheduler-lane-priority/🟦️.ts");
  await registerTests1(import.meta.vitest, { TurnScheduler }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️Tests
