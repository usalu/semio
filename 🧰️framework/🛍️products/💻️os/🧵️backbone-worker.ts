// #region Header
/**
 * 🧵️ `🧵️backbone-worker.ts` — browser backbone loader. Authenticated hub
 * document lifecycles are owned here so D1 issue/exchange/WebSocket authority cannot be bypassed
 * when the Rust WASM worker resolves; other lanes use Rust when available and the TypeScript twin
 * otherwise.
 */
// #endregion Header

import type { ArtifactBootstrapControl, ArtifactBootstrapProgress, ArtifactPresencePeer, ClientFrame, MutationEnvelope, ServerFrame, WireAckStage, WireArtifactBootstrap, WireFrontierSummary, WireLane, WireMutationEnvelope } from "@semio-tech/framework-replication";
import type { ArtifactActorConfig, ArtifactActorMsg, ArtifactBootstrapWorkerEvent, ArtifactEvent, ArtifactSyncStatus, BackboneWorkerRequest, BackboneWorkerResponse, BackboneWorkerWireMessage, BrowserBrokerPortResponseV1, CanonicalDirectoryEventPageV1, CommandAckOutcome, DirectoryAcknowledgedStream, DirectoryAdministrationPhaseV1, DirectoryCommand, DirectoryEventPageAckV1, DirectoryStreamMessage, DocumentScope, PersistenceBinding, RemoteState, SocketGrantReceiptV1 } from "./🟦️";
import { ArtifactBootstrapAssembler, DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS, decodeClientFrame, decodePresencePeer, decodeServerFrame, encodeClientFrame, encodePresencePeer, encodeServerFrame } from "@semio-tech/framework-replication";
import { DirectoryClient, DirectoryCommandError, DirectoryHttpError, HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS, createSocketGrantIssuerV1, decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, decodeDocumentPackBytes, decodePackValue, documentRuntimeKeyV1, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse, encodeDocumentPackBytes, encodePackValue, parseBrowserBrokerPortRequestV1, parseSocketGrantReceiptV1, socketGrantProtocolsV1 } from "./🟦️";
import type { DirectoryCommandErrorCodeV1, DirectoryCommandOutcomeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryCommandResultV1, DocumentExecutionTargetLeaseFieldsV1, DocumentExecutionTargetProgressV1, DocumentExecutionTargetStatusCodeV1, DocumentOpenIntentV1, DocumentOpenPlanV1, GisMapInferencePortCodeV1, GisMapInferencePortEventV1, GisMapInferencePortStatusV1, GisMapInferencePreviewV1 } from "./🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES, gisMapInferenceCodeFromStatusV1, gisMapInferencePortTerminalV1, idleGisMapInferencePortStatusV1, parseGisMapInferenceApprovalReceiptV1, parseGisMapInferenceEventPageV1, parseGisMapInferenceJobReceiptV1, reduceGisMapInferencePortV1, sealGisMapInferenceApprovalRequestV1, sealGisMapInferenceJobRequestV1 } from "./🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1, directoryCommandErrorIsTransient, directoryCommandRequestJson, directoryCommandSha256, documentExecutionTargetStatusRoleV1, leaseFieldsFromPlanV1, parseDocumentExecutionTargetLeaseFieldsV1, parseDocumentOpenIntentV1, parseDocumentOpenPlanV1, parseDocumentPlanSocketGrantIntentV1, sameLeaseFieldsV1, sealDirectoryCommandReceiptV1, sealDirectoryCommandRequestV1 } from "./🔨️modules/📇️directory/🧬️schema/🟦️.ts";
/** 🔏️ First-party BLAKE3 runtime module — Web Crypto supplies SHA-256 but has no BLAKE3, so a
 * verified execution-target component is hashed with the repository's own implementation. */
import { blake3Hex } from "@semio-tech/framework";
/** 🎚️ config-lane attach (contract freeze §4) — `OpeningPreferences` is a kernel type (domain-neutral
 * framework), never redefined here; see this file's `🔖️ConfigLane` region. */
import type { OpeningPreferences } from "@semio-tech/framework";
/** 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-backbone): the shared event-driven primitives
 * from packet `web-glue` — full-jitter reconnect backoff, single-flight revalidation, and a fetch
 * with a composed timeout. Reused rather than reimplemented (see this file's `🔖️Folder`/`🔖️Hub`
 * regions for how each is wired in). */
import { fetchWithTimeout, latestWins, retryWithJitteredBackoff, type FetchTimeoutResponse } from "@semio-tech/framework";
/** 🪪️ Identity config facet (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS
 * §C3) — self-contained TS twin (see that module's header doc for why); never redefined here. */
import type { Identity } from "./🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️";

type RustWorkerHost = {
  handleRequestBytes(bytes: Uint8Array): void;
  postReady(): void;
};

let rustHost: RustWorkerHost | null = null;

// 🧵️ Built as a variable rather than a string literal so Rollup's static import analysis — including
// the separate sub-build `vite:worker-import-meta-url` runs for this very file — can't see a resolvable
// specifier at all and leaves the `import()` genuinely dynamic; a real bundler-visible specifier here
// (even `@vite-ignore`d) still gets probed by that sub-build and, since the package is never actually
// published, either fails the build outright or emits a phantom `__vite-browser-external-*.js` chunk
// reference that 404s in production. Left dynamic, the browser's native module loader simply rejects
// the unresolvable bare specifier at runtime, which the `catch` below already treats as "unavailable".
const RUST_SYNC_WORKER_MODULE_SPECIFIER = "@semio-tech/store-worker";

async function ensureRustHost(): Promise<RustWorkerHost | null> {
  if (rustHost) return rustHost;
  if (typeof WebAssembly === "undefined") return null;
  try {
    const module = await import(RUST_SYNC_WORKER_MODULE_SPECIFIER);
    await module.default();
    rustHost = new module.BackboneWorkerHost() as RustWorkerHost;
    return rustHost;
  } catch {
    return null;
  }
}

const rustHostPromise = ensureRustHost();

type DocumentExecutionOwner = "typescript" | "rust";
type DocumentExecutionOwnerEntry = Readonly<{ owner: DocumentExecutionOwner; documentId: string; spaceId?: string }>;

const documentExecutionOwners = new Map<string, DocumentExecutionOwnerEntry>();

function documentRuntimeKeyForConfig(config: ArtifactActorConfig): string {
  const hub = hubBinding(config);
  return hub === null
    ? documentRuntimeKeyV1({ kind: "local", documentId: config.documentId })
    : documentRuntimeKeyV1({ kind: "hub", spaceId: hub.spaceId, documentId: config.documentId });
}

function ownedDocumentRuntimeKey(documentId: string, spaceId?: string): string | null {
  if (spaceId !== undefined) return documentRuntimeKeyV1({ kind: "hub", spaceId, documentId });
  const localKey = documentRuntimeKeyV1({ kind: "local", documentId });
  if (documentExecutionOwners.has(localKey)) return localKey;
  const matches = [...documentExecutionOwners].filter(([, entry]) => entry.documentId === documentId);
  return matches.length === 1 ? matches[0]![0] : matches.length === 0 ? localKey : null;
}

/** 🛡️ Keeps one document's open/send/close lifecycle on a single execution owner and
 * reserves every hub-bound document for the authenticated browser D1 transport. */
function dispatchBackboneWorkerRequest(request: BackboneWorkerRequest, host: RustWorkerHost | null, typescriptDispatch: (request: BackboneWorkerRequest) => void = handleTsRequest): void {
  const rustDispatch = (value: BackboneWorkerRequest): void => host?.handleRequestBytes(encodeBackboneWorkerRequest(value));
  if (request.kind === "directory-bootstrap-open" || request.kind === "directory-bootstrap-ack" || request.kind === "directory-bootstrap-reject" || request.kind === "directory-bootstrap-close") {
    typescriptDispatch(request);
    return;
  }
  if (request.kind === "open") {
    const next: DocumentExecutionOwner = hubBinding(request) === null && host !== null ? "rust" : "typescript";
    const runtimeKey = documentRuntimeKeyForConfig(request);
    const previous = documentExecutionOwners.get(runtimeKey);
    if (previous !== undefined && previous.owner !== next) {
      const close: BackboneWorkerRequest = { kind: "close", documentId: request.documentId, ...(previous.spaceId === undefined ? {} : { spaceId: previous.spaceId }) };
      if (previous.owner === "typescript") typescriptDispatch(close);
      else rustDispatch(close);
    }
    const hub = hubBinding(request);
    documentExecutionOwners.set(runtimeKey, { owner: next, documentId: request.documentId, ...(hub === null ? {} : { spaceId: hub.spaceId }) });
    if (next === "typescript") typescriptDispatch(request);
    else rustDispatch(request);
    return;
  }
  if (request.kind === "send" || request.kind === "close") {
    const runtimeKey = ownedDocumentRuntimeKey(request.documentId, request.spaceId);
    if (runtimeKey === null) return;
    const owner = documentExecutionOwners.get(runtimeKey)?.owner ?? (host === null ? "typescript" : "rust");
    if (owner === "typescript") typescriptDispatch(request);
    else rustDispatch(request);
    if (request.kind === "close") documentExecutionOwners.delete(runtimeKey);
    return;
  }
  if (host === null) typescriptDispatch(request);
  else rustDispatch(request);
}

function isBackboneWorkerWireMessage(message: unknown): message is BackboneWorkerWireMessage {
  return typeof message === "object" && message !== null && "wire" in message && (message as BackboneWorkerWireMessage).wire instanceof Uint8Array;
}

function decodeWorkerRequest(message: BackboneWorkerWireMessage): BackboneWorkerRequest {
  return decodeBackboneWorkerRequest(message.wire);
}

const workerScope = typeof self !== "undefined" && !Reflect.has(self, "document") ? self : null;

if (workerScope) {
  workerScope.onmessage = (messageEvent: MessageEvent<unknown>) => {
    if (typeof messageEvent.data === "object" && messageEvent.data !== null && Reflect.get(messageEvent.data, "kind") === "semio-browser-broker-port" && Reflect.get(messageEvent.data, "port") instanceof MessagePort) {
      attachLocalBrokerPort(Reflect.get(messageEvent.data, "port") as MessagePort);
      return;
    }
    // 🛡️ React DevTools and other injectors postMessage into every Worker; ignore non-wire traffic.
    if (!isBackboneWorkerWireMessage(messageEvent.data)) return;
    const request = decodeWorkerRequest(messageEvent.data);
    void rustHostPromise.then((host) => {
      dispatchBackboneWorkerRequest(request, host);
    });
  };
}

void rustHostPromise.then((host) => {
  if (host) host.postReady();
  else post({ kind: "ready" });
});

//#region 🔖️TsFallback

//#region 🔖️Constants
/** 🛰️ Must match `framework/os/core/js/index.ts`'s `BACKBONE_ENDPOINT_PATH`. */
const FOLDER_ENDPOINT_PATH = "/semio-backbone";
/** 🛟️ Sanity-fallback poll cadence (finding 1): SSE is the primary wake signal now, so this only
 * ever fires while {@link ArtifactState.sseHealthy} is `false` — a slow, jittered self-heal for
 * "the SSE stream looks fine but nothing has arrived in a while", not the primary path. Jittered
 * per tick (not a fixed `setInterval`) so many documents reconnecting/self-healing together never
 * synchronize into a request burst. */
const SANITY_POLL_MIN_MS = 24_000;
const SANITY_POLL_MAX_MS = 36_000;
/** 🔁️ SSE reconnect backoff (finding 2) — deliberately faster/tighter than the hub's
 * {@link HUB_RECONNECT_MIN_MS}/{@link HUB_RECONNECT_MAX_MS}: losing the folder watch stream is
 * cheap to retry (a GET, no handshake state) and {@link SANITY_POLL_MIN_MS}'s fallback is the
 * user-visible safety net either way. */
const SSE_RECONNECT_MIN_MS = 1_000;
const SSE_RECONNECT_MAX_MS = 30_000;
/** ⏱️ Caps how long any single folder/blob fetch can hang (finding 3) — composed with
 * {@link fetchWithTimeout} so a stalled dev-middleware response can never pin a document forever. */
const FOLDER_FETCH_TIMEOUT_MS = 15_000;
const BLOB_FETCH_TIMEOUT_MS = 15_000;
/** 🗃️ Bounded local outbound-mutation queue (finding 5) — see {@link rejectMutationQueueOverflow}
 * for the overflow contract: reject and report, never silently drop. */
const PENDING_MUTATIONS_QUEUE_LIMIT = 2_000;
const ARTIFACT_BOOTSTRAP_DEADLINE_MS = 15_000;
const ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES = 4_096;
// 🔁️ HUB_RECONNECT_MIN_MS/MAX_MS moved to `🟦️.ts`'s `🔖️HubBinding` region (imported above)
// — single source of truth shared with `DirectoryClient.stream`'s reconnect loop.
/** ♻️ Coordinator follow-up (finding 4b): how long a hub OR SSE connection must stay open before a
 * SUBSEQUENT drop is allowed to reset that transport's backoff back near its floor, instead of
 * continuing to grow from whatever `retryWithJitteredBackoff` attempt count it was already on.
 * Deliberately NOT "the socket opened" — a server that accepts a connection and immediately drops
 * it in a fast loop must still see the backoff climb (that IS the failure mode the backoff exists
 * for), so the threshold has to be comfortably longer than any such instant-drop cycle. Half of
 * {@link HUB_RECONNECT_MAX_MS}/{@link SSE_RECONNECT_MAX_MS} (both 30s): long enough that no
 * single accept-then-drop attempt could plausibly cross it, short enough that a connection which
 * has been genuinely healthy for a modest stretch still gets credit before its next blip. */
const SUSTAINED_HEALTHY_MS = 15_000;
//#endregion 🔖️Constants

//#region 🔖️Reconnect
/**
 * ♻️ Coordinator follow-up (finding 4b): drives `attempt` (one physical connection's full
 * lifecycle — connect, stay open, eventually close) through {@link retryWithJitteredBackoff}
 * forever, but as a LOOP of fresh calls rather than one long-lived call. `attempt` resolving is
 * this loop's signal that the connection stayed open long enough to count as sustainedly healthy
 * before it (ordinarily) closed — see {@link connectHubOnce}/{@link connectSseOnce} — so the NEXT
 * cycle starts a brand-new {@link retryWithJitteredBackoff} call with its own zeroed internal
 * attempt/backoff state, rather than inheriting a large accumulated delay from earlier, already-
 * resolved blips. `attempt` rejecting (a close before sustained health) is absorbed entirely
 * inside the SAME `retryWithJitteredBackoff` call, so its backoff keeps growing across those —
 * exactly the "rapid accept-then-drop cycling still backs off" case the sustained-health gate
 * exists to protect.
 *
 * `retryWithJitteredBackoff`'s own signature has no notion of "reset now" — it only stops
 * retrying on success or abort — so this reset cannot be expressed by calling it once; looping
 * fresh calls from here is the only way to get a real reset without editing
 * `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` (outside this packet's owned path).
 */
async function reconnectForever(signal: AbortSignal, attempt: () => Promise<void>, minMs: number, maxMs: number): Promise<void> {
  while (!signal.aborted) {
    try {
      await retryWithJitteredBackoff(attempt, { minMs, maxMs, signal });
    } catch {
      return; // 🛑 only reachable via abort — `attempt` never lets a real failure escape the retry loop above.
    }
  }
}
//#endregion 🔖️Reconnect

//#region 🔖️DocumentState
type ArtifactState = {
  runtimeKey: string;
  config: ArtifactActorConfig;
  openClientInstanceId: string;
  actor: string;
  hubActorReady: boolean;
  pendingSocketActorId: string | null;
  channel: BroadcastChannel;
  socket: WebSocket | null;
  presenceAuthority: Readonly<{ socket: WebSocket; scope: DocumentScope; verifiedSurfaceId: string }> | null;
  /** 🛑️ Aborted once, in {@link closeArtifact} — cancels every in-flight folder/blob fetch this
   * document owns and unblocks any pending {@link retryWithJitteredBackoff} delay for its hub/SSE
   * reconnect loops immediately (finding 3). Never re-created; a closed document stays closed. */
  docAbort: AbortController;
  /** 🪪️ The private verified execution-target owner for a non-`react` hub target, live only while
   * this document's plan/socket authority is. Never posted, cloned or encoded. */
  executionTargetLease: DocumentExecutionTargetLease | null;
  /** 🛟️ Handle for the recursive, jittered sanity-poll reschedule (finding 1) — a plain
   * `ReturnType<typeof setTimeout>`, not `setInterval`, because each tick schedules its OWN next
   * delay with fresh jitter rather than ticking on a fixed period. */
  sanityPollTimer: ReturnType<typeof setTimeout> | null;
  /** 📡️ Explicit "is the folder SSE stream currently up" flag (finding 2) — {@link startSanityPolling}
   * reads this to decide whether a given tick actually revalidates or is a no-op, and it is this
   * file's only source of truth for that question (never inferred from `EventSource.readyState`,
   * which a fake `EventSource` test double need not implement). */
  sseHealthy: boolean;
  /** 🥇️ Single-flight folder revalidation (finding 1) — built once per document with
   * {@link latestWins} over {@link pollFolderOnce}, so the SSE `onmessage` wake, the sanity-poll
   * tick, and an `externalChanged` local message all share the SAME in-flight guard and can never
   * stack overlapping reads. A no-op placeholder until {@link openArtifact} sees a folder binding. */
  revalidateFolder: () => Promise<void>;
  reconnectDelayMs: number;
  /** 🗃️ Outbound mutations not yet handed to a LIVE hub socket (finding 5) — distinct from
   * `pendingBatches`, which holds envelopes already sent and awaiting an `Ack`. Populated by
   * {@link relayMutationsToHub} when the socket isn't open and by a dead socket's `onclose` (any
   * batch that socket never acked moves back here), drained by {@link handleHubFrame}'s `Welcome`
   * branch on every successful (re)connect — this is the "flushed on reconnect" half of finding 5. */
  outbox: MutationEnvelope[];
  pendingMutations: MutationEnvelope[];
  status: ArtifactSyncStatus;
  /** 🏔️ Last frontier the hub reported (`Welcome.server_frontier` / `Commands.frontier` /
   * `Ack.frontier`) — the wire-v2 replacement for the old `sinceVersion: number` counter. */
  frontier: WireFrontierSummary | null;
  pendingResumeToken: string | null;
  requiredTailFrontier: WireFrontierSummary | null;
  artifactBootstrap: ArtifactBootstrapAssembler | null;
  artifactBootstrapDeadlineMs: number | null;
  artifactBootstrapProgress: ArtifactBootstrapProgress[];
  currentPack: Uint8Array | null;
  currentSpr: Uint8Array | null;
  hubFrameChain: Promise<void>;
  /** 🎟️ The hub's last `Welcome.resume_token`, echoed back on the next `hello` after a reconnect. */
  resumeToken: string | null;
  /** 🎨️ This connection's hub-assigned session color (`ServerFrame::Session.color`) — `null` until
   * the hub sends it (or for a folder-only document, which never connects to a hub). Stamped onto
   * every outbound heartbeat via {@link stampSession}. */
  sessionColor: number | null;
  /** 🧺️ Outbound `Commands` batches awaiting an `Ack`, keyed by `batch_id`. */
  pendingBatches: Map<number, MutationEnvelope[]>;
  nextBatchId: number;
  /** ⏰️ Logical tick counter for {@link nextWireTimestamp} on every outbound wire envelope. */
  hlcCounter: number;
  closed: boolean;
};

const artifacts = new Map<string, ArtifactState>();
let workerPostTestSink: ((message: BackboneWorkerResponse) => void) | null = null;

function artifactRuntimeKey(documentId: string, spaceId?: string): string | null {
  if (spaceId !== undefined) return documentRuntimeKeyV1({ kind: "hub", spaceId, documentId });
  const localKey = documentRuntimeKeyV1({ kind: "local", documentId });
  if (artifacts.has(localKey)) return localKey;
  const matches = [...artifacts].filter(([, state]) => state.config.documentId === documentId);
  return matches.length === 1 ? matches[0]![0] : matches.length === 0 ? localKey : null;
}

function artifactState(documentId: string, spaceId?: string): ArtifactState | undefined {
  const runtimeKey = artifactRuntimeKey(documentId, spaceId);
  return runtimeKey === null ? undefined : artifacts.get(runtimeKey);
}

function post(message: BackboneWorkerResponse): void {
  if (workerPostTestSink !== null) {
    workerPostTestSink(message);
    return;
  }
  workerScope?.postMessage({ wire: encodeBackboneWorkerResponse(message) });
}

function artifactScope(state: ArtifactState): DocumentScope | undefined {
  const binding = hubBinding(state.config);
  return binding === null ? undefined : { spaceId: binding.spaceId, documentId: state.config.documentId };
}

function emitEvent(state: ArtifactState, event: ArtifactEvent): void {
  const scope = artifactScope(state);
  if (event.kind === "presence") {
    const authority = state.presenceAuthority;
    const verified = authority !== null
      && authority.socket === state.socket
      && scope !== undefined
      && authority.scope.spaceId === scope.spaceId
      && authority.scope.documentId === scope.documentId;
    post({
      kind: "event",
      documentId: state.config.documentId,
      event: verified ? event : { ...event, peers: [] },
      ...(scope === undefined ? {} : { scope }),
      ...(verified ? { verifiedSurfaceId: authority.verifiedSurfaceId } : {}),
    });
    return;
  }
  post({ kind: "event", documentId: state.config.documentId, event, ...(scope === undefined ? {} : { scope }) });
}

const SOCKET_GRANT_REQUEST_TIMEOUT_MS = 10_000;
const SOCKET_GRANT_REQUEST_LIMIT = 256;
const DOCUMENT_OPEN_RESPONSE_MAX_BYTES = 64 * 1024;
const BROWSER_BROKER_PROOF_DOMAIN = new TextEncoder().encode("semio/browser-broker-proof/v1\0");
const BROWSER_BROKER_PROOF_TTL_MS = 15_000;
let socketGrantTestIssue: ((baseUrl: string, path: string, signal?: AbortSignal) => Promise<SocketGrantReceiptV1>) | null = null;
let localBrowserBrokerProof: Uint8Array | undefined;
let localBrowserBrokerProofExpiresAtMs = 0;
let localBrowserBrokerQueue: Promise<void> = Promise.resolve();
let localBrowserBrokerQueued = 0;
let localBrowserBrokerPort: MessagePort | undefined;
const localBrowserBrokerRpcControllers = new Map<string, AbortController>();

function hexBytes(value: string): Uint8Array | undefined {
  if (!/^[0-9a-f]{64}$/u.test(value)) return undefined;
  return Uint8Array.from({ length: 32 }, (_, index) => Number.parseInt(value.slice(index * 2, index * 2 + 2), 16));
}

function bytesHex(value: Uint8Array): string {
  return Array.from(value, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

async function browserBrokerProofDigest(value: Uint8Array): Promise<Uint8Array> {
  const input = new Uint8Array(BROWSER_BROKER_PROOF_DOMAIN.byteLength + value.byteLength);
  input.set(BROWSER_BROKER_PROOF_DOMAIN);
  input.set(value, BROWSER_BROKER_PROOF_DOMAIN.byteLength);
  const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", input));
  input.fill(0);
  return digest;
}

function clearLocalBrowserBrokerProof(): void {
  localBrowserBrokerProof?.fill(0);
  localBrowserBrokerProof = undefined;
  localBrowserBrokerProofExpiresAtMs = 0;
}

function installLocalBrowserBrokerProof(proof: string): boolean {
  const decoded = hexBytes(proof);
  if (!decoded || localBrowserBrokerProof) {
    decoded?.fill(0);
    return false;
  }
  localBrowserBrokerProof = decoded;
  localBrowserBrokerProofExpiresAtMs = Date.now() + BROWSER_BROKER_PROOF_TTL_MS;
  return true;
}

async function browserBrokerFetch(input: string, init: RequestInit = {}, options: { readonly timeoutMs: number; readonly signal?: AbortSignal }): Promise<FetchTimeoutResponse> {
  if (options.signal?.aborted) throw options.signal.reason ?? new Error("browser broker cancelled");
  if (localBrowserBrokerQueued >= 64) throw new Error("browser broker capacity exceeded");
  localBrowserBrokerQueued += 1;
  let resolveTurn: () => void = () => undefined;
  const prior = localBrowserBrokerQueue;
  localBrowserBrokerQueue = new Promise<void>((resolve) => { resolveTurn = resolve; });
  await prior;
  try {
    const current = localBrowserBrokerProof;
    if (!current || Date.now() > localBrowserBrokerProofExpiresAtMs) {
      clearLocalBrowserBrokerProof();
      throw new Error("browser broker rebootstrap required");
    }
    const next = crypto.getRandomValues(new Uint8Array(32));
    const nextDigest = await browserBrokerProofDigest(next);
    const currentHex = bytesHex(current);
    clearLocalBrowserBrokerProof();
    try {
      const response = await fetchWithTimeout(input, {
        ...init,
        headers: { ...(init.headers as Record<string, string> | undefined), "x-semio-browser-broker": currentHex, "x-semio-browser-broker-next": bytesHex(nextDigest) },
      }, options);
      if (response.headers.get("x-semio-browser-broker-advanced") === "1" && response.status !== 401) {
        localBrowserBrokerProof = next;
        localBrowserBrokerProofExpiresAtMs = Date.now() + BROWSER_BROKER_PROOF_TTL_MS;
      } else {
        next.fill(0);
        nextDigest.fill(0);
        throw new Error("browser broker rebootstrap required");
      }
      nextDigest.fill(0);
      return response;
    } catch {
      next.fill(0);
      nextDigest.fill(0);
      throw new Error("browser broker rebootstrap required");
    }
  } finally {
    localBrowserBrokerQueued -= 1;
    resolveTurn();
  }
}

function attachLocalBrokerPort(port: MessagePort): void {
  localBrowserBrokerPort?.close();
  localBrowserBrokerPort = port;
  port.onmessage = (event: MessageEvent<unknown>) => {
    const message = parseBrowserBrokerPortRequestV1(event.data);
    if (!message) return;
    if (message.kind === "initialize") {
      const response: BrowserBrokerPortResponseV1 = { kind: "initialized", ok: installLocalBrowserBrokerProof(message.proof) };
      port.postMessage(response);
      return;
    }
    if (message.kind === "cancel") {
      localBrowserBrokerRpcControllers.get(message.requestId)?.abort();
      return;
    }
    if (message.kind !== "request") return;
    if (localBrowserBrokerRpcControllers.size >= 64) {
      const response: BrowserBrokerPortResponseV1 = { kind: "response", requestId: message.requestId, status: 503, body: "" };
      port.postMessage(response);
      return;
    }
    const requestId = message.requestId;
    const controller = new AbortController();
    localBrowserBrokerRpcControllers.set(requestId, controller);
    void browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 2_000, signal: controller.signal }).then(async (response) => {
      const body = await response.text();
      const result: BrowserBrokerPortResponseV1 = { kind: "response", requestId, status: response.status, body };
      port.postMessage(result);
    }).catch((error: unknown) => {
      const result: BrowserBrokerPortResponseV1 = { kind: "response", requestId, status: error instanceof Error && error.message === "browser broker rebootstrap required" ? 428 : 503, body: "" };
      port.postMessage(result);
    }).finally(() => localBrowserBrokerRpcControllers.delete(requestId));
  };
  port.start();
}

/** 🎫 Mints one audience-bound grant inside the credential-owning broker worker. */
async function requestSocketGrant(baseUrl: string, path: string, signal?: AbortSignal): Promise<SocketGrantReceiptV1> {
  if (socketGrantTestIssue) return socketGrantTestIssue(baseUrl, path, signal);
  if (signal?.aborted || !baseUrl) throw new Error("socket grant: cancelled");
  const response = await browserBrokerFetch(`/_semio/hub${path}`, { method: "POST" }, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal });
  if (!response.ok) throw new Error("socket grant: unavailable");
  try {
    return parseSocketGrantReceiptV1(await response.json());
  } catch (error) {
    clearLocalBrowserBrokerProof();
    throw error;
  }
}

type BrowserDocumentSocketAuthorityV1 = Readonly<{
  receipt: SocketGrantReceiptV1;
  schema: string;
  packSchemaHash: readonly number[];
  parentDialect?: DocumentOpenPlanV1["parentDialect"];
  surfaceId?: string;
}>;

async function readDocumentOpenJson(response: Response): Promise<unknown> {
  const contentLength = Number(response.headers.get("content-length") ?? "0");
  if (!Number.isSafeInteger(contentLength) || contentLength < 0 || contentLength > DOCUMENT_OPEN_RESPONSE_MAX_BYTES) throw new Error("document open: invalid response");
  if (!response.body) throw new Error("document open: invalid response");
  const reader = response.body.getReader();
  const chunks: Uint8Array[] = [];
  let retained = 0;
  try {
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      retained += value.byteLength;
      if (retained > DOCUMENT_OPEN_RESPONSE_MAX_BYTES) {
        await reader.cancel();
        throw new Error("document open: invalid response");
      }
      chunks.push(value);
    }
    const bytes = new Uint8Array(retained);
    let offset = 0;
    for (const chunk of chunks) {
      bytes.set(chunk, offset);
      offset += chunk.byteLength;
      chunk.fill(0);
    }
    try {
      return JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
    } finally {
      bytes.fill(0);
    }
  } finally {
    for (const chunk of chunks) chunk.fill(0);
  }
}

//#region 🪪️ExecutionTargetLease
/** 📶️ Bounded streaming progress unit for a verified execution-target body. */
const EXECUTION_TARGET_PROGRESS_UNIT_BYTES = 64 * 1024;
/** 🧯️ Bound on the strict lease manifest JSON body. */
const EXECUTION_TARGET_MANIFEST_MAX_BYTES = 8 * 1024;

type DocumentExecutionTargetAssetV1 = "manifest" | "component" | "descriptor";

const documentExecutionTargetLeaseMintToken = Symbol("semio.os.document-execution-target-lease.mint/v1");
const documentExecutionTargetLeaseBrand = Symbol("semio.os.document-execution-target-lease/v1");

/** 🪪️ Private non-serializable owner of one verified execution target. Its constructor is
 * unreachable outside this module (a module-private mint token), it owns the verified component and
 * descriptor buffers plus any private module URL, and public callers only ever receive a frozen copy
 * of {@link DocumentExecutionTargetLeaseFieldsV1}. It is never posted, cloned or encoded. */
class DocumentExecutionTargetLease {
  readonly [documentExecutionTargetLeaseBrand] = true;
  #fields: DocumentExecutionTargetLeaseFieldsV1;
  #hubOrigin: string;
  #component: Uint8Array | null;
  #descriptor: Uint8Array | null;
  #moduleUrl: string | null = null;
  #live = true;

  constructor(token: symbol, fields: DocumentExecutionTargetLeaseFieldsV1, hubOrigin: string, component: Uint8Array, descriptor: Uint8Array) {
    if (token !== documentExecutionTargetLeaseMintToken) throw new Error("document execution target lease: private constructor");
    this.#fields = fields;
    this.#hubOrigin = hubOrigin;
    this.#component = component;
    this.#descriptor = descriptor;
    Object.freeze(this);
  }

  get live(): boolean {
    return this.#live;
  }

  get hubOrigin(): string {
    return this.#hubOrigin;
  }

  fields(): DocumentExecutionTargetLeaseFieldsV1 {
    if (!this.#live) throw new Error("document execution target lease: dropped");
    return Object.freeze(structuredClone(this.#fields));
  }

  drop(): void {
    this.#live = false;
    this.#component?.fill(0);
    this.#descriptor?.fill(0);
    this.#component = null;
    this.#descriptor = null;
    if (this.#moduleUrl !== null) {
      URL.revokeObjectURL(this.#moduleUrl);
      this.#moduleUrl = null;
    }
  }
}

/** 🧾️ Receipt-free lease projection of one parsed plan, at the byte lengths under comparison. */
function receiptFreeFields(plan: DocumentOpenPlanV1, byteLengths: { readonly component: number; readonly descriptor: number }): DocumentExecutionTargetLeaseFieldsV1 {
  return leaseFieldsFromPlanV1(plan, byteLengths);
}

function executionTargetHex(bytes: Uint8Array): string {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

async function executionTargetSha256Hex(bytes: Uint8Array): Promise<string> {
  return executionTargetHex(new Uint8Array(await crypto.subtle.digest("SHA-256", bytes)));
}

function executionTargetAssetPath(spaceId: string, documentId: string, asset: DocumentExecutionTargetAssetV1): string {
  return `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/execution-target/${asset}`;
}

/** 🚪️ The only broker operation this lane owns: exactly the three protected document-scoped asset
 * calls, always POST, always the bounded `DocumentOpenIntentV1` body, never a package, digest,
 * generation, path or receipt selector. Anything else is denied before a request exists. */
function browserExecutionTargetAssetRequest(binding: Extract<PersistenceBinding, { kind: "hub" }>, documentId: string, asset: DocumentExecutionTargetAssetV1, intent: DocumentOpenIntentV1, options: { readonly timeoutMs: number; readonly signal: AbortSignal }): Promise<FetchTimeoutResponse> {
  const path = executionTargetAssetPath(binding.spaceId, documentId, asset);
  if (!/^\/spaces\/[^/?#]+\/documents\/[^/?#]+\/execution-target\/(?:manifest|component|descriptor)$/u.test(path)) return Promise.reject(new Error("document execution target: operation denied"));
  if (intent.scope.spaceId !== binding.spaceId || intent.scope.documentId !== documentId) return Promise.reject(new Error("document execution target: operation denied"));
  return browserBrokerFetch(`/_semio/hub${path}`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(intent) }, options);
}

/** 🔭️ Observation seam for the execution-target live region, mirroring {@link socketGrantTestIssue}:
 * a worker scope posts, and a harness without one still sees the exact bounded payload. */
let executionTargetStatusObserver: ((status: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>) => void) | null = null;

function emitExecutionTargetStatus(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub" }>, code: DocumentExecutionTargetStatusCodeV1, progress?: DocumentExecutionTargetProgressV1): void {
  const scope = { spaceId: binding.spaceId, documentId: state.config.documentId };
  const status: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }> = { kind: "execution-target-status", documentId: state.config.documentId, spaceId: binding.spaceId, scope, code, ...(progress ? { progress } : {}) };
  executionTargetStatusObserver?.(status);
  post(status);
}

/** 📥️ Streams the strict lease manifest under its own explicit maximum, independently of the larger
 * plan/grant response bound. The body is wiped after decoding. */
async function readExecutionTargetManifestJson(response: FetchTimeoutResponse): Promise<unknown> {
  const declared = response.headers.get("content-length");
  if (declared !== null && !(Number.isSafeInteger(Number(declared)) && Number(declared) >= 1 && Number(declared) <= EXECUTION_TARGET_MANIFEST_MAX_BYTES)) throw new Error("document execution target: invalid manifest");
  if (!response.body) throw new Error("document execution target: invalid manifest");
  const reader = response.body.getReader();
  const chunks: Uint8Array[] = [];
  let retained = 0;
  try {
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      retained += value.byteLength;
      if (retained > EXECUTION_TARGET_MANIFEST_MAX_BYTES) throw new Error("document execution target: invalid manifest");
      chunks.push(value);
    }
    if (retained === 0) throw new Error("document execution target: invalid manifest");
    const bytes = new Uint8Array(retained);
    let offset = 0;
    for (const chunk of chunks) {
      bytes.set(chunk, offset);
      offset += chunk.byteLength;
    }
    try {
      return JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
    } finally {
      bytes.fill(0);
    }
  } finally {
    await reader.cancel().catch(() => undefined);
    for (const chunk of chunks) chunk.fill(0);
  }
}

/** 📥️ Streams one bounded asset body, enforcing the declared byte length, the shared maximum and the
 * operation's cancellation/deadline before every chunk, and reporting bounded progress only. */
async function readExecutionTargetBody(response: FetchTimeoutResponse, expectedByteLength: number, maxBytes: number, stage: "component" | "descriptor", signal: AbortSignal, report: (progress: DocumentExecutionTargetProgressV1) => void): Promise<Uint8Array> {
  const declared = Number(response.headers.get("content-length") ?? "");
  if (!response.ok || !Number.isSafeInteger(declared) || declared !== expectedByteLength || declared < 1 || declared > maxBytes || !response.body) throw new Error("document execution target: invalid body");
  const bytes = new Uint8Array(declared);
  const reader = response.body.getReader();
  let received = 0;
  let announced = 0;
  try {
    for (;;) {
      if (signal.aborted) throw new Error("document execution target: cancelled");
      const { done, value } = await reader.read();
      if (done) break;
      if (received + value.byteLength > declared) throw new Error("document execution target: invalid body");
      bytes.set(value, received);
      received += value.byteLength;
      value.fill(0);
      if (received - announced >= EXECUTION_TARGET_PROGRESS_UNIT_BYTES) {
        announced = received;
        report({ stage, completedBytes: received, totalBytes: declared });
      }
    }
    if (received !== declared) throw new Error("document execution target: invalid body");
    report({ stage, completedBytes: received, totalBytes: declared });
    return bytes;
  } catch (error) {
    bytes.fill(0);
    await reader.cancel().catch(() => undefined);
    throw error;
  }
}

/** 📜️ Strictly admits the raw descriptor bytes: canonical re-encoding equality first, then exact
 * equality between the decoded package descriptor and the verified lease fields. A sibling JSON
 * manifest or a caller-selected URL is never descriptor authority. */
function parseVerifiedPackageDescriptorV1(bytes: Uint8Array, fields: DocumentExecutionTargetLeaseFieldsV1): void {
  const decoded = decodePackValue(bytes);
  const canonical = encodePackValue(decoded);
  if (canonical.length !== bytes.length || canonical.some((byte, index) => byte !== bytes[index])) throw new Error("document execution target: descriptor is not canonical");
  const record = (value: unknown): Record<string, unknown> => {
    if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("document execution target: descriptor invalid");
    return value as Record<string, unknown>;
  };
  const descriptor = record(decoded);
  const manifest = record(descriptor.manifest);
  const hashes = record(descriptor.hashes);
  const apps = Array.isArray(manifest.apps) ? manifest.apps.map(record) : [];
  const artifactKinds = Array.isArray(manifest.artifactKinds) ? manifest.artifactKinds.map(record) : [];
  const app = apps.find((entry) => entry.id === fields.surface.appId);
  const dialect = app === undefined ? undefined : record(app.dialect);
  const windowKinds = app !== undefined && Array.isArray(app.windowKinds) ? app.windowKinds.map(record) : [];
  if (
    descriptor.descriptorVersion !== 1
    || descriptor.packageId !== fields.package.packageId
    || descriptor.execution !== "isolated"
    || manifest.pluginId !== fields.package.pluginId
    || manifest.version !== fields.package.version
    || hashes.wasmSha256 !== fields.component.sha256
    || app === undefined
    || dialect === undefined
    || app.id !== fields.surface.surfaceId
    || app.role !== fields.surface.role
    || dialect.artifactKind !== fields.parentDialect.artifactKind
    || dialect.standard !== fields.parentDialect.standard
    || dialect.subset !== fields.parentDialect.subset
    || !windowKinds.some((window) => window.id === fields.surface.windowKindId)
    || !artifactKinds.some((kind) => kind.id === fields.artifact.kind && kind.schema === fields.artifact.schema)
  ) throw new Error("document execution target: descriptor mismatch");
}

/** 🛡️ Acquires the server-selected verified execution target for one live plan and mints the private
 * lease only after every byte verifies. It shares {@link ArtifactState.docAbort} and the current
 * document-open deadline with {@link requestDocumentSocketAuthority}; a mismatch, cancellation or
 * deadline wipes every retained buffer and returns no lease. */
async function installDocumentExecutionTargetLease(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub" }>, plan: DocumentOpenPlanV1, intent: DocumentOpenIntentV1): Promise<DocumentExecutionTargetLease> {
  const signal = state.docAbort.signal;
  const options = { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal } as const;
  const report = (progress: DocumentExecutionTargetProgressV1): void => emitExecutionTargetStatus(state, binding, "verifying", progress);
  let component: Uint8Array | null = null;
  let descriptorBytes: Uint8Array | null = null;
  try {
    if (signal.aborted) throw new Error("document execution target: cancelled");
    report({ stage: "manifest", completedBytes: 0, totalBytes: 1 });
    const manifestResponse = await browserExecutionTargetAssetRequest(binding, state.config.documentId, "manifest", intent, options);
    if (!manifestResponse.ok) throw new Error("document execution target: unavailable");
    const fields = parseDocumentExecutionTargetLeaseFieldsV1(await readExecutionTargetManifestJson(manifestResponse));
    if (!sameLeaseFieldsV1(fields, receiptFreeFields(plan, { component: fields.component.byteLength, descriptor: fields.descriptor.byteLength }))) throw new Error("document execution target: manifest mismatch");
    report({ stage: "manifest", completedBytes: 1, totalBytes: 1 });

    if (signal.aborted) throw new Error("document execution target: cancelled");
    const componentResponse = await browserExecutionTargetAssetRequest(binding, state.config.documentId, "component", intent, options);
    component = await readExecutionTargetBody(componentResponse, fields.component.byteLength, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, "component", signal, report);

    if (signal.aborted) throw new Error("document execution target: cancelled");
    const descriptorResponse = await browserExecutionTargetAssetRequest(binding, state.config.documentId, "descriptor", intent, options);
    descriptorBytes = await readExecutionTargetBody(descriptorResponse, fields.descriptor.byteLength, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, "descriptor", signal, report);

    if (signal.aborted) throw new Error("document execution target: cancelled");
    report({ stage: "verify", completedBytes: 0, totalBytes: 3 });
    if (await executionTargetSha256Hex(component) !== fields.component.sha256) throw new Error("document execution target: component integrity");
    report({ stage: "verify", completedBytes: 1, totalBytes: 3 });
    if (blake3Hex(component) !== fields.component.blake3) throw new Error("document execution target: component integrity");
    report({ stage: "verify", completedBytes: 2, totalBytes: 3 });
    if (await executionTargetSha256Hex(descriptorBytes) !== fields.descriptor.sha256) throw new Error("document execution target: descriptor integrity");
    parseVerifiedPackageDescriptorV1(descriptorBytes, fields);
    report({ stage: "verify", completedBytes: 3, totalBytes: 3 });
    if (signal.aborted) throw new Error("document execution target: cancelled");
    const lease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, fields, binding.baseUrl.replace(/\/+$/u, ""), component, descriptorBytes);
    component = null;
    descriptorBytes = null;
    return lease;
  } finally {
    component?.fill(0);
    descriptorBytes?.fill(0);
  }
}

function dropDocumentExecutionTargetLease(state: ArtifactState): void {
  state.executionTargetLease?.drop();
  state.executionTargetLease = null;
}
//#endregion 🪪️ExecutionTargetLease

/** ⚖️ Compares one server plan against the complete locally installed execution-target fields through
 * the single shared {@link sameLeaseFieldsV1} relation. A non-`react` renderer target is admitted
 * only when the caller owns a live private lease whose own verified fields are the comparison input;
 * the verified target is then routed to an explicit renderer state, never to a module loader. */
function documentOpenPlanAuthority(plan: DocumentOpenPlanV1, intent: DocumentOpenIntentV1, config: ArtifactActorConfig, installed: NonNullable<Extract<PersistenceBinding, { kind: "hub" }>["installedTarget"]>, lease?: DocumentExecutionTargetLease): Omit<BrowserDocumentSocketAuthorityV1, "receipt"> {
  const leaseFields = lease !== undefined && lease.live ? lease.fields() : undefined;
  const projected = ((): DocumentExecutionTargetLeaseFieldsV1 | null => {
    try {
      return receiptFreeFields(plan, { component: installed.component.byteLength, descriptor: installed.descriptor.byteLength });
    } catch {
      return null;
    }
  })();
  if (
    projected === null ||
    plan.artifact.schema !== config.schema ||
    plan.scope.spaceId !== intent.scope.spaceId ||
    plan.scope.documentId !== intent.scope.documentId ||
    intent.requestedSurfaceId !== installed.surface.surfaceId ||
    !sameLeaseFieldsV1(projected, installed) ||
    (plan.surface.rendererTarget !== "react" && (leaseFields === undefined || !sameLeaseFieldsV1(leaseFields, installed)))
  ) throw new Error("document open: authority mismatch");
  const packSchemaHash = Array.from({ length: 32 }, (_unused, index) => Number.parseInt(plan.artifact.packSchemaHash.slice(index * 2, index * 2 + 2), 16));
  const configured = config.packSchemaHash;
  if (configured && configured.some((byte) => byte !== 0) && (configured.length !== 32 || configured.some((byte, index) => byte !== packSchemaHash[index]))) throw new Error("document open: authority mismatch");
  return { schema: plan.artifact.schema, packSchemaHash, parentDialect: plan.parentDialect, surfaceId: plan.surface.surfaceId };
}
async function requestDocumentSocketAuthority(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub" }>): Promise<BrowserDocumentSocketAuthorityV1> {
  const grantPath = `/spaces/${encodeURIComponent(binding.spaceId)}/documents/${encodeURIComponent(state.config.documentId)}/socket-grants`;
  if (socketGrantTestIssue) {
    const receipt = await socketGrantTestIssue(binding.baseUrl, grantPath, state.docAbort.signal);
    return { receipt, schema: state.config.schema, packSchemaHash: state.config.packSchemaHash ?? new Array(32).fill(0), ...(binding.installedTarget ? { surfaceId: binding.installedTarget.surface.surfaceId } : binding.requestedSurfaceId ? { surfaceId: binding.requestedSurfaceId } : {}) };
  }
  const requestedSurfaceId = binding.requestedSurfaceId ?? binding.installedTarget?.surface.surfaceId;
  if (requestedSurfaceId === undefined) throw new Error("document open: installed target unavailable");
  const intent = parseDocumentOpenIntentV1({
    schema: "semio.hub.document-open-intent/v1",
    version: 1,
    scope: { spaceId: binding.spaceId, documentId: state.config.documentId },
    requestedSurfaceId,
    clientInstanceId: state.openClientInstanceId,
  });
  const openPath = `/spaces/${encodeURIComponent(binding.spaceId)}/documents/${encodeURIComponent(state.config.documentId)}/open-plan`;
  const openResponse = await browserBrokerFetch(`/_semio/hub${openPath}`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(intent) }, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: state.docAbort.signal });
  if (!openResponse.ok) throw new Error("document open: unavailable");
  let plan: DocumentOpenPlanV1;
  try {
    plan = parseDocumentOpenPlanV1(await readDocumentOpenJson(openResponse), Date.now());
  } catch {
    clearLocalBrowserBrokerProof();
    throw new Error("document open: invalid plan");
  }
  dropDocumentExecutionTargetLease(state);
  let lease: DocumentExecutionTargetLease | undefined;
  if (plan.surface.rendererTarget !== "react" || binding.installedTarget === undefined) {
    try {
      lease = await installDocumentExecutionTargetLease(state, binding, plan, intent);
    } catch (error) {
      const cancelled = state.docAbort.signal.aborted || String((error as Error).message).includes("cancelled");
      emitExecutionTargetStatus(state, binding, cancelled ? "cancelled" : "integrity-failed");
      clearLocalBrowserBrokerProof();
      throw new Error(cancelled ? "document open: cancelled" : "document open: invalid execution target");
    }
  }
  let authority: Omit<BrowserDocumentSocketAuthorityV1, "receipt">;
  try {
    const installed = lease === undefined ? binding.installedTarget : lease.fields();
    if (installed === undefined) throw new Error("document open: installed target unavailable");
    authority = documentOpenPlanAuthority(plan, intent, state.config, installed, lease);
  } catch {
    lease?.drop();
    emitExecutionTargetStatus(state, binding, "stale");
    clearLocalBrowserBrokerProof();
    throw new Error("document open: invalid plan");
  }
  if (state.docAbort.signal.aborted || (lease !== undefined && !lease.live)) {
    lease?.drop();
    emitExecutionTargetStatus(state, binding, "cancelled");
    throw new Error("document open: cancelled");
  }
  const exchange = parseDocumentPlanSocketGrantIntentV1({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt });
  const grantResponse = await browserBrokerFetch(`/_semio/hub${grantPath}`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(exchange) }, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: state.docAbort.signal });
  if (!grantResponse.ok) {
    lease?.drop();
    if (lease !== undefined) emitExecutionTargetStatus(state, binding, "stale");
    throw new Error("document open: unavailable");
  }
  try {
    const receipt = parseSocketGrantReceiptV1(await readDocumentOpenJson(grantResponse));
    if (receipt.expiresAtMs <= Date.now() || receipt.expiresAtMs > plan.expiresAtUnixMs || (lease !== undefined && !lease.live)) throw new Error("document open: invalid grant");
    if (lease !== undefined) {
      state.executionTargetLease = lease;
      emitExecutionTargetStatus(state, binding, "renderer-unavailable");
    }
    return { receipt, ...authority };
  } catch {
    lease?.drop();
    clearLocalBrowserBrokerProof();
    throw new Error("document open: invalid grant");
  }
}

function browserDirectoryRequest(input: string, init: RequestInit = {}, options: { readonly timeoutMs: number; readonly signal?: AbortSignal }): Promise<FetchTimeoutResponse> {
  const url = new URL(input, "http://browser-broker.invalid");
  const method = init.method ?? "GET";
  const after = url.searchParams.get("after") ?? "";
  const eventPage = url.pathname === "/_semio/hub/directory/event-page/v1"
    && [...url.searchParams].length === 1
    && /^(?:0|[1-9]\d*)$/u.test(after)
    && Number.isSafeInteger(Number(after));
  const allowed = (method === "GET" && (((url.pathname === "/_semio/hub/directory/spaces" || /^\/_semio\/hub\/directory\/spaces\/[^/]+$/u.test(url.pathname)) && url.search === "") || (url.pathname === "/_semio/hub/directory/events" && [...url.searchParams].length === 1 && /^\d+$/u.test(url.searchParams.get("since") ?? "")) || eventPage))
    || (method === "POST" && url.pathname === "/_semio/hub/directory/commands" && url.search === "");
  if (!allowed) return Promise.reject(new Error("browser directory operation denied"));
  return browserBrokerFetch(`${url.pathname}${url.search}`, init, options);
}

function setStatus(state: ArtifactState, patch: Partial<ArtifactSyncStatus>): void {
  state.status = { ...state.status, ...patch };
  emitEvent(state, { kind: "status", ...state.status });
}

function setRemote(state: ArtifactState, remote: RemoteState): void {
  setStatus(state, { remote });
}

function folderBinding(config: ArtifactActorConfig): Extract<PersistenceBinding, { kind: "folder" }> | null {
  const binding = config.bindings.find((entry): entry is Extract<PersistenceBinding, { kind: "folder" }> => entry.kind === "folder");
  return binding ?? null;
}

function hubBinding(config: ArtifactActorConfig): Extract<PersistenceBinding, { kind: "hub" }> | null {
  const binding = config.bindings.find((entry): entry is Extract<PersistenceBinding, { kind: "hub" }> => entry.kind === "hub");
  return binding ?? null;
}
//#endregion 🔖️DocumentState

//#region 🔖️ConfigLane
/** 🎚️ Canonical `documentId`/`schema` for the OS-wide `os.config.opening` facet (contract freeze
 * §4 of `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`) — one
 * singleton instance per install, so the schema id doubles as its document id. */
export const OPENING_PREFERENCES_SCHEMA = "os.config.opening";

/** 🎚️ Builds the {@link ArtifactActorConfig} that opens the opening-preferences facet through
 * this SAME generic actor mechanism every other document uses — `bindings: []` is not a stub, it
 * IS the "persisted local-only" lane (contract freeze §4): {@link folderBinding}/{@link hubBinding}
 * both return `null` on an empty `bindings` array, so `openArtifact` below already skips folder
 * watch and hub websocket setup for this config entirely — no schema-specific branch exists or is
 * needed anywhere in this file for that to hold. */
export function openingPreferencesActorConfig(actor: string): ArtifactActorConfig {
  return { documentId: OPENING_PREFERENCES_SCHEMA, schema: OPENING_PREFERENCES_SCHEMA, bindings: [], actor };
}

/** 🧮️ Reduces one {@link ArtifactEvent} onto a materialized `OpeningPreferences` — event-sourced,
 * never a mutable map (contract freeze §4). This facet's `Mutation::diff` is whole-record (kernel
 * `🔖️OpeningResolver`'s `decodeOpeningPreferences` docstring), so a `remoteMutations` envelope's
 * already-diffed `diff.payload` IS the next full snapshot — folding is "last envelope wins", not a
 * replay of individual `set`/`clear` operations. Every other event kind (`status`, `presence`, …)
 * passes `base` through unchanged. `decodePayload` stays injected rather than imported from
 * `@semio-tech/framework` at the call site — the caller already has `decodeOpeningPreferences` in
 * scope wherever it decoded `MutationEnvelope.diff.payload` off the wire in the first place. */
export function foldOpeningPreferencesEvent(base: OpeningPreferences, event: ArtifactEvent, decodePayload: (payload: unknown) => OpeningPreferences | undefined): OpeningPreferences {
  if (event.kind !== "remoteMutations") return base;
  let next = base;
  for (const envelope of event.envelopes) {
    const decoded = decodePayload(envelope.diff.payload);
    if (decoded) next = decoded;
  }
  return next;
}

/** 🪪️ Canonical `documentId`/`schema` for the OS-wide `os.config.identity` facet (contract freeze
 * §C3) — mirrors {@link OPENING_PREFERENCES_SCHEMA}'s singleton-by-schema-id pattern. */
export const IDENTITY_CONFIG_SCHEMA = "os.config.identity";

/** 🪪️ Builds the {@link ArtifactActorConfig} that opens the identity facet. Unlike
 * {@link openingPreferencesActorConfig}'s `bindings: []`, identity binds to the FOLDER lane under
 * `${dataDir}/os` (contract §C3) so a reload keeps the session token — a browser tab with no
 * `S_DATA_DIR` (`dataDir` omitted) falls back to opening's local-only-in-memory pattern instead. */
export function identityActorConfig(actor: string, dataDir?: string): ArtifactActorConfig {
  const bindings: PersistenceBinding[] = dataDir ? [{ kind: "folder", path: `${dataDir}/os` }] : [];
  return { documentId: IDENTITY_CONFIG_SCHEMA, schema: IDENTITY_CONFIG_SCHEMA, bindings, actor };
}

/** 🧮️ Reduces one {@link ArtifactEvent} onto a materialized `Identity | null` — event-sourced,
 * mirrors {@link foldOpeningPreferencesEvent}: `applyIdentityConfigMutation`'s diff is whole-record
 * too (`🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️.ts`), so a `remoteMutations` envelope's
 * already-diffed `diff.payload` IS the next `Identity` (or `null` for a signed-out session) —
 * folding is "last envelope wins". `decodePayload` returning `undefined` means "not this facet's
 * payload", distinct from a legit `null` (signed out), so both must be distinguishable. */
export function foldIdentityEvent(base: Identity | null, event: ArtifactEvent, decodePayload: (payload: unknown) => Identity | null | undefined): Identity | null {
  if (event.kind !== "remoteMutations") return base;
  let next = base;
  for (const envelope of event.envelopes) {
    const decoded = decodePayload(envelope.diff.payload);
    if (decoded !== undefined) next = decoded;
  }
  return next;
}
//#endregion 🔖️ConfigLane

//#region 🔖️WireBridge
/** 🧮️ A stable, deterministic 32-bit seed for an actor id string, for `WireMutationEnvelope.
 * timestamp.actor` — the TS twin of the Rust actor's `actor_seed` (`framework/sync/rs/lib.rs`
 * `🔖️WireBridge`). Not cryptographic, just a cheap deterministic fold — matches the Rust side's own
 * `DefaultHasher`-based approach in spirit (both are wire-local ordering metadata, never round-
 * tripped back into an app-level {@link MutationEnvelope}). */
function actorSeed(actor: string): number {
  let hash = 0;
  for (let index = 0; index < actor.length; index++) {
    hash = (Math.imul(hash, 31) + actor.charCodeAt(index)) | 0;
  }
  return hash >>> 0;
}

/** ⏰️ Advances `state.hlcCounter` and stamps a fresh wire timestamp for an outbound envelope —
 * the TS twin of the Rust actor's `next_timestamp`. */
function nextWireTimestamp(state: ArtifactState): WireMutationEnvelope["timestamp"] {
  state.hlcCounter += 1;
  return { actor: actorSeed(state.actor), physical_ms: Date.now(), logical: state.hlcCounter };
}

/** #⃣ A cheap, non-cryptographic FNV-1a-style digest for {@link toWireEnvelope}'s placeholder
 * `payloadHash` on the way back through {@link fromWireEnvelope}. This TS fallback never verifies
 * `payloadHash` against anything (it's a "deliberately dumb" relay twin — see this file's header
 * doc — the real content-addressed check happens Rust-side via `semio_framework_hash::hash_bytes`
 * once the wasm actor is available), so a real blake3 dependency isn't worth adding here just to
 * fill an otherwise-unused field. */
function placeholderPayloadHash(payload: unknown): string {
  const packed = encodePackValue(payload);
  let hash = 0x811c9dc5;
  for (let index = 0; index < packed.length; index++) {
    hash ^= packed[index]!;
    hash = Math.imul(hash, 0x01000193);
  }
  return (hash >>> 0).toString(16).padStart(8, "0");
}

/** 🎞️ `store::pack_rt` wire bytes for {@link toWireEnvelope}'s diff/inverse payloads — the TS twin
 * of the Rust actor's `encode_wire_value` call in `to_wire_envelope`. */
function encodePackPayload(value: unknown): number[] {
  return Array.from(encodePackValue(value));
}

/** 🎞️ The inverse of {@link encodePackPayload} — the TS twin of `decode_wire_value` in the Rust
 * actor's `from_wire_envelope`. */
function decodePackPayload(bytes: readonly number[]): unknown {
  return decodePackValue(new Uint8Array(bytes));
}

/** 🌉️ Converts this fallback's local, camelCase {@link MutationEnvelope} into the snake_case
 * {@link WireMutationEnvelope} `protocol_wire::ClientFrame::Commands`/`ServerFrame::Commands`
 * carry — the TS twin of the Rust actor's `to_wire_envelope`. */
function toWireEnvelope(envelope: MutationEnvelope, timestamp: WireMutationEnvelope["timestamp"], actor = envelope.actor): WireMutationEnvelope {
  return {
    mutation_id: envelope.id,
    document_id: envelope.document,
    actor,
    dependencies: [...(envelope.deps ?? [])],
    diff: { schema: envelope.diff.schemaId, payload: encodePackPayload(envelope.diff.payload) },
    inverse: { schema: envelope.inverse.inverseDiff.schemaId, payload: encodePackPayload(envelope.inverse.inverseDiff.payload) },
    timestamp,
  };
}

/** 🌉️ The inverse of {@link toWireEnvelope} — the TS twin of the Rust actor's `from_wire_envelope`.
 * `baseVersion` is recovered from the payload's own `sequenceNumber` (this actor's payloads are
 * always edit-shaped JSON), mirroring the Rust side's identical recovery. */
function fromWireEnvelope(envelope: WireMutationEnvelope): MutationEnvelope {
  const payload = decodePackPayload(envelope.diff.payload);
  const sequenceNumber = payload !== null && typeof payload === "object" && "sequenceNumber" in payload ? Number((payload as Record<string, unknown>).sequenceNumber) : 0;
  return {
    id: envelope.mutation_id,
    actor: envelope.actor,
    document: envelope.document_id,
    schemaVersion: envelope.diff.schema,
    deps: [...envelope.dependencies],
    payloadHash: placeholderPayloadHash(payload),
    diff: { schemaId: envelope.diff.schema, payload },
    inverse: {
      targetOperation: envelope.mutation_id,
      inverseDiff: { schemaId: envelope.inverse.schema, payload: decodePackPayload(envelope.inverse.payload) },
      baseVersion: Number.isFinite(sequenceNumber) ? Math.max(0, sequenceNumber) : 0,
      dependencies: [],
      undoPolicy: "exactBaseOnly",
    },
  };
}

/** 🎨️ Stamps `state`'s hub-assigned session color and canonical surface onto an outbound
 * `ArtifactPresencePeer` right before `encodePresencePeer` — the ONE place either field is ever
 * filled; shells never set `peer.color`/`peer.surface` themselves (contract-freeze §C7.4). The TS
 * twin of the Rust actor's `stamp_session`. */
function stampSession(peer: ArtifactPresencePeer, state: ArtifactState): ArtifactPresencePeer {
  return { ...peer, color: state.sessionColor ?? undefined, surface: hubBinding(state.config)?.installedTarget?.surface.surfaceId };
}

/** ↩️ Synthesizes a local "undo" envelope from a speculative envelope's own precomputed `inverse` —
 * the TS twin of the Rust actor's `rollback_envelope` (see that function's doc comment for why
 * replaying the envelope's own inverse, rather than calling into typed operation-inverse machinery,
 * is the right move for this schema-agnostic relay). */
function rollbackEnvelope(envelope: MutationEnvelope): MutationEnvelope {
  const undoId = `${envelope.id}~undo`;
  return {
    id: undoId,
    actor: envelope.actor,
    document: envelope.document,
    schemaVersion: envelope.schemaVersion,
    deps: [envelope.id],
    payloadHash: placeholderPayloadHash(envelope.inverse.inverseDiff.payload),
    diff: { schemaId: envelope.inverse.inverseDiff.schemaId, payload: envelope.inverse.inverseDiff.payload },
    inverse: { targetOperation: undoId, inverseDiff: { schemaId: envelope.diff.schemaId, payload: envelope.diff.payload }, baseVersion: envelope.inverse.baseVersion, dependencies: [], undoPolicy: envelope.inverse.undoPolicy },
  };
}
//#endregion 🔖️WireBridge

//#region 🔖️Folder
/** 🌉️ `fetchWithTimeout` only declares the structural subset every OTHER call site in this file
 * needs (`ok`/`status`/`headers.get`/`json`/`text`) so its own module never requires the ambient
 * `Response` type — the folder/blob reads here are the only callers that also need raw bytes, so
 * that one extra method is added locally instead of widening the shared interface for everyone. */
type BinaryFetchTimeoutResponse = FetchTimeoutResponse & { arrayBuffer(): Promise<ArrayBuffer> };

function folderEnvelopeUrl(binding: Extract<PersistenceBinding, { kind: "folder" }>, documentId: string): string {
  return `${FOLDER_ENDPOINT_PATH}?uri=${encodeURIComponent(`folder://${binding.path}`)}&documentId=${encodeURIComponent(documentId)}`;
}

/** 📥️ One folder read — always routed through {@link ArtifactState.revalidateFolder}'s
 * `latestWins` wrapper by every caller (SSE wake, sanity-poll tick, `externalChanged`), never
 * called directly, so it can never overlap itself (finding 1). Aborts with the document
 * ({@link ArtifactState.docAbort}, finding 3); an abort is a clean shutdown, not a failure, so it
 * is swallowed without logging. */
async function pollFolderOnce(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder" }>): Promise<void> {
  try {
    const response = (await fetchWithTimeout(folderEnvelopeUrl(binding, state.config.documentId), undefined, {
      timeoutMs: FOLDER_FETCH_TIMEOUT_MS,
      signal: state.docAbort.signal,
    })) as BinaryFetchTimeoutResponse;
    if (response.status === 404) return;
    if (!response.ok) throw new Error(`folder backbone read failed (${response.status})`);
    const bundle = new Uint8Array(await response.arrayBuffer());
    const { pack, spr } = decodeDocumentPackBytes(bundle);
    emitEvent(state, { kind: "snapshotReplaced", pack: Array.from(pack), spr: Array.from(spr) });
    setStatus(state, { persisted: true });
  } catch (error) {
    if (state.docAbort.signal.aborted) return; // 🛑 closed mid-flight — not a real failure.
    console.error("[backbone-worker] folder poll failed", state.config.documentId, error);
  }
}

/** 🛟️ Slow, jittered sanity fallback (finding 1): reschedules itself with fresh jitter every tick
 * (a recursive `setTimeout`, not `setInterval`, since the delay must vary tick to tick) and only
 * actually revalidates when {@link ArtifactState.sseHealthy} is `false` — SSE is the primary wake,
 * this is the self-heal for "SSE looks fine but nothing has arrived in a while" or "SSE never
 * managed to open at all". Every revalidation goes through the same `latestWins`-wrapped
 * {@link ArtifactState.revalidateFolder} the SSE wake uses, so a tick can never overlap a
 * still-in-flight read from either source. */
function startSanityPolling(state: ArtifactState): void {
  const scheduleNext = (): void => {
    if (state.closed) return;
    const jitterMs = SANITY_POLL_MIN_MS + Math.random() * (SANITY_POLL_MAX_MS - SANITY_POLL_MIN_MS);
    state.sanityPollTimer = setTimeout(tick, jitterMs);
  };
  const tick = (): void => {
    if (state.closed) return;
    if (!state.sseHealthy) void state.revalidateFolder();
    scheduleNext();
  };
  scheduleNext();
}

/** 🔌️ One SSE connection attempt (finding 2) — resolves either once {@link ArtifactState.docAbort}
 * fires (a clean shutdown) OR once an ordinary close follows at least {@link SUSTAINED_HEALTHY_MS}
 * of unbroken uptime (coordinator follow-up, finding 4b: tells {@link reconnectForever} this cycle
 * counts as healthy, so the NEXT reconnect starts with a fresh, reset backoff); rejects on every
 * OTHER close/error (closed before reaching sustained health) so the caller's
 * {@link retryWithJitteredBackoff} loop keeps backing off within the SAME call, never resetting,
 * for a server that accepts and immediately drops connections in a loop.
 * {@link ArtifactState.sseHealthy} is the ONLY place "is SSE up" is recorded — set `true` on open,
 * `false` on every close, so {@link startSanityPolling}'s fallback always has an accurate read. */
function connectSseOnce(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder" }>): Promise<void> {
  return new Promise<void>((resolve, reject) => {
    if (state.docAbort.signal.aborted) {
      reject(state.docAbort.signal.reason ?? new Error("backbone-worker: document closed"));
      return;
    }
    let source: EventSource;
    try {
      source = new EventSource(`${FOLDER_ENDPOINT_PATH}/watch?uri=${encodeURIComponent(`folder://${binding.path}`)}`);
    } catch (error) {
      reject(error);
      return;
    }
    let sustainedHealthTimer: ReturnType<typeof setTimeout> | null = null;
    let sustainedHealthReached = false;
    const onAbort = (): void => source.close();
    state.docAbort.signal.addEventListener("abort", onAbort, { once: true });
    source.onopen = () => {
      state.sseHealthy = true;
      sustainedHealthTimer = setTimeout(() => {
        sustainedHealthReached = true;
      }, SUSTAINED_HEALTHY_MS);
    };
    source.onmessage = () => {
      void state.revalidateFolder();
    };
    source.onerror = () => {
      state.docAbort.signal.removeEventListener("abort", onAbort);
      if (sustainedHealthTimer != null) clearTimeout(sustainedHealthTimer);
      state.sseHealthy = false;
      source.close();
      if (state.docAbort.signal.aborted || sustainedHealthReached) {
        resolve();
        return;
      }
      reject(new Error("backbone-worker: folder sse dropped"));
    };
  });
}

/** 👁️ External-change watch (findings 1 + 2 + 4b): an immediate bootstrap read, a persistent SSE
 * connection with jittered, reset-after-sustained-health reconnect ({@link connectSseOnce} via
 * {@link reconnectForever}), and the slow sanity-poll fallback ({@link startSanityPolling}) that
 * only does real work while SSE is down. SSE is now the primary wake signal — the old
 * unconditional 1.5s poll is gone. */
function watchFolder(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder" }>): void {
  void state.revalidateFolder(); // 🚀 bootstrap read; doesn't wait on SSE handshake or poll cadence.
  startSanityPolling(state);
  void reconnectForever(state.docAbort.signal, () => connectSseOnce(state, binding), SSE_RECONNECT_MIN_MS, SSE_RECONNECT_MAX_MS);
}

async function writeFolder(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder" }>, pack: readonly number[], spr: readonly number[]): Promise<void> {
  const bundle = encodeDocumentPackBytes(new Uint8Array(pack), new Uint8Array(spr));
  const response = await fetchWithTimeout(
    folderEnvelopeUrl(binding, state.config.documentId),
    { method: "PUT", headers: { "content-type": "application/octet-stream" }, body: new Uint8Array(bundle) },
    { timeoutMs: FOLDER_FETCH_TIMEOUT_MS, signal: state.docAbort.signal },
  );
  if (!response.ok) throw new Error(`folder backbone write failed (${response.status})`);
  setStatus(state, { persisted: true });
}
//#endregion 🔖️Folder

//#region 🔖️Hub
/** 🔌️ One hub WebSocket connection attempt (finding 4) — resolves either once
 * {@link ArtifactState.docAbort} fires (a clean shutdown) OR once an ordinary close follows at
 * least {@link SUSTAINED_HEALTHY_MS} of unbroken uptime (coordinator follow-up, finding 4b: tells
 * {@link reconnectForever} this cycle counts as healthy, so the NEXT reconnect starts with a
 * fresh, reset backoff instead of inheriting this session's earlier accumulated delay); rejects
 * on every OTHER close (closed before reaching sustained health) so the caller's
 * {@link retryWithJitteredBackoff} loop keeps backing off within the SAME call, never resetting,
 * against a server that accepts and immediately drops connections in a loop — resetting on
 * "socket opened" alone would defeat the backoff entirely against exactly that failure mode. Any
 * batch the dying socket never acked is moved back into {@link ArtifactState.outbox} before the
 * retry either way, rather than left stranded in `pendingBatches` forever (finding 5 — a dead
 * socket will never deliver that `Ack`). */
async function connectHubOnce(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub" }>): Promise<void> {
  const authority = await requestDocumentSocketAuthority(state, binding);
  const receipt = authority.receipt;
  if (receipt.expiresAtMs <= Date.now()) throw new Error("backbone-worker: expired socket grant");
  return new Promise<void>((resolve, reject) => {
    if (state.docAbort.signal.aborted) {
      reject(state.docAbort.signal.reason ?? new Error("backbone-worker: document closed"));
      return;
    }
    state.actor = "";
    state.hubActorReady = false;
    state.pendingSocketActorId = receipt.actorId;
    setRemote(state, { kind: "connecting" });
    const wsBase = binding.baseUrl.replace(/^http/, "ws");
    // 📡️ Presence scope (contract §C0) travels out of band as `?surface=` — no `PresencePeer` wire
    // change (its flag byte is full and the file is peer-leased).
    const surfaceQuery = authority.surfaceId ? `?surface=${encodeURIComponent(authority.surfaceId)}` : "";
    const socket = new WebSocket(`${wsBase}/spaces/${encodeURIComponent(binding.spaceId)}/documents/${encodeURIComponent(state.config.documentId)}/socket/v1${surfaceQuery}`, [...socketGrantProtocolsV1(receipt)]);
    const presenceCandidate = authority.surfaceId
      ? { socket, scope: { spaceId: binding.spaceId, documentId: state.config.documentId }, verifiedSurfaceId: authority.surfaceId }
      : null;
    // 🎞️ Binary frames (`protocol_wire`), not JSON text — see this file's header + `WireBridge` region.
    socket.binaryType = "arraybuffer";
    state.socket = socket;
    let sustainedHealthTimer: ReturnType<typeof setTimeout> | null = null;
    let sustainedHealthReached = false;
    const onAbort = (): void => socket.close();
    state.docAbort.signal.addEventListener("abort", onAbort, { once: true });
    socket.onopen = () => {
      if (socket.protocol !== "semio.socket.v1") {
        socket.close(1002, "socket protocol mismatch");
        return;
      }
      state.reconnectDelayMs = HUB_RECONNECT_MIN_MS;
      sustainedHealthTimer = setTimeout(() => {
        sustainedHealthReached = true;
      }, SUSTAINED_HEALTHY_MS);
      sendWireFrame(state, {
        SocketHelloV1: {
          wire_version: 1,
          protocol_version: 1,
          schema: authority.schema,
          // 🧬️ W5.7: real hash when the shell supplied one via `ArtifactActorConfig.packSchemaHash`
          // (from the wasm renderer's `document_pack_schema_hash` export); zeros otherwise, which the
          // hub treats as "schema-agnostic client" and never validates.
          pack_schema_hash: [...authority.packSchemaHash],
          resume_token: state.resumeToken,
          frontier: state.frontier,
        },
      }, "command");
    };
    socket.onmessage = (messageEvent) => {
      state.hubFrameChain = state.hubFrameChain.then(async () => {
        if (state.socket !== socket) return;
        try {
          const bytes = new Uint8Array(messageEvent.data as ArrayBuffer);
          await handleHubFrame(state, decodeServerFrame(bytes).frame, presenceCandidate);
        } catch (error) {
          console.error("[backbone-worker] malformed hub frame", state.config.documentId, error);
          rejectArtifactBootstrap(state, error);
        }
      });
    };
    socket.onclose = () => {
      state.docAbort.signal.removeEventListener("abort", onAbort);
      if (sustainedHealthTimer != null) clearTimeout(sustainedHealthTimer);
      if (state.socket === socket) {
        if (state.presenceAuthority?.socket === socket) emitEvent(state, { kind: "presence", peers: [] });
        state.presenceAuthority = null;
        state.socket = null;
        state.actor = "";
        state.hubActorReady = false;
        state.pendingSocketActorId = null;
        dropDocumentExecutionTargetLease(state);
        abortArtifactBootstrap(state);
        requeuePendingBatches(state);
      }
      if (state.docAbort.signal.aborted) {
        resolve();
        return;
      }
      if (sustainedHealthReached) {
        // ♻️ Resets the DISPLAY estimate to match the real reset: `reconnectForever` is about to
        // start a brand-new `retryWithJitteredBackoff` call for the next cycle.
        state.reconnectDelayMs = HUB_RECONNECT_MIN_MS;
        setRemote(state, { kind: "backoff", retryInMs: 0 });
        resolve();
        return;
      }
      setRemote(state, { kind: "backoff", retryInMs: state.reconnectDelayMs });
      state.reconnectDelayMs = Math.min(state.reconnectDelayMs * 2, HUB_RECONNECT_MAX_MS);
      reject(new Error("backbone-worker: hub socket closed"));
    };
    socket.onerror = () => socket.close();
  });
}

/** 🔁️ Reconnect loop entry point — one call per document lifetime (from {@link openArtifact}),
 * looping via {@link reconnectForever} until {@link ArtifactState.docAbort} fires. Full jitter
 * (finding 4) avoids a thundering herd when several documents' hub connections drop together
 * (e.g. a hub restart); the sustained-health reset (finding 4b, {@link connectHubOnce}) keeps a
 * long-healthy session from inheriting a large accumulated backoff on its next ordinary blip. */
function connectHub(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub" }>): void {
  if (state.closed) return;
  void reconnectForever(state.docAbort.signal, () => connectHubOnce(state, binding), HUB_RECONNECT_MIN_MS, HUB_RECONNECT_MAX_MS);
}

function sendWireFrame(state: ArtifactState, frame: ClientFrame, lane: WireLane): void {
  if (state.socket?.readyState === WebSocket.OPEN) state.socket.send(encodeClientFrame(frame, lane));
}

/** 🧺️ Builds + sends one `Commands` batch, tracking it in `pendingBatches` for {@link handleAck}.
 * Mirrors the Rust actor's `relay_operations_to_hub`. Finding 5: a closed socket no longer no-ops
 * silently — the envelopes move into {@link ArtifactState.outbox} instead, and
 * {@link handleHubFrame}'s authenticated `Session` branch flushes that outbox (calling this
 * function again) only after the grant actor is proven, so nothing is lost or sent pre-authority. */
function relayMutationsToHub(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  if (envelopes.length === 0) return;
  // 🔒️ A verified read-only execution target rejects publication locally, before a worker frame or
  // an outbox entry exists. Server authorization stays an independent fence.
  if (state.executionTargetLease !== null && state.executionTargetLease.live && !state.executionTargetLease.fields().grant.write) {
    rejectReadOnlyExecutionTarget(state, envelopes);
    return;
  }
  if (!state.hubActorReady) {
    queueOutbox(state, envelopes);
    return;
  }
  if (state.socket?.readyState !== WebSocket.OPEN) {
    queueOutbox(state, envelopes);
    return;
  }
  const batchId = state.nextBatchId;
  state.nextBatchId += 1;
  const wireEnvelopes = envelopes.map((envelope) => toWireEnvelope(envelope, nextWireTimestamp(state), state.actor));
  state.pendingBatches.set(batchId, [...envelopes]);
  sendWireFrame(state, { Commands: { batch_id: batchId, envelopes: wireEnvelopes } }, "command");
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

/** 🚫️ Terminal local rejection for a verified viewer-only execution target — the same
 * {@link CommandAckOutcome} vocabulary, emitted before any queue, socket frame or retry exists. */
function rejectReadOnlyExecutionTarget(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  const batchId = nextLocalOverflowBatchId;
  nextLocalOverflowBatchId -= 1;
  emitEvent(state, {
    kind: "commandOutcome",
    batchId,
    outcome: { kind: "rejected", reason: "execution target is read-only", messages: [envelopes.length] },
  });
}

function rejectMutationQueueOverflow(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  const batchId = nextLocalOverflowBatchId;
  nextLocalOverflowBatchId -= 1;
  console.error("[backbone-worker] pending mutation queue full, rejecting batch", state.config.documentId, envelopes.length);
  emitEvent(state, {
    kind: "commandOutcome",
    batchId,
    outcome: { kind: "rejected", reason: "pending mutation queue full", messages: [envelopes.length, PENDING_MUTATIONS_QUEUE_LIMIT] },
  });
}

/** 📮️ Resolves one outbound `Commands` batch's terminal `Applied` stage — mirrors the Rust actor's
 * `handle_ack`. `pendingMutations` (the UI-facing "unconfirmed" count) is trimmed by id, the same
 * way the old per-operation `ack` frame used to. */
function handleAck(state: ArtifactState, batchId: number, stages: readonly WireAckStage[]): void {
  for (const stage of stages) {
    if (typeof stage !== "object" || !("Applied" in stage)) continue;
    const sent = state.pendingBatches.get(batchId);
    state.pendingBatches.delete(batchId);
    if (!sent) continue;
    const sentIds = new Set(sent.map((envelope) => envelope.id));
    state.pendingMutations = state.pendingMutations.filter((envelope) => !sentIds.has(envelope.id));

    const outcome = stage.Applied.outcome;
    let ackOutcome: CommandAckOutcome;
    if (outcome === "Accepted") {
      ackOutcome = { kind: "accepted" };
    } else if ("Transformed" in outcome) {
      const rollbacks = [...sent].reverse().map(rollbackEnvelope);
      if (rollbacks.length > 0) emitEvent(state, { kind: "remoteMutations", envelopes: rollbacks });
      const converted = fromWireEnvelope(outcome.Transformed.envelope);
      emitEvent(state, { kind: "remoteMutations", envelopes: [converted] });
      ackOutcome = { kind: "transformed" };
    } else {
      const rollbacks = [...sent].reverse().map(rollbackEnvelope);
      if (rollbacks.length > 0) emitEvent(state, { kind: "remoteMutations", envelopes: rollbacks });
      ackOutcome = { kind: "rejected", reason: outcome.Rejected.reason, messages: outcome.Rejected.messages };
    }
    setStatus(state, { pendingMutations: state.pendingMutations.length });
    emitEvent(state, { kind: "commandOutcome", batchId, outcome: ackOutcome });
  }
}

function equalByteArrays(left: readonly number[], right: readonly number[]): boolean {
  return left.length === right.length && left.every((byte, index) => byte === right[index]);
}

function equalFrontiers(left: WireFrontierSummary, right: WireFrontierSummary): boolean {
  return left.document_id === right.document_id
    && left.head_edit_ordinal === right.head_edit_ordinal
    && left.head_edit_id === right.head_edit_id
    && left.last_commit_seq === right.last_commit_seq
    && equalByteArrays(left.chain_hash, right.chain_hash);
}

function queueOutbox(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  const queued = new Set(state.outbox.map((envelope) => envelope.id));
  for (const envelope of envelopes) {
    if (!queued.has(envelope.id)) {
      queued.add(envelope.id);
      state.outbox.push(envelope);
    }
  }
}

function requeuePendingBatches(state: ArtifactState): void {
  const batches = [...state.pendingBatches.entries()].sort(([left], [right]) => left - right);
  state.pendingBatches.clear();
  for (const [, envelopes] of batches) queueOutbox(state, envelopes);
}

function emitBootstrapProgress(state: ArtifactState, progress: ArtifactBootstrapProgress): void {
  const previous = state.artifactBootstrapProgress.at(-1);
  if (previous && (progress.receivedBytes < previous.receivedBytes || progress.receivedChunks < previous.receivedChunks)) throw new Error("artifact bootstrap progress regressed");
  state.artifactBootstrapProgress.push(progress);
  const scope = artifactScope(state);
  post({ kind: "artifact-bootstrap-progress", documentId: state.config.documentId, ...(scope === undefined ? {} : { scope }), receivedBytes: progress.receivedBytes, totalBytes: progress.totalBytes, receivedChunks: progress.receivedChunks, totalChunks: progress.totalChunks });
}

function bootstrapControl(state: ArtifactState): ArtifactBootstrapControl {
  return {
    isCancelled: () => state.closed || state.docAbort.signal.aborted,
    nowMs: () => Date.now(),
    onProgress: (progress) => emitBootstrapProgress(state, progress),
  };
}

function abortArtifactBootstrap(state: ArtifactState): void {
  state.artifactBootstrap?.abort();
  state.artifactBootstrap = null;
  state.artifactBootstrapDeadlineMs = null;
  state.pendingResumeToken = null;
  state.requiredTailFrontier = null;
}

function boundedBootstrapDiagnostic(error: unknown): string {
  const message = error instanceof Error ? error.message : String(error);
  if (new TextEncoder().encode(message).byteLength <= ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES) return message;
  let end = message.length;
  while (end > 0 && new TextEncoder().encode(message.slice(0, end)).byteLength > ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES - 3) end -= 1;
  return `${message.slice(0, end)}...`;
}

function artifactBootstrapFailure(state: ArtifactState, error: unknown): Extract<ArtifactBootstrapWorkerEvent, { readonly kind: "artifact-bootstrap-failed" }> {
  const message = boundedBootstrapDiagnostic(error);
  const normalized = message.toLowerCase();
  const cancelled = state.closed || state.docAbort.signal.aborted || normalized.includes("cancel");
  const deadline = normalized.includes("deadline") || normalized.includes("timed out") || normalized.includes("timeout");
  const invalid = normalized.includes("snapshot") || normalized.includes("schema") || normalized.includes("digest") || normalized.includes("descriptor") || normalized.includes("scope") || normalized.includes("chunk") || normalized.includes("frontier") || normalized.includes("without an active transfer") || normalized.includes("before artifact");
  const code = cancelled ? "cancelled" : deadline ? "deadline-exceeded" : invalid ? "invalid-bootstrap" : "transport-failure";
  const scope = artifactScope(state);
  return { kind: "artifact-bootstrap-failed", documentId: state.config.documentId, ...(scope === undefined ? {} : { scope }), code, message, retryable: code !== "invalid-bootstrap" };
}

function rejectArtifactBootstrap(state: ArtifactState, error: unknown): void {
  const failure = artifactBootstrapFailure(state, error);
  abortArtifactBootstrap(state);
  post(failure);
  state.socket?.close();
}

function requireArtifactRebootstrap(state: ArtifactState): void {
  abortArtifactBootstrap(state);
  state.currentPack = null;
  state.currentSpr = null;
  state.frontier = null;
  state.resumeToken = null;
  state.artifactBootstrapProgress = [];
  setRemote(state, { kind: "connecting" });
  const scope = artifactScope(state);
  post({ kind: "artifact-rebootstrap-required", documentId: state.config.documentId, ...(scope === undefined ? {} : { scope }), message: "rebootstrap-required", retryable: true });
  state.socket?.close();
}

function validateArtifactBootstrapIdentity(state: ArtifactState, bootstrap: WireArtifactBootstrap, serverFrontier: WireFrontierSummary): void {
  if (bootstrap.artifact_schema !== state.config.schema) throw new Error("artifact bootstrap schema mismatch");
  if (bootstrap.baseline_frontier.document_id !== state.config.documentId || bootstrap.required_tail_frontier.document_id !== state.config.documentId || serverFrontier.document_id !== state.config.documentId) throw new Error("artifact bootstrap document mismatch");
  const packSchemaHash = state.config.packSchemaHash;
  if (!packSchemaHash || packSchemaHash.length !== 32 || packSchemaHash.every((byte) => byte === 0) || !equalByteArrays(bootstrap.pack_schema_hash, packSchemaHash)) throw new Error("artifact bootstrap pack schema mismatch");
  if (!equalFrontiers(bootstrap.required_tail_frontier, serverFrontier)) throw new Error("artifact bootstrap required tail does not match welcome frontier");
}

function finishCatchupIfReady(state: ArtifactState): void {
  if (!state.requiredTailFrontier || !state.frontier || !equalFrontiers(state.frontier, state.requiredTailFrontier)) return;
  state.requiredTailFrontier = null;
  if (state.pendingResumeToken !== null) state.resumeToken = state.pendingResumeToken;
  state.pendingResumeToken = null;
  setRemote(state, { kind: "live", peerCount: 0 });
  if (state.hubActorReady && state.outbox.length > 0) {
    const outbox = state.outbox.splice(0);
    relayMutationsToHub(state, outbox);
  }
}

async function installArtifactBootstrap(state: ArtifactState, assembler: ArtifactBootstrapAssembler, done: { readonly descriptor_hash: readonly number[]; readonly chunk_count: number } | null): Promise<void> {
  const pair = await assembler.finish(done, bootstrapControl(state));
  const folder = folderBinding(state.config);
  if (folder) await writeFolder(state, folder, Array.from(pair.pack), Array.from(pair.spr));
  state.currentPack = Uint8Array.from(pair.pack);
  state.currentSpr = Uint8Array.from(pair.spr);
  state.artifactBootstrap = null;
  state.artifactBootstrapDeadlineMs = null;
  state.frontier = assembler.bootstrap.baseline_frontier;
  emitEvent(state, { kind: "snapshotReplaced", pack: Array.from(pair.pack), spr: Array.from(pair.spr) });
  if (state.outbox.length > 0) emitEvent(state, { kind: "remoteMutations", envelopes: [...state.outbox] });
  finishCatchupIfReady(state);
}

async function startArtifactBootstrap(state: ArtifactState, bootstrap: WireArtifactBootstrap, resumeToken: string, serverFrontier: WireFrontierSummary): Promise<void> {
  abortArtifactBootstrap(state);
  state.artifactBootstrapProgress = [];
  validateArtifactBootstrapIdentity(state, bootstrap, serverFrontier);
  state.pendingResumeToken = resumeToken;
  state.requiredTailFrontier = bootstrap.required_tail_frontier;
  state.artifactBootstrapDeadlineMs = Date.now() + ARTIFACT_BOOTSTRAP_DEADLINE_MS;
  const assembler = new ArtifactBootstrapAssembler(bootstrap, bootstrap.descriptor_hash, DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS, state.artifactBootstrapDeadlineMs, bootstrapControl(state));
  state.artifactBootstrap = assembler;
  if (bootstrap.inline !== null) {
    await installArtifactBootstrap(state, assembler, null);
  }
}

async function handleHubFrame(
  state: ArtifactState,
  frame: ServerFrame,
  presenceCandidate: Readonly<{ socket: WebSocket; scope: DocumentScope; verifiedSurfaceId: string }> | null = null,
): Promise<void> {
  if (typeof frame === "string") return; // no unit-variant `ServerFrame` exists today; defensive.
  if ("Welcome" in frame) {
    requeuePendingBatches(state);
    const bootstrap = frame.Welcome.bootstrap;
    if (bootstrap === "None") {
      abortArtifactBootstrap(state);
      state.resumeToken = frame.Welcome.resume_token;
      state.frontier = frame.Welcome.server_frontier;
      setRemote(state, { kind: "live", peerCount: 0 });
      if (state.hubActorReady && state.outbox.length > 0) relayMutationsToHub(state, state.outbox.splice(0));
      return;
    }
    if (bootstrap === "Tail") {
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
      requireArtifactRebootstrap(state);
    }
    return;
  }
  if ("ArtifactBootstrapChunk" in frame) {
    const assembler = state.artifactBootstrap;
    if (!assembler) {
      rejectArtifactBootstrap(state, new Error("artifact bootstrap chunk arrived without an active transfer"));
      return;
    }
    try {
      assembler.push(frame.ArtifactBootstrapChunk, bootstrapControl(state));
    } catch (error) {
      rejectArtifactBootstrap(state, error);
    }
    return;
  }
  if ("ArtifactBootstrapDone" in frame) {
    const assembler = state.artifactBootstrap;
    if (!assembler) {
      rejectArtifactBootstrap(state, new Error("artifact bootstrap completion arrived without an active transfer"));
      return;
    }
    try {
      await installArtifactBootstrap(state, assembler, frame.ArtifactBootstrapDone);
    } catch (error) {
      rejectArtifactBootstrap(state, error);
    }
    return;
  }
  if ("Commands" in frame) {
    if (state.artifactBootstrap) {
      rejectArtifactBootstrap(state, new Error("tail arrived before artifact bootstrap completion"));
      return;
    }
    if (frame.Commands.origin !== state.actor) {
      const envelopes = frame.Commands.envelopes.map(fromWireEnvelope);
      emitEvent(state, { kind: "remoteMutations", envelopes });
    }
    state.frontier = frame.Commands.frontier;
    finishCatchupIfReady(state);
    return;
  }
  if ("Ack" in frame) {
    if (state.artifactBootstrap || state.requiredTailFrontier) {
      rejectArtifactBootstrap(state, new Error("ack arrived before artifact catch-up completion"));
      return;
    }
    state.frontier = frame.Ack.frontier;
    handleAck(state, frame.Ack.batch_id, frame.Ack.stages);
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
      post({ kind: "socket-actor-failed", documentId: state.config.documentId, ...(scope === undefined ? {} : { scope }), code: "session-mismatch" });
      state.socket?.close(1008, "socket actor mismatch");
      return;
    }
    state.actor = expectedActor;
    state.hubActorReady = true;
    state.pendingSocketActorId = null;
    state.sessionColor = frame.Session.color;
    state.presenceAuthority = presenceCandidate?.socket === state.socket ? presenceCandidate : null;
    if (state.outbox.length > 0) relayMutationsToHub(state, state.outbox.splice(0));
    const scope = artifactScope(state);
    post({ kind: "socket-actor", documentId: state.config.documentId, ...(scope === undefined ? {} : { scope }), actorId: expectedActor });
    emitEvent(state, { kind: "session", actor: frame.Session.actor, color: frame.Session.color });
    return;
  }
  if ("Error" in frame) {
    emitEvent(state, { kind: "conflict", message: frame.Error.message });
  }
}
//#endregion 🔖️Hub

//#region 🔖️Directory
/** 📇️ Directory hub lane (contract §C6) — the shell's only path to the directory control plane;
 * plugin surfaces never talk to the network, and the shell never opens a directory socket on the UI
 * thread. Owns exactly one {@link DirectoryClient}/{@link DirectoryStream} at a time. Reuses that
 * client's own reconnect/backoff (`🔖️HubBinding` in `🟦️.ts`) rather than a second loop
 * here — this region's only extra responsibility is the offline command queue. */
/** 📏️ Fixed transport capacity. A 65th live operation is REJECTED with a terminal `capacity`
 * result; the oldest intent is never silently dropped, because an administration mutation the
 * caller believes it issued must never vanish without a receipt. */
const DIRECTORY_COMMAND_TRANSPORT_CAPACITY = 64;

/** 🆔️ One retained command operation. `request` is sealed once and re-sent byte-identically by
 * every transient retry — a retry that changed a single byte would be a different command to the
 * hub's digest-keyed idempotency store. `sessionEpoch`/`workerEpoch` suppress delivery after an
 * identity or worker replacement; such a request may later be explicitly resolved, never auto-replayed. */
type DirectoryCommandTransportOperationV1 = {
  readonly request: DirectoryCommandRequestV1;
  readonly abort: AbortController;
  readonly sessionEpoch: number;
  readonly workerEpoch: number;
  settled: boolean;
};

let directoryClient: DirectoryClient | null = null;
let directoryStream: { close: () => void } | null = null;
const scopedDirectoryStreams = new Map<string, { close: () => void }>();
let directoryFlushing = false;
let directorySessionEpoch = 0;
let directoryWorkerEpoch = 0;
const directoryCommandOperations = new Map<string, DirectoryCommandTransportOperationV1>();
const directoryCommandQueue: DirectoryCommandTransportOperationV1[] = [];

type DirectoryBootstrapTransition = { readonly kind: "fetch"; readonly after: number } | { readonly kind: "live"; readonly since: number };

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
    if (this.phase !== "awaiting-ack" || page === null
      || ack.bootstrapEpoch !== this.bootstrapEpoch
      || ack.receiptSha256 !== page.receiptSha256
      || ack.sessionBindingSha256 !== page.sessionBindingSha256
      || ack.authorizationGeneration !== page.authorizationGeneration
      || ack.throughSeqInclusive !== page.throughSeqInclusive) throw new Error("directory bootstrap: acknowledgement mismatch");
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

type DirectoryBootstrapOwner = {
  readonly machine: DirectoryEventPageBootstrapV1;
  readonly client: DirectoryClient;
  readonly abort: AbortController;
  stream: DirectoryAcknowledgedStream | null;
  retry: ReturnType<typeof setTimeout> | null;
  fetching: boolean;
};

let directoryBootstrap: DirectoryBootstrapOwner | null = null;

function directoryStatus(): BackboneWorkerResponse {
  return { kind: "directory-status", pendingCommands: directoryCommandOperations.size };
}

function openDirectory(baseUrl: string, since: number): void {
  closeDirectory();
  directorySessionEpoch += 1;
  const issuer = createSocketGrantIssuerV1({ post: (path, options) => requestSocketGrant(baseUrl, path, options?.signal) });
  const client = new DirectoryClient(baseUrl, {
    requestBaseUrl: "/_semio/hub",
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

function scheduleDirectoryBootstrapRetry(owner: DirectoryBootstrapOwner): void {
  if (directoryBootstrap !== owner || owner.abort.signal.aborted || owner.retry !== null) return;
  const delay = HUB_RECONNECT_MIN_MS + Math.floor(Math.random() * (HUB_RECONNECT_MIN_MS + 1));
  owner.retry = setTimeout(() => {
    owner.retry = null;
    void fetchDirectoryBootstrapPage(owner);
  }, delay);
}

async function fetchDirectoryBootstrapPage(owner: DirectoryBootstrapOwner): Promise<void> {
  if (directoryBootstrap !== owner || owner.abort.signal.aborted || owner.fetching) return;
  owner.fetching = true;
  try {
    const page = await owner.client.eventPage(owner.machine.after(), { signal: owner.abort.signal });
    if (directoryBootstrap !== owner || owner.abort.signal.aborted) return;
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

function openDirectoryBootstrapLive(owner: DirectoryBootstrapOwner, since: number): void {
  const stream = owner.client.streamAcknowledged(since, (message) => {
    if (directoryBootstrap !== owner || owner.abort.signal.aborted) return;
    void flushDirectoryQueue();
    const rebootstrap = message.kind === "rebootstrap-required";
    const wakesProjection = rebootstrap || message.kind === "event" || message.kind === "heartbeat";
    if (!wakesProjection) {
      post({ kind: "directory-message", message });
      return;
    }
    const after = owner.machine.wake(rebootstrap);
    if (after === null) return;
    owner.stream?.close();
    owner.stream = null;
    if (directoryStream === stream) directoryStream = null;
    void fetchDirectoryBootstrapPage(owner);
  });
  owner.stream = stream;
  directoryStream = stream;
}

function openDirectoryBootstrap(baseUrl: string, after: number, bootstrapEpoch: number): void {
  closeDirectory();
  const abort = new AbortController();
  const client = new DirectoryClient(baseUrl, {
    requestBaseUrl: "/_semio/hub",
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

function acknowledgeDirectoryBootstrap(ack: DirectoryEventPageAckV1): void {
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

function rejectDirectoryBootstrap(bootstrapEpoch: number, receiptSha256: string): void {
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
  return documentRuntimeKeyV1({ kind: "hub", spaceId: scope.spaceId, documentId: scope.documentId });
}

function openScopedDirectory(baseUrl: string, scope: DocumentScope, since: number): void {
  const key = scopedDirectoryKey(scope);
  scopedDirectoryStreams.get(key)?.close();
  const client = new DirectoryClient(baseUrl, {
    requestBaseUrl: "/_semio/hub",
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
      post({ kind: "directory-scope-revoked", scope });
    },
  );
  scopedDirectoryStreams.set(key, stream);
}

function closeScopedDirectory(scope: DocumentScope): void {
  const key = scopedDirectoryKey(scope);
  scopedDirectoryStreams.get(key)?.close();
  scopedDirectoryStreams.delete(key);
}

function closeDirectory(): void {
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
  directoryWorkerEpoch += 1;
  if (directoryAdministration !== null) terminateDirectoryAdministration(directoryAdministration, "stale", "closed");
  const closing = [...directoryCommandOperations.keys()];
  directoryCommandQueue.length = 0;
  directoryCommandOperations.clear();
  for (const requestId of closing) post({ kind: "directory-command-failed", requestId, code: "closed" });
}

/** 🏁️ Retires one operation exactly once and answers its owner with a closed terminal code. */
function settleDirectoryCommand(operation: DirectoryCommandTransportOperationV1, response: BackboneWorkerResponse | null): void {
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
function directoryRejectionStatus(error: unknown): number | undefined {
  return typeof error === "object" && error !== null && "status" in error && typeof (error as { status: unknown }).status === "number" ? (error as { status: number }).status : undefined;
}

/** 🚨️ Classifies one rejection into the closed transport vocabulary without touching server text. */
function directoryCommandErrorCode(error: unknown, aborted: boolean): DirectoryCommandErrorCodeV1 {
  if (aborted) return "cancelled";
  if (error instanceof DirectoryCommandError) return error.code;
  if (error instanceof DirectoryHttpError) return "invalid";
  return "transport";
}

/** 🆔️ Seals one request, admits it against the fixed capacity, and starts its transport turn. A
 * sealing failure, a duplicate live id, and a full transport are all terminal — never queued. */
async function submitDirectoryCommand(requestId: string, command: DirectoryCommand): Promise<void> {
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
function cancelDirectoryCommand(requestId: string): void {
  const operation = directoryCommandOperations.get(requestId);
  if (!operation) return;
  operation.abort.abort(new DirectoryCommandError("cancelled"));
  settleDirectoryCommand(operation, { kind: "directory-command-failed", requestId, code: "cancelled" });
  post(directoryStatus());
}

/** ♻️ Drives the FIFO on every live signal from the stream. Only a transient fault retains the head
 * and its byte-identical sealed request; every terminal code answers its owner and lets the queue
 * proceed. An operation whose session or worker epoch has been replaced is suppressed, not replayed. */
async function flushDirectoryQueue(): Promise<void> {
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
//#endregion 🔖️Directory

//#region 🔖️SpaceAdministration
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

type DirectoryAdministrationOperationV1 = {
  readonly operationEpoch: number;
  readonly spaceId: string;
  readonly sessionEpoch: number;
  readonly workerEpoch: number;
  abort: AbortController;
  phase: DirectoryAdministrationPhaseV1;
  canonicalJson: string | null;
  receiptSha256: string | null;
  outcome: DirectoryCommandOutcomeV1 | null;
  inviteToken: string | null;
  inviteCapabilityStatus: "available" | "copying" | "failed" | null;
  inviteTransferEpoch: number | null;
  nextInviteTransferEpoch: number;
  requestId: string | null;
  closed: boolean;
};

let directoryAdministration: DirectoryAdministrationOperationV1 | null = null;

/** 📣️ Publishes the operation's complete renderer-visible state; never its transport internals. */
function postDirectoryAdministrationState(operation: DirectoryAdministrationOperationV1, code?: DirectoryCommandErrorCodeV1): void {
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
function terminateDirectoryAdministration(operation: DirectoryAdministrationOperationV1, phase: "cancelled" | "denied" | "stale" | "failed", code?: DirectoryCommandErrorCodeV1, notify = true): void {
  if (operation.closed) return;
  operation.closed = true;
  operation.abort.abort(new Error("directory administration closed"));
  operation.canonicalJson = null;
  operation.receiptSha256 = null;
  operation.outcome = null;
  operation.inviteToken = null;
  operation.inviteCapabilityStatus = null;
  operation.inviteTransferEpoch = null;
  operation.requestId = null;
  operation.phase = phase;
  if (directoryAdministration === operation) directoryAdministration = null;
  if (notify) postDirectoryAdministrationState(operation, code);
}

/** 🔎️ Returns the live operation for `operationEpoch`, or `null` when it has been replaced. */
function liveDirectoryAdministration(operationEpoch: number): DirectoryAdministrationOperationV1 | null {
  const operation = directoryAdministration;
  if (operation === null || operation.closed || operation.operationEpoch !== operationEpoch) return null;
  if (operation.sessionEpoch !== directorySessionEpoch || operation.workerEpoch !== directoryWorkerEpoch) {
    terminateDirectoryAdministration(operation, "stale");
    return null;
  }
  return operation;
}

/** 🚦️ Maps one page rejection onto the closed terminal vocabulary. A 401/403 clears the pane. */
function directoryAdministrationPageTermination(error: unknown, aborted: boolean): { phase: "cancelled" | "denied" | "stale" | "failed"; code: DirectoryCommandErrorCodeV1 } {
  if (aborted) return { phase: "cancelled", code: "cancelled" };
  const status = directoryRejectionStatus(error);
  if (status === 401) return { phase: "denied", code: "unauthorized" };
  if (status === 403 || status === 404) return { phase: "denied", code: "forbidden" };
  if (status === 409 || status === 410) return { phase: "stale", code: "stale-session" };
  return { phase: "failed", code: status === undefined ? "transport" : "invalid" };
}

/** 📄️ Fetches exactly one canonical page into the operation and reports `ready`. */
async function loadDirectoryAdministrationPage(operationEpoch: number, cursor: string | undefined, loading: "loading" | "refreshing"): Promise<void> {
  const operation = liveDirectoryAdministration(operationEpoch);
  if (operation === null) return;
  const client = directoryClient;
  if (client === null) {
    terminateDirectoryAdministration(operation, "failed", "transport");
    return;
  }
  operation.phase = loading;
  postDirectoryAdministrationState(operation);
  try {
    const page = await client.spaceAdministrationPage(operation.spaceId, cursor, { signal: operation.abort.signal });
    const live = liveDirectoryAdministration(operationEpoch);
    if (live === null || live !== operation) return;
    operation.canonicalJson = page.canonicalJson;
    operation.phase = "ready";
    postDirectoryAdministrationState(operation);
  } catch (error) {
    if (operation.closed) return;
    const termination = directoryAdministrationPageTermination(error, operation.abort.signal.aborted);
    terminateDirectoryAdministration(operation, termination.phase, termination.code);
  }
}

/** 🆕️ Installs the one retained operation for exactly one space, replacing any predecessor. */
function openDirectoryAdministration(operationEpoch: number, spaceId: string): void {
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
    receiptSha256: null,
    outcome: null,
    inviteToken: null,
    inviteCapabilityStatus: null,
    inviteTransferEpoch: null,
    nextInviteTransferEpoch: 0,
    requestId: null,
    closed: false,
  };
  directoryAdministration = operation;
  void loadDirectoryAdministrationPage(operationEpoch, undefined, "loading");
}

/** 📮️ Submits exactly one administration command and advances only on an exact server receipt.
 * The command is never retried: an indeterminate transport is terminal `failed`, so a `create-invite`
 * can never mint a second invite behind the operator's back. */
async function submitDirectoryAdministrationCommand(operationEpoch: number, requestId: string, command: DirectoryCommand): Promise<void> {
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
  let request: DirectoryCommandRequestV1;
  try {
    request = sealDirectoryCommandRequestV1(requestId, command);
    directoryCommandRequestJson(request);
  } catch {
    postDirectoryAdministrationState(operation, "invalid");
    return;
  }
  operation.requestId = requestId;
  operation.phase = "submitting";
  postDirectoryAdministrationState(operation);
  let receipt: DirectoryCommandReceiptV1;
  try {
    receipt = await client.command(request, { signal: operation.abort.signal });
  } catch (error) {
    if (operation.closed) return;
    const code = directoryCommandErrorCode(error, operation.abort.signal.aborted);
    if (code === "unauthorized" || code === "forbidden") {
      terminateDirectoryAdministration(operation, "denied", code);
      return;
    }
    if (code === "stale-session") {
      terminateDirectoryAdministration(operation, "stale", code);
      return;
    }
    terminateDirectoryAdministration(operation, code === "cancelled" ? "cancelled" : "failed", code);
    return;
  }
  const live = liveDirectoryAdministration(operationEpoch);
  if (live === null || live !== operation) return;
  operation.requestId = null;
  operation.receiptSha256 = receipt.receiptSha256;
  operation.outcome = receipt.outcome;
  operation.inviteToken = receipt.result.kind === "invite" ? receipt.result.inviteToken : null;
  operation.inviteCapabilityStatus = operation.inviteToken === null ? null : "available";
  operation.inviteTransferEpoch = null;
  operation.phase = "receipt";
  postDirectoryAdministrationState(operation);
  await loadDirectoryAdministrationPage(operationEpoch, undefined, "refreshing");
}

/** 🎁️ Offers the worker-retained capability for one operation-bound clipboard attempt. */
function requestDirectoryAdministrationCapability(operationEpoch: number): void {
  const operation = liveDirectoryAdministration(operationEpoch);
  if (operation === null) return;
  const inviteToken = operation.inviteToken;
  if (inviteToken === null) {
    post({ kind: "directory-administration-capability-rejected", operationEpoch, code: "already-settled" });
    return;
  }
  if (operation.inviteTransferEpoch !== null) {
    post({ kind: "directory-administration-capability-rejected", operationEpoch, transferEpoch: operation.inviteTransferEpoch, code: "capacity" });
    return;
  }
  const transferEpoch = operation.nextInviteTransferEpoch + 1;
  operation.nextInviteTransferEpoch = transferEpoch;
  operation.inviteTransferEpoch = transferEpoch;
  operation.inviteCapabilityStatus = "copying";
  postDirectoryAdministrationState(operation);
  post({ kind: "directory-administration-capability", operationEpoch, transferEpoch, inviteToken });
}

/** 📋️ Erases the capability only after an exact successful clipboard result; failure keeps it retryable. */
function settleDirectoryAdministrationCapability(operationEpoch: number, transferEpoch: number, copied: boolean): void {
  const operation = liveDirectoryAdministration(operationEpoch);
  if (operation === null) return;
  if (operation.inviteTransferEpoch === null) {
    post({ kind: "directory-administration-capability-rejected", operationEpoch, transferEpoch, code: "already-settled" });
    return;
  }
  if (!Number.isSafeInteger(transferEpoch) || transferEpoch <= 0 || transferEpoch !== operation.inviteTransferEpoch) {
    post({ kind: "directory-administration-capability-rejected", operationEpoch, transferEpoch, code: "mismatch" });
    return;
  }
  operation.inviteTransferEpoch = null;
  if (copied) {
    operation.inviteToken = null;
    operation.inviteCapabilityStatus = null;
  } else {
    operation.inviteCapabilityStatus = "failed";
  }
  postDirectoryAdministrationState(operation);
}

/** 🧯️ Retires the retained operation when its exact space loses membership through a scoped 4401.
 * Any other space's revocation leaves the operation untouched — one revoked document scope is not a
 * reason to tear down administration of an unrelated space. */
function revokeDirectoryAdministrationForScope(spaceId: string): void {
  const operation = directoryAdministration;
  if (operation === null || operation.closed || operation.spaceId !== spaceId) return;
  terminateDirectoryAdministration(operation, "denied", "forbidden");
}

/** 🛑️ Renderer unmount: cancel, erase, and report `cancelled` without touching any other lane. */
function closeDirectoryAdministration(operationEpoch: number): void {
  const operation = directoryAdministration;
  if (operation === null || operation.operationEpoch !== operationEpoch) return;
  terminateDirectoryAdministration(operation, "cancelled", "cancelled");
}
//#endregion 🔖️SpaceAdministration

//#region 💡️Inference
/** 💡️ The single shell-owned retained inference operation. It is NOT a socket, NOT a queue and NOT
 * a document command: it owns exactly one document scope, at most one submitted job, and one bounded
 * poll timer. It refuses to exist at all unless that document currently owns a LIVE verified
 * execution-target lease, and it shares the document's own `docAbort` so a close, rebootstrap or
 * identity change cancels every in-flight call before the renderer is told anything. Nothing it
 * holds is ever persisted into the document: the proposal reaches the Map only through the hub's
 * own server-stamped approval command. */
const INFERENCE_PORT_CAPACITY = 1;
/** ⏱️ Bounded, jitter-free poll cadence for one running job — a `setTimeout` chain, never an
 * interval and never a busy loop. */
const INFERENCE_POLL_INTERVAL_MS = 750;
/** 🔁️ Highest number of poll turns one job may take before it is reported indeterminate. */
const INFERENCE_MAX_POLL_TURNS = 240;
/** ⏳️ Lifetime one submitted job asks the hub for. */
const INFERENCE_JOB_LIFETIME_MS = 60_000;

type InferenceOperationV1 = {
  readonly operationEpoch: number;
  readonly scope: DocumentScope;
  readonly abort: AbortController;
  status: GisMapInferencePortStatusV1;
  turns: number;
  pollTimer: ReturnType<typeof setTimeout> | null;
  inFlight: boolean;
  /** 🛑️ Whether the one cancel request this operation may ever send has actually left. A Cancel
   * clicked while another call is in flight is recorded here and sent on the very next turn, never
   * dropped and never sent twice. */
  cancelSent: boolean;
  closed: boolean;
};

let inferencePort: InferenceOperationV1 | null = null;

/** 🔭️ Observation seam mirroring {@link executionTargetStatusObserver}: a harness without a worker
 * scope still sees the exact bounded payload the renderer would receive. */
let inferencePortStatusObserver: ((status: Extract<BackboneWorkerResponse, { kind: "inference-port-status" }>) => void) | null = null;

function postInferencePortStatus(operation: InferenceOperationV1): void {
  const status: Extract<BackboneWorkerResponse, { kind: "inference-port-status" }> = { kind: "inference-port-status", operationEpoch: operation.operationEpoch, scope: operation.scope, status: operation.status };
  inferencePortStatusObserver?.(status);
  post(status);
}

/** 🧮️ Applies one closed event through the shared reducer and publishes only on a real change. */
function advanceInferencePort(operation: InferenceOperationV1, event: GisMapInferencePortEventV1): void {
  const next = reduceGisMapInferencePortV1(operation.status, event);
  if (next === operation.status) return;
  operation.status = next;
  postInferencePortStatus(operation);
}

/** 🪪️ The precondition: only a document whose worker-private execution-target lease is minted AND
 * still live may open a port. A dropped, absent or non-writable lease is refused before any request
 * exists, and the refusal is a localized terminal, never a silent no-op. */
function inferenceLeaseVerified(scope: DocumentScope): boolean {
  const state = artifactState(scope.documentId, scope.spaceId);
  if (state === undefined || state.closed || state.docAbort.signal.aborted) return false;
  const lease = state.executionTargetLease;
  if (lease === null || !lease.live) return false;
  const fields = lease.fields();
  return fields.scope.spaceId === scope.spaceId && fields.scope.documentId === scope.documentId && fields.grant.write;
}

function inferenceJobPath(scope: DocumentScope, suffix: string): string {
  return `/spaces/${encodeURIComponent(scope.spaceId)}/documents/${encodeURIComponent(scope.documentId)}/inference/gis-map${suffix}`;
}

/** 🚪️ The only broker operation this lane owns: exactly the four protected document-scoped inference
 * calls for the port's own scope. Anything else is denied before a request exists. */
async function inferenceBrokerFetch(operation: InferenceOperationV1, suffix: string, init: { readonly method: "GET" | "POST"; readonly body?: string }): Promise<FetchTimeoutResponse> {
  const path = inferenceJobPath(operation.scope, suffix);
  if (!/^\/spaces\/[^/?#]+\/documents\/[^/?#]+\/inference\/gis-map\/jobs(?:\/[0-9a-f]{32}\/(?:events\?after=\d{1,3}|cancel|approval))?$/u.test(path)) throw new Error("gis map inference: operation denied");
  const documentAbort = artifactState(operation.scope.documentId, operation.scope.spaceId)?.docAbort.signal;
  if (documentAbort?.aborted ?? true) throw new Error("gis map inference: document closed");
  return browserBrokerFetch(`/_semio/hub${path}`, {
    method: init.method,
    ...(init.body === undefined ? {} : { headers: { "content-type": "application/json" }, body: init.body }),
  }, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: operation.abort.signal });
}

/** 📥️ Reads one bounded owner-private JSON body under the shared response maximum. */
async function readInferenceJson(response: FetchTimeoutResponse): Promise<unknown> {
  const declared = response.headers.get("content-length");
  if (declared !== null && !(Number.isSafeInteger(Number(declared)) && Number(declared) >= 1 && Number(declared) <= GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES)) throw new Error("gis map inference: invalid body");
  const text = await response.text();
  if (text.length === 0 || new TextEncoder().encode(text).length > GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES) throw new Error("gis map inference: invalid body");
  return JSON.parse(text);
}

/** 🔎️ Returns the live operation for `operationEpoch`, or `null` once it has been replaced, closed,
 * or had its document's lease invalidated under it. */
function liveInferencePort(operationEpoch: number): InferenceOperationV1 | null {
  const operation = inferencePort;
  if (operation === null || operation.closed || operation.operationEpoch !== operationEpoch) return null;
  if (!inferenceLeaseVerified(operation.scope)) {
    terminateInferencePort(operation, "inference.lease-unverified");
    return null;
  }
  return operation;
}

/** 🧯️ Reports one closed terminal exactly once and retires the operation with its timer cleared. */
function terminateInferencePort(operation: InferenceOperationV1, code: GisMapInferencePortCodeV1): void {
  if (operation.closed) return;
  operation.closed = true;
  if (operation.pollTimer !== null) clearTimeout(operation.pollTimer);
  operation.pollTimer = null;
  operation.abort.abort(new Error("gis map inference port closed"));
  if (inferencePort === operation) inferencePort = null;
  advanceInferencePort(operation, { kind: "failed", code });
}

function inferenceCodeFromRejection(error: unknown, aborted: boolean): GisMapInferencePortCodeV1 {
  if (aborted) return "inference.cancelled";
  const status = error instanceof DirectoryHttpError ? error.status : undefined;
  return status === undefined ? "inference.transport" : gisMapInferenceCodeFromStatusV1(status);
}

/** 🆕️ Installs the one retained port for exactly one document scope, retiring any predecessor. The
 * lease precondition is checked BEFORE the operation exists, so a refusal never leaves a port open. */
function openInferencePort(operationEpoch: number, scope: DocumentScope): void {
  if (inferencePort !== null && INFERENCE_PORT_CAPACITY === 1) closeInferencePort(inferencePort.operationEpoch);
  if (!Number.isSafeInteger(operationEpoch) || operationEpoch < 0 || scope.spaceId.length === 0 || scope.documentId.length === 0) {
    post({ kind: "inference-port-status", operationEpoch, scope, status: { ...idleGisMapInferencePortStatusV1(), phase: "failed", code: "inference.invalid" } });
    return;
  }
  const operation: InferenceOperationV1 = { operationEpoch, scope, abort: new AbortController(), status: idleGisMapInferencePortStatusV1(), turns: 0, pollTimer: null, inFlight: false, cancelSent: false, closed: false };
  if (!inferenceLeaseVerified(scope)) {
    operation.closed = true;
    advanceInferencePort(operation, { kind: "lease-unverified" });
    return;
  }
  inferencePort = operation;
  postInferencePortStatus(operation);
}

/** 📮️ Submits exactly one job and advances only on an exact server receipt. A submit is never
 * retried: an indeterminate transport is terminal, so a replay can never mint a second job. */
async function submitInferenceJob(operationEpoch: number, requestId: string): Promise<void> {
  const operation = liveInferencePort(operationEpoch);
  if (operation === null || operation.status.phase !== "idle" || operation.inFlight) return;
  let body: string;
  try {
    body = JSON.stringify(sealGisMapInferenceJobRequestV1(requestId, INFERENCE_JOB_LIFETIME_MS));
  } catch {
    terminateInferencePort(operation, "inference.invalid");
    return;
  }
  advanceInferencePort(operation, { kind: "start" });
  operation.inFlight = true;
  try {
    const response = await inferenceBrokerFetch(operation, "/jobs", { method: "POST", body });
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const receipt = parseGisMapInferenceJobReceiptV1(await readInferenceJson(response));
    if (liveInferencePort(operationEpoch) !== operation) return;
    advanceInferencePort(operation, { kind: "receipt", receipt });
    scheduleInferencePoll(operation);
  } catch (error) {
    if (operation.closed) return;
    terminateInferencePort(operation, inferenceCodeFromRejection(error, operation.abort.signal.aborted));
  } finally {
    operation.inFlight = false;
  }
}

/** ⏱️ Arms exactly one bounded next poll turn; a terminal phase or an exhausted turn budget arms none. */
function scheduleInferencePoll(operation: InferenceOperationV1): void {
  if (operation.pollTimer !== null) clearTimeout(operation.pollTimer);
  operation.pollTimer = null;
  if (operation.closed || gisMapInferencePortTerminalV1(operation.status.phase)) return;
  if (operation.turns >= INFERENCE_MAX_POLL_TURNS) {
    terminateInferencePort(operation, "inference.transport");
    return;
  }
  operation.pollTimer = setTimeout(() => {
    operation.pollTimer = null;
    void driveInferencePort(operation.operationEpoch);
  }, INFERENCE_POLL_INTERVAL_MS);
}

/** 🔄️ One bounded turn: a recorded-but-unsent cancellation always outranks the next progress read,
 * so a Cancel clicked while another call was in flight is transmitted on the very next turn. */
async function driveInferencePort(operationEpoch: number): Promise<void> {
  const operation = liveInferencePort(operationEpoch);
  if (operation === null) return;
  if (operation.status.cancelRequested && !operation.cancelSent) {
    await cancelInferenceJob(operationEpoch);
    return;
  }
  await pollInferenceJob(operationEpoch);
}

/** 📃️ Reads exactly one bounded owner-private page and folds it through the shared reducer. */
async function pollInferenceJob(operationEpoch: number): Promise<void> {
  const operation = liveInferencePort(operationEpoch);
  if (operation === null || operation.inFlight || operation.status.jobId === null) return;
  operation.turns += 1;
  operation.inFlight = true;
  try {
    const response = await inferenceBrokerFetch(operation, `/jobs/${operation.status.jobId}/events?after=${operation.status.cursor}`, { method: "GET" });
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const page = parseGisMapInferenceEventPageV1(await readInferenceJson(response));
    if (liveInferencePort(operationEpoch) !== operation) return;
    advanceInferencePort(operation, { kind: "page", page });
    if (gisMapInferencePortTerminalV1(operation.status.phase)) {
      retireInferencePort(operation);
      return;
    }
    scheduleInferencePoll(operation);
  } catch (error) {
    if (operation.closed) return;
    terminateInferencePort(operation, inferenceCodeFromRejection(error, operation.abort.signal.aborted));
  } finally {
    operation.inFlight = false;
  }
}

/** 🛑️ Requests cancellation. The phase does NOT move optimistically: only the server's own answer
 * may report `cancelled`, so a hub that refuses the cancel can never be misreported as honoured. */
async function cancelInferenceJob(operationEpoch: number): Promise<void> {
  const operation = liveInferencePort(operationEpoch);
  if (operation === null || operation.status.jobId === null) return;
  advanceInferencePort(operation, { kind: "cancel" });
  if (operation.inFlight || operation.cancelSent) return;
  operation.cancelSent = true;
  operation.inFlight = true;
  try {
    const response = await inferenceBrokerFetch(operation, `/jobs/${operation.status.jobId}/cancel`, { method: "POST" });
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const page = parseGisMapInferenceEventPageV1(await readInferenceJson(response));
    if (liveInferencePort(operationEpoch) !== operation) return;
    advanceInferencePort(operation, { kind: "page", page });
    if (gisMapInferencePortTerminalV1(operation.status.phase)) retireInferencePort(operation);
    else scheduleInferencePoll(operation);
  } catch (error) {
    if (operation.closed) return;
    terminateInferencePort(operation, inferenceCodeFromRejection(error, operation.abort.signal.aborted));
  } finally {
    operation.inFlight = false;
  }
}

/** ✅️ Approves exactly the offered proposal, echoing back the server's own hash. It is never
 * retried: a duplicate approval could otherwise ask for a second Map commit. */
async function approveInferenceProposal(operationEpoch: number): Promise<void> {
  const operation = liveInferencePort(operationEpoch);
  if (
    operation === null
    || operation.status.phase !== "offered"
    || operation.status.jobId === null
    || operation.status.proposalHash === null
    || operation.status.preview?.jobId !== operation.status.jobId
    || operation.status.preview.proposalHash !== operation.status.proposalHash
    || operation.inFlight
  ) return;
  let body: string;
  try {
    body = JSON.stringify(sealGisMapInferenceApprovalRequestV1(operation.status.jobId, operation.status.proposalHash));
  } catch {
    terminateInferencePort(operation, "inference.invalid");
    return;
  }
  const jobId = operation.status.jobId;
  advanceInferencePort(operation, { kind: "approve" });
  operation.inFlight = true;
  try {
    const response = await inferenceBrokerFetch(operation, `/jobs/${jobId}/approval`, { method: "POST", body });
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const receipt = parseGisMapInferenceApprovalReceiptV1(await readInferenceJson(response));
    if (liveInferencePort(operationEpoch) !== operation) return;
    advanceInferencePort(operation, { kind: "approval", receipt });
    retireInferencePort(operation);
  } catch (error) {
    if (operation.closed) return;
    terminateInferencePort(operation, inferenceCodeFromRejection(error, operation.abort.signal.aborted));
  } finally {
    operation.inFlight = false;
  }
}

/** 🏁️ Releases a port that already reported its terminal, without publishing a second one. */
function retireInferencePort(operation: InferenceOperationV1): void {
  if (operation.closed) return;
  operation.closed = true;
  if (operation.pollTimer !== null) clearTimeout(operation.pollTimer);
  operation.pollTimer = null;
  operation.abort.abort(new Error("gis map inference port retired"));
  if (inferencePort === operation) inferencePort = null;
}

/** 🛑️ Renderer unmount or document close: cancel every in-flight call and report `cancelled` once. */
function closeInferencePort(operationEpoch: number): void {
  const operation = inferencePort;
  if (operation === null || operation.operationEpoch !== operationEpoch) return;
  if (gisMapInferencePortTerminalV1(operation.status.phase)) {
    retireInferencePort(operation);
    return;
  }
  terminateInferencePort(operation, "inference.cancelled");
}
//#endregion 💡️Inference

//#region 🔖️BlobCache
/** 📦️ Must match `framework/os/core/js/index.ts`'s `BLOB_ENDPOINT_PATH`. A hub-backed fallback
 * (for documents synced through a hub rather than a dev folder) is 0G's job once that route exists —
 * this worker only ever talks to the dev middleware today. */
const BLOB_ENDPOINT_PATH = "/semio-blob";

const BLOB_CACHE_DB_NAME = "semio-blob-cache";
const BLOB_CACHE_DB_VERSION = 1;
const BLOB_CACHE_STORE_NAME = "semio-blobs";
const BLOB_CACHE_LAST_ACCESSED_INDEX = "lastAccessedAt";
/** 💾️ IndexedDB eviction budget for the browser blob cache. 512 MiB comfortably fits a working set of
 * document media (images/audio/small video clips) without risking the browser's own storage-pressure
 * eviction of the whole origin; raise this once real usage data says otherwise. */
const BLOB_CACHE_BUDGET_BYTES = 512 * 1024 * 1024;

type CachedBlobRecord = { hash: string; mediaType: string; size: number; bytes: ArrayBuffer; lastAccessedAt: number };

function idbRequest<T>(request: IDBRequest): Promise<T> {
  return new Promise((resolve, reject) => {
    request.onsuccess = () => resolve(request.result as T);
    request.onerror = () => reject(request.error ?? new Error("indexeddb request failed"));
  });
}

function openBlobCacheDb(): Promise<IDBDatabase> {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(BLOB_CACHE_DB_NAME, BLOB_CACHE_DB_VERSION);
    request.onupgradeneeded = () => {
      const db = request.result;
      if (!db.objectStoreNames.contains(BLOB_CACHE_STORE_NAME)) {
        const store = db.createObjectStore(BLOB_CACHE_STORE_NAME, { keyPath: "hash" });
        store.createIndex(BLOB_CACHE_LAST_ACCESSED_INDEX, "lastAccessedAt");
      }
    };
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error ?? new Error("failed to open blob cache database"));
  });
}

let blobCacheDbPromise: Promise<IDBDatabase> | null = null;
function blobCacheDb(): Promise<IDBDatabase> {
  if (!blobCacheDbPromise) blobCacheDbPromise = openBlobCacheDb();
  return blobCacheDbPromise;
}

/** 🧮️ Running cache size, lazily seeded from a full scan on first use and kept in sync by
 * {@link writeCachedBlob}/eviction from then on — avoids a cursor sum on every put. */
let cachedTotalBytes: number | null = null;

async function blobCacheTotalBytes(db: IDBDatabase): Promise<number> {
  if (cachedTotalBytes != null) return cachedTotalBytes;
  const tx = db.transaction(BLOB_CACHE_STORE_NAME, "readonly");
  const records = await idbRequest<CachedBlobRecord[]>(tx.objectStore(BLOB_CACHE_STORE_NAME).getAll());
  cachedTotalBytes = records.reduce((sum, record) => sum + record.size, 0);
  return cachedTotalBytes;
}

/** ♻️ Evicts least-recently-accessed entries (via the `lastAccessedAt` index, ascending order) until
 * the running total drops back under {@link BLOB_CACHE_BUDGET_BYTES}. */
async function evictBlobCacheOverBudget(db: IDBDatabase): Promise<void> {
  let total = await blobCacheTotalBytes(db);
  if (total <= BLOB_CACHE_BUDGET_BYTES) return;
  const tx = db.transaction(BLOB_CACHE_STORE_NAME, "readwrite");
  const index = tx.objectStore(BLOB_CACHE_STORE_NAME).index(BLOB_CACHE_LAST_ACCESSED_INDEX);
  await new Promise<void>((resolve, reject) => {
    const cursorRequest = index.openCursor();
    cursorRequest.onsuccess = () => {
      const cursor = cursorRequest.result;
      if (!cursor || total <= BLOB_CACHE_BUDGET_BYTES) {
        resolve();
        return;
      }
      const record = cursor.value as CachedBlobRecord;
      total -= record.size;
      cursor.delete();
      cursor.continue();
    };
    cursorRequest.onerror = () => reject(cursorRequest.error ?? new Error("blob cache eviction cursor failed"));
  });
  cachedTotalBytes = total;
}

async function readCachedBlob(hash: string): Promise<CachedBlobRecord | null> {
  const db = await blobCacheDb();
  const tx = db.transaction(BLOB_CACHE_STORE_NAME, "readonly");
  const record = await idbRequest<CachedBlobRecord | undefined>(tx.objectStore(BLOB_CACHE_STORE_NAME).get(hash));
  return record ?? null;
}

async function writeCachedBlob(record: CachedBlobRecord): Promise<void> {
  const db = await blobCacheDb();
  const tx = db.transaction(BLOB_CACHE_STORE_NAME, "readwrite");
  await idbRequest(tx.objectStore(BLOB_CACHE_STORE_NAME).put(record));
  cachedTotalBytes = (cachedTotalBytes ?? (await blobCacheTotalBytes(db))) + record.size;
  await evictBlobCacheOverBudget(db);
}

/** 📥️ Reads a blob by hash — cache-first (bumping `lastAccessedAt` for LRU), falling back to the dev
 * server's `GET ${BLOB_ENDPOINT_PATH}/:hash` on a miss and populating the cache. Nothing outside this
 * worker calls this yet (no plugin/UI surface consumes blobs today), so it stays internal rather than
 * growing {@link BackboneWorkerRequest}/{@link BackboneWorkerResponse} with variants nothing sends. */
async function getCachedBlob(hash: string): Promise<{ bytes: Uint8Array; mediaType: string } | null> {
  const cached = await readCachedBlob(hash);
  if (cached) {
    void writeCachedBlob({ ...cached, lastAccessedAt: Date.now() });
    return { bytes: new Uint8Array(cached.bytes), mediaType: cached.mediaType };
  }
  // ⏱️ Finding 3: no per-document abort context exists here (the blob cache is global, not tied to
  // one document), so a fixed timeout is the whole story — still enough that a stalled dev-server
  // response can't hang this call forever.
  const response = (await fetchWithTimeout(`${BLOB_ENDPOINT_PATH}/${encodeURIComponent(hash)}`, undefined, { timeoutMs: BLOB_FETCH_TIMEOUT_MS })) as BinaryFetchTimeoutResponse;
  if (response.status === 404) return null;
  if (!response.ok) throw new Error(`blob fetch failed (${response.status})`);
  const mediaType = response.headers.get("content-type") ?? "application/octet-stream";
  const buffer = await response.arrayBuffer();
  await writeCachedBlob({ hash, mediaType, size: buffer.byteLength, bytes: buffer, lastAccessedAt: Date.now() });
  return { bytes: new Uint8Array(buffer), mediaType };
}

/** 📤️ Writes a blob to the dev server's content-addressed store, caching it locally under the hash the
 * server returns (content-addressing means the caller can't pick the cache key up front). */
async function putCachedBlob(bytes: Uint8Array, mediaType: string): Promise<string> {
  const response = await fetchWithTimeout(
    `${BLOB_ENDPOINT_PATH}?mediaType=${encodeURIComponent(mediaType)}`,
    { method: "PUT", headers: { "content-type": "application/octet-stream" }, body: new Uint8Array(bytes) },
    { timeoutMs: BLOB_FETCH_TIMEOUT_MS },
  );
  if (!response.ok) throw new Error(`blob put failed (${response.status})`);
  const { hash } = (await response.json()) as { hash: string };
  await writeCachedBlob({ hash, mediaType, size: bytes.byteLength, bytes: bytes.slice().buffer, lastAccessedAt: Date.now() });
  return hash;
}

// 🧷️ Referenced defensively so `getCachedBlob`/`putCachedBlob` aren't flagged unused before a
// plugin/UI surface calls into them — both are the intended entry points once one does.
void getCachedBlob;
void putCachedBlob;
//#endregion 🔖️BlobCache

//#region 🔖️Lifecycle
function openArtifact(config: ArtifactActorConfig): void {
  const hub = hubBinding(config);
  const runtimeKey = documentRuntimeKeyForConfig(config);
  closeArtifactRuntime(runtimeKey);
  const channel = new BroadcastChannel(`semio-doc-${runtimeKey}`);
  const state: ArtifactState = {
    runtimeKey,
    config,
    openClientInstanceId: crypto.randomUUID(),
    actor: hub === null ? config.actor : "",
    hubActorReady: hub === null,
    pendingSocketActorId: null,
    channel,
    socket: null,
    presenceAuthority: null,
    docAbort: new AbortController(),
    executionTargetLease: null,
    sanityPollTimer: null,
    sseHealthy: false,
    revalidateFolder: async () => {}, // 🔧 replaced below once a folder binding exists.
    reconnectDelayMs: HUB_RECONNECT_MIN_MS,
    outbox: [],
    pendingMutations: [],
    status: { persisted: false, pendingMutations: 0, remote: { kind: "detached" } },
    frontier: null,
    pendingResumeToken: null,
    requiredTailFrontier: null,
    artifactBootstrap: null,
    artifactBootstrapDeadlineMs: null,
    artifactBootstrapProgress: [],
    currentPack: null,
    currentSpr: null,
    hubFrameChain: Promise.resolve(),
    resumeToken: null,
    sessionColor: null,
    pendingBatches: new Map(),
    nextBatchId: 0,
    hlcCounter: 0,
    closed: false,
  };
  artifacts.set(runtimeKey, state);
  channel.onmessage = (messageEvent) => {
    const envelopes = messageEvent.data as MutationEnvelope[];
    if (Array.isArray(envelopes) && envelopes.length > 0) emitEvent(state, { kind: "remoteMutations", envelopes });
  };
  const folder = folderBinding(config);
  if (folder) {
    // 🥇️ One single-flight guard per document (finding 1), shared by every trigger source.
    state.revalidateFolder = latestWins(() => pollFolderOnce(state, folder));
    if (config.watchExternal !== false) watchFolder(state, folder);
    else void state.revalidateFolder();
  }
  if (hub?.installedTarget === undefined && socketGrantTestIssue === null) {
    const scope = artifactScope(state);
    post({ kind: "socket-actor-failed", documentId: config.documentId, ...(scope === undefined ? {} : { scope }), code: "installed-target-unavailable" });
  } else if (hub) {
    connectHub(state, hub);
  }
  emitEvent(state, { kind: "status", ...state.status });
}

function closeArtifactRuntime(runtimeKey: string): void {
  const state = artifacts.get(runtimeKey);
  if (!state) return;
  state.closed = true;
  abortArtifactBootstrap(state);
  // 🛑️ Finding 3: cancels every in-flight folder/blob fetch this document owns and unblocks any
  // pending reconnect backoff delay immediately — no fetch or reconnect loop can pin this document
  // after this line.
  state.docAbort.abort();
  // 💡️ The inference port exists only while this document's lease does — a close retires it with a
  // localized terminal before the lease buffers are wiped.
  if (inferencePort !== null && documentRuntimeKeyV1({ kind: "hub", ...inferencePort.scope }) === runtimeKey) closeInferencePort(inferencePort.operationEpoch);
  dropDocumentExecutionTargetLease(state);
  state.socket?.close();
  if (state.sanityPollTimer != null) clearTimeout(state.sanityPollTimer);
  state.channel.close();
  artifacts.delete(runtimeKey);
}

function closeArtifact(documentId: string, spaceId?: string): void {
  const runtimeKey = artifactRuntimeKey(documentId, spaceId);
  if (runtimeKey !== null) closeArtifactRuntime(runtimeKey);
}

async function handleLocalMsg(state: ArtifactState, message: ArtifactActorMsg): Promise<void> {
  switch (message.kind) {
    case "localMutations": {
      if (message.envelopes.length === 0) break; // pure wake
      // 🚨️ Finding 5: bounded queue, rejected+reported wholesale rather than silently dropped or
      // partially accepted — see {@link rejectMutationQueueOverflow}.
      if (state.pendingMutations.length + message.envelopes.length > PENDING_MUTATIONS_QUEUE_LIMIT) {
        rejectMutationQueueOverflow(state, message.envelopes);
        break;
      }
      state.pendingMutations.push(...message.envelopes);
      setStatus(state, { pendingMutations: state.pendingMutations.length });
      state.channel.postMessage(message.envelopes);
      relayMutationsToHub(state, message.envelopes);
      const folder = folderBinding(state.config);
      // 📁️ Folder persistence only understands whole-envelope snapshots today (`vcs::FolderSqliteStorage`
      // stores one json blob per document) — a local operation still marks the document dirty so the next
      // `localSnapshot` (which every `store.dispatch` triggers via `flush_outbound`) persists it.
      if (folder) setStatus(state, { persisted: false });
      break;
    }
    case "localSnapshot": {
      const folder = folderBinding(state.config);
      if (folder) {
        try {
          await writeFolder(state, folder, message.pack, message.spr);
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
//#endregion 🔖️Lifecycle

//#region 🔖️MessageBridge
function handleTsRequest(request: BackboneWorkerRequest): void {
  switch (request.kind) {
    case "open":
      openArtifact(request);
      break;
    case "close":
      closeArtifact(request.documentId, request.spaceId);
      break;
    case "send": {
      const state = artifactState(request.documentId, request.spaceId);
      if (state) void handleLocalMsg(state, request.message);
      break;
    }
    case "directory-open":
      openDirectory(request.baseUrl, request.since);
      break;
    case "directory-bootstrap-open":
      openDirectoryBootstrap(request.baseUrl, request.after, request.bootstrapEpoch);
      break;
    case "directory-bootstrap-ack":
      acknowledgeDirectoryBootstrap(request);
      break;
    case "directory-bootstrap-reject":
      rejectDirectoryBootstrap(request.bootstrapEpoch, request.receiptSha256);
      break;
    case "directory-bootstrap-close":
      if (directoryBootstrap?.machine.bootstrapEpoch === request.bootstrapEpoch) closeDirectory();
      break;
    case "directory-scope-open":
      openScopedDirectory(request.baseUrl, request.scope, request.since);
      break;
    case "directory-scope-close":
      closeScopedDirectory(request.scope);
      break;
    case "directory-command":
      void submitDirectoryCommand(request.requestId, request.command);
      break;
    case "directory-command-cancel":
      cancelDirectoryCommand(request.requestId);
      break;
    case "directory-administration-open":
      openDirectoryAdministration(request.operationEpoch, request.spaceId);
      break;
    case "directory-administration-refresh":
      void loadDirectoryAdministrationPage(request.operationEpoch, request.cursor, "refreshing");
      break;
    case "directory-administration-submit":
      void submitDirectoryAdministrationCommand(request.operationEpoch, request.requestId, request.command);
      break;
    case "directory-administration-capability-request":
      requestDirectoryAdministrationCapability(request.operationEpoch);
      break;
    case "directory-administration-capability-result":
      settleDirectoryAdministrationCapability(request.operationEpoch, request.transferEpoch, request.copied);
      break;
    case "directory-administration-close":
      closeDirectoryAdministration(request.operationEpoch);
      break;
    case "directory-close":
      closeDirectory();
      break;
    case "inference-open":
      openInferencePort(request.operationEpoch, request.scope);
      break;
    case "inference-propose":
      void submitInferenceJob(request.operationEpoch, request.requestId);
      break;
    case "inference-poll":
      void pollInferenceJob(request.operationEpoch);
      break;
    case "inference-cancel":
      void cancelInferenceJob(request.operationEpoch);
      break;
    case "inference-approve":
      void approveInferenceProposal(request.operationEpoch);
      break;
    case "inference-close":
      closeInferencePort(request.operationEpoch);
      break;
  }
}
//#endregion 🔖️MessageBridge
//#endregion 🔖️TsFallback

//#region 🧪️Tests
// 🧵️ Whole block stripped from production builds (see this file's header doc) — `node:*` imports
// below are dynamic specifically so they never get bundled into the actual browser Worker script.
if (import.meta.vitest) {
  const { beforeEach, describe, expect, it, vi } = import.meta.vitest;

  beforeEach(() => {
    clearLocalBrowserBrokerProof();
    installLocalBrowserBrokerProof("4".repeat(64));
    socketGrantTestIssue = async () => ({
      schema: "semio.hub.socket-grant/v1",
      protocol: "semio.socket.v1",
      grant: `socket.v1.${"1".repeat(32)}.${"2".repeat(64)}`,
      actorId: `hub.v1.${"3".repeat(64)}`,
      expiresAtMs: Number.MAX_SAFE_INTEGER,
    });
  });

  describe("DirectoryEventPageBootstrapV1", () => {
    const page = (afterSeqExclusive: number, throughSeqInclusive: number, hasMore: boolean, receiptSha256: string): CanonicalDirectoryEventPageV1 => ({
      canonicalJson: "{}",
      sessionBindingSha256: "a".repeat(64),
      authorizationGeneration: 9,
      afterSeqExclusive,
      throughSeqInclusive,
      hasMore,
      receiptSha256,
    });
    const ack = (value: CanonicalDirectoryEventPageV1, bootstrapEpoch = 7): DirectoryEventPageAckV1 => ({
      bootstrapEpoch,
      sessionBindingSha256: value.sessionBindingSha256,
      authorizationGeneration: value.authorizationGeneration,
      throughSeqInclusive: value.throughSeqInclusive,
      receiptSha256: value.receiptSha256,
    });

    it("serializes page delivery, exact Home acknowledgement, and live wakeup cursor ownership", () => {
      const machine = new DirectoryEventPageBootstrapV1(7, 3);
      const first = page(3, 5, true, "b".repeat(64));
      const second = page(5, 8, false, "c".repeat(64));
      machine.present(first);
      expect(() => machine.present(second)).toThrow("page ordering mismatch");
      expect(() => machine.acknowledge({ ...ack(first), receiptSha256: "d".repeat(64) })).toThrow("acknowledgement mismatch");
      expect(machine.after()).toBe(3);
      expect(machine.acknowledge(ack(first))).toEqual({ kind: "fetch", after: 5 });
      machine.present(second);
      expect(() => machine.acknowledge(ack(second, 8))).toThrow("acknowledgement mismatch");
      expect(machine.acknowledge(ack(second))).toEqual({ kind: "live", since: 8 });
      expect(machine.wake(false)).toBe(8);
      expect(machine.wake(false)).toBeNull();
    });

    it("round trips exact worker ACK and page envelopes without a raw identity secret", () => {
      const first = page(3, 5, true, "b".repeat(64));
      const request: BackboneWorkerRequest = { kind: "directory-bootstrap-ack", ...ack(first) };
      const response: BackboneWorkerResponse = { kind: "directory-event-page", ...ack(first), canonicalJson: first.canonicalJson, afterSeqExclusive: first.afterSeqExclusive, hasMore: first.hasMore };
      expect(decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request))).toEqual(request);
      expect(decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response))).toEqual(response);
      expect(JSON.stringify(response)).not.toContain("session.v1.");
    });

    it("allowlists only one canonical safe-decimal event-page route and keeps bootstrap on the TypeScript owner", async () => {
      const originalFetch = globalThis.fetch;
      const urls: string[] = [];
      (globalThis as unknown as { fetch: unknown }).fetch = async (input: string) => {
        urls.push(input);
        return new Response("{}", { status: 200, headers: { "x-semio-browser-broker-advanced": "1" } });
      };
      try {
        await browserDirectoryRequest("/_semio/hub/directory/event-page/v1?after=3", {}, { timeoutMs: 1_000 });
        await expect(browserDirectoryRequest("/_semio/hub/directory/event-page/v1?after=03", {}, { timeoutMs: 1_000 })).rejects.toThrow("directory operation denied");
        await expect(browserDirectoryRequest("/_semio/hub/directory/event-page/v1?after=9007199254740992", {}, { timeoutMs: 1_000 })).rejects.toThrow("directory operation denied");
        expect(urls).toEqual(["/_semio/hub/directory/event-page/v1?after=3"]);
        const typescriptRequests: BackboneWorkerRequest[] = [];
        const rustRequests: BackboneWorkerRequest[] = [];
        const request: BackboneWorkerRequest = { kind: "directory-bootstrap-open", baseUrl: "http://hub.test", after: 3, bootstrapEpoch: 7 };
        dispatchBackboneWorkerRequest(request, { handleRequestBytes: (wire) => rustRequests.push(decodeBackboneWorkerRequest(wire)), postReady() {} }, (value) => typescriptRequests.push(value));
        expect(typescriptRequests).toEqual([request]);
        expect(rustRequests).toEqual([]);
      } finally {
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });
  });

  describe("browser broker proof ratchet", () => {
    it("advances only on an explicit acknowledgement and domain-binds the next proof digest", async () => {
      const originalFetch = globalThis.fetch;
      const requests: Headers[] = [];
      (globalThis as unknown as { fetch: unknown }).fetch = async (_input: unknown, init?: RequestInit) => {
        requests.push(new Headers(init?.headers));
        return new Response("{}", { status: 200, headers: { "x-semio-browser-broker-advanced": "1" } });
      };
      try {
        await browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 });
        await browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 });
        const firstNextDigest = requests[0]!.get("x-semio-browser-broker-next");
        const secondCurrent = hexBytes(requests[1]!.get("x-semio-browser-broker") ?? "");
        expect(secondCurrent).toBeDefined();
        expect(bytesHex(await browserBrokerProofDigest(secondCurrent!))).toBe(firstNextDigest);
        expect(requests[0]!.get("x-semio-browser-broker")).not.toBe(requests[1]!.get("x-semio-browser-broker"));
      } finally {
        clearLocalBrowserBrokerProof();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

    it("requires explicit rebootstrap after a lost acknowledgement, 401, or cancel-after-send", async () => {
      const originalFetch = globalThis.fetch;
      let calls = 0;
      try {
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          calls += 1;
          throw new Error("transport detail must be redacted");
        };
        await expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("browser broker rebootstrap required");
        await expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("browser broker rebootstrap required");
        expect(calls).toBe(1);

        installLocalBrowserBrokerProof("6".repeat(64));
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          calls += 1;
          return new Response("unauthorized", { status: 401, headers: { "x-semio-browser-broker-advanced": "1" } });
        };
        await expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("browser broker rebootstrap required");

        installLocalBrowserBrokerProof("7".repeat(64));
        const abort = new AbortController();
        (globalThis as unknown as { fetch: unknown }).fetch = async (_input: unknown, init?: RequestInit) => {
          calls += 1;
          await new Promise<void>((_resolve, reject) => init?.signal?.addEventListener("abort", () => reject(new Error("cancelled")), { once: true }));
          return new Response("{}");
        };
        const pending = browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000, signal: abort.signal });
        await Promise.resolve();
        abort.abort();
        await expect(pending).rejects.toThrow("browser broker rebootstrap required");
        await expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("browser broker rebootstrap required");
      } finally {
        clearLocalBrowserBrokerProof();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

    it("rejects expired, duplicate-initialized, and over-capacity broker work without exposing proof", async () => {
      const originalFetch = globalThis.fetch;
      const originalQueued = localBrowserBrokerQueued;
      let observedCurrent = "";
      (globalThis as unknown as { fetch: unknown }).fetch = async (_input: unknown, init?: RequestInit) => {
        observedCurrent = new Headers(init?.headers).get("x-semio-browser-broker") ?? "";
        return new Response("missing acknowledgement", { status: 200 });
      };
      try {
        expect(installLocalBrowserBrokerProof("8".repeat(64))).toBe(false);
        await expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("browser broker rebootstrap required");
        expect(observedCurrent).toBe("4".repeat(64));

        installLocalBrowserBrokerProof("9".repeat(64));
        localBrowserBrokerProofExpiresAtMs = Date.now() - 1;
        await expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("browser broker rebootstrap required");

        installLocalBrowserBrokerProof("a".repeat(64));
        localBrowserBrokerQueued = 64;
        await expect(browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 1_000 })).rejects.toThrow("browser broker capacity exceeded");
        expect(observedCurrent).not.toContain("a".repeat(64));
      } finally {
        localBrowserBrokerQueued = originalQueued;
        clearLocalBrowserBrokerProof();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

    it("keeps the private port and proof names out of malicious plugin shard source and transfers before activation", async () => {
      const { readFile } = await import("node:fs/promises");
      const pluginShard = await readFile(new URL("./🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts", import.meta.url), "utf8");
      const shellHost = await readFile(new URL("./🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx", import.meta.url), "utf8");
      expect(pluginShard).not.toContain("semio-browser-broker-port");
      expect(pluginShard).not.toContain("x-semio-browser-broker");
      expect(shellHost.indexOf("const worker = ensureBackboneWorker();")).toBeLessThan(shellHost.indexOf("void (async () => {\n      const outcome = await installPlugin"));
      expect(shellHost.indexOf("window.history.replaceState")).toBeLessThan(shellHost.indexOf("loadPluginModuleResilient"));
    });
  });

  function sampleEnvelope(): MutationEnvelope {
    return {
      id: "edit-1",
      actor: "actor-1",
      document: "doc-1",
      schemaVersion: "demo/v1",
      deps: [],
      payloadHash: "unused-in-this-fallback",
      diff: { schemaId: "demo/v1", payload: { n: 5, sequenceNumber: 1 } },
      inverse: { targetOperation: "edit-1", inverseDiff: { schemaId: "demo/v1", payload: { n: 0 } }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
    };
  }

  async function flushSocketGrantTurns(): Promise<void> {
    await Promise.resolve();
    await Promise.resolve();
    await Promise.resolve();
  }

  describe("backbone-worker wire bridge", () => {
    it("round-trips an MutationEnvelope through toWireEnvelope/fromWireEnvelope", () => {
      const envelope = sampleEnvelope();
      const wire = toWireEnvelope(envelope, { actor: 1, physical_ms: 2, logical: 3 });
      expect(wire.mutation_id).toBe(envelope.id);
      expect(wire.document_id).toBe(envelope.document);
      expect(wire.actor).toBe(envelope.actor);
      expect(decodePackPayload(wire.diff.payload)).toEqual(envelope.diff.payload);

      const recovered = fromWireEnvelope(wire);
      expect(recovered.id).toBe(envelope.id);
      expect(recovered.document).toBe(envelope.document);
      expect(recovered.diff.payload).toEqual(envelope.diff.payload);
      expect(recovered.inverse.inverseDiff.payload).toEqual(envelope.inverse.inverseDiff.payload);
    });

    it("rollbackEnvelope synthesizes an undo from the original inverse", () => {
      const envelope = sampleEnvelope();
      const rollback = rollbackEnvelope(envelope);
      expect(rollback.deps).toEqual([envelope.id]);
      expect(rollback.diff.payload).toEqual(envelope.inverse.inverseDiff.payload);
      expect(rollback.id).not.toBe(envelope.id);
    });

    it("decodeClientFrame terminally rejects the removed tag-zero Hello carrier", () => {
      expect(() => decodeClientFrame(new Uint8Array([0, 0]))).toThrow(/unknown tag 0/);
    });

    // 🎨️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4:
    // `stampSession` is the ONE place `peer.color`/`peer.surface` are ever filled — shells never set
    // them themselves. Overwrites whatever the caller handed in, and derives `surface` from the
    // document's own hub binding (`null`/absent for a folder-only document).
    it("stampSession fills color/surface from actor state, overwriting whatever the caller set", () => {
      const installedTarget = {
        package: { pluginId: "s.test", packageId: "s.test.codec", version: "1", componentSha256: "1".repeat(64), componentBlake3: "2".repeat(64), descriptorByteSha256: "3".repeat(64) },
        artifact: { kind: "test", schema: "demo/v1", packSchemaHash: "4".repeat(64) },
        parentDialect: { artifactKind: "test", standard: "1", subset: "*" },
        surface: { surfaceId: "s.space.home@1/*#editor", appId: "app.test", windowKindId: "window.document", role: "editor" as const, rendererTarget: "react" as const },
      };
      const hubConfig: ArtifactActorConfig = { documentId: "doc-1", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1", installedTarget }], actor: "actor-1" };
      const hubState = { config: hubConfig, sessionColor: 7 } as unknown as ArtifactState;
      const peer: ArtifactPresencePeer = { actor: "actor-1", connectedAtMs: 1000, color: 99, surface: "shell-should-never-set-this", views: [] };
      const stamped = stampSession(peer, hubState);
      expect(stamped.color).toBe(7);
      expect(stamped.surface).toBe("s.space.home@1/*#editor");

      const folderConfig: ArtifactActorConfig = { documentId: "doc-2", schema: "demo/v1", bindings: [{ kind: "folder", path: "/tmp/doc-2" }], actor: "actor-1" };
      const folderState = { config: folderConfig, sessionColor: null } as unknown as ArtifactState;
      const stampedFolder = stampSession(peer, folderState);
      expect(stampedFolder.color).toBeUndefined();
      expect(stampedFolder.surface).toBeUndefined();
    });

    it("handleHubFrame stores the hub-assigned session color on a Session frame", () => {
      const config: ArtifactActorConfig = { documentId: "doc-3", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1" };
      const state = { config, actor: "", hubActorReady: false, pendingSocketActorId: "actor-1", outbox: [], sessionColor: null } as unknown as ArtifactState;
      handleHubFrame(state, { Session: { actor: "actor-1", color: 3 } });
      expect(state.sessionColor).toBe(3);
    });

  });

  //#region 🧪️ArtifactBootstrapRestore
  type ArtifactBootstrapFixture = Readonly<{
    artifact: Readonly<{ schema: string; packSchemaHash: string; requiredTailFrontier: Readonly<{ documentId: string; headEditOrdinal: number; headEditId: string; lastCommitSeq: number; chainHash: string }> }>;
    payload: Readonly<{ packHex: string; sprHex: string }>;
    wire: Readonly<{ inlineWelcomeHex: string; chunkedWelcomeHex: string; chunkHex: readonly string[]; doneHex: string }>;
  }>;

  function bytesFromHex(hex: string): Uint8Array {
    return Uint8Array.from(hex.match(/../g)?.map((byte) => Number.parseInt(byte, 16)) ?? []);
  }

  async function artifactBootstrapFixture(): Promise<ArtifactBootstrapFixture> {
    const { readFile } = await import("node:fs/promises");
    return JSON.parse(await readFile(new URL("../../🔨️modules/📡️replication/🧫️fixtures/🚀️artifact-bootstrap/🔣️.json", import.meta.url), "utf8")) as ArtifactBootstrapFixture;
  }

  function fixtureConfig(fixture: ArtifactBootstrapFixture): ArtifactActorConfig {
    return { documentId: fixture.artifact.requiredTailFrontier.documentId, schema: fixture.artifact.schema, packSchemaHash: Array.from(bytesFromHex(fixture.artifact.packSchemaHash)), bindings: [], actor: "actor-bootstrap-test", watchExternal: false };
  }

  function decodeFixtureFrame(hex: string): ServerFrame {
    return decodeServerFrame(bytesFromHex(hex)).frame;
  }

  function fixtureRequiredFrontier(fixture: ArtifactBootstrapFixture): WireFrontierSummary {
    const frontier = fixture.artifact.requiredTailFrontier;
    return { document_id: frontier.documentId, head_edit_ordinal: frontier.headEditOrdinal, head_edit_id: frontier.headEditId, last_commit_seq: frontier.lastCommitSeq, chain_hash: Array.from(bytesFromHex(frontier.chainHash)) };
  }

  async function installFixture(fixture: ArtifactBootstrapFixture, chunked: boolean): Promise<ArtifactState> {
    const config = fixtureConfig(fixture);
    openArtifact(config);
    const state = artifactState(config.documentId)!;
    await handleHubFrame(state, decodeFixtureFrame(chunked ? fixture.wire.chunkedWelcomeHex : fixture.wire.inlineWelcomeHex));
    if (chunked) {
      for (const chunk of fixture.wire.chunkHex) await handleHubFrame(state, decodeFixtureFrame(chunk));
      await handleHubFrame(state, decodeFixtureFrame(fixture.wire.doneHex));
    }
    return state;
  }

  describe("artifact bootstrap atomic restore", () => {
    it("installs the exact neutral inline and chunked pair and reaches Live only at the authenticated tail", async () => {
      const fixture = await artifactBootstrapFixture();
      const pack = bytesFromHex(fixture.payload.packHex);
      const spr = bytesFromHex(fixture.payload.sprHex);
      const inline = await installFixture(fixture, false);
      expect(inline.currentPack).toEqual(pack);
      expect(inline.currentSpr).toEqual(spr);
      expect(inline.status.remote.kind).not.toBe("live");
      const inlinePack = inline.currentPack;
      const inlineSpr = inline.currentSpr;
      const inlineProgress = [...inline.artifactBootstrapProgress];
      await handleHubFrame(inline, { Commands: { envelopes: [], origin: inline.config.actor, frontier: fixtureRequiredFrontier(fixture) } });
      expect(inline.status.remote.kind).toBe("live");
      expect(inline.resumeToken).toBe("resume-bootstrap-1");
      closeArtifact(inline.config.documentId);

      const chunked = await installFixture(fixture, true);
      expect(chunked.currentPack).toEqual(pack);
      expect(chunked.currentSpr).toEqual(spr);
      expect(chunked.currentPack).toEqual(inlinePack);
      expect(chunked.currentSpr).toEqual(inlineSpr);
      expect(chunked.artifactBootstrapProgress.every((progress, index, all) => index === 0 || (progress.receivedBytes >= all[index - 1]!.receivedBytes && progress.receivedChunks >= all[index - 1]!.receivedChunks))).toBe(true);
      expect(chunked.artifactBootstrapProgress.at(-1)).toMatchObject({ receivedBytes: pack.length + spr.length, receivedChunks: fixture.wire.chunkHex.length });
      expect(inlineProgress.at(-1)).toMatchObject({ receivedBytes: pack.length + spr.length });
      closeArtifact(chunked.config.documentId);
    });

    it("does not reach Live for a same-ordinal frontier with a different authenticated head or chain", async () => {
      const fixture = await artifactBootstrapFixture();
      const state = await installFixture(fixture, false);
      const wrong = { ...fixtureRequiredFrontier(fixture), head_edit_id: "edit-wrong", chain_hash: Array(32).fill(0x55) };
      await handleHubFrame(state, { Commands: { envelopes: [], origin: state.config.actor, frontier: wrong } });
      expect(state.status.remote.kind).not.toBe("live");
      expect(state.requiredTailFrontier).not.toBeNull();
      expect(state.resumeToken).toBeNull();
      await handleHubFrame(state, { Commands: { envelopes: [], origin: state.config.actor, frontier: fixtureRequiredFrontier(fixture) } });
      expect(state.status.remote.kind).toBe("live");
      closeArtifact(state.config.documentId);
    });

    it("invalidates the committed session before rebootstrap and bounds typed failure diagnostics", async () => {
      const fixture = await artifactBootstrapFixture();
      const state = await installFixture(fixture, false);
      state.config = { ...state.config, bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "space-a" }] };
      state.resumeToken = "stale-resume";
      await handleHubFrame(state, {
        RebootstrapRequired: {
          control: {
            space_id: "space-a",
            document_id: state.config.documentId,
            checkpoint_id: Array(32).fill(1),
            descriptor_hash: Array(32).fill(2),
            baseline_frontier: fixtureRequiredFrontier(fixture),
          },
        },
      });
      expect(state.currentPack).toBeNull();
      expect(state.currentSpr).toBeNull();
      expect(state.frontier).toBeNull();
      expect(state.resumeToken).toBeNull();
      expect(state.status.remote.kind).toBe("connecting");
      const diagnostic = artifactBootstrapFailure(state, new Error("€".repeat(4_096)));
      expect(new TextEncoder().encode(diagnostic.message).byteLength).toBeLessThanOrEqual(ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES);
      closeArtifact(state.config.documentId);
    });

    it("discards malformed and disconnected staging, preserves the prior commit, and restarts fresh", async () => {
      const fixture = await artifactBootstrapFixture();
      const config = fixtureConfig(fixture);
      openArtifact(config);
      const state = artifactState(config.documentId)!;
      state.currentPack = Uint8Array.of(9);
      state.currentSpr = Uint8Array.of(8);
      const priorFrontier: WireFrontierSummary = { document_id: state.config.documentId, head_edit_ordinal: 1, head_edit_id: "old", last_commit_seq: 1, chain_hash: Array(32).fill(7) };
      state.frontier = priorFrontier;
      await handleHubFrame(state, decodeFixtureFrame(fixture.wire.chunkedWelcomeHex));
      const malformed = structuredClone(decodeFixtureFrame(fixture.wire.chunkHex[0]!));
      if (!("ArtifactBootstrapChunk" in malformed)) throw new Error("fixture chunk expected");
      const malformedFrame: ServerFrame = { ArtifactBootstrapChunk: { ...malformed.ArtifactBootstrapChunk, descriptor_hash: malformed.ArtifactBootstrapChunk.descriptor_hash.map((byte, index) => index === 0 ? byte ^ 0xff : byte) } };
      await handleHubFrame(state, malformedFrame);
      expect(state.currentPack).toEqual(Uint8Array.of(9));
      expect(state.currentSpr).toEqual(Uint8Array.of(8));
      expect(state.frontier).toEqual(priorFrontier);
      expect(state.artifactBootstrap).toBeNull();

      await handleHubFrame(state, { SnapshotDone: { seq_count: 1 } });
      expect(state.currentPack).toEqual(Uint8Array.of(9));
      expect(state.currentSpr).toEqual(Uint8Array.of(8));
      expect(state.frontier).toEqual(priorFrontier);
      expect(state.artifactBootstrap).toBeNull();

      await handleHubFrame(state, decodeFixtureFrame(fixture.wire.chunkedWelcomeHex));
      await handleHubFrame(state, decodeFixtureFrame(fixture.wire.chunkHex[0]!));
      abortArtifactBootstrap(state);
      expect(state.frontier).toEqual(priorFrontier);
      await handleHubFrame(state, decodeFixtureFrame(fixture.wire.chunkedWelcomeHex));
      for (const chunk of fixture.wire.chunkHex) await handleHubFrame(state, decodeFixtureFrame(chunk));
      await handleHubFrame(state, decodeFixtureFrame(fixture.wire.doneHex));
      expect(state.currentPack).toEqual(bytesFromHex(fixture.payload.packHex));
      expect(state.currentSpr).toEqual(bytesFromHex(fixture.payload.sprHex));
      closeArtifact(state.config.documentId);
    });

    it("preserves one pending local edit across replacement and catch-up without duplicate replay", async () => {
      const fixture = await artifactBootstrapFixture();
      const state = await installFixture(fixture, false);
      const local = { ...sampleEnvelope(), id: "pending-local", document: state.config.documentId, schemaVersion: state.config.schema };
      queueOutbox(state, [local, local]);
      expect(state.outbox.map((envelope) => envelope.id)).toEqual(["pending-local"]);
      await handleHubFrame(state, { Commands: { envelopes: [], origin: state.config.actor, frontier: fixtureRequiredFrontier(fixture) } });
      expect(state.outbox.map((envelope) => envelope.id)).toEqual(["pending-local"]);
      expect(state.pendingBatches.size).toBe(0);
      closeArtifact(state.config.documentId);
    });

    it("commits neither pair nor frontier when the atomic folder envelope PUT fails", async () => {
      const fixture = await artifactBootstrapFixture();
      const config = fixtureConfig(fixture);
      openArtifact(config);
      const state = artifactState(config.documentId)!;
      state.config = { ...state.config, bindings: [{ kind: "folder", path: "/tmp/bootstrap-put-failure" }] };
      state.currentPack = Uint8Array.of(1);
      state.currentSpr = Uint8Array.of(2);
      const priorFrontier: WireFrontierSummary = { document_id: state.config.documentId, head_edit_ordinal: 1, head_edit_id: "old", last_commit_seq: 1, chain_hash: Array(32).fill(6) };
      state.frontier = priorFrontier;
      const originalFetch = globalThis.fetch;
      let puts = 0;
      globalThis.fetch = (async (_input: string | URL | Request, init?: RequestInit) => {
        if (init?.method === "PUT") puts += 1;
        return { ok: false, status: 500, statusText: "fixture failure", headers: { get: () => null }, json: async () => ({}), text: async () => "" } as unknown as Response;
      }) as typeof fetch;
      try {
        await handleHubFrame(state, decodeFixtureFrame(fixture.wire.inlineWelcomeHex));
        expect(puts).toBe(1);
        expect(state.currentPack).toEqual(Uint8Array.of(1));
        expect(state.currentSpr).toEqual(Uint8Array.of(2));
        expect(state.frontier).toEqual(priorFrontier);
        expect(state.artifactBootstrap).toBeNull();
      } finally {
        globalThis.fetch = originalFetch;
        closeArtifact(state.config.documentId);
      }
    });
  });
  //#endregion 🧪️ArtifactBootstrapRestore

  //#region 🔖️IdentityTests
  describe("identity config facet", () => {
    function sampleIdentity(overrides: Partial<Identity> = {}): Identity {
      return { userId: "u-1", email: "ada@semio.dev", displayName: "Ada", hubBaseUrl: "http://hub.test", issuedAtMs: 1_000, ...overrides };
    }

    it("identityActorConfig binds the folder lane under `${dataDir}/os` when given a dataDir, else local-only", () => {
      expect(identityActorConfig("actor-1", "/tmp/s-user1")).toEqual({
        documentId: IDENTITY_CONFIG_SCHEMA,
        schema: IDENTITY_CONFIG_SCHEMA,
        bindings: [{ kind: "folder", path: "/tmp/s-user1/os" }],
        actor: "actor-1",
      });
      expect(identityActorConfig("actor-1")).toEqual({ documentId: IDENTITY_CONFIG_SCHEMA, schema: IDENTITY_CONFIG_SCHEMA, bindings: [], actor: "actor-1" });
    });

    it("sign-in -> sign-out -> sign-in round-trips through applyIdentityConfigMutation, and each inverts the last", async () => {
      const { applyIdentityConfigMutation, inverseIdentityConfigMutation, signIn, signOut } = await import("./🎚️config/🧬️schema/🧬️mutations/🟦️");

      const first = sampleIdentity();
      const afterFirstSignIn = applyIdentityConfigMutation(null, signIn(first));
      expect(afterFirstSignIn).toEqual(first);

      const afterSignOut = applyIdentityConfigMutation(afterFirstSignIn, signOut());
      expect(afterSignOut).toBeNull();

      const second = sampleIdentity({ userId: "u-2", email: "devon@semio.dev", displayName: "Devon", issuedAtMs: 2_000 });
      const afterSecondSignIn = applyIdentityConfigMutation(afterSignOut, signIn(second));
      expect(afterSecondSignIn).toEqual(second);

      // ↩️ sign-out's inverse, from the base it cleared, restores exactly the prior session.
      expect(inverseIdentityConfigMutation(signOut(), afterFirstSignIn)).toEqual([signIn(first)]);
      // ↩️ sign-out's inverse with no prior session is a no-op.
      expect(inverseIdentityConfigMutation(signOut(), null)).toEqual([]);
      // ↩️ sign-in's inverse, from no prior session, is a sign-out.
      expect(inverseIdentityConfigMutation(signIn(first), null)).toEqual([signOut()]);
      // ↩️ sign-in's inverse, from a prior session (switching accounts), restores the prior one.
      expect(inverseIdentityConfigMutation(signIn(second), afterFirstSignIn)).toEqual([signIn(first)]);
    });

    it("foldIdentityEvent folds sign-in -> sign-out -> sign-in as last-envelope-wins, ignoring non-remoteMutations events", () => {
      const first = sampleIdentity();
      const second = sampleIdentity({ userId: "u-2" });
      const decodePayload = (payload: unknown): Identity | null | undefined => {
        if (payload === null) return null;
        if (typeof payload === "object" && payload !== null && "userId" in payload) return payload as Identity;
        return undefined;
      };
      const envelope = (payload: unknown): ArtifactEvent => ({
        kind: "remoteMutations",
        envelopes: [{ id: "e", actor: "a", document: IDENTITY_CONFIG_SCHEMA, schemaVersion: IDENTITY_CONFIG_SCHEMA, payloadHash: "", diff: { schemaId: IDENTITY_CONFIG_SCHEMA, payload }, inverse: { targetOperation: "e", inverseDiff: { schemaId: IDENTITY_CONFIG_SCHEMA, payload: null }, baseVersion: 0, undoPolicy: "exactBaseOnly" } }],
      });

      let state: Identity | null = null;
      state = foldIdentityEvent(state, envelope(first), decodePayload);
      expect(state).toEqual(first);
      state = foldIdentityEvent(state, envelope(null), decodePayload);
      expect(state).toBeNull();
      state = foldIdentityEvent(state, envelope(second), decodePayload);
      expect(state).toEqual(second);
      // 🚧️ A non-`remoteMutations` event (e.g. `status`) passes state through unchanged.
      state = foldIdentityEvent(state, { kind: "status", persisted: true, pendingMutations: 0, remote: { kind: "detached" } }, decodePayload);
      expect(state).toEqual(second);
    });
  });
  //#endregion 🔖️IdentityTests

  //#region 🔖️ConfigMutationTests
  describe("config mutation TypeScript parity", () => {
    it("opening mutations replace one coordinate, preserve siblings, and invert exactly", async () => {
      const { applyOpeningConfigMutation, clearDefaultApp, inverseOpeningConfigMutation, setDefaultApp } = await import("./🎚️config/🧬️schema/🧬️mutations/🟦️");
      const dialect = { artifactKind: "s.cad.cad", standard: "1", subset: "*" };
      const viewer = { pluginId: "cad", appId: "viewer" };
      const editor = { pluginId: "cad", appId: "editor" };
      const replacement = { pluginId: "draft", appId: "drafting" };
      const base = { defaults: [{ dialect, role: "viewer" as const, app: viewer }, { dialect, role: "editor" as const, app: editor }] };
      const set = setDefaultApp(dialect, "editor", replacement);
      const afterSet = applyOpeningConfigMutation(base, set);
      expect(afterSet).toEqual({ defaults: [{ dialect, role: "viewer", app: viewer }, { dialect, role: "editor", app: replacement }] });
      expect(inverseOpeningConfigMutation(set, base)).toEqual([setDefaultApp(dialect, "editor", editor)]);
      const clear = clearDefaultApp(dialect, "editor");
      expect(applyOpeningConfigMutation(base, clear)).toEqual({ defaults: [{ dialect, role: "viewer", app: viewer }] });
      expect(inverseOpeningConfigMutation(clear, base)).toEqual([setDefaultApp(dialect, "editor", editor)]);
      expect(inverseOpeningConfigMutation(clearDefaultApp(dialect, "editor"), { defaults: [] })).toEqual([]);
    });

    it("change-merge-policy applies and inverts the prior whole-record setting", async () => {
      const { applyMergePolicyConfigMutation, changeMergePolicy, inverseMergePolicyConfigMutation } = await import("./🎚️config/🧬️schema/🧬️mutations/🟦️");
      const mutation = changeMergePolicy("Vigilant");
      expect(applyMergePolicyConfigMutation({ policy: "Normal" }, mutation)).toEqual({ policy: "Vigilant" });
      expect(inverseMergePolicyConfigMutation(mutation, { policy: "Normal" })).toEqual([changeMergePolicy("Normal")]);
    });

    it("Nx project inputs track every external OS config and plugin-host source compiled by the targets", async () => {
      const { readFile } = await import("node:fs/promises");
      const tsProject = JSON.parse(await readFile(new URL("./📦️packages/🟦️typescript/📋️project.json", import.meta.url), "utf8")) as { namedInputs: { default: string[] } };
      const hostProject = JSON.parse(await readFile(new URL("./🖥️host/📦️packages/🦀️rust/📋️project.json", import.meta.url), "utf8")) as { namedInputs: { default: string[] } };
      expect(tsProject.namedInputs.default).toContain("{workspaceRoot}/🧰️framework/🛍️products/💻️os/🎚️config/**/*");
      expect(hostProject.namedInputs.default).toEqual(expect.arrayContaining([
        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**/*.rs",
        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🎚️config/**/*.rs",
        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🎚️config/**/*.json",
      ]));
    });
  });
  //#endregion 🔖️ConfigMutationTests

  //#region 🔖️DirectoryLaneTests
  describe("backbone-worker space administration", () => {
    const SPACE = "space-admin-01";

    /** ⏳️ Settles the operation by waiting until the worker stops posting, rather than by a fixed
     * number of turns: `parseDirectorySpaceAdministrationPageV1` awaits `crypto.subtle.digest`, whose
     * turn count is not fixed, so a constant loop is load-dependent and therefore flaky. Bounded at 60
     * turns so a genuinely stuck operation still fails the test instead of hanging it. */
    const settleAdministrationTurns = async (harness?: { readonly posted: readonly unknown[] }): Promise<void> => {
      let quiet = 0;
      let seen = harness?.posted.length ?? -1;
      for (let turn = 0; turn < 60 && quiet < 4; turn += 1) {
        await new Promise((resolve) => setTimeout(resolve, 0));
        if (harness === undefined) {
          quiet += 1;
          continue;
        }
        quiet = harness.posted.length === seen ? quiet + 1 : 0;
        seen = harness.posted.length;
      }
    };

    async function sealAdministrationPage(members: readonly { userId: string; email: string; role: "author" | "spectator"; owner: boolean }[], invites: readonly { inviteId: string; createdAtMs: number }[]): Promise<string> {
      const unsigned = {
        access: "author" as const,
        schema: "semio.directory.space-administration-page.v1" as const,
        sessionBindingSha256: "a".repeat(64),
        authorizationGeneration: 5,
        spaceId: SPACE,
        space: { id: SPACE, name: "Administered", kind: "studio", visibility: "private", ownerUserId: "user-a", role: "author", memberCount: members.length, documentCount: 0, activeConnections: 0, createdAtMs: 1, updatedAtMs: 2 },
        members: { rows: members.map((row) => ({ userId: row.userId, email: row.email, displayName: row.userId, role: row.role, owner: row.owner })) },
        documents: { rows: [] as unknown[] },
        invites: { rows: invites.map((row) => ({ inviteId: row.inviteId, role: "spectator" as const, createdAtMs: row.createdAtMs, expiresAtMs: 900000, revoked: false, accepted: false })) },
        capabilities: { renameSpace: true, setVisibility: true, deleteSpace: true, upsertMember: true, removeMember: true, createInvite: true, revokeInvite: true },
      };
      const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(JSON.stringify(unsigned))));
      return JSON.stringify({ ...unsigned, receiptSha256: Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("") });
    }

    async function sealCommandReceipt(requestId: string, command: DirectoryCommand, inviteToken?: string): Promise<string> {
      const commandSha256 = await directorySha256(JSON.stringify(command));
      const unsigned = {
        schema: "semio.directory.command-receipt.v1" as const,
        requestId,
        commandSha256,
        outcome: "accepted" as const,
        events: [] as unknown[],
        result: inviteToken === undefined ? { kind: "none" as const } : { kind: "invite" as const, inviteToken },
      };
      const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(JSON.stringify(unsigned))));
      return JSON.stringify({ ...unsigned, receiptSha256: Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("") });
    }

    async function directorySha256(text: string): Promise<string> {
      const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(text)));
      return Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
    }

    function administrationHarness(pageStatuses: readonly number[], receiptStatus = 200) {
      const posted: BackboneWorkerResponse[] = [];
      const original = workerPostTestSink;
      workerPostTestSink = (message) => posted.push(message);
      const requests: string[] = [];
      let pageIndex = 0;
      const bodies: string[] = [];
      const fetches = vi.fn(async (input: string, init?: { method?: string }) => {
        requests.push(`${init?.method ?? "GET"} ${input}`);
        if ((init?.method ?? "GET") === "POST") {
          return { ok: receiptStatus < 400, status: receiptStatus, statusText: "", headers: { get: () => "application/json" }, json: async () => ({}), text: async () => bodies.shift() ?? "" };
        }
        const status = pageStatuses[Math.min(pageIndex, pageStatuses.length - 1)] ?? 200;
        pageIndex += 1;
        return { ok: status < 400, status, statusText: "", headers: { get: () => "application/json" }, json: async () => ({}), text: async () => bodies.shift() ?? "" };
      });
      return {
        posted,
        requests,
        bodies,
        fetches,
        release: () => { workerPostTestSink = original; },
      };
    }

    it("drives loading → ready → submitting → receipt → refreshing without changing state before the receipt", async () => {
      const harness = administrationHarness([200, 200]);
      const first = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }, { userId: "user-b", email: "b@example.invalid", role: "spectator", owner: false }], [{ inviteId: "invite-1", createdAtMs: 20 }]);
      try {
        directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(first);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 1, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        const loading = harness.posted.filter((message) => message.kind === "directory-administration-state");
        expect(loading.map((message) => (message as { phase: string }).phase)).toEqual(["loading", "ready"]);
        expect((loading.at(-1) as { canonicalJson?: string }).canonicalJson).toBe(first);

        const command: DirectoryCommand = { kind: "remove-member", spaceId: SPACE, userId: "user-b" };
        const receipt = await sealCommandReceipt("0".repeat(31) + "1", command, undefined);
        const second = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], [{ inviteId: "invite-1", createdAtMs: 20 }]);
        harness.bodies.push(receipt, second);
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 1, requestId: "0".repeat(31) + "1", command });
        await settleAdministrationTurns(harness);
        await settleAdministrationTurns(harness);
        const phases = harness.posted.filter((message) => message.kind === "directory-administration-state").map((message) => (message as { phase: string }).phase);
        expect(phases).toEqual(["loading", "ready", "submitting", "receipt", "refreshing", "ready"]);
        const submitting = harness.posted.filter((message) => message.kind === "directory-administration-state").find((message) => (message as { phase: string }).phase === "submitting") as { canonicalJson?: string; receiptSha256?: string };
        expect(submitting.canonicalJson).toBe(first);
        expect(submitting.receiptSha256).toBeUndefined();
        const receiptState = harness.posted.filter((message) => message.kind === "directory-administration-state").find((message) => (message as { phase: string }).phase === "receipt") as { canonicalJson?: string; receiptSha256?: string };
        expect(receiptState.canonicalJson).toBe(first);
        expect(receiptState.receiptSha256).toMatch(/^[0-9a-f]{64}$/u);
        expect(harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).toMatchObject({ phase: "ready", canonicalJson: second });
        expect(harness.requests.filter((entry) => entry.startsWith("POST"))).toHaveLength(1);
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("retains the invite capability until exact clipboard success and rejects duplicate results without redisclosure", async () => {
      const harness = administrationHarness([200, 200]);
      const page = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      try {
        directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(page);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 2, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        const command: DirectoryCommand = { kind: "create-invite", spaceId: SPACE, role: "spectator", ttlSecs: 3600 };
        harness.bodies.push(await sealCommandReceipt("1".repeat(32), command, "invite.v1.secret"), page);
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 2, requestId: "1".repeat(32), command });
        await settleAdministrationTurns(harness);
        await settleAdministrationTurns(harness);
        expect(harness.posted.some((message) => message.kind === "directory-administration-state" && (message as { inviteCapabilityPending?: boolean }).inviteCapabilityPending === true)).toBe(true);
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 2 });
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 2 });
        let capabilities = harness.posted.filter((message) => message.kind === "directory-administration-capability");
        expect(capabilities).toHaveLength(1);
        expect(capabilities[0]).toMatchObject({ operationEpoch: 2, transferEpoch: 1, inviteToken: "invite.v1.secret" });
        expect(harness.posted).toContainEqual({ kind: "directory-administration-capability-rejected", operationEpoch: 2, transferEpoch: 1, code: "capacity" });

        handleTsRequest({ kind: "directory-administration-capability-result", operationEpoch: 2, transferEpoch: 1, copied: false });
        expect(harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).toMatchObject({
          inviteCapabilityPending: true,
          inviteCapabilityStatus: "failed",
        });
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 2 });
        capabilities = harness.posted.filter((message) => message.kind === "directory-administration-capability");
        expect(capabilities).toHaveLength(2);
        expect(capabilities[1]).toMatchObject({ operationEpoch: 2, transferEpoch: 2, inviteToken: "invite.v1.secret" });

        handleTsRequest({ kind: "directory-administration-capability-result", operationEpoch: 2, transferEpoch: 1, copied: true });
        expect(harness.posted).toContainEqual({ kind: "directory-administration-capability-rejected", operationEpoch: 2, transferEpoch: 1, code: "mismatch" });
        handleTsRequest({ kind: "directory-administration-capability-result", operationEpoch: 2, transferEpoch: 2, copied: true });
        handleTsRequest({ kind: "directory-administration-capability-result", operationEpoch: 2, transferEpoch: 2, copied: true });
        expect(harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1)).not.toMatchObject({ inviteCapabilityPending: true });
        expect(harness.posted).toContainEqual({ kind: "directory-administration-capability-rejected", operationEpoch: 2, transferEpoch: 2, code: "already-settled" });
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 2 });
        expect(harness.posted.filter((message) => message.kind === "directory-administration-capability")).toHaveLength(2);
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("clears the pane on 401 and on 403 and never retains a page after the denial", async () => {
      for (const status of [401, 403]) {
        const harness = administrationHarness([status]);
        try {
          directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
          handleTsRequest({ kind: "directory-administration-open", operationEpoch: 3, spaceId: SPACE });
          await settleAdministrationTurns(harness);
          const terminal = harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string; canonicalJson?: string; code?: string };
          expect(terminal.phase).toBe("denied");
          expect(terminal.canonicalJson).toBeUndefined();
          expect(terminal.code).toBe(status === 401 ? "unauthorized" : "forbidden");
        } finally {
          harness.release();
          closeDirectory();
        }
      }
    });

    it("retires the pane on a scoped 4401 for its own space and ignores another space's revocation", async () => {
      const harness = administrationHarness([200, 200]);
      const page = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      try {
        directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(page);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 5, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        expect((harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string }).phase).toBe("ready");

        revokeDirectoryAdministrationForScope("some-other-space");
        expect((harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string }).phase).toBe("ready");

        revokeDirectoryAdministrationForScope(SPACE);
        const terminal = harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string; canonicalJson?: string; code?: string };
        expect(terminal.phase).toBe("denied");
        expect(terminal.code).toBe("forbidden");
        expect(terminal.canonicalJson).toBeUndefined();
      } finally {
        harness.release();
        closeDirectory();
      }
    });

    it("cancels on close and drops every later turn for the retired epoch", async () => {
      const harness = administrationHarness([200]);
      const page = await sealAdministrationPage([{ userId: "user-a", email: "a@example.invalid", role: "author", owner: true }], []);
      try {
        directoryClient = new DirectoryClient("http://hub.test", { request: harness.fetches as never });
        harness.bodies.push(page);
        handleTsRequest({ kind: "directory-administration-open", operationEpoch: 4, spaceId: SPACE });
        await settleAdministrationTurns(harness);
        if (directoryAdministration === null) throw new Error("administration operation missing");
        directoryAdministration.inviteToken = "invite.v1.close-secret";
        directoryAdministration.inviteCapabilityStatus = "available";
        handleTsRequest({ kind: "directory-administration-close", operationEpoch: 4 });
        const terminal = harness.posted.filter((message) => message.kind === "directory-administration-state").at(-1) as { phase: string; canonicalJson?: string };
        expect(terminal.phase).toBe("cancelled");
        expect(terminal.canonicalJson).toBeUndefined();
        const before = harness.requests.length;
        handleTsRequest({ kind: "directory-administration-submit", operationEpoch: 4, requestId: "2".repeat(32), command: { kind: "remove-member", spaceId: SPACE, userId: "user-b" } });
        handleTsRequest({ kind: "directory-administration-refresh", operationEpoch: 4 });
        handleTsRequest({ kind: "directory-administration-capability-request", operationEpoch: 4 });
        await settleAdministrationTurns(harness);
        expect(harness.requests).toHaveLength(before);
        expect(harness.posted.some((message) => message.kind === "directory-administration-capability")).toBe(false);
        expect(terminal).not.toMatchObject({ inviteCapabilityPending: true });
      } finally {
        harness.release();
        closeDirectory();
      }
    });
  });

  describe("backbone-worker directory lane", () => {
    class FakeDirectoryWebSocket {
      static instances: FakeDirectoryWebSocket[] = [];
      readonly url: string;
      readonly protocol = "semio.socket.v1";
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: string }) => void) | null = null;
      onclose: ((event: { code: number }) => void) | null = null;
      onerror: (() => void) | null = null;
      constructor(url: string, readonly protocols?: string | string[]) {
        this.url = url;
        FakeDirectoryWebSocket.instances.push(this);
      }
      send(): void {}
      close(): void {}
      triggerOpen(): void {
        this.onopen?.();
      }
      triggerClose(code: number): void {
        this.onclose?.({ code });
      }
      triggerMessage(message: DirectoryStreamMessage): void {
        this.onmessage?.({ data: JSON.stringify(message) });
      }
    }

    async function flushMicrotasks(): Promise<void> {
      await new Promise((resolve) => setTimeout(resolve, 0));
      await new Promise((resolve) => setTimeout(resolve, 0));
    }

    const commandRequestId = (index: number): string => index.toString(16).padStart(32, "b");
    const sampleCommand: DirectoryCommand = { kind: "create-space", name: "Atelier", spaceKind: "atelier", visibility: "private" };
    const inviteCommand = (spaceId: string): DirectoryCommand => ({ kind: "create-invite", spaceId, role: "spectator", ttlSecs: 3600 });

    async function commandReceiptBody(request: DirectoryCommandRequestV1, outcome: DirectoryCommandOutcomeV1 = "accepted", result: DirectoryCommandResultV1 = { kind: "none" }): Promise<string> {
      return JSON.stringify(await sealDirectoryCommandReceiptV1(request.requestId, await directoryCommandSha256(request.command), outcome, [], result));
    }

    /** ⏳️ Drains enough macrotask turns for every in-flight command turn (fetch, response text, and
     * two SHA-256 digests) to settle, so a transport assertion never races the receipt parser. */
    async function settleDirectoryTransport(): Promise<void> {
      for (let turn = 0; turn < 8; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
    }

    /** 🧪️ Installs a directory client whose transport is a recorded stub, isolating the command
     * transport owner from the browser broker's own proof ratchet (proved separately above). */
    function installFakeDirectoryClient(respond: (request: DirectoryCommandRequestV1) => Promise<{ status: number; body: string }> | { status: number; body: string }): string[] {
      const bodies: string[] = [];
      directoryClient = new DirectoryClient("http://hub.test", {
        request: async (_input: string, init: RequestInit = {}) => {
          const body = String(init.body ?? "");
          bodies.push(body);
          const response = await respond(JSON.parse(body) as DirectoryCommandRequestV1);
          return { ok: response.status >= 200 && response.status < 300, status: response.status, text: async () => response.body } as unknown as FetchTimeoutResponse;
        },
      });
      directorySessionEpoch += 1;
      return bodies;
    }

    it("retries only a transient fault with the byte-identical sealed request and never retries a terminal rejection", async () => {
      let status = 503;
      const bodies = installFakeDirectoryClient(async (request) => (status === 202 ? { status, body: await commandReceiptBody(request) } : { status, body: "hub text the client must never echo" }));
      try {
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(1), command: sampleCommand });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandQueue).toHaveLength(1);
        status = 202;
        await flushDirectoryQueue();
        expect(bodies).toHaveLength(2);
        expect(bodies[0]).toBe(bodies[1]);
        expect(bodies[0]).toBe(JSON.stringify({ schema: "semio.directory.command-request.v1", requestId: commandRequestId(1), command: sampleCommand }));
        expect(directoryCommandQueue).toHaveLength(0);
        expect(directoryCommandOperations.size).toBe(0);

        for (const [index, terminal] of [401, 403, 409, 413].entries()) {
          status = terminal;
          bodies.length = 0;
          handleTsRequest({ kind: "directory-command", requestId: commandRequestId(10 + index), command: sampleCommand });
          await settleDirectoryTransport();
          expect(bodies).toHaveLength(1);
          expect(directoryCommandQueue).toHaveLength(0);
          expect(directoryCommandOperations.size).toBe(0);
        }
      } finally {
        closeDirectory();
      }
    });

    it("bounds the transport at a fixed capacity, keeps the oldest intent, and terminates a malformed correlation", async () => {
      installFakeDirectoryClient(() => {
        throw new Error("network unreachable");
      });
      try {
        for (let index = 0; index < DIRECTORY_COMMAND_TRANSPORT_CAPACITY; index += 1) {
          handleTsRequest({ kind: "directory-command", requestId: commandRequestId(100 + index), command: sampleCommand });
        }
        await settleDirectoryTransport();
        expect(directoryCommandOperations.size).toBe(DIRECTORY_COMMAND_TRANSPORT_CAPACITY);
        expect(directoryCommandQueue).toHaveLength(DIRECTORY_COMMAND_TRANSPORT_CAPACITY);
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(9999), command: sampleCommand });
        await settleDirectoryTransport();
        expect(directoryCommandOperations.size).toBe(DIRECTORY_COMMAND_TRANSPORT_CAPACITY);
        expect(directoryCommandOperations.has(commandRequestId(100))).toBe(true);
        expect(directoryCommandOperations.has(commandRequestId(9999))).toBe(false);
        expect(directoryCommandQueue[0]!.request.requestId).toBe(commandRequestId(100));

        handleTsRequest({ kind: "directory-command", requestId: "r1", command: sampleCommand });
        await settleDirectoryTransport();
        expect(directoryCommandOperations.has("r1")).toBe(false);

        handleTsRequest({ kind: "directory-command-cancel", requestId: commandRequestId(100) });
        expect(directoryCommandOperations.size).toBe(DIRECTORY_COMMAND_TRANSPORT_CAPACITY - 1);
        expect(directoryCommandOperations.has(commandRequestId(100))).toBe(false);
        closeDirectory();
        expect(directoryCommandOperations.size).toBe(0);
        expect(directoryCommandQueue).toHaveLength(0);
      } finally {
        closeDirectory();
      }
    });

    it("stops the queue at a transient head, then drains it in order without a capability reaching any log", async () => {
      const token = "inv.01920000000070008000000000000001.dGhpcy1jYXBhYmlsaXR5LW5ldmVyLXJlYWNoZXMtYS1sb2c";
      const errorSpy = vi.spyOn(console, "error").mockImplementation(() => {});
      let head: "transient" | "accepted" = "transient";
      const bodies = installFakeDirectoryClient(async (request) => {
        if (request.requestId === commandRequestId(200) && head === "transient") return { status: 503, body: "" };
        return { status: 202, body: await commandReceiptBody(request, "accepted", request.command.kind === "create-invite" ? { kind: "invite", inviteToken: token } : { kind: "none" }) };
      });
      try {
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(200), command: inviteCommand("space-a") });
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(201), command: sampleCommand });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandQueue.map((operation) => operation.request.requestId)).toEqual([commandRequestId(200), commandRequestId(201)]);

        head = "accepted";
        await flushDirectoryQueue();
        expect(bodies).toHaveLength(3);
        expect(directoryCommandQueue).toHaveLength(0);
        expect(directoryCommandOperations.size).toBe(0);
        expect(JSON.stringify([...directoryCommandOperations.values(), ...directoryCommandQueue])).not.toContain(token);
        expect(errorSpy.mock.calls.flat().map((entry) => String(entry)).join("|")).not.toContain(token);
      } finally {
        errorSpy.mockRestore();
        closeDirectory();
      }
    });

    it("suppresses delivery for an operation whose session epoch was replaced and settles each operation exactly once", async () => {
      const bodies = installFakeDirectoryClient(async (request) => ({ status: 202, body: await commandReceiptBody(request) }));
      try {
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(300), command: sampleCommand });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandOperations.size).toBe(0);

        const stale: DirectoryCommandTransportOperationV1 = {
          request: sealDirectoryCommandRequestV1(commandRequestId(301), sampleCommand),
          abort: new AbortController(),
          sessionEpoch: directorySessionEpoch - 1,
          workerEpoch: directoryWorkerEpoch,
          settled: false,
        };
        directoryCommandOperations.set(commandRequestId(301), stale);
        directoryCommandQueue.push(stale);
        await flushDirectoryQueue();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandOperations.has(commandRequestId(301))).toBe(false);
        expect(directoryCommandQueue).toHaveLength(0);
        expect(stale.settled).toBe(true);

        settleDirectoryCommand(stale, { kind: "directory-command-failed", requestId: commandRequestId(301), code: "closed" });
        expect(directoryCommandOperations.size).toBe(0);
        expect(directoryCommandQueue).toHaveLength(0);
      } finally {
        closeDirectory();
      }
    });

    it("queues a directory command while the hub is unreachable, then flushes it in order on the next live signal", async () => {
      FakeDirectoryWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      let reachable = false;
      try {
        handleTsRequest({ kind: "directory-open", baseUrl: "http://hub.test", since: 0 });
        const bodies = installFakeDirectoryClient(async (request) => {
          if (!reachable) throw new Error("network unreachable");
          return { status: 202, body: await commandReceiptBody(request) };
        });
        handleTsRequest({ kind: "directory-command", requestId: commandRequestId(1), command: sampleCommand });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(1);
        expect(directoryCommandQueue).toHaveLength(1);
        expect(directoryCommandQueue[0]!.request.requestId).toBe(commandRequestId(1));

        // 🟢️ Hub becomes reachable — any live signal on the stream (a heartbeat here) triggers a flush.
        reachable = true;
        FakeDirectoryWebSocket.instances.at(-1)!.triggerMessage({ kind: "heartbeat", headSeq: 0 });
        await settleDirectoryTransport();
        expect(bodies).toHaveLength(2);
        expect(directoryCommandQueue).toHaveLength(0);
        expect(directoryCommandOperations.size).toBe(0);
      } finally {
        // 🧹️ Restores the real global — an un-restored `FakeDirectoryWebSocket` (no `OPEN` static)
        // previously leaked into every later test's `WebSocket.OPEN` comparisons, silently making
        // `relayMutationsToHub`/`sendWireFrame`'s "is the socket actually open" checks pass when
        // `state.socket` was `null`.
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        closeDirectory();
      }
    });

    it("backbone worker owns one full scoped stream and retires it terminally on 4401", async () => {
      vi.useFakeTimers();
      FakeDirectoryWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      const paths: string[] = [];
      socketGrantTestIssue = async (_baseUrl, path) => {
        paths.push(path);
        return {
          schema: "semio.hub.socket-grant/v1",
          protocol: "semio.socket.v1",
          grant: `socket.v1.${"1".repeat(32)}.${"2".repeat(64)}`,
          actorId: `hub.v1.${"3".repeat(64)}`,
          expiresAtMs: Number.MAX_SAFE_INTEGER,
        };
      };
      const scope = { spaceId: "space/a", documentId: "document b" };
      try {
        handleTsRequest({ kind: "directory-scope-open", baseUrl: "http://hub.test", scope, since: 7 });
        for (let turn = 0; turn < 16 && FakeDirectoryWebSocket.instances.length === 0; turn += 1) await Promise.resolve();
        const socket = FakeDirectoryWebSocket.instances[0]!;
        expect(paths).toEqual(["/directory/spaces/space%2Fa/documents/document%20b/socket-grants"]);
        expect(socket.url).toBe("ws://hub.test/directory/spaces/space%2Fa/documents/document%20b/socket/v1?since=7");
        socket.triggerOpen();
        socket.triggerClose(4401);
        await Promise.resolve();
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MAX_MS * 2);
        expect(scopedDirectoryStreams.size).toBe(0);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
      } finally {
        closeDirectory();
        socketGrantTestIssue = null;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        vi.useRealTimers();
      }
    });
  });
  //#endregion 🔖️DirectoryLaneTests

  //#region 💡️InferencePortTests
  // 💡️ Slice D — the host-owned ephemeral GIS Map inference port, driven through a FAKE, already
  // authenticated transport (an installed browser-broker proof plus a stubbed `fetch`). These are
  // the port's own laws: it refuses to start without a verified live execution-target lease, it
  // publishes no state before an exact server receipt, a Cancel click is never an optimistic
  // terminal, `stale` and `failed` are hard, and no document mutation, outbox entry or artifact
  // event is ever produced.
  describe("gis map inference port", () => {
    const SPACE = "sp-inference";
    const DOCUMENT = "doc-inference";
    const JOB = "1".repeat(32);
    const HASH = "9071779b724c67e0a45d5e23fddc8dbeb3d9b537936a4a14c293bc373960b130";
    const PREVIEW = { schema: "semio.hub.gis-map-inference-preview/v1" as const, jobId: JOB, proposalHash: HASH, regionId: `inference-${JOB}`, ring: [[7, 46], [9, 46], [9, 48], [7, 48], [7, 46]] };

    function leaseFields(write: boolean): DocumentExecutionTargetLeaseFieldsV1 {
      return { scope: { spaceId: SPACE, documentId: DOCUMENT }, grant: { read: true, write, observe: true } } as unknown as DocumentExecutionTargetLeaseFieldsV1;
    }

    function inferenceHarness(options: { readonly lease: "none" | "viewer" | "editor" }) {
      const posted: BackboneWorkerResponse[] = [];
      const original = workerPostTestSink;
      workerPostTestSink = (message) => posted.push(message);
      const originalFetch = globalThis.fetch;
      const requests: string[] = [];
      const bodies: string[] = [];
      const statuses: number[] = [];
      (globalThis as unknown as { fetch: unknown }).fetch = async (input: string, init?: { method?: string }) => {
        requests.push(`${init?.method ?? "GET"} ${input}`);
        const status = statuses.shift() ?? 200;
        const body = bodies.shift() ?? "";
        return { ok: status < 400, status, statusText: "", headers: new Headers({ "content-type": "application/json", "content-length": String(new TextEncoder().encode(body).length), "x-semio-browser-broker-advanced": "1" }), text: async () => body, json: async () => JSON.parse(body) };
      };
      clearLocalBrowserBrokerProof();
      installLocalBrowserBrokerProof("b".repeat(64));
      // 🧷️ No socket-grant issuer and no installed target means `openArtifact` opens no hub socket
      // at all, so this harness exercises the inference port's own four calls and nothing else.
      const originalIssue = socketGrantTestIssue;
      socketGrantTestIssue = null;
      openArtifact({ documentId: DOCUMENT, schema: "gis.map", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: SPACE }], actor: "caller" });
      const state = artifactState(DOCUMENT, SPACE)!;
      if (options.lease !== "none") state.executionTargetLease = new DocumentExecutionTargetLease(documentExecutionTargetLeaseMintToken, leaseFields(options.lease === "editor"), "http://hub.test", new Uint8Array(1), new Uint8Array(1));
      return {
        posted,
        requests,
        bodies,
        statuses,
        state,
        ports: () => posted.filter((message) => message.kind === "inference-port-status").map((message) => (message as Extract<BackboneWorkerResponse, { kind: "inference-port-status" }>).status),
        release: () => {
          closeArtifact(DOCUMENT, SPACE);
          clearLocalBrowserBrokerProof();
          workerPostTestSink = original;
          socketGrantTestIssue = originalIssue;
          (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        },
      };
    }

    const receiptBody = (state: string, proposalState: string, cursor: number, proposalHash?: string): string =>
      JSON.stringify({ schema: "semio.hub.inference-receipt/v1", jobId: JOB, state, proposalState, ...(proposalHash === undefined ? {} : { proposalHash }), cursor, expiresAtMs: 1_700_000_060_000 });

    const pageBody = (state: string, proposalState: string, options: { readonly nextCursor: number; readonly completed: number; readonly total: number; readonly proposalHash?: string; readonly preview?: GisMapInferencePreviewV1; readonly cancelRequested?: boolean; readonly stale?: boolean }): string =>
      JSON.stringify({
        schema: "semio.hub.inference-events/v1",
        jobId: JOB,
        state,
        proposalState,
        cancelRequested: options.cancelRequested ?? false,
        stale: options.stale ?? false,
        ...(options.proposalHash === undefined ? {} : { proposalHash: options.proposalHash }),
        ...(options.preview === undefined ? {} : { preview: options.preview }),
        events: [],
        progress: [{ cursor: options.nextCursor, runEpoch: 1, completed: options.completed, total: options.total, atMs: 1_700_000_001_000 }],
        nextCursor: options.nextCursor,
      });

    it("refuses to start at all without a verified live execution-target lease", async () => {
      const harness = inferenceHarness({ lease: "none" });
      try {
        handleTsRequest({ kind: "inference-open", operationEpoch: 1, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        await Promise.resolve();
        expect(harness.ports()).toEqual([{ phase: "failed", jobId: null, cursor: 0, completed: 0, total: 0, proposalHash: null, cancelRequested: false, code: "inference.lease-unverified" }]);
        expect(harness.requests).toEqual([]);
        handleTsRequest({ kind: "inference-propose", operationEpoch: 1, requestId: "2".repeat(32) });
        await Promise.resolve();
        expect(harness.requests).toEqual([]);
      } finally {
        harness.release();
      }
    });

    it("refuses a verified viewer-only lease, because a proposal it could never approve must not start", async () => {
      const harness = inferenceHarness({ lease: "viewer" });
      try {
        handleTsRequest({ kind: "inference-open", operationEpoch: 2, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        await Promise.resolve();
        expect(harness.ports().at(-1)?.code).toBe("inference.lease-unverified");
        expect(harness.requests).toEqual([]);
      } finally {
        harness.release();
      }
    });

    it("publishes no phase beyond submitting until an exact server receipt lands, and never mutates the document", async () => {
      const harness = inferenceHarness({ lease: "editor" });
      try {
        handleTsRequest({ kind: "inference-open", operationEpoch: 3, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        harness.statuses.push(200);
        harness.bodies.push(receiptBody("accepted", "none", 0));
        handleTsRequest({ kind: "inference-propose", operationEpoch: 3, requestId: "3".repeat(32) });
        expect(harness.ports().map((status) => status.phase)).toEqual(["idle", "submitting"]);
        await settleInferenceTurns();
        expect(harness.ports().map((status) => status.phase)).toEqual(["idle", "submitting", "running"]);
        expect(harness.ports().at(-1)?.jobId).toBe(JOB);
        expect(harness.requests).toEqual([`POST /_semio/hub/spaces/${SPACE}/documents/${DOCUMENT}/inference/gis-map/jobs`]);
        expect(harness.state.outbox).toEqual([]);
        expect(harness.state.pendingMutations).toEqual([]);
        expect(harness.posted.some((message) => message.kind === "event" && "event" in message && (message as { event: { kind: string } }).event.kind === "remoteMutations")).toBe(false);
      } finally {
        harness.release();
      }
    });

    it("records a Cancel click as requested and reaches cancelled only on the server's own answer", async () => {
      const harness = inferenceHarness({ lease: "editor" });
      try {
        handleTsRequest({ kind: "inference-open", operationEpoch: 4, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        harness.statuses.push(200);
        harness.bodies.push(receiptBody("accepted", "none", 0));
        handleTsRequest({ kind: "inference-propose", operationEpoch: 4, requestId: "4".repeat(32) });
        await settleInferenceTurns();
        harness.statuses.push(200);
        harness.bodies.push(pageBody("cancelled", "cancelled", { nextCursor: 1, completed: 1, total: 4, cancelRequested: true }));
        handleTsRequest({ kind: "inference-cancel", operationEpoch: 4 });
        const requested = harness.ports().at(-1)!;
        expect(requested.cancelRequested).toBe(true);
        expect(requested.phase).toBe("running");
        // 🛑️ A second Cancel click never sends a second cancel request.
        handleTsRequest({ kind: "inference-cancel", operationEpoch: 4 });
        await settleInferenceTurns();
        expect(harness.ports().at(-1)?.phase).toBe("cancelled");
        expect(harness.requests.filter((entry) => entry.endsWith("/cancel"))).toEqual([`POST /_semio/hub/spaces/${SPACE}/documents/${DOCUMENT}/inference/gis-map/jobs/${JOB}/cancel`]);
      } finally {
        harness.release();
      }
    });

    it("reports a stale base as a hard terminal that no later answer can move", async () => {
      const harness = inferenceHarness({ lease: "editor" });
      try {
        handleTsRequest({ kind: "inference-open", operationEpoch: 5, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        harness.statuses.push(200);
        harness.bodies.push(receiptBody("accepted", "none", 0));
        handleTsRequest({ kind: "inference-propose", operationEpoch: 5, requestId: "5".repeat(32) });
        await settleInferenceTurns();
        harness.statuses.push(200);
        harness.bodies.push(pageBody("running", "none", { nextCursor: 1, completed: 1, total: 4, stale: true }));
        handleTsRequest({ kind: "inference-cancel", operationEpoch: 5 });
        await settleInferenceTurns();
        expect(harness.ports().at(-1)?.phase).toBe("stale");
        const before = harness.ports().length;
        handleTsRequest({ kind: "inference-approve", operationEpoch: 5 });
        await settleInferenceTurns();
        expect(harness.ports().length).toBe(before);
      } finally {
        harness.release();
      }
    });

    it("approves exactly the offered hash once and applies only on a committed receipt", async () => {
      const harness = inferenceHarness({ lease: "editor" });
      try {
        handleTsRequest({ kind: "inference-open", operationEpoch: 6, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        harness.statuses.push(200);
        harness.bodies.push(receiptBody("succeeded", "offered", 1, HASH));
        handleTsRequest({ kind: "inference-propose", operationEpoch: 6, requestId: "6".repeat(32) });
        await settleInferenceTurns();
        expect(harness.ports().at(-1)?.phase).toBe("offered");
        handleTsRequest({ kind: "inference-approve", operationEpoch: 6 });
        expect(harness.requests.filter((entry) => entry.endsWith("/approval"))).toEqual([]);
        harness.statuses.push(200);
        harness.bodies.push(pageBody("succeeded", "offered", { nextCursor: 2, completed: 4, total: 4, proposalHash: HASH, preview: PREVIEW }));
        await driveInferencePort(6);
        expect(harness.ports().at(-1)?.preview).toEqual(PREVIEW);
        harness.statuses.push(200);
        harness.bodies.push(JSON.stringify({ schema: "semio.hub.inference-approval-receipt/v1", jobId: JOB, mutationId: HASH, commandHash: HASH, proposalHash: HASH, applied: true }));
        handleTsRequest({ kind: "inference-approve", operationEpoch: 6 });
        expect(harness.ports().at(-1)?.phase).toBe("approving");
        await settleInferenceTurns();
        expect(harness.ports().at(-1)?.phase).toBe("applied");
        const approvalRequests = harness.requests.filter((entry) => entry.endsWith("/approval"));
        expect(approvalRequests).toHaveLength(1);
        expect(harness.state.outbox).toEqual([]);
      } finally {
        harness.release();
      }
    });

    it("terminates on valid-looking cross-job or cross-hash previews without sending approval", async () => {
      const substituted = [
        { ...PREVIEW, jobId: "8".repeat(32), regionId: `inference-${"8".repeat(32)}` },
        { ...PREVIEW, proposalHash: "9".repeat(64) },
      ];
      for (const [index, preview] of substituted.entries()) {
        const operationEpoch = 8 + index;
        const harness = inferenceHarness({ lease: "editor" });
        try {
          handleTsRequest({ kind: "inference-open", operationEpoch, scope: { spaceId: SPACE, documentId: DOCUMENT } });
          harness.statuses.push(200);
          harness.bodies.push(receiptBody("accepted", "none", 0));
          handleTsRequest({ kind: "inference-propose", operationEpoch, requestId: `${operationEpoch}`.repeat(32) });
          await settleInferenceTurns();
          harness.statuses.push(200);
          harness.bodies.push(pageBody("succeeded", "offered", { nextCursor: 1, completed: 4, total: 4, proposalHash: HASH, preview }));
          await driveInferencePort(operationEpoch);
          expect(harness.ports().at(-1)).toMatchObject({ phase: "failed", code: "inference.transport" });
          handleTsRequest({ kind: "inference-approve", operationEpoch });
          await settleInferenceTurns();
          expect(harness.requests.filter((entry) => entry.endsWith("/approval"))).toEqual([]);
        } finally {
          harness.release();
        }
      }
    });

    it("maps a published route rejection onto the closed failure vocabulary", async () => {
      const harness = inferenceHarness({ lease: "editor" });
      try {
        handleTsRequest({ kind: "inference-open", operationEpoch: 7, scope: { spaceId: SPACE, documentId: DOCUMENT } });
        harness.statuses.push(503);
        harness.bodies.push(JSON.stringify({ schema: "semio.hub.inference-error/v1", code: "inference.unavailable" }));
        handleTsRequest({ kind: "inference-propose", operationEpoch: 7, requestId: "7".repeat(32) });
        await settleInferenceTurns();
        expect(harness.ports().at(-1)).toMatchObject({ phase: "failed", code: "inference.unavailable" });
      } finally {
        harness.release();
      }
    });

    it("keeps explicit English and German text for every phase, code and control with no default language", async () => {
      const { GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1, GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1, GIS_MAP_INFERENCE_PORT_TEXT_V1, gisMapInferencePortRoleV1 } = await import("./🔨️modules/📇️directory/🧬️schema/🟦️.ts");
      const phases: readonly GisMapInferencePortStatusV1["phase"][] = ["idle", "submitting", "running", "offered", "approving", "applied", "cancelled", "stale", "failed"];
      for (const phase of phases) {
        const row = GIS_MAP_INFERENCE_PORT_TEXT_V1[phase];
        expect(Object.keys(row).sort()).toEqual(["de", "en"]);
        expect(row.en.length).toBeGreaterThan(0);
        expect(row.de.length).toBeGreaterThan(0);
        expect(row.de).not.toBe(row.en);
      }
      for (const [code, row] of Object.entries(GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1)) {
        expect(Object.keys(row).sort(), code).toEqual(["de", "en"]);
        expect(row.de, code).not.toBe(row.en);
      }
      for (const [control, row] of Object.entries(GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1)) {
        expect(Object.keys(row).sort(), control).toEqual(["de", "en"]);
        expect(row.de, control).not.toBe(row.en);
      }
      expect(gisMapInferencePortRoleV1("running")).toBe("status");
      expect(gisMapInferencePortRoleV1("failed")).toBe("alert");
    });
  });

  /** ⏳️ Settles every turn one bounded request/response leg needs, then four quiet ones. */
  async function settleInferenceTurns(): Promise<void> {
    for (let turn = 0; turn < 40; turn += 1) await new Promise((resolve) => setTimeout(resolve, 0));
  }
  //#endregion 💡️InferencePortTests

  //#region 🔖️OfflineResilienceTests
  // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (web-backbone) findings 1/2/3/5: SSE-primary folder
  // watch with a suppressed sanity-poll fallback, reconnect after a post-open drop, abort-on-close,
  // and the bounded lossless mutation outbox. No real sleeps — `vi.useFakeTimers()` drives every
  // timer-dependent assertion, and every fetch/socket/stream is a controllable local fake.
  describe("backbone-worker offline resilience", () => {
    class FakeEventSource {
      static instances: FakeEventSource[] = [];
      readonly url: string;
      onopen: (() => void) | null = null;
      onmessage: ((event: unknown) => void) | null = null;
      onerror: (() => void) | null = null;
      closed = false;
      constructor(url: string) {
        this.url = url;
        FakeEventSource.instances.push(this);
      }
      close(): void {
        this.closed = true;
      }
    }

    class FakeHubWebSocket {
      static readonly CONNECTING = 0;
      static readonly OPEN = 1;
      static readonly CLOSING = 2;
      static readonly CLOSED = 3;
      static instances: FakeHubWebSocket[] = [];
      readonly url: string;
      readonly protocol = "semio.socket.v1";
      readonly protocols: string | string[] | undefined;
      readyState = FakeHubWebSocket.CONNECTING;
      binaryType = "blob";
      readonly sent: Uint8Array[] = [];
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
      onclose: (() => void) | null = null;
      onerror: (() => void) | null = null;
      constructor(url: string, protocols?: string | string[]) {
        this.url = url;
        this.protocols = protocols;
        FakeHubWebSocket.instances.push(this);
      }
      send(data: Uint8Array): void {
        this.sent.push(data);
      }
      open(): void {
        this.readyState = FakeHubWebSocket.OPEN;
        this.onopen?.();
      }
      close(_code?: number, _reason?: string): void {
        this.readyState = FakeHubWebSocket.CLOSED;
        this.onclose?.();
      }
    }

    function folderOnlyConfig(documentId: string): ArtifactActorConfig {
      return { documentId, schema: "demo/v1", bindings: [{ kind: "folder", path: `/tmp/${documentId}` }], actor: "actor-1" };
    }

    type BrowserDocumentOpenFixture = {
      nowMs: number;
      intent: DocumentOpenIntentV1;
      installedTarget: NonNullable<Extract<PersistenceBinding, { kind: "hub" }>["installedTarget"]>;
      plan: DocumentOpenPlanV1;
      socketGrant: SocketGrantReceiptV1;
      expected: { httpPaths: [string, string]; webSocketPath: string; protocol: string; helloSchema: string; helloPackSchemaHashByte: number; responseMaxBytes: number; rustWorkerBypassDenied: true; scopeIsolation: { left: { spaceId: string; documentId: string }; right: { spaceId: string; documentId: string }; leftKey: string; rightKey: string; localKey: string }; forbiddenSocketFragments: string[] };
    };

    async function browserDocumentOpenFixture(): Promise<BrowserDocumentOpenFixture> {
      const { readFile } = await import("node:fs/promises");
      return JSON.parse(await readFile(new URL("./🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json", import.meta.url), "utf8")) as BrowserDocumentOpenFixture;
    }

    function currentBrowserDocumentOpenFixture(fixture: BrowserDocumentOpenFixture): { plan: DocumentOpenPlanV1; grant: SocketGrantReceiptV1 } {
      const now = Date.now();
      return {
        plan: { ...structuredClone(fixture.plan), expiresAtUnixMs: now + 30_000 },
        grant: { ...fixture.socketGrant, expiresAtMs: now + 25_000 },
      };
    }

    async function waitForDocumentSocket(): Promise<FakeHubWebSocket> {
      for (let turn = 0; turn < 64; turn += 1) {
        const socket = FakeHubWebSocket.instances.at(-1);
        if (socket) return socket;
        await new Promise<void>((resolve) => setTimeout(resolve, 0));
      }
      throw new Error("browser document-open socket deadline exceeded");
    }

    function notFoundResponse() {
      return { ok: false, status: 404, statusText: "not found", headers: { get: () => null }, json: async () => ({}), text: async () => "" };
    }

    it("browser document open remains D1-owned when the Rust worker resolves", async () => {
      const fixture = await browserDocumentOpenFixture();
      const documentId = `${fixture.intent.scope.documentId}-resolved-rust`;
      const typescriptRequests: BackboneWorkerRequest[] = [];
      const rustRequests: BackboneWorkerRequest[] = [];
      const host: RustWorkerHost = {
        handleRequestBytes: (bytes) => rustRequests.push(decodeBackboneWorkerRequest(bytes)),
        postReady: () => {},
      };
      const dispatch = (request: BackboneWorkerRequest): void => dispatchBackboneWorkerRequest(request, host, (value) => typescriptRequests.push(value));
      dispatch({ kind: "open", documentId, schema: fixture.plan.artifact.schema, bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget }], actor: "caller-selected-actor" });
      dispatch({ kind: "send", documentId, spaceId: fixture.intent.scope.spaceId, message: { kind: "detach" } });
      dispatch({ kind: "close", documentId, spaceId: fixture.intent.scope.spaceId });
      expect(typescriptRequests.map(({ kind }) => kind)).toEqual(["open", "send", "close"]);
      expect(rustRequests).toHaveLength(0);
    });

    it("browser document open runtime ownership isolates the same document id across two hub spaces", async () => {
      const fixture = await browserDocumentOpenFixture();
      const originalWebSocket = globalThis.WebSocket;
      const current = currentBrowserDocumentOpenFixture(fixture);
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      socketGrantTestIssue = async () => current.grant;
      const left = fixture.expected.scopeIsolation.left;
      const right = fixture.expected.scopeIsolation.right;
      try {
        openArtifact({ documentId: left.documentId, schema: fixture.installedTarget.artifact.schema, bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: left.spaceId, installedTarget: fixture.installedTarget }], actor: "caller-selected-actor" });
        openArtifact({ documentId: right.documentId, schema: fixture.installedTarget.artifact.schema, bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: right.spaceId, installedTarget: fixture.installedTarget }], actor: "caller-selected-actor" });
        expect(documentRuntimeKeyV1({ kind: "hub", ...left })).toBe(fixture.expected.scopeIsolation.leftKey);
        expect(documentRuntimeKeyV1({ kind: "hub", ...right })).toBe(fixture.expected.scopeIsolation.rightKey);
        expect(documentRuntimeKeyV1({ kind: "local", documentId: left.documentId })).toBe(fixture.expected.scopeIsolation.localKey);
        expect(fixture.expected.scopeIsolation.leftKey).not.toBe(fixture.expected.scopeIsolation.rightKey);
        expect(fixture.expected.scopeIsolation.localKey).not.toBe(fixture.expected.scopeIsolation.leftKey);
        expect(artifacts.has(fixture.expected.scopeIsolation.leftKey)).toBe(true);
        expect(artifacts.has(fixture.expected.scopeIsolation.rightKey)).toBe(true);
        expect(artifacts.get(fixture.expected.scopeIsolation.leftKey)!.channel.name).toBe(`semio-doc-${fixture.expected.scopeIsolation.leftKey}`);
        expect(artifacts.get(fixture.expected.scopeIsolation.rightKey)!.channel.name).toBe(`semio-doc-${fixture.expected.scopeIsolation.rightKey}`);
        expect(artifactState(left.documentId)).toBeUndefined();
        closeArtifact(left.documentId, left.spaceId);
        expect(artifacts.has(fixture.expected.scopeIsolation.leftKey)).toBe(false);
        expect(artifacts.has(fixture.expected.scopeIsolation.rightKey)).toBe(true);
      } finally {
        closeArtifact(left.documentId, left.spaceId);
        closeArtifact(right.documentId, right.spaceId);
        socketGrantTestIssue = null;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
      }
    });

    it("browser document open requires exact installed package artifact and surface authority", async () => {
      const fixture = await browserDocumentOpenFixture();
      const config: ArtifactActorConfig = {
        documentId: fixture.intent.scope.documentId,
        schema: fixture.installedTarget.artifact.schema,
        bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget }],
        actor: "caller-selected-actor",
        packSchemaHash: new Array(32).fill(fixture.expected.helloPackSchemaHashByte),
      };
      expect(documentOpenPlanAuthority(fixture.plan, fixture.intent, config, fixture.installedTarget).surfaceId).toBe(fixture.installedTarget.surface.surfaceId);
      const replacements: readonly [string, string, unknown][] = [
        ["package", "pluginId", "s.foreign"],
        ["package", "packageId", "s.foreign.codec"],
        ["package", "version", "9.9.9"],
        ["package", "componentSha256", "a".repeat(64)],
        ["package", "componentBlake3", "a".repeat(64)],
        ["package", "descriptorByteSha256", "a".repeat(64)],
        ["artifact", "kind", "s.foreign"],
        ["artifact", "packSchemaHash", "a".repeat(64)],
        ["parentDialect", "artifactKind", "s.foreign"],
        ["parentDialect", "standard", "2"],
        ["parentDialect", "subset", "preview"],
        ["surface", "appId", "app.foreign"],
        ["surface", "windowKindId", "window.foreign"],
        ["surface", "role", "viewer"],
        ["surface", "rendererTarget", "wgpu"],
      ];
      for (const [section, field, value] of replacements) {
        const candidate = structuredClone(fixture.plan) as unknown as Record<string, Record<string, unknown>>;
        candidate[section]![field] = value;
        expect(() => documentOpenPlanAuthority(candidate as unknown as DocumentOpenPlanV1, fixture.intent, config, fixture.installedTarget)).toThrow("document open: authority mismatch");
      }
    });

    it("browser document open uses the authenticated D1 plan and receipt exchange before its credential-free socket", async () => {
      const fixture = await browserDocumentOpenFixture();
      const current = currentBrowserDocumentOpenFixture(fixture);
      const originalFetch = globalThis.fetch;
      const originalWebSocket = globalThis.WebSocket;
      const requests: { url: string; headers: Headers; body: string }[] = [];
      FakeHubWebSocket.instances = [];
      socketGrantTestIssue = null;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      (globalThis as unknown as { fetch: unknown }).fetch = async (input: string, init?: RequestInit) => {
        const url = String(input);
        const body = String(init?.body ?? "");
        requests.push({ url, headers: new Headers(init?.headers), body });
        const response = requests.length === 1 ? current.plan : current.grant;
        return Response.json(response, { headers: { "x-semio-browser-broker-advanced": "1" } });
      };
      try {
        openArtifact({ documentId: fixture.intent.scope.documentId, schema: fixture.plan.artifact.schema, bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget }], actor: "caller-selected-actor" });
        const socket = await waitForDocumentSocket();
        expect(requests).toHaveLength(2);
        expect(requests.map(({ url }) => url)).toEqual(fixture.expected.httpPaths.map((path) => `/_semio/hub${path}`));
        const intent = JSON.parse(requests[0]!.body) as Record<string, unknown>;
        expect(Object.keys(intent).sort()).toEqual(["clientInstanceId", "requestedSurfaceId", "schema", "scope", "version"]);
        expect(intent.scope).toEqual(fixture.intent.scope);
        expect(intent.requestedSurfaceId).toBe(fixture.intent.requestedSurfaceId);
        expect(intent.clientInstanceId).toMatch(/^[0-9a-f-]{36}$/u);
        expect(requests[0]!.body).not.toContain(current.plan.receipt);
        expect(JSON.parse(requests[1]!.body)).toEqual({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: current.plan.receipt });
        expect(requests[0]!.headers.get("x-semio-browser-broker")).not.toBe(requests[1]!.headers.get("x-semio-browser-broker"));
        expect(requests.every(({ headers }) => /^[0-9a-f]{64}$/u.test(headers.get("x-semio-browser-broker") ?? "") && /^[0-9a-f]{64}$/u.test(headers.get("x-semio-browser-broker-next") ?? ""))).toBe(true);
        expect(socket.url).toBe(`ws://hub.test${fixture.expected.webSocketPath}`);
        expect(socket.protocols).toEqual([fixture.expected.protocol, current.grant.grant]);
        for (const forbidden of fixture.expected.forbiddenSocketFragments) expect(socket.url).not.toContain(forbidden);
        socket.open();
        expect(socket.sent).toHaveLength(1);
        const hello = decodeClientFrame(socket.sent[0]!).frame;
        if (typeof hello === "string" || !("SocketHelloV1" in hello)) throw new Error("expected SocketHelloV1");
        expect(hello.SocketHelloV1.schema).toBe(fixture.expected.helloSchema);
        expect(hello.SocketHelloV1.pack_schema_hash).toEqual(new Array(32).fill(fixture.expected.helloPackSchemaHashByte));
        expect(JSON.stringify(hello)).not.toContain("open.v1.");
        expect(JSON.stringify(hello)).not.toContain("socket.v1.");
        const state = artifactState(fixture.intent.scope.documentId, fixture.intent.scope.spaceId)!;
        expect(state.hubActorReady).toBe(false);
        expect(state.actor).toBe("");
        expect(state.pendingSocketActorId).toBe(current.grant.actorId);
        await handleHubFrame(state, { Session: { actor: current.grant.actorId, color: 7 } });
        expect(state.hubActorReady).toBe(true);
        expect(state.actor).toBe(current.grant.actorId);
        expect(state.pendingSocketActorId).toBeNull();
      } finally {
        closeArtifact(fixture.intent.scope.documentId, fixture.intent.scope.spaceId);
        clearLocalBrowserBrokerProof();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
      }
    });

    it("browser document open withholds activation until an exact authenticated Session and terminally clears a mismatched actor", async () => {
      const fixture = await browserDocumentOpenFixture();
      const current = currentBrowserDocumentOpenFixture(fixture);
      const originalFetch = globalThis.fetch;
      const originalWebSocket = globalThis.WebSocket;
      FakeHubWebSocket.instances = [];
      socketGrantTestIssue = null;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      let effects = 0;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => {
        effects += 1;
        return Response.json(effects === 1 ? current.plan : current.grant, { headers: { "x-semio-browser-broker-advanced": "1" } });
      };
      try {
        openArtifact({ documentId: fixture.intent.scope.documentId, schema: fixture.plan.artifact.schema, bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget }], actor: "caller-selected-actor" });
        const socket = await waitForDocumentSocket();
        socket.open();
        const state = artifactState(fixture.intent.scope.documentId, fixture.intent.scope.spaceId)!;
        expect(state.hubActorReady).toBe(false);
        expect(state.actor).toBe("");
        expect(state.pendingSocketActorId).toBe(current.grant.actorId);
        await handleHubFrame(state, { Session: { actor: `hub.v1.${"f".repeat(64)}`, color: 9 } });
        expect(socket.readyState).toBe(FakeHubWebSocket.CLOSED);
        expect(state.hubActorReady).toBe(false);
        expect(state.actor).toBe("");
        expect(state.pendingSocketActorId).toBeNull();
      } finally {
        closeArtifact(fixture.intent.scope.documentId, fixture.intent.scope.spaceId);
        clearLocalBrowserBrokerProof();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
      }
    });

    type ExecutionTargetLeaseFixture = {
      nowMs: number;
      hubOrigin: string;
      intent: DocumentOpenIntentV1;
      plan: DocumentOpenPlanV1;
      manifest: DocumentExecutionTargetLeaseFieldsV1;
      socketGrant: SocketGrantReceiptV1;
      componentHex: string;
      descriptorHex: string;
      expected: {
        assetPaths: [string, string, string];
        openPlanPath: string;
        socketGrantPath: string;
        componentMaxBytes: number;
        descriptorMaxBytes: number;
        manifestMaxBytes: number;
        progressUnitBytes: number;
        progressStages: string[];
        rendererState: string;
        rendererClaims: Record<string, boolean>;
        viewerWriteRejectedLocally: true;
        forbiddenStatusFragments: string[];
        status: Record<string, { en: string; de: string }>;
        statusRoles: Record<string, "status" | "alert">;
        rotation: { generationA: string; generationB: string };
      };
      hostile: { name: string; stage: string; kind: string; path?: string; value?: unknown; expected: "unpublished" }[];
    };

    async function executionTargetLeaseFixture(): Promise<ExecutionTargetLeaseFixture> {
      const { readFile } = await import("node:fs/promises");
      return JSON.parse(await readFile(new URL("../../../🌎️hub/🧪️fixtures/📇️directory/🔏️document-execution-target-lease-v1/🔣️.json", import.meta.url), "utf8")) as ExecutionTargetLeaseFixture;
    }

    function executionTargetBytes(hexText: string): Uint8Array {
      return Uint8Array.from({ length: hexText.length / 2 }, (_unused, index) => Number.parseInt(hexText.slice(index * 2, index * 2 + 2), 16));
    }

    function executionTargetMutate(source: unknown, path: string, value: unknown): Record<string, unknown> {
      const candidate = structuredClone(source) as Record<string, unknown>;
      const segments = path.split(".");
      let cursor = candidate as Record<string, unknown>;
      for (const segment of segments.slice(0, -1)) cursor = cursor[segment] as Record<string, unknown>;
      cursor[segments.at(-1)!] = value;
      return candidate;
    }

    function executionTargetBodyResponse(bytes: Uint8Array, declaredLength = bytes.byteLength): Response {
      return new Response(bytes, { headers: { "content-length": String(declaredLength), "x-semio-browser-broker-advanced": "1" } });
    }

    type ExecutionTargetHarness = {
      state: ArtifactState;
      binding: Extract<PersistenceBinding, { kind: "hub" }>;
      requests: { url: string; method: string; body: string }[];
      statuses: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>[];
      release: () => void;
    };

    function executionTargetHarness(fixture: ExecutionTargetLeaseFixture, respond: (url: string, requests: { url: string; method: string; body: string }[]) => Response | Promise<Response>): ExecutionTargetHarness {
      const requests: { url: string; method: string; body: string }[] = [];
      const statuses: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>[] = [];
      const originalFetch = globalThis.fetch;
      socketGrantTestIssue = null;
      executionTargetStatusObserver = (status) => statuses.push(status);
      openArtifact({ documentId: fixture.intent.scope.documentId, schema: fixture.plan.artifact.schema, bindings: [], actor: "caller-selected-actor" });
      const state = artifactState(fixture.intent.scope.documentId)!;
      state.openClientInstanceId = fixture.intent.clientInstanceId;
      // 🪪️ The caller declares only which surface it wants: nothing forgeable is supplied, so the
      // verified lease is the sole local comparison input for this wasm target.
      const binding: Extract<PersistenceBinding, { kind: "hub" }> = { kind: "hub", baseUrl: fixture.hubOrigin, spaceId: fixture.intent.scope.spaceId, requestedSurfaceId: fixture.intent.requestedSurfaceId };
      clearLocalBrowserBrokerProof();
      installLocalBrowserBrokerProof("a".repeat(64));
      (globalThis as unknown as { fetch: unknown }).fetch = async (input: string, init?: RequestInit) => {
        const url = String(input);
        requests.push({ url, method: String(init?.method ?? "GET"), body: String(init?.body ?? "") });
        return respond(url, requests);
      };
      return {
        state,
        binding,
        requests,
        statuses,
        release: () => {
          executionTargetStatusObserver = null;
          closeArtifact(fixture.intent.scope.documentId);
          clearLocalBrowserBrokerProof();
          (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        },
      };
    }

    it("browser execution target lease verifies GIS wasm bytes before plan exchange", async () => {
      const fixture = await executionTargetLeaseFixture();
      const component = executionTargetBytes(fixture.componentHex);
      const descriptor = executionTargetBytes(fixture.descriptorHex);
      const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
      const grant = { ...fixture.socketGrant, expiresAtMs: Date.now() + 25_000 };
      const harness = executionTargetHarness(fixture, (url) => {
        if (url.endsWith("/open-plan")) return Response.json(plan, { headers: { "x-semio-browser-broker-advanced": "1" } });
        if (url.endsWith("/execution-target/manifest")) return Response.json(fixture.manifest, { headers: { "x-semio-browser-broker-advanced": "1" } });
        if (url.endsWith("/execution-target/component")) return executionTargetBodyResponse(component);
        if (url.endsWith("/execution-target/descriptor")) return executionTargetBodyResponse(descriptor);
        return Response.json(grant, { headers: { "x-semio-browser-broker-advanced": "1" } });
      });
      try {
        expect(fixture.plan.surface.rendererTarget).toBe("wasm");
        expect(fixture.manifest.grant.write).toBe(false);
        const authority = await requestDocumentSocketAuthority(harness.state, harness.binding);
        expect(authority.surfaceId).toBe(fixture.manifest.surface.surfaceId);
        expect(harness.requests.map(({ url }) => url)).toEqual([
          `/_semio/hub${fixture.expected.openPlanPath}`,
          ...fixture.expected.assetPaths.map((path) => `/_semio/hub${path}`),
          `/_semio/hub${fixture.expected.socketGrantPath}`,
        ]);
        for (const index of [1, 2, 3]) {
          expect(harness.requests[index]!.method).toBe("POST");
          expect(JSON.parse(harness.requests[index]!.body)).toEqual(fixture.intent);
          expect(harness.requests[index]!.body).not.toContain(plan.receipt);
        }
        expect(JSON.parse(harness.requests.at(-1)!.body).planReceipt).toBe(plan.receipt);
        expect(harness.state.executionTargetLease).not.toBeNull();
        expect(harness.state.executionTargetLease!.live).toBe(true);
        expect(sameLeaseFieldsV1(harness.state.executionTargetLease!.fields(), fixture.manifest)).toBe(true);
        const observed = harness.statuses.map(({ code, progress }) => `${code}:${progress?.stage ?? "-"}`);
        expect(observed.at(-1)).toBe(`${fixture.expected.rendererState}:-`);
        expect(new Set(harness.statuses.flatMap(({ progress }) => (progress ? [progress.stage] : [])))).toEqual(new Set(fixture.expected.progressStages));
        const payload = JSON.stringify(harness.statuses);
        for (const fragment of fixture.expected.forbiddenStatusFragments) expect(payload).not.toContain(fragment);
      } finally {
        harness.release();
      }
    });

    it("browser execution target lease rejects every single-field substitution without publication", async () => {
      const fixture = await executionTargetLeaseFixture();
      const component = executionTargetBytes(fixture.componentHex);
      const descriptor = executionTargetBytes(fixture.descriptorHex);
      FakeHubWebSocket.instances = [];
      const rejected: string[] = [];
      for (const vector of fixture.hostile) {
        const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
        if (vector.kind === "stale-plan" || vector.kind === "mixed-generation") plan.catalog = { generationId: fixture.expected.rotation.generationA };
        const manifest = vector.kind === "manifest-field"
          ? executionTargetMutate(fixture.manifest, vector.path!, vector.value)
          : vector.kind === "mixed-generation"
            ? structuredClone(fixture.manifest) as unknown as Record<string, unknown>
            : structuredClone(fixture.manifest) as unknown as Record<string, unknown>;
        const harness = executionTargetHarness(fixture, (url, requests) => {
          if (url.endsWith("/open-plan")) return Response.json(vector.kind === "stale-plan" ? { ...plan, catalog: { generationId: fixture.expected.rotation.generationB } } : plan, { headers: { "x-semio-browser-broker-advanced": "1" } });
          if (url.endsWith("/execution-target/manifest")) {
            if (vector.kind === "cancel" && vector.stage === "manifest") requests.length > 0 && harnessAbort();
            return Response.json(manifest, { headers: { "x-semio-browser-broker-advanced": "1" } });
          }
          if (url.endsWith("/execution-target/component")) {
            if (vector.kind === "cancel" && (vector.stage === "component" || vector.stage === "verify")) harnessAbort();
            if (vector.kind === "missing-body" && vector.stage === "component") return new Response(null, { status: 204, headers: { "x-semio-browser-broker-advanced": "1" } });
            if (vector.kind === "component-max-plus-one") return executionTargetBodyResponse(component, fixture.expected.componentMaxBytes + 1);
            if (vector.kind === "component-truncated") return executionTargetBodyResponse(component.subarray(0, component.length - 1), component.length);
            if (vector.kind === "component-extra-byte") return executionTargetBodyResponse(new Uint8Array([...component, 7]), component.length);
            if (vector.kind === "component-bytes" || vector.kind === "mixed-generation") {
              const substituted = new Uint8Array(component);
              substituted[0] = substituted[0]! ^ 0xff;
              return executionTargetBodyResponse(substituted);
            }
            if (vector.kind === "deadline") return new Promise<Response>(() => undefined);
            return executionTargetBodyResponse(component);
          }
          if (url.endsWith("/execution-target/descriptor")) {
            if (vector.kind === "cancel" && vector.stage === "descriptor") harnessAbort();
            if (vector.kind === "missing-body" && vector.stage === "descriptor") return new Response(null, { status: 204, headers: { "x-semio-browser-broker-advanced": "1" } });
            if (vector.kind === "descriptor-max-plus-one") return executionTargetBodyResponse(descriptor, fixture.expected.descriptorMaxBytes + 1);
            if (vector.kind === "descriptor-trailing-byte") return executionTargetBodyResponse(new Uint8Array([...descriptor, 0]));
            if (vector.kind === "descriptor-noncanonical") return executionTargetBodyResponse(encodePackValue({ descriptorVersion: 1, packageId: "semio:gis" }));
            if (vector.kind === "descriptor-self-hash") return executionTargetBodyResponse(encodePackValue({ ...(decodePackValue(descriptor) as Record<string, unknown>), hashes: { wasmSha256: "a".repeat(64), coreWasmSha256: "9".repeat(64), descriptorSha256: "8".repeat(64) } }));
            if (vector.kind === "descriptor-bytes") {
              const substituted = new Uint8Array(descriptor);
              substituted[substituted.length - 1] = substituted[substituted.length - 1]! ^ 0xff;
              return executionTargetBodyResponse(substituted);
            }
            return executionTargetBodyResponse(descriptor);
          }
          return Response.json({ ...fixture.socketGrant, expiresAtMs: Date.now() + 25_000 }, { headers: { "x-semio-browser-broker-advanced": "1" } });
        });
        function harnessAbort(): void {
          harness.state.docAbort.abort();
        }
        try {
          if (vector.kind === "caller-url" || vector.kind === "caller-path" || vector.kind === "caller-module") {
            const denied = await browserExecutionTargetAssetRequest(harness.binding, `${harness.state.config.documentId}/../${String(vector.value)}`, "component", fixture.intent, { timeoutMs: 1_000, signal: harness.state.docAbort.signal }).catch((error: unknown) => error as Error);
            expect((denied as Error).message).toBe("document execution target: operation denied");
            expect(harness.requests).toHaveLength(0);
            rejected.push(vector.name);
            continue;
          }
          if (vector.kind === "viewer-write") {
            await requestDocumentSocketAuthority(harness.state, harness.binding);
            expect(harness.state.executionTargetLease!.fields().grant.write).toBe(false);
            harness.state.hubActorReady = true;
            relayMutationsToHub(harness.state, [sampleEnvelope()]);
            expect(harness.state.outbox).toHaveLength(0);
            expect(harness.state.pendingBatches.size).toBe(0);
            expect(FakeHubWebSocket.instances).toHaveLength(0);
            rejected.push(vector.name);
            continue;
          }
          if (vector.kind === "reconnect-after-invalidation") {
            const authority = await requestDocumentSocketAuthority(harness.state, harness.binding).catch((error: unknown) => error as Error);
            expect(authority).not.toBeInstanceOf(Error);
            harness.state.executionTargetLease!.drop();
            expect(() => harness.state.executionTargetLease!.fields()).toThrow("document execution target lease: dropped");
            rejected.push(vector.name);
            continue;
          }
          if (vector.kind === "deadline") harness.state.docAbort.abort();
          const outcome = await requestDocumentSocketAuthority(harness.state, harness.binding).catch((error: unknown) => error as Error);
          expect(outcome).toBeInstanceOf(Error);
          expect(harness.state.executionTargetLease).toBeNull();
          expect(harness.requests.some(({ url }) => url.endsWith("/socket-grants"))).toBe(false);
          expect(FakeHubWebSocket.instances).toHaveLength(0);
          expect(vector.expected).toBe("unpublished");
          rejected.push(vector.name);
        } finally {
          harness.release();
        }
      }
      expect(rejected).toHaveLength(fixture.hostile.length);
      expect(new Set(rejected).size).toBe(fixture.hostile.length);
    });

    it("browser GIS viewer exposes localized renderer-unavailable after verified lease", async () => {
      const fixture = await executionTargetLeaseFixture();
      const component = executionTargetBytes(fixture.componentHex);
      const descriptor = executionTargetBytes(fixture.descriptorHex);
      const plan = { ...structuredClone(fixture.plan), expiresAtUnixMs: Date.now() + 30_000 };
      const harness = executionTargetHarness(fixture, (url) => {
        if (url.endsWith("/open-plan")) return Response.json(plan, { headers: { "x-semio-browser-broker-advanced": "1" } });
        if (url.endsWith("/execution-target/manifest")) return Response.json(fixture.manifest, { headers: { "x-semio-browser-broker-advanced": "1" } });
        if (url.endsWith("/execution-target/component")) return executionTargetBodyResponse(component);
        if (url.endsWith("/execution-target/descriptor")) return executionTargetBodyResponse(descriptor);
        return Response.json({ ...fixture.socketGrant, expiresAtMs: Date.now() + 25_000 }, { headers: { "x-semio-browser-broker-advanced": "1" } });
      });
      const events: ArtifactEvent[] = [];
      const originalPostMessage = (globalThis as unknown as { postMessage?: unknown }).postMessage;
      try {
        await requestDocumentSocketAuthority(harness.state, harness.binding);
        const terminal = harness.statuses.at(-1)!;
        expect(terminal.code).toBe(fixture.expected.rendererState);
        for (const [code, text] of Object.entries(fixture.expected.status)) {
          expect(DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1[code as keyof typeof DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1]).toEqual(text);
          expect(documentExecutionTargetStatusRoleV1(code as keyof typeof DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1)).toBe(fixture.expected.statusRoles[code]);
        }
        expect(Object.keys(DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1).sort()).toEqual(Object.keys(fixture.expected.status).sort());
        const source = await (await import("node:fs/promises")).readFile(new URL("./🧵️backbone-worker.ts", import.meta.url), "utf8");
        const leaseRegion = source.slice(source.indexOf("//#region 🪪️ExecutionTargetLease"), source.indexOf("//#endregion 🪪️ExecutionTargetLease"));
        expect(leaseRegion).not.toContain("loadPluginModule");
        expect(leaseRegion).not.toContain("ActivationRegistry");
        expect(leaseRegion).not.toContain("load_wasm_plugins");
        expect(leaseRegion).not.toContain("attach_backbone");
        expect(Object.values(fixture.expected.rendererClaims).every((claim) => claim === false)).toBe(true);
        harness.state.hubActorReady = true;
        const originalEmit = FakeHubWebSocket.instances.length;
        relayMutationsToHub(harness.state, [sampleEnvelope()]);
        expect(harness.state.outbox).toHaveLength(0);
        expect(FakeHubWebSocket.instances).toHaveLength(originalEmit);
        expect(events).toHaveLength(0);
      } finally {
        (globalThis as unknown as { postMessage?: unknown }).postMessage = originalPostMessage;
        harness.release();
      }
    });

    it("browser document open rejects mismatched and max-plus-one plans and cancels before receipt exchange without leaking authority", async () => {
      const fixture = await browserDocumentOpenFixture();
      const current = currentBrowserDocumentOpenFixture(fixture);
      const originalFetch = globalThis.fetch;
      FakeHubWebSocket.instances = [];
      socketGrantTestIssue = null;
      openArtifact({ documentId: fixture.intent.scope.documentId, schema: fixture.plan.artifact.schema, bindings: [], actor: "caller-selected-actor" });
      const state = artifactState(fixture.intent.scope.documentId)!;
      state.openClientInstanceId = fixture.intent.clientInstanceId;
      const binding: Extract<PersistenceBinding, { kind: "hub" }> = { kind: "hub", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId, installedTarget: fixture.installedTarget };
      try {
        const hostileReceipt = current.plan.receipt;
        let effects = 0;
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          effects += 1;
          return Response.json(current.plan, { headers: { "x-semio-browser-broker-advanced": "1" } });
        };
        const unavailable = await requestDocumentSocketAuthority(state, { kind: "hub", baseUrl: "http://hub.test", spaceId: fixture.intent.scope.spaceId }).catch((error: unknown) => error as Error);
        expect(unavailable.message).toBe("document open: installed target unavailable");
        expect(effects).toBe(0);

        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          effects += 1;
          return Response.json({ ...structuredClone(current.plan), scope: { ...current.plan.scope, spaceId: "foreign" } }, { headers: { "x-semio-browser-broker-advanced": "1" } });
        };
        const mismatch = await requestDocumentSocketAuthority(state, binding).catch((error: unknown) => error as Error);
        expect(mismatch.message).toBe("document open: invalid plan");
        expect(mismatch.message).not.toContain(hostileReceipt);
        expect(effects).toBe(1);

        clearLocalBrowserBrokerProof();
        installLocalBrowserBrokerProof("b".repeat(64));
        effects = 0;
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          effects += 1;
          return new Response("{}", { headers: { "content-length": String(fixture.expected.responseMaxBytes + 1), "x-semio-browser-broker-advanced": "1" } });
        };
        const oversized = await requestDocumentSocketAuthority(state, binding).catch((error: unknown) => error as Error);
        expect(oversized.message).toBe("document open: invalid plan");
        expect(effects).toBe(1);

        clearLocalBrowserBrokerProof();
        installLocalBrowserBrokerProof("c".repeat(64));
        effects = 0;
        (globalThis as unknown as { fetch: unknown }).fetch = async () => {
          effects += 1;
          state.docAbort.abort();
          return Response.json(current.plan, { headers: { "x-semio-browser-broker-advanced": "1" } });
        };
        const cancelled = await requestDocumentSocketAuthority(state, binding).catch((error: unknown) => error as Error);
        expect(cancelled.message).toBe("document open: cancelled");
        expect(cancelled.message).not.toContain(hostileReceipt);
        expect(effects).toBe(1);
        expect(FakeHubWebSocket.instances).toHaveLength(0);
      } finally {
        closeArtifact(fixture.intent.scope.documentId);
        clearLocalBrowserBrokerProof();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
      }
    });

    it("keeps two concurrent document grant actors isolated and rewrites caller envelopes at the wire boundary", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      const actorA = `hub.v1.${"a".repeat(64)}`;
      const actorB = `hub.v1.${"b".repeat(64)}`;
      socketGrantTestIssue = async (_baseUrl, path) => ({
        schema: "semio.hub.socket-grant/v1",
        protocol: "semio.socket.v1",
        grant: `socket.v1.${path.includes("doc-a") ? "1".repeat(32) : "2".repeat(32)}.${"3".repeat(64)}`,
        actorId: path.includes("doc-a") ? actorA : actorB,
        expiresAtMs: Number.MAX_SAFE_INTEGER,
      });
      const envelope = (document: string): MutationEnvelope => ({
        id: `edit-${document}`,
        actor: "caller-selected-actor",
        document,
        schemaVersion: "demo/v1",
        deps: [],
        payloadHash: "unused",
        diff: { schemaId: "demo/v1", payload: { document } },
        inverse: { targetOperation: `edit-${document}`, inverseDiff: { schemaId: "demo/v1", payload: {} }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
      });
      try {
        for (const documentId of ["doc-a", "doc-b"]) openArtifact({ documentId, schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "space-1" }], actor: "caller-selected-actor" });
        await flushSocketGrantTurns();
        const [socketA, socketB] = FakeHubWebSocket.instances;
        socketA!.open();
        socketB!.open();
        await handleHubFrame(artifactState("doc-a", "space-1")!, { Session: { actor: actorA, color: 1 } });
        await handleHubFrame(artifactState("doc-b", "space-1")!, { Session: { actor: actorB, color: 2 } });
        handleTsRequest({ kind: "send", documentId: "doc-a", message: { kind: "localMutations", envelopes: [envelope("doc-a")] } });
        handleTsRequest({ kind: "send", documentId: "doc-b", message: { kind: "localMutations", envelopes: [envelope("doc-b")] } });
        const commandActor = (socket: FakeHubWebSocket): string => {
          const frame = decodeClientFrame(socket.sent[1]!).frame;
          if (typeof frame === "string" || !("Commands" in frame)) throw new Error("expected commands");
          return frame.Commands.envelopes[0]!.actor;
        };
        expect(commandActor(socketA!)).toBe(actorA);
        expect(commandActor(socketB!)).toBe(actorB);
        expect(commandActor(socketA!)).not.toBe(commandActor(socketB!));
      } finally {
        closeArtifact("doc-a");
        closeArtifact("doc-b");
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
      }
    });

    async function flushMicrotasks(): Promise<void> {
      await new Promise((resolve) => setTimeout(resolve, 0));
      await new Promise((resolve) => setTimeout(resolve, 0));
    }

    it("poll never overlaps itself: concurrent revalidateFolder() calls collapse into one coalesced follow-up", async () => {
      FakeEventSource.instances = [];
      (globalThis as unknown as { EventSource: unknown }).EventSource = FakeEventSource;
      let fetchCalls = 0;
      // 🚪️ Gated rather than immediately resolved — the whole point of this test is to fire more
      // calls WHILE one is still in flight, so the fetch must stay pending until we say so.
      const gates: Array<() => void> = [];
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => {
        fetchCalls += 1;
        await new Promise<void>((resolve) => gates.push(resolve));
        return notFoundResponse();
      };

      try {
        openArtifact(folderOnlyConfig("doc-overlap"));
        const state = artifactState("doc-overlap")!;
        await flushMicrotasks(); // let watchFolder's bootstrap read actually START (not settle — it's gated).
        expect(fetchCalls).toBe(1);

        // 🥇️ Three callers race a revalidate while the bootstrap read is still stuck at its gate —
        // `latestWins` must coalesce all three into exactly ONE queued follow-up, never three reruns.
        const call2 = state.revalidateFolder();
        const call3 = state.revalidateFolder();
        const call4 = state.revalidateFolder();
        await flushMicrotasks();
        expect(fetchCalls).toBe(1); // nothing new launched synchronously — still just the bootstrap.

        gates.shift()!(); // let the bootstrap call resolve, which launches the coalesced follow-up.
        await flushMicrotasks();
        expect(fetchCalls).toBe(2); // exactly one follow-up — never a separate rerun per caller.

        gates.shift()!(); // let the follow-up resolve so call2/call3/call4 all settle.
        await Promise.all([call2, call3, call4]);
        expect(fetchCalls).toBe(2);
      } finally {
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        closeArtifact("doc-overlap");
      }
    });

    it("poll is suppressed while SSE is healthy and resumes once it drops", async () => {
      FakeEventSource.instances = [];
      (globalThis as unknown as { EventSource: unknown }).EventSource = FakeEventSource;
      let fetchCalls = 0;
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => {
        fetchCalls += 1;
        return notFoundResponse();
      };
      vi.useFakeTimers();
      // 🎯 Deterministic jitter (every delay collapses to its minimum): this test advances fake time
      // across THREE phases in sequence, and leftover jitter slack from an earlier phase could
      // otherwise let two sanity ticks land inside one later advance window — pinning `Math.random`
      // removes that risk instead of just hoping the window is wide enough.
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0);

      try {
        openArtifact(folderOnlyConfig("doc-sanity"));
        const state = artifactState("doc-sanity")!;
        await vi.advanceTimersByTimeAsync(0); // bootstrap read.
        expect(fetchCalls).toBe(1);

        // 🛟️ SSE never opened yet (`sseHealthy` still false) — the sanity fallback must still fire.
        await vi.advanceTimersByTimeAsync(SANITY_POLL_MIN_MS + 1);
        expect(fetchCalls).toBe(2);

        // 📡️ SSE opens — the very next sanity tick must be a no-op while it stays healthy.
        const source = FakeEventSource.instances.at(-1)!;
        source.onopen?.();
        expect(state.sseHealthy).toBe(true);
        await vi.advanceTimersByTimeAsync(SANITY_POLL_MIN_MS + 1);
        expect(fetchCalls).toBe(2); // suppressed — no new fetch while SSE is healthy.

        // 📴️ SSE drops — the fallback must resume on the next tick.
        source.onerror?.();
        expect(state.sseHealthy).toBe(false);
        await vi.advanceTimersByTimeAsync(SANITY_POLL_MIN_MS + 1);
        expect(fetchCalls).toBe(3);
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        closeArtifact("doc-sanity");
      }
    });

    it("a post-open SSE drop reconnects with jittered backoff", async () => {
      FakeEventSource.instances = [];
      (globalThis as unknown as { EventSource: unknown }).EventSource = FakeEventSource;
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => notFoundResponse();
      vi.useFakeTimers();

      try {
        openArtifact(folderOnlyConfig("doc-sse-reconnect"));
        await vi.advanceTimersByTimeAsync(0);
        expect(FakeEventSource.instances).toHaveLength(1);

        const first = FakeEventSource.instances[0]!;
        first.onopen?.();
        first.onerror?.(); // drops AFTER a successful open — the bug finding 2 is about.
        expect(first.closed).toBe(true);

        // 🔁️ Reconnect is jittered within [SSE_RECONNECT_MIN_MS, SSE_RECONNECT_MAX_MS] — advancing
        // past the max guarantees the next attempt has fired regardless of the random draw.
        await vi.advanceTimersByTimeAsync(SSE_RECONNECT_MAX_MS + 1);
        expect(FakeEventSource.instances.length).toBeGreaterThan(1);
      } finally {
        vi.useRealTimers();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        closeArtifact("doc-sse-reconnect");
      }
    });

    it("abort on close cancels an in-flight folder fetch", async () => {
      let capturedSignal: AbortSignal | undefined;
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = (_url: string, init?: RequestInit) => {
        capturedSignal = init?.signal ?? undefined;
        return new Promise(() => {}); // never settles — only `closeArtifact` can end this.
      };
      const originalEventSource = (globalThis as unknown as { EventSource: unknown }).EventSource;
      (globalThis as unknown as { EventSource: unknown }).EventSource = class {
        constructor() {
          throw new Error("no SSE in this test");
        }
      };

      try {
        openArtifact(folderOnlyConfig("doc-abort"));
        await Promise.resolve();
        expect(capturedSignal).toBeDefined();
        expect(capturedSignal?.aborted).toBe(false);

        closeArtifact("doc-abort");
        expect(capturedSignal?.aborted).toBe(true);
      } finally {
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        (globalThis as unknown as { EventSource: unknown }).EventSource = originalEventSource;
      }
    });

    it("queue overflow rejects and reports rather than dropping silently", () => {
      const config: ArtifactActorConfig = { documentId: "doc-overflow", schema: "demo/v1", bindings: [], actor: "actor-1" };
      openArtifact(config);
      const errorSpy = vi.spyOn(console, "error").mockImplementation(() => {});

      try {
        const state = artifactState("doc-overflow")!;
        const makeEnvelope = (index: number): MutationEnvelope => ({
          id: `edit-${index}`,
          actor: "actor-1",
          document: "doc-overflow",
          schemaVersion: "demo/v1",
          deps: [],
          payloadHash: "unused",
          diff: { schemaId: "demo/v1", payload: { n: index } },
          inverse: { targetOperation: `edit-${index}`, inverseDiff: { schemaId: "demo/v1", payload: { n: 0 } }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
        });
        const overSized = Array.from({ length: PENDING_MUTATIONS_QUEUE_LIMIT + 1 }, (_unused, index) => makeEnvelope(index));

        handleTsRequest({ kind: "send", documentId: "doc-overflow", message: { kind: "localMutations", envelopes: overSized } });

        // 🚨️ Rejected wholesale, never partially accepted or silently dropped — the queue is
        // untouched, and the rejection is explicitly logged (the shell-facing signal is the same
        // `commandOutcome`/`rejected` vocabulary a real hub rejection uses — see
        // `rejectMutationQueueOverflow`'s doc comment).
        expect(state.pendingMutations).toHaveLength(0);
        expect(errorSpy).toHaveBeenCalledWith("[backbone-worker] pending mutation queue full, rejecting batch", "doc-overflow", overSized.length);

        // ✅ A batch that fits is still accepted normally — overflow doesn't wedge the queue shut.
        handleTsRequest({ kind: "send", documentId: "doc-overflow", message: { kind: "localMutations", envelopes: [makeEnvelope(0)] } });
        expect(state.pendingMutations).toHaveLength(1);
      } finally {
        errorSpy.mockRestore();
        closeArtifact("doc-overflow");
      }
    });

    it("a mutation made while offline is queued, then flushed once the hub reconnects (Welcome flushes the outbox)", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;

      try {
        const config: ArtifactActorConfig = { documentId: "doc-hub-flush", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1" };
        openArtifact(config);
        await flushSocketGrantTurns();
        const state = artifactState("doc-hub-flush")!;
        const socket = FakeHubWebSocket.instances.at(-1)!;
        expect(socket.readyState).toBe(FakeHubWebSocket.CONNECTING);

        const envelope: MutationEnvelope = {
          id: "edit-offline-1",
          actor: "actor-1",
          document: "doc-hub-flush",
          schemaVersion: "demo/v1",
          deps: [],
          payloadHash: "unused",
          diff: { schemaId: "demo/v1", payload: { n: 1, sequenceNumber: 1 } },
          inverse: { targetOperation: "edit-offline-1", inverseDiff: { schemaId: "demo/v1", payload: { n: 0 } }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
        };
        handleTsRequest({ kind: "send", documentId: "doc-hub-flush", message: { kind: "localMutations", envelopes: [envelope] } });

        // 📴️ Socket isn't open yet — the mutation is queued in the outbox, never silently dropped.
        expect(state.outbox).toHaveLength(1);
        expect(socket.sent).toHaveLength(0);
        expect(state.pendingMutations).toHaveLength(1);

        // 🔌️ Hub (re)connects: `Hello` goes out on open, then the hub answers with `Welcome`.
        socket.open();
        expect(socket.sent).toHaveLength(1); // Hello

        const welcome: ServerFrame = {
          Welcome: {
            session_id: "s1",
            resume_token: "resume-1",
            server_frontier: { document_id: "doc-hub-flush", head_edit_ordinal: 0, head_edit_id: "e0", last_commit_seq: 0, chain_hash: new Array(32).fill(0) },
            bootstrap: "None",
          },
        };
        socket.onmessage?.({ data: encodeServerFrame(welcome, "command").buffer as ArrayBuffer });
        await state.hubFrameChain;

        expect(state.outbox).toHaveLength(1);
        expect(socket.sent).toHaveLength(1);
        await handleHubFrame(state, { Session: { actor: `hub.v1.${"3".repeat(64)}`, color: 4 } });

        // ♻️ Exact Session authority, not Welcome alone, flushes the outbox.
        expect(state.outbox).toHaveLength(0);
        expect(socket.sent).toHaveLength(2); // Hello + the flushed Commands batch.
        const commandsFrame = decodeClientFrame(socket.sent[1]!).frame;
        if (typeof commandsFrame === "string" || !("Commands" in commandsFrame)) throw new Error("expected a Commands frame");
        expect(commandsFrame.Commands.envelopes).toHaveLength(1);
        expect(commandsFrame.Commands.envelopes[0]!.mutation_id).toBe("edit-offline-1");
        expect(state.pendingBatches.size).toBe(1); // now awaiting `Ack` — no longer in the outbox.
      } finally {
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        closeArtifact("doc-hub-flush");
      }
    });

    it("a batch whose socket dies before Ack moves back into the outbox instead of being stranded", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;

      try {
        const config: ArtifactActorConfig = { documentId: "doc-hub-stranded", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1" };
        openArtifact(config);
        await flushSocketGrantTurns();
        const state = artifactState("doc-hub-stranded")!;
        const socket = FakeHubWebSocket.instances.at(-1)!;
        socket.open();
        await handleHubFrame(state, { Session: { actor: `hub.v1.${"3".repeat(64)}`, color: 5 } });

        const envelope: MutationEnvelope = {
          id: "edit-inflight-1",
          actor: "actor-1",
          document: "doc-hub-stranded",
          schemaVersion: "demo/v1",
          deps: [],
          payloadHash: "unused",
          diff: { schemaId: "demo/v1", payload: { n: 1, sequenceNumber: 1 } },
          inverse: { targetOperation: "edit-inflight-1", inverseDiff: { schemaId: "demo/v1", payload: { n: 0 } }, baseVersion: 0, dependencies: [], undoPolicy: "exactBaseOnly" },
        };
        handleTsRequest({ kind: "send", documentId: "doc-hub-stranded", message: { kind: "localMutations", envelopes: [envelope] } });
        expect(state.pendingBatches.size).toBe(1); // socket was open — sent immediately, awaiting Ack.
        expect(state.outbox).toHaveLength(0);

        // 💥️ The socket dies before the hub ever acks — the batch must not be lost.
        socket.close();
        expect(state.pendingBatches.size).toBe(0);
        expect(state.outbox).toHaveLength(1);
        expect(state.outbox[0]!.id).toBe("edit-inflight-1");
        expect(state.pendingMutations).toHaveLength(1); // status-visible pending count is unaffected.
      } finally {
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        closeArtifact("doc-hub-stranded");
      }
    });

    // 🧬️ Coordinator follow-up (finding 4b): `retryWithJitteredBackoff`'s attempt counter grows for
    // the life of one call and never resets after success — `connectHub`/`connectSseOnce` now loop
    // fresh calls via `reconnectForever`, resetting only after SUSTAINED health, never on "socket
    // opened" alone. `Math.random` is pinned throughout (not to 0 — that would collapse every
    // jittered delay to its floor and hide growth entirely) so the exact backoff value at every
    // attempt is a known, computable number, making "did it actually reset" a precise assertion
    // rather than a coincidence of timing windows.
    it("a hub drop after sustained health resets the backoff, unlike continued accumulation", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0.5);

      try {
        const config: ArtifactActorConfig = { documentId: "doc-hub-reset", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1" };
        openArtifact(config);
        await flushSocketGrantTurns();

        // 💥️ Two quick failures BEFORE any sustained health — attempt 1 → 750ms, attempt 2 → 1250ms
        // (both exact with `Math.random` pinned at 0.5: `minMs + 0.5*(min(maxMs,minMs*2**attempt)-minMs)`).
        FakeHubWebSocket.instances[0]!.close();
        await vi.advanceTimersByTimeAsync(751);
        expect(FakeHubWebSocket.instances).toHaveLength(2);
        FakeHubWebSocket.instances[1]!.close();
        await vi.advanceTimersByTimeAsync(1251);
        expect(FakeHubWebSocket.instances).toHaveLength(3);

        // ✅️ Third attempt opens and stays up long enough to count as sustainedly healthy.
        const healthy = FakeHubWebSocket.instances[2]!;
        healthy.open();
        await vi.advanceTimersByTimeAsync(SUSTAINED_HEALTHY_MS + 1);

        // 📴️ NOW it drops. If the attempt counter had kept accumulating, the next attempt (attempt 3)
        // would wait 2250ms. A reset instead starts a brand-new call — its first failure is attempt 1,
        // waiting only 750ms.
        healthy.close();
        await vi.advanceTimersByTimeAsync(0); // let the resolved promise's fresh retryWithJitteredBackoff call fire its immediate first attempt.
        expect(FakeHubWebSocket.instances).toHaveLength(4); // the fresh call's immediate (0-delay) first attempt.
        FakeHubWebSocket.instances[3]!.close(); // that immediate attempt also fails fast.

        // 🎯 The decisive check: 800ms is enough for the RESET value (750ms) but not enough for what
        // continued accumulation would have required (2250ms).
        await vi.advanceTimersByTimeAsync(800);
        expect(FakeHubWebSocket.instances).toHaveLength(5);
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        closeArtifact("doc-hub-reset");
      }
    });

    it("rapid accept-then-drop cycling does NOT reset the hub backoff — it keeps climbing", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0.5);

      try {
        const config: ArtifactActorConfig = { documentId: "doc-hub-no-reset", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1" };
        openArtifact(config);
        await flushSocketGrantTurns();

        // 🔁️ Each cycle opens (well under `SUSTAINED_HEALTHY_MS`) then drops immediately — never
        // healthy long enough to reset. Attempt 1 → 750ms, attempt 2 → 1250ms, attempt 3 → 2250ms.
        FakeHubWebSocket.instances[0]!.open();
        FakeHubWebSocket.instances[0]!.close();
        // ⏱️ 800ms is past attempt 1's 750ms floor but nowhere near attempt-2-sized delays — confirms
        // the wait is climbing on schedule, not staying flat at the floor.
        await vi.advanceTimersByTimeAsync(800);
        expect(FakeHubWebSocket.instances).toHaveLength(2);

        FakeHubWebSocket.instances[1]!.open();
        FakeHubWebSocket.instances[1]!.close();
        // 🎯 The decisive check: 800ms was enough after attempt 1 (750ms) but must NOT be enough here —
        // if this fired, the counter would have wrongly reset back down near the floor.
        await vi.advanceTimersByTimeAsync(800);
        expect(FakeHubWebSocket.instances).toHaveLength(2); // still 2 — attempt 2's 1250ms hasn't elapsed.
        await vi.advanceTimersByTimeAsync(451); // 800 + 451 = 1251 total, past attempt 2's 1250ms.
        expect(FakeHubWebSocket.instances).toHaveLength(3);
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        closeArtifact("doc-hub-no-reset");
      }
    });

    it("abort cancels the hub reconnect loop promptly, with no leaked timer", async () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;
      vi.useFakeTimers();

      try {
        const config: ArtifactActorConfig = { documentId: "doc-hub-abort", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1" };
        openArtifact(config);
        await flushSocketGrantTurns();
        FakeHubWebSocket.instances[0]!.open(); // sustained-health timer now pending too.

        closeArtifact("doc-hub-abort");
        await vi.advanceTimersByTimeAsync(0);

        // 🧹️ Nothing left pending — not the sustained-health timer, not a reconnect backoff delay.
        expect(vi.getTimerCount()).toBe(0);

        // 🚫️ …and no further reconnect attempt ever happens, however long we wait.
        await vi.advanceTimersByTimeAsync(60_000);
        expect(FakeHubWebSocket.instances).toHaveLength(1);
      } finally {
        vi.useRealTimers();
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
      }
    });

    it("an SSE drop after sustained health resets ITS backoff too (the same fix applied to connectSseOnce)", async () => {
      FakeEventSource.instances = [];
      (globalThis as unknown as { EventSource: unknown }).EventSource = FakeEventSource;
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => notFoundResponse();
      vi.useFakeTimers();
      // 🎯 SSE's own formula: `minMs=1000, maxMs=30000` → attempt 1 = 1000+0.5*(2000-1000)=1500ms.
      const randomSpy = vi.spyOn(Math, "random").mockReturnValue(0.5);

      try {
        openArtifact(folderOnlyConfig("doc-sse-reset"));
        await vi.advanceTimersByTimeAsync(0); // bootstrap read + first SSE connect attempt.
        expect(FakeEventSource.instances).toHaveLength(1);

        const healthy = FakeEventSource.instances[0]!;
        healthy.onopen?.();
        await vi.advanceTimersByTimeAsync(SUSTAINED_HEALTHY_MS + 1);
        healthy.onerror?.(); // drops AFTER sustained health.
        await vi.advanceTimersByTimeAsync(0); // fresh reconnectForever cycle's immediate first attempt.
        expect(FakeEventSource.instances).toHaveLength(2);

        // 🎯 That fresh attempt also fails fast — the wait before the NEXT one must be the reset
        // (attempt 1 ≈ 1500ms), not a continuation of any prior accumulation (there was none yet in
        // this call, so this mirrors the hub test's decisive-window shape at SSE's own numbers).
        FakeEventSource.instances[1]!.onerror?.();
        await vi.advanceTimersByTimeAsync(1600);
        expect(FakeEventSource.instances).toHaveLength(3);
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        closeArtifact("doc-sse-reset");
      }
    });
  });
  //#endregion 🔖️OfflineResilienceTests

  describe("backbone-worker scope-safe presence", () => {
    it("binds verified surface authority only after the exact socket Session and isolates equal document ids", async () => {
      const responses: BackboneWorkerResponse[] = [];
      workerPostTestSink = (message) => responses.push(message);
      const socketA = { close: vi.fn() } as unknown as WebSocket;
      const socketB = { close: vi.fn() } as unknown as WebSocket;
      const makeState = (spaceId: string, socket: WebSocket): ArtifactState => ({
        config: { documentId: "same-document", schema: "demo/v1", actor: "requested", bindings: [{ kind: "hub", baseUrl: "https://hub.example", spaceId }] },
        socket,
        actor: "",
        hubActorReady: false,
        pendingSocketActorId: `hub.v1.${spaceId === "space-a" ? "a" : "b"}`.padEnd(71, spaceId === "space-a" ? "a" : "b"),
        presenceAuthority: null,
        outbox: [],
        sessionColor: null,
      }) as unknown as ArtifactState;
      const stateA = makeState("space-a", socketA);
      const stateB = makeState("space-b", socketB);
      const actorA = stateA.pendingSocketActorId!;
      const actorB = stateB.pendingSocketActorId!;
      try {
        await handleHubFrame(stateA, { Session: { actor: actorA, color: 2 } }, { socket: socketA, scope: { spaceId: "space-a", documentId: "same-document" }, verifiedSurfaceId: "map@1/*#editor" });
        await handleHubFrame(stateB, { Session: { actor: actorB, color: 5 } }, { socket: socketB, scope: { spaceId: "space-b", documentId: "same-document" }, verifiedSurfaceId: "map@1/*#viewer" });
        emitEvent(stateA, { kind: "presence", peers: [{ actor: actorA, label: "Ada", connectedAtMs: 101, color: 2, surface: "map@1/*#editor", views: [] }] });
        emitEvent(stateB, { kind: "presence", peers: [{ actor: actorB, label: "Berta", connectedAtMs: 202, color: 5, surface: "map@1/*#viewer", views: [] }] });
        const presence = responses.filter((message): message is Extract<BackboneWorkerResponse, { kind: "event" }> => message.kind === "event" && message.event.kind === "presence");
        expect(presence.map((message) => [message.scope?.spaceId, message.verifiedSurfaceId, message.event.kind === "presence" ? message.event.peers[0]?.label : undefined])).toEqual([
          ["space-a", "map@1/*#editor", "Ada"],
          ["space-b", "map@1/*#viewer", "Berta"],
        ]);
        emitEvent(stateA, { kind: "presence", peers: [] });
        stateA.presenceAuthority = null;
        emitEvent(stateA, { kind: "presence", peers: [{ actor: actorA, connectedAtMs: 303, surface: "map@1/*#editor", views: [] }] });
        const tail = responses.slice(-2) as Extract<BackboneWorkerResponse, { kind: "event" }>[];
        expect(tail[0]).toMatchObject({ scope: { spaceId: "space-a", documentId: "same-document" }, verifiedSurfaceId: "map@1/*#editor", event: { kind: "presence", peers: [] } });
        expect(tail[1]).toMatchObject({ scope: { spaceId: "space-a", documentId: "same-document" }, event: { kind: "presence", peers: [] } });
        expect(tail[1]?.verifiedSurfaceId).toBeUndefined();
      } finally {
        workerPostTestSink = null;
      }
    });
  });
}
//#endregion 🧪️Tests
