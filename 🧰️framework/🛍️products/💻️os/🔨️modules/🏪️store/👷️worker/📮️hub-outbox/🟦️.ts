import { DOCUMENT_BACKBONE_BATCH_LIMITS, encodeClientCommandsFrameExact, DOCUMENT_BACKBONE_RETENTION_LIMITS, HISTORY_TRANSITION_DIFF_SCHEMA } from "@semio-tech/framework-replication";
import type { MutationEnvelope, ExactWireMutationEnvelope, WireFrontierSummary } from "@semio-tech/framework-replication";
import type { CommandAckOutcome } from "../../../../🟦️";
import { localCommandRejectionV1 } from "../../../../🟦️";
import { type ArtifactState, emitEvent } from "../🧾️document-state/🟦️.ts";
import { exactWireEnvelope, nextWireTimestamp, rollbackEnvelope, toWireEnvelope } from "../🌉️wire-bridge/🟦️.ts";
import { PENDING_MUTATIONS_QUEUE_LIMIT } from "../⚙️constants/🟦️.ts";

export function documentBackboneAdmissionReady(state: ArtifactState): boolean {
  if (state.artifactBootstrap !== null || state.artifactRebootstrapRequired || state.requiredTailFrontier !== null) return false;
  if (state.currentPack === null || state.currentSpr === null || state.frontier === null || state.verifiedColdPair === null) return false;
  try {
    state.verifiedColdPair.assertCurrent();
    return true;
  } catch {
    return false;
  }
}

function documentBackboneRelayReady(state: ArtifactState): boolean {
  return state.hubActorReady && state.socket?.readyState === WebSocket.OPEN && documentBackboneAdmissionReady(state);
}

/** 📦️ Bounds of one `Commands` batch. The outbox drains ack-clocked — one batch in flight per document, the next after its `Ack` —
 * in batches of at most this many envelopes and {@link HUB_OUTBOX_BATCH_BYTES} wire bytes, so a queue that grew during a
 * connection shortage never reaches the hub as one oversized batch (ticket 26/09/23 C12, run `c12short3`: a whole-outbox batch
 * was refused `DB I/O aggregate admission exhausted` and every keystroke typed during the cut was lost), and nothing is ever sent
 * behind a batch whose outcome is unknown: a transiently refused batch goes out again FIRST, and a later batch whose envelopes
 * depend on it can never reach the hub before it (the hub refuses those for good: `dependency names an unknown edit`). */
const HUB_OUTBOX_BATCH_ENVELOPES = 16;
/** 📦️ Wire bytes of one drained batch: half the declared batch maximum, as the Rust store's `announce_history`. One envelope larger
 * than this still goes out alone. */
const HUB_OUTBOX_BATCH_BYTES = DOCUMENT_BACKBONE_BATCH_LIMITS.maximumBytes / 2;

/** 🚿️ Sends the outbox's next bounded batch once the socket is live, no batch awaits its `Ack` and no transient refusal is backing
 * off. Called on every relay, `Ack`, authenticated `Session`, catch-up completion, link restore and refusal timer. */
export function flushMutationsToHubIfReady(state: ArtifactState): void {
  if (!documentBackboneRelayReady(state) || state.outbox.length === 0 || state.pendingBatches.size > 0 || state.transientRefusal?.timer != null) return;
  const limit = state.transientRefusal?.batchLimit ?? HUB_OUTBOX_BATCH_ENVELOPES;
  const envelopes: MutationEnvelope[] = [];
  const wireEnvelopes: ExactWireMutationEnvelope[] = [];
  let bytes = 0;
  for (const envelope of state.outbox) {
    if (envelopes.length === limit) break;
    const wire = hubWireEnvelope(state, envelope);
    const size = wire.mutation_id.length + wire.document_id.length + wire.actor.length + wire.diff.schema.length + wire.diff.payload.length + wire.inverse.schema.length + wire.inverse.payload.length + wire.dependencies.reduce((sum, dependency) => sum + dependency.length, 0) + wire.target.reduce((sum, segment) => sum + segment.length, 0);
    if (envelopes.length > 0 && bytes + size > HUB_OUTBOX_BATCH_BYTES) break;
    bytes += size;
    envelopes.push(envelope);
    wireEnvelopes.push(wire);
  }
  state.outbox.splice(0, envelopes.length);
  const batchId = state.nextBatchId;
  state.nextBatchId += 1;
  state.pendingBatches.set(batchId, envelopes);
  if (state.socket?.readyState === WebSocket.OPEN) state.socket.send(encodeClientCommandsFrameExact(batchId, wireEnvelopes, "command"));
}

/** 🌉️ One outbound envelope as this socket's actor sends it: the exact causal envelope a bound port authored — its authored
 * `(hlc, id)` kept, only the actor rewritten to the socket's subject, the TS twin of the Rust actors' relay — or the TS twin's
 * {@link toWireEnvelope} stamped on this worker's clock. */
function hubWireEnvelope(state: ArtifactState, envelope: MutationEnvelope): ExactWireMutationEnvelope {
  const exact = state.exactLocalEnvelopes.get(envelope)?.envelope;
  if (exact === undefined) return exactWireEnvelope(toWireEnvelope(envelope, nextWireTimestamp(state), state.actor));
  return { ...exact, actor: state.actor };
}

/** ⏮️ Puts envelopes that were sent but never applied back at the FRONT of the outbox, in their send order: they precede everything
 * queued behind them. */
export function returnToOutboxFront(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  if (envelopes.length === 0) return;
  const ids = new Set(envelopes.map((envelope) => envelope.id));
  state.outbox = [...envelopes, ...state.outbox.filter((envelope) => !ids.has(envelope.id))];
}

/** 🚨️ Local pending-mutation queue overflow (finding 5) — a mutation is NEVER silently dropped: a
 * batch that would push {@link ArtifactState.pendingMutations} past
 * {@link PENDING_MUTATIONS_QUEUE_LIMIT} is rejected wholesale and reported through the exact same
 * {@link CommandAckOutcome} vocabulary a real hub rejection uses (`kind: "rejected"`), so the
 * caller needs no separate "local overflow" code path to show the pressure — it is a
 * `commandOutcome` event either way. `batchId` counts down from -1, a range the hub-assigned ids
 * in {@link relayMutationsToHub} (which start at 0 and only increase) can never reach, so the two
 * id spaces never collide. */
let nextLocalOverflowBatchId = -1;

/** 🚫️ Emits one local refusal — answered before any queue, socket frame or retry exists — in the one
 * {@link CommandAckOutcome} rejection contract a hub refusal also arrives in. */
export function rejectLocally(state: ArtifactState, outcome: CommandAckOutcome): void {
  const batchId = nextLocalOverflowBatchId;
  nextLocalOverflowBatchId -= 1;
  if (outcome.kind === "rejected") console.error("[backbone-worker] refused a local batch", state.config.documentId, outcome.code, outcome.detail ?? {});
  emitEvent(state, { kind: "commandOutcome", batchId, outcome });
}

/** 🚫️ Terminal local rejection for a verified viewer-only execution target. */
export function rejectReadOnlyExecutionTarget(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  rejectLocally(state, localCommandRejectionV1("local.read-only", "execution target is read-only", { envelopes: envelopes.length }));
}

export function rejectMutationQueueOverflow(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  rejectLocally(state, localCommandRejectionV1("local.queue-full", "pending mutation queue full", { envelopes: envelopes.length, limit: PENDING_MUTATIONS_QUEUE_LIMIT }));
}

export function rejectDocumentBackboneCapacity(state: ArtifactState, envelopes: readonly MutationEnvelope[], bytes: number): void {
  rejectLocally(state, localCommandRejectionV1("local.backbone-capacity", "document backbone pending capacity", { envelopes: envelopes.length, bytes, limit: DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes }));
}

export function rejectDocumentBackboneBootstrapAdmission(state: ArtifactState, bytes: number): void {
  rejectLocally(state, localCommandRejectionV1("local.backbone-pair-unavailable", "document backbone canonical pair unavailable", { bytes }));
}

export function releaseDocumentBackboneOwnership(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  let releasedBytes = 0;
  let releasedMessages = 0;
  for (const envelope of envelopes) {
    const retained = state.exactLocalEnvelopes.get(envelope);
    if (retained === undefined) continue;
    releasedBytes += retained.bytes;
    releasedMessages += retained.messages;
    state.exactLocalEnvelopes.delete(envelope);
  }
  if (releasedBytes > state.pendingDocumentBackboneBytes || releasedMessages > state.pendingDocumentBackboneMessages) throw new Error("document backbone: retained byte ledger underflow");
  state.pendingDocumentBackboneBytes -= releasedBytes;
  state.pendingDocumentBackboneMessages -= releasedMessages;
}

/** 🔙️ What a batch the hub did not take as sent corrects: each operation rolls back by its own inverse, newest first, and
 * each history transition — which no inverse rolls back — is retracted from the store by its exact id (the store drops
 * every local transition depending on it too). The TS twin of the Rust actor's `rollback_envelope` and
 * `retract_refused_transitions`. */
export function refusedBatchCorrection(state: ArtifactState, sent: readonly MutationEnvelope[]): Readonly<{ rollbacks: readonly MutationEnvelope[]; retracted: readonly string[] }> {
  const transition = (envelope: MutationEnvelope) => envelope.diff.schemaId === HISTORY_TRANSITION_DIFF_SCHEMA;
  return {
    rollbacks: [...sent].reverse().filter((envelope) => !transition(envelope)).map(rollbackEnvelope),
    retracted: sent.filter(transition).map((envelope) => state.exactLocalEnvelopes.get(envelope)?.envelope.mutation_id ?? envelope.id),
  };
}

export function equalByteArrays(left: ArrayLike<number>, right: ArrayLike<number>): boolean {
  if (left.length !== right.length) return false;
  for (let index = 0; index < left.length; index++) if (left[index] !== right[index]) return false;
  return true;
}

export function equalFrontiers(left: WireFrontierSummary, right: WireFrontierSummary): boolean {
  return left.document_id === right.document_id && left.head_edit_ordinal === right.head_edit_ordinal && left.head_edit_id === right.head_edit_id && left.last_commit_seq === right.last_commit_seq && equalByteArrays(left.chain_hash, right.chain_hash);
}

export function queueOutbox(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  const queued = new Set(state.outbox.map((envelope) => envelope.id));
  for (const envelope of envelopes) {
    if (!queued.has(envelope.id)) {
      queued.add(envelope.id);
      state.outbox.push(envelope);
    }
  }
}

export function requeuePendingBatches(state: ArtifactState): void {
  const batches = [...state.pendingBatches.entries()].sort(([left], [right]) => left - right);
  state.pendingBatches.clear();
  returnToOutboxFront(state, batches.flatMap(([, envelopes]) => envelopes));
}
