// #region Header
/**
 * 🧵️ `🧵️backbone-worker.ts` — browser backbone loader. Authenticated hub
 * document lifecycles are owned here so D1 issue/exchange/WebSocket authority cannot be bypassed
 * when the Rust WASM worker resolves; other lanes use Rust when available and the TypeScript twin
 * otherwise.
 */
// #endregion Header

import type {
  ArtifactBootstrapControl,
  ArtifactBootstrapProgress,
  ArtifactPresencePeer,
  ClientFrame,
  ExactWireMutationEnvelope,
  MutationEnvelope,
  ServerFrame,
  WireAckStage,
  WireArtifactBootstrap,
  WireFrontierSummary,
  WireLane,
  WireMutationEnvelope,
} from "@semio-tech/framework-replication";
import type {
  ArtifactActorConfig,
  ArtifactActorMsg,
  ArtifactBootstrapWorkerEvent,
  ArtifactEvent,
  ArtifactSyncStatus,
  BackboneWorkerRequest,
  BackboneWorkerResponse,
  BackboneWorkerWireMessage,
  BrowserBrokerPortResponseV1,
  CanonicalDirectoryEventPageV1,
  CommandAckOutcome,
  DirectoryAcknowledgedStream,
  DirectoryAdministrationPhaseV1,
  DirectoryCommand,
  DirectoryEventPageAckV1,
  DirectoryStreamMessage,
  DocumentScope,
  GisMapApprovalHistoryStatusV1,
  PersistenceBinding,
  RemoteState,
  SocketGrantReceiptV1,
} from "./🟦️";
import { ArtifactBootstrapAssembler, DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS, DOCUMENT_BACKBONE_RETENTION_LIMITS, decodeClientFrame, decodePresencePeer, decodeServerFrame, encodeClientFrame, encodeDocumentBackboneEnvelopeBatchExact, encodePresencePeer, encodeServerFrame, extractServerCommandsDocumentBackboneBatchExact } from "@semio-tech/framework-replication";
import {
  DirectoryClient,
  DirectoryCommandError,
  DirectoryHttpError,
  HUB_RECONNECT_MAX_MS,
  HUB_RECONNECT_MIN_MS,
  createSocketGrantIssuerV1,
  decodeBackboneWorkerRequest,
  decodeBackboneWorkerResponse,
  decodeDocumentPackBytes,
  decodePackWire,
  decodePackValue,
  documentRuntimeKeyV1,
  encodeBackboneMessage,
  encodeBackboneWorkerRequest,
  encodeBackboneWorkerResponse,
  encodeDocumentPackBytes,
  encodePackValue,
  isPackInteger,
  packUIntSafeOrNull,
  packWireNatural,
  parseBrowserBrokerPortRequestV1,
  parseDocumentBackboneMessage,
  parseSocketGrantReceiptV1,
  socketGrantProtocolsV1,
} from "./🟦️";
import type { PackValue } from "./🟦️";
import { SPACE_ARTIFACT_CREATION_CATALOG_MAX_BYTES, SPACE_ARTIFACT_CREATION_MAX_BYTES, parseSpaceArtifactCreationCatalogJsonV1, parseSpaceArtifactCreationStatusJsonV1, sealSpaceArtifactCreateV1, type SpaceArtifactCreationCatalogV1 as HubSpaceArtifactCreationCatalogV1, type SpaceArtifactCreationStatusV1 as HubSpaceArtifactCreationStatusV1 } from "./🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";
import { browserActorChildCapacity, reserveBrowserActorChild, type BrowserActorChildValue } from "./🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts";
import { assertBrowserActorDescribeCapacityV1, verifyBrowserActorDescribeV1 } from "./🔨️modules/🔌️plugin/🌐️browser-bundle/🧾️describe/🟦️.ts";
import { BROWSER_ACTOR_CHILD_LIMITS, measureChildValue } from "./🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts";
import { coldDocumentPairCursorEquals, coldDocumentPairFrontierEquals, parseColdDocumentPairLifetime, parseWitColdPairIngressStatus, type ColdDocumentPairFrontier, type ColdPairIngressStatus } from "../../🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts";
import { actorInstanceCapturedReceiptMatches, actorInstanceLifetimeEquals, type ActorInstanceLifecycleReceipt, type ActorInstanceLifetime, type ActorInstanceOpenRequest } from "../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts";
import { encodeActorUiPatchReceipt } from "../../🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts";
import { browserActorUiPatchOwnerMatchesV1, captureBrowserActorUiPatchV1, type BrowserActorUiPatchOfferV1, type BrowserActorUiPatchResultV1 } from "./🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts";
import { parseBrowserActorViewStateRequest } from "./🔨️modules/🔌️plugin/🌐️browser-bundle/🪟️view-context/🟦️.ts";
import { windowViewContext, type ResolvedPluginViewState } from "../../🔨️modules/🛂️manifest/🟦️.ts";
import type {
  DirectoryCommandErrorCodeV1,
  DirectoryCommandOutcomeV1,
  DirectoryCommandReceiptV1,
  DirectoryCommandRequestV1,
  DirectoryCommandResultV1,
  DocumentExecutionTargetLeaseFieldsV1,
  DocumentExecutionTargetProgressV1,
  DocumentExecutionTargetStatusCodeV1,
  DocumentOpenIntentV1,
  DocumentOpenPlanV1,
  GisMapApprovalUndoHandleV1,
  GisMapApprovalUndoReceiptV1,
  GisMapInferenceApprovalReceiptV1,
  GisMapInferencePortCodeV1,
  GisMapInferencePortEventV1,
  GisMapInferencePortStatusV1,
  GisMapInferencePreviewV1,
} from "./🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import {
  DOCUMENT_BROWSER_ACTOR_MAX_BYTES,
  DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1,
  GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES,
  gisMapInferenceCodeFromStatusV1,
  gisMapInferencePortTerminalV1,
  idleGisMapInferencePortStatusV1,
  parseGisMapInferenceApprovalReceiptV1,
  parseGisMapApprovalUndoReceiptV1,
  parseGisMapInferenceEventPageV1,
  parseGisMapInferenceJobReceiptV1,
  reduceGisMapInferencePortV1,
  sealGisMapApprovalUndoRequestV1,
  sealGisMapInferenceApprovalRequestV1,
  sealGisMapInferenceJobRequestV1,
} from "./🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import {
  DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES,
  DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES,
  DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1,
  directoryCommandErrorIsTransient,
  directoryCommandRequestJson,
  directoryCommandSha256,
  documentExecutionTargetStatusRoleV1,
  leaseFieldsFromPlanV1,
  parseDocumentExecutionTargetLeaseFieldsV1,
  parseDocumentOpenIntentV1,
  parseDocumentOpenPlanV1,
  parseDocumentPlanSocketGrantIntentV1,
  sameLeaseFieldsV1,
  sealDirectoryCommandReceiptV1,
  sealDirectoryCommandRequestV1,
} from "./🔨️modules/📇️directory/🧬️schema/🟦️.ts";
/** 🔏️ First-party BLAKE3 runtime module — Web Crypto supplies SHA-256 but has no BLAKE3, so a
 * verified execution-target component is hashed with the repository's own implementation. */
import { blake3Hex } from "@semio-tech/framework";
/** 🎚️ config-lane attach (contract freeze §4) — `OpeningPreferences` is a kernel type (domain-neutral
 * framework), never redefined here; see this file's `🔖️ConfigLane` region. */
import type { OpeningPreferences, UiNodeRecord } from "@semio-tech/framework";
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
type DocumentExecutionOwnerEntry = Readonly<{ owner: DocumentExecutionOwner; documentId: string; clientInstanceId: string; spaceId?: string }>;

const documentExecutionOwners = new Map<string, DocumentExecutionOwnerEntry>();

function validDocumentOpeningAttemptId(value: unknown): value is string {
  return typeof value === "string" && /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/u.test(value);
}

function documentRuntimeKeyForConfig(config: ArtifactActorConfig): string {
  const hub = hubBinding(config);
  return hub === null ? documentRuntimeKeyV1({ kind: "local", documentId: config.documentId }) : documentRuntimeKeyV1({ kind: "hub", spaceId: hub.spaceId, documentId: config.documentId });
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
  if (request.kind === "browser-actor-ui-patch-result" || request.kind === "browser-actor-view-state") {
    typescriptDispatch(request);
    return;
  }
  if (request.kind === "open") {
    const clientInstanceId = request.clientInstanceId ?? crypto.randomUUID();
    if (!validDocumentOpeningAttemptId(clientInstanceId)) return;
    const next: DocumentExecutionOwner = hubBinding(request) === null && host !== null ? "rust" : "typescript";
    const runtimeKey = documentRuntimeKeyForConfig(request);
    const previous = documentExecutionOwners.get(runtimeKey);
    if (previous?.clientInstanceId === clientInstanceId) return;
    if (previous !== undefined) {
      const close: BackboneWorkerRequest = { kind: "close", documentId: request.documentId, clientInstanceId: previous.clientInstanceId, ...(previous.spaceId === undefined ? {} : { spaceId: previous.spaceId }) };
      if (previous.owner === "typescript") typescriptDispatch(close);
      else rustDispatch(close);
    }
    const hub = hubBinding(request);
    const openedRequest: BackboneWorkerRequest = { ...request, clientInstanceId };
    documentExecutionOwners.set(runtimeKey, { owner: next, documentId: request.documentId, clientInstanceId, ...(hub === null ? {} : { spaceId: hub.spaceId }) });
    if (next === "typescript") typescriptDispatch(openedRequest);
    else rustDispatch(openedRequest);
    return;
  }
  if (request.kind === "send" || request.kind === "close") {
    const runtimeKey = ownedDocumentRuntimeKey(request.documentId, request.spaceId);
    if (runtimeKey === null) return;
    const current = documentExecutionOwners.get(runtimeKey);
    if (current !== undefined && request.clientInstanceId !== current.clientInstanceId) return;
    const owner = current?.owner ?? (host === null ? "typescript" : "rust");
    if (owner === "typescript") typescriptDispatch(request);
    else rustDispatch(request);
    if (request.kind === "close" && current !== undefined && request.clientInstanceId === current.clientInstanceId) documentExecutionOwners.delete(runtimeKey);
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
const CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH = `${FOLDER_ENDPOINT_PATH}/canonical-bootstrap`;
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
  executionTargetOpen: symbol | null;
  executionTargetLease: DocumentExecutionTargetLease | null;
  browserActorReservation: DocumentBrowserActorReservation | null;
  browserActorViewState: ResolvedPluginViewState | null;
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
  /** 🪢️ Exact causal bytes retained beside bound-port mutations until their terminal Ack. */
  exactLocalEnvelopes: WeakMap<MutationEnvelope, Readonly<{ envelope: ExactWireMutationEnvelope; bytes: number; messages: number }>>;
  pendingDocumentBackboneBytes: number;
  pendingDocumentBackboneMessages: number;
  status: ArtifactSyncStatus;
  /** 🏔️ Last frontier the hub reported (`Welcome.server_frontier` / `Commands.frontier` /
   * `Ack.frontier`) — the wire-v2 replacement for the old `sinceVersion: number` counter. */
  frontier: WireFrontierSummary | null;
  pendingResumeToken: string | null;
  requiredTailFrontier: WireFrontierSummary | null;
  artifactBootstrap: ArtifactBootstrapAssembler | null;
  artifactBootstrapOwner: DocumentArtifactBootstrapOwner | null;
  artifactBootstrapDeadlineMs: number | null;
  artifactBootstrapDeadlineTimer: ReturnType<typeof setTimeout> | null;
  artifactRebootstrapOwner: DocumentArtifactRebootstrapOwner | null;
  artifactRebootstrapDeadlineMs: number | null;
  artifactRebootstrapDeadlineTimer: ReturnType<typeof setTimeout> | null;
  artifactRebootstrapRequired: boolean;
  artifactBootstrapProgress: ArtifactBootstrapProgress[];
  canonicalFolderMirror: FolderCanonicalBootstrapMirrorOwner | null;
  verifiedColdPair: VerifiedColdDocumentPair | null;
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
    const verified = authority !== null && authority.socket === state.socket && scope !== undefined && authority.scope.spaceId === scope.spaceId && authority.scope.documentId === scope.documentId;
    post({
      kind: "event",
      documentId: state.config.documentId,
      clientInstanceId: state.openClientInstanceId,
      event: verified ? event : { ...event, peers: [] },
      ...(scope === undefined ? {} : { scope }),
      ...(verified ? { verifiedSurfaceId: authority.verifiedSurfaceId } : {}),
    });
    return;
  }
  post({ kind: "event", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, event, ...(scope === undefined ? {} : { scope }) });
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
  localBrowserBrokerQueue = new Promise<void>((resolve) => {
    resolveTurn = resolve;
  });
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
      const response = await fetchWithTimeout(
        input,
        {
          ...init,
          headers: { ...(init.headers as Record<string, string> | undefined), "x-semio-browser-broker": currentHex, "x-semio-browser-broker-next": bytesHex(nextDigest) },
        },
        options,
      );
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
    void browserBrokerFetch("/_semio/hub/auth/sessions/me", { method: "GET" }, { timeoutMs: 2_000, signal: controller.signal })
      .then(async (response) => {
        const body = await response.text();
        const result: BrowserBrokerPortResponseV1 = { kind: "response", requestId, status: response.status, body };
        port.postMessage(result);
      })
      .catch((error: unknown) => {
        const result: BrowserBrokerPortResponseV1 = { kind: "response", requestId, status: error instanceof Error && error.message === "browser broker rebootstrap required" ? 428 : 503, body: "" };
        port.postMessage(result);
      })
      .finally(() => localBrowserBrokerRpcControllers.delete(requestId));
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

async function readDocumentOpenJson(response: FetchTimeoutResponse, control: ExecutionTargetReadControl): Promise<unknown> {
  return readExecutionTargetJson(response, DOCUMENT_OPEN_RESPONSE_MAX_BYTES, control);
}

//#region 🪪️ExecutionTargetLease
/** 📶️ Bounded streaming progress unit for a verified execution-target body. */
const EXECUTION_TARGET_PROGRESS_UNIT_BYTES = 64 * 1024;
const COLD_DOCUMENT_PAIR_PAGE_BYTES = 64 * 1024;
const COLD_DOCUMENT_PAIR_MAXIMUM_PAGES = 64;
const COLD_DOCUMENT_PAIR_MAXIMUM_BYTES = 4 * 1024 * 1024;
/** 🧯️ Bound on the strict lease manifest JSON body. */
const EXECUTION_TARGET_MANIFEST_MAX_BYTES = 8 * 1024;

type DocumentExecutionTargetAssetV1 = "manifest" | "component" | "descriptor" | "browser-actor";

const documentExecutionTargetLeaseMintToken = Symbol("semio.os.document-execution-target-lease.mint/v1");
const documentExecutionTargetLeaseBrand = Symbol("semio.os.document-execution-target-lease/v1");
const verifiedColdDocumentPairMintToken = Symbol("semio.os.verified-cold-document-pair.mint/v1");
let verifiedColdDocumentPairGeneration = 0n;

class VerifiedColdDocumentPair {
  readonly transferGeneration: bigint;
  readonly pageCount: number;
  readonly frontier: ColdDocumentPairFrontier;
  private readonly runtimeKey: string;
  private readonly config: ArtifactActorConfig;
  private readonly socket: WebSocket | null;
  private readonly clientInstanceId: string;
  private readonly lease: DocumentExecutionTargetLease;
  private readonly publishedPack: Uint8Array;
  private readonly publishedSpr: Uint8Array;
  private readonly descriptorSha256: Uint8Array;
  private readonly packSha256: Uint8Array;
  private readonly sprSha256: Uint8Array;
  private readonly aggregateSha256: Uint8Array;
  private pack: Uint8Array | null;
  private spr: Uint8Array | null;

  constructor(
    token: symbol,
    private readonly state: ArtifactState,
    lease: DocumentExecutionTargetLease,
    bootstrap: WireArtifactBootstrap,
    pair: Readonly<{ pack: Uint8Array; spr: Uint8Array }>,
    published: Readonly<{ pack: Uint8Array; spr: Uint8Array }>,
  ) {
    if (token !== verifiedColdDocumentPairMintToken || verifiedColdDocumentPairGeneration === 0xffffffffffffffffn) throw new Error("cold document pair: private owner");
    const total = pair.pack.byteLength + pair.spr.byteLength;
    if (pair.pack.byteLength !== bootstrap.pack_length || pair.spr.byteLength !== bootstrap.spr_length || total < 2 || total > COLD_DOCUMENT_PAIR_MAXIMUM_BYTES || Math.ceil(total / COLD_DOCUMENT_PAIR_PAGE_BYTES) > COLD_DOCUMENT_PAIR_MAXIMUM_PAGES)
      throw new Error("cold document pair: invalid capacity");
    const fields = lease.fields(),
      checkpoint = fields.checkpoint;
    if (
      fields.browserActor.kind !== "closed-browser-actor" ||
      !checkpoint ||
      checkpoint.descriptorDigestV1 !== fields.descriptorDigestV1 ||
      checkpoint.aggregateSha256 !== executionTargetHex(new Uint8Array(bootstrap.aggregate_hash)) ||
      fields.descriptorDigestV1 !== executionTargetHex(new Uint8Array(bootstrap.descriptor_hash))
    )
      throw new Error("cold document pair: invalid lease");
    this.transferGeneration = ++verifiedColdDocumentPairGeneration;
    this.pageCount = Math.ceil(total / COLD_DOCUMENT_PAIR_PAGE_BYTES);
    this.frontier = Object.freeze({
      documentId: bootstrap.baseline_frontier.document_id,
      headEditOrdinal: BigInt(bootstrap.baseline_frontier.head_edit_ordinal),
      headEditId: bootstrap.baseline_frontier.head_edit_id,
      lastCommitSeq: BigInt(bootstrap.baseline_frontier.last_commit_seq),
      chainSha256: Uint8Array.from(bootstrap.baseline_frontier.chain_hash),
    });
    this.runtimeKey = state.runtimeKey;
    this.config = state.config;
    this.socket = state.socket;
    this.clientInstanceId = state.openClientInstanceId;
    this.lease = lease;
    this.publishedPack = published.pack;
    this.publishedSpr = published.spr;
    this.descriptorSha256 = Uint8Array.from(bootstrap.descriptor_hash);
    this.packSha256 = Uint8Array.from(bootstrap.pack_hash);
    this.sprSha256 = Uint8Array.from(bootstrap.spr_hash);
    this.aggregateSha256 = Uint8Array.from(bootstrap.aggregate_hash);
    this.pack = Uint8Array.from(pair.pack);
    this.spr = Uint8Array.from(pair.spr);
    Object.freeze(this.frontier);
  }

  assertCurrent(): void {
    const pack = this.pack,
      spr = this.spr;
    if (
      !pack ||
      !spr ||
      this.state.closed ||
      this.state.docAbort.signal.aborted ||
      this.state.verifiedColdPair !== this ||
      this.state.config !== this.config ||
      this.state.runtimeKey !== this.runtimeKey ||
      artifacts.get(this.runtimeKey) !== this.state ||
      this.state.socket !== this.socket ||
      this.state.openClientInstanceId !== this.clientInstanceId ||
      this.state.executionTargetLease !== this.lease ||
      !this.lease.live ||
      this.state.currentPack !== this.publishedPack ||
      this.state.currentSpr !== this.publishedSpr ||
      !this.state.frontier ||
      !equalFrontiers(this.state.frontier, {
        document_id: this.frontier.documentId,
        head_edit_ordinal: Number(this.frontier.headEditOrdinal),
        head_edit_id: this.frontier.headEditId,
        last_commit_seq: Number(this.frontier.lastCommitSeq),
        chain_hash: Array.from(this.frontier.chainSha256),
      })
    )
      throw new Error("cold document pair: stale owner");
    documentBrowserActorLease(this.state);
  }

  page(lifetime: ActorInstanceLifetime, pageIndex: number): BrowserActorChildValue {
    this.assertCurrent();
    const pack = this.pack!,
      spr = this.spr!;
    if (!Number.isInteger(pageIndex) || pageIndex < 0 || pageIndex >= this.pageCount) throw new Error("cold document pair: invalid cursor");
    const start = pageIndex * COLD_DOCUMENT_PAIR_PAGE_BYTES;
    const end = Math.min(start + COLD_DOCUMENT_PAIR_PAGE_BYTES, pack.byteLength + spr.byteLength);
    let bytes: Uint8Array;
    if (end <= pack.byteLength) bytes = pack.slice(start, end);
    else if (start >= pack.byteLength) bytes = spr.slice(start - pack.byteLength, end - pack.byteLength);
    else {
      bytes = new Uint8Array(end - start);
      bytes.set(pack.subarray(start), 0);
      bytes.set(spr.subarray(0, end - pack.byteLength), pack.byteLength - start);
    }
    return {
      header: {
        lifetime: { ...lifetime },
        transferGeneration: this.transferGeneration,
        descriptorSha256: Array.from(this.descriptorSha256),
        baselineFrontier: {
          documentId: this.frontier.documentId,
          headEditOrdinal: this.frontier.headEditOrdinal,
          headEditId: this.frontier.headEditId,
          lastCommitSeq: this.frontier.lastCommitSeq,
          chainSha256: Array.from(this.frontier.chainSha256),
        },
        packSha256: Array.from(this.packSha256),
        sprSha256: Array.from(this.sprSha256),
        aggregateSha256: Array.from(this.aggregateSha256),
        packLength: BigInt(pack.byteLength),
        sprLength: BigInt(spr.byteLength),
        pageCount: this.pageCount,
      },
      pageIndex,
      bytes,
    };
  }

  assertApplied(status: ColdPairIngressStatus, lifetime: ActorInstanceLifetime): void {
    this.assertCurrent();
    if (
      status.kind !== "applied" ||
      !actorInstanceLifetimeEquals(status.receipt.lifetime, lifetime) ||
      status.receipt.transferGeneration !== this.transferGeneration ||
      !coldDocumentPairFrontierEquals(status.receipt.baselineFrontier, this.frontier) ||
      !equalByteArrays(status.receipt.aggregateSha256, this.aggregateSha256)
    )
      throw new Error("cold document pair: invalid applied receipt");
  }

  drop(): void {
    this.pack?.fill(0);
    this.spr?.fill(0);
    this.pack = null;
    this.spr = null;
  }
}

function dropVerifiedColdDocumentPair(state: ArtifactState): void {
  const owner = state.verifiedColdPair;
  state.verifiedColdPair = null;
  owner?.drop();
}

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
  #retirement = new AbortController();
  #browserActorGrant: DocumentBrowserActorGrant | null = null;
  #browserActorOpen: DocumentBrowserActorOpen | null = null;
  #surfaceBodyKey: string | null = null;

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

  renderBodyKey(): string {
    if (!this.#live || !this.#descriptor) throw new Error("document browser actor: dropped render descriptor");
    this.#surfaceBodyKey ??= parseVerifiedPackageDescriptorV1(this.#descriptor, this.#fields);
    return this.#surfaceBodyKey;
  }

  get retirement(): AbortSignal {
    return this.#retirement.signal;
  }

  admitBrowserActor(token: symbol, receipt: SocketGrantReceiptV1, retireAtMs: number, open: DocumentBrowserActorOpen): void {
    if (token !== documentExecutionTargetLeaseMintToken || !this.#live || this.#browserActorGrant !== null || !Number.isSafeInteger(retireAtMs)) throw new Error("document browser actor: private grant");
    const parsed = parseSocketGrantReceiptV1(receipt);
    if (parsed.expiresAtMs > retireAtMs) throw new Error("document browser actor: invalid grant");
    open.assertCurrent();
    this.#browserActorOpen = Object.freeze({ binding: structuredClone(open.binding), intent: structuredClone(open.intent), assertCurrent: open.assertCurrent });
    this.#browserActorGrant = Object.freeze({ actorId: parsed.actorId, reserveBeforeMs: parsed.expiresAtMs, retireAtMs });
  }

  assertBrowserActorCurrent(): void {
    if (!this.#live || !this.#browserActorOpen) throw new Error("document browser actor: retired open owner");
    this.#browserActorOpen.assertCurrent();
  }

  assertBrowserActorDescribeCapacity(): void {
    if (!this.#live || !this.#descriptor) throw new Error("document browser actor: dropped descriptor");
    assertBrowserActorDescribeCapacityV1(this.#descriptor.byteLength);
  }

  async activateBrowserActor(child: DocumentBrowserActorChild, signal: AbortSignal, assertSession: () => void, report: (progress: DocumentExecutionTargetProgressV1) => void): Promise<void> {
    const open = this.#browserActorOpen,
      grant = this.#browserActorGrant,
      actor = this.#fields.browserActor;
    if (!open || !grant || actor.kind !== "closed-browser-actor") throw new Error("document browser actor: missing private authority");
    const assertCurrent = () => {
      assertSession();
      open.assertCurrent();
      if (!this.#live || this.#browserActorOpen !== open || this.#browserActorGrant !== grant) throw new Error("document browser actor: retired authority");
    };
    const control: ExecutionTargetReadControl = { signal, deadlineAtMs: Math.min(grant.reserveBeforeMs, grant.retireAtMs, Date.now() + SOCKET_GRANT_REQUEST_TIMEOUT_MS), assertCurrent };
    let source: Uint8Array | undefined, result: BrowserActorChildValue | undefined;
    try {
      assertExecutionTargetRead(control);
      this.assertBrowserActorDescribeCapacity();
      const response = await browserExecutionTargetAssetRequest(open.binding, open.intent.scope.documentId, "browser-actor", open.intent, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal });
      source = await readExecutionTargetBody(response, actor.byteLength, DOCUMENT_BROWSER_ACTOR_MAX_BYTES, "browser-actor", control, report);
      assertExecutionTargetRead(control);
      if ((await executionTargetSha256Hex(source)) !== actor.sha256) throw new Error("document browser actor: body integrity");
      assertExecutionTargetRead(control);
      await child.load(source.buffer as ArrayBuffer);
      assertExecutionTargetRead(control);
      result = await child.invoke(["describe", "describe"], []);
      assertExecutionTargetRead(control);
      if (!(result instanceof Uint8Array) || !(result.buffer instanceof ArrayBuffer) || result.byteOffset !== 0 || result.byteLength !== result.buffer.byteLength || !this.#descriptor) throw new Error("document browser actor: describe shape");
      verifyBrowserActorDescribeV1(result, this.#descriptor, { decode: decodePackValue, encode: encodePackValue });
      assertExecutionTargetRead(control);
    } finally {
      if (source?.byteLength) source.fill(0);
      if (result !== undefined) for (const buffer of measureChildValue(result, BROWSER_ACTOR_CHILD_LIMITS.outputBytes).transfers) new Uint8Array(buffer).fill(0);
    }
  }

  browserActorGrant(): DocumentBrowserActorGrant | null {
    return this.#live ? this.#browserActorGrant : null;
  }

  drop(): void {
    if (!this.#live) return;
    this.#live = false;
    this.#retirement.abort();
    this.#browserActorGrant = null;
    this.#browserActorOpen = null;
    this.#component?.fill(0);
    this.#descriptor?.fill(0);
    this.#component = null;
    this.#descriptor = null;
    this.#surfaceBodyKey = null;
    if (this.#moduleUrl !== null) {
      URL.revokeObjectURL(this.#moduleUrl);
      this.#moduleUrl = null;
    }
  }
}

/** 🧾️ Receipt-free lease projection of one parsed plan, at the byte lengths under comparison. */
function receiptFreeFields(plan: DocumentOpenPlanV1, byteLengths: { readonly component: number; readonly descriptor: number; readonly browserActor?: number }): DocumentExecutionTargetLeaseFieldsV1 {
  return leaseFieldsFromPlanV1(plan, byteLengths);
}

function executionTargetHex(bytes: Uint8Array): string {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

async function executionTargetSha256Hex(bytes: Uint8Array): Promise<string> {
  return executionTargetHex(new Uint8Array(await crypto.subtle.digest("SHA-256", ownedArrayBuffer(bytes))));
}

function ownedArrayBuffer(bytes: Uint8Array): ArrayBuffer {
  const owned = new Uint8Array(bytes.byteLength);
  owned.set(bytes);
  return owned.buffer;
}

function executionTargetAssetPath(spaceId: string, documentId: string, asset: DocumentExecutionTargetAssetV1): string {
  return `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/execution-target/${asset}`;
}

/** 🚪️ The only broker operation this lane owns: exactly the four protected document-scoped asset
 * calls, always POST, always the bounded `DocumentOpenIntentV1` body, never a package, digest,
 * generation, path or receipt selector. Anything else is denied before a request exists. */
function browserExecutionTargetAssetRequest(
  binding: Extract<PersistenceBinding, { kind: "hub" }>,
  documentId: string,
  asset: DocumentExecutionTargetAssetV1,
  intent: DocumentOpenIntentV1,
  options: { readonly timeoutMs: number; readonly signal: AbortSignal },
): Promise<FetchTimeoutResponse> {
  const path = executionTargetAssetPath(binding.spaceId, documentId, asset);
  if (!/^\/spaces\/[^/?#]+\/documents\/[^/?#]+\/execution-target\/(?:manifest|component|descriptor|browser-actor)$/u.test(path)) return Promise.reject(new Error("document execution target: operation denied"));
  if (intent.scope.spaceId !== binding.spaceId || intent.scope.documentId !== documentId) return Promise.reject(new Error("document execution target: operation denied"));
  return browserBrokerFetch(`/_semio/hub${path}`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(intent) }, options);
}

/** 🔭️ Observation seam for the execution-target live region, mirroring {@link socketGrantTestIssue}:
 * a worker scope posts, and a harness without one still sees the exact bounded payload. */
let executionTargetStatusObserver: ((status: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }>) => void) | null = null;

function emitExecutionTargetStatus(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub" }>, code: DocumentExecutionTargetStatusCodeV1, progress?: DocumentExecutionTargetProgressV1): void {
  const scope = { spaceId: binding.spaceId, documentId: state.config.documentId };
  const status: Extract<BackboneWorkerResponse, { kind: "execution-target-status" }> = { kind: "execution-target-status", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, spaceId: binding.spaceId, scope, code, ...(progress ? { progress } : {}) };
  executionTargetStatusObserver?.(status);
  post(status);
}

type ExecutionTargetReadControl = Readonly<{ signal: AbortSignal; deadlineAtMs: number; assertCurrent(): void }>;
type ExecutionTargetBodyResponse = FetchTimeoutResponse & { readonly body?: ReadableStream<Uint8Array> | null };

function isExecutionTargetByteChunk(value: unknown): value is Uint8Array {
  return ArrayBuffer.isView(value) && Object.prototype.toString.call(value) === "[object Uint8Array]";
}

function assertExecutionTargetRead(control: ExecutionTargetReadControl): void {
  if (control.signal.aborted || !Number.isSafeInteger(control.deadlineAtMs) || Date.now() >= control.deadlineAtMs) throw new Error("document execution target: cancelled or expired");
  control.assertCurrent();
}

/** 🧹️ Owns one bounded body through EOF, cancellation and stale-read retirement. */
async function readBoundedExecutionTargetBody(response: FetchTimeoutResponse, expected: number | null, maximum: number, control: ExecutionTargetReadControl, report: (received: number, total: number) => void): Promise<Uint8Array> {
  const body = (response as ExecutionTargetBodyResponse).body;
  if (!(body instanceof ReadableStream)) throw new Error("document execution target: invalid body");
  const reader = body.getReader();
  let bytes: Uint8Array | null = null,
    cancelled = false,
    timer: ReturnType<typeof setTimeout> | undefined;
  let rejectStop: (error: Error) => void = () => {};
  const stop = new Promise<never>((_resolve, reject) => {
    rejectStop = reject;
  });
  void stop.catch(() => {});
  const abort = (): void => rejectStop(new Error("document execution target: cancelled or expired"));
  const cancel = (): void => {
    if (cancelled) return;
    cancelled = true;
    void reader.cancel().catch(() => {});
  };
  try {
    const header = response.headers.get("content-length");
    const declared = header === null ? null : Number(header);
    if (
      !response.ok ||
      !Number.isSafeInteger(maximum) ||
      maximum < 1 ||
      (declared !== null && (!Number.isSafeInteger(declared) || declared < 1 || declared > maximum)) ||
      (expected !== null && (declared !== expected || !Number.isSafeInteger(expected) || expected < 1 || expected > maximum))
    )
      throw new Error("document execution target: invalid body");
    assertExecutionTargetRead(control);
    control.signal.addEventListener("abort", abort, { once: true });
    timer = setTimeout(abort, Math.min(0x7fffffff, Math.max(0, control.deadlineAtMs - Date.now())));
    assertExecutionTargetRead(control);
    bytes = new Uint8Array(declared ?? maximum);
    let received = 0,
      announced = 0;
    for (;;) {
      assertExecutionTargetRead(control);
      const slot: { state: "pending" | "claimed" | "abandoned"; result?: ReadableStreamReadResult<Uint8Array> } = { state: "pending" };
      const pending = reader.read().then((result) => {
        slot.result = result;
        if (slot.state === "abandoned" && isExecutionTargetByteChunk(result.value)) result.value.fill(0);
        return result;
      });
      let result: ReadableStreamReadResult<Uint8Array>;
      try {
        result = await Promise.race([pending, stop]);
        slot.state = "claimed";
      } catch (error) {
        slot.state = "abandoned";
        if (isExecutionTargetByteChunk(slot.result?.value)) slot.result.value.fill(0);
        throw error;
      }
      try {
        assertExecutionTargetRead(control);
        if (result.done) break;
        if (!isExecutionTargetByteChunk(result.value) || result.value.byteLength > bytes.length - received) throw new Error("document execution target: invalid body");
        bytes.set(result.value, received);
        received += result.value.byteLength;
        if (received - announced >= EXECUTION_TARGET_PROGRESS_UNIT_BYTES) {
          announced = received;
          report(received, declared ?? maximum);
        }
      } finally {
        if (isExecutionTargetByteChunk(result.value)) result.value.fill(0);
      }
    }
    assertExecutionTargetRead(control);
    if (received === 0 || (declared !== null && received !== declared)) throw new Error("document execution target: invalid body");
    report(received, declared ?? received);
    assertExecutionTargetRead(control);
    if (declared === null) return bytes.slice(0, received);
    const owned = bytes;
    bytes = null;
    return owned;
  } catch (error) {
    cancel();
    throw error;
  } finally {
    clearTimeout(timer);
    control.signal.removeEventListener("abort", abort);
    bytes?.fill(0);
    reader.releaseLock();
  }
}

/** 📥️ Decodes the bounded manifest and wipes its temporary body owner. */
async function readExecutionTargetJson(response: FetchTimeoutResponse, maximum: number, control: ExecutionTargetReadControl): Promise<unknown> {
  const bytes = await readBoundedExecutionTargetBody(response, null, maximum, control, () => {});
  try {
    assertExecutionTargetRead(control);
    return JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
  } finally {
    bytes.fill(0);
  }
}

/** 📥️ Acquires exactly one declared asset body with bounded progress. */
async function readExecutionTargetBody(
  response: FetchTimeoutResponse,
  expectedByteLength: number,
  maxBytes: number,
  stage: "component" | "descriptor" | "browser-actor",
  control: ExecutionTargetReadControl,
  report: (progress: DocumentExecutionTargetProgressV1) => void,
): Promise<Uint8Array> {
  return readBoundedExecutionTargetBody(response, expectedByteLength, maxBytes, control, (completedBytes, totalBytes) => report({ stage, completedBytes, totalBytes }));
}

/** 📜️ Strictly admits the raw descriptor bytes: canonical re-encoding equality first, then exact
 * equality between the decoded package descriptor and the verified lease fields. A sibling JSON
 * manifest or a caller-selected URL is never descriptor authority. */
function parseVerifiedPackageDescriptorV1(bytes: Uint8Array, fields: DocumentExecutionTargetLeaseFieldsV1): string {
  const decoded = decodePackValue(bytes);
  const canonical = encodePackValue(decoded);
  if (canonical.length !== bytes.length || canonical.some((byte, index) => byte !== bytes[index])) throw new Error("document execution target: descriptor is not canonical");
  const record = (value: PackValue): Record<string, PackValue> => {
    if (value === null || typeof value !== "object" || Array.isArray(value) || isPackInteger(value)) throw new Error("document execution target: descriptor invalid");
    return value as Record<string, PackValue>;
  };
  const descriptor = record(decoded);
  const executionProtocol = record(descriptor.executionProtocol);
  const manifest = record(descriptor.manifest);
  const hashes = record(descriptor.hashes);
  const apps = Array.isArray(manifest.apps) ? (manifest.apps as readonly PackValue[]).map(record) : [];
  const artifactKinds = Array.isArray(manifest.artifactKinds) ? (manifest.artifactKinds as readonly PackValue[]).map(record) : [];
  const app = apps.find((entry) => entry.id === fields.surface.appId);
  const dialect = app === undefined ? undefined : record(app.dialect);
  const windowKinds = app !== undefined && Array.isArray(app.windowKinds) ? (app.windowKinds as readonly PackValue[]).map(record) : [];
  const window = windowKinds.find((window) => window.id === fields.surface.windowKindId);
  if (
    packUIntSafeOrNull(descriptor.descriptorVersion) !== 1 ||
    descriptor.packageId !== fields.package.packageId ||
    descriptor.execution !== "isolated" ||
    Object.keys(executionProtocol).length !== 1 ||
    packUIntSafeOrNull(executionProtocol.appChannelVersion) !== DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 ||
    packUIntSafeOrNull(executionProtocol.appChannelVersion) !== fields.package.executionProtocol.appChannelVersion ||
    manifest.pluginId !== fields.package.pluginId ||
    manifest.version !== fields.package.version ||
    hashes.wasmSha256 !== fields.component.sha256 ||
    app === undefined ||
    dialect === undefined ||
    app.id !== fields.surface.surfaceId ||
    app.role !== fields.surface.role ||
    dialect.artifactKind !== fields.parentDialect.artifactKind ||
    dialect.standard !== fields.parentDialect.standard ||
    dialect.subset !== fields.parentDialect.subset ||
    window === undefined || typeof window.bodyKey !== "string" || window.bodyKey.length === 0 || window.bodyKey.length > 256 ||
    !artifactKinds.some((kind) => kind.id === fields.artifact.kind && kind.schema === fields.artifact.schema)
  )
    throw new Error("document execution target: descriptor mismatch");
  return window.bodyKey as string;
}

/** 🛡️ Acquires the server-selected verified execution target for one live plan and mints the private
 * lease only after every byte verifies. It shares {@link ArtifactState.docAbort} and the current
 * document-open deadline with {@link requestDocumentSocketAuthority}; a mismatch, cancellation or
 * deadline wipes every retained buffer and returns no lease. */
async function installDocumentExecutionTargetLease(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub" }>, plan: DocumentOpenPlanV1, intent: DocumentOpenIntentV1, assertOwner: () => void): Promise<DocumentExecutionTargetLease> {
  const signal = state.docAbort.signal;
  const options = { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal } as const;
  const readControl = (): ExecutionTargetReadControl => ({ signal, deadlineAtMs: Math.min(plan.expiresAtUnixMs, Date.now() + SOCKET_GRANT_REQUEST_TIMEOUT_MS), assertCurrent: assertOwner });
  const report = (progress: DocumentExecutionTargetProgressV1): void => emitExecutionTargetStatus(state, binding, "verifying", progress);
  let component: Uint8Array | null = null;
  let descriptorBytes: Uint8Array | null = null;
  try {
    if (signal.aborted) throw new Error("document execution target: cancelled");
    report({ stage: "manifest", completedBytes: 0, totalBytes: 1 });
    const manifestControl = readControl();
    assertExecutionTargetRead(manifestControl);
    const manifestResponse = await browserExecutionTargetAssetRequest(binding, state.config.documentId, "manifest", intent, options);
    const fields = parseDocumentExecutionTargetLeaseFieldsV1(await readExecutionTargetJson(manifestResponse, EXECUTION_TARGET_MANIFEST_MAX_BYTES, manifestControl));
    if (
      !sameLeaseFieldsV1(
        fields,
        receiptFreeFields(plan, { component: fields.component.byteLength, descriptor: fields.descriptor.byteLength, browserActor: fields.browserActor.kind === "closed-browser-actor" ? fields.browserActor.byteLength : undefined }),
      )
    )
      throw new Error("document execution target: manifest mismatch");
    report({ stage: "manifest", completedBytes: 1, totalBytes: 1 });

    if (signal.aborted) throw new Error("document execution target: cancelled");
    const componentControl = readControl();
    assertExecutionTargetRead(componentControl);
    const componentResponse = await browserExecutionTargetAssetRequest(binding, state.config.documentId, "component", intent, options);
    component = await readExecutionTargetBody(componentResponse, fields.component.byteLength, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, "component", componentControl, report);

    if (signal.aborted) throw new Error("document execution target: cancelled");
    const descriptorControl = readControl();
    assertExecutionTargetRead(descriptorControl);
    const descriptorResponse = await browserExecutionTargetAssetRequest(binding, state.config.documentId, "descriptor", intent, options);
    descriptorBytes = await readExecutionTargetBody(descriptorResponse, fields.descriptor.byteLength, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, "descriptor", descriptorControl, report);

    if (signal.aborted) throw new Error("document execution target: cancelled");
    const verifyControl = readControl();
    assertExecutionTargetRead(verifyControl);
    report({ stage: "verify", completedBytes: 0, totalBytes: 3 });
    if ((await executionTargetSha256Hex(component)) !== fields.component.sha256) throw new Error("document execution target: component integrity");
    assertExecutionTargetRead(verifyControl);
    report({ stage: "verify", completedBytes: 1, totalBytes: 3 });
    if (blake3Hex(component) !== fields.component.blake3) throw new Error("document execution target: component integrity");
    assertExecutionTargetRead(verifyControl);
    report({ stage: "verify", completedBytes: 2, totalBytes: 3 });
    if ((await executionTargetSha256Hex(descriptorBytes)) !== fields.descriptor.sha256) throw new Error("document execution target: descriptor integrity");
    parseVerifiedPackageDescriptorV1(descriptorBytes, fields);
    assertExecutionTargetRead(verifyControl);
    report({ stage: "verify", completedBytes: 3, totalBytes: 3 });
    if (signal.aborted) throw new Error("document execution target: cancelled");
    assertExecutionTargetRead(verifyControl);
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
  dropVerifiedColdDocumentPair(state);
  const mirror = state.canonicalFolderMirror;
  state.canonicalFolderMirror = null;
  if (mirror) void retireFolderCanonicalBootstrapMirror(mirror);
  state.browserActorReservation?.close();
  state.executionTargetLease?.drop();
  state.executionTargetLease = null;
}

type DocumentBrowserActorChild = Awaited<ReturnType<typeof reserveBrowserActorChild>>;
type DocumentBrowserActorOpen = Readonly<{ binding: Extract<PersistenceBinding, { kind: "hub" }>; intent: DocumentOpenIntentV1; assertCurrent(): void }>;
type DocumentBrowserActorGrant = Readonly<{ actorId: string; reserveBeforeMs: number; retireAtMs: number }>;
let documentBrowserActorGeneration = 0n;
const DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT = 256;

function browserActorRecord(value: BrowserActorChildValue, code: string): Record<string, BrowserActorChildValue> {
  if (!value || typeof value !== "object" || Array.isArray(value) || value instanceof Uint8Array || value instanceof ArrayBuffer) throw new Error(code);
  return value;
}

function unwrapBrowserActorOption(value: BrowserActorChildValue | undefined): BrowserActorChildValue | undefined {
  if (value === null || value === undefined) return undefined;
  const record = browserActorRecord(value, "document browser actor: invalid option");
  if (record.tag === "none" && Object.keys(record).length === 1) return undefined;
  if (record.tag === "some" && Object.keys(record).length === 2 && Object.hasOwn(record, "val")) return record.val;
  return value;
}

function browserActorTurnResult(value: BrowserActorChildValue): Record<string, BrowserActorChildValue> {
  const result = browserActorRecord(value, "document browser actor: invalid turn result");
  const status = browserActorRecord(result.status, "document browser actor: invalid turn status");
  if (status.tag === "faulted") throw new Error("document browser actor: guest turn fault");
  if (status.tag !== "idle" && status.tag !== "more-work" && status.tag !== "checkpoint-ready") throw new Error("document browser actor: invalid turn status");
  return result;
}

function browserActorCapturedReceipt(value: BrowserActorChildValue, request: ActorInstanceOpenRequest): ActorInstanceLifecycleReceipt {
  const result = browserActorTurnResult(value);
  const raw = unwrapBrowserActorOption(result.lifecycleReceipt);
  const tagged = browserActorRecord(raw ?? null, "document browser actor: missing captured receipt");
  const body = browserActorRecord(tagged.val, "document browser actor: invalid captured receipt");
  const sequence = body.requestSequence;
  if (tagged.tag !== "captured" || typeof sequence !== "bigint" || sequence < 1n || sequence > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("document browser actor: invalid captured receipt");
  const receipt: ActorInstanceLifecycleReceipt = { kind: "captured", lifetime: parseColdDocumentPairLifetime(body.lifetime), requestSequence: Number(sequence) };
  if (!actorInstanceCapturedReceiptMatches(request, receipt)) throw new Error("document browser actor: captured receipt mismatch");
  return receipt;
}

function browserActorColdStatus(value: BrowserActorChildValue, allowLifecycleReceipt = false): ColdPairIngressStatus {
  const result = browserActorTurnResult(value);
  if (!allowLifecycleReceipt && unwrapBrowserActorOption(result.lifecycleReceipt) !== undefined) throw new Error("document browser actor: unexpected lifecycle receipt");
  return parseWitColdPairIngressStatus(result.coldPairIngress);
}

function wipeBrowserActorValue(value: BrowserActorChildValue): void {
  for (const buffer of measureChildValue(value, BROWSER_ACTOR_CHILD_LIMITS.outputBytes).transfers) if (buffer.byteLength) new Uint8Array(buffer).fill(0);
}

function browserActorTurnBudget(): BrowserActorChildValue {
  return { fuel: 80_000_000n, deadlineMs: BROWSER_ACTOR_CHILD_LIMITS.invokeMs, maxEffects: 512, maxPatchBytes: 2_097_152, maxFrames: 8 };
}

/** 🧷️ Owns one document's reserved child; only the live private lease and exchanged grant select it. */
class DocumentBrowserActorReservation {
  readonly generation: bigint;
  private readonly abort = new AbortController();
  private child: DocumentBrowserActorChild | null = null;
  private activation: Promise<void> | null = null;
  private lifetime: ActorInstanceLifetime | null = null;
  private coldOwner: VerifiedColdDocumentPair | null = null;
  private coldApplied: VerifiedColdDocumentPair | null = null;
  private coldTransfer: Promise<void> | null = null;
  private socket: WebSocket | null = null;
  private pendingUiPatch: { readonly offer: BrowserActorUiPatchOfferV1; readonly resolve: (result: BrowserActorUiPatchResultV1) => void; readonly reject: (error: Error) => void; readonly timer: ReturnType<typeof setTimeout> } | null = null;
  private renderedUiPatch = false;
  private renderedViewState: ResolvedPluginViewState | null = null;
  private viewRefresh: Promise<void> | null = null;
  private closed = false;

  refreshHostView(): void {
    if (this.closed || !this.activation || this.viewRefresh) return;
    this.viewRefresh = (async () => {
      await this.activation;
      await this.coldTransfer;
      const child = this.child;
      if (!child || !this.coldApplied) return;
      const assertCurrent = () => {
        if (this.closed || this.state.socket !== this.socket || this.state.browserActorReservation !== this || documentBrowserActorLease(this.state) !== this.lease) throw new Error("document browser actor: stale host view");
        this.lease.assertBrowserActorCurrent();
      };
      while (this.state.browserActorViewState !== null && this.state.browserActorViewState !== this.renderedViewState) {
        assertCurrent();
        await this.renderSurface(child, assertCurrent);
      }
    })().catch(() => {
      if (this.closed) return;
      const binding = hubBinding(this.state.config);
      if (binding) emitExecutionTargetStatus(this.state, binding, "renderer-unavailable");
      this.close();
    }).finally(() => { this.viewRefresh = null; });
  }

  bindApprovalUndoIfMounted(): void {
    const owner = this.coldApplied;
    if (owner !== null && this.renderedUiPatch) bindInferenceApprovalUndoToMountedPair(this.state, this, owner);
  }
  private readonly timer: ReturnType<typeof setTimeout>;
  private readonly retire = () => this.close();

  constructor(
    private readonly state: ArtifactState,
    private readonly lease: DocumentExecutionTargetLease,
    private readonly grant: DocumentBrowserActorGrant,
  ) {
    if (documentBrowserActorGeneration === 0xffffffffffffffffn) throw new Error("document browser actor: generation exhausted");
    this.generation = ++documentBrowserActorGeneration;
    state.docAbort.signal.addEventListener("abort", this.retire, { once: true });
    lease.retirement.addEventListener("abort", this.retire, { once: true });
    this.timer = setTimeout(this.retire, Math.min(0x7fffffff, Math.max(0, grant.retireAtMs - Date.now())));
  }

  async reserve(): Promise<void> {
    const fields = this.lease.fields();
    if (fields.browserActor.kind !== "closed-browser-actor") throw new Error("document browser actor: unavailable");
    const child = await reserveBrowserActorChild({ actorId: this.grant.actorId, activationGeneration: this.generation, bundleSha256: fields.browserActor.sha256, bundleByteLength: fields.browserActor.byteLength }, this.abort.signal);
    this.child = child;
    if (this.closed || this.state.browserActorReservation !== this || documentBrowserActorLease(this.state) !== this.lease || this.lease.browserActorGrant() !== this.grant || Date.now() >= this.grant.reserveBeforeMs) {
      this.close();
      throw new Error("document browser actor: stale reservation");
    }
  }

  activate(socket: WebSocket): Promise<void> {
    if (this.activation) {
      if (this.socket !== socket) return Promise.reject(new Error("document browser actor: activation socket mismatch"));
      return this.activation;
    }
    this.socket = socket;
    this.activation = (async () => {
      const assertCurrent = () => {
        if (
          this.closed ||
          this.state.socket !== socket ||
          !this.state.hubActorReady ||
          this.state.actor !== this.grant.actorId ||
          this.state.pendingSocketActorId !== null ||
          this.state.browserActorReservation !== this ||
          documentBrowserActorLease(this.state) !== this.lease ||
          this.lease.browserActorGrant() !== this.grant ||
          Date.now() >= this.grant.reserveBeforeMs ||
          Date.now() >= this.grant.retireAtMs
        )
          throw new Error("document browser actor: stale Session");
        this.lease.assertBrowserActorCurrent();
      };
      assertCurrent();
      const child = this.child,
        binding = hubBinding(this.state.config);
      if (!child || !binding) throw new Error("document browser actor: unavailable child");
      await this.lease.activateBrowserActor(child, this.abort.signal, assertCurrent, (progress) => {
        assertCurrent();
        emitExecutionTargetStatus(this.state, binding, "verifying", progress);
      });
      assertCurrent();
      this.lifetime = await this.openGuest(child, assertCurrent);
      assertCurrent();
      const owner = this.state.verifiedColdPair;
      if (owner) await this.transferColdPair(owner, child, binding, assertCurrent);
      this.refreshHostView();
    })();
    return this.activation;
  }

  async installColdPair(owner: VerifiedColdDocumentPair): Promise<void> {
    if (this.closed) throw new Error("document browser actor: closed reservation");
    if (!this.lifetime || !this.child || !this.socket || !this.activation) return;
    await this.activation;
    await this.viewRefresh;
    const binding = hubBinding(this.state.config),
      socket = this.socket;
    if (!binding) throw new Error("document browser actor: missing hub binding");
    const assertCurrent = () => {
      if (this.closed || this.state.socket !== socket || this.state.browserActorReservation !== this || documentBrowserActorLease(this.state) !== this.lease) throw new Error("document browser actor: stale cold transfer");
      owner.assertCurrent();
    };
    await this.transferColdPair(owner, this.child, binding, assertCurrent);
    this.refreshHostView();
  }

  private async openGuest(child: DocumentBrowserActorChild, assertCurrent: () => void): Promise<ActorInstanceLifetime> {
    const fields = this.lease.fields();
    const request: ActorInstanceOpenRequest = { kind: "open", activationGeneration: this.generation, instanceId: 0, requestSequence: 1 };
    const config = new Uint8Array(0),
      quotas = new Uint8Array(0);
    let result: BrowserActorChildValue | null = null;
    try {
      result = await child.invoke(
        ["reactor", "poll"],
        [
          [
            {
              tag: "instance-open",
              val: {
                instance: request.instanceId,
                activationGeneration: request.activationGeneration,
                requestSequence: BigInt(request.requestSequence),
                appId: fields.surface.appId,
                actor: this.grant.actorId,
                config,
                assets: [],
                capabilities: [],
                quotas,
              },
            },
          ],
          null,
          null,
          browserActorTurnBudget(),
        ],
      );
      assertCurrent();
      const captured = browserActorCapturedReceipt(result, request);
      if (browserActorColdStatus(result, true).kind !== "idle") throw new Error("document browser actor: cold ingress before ACK");
      if (this.captureUiPatch(result, captured.lifetime) !== null) throw new Error("document browser actor: patch before lifecycle ACK");
      const acknowledged = await child.invoke(
        ["reactor", "poll"],
        [[{ tag: "instance-lifecycle-ack", val: { tag: "captured", val: { lifetime: { ...captured.lifetime }, requestSequence: BigInt(captured.requestSequence) } } }], null, null, browserActorTurnBudget()],
      );
      wipeBrowserActorValue(result);
      result = acknowledged;
      assertCurrent();
      if (browserActorColdStatus(acknowledged).kind !== "idle") throw new Error("document browser actor: cold ingress during ACK");
      if (this.captureUiPatch(acknowledged, captured.lifetime) !== null) throw new Error("document browser actor: patch before cold load");
      return captured.lifetime;
    } finally {
      if (config.byteLength) config.fill(0);
      if (quotas.byteLength) quotas.fill(0);
      if (result !== null) wipeBrowserActorValue(result);
    }
  }

  private transferColdPair(owner: VerifiedColdDocumentPair, child: DocumentBrowserActorChild, binding: Extract<PersistenceBinding, { kind: "hub" }>, assertCurrent: () => void): Promise<void> {
    if (this.coldApplied === owner) return Promise.resolve();
    if (this.coldOwner === owner && this.coldTransfer) return this.coldTransfer;
    if (this.coldTransfer) return Promise.reject(new Error("document browser actor: cold transfer already active"));
    const lifetime = this.lifetime;
    if (!lifetime) return Promise.reject(new Error("document browser actor: guest not live"));
    this.coldOwner = owner;
    this.coldTransfer = (async () => {
      for (let pageIndex = 0; pageIndex < owner.pageCount; pageIndex += 1) {
        assertCurrent();
        const page = owner.page(lifetime, pageIndex);
        const pageRecord = browserActorRecord(page, "document browser actor: invalid cold page");
        const bytes = pageRecord.bytes;
        if (!(bytes instanceof Uint8Array)) throw new Error("document browser actor: invalid cold bytes");
        let result: BrowserActorChildValue | null = null;
        try {
          result = await child.invoke(["reactor", "poll"], [[], null, page, browserActorTurnBudget()]);
          if (bytes.byteLength !== 0) throw new Error("document browser actor: cold page ownership not transferred");
          assertCurrent();
          const status = browserActorColdStatus(result);
          if (pageIndex + 1 === owner.pageCount) {
            owner.assertApplied(status, lifetime);
            await this.reconcileUiPatches(result, child, assertCurrent);
            await this.renderSurface(child, assertCurrent);
          } else {
            const expected = { lifetime, transferGeneration: owner.transferGeneration, pageIndex, pageCount: owner.pageCount };
            if (status.kind !== "pageAccepted" || !coldDocumentPairCursorEquals(status.cursor, expected)) throw new Error("document browser actor: invalid page receipt");
            if (this.captureUiPatch(result, lifetime) !== null) throw new Error("document browser actor: patch before cold pair applied");
          }
        } finally {
          if (bytes.byteLength) bytes.fill(0);
          if (result !== null) wipeBrowserActorValue(result);
        }
      }
      assertCurrent();
      this.coldApplied = owner;
      this.bindApprovalUndoIfMounted();
      if (!this.renderedUiPatch) emitExecutionTargetStatus(this.state, binding, "renderer-unavailable");
    })().finally(() => {
      if (this.coldOwner === owner) this.coldOwner = null;
      this.coldTransfer = null;
    });
    return this.coldTransfer;
  }

  private async renderSurface(child: DocumentBrowserActorChild, assertCurrent: () => void): Promise<void> {
    const lifetime = this.lifetime;
    if (lifetime === null) throw new Error("document browser actor: missing render lifetime");
    assertCurrent();
    const hostView = this.state.browserActorViewState;
    if (hostView === null) throw new Error("document browser actor: missing host view context");
    const windowId = this.lease.fields().surface.windowKindId;
    const viewState = windowViewContext(hostView, windowId);
    if (!viewState || viewState.activeWindowKindId !== windowId) throw new Error("document browser actor: unknown host window instance");
    const surface = { instance: lifetime.instanceId, surface: windowId };
    const bodyKey = this.lease.renderBodyKey();
    for (let turn = 0; turn < DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT; turn += 1) {
      assertCurrent();
      let result: BrowserActorChildValue | null = await child.invoke(["reactor", "poll"], [[turn === 0 ? { tag: "surface-visible", val: { surface, bodyKey, viewState: encodePackValue(viewState) } } : { tag: "wake" }], null, null, browserActorTurnBudget()]);
      try {
        assertCurrent();
        if (browserActorColdStatus(result).kind !== "idle") throw new Error("document browser actor: unexpected cold ingress during render");
        const owned = result;
        result = null;
        if (!(await this.reconcileUiPatches(owned, child, assertCurrent))) {
          this.renderedViewState = hostView;
          return;
        }
      } finally {
        if (result !== null) wipeBrowserActorValue(result);
      }
    }
    throw new Error("document browser actor: render turn limit");
  }

  private captureUiPatch(value: BrowserActorChildValue, lifetime: ActorInstanceLifetime) {
    const result = browserActorTurnResult(value),
      fields = this.lease.fields();
    const rawReceipt = result.uiPatchReceipt;
    return captureBrowserActorUiPatchV1(result.uiPatches, rawReceipt instanceof Uint8Array ? rawReceipt : unwrapBrowserActorOption(rawReceipt), lifetime, fields.surface.windowKindId, { decodePack: decodePackWire, natural: packWireNatural });
  }

  private awaitUiPatchResult(offer: BrowserActorUiPatchOfferV1): Promise<BrowserActorUiPatchResultV1> {
    if (this.pendingUiPatch !== null) return Promise.reject(new Error("document browser actor: patch result already pending"));
    return new Promise((resolve, reject) => {
      const timer = setTimeout(
        () => {
          if (this.pendingUiPatch?.offer !== offer) return;
          this.pendingUiPatch = null;
          reject(new Error("document browser actor: patch result deadline"));
          this.close();
        },
        Math.min(15_000, BROWSER_ACTOR_CHILD_LIMITS.invokeMs),
      );
      this.pendingUiPatch = { offer, resolve, reject, timer };
      post({ ...offer, clientInstanceId: this.state.openClientInstanceId });
    });
  }

  settleUiPatch(result: BrowserActorUiPatchResultV1): void {
    const pending = this.pendingUiPatch;
    if (pending === null || !browserActorUiPatchOwnerMatchesV1(pending.offer, result)) return;
    clearTimeout(pending.timer);
    this.pendingUiPatch = null;
    pending.resolve(result);
  }

  private async reconcileUiPatches(initial: BrowserActorChildValue, child: DocumentBrowserActorChild, assertCurrent: () => void): Promise<boolean> {
    const lifetime = this.lifetime;
    if (lifetime === null) throw new Error("document browser actor: missing patch lifetime");
    let value: BrowserActorChildValue | null = initial;
    try {
      for (let patchCount = 0; patchCount < 8; patchCount += 1) {
        const captured = this.captureUiPatch(value, lifetime);
        if (captured === null) return browserActorRecord(browserActorTurnResult(value).status, "document browser actor: invalid render status").tag === "more-work";
        const fields = this.lease.fields();
        const offer: BrowserActorUiPatchOfferV1 = {
          kind: "browser-actor-ui-patch",
          scope: { ...fields.scope },
          verifiedSurfaceId: fields.surface.surfaceId,
          activationGeneration: this.generation.toString(),
          instanceId: captured.instanceId,
          patch: captured.patch,
          receipt: Array.from(encodeActorUiPatchReceipt(captured.receipt)),
        };
        wipeBrowserActorValue(value);
        value = null;
        const result = await this.awaitUiPatchResult(offer);
        assertCurrent();
        if (result.outcome === "acknowledged" && result.revision !== captured.patch.revision) throw new Error("document browser actor: acknowledged revision mismatch");
        if (result.outcome === "acknowledged") {
          this.renderedUiPatch = true;
          const identity = this.lease.fields();
          if (identity.browserActor.kind !== "closed-browser-actor") throw new Error("document browser actor: mounted identity mismatch");
          post({
            kind: "browser-actor-ui-mounted",
            scope: { ...identity.scope },
            clientInstanceId: this.state.openClientInstanceId,
            activationGeneration: this.generation.toString(),
            instanceId: captured.instanceId,
            verifiedSurfaceId: identity.surface.surfaceId,
            catalogGenerationId: identity.catalog.generationId,
            componentSha256: identity.package.componentSha256,
            descriptorSha256: identity.package.descriptorByteSha256,
            browserActorSha256: identity.browserActor.sha256,
            uiRevision: result.revision,
          });
        }
        const feedback = {
          tag: result.outcome === "acknowledged" ? "patch-ack" : "patch-rejected",
          val: {
            receipt: { lifetime: { ...captured.receipt.lifetime }, patchSequence: captured.receipt.patchSequence },
            surface: { instance: captured.instanceId, surface: captured.patch.surface },
            revision: BigInt(result.revision),
            ...(result.outcome === "rejected" ? { reason: result.reason } : {}),
          },
        };
        value = await child.invoke(["reactor", "poll"], [[feedback], null, null, browserActorTurnBudget()]);
        assertCurrent();
        if (browserActorColdStatus(value).kind !== "idle") throw new Error("document browser actor: cold ingress after patch feedback");
      }
      throw new Error("document browser actor: patch feedback limit");
    } finally {
      if (value !== null) wipeBrowserActorValue(value);
    }
  }

  close(): void {
    if (this.closed) return;
    this.closed = true;
    clearTimeout(this.timer);
    this.state.docAbort.signal.removeEventListener("abort", this.retire);
    this.lease.retirement.removeEventListener("abort", this.retire);
    this.abort.abort();
    if (this.pendingUiPatch !== null) {
      const pending = this.pendingUiPatch;
      this.pendingUiPatch = null;
      clearTimeout(pending.timer);
      pending.reject(new Error("document browser actor: closed with pending patch"));
    }
    this.lifetime = null;
    this.coldOwner = null;
    this.coldApplied = null;
    this.renderedUiPatch = false;
    this.renderedViewState = null;
    this.child?.close();
    this.child = null;
    if (this.state.browserActorReservation === this) this.state.browserActorReservation = null;
  }
}

/** 🔐️ Resolves authority only from the current document owner, never caller-supplied actor metadata. */
function documentBrowserActorLease(state: ArtifactState): DocumentExecutionTargetLease {
  const lease = state.executionTargetLease,
    binding = hubBinding(state.config);
  if (state.closed || state.docAbort.signal.aborted || artifacts.get(state.runtimeKey) !== state || !binding || !lease?.live) throw new Error("document browser actor: unavailable");
  const fields = lease.fields();
  if (
    state.runtimeKey !== documentRuntimeKeyForConfig(state.config) ||
    fields.scope.spaceId !== binding.spaceId ||
    fields.scope.documentId !== state.config.documentId ||
    fields.artifact.schema !== state.config.schema ||
    lease.hubOrigin !== binding.baseUrl.replace(/\/+$/u, "")
  )
    throw new Error("document browser actor: scope mismatch");
  if ((binding.requestedSurfaceId !== undefined && binding.requestedSurfaceId !== fields.surface.surfaceId) || (binding.installedTarget !== undefined && !sameLeaseFieldsV1(binding.installedTarget, fields)))
    throw new Error("document browser actor: selection mismatch");
  return lease;
}

/** 🪪️ Claims a private document slot before reserving worker capacity, with no body fetch or activation. */
async function reserveDocumentBrowserActorChild(state: ArtifactState): Promise<DocumentBrowserActorReservation | null> {
  const lease = documentBrowserActorLease(state);
  if (lease.fields().browserActor.kind === "none") return null;
  const grant = lease.browserActorGrant();
  if (!grant || Date.now() >= grant.reserveBeforeMs || state.browserActorReservation !== null) throw new Error("document browser actor: admission denied");
  const owner = new DocumentBrowserActorReservation(state, lease, grant);
  state.browserActorReservation = owner;
  try {
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
async function activateDocumentBrowserActorAfterSession(state: ArtifactState, socket: WebSocket): Promise<void> {
  const lease = state.executionTargetLease,
    attempt = state.executionTargetOpen,
    binding = hubBinding(state.config);
  if (!lease || !binding) return;
  let owner: DocumentBrowserActorReservation | null = null;
  try {
    if (state.socket !== socket || !state.hubActorReady || state.actor !== lease.browserActorGrant()?.actorId) throw new Error("document browser actor: missing Session");
    if (documentBrowserActorLease(state).fields().browserActor.kind === "none") return;
    lease.assertBrowserActorDescribeCapacity();
    owner = await reserveDocumentBrowserActorChild(state);
    if (!owner) throw new Error("document browser actor: reservation unavailable");
    await owner.activate(socket);
  } catch {
    owner?.close();
    const current = state.socket === socket && state.executionTargetOpen === attempt && (state.executionTargetLease === lease || state.executionTargetLease === null);
    if (state.executionTargetLease === lease) state.executionTargetLease = null;
    lease.drop();
    if (current && !state.closed) emitExecutionTargetStatus(state, binding, state.docAbort.signal.aborted ? "cancelled" : "integrity-failed");
    socket.close(1008, "browser actor activation failed");
  }
}

//#endregion 🪪️ExecutionTargetLease

/** ⚖️ Compares one server plan against the complete locally installed execution-target fields through
 * the single shared {@link sameLeaseFieldsV1} relation. A non-`react` renderer target is admitted
 * only when the caller owns a live private lease whose own verified fields are the comparison input;
 * the verified target is then routed to an explicit renderer state, never to a module loader. */
function documentOpenPlanAuthority(
  plan: DocumentOpenPlanV1,
  intent: DocumentOpenIntentV1,
  config: ArtifactActorConfig,
  installed: NonNullable<Extract<PersistenceBinding, { kind: "hub" }>["installedTarget"]>,
  lease?: DocumentExecutionTargetLease,
): Omit<BrowserDocumentSocketAuthorityV1, "receipt"> {
  const leaseFields = lease !== undefined && lease.live ? lease.fields() : undefined;
  const projected = ((): DocumentExecutionTargetLeaseFieldsV1 | null => {
    try {
      return receiptFreeFields(plan, { component: installed.component.byteLength, descriptor: installed.descriptor.byteLength, browserActor: installed.browserActor.kind === "closed-browser-actor" ? installed.browserActor.byteLength : undefined });
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
  )
    throw new Error("document open: authority mismatch");
  const packSchemaHash = Array.from({ length: 32 }, (_unused, index) => Number.parseInt(plan.artifact.packSchemaHash.slice(index * 2, index * 2 + 2), 16));
  const configured = config.packSchemaHash;
  if (configured && configured.some((byte) => byte !== 0) && (configured.length !== 32 || configured.some((byte, index) => byte !== packSchemaHash[index]))) throw new Error("document open: authority mismatch");
  return { schema: plan.artifact.schema, packSchemaHash, parentDialect: plan.parentDialect, surfaceId: plan.surface.surfaceId };
}
/** 🧭️ Captures a single document-open attempt and rejects every later owner or selection change. */
function captureDocumentOpenOwner(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub" }>, intent: DocumentOpenIntentV1): () => void {
  const attempt = Symbol("document-open"),
    runtimeKey = state.runtimeKey,
    schema = state.config.schema;
  const origin = binding.baseUrl.replace(/\/+$/u, ""),
    installed = binding.installedTarget === undefined ? undefined : structuredClone(binding.installedTarget);
  state.executionTargetOpen = attempt;
  return () => {
    const current = hubBinding(state.config);
    if (
      state.closed ||
      state.docAbort.signal.aborted ||
      state.executionTargetOpen !== attempt ||
      state.runtimeKey !== runtimeKey ||
      artifacts.get(runtimeKey) !== state ||
      runtimeKey !== documentRuntimeKeyForConfig(state.config) ||
      state.config.schema !== schema ||
      state.config.documentId !== intent.scope.documentId ||
      state.openClientInstanceId !== intent.clientInstanceId ||
      !current ||
      current.spaceId !== intent.scope.spaceId ||
      current.baseUrl.replace(/\/+$/u, "") !== origin ||
      (current.requestedSurfaceId ?? current.installedTarget?.surface.surfaceId) !== intent.requestedSurfaceId ||
      (installed === undefined ? current.installedTarget !== undefined : current.installedTarget === undefined || !sameLeaseFieldsV1(current.installedTarget, installed))
    )
      throw new Error("document execution target: stale owner");
  };
}

async function requestDocumentSocketAuthority(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub" }>): Promise<BrowserDocumentSocketAuthorityV1> {
  const grantPath = `/spaces/${encodeURIComponent(binding.spaceId)}/documents/${encodeURIComponent(state.config.documentId)}/socket-grants`;
  if (socketGrantTestIssue) {
    const receipt = await socketGrantTestIssue(binding.baseUrl, grantPath, state.docAbort.signal);
    return {
      receipt,
      schema: state.config.schema,
      packSchemaHash: state.config.packSchemaHash ?? new Array(32).fill(0),
      ...(binding.installedTarget ? { surfaceId: binding.installedTarget.surface.surfaceId } : binding.requestedSurfaceId ? { surfaceId: binding.requestedSurfaceId } : {}),
    };
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
  const assertOwner = captureDocumentOpenOwner(state, binding, intent);
  const openControl: ExecutionTargetReadControl = { signal: state.docAbort.signal, deadlineAtMs: Date.now() + SOCKET_GRANT_REQUEST_TIMEOUT_MS, assertCurrent: assertOwner };
  assertExecutionTargetRead(openControl);
  const openPath = `/spaces/${encodeURIComponent(binding.spaceId)}/documents/${encodeURIComponent(state.config.documentId)}/open-plan`;
  const openResponse = await browserBrokerFetch(
    `/_semio/hub${openPath}`,
    { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(intent) },
    { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: state.docAbort.signal },
  );
  if (!openResponse.ok) throw new Error("document open: unavailable");
  let plan: DocumentOpenPlanV1;
  try {
    plan = parseDocumentOpenPlanV1(await readDocumentOpenJson(openResponse, openControl), Date.now());
  } catch {
    clearLocalBrowserBrokerProof();
    throw new Error(state.docAbort.signal.aborted ? "document open: cancelled" : "document open: invalid plan");
  }
  assertOwner();
  dropDocumentExecutionTargetLease(state);
  let lease: DocumentExecutionTargetLease | undefined;
  let published = false;
  try {
    if (plan.surface.rendererTarget !== "react" || binding.installedTarget === undefined) {
      try {
        lease = await installDocumentExecutionTargetLease(state, binding, plan, intent, assertOwner);
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
    const grantControl: ExecutionTargetReadControl = { signal: state.docAbort.signal, deadlineAtMs: Math.min(plan.expiresAtUnixMs, Date.now() + SOCKET_GRANT_REQUEST_TIMEOUT_MS), assertCurrent: assertOwner };
    assertExecutionTargetRead(grantControl);
    const exchange = parseDocumentPlanSocketGrantIntentV1({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt });
    const grantResponse = await browserBrokerFetch(
      `/_semio/hub${grantPath}`,
      { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(exchange) },
      { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: state.docAbort.signal },
    );
    if (!grantResponse.ok) {
      lease?.drop();
      if (lease !== undefined) emitExecutionTargetStatus(state, binding, "stale");
      throw new Error("document open: unavailable");
    }
    try {
      const receipt = parseSocketGrantReceiptV1(await readDocumentOpenJson(grantResponse, grantControl));
      assertExecutionTargetRead(grantControl);
      if (receipt.expiresAtMs <= Date.now() || receipt.expiresAtMs > plan.expiresAtUnixMs || (lease !== undefined && !lease.live)) throw new Error("document open: invalid grant");
      if (lease !== undefined) {
        lease.admitBrowserActor(documentExecutionTargetLeaseMintToken, receipt, plan.expiresAtUnixMs, { binding, intent, assertCurrent: assertOwner });
        state.executionTargetLease = lease;
        emitExecutionTargetStatus(state, binding, "renderer-unavailable");
      }
      assertExecutionTargetRead(grantControl);
      published = true;
      return { receipt, ...authority };
    } catch {
      lease?.drop();
      clearLocalBrowserBrokerProof();
      throw new Error("document open: invalid grant");
    }
  } finally {
    if (!published && lease !== undefined) {
      if (state.executionTargetLease === lease) dropDocumentExecutionTargetLease(state);
      lease.drop();
    }
  }
}

function browserDirectoryRequest(input: string, init: RequestInit = {}, options: { readonly timeoutMs: number; readonly signal?: AbortSignal }): Promise<FetchTimeoutResponse> {
  const url = new URL(input, "http://browser-broker.invalid");
  const method = init.method ?? "GET";
  const after = url.searchParams.get("after") ?? "";
  const eventPage = url.pathname === "/_semio/hub/directory/event-page/v1" && [...url.searchParams].length === 1 && /^(?:0|[1-9]\d*)$/u.test(after) && Number.isSafeInteger(Number(after));
  const allowed =
    (method === "GET" &&
      (((url.pathname === "/_semio/hub/directory/spaces" || /^\/_semio\/hub\/directory\/spaces\/[^/]+$/u.test(url.pathname)) && url.search === "") ||
        (url.pathname === "/_semio/hub/directory/events" && [...url.searchParams].length === 1 && /^\d+$/u.test(url.searchParams.get("since") ?? "")) ||
        eventPage)) ||
    (method === "POST" && url.pathname === "/_semio/hub/directory/commands" && url.search === "");
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
 * §4 of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`) — one
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
function decodePackPayload(bytes: ArrayLike<number>): PackValue {
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
function fromWireEnvelope(envelope: WireMutationEnvelope | ExactWireMutationEnvelope): MutationEnvelope {
  const payload = decodePackPayload(envelope.diff.payload);
  const sequenceNumber = payload !== null && typeof payload === "object" && !Array.isArray(payload) && !isPackInteger(payload) && "sequenceNumber" in payload ? packUIntSafeOrNull((payload as Record<string, PackValue>).sequenceNumber) : 0;
  if (sequenceNumber === null) throw new Error("wire envelope: sequenceNumber is not an exact unsigned integer");
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
      baseVersion: sequenceNumber,
      dependencies: [],
      undoPolicy: "exactBaseOnly",
    },
  };
}

function exactWireEnvelope(envelope: WireMutationEnvelope): ExactWireMutationEnvelope {
  const exactU64 = (value: number, field: string): bigint => {
    if (!Number.isSafeInteger(value) || value < 0) throw new Error(`wire envelope: ${field} is not an exact u64`);
    return BigInt(value);
  };
  return {
    mutation_id: envelope.mutation_id,
    document_id: envelope.document_id,
    actor: envelope.actor,
    dependencies: envelope.dependencies,
    diff: { schema: envelope.diff.schema, payload: Uint8Array.from(envelope.diff.payload) },
    inverse: { schema: envelope.inverse.schema, payload: Uint8Array.from(envelope.inverse.payload) },
    timestamp: {
      actor: exactU64(envelope.timestamp.actor, "timestamp.actor"),
      physical_ms: exactU64(envelope.timestamp.physical_ms, "timestamp.physical_ms"),
      logical: exactU64(envelope.timestamp.logical, "timestamp.logical"),
    },
  };
}

function documentBackboneMessage(envelopes: readonly ExactWireMutationEnvelope[]): Uint8Array {
  return encodeBackboneMessage({ kind: "mutations", envelopes: encodeDocumentBackboneEnvelopeBatchExact(envelopes) });
}

function documentBackboneMessageFromDomain(state: ArtifactState, envelopes: readonly MutationEnvelope[]): Uint8Array {
  return documentBackboneMessage(envelopes.map((envelope) => state.exactLocalEnvelopes.get(envelope)?.envelope ?? exactWireEnvelope(toWireEnvelope(envelope, nextWireTimestamp(state)))));
}

function emitMutationEvent(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  if (envelopes.length === 0) return;
  if (hubBinding(state.config)) emitEvent(state, { kind: "documentBackbone", message: documentBackboneMessageFromDomain(state, envelopes) });
  else emitEvent(state, { kind: "remoteMutations", envelopes });
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

type FolderCanonicalBootstrapMirrorOwner = Readonly<{
  binding: Extract<PersistenceBinding, { kind: "folder" }>;
  documentId: string;
  epoch: number;
  capability: string;
}>;

function folderCanonicalBootstrapMirrorUrl(owner: Pick<FolderCanonicalBootstrapMirrorOwner, "binding" | "documentId">, action: "reserve" | "stage" | "publish" | "retire"): string {
  return `${CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH}/${action}?uri=${encodeURIComponent(`folder://${owner.binding.path}`)}&documentId=${encodeURIComponent(owner.documentId)}`;
}

function folderCanonicalBootstrapMirrorHeaders(owner: FolderCanonicalBootstrapMirrorOwner): Record<string, string> {
  return { authorization: `SemioFolderBootstrap ${owner.capability}`, "x-semio-canonical-bootstrap-epoch": String(owner.epoch) };
}

async function reserveFolderCanonicalBootstrapMirror(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder" }>, bootstrap: WireArtifactBootstrap): Promise<FolderCanonicalBootstrapMirrorOwner> {
  const identity = { binding, documentId: state.config.documentId };
  const source = JSON.stringify({
    schema: "semio.backbone.canonical-bootstrap-folder-mirror-reserve/v1",
    artifactSchema: bootstrap.artifact_schema,
    descriptorDigestV1: executionTargetHex(new Uint8Array(bootstrap.descriptor_hash)),
    aggregateSha256: executionTargetHex(new Uint8Array(bootstrap.aggregate_hash)),
    baselineFrontier: {
      documentId: bootstrap.baseline_frontier.document_id,
      headEditOrdinal: bootstrap.baseline_frontier.head_edit_ordinal,
      headEditId: bootstrap.baseline_frontier.head_edit_id,
      lastCommitSeq: bootstrap.baseline_frontier.last_commit_seq,
      chainSha256: executionTargetHex(new Uint8Array(bootstrap.baseline_frontier.chain_hash)),
    },
  });
  const response = await fetchWithTimeout(
    folderCanonicalBootstrapMirrorUrl(identity, "reserve"),
    { method: "POST", headers: { "content-type": "application/json" }, body: source },
    { timeoutMs: FOLDER_FETCH_TIMEOUT_MS, signal: state.docAbort.signal },
  );
  const receipt = response.ok ? ((await response.json()) as Record<string, unknown>) : null;
  if (
    !response.ok ||
    receipt?.schema !== "semio.backbone.canonical-bootstrap-folder-mirror-owner/v1" ||
    typeof receipt.epoch !== "number" ||
    !Number.isSafeInteger(receipt.epoch) ||
    Number(receipt.epoch) < 1 ||
    typeof receipt.capability !== "string" ||
    !/^[0-9a-f]{64}$/u.test(receipt.capability)
  )
    throw new Error(`folder canonical bootstrap reserve failed (${response.status})`);
  return { ...identity, epoch: Number(receipt.epoch), capability: receipt.capability };
}

async function stageFolderCanonicalBootstrapMirror(state: ArtifactState, owner: FolderCanonicalBootstrapMirrorOwner, pack: Uint8Array, spr: Uint8Array): Promise<void> {
  const response = await fetchWithTimeout(
    folderCanonicalBootstrapMirrorUrl(owner, "stage"),
    { method: "PUT", headers: { ...folderCanonicalBootstrapMirrorHeaders(owner), "content-type": "application/octet-stream" }, body: ownedArrayBuffer(encodeDocumentPackBytes(pack, spr)) },
    { timeoutMs: FOLDER_FETCH_TIMEOUT_MS, signal: state.docAbort.signal },
  );
  if (!response.ok) throw new Error(`folder canonical bootstrap stage failed (${response.status})`);
}

async function publishFolderCanonicalBootstrapMirror(state: ArtifactState, owner: FolderCanonicalBootstrapMirrorOwner): Promise<void> {
  const response = await fetchWithTimeout(folderCanonicalBootstrapMirrorUrl(owner, "publish"), { method: "POST", headers: folderCanonicalBootstrapMirrorHeaders(owner) }, { timeoutMs: FOLDER_FETCH_TIMEOUT_MS, signal: state.docAbort.signal });
  if (!response.ok) throw new Error(`folder canonical bootstrap publish failed (${response.status})`);
}

async function retireFolderCanonicalBootstrapMirror(owner: FolderCanonicalBootstrapMirrorOwner): Promise<boolean> {
  try {
    const response = await fetchWithTimeout(folderCanonicalBootstrapMirrorUrl(owner, "retire"), { method: "POST", headers: folderCanonicalBootstrapMirrorHeaders(owner) }, { timeoutMs: FOLDER_FETCH_TIMEOUT_MS });
    return response.ok || response.status === 409;
  } catch {
    return false;
  }
}

async function retireCurrentFolderCanonicalBootstrapMirror(state: ArtifactState): Promise<void> {
  const owner = state.canonicalFolderMirror;
  if (!owner) return;
  if (!(await retireFolderCanonicalBootstrapMirror(owner))) throw new Error("folder canonical bootstrap retirement failed");
  if (state.canonicalFolderMirror === owner) state.canonicalFolderMirror = null;
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
  await retireCurrentFolderCanonicalBootstrapMirror(state);
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
    const presenceCandidate = authority.surfaceId ? { socket, scope: { spaceId: binding.spaceId, documentId: state.config.documentId }, verifiedSurfaceId: authority.surfaceId } : null;
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
      sendWireFrame(
        state,
        {
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
        },
        "command",
      );
    };
    socket.onmessage = (messageEvent) => {
      state.hubFrameChain = state.hubFrameChain.then(async () => {
        if (state.socket !== socket) return;
        try {
          const bytes = new Uint8Array(messageEvent.data as ArrayBuffer);
          const commandBatch = extractServerCommandsDocumentBackboneBatchExact(bytes);
          await handleHubFrame(state, decodeServerFrame(bytes).frame, presenceCandidate, socket, commandBatch);
        } catch (error) {
          console.error("[backbone-worker] malformed hub frame", state.config.documentId, error);
          rejectArtifactBootstrap(state, error);
        }
      });
    };
    socket.onclose = () => {
      state.docAbort.signal.removeEventListener("abort", onAbort);
      if (sustainedHealthTimer != null) clearTimeout(sustainedHealthTimer);
      if (state.socket !== socket) {
        resolve();
        return;
      }
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
    releaseDocumentBackboneOwnership(state, envelopes);
    const refusedIds = new Set(envelopes.map((envelope) => envelope.id));
    state.pendingMutations = state.pendingMutations.filter((envelope) => !refusedIds.has(envelope.id));
    state.outbox = state.outbox.filter((envelope) => !refusedIds.has(envelope.id));
    setStatus(state, { pendingMutations: state.pendingMutations.length });
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
  const wireEnvelopes = envelopes.map((envelope) => {
    const exact = state.exactLocalEnvelopes.get(envelope)?.envelope;
    const timestamp = nextWireTimestamp(state);
    if (exact === undefined) return toWireEnvelope(envelope, timestamp, state.actor);
    return {
      mutation_id: exact.mutation_id,
      document_id: exact.document_id,
      actor: state.actor,
      dependencies: [...exact.dependencies],
      diff: { schema: exact.diff.schema, payload: Array.from(exact.diff.payload) },
      inverse: { schema: exact.inverse.schema, payload: Array.from(exact.inverse.payload) },
      timestamp,
    };
  });
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

function rejectDocumentBackboneCapacity(state: ArtifactState, envelopes: readonly MutationEnvelope[], bytes: number): void {
  const batchId = nextLocalOverflowBatchId;
  nextLocalOverflowBatchId -= 1;
  emitEvent(state, {
    kind: "commandOutcome",
    batchId,
    outcome: { kind: "rejected", reason: "document backbone pending capacity", messages: [envelopes.length, bytes, DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes] },
  });
}

function releaseDocumentBackboneOwnership(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  let releasedBytes = 0;
  let releasedMessages = 0;
  for (const envelope of envelopes) {
    const retained = state.exactLocalEnvelopes.get(envelope);
    if (retained === undefined) continue;
    releasedBytes += retained.bytes;
    releasedMessages += retained.messages;
    state.exactLocalEnvelopes.delete(envelope);
  }
  if (releasedBytes > state.pendingDocumentBackboneBytes || releasedMessages > state.pendingDocumentBackboneMessages) throw new Error("document backbone: retained byte ledger underflow");
  state.pendingDocumentBackboneBytes -= releasedBytes;
  state.pendingDocumentBackboneMessages -= releasedMessages;
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
    releaseDocumentBackboneOwnership(state, sent);
    const sentIds = new Set(sent.map((envelope) => envelope.id));
    state.pendingMutations = state.pendingMutations.filter((envelope) => !sentIds.has(envelope.id));

    const outcome = stage.Applied.outcome;
    let ackOutcome: CommandAckOutcome;
    if (outcome === "Accepted") {
      ackOutcome = { kind: "accepted" };
    } else if ("Transformed" in outcome) {
      const rollbacks = [...sent].reverse().map(rollbackEnvelope);
      emitMutationEvent(state, rollbacks);
      const converted = fromWireEnvelope(outcome.Transformed.envelope);
      emitMutationEvent(state, [converted]);
      ackOutcome = { kind: "transformed" };
    } else {
      const rollbacks = [...sent].reverse().map(rollbackEnvelope);
      emitMutationEvent(state, rollbacks);
      ackOutcome = { kind: "rejected", reason: outcome.Rejected.reason, messages: outcome.Rejected.messages };
    }
    setStatus(state, { pendingMutations: state.pendingMutations.length });
    emitEvent(state, { kind: "commandOutcome", batchId, outcome: ackOutcome });
  }
}

function equalByteArrays(left: ArrayLike<number>, right: ArrayLike<number>): boolean {
  if (left.length !== right.length) return false;
  for (let index = 0; index < left.length; index++) if (left[index] !== right[index]) return false;
  return true;
}

function equalFrontiers(left: WireFrontierSummary, right: WireFrontierSummary): boolean {
  return left.document_id === right.document_id && left.head_edit_ordinal === right.head_edit_ordinal && left.head_edit_id === right.head_edit_id && left.last_commit_seq === right.last_commit_seq && equalByteArrays(left.chain_hash, right.chain_hash);
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
  post({
    kind: "artifact-bootstrap-progress",
    documentId: state.config.documentId,
    clientInstanceId: state.openClientInstanceId,
    ...(scope === undefined ? {} : { scope }),
    receivedBytes: progress.receivedBytes,
    totalBytes: progress.totalBytes,
    receivedChunks: progress.receivedChunks,
    totalChunks: progress.totalChunks,
  });
}

type DocumentArtifactBootstrapOwner = {
  assembler: ArtifactBootstrapAssembler | null;
  folderMirror: FolderCanonicalBootstrapMirrorOwner | null;
  deadlineMs: number | null;
  deadlineTimer: ReturnType<typeof setTimeout> | null;
  readonly rebootstrapOwner: DocumentArtifactRebootstrapOwner | null;
  readonly socket: WebSocket | null;
  assertCurrent(): void;
};

/** 🧷️ Retains the document and transport selection that admitted one canonical pair transfer. */
function captureArtifactBootstrapOwner(state: ArtifactState): DocumentArtifactBootstrapOwner {
  const config = state.config,
    runtimeKey = state.runtimeKey,
    socket = state.socket,
    client = state.openClientInstanceId;
  const attempt = state.executionTargetOpen,
    lease = state.executionTargetLease,
    schema = config.schema,
    documentId = config.documentId;
  const binding = hubBinding(config),
    spaceId = binding?.spaceId,
    origin = binding?.baseUrl,
    surface = binding?.requestedSurfaceId;
  const installed = binding?.installedTarget === undefined ? undefined : structuredClone(binding.installedTarget);
  const packSchemaHash = config.packSchemaHash?.slice(),
    folder = folderBinding(config)?.path,
    rebootstrapOwner = state.artifactRebootstrapOwner;
  const owner: DocumentArtifactBootstrapOwner = {
    assembler: null,
    folderMirror: null,
    deadlineMs: null,
    deadlineTimer: null,
    rebootstrapOwner,
    socket,
    assertCurrent() {
      const current = hubBinding(state.config);
      if (
        state.closed ||
        state.docAbort.signal.aborted ||
        state.artifactBootstrapOwner !== owner ||
        state.config !== config ||
        state.runtimeKey !== runtimeKey ||
        artifacts.get(runtimeKey) !== state ||
        state.socket !== socket ||
        state.openClientInstanceId !== client ||
        state.executionTargetOpen !== attempt ||
        state.executionTargetLease !== lease ||
        config.schema !== schema ||
        config.documentId !== documentId ||
        current?.spaceId !== spaceId ||
        current?.baseUrl !== origin ||
        current?.requestedSurfaceId !== surface ||
        folderBinding(config)?.path !== folder ||
        (packSchemaHash === undefined ? config.packSchemaHash !== undefined : config.packSchemaHash === undefined || !equalByteArrays(config.packSchemaHash, packSchemaHash)) ||
        (installed === undefined ? current?.installedTarget !== undefined : current?.installedTarget === undefined || !sameLeaseFieldsV1(current.installedTarget, installed)) ||
        (owner.assembler !== null && state.artifactBootstrap !== owner.assembler) ||
        (rebootstrapOwner !== null && state.artifactRebootstrapOwner !== rebootstrapOwner) ||
        (lease !== null && !lease.live)
      )
        throw new Error("artifact bootstrap stale owner");
    },
  };
  state.artifactBootstrapOwner = owner;
  owner.assertCurrent();
  return owner;
}

type DocumentArtifactRebootstrapOwner = {
  readonly deadlineMs: number;
  readonly timer: ReturnType<typeof setTimeout>;
  assertCurrent(): void;
};

function clearArtifactRebootstrapDeadline(state: ArtifactState, owner: DocumentArtifactRebootstrapOwner | null): void {
  if (!owner) return;
  clearTimeout(owner.timer);
  if (state.artifactRebootstrapOwner === owner) {
    state.artifactRebootstrapOwner = null;
    state.artifactRebootstrapDeadlineMs = null;
    state.artifactRebootstrapDeadlineTimer = null;
  }
}

function captureArtifactRebootstrapOwner(state: ArtifactState): DocumentArtifactRebootstrapOwner {
  clearArtifactRebootstrapDeadline(state, state.artifactRebootstrapOwner);
  const config = state.config,
    runtimeKey = state.runtimeKey,
    clientInstanceId = state.openClientInstanceId,
    binding = hubBinding(config),
    spaceId = binding?.spaceId,
    baseUrl = binding?.baseUrl,
    deadlineMs = Date.now() + ARTIFACT_BOOTSTRAP_DEADLINE_MS;
  let owner: DocumentArtifactRebootstrapOwner;
  const timer = setTimeout(() => {
    if (
      state.artifactRebootstrapOwner !== owner ||
      state.artifactRebootstrapDeadlineMs !== deadlineMs ||
      state.artifactRebootstrapDeadlineTimer !== timer
    )
      return;
    clearArtifactRebootstrapDeadline(state, owner);
    rejectArtifactBootstrap(state, new Error("artifact rebootstrap deadline exceeded"));
  }, Math.max(0, deadlineMs - Date.now()));
  owner = {
    deadlineMs,
    timer,
    assertCurrent() {
      const current = hubBinding(state.config);
      if (
        state.closed ||
        state.docAbort.signal.aborted ||
        state.artifactRebootstrapOwner !== owner ||
        state.artifactRebootstrapDeadlineMs !== deadlineMs ||
        state.artifactRebootstrapDeadlineTimer !== timer ||
        state.config !== config ||
        state.runtimeKey !== runtimeKey ||
        artifacts.get(runtimeKey) !== state ||
        state.openClientInstanceId !== clientInstanceId ||
        current?.spaceId !== spaceId ||
        current?.baseUrl !== baseUrl
      )
        throw new Error("artifact rebootstrap stale owner");
    },
  };
  state.artifactRebootstrapOwner = owner;
  state.artifactRebootstrapDeadlineMs = deadlineMs;
  state.artifactRebootstrapDeadlineTimer = timer;
  state.artifactRebootstrapRequired = true;
  owner.assertCurrent();
  return owner;
}

function abortArtifactRebootstrap(state: ArtifactState): void {
  clearArtifactRebootstrapDeadline(state, state.artifactRebootstrapOwner);
  state.artifactRebootstrapRequired = false;
}

function clearArtifactBootstrapDeadline(state: ArtifactState, owner: DocumentArtifactBootstrapOwner | null): void {
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

function armArtifactBootstrapDeadline(state: ArtifactState, owner: DocumentArtifactBootstrapOwner, deadlineMs: number): void {
  if (owner.deadlineTimer !== null || state.artifactBootstrapDeadlineTimer !== null) throw new Error("artifact bootstrap deadline already armed");
  owner.deadlineMs = deadlineMs;
  state.artifactBootstrapDeadlineMs = deadlineMs;
  const timer = setTimeout(() => {
    if (
      state.artifactBootstrapOwner !== owner ||
      state.artifactBootstrap !== owner.assembler ||
      state.artifactBootstrapDeadlineMs !== deadlineMs ||
      state.artifactBootstrapDeadlineTimer !== timer ||
      owner.deadlineMs !== deadlineMs ||
      owner.deadlineTimer !== timer
    )
      return;
    owner.deadlineTimer = null;
    state.artifactBootstrapDeadlineTimer = null;
    rejectArtifactBootstrap(state, new Error("artifact bootstrap deadline exceeded"), owner);
  }, Math.max(0, deadlineMs - Date.now()));
  owner.deadlineTimer = timer;
  state.artifactBootstrapDeadlineTimer = timer;
}

function bootstrapControl(state: ArtifactState, owner = state.artifactBootstrapOwner): ArtifactBootstrapControl {
  const cancelled = () => {
    if (state.closed || state.docAbort.signal.aborted) return true;
    try {
      owner?.assertCurrent();
      return false;
    } catch {
      return true;
    }
  };
  return {
    isCancelled: cancelled,
    nowMs: () => Date.now(),
    onProgress: (progress) => {
      if (!cancelled()) emitBootstrapProgress(state, progress);
    },
  };
}

function abortArtifactBootstrap(state: ArtifactState): void {
  const owner = state.artifactBootstrapOwner;
  if (owner) clearArtifactBootstrapDeadline(state, owner);
  else if (state.artifactBootstrapDeadlineTimer !== null) clearTimeout(state.artifactBootstrapDeadlineTimer);
  state.artifactBootstrap?.abort();
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

function artifactBootstrapFailure(state: ArtifactState, error: unknown): Extract<ArtifactBootstrapWorkerEvent, { readonly kind: "artifact-bootstrap-failed" }> {
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

function rejectArtifactBootstrap(state: ArtifactState, error: unknown, owner = state.artifactBootstrapOwner): void {
  if (owner && (state.artifactBootstrapOwner !== owner || state.artifactBootstrap !== owner.assembler || state.socket !== owner.socket)) {
    owner.assembler?.abort();
    if (state.artifactBootstrapOwner === owner && state.artifactBootstrap === owner.assembler) abortArtifactBootstrap(state);
    if (state.socket !== owner.socket) owner.socket?.close();
    return;
  }
  const rebootstrapOwner = owner?.rebootstrapOwner ?? state.artifactRebootstrapOwner;
  if (rebootstrapOwner !== null && state.artifactRebootstrapOwner === rebootstrapOwner) clearArtifactRebootstrapDeadline(state, rebootstrapOwner);
  const failure = artifactBootstrapFailure(state, error);
  const socket = owner ? owner.socket : state.socket;
  abortArtifactBootstrap(state);
  post(failure);
  socket?.close();
}

async function requireArtifactRebootstrap(state: ArtifactState): Promise<void> {
  const owner = captureArtifactRebootstrapOwner(state);
  reissueInferenceApprovalUndoForRebootstrap(state);
  if (inferencePort !== null && documentRuntimeKeyV1({ kind: "hub", ...inferencePort.scope }) === state.runtimeKey && inferencePort.status.phase !== "approving") closeInferencePort(inferencePort.operationEpoch);
  await retireCurrentFolderCanonicalBootstrapMirror(state);
  if (state.artifactRebootstrapOwner !== owner) return;
  owner.assertCurrent();
  abortArtifactBootstrap(state);
  dropVerifiedColdDocumentPair(state);
  state.currentPack = null;
  state.currentSpr = null;
  state.frontier = null;
  state.resumeToken = null;
  state.artifactBootstrapProgress = [];
  setRemote(state, { kind: "connecting" });
  const scope = artifactScope(state);
  post({ kind: "artifact-rebootstrap-required", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), message: "rebootstrap-required", retryable: true });
  state.socket?.close();
}

function validateArtifactBootstrapIdentity(state: ArtifactState, bootstrap: WireArtifactBootstrap, serverFrontier: WireFrontierSummary): void {
  if (bootstrap.artifact_schema !== state.config.schema) throw new Error("artifact bootstrap schema mismatch");
  if (bootstrap.baseline_frontier.document_id !== state.config.documentId || bootstrap.required_tail_frontier.document_id !== state.config.documentId || serverFrontier.document_id !== state.config.documentId)
    throw new Error("artifact bootstrap document mismatch");
  const packSchemaHash = state.config.packSchemaHash;
  if (!packSchemaHash || packSchemaHash.length !== 32 || packSchemaHash.every((byte) => byte === 0) || !equalByteArrays(bootstrap.pack_schema_hash, packSchemaHash)) throw new Error("artifact bootstrap pack schema mismatch");
  if (!equalFrontiers(bootstrap.required_tail_frontier, serverFrontier)) throw new Error("artifact bootstrap required tail does not match welcome frontier");
  const lease = state.executionTargetLease;
  if (lease) {
    const fields = lease.fields(),
      checkpoint = fields.checkpoint,
      binding = hubBinding(state.config);
    if (
      !binding ||
      fields.scope.spaceId !== binding.spaceId ||
      fields.scope.documentId !== state.config.documentId ||
      lease.hubOrigin !== binding.baseUrl.replace(/\/+$/u, "") ||
      fields.descriptorDigestV1 !== executionTargetHex(new Uint8Array(bootstrap.descriptor_hash)) ||
      fields.artifact.kind !== bootstrap.artifact_kind ||
      fields.artifact.schema !== bootstrap.artifact_schema ||
      fields.artifact.packSchemaHash !== executionTargetHex(new Uint8Array(bootstrap.pack_schema_hash)) ||
      !checkpoint ||
      checkpoint.descriptorDigestV1 !== fields.descriptorDigestV1 ||
      checkpoint.aggregateSha256 !== executionTargetHex(new Uint8Array(bootstrap.aggregate_hash))
    )
      throw new Error("artifact bootstrap execution-target checkpoint mismatch");
    const frontier = checkpoint.baselineFrontier;
    if (!equalFrontiers(bootstrap.baseline_frontier, { document_id: frontier.documentId, head_edit_ordinal: frontier.headEditOrdinal, head_edit_id: frontier.headEditId, last_commit_seq: frontier.lastCommitSeq, chain_hash: frontier.chainHash }))
      throw new Error("artifact bootstrap execution-target frontier mismatch");
    if (lease.browserActorGrant()) lease.assertBrowserActorCurrent();
  }
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

async function installArtifactBootstrap(state: ArtifactState, owner: DocumentArtifactBootstrapOwner, done: { readonly descriptor_hash: readonly number[]; readonly chunk_count: number } | null): Promise<void> {
  const assembler = owner.assembler;
  if (!assembler) throw new Error("artifact bootstrap missing assembler");
  const previousPack = state.currentPack,
    previousSpr = state.currentSpr,
    previousFrontier = state.frontier;
  let pair: { readonly pack: Uint8Array; readonly spr: Uint8Array } | undefined;
  let publishedPack: Uint8Array | null = null,
    publishedSpr: Uint8Array | null = null,
    coldOwner: VerifiedColdDocumentPair | null = null;
  try {
    owner.assertCurrent();
    pair = await assembler.finish(done, bootstrapControl(state, owner));
    owner.assertCurrent();
    const folder = folderBinding(state.config);
    if (folder) {
      const mirror = owner.folderMirror;
      if (!mirror) throw new Error("folder canonical bootstrap reservation missing");
      try {
        await stageFolderCanonicalBootstrapMirror(state, mirror, pair.pack, pair.spr);
        owner.assertCurrent();
        await publishFolderCanonicalBootstrapMirror(state, mirror);
        owner.assertCurrent();
        state.canonicalFolderMirror = mirror;
      } catch (error) {
        await retireFolderCanonicalBootstrapMirror(mirror);
        if (owner.folderMirror === mirror) owner.folderMirror = null;
        throw error;
      }
    }
    publishedPack = Uint8Array.from(pair.pack);
    publishedSpr = Uint8Array.from(pair.spr);
    state.currentPack = publishedPack;
    state.currentSpr = publishedSpr;
    state.frontier = assembler.bootstrap.baseline_frontier;
    const lease = state.executionTargetLease;
    if (lease?.fields().browserActor.kind === "closed-browser-actor") {
      coldOwner = new VerifiedColdDocumentPair(verifiedColdDocumentPairMintToken, state, lease, assembler.bootstrap, pair, { pack: publishedPack, spr: publishedSpr });
      state.verifiedColdPair = coldOwner;
      coldOwner.assertCurrent();
      await state.browserActorReservation?.installColdPair(coldOwner);
      owner.assertCurrent();
      coldOwner.assertCurrent();
    }
    emitEvent(state, { kind: "snapshotReplaced", pack: Array.from(pair.pack), spr: Array.from(pair.spr) });
    owner.assertCurrent();
    if (state.outbox.length > 0) {
      emitMutationEvent(state, [...state.outbox]);
      owner.assertCurrent();
    }
    if (owner.rebootstrapOwner !== null) {
      owner.rebootstrapOwner.assertCurrent();
      clearArtifactRebootstrapDeadline(state, owner.rebootstrapOwner);
    }
    state.artifactRebootstrapRequired = false;
    clearArtifactBootstrapDeadline(state, owner);
    state.artifactBootstrap = null;
    state.artifactBootstrapOwner = null;
    finishCatchupIfReady(state);
  } catch (error) {
    if (state.verifiedColdPair === coldOwner) dropVerifiedColdDocumentPair(state);
    if (state.currentPack === publishedPack) {
      publishedPack?.fill(0);
      state.currentPack = previousPack;
    }
    if (state.currentSpr === publishedSpr) {
      publishedSpr?.fill(0);
      state.currentSpr = previousSpr;
    }
    if (state.currentPack === previousPack && state.currentSpr === previousSpr) state.frontier = previousFrontier;
    throw error;
  } finally {
    pair?.pack.fill(0);
    pair?.spr.fill(0);
  }
}

async function startArtifactBootstrap(state: ArtifactState, bootstrap: WireArtifactBootstrap, resumeToken: string, serverFrontier: WireFrontierSummary): Promise<void> {
  abortArtifactBootstrap(state);
  dropVerifiedColdDocumentPair(state);
  state.artifactBootstrapProgress = [];
  validateArtifactBootstrapIdentity(state, bootstrap, serverFrontier);
  const owner = captureArtifactBootstrapOwner(state);
  let assembler: ArtifactBootstrapAssembler | undefined;
  try {
    state.pendingResumeToken = resumeToken;
    state.requiredTailFrontier = bootstrap.required_tail_frontier;
    const deadlineMs = Date.now() + ARTIFACT_BOOTSTRAP_DEADLINE_MS;
    assembler = new ArtifactBootstrapAssembler(bootstrap, bootstrap.descriptor_hash, DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS, deadlineMs, bootstrapControl(state, owner));
    owner.assertCurrent();
    owner.assembler = assembler;
    state.artifactBootstrap = assembler;
    armArtifactBootstrapDeadline(state, owner, deadlineMs);
    const folder = folderBinding(state.config);
    if (folder) {
      owner.folderMirror = await reserveFolderCanonicalBootstrapMirror(state, folder, bootstrap);
      owner.assertCurrent();
    }
    if (bootstrap.inline !== null) await installArtifactBootstrap(state, owner, null);
  } catch (error) {
    assembler?.abort();
    rejectArtifactBootstrap(state, error, owner);
  }
}

async function handleHubFrame(
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
      if (state.hubActorReady && state.outbox.length > 0) relayMutationsToHub(state, state.outbox.splice(0));
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
    if (state.artifactBootstrap || state.artifactRebootstrapRequired) {
      rejectArtifactBootstrap(state, new Error("tail arrived before artifact bootstrap completion"));
      return;
    }
    if (frame.Commands.origin !== state.actor) {
      if (frame.Commands.envelopes.length > 0 && commandBatch === null) throw new Error("document backbone: exact server command batch missing");
      if (frame.Commands.envelopes.length > 0 && commandBatch !== null) emitEvent(state, { kind: "documentBackbone", message: encodeBackboneMessage({ kind: "mutations", envelopes: commandBatch }) });
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
      post({ kind: "socket-actor-failed", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), code: "session-mismatch" });
      dropDocumentExecutionTargetLease(state);
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
    post({ kind: "socket-actor", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), actorId: expectedActor });
    emitEvent(state, { kind: "session", actor: frame.Session.actor, color: frame.Session.color });
    if (sourceSocket && state.executionTargetLease) void activateDocumentBrowserActorAfterSession(state, sourceSocket);
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

const SPACE_ARTIFACT_CREATION_CAPACITY = 8;
const SPACE_ARTIFACT_CREATION_CATALOG_CAPACITY = 8;
const SPACE_ARTIFACT_CREATION_POLL_MS = 100;
const SPACE_ARTIFACT_CREATION_DEADLINE_MS = 120_000;
const SPACE_ARTIFACT_CREATION_CATALOG_DEADLINE_MS = 10_000;

type SpaceArtifactCreationOperationV1 = {
  readonly request: Extract<BackboneWorkerRequest, { readonly kind: "space-artifact-create" }>;
  readonly abort: AbortController;
  readonly workerEpoch: number;
  readonly deadlineAtMs: number;
  cancelRequested: boolean;
  cancelSent: boolean;
  latest: Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-status" }>;
};

const spaceArtifactCreationOperations = new Map<string, SpaceArtifactCreationOperationV1>();
const spaceArtifactCreationCatalogOperations = new Map<string, Readonly<{ abort: AbortController; clientInstanceId: string; workerEpoch: number }>>();
let spaceArtifactCreationTestFetch: ((path: string, init: RequestInit, signal: AbortSignal) => Promise<FetchTimeoutResponse>) | null = null;

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

async function openSpaceArtifactCreationCatalog(spaceId: string, clientInstanceId: string): Promise<void> {
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
      ? await browserBrokerFetch(`/_semio/hub${path}`, { method: "GET" }, { timeoutMs: SPACE_ARTIFACT_CREATION_CATALOG_DEADLINE_MS, signal: operation.abort.signal })
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
  if (status.requestId !== operation.request.requestId || status.spaceId !== operation.request.spaceId || (status.ready !== undefined && status.ready.kindId !== operation.request.kindId)) throw new Error("space artifact creation: owner mismatch");
  return { kind: "space-artifact-creation-status", requestId: status.requestId, spaceId: status.spaceId, phase: status.phase, ...(status.ready === undefined ? {} : { ready: status.ready }) };
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
  return browserBrokerFetch(`/_semio/hub${path}`, init, { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: operation.abort.signal });
}

async function readSpaceArtifactCreationStatus(operation: SpaceArtifactCreationOperationV1, response: FetchTimeoutResponse): Promise<HubSpaceArtifactCreationStatusV1> {
  const control: ExecutionTargetReadControl = { signal: operation.abort.signal, deadlineAtMs: operation.deadlineAtMs, assertCurrent: () => {
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
  return new Promise((resolve, reject) => {
    const timer = setTimeout(resolve, SPACE_ARTIFACT_CREATION_POLL_MS);
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
    if (Date.now() >= operation.deadlineAtMs) {
      settleSpaceArtifactCreation(operation, { ...operation.latest, phase: "indeterminate" });
      return;
    }
    try {
      let response: FetchTimeoutResponse;
      if (operation.cancelRequested && !operation.cancelSent) {
        operation.cancelSent = true;
        response = await spaceArtifactCreationFetch(operation, `/${operation.request.requestId}/cancel`, { method: "POST" });
      } else if (first) {
        first = false;
        response = await spaceArtifactCreationFetch(operation, "", { method: "POST", headers: { "content-type": "application/json" }, body: requestBody });
      } else {
        response = await spaceArtifactCreationFetch(operation, `/${operation.request.requestId}`, { method: "GET" });
      }
      if (!response.ok) {
        if (response.status >= 400 && response.status < 500) {
          settleSpaceArtifactCreation(operation, { ...operation.latest, phase: "failed" });
          return;
        }
        throw new Error("space artifact creation: unavailable");
      }
      const status = spaceArtifactCreationStatus(operation, await readSpaceArtifactCreationStatus(operation, response));
      if (!spaceArtifactCreationCurrent(operation)) return;
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

function submitSpaceArtifactCreation(request: Extract<BackboneWorkerRequest, { readonly kind: "space-artifact-create" }>): void {
  const existing = spaceArtifactCreationOperations.get(request.requestId);
  if (existing !== undefined) {
    if (existing.request.spaceId === request.spaceId && existing.request.kindId === request.kindId && existing.request.name === request.name) post(existing.latest);
    else post({ kind: "space-artifact-creation-status", requestId: request.requestId, spaceId: request.spaceId, phase: "failed" });
    return;
  }
  if (spaceArtifactCreationOperations.size >= SPACE_ARTIFACT_CREATION_CAPACITY) {
    post({ kind: "space-artifact-creation-status", requestId: request.requestId, spaceId: request.spaceId, phase: "failed" });
    return;
  }
  const operation: SpaceArtifactCreationOperationV1 = {
    request,
    abort: new AbortController(),
    workerEpoch: directoryWorkerEpoch,
    deadlineAtMs: Date.now() + SPACE_ARTIFACT_CREATION_DEADLINE_MS,
    cancelRequested: false,
    cancelSent: false,
    latest: { kind: "space-artifact-creation-status", requestId: request.requestId, spaceId: request.spaceId, phase: "accepted" },
  };
  spaceArtifactCreationOperations.set(request.requestId, operation);
  post(operation.latest);
  void driveSpaceArtifactCreation(operation);
}

function cancelSpaceArtifactCreation(requestId: string, spaceId: string): void {
  const operation = spaceArtifactCreationOperations.get(requestId);
  if (operation === undefined || operation.request.spaceId !== spaceId) return;
  operation.cancelRequested = true;
}

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
      closeArtifactRuntime(key);
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
  authorPage: boolean;
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
  operation.authorPage = false;
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
  operation.authorPage = false;
  postDirectoryAdministrationState(operation);
  try {
    const page = await client.spaceAdministrationPage(operation.spaceId, cursor, { signal: operation.abort.signal });
    const live = liveDirectoryAdministration(operationEpoch);
    if (live === null || live !== operation) return;
    operation.canonicalJson = page.canonicalJson;
    operation.authorPage = page.page.access === "author";
    if (!operation.authorPage) {
      operation.inviteToken = null;
      operation.inviteCapabilityStatus = null;
      operation.inviteTransferEpoch = null;
    }
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
    authorPage: false,
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
  if (!operation.authorPage) {
    operation.inviteToken = null;
    operation.inviteCapabilityStatus = null;
    operation.inviteTransferEpoch = null;
    post({ kind: "directory-administration-capability-rejected", operationEpoch, code: "already-settled" });
    return;
  }
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

type InferenceApprovalUndoMountV1 = Readonly<{
  activationGeneration: bigint;
  catalogGenerationId: string;
  componentSha256: string;
  descriptorSha256: string;
  browserActorSha256: string;
  directoryRevision: number;
  membershipGeneration: number;
  sessionGeneration?: number;
  shareGeneration?: number;
}>;

type InferenceApprovalUndoOwnerV1 = {
  readonly historyEpoch: number;
  readonly scope: DocumentScope;
  readonly clientInstanceId: string;
  readonly receipt: GisMapInferenceApprovalReceiptV1;
  readonly idempotencyKey: string;
  readonly sourceCatalogGenerationId: string;
  readonly sourceComponentSha256: string;
  readonly sourceDescriptorSha256: string;
  readonly sourceBrowserActorSha256: string;
  readonly sourceDirectoryRevision: number;
  readonly sourceMembershipGeneration: number;
  readonly sourceSessionGeneration?: number;
  readonly sourceShareGeneration?: number;
  readonly abort: AbortController;
  mount: InferenceApprovalUndoMountV1 | null;
  phase: "awaiting-mount" | "available" | "submitting" | "failed";
  retryable: boolean;
};

let inferenceApprovalUndoEpoch = 0;
let inferenceApprovalUndoOwner: InferenceApprovalUndoOwnerV1 | null = null;

function mintInferenceApprovalUndoIdempotencyKeyV1(): string {
  const bytes = crypto.getRandomValues(new Uint8Array(16));
  bytes[0] = (bytes[0] ?? 0) | 1;
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

function approvalUndoStatus(owner: InferenceApprovalUndoOwnerV1, status: GisMapApprovalHistoryStatusV1): void {
  post({ kind: "inference-history-status", historyEpoch: owner.historyEpoch, clientInstanceId: owner.clientInstanceId, scope: owner.scope, status });
}

function sameApprovalUndoFrontierV1(owner: InferenceApprovalUndoOwnerV1, pair: VerifiedColdDocumentPair): boolean {
  const expected = owner.receipt.undo.expectedCurrent;
  return pair.frontier.documentId === expected.documentId
    && pair.frontier.headEditOrdinal === BigInt(expected.headEditOrdinal)
    && pair.frontier.headEditId === expected.headEditId
    && pair.frontier.lastCommitSeq === BigInt(expected.lastCommitSeq)
    && bytesHex(pair.frontier.chainSha256) === expected.chainSha256;
}

function sameApprovalUndoMountV1(state: ArtifactState, owner: InferenceApprovalUndoOwnerV1): boolean {
  const lease = state.executionTargetLease;
  const reservation = state.browserActorReservation;
  const pair = state.verifiedColdPair;
  const mount = owner.mount;
  if (!lease?.live || reservation === null || pair === null || mount === null || state.closed || state.docAbort.signal.aborted) return false;
  const fields = lease.fields();
  return state.openClientInstanceId === owner.clientInstanceId
    && fields.scope.spaceId === owner.scope.spaceId
    && fields.scope.documentId === owner.scope.documentId
    && reservation.generation === mount.activationGeneration
    && fields.catalog.generationId === mount.catalogGenerationId
    && fields.package.componentSha256 === mount.componentSha256
    && fields.package.descriptorByteSha256 === mount.descriptorSha256
    && fields.browserActor.kind === "closed-browser-actor"
    && fields.browserActor.sha256 === mount.browserActorSha256
    && fields.revalidation.directoryRevision === mount.directoryRevision
    && fields.revalidation.membershipGeneration === mount.membershipGeneration
    && fields.revalidation.sessionGeneration === mount.sessionGeneration
    && fields.revalidation.shareGeneration === mount.shareGeneration
    && sameApprovalUndoFrontierV1(owner, pair);
}

function retireInferenceApprovalUndo(owner: InferenceApprovalUndoOwnerV1, notify = true): void {
  if (inferenceApprovalUndoOwner !== owner) return;
  inferenceApprovalUndoOwner = null;
  owner.abort.abort(new Error("gis map approval undo owner retired"));
  if (notify) approvalUndoStatus(owner, { phase: "unavailable", canUndo: false, code: null });
}

function reissueInferenceApprovalUndoForRebootstrap(state: ArtifactState): void {
  const owner = inferenceApprovalUndoOwner;
  if (owner === null || owner.scope.spaceId !== artifactScope(state)?.spaceId || owner.scope.documentId !== state.config.documentId || owner.clientInstanceId !== state.openClientInstanceId) return;
  inferenceApprovalUndoOwner = null;
  owner.abort.abort(new Error("gis map approval undo owner rebootstrap"));
  approvalUndoStatus(owner, { phase: "unavailable", canUndo: false, code: null });
  if (inferenceApprovalUndoEpoch >= Number.MAX_SAFE_INTEGER) return;
  inferenceApprovalUndoOwner = {
    historyEpoch: ++inferenceApprovalUndoEpoch,
    scope: structuredClone(owner.scope),
    clientInstanceId: owner.clientInstanceId,
    receipt: structuredClone(owner.receipt),
    idempotencyKey: owner.idempotencyKey,
    sourceCatalogGenerationId: owner.sourceCatalogGenerationId,
    sourceComponentSha256: owner.sourceComponentSha256,
    sourceDescriptorSha256: owner.sourceDescriptorSha256,
    sourceBrowserActorSha256: owner.sourceBrowserActorSha256,
    sourceDirectoryRevision: owner.sourceDirectoryRevision,
    sourceMembershipGeneration: owner.sourceMembershipGeneration,
    ...(owner.sourceSessionGeneration === undefined ? {} : { sourceSessionGeneration: owner.sourceSessionGeneration }),
    ...(owner.sourceShareGeneration === undefined ? {} : { sourceShareGeneration: owner.sourceShareGeneration }),
    abort: new AbortController(),
    mount: null,
    phase: "awaiting-mount",
    retryable: true,
  };
}

function bindInferenceApprovalUndoToMountedPair(state: ArtifactState, reservation: DocumentBrowserActorReservation, pair: VerifiedColdDocumentPair): void {
  const owner = inferenceApprovalUndoOwner;
  const lease = state.executionTargetLease;
  if (owner === null || owner.phase !== "awaiting-mount" || lease === null || state.browserActorReservation !== reservation || state.verifiedColdPair !== pair) return;
  const scope = artifactScope(state);
  if (!scope || scope.spaceId !== owner.scope.spaceId || scope.documentId !== owner.scope.documentId || state.openClientInstanceId !== owner.clientInstanceId) return;
  if (!sameApprovalUndoFrontierV1(owner, pair)) {
    retireInferenceApprovalUndo(owner);
    return;
  }
  const fields = lease.fields();
  if (
    fields.catalog.generationId !== owner.sourceCatalogGenerationId
    || fields.package.componentSha256 !== owner.sourceComponentSha256
    || fields.package.descriptorByteSha256 !== owner.sourceDescriptorSha256
    || fields.browserActor.kind !== "closed-browser-actor"
    || fields.browserActor.sha256 !== owner.sourceBrowserActorSha256
    || fields.revalidation.directoryRevision !== owner.sourceDirectoryRevision
    || fields.revalidation.membershipGeneration !== owner.sourceMembershipGeneration
    || fields.revalidation.sessionGeneration !== owner.sourceSessionGeneration
    || fields.revalidation.shareGeneration !== owner.sourceShareGeneration
  ) {
    retireInferenceApprovalUndo(owner);
    return;
  }
  owner.mount = Object.freeze({
    activationGeneration: reservation.generation,
    catalogGenerationId: fields.catalog.generationId,
    componentSha256: fields.package.componentSha256,
    descriptorSha256: fields.package.descriptorByteSha256,
    browserActorSha256: fields.browserActor.sha256,
    directoryRevision: fields.revalidation.directoryRevision,
    membershipGeneration: fields.revalidation.membershipGeneration,
    ...(fields.revalidation.sessionGeneration === undefined ? {} : { sessionGeneration: fields.revalidation.sessionGeneration }),
    ...(fields.revalidation.shareGeneration === undefined ? {} : { shareGeneration: fields.revalidation.shareGeneration }),
  });
  owner.phase = "available";
  owner.retryable = true;
  approvalUndoStatus(owner, { phase: "available", canUndo: true, code: null });
}

function retainInferenceApprovalUndo(operation: InferenceOperationV1, receipt: GisMapInferenceApprovalReceiptV1): void {
  const state = artifactState(operation.scope.documentId, operation.scope.spaceId);
  const lease = state?.executionTargetLease;
  const fields = lease?.live ? lease.fields() : null;
  if (!receipt.applied || receipt.undo.expectedCurrent.documentId !== operation.scope.documentId || state === undefined || fields === null || state.openClientInstanceId.length === 0 || fields.browserActor.kind !== "closed-browser-actor") throw new Error("gis map approval undo: invalid source owner");
  if (inferenceApprovalUndoOwner !== null) retireInferenceApprovalUndo(inferenceApprovalUndoOwner);
  if (inferenceApprovalUndoEpoch >= Number.MAX_SAFE_INTEGER) throw new Error("gis map approval undo: epoch exhausted");
  const owner: InferenceApprovalUndoOwnerV1 = {
    historyEpoch: ++inferenceApprovalUndoEpoch,
    scope: structuredClone(operation.scope),
    clientInstanceId: state.openClientInstanceId,
    receipt: structuredClone(receipt),
    idempotencyKey: mintInferenceApprovalUndoIdempotencyKeyV1(),
    sourceCatalogGenerationId: fields.catalog.generationId,
    sourceComponentSha256: fields.package.componentSha256,
    sourceDescriptorSha256: fields.package.descriptorByteSha256,
    sourceBrowserActorSha256: fields.browserActor.sha256,
    sourceDirectoryRevision: fields.revalidation.directoryRevision,
    sourceMembershipGeneration: fields.revalidation.membershipGeneration,
    ...(fields.revalidation.sessionGeneration === undefined ? {} : { sourceSessionGeneration: fields.revalidation.sessionGeneration }),
    ...(fields.revalidation.shareGeneration === undefined ? {} : { sourceShareGeneration: fields.revalidation.shareGeneration }),
    abort: new AbortController(),
    mount: null,
    phase: "awaiting-mount",
    retryable: true,
  };
  inferenceApprovalUndoOwner = owner;
  state.browserActorReservation?.bindApprovalUndoIfMounted();
}

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
  return browserBrokerFetch(
    `/_semio/hub${path}`,
    {
      method: init.method,
      ...(init.body === undefined ? {} : { headers: { "content-type": "application/json" }, body: init.body }),
    },
    { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: operation.abort.signal },
  );
}

async function inferenceApprovalUndoBrokerFetch(owner: InferenceApprovalUndoOwnerV1, body: string): Promise<FetchTimeoutResponse> {
  const state = artifactState(owner.scope.documentId, owner.scope.spaceId);
  if (state === undefined || state.closed || state.openClientInstanceId !== owner.clientInstanceId || state.docAbort.signal.aborted) throw new Error("gis map approval undo: document closed");
  return browserBrokerFetch(
    `/_semio/hub${inferenceJobPath(owner.scope, "/approval-undos")}`,
    { method: "POST", headers: { "content-type": "application/json" }, body },
    { timeoutMs: SOCKET_GRANT_REQUEST_TIMEOUT_MS, signal: owner.abort.signal },
  );
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
    operation === null ||
    operation.status.phase !== "offered" ||
    operation.status.jobId === null ||
    operation.status.proposalHash === null ||
    operation.status.preview?.jobId !== operation.status.jobId ||
    operation.status.preview.proposalHash !== operation.status.proposalHash ||
    operation.inFlight
  )
    return;
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
    retainInferenceApprovalUndo(operation, receipt);
    advanceInferencePort(operation, { kind: "approval", receipt });
    retireInferencePort(operation);
  } catch (error) {
    if (operation.closed) return;
    terminateInferencePort(operation, inferenceCodeFromRejection(error, operation.abort.signal.aborted));
  } finally {
    operation.inFlight = false;
  }
}

/** ↩️ Routes the ordinary Shell history action through the one exact mounted durable approval
 * owner. The retry key and Hub handle stay stable and private across an indeterminate response. */
async function undoInferenceApproval(historyEpoch: number, clientInstanceId: string, scope: DocumentScope): Promise<void> {
  const owner = inferenceApprovalUndoOwner;
  if (owner === null || owner.historyEpoch !== historyEpoch || owner.clientInstanceId !== clientInstanceId || owner.scope.spaceId !== scope.spaceId || owner.scope.documentId !== scope.documentId || (owner.phase !== "available" && !(owner.phase === "failed" && owner.retryable))) return;
  const state = artifactState(scope.documentId, scope.spaceId);
  if (state === undefined || !sameApprovalUndoMountV1(state, owner)) {
    retireInferenceApprovalUndo(owner);
    return;
  }
  owner.phase = "submitting";
  approvalUndoStatus(owner, { phase: "submitting", canUndo: false, code: null });
  try {
    const request = sealGisMapApprovalUndoRequestV1(owner.receipt.undo, owner.idempotencyKey);
    const response = await inferenceApprovalUndoBrokerFetch(owner, JSON.stringify(request));
    if (!response.ok) throw new DirectoryHttpError(response.status, "");
    const receipt: GisMapApprovalUndoReceiptV1 = parseGisMapApprovalUndoReceiptV1(await readInferenceJson(response));
    if (inferenceApprovalUndoOwner !== owner) return;
    const currentState = artifactState(owner.scope.documentId, owner.scope.spaceId);
    if (currentState === undefined || !sameApprovalUndoMountV1(currentState, owner)) {
      retireInferenceApprovalUndo(owner);
      return;
    }
    if (!receipt.applied || receipt.targetId !== owner.receipt.undo.targetId || receipt.originalJobId !== owner.receipt.jobId || receipt.frontier.documentId !== owner.scope.documentId) throw new Error("gis map approval undo: receipt mismatch");
    approvalUndoStatus(owner, { phase: "applied", canUndo: false, code: null });
    inferenceApprovalUndoOwner = null;
    owner.abort.abort(new Error("gis map approval undo applied"));
  } catch (error) {
    if (inferenceApprovalUndoOwner !== owner || owner.abort.signal.aborted) return;
    const status = error instanceof DirectoryHttpError ? error.status : undefined;
    const code = status === undefined ? "inference.transport" : gisMapInferenceCodeFromStatusV1(status);
    owner.phase = "failed";
    owner.retryable = status === undefined || status === 429 || status === 503;
    approvalUndoStatus(owner, { phase: "failed", canUndo: owner.retryable, code });
    if (!owner.retryable) {
      inferenceApprovalUndoOwner = null;
      owner.abort.abort(new Error("gis map approval undo rejected"));
    }
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
function openArtifact(request: ArtifactActorConfig & { readonly clientInstanceId?: string }): void {
  const config: ArtifactActorConfig = request;
  const hub = hubBinding(config);
  const runtimeKey = documentRuntimeKeyForConfig(config);
  retireArtifactBeforeReplacement(runtimeKey);
  const channel = new BroadcastChannel(`semio-doc-${runtimeKey}`);
  const state: ArtifactState = {
    runtimeKey,
    config,
    openClientInstanceId: request.clientInstanceId ?? crypto.randomUUID(),
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
    browserActorViewState: null,
    sanityPollTimer: null,
    sseHealthy: false,
    revalidateFolder: async () => {}, // 🔧 replaced below once a folder binding exists.
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
    hlcCounter: 0,
    closed: false,
  };
  artifacts.set(runtimeKey, state);
  channel.onmessage = (messageEvent) => {
    const value = messageEvent.data;
    if (typeof value === "object" && value !== null && !Array.isArray(value) && (value as { kind?: unknown }).kind === "documentBackbone" && (value as { message?: unknown }).message instanceof Uint8Array) {
      const parsed = parseDocumentBackboneMessage((value as { message: Uint8Array }).message);
      if (parsed.envelopes.length > 0 && parsed.envelopes.every((envelope) => envelope.document_id === state.config.documentId)) emitEvent(state, { kind: "documentBackbone", message: parsed.message });
      return;
    }
    const envelopes = value as MutationEnvelope[];
    if (Array.isArray(envelopes) && envelopes.length > 0) emitEvent(state, { kind: "remoteMutations", envelopes });
  };
  const folder = folderBinding(config);
  if (folder) {
    // 🥇️ One single-flight guard per document (finding 1), shared by every trigger source.
    state.revalidateFolder = latestWins(() => pollFolderOnce(state, folder));
    if (config.watchExternal !== false) watchFolder(state, folder);
    else void state.revalidateFolder();
  }
  if (hub && hub.requestedSurfaceId === undefined && hub.installedTarget === undefined && socketGrantTestIssue === null) {
    const scope = artifactScope(state);
    post({ kind: "socket-actor-failed", documentId: config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), code: "installed-target-unavailable" });
  } else if (hub) {
    connectHub(state, hub);
  }
  emitEvent(state, { kind: "status", ...state.status });
}

function retireArtifactBeforeReplacement(runtimeKey: string): void {
  if (!artifacts.has(runtimeKey)) return;
  closeArtifactRuntime(runtimeKey);
  if (artifacts.has(runtimeKey)) throw new Error("artifact replacement: prior owner did not retire");
}

function closeArtifactRuntime(runtimeKey: string): void {
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
  if (inferencePort !== null && documentRuntimeKeyV1({ kind: "hub", ...inferencePort.scope }) === runtimeKey) closeInferencePort(inferencePort.operationEpoch);
  if (inferenceApprovalUndoOwner !== null && documentRuntimeKeyV1({ kind: "hub", ...inferenceApprovalUndoOwner.scope }) === runtimeKey) retireInferenceApprovalUndo(inferenceApprovalUndoOwner);
  dropDocumentExecutionTargetLease(state);
  state.socket?.close();
  if (state.sanityPollTimer != null) clearTimeout(state.sanityPollTimer);
  state.channel.close();
  state.exactLocalEnvelopes = new WeakMap();
  state.pendingDocumentBackboneBytes = 0;
  state.pendingDocumentBackboneMessages = 0;
  artifacts.delete(runtimeKey);
}

function closeArtifact(documentId: string, spaceId?: string, clientInstanceId?: string): void {
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
): void {
  if (envelopes.length === 0) return;
  if (exactEnvelopes !== null && exactEnvelopes.length !== envelopes.length) throw new Error("document backbone: exact batch cardinality mismatch");
  if (state.executionTargetLease !== null && state.executionTargetLease.live && !state.executionTargetLease.fields().grant.write) {
    rejectReadOnlyExecutionTarget(state, envelopes);
    return;
  }
  if (exactEnvelopes !== null && (!Number.isSafeInteger(exactMessageBytes) || exactMessageBytes < 1 || exactMessageBytes > DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes)) throw new Error("document backbone: invalid retained byte charge");
  if (exactEnvelopes !== null && (state.pendingDocumentBackboneMessages >= DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumMessages || exactMessageBytes > DOCUMENT_BACKBONE_RETENTION_LIMITS.maximumBytes - state.pendingDocumentBackboneBytes)) {
    rejectDocumentBackboneCapacity(state, envelopes, exactMessageBytes);
    return;
  }
  if (state.pendingMutations.length + envelopes.length > PENDING_MUTATIONS_QUEUE_LIMIT) {
    rejectMutationQueueOverflow(state, envelopes);
    return;
  }
  if (exactEnvelopes !== null) {
    envelopes.forEach((envelope, index) => state.exactLocalEnvelopes.set(envelope, { envelope: exactEnvelopes[index]!, bytes: index === 0 ? exactMessageBytes : 0, messages: index === 0 ? 1 : 0 }));
    state.pendingDocumentBackboneBytes += exactMessageBytes;
    state.pendingDocumentBackboneMessages += 1;
  }
  state.pendingMutations.push(...envelopes);
  setStatus(state, { pendingMutations: state.pendingMutations.length });
  if (exactEnvelopes === null || !hubBinding(state.config)) state.channel.postMessage(channelMessage);
  relayMutationsToHub(state, envelopes);
  if (folderBinding(state.config)) setStatus(state, { persisted: false });
}

async function handleLocalMsg(state: ArtifactState, message: ArtifactActorMsg): Promise<void> {
  switch (message.kind) {
    case "documentBackbone": {
      const parsed = parseDocumentBackboneMessage(message.message);
      if (parsed.envelopes.some((envelope) => envelope.document_id !== state.config.documentId)) {
        const batchId = nextLocalOverflowBatchId;
        nextLocalOverflowBatchId -= 1;
        emitEvent(state, { kind: "commandOutcome", batchId, outcome: { kind: "rejected", reason: "document backbone scope mismatch", messages: [parsed.envelopes.length] } });
        break;
      }
      admitLocalMutations(state, parsed.envelopes.map(fromWireEnvelope), { kind: "documentBackbone", message: parsed.message }, parsed.envelopes, parsed.message.byteLength);
      break;
    }
    case "localMutations": {
      admitLocalMutations(state, message.envelopes, message.envelopes);
      break;
    }
    case "localSnapshot": {
      const folder = folderBinding(state.config);
      if (folder) {
        try {
          await writeFolder(state, folder, message.pack, message.spr);
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
//#endregion 🔖️Lifecycle

//#region 🔖️MessageBridge
function handleTsRequest(request: BackboneWorkerRequest): void {
  switch (request.kind) {
    case "open":
      openArtifact(request);
      break;
    case "close":
      closeArtifact(request.documentId, request.spaceId, request.clientInstanceId);
      break;
    case "send": {
      const state = artifactState(request.documentId, request.spaceId);
      if (state && request.clientInstanceId === state.openClientInstanceId) void handleLocalMsg(state, request.message);
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
    case "space-artifact-creation-catalog-open":
      void openSpaceArtifactCreationCatalog(request.spaceId, request.clientInstanceId);
      break;
    case "space-artifact-create":
      submitSpaceArtifactCreation(request);
      break;
    case "space-artifact-create-cancel":
      cancelSpaceArtifactCreation(request.requestId, request.spaceId);
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
    case "inference-history-undo":
      void undoInferenceApproval(request.historyEpoch, request.clientInstanceId, request.scope);
      break;
    case "browser-actor-view-state": {
      const context = parseBrowserActorViewStateRequest(request);
      const state = artifactState(context.scope.documentId, context.scope.spaceId);
      if (state?.openClientInstanceId === context.clientInstanceId && JSON.stringify(state.browserActorViewState) !== JSON.stringify(context.viewState)) {
        state.browserActorViewState = context.viewState;
        state.browserActorReservation?.refreshHostView();
      }
      break;
    }
    case "browser-actor-ui-patch-result": {
      const state = artifactState(request.scope.documentId, request.scope.spaceId);
      if (state?.openClientInstanceId === request.clientInstanceId) state.browserActorReservation?.settleUiPatch(request);
      break;
    }
  }
}
//#endregion 🔖️MessageBridge
//#endregion 🔖️TsFallback

//#region 🧪️Tests
// 🧵️ Whole block stripped from production builds (see this file's header doc) — `node:*` imports
// below are dynamic specifically so they never get bundled into the actual browser Worker script.
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts");
  const testSeams = {
    get directoryAdministration() { return directoryAdministration; },
    set directoryAdministration(value: typeof directoryAdministration) { directoryAdministration = value; },
    get directoryClient() { return directoryClient; },
    set directoryClient(value: typeof directoryClient) { directoryClient = value; },
    get directorySessionEpoch() { return directorySessionEpoch; },
    set directorySessionEpoch(value: typeof directorySessionEpoch) { directorySessionEpoch = value; },
    get executionTargetStatusObserver() { return executionTargetStatusObserver; },
    set executionTargetStatusObserver(value: typeof executionTargetStatusObserver) { executionTargetStatusObserver = value; },
    get inferenceApprovalUndoEpoch() { return inferenceApprovalUndoEpoch; },
    set inferenceApprovalUndoEpoch(value: typeof inferenceApprovalUndoEpoch) { inferenceApprovalUndoEpoch = value; },
    get inferenceApprovalUndoOwner() { return inferenceApprovalUndoOwner; },
    set inferenceApprovalUndoOwner(value: typeof inferenceApprovalUndoOwner) { inferenceApprovalUndoOwner = value; },
    get localBrowserBrokerProofExpiresAtMs() { return localBrowserBrokerProofExpiresAtMs; },
    set localBrowserBrokerProofExpiresAtMs(value: typeof localBrowserBrokerProofExpiresAtMs) { localBrowserBrokerProofExpiresAtMs = value; },
    get localBrowserBrokerQueued() { return localBrowserBrokerQueued; },
    set localBrowserBrokerQueued(value: typeof localBrowserBrokerQueued) { localBrowserBrokerQueued = value; },
    get socketGrantTestIssue() { return socketGrantTestIssue; },
    set socketGrantTestIssue(value: typeof socketGrantTestIssue) { socketGrantTestIssue = value; },
    get spaceArtifactCreationTestFetch() { return spaceArtifactCreationTestFetch; },
    set spaceArtifactCreationTestFetch(value: typeof spaceArtifactCreationTestFetch) { spaceArtifactCreationTestFetch = value; },
    get workerPostTestSink() { return workerPostTestSink; },
    set workerPostTestSink(value: typeof workerPostTestSink) { workerPostTestSink = value; },
  };
  await registerTests1(import.meta.vitest, { testSeams, DOCUMENT_BACKBONE_RETENTION_LIMITS, handleAck, ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES, ArtifactBootstrapAssembler, DIRECTORY_COMMAND_TRANSPORT_CAPACITY, DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1, DirectoryClient, DirectoryEventPageBootstrapV1, DocumentExecutionTargetLease, HUB_RECONNECT_MAX_MS, IDENTITY_CONFIG_SCHEMA, PENDING_MUTATIONS_QUEUE_LIMIT, SANITY_POLL_MIN_MS, SSE_RECONNECT_MAX_MS, SUSTAINED_HEALTHY_MS, VerifiedColdDocumentPair, abortArtifactBootstrap, artifactBootstrapFailure, artifactState, artifacts, bindInferenceApprovalUndoToMountedPair, browserActorChildCapacity, browserBrokerFetch, browserBrokerProofDigest, browserDirectoryRequest, browserExecutionTargetAssetRequest, bytesHex, clearLocalBrowserBrokerProof, closeArtifact, closeArtifactRuntime, closeDirectory, connectHubOnce, decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, decodeClientFrame, decodePackPayload, decodePackValue, decodeServerFrame, directoryAdministration, directoryClient, directoryCommandOperations, directoryCommandQueue, directoryCommandSha256, directorySessionEpoch, directoryWorkerEpoch, dispatchBackboneWorkerRequest, documentExecutionOwners, documentExecutionTargetLeaseMintToken, documentExecutionTargetStatusRoleV1, documentOpenPlanAuthority, documentRuntimeKeyForConfig, documentRuntimeKeyV1, driveInferencePort, dropDocumentExecutionTargetLease, dropVerifiedColdDocumentPair, emitEvent, encodeActorUiPatchReceipt, encodeBackboneMessage, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse, encodeDocumentBackboneEnvelopeBatchExact, encodePackValue, encodeServerFrame, executionTargetHex, executionTargetSha256Hex, executionTargetStatusObserver, extractServerCommandsDocumentBackboneBatchExact, flushDirectoryQueue, foldIdentityEvent, fromWireEnvelope, handleHubFrame, handleTsRequest, hexBytes, hubBinding, identityActorConfig, idleGisMapInferencePortStatusV1, inferenceApprovalUndoEpoch, inferenceApprovalUndoOwner, installLocalBrowserBrokerProof, localBrowserBrokerProofExpiresAtMs, localBrowserBrokerQueued, openArtifact, ownedArrayBuffer, parseDocumentBackboneMessage, parseDocumentExecutionTargetLeaseFieldsV1, parseGisMapInferenceApprovalReceiptV1, queueOutbox, readExecutionTargetBody, reissueInferenceApprovalUndoForRebootstrap, relayMutationsToHub, requestDocumentSocketAuthority, reserveDocumentBrowserActorChild, retainInferenceApprovalUndo, revokeDirectoryAdministrationForScope, rollbackEnvelope, sameLeaseFieldsV1, scopedDirectoryStreams, sealDirectoryCommandReceiptV1, sealDirectoryCommandRequestV1, settleDirectoryCommand, socketGrantTestIssue, spaceArtifactCreationCatalogOperations, spaceArtifactCreationOperations, spaceArtifactCreationTestFetch, stampSession, toWireEnvelope, undoInferenceApproval, verifiedColdDocumentPairMintToken, verifyBrowserActorDescribeV1, workerPostTestSink }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🧪️Tests
