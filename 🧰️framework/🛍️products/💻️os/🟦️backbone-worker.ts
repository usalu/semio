// #region Header
/**
 * @emoji 🧵️ `🟦️backbone-worker.ts` — thin loader for the Rust WASM `store_worker`
 * actor (`store/worker/rs`). When the wasm package is unavailable (vitest/node), falls
 * back to the embedded TypeScript actor twin so dev workflows keep working.
 */
// #endregion Header

import type { ArtifactPresencePeer, ClientFrame, MutationEnvelope, ServerFrame, WireAckStage, WireFrontierSummary, WireLane, WireMutationEnvelope } from "@semio-tech/framework-replication";
import type { ArtifactActorConfig, ArtifactActorMsg, ArtifactEvent, ArtifactSyncStatus, BackboneWorkerRequest, BackboneWorkerResponse, BackboneWorkerWireMessage, CommandAckOutcome, DirectoryCommand, DirectoryStreamMessage, PersistenceBinding, RemoteState } from "./🟦️component";
import { decodeClientFrame, decodePresencePeer, decodeServerFrame, encodeClientFrame, encodePresencePeer, encodeServerFrame } from "@semio-tech/framework-replication";
import { DirectoryClient, HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS, decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, decodeDocumentPackBytes, decodePackValue, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse, encodeDocumentPackBytes, encodePackValue } from "./🟦️component";
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
import type { Identity } from "./🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️component";

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

function isBackboneWorkerWireMessage(message: unknown): message is BackboneWorkerWireMessage {
  return typeof message === "object" && message !== null && "wire" in message && (message as BackboneWorkerWireMessage).wire instanceof Uint8Array;
}

function decodeWorkerRequest(message: BackboneWorkerWireMessage): BackboneWorkerRequest {
  return decodeBackboneWorkerRequest(message.wire);
}

const workerScope = typeof self !== "undefined" && !Reflect.has(self, "document") ? self : null;

if (workerScope) {
  workerScope.onmessage = (messageEvent: MessageEvent<unknown>) => {
    // 🛡️ React DevTools and other injectors postMessage into every Worker; ignore non-wire traffic.
    if (!isBackboneWorkerWireMessage(messageEvent.data)) return;
    const request = decodeWorkerRequest(messageEvent.data);
    void rustHostPromise.then((host) => {
      if (host) {
        host.handleRequestBytes(encodeBackboneWorkerRequest(request));
        return;
      }
      handleTsRequest(request);
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
// 🔁️ HUB_RECONNECT_MIN_MS/MAX_MS moved to `🟦️component.ts`'s `🔖️HubBinding` region (imported above)
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
 * `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts` (outside this packet's owned path).
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
  config: ArtifactActorConfig;
  channel: BroadcastChannel;
  socket: WebSocket | null;
  /** 🛑️ Aborted once, in {@link closeArtifact} — cancels every in-flight folder/blob fetch this
   * document owns and unblocks any pending {@link retryWithJitteredBackoff} delay for its hub/SSE
   * reconnect loops immediately (finding 3). Never re-created; a closed document stays closed. */
  docAbort: AbortController;
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

function post(message: BackboneWorkerResponse): void {
  workerScope?.postMessage({ wire: encodeBackboneWorkerResponse(message) });
}

function emitEvent(documentId: string, event: ArtifactEvent): void {
  post({ kind: "event", documentId, event });
}

function setStatus(state: ArtifactState, patch: Partial<ArtifactSyncStatus>): void {
  state.status = { ...state.status, ...patch };
  emitEvent(state.config.documentId, { kind: "status", ...state.status });
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
 * too (`🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️component.ts`), so a `remoteMutations` envelope's
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
  return { actor: actorSeed(state.config.actor), physical_ms: Date.now(), logical: state.hlcCounter };
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
function toWireEnvelope(envelope: MutationEnvelope, timestamp: WireMutationEnvelope["timestamp"]): WireMutationEnvelope {
  return {
    mutation_id: envelope.id,
    document_id: envelope.document,
    actor: envelope.actor,
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
  return { ...peer, color: state.sessionColor ?? undefined, surface: hubBinding(state.config)?.surface };
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
    emitEvent(state.config.documentId, { kind: "snapshotReplaced", pack: Array.from(pack), spr: Array.from(spr) });
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
function connectHubOnce(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub" }>): Promise<void> {
  return new Promise<void>((resolve, reject) => {
    if (state.docAbort.signal.aborted) {
      reject(state.docAbort.signal.reason ?? new Error("backbone-worker: document closed"));
      return;
    }
    setRemote(state, { kind: "connecting" });
    const wsBase = binding.baseUrl.replace(/^http/, "ws");
    // 📡️ Presence scope (contract §C0) travels out of band as `?surface=` — no `PresencePeer` wire
    // change (its flag byte is full and the file is peer-leased).
    const surfaceQuery = binding.surface ? `?surface=${encodeURIComponent(binding.surface)}` : "";
    const socket = new WebSocket(`${wsBase}/spaces/${encodeURIComponent(binding.spaceId)}/documents/${encodeURIComponent(state.config.documentId)}/ws${surfaceQuery}`);
    // 🎞️ Binary frames (`protocol_wire`), not JSON text — see this file's header + `WireBridge` region.
    socket.binaryType = "arraybuffer";
    state.socket = socket;
    let sustainedHealthTimer: ReturnType<typeof setTimeout> | null = null;
    let sustainedHealthReached = false;
    const onAbort = (): void => socket.close();
    state.docAbort.signal.addEventListener("abort", onAbort, { once: true });
    socket.onopen = () => {
      state.reconnectDelayMs = HUB_RECONNECT_MIN_MS;
      sustainedHealthTimer = setTimeout(() => {
        sustainedHealthReached = true;
      }, SUSTAINED_HEALTHY_MS);
      sendWireFrame(state, {
        Hello: {
          wire_version: 1,
          protocol_version: 1,
          schema: state.config.schema,
          // 🧬️ W5.7: real hash when the shell supplied one via `ArtifactActorConfig.packSchemaHash`
          // (from the wasm renderer's `document_pack_schema_hash` export); zeros otherwise, which the
          // hub treats as "schema-agnostic client" and never validates.
          pack_schema_hash: [...(state.config.packSchemaHash ?? new Array(32).fill(0))],
          actor: state.config.actor,
          token: binding.token ?? null,
          resume_token: state.resumeToken,
          frontier: state.frontier,
        },
      }, "command");
    };
    socket.onmessage = (messageEvent) => {
      try {
        const bytes = new Uint8Array(messageEvent.data as ArrayBuffer);
        handleHubFrame(state, decodeServerFrame(bytes).frame);
      } catch (error) {
        console.error("[backbone-worker] malformed hub frame", state.config.documentId, error);
      }
    };
    socket.onclose = () => {
      state.docAbort.signal.removeEventListener("abort", onAbort);
      if (sustainedHealthTimer != null) clearTimeout(sustainedHealthTimer);
      if (state.socket === socket) state.socket = null;
      for (const envelopes of state.pendingBatches.values()) state.outbox.push(...envelopes);
      state.pendingBatches.clear();
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
 * {@link handleHubFrame}'s `Welcome` branch flushes that outbox (calling this function again) the
 * moment the hub is reachable again, so nothing queued offline is lost or left unsent. */
function relayMutationsToHub(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  if (envelopes.length === 0) return;
  if (state.socket?.readyState !== WebSocket.OPEN) {
    state.outbox.push(...envelopes);
    return;
  }
  const batchId = state.nextBatchId;
  state.nextBatchId += 1;
  const wireEnvelopes = envelopes.map((envelope) => toWireEnvelope(envelope, nextWireTimestamp(state)));
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
function rejectMutationQueueOverflow(state: ArtifactState, envelopes: readonly MutationEnvelope[]): void {
  const batchId = nextLocalOverflowBatchId;
  nextLocalOverflowBatchId -= 1;
  console.error("[backbone-worker] pending mutation queue full, rejecting batch", state.config.documentId, envelopes.length);
  emitEvent(state.config.documentId, {
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
      if (rollbacks.length > 0) emitEvent(state.config.documentId, { kind: "remoteMutations", envelopes: rollbacks });
      const converted = fromWireEnvelope(outcome.Transformed.envelope);
      emitEvent(state.config.documentId, { kind: "remoteMutations", envelopes: [converted] });
      ackOutcome = { kind: "transformed" };
    } else {
      const rollbacks = [...sent].reverse().map(rollbackEnvelope);
      if (rollbacks.length > 0) emitEvent(state.config.documentId, { kind: "remoteMutations", envelopes: rollbacks });
      ackOutcome = { kind: "rejected", reason: outcome.Rejected.reason, messages: outcome.Rejected.messages };
    }
    setStatus(state, { pendingMutations: state.pendingMutations.length });
    emitEvent(state.config.documentId, { kind: "commandOutcome", batchId, outcome: ackOutcome });
  }
}

function handleHubFrame(state: ArtifactState, frame: ServerFrame): void {
  if (typeof frame === "string") return; // no unit-variant `ServerFrame` exists today; defensive.
  if ("Welcome" in frame) {
    state.resumeToken = frame.Welcome.resume_token;
    state.frontier = frame.Welcome.server_frontier;
    // 📡️ `Welcome` no longer carries a presence roster (wire v2 splits it into its own `Presence`
    // frame) — `peerCount` is corrected once that frame arrives.
    setRemote(state, { kind: "live", peerCount: 0 });
    // 📦️ Pack-based snapshot bootstrap (`Welcome.bootstrap.Snapshot`): no client-side pack decoder
    // wired this wave (db/pack integration is a CW6+ hub-rebuild concern, mirrors the Rust actor's
    // identical deferral) — accepted and ignored; catch-up relies on the hub's follow-up `Commands`.
    // ♻️ Finding 5's "flushed on reconnect" half: anything queued while the hub was unreachable
    // (offline edits, or a prior batch whose dead socket never delivered its `Ack`) is relayed now
    // that the handshake succeeded — never left stranded waiting for another local edit to trigger it.
    if (state.outbox.length > 0) relayMutationsToHub(state, state.outbox.splice(0));
    return;
  }
  if ("SnapshotChunk" in frame || "SnapshotDone" in frame) {
    // 📦️ See the `Welcome.bootstrap.Snapshot` note above — accepted and ignored.
    return;
  }
  if ("Commands" in frame) {
    state.frontier = frame.Commands.frontier;
    if (frame.Commands.origin !== state.config.actor) {
      const envelopes = frame.Commands.envelopes.map(fromWireEnvelope);
      emitEvent(state.config.documentId, { kind: "remoteMutations", envelopes });
    }
    return;
  }
  if ("Ack" in frame) {
    state.frontier = frame.Ack.frontier;
    handleAck(state, frame.Ack.batch_id, frame.Ack.stages);
    return;
  }
  if ("Preview" in frame) {
    if (frame.Preview.actor !== state.config.actor) emitEvent(state.config.documentId, { kind: "preview", actor: frame.Preview.actor, key: frame.Preview.key, seq: frame.Preview.seq, payload: frame.Preview.payload });
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
    emitEvent(state.config.documentId, { kind: "presence", peers });
    return;
  }
  if ("CreditGrant" in frame) {
    // 🪙️ Command-lane credit-based flow control: no client-side backpressure implemented this wave
    // (scope is frame plumbing, not congestion control) — accepted and ignored.
    return;
  }
  if ("Session" in frame) {
    // 🎨️ Flows through the SAME generic `{kind:"event",...}` wrapping every other `ArtifactEvent`
    // gets — the real wasm host (`👷️worker/🦀️component.rs`) wraps every `ArtifactEvent` uniformly
    // with zero per-variant special-casing, so this fallback must match rather than post a
    // one-off top-level `BackboneWorkerResponse` shape the wasm path would never produce.
    state.sessionColor = frame.Session.color;
    emitEvent(state.config.documentId, { kind: "session", actor: frame.Session.actor, color: frame.Session.color });
    return;
  }
  if ("Error" in frame) {
    emitEvent(state.config.documentId, { kind: "conflict", message: frame.Error.message });
  }
}
//#endregion 🔖️Hub

//#region 🔖️Directory
/** 📇️ Directory hub lane (contract §C6) — the shell's only path to the directory control plane;
 * plugin surfaces never talk to the network, and the shell never opens a directory socket on the UI
 * thread. Owns exactly one {@link DirectoryClient}/{@link DirectoryStream} at a time. Reuses that
 * client's own reconnect/backoff (`🔖️HubBinding` in `🟦️component.ts`) rather than a second loop
 * here — this region's only extra responsibility is the offline command queue. */
const DIRECTORY_COMMAND_QUEUE_LIMIT = 200;

type QueuedDirectoryCommand = { requestId: string; command: DirectoryCommand };

let directoryClient: DirectoryClient | null = null;
let directoryStream: { close: () => void } | null = null;
let directoryFlushing = false;
const directoryCommandQueue: QueuedDirectoryCommand[] = [];

function directoryStatus(): BackboneWorkerResponse {
  return { kind: "directory-status", pendingCommands: directoryCommandQueue.length };
}

function openDirectory(baseUrl: string, token: string | undefined, since: number): void {
  closeDirectory();
  const client = new DirectoryClient(baseUrl, token);
  directoryClient = client;
  directoryStream = client.stream(since, (message: DirectoryStreamMessage) => {
    post({ kind: "directory-message", message });
    void flushDirectoryQueue();
  });
  post(directoryStatus());
}

function closeDirectory(): void {
  directoryStream?.close();
  directoryStream = null;
  directoryClient = null;
  directoryCommandQueue.length = 0;
}

/** 🗃️ Pushes a command onto the bounded offline queue, dropping the OLDEST entry past
 * {@link DIRECTORY_COMMAND_QUEUE_LIMIT} (logged, never silently) — a full queue means a very long
 * outage, and the newest intent is more likely still relevant than the oldest. */
function enqueueDirectoryCommand(requestId: string, command: DirectoryCommand): void {
  directoryCommandQueue.push({ requestId, command });
  while (directoryCommandQueue.length > DIRECTORY_COMMAND_QUEUE_LIMIT) {
    const dropped = directoryCommandQueue.shift();
    console.error("[backbone-worker] directory command queue full, dropped oldest", dropped?.requestId);
  }
  post(directoryStatus());
}

/** 🚨️ `true` for a {@link DirectoryHttpError}-shaped rejection (the hub answered and rejected the
 * command — authz/validation, never retried); `false` for anything else (network failure — queue
 * and retry). Structural rather than an `instanceof DirectoryHttpError` check, since this file's
 * `DirectoryClient` import and the wasm host's own may not share a class identity. */
function directoryRejectionStatus(error: unknown): number | undefined {
  return typeof error === "object" && error !== null && "status" in error && typeof (error as { status: unknown }).status === "number" ? (error as { status: number }).status : undefined;
}

async function submitDirectoryCommand(requestId: string, command: DirectoryCommand): Promise<void> {
  const client = directoryClient;
  if (!client) {
    enqueueDirectoryCommand(requestId, command);
    return;
  }
  try {
    const result = await client.command(command);
    post({ kind: "directory-command-result", requestId, ok: true, events: result.events });
  } catch (error) {
    const status = directoryRejectionStatus(error);
    if (status !== undefined) {
      post({ kind: "directory-command-result", requestId, ok: false, error: error instanceof Error ? error.message : String(error) });
      return;
    }
    console.error("[backbone-worker] directory command unreachable, queued for retry", requestId, error);
    enqueueDirectoryCommand(requestId, command);
  }
}

/** ♻️ Retries the queue in order on every live signal from the stream (contract §C6 "flush on
 * reconnect") — stops at the first still-unreachable command so ordering is preserved; a definitive
 * rejection is surfaced and dropped rather than retried forever. Re-entrancy guarded: a burst of
 * stream messages must not run overlapping flushes. */
async function flushDirectoryQueue(): Promise<void> {
  if (directoryFlushing) return;
  directoryFlushing = true;
  try {
    while (directoryCommandQueue.length > 0 && directoryClient) {
      const next = directoryCommandQueue[0]!;
      try {
        const result = await directoryClient.command(next.command);
        directoryCommandQueue.shift();
        post({ kind: "directory-command-result", requestId: next.requestId, ok: true, events: result.events });
      } catch (error) {
        const status = directoryRejectionStatus(error);
        if (status !== undefined) {
          directoryCommandQueue.shift();
          post({ kind: "directory-command-result", requestId: next.requestId, ok: false, error: error instanceof Error ? error.message : String(error) });
          continue;
        }
        break;
      }
    }
  } finally {
    directoryFlushing = false;
    post(directoryStatus());
  }
}
//#endregion 🔖️Directory

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
  closeArtifact(config.documentId);
  const channel = new BroadcastChannel(`semio-doc-${config.documentId}`);
  const state: ArtifactState = {
    config,
    channel,
    socket: null,
    docAbort: new AbortController(),
    sanityPollTimer: null,
    sseHealthy: false,
    revalidateFolder: async () => {}, // 🔧 replaced below once a folder binding exists.
    reconnectDelayMs: HUB_RECONNECT_MIN_MS,
    outbox: [],
    pendingMutations: [],
    status: { persisted: false, pendingMutations: 0, remote: { kind: "detached" } },
    frontier: null,
    resumeToken: null,
    sessionColor: null,
    pendingBatches: new Map(),
    nextBatchId: 0,
    hlcCounter: 0,
    closed: false,
  };
  artifacts.set(config.documentId, state);
  channel.onmessage = (messageEvent) => {
    const envelopes = messageEvent.data as MutationEnvelope[];
    if (Array.isArray(envelopes) && envelopes.length > 0) emitEvent(config.documentId, { kind: "remoteMutations", envelopes });
  };
  const folder = folderBinding(config);
  if (folder) {
    // 🥇️ One single-flight guard per document (finding 1), shared by every trigger source.
    state.revalidateFolder = latestWins(() => pollFolderOnce(state, folder));
    if (config.watchExternal !== false) watchFolder(state, folder);
    else void state.revalidateFolder();
  }
  const hub = hubBinding(config);
  if (hub) connectHub(state, hub);
  emitEvent(config.documentId, { kind: "status", ...state.status });
}

function closeArtifact(documentId: string): void {
  const state = artifacts.get(documentId);
  if (!state) return;
  state.closed = true;
  // 🛑️ Finding 3: cancels every in-flight folder/blob fetch this document owns and unblocks any
  // pending reconnect backoff delay immediately — no fetch or reconnect loop can pin this document
  // after this line.
  state.docAbort.abort();
  state.socket?.close();
  if (state.sanityPollTimer != null) clearTimeout(state.sanityPollTimer);
  state.channel.close();
  artifacts.delete(documentId);
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
      closeArtifact(state.config.documentId);
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
      closeArtifact(request.documentId);
      break;
    case "send": {
      const state = artifacts.get(request.documentId);
      if (state) void handleLocalMsg(state, request.message);
      break;
    }
    case "directory-open":
      openDirectory(request.baseUrl, request.token, request.since);
      break;
    case "directory-command":
      void submitDirectoryCommand(request.requestId, request.command);
      break;
    case "directory-close":
      closeDirectory();
      break;
  }
}
//#endregion 🔖️MessageBridge
//#endregion 🔖️TsFallback

//#region 🧪️Tests
// 🧵️ Whole block stripped from production builds (see this file's header doc) — `node:*` imports
// below are dynamic specifically so they never get bundled into the actual browser Worker script.
if (import.meta.vitest) {
  const { describe, expect, it, vi } = import.meta.vitest;

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

    it("encodeClientFrame/decodeClientFrame round-trip a Hello frame", () => {
      const frame: ClientFrame = { Hello: { wire_version: 1, protocol_version: 1, schema: "demo/v1", pack_schema_hash: new Array(32).fill(0), actor: "actor-1", token: null, resume_token: null, frontier: null } };
      const bytes = encodeClientFrame(frame, "command");
      const decoded = decodeClientFrame(bytes);
      expect(decoded.lane).toBe("command");
      expect(decoded.frame).toEqual(frame);
    });

    // 🎨️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4:
    // `stampSession` is the ONE place `peer.color`/`peer.surface` are ever filled — shells never set
    // them themselves. Overwrites whatever the caller handed in, and derives `surface` from the
    // document's own hub binding (`null`/absent for a folder-only document).
    it("stampSession fills color/surface from actor state, overwriting whatever the caller set", () => {
      const hubConfig: ArtifactActorConfig = { documentId: "doc-1", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1", surface: "s.space.home@1/*#editor" }], actor: "actor-1" };
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
      const state = { config, sessionColor: null } as unknown as ArtifactState;
      handleHubFrame(state, { Session: { actor: "actor-1", color: 3 } });
      expect(state.sessionColor).toBe(3);
    });

  });

  //#region 🔖️IdentityTests
  describe("identity config facet", () => {
    function sampleIdentity(overrides: Partial<Identity> = {}): Identity {
      return { userId: "u-1", email: "ada@semio.dev", displayName: "Ada", hubBaseUrl: "http://hub.test", sessionToken: "tok-1", issuedAtMs: 1_000, ...overrides };
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
      const { applyIdentityConfigMutation, inverseIdentityConfigMutation, signIn, signOut } = await import("./🎚️config/🧬️schema/🧬️mutations/🟦️component");

      const first = sampleIdentity();
      const afterFirstSignIn = applyIdentityConfigMutation(null, signIn(first));
      expect(afterFirstSignIn).toEqual(first);

      const afterSignOut = applyIdentityConfigMutation(afterFirstSignIn, signOut());
      expect(afterSignOut).toBeNull();

      const second = sampleIdentity({ userId: "u-2", email: "devon@semio.dev", displayName: "Devon", sessionToken: "tok-2", issuedAtMs: 2_000 });
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
      const second = sampleIdentity({ userId: "u-2", sessionToken: "tok-2" });
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
      const { applyOpeningConfigMutation, clearDefaultApp, inverseOpeningConfigMutation, setDefaultApp } = await import("./🎚️config/🧬️schema/🧬️mutations/🟦️component");
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
      const { applyMergePolicyConfigMutation, changeMergePolicy, inverseMergePolicyConfigMutation } = await import("./🎚️config/🧬️schema/🧬️mutations/🟦️component");
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
  describe("backbone-worker directory lane", () => {
    class FakeDirectoryWebSocket {
      static instances: FakeDirectoryWebSocket[] = [];
      readonly url: string;
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: string }) => void) | null = null;
      onclose: (() => void) | null = null;
      onerror: (() => void) | null = null;
      constructor(url: string) {
        this.url = url;
        FakeDirectoryWebSocket.instances.push(this);
      }
      send(): void {}
      close(): void {}
      triggerMessage(message: DirectoryStreamMessage): void {
        this.onmessage?.({ data: JSON.stringify(message) });
      }
    }

    async function flushMicrotasks(): Promise<void> {
      await new Promise((resolve) => setTimeout(resolve, 0));
      await new Promise((resolve) => setTimeout(resolve, 0));
    }

    it("queues a directory command while the hub is unreachable, then flushes it in order on the next live signal", async () => {
      FakeDirectoryWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      let fetchCalls = 0;
      const originalFetch = globalThis.fetch;
      (globalThis as unknown as { fetch: unknown }).fetch = async () => {
        fetchCalls += 1;
        throw new Error("network unreachable");
      };

      try {
        handleTsRequest({ kind: "directory-open", baseUrl: "http://hub.test", token: "tok-1", since: 0 });
        handleTsRequest({ kind: "directory-command", requestId: "r1", command: { kind: "create-space", name: "Atelier", spaceKind: "atelier", visibility: "private" } });
        await flushMicrotasks();
        expect(fetchCalls).toBeGreaterThan(0);
        expect(directoryCommandQueue).toHaveLength(1);
        expect(directoryCommandQueue[0]!.requestId).toBe("r1");

        // 🟢️ Hub becomes reachable — any live signal on the stream (a heartbeat here) triggers a flush.
        (globalThis as unknown as { fetch: unknown }).fetch = async () => ({ ok: true, status: 202, json: async () => ({ events: [] }) });
        const socket = FakeDirectoryWebSocket.instances.at(-1)!;
        socket.triggerMessage({ kind: "heartbeat", headSeq: 0 });
        await flushMicrotasks();
        expect(directoryCommandQueue).toHaveLength(0);
      } finally {
        (globalThis as unknown as { fetch: unknown }).fetch = originalFetch;
        // 🧹️ Restores the real global — an un-restored `FakeDirectoryWebSocket` (no `OPEN` static)
        // previously leaked into every later test's `WebSocket.OPEN` comparisons, silently making
        // `relayMutationsToHub`/`sendWireFrame`'s "is the socket actually open" checks pass when
        // `state.socket` was `null`.
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = originalWebSocket;
        closeDirectory();
      }
    });
  });
  //#endregion 🔖️DirectoryLaneTests

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
      readyState = FakeHubWebSocket.CONNECTING;
      binaryType = "blob";
      readonly sent: Uint8Array[] = [];
      onopen: (() => void) | null = null;
      onmessage: ((event: { data: ArrayBuffer }) => void) | null = null;
      onclose: (() => void) | null = null;
      onerror: (() => void) | null = null;
      constructor(url: string) {
        this.url = url;
        FakeHubWebSocket.instances.push(this);
      }
      send(data: Uint8Array): void {
        this.sent.push(data);
      }
      open(): void {
        this.readyState = FakeHubWebSocket.OPEN;
        this.onopen?.();
      }
      close(): void {
        this.readyState = FakeHubWebSocket.CLOSED;
        this.onclose?.();
      }
    }

    function folderOnlyConfig(documentId: string): ArtifactActorConfig {
      return { documentId, schema: "demo/v1", bindings: [{ kind: "folder", path: `/tmp/${documentId}` }], actor: "actor-1" };
    }

    function notFoundResponse() {
      return { ok: false, status: 404, statusText: "not found", headers: { get: () => null }, json: async () => ({}), text: async () => "" };
    }

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
        const state = artifacts.get("doc-overlap")!;
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
        const state = artifacts.get("doc-sanity")!;
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
        const state = artifacts.get("doc-overflow")!;
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

    it("a mutation made while offline is queued, then flushed once the hub reconnects (Welcome flushes the outbox)", () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;

      try {
        const config: ArtifactActorConfig = { documentId: "doc-hub-flush", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1" };
        openArtifact(config);
        const state = artifacts.get("doc-hub-flush")!;
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

        // ♻️ The `Welcome` handshake flushes the outbox — the offline mutation is relayed now,
        // never left stranded waiting for the next local edit to trigger a resend.
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

    it("a batch whose socket dies before Ack moves back into the outbox instead of being stranded", () => {
      FakeHubWebSocket.instances = [];
      const originalWebSocket = globalThis.WebSocket;
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeHubWebSocket;

      try {
        const config: ArtifactActorConfig = { documentId: "doc-hub-stranded", schema: "demo/v1", bindings: [{ kind: "hub", baseUrl: "http://hub.test", spaceId: "studio-1" }], actor: "actor-1" };
        openArtifact(config);
        const state = artifacts.get("doc-hub-stranded")!;
        const socket = FakeHubWebSocket.instances.at(-1)!;
        socket.open();

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
}
//#endregion 🧪️Tests
