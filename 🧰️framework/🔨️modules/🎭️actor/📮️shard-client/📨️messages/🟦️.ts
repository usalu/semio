import type { OriginalOperationAdmission, OriginalOperationReceipt } from "../../🎟️retained-turn/🟦️.ts";
import type { ActorCodecRequest } from "../../🧬️codec/🟦️.ts";
import { hopTrace, hopTraceEpochToTimeline } from "../../../⏱️trace/🟦️.ts";
import type { ShardAsset, ShardBudget, ShardCapabilityGrant, ShardCommandIngressPage, ShardEventEnvelope, ShardFrame, ShardJobBudget } from "../📜️wire/🟦️.ts";

export type OutboundMessage =
  | { readonly kind: "activate"; readonly requestId: string; readonly actorId: string; readonly activationGeneration: bigint; readonly originalAdmission: OriginalOperationAdmission; readonly moduleUrl: string; readonly caps: readonly ShardCapabilityGrant[]; readonly budget: ShardBudget; readonly assets: readonly ShardAsset[] }
  | { readonly kind: "turn"; readonly requestId: string; readonly actorId: string; readonly activationGeneration: bigint; readonly events: readonly ShardEventEnvelope[]; readonly commandPage?: ShardCommandIngressPage; readonly budget: ShardBudget; readonly postedAtEpochMs?: number }
  | { readonly kind: "startJob"; readonly requestId: string; readonly actorId: string; readonly job: bigint; readonly jobKind: string; readonly input: Uint8Array }
  | { readonly kind: "stepJob"; readonly requestId: string; readonly actorId: string; readonly job: bigint; readonly budget: ShardJobBudget }
  | { readonly kind: "cancelJob"; readonly requestId: string; readonly actorId: string; readonly job: bigint }
  | { readonly kind: "takeSegmentedDownloadChunk"; readonly requestId: string; readonly actorId: string; readonly instanceId: number; readonly operationId: bigint }
  | { readonly kind: "checkpoint"; readonly requestId: string; readonly actorId: string }
  | { readonly kind: "restore"; readonly requestId: string; readonly actorId: string; readonly state: Uint8Array }
  | { readonly kind: "codec"; readonly requestId: string; readonly actorId: string; readonly activationGeneration: bigint; readonly request: ActorCodecRequest }
  | { readonly kind: "dispose"; readonly actorId: string; readonly activationGeneration: bigint }
  /** 📨️ terra-web-shardframe: the ONE new wire message every {@link ShardFrame} variant travels over —
   * additive alongside `"activate"`/`"turn"`/etc above, none of which this message kind replaces or
   * changes. `actorId` is carried alongside `frame` (rather than requiring every handler to destructure
   * it back out of `frame`) purely so `send()`'s existing `"actorId" in message` pending-entry bookkeeping
   * keeps working unmodified for this kind too. */
  | { readonly kind: "frame"; readonly requestId: string; readonly actorId: string; readonly activationGeneration: bigint; readonly frame: ShardFrame };

/** ⏱️ What the generated `🟨️shard-worker.js` says about its OWN side of one turn, on the shared
 * epoch clock (`hopTraceEpochNowMs`): when the host posted, when the handler entered, when the guest
 * was entered and left, and when the reply was posted. Every field is a pure accounting read — a
 * missing or malformed one publishes no span and changes nothing about the turn. */
type ShardWorkerTurnTimings = Readonly<{
  readonly postedAtEpochMs: number | null;
  readonly receivedAtEpochMs: number;
  readonly guestEnteredAtEpochMs: number | null;
  readonly guestLeftAtEpochMs: number | null;
  readonly repliedAtEpochMs: number | null;
  readonly events: number;
  readonly eventKinds: string;
  readonly previousReplyCloneMs: number;
  readonly patches: number;
  readonly status?: string;
  readonly commandPageBytes: number;
  /** 🚚️ How many guest turns the WORKER ran inside this one crossing, out of the ceiling it derived
   * from the granted wall, and why it stopped (`carried`/`idle`/`input`/`steps`/`budget`/`closed`).
   * Diagnostic: it is what makes "the worker owns the MoreWork drive" a reading rather than an
   * inference from `worker.guest`'s mean. */
  readonly drivePolls?: number;
  readonly driveSteps?: number;
  readonly driveStopped?: string;
}>;

/** 📮️ Places the worker's four stages on the PAGE's own hop timeline, where `channel`, `refresh.turn`
 * and every other stage already live, so one probe read returns a breakdown that adds up:
 * `worker.receive + worker.decode + worker.guest + worker.reply` is exactly the host-observed
 * crossing (post → answer), and `worker.turn` is the worker's own busy span inside it.
 *
 * 🩺️ This is the split `📓️react-hop-latency-2026-09-14.md` §10 had to leave unmeasured — the CDP
 * `Performance` domain answers nothing on a shard-worker target, so the only instrument that can see
 * it is one the worker carries itself. Accounting only: a malformed timings record publishes nothing.
 */
export function publishShardWorkerTurnSpans(timings: ShardWorkerTurnTimings, answeredAtEpochMs: number, actorId: string): void {
  const finite = (value: unknown): value is number => typeof value === "number" && Number.isFinite(value);
  if (!finite(timings.receivedAtEpochMs)) return;
  const detail = { actorId, events: finite(timings.events) ? timings.events : 0, eventKinds: typeof timings.eventKinds === "string" ? timings.eventKinds : "", replyCloneMs: finite(timings.previousReplyCloneMs) ? Math.round(timings.previousReplyCloneMs * 100) / 100 : -1, patches: finite(timings.patches) ? timings.patches : 0, status: typeof timings.status === "string" ? timings.status : "", commandPageBytes: finite(timings.commandPageBytes) ? timings.commandPageBytes : 0, drivePolls: finite(timings.drivePolls) ? timings.drivePolls : -1, driveSteps: finite(timings.driveSteps) ? timings.driveSteps : -1, driveStopped: typeof timings.driveStopped === "string" ? timings.driveStopped : "" };
  const span = (stage: "worker.turn" | "worker.receive" | "worker.decode" | "worker.guest" | "worker.reply", fromEpochMs: unknown, toEpochMs: unknown): void => {
    if (!finite(fromEpochMs) || !finite(toEpochMs) || toEpochMs < fromEpochMs) return;
    hopTrace.record(stage, hopTraceEpochToTimeline(fromEpochMs), toEpochMs - fromEpochMs, detail);
  };
  span("worker.receive", timings.postedAtEpochMs, timings.receivedAtEpochMs);
  span("worker.decode", timings.receivedAtEpochMs, timings.guestEnteredAtEpochMs);
  span("worker.guest", timings.guestEnteredAtEpochMs, timings.guestLeftAtEpochMs);
  span("worker.reply", timings.guestLeftAtEpochMs, answeredAtEpochMs);
  span("worker.turn", timings.receivedAtEpochMs, timings.repliedAtEpochMs);
}

/** 🫀️ A liveness beat CARRIED by a message that was crossing anyway, instead of costing a
 * `postMessage` of its own.
 *
 * 🐛️ Every turn crossing used to cost three main-thread messages: a start-of-request `heartbeat`, a
 * `turn-step` `heartbeat`, and the `result` itself — on a main thread the hop measurement shows is the
 * binding constraint (`📓️reactor-reconcile-spin-2026-09-14.md` §7 item 3). Neither posted beat told
 * this class anything the reply does not: {@link ShardClient.noteLiveness} treats EVERY inbound
 * message as proof of life, and {@link evaluateShardLiveness} already counts the request's own start
 * instant as proven-alive. So the beat rides, and {@link ShardClient.handleMessage} folds it into the
 * same `recordHeartbeat` a dedicated `heartbeat` message reaches — the turn sequence stays monotone
 * and the phase still names the boundary. */
export type ShardCarriedBeat = { readonly turnSeq: number; readonly phase: string | null };

export type InboundMessage =
  | { readonly kind: "original-turn-receipt"; readonly actorId: string; readonly activationGeneration: bigint; readonly receipt: OriginalOperationReceipt }
  | { readonly kind: "original-producer-held"; readonly actorId: string; readonly activationGeneration: bigint; readonly operation: bigint; readonly generation: bigint; readonly refusal: unknown }
  | { readonly kind: "original-source-close"; readonly actorId: string; readonly activationGeneration: bigint; readonly operation: bigint; readonly generation: bigint; readonly sourceClosed: boolean }
  | { readonly kind: "result"; readonly requestId: string; readonly ok: true; readonly value: unknown; readonly timings?: ShardWorkerTurnTimings; readonly beat?: ShardCarriedBeat }
  | { readonly kind: "result"; readonly requestId: string; readonly ok: false; readonly error: string; readonly stack?: string; readonly type?: string; readonly framesBytes?: number; readonly retryableLifecycle?: boolean; readonly beat?: ShardCarriedBeat }
  /** 🫀️ `phase` names the generated worker's await boundary this beat was emitted at
   * (`module-fetch`/`module-ready`/`actor-ready`, or `progress` from its while-busy ticker); absent on
   * the unconditional start-of-request beat. Diagnostic only — {@link evaluateShardLiveness} treats
   * every beat identically, so a future phase needs no host change to keep a shard alive.
   *
   * A dedicated `heartbeat` message is now only for a beat with NO crossing to ride; the two beats a
   * turn crossing used to cost ride the reply instead, as {@link ShardCarriedBeat}. */
  | { readonly kind: "heartbeat"; readonly turnSeq: number; readonly phase?: string | null }
  | { readonly kind: "trap"; readonly actorId: string; readonly activationGeneration: bigint | null; readonly message: string }
  /** 🩺️ The worker's own account of a fault the host would otherwise see as an anonymous `ErrorEvent`
   * (or not at all): an exception escaping a message handler, an unhandled rejection, or a caught
   * handler error, tagged with the boundary it happened on (`load-bridge`/`instantiate`/`first-step`/
   * the request kind), the actor and the module URL. Carries no `requestId` — it is never an answer to
   * anything, so it is dispatched before the generic pending lookup, exactly like `"trap"`. */
  | { readonly kind: "worker-fault"; readonly source: string; readonly phase: string; readonly actorId: string | null; readonly moduleUrl: string | null; readonly message: string; readonly stack?: string; readonly filename?: string; readonly lineno?: number }
  /** 📨️ terra-shard-effect-bridge: the worker→kernel direction of `🟨️.js`'s `effectRequest`
   * (🧪️ terra-web-bridges) — an async host import the guest `.await`ed. Reuses `ShardFrame`'s own
   * `Envelope` shape verbatim (`frame.envelope.payload` is `{kind:"effect-request", payload:{effect,
   * requestId, params}}`), never a second wire. Carries no `requestId` of its OWN at this outer level
   * (unlike every other inbound kind) — correlation lives inside `frame.envelope.payload.payload`. */
  | { readonly kind: "frame"; readonly actorId: string; readonly activationGeneration: bigint; readonly frame: ShardFrame };
