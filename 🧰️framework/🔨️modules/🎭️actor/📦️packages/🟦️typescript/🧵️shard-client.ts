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
//#endregion 🌉️WorkerLike

//#region 📨️WireMessages
type OutboundMessage =
  | { readonly kind: "activate"; readonly requestId: string; readonly actorId: string; readonly moduleUrl: string; readonly caps: readonly ShardCapabilityGrant[]; readonly budget: ShardBudget; readonly assets: readonly ShardAsset[] }
  | { readonly kind: "turn"; readonly requestId: string; readonly actorId: string; readonly events: readonly ShardEventEnvelope[]; readonly budget: ShardBudget }
  | { readonly kind: "startJob"; readonly requestId: string; readonly actorId: string; readonly job: number; readonly jobKind: string; readonly input: Uint8Array }
  | { readonly kind: "stepJob"; readonly requestId: string; readonly actorId: string; readonly job: number; readonly budget: ShardJobBudget }
  | { readonly kind: "cancelJob"; readonly actorId: string; readonly job: number }
  | { readonly kind: "checkpoint"; readonly requestId: string; readonly actorId: string }
  | { readonly kind: "restore"; readonly requestId: string; readonly actorId: string; readonly state: Uint8Array }
  | { readonly kind: "dispose"; readonly actorId: string }
  /** 📨️ terra-web-shardframe: the ONE new wire message every {@link ShardFrame} variant travels over —
   * additive alongside `"activate"`/`"turn"`/etc above, none of which this message kind replaces or
   * changes. `actorId` is carried alongside `frame` (rather than requiring every handler to destructure
   * it back out of `frame`) purely so `send()`'s existing `"actorId" in message` pending-entry bookkeeping
   * keeps working unmodified for this kind too. */
  | { readonly kind: "frame"; readonly requestId: string; readonly actorId: string; readonly frame: ShardFrame };

type InboundMessage =
  | { readonly kind: "result"; readonly requestId: string; readonly ok: true; readonly value: unknown }
  | { readonly kind: "result"; readonly requestId: string; readonly ok: false; readonly error: string; readonly stack?: string; readonly type?: string; readonly framesBytes?: number }
  | { readonly kind: "heartbeat"; readonly turnSeq: number }
  | { readonly kind: "trap"; readonly actorId: string; readonly message: string }
  /** 📨️ terra-shard-effect-bridge: the worker→kernel direction of `🟨️host-shim.js`'s `effectRequest`
   * (🧪️ terra-web-bridges) — an async host import the guest `.await`ed. Reuses `ShardFrame`'s own
   * `Envelope` shape verbatim (`frame.envelope.payload` is `{kind:"effect-request", payload:{effect,
   * requestId, params}}`), never a second wire. Carries no `requestId` of its OWN at this outer level
   * (unlike every other inbound kind) — correlation lives inside `frame.envelope.payload.payload`. */
  | { readonly kind: "frame"; readonly actorId: string; readonly frame: ShardFrame };
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

type PendingEntry = { readonly resolve: (value: unknown) => void; readonly reject: (error: Error) => void; readonly shardIndex: number; readonly startedAtMs: number; readonly actorId: string };

type ShardSlot = {
  index: number;
  worker: ShardWorkerLike;
  readonly heartbeat: ShardHeartbeatState;
  readonly pendingRequestIds: Set<string>;
  readonly actorIds: Set<string>;
};

export interface ShardClientOptions {
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
  private readonly shards: ShardSlot[] = [];
  private readonly actorShard = new Map<string, number>();
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
  private readonly outstandingEffectsByActor = new Map<string, Map<string, AbortController>>();
  private effectReplySeq = 0;
  private nextRoundRobin = 0;
  private requestSeq = 0;
  private watchdogHandle: ReturnType<typeof setInterval> | null = null;

  constructor(options: ShardClientOptions) {
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
  private spawnShard(index: number): ShardSlot {
    const worker = this.createWorker(index);
    const slot: ShardSlot = { index, worker, heartbeat: freshHeartbeatState(this.now()), pendingRequestIds: new Set(), actorIds: new Set() };
    worker.onmessage = (event) => this.handleMessage(slot, event.data as InboundMessage);
    worker.onerror = (error) => {
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
    if (message.kind === "heartbeat") {
      this.recordHeartbeat(slot, message.turnSeq, this.now());
      return;
    }
    if (message.kind === "trap") {
      this.onActorTrap?.(message.actorId, message.message);
      return;
    }
    if (message.kind === "frame") {
      this.handleInboundFrame(slot, message.actorId, message.frame);
      return;
    }
    const entry = this.pending.get(message.requestId);
    if (!entry) return;
    this.pending.delete(message.requestId);
    slot.pendingRequestIds.delete(message.requestId);
    this.recomputeOldestPending(slot);
    if (message.ok) entry.resolve(message.value);
    else entry.reject(graftWorkerStack(entry.actorId, message.error, message.stack, message.type, message.framesBytes));
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
      this.actorShard.delete(actorId);
    }
    slot.actorIds.clear();
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
    this.shards[index]!.actorIds.delete(actorId);
    this.actorShard.delete(actorId);
  }

  shardIndexFor(actorId: string): number | undefined {
    return this.actorShard.get(actorId);
  }
  //#endregion 🧭️Assignment

  //#region 📮️Requests
  private nextRequestId(): string {
    this.requestSeq += 1;
    return `r${this.requestSeq}`;
  }

  private send<T>(slot: ShardSlot, message: OutboundMessage, requestId: string | null): Promise<T> {
    if (requestId === null) {
      slot.worker.postMessage(message);
      return Promise.resolve(undefined as T);
    }
    return new Promise<T>((resolve, reject) => {
      const startedAtMs = this.now();
      this.pending.set(requestId, { resolve: resolve as (value: unknown) => void, reject, shardIndex: slot.index, startedAtMs, actorId: "actorId" in message ? message.actorId : "" });
      slot.pendingRequestIds.add(requestId);
      if (slot.heartbeat.oldestPendingStartedAtMs === null) slot.heartbeat.oldestPendingStartedAtMs = startedAtMs;
      slot.worker.postMessage(message);
    });
  }

  async activate(actorId: string, moduleUrl: string, caps: readonly ShardCapabilityGrant[], budget: ShardBudget, assets: readonly ShardAsset[] = []): Promise<void> {
    const slot = this.assignShard(actorId);
    const requestId = this.nextRequestId();
    await this.send<void>(slot, { kind: "activate", requestId, actorId, moduleUrl, caps, budget, assets }, requestId);
  }

  /** ▶️ One turn (`reactor::poll`), never more than one in flight per `actorId` at a time — a second
   * `turn()` call for the same actor before the first resolves is a caller bug (the scheduler's own
   * per-actor serialization, not this transport's job, per design's "runs one turn at a time per
   * actor"), so it is rejected rather than silently queued. */
  async turn(actorId: string, events: readonly ShardEventEnvelope[], budget: ShardBudget): Promise<unknown> {
    const shardIndex = this.actorShard.get(actorId);
    if (shardIndex === undefined) throw new Error(`[DEBUG] ShardClient.turn(${actorId}): not activated on any shard`);
    const slot = this.shards[shardIndex]!;
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "turn", requestId, actorId, events, budget }, requestId);
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
    const requestId = this.nextRequestId();
    return this.send(slot, { kind: "frame", requestId, actorId: shardEnvelope.to, frame: { kind: "Envelope", envelope: shardEnvelope } }, requestId);
  }

  /** ⚖️ terra-web-shardframe: `ShardFrame::Grant` — `budget` travels WITH `envelopes` in ONE wire
   * message (design-runtime.md's DRR promise), sent to the worker in LANE-PRIORITY order via
   * {@link orderEnvelopesByLane} rather than push/arrival order. The worker remembers `budget` as
   * `actorId`'s new granted budget for any later {@link envelope} passthrough. */
  async grant(actorId: string, budget: ShardBudget, envelopes: readonly ShardEnvelope[]): Promise<unknown> {
    const slot = this.requireShard(actorId);
    const requestId = this.nextRequestId();
    const ordered = orderEnvelopesByLane(envelopes);
    return this.send(slot, { kind: "frame", requestId, actorId, frame: { kind: "Grant", actor: actorId, budget, envelopes: ordered } }, requestId);
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
    const shardIndex = this.actorShard.get(actorId);
    if (shardIndex === undefined) return;
    this.abortOutstandingEffects(actorId);
    this.shards[shardIndex]!.worker.postMessage({ kind: "dispose", actorId } satisfies OutboundMessage);
    this.shards[shardIndex]!.actorIds.delete(actorId);
    this.actorShard.delete(actorId);
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
  private handleInboundFrame(slot: ShardSlot, actorId: string, frame: ShardFrame): void {
    if (frame.kind !== "Envelope") return;
    const payload = frame.envelope.payload;
    if (payload.kind !== "effect-request") return;
    const request = payload.payload as { readonly effect: string; readonly requestId: string; readonly params: unknown };
    this.handleEffectRequest(slot, actorId, request.effect, request.requestId, request.params);
  }

  /** 🚪️ Answers one `effect-request`: quota-checks against {@link maxOutstandingEffectsPerActor}, then
   * hands off to {@link onHostEffect} — or, absent one, fails FAST with an explicit `effect-error`
   * rather than ever leaving the guest's `.await` pending, per this ticket's own acceptance bar. Always
   * settles exactly once, via {@link replyEffectComplete}/{@link replyEffectError}. */
  private handleEffectRequest(slot: ShardSlot, actorId: string, effect: string, requestId: string, params: unknown): void {
    const outstanding = this.outstandingEffectsByActor.get(actorId) ?? new Map<string, AbortController>();
    if (outstanding.size >= this.maxOutstandingEffectsPerActor) {
      const breach: ShardQuotaBreach = { quota: "outstandingRequests", limit: this.maxOutstandingEffectsPerActor, actual: outstanding.size };
      this.replyEffectError(slot, actorId, requestId, formatQuotaBreachMessage(breach));
      return;
    }
    if (!this.onHostEffect) {
      this.replyEffectError(slot, actorId, requestId, "no host effect handler installed");
      return;
    }
    const controller = new AbortController();
    outstanding.set(requestId, controller);
    this.outstandingEffectsByActor.set(actorId, outstanding);
    this.onHostEffect(actorId, effect, params, controller.signal).then(
      (value) => {
        if (this.settleEffect(actorId, requestId)) this.replyEffectComplete(slot, actorId, requestId, value);
      },
      (error: unknown) => {
        if (this.settleEffect(actorId, requestId)) this.replyEffectError(slot, actorId, requestId, error instanceof Error ? error.message : String(error));
      },
    );
  }

  /** ✅ Removes `requestId` from the outstanding-effect ledger; returns `false` if it was already gone
   * (settled once already, or cleared by {@link abortOutstandingEffects} while in flight) — the caller
   * must then skip posting a reply: the shard/actor a late reply would target may already be gone, or
   * worse, a DIFFERENT actor instance may since have reused the same id after a fresh `activate()`. */
  private settleEffect(actorId: string, requestId: string): boolean {
    const outstanding = this.outstandingEffectsByActor.get(actorId);
    if (!outstanding || !outstanding.delete(requestId)) return false;
    if (outstanding.size === 0) this.outstandingEffectsByActor.delete(actorId);
    return true;
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
    for (const controller of outstanding.values()) controller.abort();
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
  private postEffectReply(slot: ShardSlot, actorId: string, kind: "effect-complete" | "effect-error", innerPayload: unknown): void {
    this.effectReplySeq += 1;
    const frame: ShardFrame = {
      kind: "Envelope",
      envelope: { to: actorId, from: { kind: "kernel" }, lane: "Background", seq: this.effectReplySeq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload: innerPayload } },
    };
    slot.worker.postMessage({ kind: "frame", requestId: this.nextRequestId(), actorId, frame } satisfies OutboundMessage);
  }

  private replyEffectComplete(slot: ShardSlot, actorId: string, requestId: string, value: unknown): void {
    this.postEffectReply(slot, actorId, "effect-complete", { requestId, value });
  }

  private replyEffectError(slot: ShardSlot, actorId: string, requestId: string, message: string): void {
    this.postEffectReply(slot, actorId, "effect-error", { requestId, message });
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
  function makeEffectRequestFrame(actorId: string, effect: string, requestId: string, params: unknown): InboundMessage {
    return { kind: "frame", actorId, frame: { kind: "Envelope", envelope: { to: "kernel", from: { kind: "actor", id: actorId }, lane: "Background", seq: 1, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind: "effect-request", payload: { effect, requestId, params } } } } };
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
      workers[0]!.deliver(makeEffectRequestFrame("b", "http-fetch", "b:http-fetch:1", {}));

      client.dispose("a");
      expect(signals.a?.aborted).toBe(true);
      expect(signals.b?.aborted).toBe(false);
    });
  });
  //#endregion 🌉️HostEffectBridge tests

  void vi;
}
//#endregion 🧪️Tests
