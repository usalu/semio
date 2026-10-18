import type { MutationEnvelope, WireAckStage } from "@semio-tech/framework-replication";
import { encodeBackboneMessage, hubTransientApplyRefusalV1, hubCommandRejectionV1 } from "../../../../🟦️";
import type { CommandAckOutcome } from "../../../../🟦️";
import { type ArtifactState, emitEvent } from "../🧾️document-state/🟦️.ts";
import { flushMutationsToHubIfReady, refusedBatchCorrection, releaseDocumentBackboneOwnership } from "../📮️hub-outbox/🟦️.ts";
import { requireArtifactRebootstrap } from "../🚀️artifact-bootstrap/🟦️.ts";
import { requestDocumentActorRecoveryV1 } from "../📥️execution-target-read/🟦️.ts";
import { emitMutationEvent, fromWireEnvelope } from "../🌉️wire-bridge/🟦️.ts";
import { hubBinding } from "../🧩️actor-wire-helpers/🟦️.ts";
import { resendAfterTransientRefusal } from "../🛰️hub-relay/🟦️.ts";
import { documentAwaitsBrowserActor, setStatus } from "../🎭️actor-lifecycle/🟦️.ts";

/** 🔁️ Applies the hub's correction of one of this human's refused or transformed batches. The Shell's local instance
 * takes it as envelopes: the rollback, then the hub's replacement. A verified browser actor cannot — its guest has no
 * rollback protocol, and the TS inverse twin is no op its VCS decodes (`module.vcs: unsupported op format 0`: the child
 * faulted on it, run `c10gp14h`) — while leaving the actor alone kept the refused edit on the author's screen, a phantom
 * edit the hub never accepted (ticket 26/09/23 C10, G-P2-3). The actor's speculative state is discarded instead: the
 * document rebootstraps from the hub's authoritative pair (which lacks the refused edit or already holds the transformed
 * one), and the batches queued behind it replay on top. */
async function applyAckCorrection(state: ArtifactState, sent: readonly MutationEnvelope[], replacement: MutationEnvelope | null): Promise<void> {
  const { rollbacks, retracted } = refusedBatchCorrection(state, sent),
    retraction = retracted.length > 0 ? encodeBackboneMessage({ kind: "retract", mutationIds: retracted }) : null,
    reservation = state.browserActorReservation;
  if (reservation !== null) {
    if (rollbacks.length > 0 || replacement !== null) {
      await requireArtifactRebootstrap(state);
      return;
    }
    if (retraction === null) return;
    try {
      await reservation.receiveBackbone(retraction);
    } catch (error) {
      console.error("[backbone-worker] the browser actor child refused a retraction; reopening from the last confirmed state", state.config.documentId, error);
      requestDocumentActorRecoveryV1(state, "inbound-frame");
    }
    return;
  }
  emitMutationEvent(state, replacement === null ? rollbacks : [...rollbacks, replacement]);
  if (retraction !== null && hubBinding(state.config)) emitEvent(state, { kind: "documentBackbone", message: retraction });
}

/** 📮️ Resolves one outbound `Commands` batch's terminal `Applied` stage — mirrors the Rust actor's
 * `handle_ack`. `pendingMutations` (the UI-facing "unconfirmed" count) is trimmed by id, the same
 * way the old per-operation `ack` frame used to. */
export async function handleAck(state: ArtifactState, batchId: number, stages: readonly WireAckStage[]): Promise<void> {
  for (const stage of stages) {
    if (typeof stage !== "object" || !("Applied" in stage)) continue;
    const sent = state.pendingBatches.get(batchId);
    state.pendingBatches.delete(batchId);
    if (!sent) continue;
    const outcome = stage.Applied.outcome;
    if (typeof outcome === "object" && "Rejected" in outcome && hubTransientApplyRefusalV1(outcome.Rejected.messages)) {
      resendAfterTransientRefusal(state, sent);
      continue;
    }
    releaseDocumentBackboneOwnership(state, sent);
    const sentIds = new Set(sent.map((envelope) => envelope.id));
    state.pendingMutations = state.pendingMutations.filter((envelope) => !sentIds.has(envelope.id));

    let ackOutcome: CommandAckOutcome;
    let reorder = false;
    if (outcome === "Accepted") {
      if (state.transientRefusal?.timer != null) clearTimeout(state.transientRefusal.timer);
      state.transientRefusal = null;
      ackOutcome = { kind: "accepted" };
      reorder = state.remoteFoldedOverLocal && state.pendingMutations.length === 0 && documentAwaitsBrowserActor(state);
    } else if ("Transformed" in outcome) {
      await applyAckCorrection(state, sent, fromWireEnvelope(outcome.Transformed.envelope));
      ackOutcome = { kind: "transformed" };
    } else {
      await applyAckCorrection(state, sent, null);
      ackOutcome = hubCommandRejectionV1(outcome.Rejected);
    }
    setStatus(state, { pendingMutations: state.pendingMutations.length });
    emitEvent(state, { kind: "commandOutcome", batchId, outcome: ackOutcome });
    if (reorder) await requireArtifactRebootstrap(state);
  }
  flushMutationsToHubIfReady(state);
}

/** ✅️ Settles this replica's own operations the hub's log already holds: a `Commands` frame (a catch-up tail or a relay) that carries
 * an operation still in the outbox or a pending batch proves the hub committed it although its `Ack` was lost with the socket, so it
 * leaves both (an emptied batch leaves too), its retention is released, its echo stays suppressed, and it is never resent — a
 * resend is re-stamped and the hub refuses it as a replayed operation. TS twin of the Rust `settle_committed_envelopes`, both
 * replaying the parity scenario `lost-ack-committed-op-settles` (`🔄️sync/⚖️parity/🧫️fixtures`). */
export function settleCommittedEnvelopes(state: ArtifactState, committedIds: readonly string[]): void {
  const committed = new Set(committedIds);
  const settled = new Set<MutationEnvelope>();
  const keep = (envelope: MutationEnvelope): boolean => {
    if (!committed.has(state.exactLocalEnvelopes.get(envelope)?.envelope.mutation_id ?? envelope.id)) return true;
    settled.add(envelope);
    return false;
  };
  state.outbox = state.outbox.filter(keep);
  for (const [batchId, batch] of [...state.pendingBatches]) {
    const kept = batch.filter(keep);
    if (kept.length === 0) state.pendingBatches.delete(batchId);
    else state.pendingBatches.set(batchId, kept);
  }
  if (settled.size === 0) return;
  releaseDocumentBackboneOwnership(state, [...settled]);
  state.pendingMutations = state.pendingMutations.filter((envelope) => !settled.has(envelope));
  setStatus(state, { pendingMutations: state.pendingMutations.length });
  flushMutationsToHubIfReady(state);
}
