import type { SocketGrantReceiptV1, DocumentSocketGrantReceiptV1, HubSessionPortResponseV1, CanonicalDirectoryEventPageV1, DirectoryEventPageAckV1, BackboneWorkerResponse, DirectoryStreamMessage } from "../../../../🟦️";
import { type DirectorySessionAuthorityV1, DIRECTORY_SESSION_AUTHORITY_MAX_BYTES } from "@semio-tech/framework/directory/schema/session-authority-v1";
import { type FetchTimeoutResponse, fetchWithTimeout } from "@semio-tech/framework";
import { parseDirectorySessionAuthorityJsonV1 } from "@semio-tech/framework/directory/io/text/session-authority-v1";
import { HUB_SESSION_CAPABILITY_PATTERN_V1, parseHubSessionPortRequestV1, parseSocketGrantReceiptV1, DirectoryClient, createSocketGrantIssuerV1, HUB_RECONNECT_MIN_MS, DirectoryHttpError, DirectoryCommandError } from "../../../../🟦️";
import type { DocumentOpenPlanV1, DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1 } from "@semio-tech/framework/directory/schema";
import { DIRECTORY_PREFERENCE_PAGE_PATH_V1 } from "@semio-tech/framework/directory/schema";
import { directoryStreamWakeV1 } from "@semio-tech/framework/directory";
import { directoryCommandErrorIsTransient } from "@semio-tech/framework/directory/schema";
import { artifacts, documentServices, post } from "../🧾️document-state/🟦️.ts";
import { hubBinding } from "../🧩️actor-wire-helpers/🟦️.ts";
import { type ExecutionTargetReadControl, emitExecutionTargetStatus, readBoundedExecutionTargetBody, readExecutionTargetJson } from "../📥️execution-target-read/🟦️.ts";
import { artifactBootstrapFailure, closeArtifactRuntime, directoryCommandOperations, directoryCommandQueue, scopedDirectoryStreams, spaceArtifactCreationCatalogOperations, spaceArtifactCreationOperations } from "../🛑️session-teardown/🟦️.ts";
import { SOCKET_GRANT_REQUEST_TIMEOUT_MS } from "../⚙️constants/🟦️.ts";
import type { DirectoryBootstrapOwner, DirectoryBootstrapTransition } from "../🧭️directory-bootstrap/🟦️.ts";
import type { DirectoryCommandTransportOperationV1 } from "../📇️space-artifact-creation/🟦️.ts";
import type { DirectoryAdministrationOperationV1 } from "../🛡️directory-administration/🟦️.ts";

const SOCKET_GRANT_REQUEST_LIMIT = 256;
const DOCUMENT_OPEN_RESPONSE_MAX_BYTES = 64 * 1024;
/** 🌐️ The shell's own same-origin route to its hub. In development the vite server forwards it to
 * `S_HUB_URL` verbatim, `Authorization` header included (`🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`); in a
 * deployment whatever fronts the app mounts the hub here. Same-origin is what keeps the human's
 * session capability out of a cross-origin preflight and lets one code path serve both. */
const HUB_REQUEST_ROUTE_PREFIX = "/_semio/hub";
export let socketGrantTestIssue: ((baseUrl: string, path: string, signal?: AbortSignal) => Promise<SocketGrantReceiptV1>) | null = null;
/** 🧪️ Assigns `socketGrantTestIssue` for the test seam, because an imported binding cannot be assigned outside its declaring module. */
export function setSocketGrantTestIssue(value: typeof socketGrantTestIssue): void {
  socketGrantTestIssue = value;
}

/** 🧪️ Test seam for the document open-plan exchange, answering in place of `/socket-grants`. */
export let documentSocketGrantTestIssue: ((baseUrl: string, path: string, signal?: AbortSignal) => Promise<DocumentSocketGrantReceiptV1>) | null = null;
/** 🧪️ Assigns `documentSocketGrantTestIssue` for the test seam, because an imported binding cannot be assigned outside its declaring module. */
export function setDocumentSocketGrantTestIssue(value: typeof documentSocketGrantTestIssue): void {
  documentSocketGrantTestIssue = value;
}

/** 🎫️ The signed-in human's own hub session capability — the ONE credential this worker ever holds.
 * It arrives over the private port from the shell that minted it (`POST /auth/sessions`) and is
 * dropped the moment the hub refuses it, so the worker can never act as a principal the human is not. */
export let hubSessionCapability: string | undefined;
let hubSessionOwner: object = {};
let hubSessionAdmission: object = {};
export let browserSessionAuthority: DirectorySessionAuthorityV1 | null = null;
export let browserSessionOperationFence: object = {};
let hubSessionQueue: Promise<void> = Promise.resolve();
export let hubSessionQueued = 0;
/** 🧪️ Assigns `hubSessionQueued` for the test seam, because an imported binding cannot be assigned outside its declaring module. */
export function setHubSessionQueued(value: typeof hubSessionQueued): void {
  hubSessionQueued = value;
}

let hubSessionPort: MessagePort | undefined;
const hubSessionRpcControllers = new Map<string, AbortController>();

export function bytesHex(value: Uint8Array): string {
  return Array.from(value, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

export function clearHubSessionCapability(): void {
  hubSessionAdmission = {};
  consumeHubSessionCapability();
  retireBrowserSessionAuthority();
}

type BrowserSessionOperationFenceV1 = Readonly<{ token: object; sessionBindingSha256: string; authorizationGeneration: number }>;

export function captureBrowserSessionOperationFence(): BrowserSessionOperationFenceV1 | null {
  const authority = browserSessionAuthority;
  const observedAtMs = Date.now();
  if (authority === null || hubSessionCapability === undefined || observedAtMs >= authority.expiresAt) return null;
  return Object.freeze({ token: browserSessionOperationFence, sessionBindingSha256: authority.sessionBindingSha256, authorizationGeneration: authority.authorizationGeneration });
}

export function sameBrowserSessionOperationFence(fence: BrowserSessionOperationFenceV1): boolean {
  const authority = browserSessionAuthority;
  return fence.token === browserSessionOperationFence
    && authority !== null
    && Date.now() < authority.expiresAt
    && authority.sessionBindingSha256 === fence.sessionBindingSha256
    && authority.authorizationGeneration === fence.authorizationGeneration;
}

/** 🪪️ Retires authenticated work without forgetting an already submitted inference request. */
function retireBrowserSessionAuthority(): void {
  const hadAuthority = browserSessionAuthority !== null;
  browserSessionAuthority = null;
  browserSessionOperationFence = {};
  if (hadAuthority) directorySessionEpoch += 1;
  closeDirectory();
  documentServices.sessionRetired();
  for (const [runtimeKey, state] of artifacts) {
    const binding = hubBinding(state.config);
    if (binding === null) continue;
    emitExecutionTargetStatus(state, binding, "stale");
    post({ ...artifactBootstrapFailure(state, new Error("document authority cancelled")), retryable: false });
    closeArtifactRuntime(runtimeKey);
  }
}

/** 🔐️ Installs only the hub's own canonical authority for the capability this worker holds. */
export async function acceptBrowserSessionAuthority(response: FetchTimeoutResponse, admission: object, signal?: AbortSignal): Promise<FetchTimeoutResponse> {
  if (response.status !== 200) throw new Error("directory session authority: unexpected response");
  const assertCurrent = (): void => {
    if (admission !== hubSessionAdmission) throw new Error("hub session owner retired");
  };
  const bytes = await readBoundedExecutionTargetBody(response, null, DIRECTORY_SESSION_AUTHORITY_MAX_BYTES, { signal: signal ?? new AbortController().signal, deadlineAtMs: Date.now() + 2000, assertCurrent }, () => {});
  assertCurrent();
  const body = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  const authority = Object.freeze(parseDirectorySessionAuthorityJsonV1(body));
  if (authority.expiresAt <= Date.now()) throw new Error("directory session authority: expired");
  const current = browserSessionAuthority;
  if (current !== null && (current.sessionBindingSha256 !== authority.sessionBindingSha256 || current.authorizationGeneration !== authority.authorizationGeneration)) {
    hubSessionAdmission = {};
    retireBrowserSessionAuthority();
  } else if (current !== null && (current.userId !== authority.userId || current.expiresAt !== authority.expiresAt || current.sessionKind !== authority.sessionKind)) {
    throw new Error("directory session authority: contradictory binding");
  }
  browserSessionAuthority = authority;
  return { ok: response.ok, status: response.status, statusText: response.statusText, headers: response.headers, text: async () => body, json: async () => JSON.parse(body) };
}

function consumeHubSessionCapability(): void {
  hubSessionOwner = {};
  hubSessionCapability = undefined;
}

/** 🎫️ Adopts the capability the signed-in human's shell minted. Re-installing a DIFFERENT capability
 * retires every document, directory and inference owner bound to the previous one first, so one
 * worker can never mix two principals' work; re-installing the SAME one is idempotent, which is what
 * lets a reloaded page hand back the session it restored from storage without tearing anything down. */
export function installHubSessionCapability(capability: string): boolean {
  if (!HUB_SESSION_CAPABILITY_PATTERN_V1.test(capability)) return false;
  if (hubSessionCapability === capability) return true;
  retireBrowserSessionAuthority();
  hubSessionCapability = capability;
  hubSessionOwner = {};
  hubSessionAdmission = {};
  return true;
}

/** 🌐️ The single authenticated hub lane. Every hub HTTP call this worker makes goes through here,
 * carries the human's own `Authorization: Bearer`, and is serialized behind one queue so a retired
 * owner can never observe a response minted for its successor. A `401` is the hub's own statement
 * that the capability is gone: it drops the session rather than retrying, which is what surfaces
 * re-authentication in the shell instead of a silent stall. */
export async function hubSessionFetch(path: string, init: RequestInit = {}, options: { readonly timeoutMs: number; readonly signal?: AbortSignal; readonly admit?: () => boolean; readonly retain?: () => boolean; readonly accept?: (response: FetchTimeoutResponse, admission: object) => Promise<FetchTimeoutResponse> }): Promise<FetchTimeoutResponse> {
  if (options.signal?.aborted) throw options.signal.reason ?? new Error("hub session cancelled");
  const admission = hubSessionAdmission;
  if (hubSessionQueued >= 64) throw new Error("hub session capacity exceeded");
  hubSessionQueued += 1;
  let resolveTurn: () => void = () => undefined;
  const prior = hubSessionQueue;
  hubSessionQueue = new Promise<void>((resolve) => {
    resolveTurn = resolve;
  });
  await prior;
  try {
    if (options.signal?.aborted) throw options.signal.reason ?? new Error("hub session cancelled");
    if (admission !== hubSessionAdmission || options.admit?.() === false) throw new Error("hub session owner retired");
    const capability = hubSessionCapability;
    if (capability === undefined) throw new Error("hub session rebootstrap required");
    const owner = hubSessionOwner;
    let response: FetchTimeoutResponse;
    try {
      response = await fetchWithTimeout(
        `${HUB_REQUEST_ROUTE_PREFIX}${path}`,
        { ...init, headers: { ...(init.headers as Record<string, string> | undefined), authorization: `Bearer ${capability}` } },
        options,
      );
    } catch {
      throw new Error("hub session unreachable");
    }
    if (response.status === 401) {
      if (hubSessionOwner === owner) clearHubSessionCapability();
      throw new Error("hub session rebootstrap required");
    }
    if (admission !== hubSessionAdmission || hubSessionOwner !== owner || (options.retain ?? options.admit)?.() === false) throw new Error("hub session owner retired");
    return options.accept === undefined ? response : await options.accept(response, admission);
  } finally {
    hubSessionQueued -= 1;
    resolveTurn();
  }
}

/** 🔌️ Retires one private port and every body reader it admitted before a successor attaches. */
export function detachHubSessionPort(): void {
  if (hubSessionPort === undefined) return;
  hubSessionPort.onmessage = null;
  hubSessionPort.close();
  hubSessionPort = undefined;
  clearHubSessionCapability();
  for (const controller of hubSessionRpcControllers.values()) controller.abort(new Error("hub session port retired"));
  hubSessionRpcControllers.clear();
}

export function attachHubSessionPort(port: MessagePort): void {
  detachHubSessionPort();
  hubSessionPort = port;
  port.onmessage = (event: MessageEvent<unknown>) => {
    const message = parseHubSessionPortRequestV1(event.data);
    if (!message) return;
    if (message.kind === "initialize") {
      const response: HubSessionPortResponseV1 = { kind: "initialized", ok: installHubSessionCapability(message.capability) };
      port.postMessage(response);
      return;
    }
    if (message.kind === "close") {
      detachHubSessionPort();
      return;
    }
    if (message.kind === "cancel") {
      hubSessionRpcControllers.get(message.requestId)?.abort();
      return;
    }
    if (message.kind !== "request") return;
    if (hubSessionRpcControllers.size >= 64) {
      const response: HubSessionPortResponseV1 = { kind: "response", requestId: message.requestId, status: 503, body: "" };
      port.postMessage(response);
      return;
    }
    const requestId = message.requestId;
    const controller = new AbortController();
    hubSessionRpcControllers.set(requestId, controller);
    void hubSessionFetch("/auth/sessions/me", { method: "GET" }, { timeoutMs: 2_000, signal: controller.signal, accept: (response, admission) => acceptBrowserSessionAuthority(response, admission, controller.signal) })
      .then(async (response) => {
        const body = await response.text();
        const result: HubSessionPortResponseV1 = { kind: "response", requestId, status: response.status, body };
        port.postMessage(result);
      })
      .catch((error: unknown) => {
        const result: HubSessionPortResponseV1 = { kind: "response", requestId, status: error instanceof Error && error.message === "hub session rebootstrap required" ? 401 : 503, body: "" };
        port.postMessage(result);
      })
      .finally(() => {
        if (hubSessionRpcControllers.get(requestId) === controller) hubSessionRpcControllers.delete(requestId);
      });
  };
  port.start();
}

/** 🎫 Mints one audience-bound grant inside the credential-owning broker worker. */
export async function requestSocketGrant(baseUrl: string, path: string, signal?: AbortSignal): Promise<SocketGrantReceiptV1> {
  if (socketGrantTestIssue) return socketGrantTestIssue(baseUrl, path, signal);
  if (signal?.aborted || !baseUrl) throw new Error("socket grant: cancelled");
  const response = await hubSessionFetch(`${path}`, { method: "POST" }, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal });
  if (!response.ok) throw new Error("socket grant: unavailable");
  try {
    return parseSocketGrantReceiptV1(await response.json());
  } catch (error) {
    clearHubSessionCapability();
    throw error;
  }
}

export type BrowserDocumentSocketAuthorityV1 = Readonly<{
  receipt: DocumentSocketGrantReceiptV1;
  schema: string;
  packSchemaHash: readonly number[];
  parentDialect?: DocumentOpenPlanV1["parentDialect"];
  surfaceId?: string;
}>;

export async function readDocumentOpenJson(response: FetchTimeoutResponse, control: ExecutionTargetReadControl): Promise<unknown> {
  return readExecutionTargetJson(response, DOCUMENT_OPEN_RESPONSE_MAX_BYTES, control);
}

/** 🚦️ The only hub directory routes a page's worker may call with the human's session: the space list and one space, the event
 * page and the caller's own PREFERENCE page (each one canonical safe-decimal `after` and nothing else), the event stream and
 * the command lane. Rows: `💻️os/🧫️fixtures/📇️directory/🚦️browser-directory-routes.json`. The preference page was missing, so
 * every device's preference lane failed before it reached the network and no preference ever crossed devices (ticket
 * 26/09/23 S16). */
export function browserDirectoryRequest(input: string, init: RequestInit = {}, options: { readonly timeoutMs: number; readonly signal?: AbortSignal }): Promise<FetchTimeoutResponse> {
  const url = new URL(input, "http://hub-session.invalid");
  const method = init.method ?? "GET";
  const after = url.searchParams.get("after") ?? "";
  const canonicalAfter = [...url.searchParams].length === 1 && /^(?:0|[1-9]\d*)$/u.test(after) && Number.isSafeInteger(Number(after));
  const eventPage = (url.pathname === "/directory/event-page/v1" || url.pathname === DIRECTORY_PREFERENCE_PAGE_PATH_V1) && canonicalAfter;
  const allowed =
    (method === "GET" &&
      (((url.pathname === "/directory/spaces" || /^\/directory\/spaces\/[^/]+$/u.test(url.pathname)) && url.search === "") ||
        (url.pathname === "/directory/events" && [...url.searchParams].length === 1 && /^\d+$/u.test(url.searchParams.get("since") ?? "")) ||
        eventPage)) ||
    (method === "POST" && url.pathname === "/directory/commands" && url.search === "");
  if (!allowed) return Promise.reject(new Error("browser directory operation denied"));
  return hubSessionFetch(`${url.pathname}${url.search}`, init, options);
}

export let directoryClient: DirectoryClient | null = null;
/** 🧪️ Assigns `directoryClient` for the test seam, because an imported binding cannot be assigned outside its declaring module. */
export function setDirectoryClient(value: typeof directoryClient): void {
  directoryClient = value;
}

let directoryStream: { close: () => void } | null = null;
let directoryFlushing = false;
export let directorySessionEpoch = 0;
/** 🧪️ Assigns `directorySessionEpoch` for the test seam, because an imported binding cannot be assigned outside its declaring module. */
export function setDirectorySessionEpoch(value: typeof directorySessionEpoch): void {
  directorySessionEpoch = value;
}

export let directoryWorkerEpoch = 0;

/** 🧭️ Sole browser-worker owner of the fetch → retained Home ACK → live cursor. */
export class DirectoryEventPageBootstrapV1 {
  readonly bootstrapEpoch: number;
  private acknowledgedThrough: number;
  private pending: CanonicalDirectoryEventPageV1 | null = null;
  private phase: "fetching" | "awaiting-ack" | "live" | "closed" = "fetching";

  constructor(bootstrapEpoch: number, after: number) {
    if (!Number.isSafeInteger(bootstrapEpoch) || bootstrapEpoch < 0 || !Number.isSafeInteger(after) || after < 0) throw new Error("directory bootstrap: invalid owner");
    this.bootstrapEpoch = bootstrapEpoch;
    this.acknowledgedThrough = after;
  }

  after(): number {
    return this.acknowledgedThrough;
  }

  present(page: CanonicalDirectoryEventPageV1): void {
    if (this.phase !== "fetching" || page.afterSeqExclusive !== this.acknowledgedThrough || page.throughSeqInclusive < this.acknowledgedThrough) throw new Error("directory bootstrap: page ordering mismatch");
    this.pending = page;
    this.phase = "awaiting-ack";
  }

  acknowledge(ack: DirectoryEventPageAckV1): DirectoryBootstrapTransition {
    const page = this.pending;
    if (
      this.phase !== "awaiting-ack" ||
      page === null ||
      ack.bootstrapEpoch !== this.bootstrapEpoch ||
      ack.receiptSha256 !== page.receiptSha256 ||
      ack.sessionBindingSha256 !== page.sessionBindingSha256 ||
      ack.authorizationGeneration !== page.authorizationGeneration ||
      ack.throughSeqInclusive !== page.throughSeqInclusive
    )
      throw new Error("directory bootstrap: acknowledgement mismatch");
    this.acknowledgedThrough = page.throughSeqInclusive;
    this.pending = null;
    this.phase = page.hasMore ? "fetching" : "live";
    return page.hasMore ? { kind: "fetch", after: this.acknowledgedThrough } : { kind: "live", since: this.acknowledgedThrough };
  }

  reject(bootstrapEpoch: number, receiptSha256: string): number {
    if (this.phase !== "awaiting-ack" || this.pending === null || bootstrapEpoch !== this.bootstrapEpoch || receiptSha256 !== this.pending.receiptSha256) throw new Error("directory bootstrap: rejection mismatch");
    this.pending = null;
    this.phase = "fetching";
    return this.acknowledgedThrough;
  }

  wake(rebootstrap: boolean): number | null {
    if (this.phase !== "live") return null;
    if (rebootstrap) this.acknowledgedThrough = 0;
    this.phase = "fetching";
    return this.acknowledgedThrough;
  }

  close(): void {
    this.pending = null;
    this.phase = "closed";
  }
}

export let directoryBootstrap: DirectoryBootstrapOwner | null = null;

export function directoryStatus(): BackboneWorkerResponse {
  return { kind: "directory-status", pendingCommands: directoryCommandOperations.size };
}

export function openDirectory(baseUrl: string, since: number): void {
  closeDirectory();
  directorySessionEpoch += 1;
  const issuer = createSocketGrantIssuerV1({ post: (path, options) => requestSocketGrant(baseUrl, path, options?.signal) });
  const client = new DirectoryClient(baseUrl, {
    requestBaseUrl: "",
    socketGrantIssuer: issuer,
    request: browserDirectoryRequest,
  });
  directoryClient = client;
  directoryStream = client.stream(since, (message: DirectoryStreamMessage) => {
    post({ kind: "directory-message", message });
    void flushDirectoryQueue();
  });
  post(directoryStatus());
}

export function scheduleDirectoryBootstrapRetry(owner: DirectoryBootstrapOwner): void {
  if (directoryBootstrap !== owner || owner.abort.signal.aborted || owner.retry !== null) return;
  const delay = HUB_RECONNECT_MIN_MS + Math.floor(Math.random() * (HUB_RECONNECT_MIN_MS + 1));
  owner.retry = setTimeout(() => {
    owner.retry = null;
    void fetchDirectoryBootstrapPage(owner);
  }, delay);
}

export async function fetchDirectoryBootstrapPage(owner: DirectoryBootstrapOwner): Promise<void> {
  if (directoryBootstrap !== owner || owner.abort.signal.aborted || owner.fetching) return;
  owner.fetching = true;
  try {
    const page = await owner.client.eventPage(owner.machine.after(), { signal: owner.abort.signal });
    if (directoryBootstrap !== owner || owner.abort.signal.aborted) return;
    const authority = browserSessionAuthority;
    if (
      authority === null ||
      authority.expiresAt <= Date.now() ||
      page.sessionBindingSha256 !== authority.sessionBindingSha256 ||
      page.authorizationGeneration !== authority.authorizationGeneration
    ) {
      post({ kind: "directory-bootstrap-failed", bootstrapEpoch: owner.machine.bootstrapEpoch, code: "unauthorized", retryable: false });
      clearHubSessionCapability();
      closeDirectory();
      return;
    }
    owner.machine.present(page);
    post({
      kind: "directory-event-page",
      bootstrapEpoch: owner.machine.bootstrapEpoch,
      canonicalJson: page.canonicalJson,
      sessionBindingSha256: page.sessionBindingSha256,
      authorizationGeneration: page.authorizationGeneration,
      afterSeqExclusive: page.afterSeqExclusive,
      throughSeqInclusive: page.throughSeqInclusive,
      hasMore: page.hasMore,
      receiptSha256: page.receiptSha256,
    });
  } catch (error) {
    if (directoryBootstrap !== owner) return;
    const aborted = owner.abort.signal.aborted;
    const unauthorized = error instanceof DirectoryHttpError && error.status === 401;
    const invalid = error instanceof DirectoryHttpError || (error instanceof Error && error.message.startsWith("directory event page:"));
    const code = aborted ? "cancelled" : unauthorized ? "unauthorized" : invalid ? "invalid-page" : "transport";
    const retryable = !aborted && !unauthorized && !invalid;
    post({ kind: "directory-bootstrap-failed", bootstrapEpoch: owner.machine.bootstrapEpoch, code, retryable });
    if (retryable) scheduleDirectoryBootstrapRetry(owner);
  } finally {
    owner.fetching = false;
  }
}

export function openDirectoryBootstrapLive(owner: DirectoryBootstrapOwner, since: number): void {
  const stream = owner.client.streamAcknowledged(since, (message) => {
    if (directoryBootstrap !== owner || owner.abort.signal.aborted) return;
    void flushDirectoryQueue();
    const wake = directoryStreamWakeV1(message);
    if (wake === "none") {
      post({ kind: "directory-message", message });
      return;
    }
    const after = owner.machine.wake(wake === "origin");
    if (after === null) return;
    owner.stream?.close();
    owner.stream = null;
    if (directoryStream === stream) directoryStream = null;
    void fetchDirectoryBootstrapPage(owner);
  });
  owner.stream = stream;
  directoryStream = stream;
}

export function openDirectoryBootstrap(baseUrl: string, after: number, bootstrapEpoch: number): void {
  closeDirectory();
  const abort = new AbortController();
  const client = new DirectoryClient(baseUrl, {
    requestBaseUrl: "",
    socketGrantIssuer: createSocketGrantIssuerV1({ post: (path, options) => requestSocketGrant(baseUrl, path, options?.signal) }),
    request: browserDirectoryRequest,
  });
  const owner: DirectoryBootstrapOwner = {
    machine: new DirectoryEventPageBootstrapV1(bootstrapEpoch, after),
    client,
    abort,
    stream: null,
    retry: null,
    fetching: false,
  };
  directoryBootstrap = owner;
  directoryClient = client;
  post(directoryStatus());
  void fetchDirectoryBootstrapPage(owner);
}

/** 📇️ Every mounted space index's directory lane, by space id. */
export const spaceDirectoryLanes = new Map<string, { readonly close: () => void }>();

export function closeDirectory(): void {
  if (directoryBootstrap !== null) {
    directoryBootstrap.machine.close();
    directoryBootstrap.abort.abort(new Error("directory bootstrap closed"));
    if (directoryBootstrap.retry !== null) clearTimeout(directoryBootstrap.retry);
    directoryBootstrap.stream?.close();
    directoryBootstrap = null;
  }
  directoryStream?.close();
  directoryStream = null;
  directoryClient = null;
  for (const stream of scopedDirectoryStreams.values()) stream.close();
  scopedDirectoryStreams.clear();
  for (const lane of spaceDirectoryLanes.values()) lane.close();
  spaceDirectoryLanes.clear();
  directoryWorkerEpoch += 1;
  for (const operation of spaceArtifactCreationCatalogOperations.values()) operation.abort.abort(new Error("space artifact creation catalog: directory closed"));
  spaceArtifactCreationCatalogOperations.clear();
  for (const operation of spaceArtifactCreationOperations.values()) operation.abort.abort(new Error("space artifact creation: directory closed"));
  spaceArtifactCreationOperations.clear();
  if (directoryAdministration !== null) terminateDirectoryAdministration(directoryAdministration, "stale", "closed");
  const closing = [...directoryCommandOperations.keys()];
  directoryCommandQueue.length = 0;
  directoryCommandOperations.clear();
  for (const requestId of closing) post({ kind: "directory-command-failed", requestId, code: "closed" });
}

/** 🏁️ Retires one operation exactly once and answers its owner with a closed terminal code. */
export function settleDirectoryCommand(operation: DirectoryCommandTransportOperationV1, response: BackboneWorkerResponse | null): void {
  if (operation.settled) return;
  operation.settled = true;
  directoryCommandOperations.delete(operation.request.requestId);
  const index = directoryCommandQueue.indexOf(operation);
  if (index >= 0) directoryCommandQueue.splice(index, 1);
  if (response !== null) post(response);
}

/** 🚨️ `Some(status)` for a {@link DirectoryHttpError}-shaped rejection (the hub answered and
 * rejected the read — authz/validation, never retried); `undefined` for anything else (a network
 * failure). Structural rather than an `instanceof DirectoryHttpError` check, since this file's
 * `DirectoryClient` import and the wasm host's own may not share a class identity. */
export function directoryRejectionStatus(error: unknown): number | undefined {
  return typeof error === "object" && error !== null && "status" in error && typeof (error as { status: unknown }).status === "number" ? (error as { status: number }).status : undefined;
}

/** 🚨️ Classifies one rejection into the closed transport vocabulary without touching server text. */
export function directoryCommandErrorCode(error: unknown, aborted: boolean): DirectoryCommandErrorCodeV1 {
  if (aborted) return "cancelled";
  if (error instanceof DirectoryCommandError) return error.code;
  if (error instanceof DirectoryHttpError) return "invalid";
  return "transport";
}

/** ♻️ Drives the FIFO on every live signal from the stream. Only a transient fault retains the head
 * and its byte-identical sealed request; every terminal code answers its owner and lets the queue
 * proceed. An operation whose session or worker epoch has been replaced is suppressed, not replayed. */
export async function flushDirectoryQueue(): Promise<void> {
  if (directoryFlushing) return;
  directoryFlushing = true;
  try {
    while (directoryCommandQueue.length > 0 && directoryClient) {
      const operation = directoryCommandQueue[0]!;
      if (operation.settled) {
        directoryCommandQueue.shift();
        continue;
      }
      if (operation.sessionEpoch !== directorySessionEpoch || operation.workerEpoch !== directoryWorkerEpoch) {
        settleDirectoryCommand(operation, null);
        continue;
      }
      try {
        const receipt = await directoryClient.command(operation.request, { signal: operation.abort.signal });
        settleDirectoryCommand(operation, operation.sessionEpoch === directorySessionEpoch && operation.workerEpoch === directoryWorkerEpoch ? { kind: "directory-command-receipt", requestId: operation.request.requestId, receipt } : null);
      } catch (error) {
        const code = directoryCommandErrorCode(error, operation.abort.signal.aborted);
        if (directoryCommandErrorIsTransient(code)) break;
        settleDirectoryCommand(operation, operation.sessionEpoch === directorySessionEpoch && operation.workerEpoch === directoryWorkerEpoch ? { kind: "directory-command-failed", requestId: operation.request.requestId, code } : null);
      }
    }
  } finally {
    directoryFlushing = false;
    post(directoryStatus());
  }
}

/** 🏛️ The single shell-owned retained space-administration operation (contract §C6 + the P0
 * packet's "one operation, fixed capacity"). It is NOT a second socket and NOT a generic queue: it
 * owns exactly one canonical page, at most one in-flight command request/receipt, and at most one
 * one-shot invite capability. The capability remains worker-owned until an exact clipboard-success
 * result for the live transfer epoch. Every terminal transition erases all three BEFORE the renderer is
 * notified, so an unmount, identity change, 401/403, or scoped 4401 can never leave a secret,
 * a stale page, or a stale capability reachable. Administration mutations never auto-retry: an
 * indeterminate transport becomes `failed` ("unknown outcome / refresh required"), because only an
 * exact server receipt may advance the pane. */
const DIRECTORY_ADMINISTRATION_CAPACITY = 1;

export let directoryAdministration: DirectoryAdministrationOperationV1 | null = null;
/** 🧪️ Assigns `directoryAdministration` for the test seam, because an imported binding cannot be assigned outside its declaring module. */
export function setDirectoryAdministration(value: typeof directoryAdministration): void {
  directoryAdministration = value;
}

/** 📣️ Publishes the operation's complete renderer-visible state; never its transport internals. */
export function postDirectoryAdministrationState(operation: DirectoryAdministrationOperationV1, code?: DirectoryCommandErrorCodeV1): void {
  post({
    kind: "directory-administration-state",
    operationEpoch: operation.operationEpoch,
    spaceId: operation.spaceId,
    phase: operation.phase,
    ...(operation.canonicalJson === null ? {} : { canonicalJson: operation.canonicalJson }),
    ...(operation.receiptSha256 === null ? {} : { receiptSha256: operation.receiptSha256 }),
    ...(operation.outcome === null ? {} : { outcome: operation.outcome }),
    ...(code === undefined ? {} : { code }),
    ...(operation.inviteToken === null ? {} : { inviteCapabilityPending: true }),
    ...(operation.inviteCapabilityStatus === null ? {} : { inviteCapabilityStatus: operation.inviteCapabilityStatus }),
  });
}

/** 🧯️ Erases page, receipt, and capability, then reports one terminal phase exactly once. */
export function terminateDirectoryAdministration(operation: DirectoryAdministrationOperationV1, phase: "cancelled" | "denied" | "stale" | "failed", code?: DirectoryCommandErrorCodeV1, notify = true): void {
  if (operation.closed) return;
  operation.closed = true;
  operation.abort.abort(new Error("directory administration closed"));
  operation.pageRead?.abort(new Error("directory administration closed"));
  operation.pageRead = null;
  operation.canonicalJson = null;
  operation.page = null;
  operation.receiptSha256 = null;
  operation.outcome = null;
  operation.inviteToken = null;
  operation.inviteCapabilityStatus = null;
  operation.inviteTransferEpoch = null;
  operation.requestId = null;
  operation.authorPage = false;
  operation.phase = phase;
  if (directoryAdministration === operation) directoryAdministration = null;
  if (notify) postDirectoryAdministrationState(operation, code);
}

/** 🗑️ Settles the destructive command only after an accepted receipt and a missing page. */
export function completeDirectoryAdministrationDeletion(operation: DirectoryAdministrationOperationV1, receipt: DirectoryCommandReceiptV1): void {
  operation.closed = true;
  operation.abort.abort(new Error("directory administration deleted"));
  operation.pageRead?.abort(new Error("directory administration deleted"));
  operation.pageRead = null;
  operation.canonicalJson = null;
  operation.page = null;
  operation.receiptSha256 = receipt.receiptSha256;
  operation.outcome = receipt.outcome;
  operation.inviteToken = null;
  operation.inviteCapabilityStatus = null;
  operation.inviteTransferEpoch = null;
  operation.requestId = null;
  operation.authorPage = false;
  operation.phase = "deleted";
  if (directoryAdministration === operation) directoryAdministration = null;
  postDirectoryAdministrationState(operation);
}

/** 🔎️ Returns the live operation for `operationEpoch`, or `null` when it has been replaced. */
export function liveDirectoryAdministration(operationEpoch: number): DirectoryAdministrationOperationV1 | null {
  const operation = directoryAdministration;
  if (operation === null || operation.closed || operation.operationEpoch !== operationEpoch) return null;
  if (operation.sessionEpoch !== directorySessionEpoch || operation.workerEpoch !== directoryWorkerEpoch) {
    terminateDirectoryAdministration(operation, "stale");
    return null;
  }
  return operation;
}

/** 🚦️ Maps one page rejection onto the closed terminal vocabulary. A 401/403 clears the pane. */
export function directoryAdministrationPageTermination(error: unknown, aborted: boolean): { phase: "cancelled" | "denied" | "stale" | "failed"; code: DirectoryCommandErrorCodeV1 } {
  if (aborted) return { phase: "cancelled", code: "cancelled" };
  const status = directoryRejectionStatus(error);
  if (status === 401) return { phase: "denied", code: "unauthorized" };
  if (status === 403 || status === 404) return { phase: "denied", code: "forbidden" };
  if (status === 409 || status === 410) return { phase: "stale", code: "stale-session" };
  return { phase: "failed", code: status === undefined ? "transport" : "invalid" };
}

/** 📄️ Fetches exactly one canonical page into the operation and reports `ready`. */
export async function loadDirectoryAdministrationPage(operationEpoch: number, cursor: string | undefined, loading: "loading" | "refreshing"): Promise<void> {
  const operation = liveDirectoryAdministration(operationEpoch);
  if (operation === null) return;
  if (operation.requestId !== null || operation.phase === "submitting") {
    postDirectoryAdministrationState(operation, "capacity");
    return;
  }
  const client = directoryClient;
  if (client === null) {
    terminateDirectoryAdministration(operation, "failed", "transport");
    return;
  }
  const read = new AbortController();
  operation.pageRead?.abort(new Error("directory administration page replaced"));
  operation.pageRead = read;
  operation.phase = loading;
  operation.page = null;
  operation.authorPage = false;
  postDirectoryAdministrationState(operation);
  try {
    const page = await client.spaceAdministrationPage(operation.spaceId, cursor, { signal: read.signal });
    const live = liveDirectoryAdministration(operationEpoch);
    if (live === null || live !== operation || operation.pageRead !== read) return;
    operation.canonicalJson = page.canonicalJson;
    operation.page = page.page;
    operation.authorPage = page.page.access === "author";
    if (!operation.authorPage) {
      operation.inviteToken = null;
      operation.inviteCapabilityStatus = null;
      operation.inviteTransferEpoch = null;
    }
    operation.phase = "ready";
    postDirectoryAdministrationState(operation);
  } catch (error) {
    if (operation.closed || operation.pageRead !== read) return;
    const termination = directoryAdministrationPageTermination(error, read.signal.aborted);
    terminateDirectoryAdministration(operation, termination.phase, termination.code);
  } finally {
    if (operation.pageRead === read) operation.pageRead = null;
  }
}

/** 🆕️ Installs the one retained operation for exactly one space, replacing any predecessor. */
export function openDirectoryAdministration(operationEpoch: number, spaceId: string): void {
  const live = directoryAdministration === null ? 0 : 1;
  if (live >= DIRECTORY_ADMINISTRATION_CAPACITY && directoryAdministration !== null) terminateDirectoryAdministration(directoryAdministration, "cancelled", "cancelled");
  if (!Number.isSafeInteger(operationEpoch) || operationEpoch < 0 || spaceId.length === 0 || new TextEncoder().encode(spaceId).byteLength > 256) {
    post({ kind: "directory-administration-state", operationEpoch, spaceId, phase: "failed", code: "invalid" });
    return;
  }
  const operation: DirectoryAdministrationOperationV1 = {
    operationEpoch,
    spaceId,
    sessionEpoch: directorySessionEpoch,
    workerEpoch: directoryWorkerEpoch,
    abort: new AbortController(),
    phase: "loading",
    canonicalJson: null,
    page: null,
    pageRead: null,
    receiptSha256: null,
    outcome: null,
    inviteToken: null,
    inviteCapabilityStatus: null,
    inviteTransferEpoch: null,
    nextInviteTransferEpoch: 0,
    requestId: null,
    authorPage: false,
    closed: false,
  };
  directoryAdministration = operation;
  void loadDirectoryAdministrationPage(operationEpoch, undefined, "loading");
}
