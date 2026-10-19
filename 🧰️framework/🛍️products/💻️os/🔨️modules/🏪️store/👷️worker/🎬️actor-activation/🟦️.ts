import { documentLinkTransition, HUB_RECONNECT_MIN_MS, HUB_RECONNECT_MAX_MS } from "../../../../🟦️";
import { documentActorRecoveryStepV1 } from "../🚑️actor-recovery/🟦️.ts";
import { closeHubSocketV1 } from "@semio-tech/framework/directory/client/socket-close";
import type { WireFrontierSummary } from "@semio-tech/framework-replication";
import { retryWithJitteredBackoff } from "@semio-tech/framework";
import type { ArtifactState } from "../🧾️document-state/🟦️.ts";
import { DocumentBrowserActorReservation } from "../🧷️actor-reservation/🟦️.ts";
import { documentBrowserActorLease, suspendedDocumentBrowserActor } from "../🎭️actor-lifecycle/🟦️.ts";
import { hubBinding } from "../🧩️actor-wire-helpers/🟦️.ts";
import { flushMutationsToHubIfReady } from "../📮️hub-outbox/🟦️.ts";
import { canonicalCheckpointPairRetryable, finishCatchupIfReady, seedColdPairFromCanonicalCheckpoint } from "../🚀️artifact-bootstrap/🟦️.ts";
import { emitExecutionTargetStatus } from "../📥️execution-target-read/🟦️.ts";
import type { DocumentExecutionTargetLease } from "../🪪️execution-lease/🟦️.ts";

/** 🪪️ Claims a private document slot before reserving worker capacity, with no body fetch or activation. */
export async function reserveDocumentBrowserActorChild(state: ArtifactState): Promise<DocumentBrowserActorReservation | null> {
  const lease = documentBrowserActorLease(state);
  if (lease.fields().browserActor.kind === "none") return null;
  const grant = lease.browserActorGrant();
  if (!grant || Date.now() >= grant.reserveBeforeMs || state.browserActorReservation !== null) throw new Error("document browser actor: admission denied");
  const owner = new DocumentBrowserActorReservation(state, lease, grant);
  state.browserActorReservation = owner;
  try {
    for (const message of state.browserActorBackboneBeforeReservation.splice(0)) await owner.receiveBackbone(message);
    await owner.reserve();
    return owner;
  } catch (error) {
    owner.close();
    if (state.executionTargetLease === lease) state.executionTargetLease = null;
    lease.drop();
    throw error;
  }
}

/** 🚦️ Activates once under the actual accepted socket; a failed exchange owner cannot be reused. */
export async function activateDocumentBrowserActorAfterSession(state: ArtifactState, socket: WebSocket): Promise<void> {
  const lease = state.executionTargetLease,
    attempt = state.executionTargetOpen,
    binding = hubBinding(state.config);
  if (!lease || !binding) return;
  const suspended = suspendedDocumentBrowserActor(state);
  if (suspended !== null && suspended.lease === lease && state.socket === socket && state.hubActorReady && state.actor === lease.browserActorGrant()?.actorId && suspended.reservation.resumeLink(socket)) {
    suspended.pair.resumeLink(socket);
    state.link = documentLinkTransition(state.link, { kind: "restored", nowMs: Date.now() });
    if (state.linkShortageTimer !== null) clearTimeout(state.linkShortageTimer);
    state.linkShortageTimer = null;
    flushMutationsToHubIfReady(state);
    return;
  }
  let owner: DocumentBrowserActorReservation | null = null;
  try {
    if (state.socket !== socket || !state.hubActorReady || state.actor !== lease.browserActorGrant()?.actorId) throw new Error("document browser actor: missing Session");
    if (documentBrowserActorLease(state).fields().browserActor.kind === "none") return;
    if (state.actorRecovery.exhausted) return;
    lease.assertBrowserActorDescribeCapacity();
    owner = await reserveDocumentBrowserActorChild(state);
    if (!owner) throw new Error("document browser actor: reservation unavailable");
    const tailFrontier = state.requiredTailFrontier,
      tailResumeToken = state.pendingResumeToken;
    let seeded = false;
    try {
      seeded = await seedColdPairFromCanonicalCheckpoint(state, tailResumeToken ?? state.resumeToken ?? "");
    } catch (error) {
      if (!canonicalCheckpointPairRetryable(error)) throw error;
      void retryCanonicalCheckpointPair(state, lease, socket, tailResumeToken, tailFrontier);
    }
    if (seeded && tailFrontier !== null) {
      state.pendingResumeToken = tailResumeToken;
      state.requiredTailFrontier = tailFrontier;
      finishCatchupIfReady(state);
    }
    await owner.activate(socket);
    state.actorRecovery = documentActorRecoveryStepV1(state.actorRecovery, { kind: "mounted", atMs: Date.now() }).memory;
  } catch (error) {
    const diagnostic = error instanceof Error ? error.message : String(error);
    owner?.close();
    const current = state.socket === socket && state.executionTargetOpen === attempt && (state.executionTargetLease === lease || state.executionTargetLease === null);
    if (state.executionTargetLease === lease) state.executionTargetLease = null;
    lease.drop();
    if (current && !state.closed) emitExecutionTargetStatus(state, binding, state.docAbort.signal.aborted ? "cancelled" : "integrity-failed", undefined, diagnostic);
    closeHubSocketV1(socket, "actorActivationFailed");
  }
}

/** 🔁️ Repeats a canonical pair request the hub refused transiently (busy, unreachable), with jittered backoff, for as
 * long as the lease and its socket live; the activated child is then seeded through the ordinary post-activation
 * install. The document stays at "verifying" meanwhile — never a child without a document. A final refusal ends the
 * attempt like any integrity failure: the lease drops and the socket closes, so the ordinary reopen runs. */
async function retryCanonicalCheckpointPair(state: ArtifactState, lease: DocumentExecutionTargetLease, socket: WebSocket, tailResumeToken: string | null, tailFrontier: WireFrontierSummary | null): Promise<void> {
  const binding = hubBinding(state.config);
  if (binding === null) return;
  const current = (): boolean => state.executionTargetLease === lease && lease.live && state.socket === socket && !state.closed;
  let outcome: Readonly<{ kind: "seeded" | "retired" } | { kind: "refused"; error: unknown }>;
  try {
    outcome = await retryWithJitteredBackoff(async () => {
      if (!current()) return { kind: "retired" } as const;
      try {
        return { kind: (await seedColdPairFromCanonicalCheckpoint(state, tailResumeToken ?? state.resumeToken ?? "")) ? "seeded" : "retired" } as const;
      } catch (error) {
        if (canonicalCheckpointPairRetryable(error)) throw error;
        return { kind: "refused", error } as const;
      }
    }, { minMs: HUB_RECONNECT_MIN_MS, maxMs: HUB_RECONNECT_MAX_MS, signal: lease.retirement });
  } catch {
    return;
  }
  if (!current()) return;
  if (outcome.kind === "seeded" && tailFrontier !== null) {
    state.pendingResumeToken = tailResumeToken;
    state.requiredTailFrontier = tailFrontier;
    finishCatchupIfReady(state);
  }
  if (outcome.kind !== "refused") return;
  state.browserActorReservation?.close();
  state.executionTargetLease = null;
  lease.drop();
  emitExecutionTargetStatus(state, binding, "integrity-failed", undefined, outcome.error instanceof Error ? outcome.error.message : String(outcome.error));
  closeHubSocketV1(socket, "checkpointPairRefused");
}
