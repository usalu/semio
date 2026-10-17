/** 🎭️ Artifact actor wire: the actor configuration, messages and events with their wire forms and parsers. */

import { type RetainedCloneGrant } from "@semio-tech/value-ts/retained-clone/wire";
import type { ArtifactFrontier } from "../../📄️document/🪪️scope/🟦️.ts";
import type { ArtifactPresencePeer, MutationEnvelope } from "@semio-tech/framework-replication";
import { decodeCausalEnvelopeBatch, encodeCausalEnvelopeBatch } from "@semio-tech/framework-replication";
import type { DocumentScope } from "../../📇️directory/🟦️.ts";
import { replicationPackCodec } from "../../🎁️packs/🟦️.ts";
import { BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES, type CommandAckOutcome, commandAckOutcomeOfValueV1, commandBatchIdOfValueV1, decodeDocumentArchiveBytes, DOCUMENT_ARCHIVE_MAXIMUM_BYTES } from "../../🪢️wire-contracts/🟦️.ts";
import { parseDocumentBackboneMessage, parseInboundDocumentBackboneMessage } from "../🔀️message/🟦️.ts";
import type { PersistenceBinding } from "../🔑️persistence-binding/🟦️.ts";

//#region 🔖️BackboneWorkerProtocol
/** 🧾️ Everything the worker needs to open one artifact's actor — mirrors `ArtifactActorConfig`. */
export type ArtifactActorConfig = {
  readonly documentId: string;
  readonly schema: string;
  readonly bindings: readonly PersistenceBinding[];
  readonly watchExternal?: boolean;
  readonly actor: string;
  readonly actorIdentityGrant: RetainedCloneGrant;
  /** 🧬️ W5.7: this document kind's `store::DocumentCodec.pack_schema_hash`, for hub schema-hash
   * validation (`ClientFrame::SocketHelloV1.pack_schema_hash`) — the shell fills this from the wasm
   * renderer's `document_pack_schema_hash(schema)` export before calling `openArtifact`. Omitted
   * (or all-zero) means "schema-agnostic client", which the hub never validates. */
  readonly packSchemaHash?: readonly number[];
};

/** 📨️ Caller→actor control messages — mirrors Rust `ArtifactActorMsg`. */
export type ArtifactActorMsg =
  | { readonly kind: "localMutations"; readonly envelopes: readonly MutationEnvelope[] }
  | { readonly kind: "documentBackbone"; readonly message: Uint8Array }
  | { readonly kind: "localDocumentArchive"; readonly archive: readonly number[] }
  | { readonly kind: "presenceHeartbeat"; readonly peer: ArtifactPresencePeer }
  | { readonly kind: "publishPreview"; readonly key: string; readonly seq: number; readonly payload: readonly number[] }
  | { readonly kind: "externalChanged" }
  | { readonly kind: "detach" };

/** 📶️ Connection state of a document's remote (hub) transport — mirrors Rust `RemoteState`. */
export type RemoteState = { readonly kind: "detached" } | { readonly kind: "connecting" } | { readonly kind: "live"; readonly peerCount: number } | { readonly kind: "backoff"; readonly retryInMs: number };

/** 🚦️ Sync health snapshot for status badges — mirrors Rust `ArtifactSyncStatus`. */
export type ArtifactSyncStatus = {
  readonly persisted: boolean;
  readonly pendingMutations: number;
  readonly remote: RemoteState;
};

/** ⚠️ A structural sync conflict — `ArtifactEvent::Conflict` wraps a `MutationMessage` (contract
 * freeze `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C10's frozen
 * diagnostic-bag vocabulary, replacing the deleted transport-level conflict bag it used to carry),
 * loosely typed here rather than importing the full shape — the shell only needs enough to render a
 * conflict card / offer "fork alternative" vs "take theirs", and `message` alone already covers that. */
export type SyncConflict = { readonly message?: string } & Record<string, unknown>;

/** 📬️ Actor→subscriber events — mirrors Rust `ArtifactEvent`. */
export type ArtifactEvent =
  | { readonly kind: "remoteMutations"; readonly envelopes: readonly MutationEnvelope[] }
  | { readonly kind: "documentBackbone"; readonly message: Uint8Array }
  | { readonly kind: "documentArchiveReplaced"; readonly archive: readonly number[] }
  /** 🫙️ The bound folder holds no archive of this document: once per open, from the bootstrap folder read alone, never
   * after an archive was read or written — mirrors Rust `ArtifactEvent::DocumentArchiveAbsent`. */
  | { readonly kind: "documentArchiveAbsent" }
  | ({ readonly kind: "status" } & ArtifactSyncStatus)
  | { readonly kind: "presence"; readonly peers: readonly ArtifactPresencePeer[] }
  /** 🎨️ The hub's one-time session color assignment (`ServerFrame::Session`) — mirrors Rust
   * `ArtifactEvent::Session`. Flows through the SAME generic `{kind:"event",documentId,event}`
   * wrapping every other `ArtifactEvent` variant gets (both the real wasm `👷️worker/🦀️.rs`
   * host, which wraps every `ArtifactEvent` uniformly with zero per-variant special-casing, and this
   * file's TS fallback via `emitEvent`), never a separate top-level `BackboneWorkerResponse` member. */
  | { readonly kind: "session"; readonly actor: string; readonly color: number }
  | { readonly kind: "preview"; readonly actor: string; readonly key: string; readonly seq: number; readonly payload: readonly number[] }
  | { readonly kind: "commandOutcome"; readonly batchId: number; readonly outcome: CommandAckOutcome }
  | ({ readonly kind: "conflict" } & SyncConflict);

//#region 🔖️EnvelopeCodec
export type BrowserActorUiMountedV1 = Readonly<{
  kind: "browser-actor-ui-mounted";
  scope: DocumentScope;
  clientInstanceId: string;
  activationGeneration: string;
  instanceId: number;
  verifiedSurfaceId: string;
  /** 🪟️ The lease's window: the one whose acknowledged revision `uiRevision` is. */
  windowKindId: string;
  catalogGenerationId: string;
  componentSha256: string;
  descriptorSha256: string;
  browserActorSha256: string;
  activeCheckpointId: string;
  descriptorDigestV1: string;
  frontier: ArtifactFrontier;
  uiRevision: number;
}>;

export function wireArtifactActorMsg(message: ArtifactActorMsg): unknown {
  if (message.kind === "documentBackbone") {
    return { kind: "documentBackbone", message: Array.from(parseDocumentBackboneMessage(message.message).message) };
  }
  if (message.kind === "localMutations") {
    return { kind: "localMutations", envelopes: encodeCausalEnvelopeBatch(message.envelopes, replicationPackCodec) };
  }
  if (message.kind === "localDocumentArchive") {
    const archive = Uint8Array.from(message.archive);
    decodeDocumentArchiveBytes(archive);
    return { kind: "localDocumentArchive", archive: Array.from(archive) };
  }
  return message;
}

export function parseArtifactActorMsg(message: Record<string, unknown>): ArtifactActorMsg {
  if (message.kind === "documentBackbone") {
    if (Object.keys(message).sort().join(",") !== "kind,message" || !Array.isArray(message.message) || message.message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || !message.message.every((entry) => Number.isInteger(entry) && entry >= 0 && entry <= 255)) throw new Error("backbone worker request: invalid document backbone message");
    return { kind: "documentBackbone", message: parseDocumentBackboneMessage(Uint8Array.from(message.message as readonly number[])).message };
  }
  if (message.kind === "localMutations" && Array.isArray(message.envelopes) && message.envelopes.every((entry) => typeof entry === "number")) {
    return { kind: "localMutations", envelopes: decodeCausalEnvelopeBatch(message.envelopes as readonly number[], replicationPackCodec) };
  }
  if (message.kind === "localDocumentArchive") {
    if (Object.keys(message).sort().join(",") !== "archive,kind" || !Array.isArray(message.archive) || message.archive.length === 0 || message.archive.length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES || !message.archive.every((entry) => Number.isInteger(entry) && entry >= 0 && entry <= 255)) throw new Error("backbone worker request: invalid document archive");
    const archive = Uint8Array.from(message.archive as readonly number[]);
    decodeDocumentArchiveBytes(archive);
    return { kind: "localDocumentArchive", archive: Array.from(archive) };
  }
  return message as ArtifactActorMsg;
}

export function wireArtifactEvent(event: ArtifactEvent): unknown {
  if (event.kind === "documentBackbone") {
    return { kind: "documentBackbone", message: Array.from(parseInboundDocumentBackboneMessage(event.message)) };
  }
  if (event.kind === "remoteMutations") {
    return { kind: "remoteMutations", envelopes: encodeCausalEnvelopeBatch(event.envelopes, replicationPackCodec) };
  }
  if (event.kind === "documentArchiveReplaced") {
    const archive = Uint8Array.from(event.archive);
    decodeDocumentArchiveBytes(archive);
    return { kind: "documentArchiveReplaced", archive: Array.from(archive) };
  }
  return event;
}

export function parseArtifactEvent(event: Record<string, unknown>): ArtifactEvent {
  if (event.kind === "documentBackbone") {
    if (Object.keys(event).sort().join(",") !== "kind,message" || !Array.isArray(event.message) || event.message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || !event.message.every((entry) => Number.isInteger(entry) && entry >= 0 && entry <= 255)) throw new Error("backbone worker response: invalid document backbone message");
    return { kind: "documentBackbone", message: parseInboundDocumentBackboneMessage(Uint8Array.from(event.message as readonly number[])) };
  }
  if (event.kind === "remoteMutations" && Array.isArray(event.envelopes) && event.envelopes.every((entry) => typeof entry === "number")) {
    return { kind: "remoteMutations", envelopes: decodeCausalEnvelopeBatch(event.envelopes as readonly number[], replicationPackCodec) };
  }
  if (event.kind === "documentArchiveReplaced") {
    if (Object.keys(event).sort().join(",") !== "archive,kind" || !Array.isArray(event.archive) || event.archive.length === 0 || event.archive.length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES || !event.archive.every((entry) => Number.isInteger(entry) && entry >= 0 && entry <= 255)) throw new Error("backbone worker response: invalid document archive");
    const archive = Uint8Array.from(event.archive as readonly number[]);
    decodeDocumentArchiveBytes(archive);
    return { kind: "documentArchiveReplaced", archive: Array.from(archive) };
  }
  if (event.kind === "documentArchiveAbsent") {
    if (Object.keys(event).join(",") !== "kind") throw new Error("backbone worker response: invalid document archive absence");
    return { kind: "documentArchiveAbsent" };
  }
  if (event.kind === "commandOutcome") {
    const batchId = commandBatchIdOfValueV1(event.batchId);
    const outcome = commandAckOutcomeOfValueV1(event.outcome);
    if (Object.keys(event).sort().join(",") !== "batchId,kind,outcome" || batchId === null || outcome === null) throw new Error("backbone worker response: invalid command outcome");
    return { kind: "commandOutcome", batchId, outcome };
  }
  return event as ArtifactEvent;
}
//#endregion 🔖️EnvelopeCodec
//#endregion 🔖️BackboneWorkerProtocol
