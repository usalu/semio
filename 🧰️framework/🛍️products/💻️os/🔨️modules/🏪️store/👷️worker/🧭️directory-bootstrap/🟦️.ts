import { DirectoryClient, documentRuntimeKeyV1, createSocketGrantIssuerV1, HUB_RECONNECT_MIN_MS, HUB_RECONNECT_MAX_MS, DirectoryCommandError } from "../../../../🟦️";
import type { DirectoryAcknowledgedStream, DirectoryEventPageAckV1, DocumentScope, DirectoryCommand } from "../../../../🟦️";
import type { DirectoryEvent } from "@semio-tech/framework/directory/schema";
import { retryWithJitteredBackoff } from "@semio-tech/framework";
import { parseDirectoryEventPageV1 } from "@semio-tech/framework/directory/io/text";
import type { DirectoryCommandRequestV1 } from "@semio-tech/framework/directory/schema";
import { sealDirectoryCommandRequestV1, directoryCommandRequestJson } from "@semio-tech/framework/directory/io/text";
import { type DirectoryEventPageBootstrapV1, browserDirectoryRequest, directoryBootstrap, directorySessionEpoch, directoryStatus, directoryWorkerEpoch, fetchDirectoryBootstrapPage, flushDirectoryQueue, openDirectoryBootstrapLive, requestSocketGrant, scheduleDirectoryBootstrapRetry, settleDirectoryCommand, spaceDirectoryLanes } from "../🔐️hub-session/🟦️.ts";
import { post } from "../🧾️document-state/🟦️.ts";
import { closeArtifactRuntime, directoryCommandOperations, directoryCommandQueue, scopedDirectoryStreams } from "../🛑️session-teardown/🟦️.ts";
import { revokeDirectoryAdministrationForScope } from "../🛡️directory-administration/🟦️.ts";
import { DIRECTORY_COMMAND_TRANSPORT_CAPACITY, type DirectoryCommandTransportOperationV1 } from "../📇️space-artifact-creation/🟦️.ts";

export type DirectoryBootstrapTransition = { readonly kind: "fetch"; readonly after: number } | { readonly kind: "live"; readonly since: number };

export type DirectoryBootstrapOwner = {
  readonly machine: DirectoryEventPageBootstrapV1;
  readonly client: DirectoryClient;
  readonly abort: AbortController;
  stream: DirectoryAcknowledgedStream | null;
  retry: ReturnType<typeof setTimeout> | null;
  fetching: boolean;
};

export function acknowledgeDirectoryBootstrap(ack: DirectoryEventPageAckV1): void {
  const owner = directoryBootstrap;
  if (owner === null) return;
  try {
    const transition = owner.machine.acknowledge(ack);
    if (transition.kind === "fetch") void fetchDirectoryBootstrapPage(owner);
    else openDirectoryBootstrapLive(owner, transition.since);
  } catch {
    post({ kind: "directory-bootstrap-failed", bootstrapEpoch: ack.bootstrapEpoch, code: "invalid-page", retryable: false });
  }
}

export function rejectDirectoryBootstrap(bootstrapEpoch: number, receiptSha256: string): void {
  const owner = directoryBootstrap;
  if (owner === null) return;
  try {
    owner.machine.reject(bootstrapEpoch, receiptSha256);
    scheduleDirectoryBootstrapRetry(owner);
  } catch {
    post({ kind: "directory-bootstrap-failed", bootstrapEpoch, code: "invalid-page", retryable: false });
  }
}

function scopedDirectoryKey(scope: DocumentScope): string {
  return documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId: scope.spaceId, documentId: scope.documentId });
}

export function openScopedDirectory(baseUrl: string, scope: DocumentScope, since: number): void {
  const key = scopedDirectoryKey(scope);
  scopedDirectoryStreams.get(key)?.close();
  const client = new DirectoryClient(baseUrl, {
    requestBaseUrl: "",
    socketGrantIssuer: createSocketGrantIssuerV1({ post: (path, options) => requestSocketGrant(baseUrl, path, options?.signal) }),
    request: browserDirectoryRequest,
  });
  let stream: { close: () => void } | null = null;
  stream = client.streamScoped(
    scope,
    since,
    (message) => post({ kind: "directory-message", message }),
    () => {
      if (scopedDirectoryStreams.get(key) !== stream) return;
      scopedDirectoryStreams.delete(key);
      // 🧯️ A scoped 4401 is an authoritative membership revocation for exactly this space. If the one
      // retained administration operation is administering it, that operation must erase its page,
      // receipt and capability NOW rather than keep an authoritative-looking pane alive until the next
      // read happens to answer 403/404 (packet §3: "unmount, identity/session generation change,
      // 401/403, or scoped 4401").
      revokeDirectoryAdministrationForScope(scope.spaceId);
      closeArtifactRuntime(key);
      post({ kind: "directory-scope-revoked", scope });
    },
  );
  scopedDirectoryStreams.set(key, stream);
}

/** 📇️ Opens one space index's directory lane: the space's full event history from the first sealed directory page on
 * (retried with jittered backoff through a short link loss), then every live event of that space from the global
 * directory stream, each posted as `directory-space-events` in ascending `seq`. The space index is the shell's
 * projection of the space's directory — it has no document scope, so it never asks for a document-scoped stream. */
export function openSpaceDirectory(baseUrl: string, spaceId: string): void {
  closeSpaceDirectory(spaceId);
  const abort = new AbortController();
  let stream: { close: () => void } | null = null;
  const lane = {
    close: () => {
      abort.abort(new Error("space directory closed"));
      stream?.close();
    },
  };
  spaceDirectoryLanes.set(spaceId, lane);
  const client = new DirectoryClient(baseUrl, {
    requestBaseUrl: "",
    socketGrantIssuer: createSocketGrantIssuerV1({ post: (path, options) => requestSocketGrant(baseUrl, path, options?.signal) }),
    request: browserDirectoryRequest,
  });
  const deliver = (events: readonly DirectoryEvent[]): void => {
    const own = events.filter((event) => event.spaceId === spaceId);
    if (own.length > 0 && spaceDirectoryLanes.get(spaceId) === lane) post({ kind: "directory-space-events", spaceId, events: own });
  };
  void (async () => {
    let after = 0;
    for (let more = true; more; ) {
      const page = await retryWithJitteredBackoff(async () => parseDirectoryEventPageV1((await client.eventPage(after, { signal: abort.signal })).canonicalJson), { minMs: HUB_RECONNECT_MIN_MS, maxMs: HUB_RECONNECT_MAX_MS, signal: abort.signal });
      deliver(page.events);
      after = page.throughSeqInclusive;
      more = page.hasMore;
    }
    if (abort.signal.aborted) return;
    stream = client.stream(after, (message) => {
      if (message.kind === "event") deliver([message.event]);
    });
  })().catch(() => undefined);
}

export function closeSpaceDirectory(spaceId: string): void {
  spaceDirectoryLanes.get(spaceId)?.close();
  spaceDirectoryLanes.delete(spaceId);
}

export function closeScopedDirectory(scope: DocumentScope): void {
  const key = scopedDirectoryKey(scope);
  scopedDirectoryStreams.get(key)?.close();
  scopedDirectoryStreams.delete(key);
}

/** 🆔️ Seals one request, admits it against the fixed capacity, and starts its transport turn. A
 * sealing failure, a duplicate live id, and a full transport are all terminal — never queued. */
export async function submitDirectoryCommand(requestId: string, command: DirectoryCommand): Promise<void> {
  if (directoryCommandOperations.has(requestId)) {
    post({ kind: "directory-command-failed", requestId, code: "request-conflict" });
    return;
  }
  if (directoryCommandOperations.size >= DIRECTORY_COMMAND_TRANSPORT_CAPACITY) {
    post({ kind: "directory-command-failed", requestId, code: "capacity" });
    return;
  }
  let request: DirectoryCommandRequestV1;
  try {
    request = sealDirectoryCommandRequestV1(requestId, command);
    directoryCommandRequestJson(request);
  } catch {
    post({ kind: "directory-command-failed", requestId, code: "invalid" });
    return;
  }
  const operation: DirectoryCommandTransportOperationV1 = { request, abort: new AbortController(), sessionEpoch: directorySessionEpoch, workerEpoch: directoryWorkerEpoch, settled: false };
  directoryCommandOperations.set(requestId, operation);
  directoryCommandQueue.push(operation);
  post(directoryStatus());
  await flushDirectoryQueue();
}

/** 🛑️ Cancels one live operation's HTTP wait. The hub may already have linearized the command, so
 * the owner is told `cancelled` (indeterminate) and the id is never silently reissued. */
export function cancelDirectoryCommand(requestId: string): void {
  const operation = directoryCommandOperations.get(requestId);
  if (!operation) return;
  operation.abort.abort(new DirectoryCommandError("cancelled"));
  settleDirectoryCommand(operation, { kind: "directory-command-failed", requestId, code: "cancelled" });
  post(directoryStatus());
}
