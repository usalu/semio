import type { BackboneWorkerRequest, DocumentScope } from "../../../../🟦️";
import { type DocumentCheckInStatusV1, type DocumentCheckInRefusalV1, DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1, isTerminalDocumentCheckInPhaseV1, DOCUMENT_CHECK_IN_MAX_BYTES, DOCUMENT_CHECK_IN_SCHEMA_V1 } from "@semio-tech/framework/document/check-in";
import type { FetchTimeoutResponse } from "@semio-tech/framework";
import { parseDocumentCheckInStatusV1, documentCheckInCanonicalJson } from "../../../📇️directory/🚪️io/📝️text/📌️document-check-in-v1/🟦️.ts";
import type { WireFrontierSummary } from "@semio-tech/framework-replication";
import { artifactState, post } from "../🧾️document-state/🟦️.ts";
import { type ExecutionTargetReadControl, readBoundedExecutionTargetBody } from "../📥️execution-target-read/🟦️.ts";
import { hubSessionFetch } from "../🔐️hub-session/🟦️.ts";
import { SOCKET_GRANT_REQUEST_TIMEOUT_MS } from "../⚙️constants/🟦️.ts";

/** 🧯️ Concurrent hub Check Ins one worker drives. */
const DOCUMENT_CHECK_IN_CAPACITY = 8;
/** ⏳️ How long a Check In waits for every local edit of its document to be acknowledged. */
const DOCUMENT_CHECK_IN_QUIESCENCE_MS = 15_000;
/** ⏱️ The whole Check In, from request to a terminal status. */
const DOCUMENT_CHECK_IN_DEADLINE_MS = 180_000;
const DOCUMENT_CHECK_IN_POLL_MS = 100;

type DocumentCheckInOperationV1 = {
  readonly request: Extract<BackboneWorkerRequest, { readonly kind: "document-check-in" }>;
  readonly abort: AbortController;
  readonly deadlineAtMs: number;
  cancelRequested: boolean;
  cancelSent: boolean;
  latest: DocumentCheckInStatusV1;
};

const documentCheckInOperations = new Map<string, DocumentCheckInOperationV1>();

function documentCheckInLocalStatus(requestId: string, phase: "accepted" | "failed" | "cancelled", refusal?: DocumentCheckInRefusalV1): DocumentCheckInStatusV1 {
  return { schema: DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1, requestId, phase, progress: { completedUnits: 0, totalUnits: 8 }, ...(refusal === undefined ? {} : { refusal }) };
}

function documentCheckInCurrent(operation: DocumentCheckInOperationV1): boolean {
  return documentCheckInOperations.get(operation.request.requestId) === operation && !operation.abort.signal.aborted;
}

function postDocumentCheckIn(operation: DocumentCheckInOperationV1, status: DocumentCheckInStatusV1): void {
  operation.latest = status;
  post({ kind: "document-check-in-status", clientInstanceId: operation.request.clientInstanceId, scope: operation.request.scope, status });
  if (isTerminalDocumentCheckInPhaseV1(status.phase)) {
    documentCheckInOperations.delete(operation.request.requestId);
    operation.abort.abort(new Error("document check-in: settled"));
  }
}

function documentCheckInPath(operation: DocumentCheckInOperationV1, suffix = ""): string {
  return `/spaces/${encodeURIComponent(operation.request.scope.spaceId)}/documents/${encodeURIComponent(operation.request.scope.documentId)}/check-ins${suffix}`;
}

async function readDocumentCheckInStatus(operation: DocumentCheckInOperationV1, response: FetchTimeoutResponse): Promise<DocumentCheckInStatusV1 | null> {
  const control: ExecutionTargetReadControl = { signal: operation.abort.signal, deadlineAtMs: operation.deadlineAtMs, assertCurrent: () => {
    if (!documentCheckInCurrent(operation)) throw new Error("document check-in: stale owner");
  } };
  const bytes = await readBoundedExecutionTargetBody(response, null, DOCUMENT_CHECK_IN_MAX_BYTES, control, () => {});
  try {
    const status = parseDocumentCheckInStatusV1(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
    return status !== null && status.requestId === operation.request.requestId ? status : null;
  } finally {
    bytes.fill(0);
  }
}

function waitForDocumentCheckIn(operation: DocumentCheckInOperationV1, ms: number): Promise<void> {
  return new Promise((resolve) => {
    const timer = setTimeout(resolve, ms);
    operation.abort.signal.addEventListener("abort", () => {
      clearTimeout(timer);
      resolve();
    }, { once: true });
  });
}

/** ⏳️ The document's acknowledged head once nothing local is still unacknowledged, or `null` when the
 * document is not hub-bound, not this client's, or never quiesces in time. */
async function documentCheckInHead(operation: DocumentCheckInOperationV1): Promise<WireFrontierSummary | null> {
  const quiescentBy = Date.now() + DOCUMENT_CHECK_IN_QUIESCENCE_MS;
  while (documentCheckInCurrent(operation) && !operation.cancelRequested) {
    const state = artifactState(operation.request.scope.documentId, operation.request.scope.spaceId);
    if (state === undefined || state.closed || state.openClientInstanceId !== operation.request.clientInstanceId) return null;
    if (state.frontier !== null && state.pendingBatches.size === 0 && state.outbox.length === 0 && state.pendingMutations.length === 0) return state.frontier;
    if (Date.now() >= quiescentBy) return null;
    await waitForDocumentCheckIn(operation, DOCUMENT_CHECK_IN_POLL_MS);
  }
  return null;
}

/** 📌️ Names the acknowledged head, posts the Check In, and follows the hub's status to a terminal
 * phase, relaying every status; a cancellation reaches the hub as its own cancel command. */
async function driveDocumentCheckIn(operation: DocumentCheckInOperationV1): Promise<void> {
  const head = await documentCheckInHead(operation);
  if (!documentCheckInCurrent(operation)) return;
  if (operation.cancelRequested) {
    postDocumentCheckIn(operation, documentCheckInLocalStatus(operation.request.requestId, "cancelled"));
    return;
  }
  if (head === null || head.head_edit_ordinal === 0) {
    postDocumentCheckIn(operation, documentCheckInLocalStatus(operation.request.requestId, "failed", head === null ? "unavailable" : "stale-head"));
    return;
  }
  const body = documentCheckInCanonicalJson({
    schema: DOCUMENT_CHECK_IN_SCHEMA_V1,
    requestId: operation.request.requestId,
    head: { documentId: operation.request.scope.documentId, headEditOrdinal: head.head_edit_ordinal, headEditId: head.head_edit_id, lastCommitSeq: head.last_commit_seq, chainSha256: Array.from(head.chain_hash, (byte) => byte.toString(16).padStart(2, "0")).join("") },
  });
  let posted = false;
  while (documentCheckInCurrent(operation)) {
    if (Date.now() >= operation.deadlineAtMs) {
      postDocumentCheckIn(operation, { schema: DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1, requestId: operation.request.requestId, phase: "failed", progress: operation.latest.progress, refusal: "unavailable" });
      return;
    }
    try {
      let response: FetchTimeoutResponse;
      if (!posted) {
        response = await hubSessionFetch(documentCheckInPath(operation), { method: "POST", headers: { "content-type": "application/json" }, body }, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: operation.abort.signal });
        posted = response.ok;
      } else if (operation.cancelRequested && !operation.cancelSent) {
        operation.cancelSent = true;
        response = await hubSessionFetch(documentCheckInPath(operation, `/${operation.request.requestId}/cancel`), { method: "POST" }, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: operation.abort.signal });
      } else {
        response = await hubSessionFetch(documentCheckInPath(operation, `/${operation.request.requestId}`), { method: "GET" }, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: operation.abort.signal });
      }
      if (!documentCheckInCurrent(operation)) return;
      if (!response.ok) {
        if (response.status === 401 || response.status === 403) {
          postDocumentCheckIn(operation, documentCheckInLocalStatus(operation.request.requestId, "failed", "authority-changed"));
          return;
        }
        if (response.status >= 400 && response.status < 500) {
          postDocumentCheckIn(operation, documentCheckInLocalStatus(operation.request.requestId, "failed", "unavailable"));
          return;
        }
        throw new Error("document check-in: unavailable");
      }
      const status = await readDocumentCheckInStatus(operation, response);
      if (!documentCheckInCurrent(operation)) return;
      if (status !== null) postDocumentCheckIn(operation, status);
      if (status !== null && isTerminalDocumentCheckInPhaseV1(status.phase)) return;
    } catch {
      if (!documentCheckInCurrent(operation)) return;
    }
    await waitForDocumentCheckIn(operation, DOCUMENT_CHECK_IN_POLL_MS);
  }
}

export function submitDocumentCheckIn(request: Extract<BackboneWorkerRequest, { readonly kind: "document-check-in" }>): void {
  const existing = documentCheckInOperations.get(request.requestId);
  if (existing !== undefined) {
    post({ kind: "document-check-in-status", clientInstanceId: request.clientInstanceId, scope: request.scope, status: existing.latest });
    return;
  }
  if (documentCheckInOperations.size >= DOCUMENT_CHECK_IN_CAPACITY) {
    post({ kind: "document-check-in-status", clientInstanceId: request.clientInstanceId, scope: request.scope, status: documentCheckInLocalStatus(request.requestId, "failed", "unavailable") });
    return;
  }
  const operation: DocumentCheckInOperationV1 = { request, abort: new AbortController(), deadlineAtMs: Date.now() + DOCUMENT_CHECK_IN_DEADLINE_MS, cancelRequested: false, cancelSent: false, latest: documentCheckInLocalStatus(request.requestId, "accepted") };
  documentCheckInOperations.set(request.requestId, operation);
  post({ kind: "document-check-in-status", clientInstanceId: request.clientInstanceId, scope: request.scope, status: operation.latest });
  void driveDocumentCheckIn(operation);
}

export function cancelDocumentCheckIn(requestId: string, scope: DocumentScope): void {
  const operation = documentCheckInOperations.get(requestId);
  if (operation === undefined || operation.request.scope.spaceId !== scope.spaceId || operation.request.scope.documentId !== scope.documentId) return;
  operation.cancelRequested = true;
}
