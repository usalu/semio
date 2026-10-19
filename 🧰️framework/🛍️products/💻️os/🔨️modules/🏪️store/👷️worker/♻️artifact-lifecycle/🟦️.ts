import type { ArtifactActorConfig, ArtifactActorMsg } from "../../../../🟦️";
import { HUB_RECONNECT_MIN_MS, noteAuthoredEnvelopeIds, parseDocumentBackboneMessage, localCommandRejectionV1 } from "../../../../🟦️";
import { DOCUMENT_ACTOR_RECOVERY_FRESH_V1 } from "../🚑️actor-recovery/🟦️.ts";
import type { MutationEnvelope, ExactWireMutationEnvelope } from "@semio-tech/framework-replication";
import { DOCUMENT_BACKBONE_RETENTION_LIMITS, encodePresencePeer } from "@semio-tech/framework-replication";
import { type ArtifactState, artifactRuntimeKey, artifacts } from "../🧾️document-state/🟦️.ts";
import { hubBinding } from "../🧩️actor-wire-helpers/🟦️.ts";
import { closeArtifactRuntime } from "../🛑️session-teardown/🟦️.ts";
import { documentBackboneAdmissionReady, rejectDocumentBackboneBootstrapAdmission, rejectDocumentBackboneCapacity, rejectLocally, rejectMutationQueueOverflow, rejectReadOnlyExecutionTarget, releaseDocumentBackboneOwnership } from "../📮️hub-outbox/🟦️.ts";
import { PENDING_MUTATIONS_QUEUE_LIMIT } from "../⚙️constants/🟦️.ts";
import { folderBinding, setStatus } from "../🎭️actor-lifecycle/🟦️.ts";
import { relayMutationsToHub, sendWireFrame } from "../🛰️hub-relay/🟦️.ts";
import { opaqueEnvelopeFromWire, stampSession } from "../🌉️wire-bridge/🟦️.ts";
import { writeFolder } from "../📁️folder/🟦️.ts";

/** 🌱️ A just-opened document's runtime state: nothing linked, nothing pending, no pair, no lease. The ONE constructor —
 * {@link openArtifact} and every law's hand-built state go through it, so a new field is never missing from a test state.
 * `revalidateFolder` is a no-op until {@link openArtifact} binds a folder. */
export function newArtifactState(config: ArtifactActorConfig, runtimeKey: string, channel: BroadcastChannel, openClientInstanceId: string): ArtifactState {
  const hub = hubBinding(config);
  return {
    runtimeKey,
    config,
    openClientInstanceId,
    actor: hub === null ? config.actor : "",
    hubActorReady: hub === null,
    pendingSocketActorId: null,
    channel,
    socket: null,
    presenceAuthority: null,
    docAbort: new AbortController(),
    executionTargetOpen: null,
    executionTargetLease: null,
    browserActorReservation: null,
    browserActorBackboneBeforeReservation: [],
    remoteFoldedOverLocal: false,
    link: { kind: "linked" },
    linkShortageTimer: null,
    transientRefusal: null,
    browserActorViewState: null,
    sanityPollTimer: null,
    watchHealthy: false,
    revalidateFolder: async () => {},
    folderArchive: null,
    folderBootstrapped: false,
        facetValue: undefined,
    facetBacklog: [],
reconnectDelayMs: HUB_RECONNECT_MIN_MS,
    outbox: [],
    pendingMutations: [],
    exactLocalEnvelopes: new WeakMap(),
    pendingDocumentBackboneBytes: 0,
    pendingDocumentBackboneMessages: 0,
    status: { persisted: false, pendingMutations: 0, remote: { kind: "detached" } },
    frontier: null,
    pendingResumeToken: null,
    requiredTailFrontier: null,
    artifactBootstrap: null,
    artifactBootstrapOwner: null,
    artifactBootstrapDeadlineMs: null,
    artifactBootstrapDeadlineTimer: null,
    artifactRebootstrapOwner: null,
    artifactRebootstrapDeadlineMs: null,
    artifactRebootstrapDeadlineTimer: null,
    artifactRebootstrapRequired: false,
    actorRecovery: DOCUMENT_ACTOR_RECOVERY_FRESH_V1,
    actorRecoveryRequested: false,
    artifactBootstrapProgress: [],
    canonicalFolderMirror: null,
    verifiedColdPair: null,
    currentPack: null,
    currentSpr: null,
    hubFrameChain: Promise.resolve(),
    resumeToken: null,
    sessionColor: null,
    pendingBatches: new Map(),
    nextBatchId: 0,
    ingestedMutationIds: new Set(),
    hlcCounter: 0,
    closed: false,
  };
}

export function retireArtifactBeforeReplacement(runtimeKey: string): void {
  if (!artifacts.has(runtimeKey)) return;
  closeArtifactRuntime(runtimeKey);
  if (artifacts.has(runtimeKey)) throw new Error("artifact replacement: prior owner did not retire");
}

export function closeArtifact(documentId: string, spaceId?: string, clientInstanceId?: string): void {
  const runtimeKey = artifactRuntimeKey(documentId, spaceId);
  if (runtimeKey === null) return;
  const state = artifacts.get(runtimeKey);
  if (state !== undefined && clientInstanceId !== undefined && state.openClientInstanceId !== clientInstanceId) return;
  closeArtifactRuntime(runtimeKey);
}

function admitLocalMutations(
  state: ArtifactState,
  envelopes: readonly MutationEnvelope[],
  channelMessage: unknown,
  exactEnvelopes: readonly ExactWireMutationEnvelope[] | null = null,
  exactMessageBytes = 0,
): boolean {
  if (envelopes.length === 0) return false;
  if (exactEnvelopes !== null && exactEnvelopes.length !== envelopes.length) throw new Error("document backbone: exact batch cardinality mismatch");
  if (state.executionTargetLease !== null && state.executionTargetLease.live && !state.executionTargetLease.fields().grant.write) {
    rejectReadOnlyExecutionTarget(state, envelopes);
    return false;
  }
  if (exactEnvelopes !== null && (!Number.isSafeInteger(exactMessageBytes) || exactMessageBytes < 1 || exactMessageBytes > DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes)) throw new Error("document backbone: invalid retained byte charge");
  if (exactEnvelopes !== null && (state.pendingDocumentBackboneMessages >= DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumMessages || exactMessageBytes > DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes - state.pendingDocumentBackboneBytes)) {
    rejectDocumentBackboneCapacity(state, envelopes, exactMessageBytes);
    return false;
  }
  if (state.pendingMutations.length + envelopes.length > PENDING_MUTATIONS_QUEUE_LIMIT) {
    rejectMutationQueueOverflow(state, envelopes);
    return false;
  }
  if (exactEnvelopes !== null) {
    envelopes.forEach((envelope, index) => state.exactLocalEnvelopes.set(envelope, { envelope: exactEnvelopes[index]!, bytes: index === 0 ? exactMessageBytes : 0, messages: index === 0 ? 1 : 0 }));
    state.pendingDocumentBackboneBytes += exactMessageBytes;
    state.pendingDocumentBackboneMessages += 1;
  }
  noteAuthoredEnvelopeIds(state.ingestedMutationIds, envelopes.map((envelope, index) => exactEnvelopes?.[index]?.mutation_id ?? envelope.id));
  state.pendingMutations.push(...envelopes);
  setStatus(state, { pendingMutations: state.pendingMutations.length });
  if (exactEnvelopes === null || !hubBinding(state.config)) state.channel.postMessage(channelMessage);
  relayMutationsToHub(state, envelopes);
  if (folderBinding(state.config)) setStatus(state, { persisted: false });
  return true;
}

export async function handleLocalMsg(state: ArtifactState, message: ArtifactActorMsg): Promise<void> {
  switch (message.kind) {
    case "documentBackbone": {
      if (hubBinding(state.config) !== null && !documentBackboneAdmissionReady(state)) {
        rejectDocumentBackboneBootstrapAdmission(state, message.message.byteLength);
        break;
      }
      const parsed = parseDocumentBackboneMessage(message.message);
      if (parsed.envelopes.some((envelope) => envelope.document_id !== state.config.documentId)) {
        rejectLocally(state, localCommandRejectionV1("local.backbone-scope-mismatch", "document backbone scope mismatch", { envelopes: parsed.envelopes.length }));
        break;
      }
      admitLocalMutations(state, parsed.envelopes.map(opaqueEnvelopeFromWire), { kind: "documentBackbone", message: parsed.message }, parsed.envelopes, parsed.message.byteLength);
      break;
    }
    case "localMutations": {
      if (hubBinding(state.config) && !documentBackboneAdmissionReady(state)) {
        rejectDocumentBackboneBootstrapAdmission(state, 0);
        break;
      }
      if (admitLocalMutations(state, message.envelopes, message.envelopes)) foldConfigFacet(state, message.envelopes);
      break;
    }
    case "localDocumentArchive": {
            if (CONFIG_FACETS.has(state.config.schema)) {
        rejectLocally(state, localCommandRejectionV1("local.backbone-malformed", "a config facet changes by mutation, never by archive put", { bytes: message.archive.length }));
        break;
      }
const folder = folderBinding(state.config);
      if (folder) {
        try {
          await writeFolder(state, folder, message.archive);
          releaseDocumentBackboneOwnership(state, state.pendingMutations);
          state.pendingMutations = [];
          setStatus(state, { pendingMutations: 0 });
        } catch (error) {
          console.error("[backbone-worker] folder write failed", state.config.documentId, error);
        }
      }
      // 📸️ No client -> hub whole-envelope push exists in wire v2 (`ClientFrame` has no snapshot-put
      // variant, only causally-ordered `Commands`) — mirrors the Rust actor's identical deferral
      // (`framework/sync/rs/lib.rs` `drain_and_relay`'s `BackboneMessage::Snapshot` arm) rather than
      // a bug here; the folder write above still persists it.
      break;
    }
    case "presenceHeartbeat":
      sendWireFrame(state, { Presence: { peer: encodePresencePeer(stampSession(message.peer, state)) } }, "preview");
      break;
    case "publishPreview":
      sendWireFrame(state, { PreviewPublish: { key: message.key, seq: message.seq, payload: message.payload } }, "preview");
      break;
    case "externalChanged": {
      // 🥇️ Routed through the same single-flight guard as the SSE wake / sanity poll (finding 1).
      if (folderBinding(state.config)) void state.revalidateFolder();
      break;
    }
    case "detach":
      closeArtifactRuntime(state.runtimeKey);
      break;
  }
}
