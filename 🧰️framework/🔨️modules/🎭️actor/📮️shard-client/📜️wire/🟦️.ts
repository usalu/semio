import { type ActorBytePage, ACTOR_BYTE_PAGE_BYTES, createActorBytePage } from "../../📃️page/🟦️.ts";
import { GUEST_HOST_ANSWER_CEILING_BYTES } from "../../../⏱️trace/🧮️memory/🟦️.ts";
import type { Lane, CoalesceKey } from "../../🤖️generated/🎭️actor/🟦️.ts";

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
  readonly maxFrames: number;
}

/** ⚖️ Stand-in for `semio_framework_actor::JobBudget` (design-abi.md `jobs::job-budget`). `fuel` is
 * a WIT `u64` and therefore a `bigint` — the worker hands this record straight to the guest's
 * `step-job`, whose `u64` lowering rejects a `number`. */
export interface ShardJobBudget {
  readonly fuel: bigint;
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

export type ShardCommandIngressPage = { readonly cursor: ShardCommandPageCursor; readonly bytes: Uint8Array; readonly page: ActorBytePage };

/** 📄️ Pages one command may cross in, derived — never chosen — from the same budget
 * `COMMAND_MAXIMUM_PAGES` in `📡️spr/🧵️channel/🦀️.rs` reads: the assembled host-answer ceiling over
 * the page extent. The guest reassembles a command page by page, each page its own 4 KiB block, so
 * the only bound left is what a guest may be handed for ONE outstanding request. A 64-page ceiling
 * (262 144 B) refused a 272 089-char contributions pack outright — ticket
 * 26/09/09/PROCEDURAL-3D-END-TO-END. */
export const SHARD_COMMAND_MAXIMUM_PAGES = GUEST_HOST_ANSWER_CEILING_BYTES / ACTOR_BYTE_PAGE_BYTES;

/** 📥️ Splits channel command bytes into the `reactor.stage-command-page` pages shared with Rust — a
 * cursor and the page's live bytes, never a fixed 4 KiB block record. */
export function createShardCommandIngressPages(input: {
  readonly owner: bigint;
  readonly generation: bigint;
  readonly commandIndex: number;
  readonly commandCount: number;
  readonly instance: number;
  readonly seq: bigint;
  readonly command: Uint8Array;
}): readonly ShardCommandIngressPage[] {
  if (input.command.length === 0) throw new Error("command ingress cannot encode an empty command");
  const pageCount = Math.ceil(input.command.length / ACTOR_BYTE_PAGE_BYTES);
  if (pageCount > SHARD_COMMAND_MAXIMUM_PAGES) throw new Error(`command ingress exceeds ${SHARD_COMMAND_MAXIMUM_PAGES} pages`);
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
      bytes: bytes.slice(),
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
 * emitted `🤖️generated/🎭️actor/🟦️.ts`" reason as `ShardBudget` above) — MICROKERNEL-POOLED-ACTOR-PLUGIN-
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

/** ⚖️ `semio_framework_actor::lane_defaults::budget_for(Lane::Maintenance)` (`🎭️actor/🦀️.rs`,
 * `lane_defaults` module) — the ONLY floor a granted-less actor may fall back to, mirrored field-for-
 * field/value-for-value: `{ fuel: 80_000_000, wall_ms: 200, memory_bytes: 256 MiB, ui_nodes: 4_000,
 * mailbox_len: 1024, max_effects: 512, max_patch_bytes: 2_097_152 }`. {@link GrantedBudgetTracker}
 * falls back to this so a budget-less `Envelope` arriving before any `Grant` for its actor never
 * invents its own number — the same floor `ShardLoop::granted_budget` falls back to natively. */
export const MAINTENANCE_LANE_DEFAULT_BUDGET: ShardBudget = { fuel: 80_000_000, wallMs: 200, memoryBytes: 256 * 1024 * 1024, uiNodes: 4_000, mailboxLen: 1024, maxEffects: 512, maxPatchBytes: 2_097_152, maxFrames: 8 };

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
 * `Grant` (a budget travels WITH the envelopes it grants, read by {@link ShardClient.grant}). The
 * retained-turn fields of `Grant` and the whole `MeshReceived` variant are Rust-originated: the browser
 * path never builds them and {@link interpretShardFrame} answers `MeshReceived` as `"unknown"`. */
export type ShardFrame =
  | { readonly kind: "Register"; readonly actor: string }
  | { readonly kind: "Unregister"; readonly actor: string }
  | { readonly kind: "Grant"; readonly actor: string; readonly budget: ShardBudget; readonly retained?: unknown; readonly retained_policy?: unknown; readonly work_fuel?: bigint; readonly deadline_us?: bigint; readonly envelopes: readonly ShardEnvelope[] }
  | { readonly kind: "Envelope"; readonly envelope: ShardEnvelope }
  | { readonly kind: "MeshReceived"; readonly actor: string; readonly input: unknown; readonly receiving_sequence: bigint; readonly spent: unknown; readonly work_units_spent: bigint; readonly accepted: boolean };

/** 🧬️ Runtime twin of {@link ShardFrame}'s own field names — TS union types erase at runtime, so the
 * in-source parity test reads THIS array (not the type) to diff against the live Rust source. Keep in
 * lockstep with the union above by hand; the test fails loudly the moment either drifts from
 * `component.rs`. `Envelope`'s Rust variant is a TUPLE (`Envelope(Envelope)`, no Rust field name) —
 * `"envelope"` is this mirror's OWN naming choice for that lone position, not a Rust-sourced name. */
export const SHARD_FRAME_VARIANT_FIELDS: ReadonlyArray<{ readonly kind: ShardFrame["kind"]; readonly fields: readonly string[] }> = [
  { kind: "Register", fields: ["actor"] },
  { kind: "Unregister", fields: ["actor"] },
  { kind: "Grant", fields: ["actor", "budget", "retained", "retained_policy", "work_fuel", "deadline_us", "envelopes"] },
  { kind: "Envelope", fields: ["envelope"] },
  { kind: "MeshReceived", fields: ["actor", "input", "receiving_sequence", "spent", "work_units_spent", "accepted"] },
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
