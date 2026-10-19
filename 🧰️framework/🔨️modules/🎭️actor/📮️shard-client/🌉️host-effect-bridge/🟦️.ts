import type { ShardClientState } from "../🗃️state/🟦️.ts";
import type { ShardClientHubPort } from "../🧩️hub/🟦️.ts";
import type { HostEffectLedger, PendingHostEffect, ShardActivation } from "../🔐️activation/🟦️.ts";
import { type ShardQuotaBreach, formatQuotaBreachMessage } from "../🚨️quota/🟦️.ts";
import type { OutboundMessage } from "../📨️messages/🟦️.ts";
import type { ShardFrame } from "../📜️wire/🟦️.ts";

/** 🌉️ Bridges the `effect-request` frames a guest sends up to the host effect handler and posts the replies back down. */
export class ShardHostEffectBridge {
  private readonly state: ShardClientState;
  private readonly hub: ShardClientHubPort;
  constructor(hub: ShardClientHubPort) { this.hub = hub; this.state = hub.state; }

  /** 📨️ terra-shard-effect-bridge: dispatches an inbound `"frame"` — today the ONLY frame a worker
   * ever sends UP is `🟨️.js`'s `effect-request` (🧪️ terra-web-bridges); any other `Envelope`
   * payload kind (the `effect-emit`/`ui-patch-emit` fire-and-forget doors, or a future frame kind
   * entirely) is intentionally ignored here rather than thrown on — the same forward-compat tolerance
   * {@link interpretShardFrame}'s own `"unknown"` branch already established for this file. Routing
   * `effect-emit`/`ui-patch-emit` to a real handler is explicitly out of this packet's scope (its own
   * ticket brief only closes the effect-request/effect-complete/effect-error loop) — flagged in the
   * accompanying report as a known gap for whoever owns emit routing next. */
  handleInboundFrame(activation: ShardActivation, frame: ShardFrame): void {
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
    const outstanding = this.state.outstandingEffectsByActor.get(actorId) ?? { activation, requests: new Map<string, PendingHostEffect>(), head: null, tail: null };
    if (outstanding.activation !== activation || outstanding.requests.has(requestId)) return;
    if (outstanding.requests.size >= this.state.maxOutstandingEffectsPerActor) {
      const breach: ShardQuotaBreach = { quota: "outstandingRequests", limit: this.state.maxOutstandingEffectsPerActor, actual: outstanding.requests.size };
      this.replyEffectError(activation, requestId, formatQuotaBreachMessage(breach));
      return;
    }
    if (!this.state.onHostEffect) {
      this.replyEffectError(activation, requestId, "no host effect handler installed");
      return;
    }
    const controller = new AbortController();
    const entry: PendingHostEffect = { activation, controller, requestId, previous: outstanding.tail, next: null };
    if (outstanding.tail) outstanding.tail.next = entry; else outstanding.head = entry;
    outstanding.tail = entry;
    outstanding.requests.set(requestId, entry);
    this.state.outstandingEffectsByActor.set(actorId, outstanding);
    this.state.onHostEffect(actorId, effect, params, controller.signal).then(
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
    const outstanding = this.state.outstandingEffectsByActor.get(actorId);
    if (outstanding?.activation !== entry.activation || outstanding.requests.get(requestId) !== entry) return false;
    this.removeEffect(outstanding, entry);
    if (outstanding.requests.size === 0) this.state.outstandingEffectsByActor.delete(actorId);
    return !entry.controller.signal.aborted && this.hub.activations.activationIsActive(entry.activation);
  }

  /** 🧹️ Aborts and clears every outstanding host-effect for one actor — called by {@link failShard}
   * (whole shard lost) and {@link dispose} (single actor gone), so losing a worker never strands a
   * pending effect. A handler using `signal` (e.g. `fetch`) genuinely stops the underlying work; either
   * way the ledger entry is gone immediately, so a later settle callback for the same `requestId` is
   * recognized as stale by {@link settleEffect} and posts no reply. */
  abortOutstandingEffects(actorId: string): void {
    const outstanding = this.state.outstandingEffectsByActor.get(actorId);
    if (!outstanding) return;
    this.state.outstandingEffectsByActor.delete(actorId);
    while (outstanding.head) this.cancelOneEffect(outstanding);
  }

  private removeEffect(ledger: HostEffectLedger, entry: PendingHostEffect): void {
    if (entry.previous) entry.previous.next = entry.next; else ledger.head = entry.next;
    if (entry.next) entry.next.previous = entry.previous; else ledger.tail = entry.previous;
    ledger.requests.delete(entry.requestId);
    entry.previous = null;
    entry.next = null;
  }

  cancelOneEffect(ledger: HostEffectLedger): void {
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
    if (!this.hub.activations.activationIsActive(activation)) return;
    const { actorId, slot, generation } = activation;
    this.state.effectReplySeq += 1;
    const frame: ShardFrame = {
      kind: "Envelope",
      envelope: { to: actorId, from: { kind: "kernel" }, lane: "Background", seq: this.state.effectReplySeq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload: innerPayload } },
    };
    slot.worker.postMessage({ kind: "frame", requestId: this.hub.transport.nextRequestId(), actorId, activationGeneration: generation, frame } satisfies OutboundMessage);
  }

  private replyEffectComplete(activation: ShardActivation, requestId: string, value: unknown): void {
    this.postEffectReply(activation, "effect-complete", { requestId, value });
  }

  private replyEffectError(activation: ShardActivation, requestId: string, message: string): void {
    this.postEffectReply(activation, "effect-error", { requestId, message });
  }
}
