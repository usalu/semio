/** 🧵️ `ShardClient` — the web `ShardTransport` (design-runtime.md §1 `ShardTransport` /
 * §3 "Web shard"): a bounded pool of `🟨️shard-worker.js` Web Workers multiplexed by `actorId`,
 * replacing one-Worker-per-plugin (`PluginWorkerClient`, deleted from `🎠️kernel/🟦️.ts` in
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
import type { Lane, CoalesceKey } from "../🤖️generated/🟦️actor.ts";
import { actorInstanceCapturedReceiptMatches, actorInstanceCloseReceiptMatches, actorInstanceLifecycleReceiptEquals, actorInstanceLifetimeEquals, decodeActorInstanceLifecycle, encodeActorInstanceLifecycle, type ActorInstanceLifecycleReceipt, type ActorInstanceCloseRequest, type ActorInstanceOpenRequest, type ActorInstanceLifetime } from "../🚪️lifetime/🟦️.ts";
import { actorUiPatchReceiptEquals, decodeActorUiPatchReceipt, encodeActorUiPatchReceipt, validateActorUiPatchPairing, type ActorUiPatchReceipt } from "../🚪️lifetime/🩹️patch/🟦️.ts";
import { OwnedActorTurnOutputs, OwnedActorTurnOutput } from "../🪪️activation/🚪️instance/📥️output/🟦️.ts";
import { ACTOR_BYTE_PAGE_BYTES, createActorBytePage, type ActorBytePage } from "../📃️page/🟦️.ts";
import { encodeActorReturnDrive, decodeActorReturnResult, type ActorReturnOrigin, type ActorReturnIdentity, type ActorReturnPageReceipt, type ActorReturnDrive, type ActorReturnResult } from "../📤️return/🟦️.ts";
export { encodeActorReturnDrive, decodeActorReturnDrive, encodeActorReturnResult, decodeActorReturnResult, ACTOR_RETURN_RESULT_MAXIMUM_BYTES, type ActorReturnOrigin, type ActorReturnIdentity, type ActorReturnPageReceipt, type ActorReturnControl, type ActorReturnDrive, type ActorReturnResult, type ActorReturnFault } from "../📤️return/🟦️.ts";
import { OwnedUiInstance, OwnedUiInstanceRetirement, OwnedUiPatchAcknowledgement, OwnedUiPatchInputAcceptance, OwnedUiPatchInputRetirement } from "../../🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts";
import { OwnedKernelReturnContent } from "../../🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts";
import { OwnedResidentLedger, OwnedResidentRecordDetachment, OwnedResidentRetirement, type OwnedResidentAdmission, type OwnedResidentRecord, type ResidentGrant, type ResidentStep } from "../../🌱️value/💾️resident/🟦️.ts";
import { OwnedUiResidentPool, OwnedUiResidentPoolRetirement, type OwnedUiResidentInstance, type OwnedUiResidentPayload, type OwnedUiResidentPayloadSourceRelease as UiResidentSourceProof } from "../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts";
import { uiResidentMetadataEnvelope } from "../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🟦️.ts";
/** 🧬️ Brand-check accessor for {@link OwnedResidentLedger}, resolved LAZILY on first use.
 * `OwnedResidentLedger` arrives over an import cycle (`📮️shard-client` → `🎠️kernel/📥️input` →
 * `🖱️ui/…/💾️resident` → back here), and reading `.prototype` at module-evaluation time touches the
 * binding while that cycle is still initializing — which is a TDZ
 * (`ReferenceError: Cannot access 'X' before initialization`) in any bundled build, killing the whole
 * preview before it mounts. Deferring the lookup to first call moves it past module evaluation; the
 * check itself is unchanged. */
let residentCapacityGetter: (() => number) | undefined;
const residentCapacity = (): (() => number) => (residentCapacityGetter ??= Object.getOwnPropertyDescriptor(OwnedResidentLedger.prototype, "capacity")!.get! as () => number);
const NO_RESIDENT_FAULT = Symbol("actor-resident.no-fault");
const poolUiEnvelope = uiResidentMetadataEnvelope("pool");
const poolRecordEnvelope = poolUiEnvelope;
const poolControllerEnvelope = Object.freeze({ bytes: 224, slots: 1, owners: 1 });
const workerControllerEnvelope = Object.freeze({ bytes: 128, slots: 0, owners: 0 });
const residentStep = (kind: ResidentStep["kind"], phase: string, bytes = 0): ResidentStep => ({ kind, phase, items: bytes ? 1 : 0, bytes });
const residentGrant = (grant: ResidentGrant, bytes: number): boolean => Number.isSafeInteger(grant.maxItems) && grant.maxItems >= 1 && Number.isSafeInteger(grant.maxBytes) && grant.maxBytes >= bytes;
function residentChild(current: ResidentStep, grant: ResidentGrant): ResidentStep {
  if (!Number.isSafeInteger(current.items) || current.items < 0 || current.items > 1 || !Number.isSafeInteger(current.bytes) || current.bytes < 0 || current.bytes > grant.maxBytes) return residentStep("rejected", "actor-resident.child-grant");
  return current.kind === "complete" || current.kind === "ready" ? { ...current, kind: "pending" } : current;
}
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
 * activates, see `🟦️.ts`'s `shardWorkerSource` doc). */
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
/** ⚖️ `semio_framework_actor::lane_defaults::budget_for(Lane::Maintenance)` (`🎭️actor/🦀️.rs`,
 * `lane_defaults` module) — the ONLY floor a granted-less actor may fall back to, mirrored field-for-
 * field/value-for-value: `{ fuel: 80_000_000, wall_ms: 200, memory_bytes: 256 MiB, ui_nodes: 4_000,
 * mailbox_len: 1024, max_effects: 512, max_patch_bytes: 2_097_152 }`. {@link GrantedBudgetTracker}
 * falls back to this so a budget-less `Envelope` arriving before any `Grant` for its actor never
 * invents its own number — the same floor `ShardLoop::granted_budget` falls back to natively. */
export const MAINTENANCE_LANE_DEFAULT_BUDGET: ShardBudget = { fuel: 80_000_000, wallMs: 200, memoryBytes: 256 * 1024 * 1024, uiNodes: 4_000, mailboxLen: 1024, maxEffects: 512, maxPatchBytes: 2_097_152 };

/** ⚖️ Rust `Origin` mirror (`🎭️actor/🦀️.rs`) — who sent a {@link ShardEnvelope}. `window`
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

/** 📨️ TypeScript mirror of Rust `ShardFrame` (`🖥️host/🧵️shard/🦀️.rs`) — see the in-source
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

/** 🎯️ Stable sort by {@link Lane} priority — the SAME `LANE_ORDER` `🟦️.ts`'s
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
 * `granted_budget()` (`🖥️host/🧵️shard/🦀️.rs`) — remembers the LAST `ShardFrame::Grant`
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
 * (`🟨️.js`'s `effectRequest` — 🧪️ terra-web-bridges) — the ONE seam `http-fetch`/`blob-read`/
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
  /** 🫀️ `phase` names the generated worker's await boundary this beat was emitted at
   * (`module-fetch`/`module-ready`/`actor-ready`, or `progress` from its while-busy ticker); absent on
   * the unconditional start-of-request beat. Diagnostic only — {@link evaluateShardLiveness} treats
   * every beat identically, so a future phase needs no host change to keep a shard alive. */
  | { readonly kind: "heartbeat"; readonly turnSeq: number; readonly phase?: string }
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
//#endregion 📨️WireMessages

//#region ⏱️Heartbeat
/** 🫀️ THE shard liveness policy — one record every liveness clock in the system reads, so a value can
 * never drift between the host watchdog here, the generated `🟨️shard-worker.js` progress ticker
 * (`🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`'s `shardWorkerSource`, which interpolates
 * `progressIntervalMs` straight out of the fixture below) and the shell's per-plugin load deadline
 * (`🛠️ShellHelpers/🟦️.tsx`'s `loadPluginModuleResilient`). Language-agnostic owner:
 * `🧬️schema/🔣️.json` (`https://semio.tech/schema/framework/actor/shard-client/schema.json#/$defs/ShardClient`) + `🧪️fixture/🔣️.json`'s `policy` block; this
 * mirror is asserted field-for-field equal to that fixture by this file's own in-source suite, so a
 * literal edited here alone fails closed rather than silently diverging. */
export const SHARD_LIVENESS_POLICY = Object.freeze({
  heartbeatTimeoutMs: 5000,
  missedLimit: 3,
  progressIntervalMs: 1000,
  pluginLoadIdleTimeoutMs: 30_000,
  pluginLoadCeilingMs: 300_000,
});
/** 🚦️ terra-shard-effect-bridge: default cap on CONCURRENT unresolved `effect-request`s per actor —
 * see {@link ShardClientOptions.maxOutstandingEffectsPerActor}'s own doc for why this mirrors
 * `QuotaSchema.outstanding_requests` without being it. */
const DEFAULT_MAX_OUTSTANDING_EFFECTS_PER_ACTOR = 64;

type ShardHeartbeatState = {
  lastHeartbeatAtMs: number;
  lastHeartbeatTurnSeq: number;
  lastLivenessAtMs: number;
  oldestPendingStartedAtMs: number | null;
  missedCount: number;
  lastMissCountedAtMs: number;
};

/** A freshly spawned shard has never actually heartbeated yet — `-Infinity` (never `nowMs`) so a turn
 * that starts in the very same tick as `spawnShard` doesn't spuriously count spawn-time as proof of
 * life for that turn's whole timeout window. */
function freshHeartbeatState(nowMs: number): ShardHeartbeatState {
  return { lastHeartbeatAtMs: Number.NEGATIVE_INFINITY, lastHeartbeatTurnSeq: 0, lastLivenessAtMs: Number.NEGATIVE_INFINITY, oldestPendingStartedAtMs: null, missedCount: 0, lastMissCountedAtMs: nowMs };
}

/** 🫀️ One watchdog window's worth of input — every field the rule below reads, and nothing else, so
 * the same decision can be replayed from a JSON timeline with no `ShardClient` in the picture. */
export type ShardLivenessWindow = {
  readonly nowMs: number;
  readonly oldestPendingStartedAtMs: number | null;
  readonly lastLivenessAtMs: number;
  readonly missedCount: number;
  readonly lastMissCountedAtMs: number;
  readonly heartbeatTimeoutMs: number;
};

export type ShardLivenessDecision = {
  readonly missedCount: number;
  readonly lastMissCountedAtMs: number;
  readonly terminate: boolean;
};

/** 🚑️ design-runtime.md §1 `FailurePolicy`, restated so BUSY and DEAD stop being the same thing.
 *
 * A shard is only a candidate while it has an in-flight request — an idle shard can never be flagged.
 * The clock that matters is `lastLivenessAtMs`: the last moment ANY message arrived from that worker
 * (a start-of-request heartbeat, one of the generated worker's boundary/progress heartbeats, a
 * `result`, a `trap`, an effect `frame`). `max(lastLivenessAtMs, oldestPendingStartedAtMs)` is the
 * newest instant this shard is PROVEN to have been alive with work outstanding; anything older than
 * `heartbeatTimeoutMs` is real silence and costs one miss per window, `missedLimit` of them in a row
 * terminating the worker.
 *
 * The pre-fix rule compared `lastHeartbeatAtMs >= oldestPendingStartedAtMs` — an absolute timestamp
 * against a DIFFERENT request's start — so a worker that heartbeated once and then wedged forever
 * looked healthy, while a worker legitimately busy inside one multi-second `await import()` of a
 * multi-MB wasm component (nothing to heartbeat about mid-turn) was killed the moment an unrelated
 * newer request became the oldest pending. That is the `shard 0 terminated` boot fault this rule
 * replaces: liveness is now proven CONTINUOUSLY by the worker's progress ticker (which can only fire
 * while its event loop is actually running), so "busy" keeps proving itself and "dead" — no messages
 * at all — still dies after exactly the same `missedLimit` windows. */
export function evaluateShardLiveness(window: ShardLivenessWindow): ShardLivenessDecision {
  const unchanged = { missedCount: window.missedCount, lastMissCountedAtMs: window.lastMissCountedAtMs, terminate: false };
  if (window.oldestPendingStartedAtMs === null) return unchanged;
  const provenAliveAtMs = Math.max(window.lastLivenessAtMs, window.oldestPendingStartedAtMs);
  if (window.nowMs - provenAliveAtMs <= window.heartbeatTimeoutMs) return unchanged;
  if (window.nowMs - window.lastMissCountedAtMs < window.heartbeatTimeoutMs) return unchanged;
  const missedCount = window.missedCount + 1;
  return { missedCount, lastMissCountedAtMs: window.nowMs, terminate: missedCount >= SHARD_LIVENESS_POLICY.missedLimit };
}

/** 🩺️ Names the one failure the shell's boot path may retry rather than surface: the watchdog above
 * (or a `worker.onerror` crash) took this shard down under load, `rebuild()` already replaced it, and
 * every request in flight was rejected with this message. */
export function isShardLostError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : typeof error === "string" ? error : "";
  return /shard \d+ (?:terminated|worker crashed)/.test(message);
}

/** 🩺️ A `Worker`'s `onerror` hands the main thread an `ErrorEvent`, and `console.error`-ing that
 * object prints `Event` and nothing else — which is all the boot log said while four shards died on a
 * top-of-file `throw` inside the worker script. Reads the three fields that actually name the cause
 * (`message`/`filename`/`lineno`) and degrades explicitly when the event is the redacted, message-less
 * kind a cross-origin or failed-to-load worker script produces. */
export function describeShardWorkerError(event: unknown): string {
  const record = (event ?? {}) as { readonly message?: unknown; readonly filename?: unknown; readonly lineno?: unknown; readonly colno?: unknown; readonly type?: unknown; readonly error?: { readonly message?: unknown } };
  const message =
    typeof record.message === "string" && record.message.length > 0
      ? record.message
      : typeof record.error?.message === "string" && record.error.message.length > 0
        ? record.error.message
        : typeof record.type === "string"
          ? `redacted "${record.type}" event with no message — the worker script threw before it could report, or failed to load`
          : String(event);
  if (typeof record.filename !== "string" || record.filename.length === 0) return message;
  return `${message} at ${record.filename}:${typeof record.lineno === "number" ? record.lineno : "?"}:${typeof record.colno === "number" ? record.colno : "?"}`;
}

/** 🩺️ One readable line out of a `worker-fault` payload — phase, actor, module URL and location, in
 * that order, so a boot log names WHERE inside the worker it died rather than only THAT it did. */
export function formatShardWorkerFault(shardIndex: number, fault: { readonly source: string; readonly phase: string; readonly actorId: string | null; readonly moduleUrl: string | null; readonly message: string; readonly filename?: string; readonly lineno?: number }): string {
  const where = fault.filename ? ` at ${fault.filename}:${fault.lineno ?? "?"}` : "";
  const actor = fault.actorId ? ` actor=${fault.actorId}` : "";
  const module = fault.moduleUrl ? ` module=${fault.moduleUrl}` : "";
  return `shard ${shardIndex} worker fault [${fault.source}/${fault.phase}]${actor}${module}: ${fault.message}${where}`;
}

/** 🧪️ The one capability every shard worker needs before it can host anything: jco's glue for the
 * fully async-lifted `world actor` unconditionally constructs `WebAssembly.Suspending` /
 * `WebAssembly.promising`, so without JSPI the worker script throws at MODULE TOP LEVEL — every shard
 * in the pool dies at spawn and the boot degrades into dozens of unrelated-looking load timeouts.
 * Probed on the main thread, BEFORE any worker exists, so that failure is reported once, by name,
 * instead of `missedLimit` windows later as four anonymous `Event`s. */
export const SHARD_JSPI_FAULT_CODE = "plugin.runtime.jspi-unavailable";

/** 🌐️ English first, German second, no default language (this is net-new operator vocabulary and the
 * shell's own chrome dictionary is not this module's to extend). */
export const SHARD_JSPI_FAULT_TEXT = Object.freeze({
  en: "This browser cannot run semio plugins: WebAssembly JavaScript Promise Integration (WebAssembly.Suspending / WebAssembly.promising) is unavailable. Chromium-based browsers ship it on by default; Firefox needs javascript.options.wasm_js_promise_integration in about:config, Node.js needs --experimental-wasm-jspi, and headless Chromium needs --enable-features=WebAssemblyJavaScriptPromiseIntegration.",
  de: "Dieser Browser kann semio-Plugins nicht ausführen: WebAssembly JavaScript Promise Integration (WebAssembly.Suspending / WebAssembly.promising) ist nicht verfügbar. Chromium-basierte Browser liefern sie standardmäßig aus; Firefox benötigt javascript.options.wasm_js_promise_integration in about:config, Node.js --experimental-wasm-jspi und headless Chromium --enable-features=WebAssemblyJavaScriptPromiseIntegration.",
});

export type ShardJspiScope = { readonly WebAssembly?: { readonly Suspending?: unknown; readonly promising?: unknown } };

export function shardJspiAvailable(scope: ShardJspiScope = globalThis as ShardJspiScope): boolean {
  const runtime = scope.WebAssembly;
  return typeof runtime === "object" && runtime !== null && typeof runtime.Suspending === "function" && typeof runtime.promising === "function";
}

/** 🩺️ Typed, bilingual, fail-fast form of the probe above — `code` is what `windowFaultFromError`
 * reads off the shell's error surface, `text` is what a human reads. */
export class ShardJspiUnavailableError extends Error {
  readonly code = SHARD_JSPI_FAULT_CODE;
  readonly text = SHARD_JSPI_FAULT_TEXT;
  constructor() {
    super(`${SHARD_JSPI_FAULT_CODE}: ${SHARD_JSPI_FAULT_TEXT.en} — ${SHARD_JSPI_FAULT_TEXT.de}`);
  }
}

export function assertShardJspiAvailable(scope: ShardJspiScope = globalThis as ShardJspiScope): void {
  if (!shardJspiAvailable(scope)) throw new ShardJspiUnavailableError();
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
  reserveReturn(maximumResponses: number, grant: ResidentGrant): ShardReturnAdmission;
  retireUnusedReturn(grant: ResidentGrant): ResidentStep;
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

const ACTIVATION_MINT = Symbol("actor-activation.private-lease");
let mintCapturedActivation: (client: ShardClient, actorId: string, generation: bigint, assertActive: ShardActorActivationLease["assertActive"], turn: ShardActorActivationLease["turn"]) => ShardActorActivationLease;
let capturedActivationMatches: (activation: unknown, client: ShardClient) => boolean;
/** 🧷️ Original composition identity survives revocation without granting new operation authority. */
class CapturedShardActivation implements ShardActorActivationLease {
  readonly #client: ShardClient;
  readonly actorId: string;
  readonly activationGeneration: bigint;
  readonly assertActive: ShardActorActivationLease["assertActive"];
  readonly turn: ShardActorActivationLease["turn"];
  private constructor(mint: symbol, client: ShardClient, actorId: string, generation: bigint, assertActive: ShardActorActivationLease["assertActive"], turn: ShardActorActivationLease["turn"]) {
    if (mint !== ACTIVATION_MINT) throw new Error("actor-activation.private-lease");
    this.#client = client; this.actorId = actorId; this.activationGeneration = generation; this.assertActive = assertActive; this.turn = turn; Object.freeze(this);
  }
  static {
    mintCapturedActivation = (client, actorId, generation, assertActive, turn) => new CapturedShardActivation(ACTIVATION_MINT, client, actorId, generation, assertActive, turn);
    capturedActivationMatches = (activation, client) => activation !== null && typeof activation === "object" && #client in activation && activation.#client === client;
  }
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
  returnCell: OwnedResidentAdmission | null;
  returnRecord: OwnedResidentRecord | null;
  returnPhase: ReturnAdmissionPhase;
  returnFault: unknown;
  returnCapacity: number;
};

//#region 📤️CapturedReturnAuthority
export type ShardReturnReport = Exclude<ActorReturnResult, { kind: "page" }> | { readonly kind: "page"; readonly receipt: ActorReturnPageReceipt };
export type ShardReturnAdmission = { readonly step: ResidentStep; readonly source: OwnedShardReturn | null };
type ReturnAdmissionPhase = "empty" | "preparing" | "cell-held" | "claiming" | "claimed" | "record-admitting" | "record-held" | "installing" | "installed" | "state-held" | "roster-held" | "facade-held" | "published" | "closing" | "rejected";
type CapturedReturnWork = { readonly kind: "execute"; readonly events: readonly ShardEventEnvelope[] } | { readonly kind: "retry" | "poll" | "cancel" };
type CapturedReturn = { instance: ShardInstanceOwner | null; outputs: OwnedActorTurnOutputs | null; client: ShardClient | null; facade: OwnedShardReturn | null; origin: ActorReturnOrigin | null; identity: ActorReturnIdentity | null; events: readonly ShardEventEnvelope[] | null; latest: OwnedActorTurnOutput | null; page: OwnedShardReturnPage | null; content: OwnedKernelReturnContent | null; inFlight: boolean; retry: boolean; failed: boolean; fault: unknown; cancelled: boolean; retired: boolean };
const RETURN_MINT = Object.freeze({});
const NO_RETURN_FAULT = Object.freeze({});
const returnDomainEnvelope = Object.freeze({ bytes: 800, slots: 4, owners: 4 });
function returnAdmission(kind: ResidentStep["kind"], phase: string, bytes = 0): ShardReturnAdmission { return { step: residentStep(kind, phase, bytes), source: null }; }
let mintCapturedReturn: (state: CapturedReturn) => OwnedShardReturn;
let capturedReturnState: (owner: OwnedShardReturn) => CapturedReturn;
let submitCapturedReturn: (client: ShardClient, state: CapturedReturn, work: CapturedReturnWork, budget: ShardBudget) => Promise<ShardReturnReport>;
let reserveCapturedResponse: (client: ShardClient, state: CapturedReturn, grant: ResidentGrant) => ResidentStep;
let mintCapturedReturnPage: (state: CapturedReturn, output: OwnedActorTurnOutput, receipt: ActorReturnPageReceipt, page: ActorBytePage) => OwnedShardReturnPage;
function sameReturnOrigin(left: ActorReturnOrigin, right: ActorReturnOrigin): boolean { return left.activationGeneration === right.activationGeneration && left.requestSequence === right.requestSequence; }
function sameReturnIdentity(left: ActorReturnIdentity, right: ActorReturnIdentity): boolean { return sameReturnOrigin(left.origin, right.origin) && left.returnSequence === right.returnSequence; }

/** 📤️ A captured instance retains fixed responses before callers can observe them; controls carry no new semantic events. */
export class OwnedShardReturn {
  readonly #state: CapturedReturn;
  private constructor(mint: object, state: CapturedReturn) { if (mint !== RETURN_MINT) throw new Error("actor-return.private-owner"); this.#state = state; state.facade = this; Object.freeze(this); }
  static { mintCapturedReturn = state => new OwnedShardReturn(RETURN_MINT, state); capturedReturnState = owner => owner.#state; }
  static matchesOwner(source: unknown, owner: OwnedUiInstance, activation: ShardActorActivationLease, lifetime: ActorInstanceLifetime): source is OwnedShardReturn {
    if (source === null || typeof source !== "object" || !(#state in source)) return false;
    const instance = source.#state.instance;
    return instance !== null && instance.host === owner && instance.operation === activation && instance.lifetime !== null && actorInstanceLifetimeEquals(instance.lifetime, lifetime);
  }
  get origin(): ActorReturnOrigin | null { return this.#state.origin; }
  get page(): OwnedShardReturnPage | null { return this.#state.page; }
  get content(): OwnedKernelReturnContent | null { return this.#state.content; }
  bindContent(content: OwnedKernelReturnContent): boolean {
    const state = this.#state; const instance = state.instance;
    if (!instance || instance.returnPhase !== "published" || state.content !== null || !instance.host || !instance.lifetime || !OwnedKernelReturnContent.matches(content, this, instance.host, instance.operation, instance.lifetime)) return false;
    state.content = content; return true;
  }
  get retainedResponses(): number { return this.#state.outputs?.pending ?? 0; }
  reserveResponse(grant: ResidentGrant): ResidentStep { return this.#state.client ? reserveCapturedResponse(this.#state.client, this.#state, grant) : residentStep("rejected", "actor-return.closed"); }
  execute(events: readonly ShardEventEnvelope[], budget: ShardBudget): Promise<ShardReturnReport> { return this.#submit({ kind: "execute", events }, budget); }
  retry(budget: ShardBudget): Promise<ShardReturnReport> { return this.#submit({ kind: "retry" }, budget); }
  poll(budget: ShardBudget): Promise<ShardReturnReport> { return this.#submit({ kind: "poll" }, budget); }
  cancel(budget: ShardBudget): Promise<ShardReturnReport> { return this.#submit({ kind: "cancel" }, budget); }
  #submit(work: CapturedReturnWork, budget: ShardBudget): Promise<ShardReturnReport> { return this.#state.client ? submitCapturedReturn(this.#state.client, this.#state, work, budget) : Promise.reject(new Error("actor-return.closed")); }
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
    return instance !== null && instance.host === owner && instance.operation === activation && instance.lifetime !== null && actorInstanceLifetimeEquals(instance.lifetime, lifetime) && page.#receipt.identity.origin.activationGeneration === lifetime.activationGeneration && page.#output.responseEnvelope !== null;
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
  #uiResidentControllerCell: OwnedResidentAdmission | null = null;
  #uiResidentControllerRecord: OwnedResidentRecord | null = null;
  #uiResidentCell: OwnedResidentAdmission | null = null;
  #uiResidentRecord: OwnedResidentRecord | null = null;
  #uiResidentPool: OwnedUiResidentPool | null = null;
  #uiResidentPhase: "controller-empty" | "controller-preparing" | "controller-prepare-refused" | "controller-cell-held" | "controller-claiming" | "controller-claimed" | "controller-record-admitting" | "controller-installing" | "controller-observing" | "controller-rejected" | "empty" | "preparing" | "prepare-refused" | "cell-held" | "claiming" | "claimed" | "record-admitting" | "prepared" | "rejected" | "unused-closing" | "record-observing" | "cell-closing" | "cell-observing" | "owned" | "pool-closing" | "pool-observing" | "pool-proved" | "closing" | "detached" | "retired" = "controller-empty";
  #uiResidentWitness: OwnedUiResidentPoolRetirement | null = null;
  #uiResidentFault: unknown = NO_RESIDENT_FAULT;
  #uiResidentClosing = false;
  #clientAdmissionPurpose: "none" | "ui-pool" | "worker-root" = "none";
  #workerBootstrapCell: OwnedResidentAdmission | null = null;
  #workerBootstrapRecord: OwnedResidentRecord | null = null;
  #workerBootstrapPhase: "empty" | "preparing" | "prepare-refused" | "cell-held" | "claiming" | "claim-refused" | "claimed" | "record-admitting" | "record-refused" | "installing" | "observing" | "ready" | "close-preparing" | "close-prepare-refused" | "close-cell-held" | "close-claiming" | "close-record-admitting" | "close-record-refused" | "record-held" | "cell-closing" | "close-attempted" | "pending-release-observing" | "cell-observing" | "fault-held" | "cancelled" = "empty";
  #workerBootstrapFault: unknown = NO_RESIDENT_FAULT;
  #workerAdmissionCell: OwnedResidentAdmission | null = null;
  #workerAdmissionRecord: OwnedResidentRecord | null = null;
  #workerAdmissionIndex: number | null = null;
  #workerAdmissionShell: ShardSlot | null = null;
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
    try { Reflect.apply(residentCapacity(), options.residentLedger, []); } catch { throw new Error("actor-resident.invalid-ledger"); }
    this.#residentLedger = options.residentLedger;
    if (options.shardCount < 1) throw new Error("[DEBUG] ShardClient requires shardCount >= 1");
    this.createWorker = options.createWorker;
    this.now = options.now ?? (() => Date.now());
    this.heartbeatTimeoutMs = options.heartbeatTimeoutMs ?? SHARD_LIVENESS_POLICY.heartbeatTimeoutMs;
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
  static { submitCapturedReturn = (client, state, work, budget) => client.sendCapturedReturn(state, work, budget); reserveCapturedResponse = (client, state, grant) => client.reserveReturnResponse(state, grant); }
  static matchesResidentLedger(client: unknown, ledger: unknown): client is ShardClient { return client !== null && typeof client === "object" && #residentLedger in client && client.#residentLedger === ledger; }

  /** 🧭️ Checks the privately captured original client; operation liveness is independently asserted. */
  static matchesActivation(client: unknown, activation: unknown): activation is ShardActorActivationLease {
    return client !== null && typeof client === "object" && #residentLedger in client && capturedActivationMatches(activation, client);
  }

  /** 🏗️ Prepares only the original client's separately charged worker metadata. */
  prepareWorkerBootstrap(grant: ResidentGrant): ResidentStep {
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-worker.prepare");
    switch (this.#workerBootstrapPhase) {
      case "close-preparing": case "close-prepare-refused": case "close-cell-held": case "close-claiming": case "close-record-admitting": case "close-record-refused": case "record-held": case "cell-closing": case "close-attempted": case "pending-release-observing": case "cell-observing": case "fault-held": case "cancelled": return residentStep("rejected", "actor-worker.stopped");
    }
    try {
      const recovered = this.#recoverWorkerBootstrap(); if (recovered) return recovered;
      if (this.#workerBootstrapFault !== NO_RESIDENT_FAULT || this.#workerBootstrapCell?.hasFailure) return residentStep("rejected", "actor-worker.fault-held");
    } catch (error) { this.#captureWorkerBootstrapFault(error); return residentStep("rejected", "actor-worker.recovery-fault"); }
    try { const shared = this.#prepareSharedResidentController(grant); if (shared) return shared; }
    catch (error) { this.captureUiResidentPoolFault(error); return residentStep("rejected", "actor-worker.shared-fault"); }
    try {
      if (this.#workerBootstrapPhase === "ready") return residentStep(this.#workerBootstrapRecord?.matchesLiveShell(this) ? "ready" : "rejected", "actor-worker.prepared");
      if (this.#workerBootstrapPhase === "empty") {
        if (this.#clientAdmissionPurpose !== "none") return residentStep("blocked", "actor-worker.foreign-purpose");
        if (this.#workerAdmissionCell || this.#workerAdmissionRecord || this.#workerAdmissionIndex !== null || this.#workerAdmissionShell) return residentStep("rejected", "actor-worker.child-held");
        if (!residentGrant(grant, 296)) return residentStep("blocked", "actor-worker.bootstrap");
        this.#clientAdmissionPurpose = "worker-root"; this.#workerBootstrapPhase = "preparing";
        const current = this.#residentLedger.prepareAdmission(this, "data", grant);
        if (current.kind === "blocked" || current.kind === "rejected") this.#workerBootstrapPhase = "prepare-refused";
        return residentChild(current, grant);
      }
      const cell = this.#workerBootstrapCell; if (!cell) return residentStep("rejected", "actor-worker.cell");
      if (this.#workerBootstrapPhase === "cell-held") {
        if (this.#clientAdmissionPurpose !== "worker-root") return residentStep("blocked", "actor-worker.foreign-purpose");
        this.#workerBootstrapPhase = "claiming"; const current = this.#residentLedger.claimAdmission(this, cell, grant);
        if (current.kind === "blocked") this.#workerBootstrapPhase = "cell-held";
        else if (current.kind === "rejected") this.#workerBootstrapPhase = "claim-refused";
        return residentChild(current, grant);
      }
      if (this.#workerBootstrapPhase === "claimed") {
        if (!residentGrant(grant, 264)) return residentStep("blocked", "actor-worker.record");
        this.#workerBootstrapPhase = "record-admitting"; const admitted = this.#residentLedger.reserveRecord("data", workerControllerEnvelope, cell, grant);
        if (admitted.step.kind === "blocked") this.#workerBootstrapPhase = "claimed";
        else if (admitted.step.kind === "rejected") this.#workerBootstrapPhase = "record-refused";
        return residentChild(admitted.step, grant);
      }
      if (this.#workerBootstrapPhase === "installing" && this.#workerBootstrapRecord) {
        this.#workerBootstrapPhase = "observing"; return residentChild(this.#workerBootstrapRecord.install(this, grant), grant);
      }
      return residentStep("rejected", "actor-worker.admission");
    } catch (error) { this.#captureWorkerBootstrapFault(error); return residentStep("rejected", "actor-worker.prepare-fault"); }
  }

  #captureWorkerBootstrapFault(error: unknown): void {
    if (this.#workerBootstrapFault === NO_RESIDENT_FAULT) this.#workerBootstrapFault = error;
    else if (!Object.is(this.#workerBootstrapFault, error)) throw error;
  }

  #recoverWorkerBootstrap(): ResidentStep | null {
    const phase = this.#workerBootstrapPhase;
    if (phase === "preparing" || phase === "prepare-refused" || phase === "close-preparing" || phase === "close-prepare-refused") {
      if (this.#clientAdmissionPurpose !== "worker-root") return residentStep("blocked", "actor-worker.foreign-purpose");
      const cell = this.#residentLedger.preparedAdmission(this);
      if (!cell) {
        if ((phase !== "prepare-refused" && phase !== "close-prepare-refused") || this.#workerBootstrapFault !== NO_RESIDENT_FAULT) return residentStep("blocked", "actor-worker.admission-handoff");
        this.#clientAdmissionPurpose = "none"; this.#workerBootstrapPhase = phase === "close-prepare-refused" ? "cancelled" : "empty";
        return residentStep("pending", "actor-worker.empty-admission-observation", 64);
      }
      this.#workerBootstrapCell = cell; this.#workerBootstrapPhase = phase === "close-preparing" || phase === "close-prepare-refused" ? "close-cell-held" : "cell-held";
      return residentStep("pending", "actor-worker.cell-observation", 64);
    }
    const cell = this.#workerBootstrapCell;
    if ((phase === "claiming" || phase === "close-claiming") && cell) {
      if (this.#clientAdmissionPurpose !== "worker-root") return residentStep("blocked", "actor-worker.foreign-purpose");
      const pending = this.#residentLedger.preparedAdmission(this);
      if (cell.claimed && pending === null) {
        this.#clientAdmissionPurpose = "none"; this.#workerBootstrapPhase = phase === "close-claiming" ? "close-cell-held" : "claimed";
        return residentStep("pending", "actor-worker.claim-observation", 64);
      }
      if (phase === "close-claiming" && pending === cell && !cell.claimed) { this.#workerBootstrapPhase = "close-cell-held"; return residentStep("pending", "actor-worker.unclaimed-close-observation", 64); }
      return residentStep("blocked", "actor-worker.unclaimed", 64);
    }
    if ((phase === "record-admitting" || phase === "close-record-admitting" || phase === "record-refused" || phase === "close-record-refused") && cell) {
      const result = cell.result; this.#workerBootstrapRecord = result?.record ?? null;
      if (!result && (phase === "record-refused" || phase === "close-record-refused") && this.#workerBootstrapFault === NO_RESIDENT_FAULT) { this.#workerBootstrapPhase = phase === "close-record-refused" ? "close-cell-held" : "claimed"; return residentStep("pending", "actor-worker.unused-record-refusal-observation", 64); }
      const ready = phase !== "close-record-admitting" && this.#workerBootstrapRecord !== null && result?.step.kind === "ready" && !cell.hasFailure && this.#workerBootstrapFault === NO_RESIDENT_FAULT;
      this.#workerBootstrapPhase = ready ? "installing" : "record-held";
      return residentStep(ready || phase === "close-record-admitting" ? "pending" : "rejected", "actor-worker.record-observation", 64);
    }
    if (phase === "observing" && this.#workerBootstrapRecord) {
      if (!this.#workerBootstrapRecord.matchesShell(this)) return residentStep("blocked", "actor-worker.installation", 64);
      const live = this.#workerBootstrapFault === NO_RESIDENT_FAULT && this.#workerBootstrapRecord.matchesLiveShell(this);
      this.#workerBootstrapPhase = live ? "ready" : "record-held";
      return residentStep(live ? "pending" : "rejected", "actor-worker.installation-observation", 64);
    }
    return null;
  }

  /** 🛑️ Stops this metadata admission; admitted controller records require a later whole-client witness. */
  closeWorkerBootstrapStep(grant: ResidentGrant): ResidentStep {
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-worker.close");
    if (this.#workerBootstrapPhase === "cancelled") return residentStep("complete", "actor-worker.close");
    switch (this.#workerBootstrapPhase) {
      case "empty": this.#workerBootstrapPhase = "cancelled"; return residentStep("complete", "actor-worker.unstarted-close", 64);
      case "preparing": this.#workerBootstrapPhase = "close-preparing"; break;
      case "prepare-refused": this.#workerBootstrapPhase = "close-prepare-refused"; break;
      case "claiming": this.#workerBootstrapPhase = "close-claiming"; break;
      case "record-admitting": this.#workerBootstrapPhase = "close-record-admitting"; break;
      case "record-refused": this.#workerBootstrapPhase = "close-record-refused"; break;
      case "cell-held": case "claim-refused": case "claimed": this.#workerBootstrapPhase = "close-cell-held"; break;
      case "installing": case "observing": case "ready": this.#workerBootstrapPhase = "record-held"; return residentStep("pending", "actor-worker.record-retained", 64);
    }
    try {
      const recovered = this.#recoverWorkerBootstrap(); if (recovered) return recovered;
      const cell = this.#workerBootstrapCell;
      if (this.#workerBootstrapFault !== NO_RESIDENT_FAULT) {
        if (!cell) return residentStep("blocked", "actor-worker.fault-without-cell");
        if (!cell.hasFailure) return cell.retainFailure(this.#workerBootstrapFault, grant);
        if (!Object.is(cell.failure, this.#workerBootstrapFault)) return residentStep("blocked", "actor-worker.distinct-fault");
      }
      if (this.#workerBootstrapRecord || cell?.result || this.#workerBootstrapPhase === "record-held") return residentStep("blocked", "actor-worker.record-retained");
      if (!cell) return residentStep("blocked", "actor-worker.cell-proof");
      if (this.#workerBootstrapPhase === "close-cell-held") { this.#workerBootstrapPhase = "cell-closing"; cell.beginClose(); return residentStep("pending", "actor-worker.cell-begin-close", 64); }
      if (this.#workerBootstrapPhase === "pending-release-observing") {
        if (this.#residentLedger.preparedAdmission(this) === cell) return residentStep("blocked", "actor-worker.pending-release-proof", 64);
        if (cell.hasFailure) { if (this.#clientAdmissionPurpose === "worker-root") this.#clientAdmissionPurpose = "none"; this.#workerBootstrapPhase = "fault-held"; }
        else this.#workerBootstrapPhase = "cell-closing";
        return residentStep("pending", "actor-worker.pending-release-observation", 64);
      }
      if (this.#workerBootstrapPhase === "cell-observing") {
        if (cell.hasFailure || this.#workerBootstrapFault !== NO_RESIDENT_FAULT || !OwnedResidentRetirement.matches(cell.retirement, cell) || !cell.terminalIsEmpty()) return residentStep("blocked", "actor-worker.cell-retirement", 64);
        this.#workerBootstrapCell = null; if (this.#clientAdmissionPurpose === "worker-root") this.#clientAdmissionPurpose = "none"; this.#workerBootstrapPhase = "cancelled";
        return residentStep("complete", "actor-worker.cell-unlink-observation", 64);
      }
      if (this.#workerBootstrapPhase === "close-attempted" || this.#workerBootstrapPhase === "fault-held") return residentStep("blocked", "actor-worker.close-handoff");
      if (this.#workerBootstrapPhase === "cell-closing") {
        this.#workerBootstrapPhase = "close-attempted"; const current = cell.closeStep(grant);
        this.#workerBootstrapPhase = current.kind === "complete" ? "cell-observing" : current.kind === "pending" && current.phase === "resident-admission-bootstrap-release" ? "pending-release-observing" : "cell-closing";
        return residentChild(current, grant);
      }
      return residentStep("blocked", "actor-worker.close-phase");
    } catch (error) { this.#captureWorkerBootstrapFault(error); return residentStep("rejected", "actor-worker.close-fault"); }
  }

  prepareUiResidentPool(ledger: OwnedResidentLedger, grant: ResidentGrant): ResidentStep {
    if (ledger !== this.#residentLedger) return residentStep("rejected", "actor-resident.foreign-ledger");
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-resident.pool-prepare");
    if (this.#uiResidentClosing || this.#uiResidentPool) return residentStep("rejected", "actor-resident.pool-owned");
    try {
      const recovered = this.#recoverUiResidentPool(); if (recovered) return recovered;
      const shared = this.#prepareSharedResidentController(grant); if (shared) return shared;
      if (this.#uiResidentFault !== NO_RESIDENT_FAULT || this.#uiResidentCell?.hasFailure) return residentStep("rejected", "actor-resident.pool-fault-retirement");
      if (this.#uiResidentPhase === "prepared") return residentStep("ready", "actor-resident.pool-prepare");
      if (this.#uiResidentPhase === "empty") {
        if (this.#clientAdmissionPurpose !== "none") return residentStep("blocked", "actor-resident.foreign-purpose");
        if (!residentGrant(grant, 296)) return residentStep("blocked", "actor-resident.pool-bootstrap");
        this.#clientAdmissionPurpose = "ui-pool";
        this.#uiResidentPhase = "preparing"; const current = ledger.prepareAdmission(this, "data", grant);
        if (current.kind === "blocked" || current.kind === "rejected") this.#uiResidentPhase = "prepare-refused"; return residentChild(current, grant);
      }
      const cell = this.#uiResidentCell; if (!cell) return residentStep("rejected", "actor-resident.pool-cell");
      if (this.#uiResidentPhase === "cell-held") {
        if (this.#clientAdmissionPurpose !== "ui-pool") return residentStep("blocked", "actor-resident.foreign-purpose");
        this.#uiResidentPhase = "claiming"; const current = ledger.claimAdmission(this, cell, grant);
        if (current.kind === "blocked") this.#uiResidentPhase = "cell-held"; return residentChild(current, grant);
      }
      if (this.#uiResidentPhase === "claimed") {
        if (!residentGrant(grant, 264)) return residentStep("blocked", "actor-resident.pool-record");
        this.#uiResidentPhase = "record-admitting"; const admitted = ledger.reserveRecord("data", poolRecordEnvelope, cell, grant);
        if (admitted.step.kind === "blocked") this.#uiResidentPhase = "claimed"; return residentChild(admitted.step, grant);
      }
      return residentStep("rejected", "actor-resident.pool-owned");
    } catch (error) { this.captureUiResidentPoolFault(error); return residentStep("rejected", "actor-resident.pool-prepare-fault"); }
  }

  captureUiResidentPoolFault(error: unknown): void {
    if (this.#uiResidentFault === NO_RESIDENT_FAULT) this.#uiResidentFault = error;
    else if (!Object.is(this.#uiResidentFault, error)) throw error;
  }

  #prepareSharedResidentController(grant: ResidentGrant): ResidentStep | null {
    const recovered = this.#recoverUiResidentController(); if (recovered) return recovered;
    if (this.#uiResidentFault !== NO_RESIDENT_FAULT || this.#uiResidentControllerCell?.hasFailure) return residentStep("rejected", "actor-resident.controller-fault-held");
    if (this.#uiResidentPhase === "retired" && !this.#uiResidentControllerCell && !this.#uiResidentControllerRecord) { this.#uiResidentPhase = "controller-empty"; return residentStep("pending", "actor-resident.shared-unstarted-observation", 64); }
    const prepared = this.#prepareUiResidentController(grant); if (prepared) return prepared;
    return this.#uiResidentControllerRecord?.matchesLiveShell(this) ? null : residentStep("rejected", "actor-resident.controller-not-live");
  }

  #recoverUiResidentController(): ResidentStep | null {
    if (this.#uiResidentPhase === "controller-preparing" || this.#uiResidentPhase === "controller-prepare-refused") {
      const cell = this.#residentLedger.preparedAdmission(this);
      if (!cell) {
        if (this.#uiResidentPhase !== "controller-prepare-refused" || this.#uiResidentFault !== NO_RESIDENT_FAULT) return residentStep("blocked", "actor-resident.controller-admission-handoff");
        this.#uiResidentPhase = "controller-empty"; return residentStep("pending", "actor-resident.controller-empty-admission", 64);
      }
      this.#uiResidentControllerCell = cell; this.#uiResidentPhase = "controller-cell-held"; return residentStep("pending", "actor-resident.controller-cell-observation", 64);
    }
    if (this.#uiResidentPhase === "controller-claiming" && this.#uiResidentControllerCell) {
      if (!this.#uiResidentControllerCell.claimed || this.#residentLedger.preparedAdmission(this) !== null) return residentStep("blocked", "actor-resident.controller-unclaimed", 64);
      this.#uiResidentPhase = "controller-claimed"; return residentStep("pending", "actor-resident.controller-claim-observation", 64);
    }
    if (this.#uiResidentPhase === "controller-record-admitting" && this.#uiResidentControllerCell) {
      const result = this.#uiResidentControllerCell.result; this.#uiResidentControllerRecord = result?.record ?? null;
      const ready = this.#uiResidentControllerRecord !== null && result?.step.kind === "ready" && !this.#uiResidentControllerCell.hasFailure && this.#uiResidentFault === NO_RESIDENT_FAULT;
      this.#uiResidentPhase = ready ? "controller-installing" : "controller-rejected"; return residentStep(ready ? "pending" : "rejected", "actor-resident.controller-record-observation", 64);
    }
    if (this.#uiResidentPhase === "controller-observing" && this.#uiResidentControllerRecord) {
      if (!this.#uiResidentControllerRecord.matchesShell(this)) return residentStep("blocked", "actor-resident.controller-installation", 64);
      if (this.#uiResidentFault !== NO_RESIDENT_FAULT || !this.#uiResidentControllerRecord.matchesLiveShell(this)) { this.#uiResidentPhase = "controller-rejected"; return residentStep("rejected", "actor-resident.controller-not-live", 64); }
      this.#uiResidentPhase = "empty"; return residentStep("pending", "actor-resident.controller-installation", 64);
    }
    return null;
  }

  #prepareUiResidentController(grant: ResidentGrant): ResidentStep | null {
    if (this.#uiResidentPhase === "controller-empty") {
      if (!residentGrant(grant, 296)) return residentStep("blocked", "actor-resident.controller-bootstrap");
      this.#uiResidentPhase = "controller-preparing"; const current = this.#residentLedger.prepareAdmission(this, "data", grant);
      if (current.kind === "blocked" || current.kind === "rejected") this.#uiResidentPhase = "controller-prepare-refused"; return residentChild(current, grant);
    }
    const cell = this.#uiResidentControllerCell;
    if (this.#uiResidentPhase === "controller-cell-held" && cell) {
      this.#uiResidentPhase = "controller-claiming"; const current = this.#residentLedger.claimAdmission(this, cell, grant);
      if (current.kind === "blocked") this.#uiResidentPhase = "controller-cell-held"; return residentChild(current, grant);
    }
    if (this.#uiResidentPhase === "controller-claimed" && cell) {
      if (!residentGrant(grant, 264)) return residentStep("blocked", "actor-resident.controller-record");
      this.#uiResidentPhase = "controller-record-admitting"; const admitted = this.#residentLedger.reserveRecord("data", poolControllerEnvelope, cell, grant);
      if (admitted.step.kind === "blocked") this.#uiResidentPhase = "controller-claimed"; return residentChild(admitted.step, grant);
    }
    if (this.#uiResidentPhase === "controller-installing" && this.#uiResidentControllerRecord) {
      this.#uiResidentPhase = "controller-observing"; return residentChild(this.#uiResidentControllerRecord.install(this, grant), grant);
    }
    return this.#uiResidentPhase === "controller-rejected" ? residentStep("rejected", "actor-resident.controller-admission") : null;
  }

  #recoverUiResidentPool(): ResidentStep | null {
    if (!this.#uiResidentCell && (this.#uiResidentPhase === "preparing" || this.#uiResidentPhase === "prepare-refused")) {
      if (this.#clientAdmissionPurpose !== "ui-pool") return residentStep("blocked", "actor-resident.foreign-purpose");
      const cell = this.#residentLedger.preparedAdmission(this);
      if (!cell) {
        if (this.#uiResidentPhase !== "prepare-refused" || this.#uiResidentFault !== NO_RESIDENT_FAULT) return residentStep("blocked", "actor-resident.pool-admission-handoff");
        this.#clientAdmissionPurpose = "none";
        this.#uiResidentPhase = "empty"; return residentStep("pending", "actor-resident.pool-empty-admission", 64);
      }
      this.#uiResidentCell = cell; this.#uiResidentPhase = "cell-held"; return residentStep("pending", "actor-resident.pool-cell-observation", 64);
    }
    if (this.#uiResidentPhase === "claiming" && this.#uiResidentCell) {
      if (this.#clientAdmissionPurpose !== "ui-pool" || !this.#uiResidentCell.claimed || this.#residentLedger.preparedAdmission(this) !== null) return residentStep("blocked", "actor-resident.pool-unclaimed", 64);
      this.#clientAdmissionPurpose = "none"; this.#uiResidentPhase = "claimed"; return residentStep("pending", "actor-resident.pool-claim-observation", 64);
    }
    if (this.#uiResidentCell && this.#uiResidentPhase === "record-admitting") {
      const result = this.#uiResidentCell.result; this.#uiResidentRecord = result?.record ?? null;
      const ready = this.#uiResidentRecord !== null && result?.step.kind === "ready" && !this.#uiResidentCell.hasFailure && this.#uiResidentFault === NO_RESIDENT_FAULT;
      this.#uiResidentPhase = ready ? "prepared" : "rejected"; return residentStep(ready ? "pending" : "rejected", "actor-resident.pool-record-observation", 64);
    }
    return null;
  }

  #handoffUiResidentFault(grant: ResidentGrant): ResidentStep | null {
    if (this.#uiResidentFault === NO_RESIDENT_FAULT) return null; const cell = this.#uiResidentCell ?? this.#uiResidentControllerCell;
    if (!cell) return residentStep("blocked", "actor-resident.pool-fault-retirement");
    if (cell.hasFailure) return Object.is(cell.failure, this.#uiResidentFault) ? null : residentStep("blocked", "actor-resident.pool-distinct-fault");
    return cell.retainFailure(this.#uiResidentFault, grant);
  }

  #closeUiResidentAdmission(grant: ResidentGrant): ResidentStep {
    const cell = this.#uiResidentCell; const record = this.#uiResidentRecord;
    if (!cell) return residentStep("blocked", "actor-resident.pool-cell-proof");
    if (this.#uiResidentPhase === "record-observing") {
      if (!record || !OwnedResidentRetirement.matches(record.retirement, record)) return residentStep("blocked", "actor-resident.pool-record-proof", 64);
      cell.beginClose(); this.#uiResidentPhase = "cell-closing"; return residentStep("pending", "actor-resident.pool-record-observation", 64);
    }
    if (this.#uiResidentPhase === "cell-closing") {
      const current = cell.closeStep(grant); if (current.kind === "complete") this.#uiResidentPhase = "cell-observing"; return residentChild(current, grant);
    }
    if (this.#uiResidentPhase === "cell-observing") {
      if (this.#uiResidentFault !== NO_RESIDENT_FAULT || !OwnedResidentRetirement.matches(cell.retirement, cell) || !cell.terminalIsEmpty() || record !== null && (!OwnedResidentRetirement.matches(record.retirement, record) || !record.terminalIsEmpty())) return residentStep("blocked", "actor-resident.pool-cell-proof", 64);
      this.#uiResidentRecord = null; this.#uiResidentCell = null; this.#uiResidentPool = null; this.#uiResidentWitness = null; if (this.#clientAdmissionPurpose === "ui-pool") this.#clientAdmissionPurpose = "none"; this.#uiResidentPhase = "retired"; return residentStep("complete", "actor-resident.pool-release", 64);
    }
    return residentStep("blocked", "actor-resident.pool-cell-phase");
  }

  ownsUiResidentPool(pool: unknown): boolean { return pool !== null && this.#uiResidentPool === pool; }

  closeUiResidentPoolStep(grant: ResidentGrant): ResidentStep {
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-resident.pool-parent-close");
    if (this.#uiResidentPhase === "retired") return residentStep("complete", "actor-resident.pool-parent-close");
    this.#uiResidentClosing = true;
    try {
      const controller = this.#recoverUiResidentController(); if (controller) return controller;
      const recovered = this.#recoverUiResidentPool(); if (recovered) return recovered;
      const handoff = this.#handoffUiResidentFault(grant); if (handoff) return handoff;
      if (this.#uiResidentControllerCell?.hasFailure) {
        if (!this.#uiResidentControllerRecord && !this.#uiResidentControllerCell.claimed) { this.#uiResidentControllerCell.beginClose(); return this.#uiResidentControllerCell.closeStep(grant); }
        return residentStep("rejected", "actor-resident.controller-fault-held");
      }
      if (this.#uiResidentPhase === "controller-empty" && !this.#uiResidentControllerCell) { this.#uiResidentPhase = "retired"; return residentStep("complete", "actor-resident.pool-unstarted-close", 64); }
      const prepared = this.#prepareUiResidentController(grant); if (prepared) return prepared;
      const cell = this.#uiResidentCell; const record = this.#uiResidentRecord; const pool = this.#uiResidentPool;
      if (!cell) {
        if (this.#uiResidentPhase !== "empty") return residentStep("blocked", "actor-resident.pool-admission-handoff");
        this.#uiResidentPhase = "retired"; return residentStep("complete", "actor-resident.pool-parent-close", 64);
      }
      if (this.#uiResidentPhase === "record-observing" || this.#uiResidentPhase === "cell-closing" || this.#uiResidentPhase === "cell-observing") return this.#closeUiResidentAdmission(grant);
      if (!pool) {
        if (!record) { cell.beginClose(); this.#uiResidentPhase = "cell-closing"; return residentStep("pending", "actor-resident.pool-unused-cell-close", 64); }
        if (this.#uiResidentPhase === "prepared" || this.#uiResidentPhase === "rejected") { this.#uiResidentPhase = "unused-closing"; record.beginClose(); return residentStep("pending", "actor-resident.pool-unused-close", 64); }
        if (this.#uiResidentPhase === "unused-closing") { const current = record.closeStep(grant); if (current.kind === "complete") this.#uiResidentPhase = "record-observing"; return residentChild(current, grant); }
        return residentStep("blocked", "actor-resident.pool-unused-proof");
      }
      if (this.#uiResidentPhase === "owned") { this.#uiResidentPhase = "pool-closing"; pool.beginClose(); return residentStep("pending", "actor-resident.pool-begin-close", 64); }
      if (this.#uiResidentPhase === "pool-closing") {
        const current = pool.closeStep(grant); const forwarded = residentChild(current, grant);
        if (current.kind === "complete" && forwarded.kind === "pending") this.#uiResidentPhase = "pool-observing"; return forwarded;
      }
      if (this.#uiResidentPhase === "pool-observing") {
        const witness = pool.retirement; if (!OwnedUiResidentPoolRetirement.matches(witness, pool, this, this.#residentLedger)) return residentStep("blocked", "actor-resident.pool-private-proof", 64);
        this.#uiResidentWitness = witness; this.#uiResidentPhase = "pool-proved"; return residentStep("pending", "actor-resident.pool-observation", 64);
      }
      return this.releaseUiResidentPool(pool, this.#uiResidentWitness, grant);
    } catch (error) { this.captureUiResidentPoolFault(error); return residentStep("rejected", "actor-resident.pool-parent-fault"); }
  }

  installUiResidentPool(pool: OwnedUiResidentPool, grant: ResidentGrant): ResidentStep {
    const record = this.#uiResidentRecord;
    if (!record || (this.#uiResidentPhase !== "prepared" && this.#uiResidentPhase !== "owned") || !OwnedUiResidentPool.matchesComposition(pool, this, this.#residentLedger) || this.#uiResidentPool !== null && this.#uiResidentPool !== pool) return residentStep("rejected", "actor-resident.pool-install");
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-resident.pool-install");
    this.#uiResidentPool = pool; this.#uiResidentPhase = "owned";
    try {
      if (record.matchesShell(pool)) return residentStep("ready", "actor-resident.pool-installed");
      if (this.#uiResidentFault !== NO_RESIDENT_FAULT || this.#uiResidentCell?.hasFailure) return residentStep("blocked", "actor-resident.pool-fault-retirement");
      return record.install(pool, grant);
    } catch (error) { this.captureUiResidentPoolFault(error); return residentStep("rejected", "actor-resident.pool-install-fault"); }
  }

  releaseUiResidentPool(pool: OwnedUiResidentPool, witness: unknown, grant: ResidentGrant): ResidentStep {
    const record = this.#uiResidentRecord;
    if (!record || this.#uiResidentPool !== pool || !OwnedUiResidentPoolRetirement.matches(witness, pool, this, this.#residentLedger) || this.#uiResidentWitness !== null && this.#uiResidentWitness !== witness) return residentStep("rejected", "actor-resident.pool-witness");
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-resident.pool-release");
    try {
      const handoff = this.#handoffUiResidentFault(grant); if (handoff) return handoff;
      this.#uiResidentWitness = witness;
      if (this.#uiResidentPhase === "closing" && OwnedResidentRecordDetachment.matches(record.detachment, record, pool)) { this.#uiResidentPhase = "detached"; return residentStep("pending", "actor-resident.pool-detachment", 64); }
      if (this.#uiResidentPhase === "owned" || this.#uiResidentPhase === "pool-proved") { this.#uiResidentPhase = "closing"; record.beginClose(); return residentStep("pending", "actor-resident.pool-close-record", 64); }
      if (this.#uiResidentPhase === "closing") return record.detach(pool, grant);
      if (this.#uiResidentPhase === "detached") {
        const current = record.closeStep(grant); if (current.kind === "complete") this.#uiResidentPhase = "record-observing"; return residentChild(current, grant);
      }
      return this.#closeUiResidentAdmission(grant);
    } catch (error) { this.captureUiResidentPoolFault(error); return residentStep("rejected", "actor-resident.pool-release-fault"); }
  }
  private spawnShard(index: number): ShardSlot {
    const worker = this.createWorker(index);
    const slot: ShardSlot = { index, worker, available: true, heartbeat: freshHeartbeatState(this.now()), pendingRequestIds: new Set(), actorIds: new Set() };
    worker.onmessage = (event) => this.handleMessage(slot, event.data as InboundMessage);
    worker.onerror = (error) => {
      if (this.shards[index] !== slot) return;
      const detail = describeShardWorkerError(error);
      console.error(`[DEBUG] shard ${index} worker error: ${detail}`, error);
      this.failShard(slot, new Error(`shard ${index} worker crashed: ${detail}`));
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
    this.noteLiveness(slot, this.now());
    if (message.kind === "heartbeat") {
      this.recordHeartbeat(slot, message.turnSeq, this.now());
      return;
    }
    if (message.kind === "worker-fault") {
      const detail = formatShardWorkerFault(slot.index, message);
      console.error(`[DEBUG] ${detail}`, message.stack ?? "");
      this.onActorTrap?.(message.actorId ?? "*", detail);
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
    return mintCapturedActivation(this, actorId, activation.generation, assertActive,
      async (events: readonly ShardEventEnvelope[], budget: ShardBudget, commandPage?: ShardCommandIngressPage): Promise<unknown> => {
        assertActive();
        if (activation.returned !== null) throw new Error("actor-return.already-owned");
        const owner = activation.instance;
        const requestId = this.nextRequestId();
        const result = await this.send(slot, { kind: "turn", requestId, actorId, activationGeneration: activation.generation, events, commandPage, budget }, requestId);
        if (owner) this.recordInstanceTurn(owner, result);
        try { assertActive(); } catch (error) { if (owner) owner.interruptedTurn = result; throw error; }
        return result;
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
    const owner: ShardInstanceOwner = { activation, operation, open, phase: "opening", lifetime: null, receipt: null, accepted: null, close: null, host: null, inFlight: false, failure: null, interruptedTurn: null, cancellation: null, lastPatchSequence: 0n, returnCell: null, returnRecord: null, returnPhase: "empty", returnFault: NO_RETURN_FAULT, returnCapacity: 0 };
    activation.instance = owner;
    this.instanceLifecycles.set(open.requestSequence, owner);
    return Object.freeze({
      activation: operation,
      openRequest: open,
      get lifetime() { return owner.lifetime; },
      get pendingReceipt() { return owner.receipt; },
      get interruptedTurn() { return owner.interruptedTurn; },
      get pendingReturn() { return owner.activation.returned?.instance === owner ? owner.activation.returned.facade : null; },
      reserveReturn: (maximumResponses: number, grant: ResidentGrant) => this.reserveInstanceReturn(owner, maximumResponses, grant),
      retireUnusedReturn: (grant: ResidentGrant) => this.retireUnusedInstanceReturn(owner, grant),
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

  private reserveInstanceReturn(instance: ShardInstanceOwner, maximumResponses: number, grant: ResidentGrant): ShardReturnAdmission {
    if (instance.returnPhase === "closing") return returnAdmission("rejected", "actor-return.closing");
    if (!Number.isSafeInteger(maximumResponses) || maximumResponses < 1 || maximumResponses > 0xffffffff) return returnAdmission("rejected", "actor-return.capacity");
    if (!residentGrant(grant, 64)) return returnAdmission("blocked", "actor-return.admission");
    if (instance.returnCapacity !== 0 && instance.returnCapacity !== maximumResponses || instance.activation.returned !== null && instance.activation.returned.instance !== instance) return returnAdmission("rejected", "actor-return.original-owner");
    let spent = 64;
    try {
      const ledger = this.#residentLedger;
      if (instance.returnPhase === "preparing") {
        const cell = ledger.preparedAdmission(instance);
        if (!cell) {
          if (instance.returnFault !== NO_RETURN_FAULT) return returnAdmission("blocked", "actor-return.cell-handoff");
          instance.returnPhase = "empty"; instance.returnCapacity = 0; return returnAdmission("pending", "actor-return.empty-admission", 64);
        }
        instance.returnCell = cell; instance.returnPhase = "cell-held"; return returnAdmission("pending", "actor-return.cell-observation", 64);
      }
      const cell = instance.returnCell;
      if (instance.returnPhase === "record-admitting" && cell) {
        const result = cell.result; instance.returnRecord = result?.record ?? null;
        const ready = instance.returnRecord !== null && result?.step.kind === "ready" && !cell.hasFailure && instance.returnFault === NO_RETURN_FAULT;
        instance.returnPhase = ready ? "record-held" : "rejected"; return returnAdmission(ready ? "pending" : "rejected", "actor-return.record-observation", 64);
      }
      if (instance.returnPhase === "installing" && instance.returnRecord) {
        if (!instance.returnRecord.matchesShell(instance)) return returnAdmission("blocked", "actor-return.parent-installation", 64);
        instance.returnPhase = "installed"; return returnAdmission("pending", "actor-return.parent-observation", 64);
      }
      if (instance.returnFault !== NO_RETURN_FAULT) {
        if (cell && !cell.hasFailure) return { step: residentChild(cell.retainFailure(instance.returnFault, grant), grant), source: null };
        return returnAdmission("rejected", "actor-return.construction-fault");
      }
      if (cell?.hasFailure || instance.returnPhase === "rejected") return returnAdmission("rejected", "actor-return.admission-refused");
      const state = instance.activation.returned;
      if ((state !== null || instance.returnPhase === "installed") && !instance.returnRecord?.matchesLiveShell(instance)) return returnAdmission("rejected", "actor-return.parent-not-live");
      if (instance.returnPhase === "published") return state?.facade && !state.failed ? { step: residentStep("ready", "actor-return.original-source"), source: state.facade } : returnAdmission("rejected", "actor-return.owner-fault");
      instance.operation.assertActive();
      if (instance.inFlight) return returnAdmission("blocked", "actor-return.request-pending");
      if (instance.returnPhase === "empty") {
        if (!residentGrant(grant, 296)) return returnAdmission("blocked", "actor-return.bootstrap");
        spent = 296;
        instance.returnCapacity = maximumResponses; instance.returnPhase = "preparing"; const current = ledger.prepareAdmission(instance, "data", grant);
        if (current.kind === "blocked") { instance.returnPhase = "empty"; instance.returnCapacity = 0; }
        return { step: residentChild(current, grant), source: null };
      }
      if (instance.returnPhase === "cell-held" && cell) {
        instance.returnPhase = "claiming"; const current = ledger.claimAdmission(instance, cell, grant);
        if (current.kind === "blocked") instance.returnPhase = "cell-held"; return { step: residentChild(current, grant), source: null };
      }
      if (instance.returnPhase === "claiming" && cell) {
        if (!cell.claimed) return returnAdmission("rejected", "actor-return.unclaimed");
        instance.returnPhase = "claimed"; return returnAdmission("pending", "actor-return.claim-observation", 64);
      }
      if (instance.returnPhase === "claimed" && cell) {
        if (!residentGrant(grant, 264)) return returnAdmission("blocked", "actor-return.record");
        spent = 264;
        instance.returnPhase = "record-admitting"; const admitted = ledger.reserveRecord("data", returnDomainEnvelope, cell, grant);
        if (admitted.step.kind === "blocked") instance.returnPhase = "claimed"; return { step: residentChild(admitted.step, grant), source: null };
      }
      if (instance.returnPhase === "record-held" && instance.returnRecord) {
        instance.returnPhase = "installing"; const current = instance.returnRecord.install(instance, grant);
        if (current.kind === "blocked") instance.returnPhase = "record-held"; return { step: residentChild(current, grant), source: null };
      }
      if (instance.returnPhase === "installed") {
        if (!residentGrant(grant, 320)) return returnAdmission("blocked", "actor-return.state");
        spent = 320;
        const created: CapturedReturn = { instance, outputs: null, client: this, facade: null, origin: null, identity: null, events: null, latest: null, page: null, content: null, inFlight: false, retry: false, failed: false, fault: NO_RETURN_FAULT, cancelled: false, retired: false };
        instance.activation.returned = created; instance.returnPhase = "state-held"; Object.seal(created); return returnAdmission("pending", "actor-return.state", 320);
      }
      if (instance.returnPhase === "state-held" && state) {
        if (!residentGrant(grant, 256)) return returnAdmission("blocked", "actor-return.roster");
        spent = 256;
        state.outputs = new OwnedActorTurnOutputs(instance, instance.returnCapacity, ledger); instance.returnPhase = "roster-held"; Object.freeze(state.outputs); return returnAdmission("pending", "actor-return.roster", 256);
      }
      if (instance.returnPhase === "roster-held" && state) {
        if (!residentGrant(grant, 80)) return returnAdmission("blocked", "actor-return.facade");
        spent = 80;
        instance.returnPhase = "facade-held"; mintCapturedReturn(state); return returnAdmission("pending", "actor-return.facade", 80);
      }
      if (instance.returnPhase === "facade-held" && state?.facade && state.outputs && instance.returnRecord?.matchesShell(instance)) {
        instance.returnPhase = "published"; return { step: residentStep("ready", "actor-return.publication", 64), source: state.facade };
      }
      return returnAdmission("rejected", "actor-return.admission-phase");
    } catch (error) {
      if (instance.returnFault !== NO_RETURN_FAULT && !Object.is(instance.returnFault, error)) throw error;
      instance.returnFault = error; const state = instance.activation.returned;
      if (state?.instance === instance) { state.failed = true; state.fault = error; }
      return returnAdmission("rejected", "actor-return.construction-fault", spent);
    }
  }

  /** 🧺️ Only a never-executed original return can retire without a guest return/page/content discharge. */
  private retireUnusedInstanceReturn(instance: ShardInstanceOwner, grant: ResidentGrant): ResidentStep {
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-return.close-grant");
    const state = instance.activation.returned;
    if (state && state.instance !== instance && !(state.instance === null && instance.returnPhase === "closing")) return residentStep("rejected", "actor-return.foreign-owner");
    if (instance.inFlight || state?.inFlight) return residentStep("blocked", "actor-return.request-pending");
    if (instance.returnFault !== NO_RETURN_FAULT || state && (state.origin !== null || state.identity !== null || state.events !== null || state.page !== null || state.content !== null || state.retry || state.failed || state.fault !== NO_RETURN_FAULT)) return residentStep("blocked", "actor-return.domain-discharge-required");
    if (instance.returnPhase === "empty" && state === null && instance.returnCell === null && instance.returnRecord === null && instance.returnCapacity === 0) return residentStep("complete", "actor-return.unused-retired");
    if (!instance.returnCell) {
      const cell = this.#residentLedger.preparedAdmission(instance);
      if (!cell) return residentStep("blocked", "actor-return.cell-handoff");
      instance.returnCell = cell; instance.returnPhase = "closing";
      return residentStep("pending", "actor-return.cell-observation", 64);
    }
    instance.returnPhase = "closing";
    const cell = instance.returnCell;
    if (cell.hasFailure) return residentStep("blocked", "actor-return.admission-fault");
    if (state?.outputs) {
      if (!state.outputs.terminalIsEmpty()) { state.outputs.beginClose(); return residentChild(state.outputs.closeStep(grant), grant); }
      state.outputs = null; state.latest = null;
      return residentStep("pending", "actor-return.roster-detachment", 64);
    }
    if (state?.latest) return residentStep("blocked", "actor-return.response-held");
    if (state && (state.instance !== null || state.client !== null || state.facade !== null)) {
      if (!residentGrant(grant, 128)) return residentStep("blocked", "actor-return.state-detachment");
      state.instance = null; state.client = null; state.facade = null;
      return residentStep("pending", "actor-return.state-detachment", 128);
    }
    const record = instance.returnRecord ?? cell.result?.record ?? null;
    if (record && instance.returnRecord !== record) { instance.returnRecord = record; return residentStep("pending", "actor-return.record-observation", 64); }
    if (record?.matchesShell(instance)) { record.beginClose(); return record.detach(instance, grant); }
    if (!cell.terminalIsEmpty()) { cell.beginClose(); return residentChild(cell.closeStep(grant), grant); }
    if (!OwnedResidentRetirement.matches(cell.retirement, cell) || record && (!record.terminalIsEmpty() || !OwnedResidentRetirement.matches(record.retirement, record) || record.detachment !== null && !OwnedResidentRecordDetachment.matches(record.detachment, record, instance))) return residentStep("blocked", "actor-return.retirement-proof");
    if (!residentGrant(grant, 128)) return residentStep("blocked", "actor-return.parent-detachment");
    if (instance.activation.returned !== state) return residentStep("rejected", "actor-return.replaced-owner");
    instance.activation.returned = null; instance.returnCell = null; instance.returnRecord = null; instance.returnCapacity = 0; instance.returnPhase = "empty";
    return residentStep("complete", "actor-return.unused-retired", 128);
  }

  private reserveReturnResponse(state: CapturedReturn, grant: ResidentGrant): ResidentStep {
    const instance = state.instance;
    if (!instance) return residentStep("rejected", "actor-return.closed");
    const activation = instance.activation; const slot = activation.slot;
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-return.response-grant");
    if (!activation.available || !slot.available || this.shards[slot.index] !== slot) return residentStep("rejected", "actor-return.worker-lost");
    if (state.inFlight || instance.inFlight) return residentStep("blocked", "actor-return.request-pending");
    if (instance.returnPhase !== "published" || !state.outputs || !instance.returnRecord?.matchesLiveShell(instance)) return residentStep("rejected", "actor-return.parent-not-live");
    if (OwnedActorTurnOutput.reserved(state.latest, instance)) return residentStep("ready", "actor-return.response-ready");
    try {
      const current = state.outputs.reserve(grant);
      if (current.step.kind === "ready" && current.output) state.latest = current.output;
      if (current.step.kind === "rejected") state.failed = true;
      return current.step;
    } catch (error) {
      state.failed = true; if (state.fault === NO_RETURN_FAULT) state.fault = error; throw error;
    }
  }

  private async sendCapturedReturn(state: CapturedReturn, work: CapturedReturnWork, budget: ShardBudget): Promise<ShardReturnReport> {
    const instance = state.instance;
    if (!instance) throw new Error("actor-return.closed");
    const activation = instance.activation; const slot = activation.slot;
    if (!activation.available || !slot.available || this.shards[slot.index] !== slot) throw new Error("actor-return.worker-lost");
    if (state.inFlight || instance.inFlight) throw new Error("actor-return.request-pending");
    if (state.failed && work.kind !== "cancel") throw new Error("actor-return.owner-fault");
    if (instance.returnPhase !== "published" || state.outputs === null) throw new Error("actor-return.construction-pending");
    if (work.kind === "execute" && state.origin !== null) throw new Error("actor-return.execute-already-owned");
    const execution = work.kind === "execute" || work.kind === "retry";
    if (execution) {
      instance.operation.assertActive();
      if (work.kind === "retry" && !state.retry) throw new Error("actor-return.retry-not-admitted");
    } else if (state.identity === null) throw new Error("actor-return.identity-pending");
    const output = state.latest;
    if (!OwnedActorTurnOutput.reserved(output, instance)) throw new Error("actor-return.response-admission-required");
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
    if (!state.instance) throw new Error("actor-return.closed");
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
   * ever sends UP is `🟨️.js`'s `effect-request` (🧪️ terra-web-bridges); any other `Envelope`
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
  /** 🫀️ Every inbound message is proof this worker's event loop is running — a start-of-request
   * heartbeat, one of the generated worker's `module-fetch`/`module-ready`/`actor-ready`/`progress`
   * heartbeats, a `result`, a `trap`, an effect `frame`. {@link evaluateShardLiveness} reads exactly
   * this clock, so a long turn that keeps ticking is never mistaken for a wedged one. */
  private noteLiveness(slot: ShardSlot, atMs: number): void {
    slot.heartbeat.lastLivenessAtMs = atMs;
    slot.heartbeat.missedCount = 0;
    slot.heartbeat.lastMissCountedAtMs = atMs;
  }

  private recordHeartbeat(slot: ShardSlot, turnSeq: number, atMs: number): void {
    slot.heartbeat.lastHeartbeatAtMs = atMs;
    slot.heartbeat.lastHeartbeatTurnSeq = turnSeq;
    this.noteLiveness(slot, atMs);
  }

  /** 🔭️ SAB path: polls every shard's `Atomics.load` slot and folds any advance into the SAME state
   * machine `postMessage` heartbeats update — call alongside `checkHeartbeats` on a scheduler tick.
   * A no-op when this client was built without a `heartbeatSab` (postMessage is the only source then;
   * see this class's header doc — correctness never depends on this method being called at all). */
  pollHeartbeatSab(nowMs: number = this.now()): void {
    if (!this.heartbeatSabView) return;
    for (const slot of this.shards) {
      const seq = Atomics.load(this.heartbeatSabView, slot.index);
      /** 🫀️ Only a MONOTONIC ADVANCE is proof of life. `!==` used to accept a REGRESSION too, which a
       * worker that never processed `attachHeartbeatSab` produces on every tick (its slot stays `0`
       * while `postMessage` beats advance `lastHeartbeatTurnSeq`) — under the liveness rule that would
       * refresh a dead shard's clock forever and disarm the watchdog entirely. */
      if (seq > slot.heartbeat.lastHeartbeatTurnSeq || slot.heartbeat.oldestPendingStartedAtMs === null) {
        this.recordHeartbeat(slot, seq, nowMs);
      }
    }
  }

  /** 🚑️ Applies {@link evaluateShardLiveness} — the whole decision, including what "silence" means —
   * to every shard and executes the one side effect it can ask for. `SHARD_LIVENESS_POLICY.missedLimit`
   * consecutive timeout windows of real silence (not that many calls to this method) trigger
   * `terminate()` + `rebuild()` and `onShardLost`. */
  checkHeartbeats(nowMs: number = this.now()): void {
    for (const slot of this.shards) {
      const decision = evaluateShardLiveness({
        nowMs,
        oldestPendingStartedAtMs: slot.heartbeat.oldestPendingStartedAtMs,
        lastLivenessAtMs: slot.heartbeat.lastLivenessAtMs,
        missedCount: slot.heartbeat.missedCount,
        lastMissCountedAtMs: slot.heartbeat.lastMissCountedAtMs,
        heartbeatTimeoutMs: this.heartbeatTimeoutMs,
      });
      slot.heartbeat.missedCount = decision.missedCount;
      slot.heartbeat.lastMissCountedAtMs = decision.lastMissCountedAtMs;
      if (!decision.terminate) continue;
      const actorIds = [...slot.actorIds];
      this.terminate(slot.index);
      this.rebuild(slot.index);
      this.onShardLost?.(slot.index, actorIds);
    }
  }

  /** ▶️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T-P4): self-ticks {@link pollHeartbeatSab} +
   * {@link checkHeartbeats} on a real interval. Before this method existed, NEITHER was ever called
   * outside this file's own tests (verified with `grep -rn "checkHeartbeats(\|pollHeartbeatSab("` across
   * the repo) — the watchdog's whole failure ladder was wired but nothing in production ever turned
   * the crank, so a wedged shard went undetected forever in the real app. Idempotent: calling this
   * again while already running is a no-op (use {@link stopWatchdog} first to change the interval).
   * Uses the real `setInterval`/`clearInterval` — same convention as this repo's own
   * `ActivationRegistry.startRuntimeMetricsPublisher` (kernel `🟦️.ts`), which tests with
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
  const { registerTests1 } = await import("./🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts");
  await registerTests1(import.meta.vitest, { ACTOR_BYTE_PAGE_BYTES, MAINTENANCE_LANE_DEFAULT_BUDGET, MAX_SEGMENTED_DOWNLOAD_OPERATION_ID, NO_RESIDENT_FAULT, OwnedActorTurnOutput, OwnedActorTurnOutputs, OwnedKernelReturnContent, OwnedNativeUiPatchAuthority, OwnedNativeUiPatchSubmissionReceipt, OwnedResidentLedger, OwnedResidentRetirement, OwnedShardReturn, OwnedShardReturnPage, OwnedUiInstance, OwnedUiInstanceRetirement, OwnedUiPatchAcknowledgement, OwnedUiPatchInputRetirement, OwnedUiResidentPool, SHARD_FRAME_VARIANT_FIELDS, SHARD_JSPI_FAULT_CODE, SHARD_LIVENESS_POLICY, ShardClient, ShardJspiUnavailableError, assertShardJspiAvailable, capturedReturnState, createActorBytePage, createGrantedBudgetTracker, createShardCommandIngressPages, describeShardWorkerError, encodeActorInstanceLifecycle, encodeActorUiPatchReceipt, interpretShardFrame, isShardLostError, orderEnvelopesByLane, poolControllerEnvelope, poolUiEnvelope, shardJspiAvailable, uiResidentMetadataEnvelope }, { directory: (await import("node:url")).fileURLToPath(new URL(".", import.meta.url)), url: import.meta.url });
}
//#endregion 🧪️Tests
