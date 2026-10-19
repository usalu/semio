import type { ClientFrame, WireLane, MutationEnvelope } from "@semio-tech/framework-replication";
import { encodeClientFrame } from "@semio-tech/framework-replication";
import type { ArtifactState } from "../🧾️document-state/🟦️.ts";
import { flushMutationsToHubIfReady, queueOutbox, rejectReadOnlyExecutionTarget, releaseDocumentBackboneOwnership, returnToOutboxFront } from "../📮️hub-outbox/🟦️.ts";
import { setStatus } from "../🎭️actor-lifecycle/🟦️.ts";

export function sendWireFrame(state: ArtifactState, frame: ClientFrame, lane: WireLane): void {
  if (state.socket?.readyState === WebSocket.OPEN) state.socket.send(encodeClientFrame(frame, lane));
}

/** ⏳️ Whether a mounted document is still catching up with the hub: its actor is not bound to the document port or the retained
 * catch-up tail has not drained into it, an artifact bootstrap or rebuild is in flight, the tail the hub named in its `Welcome` is
 * not reached yet, or the cold pair is not applied. An input then would apply to a document older than the hub's — a whole-value
 * verb (typing's `textEdit`) would overwrite what the tail brings — so it is refused typed (`action-catching-up`), never applied. */
export function documentCatchingUpV1(facts: Readonly<{ bound: boolean; backboneReady: boolean; bootstrap: boolean; rebootstrap: boolean; tail: boolean; coldApplied: boolean; coldTransfer: boolean }>): boolean {
  return !facts.bound || !facts.backboneReady || facts.bootstrap || facts.rebootstrap || facts.tail || !facts.coldApplied || facts.coldTransfer;
}
/** ⏳️ Resend delay after a transient refusal: doubled per consecutive refusal between these bounds, jittered. */
const HUB_TRANSIENT_REFUSAL_MIN_MS = 250;
const HUB_TRANSIENT_REFUSAL_MAX_MS = 5_000;

/** ⏳️ Keeps a batch the hub refused for a transient reason: its envelopes return to the front of the outbox (nothing was applied,
 * nothing is rolled back or rebuilt), the next drain is bounded to half the refused batch and resent after a jittered backoff. */
export function resendAfterTransientRefusal(state: ArtifactState, sent: readonly MutationEnvelope[]): void {
  returnToOutboxFront(state, sent);
  const previous = state.transientRefusal;
  if (previous?.timer != null) clearTimeout(previous.timer);
  const backoffMs = Math.min(HUB_TRANSIENT_REFUSAL_MAX_MS, Math.max(HUB_TRANSIENT_REFUSAL_MIN_MS, (previous?.backoffMs ?? 0) * 2));
  const refusal: NonNullable<ArtifactState["transientRefusal"]> = { timer: null, backoffMs, batchLimit: Math.max(1, Math.ceil(sent.length / 2)) };
  refusal.timer = setTimeout(() => {
    refusal.timer = null;
    if (state.transientRefusal === refusal) flushMutationsToHubIfReady(state);
  }, backoffMs / 2 + Math.random() * (backoffMs / 2));
  state.transientRefusal = refusal;
  setStatus(state, { pendingMutations: state.pendingMutations.length });
}

/** 🧺️ Hands locally authored envelopes to the hub: they join the outbox and {@link flushMutationsToHubIfReady} drains it.
 * Mirrors the Rust actor's `relay_operations_to_hub`. Finding 5: a closed socket no longer no-ops silently — the envelopes wait
 * in {@link ArtifactState.outbox}, and {@link handleHubFrame}'s authenticated `Session` branch drains that outbox only after the
 * grant actor is proven, so nothing is lost or sent pre-authority. */
export function relayMutationsToHub(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  if (envelopes.length === 0) return;
  if (state.executionTargetLease !== null && state.executionTargetLease.live && !state.executionTargetLease.fields().grant.write) {
    releaseDocumentBackboneOwnership(state, envelopes);
    const refusedIds = new Set(envelopes.map((envelope) => envelope.id));
    state.pendingMutations = state.pendingMutations.filter((envelope) => !refusedIds.has(envelope.id));
    state.outbox = state.outbox.filter((envelope) => !refusedIds.has(envelope.id));
    setStatus(state, { pendingMutations: state.pendingMutations.length });
    rejectReadOnlyExecutionTarget(state, envelopes);
    return;
  }
  queueOutbox(state, envelopes);
  flushMutationsToHubIfReady(state);
}
