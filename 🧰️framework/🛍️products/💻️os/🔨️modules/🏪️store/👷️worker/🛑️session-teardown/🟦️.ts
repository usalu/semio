import type { ArtifactBootstrapWorkerEvent } from "../../../../🟦️";
import { type ArtifactState, artifactScope, artifacts, documentServices } from "../🧾️document-state/🟦️.ts";
import type { DocumentArtifactBootstrapOwner, DocumentArtifactRebootstrapOwner } from "../🚀️artifact-bootstrap/🟦️.ts";
import { retireFolderCanonicalBootstrapMirror } from "../🌉️wire-bridge/🟦️.ts";
import { ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES } from "../⚙️constants/🟦️.ts";
import type { DirectoryCommandTransportOperationV1, SpaceArtifactCreationOperationV1 } from "../📇️space-artifact-creation/🟦️.ts";
import { dropDocumentExecutionTargetLease } from "../📥️execution-target-read/🟦️.ts";

export function clearArtifactRebootstrapDeadline(state: ArtifactState, owner: DocumentArtifactRebootstrapOwner | null): void {
  if (!owner) return;
  clearTimeout(owner.timer);
  if (state.artifactRebootstrapOwner === owner) {
    state.artifactRebootstrapOwner = null;
    state.artifactRebootstrapDeadlineMs = null;
    state.artifactRebootstrapDeadlineTimer = null;
  }
}

export function abortArtifactRebootstrap(state: ArtifactState): void {
  clearArtifactRebootstrapDeadline(state, state.artifactRebootstrapOwner);
  state.artifactRebootstrapRequired = false;
}

export function clearArtifactBootstrapDeadline(state: ArtifactState, owner: DocumentArtifactBootstrapOwner | null): void {
  const timer = owner?.deadlineTimer ?? null;
  if (timer !== null) clearTimeout(timer);
  if (owner) {
    owner.deadlineTimer = null;
    owner.deadlineMs = null;
  }
  if (state.artifactBootstrapOwner === owner) {
    state.artifactBootstrapDeadlineTimer = null;
    state.artifactBootstrapDeadlineMs = null;
  }
}

export function abortArtifactBootstrap(state: ArtifactState): void {
  const owner = state.artifactBootstrapOwner;
  if (owner) clearArtifactBootstrapDeadline(state, owner);
  else if (state.artifactBootstrapDeadlineTimer !== null) clearTimeout(state.artifactBootstrapDeadlineTimer);
  const bootstrap = state.artifactBootstrap;
  if (bootstrap && typeof bootstrap.abort === "function") bootstrap.abort();
  state.artifactBootstrap = null;
  state.artifactBootstrapOwner = null;
  state.artifactBootstrapDeadlineMs = null;
  state.artifactBootstrapDeadlineTimer = null;
  state.pendingResumeToken = null;
  state.requiredTailFrontier = null;
  if (owner?.folderMirror && state.canonicalFolderMirror !== owner.folderMirror) void retireFolderCanonicalBootstrapMirror(owner.folderMirror);
}

function boundedBootstrapDiagnostic(error: unknown): string {
  const message = error instanceof Error ? error.message : String(error);
  if (new TextEncoder().encode(message).byteLength <= ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES) return message;
  let end = message.length;
  while (end > 0 && new TextEncoder().encode(message.slice(0, end)).byteLength > ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES - 3) end -= 1;
  return `${message.slice(0, end)}...`;
}

export function artifactBootstrapFailure(state: ArtifactState, error: unknown): Extract<ArtifactBootstrapWorkerEvent, { readonly kind: "artifact-bootstrap-failed" }> {
  const message = boundedBootstrapDiagnostic(error);
  const normalized = message.toLowerCase();
  const cancelled = state.closed || state.docAbort.signal.aborted || normalized.includes("cancel");
  const deadline = normalized.includes("deadline") || normalized.includes("timed out") || normalized.includes("timeout");
  const invalid =
    normalized.includes("snapshot") ||
    normalized.includes("schema") ||
    normalized.includes("digest") ||
    normalized.includes("descriptor") ||
    normalized.includes("canonical pair") ||
    normalized.includes("scope") ||
    normalized.includes("chunk") ||
    normalized.includes("frontier") ||
    normalized.includes("without an active transfer") ||
    normalized.includes("before artifact");
  const code = cancelled ? "cancelled" : deadline ? "deadline-exceeded" : invalid ? "invalid-bootstrap" : "transport-failure";
  const scope = artifactScope(state);
  return { kind: "artifact-bootstrap-failed", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), code, message, retryable: code !== "invalid-bootstrap" };
}
export const scopedDirectoryStreams = new Map<string, { close: () => void }>();
export const directoryCommandOperations = new Map<string, DirectoryCommandTransportOperationV1>();
export const directoryCommandQueue: DirectoryCommandTransportOperationV1[] = [];

export const spaceArtifactCreationOperations = new Map<string, SpaceArtifactCreationOperationV1>();
export const spaceArtifactCreationCatalogOperations = new Map<string, Readonly<{ abort: AbortController; clientInstanceId: string; workerEpoch: number }>>();

export function closeArtifactRuntime(runtimeKey: string): void {
  const state = artifacts.get(runtimeKey);
  if (!state) return;
  state.closed = true;
  state.browserActorViewState = null;
  abortArtifactRebootstrap(state);
  abortArtifactBootstrap(state);
  // 🛑️ Finding 3: cancels every in-flight folder/blob fetch this document owns and unblocks any
  // pending reconnect backoff delay immediately — no fetch or reconnect loop can pin this document
  // after this line.
  state.docAbort.abort();
  // 💡️ The inference port exists only while this document's lease does — a close retires it with a
  // localized terminal before the lease buffers are wiped.
  documentServices.documentClosed(runtimeKey);
  dropDocumentExecutionTargetLease(state);
  state.socket?.close();
  if (state.sanityPollTimer != null) clearTimeout(state.sanityPollTimer);
  if (state.transientRefusal?.timer != null) clearTimeout(state.transientRefusal.timer);
  state.transientRefusal = null;
  state.channel.close();
  state.exactLocalEnvelopes = new WeakMap();
  state.pendingDocumentBackboneBytes = 0;
  state.pendingDocumentBackboneMessages = 0;
  artifacts.delete(runtimeKey);
}
