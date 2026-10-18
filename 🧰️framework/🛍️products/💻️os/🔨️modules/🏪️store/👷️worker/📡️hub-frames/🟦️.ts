import type { ServerFrame } from "@semio-tech/framework-replication";
import type { DocumentScope } from "../../../../🟦️";
import { admitRemoteEnvelopes, encodeBackboneMessage } from "../../../../🟦️";
import { encodeDocumentBackboneEnvelopeBatchExact, decodeDocumentBackboneEnvelopeBatchExact, decodePresencePeer } from "@semio-tech/framework-replication";
import { closeHubSocketV1 } from "@semio-tech/framework/directory/client/socket-close";
import { type ArtifactState, artifactScope, emitEvent, post } from "../🧾️document-state/🟦️.ts";
import { flushMutationsToHubIfReady, requeuePendingBatches } from "../📮️hub-outbox/🟦️.ts";
import { documentAwaitsBrowserActor, retainBackboneBeforeBrowserActor, setRemote } from "../🎭️actor-lifecycle/🟦️.ts";
import { abortArtifactBootstrap, abortArtifactRebootstrap, clearArtifactRebootstrapDeadline } from "../🛑️session-teardown/🟦️.ts";
import { bootstrapControl, finishCatchupIfReady, installArtifactBootstrap, rejectArtifactBootstrap, requireArtifactRebootstrap, startArtifactBootstrap } from "../🚀️artifact-bootstrap/🟦️.ts";
import { retireCurrentFolderCanonicalBootstrapMirror } from "../📁️folder/🟦️.ts";
import { hubBinding } from "../🧩️actor-wire-helpers/🟦️.ts";
import { handleAck, settleCommittedEnvelopes } from "../🧮️hub-ack/🟦️.ts";
import { wireEnvelopeId } from "../🌉️wire-bridge/🟦️.ts";
import { dropDocumentExecutionTargetLease, requestDocumentActorRecoveryV1 } from "../📥️execution-target-read/🟦️.ts";
import { activateDocumentBrowserActorAfterSession } from "../🎬️actor-activation/🟦️.ts";

export async function handleHubFrame(
  state: ArtifactState,
  frame: ServerFrame,
  presenceCandidate: Readonly<{ socket: WebSocket; scope: DocumentScope; verifiedSurfaceId: string }> | null = null,
  sourceSocket: WebSocket | null = null,
  commandBatch: Uint8Array | null = null,
): Promise<void> {
  if (sourceSocket !== null && state.socket !== sourceSocket) return;
  if (typeof frame === "string") return; // no unit-variant `ServerFrame` exists today; defensive.
  if ("Welcome" in frame) {
    requeuePendingBatches(state);
    const bootstrap = frame.Welcome.bootstrap;
    if ((bootstrap === "None" || bootstrap === "Tail") && state.artifactRebootstrapRequired && documentAwaitsBrowserActor(state)) abortArtifactRebootstrap(state);
    if (bootstrap === "None") {
      if (state.artifactRebootstrapRequired) {
        clearArtifactRebootstrapDeadline(state, state.artifactRebootstrapOwner);
        rejectArtifactBootstrap(state, new Error("artifact rebootstrap returned no canonical pair"));
        return;
      }
      await retireCurrentFolderCanonicalBootstrapMirror(state);
      abortArtifactBootstrap(state);
      state.resumeToken = frame.Welcome.resume_token;
      state.frontier = frame.Welcome.server_frontier;
      setRemote(state, { kind: "live", peerCount: 0 });
      flushMutationsToHubIfReady(state);
      return;
    }
    if (bootstrap === "Tail") {
      if (state.artifactRebootstrapRequired) {
        clearArtifactRebootstrapDeadline(state, state.artifactRebootstrapOwner);
        rejectArtifactBootstrap(state, new Error("artifact rebootstrap returned tail without a canonical pair"));
        return;
      }
      await retireCurrentFolderCanonicalBootstrapMirror(state);
      abortArtifactBootstrap(state);
      state.pendingResumeToken = frame.Welcome.resume_token;
      state.requiredTailFrontier = frame.Welcome.server_frontier;
      finishCatchupIfReady(state);
      return;
    }
    if ("Snapshot" in bootstrap) {
      rejectArtifactBootstrap(state, new Error("database-private snapshot cannot seed an artifact client"));
      return;
    }
    try {
      await startArtifactBootstrap(state, bootstrap.ArtifactBootstrap, frame.Welcome.resume_token, frame.Welcome.server_frontier);
    } catch (error) {
      rejectArtifactBootstrap(state, error);
    }
    return;
  }
  if ("SnapshotChunk" in frame || "SnapshotDone" in frame) {
    rejectArtifactBootstrap(state, new Error("database-private snapshot frame cannot seed an artifact client"));
    return;
  }
  if ("RebootstrapRequired" in frame) {
    const binding = hubBinding(state.config);
    const control = frame.RebootstrapRequired.control;
    if (!binding || control.space_id !== binding.spaceId || control.document_id !== state.config.documentId || control.baseline_frontier.document_id !== state.config.documentId) {
      rejectArtifactBootstrap(state, new Error("rebootstrap control scope mismatch"));
    } else {
      await requireArtifactRebootstrap(state);
    }
    return;
  }
  if ("ArtifactBootstrapChunk" in frame) {
    const assembler = state.artifactBootstrap;
    if (!assembler) {
      rejectArtifactBootstrap(state, new Error("artifact bootstrap chunk arrived without an active transfer"));
      return;
    }
    const owner = state.artifactBootstrapOwner;
    try {
      if (!owner || owner.assembler !== assembler) throw new Error("artifact bootstrap owner mismatch");
      assembler.push(frame.ArtifactBootstrapChunk, bootstrapControl(state, owner));
      owner.assertCurrent();
    } catch (error) {
      rejectArtifactBootstrap(state, error, owner);
    }
    return;
  }
  if ("ArtifactBootstrapDone" in frame) {
    const assembler = state.artifactBootstrap;
    if (!assembler) {
      rejectArtifactBootstrap(state, new Error("artifact bootstrap completion arrived without an active transfer"));
      return;
    }
    const owner = state.artifactBootstrapOwner;
    try {
      if (!owner || owner.assembler !== assembler) throw new Error("artifact bootstrap owner mismatch");
      await installArtifactBootstrap(state, owner, frame.ArtifactBootstrapDone);
    } catch (error) {
      rejectArtifactBootstrap(state, error, owner);
    }
    return;
  }
  if ("Commands" in frame) {
    if (state.artifactRebootstrapRequired) return;
    if (state.artifactBootstrap) {
      rejectArtifactBootstrap(state, new Error("tail arrived before artifact bootstrap completion"));
      return;
    }
    settleCommittedEnvelopes(state, frame.Commands.envelopes.map(wireEnvelopeId));
    const fresh = admitRemoteEnvelopes(state.ingestedMutationIds, frame.Commands.envelopes, wireEnvelopeId);
    if (fresh.length > 0 && commandBatch === null) throw new Error("document backbone: exact server command batch missing");
    if (fresh.length > 0 && commandBatch !== null) {
      if (state.pendingMutations.length > 0) state.remoteFoldedOverLocal = true;
      const admitted = new Set(fresh.map(wireEnvelopeId)),
        batch = fresh.length === frame.Commands.envelopes.length ? commandBatch : encodeDocumentBackboneEnvelopeBatchExact(decodeDocumentBackboneEnvelopeBatchExact(commandBatch).filter((envelope) => admitted.has(envelope.mutation_id))),
        message = encodeBackboneMessage({ kind: "mutations", envelopes: batch }),
        reservation = state.browserActorReservation;
      if (reservation === null && documentAwaitsBrowserActor(state)) retainBackboneBeforeBrowserActor(state, message);
      else if (reservation === null) emitEvent(state, { kind: "documentBackbone", message });
      else {
        try {
          await reservation.receiveBackbone(message);
        } catch (error) {
          console.error("[backbone-worker] the browser actor child refused an inbound hub frame; reopening from the last confirmed state", state.config.documentId, error);
          requestDocumentActorRecoveryV1(state, "inbound-frame");
          return;
        }
      }
    }
    state.frontier = frame.Commands.frontier;
    finishCatchupIfReady(state);
    return;
  }
  if ("Ack" in frame) {
    if (state.artifactRebootstrapRequired) return;
    if (state.artifactBootstrap || state.requiredTailFrontier) {
      rejectArtifactBootstrap(state, new Error("ack arrived before artifact catch-up completion"));
      return;
    }
    state.frontier = frame.Ack.frontier;
    await handleAck(state, frame.Ack.batch_id, frame.Ack.stages);
    return;
  }
  if ("Preview" in frame) {
    if (frame.Preview.actor !== state.actor) emitEvent(state, { kind: "preview", actor: frame.Preview.actor, key: frame.Preview.key, seq: frame.Preview.seq, payload: frame.Preview.payload });
    return;
  }
  if ("Presence" in frame) {
    // 📡️ `ServerFrame::Presence.peers` is `Vec<Vec<u8>>` of `encode_presence_peer` blobs on the wire
    // (real binary — see `decodePresencePeer`'s doc). A malformed entry is dropped rather than
    // failing the whole roster, mirroring the Rust actor's `presence_from_bytes` (`Option`-returning,
    // never panics on a bad peer) at the same trust boundary.
    const peers = frame.Presence.peers.flatMap((bytes) => {
      try {
        return [decodePresencePeer(new Uint8Array(bytes), [0])];
      } catch {
        return [];
      }
    });
    if (state.status.remote.kind === "live" && state.artifactBootstrap === null && state.requiredTailFrontier === null) {
      setRemote(state, { kind: "live", peerCount: peers.length });
    }
    emitEvent(state, { kind: "presence", peers });
    return;
  }
  if ("CreditGrant" in frame) {
    // 🪙️ Command-lane credit-based flow control: no client-side backpressure implemented this wave
    // (scope is frame plumbing, not congestion control) — accepted and ignored.
    return;
  }
  if ("Session" in frame) {
    // 🎨️ Flows through the SAME generic `{kind:"event",...}` wrapping every other `ArtifactEvent`
    // gets — the real wasm host (`👷️worker/🦀️.rs`) wraps every `ArtifactEvent` uniformly
    // with zero per-variant special-casing, so this fallback must match rather than post a
    // one-off top-level `BackboneWorkerResponse` shape the wasm path would never produce.
    const expectedActor = state.pendingSocketActorId;
    if (expectedActor === null || frame.Session.actor !== expectedActor) {
      state.actor = "";
      state.hubActorReady = false;
      state.pendingSocketActorId = null;
      state.presenceAuthority = null;
      const scope = artifactScope(state);
      post({ kind: "socket-actor-failed", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), code: "session-mismatch" });
      dropDocumentExecutionTargetLease(state);
      if (state.socket) closeHubSocketV1(state.socket, "actorMismatch");
      return;
    }
    state.actor = expectedActor;
    state.hubActorReady = true;
    state.pendingSocketActorId = null;
    state.sessionColor = frame.Session.color;
    state.presenceAuthority = presenceCandidate?.socket === state.socket ? presenceCandidate : null;
    flushMutationsToHubIfReady(state);
    const scope = artifactScope(state);
    post({ kind: "socket-actor", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), actorId: expectedActor });
    emitEvent(state, { kind: "session", actor: frame.Session.actor, color: frame.Session.color });
    if (sourceSocket && state.executionTargetLease) void activateDocumentBrowserActorAfterSession(state, sourceSocket);
    return;
  }
  if ("Error" in frame) {
    emitEvent(state, { kind: "conflict", message: frame.Error.message });
  }
}
