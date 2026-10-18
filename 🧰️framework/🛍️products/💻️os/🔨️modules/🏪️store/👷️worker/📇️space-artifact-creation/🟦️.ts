import type { DirectoryCommandRequestV1 } from "@semio-tech/framework/directory/schema";
import type { BackboneWorkerRequest, BackboneWorkerResponse } from "../../../../🟦️";
import type { FetchTimeoutResponse } from "@semio-tech/framework";
import { type SpaceArtifactCreationCatalogV1 as HubSpaceArtifactCreationCatalogV1, SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES, type SpaceArtifactCreationStatusV1 as HubSpaceArtifactCreationStatusV1, SPACE_ARTIFACT_CREATION_MAX_BYTES, sealSpaceArtifactCreateV1 } from "../../../📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";
import { parseSpaceArtifactCreationCatalogJsonV1, parseSpaceArtifactCreationStatusJsonV1 } from "../../../📇️directory/🚪️io/📝️text/🌱️space-artifact-creation-v1/🟦️.ts";
import { spaceArtifactCreationPollDelayV1, spaceArtifactCreationUnreachableV1 } from "../🌱️creation-polling/🟦️.ts";
import { spaceArtifactCreationCatalogOperations, spaceArtifactCreationOperations } from "../🛑️session-teardown/🟦️.ts";
import { post } from "../🧾️document-state/🟦️.ts";
import { directoryWorkerEpoch, hubSessionFetch } from "../🔐️hub-session/🟦️.ts";
import { type ExecutionTargetReadControl, readBoundedExecutionTargetBody } from "../📥️execution-target-read/🟦️.ts";
import { SOCKET_GRANT_REQUEST_TIMEOUT_MS } from "../⚙️constants/🟦️.ts";

/** 📇️ Directory hub lane (contract §C6) — the shell's only path to the directory control plane;
 * plugin surfaces never talk to the network, and the shell never opens a directory socket on the UI
 * thread. Owns exactly one {@link DirectoryClient}/{@link DirectoryStream} at a time. Reuses that
 * client's own reconnect/backoff (`🔖️HubBinding` in `🟦️.ts`) rather than a second loop
 * here — this region's only extra responsibility is the offline command queue. */
/** 📏️ Fixed transport capacity. A 65th live operation is REJECTED with a terminal `capacity`
 * result; the oldest intent is never silently dropped, because an administration mutation the
 * caller believes it issued must never vanish without a receipt. */
export const DIRECTORY_COMMAND_TRANSPORT_CAPACITY = 64;

/** 🆔️ One retained command operation. `request` is sealed once and re-sent byte-identically by
 * every transient retry — a retry that changed a single byte would be a different command to the
 * hub's digest-keyed idempotency store. `sessionEpoch`/`workerEpoch` suppress delivery after an
 * identity or worker replacement; such a request may later be explicitly resolved, never auto-replayed. */
export type DirectoryCommandTransportOperationV1 = {
  readonly request: DirectoryCommandRequestV1;
  readonly abort: AbortController;
  readonly sessionEpoch: number;
  readonly workerEpoch: number;
  settled: boolean;
};

const SPACE_ARTIFACT_CREATION_CAPACITY = 8;
const SPACE_ARTIFACT_CREATION_CATALOG_CAPACITY = 8;
const SPACE_ARTIFACT_CREATION_CATALOG_DEADLINE_MS = 10_000;

export type SpaceArtifactCreationOperationV1 = {
  readonly request: Extract<BackboneWorkerRequest, { readonly kind: "space-artifact-create" }>;
  readonly abort: AbortController;
  readonly workerEpoch: number;
  lastAnsweredAtMs: number;
  polls: number;
  cancelRequested: boolean;
  cancelSent: boolean;
  latest: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }>;
};
export let spaceArtifactCreationTestFetch: ((path: string, init: RequestInit, signal: AbortSignal) => Promise<FetchTimeoutResponse>) | null = null;
/** 🧪️ Assigns `spaceArtifactCreationTestFetch` for the test seam, because an imported binding cannot be assigned outside its declaring module. */
export function setSpaceArtifactCreationTestFetch(value: typeof spaceArtifactCreationTestFetch): void {
  spaceArtifactCreationTestFetch = value;
}

function spaceArtifactCreationCatalogPresentation(
  spaceId: string,
  clientInstanceId: string,
  phase: "loading" | "ready" | "unavailable",
): Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog-status" }> {
  return { kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId, phase };
}

function spaceArtifactCreationCatalogStatus(spaceId: string, clientInstanceId: string, catalog: HubSpaceArtifactCreationCatalogV1): Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog" }> {
  if (catalog.spaceId !== spaceId) throw new Error("space artifact creation catalog: scope mismatch");
  return { kind: "space-artifact-creation-catalog", clientInstanceId, spaceId, catalogGenerationId: catalog.catalogGenerationId, kinds: catalog.kinds };
}

export async function openSpaceArtifactCreationCatalog(spaceId: string, clientInstanceId: string): Promise<void> {
  const prior = spaceArtifactCreationCatalogOperations.get(spaceId);
  prior?.abort.abort(new Error("space artifact creation catalog: superseded"));
  spaceArtifactCreationCatalogOperations.delete(spaceId);
  if (spaceArtifactCreationCatalogOperations.size >= SPACE_ARTIFACT_CREATION_CATALOG_CAPACITY) {
    post(spaceArtifactCreationCatalogPresentation(spaceId, clientInstanceId, "unavailable"));
    return;
  }
  const operation = { abort: new AbortController(), clientInstanceId, workerEpoch: directoryWorkerEpoch };
  spaceArtifactCreationCatalogOperations.set(spaceId, operation);
  post(spaceArtifactCreationCatalogPresentation(spaceId, clientInstanceId, "loading"));
  const current = (): boolean => spaceArtifactCreationCatalogOperations.get(spaceId) === operation && operation.workerEpoch === directoryWorkerEpoch && !operation.abort.signal.aborted;
  try {
    const encodedSpaceId = encodeURIComponent(spaceId);
    if (!/^[A-Za-z0-9][A-Za-z0-9._:-]{0,255}$/u.test(spaceId) || encodedSpaceId !== spaceId) throw new Error("space artifact creation catalog: invalid scope");
    const path = `/spaces/${encodedSpaceId}/artifact-creations`;
    const response = spaceArtifactCreationTestFetch === null
      ? await hubSessionFetch(`${path}`, { method: "GET" }, { timeoutMs: SPACE_ARTIFACT_CREATION_CATALOG_DEADLINE_MS, signal: operation.abort.signal })
      : await spaceArtifactCreationTestFetch(path, { method: "GET" }, operation.abort.signal);
    if (!response.ok || !current()) throw new Error("space artifact creation catalog: unavailable");
    const control: ExecutionTargetReadControl = { signal: operation.abort.signal, deadlineAtMs: Date.now() + SPACE_ARTIFACT_CREATION_CATALOG_DEADLINE_MS, assertCurrent: () => {
      if (!current()) throw new Error("space artifact creation catalog: stale owner");
    } };
    const bytes = await readBoundedExecutionTargetBody(response, null, SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES, control, () => {});
    try {
      const catalog = parseSpaceArtifactCreationCatalogJsonV1(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
      if (current()) {
        post(spaceArtifactCreationCatalogStatus(spaceId, clientInstanceId, catalog));
        post(spaceArtifactCreationCatalogPresentation(spaceId, clientInstanceId, "ready"));
      }
    } finally {
      bytes.fill(0);
    }
  } catch {
    if (current()) post(spaceArtifactCreationCatalogPresentation(spaceId, clientInstanceId, "unavailable"));
  } finally {
    if (spaceArtifactCreationCatalogOperations.get(spaceId) === operation) spaceArtifactCreationCatalogOperations.delete(spaceId);
  }
}

function spaceArtifactCreationStatus(operation: SpaceArtifactCreationOperationV1, status: HubSpaceArtifactCreationStatusV1): Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }> {
  if (status.requestId !== operation.request.requestId || status.spaceId !== operation.request.spaceId || status.catalogGenerationId !== operation.request.expectedCatalogGenerationId || (status.ready !== undefined && status.ready.kindId !== operation.request.kindId)) throw new Error("space artifact creation: owner mismatch");
  return { kind: "space-artifact-creation-status", requestId: status.requestId, spaceId: status.spaceId, catalogGenerationId: status.catalogGenerationId, phase: status.phase, ...(status.ready === undefined ? {} : { ready: status.ready }) };
}

function spaceArtifactCreationCurrent(operation: SpaceArtifactCreationOperationV1): boolean {
  return spaceArtifactCreationOperations.get(operation.request.requestId) === operation && operation.workerEpoch === directoryWorkerEpoch && !operation.abort.signal.aborted;
}

function spaceArtifactCreationTerminal(phase: HubSpaceArtifactCreationStatusV1["phase"]): boolean {
  return phase === "ready" || phase === "indeterminate" || phase === "failed" || phase === "cancelled";
}

function spaceArtifactCreationPath(operation: SpaceArtifactCreationOperationV1, suffix = ""): string {
  const path = `/spaces/${encodeURIComponent(operation.request.spaceId)}/artifact-creations${suffix}`;
  if (!/^\/spaces\/[^/?#]+\/artifact-creations(?:\/[0-9a-f]{32}(?:\/cancel)?)?$/u.test(path)) throw new Error("space artifact creation: operation denied");
  return path;
}

function spaceArtifactCreationFetch(operation: SpaceArtifactCreationOperationV1, suffix: string, init: RequestInit): Promise<FetchTimeoutResponse> {
  const path = spaceArtifactCreationPath(operation, suffix);
  if (!spaceArtifactCreationCurrent(operation)) return Promise.reject(new Error("space artifact creation: stale owner"));
  if (spaceArtifactCreationTestFetch !== null) return spaceArtifactCreationTestFetch(path, init, operation.abort.signal);
  return hubSessionFetch(`${path}`, init, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: operation.abort.signal });
}

async function readSpaceArtifactCreationStatus(operation: SpaceArtifactCreationOperationV1, response: FetchTimeoutResponse): Promise<HubSpaceArtifactCreationStatusV1> {
  const control: ExecutionTargetReadControl = { signal: operation.abort.signal, deadlineAtMs: Date.now() + SOCKET_GRANT_REQUEST_TIMEOUT_MS, assertCurrent: () => {
    if (!spaceArtifactCreationCurrent(operation)) throw new Error("space artifact creation: stale owner");
  } };
  const bytes = await readBoundedExecutionTargetBody(response, null, SPACE_ARTIFACT_CREATION_MAX_BYTES, control, () => {});
  try {
    return parseSpaceArtifactCreationStatusJsonV1(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
  } finally {
    bytes.fill(0);
  }
}

function waitForSpaceArtifactCreationPoll(operation: SpaceArtifactCreationOperationV1): Promise<void> {
  const delayMs = spaceArtifactCreationPollDelayV1(operation.polls);
  operation.polls += 1;
  return new Promise((resolve, reject) => {
    const timer = setTimeout(resolve, delayMs);
    operation.abort.signal.addEventListener("abort", () => {
      clearTimeout(timer);
      reject(new Error("space artifact creation: cancelled"));
    }, { once: true });
  });
}

function settleSpaceArtifactCreation(operation: SpaceArtifactCreationOperationV1, status?: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }>): void {
  if (!spaceArtifactCreationCurrent(operation)) return;
  spaceArtifactCreationOperations.delete(operation.request.requestId);
  operation.abort.abort(new Error("space artifact creation: settled"));
  if (status !== undefined) post(status);
}

async function driveSpaceArtifactCreation(operation: SpaceArtifactCreationOperationV1): Promise<void> {
  const requestBody = JSON.stringify(sealSpaceArtifactCreateV1(operation.request));
  let first = true;
  while (spaceArtifactCreationCurrent(operation)) {
    if (spaceArtifactCreationUnreachableV1(operation.lastAnsweredAtMs, Date.now())) {
      settleSpaceArtifactCreation(operation, { ...operation.latest, phase: "indeterminate" });
      return;
    }
    try {
      let response: FetchTimeoutResponse;
      let initialPost = false;
      if (operation.cancelRequested && !operation.cancelSent) {
        operation.cancelSent = true;
        response = await spaceArtifactCreationFetch(operation, `/${operation.request.requestId}/cancel`, { method: "POST" });
      } else if (first) {
        first = false;
        initialPost = true;
        response = await spaceArtifactCreationFetch(operation, "", { method: "POST", headers: { "content-type": "application/json" }, body: requestBody });
      } else {
        response = await spaceArtifactCreationFetch(operation, `/${operation.request.requestId}`, { method: "GET" });
      }
      if (!spaceArtifactCreationCurrent(operation)) return;
      if (!response.ok) {
        if (initialPost && response.status === 409) {
          post({ kind: "space-artifact-creation-catalog-refresh-required", requestId: operation.request.requestId, spaceId: operation.request.spaceId, catalogGenerationId: operation.request.expectedCatalogGenerationId });
          settleSpaceArtifactCreation(operation, { ...operation.latest, phase: "failed" });
          return;
        }
        if (response.status >= 400 && response.status < 500) {
          settleSpaceArtifactCreation(operation, { ...operation.latest, phase: "failed" });
          return;
        }
        throw new Error("space artifact creation: unavailable");
      }
      const status = spaceArtifactCreationStatus(operation, await readSpaceArtifactCreationStatus(operation, response));
      if (!spaceArtifactCreationCurrent(operation)) return;
      operation.lastAnsweredAtMs = Date.now();
      operation.latest = status;
      post(status);
      if (spaceArtifactCreationTerminal(status.phase)) {
        settleSpaceArtifactCreation(operation);
        return;
      }
    } catch {
      if (!spaceArtifactCreationCurrent(operation)) return;
    }
    await waitForSpaceArtifactCreationPoll(operation).catch(() => {});
  }
}

export function submitSpaceArtifactCreation(request: Extract<BackboneWorkerRequest, { readonly kind: "space-artifact-create" }>): void {
  const existing = spaceArtifactCreationOperations.get(request.requestId);
  if (existing !== undefined) {
    if (existing.request.spaceId === request.spaceId && existing.request.expectedCatalogGenerationId === request.expectedCatalogGenerationId && existing.request.kindId === request.kindId && existing.request.name === request.name) post(existing.latest);
    else post({ kind: "space-artifact-creation-status", requestId: request.requestId, spaceId: request.spaceId, catalogGenerationId: request.expectedCatalogGenerationId, phase: "failed" });
    return;
  }
  if (spaceArtifactCreationOperations.size >= SPACE_ARTIFACT_CREATION_CAPACITY) {
    post({ kind: "space-artifact-creation-status", requestId: request.requestId, spaceId: request.spaceId, catalogGenerationId: request.expectedCatalogGenerationId, phase: "failed" });
    return;
  }
  const operation: SpaceArtifactCreationOperationV1 = {
    request,
    abort: new AbortController(),
    workerEpoch: directoryWorkerEpoch,
    lastAnsweredAtMs: Date.now(),
    polls: 0,
    cancelRequested: false,
    cancelSent: false,
    latest: { kind: "space-artifact-creation-status", requestId: request.requestId, spaceId: request.spaceId, catalogGenerationId: request.expectedCatalogGenerationId, phase: "accepted" },
  };
  spaceArtifactCreationOperations.set(request.requestId, operation);
  post(operation.latest);
  void driveSpaceArtifactCreation(operation);
}

export function cancelSpaceArtifactCreation(requestId: string, spaceId: string): void {
  const operation = spaceArtifactCreationOperations.get(requestId);
  if (operation === undefined || operation.request.spaceId !== spaceId) return;
  operation.cancelRequested = true;
}
