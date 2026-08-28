/** 🧵️ `ShardClient` — the web `ShardTransport` (design-runtime.md §1 `ShardTransport` /
 * §3 "Web shard"): a bounded pool of `🟨️shard-worker.js` Web Workers multiplexed by `actorId`,
 * replacing one-Worker-per-plugin (`PluginWorkerClient`, deleted from `🎠️kernel/🟦️component.ts` in
 * the same packet). V8 reserves a 4 GiB guard region per wasm module per worker, so
 * one-worker-per-plugin capped the browser at ~20 plugins; this pools K = `min(hardwareConcurrency-1,
 * 4)` (design's `ShardTable`) workers and pins actors onto them instead.
 *
 * 🚧 UPDATE (terra-web-shardframe): A1's `🤖️generated/🟦️actor.ts` has now landed clean (no more
 * un-typeable `object & string` intersections — `Lane`/`Envelope`/`Payload`/`Origin` are real types),
 * which is what makes the region below possible. `turn()`/`activate()` stay EXACTLY as they were —
 * opaque-to-this-module `ShardEventEnvelope[]` JSON in, `unknown` out, nothing broken — while
 * `📨️ShardFrame` below adds the Rust `ShardFrame` enum's SHAPE (`Register`/`Unregister`/`Grant`/
 * `Envelope`) as a NEW, additive wire alongside them: {@link ShardClient.grant}/{@link
 * ShardClient.envelope}. Deliberately shape-only, not byte-for-byte — see that region's own header
 * doc for the encoding decision (structured clone, no hand-rolled pack codec, on purpose, for now).
 */

//#region 🔌️WireTypes
/** ⚖️ `Lane`/`CoalesceKey` taken from the owned-schema mirror — real wire types, same reasoning
 * `📬️mailbox.ts`'s own header doc already gives for importing rather than redeclaring them. */
import type { Lane, CoalesceKey } from "../../🤖️generated/🟦️actor.ts";
import { actorInstanceCapturedReceiptMatches, actorInstanceCloseReceiptMatches, actorInstanceLifecycleReceiptEquals, actorInstanceLifetimeEquals, decodeActorInstanceLifecycle, encodeActorInstanceLifecycle, type ActorInstanceLifecycleReceipt, type ActorInstanceCloseRequest, type ActorInstanceOpenRequest, type ActorInstanceLifetime } from "../../🚪️lifetime/🟦️component.ts";
import { actorUiPatchReceiptEquals, decodeActorUiPatchReceipt, encodeActorUiPatchReceipt, validateActorUiPatchPairing, type ActorUiPatchReceipt } from "../../🚪️lifetime/🩹️patch/🟦️component.ts";
import { OwnedActorTurnOutputs, type OwnedActorTurnOutput } from "../../🪪️activation/🚪️instance/📥️output/🟦️component.ts";
import { ACTOR_BYTE_PAGE_BYTES, createActorBytePage, type ActorBytePage } from "../../📄️page/🟦️component.ts";
import { encodeActorReturnDrive, decodeActorReturnResult, type ActorReturnOrigin, type ActorReturnIdentity, type ActorReturnPageReceipt, type ActorReturnDrive, type ActorReturnResult } from "../../📤️return/🟦️component.ts";
export { encodeActorReturnDrive, decodeActorReturnDrive, encodeActorReturnResult, decodeActorReturnResult, ACTOR_RETURN_RESULT_MAXIMUM_BYTES, type ActorReturnOrigin, type ActorReturnIdentity, type ActorReturnPageReceipt, type ActorReturnControl, type ActorReturnDrive, type ActorReturnResult, type ActorReturnFault } from "../../📤️return/🟦️component.ts";
import { OwnedUiInstance, OwnedUiInstanceRetirement, OwnedUiPatchAcknowledgement, OwnedUiPatchInputAcceptance, OwnedUiPatchInputRetirement } from "../../../🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️component.ts";
import { OwnedKernelReturnContent } from "../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts";
import { OwnedResidentLedger, OwnedResidentRecordDetachment, OwnedResidentRetirement, type OwnedResidentRecord, type ResidentGrant, type ResidentStep } from "../../../🌱️value/💾️resident/🟦️component.ts";
import { OwnedUiResidentPool, OwnedUiResidentPoolRetirement } from "../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts";
import { uiResidentMetadataEnvelope } from "../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🟦️component.ts";
const residentCapacity = Object.getOwnPropertyDescriptor(OwnedResidentLedger.prototype, "capacity")!.get!;
const NO_RESIDENT_FAULT = Symbol("actor-resident.no-fault");
const poolUiEnvelope = uiResidentMetadataEnvelope("pool");
const poolRecordEnvelope = Object.freeze({ bytes: poolUiEnvelope.bytes + 144, slots: poolUiEnvelope.slots + 1, owners: poolUiEnvelope.owners + 1 });
const residentStep = (kind: ResidentStep["kind"], phase: string, bytes = 0): ResidentStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
//#endregion 🔌️WireTypes

//#region 🧬️Types
/** ⚖️ Stand-in for the generated mirror of Rust `semio_framework_actor::Budget` (same pattern as A1's own
 * `CapabilityGrant` stand-in — depending on the not-yet-emitted generated file would break every
 * consumer of this module until typegen lands). Field-for-field with design-runtime.md §1's `Budget`,
 * camelCased. */
export interface ShardBudget {
  readonly fuel: number;
  readonly wallMs: number;
  readonly memoryBytes: number;
  readonly uiNodes: number;
  readonly mailboxLen: number;
  readonly maxEffects: number;
  readonly maxPatchBytes: number;
}

/** ⚖️ Stand-in for `semio_framework_actor::JobBudget` (design-abi.md `jobs::job-budget`). */
export interface ShardJobBudget {
  readonly fuel: number;
  readonly deadlineMs: number;
}

/** ⚖️ Stand-in for the WIT `capabilities::capability-grant` record — opaque to the shard worker, just
 * forwarded into the guest's `instance-open` event. */
export interface ShardCapabilityGrant {
  readonly id: string;
  readonly token: string;
  readonly scope: string;
  readonly expiresMs: number | null;
}

/** 🚧 Interim, package-agnostic event envelope `🟨️shard-worker.js` decodes into the guest's WIT
 * `event` variant — see this module's header doc for why this isn't the real pack-encoded `Envelope`
 * yet. `kind` mirrors the WIT `event` variant's own tag name (kebab-case, e.g. `"app-command"`,
 * `"surface-visible"`, `"instance-open"`); `payload` is the variant's own record, JSON-shaped. */
export interface ShardEventEnvelope {
  readonly kind: string;
  readonly payload: unknown;
}

export interface ShardCommandPageCursor {
  readonly owner: bigint;
  readonly generation: bigint;
  readonly commandIndex: number;
  readonly commandCount: number;
  readonly instance: number;
  readonly seq: bigint;
  readonly kind: number;
  readonly pageIndex: number;
  readonly pageCount: number;
  readonly itemCount: number;
  readonly metadata: number;
}

export type ShardCommandIngressPage = { readonly cursor: ShardCommandPageCursor; readonly page: ActorBytePage };

export const SHARD_COMMAND_MAXIMUM_PAGES = 64;

/** 📥️ Encodes channel command bytes into the exact fixed WIT command-page authority shared with Rust. */
export function createShardCommandIngressPages(input: {
  readonly owner: bigint;
  readonly generation: bigint;
  readonly commandIndex: number;
  readonly commandCount: number;
  readonly instance: number;
  readonly seq: bigint;
  readonly command: Uint8Array;
}): readonly ShardCommandIngressPage[] {
  if (input.command.length === 0) throw new Error("[DEBUG] command ingress cannot encode an empty command");
  const pageCount = Math.ceil(input.command.length / ACTOR_BYTE_PAGE_BYTES);
  if (pageCount > SHARD_COMMAND_MAXIMUM_PAGES) throw new Error(`[DEBUG] command ingress exceeds ${SHARD_COMMAND_MAXIMUM_PAGES} pages`);
  const pages: ShardCommandIngressPage[] = [];
  for (let pageIndex = 0; pageIndex < pageCount; pageIndex += 1) {
    const start = pageIndex * ACTOR_BYTE_PAGE_BYTES;
    const bytes = input.command.subarray(start, Math.min(start + ACTOR_BYTE_PAGE_BYTES, input.command.length));
    pages.push({
      cursor: {
        owner: input.owner,
        generation: input.generation,
        commandIndex: input.commandIndex,
        commandCount: input.commandCount,
        instance: input.instance,
        seq: input.seq,
        kind: input.command[0]!,
        pageIndex,
        pageCount,
        itemCount: 0,
        metadata: 0,
      },
      page: createActorBytePage(bytes),
    });
  }
  return pages;
}

/** 🪶️ One named asset pack delivered on `instance-open` (design-runtime.md §3: `guestSlimAssets`
 * becomes a declared asset here rather than a worker-bootstrap special case) — `events::
 * instance-open-event.assets: list<tuple<string, pack>>`. `bytes` is transferred structured-clone
 * (never `Transferable`-detached: the SAME buffer is reused across every actor this shard pool
 * activates, see `🌐plugin-web-materialize.ts`'s `shardWorkerSource` doc). */
export type ShardAsset = readonly [name: string, bytes: ArrayBuffer];

export type ShardJobStep = { readonly status: "running"; readonly progress?: Uint8Array } | { readonly status: "done"; readonly value: Uint8Array } | { readonly status: "failed"; readonly value: Uint8Array };

/** ⚖️ Stand-in for the generated mirror of Rust `semio_framework_actor::ShardMetrics` (same "not-yet-
 * emitted `🤖️generated/🟦️actor.ts`" reason as `ShardBudget` above) — MICROKERNEL-POOLED-ACTOR-PLUGIN-
 * RUNTIME T1. Field-for-field with the Rust struct, camelCased. */
export interface ShardMetrics {
  readonly actors: number;
  readonly busyRatio: number;
  readonly heartbeatAgeMs: number;
}

/** ⚖️ Stand-in for `semio_framework_actor::ShardMetricsSample` — one row of the `os.runtime.metrics`
 * publication's shard table, as `ShardClient.shardMetricsSamples` can observe it purely from data this
 * class already owns (`actorIds`/`pendingRequestIds`/heartbeat state) — no wasm `Kernel` call needed. */
export interface ShardMetricsSample {
  readonly shard: number;
  readonly metrics: ShardMetrics;
}
//#endregion 🧬️Types

//#region 📨️ShardFrame
/** ⚖️ `semio_framework_actor::lane_defaults::budget_for(Lane::Maintenance)` (`🎭️actor/🦀️component.rs`,
 * `lane_defaults` module) — the ONLY floor a granted-less actor may fall back to, mirrored field-for-
 * field/value-for-value: `{ fuel: 80_000_000, wall_ms: 200, memory_bytes: 256 MiB, ui_nodes: 4_000,
 * mailbox_len: 1024, max_effects: 512, max_patch_bytes: 2_097_152 }`. {@link GrantedBudgetTracker}
 * falls back to this so a budget-less `Envelope` arriving before any `Grant` for its actor never
 * invents its own number — the same floor `ShardLoop::granted_budget` falls back to natively. */
export const MAINTENANCE_LANE_DEFAULT_BUDGET: ShardBudget = { fuel: 80_000_000, wallMs: 200, memoryBytes: 256 * 1024 * 1024, uiNodes: 4_000, mailboxLen: 1024, maxEffects: 512, maxPatchBytes: 2_097_152 };

/** ⚖️ Rust `Origin` mirror (`🎭️actor/🦀️component.rs`) — who sent a {@link ShardEnvelope}. `window`
 * stays a plain `number` (the generated mirror's own `WindowId`), matching this file's existing
 * convention of not importing bigint-carrying generated types across the `postMessage` boundary. */
export type ShardOrigin = { readonly kind: "ui"; readonly window: number } | { readonly kind: "actor"; readonly id: string } | { readonly kind: "kernel" } | { readonly kind: "bus"; readonly topic: string };

/** 🚧 terra-web-shardframe's own encoding decision (see this ticket's report, `## encoding decision`):
 * the Rust `Payload` this field would literally mirror is `{"kind":"event", bytes:[...]}` — an opaque
 * PACK-ENCODED blob no TS codec exists to decode yet (this file's header doc). Rather than carry
 * unusable opaque bytes, `payload` stays the interim, ALREADY-DECODED {@link ShardEventEnvelope} shape
 * `🟨️shard-worker.js` has executed since H2 — this packet adopts `ShardFrame`'s ENVELOPE-METADATA
 * shape (`to`/`from`/`lane`/`seq`/`deadlineMs`/`coalesce`/`cancelOf`) over structured clone, not its
 * byte codec. A future byte-level unification only needs to swap THIS field's type, once a real
 * pack-decode step exists on web (see the header doc's own "mechanical once it exists" note). */
export interface ShardEnvelope {
  readonly to: string;
  readonly from: ShardOrigin;
  readonly lane: Lane;
  readonly seq: number;
  readonly deadlineMs: number | null;
  readonly coalesce: CoalesceKey | null;
  readonly cancelOf: number | null;
  readonly payload: ShardEventEnvelope;
}

/** 📨️ TypeScript mirror of Rust `ShardFrame` (`🖥️host/🧵️shard/🦀️component.rs`) — see the in-source
 * parity test below (and this ticket's report, `## ShardFrame TS ↔ Rust variant table`) for the
 * enforcement that variant/field names stay in lockstep with that enum, read fresh off the Rust
 * source on every test run. `actor` is a plain `string` here (this class's own established id
 * vocabulary — see `ShardWorkerLike`'s header doc), not the generated mirror's bit-packed `ActorId`
 * bigint. Adopted in the order the Rust enum's own doc prescribes: `Envelope` passthrough (wraps
 * today's already-decoded turn payload; nothing existing breaks) lands with THIS packet alongside
 * `Grant` (a budget travels WITH the envelopes it grants, read by {@link ShardClient.grant}). */
export type ShardFrame =
  | { readonly kind: "Register"; readonly actor: string }
  | { readonly kind: "Unregister"; readonly actor: string }
  | { readonly kind: "Grant"; readonly actor: string; readonly budget: ShardBudget; readonly envelopes: readonly ShardEnvelope[] }
  | { readonly kind: "Envelope"; readonly envelope: ShardEnvelope };

/** 🧬️ Runtime twin of {@link ShardFrame}'s own field names — TS union types erase at runtime, so the
 * in-source parity test reads THIS array (not the type) to diff against the live Rust source. Keep in
 * lockstep with the union above by hand; the test fails loudly the moment either drifts from
 * `component.rs`. `Envelope`'s Rust variant is a TUPLE (`Envelope(Envelope)`, no Rust field name) —
 * `"envelope"` is this mirror's OWN naming choice for that lone position, not a Rust-sourced name. */
export const SHARD_FRAME_VARIANT_FIELDS: ReadonlyArray<{ readonly kind: ShardFrame["kind"]; readonly fields: readonly string[] }> = [
  { kind: "Register", fields: ["actor"] },
  { kind: "Unregister", fields: ["actor"] },
  { kind: "Grant", fields: ["actor", "budget", "envelopes"] },
  { kind: "Envelope", fields: ["envelope"] },
];

const SHARD_FRAME_LANE_ORDER: readonly Lane[] = ["Interactive", "UserVisible", "Background", "Maintenance"];

/** 🎯️ Stable sort by {@link Lane} priority — the SAME `LANE_ORDER` `🧵️turn-scheduler.ts`'s
 * `pickNextReadyActor` already applies ACROSS actors, applied here WITHIN one `Grant`'s own envelope
 * batch so dispatch order follows the grant's priorities, not push/arrival order. Stable: envelopes
 * tied on lane keep their relative order (mirrors `ShardLoop::pump`'s own "preserving arrival order"
 * per-actor grouping for same-lane envelopes). */
export function orderEnvelopesByLane(envelopes: readonly ShardEnvelope[]): readonly ShardEnvelope[] {
  return envelopes
    .map((envelope, index) => ({ envelope, index }))
    .sort((left, right) => {
      const rank = SHARD_FRAME_LANE_ORDER.indexOf(left.envelope.lane) - SHARD_FRAME_LANE_ORDER.indexOf(right.envelope.lane);
      return rank !== 0 ? rank : left.index - right.index;
    })
    .map((entry) => entry.envelope);
}

/** ⚖️ TypeScript twin of `ShardLoop`'s own `granted_budgets: HashMap<u64, Budget>` +
 * `granted_budget()` (`🖥️host/🧵️shard/🦀️component.rs`) — remembers the LAST `ShardFrame::Grant`
 * budget per actor so a later budget-less `Envelope` frame for the same actor runs under it instead of
 * any caller-cached constant. An actor never granted at all resolves to
 * {@link MAINTENANCE_LANE_DEFAULT_BUDGET} — the same documented floor the Rust side uses, never an
 * invented number. */
export interface GrantedBudgetTracker {
  recordGrant(actorId: string, budget: ShardBudget): void;
  forget(actorId: string): void;
  granted(actorId: string): ShardBudget;
}

export function createGrantedBudgetTracker(fallback: ShardBudget = MAINTENANCE_LANE_DEFAULT_BUDGET): GrantedBudgetTracker {
  const budgets = new Map<string, ShardBudget>();
  return {
    recordGrant(actorId, budget) {
      budgets.set(actorId, budget);
    },
    forget(actorId) {
      budgets.delete(actorId);
    },
    granted(actorId) {
      return budgets.get(actorId) ?? fallback;
    },
  };
}

/** 📤️ What interpreting one {@link ShardFrame} resolves to. */
export type ShardFrameDispatch =
  | { readonly action: "register" | "unregister"; readonly actor: string }
  | { readonly action: "runEnvelopes"; readonly actor: string; readonly budget: ShardBudget; readonly envelopes: readonly ShardEnvelope[] }
  | { readonly action: "unknown"; readonly frame: unknown };

/** 🧠️ Mirrors `ShardLoop::pump`'s per-frame dispatch: `Grant` records its budget (via `tracker`) and
 * hands back its envelopes IN LANE-PRIORITY ORDER; a budget-less `Envelope` resolves its actor's LAST
 * granted budget (never a fixed constant — see {@link GrantedBudgetTracker}); `Register`/`Unregister`
 * are pure bookkeeping (`Register` has no local instantiation side effect, matching that variant's own
 * Rust doc); a frame kind this file has never heard of resolves to `"unknown"` rather than throwing,
 * so a future Rust-side `ShardFrame` variant can reach a caller before its TS mirror lands without
 * wedging it. `🟨️shard-worker.js`'s own `"frame"` message handler (`plugin-web-materialize.ts`'s
 * `shardWorkerSource`) is a hand-transcribed mirror of exactly this function — a template-string
 * worker body cannot `import` it, so the logic is duplicated-by-necessity, not by choice; exercising
 * THIS function is how the in-source tests below prove the budget-threading/lane-ordering/forward-
 * compat behavior without spinning up a real `Worker`. */
export function interpretShardFrame(frame: ShardFrame, tracker: GrantedBudgetTracker): ShardFrameDispatch {
  switch (frame.kind) {
    case "Register":
      return { action: "register", actor: frame.actor };
    case "Unregister":
      tracker.forget(frame.actor);
      return { action: "unregister", actor: frame.actor };
    case "Grant":
      tracker.recordGrant(frame.actor, frame.budget);
      return { action: "runEnvelopes", actor: frame.actor, budget: frame.budget, envelopes: orderEnvelopesByLane(frame.envelopes) };
    case "Envelope":
      return { action: "runEnvelopes", actor: frame.envelope.to, budget: tracker.granted(frame.envelope.to), envelopes: [frame.envelope] };
    default:
      return { action: "unknown", frame };
  }
}
//#endregion 📨️ShardFrame

//#region 🌉️HostEffect
/** ⚖️ Stand-in for the generated mirror of Rust `semio_framework_kernel::QuotaBreach` (same "hand-mirrored,
 * not-yet-emitted generated type" pattern as {@link ShardBudget} above) — describes ONE outstanding-
 * effects cap breach, mirroring `QuotaBreach { quota, limit, actual }` field-for-field rather than
 * inventing a parallel vocabulary. `design-abi.md`'s `QuotaSchema.outstanding_requests` is the host-side
 * analog this client-side cap protects independently of — a guest's granted budget is enforced
 * host-side too, but this cap exists so `ShardClient` itself never queues unbounded concurrent
 * host-effect handler invocations regardless of what the host later decides. */
export interface ShardQuotaBreach {
  readonly quota: string;
  readonly limit: number;
  readonly actual: number;
}

function formatQuotaBreachMessage(breach: ShardQuotaBreach): string {
  return `outstanding effect quota exceeded: ${breach.quota} limit=${breach.limit} actual=${breach.actual}`;
}

/** 🌉️ What `ShardClient` calls for every `"effect-request"` frame a worker posts up
 * (`🟨️host-shim.js`'s `effectRequest` — 🧪️ terra-web-bridges) — the ONE seam `http-fetch`/`blob-read`/
 * `storage-read`/… etc actually resolve through. `ShardClient` implements NONE of these itself (`🎭️actor`
 * stays free of `web_sys`/host assumptions per this ticket's naming-hazards rule) — the React host, the
 * wgpu host, and tests each supply their own. `signal` aborts when the owning shard is lost
 * (`terminate`/watchdog rebuild) or the actor is `dispose`d, so a real fetch-backed handler can hand it
 * straight to `fetch(url, { signal })` and genuinely cancel a dead actor's in-flight network request
 * rather than merely forgetting it. Resolve with the effect's success value; reject (throw) to signal
 * failure — the rejection's `message` becomes the guest's `effect-error` `.message`. */
export type HostEffectHandler = (actorId: string, effect: string, params: unknown, signal: AbortSignal) => Promise<unknown>;
//#endregion 🌉️HostEffect

//#region 🌉️WorkerLike
/** 🌉️ The slice of `Worker` `ShardClient` depends on — lets tests (and any non-browser host) inject a
 * fake without a real `Worker`/`MessagePort`. A real browser `Worker` satisfies this structurally. */
export interface ShardWorkerLike {
  postMessage(message: unknown, transfer?: readonly Transferable[]): void;
  terminate(): void;
  onmessage: ((event: { readonly data: unknown }) => void) | null;
  onerror: ((event: unknown) => void) | null;
}

export type CreateShardWorker = (shardIndex: number) => ShardWorkerLike;
const MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES = 4_096;
const MAX_SEGMENTED_DOWNLOAD_OPERATION_ID = (1n << 64n) - 1n;
//#endregion 🌉️WorkerLike

//#region 📨️WireMessages
type OutboundMessage =
  | { readonly kind: "activate"; readonly requestId: string; readonly actorId: string; readonly activationGeneration: bigint; readonly moduleUrl: string; readonly caps: readonly ShardCapabilityGrant[]; readonly budget: ShardBudget; readonly assets: readonly ShardAsset[] }
  | { readonly kind: "turn"; readonly requestId: string; readonly actorId: string; readonly activationGeneration: bigint; readonly events: readonly ShardEventEnvelope[]; readonly commandPage?: ShardCommandIngressPage; readonly budget: ShardBudget }
  | { readonly kind: "startJob"; readonly requestId: string; readonly actorId: string; readonly job: number; readonly jobKind: string; readonly input: Uint8Array }
  | { readonly kind: "stepJob"; readonly requestId: string; readonly actorId: string; readonly job: number; readonly budget: ShardJobBudget }
  | { readonly kind: "cancelJob"; readonly actorId: string; readonly job: number }
  | { readonly kind: "takeSegmentedDownloadChunk"; readonly requestId: string; readonly actorId: string; readonly instanceId: number; readonly operationId: bigint }
  | { readonly kind: "checkpoint"; readonly requestId: string; readonly actorId: string }
  | { readonly kind: "restore"; readonly requestId: string; readonly actorId: string; readonly state: Uint8Array }
  | { readonly kind: "dispose"; readonly actorId: string; readonly activationGeneration: bigint }
  /** 📨️ terra-web-shardframe: the ONE new wire message every {@link ShardFrame} variant travels over —
   * additive alongside `"activate"`/`"turn"`/etc above, none of which this message kind replaces or
   * changes. `actorId` is carried alongside `frame` (rather than requiring every handler to destructure
   * it back out of `frame`) purely so `send()`'s existing `"actorId" in message` pending-entry bookkeeping
   * keeps working unmodified for this kind too. */
  | { readonly kind: "frame"; readonly requestId: string; readonly actorId: string; readonly activationGeneration: bigint; readonly frame: ShardFrame };

type InboundMessage =
  | { readonly kind: "result"; readonly requestId: string; readonly ok: true; readonly value: unknown }
  | { readonly kind: "result"; readonly requestId: string; readonly ok: false; readonly error: string; readonly stack?: string; readonly type?: string; readonly framesBytes?: number }
  | { readonly kind: "heartbeat"; readonly turnSeq: number }
  | { readonly kind: "trap"; readonly actorId: string; readonly activationGeneration: bigint | null; readonly message: string }
  /** 📨️ terra-shard-effect-bridge: the worker→kernel direction of `🟨️host-shim.js`'s `effectRequest`
   * (🧪️ terra-web-bridges) — an async host import the guest `.await`ed. Reuses `ShardFrame`'s own
   * `Envelope` shape verbatim (`frame.envelope.payload` is `{kind:"effect-request", payload:{effect,
   * requestId, params}}`), never a second wire. Carries no `requestId` of its OWN at this outer level
   * (unlike every other inbound kind) — correlation lives inside `frame.envelope.payload.payload`. */
  | { readonly kind: "frame"; readonly actorId: string; readonly activationGeneration: bigint; readonly frame: ShardFrame };
//#endregion 📨️WireMessages

//#region ⏱️Heartbeat
const DEFAULT_HEARTBEAT_TIMEOUT_MS = 5000;
const HEARTBEAT_MISSED_LIMIT = 3;
/** 🚦️ terra-shard-effect-bridge: default cap on CONCURRENT unresolved `effect-request`s per actor —
 * see {@link ShardClientOptions.maxOutstandingEffectsPerActor}'s own doc for why this mirrors
 * `QuotaSchema.outstanding_requests` without being it. */
const DEFAULT_MAX_OUTSTANDING_EFFECTS_PER_ACTOR = 64;

type ShardHeartbeatState = {
  lastHeartbeatAtMs: number;
  lastHeartbeatTurnSeq: number;
  oldestPendingStartedAtMs: number | null;
  missedCount: number;
  lastMissCountedAtMs: number;
};

/** A freshly spawned shard has never actually heartbeated yet — `-Infinity` (never `nowMs`) so a turn
 * that starts in the very same tick as `spawnShard` doesn't spuriously count spawn-time as proof of
 * life for that turn's whole timeout window. */
function freshHeartbeatState(nowMs: number): ShardHeartbeatState {
  return { lastHeartbeatAtMs: Number.NEGATIVE_INFINITY, lastHeartbeatTurnSeq: 0, oldestPendingStartedAtMs: null, missedCount: 0, lastMissCountedAtMs: nowMs };
}
//#endregion ⏱️Heartbeat

//#region 🧵️ShardClient
/** 🩺️ Rebuilds a worker-side failure as a main-thread `Error` that still carries the worker's own
 * stack. Without this the only frame a caller ever sees is `handleMessage`, because the structured
 * clone across `postMessage` cannot carry an `Error` — which is exactly why the collaboration e2e's
 * `Maximum call stack size exceeded` was undiagnosable. The `[DEBUG] ` line is deliberate, permanent
 * diagnostic infrastructure the e2e log parses; it is not leftover scaffolding. */
function graftWorkerStack(actorId: string, reason: string, stack: string | undefined, kind: string | undefined, framesBytes: number | undefined): Error {
  const error = new Error(reason);
  if (stack) error.stack = `${stack}\n    \u21b3 main: ${error.stack ?? ""}`;
  console.log(`[DEBUG] program worker ${actorId || "unknown"} error type=${kind ?? "unknown"} framesBytes=${framesBytes ?? "n/a"}`);
  return error;
}

type PendingEntry = { readonly resolve: (value: unknown) => void; readonly reject: (error: unknown) => void; readonly slot: ShardSlot; readonly startedAtMs: number; readonly actorId: string; readonly output: OwnedActorTurnOutput | null };

type ShardSlot = {
  index: number;
  worker: ShardWorkerLike;
  available: boolean;
  readonly heartbeat: ShardHeartbeatState;
  readonly pendingRequestIds: Set<string>;
  readonly actorIds: Set<string>;
};

/** 🧾️ Supplies the authored open payload without permitting arbitrary event submission. */
export interface ShardInstanceOpenInput {
  readonly appId: string;
  readonly actor: unknown;
  readonly config: Uint8Array | readonly number[];
  readonly assets: readonly ShardAsset[];
  readonly capabilities: readonly ShardCapabilityGrant[];
  readonly quotas: Uint8Array | readonly number[];
}

/** 🚪️ Retains lifecycle authority independently of revocable operation admission. */
export interface ShardInstanceLifecycleLease {
  readonly activation: ShardActorActivationLease;
  readonly openRequest: ActorInstanceOpenRequest;
  readonly lifetime: ActorInstanceLifetime | null;
  readonly pendingReceipt: ActorInstanceLifecycleReceipt | null;
  readonly interruptedTurn: unknown;
  readonly pendingReturn: OwnedShardReturn | null;
  reserveReturn(maximumResponses: number): OwnedShardReturn;
  open(input: ShardInstanceOpenInput, budget: ShardBudget): Promise<unknown>;
  poll(budget: ShardBudget): Promise<unknown>;
  beginClose(): ActorInstanceCloseRequest;
  close(budget: ShardBudget): Promise<unknown>;
  acknowledge(receipt: ActorInstanceLifecycleReceipt, budget: ShardBudget, retirement?: OwnedUiInstanceRetirement): Promise<unknown>;
  bindHostRetirement(participant: OwnedUiInstance): void;
  captureUiPatchAuthority(originalTurn: object, patchIndex: number): OwnedNativeUiPatchAuthority;
  submitUiAcknowledgement(source: OwnedNativeUiPatchAuthority, token: OwnedUiPatchAcknowledgement, budget: ShardBudget): Promise<{ readonly receipt: OwnedNativeUiPatchSubmissionReceipt; readonly result: unknown }>;
  dispose(): void;
  progress(): { readonly kind: ShardInstancePhase | "blocked"; readonly failure: ShardInstanceFailure | null };
}

/** 🪪️ Pins operation admission to one activation and worker; revocation does not release its close owner. */
export interface ShardActorActivationLease {
  readonly actorId: string;
  readonly activationGeneration: bigint;
  assertActive(): void;
  turn(events: readonly ShardEventEnvelope[], budget: ShardBudget, commandPage?: ShardCommandIngressPage): Promise<unknown>;
}

type ShardActivation = { readonly slot: ShardSlot; readonly actorId: string; readonly generation: bigint; available: boolean; activated: boolean; teardownPosted: boolean; operationsAllowed: boolean; operationGeneration: bigint; lastGuestLifetime: bigint; lastReturnSequence: bigint; returned: CapturedReturn | null; instance: ShardInstanceOwner | null; close: ShardInstanceOwner | null };
type PendingHostEffect = { readonly activation: ShardActivation; readonly controller: AbortController; readonly requestId: string; previous: PendingHostEffect | null; next: PendingHostEffect | null };
type HostEffectLedger = { readonly activation: ShardActivation; readonly requests: Map<string, PendingHostEffect>; head: PendingHostEffect | null; tail: PendingHostEffect | null };
type ShardInstancePhase = "opening" | "captured" | "open" | "closing" | "accepted" | "retired" | "complete";
type ShardInstanceFailure = "transport-refused" | "worker-refused" | "worker-lost" | "invalid-receipt";
type ShardInstanceOwner = {
  readonly activation: ShardActivation;
  readonly operation: ShardActorActivationLease;
  readonly open: ActorInstanceOpenRequest;
  phase: ShardInstancePhase;
  lifetime: ActorInstanceLifetime | null;
  receipt: ActorInstanceLifecycleReceipt | null;
  accepted: ActorInstanceLifecycleReceipt | null;
  close: ActorInstanceCloseRequest | null;
  host: OwnedUiInstance | null;
  inFlight: boolean;
  failure: ShardInstanceFailure | null;
  interruptedTurn: unknown;
  cancellation: HostEffectLedger | null;
  lastPatchSequence: bigint;
};

//#region 📤️CapturedReturnAuthority
export type ShardReturnReport = Exclude<ActorReturnResult, { kind: "page" }> | { readonly kind: "page"; readonly receipt: ActorReturnPageReceipt };
type CapturedReturnWork = { readonly kind: "execute"; readonly events: readonly ShardEventEnvelope[] } | { readonly kind: "retry" | "poll" | "cancel" };
type CapturedReturn = { readonly instance: ShardInstanceOwner; readonly outputs: OwnedActorTurnOutputs; readonly submit: (work: CapturedReturnWork, budget: ShardBudget) => Promise<ShardReturnReport>; facade: OwnedShardReturn | null; origin: ActorReturnOrigin | null; identity: ActorReturnIdentity | null; events: readonly ShardEventEnvelope[] | null; latest: OwnedActorTurnOutput | null; page: OwnedShardReturnPage | null; content: OwnedKernelReturnContent | null; inFlight: boolean; retry: boolean; failed: boolean; cancelled: boolean; retired: boolean };
const RETURN_MINT = Object.freeze({});
let mintCapturedReturn: (state: CapturedReturn) => OwnedShardReturn;
let capturedReturnState: (owner: OwnedShardReturn) => CapturedReturn;
let mintCapturedReturnPage: (state: CapturedReturn, output: OwnedActorTurnOutput, receipt: ActorReturnPageReceipt, page: ActorBytePage) => OwnedShardReturnPage;
function sameReturnOrigin(left: ActorReturnOrigin, right: ActorReturnOrigin): boolean { return left.activationGeneration === right.activationGeneration && left.requestSequence === right.requestSequence; }
function sameReturnIdentity(left: ActorReturnIdentity, right: ActorReturnIdentity): boolean { return sameReturnOrigin(left.origin, right.origin) && left.returnSequence === right.returnSequence; }

/** 📤️ A captured instance retains fixed responses before callers can observe them; controls carry no new semantic events. */
export class OwnedShardReturn {
  readonly #state: CapturedReturn;
  private constructor(mint: object, state: CapturedReturn) { if (mint !== RETURN_MINT) throw new Error("actor-return.private-owner"); this.#state = state; Object.freeze(this); }
  static { mintCapturedReturn = state => new OwnedShardReturn(RETURN_MINT, state); capturedReturnState = owner => owner.#state; }
  static matchesOwner(source: unknown, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): source is OwnedShardReturn {
    if (source === null || typeof source !== "object" || !(#state in source)) return false;
    const instance = source.#state.instance;
    return instance.host === owner && instance.operation === activation && instance.lifetime !== null && actorInstanceLifetimeEquals(instance.lifetime, lifetime);
  }
  get origin(): ActorReturnOrigin | null { return this.#state.origin; }
  get page(): OwnedShardReturnPage | null { return this.#state.page; }
  get content(): OwnedKernelReturnContent | null { return this.#state.content; }
  bindContent(content: OwnedKernelReturnContent): boolean {
    const state = this.#state; const instance = state.instance;
    if (state.content !== null || !instance.host || !instance.lifetime || !OwnedKernelReturnContent.matches(content, this, instance.host, instance.operation, instance.lifetime)) return false;
    state.content = content; return true;
  }
  get retainedResponses(): number { return this.#state.outputs.pending; }
  execute(events: readonly ShardEventEnvelope[], budget: ShardBudget): Promise<ShardReturnReport> { return this.#state.submit({ kind: "execute", events }, budget); }
  retry(budget: ShardBudget): Promise<ShardReturnReport> { return this.#state.submit({ kind: "retry" }, budget); }
  poll(budget: ShardBudget): Promise<ShardReturnReport> { return this.#state.submit({ kind: "poll" }, budget); }
  cancel(budget: ShardBudget): Promise<ShardReturnReport> { return this.#state.submit({ kind: "cancel" }, budget); }
}

/** 📄️ Only exact captured response settlement mints this page; its raw response remains strongly retained. */
export class OwnedShardReturnPage {
  readonly #state: CapturedReturn;
  readonly #output: OwnedActorTurnOutput;
  readonly #receipt: ActorReturnPageReceipt;
  readonly #page: ActorBytePage;
  private constructor(mint: object, state: CapturedReturn, output: OwnedActorTurnOutput, receipt: ActorReturnPageReceipt, page: ActorBytePage) { if (mint !== RETURN_MINT) throw new Error("actor-return.private-page"); this.#state = state; this.#output = output; this.#receipt = receipt; this.#page = page; Object.freeze(this); }
  static { mintCapturedReturnPage = (state, output, receipt, page) => new OwnedShardReturnPage(RETURN_MINT, state, output, receipt, page); }
  static matchesOwner(page: unknown, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): page is OwnedShardReturnPage {
    if (page === null || typeof page !== "object" || !(#state in page)) return false;
    const instance = page.#state.instance;
    return instance.host === owner && instance.operation === activation && instance.lifetime !== null && actorInstanceLifetimeEquals(instance.lifetime, lifetime) && page.#receipt.identity.origin.activationGeneration === lifetime.activationGeneration && page.#output.responseEnvelope !== null;
  }
  get receipt(): ActorReturnPageReceipt { return this.#receipt; }
  byteAt(index: number): number {
    if (this.#state.failed || this.#state.cancelled || !Number.isInteger(index) || index < 0 || index >= this.#receipt.length) throw new Error("actor-return.page-read");
    const block = this.#page[`block${Math.floor(index / 64).toString().padStart(2, "0")}` as keyof ActorBytePage] as ActorBytePage["block00"];
    const word = block[`word${Math.floor(index % 64 / 8)}` as keyof typeof block];
    return Number(word >> BigInt(index % 8 * 8) & 255n);
  }
}
//#endregion 📤️CapturedReturnAuthority

//#region 🩹️NativePatchAuthority
export type OwnedNativeUiPatchValue = { readonly activation: ShardActorActivationLease; readonly lifetime: ActorInstanceLifetime; readonly receipt: ActorUiPatchReceipt; readonly surface: string; readonly baseRevision: number; readonly revision: number; readonly operationCount: number };
type NativeUiPatchState = { readonly owner: ShardInstanceOwner; readonly turn: object; readonly patch: object; readonly operations: readonly unknown[]; readonly value: OwnedNativeUiPatchValue; ordinal: number; read: boolean; original: unknown; input: OwnedUiPatchInputAcceptance | null; token: OwnedUiPatchAcknowledgement | null; submission: Promise<{ readonly receipt: OwnedNativeUiPatchSubmissionReceipt; readonly result: unknown }> | null };
const NATIVE_PATCH_MINT = Object.freeze({});
let mintNativePatch: (state: NativeUiPatchState) => OwnedNativeUiPatchAuthority;
let nativePatchState: (source: OwnedNativeUiPatchAuthority) => NativeUiPatchState;
let mintNativeSubmission: (source: OwnedNativeUiPatchAuthority, token: OwnedUiPatchAcknowledgement) => OwnedNativeUiPatchSubmissionReceipt;

/** 🩹️ A private claim on one patch returned by the original instance turn. */
export class OwnedNativeUiPatchAuthority {
  readonly #state: NativeUiPatchState;
  private constructor(mint: object, state: NativeUiPatchState) { if (mint !== NATIVE_PATCH_MINT) throw new Error("actor-lifecycle.patch-mint"); this.#state = state; Object.freeze(this); }
  static { mintNativePatch = state => new OwnedNativeUiPatchAuthority(NATIVE_PATCH_MINT, state); nativePatchState = source => source.#state; }
  static matches(source: unknown, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): source is OwnedNativeUiPatchAuthority {
    return source !== null && typeof source === "object" && #state in source && source.#state.value.activation === activation && actorInstanceLifetimeEquals(source.#state.value.lifetime, lifetime);
  }
  static matchesOwner(source: unknown, owner: unknown): source is OwnedNativeUiPatchAuthority {
    return source !== null && typeof source === "object" && #state in source && source.#state.owner.host !== null && source.#state.owner.host === owner;
  }
  get value(): OwnedNativeUiPatchValue { return this.#state.value; }
  operation(index: number): unknown {
    const state = this.#state;
    if (!Number.isSafeInteger(index) || index !== state.ordinal || index >= state.value.operationCount) throw new Error("actor-lifecycle.patch-operation-index");
    if (!state.read) { state.original = state.operations[index]; state.read = true; }
    return state.original;
  }
  acceptInput(claim: OwnedUiPatchInputAcceptance): boolean {
    const state = this.#state;
    if (!state.read || state.input !== null && state.input !== claim || state.operations[state.ordinal] !== state.original || !OwnedUiPatchInputAcceptance.matches(claim, this, state.ordinal, state.original)) return false;
    state.input = claim;
    return true;
  }
  releaseInput(token: OwnedUiPatchInputRetirement): boolean {
    const state = this.#state;
    if (!state.read || !state.input || state.operations[state.ordinal] !== state.original || !OwnedUiPatchInputRetirement.matches(token, this, state.ordinal, state.original)) return false;
    state.read = false; state.original = undefined; state.input = null; state.ordinal++;
    return true;
  }
  /** 📥️ Confirms transferred UI inputs only; raw operation and turn roots remain retained separately. */
  get inputRetired(): boolean { return this.#state.ordinal === this.#state.value.operationCount && !this.#state.read; }
}

/** 📨️ A successful native submission is inseparable from both original acknowledgement authorities. */
export class OwnedNativeUiPatchSubmissionReceipt {
  readonly #source: OwnedNativeUiPatchAuthority;
  readonly #token: OwnedUiPatchAcknowledgement;
  private constructor(mint: object, source: OwnedNativeUiPatchAuthority, token: OwnedUiPatchAcknowledgement) { if (mint !== NATIVE_PATCH_MINT) throw new Error("actor-lifecycle.submission-mint"); this.#source = source; this.#token = token; Object.freeze(this); }
  static { mintNativeSubmission = (source, token) => new OwnedNativeUiPatchSubmissionReceipt(NATIVE_PATCH_MINT, source, token); }
  static matches(receipt: unknown, source: object, token: object): receipt is OwnedNativeUiPatchSubmissionReceipt {
    return receipt !== null && typeof receipt === "object" && #source in receipt && receipt.#source === source && receipt.#token === token;
  }
}
//#endregion 🩹️NativePatchAuthority

export interface ShardClientOptions {
  readonly residentLedger: OwnedResidentLedger;
  /** Fixed pool size — design-runtime.md §1 `ShardTable`: web `min(hardwareConcurrency-1, 4)`. Caller
   * computes the number; this class only ever spawns exactly this many workers. */
  readonly shardCount: number;
  readonly createWorker: CreateShardWorker;
  /** ≤2 exclusive shards reserved for `leaseExclusive` — the tail of the shard index range. Clamped to
   * `shardCount`. */
  readonly exclusiveShardCount?: number;
  /** One `Int32Array` slot per shard index — `Atomics.store(sab, shardIdx, turnSeq)` heartbeat path.
   * Omitted entirely when `SharedArrayBuffer` is unavailable (no COOP/COEP): the `postMessage`
   * `heartbeat` message is ALWAYS honored regardless, so correctness never depends on this. */
  readonly heartbeatSab?: SharedArrayBuffer;
  readonly heartbeatTimeoutMs?: number;
  /** Cadence for {@link ShardClient.startWatchdog}'s self-tick when called with no explicit override.
   * Defaults to `heartbeatTimeoutMs` — the same cadence the pre-existing manual `checkHeartbeats()`
   * call pattern already assumed (see that method's own doc: "three consecutive timeout windows"). */
  readonly watchdogIntervalMs?: number;
  readonly now?: () => number;
  /** Fired when a shard is torn down (3 missed heartbeats, or an explicit `terminate()`) — the caller
   * (kernel-side scheduler) is responsible for restoring every listed actor from its last checkpoint
   * on a freshly `rebuild()`-ed shard; this class only does the mechanical worker lifecycle. */
  readonly onShardLost?: (shardIndex: number, actorIds: readonly string[]) => void;
  readonly onActorTrap?: (actorId: string, message: string) => void;
  /** 🌉️ terra-shard-effect-bridge: answers `effect-request` frames — `http-fetch`/`blob-read`/
   * `storage-read`/… . Omitted entirely means every effect-request fails FAST with `"no host effect
   * handler installed"` rather than hanging the guest's `.await` forever; see {@link HostEffectHandler}'s
   * own doc for the full contract (including the cancellation `signal`). */
  readonly onHostEffect?: HostEffectHandler;
  /** 🚦️ Per-actor cap on CONCURRENT unresolved `effect-request`s — mirrors the CONCEPT of
   * `QuotaSchema.outstanding_requests` (the host-side per-instance quota) on the client's own ledger,
   * independent of it: this is `ShardClient`'s own backpressure against queuing unbounded concurrent
   * host-effect handler invocations, regardless of what a later host-side quota decides. A request
   * beyond the cap is rejected immediately with a {@link ShardQuotaBreach}-shaped message. Defaults to
   * {@link DEFAULT_MAX_OUTSTANDING_EFFECTS_PER_ACTOR}. */
  readonly maxOutstandingEffectsPerActor?: number;
}

/** 🧵️ One `ShardClient` instance owns the WHOLE bounded pool (design: "ShardClient... replaces both
 * `PluginWorkerClient`s [both former per-plugin copies] and `pluginHandleForBridge`") — internally one
 * `ShardWorkerLike` ("MessagePort") per shard, `actorId`-tagged request/reply multiplexing so several
 * actors share a shard's single message channel without cross-talk. */
export class ShardClient {
  readonly #residentLedger: OwnedResidentLedger;
  #uiResidentRecord: OwnedResidentRecord | null = null;
  #uiResidentPool: OwnedUiResidentPool | null = null;
  #uiResidentPhase: "empty" | "preparing" | "prepared" | "rejected" | "unused-closing" | "unused-refunding" | "owned" | "pool-closing" | "pool-observing" | "pool-proved" | "closing" | "detached" | "refunding" | "retired" | "fault" = "empty";
  #uiResidentWitness: OwnedUiResidentPoolRetirement | null = null;
  #uiResidentFault: unknown = NO_RESIDENT_FAULT;
  private readonly shards: ShardSlot[] = [];
  private readonly actorShard = new Map<string, number>();
  private readonly actorActivations = new Map<string, ShardActivation>();
  private readonly instanceLifecycles = new Map<number, ShardInstanceOwner>();
  private readonly instanceTurns = new WeakMap<object, { readonly owner: ShardInstanceOwner; readonly patches: WeakMap<object, OwnedNativeUiPatchAuthority> }>();
  private readonly pending = new Map<string, PendingEntry>();
  private readonly exclusiveIndices: ReadonlySet<number>;
  private readonly heartbeatSabView: Int32Array | null;
  private readonly heartbeatTimeoutMs: number;
  private readonly watchdogIntervalMs: number;
  private readonly now: () => number;
  private readonly createWorker: CreateShardWorker;
  private readonly onShardLost?: ShardClientOptions["onShardLost"];
  private readonly onActorTrap?: ShardClientOptions["onActorTrap"];
  private readonly onHostEffect?: HostEffectHandler;
  private readonly maxOutstandingEffectsPerActor: number;
  /** 🌉️ terra-shard-effect-bridge: `actorId` → (`requestId` → its `AbortController`) — the ledger
   * {@link handleEffectRequest}/{@link settleEffect}/{@link abortOutstandingEffects} share; its size
   * per actor IS the outstanding-effect count {@link handleEffectRequest} caps. */
  private readonly outstandingEffectsByActor = new Map<string, HostEffectLedger>();
  private effectReplySeq = 0;
  private nextRoundRobin = 0;
  private requestSeq = 0;
  private activationGeneration = 0n;
  private watchdogHandle: ReturnType<typeof setInterval> | null = null;

  constructor(options: ShardClientOptions) {
    try { Reflect.apply(residentCapacity, options.residentLedger, []); } catch { throw new Error("actor-resident.invalid-ledger"); }
    this.#residentLedger = options.residentLedger;
    if (options.shardCount < 1) throw new Error("[DEBUG] ShardClient requires shardCount >= 1");
    this.createWorker = options.createWorker;
    this.now = options.now ?? (() => Date.now());
    this.heartbeatTimeoutMs = options.heartbeatTimeoutMs ?? DEFAULT_HEARTBEAT_TIMEOUT_MS;
    this.watchdogIntervalMs = options.watchdogIntervalMs ?? this.heartbeatTimeoutMs;
    this.heartbeatSabView = options.heartbeatSab ? new Int32Array(options.heartbeatSab) : null;
    this.onShardLost = options.onShardLost;
    this.onActorTrap = options.onActorTrap;
    this.onHostEffect = options.onHostEffect;
    this.maxOutstandingEffectsPerActor = options.maxOutstandingEffectsPerActor ?? DEFAULT_MAX_OUTSTANDING_EFFECTS_PER_ACTOR;
    const exclusiveCount = Math.max(0, Math.min(options.exclusiveShardCount ?? Math.min(2, options.shardCount - 1), options.shardCount - 1));
    const exclusive = new Set<number>();
    for (let index = options.shardCount - exclusiveCount; index < options.shardCount; index += 1) exclusive.add(index);
    this.exclusiveIndices = exclusive;
    for (let index = 0; index < options.shardCount; index += 1) this.shards.push(this.spawnShard(index));
  }

  //#region 🌱️Lifecycle
  static matchesResidentLedger(client: unknown, ledger: unknown): client is ShardClient { return client !== null && typeof client === "object" && #residentLedger in client && client.#residentLedger === ledger; }

  prepareUiResidentPool(ledger: OwnedResidentLedger, grant: ResidentGrant): ResidentStep {
    if (ledger !== this.#residentLedger) return residentStep("rejected", "actor-resident.foreign-ledger");
    if (this.#uiResidentPool || (this.#uiResidentPhase !== "empty" && this.#uiResidentPhase !== "prepared")) return residentStep("rejected", "actor-resident.pool-owned");
    if (!Number.isSafeInteger(grant.maxItems) || grant.maxItems < 1 || !Number.isSafeInteger(grant.maxBytes) || grant.maxBytes < 256) return residentStep("blocked", "actor-resident.pool-prepare");
    if (this.#uiResidentRecord) return residentStep("ready", "actor-resident.pool-prepare");
    this.#uiResidentPhase = "preparing";
    try {
      const admitted = ledger.reserveRecord("data", poolRecordEnvelope, grant);
      this.#uiResidentRecord = admitted.record;
      this.#uiResidentPhase = admitted.record ? admitted.step.kind === "ready" ? "prepared" : "rejected" : "empty";
      return admitted.record && admitted.step.kind === "ready" ? { ...admitted.step, kind: "pending" } : admitted.step;
    } catch (error) { this.#uiResidentPhase = "fault"; if (this.#uiResidentFault === NO_RESIDENT_FAULT) this.#uiResidentFault = error; return residentStep("rejected", "actor-resident.pool-prepare-fault"); }
  }

  ownsUiResidentPool(pool: unknown): boolean { return pool !== null && this.#uiResidentPool === pool; }

  closeUiResidentPoolStep(grant: ResidentGrant): ResidentStep {
    if (!Number.isSafeInteger(grant.maxItems) || grant.maxItems < 1 || !Number.isSafeInteger(grant.maxBytes) || grant.maxBytes < 64) return residentStep("blocked", "actor-resident.pool-parent-close");
    if (this.#uiResidentPhase === "retired") return residentStep("complete", "actor-resident.pool-parent-close");
    if (this.#uiResidentFault !== NO_RESIDENT_FAULT) return residentStep("blocked", "actor-resident.pool-fault-retirement");
    const record = this.#uiResidentRecord; const pool = this.#uiResidentPool;
    try {
      if (!record) {
        if (this.#uiResidentPhase !== "empty") return residentStep("blocked", "actor-resident.pool-admission-handoff");
        this.#uiResidentPhase = "retired"; return residentStep("complete", "actor-resident.pool-parent-close", 64);
      }
      if (!pool) {
        if (this.#uiResidentPhase === "prepared" || this.#uiResidentPhase === "rejected") { record.beginClose(); this.#uiResidentPhase = "unused-closing"; return residentStep("pending", "actor-resident.pool-unused-close", 64); }
        if (this.#uiResidentPhase === "unused-closing") { const current = record.closeStep(grant); if (current.kind === "complete") { this.#uiResidentPhase = "unused-refunding"; return { ...current, kind: "pending" }; } return current; }
        if (this.#uiResidentPhase === "unused-refunding" && OwnedResidentRetirement.matches(record.retirement, record)) { this.#uiResidentRecord = null; this.#uiResidentPhase = "retired"; return residentStep("complete", "actor-resident.pool-unused-release", 64); }
        return residentStep("blocked", "actor-resident.pool-unused-proof");
      }
      if (this.#uiResidentPhase === "owned") { pool.beginClose(); this.#uiResidentPhase = "pool-closing"; return residentStep("pending", "actor-resident.pool-begin-close", 64); }
      if (this.#uiResidentPhase === "pool-closing") {
        const current = pool.closeStep(grant);
        if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes) return residentStep("rejected", "actor-resident.pool-child-grant");
        if (current.kind === "complete") { this.#uiResidentPhase = "pool-observing"; return { ...current, kind: "pending" }; } return current;
      }
      if (this.#uiResidentPhase === "pool-observing") {
        const witness = pool.retirement; if (!OwnedUiResidentPoolRetirement.matches(witness, pool, this, this.#residentLedger)) return residentStep("blocked", "actor-resident.pool-private-proof", 64);
        this.#uiResidentWitness = witness; this.#uiResidentPhase = "pool-proved"; return residentStep("pending", "actor-resident.pool-observation", 64);
      }
      return this.releaseUiResidentPool(pool, this.#uiResidentWitness, grant);
    } catch (error) { if (this.#uiResidentFault === NO_RESIDENT_FAULT) this.#uiResidentFault = error; return residentStep("rejected", "actor-resident.pool-parent-fault"); }
  }

  installUiResidentPool(pool: OwnedUiResidentPool, grant: ResidentGrant): ResidentStep {
    const record = this.#uiResidentRecord;
    if (!record || (this.#uiResidentPhase !== "prepared" && this.#uiResidentPhase !== "owned") || !OwnedUiResidentPool.matchesComposition(pool, this, this.#residentLedger) || this.#uiResidentPool !== null && this.#uiResidentPool !== pool) return residentStep("rejected", "actor-resident.pool-install");
    if (!Number.isSafeInteger(grant.maxItems) || grant.maxItems < 1 || !Number.isSafeInteger(grant.maxBytes) || grant.maxBytes < 64) return residentStep("blocked", "actor-resident.pool-install");
    this.#uiResidentPool = pool; this.#uiResidentPhase = "owned";
    if (record.matchesShell(pool)) return residentStep("ready", "actor-resident.pool-installed");
    if (this.#uiResidentFault !== NO_RESIDENT_FAULT) return residentStep("blocked", "actor-resident.pool-fault-retirement");
    try { return record.install(pool, grant); } catch (error) { if (this.#uiResidentFault === NO_RESIDENT_FAULT) this.#uiResidentFault = error; return residentStep("rejected", "actor-resident.pool-install-fault"); }
  }

  releaseUiResidentPool(pool: OwnedUiResidentPool, witness: unknown, grant: ResidentGrant): ResidentStep {
    const record = this.#uiResidentRecord;
    if (!record || this.#uiResidentPool !== pool || !OwnedUiResidentPoolRetirement.matches(witness, pool, this, this.#residentLedger) || this.#uiResidentWitness !== null && this.#uiResidentWitness !== witness) return residentStep("rejected", "actor-resident.pool-witness");
    if (!Number.isSafeInteger(grant.maxItems) || grant.maxItems < 1 || !Number.isSafeInteger(grant.maxBytes) || grant.maxBytes < 64) return residentStep("blocked", "actor-resident.pool-release");
    if (this.#uiResidentPhase === "closing" && OwnedResidentRecordDetachment.matches(record.detachment, record, pool)) { this.#uiResidentPhase = "detached"; return residentStep("pending", "actor-resident.pool-detachment", 64); }
    if (this.#uiResidentFault !== NO_RESIDENT_FAULT) return residentStep("blocked", "actor-resident.pool-fault-retirement");
    this.#uiResidentWitness = witness;
    try {
      if (this.#uiResidentPhase === "owned" || this.#uiResidentPhase === "pool-proved") { record.beginClose(); this.#uiResidentPhase = "closing"; return residentStep("pending", "actor-resident.pool-close-record", 64); }
      if (this.#uiResidentPhase === "closing") return record.detach(pool, grant);
      if (this.#uiResidentPhase === "detached") {
        const current = record.closeStep(grant);
        if (current.kind === "complete") { this.#uiResidentPhase = "refunding"; return { ...current, kind: "pending" }; }
        return current;
      }
      if (this.#uiResidentPhase === "refunding" && OwnedResidentRetirement.matches(record.retirement, record)) { this.#uiResidentRecord = null; this.#uiResidentPool = null; this.#uiResidentWitness = null; this.#uiResidentPhase = "retired"; return residentStep("complete", "actor-resident.pool-release", 64); }
      return residentStep("rejected", "actor-resident.pool-release-phase");
    } catch (error) { if (this.#uiResidentFault === NO_RESIDENT_FAULT) this.#uiResidentFault = error; return residentStep("rejected", "actor-resident.pool-release-fault"); }
  }
  private spawnShard(index: number): ShardSlot {
    const worker = this.createWorker(index);
    const slot: ShardSlot = { index, worker, available: true, heartbeat: freshHeartbeatState(this.now()), pendingRequestIds: new Set(), actorIds: new Set() };
    worker.onmessage = (event) => this.handleMessage(slot, event.data as InboundMessage);
    worker.onerror = (error) => {
      if (this.shards[index] !== slot) return;
      console.error(`[DEBUG] shard ${index} worker error`, error);
      this.failShard(slot, new Error(`shard ${index} worker crashed`));
    };
    if (this.heartbeatSabView) worker.postMessage({ kind: "attachHeartbeatSab", shardIndex: index, sab: this.heartbeatSabView.buffer });
    return slot;
  }

  /** 📬️ Per-worker inbound dispatch. `"frame"` is checked BEFORE the generic `pending`-lookup path
   * below (`"result"`'s implicit fallthrough) — mirroring `🟨️shard-worker.js`'s own `deliverEffectResult`
   * ordering note (🧪️ terra-web-bridges): an inbound `"frame"` carries no `requestId` of its own and is
   * never an answer this class is waiting on, so falling through to the generic path would look up a
   * `requestId` nothing ever registered and silently no-op, masking a real effect-request. */
  private handleMessage(slot: ShardSlot, message: InboundMessage): void {
    if (!slot.available || this.shards[slot.index] !== slot) return;
    if (message.kind === "heartbeat") {
      this.recordHeartbeat(slot, message.turnSeq, this.now());
      return;
    }
    if (message.kind === "trap") {
      if ((message.actorId === "*" && message.activationGeneration === null) || this.inboundActivation(slot, message.actorId, message.activationGeneration)) this.onActorTrap?.(message.actorId, message.message);
      return;
    }
    if (message.kind === "frame") {
      const activation = this.inboundActivation(slot, message.actorId, message.activationGeneration);
      if (activation) this.handleInboundFrame(activation, message.frame);
      return;
    }
    const entry = this.pending.get(message.requestId);
    if (!entry || entry.slot !== slot || this.shards[slot.index] !== slot) return;
    if (entry.output && !entry.output.captureResponse(message)) { entry.reject(new Error("actor-output.response-refused")); return; }
    try {
      this.pending.delete(message.requestId);
      slot.pendingRequestIds.delete(message.requestId);
      this.recomputeOldestPending(slot);
      if (message.ok) entry.resolve(message.value);
      else entry.reject(graftWorkerStack(entry.actorId, message.error, message.stack, message.type, message.framesBytes));
    } catch (error) { entry.reject(error); }
  }

  private recomputeOldestPending(slot: ShardSlot): void {
    let oldest: number | null = null;
    for (const requestId of slot.pendingRequestIds) {
      const entry = this.pending.get(requestId);
      if (!entry) continue;
      if (oldest === null || entry.startedAtMs < oldest) oldest = entry.startedAtMs;
    }
    slot.heartbeat.oldestPendingStartedAtMs = oldest;
  }

  /** 🩹️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P4 fix): before this, `failShard` rejected every
   * in-flight request but left `this.actorShard`/`slot.actorIds` untouched — verified by reading the
   * pre-fix source: only `pendingRequestIds` and `heartbeat.oldestPendingStartedAtMs` were cleared.
   * That meant a shard whose worker died via `onerror` (which calls `failShard` directly, WITHOUT
   * `terminate()`+`rebuild()` — see `spawnShard`'s `worker.onerror` below) kept every one of its
   * actors routed to it: a later `activate()`/`turn()` for the same `actorId` would `postMessage` into
   * the same dead worker and hang forever, undetectable until the heartbeat watchdog's own 3-strike
   * ladder eventually caught it. Clearing the routing here means a dead shard stops receiving newly
   * routed work immediately, not only after `rebuild()` runs. */
  private failShard(slot: ShardSlot, error: Error): void {
    slot.available = false;
    for (const requestId of slot.pendingRequestIds) {
      const entry = this.pending.get(requestId);
      if (!entry) continue;
      this.pending.delete(requestId);
      entry.reject(error);
    }
    slot.pendingRequestIds.clear();
    slot.heartbeat.oldestPendingStartedAtMs = null;
    for (const actorId of slot.actorIds) {
      this.abortOutstandingEffects(actorId);
      const activation = this.actorActivations.get(actorId);
      if (activation?.slot === slot) {
        activation.available = false;
        if (activation.instance) activation.instance.failure = "worker-lost";
      }
      this.actorShard.delete(actorId);
    }
    slot.actorIds.clear();
  }

  private rejectActorPending(slot: ShardSlot, actorId: string, error: Error): void {
    for (const requestId of [...slot.pendingRequestIds]) {
      const entry = this.pending.get(requestId);
      if (entry?.actorId !== actorId) continue;
      this.pending.delete(requestId);
      slot.pendingRequestIds.delete(requestId);
      entry.reject(error);
    }
    this.recomputeOldestPending(slot);
  }
  //#endregion 🌱️Lifecycle

  //#region 🧭️Assignment
  private assignShard(actorId: string): ShardSlot {
    const existing = this.actorShard.get(actorId);
    if (existing !== undefined) return this.shards[existing]!;
    const roundRobinCount = this.shards.length - this.exclusiveIndices.size;
    let index = this.nextRoundRobin % Math.max(roundRobinCount, 1);
    while (this.exclusiveIndices.has(index)) index = (index + 1) % this.shards.length;
    this.nextRoundRobin = (this.nextRoundRobin + 1) % Math.max(roundRobinCount, 1);
    this.actorShard.set(actorId, index);
    this.shards[index]!.actorIds.add(actorId);
    return this.shards[index]!;
  }

  /** ▶️ design-runtime.md §1 `ShardTable::request_exclusive` — moves `actorId` onto one of the ≤2
   * exclusive shards for the duration of foreground work. Purely a routing decision: this class does
   * NOT migrate any in-worker instance state (design: migration only happens at a quiescent point via
   * application-level checkpoint) — if the actor was already activated on a different shard, the
   * caller must `checkpoint()` there and `activate()`+`restore()` on the returned exclusive shard. */
  leaseExclusive(actorId: string, options?: { readonly force?: boolean }): number {
    const already = this.actorShard.get(actorId);
    if (already !== undefined && this.exclusiveIndices.has(already)) return already;
    for (const index of this.exclusiveIndices) {
      const slot = this.shards[index]!;
      if (slot.actorIds.size === 0 || options?.force) {
        const activation = this.actorActivations.get(actorId);
        if (activation) activation.operationsAllowed = false;
        this.abortOutstandingEffects(actorId);
        if (already !== undefined) this.shards[already]!.actorIds.delete(actorId);
        slot.actorIds.add(actorId);
        this.actorShard.set(actorId, index);
        return index;
      }
    }
    throw new Error(`[DEBUG] ShardClient.leaseExclusive(${actorId}): no free exclusive shard (${this.exclusiveIndices.size} reserved, all leased)`);
  }

  /** ◀️ Returns `actorId` to the round-robin pool — its NEXT `activate()`/`turn()` targets whichever
   * shard round-robin picks, not necessarily the one it just left. */
  releaseExclusive(actorId: string): void {
    const index = this.actorShard.get(actorId);
    if (index === undefined || !this.exclusiveIndices.has(index)) return;
    const activation = this.actorActivations.get(actorId);
    if (activation) activation.operationsAllowed = false;
    this.abortOutstandingEffects(actorId);
    this.shards[index]!.actorIds.delete(actorId);
    this.actorShard.delete(actorId);
  }

  shardIndexFor(actorId: string): number | undefined {
    return this.actorShard.get(actorId);
  }
  //#endregion 🧭️Assignment

  //#region 📮️Requests
  private nextRequestId(): string {
    if (this.requestSeq >= Number.MAX_SAFE_INTEGER) throw new Error("shard-request.sequence-exhausted");
    this.requestSeq += 1;
    return `r${this.requestSeq}`;
  }

  private send<T>(slot: ShardSlot, message: OutboundMessage, requestId: string | null, posted?: () => void, output: OwnedActorTurnOutput | null = null): Promise<T> {
    if (requestId === null) {
      slot.worker.postMessage(message);
      posted?.();
      return Promise.resolve(undefined as T);
    }
    return new Promise<T>((resolve, reject) => {
      const startedAtMs = this.now();
      this.pending.set(requestId, { resolve: resolve as (value: unknown) => void, reject, slot, startedAtMs, actorId: "actorId" in message ? message.actorId : "", output });
      slot.pendingRequestIds.add(requestId);
      if (slot.heartbeat.oldestPendingStartedAtMs === null) slot.heartbeat.oldestPendingStartedAtMs = startedAtMs;
      try { slot.worker.postMessage(message); posted?.(); }
      catch (error) {
        this.pending.delete(requestId);
        slot.pendingRequestIds.delete(requestId);
        this.recomputeOldestPending(slot);
        reject(error);
      }
    });
  }

  async activate(actorId: string, moduleUrl: string, caps: readonly ShardCapabilityGrant[], budget: ShardBudget, assets: readonly ShardAsset[] = []): Promise<void> {
    if (this.actorActivations.get(actorId)?.available) throw new Error("actor-close.activation-already-owned");
    if (this.activationGeneration >= 0xffffffffffffffffn) throw new Error("actor-close.activation-generation-exhausted");
    const requestId = this.nextRequestId();
    const generation = this.activationGeneration + 1n;
    const slot = this.assignShard(actorId);
    const activation: ShardActivation = { slot, actorId, generation, available: true, activated: false, teardownPosted: false, operationsAllowed: true, operationGeneration: 0n, lastGuestLifetime: 0n, lastReturnSequence: 0n, returned: null, instance: null, close: null };
    this.activationGeneration = generation;
    this.actorActivations.set(actorId, activation);
    await this.send<void>(slot, { kind: "activate", requestId, actorId, activationGeneration: generation, moduleUrl, caps, budget, assets }, requestId);
    activation.activated = true;
  }

  //#region 🪪️ExactActivation
  private activationIsActive(activation: ShardActivation): boolean {
    return activation.available && activation.slot.available && activation.activated && activation.operationsAllowed && activation.close === null && this.actorActivations.get(activation.actorId) === activation && this.actorShard.get(activation.actorId) === activation.slot.index && this.shards[activation.slot.index] === activation.slot;
  }

  private inboundActivation(slot: ShardSlot, actorId: string, generation: bigint | null): ShardActivation | undefined {
    const activation = this.actorActivations.get(actorId);
    return activation && activation.slot === slot && activation.generation === generation && this.activationIsActive(activation) ? activation : undefined;
  }

  /** 🎯️ Captures once before asynchronous work; every turn checks before dispatch and after settlement. */
  captureActorActivation(actorId: string): ShardActorActivationLease {
    const activation = this.actorActivations.get(actorId);
    if (!activation?.activated) throw new Error("actor-activation.not-ready");
    const slot = activation.slot;
    const worker = slot.worker;
    const operationGeneration = activation.operationGeneration;
    const assertActive = (): void => {
      if (!activation.available || !slot.available || !activation.operationsAllowed || activation.operationGeneration !== operationGeneration || activation.close !== null || this.actorActivations.get(actorId) !== activation || this.actorShard.get(actorId) !== slot.index || this.shards[slot.index] !== slot || slot.worker !== worker) throw new Error("actor-activation.revoked");
    };
    assertActive();
    return Object.freeze({
      actorId,
      activationGeneration: activation.generation,
      assertActive,
      turn: async (events: readonly ShardEventEnvelope[], budget: ShardBudget, commandPage?: ShardCommandIngressPage): Promise<unknown> => {
        assertActive();
        if (activation.returned !== null) throw new Error("actor-return.already-owned");
        const owner = activation.instance;
        const requestId = this.nextRequestId();
        const result = await this.send(slot, { kind: "turn", requestId, actorId, activationGeneration: activation.generation, events, commandPage, budget }, requestId);
        if (owner) this.recordInstanceTurn(owner, result);
        try { assertActive(); } catch (error) { if (owner) owner.interruptedTurn = result; throw error; }
        return result;
      },
    });
  }
  //#endregion 🪪️ExactActivation

  //#region 🚪️ExactInstanceLifecycle
  /** 🪪️ Reserves an exact pre-open owner; only a guest receipt can populate its lifetime. */
  captureInstanceLifecycle(actorId: string, instanceId: number): ShardInstanceLifecycleLease {
    if (!Number.isInteger(instanceId) || instanceId < 0 || instanceId > 0xffffffff) throw new Error("actor-lifecycle.invalid-instance");
    const activation = this.actorActivations.get(actorId);
    if (!activation || !this.activationIsActive(activation)) throw new Error("actor-lifecycle.activation-not-ready");
    if (activation.instance !== null) throw new Error("actor-lifecycle.instance-already-owned");
    const operation = this.captureActorActivation(actorId);
    this.nextRequestId();
    const open: ActorInstanceOpenRequest = Object.freeze({ kind: "open", activationGeneration: activation.generation, instanceId, requestSequence: this.requestSeq });
    const owner: ShardInstanceOwner = { activation, operation, open, phase: "opening", lifetime: null, receipt: null, accepted: null, close: null, host: null, inFlight: false, failure: null, interruptedTurn: null, cancellation: null, lastPatchSequence: 0n };
    activation.instance = owner;
    this.instanceLifecycles.set(open.requestSequence, owner);
    return Object.freeze({
      activation: operation,
      openRequest: open,
      get lifetime() { return owner.lifetime; },
      get pendingReceipt() { return owner.receipt; },
      get interruptedTurn() { return owner.interruptedTurn; },
      get pendingReturn() { return owner.activation.returned?.instance === owner ? owner.activation.returned.facade : null; },
      reserveReturn: (maximumResponses: number) => this.reserveInstanceReturn(owner, maximumResponses),
      open: async (input: ShardInstanceOpenInput, budget: ShardBudget): Promise<unknown> => {
        if (owner.phase !== "opening") throw new Error("actor-lifecycle.open-already-captured");
        operation.assertActive();
        return this.sendInstanceLifecycle(owner, [{ kind: "instance-open", payload: { instance: instanceId, activationGeneration: open.activationGeneration, requestSequence: open.requestSequence, appId: input.appId, actor: input.actor, config: input.config, assets: input.assets, capabilities: input.capabilities, quotas: input.quotas } }], budget);
      },
      poll: (budget: ShardBudget) => this.sendInstanceLifecycle(owner, [], budget),
      beginClose: () => this.beginInstanceLifecycleClose(owner),
      close: async (budget: ShardBudget): Promise<unknown> => {
        const request = this.beginInstanceLifecycleClose(owner);
        if (owner.receipt !== null) throw new Error("actor-lifecycle.receipt-ack-required");
        return this.sendInstanceLifecycle(owner, [{ kind: "instance-close", payload: request }], budget);
      },
      acknowledge: async (receipt: ActorInstanceLifecycleReceipt, budget: ShardBudget, retirement?: OwnedUiInstanceRetirement): Promise<unknown> => {
        if (!owner.receipt || !actorInstanceLifecycleReceiptEquals(owner.receipt, receipt)) throw new Error("actor-lifecycle.ack-mismatch");
        if (receipt.kind === "retired" && (owner.cancellation !== null || !owner.host || !owner.lifetime || !OwnedUiInstanceRetirement.matches(retirement, owner.host, operation, owner.lifetime))) throw new Error("actor-lifecycle.host-retirement-pending");
        return this.sendInstanceLifecycle(owner, [{ kind: "instance-lifecycle-ack", payload: { kind: "ack", receipt: owner.receipt } }], budget, owner.receipt);
      },
      bindHostRetirement: (participant: OwnedUiInstance): void => {
        if (owner.host !== null || owner.lifetime === null || !OwnedUiInstance.matches(participant, operation, owner.lifetime)) throw new Error("actor-lifecycle.host-owner-mismatch");
        owner.host = participant;
      },
      captureUiPatchAuthority: (originalTurn: object, patchIndex: number) => this.captureInstanceUiPatch(owner, originalTurn, patchIndex),
      submitUiAcknowledgement: (source: OwnedNativeUiPatchAuthority, token: OwnedUiPatchAcknowledgement, budget: ShardBudget) => this.submitInstanceUiAcknowledgement(owner, source, token, budget),
      dispose: () => {
        if (owner.phase !== "complete") throw new Error("actor-close.native-retirement-pending");
        this.disposeActivation(owner.activation);
      },
      progress: (): ReturnType<ShardInstanceLifecycleLease["progress"]> => ({ kind: owner.failure === null ? owner.phase : "blocked", failure: owner.failure }),
    });
  }

  private reserveInstanceReturn(instance: ShardInstanceOwner, maximumResponses: number): OwnedShardReturn {
    instance.operation.assertActive();
    if (instance.inFlight) throw new Error("actor-return.request-pending");
    if (instance.activation.returned !== null) throw new Error("actor-return.already-owned");
    const outputs = new OwnedActorTurnOutputs(instance, maximumResponses);
    const state: CapturedReturn = { instance, outputs, submit: (work, budget) => this.sendCapturedReturn(state, work, budget), facade: null, origin: null, identity: null, events: null, latest: null, page: null, content: null, inFlight: false, retry: false, failed: false, cancelled: false, retired: false };
    const facade = mintCapturedReturn(state); state.facade = facade; instance.activation.returned = state;
    return facade;
  }

  private async sendCapturedReturn(state: CapturedReturn, work: CapturedReturnWork, budget: ShardBudget): Promise<ShardReturnReport> {
    const instance = state.instance; const activation = instance.activation; const slot = activation.slot;
    if (!activation.available || !slot.available || this.shards[slot.index] !== slot) throw new Error("actor-return.worker-lost");
    if (state.inFlight || instance.inFlight) throw new Error("actor-return.request-pending");
    if (state.failed && work.kind !== "cancel") throw new Error("actor-return.owner-fault");
    if (work.kind === "execute" && state.origin !== null) throw new Error("actor-return.execute-already-owned");
    const execution = work.kind === "execute" || work.kind === "retry";
    if (execution) {
      instance.operation.assertActive();
      if (work.kind === "retry" && !state.retry) throw new Error("actor-return.retry-not-admitted");
    } else if (state.identity === null) throw new Error("actor-return.identity-pending");
    const output = state.outputs.reserve();
    if (!output) throw new Error("actor-return.response-capacity");
    let requestId: string;
    try { requestId = this.nextRequestId(); } catch (error) { output.cancelEmpty(); throw error; }
    if (work.kind === "execute") { state.origin = Object.freeze({ activationGeneration: activation.generation, requestSequence: this.requestSeq }); state.events = work.events; }
    const drive: ActorReturnDrive = execution ? { kind: "execute", origin: state.origin! } : { kind: "control", control: { kind: work.kind as "poll" | "cancel", identity: state.identity! } };
    const message: OutboundMessage & { readonly returnDrive: Uint8Array } = { kind: "turn", requestId, actorId: activation.actorId, activationGeneration: activation.generation, events: execution ? state.events! : [], budget, returnDrive: encodeActorReturnDrive(drive) };
    state.inFlight = true; state.retry = false; state.latest = output;
    let posted = false;
    try {
      const raw = await output.run(() => this.send<unknown>(slot, message, requestId, () => { posted = true; }, output));
      if (!activation.available || !slot.available || this.shards[slot.index] !== slot) throw new Error("actor-return.worker-lost");
      if (!(raw instanceof Uint8Array)) throw new Error("actor-return.fixed-result-required");
      const result = decodeActorReturnResult(raw);
      this.acceptCapturedReturn(state, drive, result, output);
      return result.kind === "page" ? Object.freeze({ kind: "page", receipt: result.receipt }) : result;
    } catch (error) {
      state.retry = execution && !posted;
      if (posted) state.failed = true;
      throw error;
    } finally { state.inFlight = false; }
  }

  private acceptCapturedReturn(state: CapturedReturn, drive: ActorReturnDrive, result: ActorReturnResult, output: OwnedActorTurnOutput): void {
    if (result.kind === "protocolFault") { state.failed = true; return; }
    const identity = result.kind === "page" ? result.receipt.identity : result.kind === "pending" || result.kind === "retired" ? result.identity : result.kind === "control" ? result.control.kind === "inputAck" ? result.control.receipt.identity : result.control.identity : null;
    const origin = result.kind === "refused" ? result.origin : identity!.origin;
    if (!state.origin || !sameReturnOrigin(state.origin, origin)) throw new Error("actor-return.foreign-origin");
    if (drive.kind === "execute") {
      if (result.kind === "control") throw new Error("actor-return.unexpected-control");
      if (result.kind === "refused") { state.retry = true; return; }
    } else {
      if (result.kind === "refused") throw new Error("actor-return.unexpected-refusal");
      if (drive.control.kind !== "poll" && result.kind !== "control") throw new Error("actor-return.control-result-required");
      if (result.kind === "control") {
        const expected = encodeActorReturnDrive(drive); const actual = encodeActorReturnDrive({ kind: "control", control: result.control });
        if (expected.length !== actual.length || expected.some((byte, index) => byte !== actual[index])) throw new Error("actor-return.foreign-control");
      }
    }
    if (identity) {
      if (state.identity === null) {
        if (identity.returnSequence <= state.instance.activation.lastReturnSequence) throw new Error("actor-return.stale-sequence");
        state.identity = identity; state.instance.activation.lastReturnSequence = identity.returnSequence;
      } else if (!sameReturnIdentity(state.identity, identity)) throw new Error("actor-return.foreign-identity");
    }
    if (result.kind === "page") {
      if (state.page !== null || state.cancelled || state.retired) throw new Error("actor-return.page-already-owned");
      state.page = mintCapturedReturnPage(state, output, result.receipt, result.page);
    } else if (result.kind === "control" && result.control.kind === "cancel" && (result.outcome === "accepted" || result.outcome === "duplicate")) state.cancelled = true;
    else if (result.kind === "retired") {
      if (state.page !== null) throw new Error("actor-return.input-retirement-pending");
      state.retired = true;
    }
  }

  private beginInstanceLifecycleClose(owner: ShardInstanceOwner): ActorInstanceCloseRequest {
    if (owner.close) return owner.close;
    if (!owner.activation.available || !owner.activation.slot.available || this.shards[owner.activation.slot.index] !== owner.activation.slot) { owner.failure = "worker-lost"; throw new Error("actor-lifecycle.worker-lost"); }
    if (owner.lifetime === null || owner.phase !== "open") throw new Error("actor-lifecycle.capture-pending");
    if (owner.activation.operationGeneration >= 0xffffffffffffffffn) throw new Error("actor-lifecycle.operation-generation-exhausted");
    this.nextRequestId();
    owner.close = Object.freeze({ kind: "close", lifetime: owner.lifetime, requestSequence: this.requestSeq });
    owner.phase = "closing";
    owner.activation.close = owner;
    owner.activation.operationGeneration += 1n;
    const ledger = this.outstandingEffectsByActor.get(owner.activation.actorId);
    if (ledger?.activation === owner.activation) {
      this.outstandingEffectsByActor.delete(owner.activation.actorId);
      owner.cancellation = ledger;
    }
    return owner.close;
  }

  private async sendInstanceLifecycle(owner: ShardInstanceOwner, events: readonly ShardEventEnvelope[], budget: ShardBudget, acknowledged?: ActorInstanceLifecycleReceipt): Promise<unknown> {
    const { activation } = owner;
    const { slot } = activation;
    if (!activation.available || !slot.available || this.shards[slot.index] !== slot) { owner.failure = "worker-lost"; throw new Error("actor-lifecycle.worker-lost"); }
    if (activation.returned !== null) throw new Error("actor-return.retirement-pending");
    if (owner.phase === "complete") throw new Error("actor-lifecycle.already-complete");
    if (owner.inFlight) throw new Error("actor-lifecycle.turn-already-pending");
    const requestId = this.nextRequestId();
    owner.inFlight = true;
    let posted = false;
    try {
      if (owner.cancellation) {
        this.cancelOneEffect(owner.cancellation);
        if (owner.cancellation.head === null) owner.cancellation = null;
      }
      const result = await this.send<unknown>(slot, { kind: "turn", requestId, actorId: activation.actorId, activationGeneration: activation.generation, events, budget }, requestId, () => { posted = true; });
      this.recordInstanceTurn(owner, result);
      if (!activation.available || !slot.available || this.shards[slot.index] !== slot) { owner.failure = "worker-lost"; throw new Error("actor-lifecycle.worker-lost"); }
      const status = result !== null && typeof result === "object" ? Reflect.get(result, "status") : undefined;
      const admitted = status !== null && typeof status === "object" && ["idle", "more-work", "checkpoint-ready"].includes(Reflect.get(status, "tag"));
      try { this.acceptInstanceLifecycleResult(owner, result, admitted ? acknowledged : undefined); }
      catch (error) { owner.failure = "invalid-receipt"; throw error; }
      if (!admitted) {
        owner.interruptedTurn = result;
        owner.failure = "worker-refused";
        throw new Error(acknowledged ? "actor-lifecycle.ack-not-admitted" : "actor-lifecycle.turn-not-admitted");
      }
      owner.failure = null;
      return result;
    } catch (error) {
      owner.failure ??= !activation.available || !slot.available || this.shards[slot.index] !== slot ? "worker-lost" : posted ? "worker-refused" : "transport-refused";
      throw error;
    } finally {
      owner.inFlight = false;
    }
  }

  private recordInstanceTurn(owner: ShardInstanceOwner, result: unknown): void {
    if (result !== null && typeof result === "object") this.instanceTurns.set(result, { owner, patches: new WeakMap() });
  }

  private captureInstanceUiPatch(owner: ShardInstanceOwner, turn: object, patchIndex: number): OwnedNativeUiPatchAuthority {
    const captured = this.instanceTurns.get(turn);
    if (!captured || captured.owner !== owner || owner.lifetime === null) throw new Error("actor-lifecycle.foreign-turn");
    const patches: unknown = Reflect.get(turn, "uiPatches");
    if (!Array.isArray(patches) || !Number.isSafeInteger(patchIndex) || patchIndex < 0 || patchIndex >= patches.length) throw new Error("actor-lifecycle.patch-index");
    const wire = Reflect.get(turn, "uiPatchReceipt");
    const decoded = wire == null ? null : decodeActorUiPatchReceipt(wire);
    validateActorUiPatchPairing(patches.length, decoded);
    if (!decoded || !actorInstanceLifetimeEquals(decoded.lifetime, owner.lifetime)) throw new Error("actor-ui-patch.lifetime-mismatch");
    const patch: unknown = patches[patchIndex];
    if (patch === null || typeof patch !== "object") throw new Error("actor-lifecycle.patch-envelope");
    const existing = captured.patches.get(patch);
    if (existing) {
      if (!actorUiPatchReceiptEquals(existing.value.receipt, decoded)) throw new Error("actor-ui-patch.receipt-mismatch");
      return existing;
    }
    if (decoded.patchSequence <= owner.lastPatchSequence) throw new Error("actor-ui-patch.duplicate-sequence");
    const surface: unknown = Reflect.get(patch, "surface");
    const operations: unknown = Reflect.get(patch, "ops");
    const revision: unknown = Reflect.get(patch, "revision");
    const base: unknown = Reflect.get(patch, "baseRevision");
    const exactRevision = (value: unknown): number => {
      if (typeof value === "bigint" && value >= 0n && value <= BigInt(Number.MAX_SAFE_INTEGER)) return Number(value);
      if (typeof value === "number" && Number.isSafeInteger(value) && value >= 0) return value;
      throw new Error("actor-lifecycle.patch-revision");
    };
    if (!surface || typeof surface !== "object" || Reflect.get(surface, "instance") !== owner.lifetime.instanceId || !Array.isArray(operations)) throw new Error("actor-lifecycle.patch-envelope");
    const name: unknown = Reflect.get(surface, "surface");
    if (typeof name !== "string" || name.length > 512 || new TextEncoder().encode(name).length > 512) throw new Error("actor-lifecycle.patch-surface");
    const receipt = Object.freeze({ lifetime: Object.freeze(decoded.lifetime), patchSequence: decoded.patchSequence });
    const value = Object.freeze({ activation: owner.operation, lifetime: owner.lifetime, receipt, surface: name, revision: exactRevision(revision), baseRevision: exactRevision(base), operationCount: operations.length });
    const authority = mintNativePatch({ owner, turn, patch, operations, value, ordinal: 0, read: false, original: undefined, input: null, token: null, submission: null });
    captured.patches.set(patch, authority);
    owner.lastPatchSequence = receipt.patchSequence;
    return authority;
  }

  private async submitInstanceUiAcknowledgement(owner: ShardInstanceOwner, source: OwnedNativeUiPatchAuthority, token: OwnedUiPatchAcknowledgement, budget: ShardBudget): Promise<{ readonly receipt: OwnedNativeUiPatchSubmissionReceipt; readonly result: unknown }> {
    if (!owner.lifetime || !OwnedNativeUiPatchAuthority.matches(source, owner.operation, owner.lifetime) || !OwnedUiPatchAcknowledgement.matches(token, source)) throw new Error("actor-lifecycle.ui-ack-mismatch");
    const state = nativePatchState(source);
    const value = token.value;
    if (state.owner !== owner || state.token !== null && state.token !== token || value.actor !== owner.activation.actorId || value.instance !== owner.lifetime.instanceId || value.surface !== state.value.surface || value.revision !== state.value.revision || !actorInstanceLifetimeEquals(value.lifetime, owner.lifetime) || !actorUiPatchReceiptEquals(value.receipt, state.value.receipt)) throw new Error("actor-lifecycle.ui-ack-mismatch");
    if (!source.inputRetired) throw new Error("actor-lifecycle.ui-input-pending");
    if (state.submission) return state.submission;
    state.token = token;
    state.submission = (async () => {
      const result = await this.sendInstanceLifecycle(owner, [{ kind: "patch-ack", payload: { receipt: state.value.receipt, surface: { instance: owner.lifetime!.instanceId, surface: state.value.surface }, revision: BigInt(state.value.revision) } }], budget);
      const status = result !== null && typeof result === "object" ? Reflect.get(result, "status") : undefined;
      if (!status || typeof status !== "object" || !["idle", "more-work"].includes(Reflect.get(status, "tag"))) throw new Error("actor-lifecycle.ui-ack-not-admitted");
      return Object.freeze({ receipt: mintNativeSubmission(source, token), result });
    })();
    try { return await state.submission; } catch (error) { state.submission = null; throw error; }
  }

  private acceptInstanceLifecycleResult(owner: ShardInstanceOwner, result: unknown, acknowledged?: ActorInstanceLifecycleReceipt): void {
    const wire = result && typeof result === "object" ? Reflect.get(result, "lifecycleReceipt") : undefined;
    let incoming: ActorInstanceLifecycleReceipt | null = null;
    if (wire !== undefined && wire !== null) {
      const decoded = decodeActorInstanceLifecycle(wire);
      if (decoded.kind !== "captured" && decoded.kind !== "accepted" && decoded.kind !== "retired") throw new Error("actor-lifecycle.receipt-required");
      incoming = Object.freeze({ ...decoded, lifetime: Object.freeze(decoded.lifetime) });
    }
    let phase = owner.phase;
    let pending = owner.receipt;
    if (acknowledged && (!incoming || !actorInstanceLifecycleReceiptEquals(acknowledged, incoming))) {
      pending = null;
      phase = acknowledged.kind === "captured" ? "open" : acknowledged.kind === "retired" ? "complete" : "accepted";
    }
    if (incoming) {
      if (phase === "complete" || pending !== null && !actorInstanceLifecycleReceiptEquals(pending, incoming)) throw new Error("actor-lifecycle.receipt-mismatch");
      if (incoming.kind === "captured") {
        if (!actorInstanceCapturedReceiptMatches(owner.open, incoming) || owner.lifetime !== null && !actorInstanceLifetimeEquals(owner.lifetime, incoming.lifetime) || owner.lifetime === null && incoming.lifetime.guestLifetime <= owner.activation.lastGuestLifetime) throw new Error("actor-lifecycle.receipt-mismatch");
        phase = "captured";
      } else {
        if (!owner.close || !actorInstanceCloseReceiptMatches(owner.close, owner.accepted, incoming)) throw new Error("actor-lifecycle.receipt-mismatch");
        phase = incoming.kind;
      }
      pending = incoming;
    }
    if (incoming?.kind === "captured") {
      owner.lifetime = incoming.lifetime;
      owner.activation.lastGuestLifetime = incoming.lifetime.guestLifetime;
    }
    if (incoming?.kind === "accepted") owner.accepted = incoming;
    owner.receipt = pending;
    owner.phase = phase;
    if (phase === "complete") {
      if (owner.activation.instance === owner) owner.activation.instance = null;
      if (owner.activation.close === owner) owner.activation.close = null;
      this.instanceLifecycles.delete(owner.open.requestSequence);
    }
  }
  //#endregion 🚪️ExactInstanceLifecycle

  /** ▶️ One turn (`reactor::poll`), never more than one in flight per `actorId` at a time — a second
   * `turn()` call for the same actor before the first resolves is a caller bug (the scheduler's own
   * per-actor serialization, not this transport's job, per design's "runs one turn at a time per
   * actor"), so it is rejected rather than silently queued. */
  async turn(actorId: string, events: readonly ShardEventEnvelope[], budget: ShardBudget, commandPage?: ShardCommandIngressPage): Promise<unknown> {
    if (!this.actorShard.has(actorId)) throw new Error(`[DEBUG] ShardClient.turn(${actorId}): not activated on any shard`);
    return this.captureActorActivation(actorId).turn(events, budget, commandPage);
  }

  /** 📨️ terra-web-shardframe: `ShardFrame::Envelope` passthrough — wraps ONE envelope's worth of work
   * in the Rust-mirrored shape, deliberately budget-LESS on the wire (the worker resolves
   * `envelope.to`'s LAST {@link ShardFrame.Grant} via a {@link GrantedBudgetTracker}, falling back to
   * {@link MAINTENANCE_LANE_DEFAULT_BUDGET} for an actor never granted one) — exactly the incremental
   * adoption step the Rust `ShardFrame::Envelope` doc calls for ("kept so the web `ShardClient`... can
   * adopt this wire incrementally... without both ends changing atomically"). {@link turn} is left
   * completely untouched by this method; both wire shapes coexist on purpose. */
  async envelope(shardEnvelope: ShardEnvelope): Promise<unknown> {
    const slot = this.requireShard(shardEnvelope.to);
    const activation = this.captureActorActivation(shardEnvelope.to);
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "frame", requestId, actorId: shardEnvelope.to, activationGeneration: activation.activationGeneration, frame: { kind: "Envelope", envelope: shardEnvelope } }, requestId);
  }

  /** ⚖️ terra-web-shardframe: `ShardFrame::Grant` — `budget` travels WITH `envelopes` in ONE wire
   * message (design-runtime.md's DRR promise), sent to the worker in LANE-PRIORITY order via
   * {@link orderEnvelopesByLane} rather than push/arrival order. The worker remembers `budget` as
   * `actorId`'s new granted budget for any later {@link envelope} passthrough. */
  async grant(actorId: string, budget: ShardBudget, envelopes: readonly ShardEnvelope[]): Promise<unknown> {
    const slot = this.requireShard(actorId);
    const activation = this.captureActorActivation(actorId);
    const requestId = this.nextRequestId();
    const ordered = orderEnvelopesByLane(envelopes);
    return this.send(slot, { kind: "frame", requestId, actorId, activationGeneration: activation.activationGeneration, frame: { kind: "Grant", actor: actorId, budget, envelopes: ordered } }, requestId);
  }

  async startJob(actorId: string, job: number, jobKind: string, input: Uint8Array): Promise<void> {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    await this.send<void>(slot, { kind: "startJob", requestId, actorId, job, jobKind, input }, requestId);
  }

  async stepJob(actorId: string, job: number, budget: ShardJobBudget): Promise<ShardJobStep> {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    return this.send<ShardJobStep>(slot, { kind: "stepJob", requestId, actorId, job, budget }, requestId);
  }

  cancelJob(actorId: string, job: number): void {
    const slot = this.requireShard(actorId);
    void this.send(slot, { kind: "cancelJob", actorId, job }, null);
  }

  /** 🧵 Requests exactly one operation-owned item and enforces the transport byte credit. */
  async takeSegmentedDownloadChunk(actorId: string, instanceId: number, operationId: bigint): Promise<Uint8Array | undefined> {
    if (!Number.isSafeInteger(instanceId) || instanceId < 0 || typeof operationId !== "bigint" || operationId <= 0n || operationId > MAX_SEGMENTED_DOWNLOAD_OPERATION_ID) throw new Error("segmented-download-authority-invalid");
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    const value = await this.send<unknown>(slot, { kind: "takeSegmentedDownloadChunk", requestId, actorId, instanceId, operationId }, requestId);
    if (value === undefined || value === null) return undefined;
    if (Object.prototype.toString.call(value) !== "[object Uint8Array]") throw new Error("segmented-download-transport-type");
    const chunk = value as Uint8Array;
    if (chunk.byteLength === 0 || chunk.byteLength > MAX_SEGMENTED_DOWNLOAD_CHUNK_BYTES) throw new Error("segmented-download-transport-limit");
    return chunk;
  }

  async checkpoint(actorId: string): Promise<Uint8Array> {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    return this.send<Uint8Array>(slot, { kind: "checkpoint", requestId, actorId }, requestId);
  }

  async restore(actorId: string, state: Uint8Array): Promise<void> {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    await this.send<void>(slot, { kind: "restore", requestId, actorId, state }, requestId);
  }

  /** ⏏️ Frees the worker-side actor entry — does not touch this shard's routing entry so a caller that
   * immediately re-`activate()`s the same `actorId` (hot reload) can still target the same shard;
   * pair with a routing-level `actorShard.delete` only when the actor is gone for good (unusual — most
   * callers instead let `activate` overwrite in place). */
  dispose(actorId: string): void {
    const activation = this.actorActivations.get(actorId);
    if (activation) { this.disposeActivation(activation); return; }
    const shardIndex = this.actorShard.get(actorId);
    if (shardIndex === undefined) return;
    this.shards[shardIndex]!.actorIds.delete(actorId);
    this.actorShard.delete(actorId);
  }

  private disposeActivation(activation: ShardActivation): void {
    if (activation.teardownPosted) return;
    if (activation.instance !== null || activation.close !== null) throw new Error("actor-close.native-retirement-pending");
    if (activation.returned !== null) throw new Error("actor-return.retirement-pending");
    const { actorId, slot } = activation;
    if (!activation.available || !slot.available || this.shards[slot.index] !== slot) throw new Error("actor-close.worker-lost");
    activation.operationsAllowed = false;
    slot.worker.postMessage({ kind: "dispose", actorId, activationGeneration: activation.generation } satisfies OutboundMessage);
    activation.teardownPosted = true;
    activation.available = false;
    if (this.outstandingEffectsByActor.get(actorId)?.activation === activation) this.abortOutstandingEffects(actorId);
    this.rejectActorPending(slot, actorId, new Error(`ShardClient actor disposed: ${actorId}`));
    slot.actorIds.delete(actorId);
    if (this.actorActivations.get(actorId) === activation) {
      const route = this.actorShard.get(actorId);
      if (route !== undefined) this.shards[route]!.actorIds.delete(actorId);
      this.actorShard.delete(actorId);
      this.actorActivations.delete(actorId);
    }
  }

  private requireShard(actorId: string): ShardSlot {
    const index = this.actorShard.get(actorId);
    if (index === undefined) throw new Error(`[DEBUG] ShardClient: actor ${actorId} is not activated on any shard`);
    return this.shards[index]!;
  }
  //#endregion 📮️Requests

  //#region 🌉️HostEffectBridge
  /** 📨️ terra-shard-effect-bridge: dispatches an inbound `"frame"` — today the ONLY frame a worker
   * ever sends UP is `🟨️host-shim.js`'s `effect-request` (🧪️ terra-web-bridges); any other `Envelope`
   * payload kind (the `effect-emit`/`ui-patch-emit` fire-and-forget doors, or a future frame kind
   * entirely) is intentionally ignored here rather than thrown on — the same forward-compat tolerance
   * {@link interpretShardFrame}'s own `"unknown"` branch already established for this file. Routing
   * `effect-emit`/`ui-patch-emit` to a real handler is explicitly out of this packet's scope (its own
   * ticket brief only closes the effect-request/effect-complete/effect-error loop) — flagged in the
   * accompanying report as a known gap for whoever owns emit routing next. */
  private handleInboundFrame(activation: ShardActivation, frame: ShardFrame): void {
    if (frame.kind !== "Envelope") return;
    if (frame.envelope.to !== "kernel" || frame.envelope.from.kind !== "actor" || frame.envelope.from.id !== activation.actorId) return;
    const payload = frame.envelope.payload;
    if (payload.kind !== "effect-request") return;
    const request = payload.payload as { readonly effect: string; readonly requestId: string; readonly params: unknown };
    this.handleEffectRequest(activation, request.effect, request.requestId, request.params);
  }

  /** 🚪️ Answers one `effect-request`: quota-checks against {@link maxOutstandingEffectsPerActor}, then
   * hands off to {@link onHostEffect} — or, absent one, fails FAST with an explicit `effect-error`
   * rather than ever leaving the guest's `.await` pending, per this ticket's own acceptance bar. Always
   * settles exactly once, via {@link replyEffectComplete}/{@link replyEffectError}. */
  private handleEffectRequest(activation: ShardActivation, effect: string, requestId: string, params: unknown): void {
    const actorId = activation.actorId;
    const outstanding = this.outstandingEffectsByActor.get(actorId) ?? { activation, requests: new Map<string, PendingHostEffect>(), head: null, tail: null };
    if (outstanding.activation !== activation || outstanding.requests.has(requestId)) return;
    if (outstanding.requests.size >= this.maxOutstandingEffectsPerActor) {
      const breach: ShardQuotaBreach = { quota: "outstandingRequests", limit: this.maxOutstandingEffectsPerActor, actual: outstanding.requests.size };
      this.replyEffectError(activation, requestId, formatQuotaBreachMessage(breach));
      return;
    }
    if (!this.onHostEffect) {
      this.replyEffectError(activation, requestId, "no host effect handler installed");
      return;
    }
    const controller = new AbortController();
    const entry: PendingHostEffect = { activation, controller, requestId, previous: outstanding.tail, next: null };
    if (outstanding.tail) outstanding.tail.next = entry; else outstanding.head = entry;
    outstanding.tail = entry;
    outstanding.requests.set(requestId, entry);
    this.outstandingEffectsByActor.set(actorId, outstanding);
    this.onHostEffect(actorId, effect, params, controller.signal).then(
      (value) => {
        if (this.settleEffect(requestId, entry)) this.replyEffectComplete(activation, requestId, value);
      },
      (error: unknown) => {
        if (this.settleEffect(requestId, entry)) this.replyEffectError(activation, requestId, error instanceof Error ? error.message : String(error));
      },
    );
  }

  /** ✅ Removes `requestId` from the outstanding-effect ledger; returns `false` if it was already gone
   * (settled once already, or cleared by {@link abortOutstandingEffects} while in flight) — the caller
   * must then skip posting a reply: the shard/actor a late reply would target may already be gone, or
   * worse, a DIFFERENT actor instance may since have reused the same id after a fresh `activate()`. */
  private settleEffect(requestId: string, entry: PendingHostEffect): boolean {
    const actorId = entry.activation.actorId;
    const outstanding = this.outstandingEffectsByActor.get(actorId);
    if (outstanding?.activation !== entry.activation || outstanding.requests.get(requestId) !== entry) return false;
    this.removeEffect(outstanding, entry);
    if (outstanding.requests.size === 0) this.outstandingEffectsByActor.delete(actorId);
    return !entry.controller.signal.aborted && this.activationIsActive(entry.activation);
  }

  /** 🧹️ Aborts and clears every outstanding host-effect for one actor — called by {@link failShard}
   * (whole shard lost) and {@link dispose} (single actor gone), so losing a worker never strands a
   * pending effect. A handler using `signal` (e.g. `fetch`) genuinely stops the underlying work; either
   * way the ledger entry is gone immediately, so a later settle callback for the same `requestId` is
   * recognized as stale by {@link settleEffect} and posts no reply. */
  private abortOutstandingEffects(actorId: string): void {
    const outstanding = this.outstandingEffectsByActor.get(actorId);
    if (!outstanding) return;
    this.outstandingEffectsByActor.delete(actorId);
    while (outstanding.head) this.cancelOneEffect(outstanding);
  }

  private removeEffect(ledger: HostEffectLedger, entry: PendingHostEffect): void {
    if (entry.previous) entry.previous.next = entry.next; else ledger.head = entry.next;
    if (entry.next) entry.next.previous = entry.previous; else ledger.tail = entry.previous;
    ledger.requests.delete(entry.requestId);
    entry.previous = null;
    entry.next = null;
  }

  private cancelOneEffect(ledger: HostEffectLedger): void {
    const entry = ledger.head;
    if (!entry) return;
    entry.controller.abort();
    this.removeEffect(ledger, entry);
  }

  /** 📤️ Posts one `ShardFrame::Envelope` DOWN to the worker — `kernel`→`actorId`, mirroring
   * {@link handleEffectRequest}'s own up-going shape exactly (`to`/`from`/`lane`/`seq`/`deadlineMs`/
   * `coalesce`/`cancelOf`/`payload`) with `payload.kind` `"effect-complete"`/`"effect-error"`.
   * Fire-and-forget on the wire (posted directly via `slot.worker.postMessage`, never through
   * {@link send}) — `🟨️shard-worker.js`'s own dispatch settles the guest's Promise and sends nothing
   * back, so awaiting a `"result"` here would hang forever. The fresh `requestId` on the OUTER
   * `OutboundMessage` only satisfies that message kind's shape; the worker's effect-complete/
   * effect-error branch never reads it (🧪️ terra-web-bridges: it dispatches on
   * `frame.envelope.payload.kind` alone, before the generic `requestId` gate). */
  private postEffectReply(activation: ShardActivation, kind: "effect-complete" | "effect-error", innerPayload: unknown): void {
    if (!this.activationIsActive(activation)) return;
    const { actorId, slot, generation } = activation;
    this.effectReplySeq += 1;
    const frame: ShardFrame = {
      kind: "Envelope",
      envelope: { to: actorId, from: { kind: "kernel" }, lane: "Background", seq: this.effectReplySeq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload: innerPayload } },
    };
    slot.worker.postMessage({ kind: "frame", requestId: this.nextRequestId(), actorId, activationGeneration: generation, frame } satisfies OutboundMessage);
  }

  private replyEffectComplete(activation: ShardActivation, requestId: string, value: unknown): void {
    this.postEffectReply(activation, "effect-complete", { requestId, value });
  }

  private replyEffectError(activation: ShardActivation, requestId: string, message: string): void {
    this.postEffectReply(activation, "effect-error", { requestId, message });
  }
  //#endregion 🌉️HostEffectBridge

  //#region ⏱️HeartbeatWatchdog
  private recordHeartbeat(slot: ShardSlot, turnSeq: number, atMs: number): void {
    slot.heartbeat.lastHeartbeatAtMs = atMs;
    slot.heartbeat.lastHeartbeatTurnSeq = turnSeq;
    slot.heartbeat.missedCount = 0;
    slot.heartbeat.lastMissCountedAtMs = atMs;
  }

  /** 🔭️ SAB path: polls every shard's `Atomics.load` slot and folds any advance into the SAME state
   * machine `postMessage` heartbeats update — call alongside `checkHeartbeats` on a scheduler tick.
   * A no-op when this client was built without a `heartbeatSab` (postMessage is the only source then;
   * see this class's header doc — correctness never depends on this method being called at all). */
  pollHeartbeatSab(nowMs: number = this.now()): void {
    if (!this.heartbeatSabView) return;
    for (const slot of this.shards) {
      const seq = Atomics.load(this.heartbeatSabView, slot.index);
      if (seq !== slot.heartbeat.lastHeartbeatTurnSeq || slot.heartbeat.oldestPendingStartedAtMs === null) {
        this.recordHeartbeat(slot, seq, nowMs);
      }
    }
  }

  /** 🚑️ design-runtime.md §1 `FailurePolicy` watchdog: a shard only "misses" a heartbeat while it has
   * an in-flight turn/job older than `heartbeatTimeoutMs` with no fresher heartbeat since — an idle
   * shard with nothing pending can never be flagged. Three consecutive timeout windows of continued
   * silence (not three calls to this method) trigger `terminate()` + `rebuild()` and `onShardLost`. */
  checkHeartbeats(nowMs: number = this.now()): void {
    for (const slot of this.shards) {
      const pendingSince = slot.heartbeat.oldestPendingStartedAtMs;
      if (pendingSince === null) continue;
      if (slot.heartbeat.lastHeartbeatAtMs >= pendingSince) continue;
      const silentForMs = nowMs - pendingSince;
      if (silentForMs <= this.heartbeatTimeoutMs) continue;
      if (nowMs - slot.heartbeat.lastMissCountedAtMs < this.heartbeatTimeoutMs) continue;
      slot.heartbeat.missedCount += 1;
      slot.heartbeat.lastMissCountedAtMs = nowMs;
      if (slot.heartbeat.missedCount >= HEARTBEAT_MISSED_LIMIT) {
        const actorIds = [...slot.actorIds];
        this.terminate(slot.index);
        this.rebuild(slot.index);
        this.onShardLost?.(slot.index, actorIds);
      }
    }
  }

  /** ▶️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P4): self-ticks {@link pollHeartbeatSab} +
   * {@link checkHeartbeats} on a real interval. Before this method existed, NEITHER was ever called
   * outside this file's own tests (verified with `grep -rn "checkHeartbeats(\|pollHeartbeatSab("` across
   * the repo) — the watchdog's whole failure ladder was wired but nothing in production ever turned
   * the crank, so a wedged shard went undetected forever in the real app. Idempotent: calling this
   * again while already running is a no-op (use {@link stopWatchdog} first to change the interval).
   * Uses the real `setInterval`/`clearInterval` — same convention as this repo's own
   * `ActivationRegistry.startRuntimeMetricsPublisher` (kernel `🟦️component.ts`), which tests with
   * `vi.useFakeTimers()`/`vi.advanceTimersByTime` rather than an injected interval function; this
   * class's `now` option already covers the OTHER half of the clock (what "too long ago" means), so
   * no separate injectable timer is needed for correctness, only fake timers for tests. */
  startWatchdog(intervalMs: number = this.watchdogIntervalMs): void {
    if (this.watchdogHandle !== null) return;
    this.watchdogHandle = setInterval(() => {
      this.pollHeartbeatSab();
      this.checkHeartbeats();
    }, intervalMs);
  }

  /** ⏹️ Cancels a running {@link startWatchdog} loop. Idempotent; also called from {@link disposeAll}. */
  stopWatchdog(): void {
    if (this.watchdogHandle === null) return;
    clearInterval(this.watchdogHandle);
    this.watchdogHandle = null;
  }
  //#endregion ⏱️HeartbeatWatchdog

  //#region 📈️RuntimeMetrics
  /** 📈️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): one `ShardMetricsSample` per shard, built
   * purely from state this class already tracks — `actorIds.size` (`ShardMetrics.actors`),
   * `pendingRequestIds.size / actorIds.size` (`busyRatio`: the fraction of this shard's resident
   * actors with an in-flight `turn()` right now — the same "busy" proxy `checkHeartbeats` already
   * uses via `oldestPendingStartedAtMs`), and `nowMs - lastHeartbeatAtMs` (`heartbeatAgeMs`; a shard
   * that has never heartbeated reports `Number.POSITIVE_INFINITY`, matching `freshHeartbeatState`'s
   * own "never `nowMs`" convention). Field-compatible with the Rust `ShardMetricsSample` the native
   * host publishes — see that type's own doc comment for why `heartbeatAgeMs` needs a host overlay
   * there but not here (this class IS the thing holding the heartbeat clock on web). */
  shardMetricsSamples(nowMs: number = this.now()): readonly ShardMetricsSample[] {
    return this.shards.map((slot) => {
      const actors = slot.actorIds.size;
      const busyRatio = actors > 0 ? slot.pendingRequestIds.size / actors : 0;
      const heartbeatAgeMs = Number.isFinite(slot.heartbeat.lastHeartbeatAtMs) ? Math.max(0, nowMs - slot.heartbeat.lastHeartbeatAtMs) : Number.POSITIVE_INFINITY;
      return { shard: slot.index, metrics: { actors, busyRatio, heartbeatAgeMs } };
    });
  }
  //#endregion 📈️RuntimeMetrics

  //#region 🔁️TerminateRebuild
  /** 🔪️ Kills shard `index`'s worker and rejects every in-flight request on it. Actor routing entries
   * (`actorShard`) are left pointing at this now-dead index deliberately — `rebuild()` respawns a
   * fresh worker at the SAME index, so a caller's already-resolved `shardIndexFor(actorId)` stays
   * valid once the caller re-`activate()`s (from checkpoint) on the rebuilt shard. */
  terminate(index: number): readonly string[] {
    const slot = this.shards[index];
    if (!slot) throw new Error(`[DEBUG] ShardClient.terminate: no shard ${index}`);
    const actorIds = [...slot.actorIds];
    this.failShard(slot, new Error(`shard ${index} terminated`));
    slot.worker.terminate();
    return actorIds;
  }

  /** 🌱️ Respawns shard `index` with a fresh worker. Every actor formerly on it needs a fresh
   * `activate()` + `restore()` from its last checkpoint — this class does not do that itself (it has
   * no checkpoint bytes to restore from; that's the kernel-side `ActivationRegistry`'s job), so it
   * clears the routing entries for whoever was there rather than silently leaving them dangling. */
  rebuild(index: number): void {
    const old = this.shards[index];
    if (!old) throw new Error(`[DEBUG] ShardClient.rebuild: no shard ${index}`);
    for (const actorId of old.actorIds) this.actorShard.delete(actorId);
    this.shards[index] = this.spawnShard(index);
  }
  //#endregion 🔁️TerminateRebuild

  /** ⏏️ Tears down every shard's worker — call on full app shutdown, never per-actor (use {@link dispose}). */
  disposeAll(): void {
    this.stopWatchdog();
    for (const slot of this.shards) {
      this.failShard(slot, new Error("ShardClient disposed"));
      slot.worker.terminate();
    }
  }
}
//#endregion 🧵️ShardClient

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it, vi } = import.meta.vitest;

  it("ShardResidentComposition requires the original ledger before creating workers", async () => {
    const { OwnedResidentLedger } = await import("../../../🌱️value/💾️resident/🟦️component.ts");
    const { default: fixture } = await import("../../🏘️composition/🧪️fixture.json"); const { default: schema } = await import("../../🏘️composition/🧬️schema.json");
    const { default: residentSchema } = await import("../../../🌱️value/💾️resident/🧬️schema.json"); const { default: Ajv } = await import("ajv");
    expect(new Ajv({ strict: true }).addSchema(residentSchema).compile(schema)(fixture)).toBe(true);
    const ledger = new OwnedResidentLedger(fixture.capacity); let workers = 0;
    const createWorker = () => { workers++; return new FakeShardWorker(0); };
    for (const candidate of [undefined, { capacity: fixture.capacity }, Object.create(OwnedResidentLedger.prototype), new Proxy(ledger, {})]) {
      expect(() => Reflect.construct(ShardClient, [{ shardCount: 1, createWorker, residentLedger: candidate }])).toThrow("actor-resident.invalid-ledger");
      expect(workers).toBe(fixture.boundaries.workersBeforeValidLedger);
    }
    const client: ShardClient = Reflect.construct(ShardClient, [{ shardCount: 1, createWorker, residentLedger: ledger }]);
    expect(ShardClient.matchesResidentLedger(client, ledger)).toBe(fixture.binding.sameLedger);
    expect(ShardClient.matchesResidentLedger(client, new OwnedResidentLedger(fixture.capacity))).toBe(fixture.binding.foreignLedger);
    expect(ShardClient.matchesResidentLedger(Object.create(ShardClient.prototype), ledger)).toBe(fixture.binding.structuralClient);
    expect(Object.keys(client).includes(fixture.binding.requiredOption)).toBe(fixture.binding.publicLedgerProperty);
    expect(workers).toBe(1); expect(ledger.usage).toEqual({ data: { bytes: 0, slots: 0, owners: 0 }, control: { bytes: 0, slots: 0, owners: 0 } }); client.disposeAll();
  });

  it("ShardResidentComposition preadmits the exact shared pool record without reusing a child grant", async () => {
    const { default: fixture } = await import("../../🏘️composition/🧪️fixture.json"); const { produce } = await import("immer");
    const ledger = new OwnedResidentLedger(fixture.capacity); const foreign = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const grant = { maxItems: 1, maxBytes: 256 }; const row = fixture.poolPreparation;
    expect(client.prepareUiResidentPool(foreign, grant).kind).toBe("rejected"); expect(foreign.usage.data).toEqual({ bytes: 0, slots: 0, owners: 0 });
    for (const refused of [{ maxItems: 0, maxBytes: 256 }, { maxItems: 1, maxBytes: 255 }]) expect(client.prepareUiResidentPool(ledger, refused).kind).toBe("blocked");
    expect(ledger.usage.data).toEqual({ bytes: 0, slots: 0, owners: 0 }); const first = client.prepareUiResidentPool(ledger, grant);
    expect(first).toMatchObject({ kind: "pending", items: 1, bytes: row.prepareBytes }); expect(ledger.usage.data).toEqual(row.total);
    expect(uiResidentMetadataEnvelope("pool")).toEqual(row.uiEnvelope);
    const expected = produce({ bytes: 0, slots: 0, owners: 0 }, value => { for (const envelope of [row.controllerEnvelope, row.uiEnvelope, row.intrinsicEnvelope]) { value.bytes += envelope.bytes; value.slots += envelope.slots; value.owners += envelope.owners; } }); expect(expected).toEqual(row.total);
    const second = client.prepareUiResidentPool(ledger, grant); expect(second).toMatchObject({ kind: "ready", items: 0, bytes: row.samePreparedBytes }); expect(ledger.usage.data).toEqual(row.total);
    expect(client.prepareUiResidentPool(foreign, grant).kind).toBe("rejected"); expect(client.ownsUiResidentPool({})).toBe(false); client.disposeAll();
  });

  it("ShardResidentComposition releases only its actual pool's private terminal witness", async () => {
    const { OwnedUiResidentPool } = await import("../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../🏘️composition/🧪️fixture.json");
    const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const { client: foreign } = harness(1, { residentLedger: ledger }); const grant = { maxItems: 1, maxBytes: 256 }; const row = fixture.poolLifecycle;
    expect(OwnedUiResidentPool.begin(client, ledger, grant).step.kind).toBe("pending"); const admitted = OwnedUiResidentPool.begin(client, ledger, grant); expect(admitted.step.kind).toBe("ready"); const pool = admitted.pool; if (!pool) throw new Error("Actual pool admission missing");
    expect(Object.keys(pool)).toEqual(row.publicCapabilityKeys); expect(client.ownsUiResidentPool(pool)).toBe(true); expect(OwnedUiResidentPool.begin(client, ledger, grant).step.kind).toBe(row.postConstructionRepeat);
    expect(client.releaseUiResidentPool(pool, pool.retirement, grant).kind).toBe(row.premature);
    let reads = 0; const fabricated = { get terminal() { reads++; return true; } }; expect(client.releaseUiResidentPool(pool, fabricated, grant).kind).toBe(row.structural); expect(reads).toBe(0);
    OwnedUiResidentPool.begin(foreign, ledger, grant); const other = OwnedUiResidentPool.begin(foreign, ledger, grant).pool; if (!other) throw new Error("Foreign actual pool missing"); other.beginClose(); expect(other.closeStep(grant).kind).toBe("complete");
    expect(client.releaseUiResidentPool(pool, other.retirement, grant).kind).toBe(row.foreign); expect(ledger.usage.data.bytes).toBe(fixture.poolPreparation.total.bytes * 2);
    pool.beginClose(); expect(pool.closeStep(grant).kind).toBe("complete"); const witness = pool.retirement; expect(witness).not.toBeNull();
    for (const refused of [{ maxItems: 0, maxBytes: 256 }, { maxItems: 1, maxBytes: 63 }]) expect(client.releaseUiResidentPool(pool, witness, refused).kind).toBe("blocked");
    for (let index = 0; index < row.releaseBytes.length; index++) { const result = client.releaseUiResidentPool(pool, witness, { maxItems: 1, maxBytes: row.releaseBytes[index]! }); expect(result).toMatchObject({ kind: row.releaseKinds[index], bytes: row.releaseBytes[index], items: 1 }); }
    expect(client.ownsUiResidentPool(pool)).toBe(false); expect(client.releaseUiResidentPool(pool, witness, grant).kind).toBe(row.replay); expect(ledger.usage.data).toEqual(fixture.poolPreparation.total);
    for (const bytes of row.releaseBytes) foreign.releaseUiResidentPool(other, other.retirement, { maxItems: 1, maxBytes: bytes }); expect(ledger.usage.data).toEqual({ bytes: 0, slots: 0, owners: 0 }); client.disposeAll(); foreign.disposeAll();
  });

  it("ShardResidentComposition retains rejected canonical admission and closes only that unused record", async () => {
    const { OwnedResidentRecord } = await import("../../../🌱️value/💾️resident/🟦️component.ts"); const { default: fixture } = await import("../../🏘️composition/🧪️fixture.json"); const { produce } = await import("immer");
    const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const { client: peer } = harness(1, { residentLedger: ledger }); const grant = { maxItems: 1, maxBytes: 256 }; const row = fixture.rejectedPreparation;
    expect(peer.prepareUiResidentPool(ledger, grant).kind).toBe("pending"); const before = ledger.usage.data; const original: { record: OwnedResidentRecord | null } = { record: null }; const freeze = Object.freeze;
    const trap = vi.spyOn(Object, "freeze").mockImplementation(value => { const frozen = freeze(value); if (value instanceof OwnedResidentRecord) { original.record = value; throw null; } return frozen; });
    let refused: ResidentStep; try { refused = client.prepareUiResidentPool(ledger, grant); } finally { trap.mockRestore(); }
    expect(original.record).not.toBeNull(); expect(refused).toMatchObject({ kind: row.step, items: 1, bytes: 256 });
    const doubled = produce({ ...before }, usage => { usage.bytes *= row.recordsAfterRefusal; usage.slots *= row.recordsAfterRefusal; usage.owners *= row.recordsAfterRefusal; }); expect(ledger.usage.data).toEqual(doubled);
    expect(client.prepareUiResidentPool(ledger, grant).kind).toBe(row.retry); const pool = OwnedUiResidentPool.begin(client, ledger, grant); expect(pool.step.kind).toBe(row.retry); expect(pool.pool).toBe(row.pool); expect(ledger.usage.data).toEqual(doubled);
    for (const blocked of [{ maxItems: 0, maxBytes: 256 }, { maxItems: 1, maxBytes: 63 }]) expect(client.closeUiResidentPoolStep(blocked).kind).toBe("blocked");
    for (let index = 0; index < row.releaseBytes.length; index++) { const current = client.closeUiResidentPoolStep({ maxItems: 1, maxBytes: row.releaseBytes[index]! }); expect(current).toMatchObject({ kind: row.releaseKinds[index], items: 1, bytes: row.releaseBytes[index] }); }
    expect(original.record!.terminalIsEmpty()).toBe(true); expect(ledger.usage.data).toEqual(before); expect(peer.prepareUiResidentPool(ledger, grant).kind).toBe("ready");
    const live = ledger.reserveRecord("data", { bytes: 0, slots: 0, owners: 0 }, grant); expect(live.step.kind === "ready").toBe(row.ledgerStillOpen); if (!live.record) throw new Error("Live peer admission missing"); live.record.beginClose(); expect(live.record.closeStep(grant).kind).toBe("complete");
    for (const bytes of row.releaseBytes) peer.closeUiResidentPoolStep({ maxItems: 1, maxBytes: bytes }); expect(ledger.usage.data).toEqual({ bytes: 0, slots: 0, owners: 0 }); client.disposeAll(); peer.disposeAll();
  });

  it("ShardResidentComposition parent closes its installed pool with separate child and proof turns", async () => {
    const { default: fixture } = await import("../../🏘️composition/🧪️fixture.json"); const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const grant = { maxItems: 1, maxBytes: 256 }; const row = fixture.parentClose;
    expect(OwnedUiResidentPool.begin(client, ledger, grant).step.kind).toBe("pending"); const pool = OwnedUiResidentPool.begin(client, ledger, grant).pool; if (!pool) throw new Error("Original pool missing");
    const close = vi.spyOn(OwnedUiResidentPool.prototype, "closeStep"); const retirement = vi.spyOn(OwnedUiResidentPool.prototype, "retirement", "get");
    try {
      for (let index = 0; index < row.releaseBytes.length; index++) {
        close.mockClear(); retirement.mockClear(); const current = client.closeUiResidentPoolStep({ maxItems: 1, maxBytes: row.releaseBytes[index]! });
        expect(current).toMatchObject({ kind: row.releaseKinds[index], items: 1, bytes: row.releaseBytes[index] }); expect(close.mock.calls.length).toBeLessThanOrEqual(row.maxChildCallsPerTurn); if (close.mock.calls.length) expect(retirement).not.toHaveBeenCalled();
      }
      expect(pool.terminalIsEmpty()).toBe(true); expect(client.ownsUiResidentPool(pool)).toBe(false); expect(ledger.usage.data).toEqual({ bytes: 0, slots: 0, owners: 0 }); expect(client.closeUiResidentPoolStep(grant)).toMatchObject({ kind: "complete", items: 0, bytes: row.retryBytes });
    } finally { close.mockRestore(); retirement.mockRestore(); client.disposeAll(); }
  });

  it("ShardResidentComposition preserves every thrown value after an actual parent close transition", async () => {
    const { default: fixture } = await import("../../🏘️composition/🧪️fixture.json"); const { OwnedResidentRecord } = await import("../../../🌱️value/💾️resident/🟦️component.ts"); const row = fixture.parentFault; const grant = { maxItems: 1, maxBytes: 256 };
    let getterReads = 0; const values = new Map<string, unknown>([["null", null], ["undefined", undefined], ["false", false], ["zero", 0], ["object", { payload: new Uint8Array(8193), get message() { getterReads++; return "unread"; } }]]);
    for (const name of row.values) {
      const ledger = new OwnedResidentLedger(fixture.capacity); const { client } = harness(1, { residentLedger: ledger }); const { client: peer } = harness(1, { residentLedger: ledger }); expect(client.prepareUiResidentPool(ledger, grant).kind).toBe("pending"); const before = ledger.usage.data;
      const begin = OwnedResidentRecord.prototype.beginClose; const fault = values.get(name); let transitions = 0;
      const trap = vi.spyOn(OwnedResidentRecord.prototype, "beginClose").mockImplementation(function (this: OwnedResidentRecord) { Reflect.apply(begin, this, []); transitions++; throw fault; });
      try { expect(client.closeUiResidentPoolStep(grant).kind).toBe(row.first); } finally { trap.mockRestore(); }
      expect(transitions).toBe(1); expect(client.closeUiResidentPoolStep(grant)).toMatchObject({ kind: row.retry, phase: row.phase, items: 0, bytes: 0 }); expect(ledger.usage.data).toEqual(before); expect(getterReads).toBe(row.getterReads);
      expect(peer.prepareUiResidentPool(ledger, grant).kind === "pending").toBe(row.independentPeer); for (const bytes of fixture.rejectedPreparation.releaseBytes) peer.closeUiResidentPoolStep({ maxItems: 1, maxBytes: bytes }); expect(ledger.usage.data).toEqual(before); client.disposeAll(); peer.disposeAll();
    }
  });

  class FakeShardWorker implements ShardWorkerLike {
    readonly index: number;
    onmessage: ((event: { readonly data: unknown }) => void) | null = null;
    onerror: ((event: unknown) => void) | null = null;
    readonly sent: unknown[] = [];
    terminated = false;
    constructor(index: number) {
      this.index = index;
    }
    postMessage(message: unknown): void {
      this.sent.push(message);
    }
    terminate(): void {
      this.terminated = true;
    }
    deliver(message: InboundMessage): void {
      this.onmessage?.({ data: message });
    }
  }

  function harness(shardCount = 2, extra?: Partial<ShardClientOptions>) {
    const workers: FakeShardWorker[] = [];
    let nowMs = 0;
    const client = new ShardClient({
      residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }),
      shardCount,
      createWorker: (index) => {
        const worker = new FakeShardWorker(index);
        workers.push(worker);
        return worker;
      },
      now: () => nowMs,
      ...extra,
    });
    return { client, workers, advance: (ms: number) => (nowMs += ms), setNow: (ms: number) => (nowMs = ms) };
  }

  const BUDGET: ShardBudget = { fuel: 1000, wallMs: 4, memoryBytes: 1 << 20, uiNodes: 100, mailboxLen: 16, maxEffects: 8, maxPatchBytes: 1 << 16 };
  const fixtureHosts = new WeakMap<ShardInstanceLifecycleLease, OwnedUiInstance>();
  function bindFixtureHost(lease: ShardInstanceLifecycleLease): OwnedUiInstance {
    const host = new OwnedUiInstance(lease.activation, lease.lifetime!, { maxNodes: 128, maxDepth: 16, maxChildren: 32, maxTextBytes: 4096, maxPatchOps: 128, maxPatchBytes: 65536 }, { usizeBits: 32 });
    lease.bindHostRetirement(host);
    fixtureHosts.set(lease, host);
    return host;
  }
  function fixtureRetirement(lease: ShardInstanceLifecycleLease): OwnedUiInstanceRetirement {
    const host = fixtureHosts.get(lease)!;
    host.beginClose();
    for (let step = 0; step < 32 && !host.terminalIsEmpty(); step += 1) host.closeStep({ maxItems: 1, maxBytes: 4096 });
    const witness = host.takeRetirementWitness();
    if (!witness) throw new Error("fixture host retirement did not complete");
    return witness;
  }

  async function answerLifecycle(worker: FakeShardWorker, pending: Promise<unknown>, receipt?: ActorInstanceLifecycleReceipt): Promise<unknown> {
    const message = worker.sent.at(-1) as { requestId: string };
    worker.deliver({ kind: "result", requestId: message.requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: receipt ? encodeActorInstanceLifecycle(receipt) : undefined } });
    return pending;
  }

  async function captureFixtureInstance(client: ShardClient, worker: FakeShardWorker, actorId: string, instanceId = 7, guestLifetime = 13n): Promise<ShardInstanceLifecycleLease> {
    const lease = client.captureInstanceLifecycle(actorId, instanceId);
    const captured: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: { activationGeneration: lease.activation.activationGeneration, instanceId, guestLifetime }, requestSequence: lease.openRequest.requestSequence };
    await answerLifecycle(worker, lease.open({ appId: "fixture", actor: "fixture", config: [], assets: [], capabilities: [], quotas: [] }, BUDGET), captured);
    await answerLifecycle(worker, lease.acknowledge(captured, BUDGET));
    bindFixtureHost(lease);
    return lease;
  }

  async function retireFixtureInstance(worker: FakeShardWorker, lease: ShardInstanceLifecycleLease): Promise<void> {
    const request = lease.beginClose();
    const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: lease.lifetime!, requestSequence: request.requestSequence, closeGeneration: 31n };
    if (!lease.pendingReceipt) await answerLifecycle(worker, lease.close(BUDGET), accepted);
    const retired: ActorInstanceLifecycleReceipt = { ...accepted, kind: "retired" };
    await answerLifecycle(worker, lease.acknowledge(lease.pendingReceipt!, BUDGET), retired);
    await answerLifecycle(worker, lease.acknowledge(retired, BUDGET, fixtureRetirement(lease)));
  }

  //#region 🚪️CapturedLifecycle
  describe("ShardClient reserved response settlement", () => {
    it("captures the exact response before actual pending removal, heartbeat recomputation and caller settlement", async () => {
      const { default: fixture } = await import("../../🪪️activation/🚪️instance/📥️output/🧪️fixture.json");
      const { client, workers } = harness(1); const worker = workers[0]!;
      const slot: ShardSlot = Reflect.get(client, "shards")[0];
      const send = Reflect.get(client, "send").bind(client) as (slot: ShardSlot, message: OutboundMessage, request: string, posted: undefined, output: OwnedActorTurnOutput) => Promise<unknown>;
      const pendingEntries: Map<string, PendingEntry> = Reflect.get(client, "pending");
      const queue = new OwnedActorTurnOutputs({}, fixture.capacity); const output = queue.reserve()!;
      const raw = { kind: "result" as const, requestId: "reserved-fixture", ok: true as const, value: { uiPatches: [] }, unknown: new Uint8Array(fixture.responseSettlement.unknownPayloadBytes) };
      const order: string[] = [];
      const remove = pendingEntries.delete.bind(pendingEntries);
      pendingEntries.delete = key => { expect(output.responseEnvelope).toBe(raw); order.push("pending-removal"); return remove(key); };
      const recompute = Reflect.get(client, "recomputeOldestPending").bind(client);
      Reflect.set(client, "recomputeOldestPending", (target: ShardSlot) => { expect(output.responseEnvelope).toBe(raw); order.push("heartbeat-recompute"); return recompute(target); });
      const result = output.run(() => send(slot, { kind: "turn", requestId: raw.requestId, actorId: "reserved-fixture", activationGeneration: 1n, events: [], budget: BUDGET }, raw.requestId, undefined, output));
      const observed = expect(result.then(() => { expect(output.responseEnvelope).toBe(raw); order.push("external-continuation"); throw new Error("fixture publication fault"); })).rejects.toThrow("fixture publication fault");
      worker.deliver(raw); await observed;
      expect(order).toEqual(fixture.responseSettlement.phases.filter(phase => phase !== "capture" && phase !== "error-graft"));
      expect(output.outcome?.value).toBe(raw.value); expect(queue.peek()?.responseEnvelope).toBe(raw);
      expect(pendingEntries.has(raw.requestId)).toBe(false); client.disposeAll();
    });

    it("retains the original failed response when actual worker error grafting throws", async () => {
      const { default: fixture } = await import("../../🪪️activation/🚪️instance/📥️output/🧪️fixture.json");
      const { client, workers } = harness(1); const worker = workers[0]!;
      const slot: ShardSlot = Reflect.get(client, "shards")[0];
      const send = Reflect.get(client, "send").bind(client) as (slot: ShardSlot, message: OutboundMessage, request: string, posted: undefined, output: OwnedActorTurnOutput) => Promise<unknown>;
      const queue = new OwnedActorTurnOutputs({}, fixture.capacity); const output = queue.reserve()!;
      const raw = { kind: "result" as const, requestId: "graft-fixture", ok: false as const, error: "native refusal", stack: "native stack", framesBytes: 8192, unknown: new Uint8Array(fixture.responseSettlement.unknownPayloadBytes) };
      const result = output.run(() => send(slot, { kind: "turn", requestId: raw.requestId, actorId: "graft-fixture", activationGeneration: 1n, events: [], budget: BUDGET }, raw.requestId, undefined, output));
      const failure = new Error("fixture diagnostic fault"); const observed = expect(result).rejects.toBe(failure);
      const log = vi.spyOn(console, "log").mockImplementation(() => { expect(output.responseEnvelope).toBe(raw); throw failure; });
      try { expect(() => worker.deliver(raw)).not.toThrow(); await observed; } finally { log.mockRestore(); }
      expect(output.responseEnvelope).toBe(raw); expect(output.outcome?.value).toBe(failure); expect(raw.unknown.byteLength).toBe(fixture.responseSettlement.unknownPayloadBytes);
      expect(queue.peek()).toBe(output); client.disposeAll();
    });
  });

  describe("ShardClient captured return authority", () => {
    async function captured() {
      const { default: row } = await import("../../🪪️activation/📤️return/🧪️fixture.json");
      const { client, workers } = harness(2, { exclusiveShardCount: 1 });
      const pending = client.activate(row.actorId, "https://fixture.invalid/component.js", [], BUDGET);
      const worker = workers[0]!;
      worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: undefined });
      await pending;
      const instance = await captureFixtureInstance(client, worker, row.actorId);
      return { row, client, workers, worker, instance };
    }

    async function inputStream(lifetime: ActorInstanceLifetime, payloadBytes?: number) {
      const { default: vector } = await import("../../../🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧪️fixture.json");
      const name = "@webassemblyjs/leb128/lib/leb.js"; const lib = await import(name); const encode = (lib.default ?? lib).encodeUIntBuffer;
      const uint = (n: bigint | number) => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(n)); return Buffer.from(encode(bytes)); };
      const frame = (tag: number, body: Buffer) => Buffer.concat([Buffer.of(tag), uint(body.length), body]);
      const receipt = { lifetime, patchSequence: BigInt(vector.patchSequence) };
      const authority = Buffer.concat([uint(lifetime.activationGeneration), uint(lifetime.instanceId), uint(lifetime.guestLifetime), uint(receipt.patchSequence)]);
      expect(authority).toEqual(Buffer.from(encodeActorUiPatchReceipt(receipt)));
      const surface = Buffer.from(vector.surface); const payload = payloadBytes === undefined ? Buffer.from(vector.payloadHex, "hex") : Buffer.alloc(payloadBytes);
      if (payloadBytes !== undefined) for (let index = 0; index < payload.length; index++) payload[index] = (index * 37 + 11) % 256;
      const bytes = Buffer.concat([Buffer.from("73727401", "hex"), frame(0, Buffer.of(0, 0, 1, 0, 0)), frame(2, Buffer.concat([authority, uint(surface.length), surface, Buffer.of(0, 1, 1)])), frame(3, Buffer.concat([Buffer.of(vector.opcode), uint(BigInt(vector.node)), uint(payload.length), payload])), frame(4, Buffer.alloc(0)), frame(7, Buffer.alloc(13)), frame(9, Buffer.alloc(0))]);
      return { bytes, receipt, vector, payload };
    }
    async function deliveredInput(foreign?: "activationGeneration" | "instanceId" | "guestLifetime", payloadBytes?: number, pageLength?: number) {
      const context = await captured(); const { instance, worker, row } = context;
      const lifetime = { ...instance.lifetime! };
      if (foreign === "instanceId") lifetime.instanceId++; else if (foreign) lifetime[foreign]++;
      const stream = await inputStream(lifetime, payloadBytes);
      const source = instance.reserveReturn(row.responseSlots); const pending = source.execute([], BUDGET);
      const request = worker.sent.at(-1) as { requestId: string };
      const { encodeActorReturnResult } = await import("../../📤️return/🟦️component.ts");
      const identity = { origin: source.origin!, returnSequence: BigInt(row.returnSequence) };
      const pageBytes = pageLength === undefined ? stream.bytes : stream.bytes.subarray(0, pageLength);
      const response = { kind: "result" as const, requestId: request.requestId, ok: true as const, value: encodeActorReturnResult({ kind: "page", receipt: { identity, pageSequence: 1n, length: pageBytes.length, final: pageBytes.length === stream.bytes.length }, page: createActorBytePage(pageBytes) }) };
      worker.deliver(response); const report = await pending;
      return { ...context, ...stream, source, response, report };
    }
    it("OwnedKernelReturnInput keeps decoded page storage behind its private facade", async () => {
      const { client, instance, source, response, report, vector } = await deliveredInput();
      expect(report.kind).toBe("page");
      expect("page" in report).toBe(vector.boundaries.publicDecodedPage);
      expect(Object.isFrozen(report)).toBe(true);
      expect(source.page?.byteAt(0)).toBe(0x73);
      expect(OwnedShardReturnPage.matchesOwner(source.page, fixtureHosts.get(instance)!, instance.activation, instance.lifetime!)).toBe(true);
      expect(capturedReturnState(source).outputs.peek()?.responseEnvelope).toBe(response);
      client.disposeAll();
    });
    it("OwnedKernelReturnInput captures one content owner and exact grammar-selected private field", async () => {
      const { OwnedKernelReturnContent, OwnedKernelReturnInputField, OwnedKernelReturnInputFragment } = await import("../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts");
      const { default: schema } = await import("../../../🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧪️schema.json");
      const { default: Ajv } = await import("ajv");
      const { client, instance, source, response, receipt, payload, vector } = await deliveredInput();
      expect(new Ajv({ strict: true }).compile(schema)(vector)).toBe(true);
      const host = fixtureHosts.get(instance)!;
      const input = new OwnedKernelReturnContent(source, host, instance.activation, instance.lifetime!);
      expect(source.content).toBe(input);
      expect(() => new OwnedKernelReturnContent(source, host, instance.activation, instance.lifetime!)).toThrow(/content-owned/);
      expect(input.advance({ maxItems: 0, maxBytes: 4096 }).kind).toBe("blocked");
      for (let turn = 0; turn < 256 && input.field === null; turn++) {
        const step = input.advance({ maxItems: 1, maxBytes: 4096 });
        expect(step.items).toBeLessThanOrEqual(1); expect(step.bytes).toBeLessThanOrEqual(1);
      }
      const field = input.field!;
      expect(field.value).toEqual({ operation: 0, opcode: vector.opcode, node: BigInt(vector.node), name: vector.field, byteLength: BigInt(payload.length), receipt });
      expect(OwnedKernelReturnInputField.matchesOwner(field, host, instance.activation, instance.lifetime!)).toBe(true);
      expect(OwnedKernelReturnInputField.matchesOwner({ value: field.value }, host, instance.activation, instance.lifetime!)).toBe(false);
      expect(() => Reflect.construct(OwnedKernelReturnInputField, [field.value])).toThrow();
      const fragment = field.fragment!;
      expect(fragment.offset).toBe(0n); expect(fragment.length).toBe(payload.length); expect(fragment.field).toBe(field);
      expect(OwnedKernelReturnInputFragment.matches(fragment, field)).toBe(true);
      expect(OwnedKernelReturnInputFragment.matches({ field }, field)).toBe(false);
      expect(() => fragment.byteAt(0, {})).toThrow(/builder/);
      expect(fragment.release({})).toBeNull(); expect(field.fragment).toBe(fragment);
      input.beginClose(); expect(input.advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("blocked");
      expect(() => fragment.byteAt(0, {})).toThrow();
      expect(source.content).toBe(input); expect(input.field).toBe(field); expect(field.fragment).toBe(fragment);
      expect(capturedReturnState(source).outputs.peek()?.responseEnvelope).toBe(response); client.disposeAll();
    });
    it("OwnedKernelReturnInput refuses foreign concrete hosts and patch lifetime fields before field mint", async () => {
      const { OwnedKernelReturnContent } = await import("../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts");
      for (const key of ["activationGeneration", "instanceId", "guestLifetime"] as const) {
        const { client, instance, source, response } = await deliveredInput(key);
        const host = fixtureHosts.get(instance)!;
        const foreign = new OwnedUiInstance(instance.activation, instance.lifetime!, { maxNodes: 1, maxDepth: 1, maxChildren: 1, maxTextBytes: 16, maxPatchOps: 1, maxPatchBytes: 256 }, { usizeBits: 32 });
        expect(() => new OwnedKernelReturnContent(source, foreign, instance.activation, instance.lifetime!)).toThrow(/owner/);
        expect(source.content).toBeNull();
        const input = new OwnedKernelReturnContent(source, host, instance.activation, instance.lifetime!);
        for (let turn = 0; turn < 256 && input.failure === null; turn++) input.advance({ maxItems: 1, maxBytes: 4096 });
        expect(input.failure).toMatch(/patch-lifetime/); expect(input.field).toBeNull();
        expect(capturedReturnState(source).outputs.peek()?.responseEnvelope).toBe(response); client.disposeAll();
      }
    });
    it("OwnedKernelReturnInput bounds a large field to the exact currently captured page range", async () => {
      const { OwnedKernelReturnContent } = await import("../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts");
      const { default: vector } = await import("../../../🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧪️fixture.json");
      const { client, instance, source, bytes, payload, response } = await deliveredInput(undefined, vector.crossPage.payloadBytes, vector.crossPage.firstPageBytes);
      const input = new OwnedKernelReturnContent(source, fixtureHosts.get(instance)!, instance.activation, instance.lifetime!);
      for (let turn = 0; turn < 256 && input.field === null; turn++) input.advance({ maxItems: 1, maxBytes: 4096 });
      const field = input.field!; const fragment = field.fragment!;
      const start = bytes.indexOf(payload);
      expect(start).toBeGreaterThan(0);
      expect(field.value.byteLength).toBe(BigInt(vector.crossPage.payloadBytes));
      expect(fragment.length).toBe(vector.crossPage.firstPageBytes - start);
      expect(fragment.length).toBeLessThanOrEqual(vector.crossPage.maximumFragmentBytes);
      expect(fragment.offset).toBe(0n);
      expect("acknowledgeInput" in source).toBe(vector.crossPage.inputAckBeforeCopy);
      expect(input.advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("ready");
      expect(field.fragment).toBe(fragment); expect(capturedReturnState(source).outputs.peek()?.responseEnvelope).toBe(response);
      input.beginClose(); expect(field.fragment).toBe(fragment); client.disposeAll();
    });

    it("OwnedKernelReturnInput privately identifies its bound builder after the real bind call throws", async () => {
      const { OwnedKernelReturnContent, OwnedKernelReturnInputField } = await import("../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts");
      const { OwnedUiResidentPool } = await import("../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts");
      const { OwnedUiOperationPayloadBuilder } = await import("../../../🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts");
      const { default: schema } = await import("../../../🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧪️schema.json");
      const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
      const { client, instance, source, response, vector } = await deliveredInput();
      expect(new Ajv({ strict: true }).compile(schema)(vector)).toBe(true);
      const owner = fixtureHosts.get(instance)!; const activation = instance.activation; const lifetime = instance.lifetime!;
      const input = new OwnedKernelReturnContent(source, owner, activation, lifetime);
      for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 });
      const field = input.field!; const fragment = field.fragment!;
      const pool = new OwnedUiResidentPool({ maxResidentBytes: 8192, maxPages: 32, maxOwners: 64 });
      const resident = pool.bindInstance(owner, activation, lifetime)!.beginPayload()!;
      expect(OwnedKernelReturnInputField.matchesBuilder(field, null)).toBe(vector.binding.nullBuilder);
      expect(OwnedKernelReturnInputField.matchesBuilder(field, {})).toBe(vector.binding.before);
      const captured: { builder: object | null; bound: boolean } = { builder: null, bound: false };
      const bind = OwnedKernelReturnInputField.prototype.bind;
      const intercept = vi.spyOn(OwnedKernelReturnInputField.prototype, "bind").mockImplementation(function (this: typeof field, builder) {
        captured.builder = builder; captured.bound = bind.call(this, builder);
        if (captured.bound) throw new Error("fixture after actual native binding"); return false;
      });
      try {
        const admission = OwnedUiOperationPayloadBuilder.begin(owner, activation, lifetime, field, resident, { maxItems: 1, maxBytes: 4096 });
        expect(admission.builder).toBeNull(); expect(admission.step.kind).toBe("rejected");
      } finally { intercept.mockRestore(); }
      expect(captured.bound).toBe(true); expect(captured.builder).not.toBeNull();
      expect(OwnedKernelReturnInputField.matchesBuilder(field, captured.builder)).toBe(vector.binding.afterBindThenThrow);
      const observed = produce({ bound: vector.binding.before }, state => { state.bound = captured.bound; });
      expect(OwnedKernelReturnInputField.matchesBuilder(field, captured.builder)).toBe(observed.bound);
      field.beginClose(); expect(OwnedKernelReturnInputField.matchesBuilder(field, captured.builder)).toBe(vector.binding.afterBeginClose);
      expect(OwnedKernelReturnInputField.matchesBuilder(field, {})).toBe(vector.binding.foreignBuilder);
      expect(OwnedKernelReturnInputField.matchesBuilder(field, null)).toBe(vector.binding.nullBuilder);
      let reads = 0; const forged = { get builder() { reads++; return captured.builder; } };
      expect(OwnedKernelReturnInputField.matchesBuilder(forged, captured.builder)).toBe(vector.binding.forgedField);
      expect(OwnedKernelReturnInputField.matchesBuilder(Object.create(OwnedKernelReturnInputField.prototype), captured.builder)).toBe(false);
      expect(OwnedKernelReturnInputField.matchesBuilder(new Proxy(field, { has() { reads++; return true; }, get() { reads++; return captured.builder; } }), captured.builder)).toBe(false);
      expect(reads).toBe(vector.binding.publicReads); expect(field.fragment).toBe(fragment);
      expect(capturedReturnState(source).outputs.peek()?.responseEnvelope).toBe(response); client.disposeAll();
    });

    it("OwnedKernelReturnInput advances no framing on unread or genuinely cancelled fragments", async () => {
      const { OwnedKernelReturnContent } = await import("../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts");
      const { OwnedUiResidentPool } = await import("../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts");
      const { OwnedUiOperationPayloadBuilder } = await import("../../../🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts");
      const { default: schema } = await import("../../../🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧪️schema.json");
      const { default: Ajv } = await import("ajv");
      const { client, instance, source, response, vector, payload } = await deliveredInput();
      expect(new Ajv({ strict: true }).compile(schema)(vector)).toBe(true);
      const owner = fixtureHosts.get(instance)!; const activation = instance.activation; const lifetime = instance.lifetime!;
      const input = new OwnedKernelReturnContent(source, owner, activation, lifetime);
      for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 });
      const field = input.field!; const fragment = field.fragment!;
      const pool = new OwnedUiResidentPool({ maxResidentBytes: 8192, maxPages: 32, maxOwners: 64 });
      const scope = pool.bindInstance(owner, activation, lifetime)!; const resident = scope.beginPayload()!;
      const builder = OwnedUiOperationPayloadBuilder.begin(owner, activation, lifetime, field, resident, { maxItems: 1, maxBytes: 4096 }).builder!;
      expect(builder).not.toBeNull();
      expect(Buffer.from(Array.from({ length: fragment.length }, (_, index) => fragment.byteAt(index, builder)))).toEqual(payload);
      expect(field.advance({ maxItems: 1, maxBytes: 4096 }, builder)).toEqual({ kind: vector.continuation.beforeProof, items: 0, bytes: 0 });
      expect(field.consumed).toBe(0n); expect(field.complete).toBe(false);
      builder.beginClose();
      for (let turn = 0; turn < 32 && !builder.terminalIsEmpty(); turn++) builder.closeStep({ maxItems: 1, maxBytes: 4096 });
      expect(builder.terminalIsEmpty()).toBe(true);
      expect(field.advance({ maxItems: 1, maxBytes: 4096 }, builder)).toEqual({ kind: vector.continuation.afterCancellation, items: 0, bytes: 0 });
      expect(field.consumed.toString()).toBe(vector.continuation.cancelledConsumed); expect(field.complete).toBe(vector.continuation.cancelledComplete);
      expect(field.fragment).toBe(fragment); expect(() => fragment.byteAt(0, builder)).toThrow();
      expect("acknowledgeInput" in source).toBe(vector.continuation.pageInputAck);
      expect(source.page).not.toBeNull(); expect(capturedReturnState(source).outputs.peek()?.responseEnvelope).toBe(response);
      pool.beginClose(); for (let turn = 0; turn < 64 && !pool.terminalIsEmpty(); turn++) pool.closeStep({ maxItems: 1, maxBytes: 4096 });
      expect(pool.terminalIsEmpty()).toBe(true); expect(input.field).toBe(field); client.disposeAll();
    });

    it("OwnedKernelReturnInput consumes only privately copied bytes and retains the containing raw page", async () => {
      const { OwnedKernelReturnContent } = await import("../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts");
      const { OwnedUiResidentPool } = await import("../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts");
      const { OwnedUiOperationPayloadBuilder } = await import("../../../🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts");
      const { default: vector } = await import("../../../🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧪️fixture.json");
      for (const length of vector.continuation.copiedPayloadBytes) {
        const { client, instance, source, response, payload } = await deliveredInput(undefined, length);
        const owner = fixtureHosts.get(instance)!; const activation = instance.activation; const lifetime = instance.lifetime!;
        const input = new OwnedKernelReturnContent(source, owner, activation, lifetime);
        for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 });
        const field = input.field!; const fragment = field.fragment!;
        const pool = new OwnedUiResidentPool({ maxResidentBytes: 8192, maxPages: 32, maxOwners: 64 });
        const resident = pool.bindInstance(owner, activation, lifetime)!.beginPayload()!;
        const builder = OwnedUiOperationPayloadBuilder.begin(owner, activation, lifetime, field, resident, { maxItems: 1, maxBytes: 4096 }).builder!;
        let copied = false;
        const copyTurns = length + Math.ceil(length / 256) * 3 + 5;
        for (let turn = 0; turn < copyTurns; turn++) {
          const step = builder.advance({ maxItems: 1, maxBytes: 4096 }); expect(step.kind).not.toBe("rejected");
          if (step.phase === "paged-input-copy-release-retire") { copied = true; break; }
        }
        expect(copied).toBe(true); expect(builder.failure).toBeNull(); expect(field.consumed).toBe(0n); expect(field.complete).toBe(false);
        expect(() => fragment.byteAt(0, builder)).toThrow();
        expect(field.advance({ maxItems: 1, maxBytes: 4096 }, {})).toEqual({ kind: vector.driver.foreignAfterCopy, items: 0, bytes: 0 });
        expect(field.consumed.toString()).toBe(vector.driver.foreignConsumed);
        const unchanged = field.consumed; input.advance({ maxItems: 1, maxBytes: 4096 });
        expect(field.consumed !== unchanged).toBe(vector.driver.contentDrivesField);
        expect(field.advance({ maxItems: 0, maxBytes: 4096 }, builder).kind).toBe("blocked");
        expect(field.advance({ maxItems: 1, maxBytes: 0 }, builder).kind).toBe("blocked");
        let ready = false;
        const continuationTurns = Math.max(1, length) * 2 + 1;
        for (let turn = 0; turn < continuationTurns; turn++) {
          const before = field.consumed; const step = builder.advance({ maxItems: 1, maxBytes: 4096 });
          expect(step.kind).not.toBe("rejected"); expect(step.items).toBeLessThanOrEqual(vector.continuation.maximumItems);
          if (step.phase === "paged-source-advance") { expect(step.bytes).toBeLessThanOrEqual(vector.continuation.maximumBytes); expect(field.consumed - before).toBe(BigInt(step.bytes)); }
          else { expect(step.bytes).toBe(step.kind === "ready" ? 0 : vector.driver.observationBytes); expect(field.consumed).toBe(before); }
          if (step.kind === "ready") { ready = true; break; }
        }
        expect(ready).toBe(true);
        expect(field.complete).toBe(vector.continuation.copiedComplete); expect(field.consumed).toBe(BigInt(length)); expect(field.fragment).toBeNull();
        expect(field.advance({ maxItems: 1, maxBytes: 4096 }, builder)).toEqual({ kind: "complete", items: 0, bytes: 0 });
        const reader = builder.beginRead({ maxItems: 1, maxBytes: 4096 }).reader!; expect(reader).not.toBeNull();
        const actual: number[] = [];
        for (let turn = 0; turn < length + 16; turn++) { const step = reader.advance({ maxItems: 1, maxBytes: 4096 }); if (step.kind === "byte") actual.push(step.value); else if (step.kind === "complete") break; }
        expect(Buffer.from(actual)).toEqual(payload); expect(input.field).toBe(field);
        expect(source.page).not.toBeNull(); expect(capturedReturnState(source).outputs.peek()?.responseEnvelope).toBe(response);
        expect("acknowledgeInput" in source).toBe(vector.continuation.pageInputAck);
        pool.beginClose(); for (let turn = 0; turn < length * 3 + 128 && !pool.terminalIsEmpty(); turn++) pool.closeStep({ maxItems: 1, maxBytes: 4096 });
        expect(pool.terminalIsEmpty()).toBe(true); client.disposeAll();
      }
    });

    it("OwnedKernelReturnInput stops at the exact copied page boundary without fabricating a next range", async () => {
      const { OwnedKernelReturnContent } = await import("../../../🎠️kernel/📤️return/📦️content/📥️input/🟦️component.ts");
      const { OwnedUiResidentPool } = await import("../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts");
      const { OwnedUiOperationPayloadBuilder } = await import("../../../🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts");
      const { default: vector } = await import("../../../🎠️kernel/📤️return/📦️content/📥️input/🪪️authority/🧪️fixture.json");
      const { client, instance, source, response } = await deliveredInput(undefined, vector.crossPage.payloadBytes, vector.crossPage.firstPageBytes);
      const owner = fixtureHosts.get(instance)!; const activation = instance.activation; const lifetime = instance.lifetime!;
      const input = new OwnedKernelReturnContent(source, owner, activation, lifetime);
      for (let turn = 0; turn < 256 && !input.field; turn++) input.advance({ maxItems: 1, maxBytes: 4096 });
      const field = input.field!; const fragment = field.fragment!;
      const pool = new OwnedUiResidentPool({ maxResidentBytes: 8192, maxPages: 32, maxOwners: 64 });
      const resident = pool.bindInstance(owner, activation, lifetime)!.beginPayload()!;
      const builder = OwnedUiOperationPayloadBuilder.begin(owner, activation, lifetime, field, resident, { maxItems: 1, maxBytes: 4096 }).builder!;
      let waiting = false;
      const boundaryTurns = fragment.length * 3 + Math.ceil(fragment.length / 256) * 3 + 6;
      for (let turn = 0; turn < boundaryTurns; turn++) {
        const step = builder.advance({ maxItems: 1, maxBytes: 4096 }); expect(step.kind).not.toBe("rejected");
        if (step.kind === "blocked" && step.phase === "paged-source-continuation") { waiting = true; break; }
      }
      expect(waiting).toBe(true); expect(builder.failure).toBeNull();
      expect(field.consumed).toBe(BigInt(fragment.length)); expect(field.complete).toBe(vector.continuation.pageBoundaryComplete); expect(field.fragment).toBeNull();
      expect(field.advance({ maxItems: 1, maxBytes: 4096 }, builder)).toEqual({ kind: "blocked", items: 0, bytes: 0 });
      expect(builder.advance({ maxItems: 1, maxBytes: 4096 }).kind).toBe("blocked"); expect(builder.beginRead({ maxItems: 1, maxBytes: 4096 }).reader).toBeNull();
      expect(source.page).not.toBeNull(); expect(capturedReturnState(source).outputs.peek()?.responseEnvelope).toBe(response);
      pool.beginClose(); for (let turn = 0; turn < vector.crossPage.firstPageBytes * 3 && !pool.terminalIsEmpty(); turn++) pool.closeStep({ maxItems: 1, maxBytes: 4096 });
      expect(pool.terminalIsEmpty()).toBe(true); expect(field.complete).toBe(false); client.disposeAll();
    });

    it("mints a page only from the original captured response and keeps controls on its old worker", async () => {
      const { default: Ajv } = await import("ajv");
      const { default: schema } = await import("../../🪪️activation/📤️return/🧪️schema.json");
      const { encodeActorReturnResult, decodeActorReturnDrive } = await import("../../📤️return/🟦️component.ts");
      const { row, client, workers, worker, instance } = await captured();
      const oracle = new Ajv({ strict: true }); expect(oracle.validate(schema, row)).toBe(true);
      const source = instance.reserveReturn(row.responseSlots);
      expect(instance.pendingReturn).toBe(source);
      expect(() => instance.reserveReturn(row.responseSlots)).toThrow("actor-return.already-owned");
      const running = source.execute([], BUDGET);
      const execute = worker.sent.at(-1) as { requestId: string; returnDrive: Uint8Array };
      const drive = decodeActorReturnDrive(execute.returnDrive);
      if (drive.kind !== "execute") throw new Error("expected execute");
      expect(drive.origin.requestSequence).toBe(Number(execute.requestId.slice(1)));
      const identity = { origin: drive.origin, returnSequence: BigInt(row.returnSequence) };
      const first = { kind: "result" as const, requestId: execute.requestId, ok: true as const, value: encodeActorReturnResult({ kind: "pending", identity, reason: "working" }) };
      const entries: Map<string, PendingEntry> = Reflect.get(client, "pending"); const remove = entries.delete.bind(entries);
      entries.delete = key => { if (key === execute.requestId) expect(capturedReturnState(source).outputs.peek()?.responseEnvelope).toBe(first); return remove(key); };
      worker.deliver(first); await running;
      client.leaseExclusive(row.actorId);
      const polling = source.poll(BUDGET);
      const poll = worker.sent.at(-1) as { requestId: string; returnDrive: Uint8Array; events: unknown[] };
      expect(poll.events).toEqual([]); expect(decodeActorReturnDrive(poll.returnDrive)).toEqual({ kind: "control", control: { kind: "poll", identity } });
      const receipt = { identity, pageSequence: BigInt(row.pageSequence), length: row.pageBytes.length, final: true };
      const pageResponse = { kind: "result" as const, requestId: poll.requestId, ok: true as const, value: encodeActorReturnResult({ kind: "page", receipt, page: createActorBytePage(Uint8Array.from(row.pageBytes)) }) };
      worker.deliver(pageResponse); await polling;
      const pageOutput = capturedReturnState(source).latest;
      const page = source.page!;
      expect(row.pageBytes.map((_, index) => page.byteAt(index))).toEqual(row.pageBytes);
      expect(OwnedShardReturnPage.matchesOwner(page, fixtureHosts.get(instance)!, instance.activation, instance.lifetime!)).toBe(true);
      expect(OwnedShardReturnPage.matchesOwner({ receipt }, fixtureHosts.get(instance)!, instance.activation, instance.lifetime!)).toBe(false);
      expect(() => Reflect.construct(OwnedShardReturnPage, [receipt])).toThrow("actor-return.private-page");
      await expect(source.execute([], BUDGET)).rejects.toThrow("actor-return.execute-already-owned");
      const cancelling = source.cancel(BUDGET);
      const cancel = worker.sent.at(-1) as { requestId: string; returnDrive: Uint8Array; events: unknown[] };
      expect(cancel.events).toEqual([]);
      worker.deliver({ kind: "result", requestId: cancel.requestId, ok: true, value: encodeActorReturnResult({ kind: "control", control: { kind: "cancel", identity }, outcome: "accepted", fault: "none" }) });
      await cancelling;
      const actual = { originalControlPosts: worker.sent.filter(value => value !== null && typeof value === "object" && Reflect.has(value, "returnDrive")).length - 1, replacementControlPosts: workers[1]!.sent.length, sameOrigin: source.origin?.requestSequence === drive.origin.requestSequence, sameParent: source.page === page && instance.pendingReturn === source, retainedResponses: source.retainedResponses, inputAckAvailable: "acknowledgeInput" in source };
      expect(actual).toEqual(row.expected); expect(oracle.validate({ const: row.expected }, actual)).toBe(true);
      expect(pageOutput?.responseEnvelope).toBe(pageResponse);
      await expect(source.poll(BUDGET)).rejects.toThrow("actor-return.response-capacity");
      expect(source.page).toBe(page); expect(source.retainedResponses).toBe(row.responseSlots);
      expect(() => instance.dispose()).toThrow("actor-close.native-retirement-pending");
      client.disposeAll();
    });

    it("retains foreign replies and refuses replaced workers without redirecting cancellation", async () => {
      const { encodeActorReturnResult, decodeActorReturnDrive } = await import("../../📤️return/🟦️component.ts");
      const { row, client, workers, worker, instance } = await captured();
      const source = instance.reserveReturn(row.responseSlots);
      const running = source.execute([], BUDGET); const observed = expect(running).rejects.toThrow("actor-return.foreign-origin");
      const execute = worker.sent.at(-1) as { requestId: string; returnDrive: Uint8Array };
      const drive = decodeActorReturnDrive(execute.returnDrive); if (drive.kind !== "execute") throw new Error("expected execute");
      const raw = { kind: "result" as const, requestId: execute.requestId, ok: true as const, value: encodeActorReturnResult({ kind: "pending", identity: { origin: { ...drive.origin, requestSequence: drive.origin.requestSequence + 1 }, returnSequence: 1n }, reason: "working" }), unknown: new Uint8Array(8192) };
      worker.deliver(raw); await observed;
      expect(capturedReturnState(source).outputs.peek()?.responseEnvelope).toBe(raw); expect(raw.unknown.byteLength).toBe(8192);
      expect(source.page).toBeNull(); expect(instance.pendingReturn).toBe(source);
      client.terminate(0); client.rebuild(0);
      const before = workers[2]!.sent.length;
      await expect(source.cancel(BUDGET)).rejects.toThrow("actor-return.worker-lost");
      expect(workers[2]!.sent.length).toBe(before); expect(capturedReturnState(source).outputs.peek()?.responseEnvelope).toBe(raw);
      client.disposeAll();
    });

    it("retries a refused execute with its frozen original origin and retains the refused owner", async () => {
      const { encodeActorReturnResult, decodeActorReturnDrive } = await import("../../📤️return/🟦️component.ts");
      const { row, client, worker, instance } = await captured();
      const source = instance.reserveReturn(row.responseSlots); const post = worker.postMessage.bind(worker);
      worker.postMessage = () => { throw new Error("fixture return post refusal"); };
      await expect(source.execute([], BUDGET)).rejects.toThrow("fixture return post refusal");
      const origin = source.origin!; expect(source.retainedResponses).toBe(1); expect(instance.pendingReturn).toBe(source);
      worker.postMessage = post;
      const retried = source.retry(BUDGET); const message = worker.sent.at(-1) as { requestId: string; returnDrive: Uint8Array };
      expect(Number(message.requestId.slice(1))).toBeGreaterThan(origin.requestSequence);
      expect(decodeActorReturnDrive(message.returnDrive)).toEqual({ kind: "execute", origin });
      worker.deliver({ kind: "result", requestId: message.requestId, ok: true, value: encodeActorReturnResult({ kind: "pending", identity: { origin, returnSequence: BigInt(row.returnSequence) }, reason: "working" }) });
      await retried; expect(source.origin).toBe(origin); expect(source.retainedResponses).toBe(2);
      client.disposeAll();
    });

    it("keeps raw envelopes private and blocks other turn paths while the captured return is owned", async () => {
      const { row, client, instance } = await captured();
      const source = instance.reserveReturn(row.responseSlots);
      expect("firstResponse" in source || "latestResponse" in source).toBe(row.boundaries.publicRawResponse);
      await expect(instance.activation.turn([], BUDGET)).rejects.toThrow("actor-return.already-owned");
      await expect(instance.poll(BUDGET)).rejects.toThrow("actor-return.retirement-pending");
      client.disposeAll();
    });

    it("observes original worker loss even after the routing roster moved away", async () => {
      const { encodeActorReturnResult } = await import("../../📤️return/🟦️component.ts");
      const { row, client, worker, instance } = await captured();
      const source = instance.reserveReturn(row.responseSlots); const running = source.execute([], BUDGET);
      const request = worker.sent.at(-1) as { requestId: string };
      const identity = { origin: source.origin!, returnSequence: BigInt(row.returnSequence) };
      worker.deliver({ kind: "result", requestId: request.requestId, ok: true, value: encodeActorReturnResult({ kind: "pending", identity, reason: "working" }) });
      await running; client.leaseExclusive(row.actorId);
      worker.onerror?.(new Error("original worker failed after route move"));
      const before = worker.sent.length;
      const failure = source.cancel(BUDGET).then(() => null, error => error);
      if (worker.sent.length !== before) {
        const request = worker.sent.at(-1) as { requestId: string };
        worker.deliver({ kind: "result", requestId: request.requestId, ok: true, value: encodeActorReturnResult({ kind: "control", control: { kind: "cancel", identity }, outcome: "accepted", fault: "none" }) });
      }
      expect((await failure)?.message).toBe("actor-return.worker-lost");
      expect(worker.sent.length - before).toBe(row.boundaries.lostOriginalWorkerControlPosts);
      expect(instance.pendingReturn).toBe(source); client.disposeAll();
    });

    it("retains cancellation authority after a malformed response with a previously captured identity", async () => {
      const { encodeActorReturnResult } = await import("../../📤️return/🟦️component.ts");
      const { row, client, worker, instance } = await captured();
      const source = instance.reserveReturn(row.responseSlots); const running = source.execute([], BUDGET);
      let request = worker.sent.at(-1) as { requestId: string };
      const identity = { origin: source.origin!, returnSequence: BigInt(row.returnSequence) };
      worker.deliver({ kind: "result", requestId: request.requestId, ok: true, value: encodeActorReturnResult({ kind: "pending", identity, reason: "working" }) }); await running;
      const polling = source.poll(BUDGET); const observed = expect(polling).rejects.toThrow();
      request = worker.sent.at(-1) as { requestId: string };
      const malformed = { kind: "result" as const, requestId: request.requestId, ok: true as const, value: Uint8Array.of(255), unknown: new Uint8Array(8192) };
      worker.deliver(malformed); await observed;
      const retained = capturedReturnState(source).latest;
      client.leaseExclusive(row.actorId);
      const cancellation = source.cancel(BUDGET).then(() => true, () => false);
      request = worker.sent.at(-1) as { requestId: string };
      if (request.requestId !== malformed.requestId) worker.deliver({ kind: "result", requestId: request.requestId, ok: true, value: encodeActorReturnResult({ kind: "control", control: { kind: "cancel", identity }, outcome: "accepted", fault: "none" }) });
      expect(await cancellation).toBe(row.boundaries.cancellationAfterMalformedResult);
      expect(retained?.responseEnvelope).toBe(malformed); expect(malformed.unknown.byteLength).toBe(8192); client.disposeAll();
    });
  });

  describe("ShardClient captured instance lifecycle", () => {
    const openInput = { appId: "fixture", actor: "fixture", config: new Uint8Array(), assets: [], capabilities: [], quotas: new Uint8Array() };
    async function activateCaptured(client: ShardClient, worker: FakeShardWorker, actorId = "captured") {
      const activated = client.activate(actorId, "https://fixture.invalid/actor.js", [], BUDGET);
      worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: undefined });
      await activated;
      return client.captureInstanceLifecycle(actorId, 7);
    }
    async function answer(worker: FakeShardWorker, pending: Promise<unknown>, receipt?: ActorInstanceLifecycleReceipt) {
      const message = worker.sent.at(-1) as { requestId: string };
      worker.deliver({ kind: "result", requestId: message.requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: receipt ? encodeActorInstanceLifecycle(receipt) : undefined } });
      return pending;
    }
    async function openCaptured(worker: FakeShardWorker, lease: ShardInstanceLifecycleLease, guestLifetime = 13n) {
      const captured: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: { activationGeneration: lease.activation.activationGeneration, instanceId: 7, guestLifetime }, requestSequence: lease.openRequest.requestSequence };
      await answer(worker, lease.open(openInput, BUDGET), captured);
      await answer(worker, lease.acknowledge(captured, BUDGET));
      return captured;
    }
    it("joins canonical captured accepted retired and exact ACK with host retirement", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const fixture = JSON.parse(readFileSync(new URL("../../🪪️activation/🚪️instance/🧪️fixture.json", import.meta.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL("../../🪪️activation/🚪️instance/🧪️schema.json", import.meta.url), "utf8"));
      const oracle = new Ajv({ strict: true });
      expect(oracle.validate(schema, fixture)).toBe(true);
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      const phases = [lease.progress().kind];
      expect(lease.lifetime).toBeNull();
      expect(() => lease.beginClose()).toThrow("actor-lifecycle.capture-pending");
      expect(() => client.dispose("captured")).toThrow("actor-close.native-retirement-pending");
      const captured: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: { activationGeneration: 1n, instanceId: fixture.instanceId, guestLifetime: BigInt(fixture.guestLifetime) }, requestSequence: lease.openRequest.requestSequence };
      await answer(worker, lease.open(openInput, BUDGET), captured);
      phases.push(lease.progress().kind);
      expect(lease.lifetime).toEqual(captured.lifetime);
      expect(Object.isFrozen(lease.lifetime)).toBe(true);
      await answer(worker, lease.acknowledge(captured, BUDGET));
      phases.push(lease.progress().kind);
      const host = new OwnedUiInstance(lease.activation, lease.lifetime!, { maxNodes: 128, maxDepth: 16, maxChildren: 32, maxTextBytes: 4096, maxPatchOps: 128, maxPatchBytes: 65536 }, { usizeBits: 32 });
      lease.bindHostRetirement(host);
      const close = lease.beginClose();
      phases.push(lease.progress().kind);
      expect(lease.beginClose()).toBe(close);
      await expect(lease.activation.turn([{ kind: "request", payload: {} }], BUDGET)).rejects.toThrow("actor-activation.revoked");
      const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: captured.lifetime, requestSequence: close.requestSequence, closeGeneration: 41n };
      await answer(worker, lease.close(BUDGET), accepted);
      phases.push(lease.progress().kind);
      const retired: ActorInstanceLifecycleReceipt = { ...accepted, kind: "retired" };
      await answer(worker, lease.acknowledge(accepted, BUDGET), retired);
      phases.push(lease.progress().kind);
      const before = worker.sent.length;
      await expect(lease.acknowledge(retired, BUDGET)).rejects.toThrow("actor-lifecycle.host-retirement-pending");
      expect(worker.sent).toHaveLength(before);
      expect(() => client.dispose("captured")).toThrow("actor-close.native-retirement-pending");
      host.beginClose();
      expect(host.closeStep({ maxItems: 1, maxBytes: 4096 }).kind).toBe("complete");
      const witness = host.takeRetirementWitness()!;
      await expect(lease.acknowledge(retired, BUDGET, Object.create(OwnedUiInstanceRetirement.prototype))).rejects.toThrow("actor-lifecycle.host-retirement-pending");
      await answer(worker, lease.acknowledge(retired, BUDGET, witness));
      phases.push(lease.progress().kind);
      expect(phases).toEqual(fixture.phases);
      expect(oracle.validate({ const: fixture.phases }, phases)).toBe(true);
      const events = worker.sent.flatMap(value => (value as { events?: ShardEventEnvelope[] }).events ?? []);
      expect(events.map(event => event.kind)).toEqual(["instance-open", "instance-lifecycle-ack", "instance-close", "instance-lifecycle-ack", "instance-lifecycle-ack"]);
      expect(events[1]!.payload).toEqual({ kind: "ack", receipt: captured });
      client.dispose("captured");
    });
    it("keeps exact lifecycle authority after route revocation and rejects a replacement guest receipt", async () => {
      const { client, workers } = harness(2, { exclusiveShardCount: 1 });
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      await openCaptured(worker, lease);
      client.leaseExclusive("captured");
      const close = lease.beginClose();
      const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: lease.lifetime!, requestSequence: close.requestSequence, closeGeneration: 41n };
      const pending = lease.close(BUDGET);
      const requestId = (worker.sent.at(-1) as { requestId: string }).requestId;
      workers[1]!.deliver({ kind: "result", requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle(accepted) } });
      expect(lease.pendingReceipt).toBeNull();
      await answer(worker, pending, accepted);
      expect(workers[1]!.sent).toHaveLength(0);
      await expect(answer(worker, lease.acknowledge(accepted, BUDGET), { ...accepted, kind: "retired", lifetime: { ...accepted.lifetime, guestLifetime: 14n } })).rejects.toThrow("actor-lifecycle.receipt-mismatch");
      expect(lease.progress().kind).not.toBe("complete");
      client.disposeAll();
    });
    it("retains close authority after transport failure and worker loss without admitting commands", async () => {
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      await openCaptured(worker, lease);
      const request = lease.beginClose();
      const post = worker.postMessage.bind(worker);
      worker.postMessage = () => { throw new Error("fixture transport refused"); };
      await expect(lease.close(BUDGET)).rejects.toThrow("fixture transport refused");
      expect(lease.progress()).toEqual({ kind: "blocked", failure: "transport-refused" });
      expect(lease.beginClose()).toBe(request);
      worker.postMessage = post;
      const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: lease.lifetime!, requestSequence: request.requestSequence, closeGeneration: 41n };
      await answer(worker, lease.close(BUDGET), accepted);
      const pending = lease.acknowledge(accepted, BUDGET);
      const observed = expect(pending).rejects.toThrow("terminated");
      const requestId = (worker.sent.at(-1) as { requestId: string }).requestId;
      client.terminate(0);
      await observed;
      client.rebuild(0);
      const replacement = await activateCaptured(client, workers[1]!);
      worker.deliver({ kind: "result", requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle({ ...accepted, kind: "retired" }) } });
      expect(lease.progress()).toEqual({ kind: "blocked", failure: "worker-lost" });
      expect(replacement.lifetime).toBeNull();
      await expect(lease.poll(BUDGET)).rejects.toThrow("actor-lifecycle.worker-lost");
      client.disposeAll();
    });
    it("mints patch authority only from the exact captured turn and rejects forged UI ACKs", async () => {
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      await openCaptured(worker, lease);
      const patch = { surface: { instance: 7, surface: "main" }, revision: 1n, baseRevision: 0n, ops: [{ tag: "set-root", val: 0n }] };
      const result = { uiPatches: [patch], uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime: lease.lifetime!, patchSequence: 1n }), lifecycleReceipt: undefined, status: { tag: "idle" } };
      const pending = lease.poll(BUDGET);
      worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: result });
      expect(await pending).toBe(result);
      expect(() => lease.captureUiPatchAuthority({ ...result }, 0)).toThrow("actor-lifecycle.foreign-turn");
      expect(() => lease.captureUiPatchAuthority(result, 1)).toThrow("actor-lifecycle.patch-index");
      const source = lease.captureUiPatchAuthority(result, 0);
      expect(lease.captureUiPatchAuthority(result, 0)).toBe(source);
      expect(OwnedNativeUiPatchAuthority.matches(source, lease.activation, lease.lifetime!)).toBe(true);
      expect(OwnedNativeUiPatchAuthority.matches(Object.create(OwnedNativeUiPatchAuthority.prototype), lease.activation, lease.lifetime!)).toBe(false);
      expect(source.operation(0)).toBe(patch.ops[0]);
      expect(source.acceptInput({} as never)).toBe(false);
      expect(source.releaseInput({ ordinal: 0 } as OwnedUiPatchInputRetirement)).toBe(false);
      expect(source.inputRetired).toBe(false);
      expect(source.value).toMatchObject({ surface: "main", revision: 1, baseRevision: 0, operationCount: 1 });
      const before = worker.sent.length;
      await expect(lease.submitUiAcknowledgement(source, {} as OwnedUiPatchAcknowledgement, BUDGET)).rejects.toThrow("actor-lifecycle.ui-ack-mismatch");
      expect(worker.sent).toHaveLength(before);
      expect(OwnedNativeUiPatchSubmissionReceipt.matches(Object.create(OwnedNativeUiPatchSubmissionReceipt.prototype), source, {})).toBe(false);
      client.disposeAll();
    });
    it("requires the exact producer UI patch receipt and retains original claims on malformed or duplicate turns", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../../🚪️lifetime/🩹️patch/🧪️fixture.json", import.meta.url), "utf8"));
      const { client, workers } = harness(1); const worker = workers[0]!; const lease = await activateCaptured(client, worker); await openCaptured(worker, lease);
      const receipt = { lifetime: lease.lifetime!, patchSequence: BigInt(fixture.vectors[1].value.patchSequence) };
      const patch = { surface: { instance: 7, surface: fixture.feedback.surface }, revision: 1n, baseRevision: 0n, ops: [] };
      async function returned(uiPatchReceipt: unknown, patches: unknown[] = [patch]) {
        const result = { status: { tag: "idle" }, uiPatches: patches, uiPatchReceipt };
        const pending = lease.poll(BUDGET);
        worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: result });
        expect(await pending).toBe(result); return result;
      }
      const validBytes = encodeActorUiPatchReceipt(receipt);
      const invalid: unknown[] = [undefined, null, new Uint8Array(36), encodeActorUiPatchReceipt({ ...receipt, lifetime: { ...receipt.lifetime, guestLifetime: receipt.lifetime.guestLifetime + 1n } }), encodeActorUiPatchReceipt({ ...receipt, lifetime: { ...receipt.lifetime, activationGeneration: receipt.lifetime.activationGeneration + 1n } })];
      for (const hex of fixture.invalidHex) invalid.push(Buffer.from(hex, "hex"));
      for (const wire of invalid) {
        const result = await returned(wire);
        expect(() => lease.captureUiPatchAuthority(result, 0)).toThrow();
        expect(result.uiPatches[0]).toBe(patch); expect(result.uiPatchReceipt).toBe(wire);
      }
      const overfull = await returned(validBytes, [patch, patch]);
      expect(() => lease.captureUiPatchAuthority(overfull, 0)).toThrow("actor-ui-patch.pairing");
      const original = await returned(validBytes); const source = lease.captureUiPatchAuthority(original, 0);
      expect(source.value.receipt).toEqual(receipt); expect(Object.isFrozen(source.value.receipt)).toBe(true); expect(Object.isFrozen(source.value.receipt.lifetime)).toBe(true);
      expect(lease.captureUiPatchAuthority(original, 0)).toBe(source);
      const duplicate = await returned(validBytes);
      expect(() => lease.captureUiPatchAuthority(duplicate, 0)).toThrow("actor-ui-patch.duplicate-sequence");
      expect(lease.captureUiPatchAuthority(original, 0)).toBe(source); expect(duplicate.uiPatches[0]).toBe(patch);
      validBytes.fill(0); expect(source.value.receipt).toEqual(receipt);
      client.disposeAll();
    });
    it("retains ordinary turn output on its exact captured instance through settlement revocation", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../../🪪️activation/🚪️instance/🧪️fixture.json", import.meta.url), "utf8"));
      for (const outcome of fixture.ordinaryOutput) {
        const { client, workers } = harness(1); const worker = workers[0]!; const lease = await activateCaptured(client, worker); await openCaptured(worker, lease);
        const result = { uiPatches: [{ surface: { instance: fixture.instanceId, surface: "main" }, revision: 1n, baseRevision: 0n, ops: [] }], uiPatchReceipt: encodeActorUiPatchReceipt({ lifetime: lease.lifetime!, patchSequence: 1n }), effects: [], status: { tag: "idle" } };
        const pending = client.turn(lease.activation.actorId, [], BUDGET);
        const observed = outcome === "revoked" ? expect(pending).rejects.toThrow("actor-activation.revoked") : expect(pending).resolves.toBe(result);
        if (outcome === "revoked") lease.beginClose();
        worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: result });
        await observed;
        const source = lease.captureUiPatchAuthority(result, 0); expect(OwnedNativeUiPatchAuthority.matches(source, lease.activation, lease.lifetime!)).toBe(true);
        if (outcome === "revoked") expect(lease.interruptedTurn).toBe(result);
        client.disposeAll();
      }
    });

    it("retains exact receipt authority on faulted refused clock-fault and malformed ACK turns", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../../🪪️activation/🚪️instance/🧪️fixture.json", import.meta.url), "utf8"));
      for (const status of fixture.invalidAckStatuses) {
        const { client, workers } = harness(1);
        const worker = workers[0]!;
        const lease = await activateCaptured(client, worker);
        const captured: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: { activationGeneration: 1n, instanceId: 7, guestLifetime: 13n }, requestSequence: lease.openRequest.requestSequence };
        await answer(worker, lease.open(openInput, BUDGET), captured);
        const pending = lease.acknowledge(captured, BUDGET);
        const rejected = expect(pending).rejects.toThrow("actor-lifecycle.ack-not-admitted");
        worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: { status } });
        await rejected;
        expect(lease.pendingReceipt).toEqual(captured);
        expect(() => lease.beginClose()).toThrow("actor-lifecycle.capture-pending");
        await answer(worker, lease.acknowledge(captured, BUDGET));
        expect(lease.progress().kind).toBe("open");
        client.disposeAll();
      }
    });
    it("cancels only the captured activation one effect per lifecycle turn", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const fixture = JSON.parse(readFileSync(new URL("../../🪪️activation/🚪️instance/🧪️fixture.json", import.meta.url), "utf8"));
      const signals: AbortSignal[] = [];
      const { client, workers } = harness(1, { onHostEffect: (_actor, _effect, _params, signal) => { signals.push(signal); return new Promise(() => {}); } });
      const worker = workers[0]!;
      const lease = await activateCaptured(client, worker);
      await openCaptured(worker, lease);
      for (const requestId of ["one", "two"]) worker.deliver(makeEffectRequestFrame("captured", "storage-read", requestId, {}));
      lease.beginClose();
      const counts = [signals.filter(signal => signal.aborted).length];
      await answer(worker, lease.poll(BUDGET));
      counts.push(signals.filter(signal => signal.aborted).length);
      await answer(worker, lease.poll(BUDGET));
      counts.push(signals.filter(signal => signal.aborted).length);
      expect(counts).toEqual(fixture.closeCancellation);
      expect(new Ajv().validate({ const: fixture.closeCancellation }, counts)).toBe(true);
      client.terminate(0);
      client.rebuild(0);
      const replacement = await activateCaptured(client, workers[1]!);
      await openCaptured(workers[1]!, replacement);
      workers[1]!.deliver(makeEffectRequestFrame("captured", "storage-read", "replacement", {}, replacement.activation.activationGeneration));
      expect(() => lease.beginClose()).not.toThrow();
      expect(signals.at(-1)!.aborted).toBe(false);
      client.disposeAll();
    });
    it("cannot close an old open owner through the replacement activation effect ledger", async () => {
      const signals: AbortSignal[] = [];
      const { client, workers } = harness(1, { onHostEffect: (_actor, _effect, _params, signal) => { signals.push(signal); return new Promise(() => {}); } });
      const old = await activateCaptured(client, workers[0]!);
      await openCaptured(workers[0]!, old);
      client.terminate(0);
      client.rebuild(0);
      const fresh = await activateCaptured(client, workers[1]!);
      await openCaptured(workers[1]!, fresh);
      workers[1]!.deliver(makeEffectRequestFrame("captured", "storage-read", "replacement", {}, fresh.activation.activationGeneration));
      expect(() => old.beginClose()).toThrow("actor-lifecycle.worker-lost");
      expect(signals[0]!.aborted).toBe(false);
      client.disposeAll();
    });
  });
  //#endregion 🚪️CapturedLifecycle

  //#region 🪪️ActivationLease
  describe("ShardClient activation lease", () => {
    async function fixture() {
      const { readFileSync } = await import("node:fs");
      return JSON.parse(readFileSync(new URL("../../🪪️activation/🧪️fixture.json", import.meta.url), "utf8")) as { actorId: string; instanceId: number; revocations: Array<{ action: string; expected: { activeBefore: boolean; activeAfter: boolean; newTurns: number } }> };
    }

    async function activate(client: ShardClient, worker: FakeShardWorker, actorId: string) {
      const pending = client.activate(actorId, "https://fixture.invalid/actor.js", [], BUDGET);
      const message = worker.sent.at(-1) as { requestId: string };
      worker.deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
      await pending;
      return client.captureActorActivation(actorId);
    }

    it("validates the neutral cases and captures only a ready immutable activation", async () => {
      const { default: Ajv } = await import("ajv");
      const { readFileSync } = await import("node:fs");
      const row = await fixture();
      const schema = JSON.parse(readFileSync(new URL("../../🪪️activation/🧪️schema.json", import.meta.url), "utf8"));
      expect(new Ajv().validate(schema, row)).toBe(true);
      const { client, workers } = harness(1);
      expect(() => client.captureActorActivation(row.actorId)).toThrow("actor-activation.not-ready");
      const pending = client.activate(row.actorId, "https://fixture.invalid/actor.js", [], BUDGET);
      expect(() => client.captureActorActivation(row.actorId)).toThrow("actor-activation.not-ready");
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await pending;
      const lease = client.captureActorActivation(row.actorId);
      expect(Object.isFrozen(lease)).toBe(true);
      expect(lease.actorId).toBe(row.actorId);
      expect(lease.activationGeneration).toBe(1n);
      expect(() => lease.assertActive()).not.toThrow();
      expect(workers[0]!.sent).toHaveLength(1);
      client.dispose(row.actorId);
    });

    it("revokes each close loss or disposal without dispatching another operation", async () => {
      const { default: Ajv } = await import("ajv");
      const row = await fixture();
      const oracle = new Ajv();
      for (const test of row.revocations) {
        const { client, workers } = harness(1);
        const worker = workers[0]!;
        const lease = await activate(client, worker, row.actorId);
        lease.assertActive();
        const close = test.action.startsWith("close") ? await captureFixtureInstance(client, worker, row.actorId, row.instanceId) : null;
        const post = worker.postMessage.bind(worker);
        if (test.action === "close-refused") worker.postMessage = () => { throw new Error("fixture transport refused"); };
        if (close) {
          close.beginClose();
          if (test.action === "close-refused") await expect(close.close(BUDGET)).rejects.toThrow("fixture transport refused");
        }
        else if (test.action === "dispose") client.dispose(row.actorId);
        else if (test.action === "worker-error") worker.onerror?.(new Error("fixture worker lost"));
        else if (test.action === "terminate") client.terminate(0);
        else client.disposeAll();
        const before = worker.sent.length;
        let activeAfter = true;
        try { lease.assertActive(); } catch { activeAfter = false; }
        await expect(lease.turn([], BUDGET)).rejects.toThrow("actor-activation.revoked");
        const actual = { activeBefore: true, activeAfter, newTurns: worker.sent.length - before };
        expect(actual, test.action).toEqual(test.expected);
        expect(oracle.validate({ const: test.expected }, actual), test.action).toBe(true);
        if (close) {
          expect(close.progress().kind).toBe(test.action === "close" ? "closing" : "blocked");
          worker.postMessage = post;
          await retireFixtureInstance(worker, close);
          client.dispose(row.actorId);
        }
      }
    });

    it("never refreshes an old lease when the same actor name is activated again", async () => {
      const { actorId } = await fixture();
      const { client, workers } = harness(1);
      const old = await activate(client, workers[0]!, actorId);
      client.dispose(actorId);
      const fresh = await activate(client, workers[0]!, actorId);
      expect(fresh.activationGeneration).toBe(old.activationGeneration + 1n);
      expect(() => old.assertActive()).toThrow("actor-activation.revoked");
      expect(() => fresh.assertActive()).not.toThrow();
      client.dispose(actorId);
    });

    it("never refreshes an old lease after replacement of its worker at the same slot", async () => {
      const { actorId } = await fixture();
      const { client, workers } = harness(1);
      const old = await activate(client, workers[0]!, actorId);
      client.terminate(0);
      client.rebuild(0);
      const fresh = await activate(client, workers[1]!, actorId);
      expect(workers[1]!.index).toBe(workers[0]!.index);
      expect(() => old.assertActive()).toThrow("actor-activation.revoked");
      expect(() => fresh.assertActive()).not.toThrow();
      client.dispose(actorId);
    });

    it("permanently revokes a released route even when the same worker route is reacquired", async () => {
      const { actorId, instanceId } = await fixture();
      const { client, workers } = harness(2, { exclusiveShardCount: 1 });
      client.leaseExclusive(actorId);
      const operation = await activate(client, workers[1]!, actorId);
      const close = await captureFixtureInstance(client, workers[1]!, actorId, instanceId);
      client.releaseExclusive(actorId);
      expect(client.leaseExclusive(actorId)).toBe(1);
      expect(() => operation.assertActive()).toThrow("actor-activation.revoked");
      expect(() => client.captureActorActivation(actorId)).toThrow("actor-activation.revoked");
      close.beginClose();
      expect(close.progress().kind).toBe("closing");
      await retireFixtureInstance(workers[1]!, close);
      client.dispose(actorId);
    });

    it("keeps the old close root after moving operations onto an exclusive route", async () => {
      const { actorId, instanceId } = await fixture();
      const { client, workers } = harness(2, { exclusiveShardCount: 1 });
      const operation = await activate(client, workers[0]!, actorId);
      const close = await captureFixtureInstance(client, workers[0]!, actorId, instanceId);
      client.leaseExclusive(actorId);
      expect(() => operation.assertActive()).toThrow("actor-activation.revoked");
      close.beginClose();
      expect(workers[1]!.sent).toHaveLength(0);
      await retireFixtureInstance(workers[0]!, close);
      expect(close.progress().kind).toBe("complete");
      client.disposeAll();
    });

    it("disposes the captured worker after a moved or released route and preserves refusal for retry", async () => {
      const { default: rows } = await import("../../🪪️activation/🧪️fixture.json");
      const { default: schema } = await import("../../🪪️activation/🧪️schema.json");
      const { default: Ajv } = await import("ajv");
      const oracle = new Ajv({ strict: true });
      expect(oracle.validate(schema, rows)).toBe(true);
      for (const row of rows.disposals) {
        const { client, workers } = harness(2, { exclusiveShardCount: 1 });
        const original = workers[row.route === "released" ? 1 : 0]!;
        const other = workers[row.route === "released" ? 0 : 1]!;
        if (row.route === "released") client.leaseExclusive(rows.actorId);
        const operation = await activate(client, original, rows.actorId);
        const owner = await captureFixtureInstance(client, original, rows.actorId, rows.instanceId);
        if (row.route === "released") client.releaseExclusive(rows.actorId); else client.leaseExclusive(rows.actorId);
        expect(() => operation.assertActive()).toThrow("actor-activation.revoked");
        await retireFixtureInstance(original, owner);
        const route = client.shardIndexFor(rows.actorId);
        const post = original.postMessage.bind(original);
        if (row.refuseFirst) {
          original.postMessage = () => { throw new Error("fixture dispose refused"); };
          expect(() => client.dispose(rows.actorId)).toThrow("fixture dispose refused");
          expect(client.shardIndexFor(rows.actorId)).toBe(route);
          expect(owner.progress().kind).toBe("complete");
          await expect(client.activate(rows.actorId, "https://fixture.invalid/replacement.js", [], BUDGET)).rejects.toThrow("actor-close.activation-already-owned");
          original.postMessage = post;
        }
        client.dispose(rows.actorId);
        const isDisposal = (value: unknown): value is Extract<OutboundMessage, { kind: "dispose" }> => value !== null && typeof value === "object" && Reflect.get(value, "kind") === "dispose";
        const messages = original.sent.filter(isDisposal);
        expect(messages.length, row.route).toBe(row.expected.originalPosts);
        expect(other.sent.filter(isDisposal).length, row.route).toBe(row.expected.otherPosts);
        const routeReleased = client.shardIndexFor(rows.actorId) === undefined;
        const fresh = await activate(client, workers[0]!, rows.actorId);
        const before = workers.reduce((count, worker) => count + worker.sent.length, 0);
        owner.dispose();
        let replacementActive = true;
        try { fresh.assertActive(); } catch { replacementActive = false; }
        const actual = { originalPosts: messages.length, otherPosts: other.sent.filter(isDisposal).length, sameGeneration: messages[0]?.activationGeneration === operation.activationGeneration, routeReleased, oldLeasePosts: workers.reduce((count, worker) => count + worker.sent.length, 0) - before, replacementActive };
        expect(actual, JSON.stringify(row)).toEqual(row.expected);
        expect(oracle.validate({ const: row.expected }, actual)).toBe(true);
        client.dispose(rows.actorId);
      }
    });

    it("refuses captured disposal before exact close and never addresses a replacement worker", async () => {
      const { client, workers } = harness(1);
      await activate(client, workers[0]!, "captured-dispose");
      const owner = await captureFixtureInstance(client, workers[0]!, "captured-dispose");
      expect(() => owner.dispose()).toThrow("actor-close.native-retirement-pending");
      await retireFixtureInstance(workers[0]!, owner);
      client.terminate(0);
      client.rebuild(0);
      const fresh = await activate(client, workers[1]!, "captured-dispose");
      const before = workers[1]!.sent.length;
      expect(() => owner.dispose()).toThrow("actor-close.worker-lost");
      expect(workers[1]!.sent.length).toBe(before);
      expect(() => fresh.assertActive()).not.toThrow();
      client.dispose("captured-dispose");
    });

    it("refuses a reply from a foreign or replaced worker incarnation", async () => {
      const { actorId } = await fixture();
      const { client, workers } = harness(2, { exclusiveShardCount: 1 });
      await activate(client, workers[0]!, actorId);
      client.terminate(0);
      client.rebuild(0);
      const lease = await activate(client, workers[2]!, actorId);
      let settled = false;
      const turning = lease.turn([], BUDGET).then((result) => { settled = true; return result; });
      const message = workers[2]!.sent.at(-1) as { requestId: string };
      for (const foreign of [workers[0]!, workers[1]!]) foreign.deliver({ kind: "result", requestId: message.requestId, ok: true, value: "foreign" });
      await Promise.resolve();
      await Promise.resolve();
      expect(settled).toBe(false);
      workers[2]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: "owned" });
      await expect(turning).resolves.toBe("owned");
      client.dispose(actorId);
    });

    it("rejects a settled result after close begins without acknowledging or releasing the close root", async () => {
      const { actorId, instanceId } = await fixture();
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const lease = await activate(client, worker, actorId);
      const close = await captureFixtureInstance(client, worker, actorId, instanceId);
      const turning = lease.turn([{ kind: "completed", payload: { req: 41n } }], BUDGET);
      const observed = expect(turning).rejects.toThrow("actor-activation.revoked");
      const turn = worker.sent.at(-1) as { requestId: string };
      const before = worker.sent.length;
      close.beginClose();
      worker.deliver({ kind: "result", requestId: turn.requestId, ok: true, value: { uiPatches: ["retained"] } });
      await observed;
      expect(worker.sent).toHaveLength(before);
      expect(close.progress().kind).toBe("closing");
      expect(() => client.dispose(actorId)).toThrow("actor-close.native-retirement-pending");
      await retireFixtureInstance(worker, close);
      client.dispose(actorId);
    });
  });
  //#endregion 🪪️ActivationLease

  describe("ShardClient exact instance close transport", () => {
    it("retains and retries the same close authority after transport refusal", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../../🚪️lifetime/🧪️fixture.json", import.meta.url), "utf8"));
      for (const failure of fixture.leaseFailures) {
        const { client, workers } = harness(1);
        const worker = workers[0]!;
        const activation = client.activate("retry", "https://x/retry.js", [], BUDGET);
        worker.deliver({ kind: "result", requestId: (worker.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
        await activation;
        const lease = await captureFixtureInstance(client, worker, "retry");
        const request = lease.beginClose();
        const send = worker.postMessage.bind(worker);
        let original: unknown;
        worker.postMessage = (message) => {
          original = message;
          if (failure === "postMessage-throw") throw new Error("fixture transport refusal");
          send(message);
        };
        const pending = lease.close(BUDGET);
        const observed = expect(pending).rejects.toThrow("fixture");
        if (failure === "worker-error") worker.deliver({ kind: "result", requestId: (original as { requestId: string }).requestId, ok: false, error: "fixture worker refusal" });
        await observed;
        expect(lease.progress()).toEqual({ kind: "blocked", failure: failure === "postMessage-throw" ? "transport-refused" : "worker-refused" });
        expect(() => client.dispose("retry")).toThrow("actor-close.native-retirement-pending");
        worker.postMessage = send;
        const retried = lease.close(BUDGET);
        const message = worker.sent.at(-1) as { events: ShardEventEnvelope[] };
        expect(message.events[0]!.payload).toBe(request);
        expect(message.events).toEqual((original as { events: ShardEventEnvelope[] }).events);
        await answerLifecycle(worker, retried);
        await retireFixtureInstance(worker, lease);
        expect(lease.progress()).toEqual({ kind: "complete", failure: null });
        client.dispose("retry");
      }
    });

    it("waits for the captured worker's exact accepted and retired receipts", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../../🚪️lifetime/🧪️fixture.json", import.meta.url), "utf8"));
      for (const row of fixture.leaseReceipts) {
        const { client, workers } = harness(2);
        const worker = workers[0]!;
        const activation = client.activate("close-owner", "https://x/close.js", [], BUDGET);
        worker.deliver({ kind: "result", requestId: (worker.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
        await activation;
        const lease = await captureFixtureInstance(client, worker, "close-owner");
        const request = lease.beginClose();
        await answerLifecycle(worker, lease.close(BUDGET));
        const accepted: ActorInstanceLifecycleReceipt = { kind: "accepted", lifetime: lease.lifetime!, requestSequence: request.requestSequence, closeGeneration: 17n };
        for (const event of row.events) {
          const receipt: ActorInstanceLifecycleReceipt = { ...accepted, kind: event.endsWith("retired") ? "retired" : "accepted", ...(event === "old-activation-accepted" ? { lifetime: { ...lease.lifetime!, activationGeneration: lease.lifetime!.activationGeneration + 1n } } : {}), ...(event === "wrong-request-accepted" ? { requestSequence: request.requestSequence + 1 } : {}), ...(event === "wrong-generation-retired" ? { closeGeneration: 16n } : {}) };
          const pending = lease.poll(BUDGET);
          if (event === "foreign-worker-accepted") {
            const requestId = (worker.sent.at(-1) as { requestId: string }).requestId;
            workers[1]!.deliver({ kind: "result", requestId, ok: true, value: { status: { tag: "idle" }, lifecycleReceipt: encodeActorInstanceLifecycle(receipt) } });
            await answerLifecycle(worker, pending);
          } else {
            const delivered = answerLifecycle(worker, pending, receipt);
            if (event.startsWith("wrong-") || event === "old-activation-accepted" || receipt.kind === "retired" && lease.progress().kind !== "accepted") await expect(delivered).rejects.toThrow("actor-lifecycle.receipt-mismatch");
            else await delivered;
          }
          if (lease.pendingReceipt?.kind === "accepted") await answerLifecycle(worker, lease.acknowledge(lease.pendingReceipt, BUDGET));
        }
        expect(lease.pendingReceipt?.kind === "retired", row.name).toBe(row.settled);
        expect(client.shardIndexFor("close-owner")).toBe(0);
        if (lease.pendingReceipt?.kind !== "retired") {
          await answerLifecycle(worker, lease.poll(BUDGET), accepted);
          await answerLifecycle(worker, lease.acknowledge(accepted, BUDGET), { ...accepted, kind: "retired" });
        }
        await answerLifecycle(worker, lease.acknowledge(lease.pendingReceipt!, BUDGET, fixtureRetirement(lease)));
        expect(lease.progress().kind).toBe("complete");
        client.dispose("close-owner");
      }
    });

    it("does not revive a completed owner when the numeric ID or actor activation is reused", async () => {
      const { client, workers } = harness(1);
      const worker = workers[0]!;
      const activate = async () => {
        const pending = client.activate("reused", "https://x/close.js", [], BUDGET);
        worker.deliver({ kind: "result", requestId: (worker.sent.at(-1) as { requestId: string }).requestId, ok: true, value: undefined });
        await pending;
      };
      await activate();
      const old = await captureFixtureInstance(client, worker, "reused");
      await retireFixtureInstance(worker, old);
      const sameActivation = await captureFixtureInstance(client, worker, "reused", 7, 14n);
      expect(sameActivation.lifetime!.activationGeneration).toBe(old.lifetime!.activationGeneration);
      expect(sameActivation.lifetime!.guestLifetime).not.toBe(old.lifetime!.guestLifetime);
      expect(() => old.activation.assertActive()).toThrow("actor-activation.revoked");
      const before = worker.sent.length;
      await expect(old.poll(BUDGET)).rejects.toThrow("actor-lifecycle.already-complete");
      expect(worker.sent).toHaveLength(before);
      await retireFixtureInstance(worker, sameActivation);
      client.dispose("reused");
      await activate();
      const fresh = await captureFixtureInstance(client, worker, "reused");
      expect(fresh.lifetime!.activationGeneration).toBe(old.lifetime!.activationGeneration + 1n);
      await retireFixtureInstance(worker, fresh);
      client.dispose("reused");
    });
  });

  describe("ShardClient activation + turn round-trip", () => {
    it("routes activate then turn to the same shard, resolving the reply by requestId", async () => {
      const { client, workers } = harness(2);
      const activatePromise = client.activate("actor-1", "https://x/plugin.js", [], BUDGET);
      const activateMsg = workers[0]!.sent[0] as { readonly kind: string; readonly requestId: string };
      expect(activateMsg.kind).toBe("activate");
      workers[0]!.deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
      await activatePromise;

      const turnPromise = client.turn("actor-1", [{ kind: "wake", payload: {} }], BUDGET);
      const turnMsg = workers[0]!.sent[1] as { readonly kind: string; readonly requestId: string };
      expect(turnMsg.kind).toBe("turn");
      workers[0]!.deliver({ kind: "result", requestId: turnMsg.requestId, ok: true, value: { effects: [] } });
      await expect(turnPromise).resolves.toEqual({ effects: [] });
    });

    it("rejects turn() for an actor never activated", async () => {
      const { client } = harness(2);
      await expect(client.turn("ghost", [], BUDGET)).rejects.toThrow(/not activated/);
    });
  });

  describe("ShardClient segmented-download transport", () => {
    async function activatedHarness() {
      const state = harness(1);
      const activation = state.client.activate("actor-download", "https://x/plugin.js", [], BUDGET);
      const message = state.workers[0]!.sent[0] as { readonly requestId: string };
      state.workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
      await activation;
      return state;
    }

    it("preserves operation identity and last-Some then None ordering", async () => {
      const { client, workers } = await activatedHarness();
      for (const expected of [new Uint8Array([1, 2]), new Uint8Array([3]), undefined]) {
        const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
        const message = workers[0]!.sent.at(-1) as { readonly kind: string; readonly requestId: string; readonly instanceId: number; readonly operationId: bigint };
        expect(message).toMatchObject({ kind: "takeSegmentedDownloadChunk", instanceId: 17, operationId: 91n });
        workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: expected });
        await expect(read).resolves.toEqual(expected);
      }
    });

    it("propagates unknown-operation errors without manufacturing a terminal None", async () => {
      const { client, workers } = await activatedHarness();
      const read = client.takeSegmentedDownloadChunk("actor-download", 17, 404n);
      const message = workers[0]!.sent.at(-1) as { readonly requestId: string };
      workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: false, error: "interactive-job.unknown-segmented-download" });
      await expect(read).rejects.toThrow("interactive-job.unknown-segmented-download");
    });

    it("rejects oversized or empty response items", async () => {
      const { client, workers } = await activatedHarness();
      for (const invalid of [new Uint8Array(4_097), new Uint8Array(0)]) {
        const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
        const message = workers[0]!.sent.at(-1) as { readonly requestId: string };
        workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: invalid });
        await expect(read).rejects.toThrow("segmented-download-transport-limit");
      }
    });

    it("rejects an in-flight read when actor disposal cancels its transport ownership", async () => {
      const { client } = await activatedHarness();
      const read = client.takeSegmentedDownloadChunk("actor-download", 17, 91n);
      client.dispose("actor-download");
      await expect(read).rejects.toThrow("ShardClient actor disposed");
    });

    it("preserves the complete u64 operation authority through structured clone", async () => {
      const { client, workers } = await activatedHarness();
      for (const operationId of [(1n << 53n) + 1n, MAX_SEGMENTED_DOWNLOAD_OPERATION_ID]) {
        const read = client.takeSegmentedDownloadChunk("actor-download", 17, operationId);
        const message = structuredClone(workers[0]!.sent.at(-1)) as { readonly requestId: string; readonly operationId: bigint };
        expect(message.operationId).toBe(operationId);
        workers[0]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
        await expect(read).resolves.toBeUndefined();
      }
    });

    it("rejects zero and overflowing operation authorities", async () => {
      const { client } = await activatedHarness();
      await expect(client.takeSegmentedDownloadChunk("actor-download", 17, 0n)).rejects.toThrow("segmented-download-authority-invalid");
      await expect(client.takeSegmentedDownloadChunk("actor-download", 17, 1n << 64n)).rejects.toThrow("segmented-download-authority-invalid");
    });
  });

  describe("ShardClient actor-id multiplexing", () => {
    it("distinguishes two actors' replies on the same shard even when they resolve out of order", async () => {
      const { client, workers } = harness(1);
      const p1 = client.activate("a", "https://x/a.js", [], BUDGET);
      const p2 = client.activate("b", "https://x/b.js", [], BUDGET);
      const [msgA, msgB] = workers[0]!.sent as { readonly requestId: string }[];
      // out-of-order reply: b's activate lands before a's
      workers[0]!.deliver({ kind: "result", requestId: msgB!.requestId, ok: true, value: undefined });
      workers[0]!.deliver({ kind: "result", requestId: msgA!.requestId, ok: true, value: undefined });
      await expect(p1).resolves.toBeUndefined();
      await expect(p2).resolves.toBeUndefined();
      expect(client.shardIndexFor("a")).toBe(0);
      expect(client.shardIndexFor("b")).toBe(0);
    });

    it("round-robins fresh actors across shards, skipping the reserved exclusive tail", async () => {
      const { client } = harness(4, { exclusiveShardCount: 1 });
      const indices = ["a", "b", "c"].map((id) => {
        void client.activate(id, "https://x/y.js", [], BUDGET);
        return client.shardIndexFor(id);
      });
      expect(new Set(indices).has(3)).toBe(false); // shard 3 reserved exclusive
      expect(indices).toEqual([0, 1, 2]);
    });
  });

  describe("ShardClient heartbeat watchdog (postMessage path)", () => {
    it("does not miss while idle (no pending turn)", () => {
      const { client, advance } = harness(1);
      advance(100_000);
      client.checkHeartbeats(100_000);
      // no throw, no rebuild callback would have fired — nothing to assert but absence of an exception
      expect(true).toBe(true);
    });

    it("terminates + rebuilds after 3 consecutive missed-heartbeat windows on a stuck turn", async () => {
      const lost: Array<{ index: number; actorIds: readonly string[] }> = [];
      const { client, workers, advance, setNow } = harness(1, { heartbeatTimeoutMs: 1000, onShardLost: (index, actorIds) => lost.push({ index, actorIds }) });
      setNow(0);
      const activatePromise = client.activate("stuck", "https://x/stuck.js", [], BUDGET);
      const activateMsg = workers[0]!.sent[0] as { readonly requestId: string };
      workers[0]!.deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
      await activatePromise;

      const originalWorker = workers[0]!;
      client.turn("stuck", [], BUDGET).catch(() => {}); // never replies — simulates a hung guest; terminate() rejects it

      advance(1001);
      client.checkHeartbeats();
      expect(lost).toEqual([]);
      expect(originalWorker.terminated).toBe(false);

      advance(1001);
      client.checkHeartbeats();
      advance(1001);
      client.checkHeartbeats();

      expect(originalWorker.terminated).toBe(true);
      expect(lost).toEqual([{ index: 0, actorIds: ["stuck"] }]);
      expect(client.shardIndexFor("stuck")).toBeUndefined(); // rebuild cleared routing; caller must re-activate
    });

    it("a fresh heartbeat resets the miss count", async () => {
      const { client, workers, advance, setNow } = harness(1, { heartbeatTimeoutMs: 1000 });
      setNow(0);
      const activatePromise = client.activate("busy", "https://x/busy.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;
      void client.turn("busy", [], BUDGET);

      advance(1001);
      client.checkHeartbeats();
      advance(1001);
      client.checkHeartbeats();
      // guest emits a heartbeat right before the 3rd window would elapse
      workers[0]!.deliver({ kind: "heartbeat", turnSeq: 7 });
      advance(1001);
      client.checkHeartbeats();
      expect(workers[0]!.terminated).toBe(false);
    });
  });

  describe("ShardClient SAB heartbeat path", () => {
    it("pollHeartbeatSab reads Atomics-stored turnSeq and feeds the same miss-count state machine", async () => {
      const sab = new SharedArrayBuffer(4 * Int32Array.BYTES_PER_ELEMENT);
      const view = new Int32Array(sab);
      const { client, workers, advance, setNow } = harness(1, { heartbeatSab: sab, heartbeatTimeoutMs: 1000 });
      setNow(0);
      const activatePromise = client.activate("sab-actor", "https://x/s.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent.find((m) => (m as { kind: string }).kind === "activate") as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;
      void client.turn("sab-actor", [], BUDGET);

      advance(1001);
      client.checkHeartbeats();
      expect(workers[0]!.terminated).toBe(false);

      Atomics.store(view, 0, 5);
      client.pollHeartbeatSab();
      advance(1001);
      client.checkHeartbeats();
      expect(workers[0]!.terminated).toBe(false); // heartbeat seen via SAB reset the window
    });
  });

  describe("ShardClient leaseExclusive", () => {
    it("moves an actor onto a reserved exclusive shard and back", async () => {
      const { client } = harness(4, { exclusiveShardCount: 2 });
      void client.activate("heavy", "https://x/h.js", [], BUDGET);
      expect(client.shardIndexFor("heavy")).toBe(0);
      const exclusiveIndex = client.leaseExclusive("heavy");
      expect([2, 3]).toContain(exclusiveIndex);
      expect(client.shardIndexFor("heavy")).toBe(exclusiveIndex);
      client.releaseExclusive("heavy");
      expect(client.shardIndexFor("heavy")).toBeUndefined();
    });

    it("throws once every exclusive shard is leased and force is not set", () => {
      const { client } = harness(3, { exclusiveShardCount: 1 });
      void client.activate("first", "https://x/1.js", [], BUDGET);
      client.leaseExclusive("first");
      void client.activate("second", "https://x/2.js", [], BUDGET);
      expect(() => client.leaseExclusive("second")).toThrow(/no free exclusive shard/);
    });

    it("is idempotent for the same actor already leased", () => {
      const { client } = harness(2, { exclusiveShardCount: 1 });
      void client.activate("a", "https://x/a.js", [], BUDGET);
      const first = client.leaseExclusive("a");
      const second = client.leaseExclusive("a");
      expect(first).toBe(second);
    });
  });

  describe("ShardClient.startWatchdog / stopWatchdog", () => {
    it("self-ticks checkHeartbeats + pollHeartbeatSab with no external caller, detects a missed heartbeat, and rebuilds", async () => {
      vi.useFakeTimers();
      try {
        const lost: Array<{ index: number; actorIds: readonly string[] }> = [];
        const workers: FakeShardWorker[] = [];
        const client = new ShardClient({
          residentLedger: new OwnedResidentLedger({ bytes: 1048576, slots: 4096, owners: 4096, control: { bytes: 65536, slots: 256, owners: 256 } }),
          shardCount: 1,
          createWorker: (index) => {
            const worker = new FakeShardWorker(index);
            workers.push(worker);
            return worker;
          },
          heartbeatTimeoutMs: 1000,
          onShardLost: (index, actorIds) => lost.push({ index, actorIds }),
        });

        const activatePromise = client.activate("stuck", "https://x/stuck.js", [], BUDGET);
        const activateMsg = workers[0]!.sent[0] as { readonly requestId: string };
        workers[0]!.deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
        await activatePromise;

        client.turn("stuck", [], BUDGET).catch(() => {}); // never replies — nothing external ticks the watchdog for it
        client.startWatchdog(1001); // same cadence the pre-existing manual test used, driven this time by the self-tick

        vi.advanceTimersByTime(1001); // window 1
        expect(lost).toEqual([]);
        vi.advanceTimersByTime(1001); // window 2
        expect(lost).toEqual([]);
        vi.advanceTimersByTime(1001); // window 3 — self-ticked, no manual checkHeartbeats() call anywhere in this test
        expect(workers[0]!.terminated).toBe(true);
        expect(lost).toEqual([{ index: 0, actorIds: ["stuck"] }]);
        expect(client.shardIndexFor("stuck")).toBeUndefined();

        client.stopWatchdog();
        const lostCountAfterStop = lost.length;
        vi.advanceTimersByTime(10_000);
        expect(lost.length).toBe(lostCountAfterStop); // stopped — no further ticks fire
      } finally {
        vi.useRealTimers();
      }
    });

    it("is idempotent to call twice, and stopWatchdog before ever starting is a no-op", () => {
      vi.useFakeTimers();
      try {
        const { client } = harness(1);
        client.startWatchdog(500);
        client.startWatchdog(500); // second call while running: no-op, does not leak a second interval
        client.stopWatchdog();
        client.stopWatchdog(); // already stopped: no-op, does not throw
        expect(true).toBe(true);
      } finally {
        vi.useRealTimers();
      }
    });
  });

  describe("ShardClient failShard clears routing", () => {
    it("clears actorShard + slot.actorIds immediately on a worker crash (onerror), before any terminate()/rebuild()", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("x", "https://x/x.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;
      expect(client.shardIndexFor("x")).toBe(0);

      workers[0]!.onerror?.(new Error("boom")); // failShard runs; onerror does NOT call terminate()/rebuild()

      // routing cleared right away — a dead shard must stop receiving newly routed work immediately,
      // not only once a later rebuild() happens to run.
      expect(client.shardIndexFor("x")).toBeUndefined();
      await expect(client.turn("x", [], BUDGET)).rejects.toThrow(/not activated/);
    });
  });

  describe("ShardClient terminate/rebuild", () => {
    it("rejects in-flight requests on terminate and spawns a fresh worker on rebuild", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("x", "https://x/x.js", [], BUDGET);
      const rejection = expect(activatePromise).rejects.toThrow(/terminated/);
      const oldWorker = workers[0]!;
      const actorIds = client.terminate(0);
      expect(actorIds).toEqual(["x"]);
      await rejection;
      expect(oldWorker.terminated).toBe(true);
      client.rebuild(0);
      expect(workers.length).toBe(2);
      expect(client.shardIndexFor("x")).toBeUndefined();
    });
  });

  describe("ShardClient worker crash", () => {
    it("onerror fails every pending request on that shard", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("crashy", "https://x/c.js", [], BUDGET);
      workers[0]!.onerror?.(new Error("boom"));
      await expect(activatePromise).rejects.toThrow(/crashed/);
    });
  });

  describe("ShardClient.shardMetricsSamples", () => {
    it("reports zero actors/busyRatio and an infinite heartbeat age for a fresh, never-touched shard", () => {
      const { client } = harness(2);
      const samples = client.shardMetricsSamples(1_000);
      expect(samples).toHaveLength(2);
      for (const sample of samples) {
        expect(sample.metrics.actors).toBe(0);
        expect(sample.metrics.busyRatio).toBe(0);
        expect(sample.metrics.heartbeatAgeMs).toBe(Number.POSITIVE_INFINITY);
      }
    });

    it("counts resident actors and in-flight turns as busyRatio, and ages the heartbeat off the injected clock", async () => {
      const { client, workers, setNow } = harness(1);
      setNow(0);
      const activateA = client.activate("a", "https://x/a.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activateA;
      const activateB = client.activate("b", "https://x/b.js", [], BUDGET); // second actor, same (only) shard
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[1] as { requestId: string }).requestId, ok: true, value: undefined });
      await activateB; // both activations settled — only the turn below should count toward busyRatio

      workers[0]!.deliver({ kind: "heartbeat", turnSeq: 1 });
      void client.turn("a", [], BUDGET); // one in-flight turn out of two resident actors, deliberately never replied

      setNow(300);
      const [sample] = client.shardMetricsSamples(300);
      expect(sample!.metrics.actors).toBe(2);
      expect(sample!.metrics.busyRatio).toBeCloseTo(0.5);
      expect(sample!.metrics.heartbeatAgeMs).toBe(300);
    });
  });

  //#region 📨️ShardFrame tests
  function makeEnvelope(to: string, lane: Lane, seq: number, kind = "wake"): ShardEnvelope {
    return { to, from: { kind: "kernel" }, lane, seq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload: {} } };
  }

  describe("orderEnvelopesByLane", () => {
    it("sorts by Lane priority, not arrival order, stable within a tied lane", () => {
      const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2), makeEnvelope("a", "Maintenance", 3), makeEnvelope("a", "Interactive", 4)];
      const ordered = orderEnvelopesByLane(envelopes);
      expect(ordered.map((envelope) => envelope.seq)).toEqual([2, 4, 1, 3]);
    });

    it("is a no-op for an already-lane-sorted, single-lane batch", () => {
      const envelopes = [makeEnvelope("a", "Interactive", 1), makeEnvelope("a", "Interactive", 2)];
      expect(orderEnvelopesByLane(envelopes).map((envelope) => envelope.seq)).toEqual([1, 2]);
    });
  });

  describe("GrantedBudgetTracker + interpretShardFrame", () => {
    it("a Grant records its budget and hands back envelopes in lane-priority order", () => {
      const tracker = createGrantedBudgetTracker();
      const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2)];
      const grantBudget: ShardBudget = { ...BUDGET, fuel: 999 };
      const result = interpretShardFrame({ kind: "Grant", actor: "a", budget: grantBudget, envelopes }, tracker);
      expect(result).toEqual({ action: "runEnvelopes", actor: "a", budget: grantBudget, envelopes: [envelopes[1], envelopes[0]] });
      expect(tracker.granted("a")).toBe(grantBudget);
    });

    it("an Envelope with no prior Grant runs under the Maintenance-lane default, never an invented constant", () => {
      const tracker = createGrantedBudgetTracker();
      const lonelyEnvelope = makeEnvelope("never-granted", "Interactive", 1);
      const result = interpretShardFrame({ kind: "Envelope", envelope: lonelyEnvelope }, tracker);
      expect(result).toEqual({ action: "runEnvelopes", actor: "never-granted", budget: MAINTENANCE_LANE_DEFAULT_BUDGET, envelopes: [lonelyEnvelope] });
    });

    it("an Envelope AFTER a Grant for the same actor runs under THAT granted budget — proving the old constant no longer influences it", () => {
      const tracker = createGrantedBudgetTracker();
      const grantBudget: ShardBudget = { ...BUDGET, fuel: 42 };
      interpretShardFrame({ kind: "Grant", actor: "a", budget: grantBudget, envelopes: [] }, tracker);
      const followUp = makeEnvelope("a", "Interactive", 5);
      const result = interpretShardFrame({ kind: "Envelope", envelope: followUp }, tracker);
      expect(result.action).toBe("runEnvelopes");
      expect((result as { readonly budget: ShardBudget }).budget).toBe(grantBudget);
      expect((result as { readonly budget: ShardBudget }).budget).not.toBe(MAINTENANCE_LANE_DEFAULT_BUDGET);
    });

    it("Register/Unregister are pure bookkeeping; Unregister forgets a previously granted budget", () => {
      const tracker = createGrantedBudgetTracker();
      interpretShardFrame({ kind: "Grant", actor: "a", budget: BUDGET, envelopes: [] }, tracker);
      expect(tracker.granted("a")).toBe(BUDGET);
      expect(interpretShardFrame({ kind: "Register", actor: "a" }, tracker)).toEqual({ action: "register", actor: "a" });
      expect(interpretShardFrame({ kind: "Unregister", actor: "a" }, tracker)).toEqual({ action: "unregister", actor: "a" });
      expect(tracker.granted("a")).toEqual(MAINTENANCE_LANE_DEFAULT_BUDGET); // forgotten — back to the floor, not a stale grant
    });

    it("an unknown/future frame variant resolves to 'unknown' instead of throwing (forward-compat)", () => {
      const tracker = createGrantedBudgetTracker();
      const futureFrame = { kind: "Checkpoint", actor: "a" } as unknown as ShardFrame;
      expect(() => interpretShardFrame(futureFrame, tracker)).not.toThrow();
      expect(interpretShardFrame(futureFrame, tracker)).toEqual({ action: "unknown", frame: futureFrame });
    });
  });

  describe("ShardClient.grant / ShardClient.envelope wire adoption", () => {
    it("grant() sends a ShardFrame::Grant frame with envelopes pre-sorted by lane, budget carried alongside them", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("a", "https://x/a.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;

      const grantBudget: ShardBudget = { ...BUDGET, fuel: 12345 };
      const envelopes = [makeEnvelope("a", "Background", 1), makeEnvelope("a", "Interactive", 2)];
      void client.grant("a", grantBudget, envelopes);

      const sent = workers[0]!.sent[1] as { readonly kind: string; readonly frame: ShardFrame };
      expect(sent.kind).toBe("frame");
      expect(sent.frame.kind).toBe("Grant");
      const grantFrame = sent.frame as { readonly kind: "Grant"; readonly actor: string; readonly budget: ShardBudget; readonly envelopes: readonly ShardEnvelope[] };
      expect(grantFrame.budget).toBe(grantBudget);
      expect(grantFrame.envelopes.map((envelope) => envelope.seq)).toEqual([2, 1]); // Interactive dispatched before Background
    });

    it("envelope() sends a ShardFrame::Envelope frame with NO budget field on the wire at all", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("a", "https://x/a.js", [], BUDGET);
      workers[0]!.deliver({ kind: "result", requestId: (workers[0]!.sent[0] as { requestId: string }).requestId, ok: true, value: undefined });
      await activatePromise;

      void client.envelope(makeEnvelope("a", "Interactive", 1));
      const sent = workers[0]!.sent[1] as { readonly kind: string; readonly frame: ShardFrame };
      expect(sent.kind).toBe("frame");
      expect(sent.frame.kind).toBe("Envelope");
      expect(Object.keys(sent.frame)).toEqual(["kind", "envelope"]); // structurally budget-less on the wire
    });

    it("turn()/activate() keep working completely unchanged alongside the new frame wire (incremental adoption really is incremental)", async () => {
      const { client, workers } = harness(1);
      const activatePromise = client.activate("legacy", "https://x/legacy.js", [], BUDGET);
      const activateMsg = workers[0]!.sent[0] as { readonly kind: string; readonly requestId: string };
      expect(activateMsg.kind).toBe("activate");
      workers[0]!.deliver({ kind: "result", requestId: activateMsg.requestId, ok: true, value: undefined });
      await activatePromise;

      const turnPromise = client.turn("legacy", [{ kind: "wake", payload: {} }], BUDGET);
      const turnMsg = workers[0]!.sent[1] as { readonly kind: string; readonly requestId: string };
      expect(turnMsg.kind).toBe("turn");
      workers[0]!.deliver({ kind: "result", requestId: turnMsg.requestId, ok: true, value: { effects: [] } });
      await expect(turnPromise).resolves.toEqual({ effects: [] });
    });
  });

  describe("fixed command ingress pages", () => {
    it("matches a DataView little-endian oracle across a full page boundary", () => {
      const command = Uint8Array.from({ length: ACTOR_BYTE_PAGE_BYTES + 5 }, (_, index) => index & 0xff);
      const pages = createShardCommandIngressPages({ owner: 7n, generation: 11n, commandIndex: 1, commandCount: 3, instance: 13, seq: 17n, command });
      expect(pages).toHaveLength(2);
      expect(pages.map((page) => page.page.length)).toEqual([ACTOR_BYTE_PAGE_BYTES, 5]);
      expect(pages.map((page) => Object.keys(page))).toEqual([["cursor", "page"], ["cursor", "page"]]);
      expect(pages[0]!.cursor).toMatchObject({ owner: 7n, generation: 11n, commandIndex: 1, commandCount: 3, instance: 13, seq: 17n, kind: 0, pageIndex: 0, pageCount: 2 });
      const oracle = new DataView(command.buffer, command.byteOffset, command.byteLength);
      expect(pages[0]!.page.block00.word0).toBe(oracle.getBigUint64(0, true));
      expect(pages[0]!.page.block63.word7).toBe(oracle.getBigUint64(ACTOR_BYTE_PAGE_BYTES - 8, true));
      expect(pages[1]!.page.block00.word0).toBe(0x0000000403020100n);
      expect(pages[1]!.page.block00.word1).toBe(0n);
      expect(pages[1]!.page.block63.word7).toBe(0n);
    });

    it("forwards the fixed page as the dedicated turn argument", async () => {
      const { client, workers } = harness(1);
      await activateActor(client, workers, "paged");
      const page = createShardCommandIngressPages({ owner: 1n, generation: 1n, commandIndex: 0, commandCount: 1, instance: 1, seq: 1n, command: Uint8Array.of(9, 8, 7) })[0]!;
      void client.turn("paged", [], BUDGET, page);
      expect(workers[0]!.sent.at(-1)).toMatchObject({ kind: "turn", actorId: "paged", events: [], commandPage: page });
      expect(Object.keys(page)).toEqual(["cursor", "page"]);
    });
  });

  describe("ShardFrame parity with Rust component.rs", () => {
    it("TS ShardFrame variant/field names match the live Rust enum in 🖥️host/🧵️shard/🦀️component.rs", async () => {
      const { readFileSync } = await import("node:fs");
      const rustUrl = new URL("../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs", import.meta.url);
      const source = readFileSync(rustUrl, "utf8");
      const enumMatch = source.match(/pub enum ShardFrame \{([\s\S]*?)\n\}\s*\n\s*impl ShardFrame/);
      expect(enumMatch).not.toBeNull(); // [DEBUG] `pub enum ShardFrame { ... } impl ShardFrame` shape not found — Rust source changed, update this test's regex
      const body = enumMatch![1]!.replace(/\/\/\/.*$/gm, "").replace(/\/\/.*$/gm, "");
      const variantPattern = /(\w+)\s*(?:\{([^{}]*)\}|\(([^()]*)\))?\s*,/g;
      const rustVariants: Array<{ readonly name: string; readonly fields: readonly string[] | null }> = [];
      let match: RegExpExecArray | null;
      while ((match = variantPattern.exec(body)) !== null) {
        const [, name, structFields, tupleType] = match;
        if (structFields !== undefined) {
          const fields = structFields
            .split(",")
            .map((part) => part.trim())
            .filter((part) => part.length > 0)
            .map((part) => part.split(":")[0]!.trim());
          rustVariants.push({ name: name!, fields });
        } else if (tupleType !== undefined) {
          rustVariants.push({ name: name!, fields: null });
        }
      }

      // Same variant NAMES, in the same order, as this file's own runtime twin.
      expect(rustVariants.map((variant) => variant.name)).toEqual(SHARD_FRAME_VARIANT_FIELDS.map((variant) => variant.kind));
      for (const rustVariant of rustVariants) {
        if (rustVariant.fields === null) continue; // tuple variant (`Envelope(Envelope)`) — Rust has no field name to diff against; see SHARD_FRAME_VARIANT_FIELDS's own doc
        const tsVariant = SHARD_FRAME_VARIANT_FIELDS.find((variant) => variant.kind === rustVariant.name)!;
        expect(tsVariant.fields).toEqual(rustVariant.fields);
      }
    });
  });
  //#endregion 📨️ShardFrame tests

  //#region 🌉️HostEffectBridge tests
  function makeEffectRequestFrame(actorId: string, effect: string, requestId: string, params: unknown, activationGeneration = 1n): InboundMessage {
    return { kind: "frame", actorId, activationGeneration, frame: { kind: "Envelope", envelope: { to: "kernel", from: { kind: "actor", id: actorId }, lane: "Background", seq: 1, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind: "effect-request", payload: { effect, requestId, params } } } } };
  }

  type EffectReplyPayload = { readonly requestId: string; readonly value?: unknown; readonly message?: string };
  type EffectReplyMessage = { readonly kind: "frame"; readonly frame: { readonly kind: "Envelope"; readonly envelope: { readonly payload: { readonly kind: "effect-complete" | "effect-error"; readonly payload: EffectReplyPayload } } } };

  function findEffectReply(sent: readonly unknown[], requestId: string, kind: "effect-complete" | "effect-error"): EffectReplyMessage | undefined {
    return (sent as readonly { readonly kind: string; readonly frame?: { readonly kind: string; readonly envelope?: { readonly payload?: { readonly kind?: string; readonly payload?: { readonly requestId?: string } } } } }[]).find(
      (message) => message.kind === "frame" && message.frame?.kind === "Envelope" && message.frame.envelope?.payload?.kind === kind && message.frame.envelope.payload?.payload?.requestId === requestId,
    ) as EffectReplyMessage | undefined;
  }

  async function activateActor(client: ShardClient, workers: readonly FakeShardWorker[], actorId: string, shardIndex = 0): Promise<void> {
    const promise = client.activate(actorId, `https://x/${actorId}.js`, [], BUDGET);
    const message = workers[shardIndex]!.sent.at(-1) as { readonly requestId: string };
    workers[shardIndex]!.deliver({ kind: "result", requestId: message.requestId, ok: true, value: undefined });
    await promise;
  }

  function flushMicrotasks(): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, 0));
  }

  describe("ShardClient host-effect bridge — handler success", () => {
    it("resolves an effect-request through onHostEffect and posts an effect-complete frame back to the worker", async () => {
      const { client, workers } = harness(1, { onHostEffect: async (actorId, effect, params) => ({ actorId, effect, params, from: "handler" }) });
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", { url: "https://example.test" }));
      await flushMicrotasks();

      const reply = findEffectReply(workers[0]!.sent, "a:http-fetch:1", "effect-complete");
      expect(reply).toBeDefined();
      expect(reply?.frame.envelope.payload.payload.value).toEqual({ actorId: "a", effect: "http-fetch", params: { url: "https://example.test" }, from: "handler" });
    });
  });

  describe("ShardClient host-effect bridge — handler error", () => {
    it("a rejected onHostEffect settles as effect-error, never a hang", async () => {
      const { client, workers } = harness(1, { onHostEffect: async () => { throw new Error("boom"); } });
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "blob-read", "a:blob-read:1", { hash: "x" }));
      await flushMicrotasks();

      const reply = findEffectReply(workers[0]!.sent, "a:blob-read:1", "effect-error");
      expect(reply?.frame.envelope.payload.payload.message).toBe("boom");
    });
  });

  describe("ShardClient host-effect bridge — no handler installed", () => {
    it("fails FAST with an explicit effect-error, synchronously, never a silent hang", async () => {
      const { client, workers } = harness(1); // no onHostEffect
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "storage-read", "a:storage-read:1", {}));
      // no `await flushMicrotasks()` — the reply must already be sent, proving this path never touches a Promise chain
      const reply = findEffectReply(workers[0]!.sent, "a:storage-read:1", "effect-error");
      expect(reply?.frame.envelope.payload.payload.message).toBe("no host effect handler installed");
    });
  });

  describe("ShardClient host-effect bridge — backpressure cap", () => {
    it("rejects an effect-request beyond maxOutstandingEffectsPerActor with a quota-shaped effect-error, while the earlier one stays pending", async () => {
      const { client, workers } = harness(1, { maxOutstandingEffectsPerActor: 1, onHostEffect: () => new Promise(() => {}) });
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "spawn-job", "a:spawn-job:1", {}));
      workers[0]!.deliver(makeEffectRequestFrame("a", "spawn-job", "a:spawn-job:2", {}));

      expect(findEffectReply(workers[0]!.sent, "a:spawn-job:1", "effect-error")).toBeUndefined();
      expect(findEffectReply(workers[0]!.sent, "a:spawn-job:1", "effect-complete")).toBeUndefined();
      const reply = findEffectReply(workers[0]!.sent, "a:spawn-job:2", "effect-error");
      expect(reply?.frame.envelope.payload.payload.message).toMatch(/outstandingRequests.*limit=1.*actual=1/);
    });
  });

  describe("ShardClient host-effect bridge — shard-loss settlement", () => {
    it("terminate() aborts every outstanding effect for its actors, and a late handler resolution posts no reply to the dead worker", async () => {
      let capturedSignal: AbortSignal | undefined;
      const { client, workers } = harness(1, {
        onHostEffect: (_actorId, _effect, _params, signal) =>
          new Promise((resolve) => {
            capturedSignal = signal;
            signal.addEventListener("abort", () => resolve("too-late"));
          }),
      });
      await activateActor(client, workers, "a");

      workers[0]!.deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", {}));
      expect(capturedSignal?.aborted).toBe(false);

      client.terminate(0);
      expect(capturedSignal?.aborted).toBe(true);

      const sentBeforeLateResolve = workers[0]!.sent.length;
      await flushMicrotasks();
      expect(workers[0]!.sent.length).toBe(sentBeforeLateResolve); // the abort-triggered resolve did NOT produce a late effect-complete/effect-error post
      expect(findEffectReply(workers[0]!.sent, "a:http-fetch:1", "effect-complete")).toBeUndefined();
      expect(findEffectReply(workers[0]!.sent, "a:http-fetch:1", "effect-error")).toBeUndefined();
    });

    it("dispose(actorId) aborts that actor's outstanding effects without touching a sibling actor's", async () => {
      const signals: Record<string, AbortSignal> = {};
      const { client, workers } = harness(1, {
        onHostEffect: (actorId, _effect, _params, signal) => {
          signals[actorId] = signal;
          return new Promise(() => {});
        },
      });
      await activateActor(client, workers, "a");
      await activateActor(client, workers, "b");

      workers[0]!.deliver(makeEffectRequestFrame("a", "http-fetch", "a:http-fetch:1", {}));
      workers[0]!.deliver(makeEffectRequestFrame("b", "http-fetch", "b:http-fetch:1", {}, client.captureActorActivation("b").activationGeneration));

      client.dispose("a");
      expect(signals.a?.aborted).toBe(true);
      expect(signals.b?.aborted).toBe(false);
    });
  });
  //#endregion 🌉️HostEffectBridge tests

  //#region 🪪️InboundActivation
  describe("ShardClient inbound activation authority", () => {
    it("matches the neutral stale-source matrix without consuming exact close receipts", async () => {
      const { default: Ajv } = await import("ajv");
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("../../🪪️activation/📨️inbound/🧪️fixture.json", import.meta.url), "utf8")) as { actorId: string; requestId: string; cases: Array<{ name: string; effects: number; traps: number }> };
      const schema = JSON.parse(readFileSync(new URL("../../🪪️activation/📨️inbound/🧪️schema.json", import.meta.url), "utf8"));
      const oracle = new Ajv();
      expect(oracle.validate(schema, fixture)).toBe(true);
      for (const row of fixture.cases) {
        const effects: string[] = [];
        const traps: string[] = [];
        const { client, workers } = harness(1, { onHostEffect: async (_actor, effect) => { effects.push(effect); return "ok"; }, onActorTrap: (_actor, message) => traps.push(message) });
        await activateActor(client, workers, fixture.actorId);
        const source = workers[0]!;
        const generation = client.captureActorActivation(fixture.actorId).activationGeneration;
        let close: ShardInstanceLifecycleLease | undefined;
        if (row.name === "replaced-worker") {
          client.terminate(0);
          client.rebuild(0);
          await activateActor(client, workers, fixture.actorId, 1);
        } else if (row.name === "reactivated") {
          client.dispose(fixture.actorId);
          await activateActor(client, workers, fixture.actorId);
        } else if (row.name === "closing") {
          close = await captureFixtureInstance(client, source, fixture.actorId);
          close.beginClose();
        }
        const frame = makeEffectRequestFrame(fixture.actorId, "storage-read", fixture.requestId, {});
        const message: Record<string, unknown> = { ...frame, activationGeneration: row.name === "foreign-generation" ? generation + 1n : generation };
        const trap: Record<string, unknown> = { kind: "trap", actorId: fixture.actorId, activationGeneration: message.activationGeneration, message: "fixture-trap" };
        if (row.name === "missing-generation") { delete message.activationGeneration; delete trap.activationGeneration; }
        if (row.name === "foreign-envelope" && frame.kind === "frame" && frame.frame.kind === "Envelope") {
          message.frame = { ...frame.frame, envelope: { ...frame.frame.envelope, from: { kind: "actor", id: "another-actor" } } };
        }
        source.onmessage?.({ data: message });
        source.onmessage?.({ data: trap });
        await flushMicrotasks();
        const actual = { name: row.name, effects: effects.length, traps: traps.length };
        expect(actual, row.name).toEqual(row);
        expect(oracle.validate({ const: row }, actual), row.name).toBe(true);
        if (close) {
          await retireFixtureInstance(source, close);
          expect(close.progress().kind).toBe("complete");
        }
        client.dispose(fixture.actorId);
      }
    });

    it("keeps a replacement effect with a reused request id when the old handler settles", async () => {
      const settlements: Array<(value: unknown) => void> = [];
      const signals: AbortSignal[] = [];
      const { client, workers } = harness(1, { onHostEffect: (_actor, _effect, _params, signal) => new Promise((resolve) => { signals.push(signal); settlements.push(resolve); }) });
      await activateActor(client, workers, "a");
      const oldGeneration = client.captureActorActivation("a").activationGeneration;
      workers[0]!.onmessage?.({ data: { ...makeEffectRequestFrame("a", "storage-read", "shared", {}), activationGeneration: oldGeneration } });
      client.dispose("a");
      expect(signals[0]!.aborted).toBe(true);
      await activateActor(client, workers, "a");
      const generation = client.captureActorActivation("a").activationGeneration;
      workers[0]!.onmessage?.({ data: { ...makeEffectRequestFrame("a", "storage-read", "shared", {}), activationGeneration: generation } });
      settlements[0]!("old");
      await flushMicrotasks();
      expect(findEffectReply(workers[0]!.sent, "shared", "effect-complete")).toBeUndefined();
      expect(signals[1]!.aborted).toBe(false);
      settlements[1]!("new");
      await flushMicrotasks();
      const reply = findEffectReply(workers[0]!.sent, "shared", "effect-complete");
      expect(reply?.frame.envelope.payload.payload.value).toBe("new");
      expect((reply as unknown as { activationGeneration: bigint }).activationGeneration).toBe(generation);
      client.dispose("a");
    });

    it("aborts the admitted host effect on close while retaining the native close owner", async () => {
      let signal: AbortSignal | undefined;
      let settle!: (value: unknown) => void;
      const { client, workers } = harness(1, { onHostEffect: (_actor, _effect, _params, captured) => new Promise((resolve) => { signal = captured; settle = resolve; }) });
      await activateActor(client, workers, "a");
      workers[0]!.onmessage?.({ data: { ...makeEffectRequestFrame("a", "storage-read", "closing", {}), activationGeneration: client.captureActorActivation("a").activationGeneration } });
      const lease = await captureFixtureInstance(client, workers[0]!, "a");
      void lease.beginClose();
      expect(signal?.aborted).toBe(false);
      await answerLifecycle(workers[0]!, lease.poll(BUDGET));
      expect(signal?.aborted).toBe(true);
      settle("late");
      await flushMicrotasks();
      expect(findEffectReply(workers[0]!.sent, "closing", "effect-complete")).toBeUndefined();
      expect(lease.progress().kind).toBe("closing");
      expect(() => client.dispose("a")).toThrow("actor-close.native-retirement-pending");
    });

    it("ignores late old-worker errors and worker traps after slot replacement", async () => {
      const traps: string[] = [];
      const { client, workers } = harness(1, { onActorTrap: (_actor, message) => traps.push(message) });
      await activateActor(client, workers, "a");
      const staleError = workers[0]!.onerror!;
      client.terminate(0);
      client.rebuild(0);
      await activateActor(client, workers, "a", 1);
      const lease = client.captureActorActivation("a");
      staleError(new Error("old-worker"));
      workers[0]!.onmessage?.({ data: { kind: "trap", actorId: "*", activationGeneration: null, message: "old-trap" } });
      expect(() => lease.assertActive()).not.toThrow();
      expect(traps).toEqual([]);
      workers[1]!.onmessage?.({ data: { kind: "trap", actorId: "*", activationGeneration: null, message: "current-worker-trap" } });
      expect(traps).toEqual(["current-worker-trap"]);
      client.dispose("a");
    });
  });
  //#endregion 🪪️InboundActivation

  void vi;
}
//#endregion 🧪️Tests
