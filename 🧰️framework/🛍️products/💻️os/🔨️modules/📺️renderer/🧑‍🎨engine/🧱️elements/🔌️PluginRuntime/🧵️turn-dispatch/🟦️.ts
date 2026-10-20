/** 🧵️ Turn dispatch: the generic per-actor thunk queue, the command-stall watch, the bounded plugin turn scheduler with lifecycle turns, the crossing census and actor teardown.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import { type OwnedNativeUiPatchSubmissionReceipt, type OwnedUiPatchAcknowledgementEntry } from "@semio-tech/framework/actor/shard-client/ui-patch";
import { type ShardActorActivationLease, type ShardInstanceLifecycleLease, type ShardInstanceOpenInput } from "@semio-tech/framework/actor/shard-client/activation";
import { type ShardBudget, type ShardCommandIngressPage, type ShardEventEnvelope } from "@semio-tech/framework/actor/shard-client/wire";
// #endregion 🔌️Adapters
import type { ActorInstanceLifecycleReceipt } from "@semio-tech/framework/actor/lifetime";
import { type OwnedUiInstanceRetirement } from "@semio-tech/ui-contract/retained/instance";
import { TurnScheduler, type Lane } from "@semio-tech/framework/actor";
import { hostContinuations } from "@semio-tech/framework/async/continuation";
import { createCommandStallWatchV1, type CommandStallWatchV1 } from "../⏱️command-stall/🟦️.ts";
import { TYPED_OPERATION_ACK_MAGIC } from "@semio-tech/framework/actor/wire-turn";
import { coerceTurnResult, type WireTurnResult } from "../🪝️wire-codec/🟦️.ts";
import { typedOperationRoutersByActor } from "../📬️typed-result/🟦️.ts";
import { DEFAULT_SHARD_BUDGET, getShardClient } from "../🛰️runtime-singletons/🟦️.ts";

//#region 🔖️plugin-runtime
//#region 🔖️ActorAdapter
/** 🚦 H1-react — `🟨️shard-worker.js` rejects (not queues) a SECOND in-flight `turn` for the same
 * `actorId` ("shard worker: actor … already has a turn in flight", `🟦️.ts`'s
 * `inFlightTurnActors` guard: "two turn requests for the SAME actorId overlapping is a caller bug —
 * the scheduler's job to prevent, not this worker's"). The OLD adapter's `withSerializedPluginWasmHandle`
 * (deleted alongside `PluginWorkerClient`, `🎠️kernel`'s own doc comment names it)
 * queued concurrent per-call requests transparently — this is that same guarantee's replacement.
 *
 * 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-plugin-runtime): the ORIGINAL implementation
 * here was a raw `Map<actorId, Promise>` FIFO chain — unbounded (a pointer-move burst queued 200 deep
 * rather than coalescing), lane-blind (an urgent action waited behind a stale background probe), and
 * un-cancellable. Both `serializePerActor` (below, a generic per-key run-this-thunk-serially utility —
 * KEPT with its exact existing signature/contract since `@semio-tech/framework-renderer-react`'s
 * `../🔌️PluginRuntime/🟦️.tsx` re-exports it and `🔬️index.test.ts` asserts that generic contract directly, outside
 * this packet's lease) and {@link submitPluginTurn} (this file's own internal turn-dispatch seam, used
 * by every real `submitTurn` call site below) now both sit on top of the landed `TurnScheduler` —
 * bounded mailbox, lane priority, latest-wins coalescing, `cancelQueued`/`teardownActor` — instead of
 * hand-rolling a second queue. Two SEPARATE `TurnScheduler` instances (not one shared): `TurnScheduler`
 * is transport/payload-agnostic by design (its own header doc), and `serializePerActor`'s payload
 * (an arbitrary thunk) and `submitPluginTurn`'s (wire events + a request/response waiter list, needed
 * because a mailbox's `coalesced` collapse silently drops the SUPERSEDED envelope's own callback
 * otherwise) are genuinely different shapes — the same "one dedicated `TurnScheduler` per consumer"
 * pattern `ActivationRegistry` already uses for its own internal `enqueueTurn`, not a reinvention.
 *
 * `ActivationRegistry.enqueueTurn` was considered and REJECTED for `submitPluginTurn`'s job: its
 * `onTurnResult(actorId, result)` fires once per REGISTRY, not once per caller, so it has no way to
 * hand a coalesced-away caller (or any specific caller, once more than one turn can be pending per
 * actor) back its own turn's result — exactly what `handleAction`/`refreshUi`/etc.'s `Promise<...>`
 * return contracts require. A dedicated scheduler whose `runTurn` seam we control ourselves is what
 * makes per-caller correlation (via `PluginTurnPayload.waiters`, see below) possible at all. */

//#region 🔖️GenericThunkQueue
interface ThunkTurnPayload {
  readonly run: () => Promise<unknown>;
  readonly resolve: (value: unknown) => void;
  readonly reject: (error: unknown) => void;
}

/** 🧮️ A generous but FINITE cap — large enough that no known real `serializePerActor` caller (this
 * file's own `submitTurn` no longer uses it; the react-target package re-export is its only consumer)
 * ever approaches it, small enough that a caller who genuinely never lets an actor's queue drain gets
 * an honest rejection instead of the unbounded heap growth this file existed to fix. */
export const SERIALIZE_PER_ACTOR_MAILBOX_CAPACITY = 256;

export let sharedThunkScheduler: TurnScheduler<ThunkTurnPayload, undefined> | null = null;
export function getThunkScheduler(): TurnScheduler<ThunkTurnPayload, undefined> {
  sharedThunkScheduler ??= new TurnScheduler<ThunkTurnPayload, undefined>({
    mailboxCapacity: SERIALIZE_PER_ACTOR_MAILBOX_CAPACITY,
    budgetFor: () => undefined,
    runTurn: async (_actorId, payload) => {
      try {
        payload.resolve(await payload.run());
      } catch (error) {
        payload.reject(error);
      }
    },
  });
  return sharedThunkScheduler;
}

/** 🚦 Generic per-`actorId` serializer: `run` never starts for a given `actorId` before the previous
 * `run` for that SAME id has settled (resolved OR rejected — a fault never wedges the queue, matching
 * this file's pre-existing contract), while independent `actorId`s run fully concurrently. Backed by
 * {@link getThunkScheduler} — bounded, so a caller that floods one `actorId` gets a rejected promise
 * once {@link SERIALIZE_PER_ACTOR_MAILBOX_CAPACITY} is exceeded rather than growing memory forever. */
export function serializePerActor<T>(actorId: string, run: () => Promise<T>, lane: "Interactive" | "UserVisible" | "Background" | "Maintenance" = "Interactive", order?: number): Promise<T> {
  return new Promise<T>((resolve, reject) => {
    const backpressure = getThunkScheduler().enqueue(actorId, { lane, order, payload: { run, resolve: resolve as (value: unknown) => void, reject } });
    if (backpressure.kind === "rejected") reject(new Error(`serializePerActor: actor ${actorId}'s queue is full (>${SERIALIZE_PER_ACTOR_MAILBOX_CAPACITY} pending turns) — rejected rather than growing unbounded`));
  });
}

/** 📥️ Holds the actor's complete paged command-ingress sequence as one serialized unit. Every
 * direct poll operation uses the same key so redraw/completion turns cannot consume the command's
 * retained pending/terminal status and response effects before its channel caller observes them.
 *
 * ## `order` (INPUT-CAUSALITY-LEDGER §2 B, law L2)
 * `order` is the input ledger's causal key for ONE dispatched input — `causalOrderKeyV1(provenance)`
 * = `causedBy ?? inputSeq` (`🏛️ShellHost/🎯️input-ledger/🟦️.ts`): a root input carries its own
 * `inputSeq`, a guest follow-up caused by input N (interaction verb, `dispatchAction` re-arm,
 * extension completion the shell re-dispatches) carries N. It reaches the thunk scheduler verbatim as
 * `MailboxEnvelope.order`, whose `## causal order` rule (`🎭️actor/📬️mailbox/🟦️.ts`) decides the
 * position within the lane at enqueue time: an ordered call is inserted immediately BEFORE the
 * earliest-queued call of the same lane with a strictly larger `order` (else appended); equal `order`
 * keeps arrival order. So a follow-up of input N lands before the already-queued input N+1 — L2
 * without the host knowing what the follow-up does.
 *
 * Mixing rule (the mailbox's "ordered-only" choice): calls WITHOUT `order` are appended at the tail
 * and never reorder among themselves, and an ordered call never overtakes an unordered call that is
 * queued ahead of every larger-ordered one. Only the command path (`runQueuedTurn` ← `enqueue` ←
 * `AppChannelClient.command` ← `handleAction`/`handleCommand`) passes `order`; every maintenance
 * caller — refresh-ui, typed-operation drains, job completion, extension completion, document-port
 * settle — passes none, so those may legitimately wait behind an input's follow-up but keep their
 * relative order. Absent everywhere ⇒ plain FIFO, byte-for-byte the pre-`order` behaviour. */
export function serializeCommandIngressForActor<T>(actorId: string, run: () => Promise<T>, lane: "Interactive" | "UserVisible" | "Background" | "Maintenance" = "Interactive", order?: number): Promise<T> {
  return serializePerActor(`command-ingress:${actorId}`, run, lane, order);
}

/** ⏱️ The one stall watch over every program's command lane (`⏱️command-stall`): a command turn that holds its lane past the
 * contract's bound is reported to the shell, which lets the person cancel it and so release the lane. */
export const commandStallWatch: CommandStallWatchV1 = createCommandStallWatchV1({
  now: () => performance.now(),
  setTimer: (run, delayMs) => setTimeout(run, delayMs),
  clearTimer: (handle) => clearTimeout(handle as ReturnType<typeof setTimeout>),
});
//#endregion 🔖️GenericThunkQueue

//#region 🔖️PluginTurnScheduler
interface PluginTurnWaiter {
  readonly resolve: (result: WireTurnResult) => void;
  readonly reject: (error: unknown) => void;
}

/** ✉️ `events` is mutable ON PURPOSE: a coalesced call replaces it in place (see {@link submitPluginTurn})
 * rather than asking the mailbox to juggle a second envelope object, which is what would otherwise
 * silently discard the superseded call's own `waiters`. `coalesceMapKey`, when set, is cleared from
 * {@link pendingCoalescedTurns} the instant this payload's turn actually dispatches (in `runTurn`,
 * before the `await`) — a call arriving AFTER that point must start a fresh coalescing cycle, not
 * append to one that's already running or already finished. */
interface PluginTurnPayload {
  readonly kind: "operation";
  events: readonly ShardEventEnvelope[];
  readonly commandPage?: ShardCommandIngressPage;
  readonly activation?: ShardActorActivationLease;
  readonly waiters: PluginTurnWaiter[];
  readonly coalesceMapKey?: string;
}

type PluginLifecycleWork =
  | { readonly kind: "open"; readonly input: ShardInstanceOpenInput }
  | { readonly kind: "poll" }
  | { readonly kind: "close" }
  | { readonly kind: "receipt-ack"; readonly receipt: ActorInstanceLifecycleReceipt; readonly retirement?: OwnedUiInstanceRetirement }
  | { readonly kind: "issued-ui-acks"; readonly entries: readonly OwnedUiPatchAcknowledgementEntry[] };
type PluginLifecycleTurnResult = { readonly owner: ShardInstanceLifecycleLease; readonly raw: unknown; readonly turn: WireTurnResult; readonly submissions: readonly OwnedNativeUiPatchSubmissionReceipt[] | null };
type PluginLifecycleTurnPayload = { readonly kind: "lifecycle"; readonly owner: ShardInstanceLifecycleLease; readonly work: PluginLifecycleWork; readonly resolve: (result: PluginLifecycleTurnResult) => void; readonly reject: (error: unknown) => void };
type PendingPluginTurn = PluginTurnPayload | PluginLifecycleTurnPayload;

/** 🚪️ Only the captured lifecycle owner can dispatch retirement-authorized work. */
export async function runPluginLifecycleTurn(owner: ShardInstanceLifecycleLease, work: PluginLifecycleWork, budget: ShardBudget): Promise<PluginLifecycleTurnResult> {
  let raw: unknown;
  let submissions: readonly OwnedNativeUiPatchSubmissionReceipt[] | null = null;
  switch (work.kind) {
    case "open": raw = await owner.open(work.input, budget); break;
    case "poll": raw = await owner.poll(budget); break;
    case "close": raw = await owner.close(budget); break;
    case "receipt-ack": raw = await owner.acknowledge(work.receipt, budget, work.retirement); break;
    case "issued-ui-acks": { const result = await owner.submitUiAcknowledgements(work.entries, budget); raw = result.result; submissions = result.receipts; break; }
    default: throw new Error("actor-lifecycle.work-kind");
  }
  return Object.freeze({ owner, raw, turn: coerceTurnResult(raw), submissions });
}

/** 🧮️ Matches `ActivationRegistry`'s own `DEFAULT_TURN_MAILBOX_CAPACITY` — no reason for this file's
 * per-actor turn queue to be shaped differently from the kernel's own default. */
export const PLUGIN_TURN_MAILBOX_CAPACITY = 32;

export const pendingCoalescedTurns = new Map<string, PluginTurnPayload>();
export const pendingLifecycleTurns = new Map<string, number>();
export const tearingDownPluginActors = new Set<string>();

/** 🧾️ [TRACE] Crossing census — every worker round trip this host posts, classified by the events it
 * carried and by what came back, so "172 `message` crossings carrying nothing" becomes a taxonomy
 * instead of one bucket. Read off `globalThis.__semioCrossingCensus` by `🐍️react-hop-cost-probe.mjs`. */
type CrossingCensusRowV1 = { crossings: number; events: number; patches: number; effects: number; width: Record<number, number> };
const crossingCensusV1 = new Map<string, CrossingCensusRowV1>();

const typedOpAckCensusV1 = new Map<string, { acks: number; minSequence: number; maxSequence: number }>();

function messageCrossingClassV1(event: ShardEventEnvelope): string {
  const payload = event.payload as { readonly source?: { readonly tag?: string; readonly val?: string }; readonly payload?: readonly number[] } | undefined;
  const tag = payload?.source?.tag ?? "?";
  const bytes = payload?.payload;
  if (tag === "shell" && Array.isArray(bytes) && TYPED_OPERATION_ACK_MAGIC.every((byte, index) => bytes[index] === byte)) {
    const token = new DataView(Uint8Array.from(bytes.slice(TYPED_OPERATION_ACK_MAGIC.length)).buffer);
    const operation = String(token.getBigUint64(4, true));
    const sequence = token.getUint32(20, true);
    const row = typedOpAckCensusV1.get(operation) ?? { acks: 0, minSequence: sequence, maxSequence: sequence };
    row.acks += 1;
    row.minSequence = Math.min(row.minSequence, sequence);
    row.maxSequence = Math.max(row.maxSequence, sequence);
    typedOpAckCensusV1.set(operation, row);
    return "message.typed-op-ack";
  }
  return `message.${tag}`;
}

function crossingCensusClassV1(events: readonly ShardEventEnvelope[]): string {
  if (events.length === 0) return "(none)";
  const kinds = new Set(events.map((event) => (event.kind === "message" ? messageCrossingClassV1(event) : event.kind)));
  return [...kinds].sort().join("+");
}

function noteCrossingCensusV1(events: readonly ShardEventEnvelope[], result: WireTurnResult | null): void {
  const key = crossingCensusClassV1(events);
  const row = crossingCensusV1.get(key) ?? { crossings: 0, events: 0, patches: 0, effects: 0, width: {} };
  row.crossings += 1;
  row.events += events.length;
  row.patches += result?.uiPatches.length ?? 0;
  row.effects += result?.effects.length ?? 0;
  row.width[events.length] = (row.width[events.length] ?? 0) + 1;
  crossingCensusV1.set(key, row);
}

Object.defineProperty(globalThis, "__semioCrossingCensus", { configurable: true, get: () => Object.fromEntries(crossingCensusV1), set: () => {} });
Object.defineProperty(globalThis, "__semioTypedOpAckCensus", { configurable: true, get: () => Object.fromEntries(typedOpAckCensusV1), set: () => {} });

export let sharedPluginTurnScheduler: TurnScheduler<PendingPluginTurn, ShardBudget> | null = null;
/** 🧵️ Reads `getShardClient()` INSIDE `runTurn` (not once at construction) purely so a test can swap
 * the module-private `sharedShardClient` for a fake between calls without this scheduler ever pinning
 * itself to whichever shard client happened to exist first — in production there is only ever one. */
export function getPluginTurnScheduler(): TurnScheduler<PendingPluginTurn, ShardBudget> {
  sharedPluginTurnScheduler ??= new TurnScheduler<PendingPluginTurn, ShardBudget>({
    mailboxCapacity: PLUGIN_TURN_MAILBOX_CAPACITY,
    budgetFor: () => DEFAULT_SHARD_BUDGET,
    runTurn: async (actorId, payload, budget) => {
      if (payload.kind === "lifecycle") releasePendingLifecycleTurn(actorId);
      if (payload.kind === "operation" && payload.coalesceMapKey) pendingCoalescedTurns.delete(payload.coalesceMapKey);
      try {
        if (payload.kind === "lifecycle") {
          const settled = await runPluginLifecycleTurn(payload.owner, payload.work, budget);
          noteCrossingCensusV1(payload.work.kind === "issued-ui-acks" ? payload.work.entries.map(() => ({ kind: "patch-ack", payload: null })) : [{ kind: `lifecycle.${payload.work.kind}`, payload: null }], settled.turn);
          payload.resolve(settled);
          return;
        }
        payload.activation?.assertActive();
        const raw = await (payload.activation ? payload.activation.turn(payload.events, budget, payload.commandPage) : getShardClient().turn(actorId, payload.events, budget, payload.commandPage));
        payload.activation?.assertActive();
        const result = coerceTurnResult(raw);
        noteCrossingCensusV1(payload.commandPage ? [...payload.events, { kind: "command-page", payload: null }] : payload.events, result);
        for (const waiter of payload.waiters) waiter.resolve(result);
      } catch (error) {
        if (payload.kind === "lifecycle") payload.reject(error);
        else for (const waiter of payload.waiters) waiter.reject(error);
        if (!tearingDownPluginActors.has(actorId)) throw error;
      }
    },
    onTurnError: (actorId, error) => undefined,
  });
  return sharedPluginTurnScheduler;
}

export function releasePendingLifecycleTurn(actorId: string): void {
  const remaining = (pendingLifecycleTurns.get(actorId) ?? 1) - 1;
  if (remaining === 0) pendingLifecycleTurns.delete(actorId); else pendingLifecycleTurns.set(actorId, remaining);
}

export function enqueuePluginTurn(actorId: string, payload: PendingPluginTurn, lane: Lane, coalesce?: string): ReturnType<TurnScheduler<PendingPluginTurn, ShardBudget>["enqueue"]> {
  const scheduler = getPluginTurnScheduler();
  if ((payload.kind === "lifecycle" || pendingLifecycleTurns.has(actorId)) && scheduler.pendingCount(actorId) >= PLUGIN_TURN_MAILBOX_CAPACITY) return { kind: "rejected" };
  const backpressure = scheduler.enqueue(actorId, { lane, coalesce, payload });
  if (payload.kind === "lifecycle" && backpressure.kind !== "rejected") pendingLifecycleTurns.set(actorId, (pendingLifecycleTurns.get(actorId) ?? 0) + 1);
  return backpressure;
}

/**
 * 🚦 This file's own internal turn-dispatch seam — replaces the old unbounded `actorTurnQueue` chain.
 * `lane` prioritizes across an actor's own pending turns (`"Interactive"` for anything a caller awaits
 * a specific reply from — `runQueuedTurn`/`createApp`/`captureExtensionCompletion` all use it below —
 * `"UserVisible"` for {@link loadPluginModule}'s opportunistic `refreshUi` probe, so a real command
 * always preempts a mere redraw poll). `turnKey`, when passed, collapses a burst of same-key calls
 * for the SAME actor into the single latest one — every caller in the burst (not just the winner) still
 * gets the SAME final result, via {@link PluginTurnPayload.waiters} rather than the mailbox's own
 * envelope-replacement (which has no callback for the superseded call).
 *
 * 🚧️ Honest gap: `BoundedMailbox.enqueue`'s `dropped` backpressure (this actor's mailbox at capacity,
 * evicting the lowest-priority NONEMPTY lane below the incoming one) has no callback for the evicted
 * envelope — a caller whose turn was silently evicted this way would never see its promise settle.
 * Mitigated, not eliminated, by lane discipline: every call site below uses only two lanes
 * (`"Interactive"`/`"UserVisible"`), and `"UserVisible"` traffic is deduplicated to at most one pending
 * envelope per actor by `pendingCoalescedTurns` before it ever reaches the mailbox — so an eviction can
 * only happen if a single actor accumulates more than `PLUGIN_TURN_MAILBOX_CAPACITY` (32) genuinely
 * distinct `"Interactive"` turns, which no real call site here does. Flagged rather than silently
 * risked, per this repo's own "must not assume" rule — see `📓️terra-web-plugin-runtime-report.md`
 * `## honest gaps`.
 */
export function submitPluginTurn(actorId: string, events: readonly ShardEventEnvelope[], lane: Lane, turnKey?: string, commandPage?: ShardCommandIngressPage, activation?: ShardActorActivationLease): Promise<WireTurnResult> {
  return new Promise<WireTurnResult>((resolve, reject) => {
    if (activation && (activation.actorId !== actorId || turnKey !== undefined)) throw new Error("actor-activation.turn-owner-mismatch");
    activation?.assertActive();
    const waiter: PluginTurnWaiter = { resolve, reject };
    if (turnKey !== undefined) {
      const mapKey = `${actorId} ${turnKey}`;
      const pending = pendingCoalescedTurns.get(mapKey);
      if (pending) {
        pending.events = events;
        pending.waiters.push(waiter);
        return;
      }
      const payload: PluginTurnPayload = { kind: "operation", events, waiters: [waiter], coalesceMapKey: mapKey, commandPage };
      pendingCoalescedTurns.set(mapKey, payload);
      const backpressure = enqueuePluginTurn(actorId, payload, lane, turnKey);
      if (backpressure.kind === "rejected") {
        pendingCoalescedTurns.delete(mapKey);
        reject(new Error(`PluginRuntime: actor ${actorId}'s turn queue is full — rejected rather than growing unbounded`));
      }
      return;
    }
    const payload: PluginTurnPayload = { kind: "operation", events, waiters: [waiter], commandPage, activation };
    const backpressure = enqueuePluginTurn(actorId, payload, lane);
    if (backpressure.kind === "rejected") reject(new Error(`PluginRuntime: actor ${actorId}'s turn queue is full — rejected rather than growing unbounded`));
  });
}

/** 📨️ Lifecycle work shares actor serialization without borrowing revoked command authority. */
export function submitPluginLifecycleTurn(owner: ShardInstanceLifecycleLease, work: PluginLifecycleWork, lane: Lane): Promise<PluginLifecycleTurnResult> {
  return new Promise((resolve, reject) => {
    let captured: PluginLifecycleWork;
    switch (work.kind) {
      case "open": captured = Object.freeze({ kind: work.kind, input: work.input }); break;
      case "poll": case "close": captured = Object.freeze({ kind: work.kind }); break;
      case "receipt-ack": captured = Object.freeze({ kind: work.kind, receipt: work.receipt, retirement: work.retirement }); break;
      case "issued-ui-acks": captured = Object.freeze({ kind: work.kind, entries: work.entries }); break;
      default: throw new Error("actor-lifecycle.work-kind");
    }
    const actorId = owner.activation.actorId;
    const payload: PluginLifecycleTurnPayload = { kind: "lifecycle", owner, work: captured, resolve, reject };
    const backpressure = enqueuePluginTurn(actorId, payload, lane);
    if (backpressure.kind === "rejected") reject(new Error("actor-lifecycle.queue-full"));
  });
}

export function teardownPluginActor(actorId: string): void {
  const fault = new Error(`plugin actor ${actorId} disposed`);
  tearingDownPluginActors.add(actorId);
  getPluginTurnScheduler().teardownActor(actorId, (payload) => {
    if (payload.kind === "lifecycle") { releasePendingLifecycleTurn(actorId); payload.reject(fault); return; }
    if (payload.coalesceMapKey) pendingCoalescedTurns.delete(payload.coalesceMapKey);
    for (const waiter of payload.waiters) waiter.reject(fault);
  });
  getThunkScheduler().teardownActor(`command-ingress:${actorId}`, (payload) => payload.reject(fault));
  typedOperationRoutersByActor.get(actorId)?.closeAll();
  typedOperationRoutersByActor.delete(actorId);
  hostContinuations.schedule(() => tearingDownPluginActors.delete(actorId), 0);
}
//#endregion 🔖️PluginTurnScheduler
//#endregion 🔖️ActorAdapter
//#endregion 🔖️plugin-runtime
