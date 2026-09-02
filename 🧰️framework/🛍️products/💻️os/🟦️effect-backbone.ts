/** 📡️ `EffectBackbone` — the TypeScript counterpart of the Rust host's per-instance
 * `BackboneRegistry` (`🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs`, `#region 📡️EffectBackbone`),
 * which replaced the deleted PROCESS-GLOBAL `set_host_backbone_channel` that left guest↔store sync
 * with NO path at all. This module is that same fix for the web host: every `EffectBackbone` is
 * constructed ONE PER PLUGIN INSTANCE (never a module-level singleton) — a process-global here would
 * repeat exactly the bug this ticket exists to remove, since pooled multi-instance actors would then
 * silently share one instance's endpoints/subscriptions with another's.
 *
 * 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-web-effect-backbone). Rust is the SSOT for the
 * wire shapes below — this file conforms to it, not the reverse (see `## Rust↔TS wire parity table`
 * in this ticket's `📓️terra-web-effect-backbone-report.md`). The in-source parity test at the bottom
 * of this file reads `🎠️kernel/🦀️.rs` fresh on every run and fails loudly the moment
 * `MessageEndpoint`/`Effect::SendMessage`/`Event::Message` drift from this file's mirror.
 *
 * 🗃️ State classification (CLAUDE.md: persisted-local-only / persisted-shared / ephemeral-local-only
 * / ephemeral-shared) — EVERY field this file owns is **ephemeral-local-only**: `endpoints`,
 * `byActor`/`byUri` subscriptions, their `BoundedMailbox` queues, and the per-uri `revisions` counter
 * all live only in this instance's memory, are never written to disk, and are rebuilt from scratch
 * on the next plugin activation. Anything **persisted** or **shared** (a document's pack/spr bytes, a
 * hub session, an offline mutation outbox) belongs entirely to `🟦️backbone-worker.ts`'s own state
 * (already classified in that file) — this module never duplicates or reclassifies it, only calls
 * through to it (`#region 🌉️WorkerTransport`).
 */

//#region 🧬️WireTypes
/** ⚖️ TypeScript mirror of Rust `MessageEndpoint` (`🎠️kernel/🦀️.rs`, `#region 🔖️Event`) —
 * who a {@link EventMessage} came from / an {@link EffectSendMessage} targets. Externally tagged,
 * camelCase (`#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]`, no `tag=`), so
 * `MessageEndpoint::Backbone { uri }` is `{ "backbone": { "uri": "..." } }` on the wire — see the
 * in-source parity test (`#region 🧪️Tests`) for the enforcement that this stays in lockstep with the
 * live Rust enum. `instance`/`id` stay plain `string` (this codebase's established actor/plugin-id
 * stand-in — same convention `🧵️shard-client.ts`'s `ShardFrame` already uses), not the generated
 * mirror's bit-packed ids. */
export type MessageEndpoint =
  | { readonly shell: { readonly instance: string } }
  | { readonly backbone: { readonly uri: string } }
  | { readonly pluginInstance: { readonly id: string } }
  | { readonly extension: { readonly id: string } }
  | { readonly topic: { readonly name: string } };

/** 🧬️ Runtime twin of {@link MessageEndpoint}'s own variant/field names — TS union types erase at
 * runtime, so the in-source parity test reads THIS array (not the type) to diff against the live
 * Rust `MessageEndpoint` enum. Keep in lockstep with the type above by hand; the test fails loudly
 * the moment either drifts from `component.rs`. */
export const MESSAGE_ENDPOINT_VARIANT_FIELDS: ReadonlyArray<{ readonly kind: string; readonly fields: readonly string[] }> = [
  { kind: "Shell", fields: ["instance"] },
  { kind: "Backbone", fields: ["uri"] },
  { kind: "PluginInstance", fields: ["id"] },
  { kind: "Extension", fields: ["id"] },
  { kind: "Topic", fields: ["name"] },
];

/** 📤️ TypeScript mirror of Rust `Effect::SendMessage { target: MessageEndpoint, payload: Vec<u8> }`
 * (`🎠️kernel/🦀️.rs`) — field-for-field, camelCase-tagged like every other `Effect` variant.
 * Declared fresh here (not imported from `🎠️kernel/🟦️.ts`, which stubs `target: unknown` and
 * is outside this packet's owned/leased paths) so this module can type `target` precisely without
 * editing a file it does not own. */
export type EffectSendMessage = { readonly sendMessage: { readonly target: MessageEndpoint; readonly payload: readonly number[] } };

/** 📥️ TypeScript mirror of Rust `Event::Message { source: MessageEndpoint, payload: Vec<u8> }`
 * (`🎠️kernel/🦀️.rs`) — the inbound counterpart of {@link EffectSendMessage}, same reasoning
 * for being declared fresh here rather than imported. */
export type EventMessage = { readonly message: { readonly source: MessageEndpoint; readonly payload: readonly number[] } };

/** 🏗️ Builds the `MessageEndpoint::Backbone { uri }` wire shape — the only endpoint kind this module
 * itself constructs (`Shell`/`PluginInstance`/`Extension`/`Topic` targets are the documented no-op
 * gap the Rust `AsyncEffectExecutor` report also flags: "no id→ActorId directory owned here"). */
export function backboneMessageEndpoint(uri: string): MessageEndpoint {
  return { backbone: { uri } };
}

function isBackboneEndpoint(endpoint: MessageEndpoint): endpoint is { readonly backbone: { readonly uri: string } } {
  return "backbone" in endpoint;
}
//#endregion 🧬️WireTypes

//#region 🔑️Capability
/** 🔑️ TypeScript twin of Rust `CapabilityChecker` (`⚡️effects/🦀️.rs`) — whether `actor`
 * currently holds a grant covering `scope` (e.g. `"messaging.backbone:<uri>"`). A seam, not a real
 * grant table, exactly like its Rust counterpart. `actor` is a plain `string` (this codebase's
 * established id stand-in), mirroring Rust's `actor: u64` in spirit, not in wire type. */
export interface CapabilityChecker {
  isGranted(actor: string, scope: string): boolean;
}

/** 🧪️ Permissive default/test double — grants everything. TS twin of Rust `AllowAllCapabilities`. */
export class AllowAllCapabilities implements CapabilityChecker {
  isGranted(): boolean {
    return true;
  }
}

/** 🔑️ `messaging.backbone:<uri>` — MUST match Rust `BackboneRegistry::send`'s
 * `format!("messaging.backbone:{uri}")` exactly; this is the one string every capability grant this
 * module checks is compared against. */
export function backboneCapabilityScope(uri: string): string {
  return `messaging.backbone:${uri}`;
}
//#endregion 🔑️Capability

//#region 🧯️Outcomes
/** 🧯️ TypeScript twin of Rust `BackboneError` (`⚡️effects/🦀️.rs`) — NOT a wire/serde type on
 * the Rust side either (it derives `PartialEq`/`Eq`/`Debug` only, no `Serialize`), so this mirror is
 * for cross-language debugging/log-message parity, not machine-diffed against the Rust source the
 * way {@link MessageEndpoint} is. */
export type BackboneError = { readonly kind: "capabilityDenied"; readonly uri: string } | { readonly kind: "noSuchEndpoint"; readonly uri: string } | { readonly kind: "transport"; readonly message: string };

/** 📤️ What {@link EffectBackbone.send} resolves to — TS twin of Rust `Result<(), BackboneError>`. */
export type BackboneSendOutcome = { readonly ok: true } | { readonly ok: false; readonly error: BackboneError };

/** 📤️ What {@link EffectBackbone.dispatchSendMessage} resolves to — the full `Effect::SendMessage`
 * routing decision, one layer above {@link BackboneSendOutcome}: a non-`Backbone` target is the
 * documented no-op gap (mirrors the Rust `AsyncEffectExecutor` report's own routing-table note), not
 * an error. */
export type BackboneDispatchOutcome = { readonly kind: "sent" } | { readonly kind: "notBackboneTarget" } | { readonly kind: "denied"; readonly error: BackboneError };

/** 📥️ What one delta-fanout / inbound-message delivery resolves to for ONE subscriber — TS twin of
 * Rust `PublishOutcome` (`🛎️services/🦀️component.rs`; also not `Serialize` there, in-process only),
 * derived from {@link Backpressure} (this module's actual delivery primitive — see `#region
 * 📡️EffectBackbone`'s header doc for why reusing {@link createBoundedMailbox} makes this a thin
 * relabeling rather than a second backpressure vocabulary). */
export type DeliveryOutcome = { readonly kind: "delivered" } | { readonly kind: "collapsed" } | { readonly kind: "rejectedFull" } | { readonly kind: "droppedLane"; readonly lane: Lane } | { readonly kind: "noSuchSubscriber" };

function deliveryOutcomeFromBackpressure(pressure: Backpressure): DeliveryOutcome {
  switch (pressure.kind) {
    case "accept":
      return { kind: "delivered" };
    case "coalesced":
      return { kind: "collapsed" };
    case "dropped":
      return { kind: "droppedLane", lane: pressure.lane };
    case "rejected":
      return { kind: "rejectedFull" };
  }
}
//#endregion 🧯️Outcomes

//#region 📊️OverflowReporting
/** 🚨️ One queue-overflow occurrence — `never silently dropped` (mission contract): every
 * `rejectedFull` {@link DeliveryOutcome} this module produces is ALSO handed to a
 * {@link BackboneOverflowReporter}, the same "reject-and-report, never drop" shape
 * `🟦️backbone-worker.ts`'s own `rejectMutationQueueOverflow` already uses for its outbound mutation
 * queue (finding 5 in that file) — this module stays consistent with that established contract rather
 * than inventing a second one. `droppedLane` is reported too: it is a cross-lane EVICTION, not a
 * silent no-signal loss (the caller always gets a {@link DeliveryOutcome} back), but it still moves
 * data out of a queue under pressure and belongs in the same audit trail. */
export type BackboneOverflowEvent = { readonly actor: string; readonly uri: string; readonly channel: "send" | "delta"; readonly outcome: DeliveryOutcome };

export interface BackboneOverflowReporter {
  reportOverflow(event: BackboneOverflowEvent): void;
}

/** 🖨️ Default reporter — logs to the console. EN: "backbone queue overflow, message rejected — never
 * silently dropped." DE: "Backbone-Warteschlange überlastet, Nachricht abgelehnt — nie stillschweigend
 * verworfen." */
export class ConsoleBackboneOverflowReporter implements BackboneOverflowReporter {
  reportOverflow(event: BackboneOverflowEvent): void {
    console.error("[effect-backbone] queue overflow, message rejected — never silently dropped / Warteschlange überlastet, Nachricht abgelehnt", event.actor, event.uri, event.channel, event.outcome);
  }
}

/** 🧪️ Test double recording every overflow report in order — TS twin of Rust `RecordingMetrics`. */
export class RecordingBackboneOverflowReporter implements BackboneOverflowReporter {
  private readonly events: BackboneOverflowEvent[] = [];
  reportOverflow(event: BackboneOverflowEvent): void {
    this.events.push(event);
  }
  recorded(): readonly BackboneOverflowEvent[] {
    return this.events;
  }
}
//#endregion 📊️OverflowReporting

//#region 📡️EffectBackbone
/** ⚖️ `Lane`/`CoalesceKey`/`BoundedMailbox` are the actor module's own wire-accurate primitives
 * (`🎭️actor/📬️mailbox/🟦️.ts`) — reused rather than reimplemented (CLAUDE.md:
 * "use as many existing libraries as possible"). That package's `package.json` only exports its `.`
 * entry (`🧵️shard-client.ts`), not a `📬️mailbox` subpath, so this file imports it by relative path —
 * the SAME "sidestep a missing subpath export rather than edit a foreign-leased config file" pattern
 * `ShellHost/🟦️.tsx` already documents for its own `🟦️backbone-worker.ts` import. */
import { createBoundedMailbox, type Backpressure, type BoundedMailbox, type CoalesceKey, type Lane } from "../../🔨️modules/🎭️actor/📬️mailbox/🟦️.ts";
export type { Backpressure, Lane };

/** 🌉️ TypeScript twin of Rust `BackboneTransport` (`⚡️effects/🦀️.rs`) — a seam, not a real
 * implementation. `#region 🌉️WorkerTransport` below provides the reference implementation that routes
 * through `🟦️backbone-worker.ts`'s existing document transport. */
export interface BackboneTransport {
  send(uri: string, payload: readonly number[]): void;
}

/** 🚦️ Single lane every backbone queue enqueues under — deliberately the ONLY lane in use, so a full
 * queue can NEVER cross-lane-evict something else (mailbox eviction only ever targets a LOWER-priority
 * nonempty lane; with exactly one lane populated there is none to evict), which is what makes overflow
 * always resolve to a clean `rejectedFull` rather than an incidental `droppedLane` — see `## overflow +
 * loss semantics` in this ticket's report. */
const BACKBONE_QUEUE_LANE: Lane = "Background";
const DEFAULT_SEND_QUEUE_CAPACITY = 256;
/** 📥️ The delta mailbox ALWAYS enqueues under the SAME {@link CoalesceKey} (the uri itself) — a
 * capacity of 1 is provably sufficient: the first delta accepts, every later one for the same uri
 * replaces it in place (`coalesced`), so the queue can never hold more than one pending delta. */
const DELTA_QUEUE_CAPACITY = 1;

export interface EffectBackboneOptions {
  readonly capabilities?: CapabilityChecker;
  readonly reporter?: BackboneOverflowReporter;
  readonly sendQueueCapacity?: number;
}

interface UriSubscription {
  readonly sendMailbox: BoundedMailbox<EventMessage>;
  readonly deltaMailbox: BoundedMailbox<EventMessage>;
}

/** 📡️ Per-PLUGIN-INSTANCE backbone bridge — construct ONE per plugin instance, never a shared/module-
 * level singleton (see this file's header doc). Mirrors Rust `BackboneRegistry` field-for-field in
 * behavior:
 * - {@link registerEndpoint} / {@link send} — outbound `Effect::SendMessage{Backbone(uri)}`,
 *   capability-gated (`messaging.backbone:<uri>`), dispatched to the registered transport. No
 *   queueing: exactly like Rust, a send either reaches the transport now or fails now.
 * - {@link subscribe} / {@link fanoutDelta} / {@link deliverMessage} / {@link drain} — inbound
 *   `Event::Message`, queued per `(actor, uri)` until the guest polls via {@link drain}. Deltas
 *   coalesce latest-wins per uri (mirrors `ChannelPolicy::Coalesced { key: uri }`); direct sends are
 *   lossless (mirrors a `LosslessBounded` policy) and reject-and-report once their queue is full,
 *   never silently drop (see `#region 📊️OverflowReporting`).
 */
export class EffectBackbone {
  private readonly endpoints = new Map<string, BackboneTransport>();
  private readonly byActor = new Map<string, Map<string, UriSubscription>>();
  private readonly byUri = new Map<string, Set<string>>();
  private readonly revisions = new Map<string, number>();
  private readonly capabilities: CapabilityChecker;
  private readonly reporter: BackboneOverflowReporter;
  private readonly sendQueueCapacity: number;

  constructor(options: EffectBackboneOptions = {}) {
    this.capabilities = options.capabilities ?? new AllowAllCapabilities();
    this.reporter = options.reporter ?? new ConsoleBackboneOverflowReporter();
    this.sendQueueCapacity = options.sendQueueCapacity ?? DEFAULT_SEND_QUEUE_CAPACITY;
  }

  //#region 🌱️Endpoints
  /** ➕️ TS twin of Rust `BackboneRegistry::register`. */
  registerEndpoint(uri: string, transport: BackboneTransport): void {
    this.endpoints.set(uri, transport);
  }

  unregisterEndpoint(uri: string): void {
    this.endpoints.delete(uri);
  }
  //#endregion 🌱️Endpoints

  //#region 📤️Outbound
  /** 📤️ TS twin of Rust `BackboneRegistry::send(actor, uri, payload)` — capability-gated
   * (`messaging.backbone:<uri>`), no queueing. */
  send(actor: string, uri: string, payload: readonly number[]): BackboneSendOutcome {
    if (!this.capabilities.isGranted(actor, backboneCapabilityScope(uri))) {
      return { ok: false, error: { kind: "capabilityDenied", uri } };
    }
    const transport = this.endpoints.get(uri);
    if (!transport) return { ok: false, error: { kind: "noSuchEndpoint", uri } };
    try {
      transport.send(uri, payload);
      return { ok: true };
    } catch (error) {
      return { ok: false, error: { kind: "transport", message: error instanceof Error ? error.message : String(error) } };
    }
  }

  /** 📤️ The full `Effect::SendMessage` routing decision — unwraps {@link MessageEndpoint} and routes
   * ONLY a `Backbone` target through {@link send}; every other target is the documented no-op gap
   * (`Shell`/`PluginInstance`/`Extension`/`Topic` — no id→ActorId directory owned by this module,
   * mirrors the Rust `AsyncEffectExecutor` report's identical gap for the SAME reason). */
  dispatchSendMessage(actor: string, effect: EffectSendMessage): BackboneDispatchOutcome {
    const target = effect.sendMessage.target;
    if (!isBackboneEndpoint(target)) return { kind: "notBackboneTarget" };
    const outcome = this.send(actor, target.backbone.uri, effect.sendMessage.payload);
    return outcome.ok ? { kind: "sent" } : { kind: "denied", error: outcome.error };
  }
  //#endregion 📤️Outbound

  //#region 📥️Inbound
  private subscription(actor: string, uri: string): UriSubscription {
    let perActor = this.byActor.get(actor);
    if (!perActor) {
      perActor = new Map();
      this.byActor.set(actor, perActor);
    }
    let entry = perActor.get(uri);
    if (!entry) {
      entry = { sendMailbox: createBoundedMailbox<EventMessage>(this.sendQueueCapacity), deltaMailbox: createBoundedMailbox<EventMessage>(DELTA_QUEUE_CAPACITY) };
      perActor.set(uri, entry);
      let subscribers = this.byUri.get(uri);
      if (!subscribers) {
        subscribers = new Set();
        this.byUri.set(uri, subscribers);
      }
      subscribers.add(actor);
    }
    return entry;
  }

  /** ➕️ Registers `actor` to receive inbound `Event::Message`s (both direct sends and coalesced
   * deltas) for `uri` — TS twin of Rust `EventRouter::subscribe(topic, actor, policy)`, with the
   * policy fixed per-channel (lossless for direct sends, `Coalesced { key: uri }` for deltas) rather
   * than caller-supplied, since this module only ever needs those two. Idempotent. */
  subscribe(actor: string, uri: string): void {
    this.subscription(actor, uri);
  }

  /** ➖️ TS twin of Rust `EventRouter::unsubscribe`. */
  unsubscribe(actor: string, uri: string): void {
    const perActor = this.byActor.get(actor);
    if (perActor?.delete(uri) && perActor.size === 0) this.byActor.delete(actor);
    const subscribers = this.byUri.get(uri);
    if (subscribers) {
      subscribers.delete(actor);
      if (subscribers.size === 0) this.byUri.delete(uri);
    }
  }

  private enqueue(actor: string, uri: string, channel: "send" | "delta", mailbox: BoundedMailbox<EventMessage>, coalesce: CoalesceKey | undefined, event: EventMessage): DeliveryOutcome {
    const pressure = mailbox.enqueue({ lane: BACKBONE_QUEUE_LANE, coalesce, payload: event });
    const outcome = deliveryOutcomeFromBackpressure(pressure);
    if (outcome.kind === "rejectedFull" || outcome.kind === "droppedLane") this.reporter.reportOverflow({ actor, uri, channel, outcome });
    return outcome;
  }

  /** 📥️ Direct, LOSSLESS inbound `Event::Message` for one subscribed actor — the "send" half of the
   * Rust doc's `{"kind":"send"|"delta",...}` wire shape (see `#region 🌉️GuestWire`). Never coalesces:
   * a full queue REJECTS and REPORTS (see `#region 📊️OverflowReporting`), it never silently drops or
   * evicts an older still-undelivered message — mutations are lossless user work (mission contract). */
  deliverMessage(actor: string, uri: string, payload: readonly number[]): DeliveryOutcome {
    const perActor = this.byActor.get(actor);
    const entry = perActor?.get(uri);
    if (!entry) return { kind: "noSuchSubscriber" };
    return this.enqueue(actor, uri, "send", entry.sendMailbox, undefined, { message: { source: backboneMessageEndpoint(uri), payload } });
  }

  /** 📥️ TS twin of Rust `BackboneRegistry::fanout_delta(uri, delta)` — fans a store-sync delta out to
   * every actor subscribed to `uri` (mirrors `ChannelPolicy::Coalesced { key: uri }` on topic
   * `backbone.delta.<uri>`): a burst of deltas for the SAME uri collapses to the latest instead of
   * queueing, because every delta enqueue for a given `(actor, uri)` uses the SAME coalesce key (the
   * uri itself) into a dedicated 1-capacity mailbox. Returns one {@link DeliveryOutcome} per
   * subscribed actor, exactly like Rust returning `Vec<(ActorId, PublishOutcome)>`. Also advances this
   * uri's monotonic revision counter (see {@link nextRevision}), for {@link encodeBackboneGuestMessage}
   * callers that need to let a guest detect a collapsed delta. */
  fanoutDelta(uri: string, payload: readonly number[]): ReadonlyMap<string, DeliveryOutcome> {
    const outcomes = new Map<string, DeliveryOutcome>();
    const subscribers = this.byUri.get(uri);
    if (!subscribers) return outcomes;
    const event: EventMessage = { message: { source: backboneMessageEndpoint(uri), payload } };
    for (const actor of subscribers) {
      const entry = this.byActor.get(actor)!.get(uri)!;
      outcomes.set(actor, this.enqueue(actor, uri, "delta", entry.deltaMailbox, uri, event));
    }
    return outcomes;
  }

  /** ⏰️ Monotonic per-uri revision counter — the TS twin of the `revision` field
   * `#region 🌉️GuestWire`'s wire shape carries so a guest can detect a collapsed (skipped) delta the
   * same way `UiPatch.base_revision` lets it detect a stale diff. Advances once per
   * {@link fanoutDelta} call for that uri, regardless of how many (or how few) subscribers it reached. */
  nextRevision(uri: string): number {
    const next = (this.revisions.get(uri) ?? 0) + 1;
    this.revisions.set(uri, next);
    return next;
  }

  /** 📮️ TS twin of Rust `EventRouter::drain(topic, actor)`, generalized across every uri `actor` is
   * subscribed to (a guest's `poll` drains its whole inbox in one call, never one topic at a time) —
   * direct sends first (higher priority than passive deltas), then deltas, per-uri in subscription
   * order. */
  drain(actor: string): readonly EventMessage[] {
    const perActor = this.byActor.get(actor);
    if (!perActor) return [];
    const drained: EventMessage[] = [];
    for (const entry of perActor.values()) {
      let envelope = entry.sendMailbox.popNext();
      while (envelope) {
        drained.push(envelope.payload);
        envelope = entry.sendMailbox.popNext();
      }
      envelope = entry.deltaMailbox.popNext();
      while (envelope) {
        drained.push(envelope.payload);
        envelope = entry.deltaMailbox.popNext();
      }
    }
    return drained;
  }
  //#endregion 📥️Inbound

  /** 🧹️ Releases every endpoint/subscription this instance owns — call once when the OWNING plugin
   * instance is torn down. Never affects any other `EffectBackbone` instance (each owns disjoint
   * `Map`/`Set` state — see this file's header doc on per-instance isolation). */
  dispose(): void {
    this.endpoints.clear();
    this.byActor.clear();
    this.byUri.clear();
    this.revisions.clear();
  }
}
//#endregion 📡️EffectBackbone

//#region 🌉️GuestWire
/** 🌉️ Host↔guest wire shape for one backbone message, EXACTLY as specified by the Rust
 * `BackboneRegistry` module doc (`⚡️effects/🦀️.rs`, `#region 📡️EffectBackbone`, "Wire shape
 * for the TypeScript counterpart"): `{ "kind": "send" | "delta", "uri": string, "payload": <base64>,
 * "revision"?: number }` — `send` mirrors `Effect::SendMessage`'s `payload: Vec<u8>` (base64 here,
 * raw bytes host-side); `delta` carries {@link EffectBackbone.nextRevision} so the guest can detect a
 * collapsed delta the same way `UiPatch.base_revision` detects a stale diff. This is a DIFFERENT
 * concern from {@link EffectSendMessage}/{@link EventMessage} (the internal kernel `Effect`/`Event`
 * enum shapes, byte-array payloads, checked field-for-field against Rust): THIS shape is what
 * actually crosses an ABI boundary a wasm guest can read (base64 JSON), spec-conformant by inspection
 * against the Rust doc comment (an English doc, not code — NOT machine-diffed the way
 * {@link MessageEndpoint} is; see `## honest gaps`). */
export type BackboneGuestMessage = { readonly kind: "send"; readonly uri: string; readonly payload: string } | { readonly kind: "delta"; readonly uri: string; readonly payload: string; readonly revision: number };

/** 📦️ Chunk size for `String.fromCharCode(...chunk)` below — keeps a very large payload from ever
 * exceeding V8's per-call spread-argument limit (~65536), independent of the wasm guard-region sizing
 * `🧵️shard-client.ts`'s header doc discusses (unrelated concern, same "don't blow a hard limit" shape). */
const BASE64_CHUNK_SIZE = 0x8000;

function bytesToBase64(bytes: readonly number[]): string {
  let binary = "";
  for (let offset = 0; offset < bytes.length; offset += BASE64_CHUNK_SIZE) {
    binary += String.fromCharCode(...bytes.slice(offset, offset + BASE64_CHUNK_SIZE));
  }
  return btoa(binary);
}

function base64ToBytes(encoded: string): readonly number[] {
  const binary = atob(encoded);
  const bytes = new Array<number>(binary.length);
  for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index);
  return bytes;
}

/** 🌉️ {@link EventMessage} → {@link BackboneGuestMessage}. Throws if `source` is not a `Backbone`
 * endpoint (this wire shape only ever represents a backbone message, by the Rust doc's own scope). */
export function encodeBackboneGuestMessage(event: EventMessage, revision?: number): BackboneGuestMessage {
  const source = event.message.source;
  if (!isBackboneEndpoint(source)) throw new Error("encodeBackboneGuestMessage: EventMessage.source is not a Backbone endpoint");
  const uri = source.backbone.uri;
  const payload = bytesToBase64(event.message.payload);
  return revision === undefined ? { kind: "send", uri, payload } : { kind: "delta", uri, payload, revision };
}

/** 🌉️ The inverse of {@link encodeBackboneGuestMessage}. */
export function decodeBackboneGuestMessage(message: BackboneGuestMessage): EventMessage {
  return { message: { source: backboneMessageEndpoint(message.uri), payload: base64ToBytes(message.payload) } };
}
//#endregion 🌉️GuestWire

//#region 🌉️WorkerTransport
/** 🌉️ terra-web-effect-backbone: "route through `🟦️backbone-worker.ts`'s existing document/blob
 * transport rather than opening a second path to the hub" — this region never opens a WebSocket or
 * `EventSource` itself; it only posts the ALREADY-EXISTING `BackboneWorkerRequest` kinds
 * (`open`/`send`) that file's own `🔖️BackboneWorkerProtocol` region (`🟦️.ts`) already
 * exports, and decodes the ALREADY-EXISTING `BackboneWorkerResponse` events it emits. Neither
 * `🟦️backbone-worker.ts` nor `🟦️.ts` is edited by this packet.
 *
 * Encoding choice: an `EffectBackbone` uri is modeled as one opened `documentId` with
 * `ArtifactActorMsg.publishPreview`/`ArtifactEvent.preview` as the send/receive primitive — the ONLY
 * generic, arbitrary-key-and-payload, EPHEMERAL (never persisted — correctly matching a backbone
 * message's own classification) pub/sub-shaped vocabulary that protocol already exposes. Every OTHER
 * `ArtifactActorMsg` kind (`localMutations`, `localSnapshot`, ...) is CRDT-document-shaped, not a fit
 * for an arbitrary backbone send. See `## honest gaps` in this ticket's report for what this reuse
 * does NOT give you (no lossless queueing across a hub drop — matches Rust's own `send` having no
 * queueing either; no dedicated inbound "send" vs "delta" distinction — `bridgeBackboneWorkerInbound`
 * below can only feed {@link EffectBackbone.fanoutDelta}, not {@link EffectBackbone.deliverMessage}). */
import type { ArtifactActorConfig, ArtifactActorMsg, BackboneWorkerResponse, BackboneWorkerWireMessage, PersistenceBinding } from "./🟦️.ts";
import { decodeBackboneWorkerResponse, encodeBackboneWorkerRequest } from "./🟦️.ts";

/** 🌉️ The slice of `Worker` this bridge depends on — lets tests (and any non-browser host) inject a
 * fake without a real `Worker`/`MessagePort`. A real browser `Worker` satisfies this structurally
 * (same seam shape as `🧵️shard-client.ts`'s own `ShardWorkerLike`). */
export interface BackboneWorkerLike {
  postMessage(message: BackboneWorkerWireMessage): void;
  onmessage: ((event: { readonly data: unknown }) => void) | null;
}

const BACKBONE_EFFECT_DOCUMENT_SCHEMA = "os.effect.backbone";

export interface BackboneWorkerTransportOptions {
  readonly actor: string;
  readonly hub: Extract<PersistenceBinding, { readonly kind: "hub" }>;
}

/** 🏗️ Builds a {@link BackboneTransport} for ONE uri, bound to a live (or fake) backbone worker —
 * register it with `effectBackbone.registerEndpoint(uri, createBackboneWorkerTransport(worker, uri,
 * options))`. Opens the underlying document lazily, on the FIRST send (not eagerly at construction),
 * so registering an endpoint that never sends never opens a hub connection for it. */
export function createBackboneWorkerTransport(worker: BackboneWorkerLike, uri: string, options: BackboneWorkerTransportOptions): BackboneTransport {
  let opened = false;
  let seq = 0;
  function ensureOpen(): void {
    if (opened) return;
    opened = true;
    const config: ArtifactActorConfig = { documentId: uri, schema: BACKBONE_EFFECT_DOCUMENT_SCHEMA, bindings: [options.hub], actor: options.actor };
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "open", ...config }) });
  }
  return {
    send(sendUri: string, payload: readonly number[]): void {
      if (sendUri !== uri) throw new Error(`createBackboneWorkerTransport: transport bound to "${uri}", got a send for "${sendUri}"`);
      ensureOpen();
      seq += 1;
      const message: ArtifactActorMsg = { kind: "publishPreview", key: uri, seq, payload: [...payload] };
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: uri, message }) });
    },
  };
}

/** 🏗️ Wires INBOUND `preview` events from `worker` into `backbone.fanoutDelta` — every backbone uri
 * that has been opened (via {@link createBackboneWorkerTransport} or otherwise) reaches subscribed
 * actors through the SAME coalesced-delta path {@link EffectBackbone.fanoutDelta} already implements;
 * a uri nobody has subscribed to is simply a no-op fanout (empty subscriber set), never an error.
 * Chains onto any PRE-EXISTING `worker.onmessage` rather than replacing it, so this can be installed
 * alongside other consumers of the same worker. Returns a disposer that restores the previous handler. */
export function bridgeBackboneWorkerInbound(backbone: EffectBackbone, worker: BackboneWorkerLike): () => void {
  const previous = worker.onmessage;
  worker.onmessage = (event: { readonly data: unknown }) => {
    previous?.(event);
    const data = event.data;
    if (typeof data !== "object" || data === null || !("wire" in data)) return;
    let response: BackboneWorkerResponse;
    try {
      response = decodeBackboneWorkerResponse((data as BackboneWorkerWireMessage).wire);
    } catch {
      return;
    }
    if (response.kind !== "event" || response.event.kind !== "preview") return;
    backbone.fanoutDelta(response.documentId, response.event.payload);
  };
  return () => {
    worker.onmessage = previous;
  };
}
//#endregion 🌉️WorkerTransport

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  function fakeTransport(): { readonly transport: BackboneTransport; readonly received: Array<{ readonly uri: string; readonly payload: readonly number[] }> } {
    const received: Array<{ readonly uri: string; readonly payload: readonly number[] }> = [];
    return { transport: { send: (uri, payload) => received.push({ uri, payload }) }, received };
  }

  class DenyAll implements CapabilityChecker {
    isGranted(): boolean {
      return false;
    }
  }

  //#region 🔑️CapabilityTests
  describe("EffectBackbone capability gating", () => {
    it("mirrors backbone_send_is_rejected_without_the_capability: send is rejected without the messaging.backbone:<uri> capability", () => {
      const backbone = new EffectBackbone({ capabilities: new DenyAll() });
      const { transport } = fakeTransport();
      backbone.registerEndpoint("studio-42", transport);
      const outcome = backbone.send("actor-1", "studio-42", [1, 2, 3]);
      expect(outcome).toEqual({ ok: false, error: { kind: "capabilityDenied", uri: "studio-42" } });
    });

    it("mirrors backbone_send_reaches_the_registered_transport_once_granted: send reaches the registered transport once granted", () => {
      const backbone = new EffectBackbone({ capabilities: new AllowAllCapabilities() });
      const { transport, received } = fakeTransport();
      backbone.registerEndpoint("studio-42", transport);
      const outcome = backbone.send("actor-1", "studio-42", [1, 2, 3]);
      expect(outcome).toEqual({ ok: true });
      expect(received).toEqual([{ uri: "studio-42", payload: [1, 2, 3] }]);
    });

    it("send against an unregistered uri fails noSuchEndpoint even when granted", () => {
      const backbone = new EffectBackbone({ capabilities: new AllowAllCapabilities() });
      const outcome = backbone.send("actor-1", "nowhere", [1]);
      expect(outcome).toEqual({ ok: false, error: { kind: "noSuchEndpoint", uri: "nowhere" } });
    });

    it("dispatchSendMessage routes a Backbone target through send, and reports a non-Backbone target as the documented no-op gap", () => {
      const backbone = new EffectBackbone({ capabilities: new AllowAllCapabilities() });
      const { transport, received } = fakeTransport();
      backbone.registerEndpoint("studio-42", transport);
      const sent = backbone.dispatchSendMessage("actor-1", { sendMessage: { target: backboneMessageEndpoint("studio-42"), payload: [9] } });
      expect(sent).toEqual({ kind: "sent" });
      expect(received).toEqual([{ uri: "studio-42", payload: [9] }]);
      const skipped = backbone.dispatchSendMessage("actor-1", { sendMessage: { target: { topic: { name: "whatever" } }, payload: [9] } });
      expect(skipped).toEqual({ kind: "notBackboneTarget" });
    });
  });
  //#endregion 🔑️CapabilityTests

  //#region 📥️DeltaFanoutTests
  describe("EffectBackbone delta fan-out", () => {
    it("mirrors backbone_delta_fanout_coalesces_a_burst_for_the_same_uri: a burst for the same uri collapses to the latest, not queued", () => {
      const backbone = new EffectBackbone();
      backbone.subscribe("actor-1", "studio-42");
      const first = backbone.fanoutDelta("studio-42", [1]);
      const second = backbone.fanoutDelta("studio-42", [2]);
      expect(first.get("actor-1")).toEqual({ kind: "delivered" });
      expect(second.get("actor-1")).toEqual({ kind: "collapsed" });
      const drained = backbone.drain("actor-1");
      expect(drained).toEqual([{ message: { source: { backbone: { uri: "studio-42" } }, payload: [2] } }]);
    });

    it("fanoutDelta only reaches actors subscribed to that specific uri", () => {
      const backbone = new EffectBackbone();
      backbone.subscribe("actor-1", "studio-42");
      backbone.subscribe("actor-2", "studio-99");
      const outcomes = backbone.fanoutDelta("studio-42", [7]);
      expect([...outcomes.keys()]).toEqual(["actor-1"]);
      expect(backbone.drain("actor-2")).toEqual([]);
    });

    it("fanoutDelta against a uri with no subscribers is an empty, error-free no-op", () => {
      const backbone = new EffectBackbone();
      expect(backbone.fanoutDelta("nobody-here", [1]).size).toBe(0);
    });

    it("nextRevision is monotonic per uri and independent across uris", () => {
      const backbone = new EffectBackbone();
      expect(backbone.nextRevision("a")).toBe(1);
      expect(backbone.nextRevision("a")).toBe(2);
      expect(backbone.nextRevision("b")).toBe(1);
    });
  });
  //#endregion 📥️DeltaFanoutTests

  //#region 🚨️OverflowTests
  describe("EffectBackbone queue overflow", () => {
    it("a lossless direct-send queue rejects-and-reports once full, rather than silently dropping (consistent with backbone-worker's outbox contract)", () => {
      const reporter = new RecordingBackboneOverflowReporter();
      const backbone = new EffectBackbone({ sendQueueCapacity: 2, reporter });
      backbone.subscribe("actor-1", "studio-42");
      expect(backbone.deliverMessage("actor-1", "studio-42", [1])).toEqual({ kind: "delivered" });
      expect(backbone.deliverMessage("actor-1", "studio-42", [2])).toEqual({ kind: "delivered" });
      const third = backbone.deliverMessage("actor-1", "studio-42", [3]);
      expect(third).toEqual({ kind: "rejectedFull" });
      expect(reporter.recorded()).toHaveLength(1);
      expect(reporter.recorded()[0]).toMatchObject({ actor: "actor-1", uri: "studio-42", channel: "send", outcome: { kind: "rejectedFull" } });
      // 🧾️ Nothing was silently dropped: both accepted messages are still there, in order.
      expect(backbone.drain("actor-1")).toEqual([
        { message: { source: { backbone: { uri: "studio-42" } }, payload: [1] } },
        { message: { source: { backbone: { uri: "studio-42" } }, payload: [2] } },
      ]);
    });

    it("deliverMessage against a uri actor never subscribed to is noSuchSubscriber, not a crash", () => {
      const backbone = new EffectBackbone();
      expect(backbone.deliverMessage("actor-1", "studio-42", [1])).toEqual({ kind: "noSuchSubscriber" });
    });
  });
  //#endregion 🚨️OverflowTests

  //#region 🪪️IsolationTests
  describe("EffectBackbone per-instance isolation", () => {
    it("an inbound Event::Message reaches only the subscribed actor, not other actors", () => {
      const backbone = new EffectBackbone();
      backbone.subscribe("actor-1", "studio-42");
      backbone.subscribe("actor-2", "studio-42");
      backbone.deliverMessage("actor-1", "studio-42", [123]);
      expect(backbone.drain("actor-1")).toHaveLength(1);
      expect(backbone.drain("actor-2")).toHaveLength(0);
    });

    it("two EffectBackbone instances for the SAME plugin never share endpoints or subscriptions", () => {
      const instanceA = new EffectBackbone({ capabilities: new AllowAllCapabilities() });
      const instanceB = new EffectBackbone({ capabilities: new AllowAllCapabilities() });
      const { transport } = fakeTransport();
      instanceA.registerEndpoint("studio-42", transport);
      instanceA.subscribe("actor-1", "studio-42");
      instanceA.deliverMessage("actor-1", "studio-42", [1]);

      // instanceB has no endpoint registered for studio-42 at all — a send through it fails noSuchEndpoint.
      expect(instanceB.send("actor-1", "studio-42", [1])).toEqual({ ok: false, error: { kind: "noSuchEndpoint", uri: "studio-42" } });
      // instanceB was never subscribed — nothing instanceA delivered leaks across.
      expect(instanceB.drain("actor-1")).toEqual([]);
      // instanceA's own state is untouched by instanceB's independent existence.
      expect(instanceA.drain("actor-1")).toHaveLength(1);
    });
  });
  //#endregion 🪪️IsolationTests

  //#region 🌉️GuestWireTests
  describe("BackboneGuestMessage wire shape", () => {
    it("round-trips a send message through base64, matching the Rust doc's {kind,uri,payload} shape", () => {
      const event: EventMessage = { message: { source: backboneMessageEndpoint("studio-42"), payload: [1, 2, 3, 255] } };
      const wire = encodeBackboneGuestMessage(event);
      expect(wire.kind).toBe("send");
      expect(Object.keys(wire).sort()).toEqual(["kind", "payload", "uri"]);
      expect(decodeBackboneGuestMessage(wire)).toEqual(event);
    });

    it("round-trips a delta message with its revision, matching the Rust doc's {kind,uri,payload,revision} shape", () => {
      const event: EventMessage = { message: { source: backboneMessageEndpoint("studio-42"), payload: [9] } };
      const wire = encodeBackboneGuestMessage(event, 7);
      expect(wire).toEqual({ kind: "delta", uri: "studio-42", payload: btoa(String.fromCharCode(9)), revision: 7 });
      expect(decodeBackboneGuestMessage(wire)).toEqual(event);
    });

    it("refuses to encode a non-Backbone source", () => {
      const event: EventMessage = { message: { source: { topic: { name: "x" } }, payload: [1] } };
      expect(() => encodeBackboneGuestMessage(event)).toThrow();
    });
  });
  //#endregion 🌉️GuestWireTests

  //#region 🌉️WorkerTransportTests
  describe("createBackboneWorkerTransport / bridgeBackboneWorkerInbound", () => {
    function fakeWorker(): { readonly worker: BackboneWorkerLike; readonly posted: BackboneWorkerWireMessage[] } {
      const posted: BackboneWorkerWireMessage[] = [];
      return { worker: { postMessage: (message) => posted.push(message), onmessage: null }, posted };
    }

    it("send lazily opens the document once, then posts publishPreview through the existing send request kind", async () => {
      const { decodeBackboneWorkerRequest } = await import("./🟦️.ts");
      const { worker, posted } = fakeWorker();
      const transport = createBackboneWorkerTransport(worker, "studio-42", { actor: "actor-1", hub: { kind: "hub", baseUrl: "https://hub.example", spaceId: "space-1" } });
      transport.send("studio-42", [1, 2]);
      transport.send("studio-42", [3, 4]);
      expect(posted).toHaveLength(3); // one "open" + two "send"
      const decoded = posted.map((message) => decodeBackboneWorkerRequest(message.wire));
      expect(decoded[0]).toMatchObject({ kind: "open", documentId: "studio-42", actor: "actor-1" });
      expect(decoded[1]).toMatchObject({ kind: "send", documentId: "studio-42", message: { kind: "publishPreview", key: "studio-42", seq: 1, payload: [1, 2] } });
      expect(decoded[2]).toMatchObject({ kind: "send", documentId: "studio-42", message: { kind: "publishPreview", key: "studio-42", seq: 2, payload: [3, 4] } });
    });

    it("send throws if used for a different uri than it was bound to", () => {
      const { worker } = fakeWorker();
      const transport = createBackboneWorkerTransport(worker, "studio-42", { actor: "actor-1", hub: { kind: "hub", baseUrl: "https://hub.example", spaceId: "space-1" } });
      expect(() => transport.send("other-uri", [1])).toThrow();
    });

    it("bridgeBackboneWorkerInbound turns an inbound preview event into a fanoutDelta reaching a subscribed actor, chaining any prior onmessage", async () => {
      const { encodeBackboneWorkerResponse } = await import("./🟦️.ts");
      const { worker } = fakeWorker();
      const priorCalls: unknown[] = [];
      const priorHandler = (event: { readonly data: unknown }): void => {
        priorCalls.push(event.data);
      };
      worker.onmessage = priorHandler;
      const backbone = new EffectBackbone();
      backbone.subscribe("actor-1", "studio-42");
      const dispose = bridgeBackboneWorkerInbound(backbone, worker);
      const wire = { wire: encodeBackboneWorkerResponse({ kind: "event", documentId: "studio-42", event: { kind: "preview", actor: "peer", key: "studio-42", seq: 1, payload: [42] } }) };
      worker.onmessage?.({ data: wire });
      expect(priorCalls).toEqual([wire]); // chained, not replaced
      expect(backbone.drain("actor-1")).toEqual([{ message: { source: { backbone: { uri: "studio-42" } }, payload: [42] } }]);
      dispose();
      expect(worker.onmessage).toBe(priorHandler); // restored, not nulled
    });

    it("bridgeBackboneWorkerInbound ignores non-event and non-preview responses without throwing", async () => {
      const { encodeBackboneWorkerResponse } = await import("./🟦️.ts");
      const { worker } = fakeWorker();
      const backbone = new EffectBackbone();
      backbone.subscribe("actor-1", "studio-42");
      bridgeBackboneWorkerInbound(backbone, worker);
      worker.onmessage?.({ data: { wire: encodeBackboneWorkerResponse({ kind: "ready" }) } });
      worker.onmessage?.({ data: "not a wire message at all" });
      expect(backbone.drain("actor-1")).toEqual([]);
    });
  });
  //#endregion 🌉️WorkerTransportTests

  //#region 🧬️ParityTests
  describe("EffectBackbone Rust↔TS wire parity", () => {
    function parseRustVariants(body: string): Array<{ readonly name: string; readonly fields: readonly string[] | null }> {
      const stripped = body.replace(/\/\/\/.*$/gm, "").replace(/\/\/.*$/gm, "");
      const variantPattern = /(\w+)\s*(?:\{([^{}]*)\}|\(([^()]*)\))?\s*,/g;
      const variants: Array<{ readonly name: string; readonly fields: readonly string[] | null }> = [];
      let match: RegExpExecArray | null;
      while ((match = variantPattern.exec(stripped)) !== null) {
        const [, name, structFields, tupleType] = match;
        if (structFields !== undefined) {
          const fields = structFields
            .split(",")
            .map((part) => part.trim())
            .filter((part) => part.length > 0)
            .map((part) => part.split(":")[0]!.trim());
          variants.push({ name: name!, fields });
        } else if (tupleType !== undefined) {
          variants.push({ name: name!, fields: null });
        }
      }
      return variants;
    }

    function parseFieldList(fields: string): readonly string[] {
      return fields
        .split(",")
        .map((part) => part.trim())
        .filter((part) => part.length > 0)
        .map((part) => part.split(":")[0]!.trim());
    }

    it("MessageEndpoint variant/field names match the live Rust enum in 🎠️kernel/🦀️.rs", async () => {
      const { readFileSync } = await import("node:fs");
      const kernelUrl = new URL("../../🔨️modules/🎠️kernel/🦀️.rs", import.meta.url);
      const source = readFileSync(kernelUrl, "utf8");
      const enumMatch = source.match(/pub enum MessageEndpoint \{([\s\S]*?)\n\}/);
      expect(enumMatch).not.toBeNull(); // [DEBUG] `pub enum MessageEndpoint { ... }` shape not found — Rust source changed, update this test's regex
      const rustVariants = parseRustVariants(enumMatch![1]!);
      expect(rustVariants.map((variant) => variant.name)).toEqual(MESSAGE_ENDPOINT_VARIANT_FIELDS.map((variant) => variant.kind));
      for (const rustVariant of rustVariants) {
        if (rustVariant.fields === null) continue;
        const tsVariant = MESSAGE_ENDPOINT_VARIANT_FIELDS.find((variant) => variant.kind === rustVariant.name)!;
        expect(tsVariant.fields).toEqual(rustVariant.fields);
      }
    });

    it("Effect::SendMessage fields match the live Rust variant in 🎠️kernel/🦀️.rs", async () => {
      const { readFileSync } = await import("node:fs");
      const kernelUrl = new URL("../../🔨️modules/🎠️kernel/🦀️.rs", import.meta.url);
      const source = readFileSync(kernelUrl, "utf8");
      const variantMatch = source.match(/\bSendMessage\s*\{([^{}]*)\}/);
      expect(variantMatch).not.toBeNull(); // [DEBUG] `SendMessage { ... }` not found — Rust `Effect::SendMessage` changed, update this test
      expect(parseFieldList(variantMatch![1]!)).toEqual(["target", "payload"]);
    });

    it("Event::Message fields match the live Rust variant in 🎠️kernel/🦀️.rs", async () => {
      const { readFileSync } = await import("node:fs");
      const kernelUrl = new URL("../../🔨️modules/🎠️kernel/🦀️.rs", import.meta.url);
      const source = readFileSync(kernelUrl, "utf8");
      const variantMatch = source.match(/\bMessage\s*\{([^{}]*)\}/);
      expect(variantMatch).not.toBeNull(); // [DEBUG] `Message { ... }` not found — Rust `Event::Message` changed, update this test
      expect(parseFieldList(variantMatch![1]!)).toEqual(["source", "payload"]);
    });
  });
  //#endregion 🧬️ParityTests
}
//#endregion 🧪️Tests
