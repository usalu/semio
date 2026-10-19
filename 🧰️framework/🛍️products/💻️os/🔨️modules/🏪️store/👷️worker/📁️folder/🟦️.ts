import { type FetchTimeoutResponse, fetchWithTimeout } from "@semio-tech/framework";
import type { PersistenceBinding } from "../../../../🟦️";
import type { FolderCanonicalBootstrapMirrorOwner } from "./🏷️mirror-owner/🟦️.ts";
import type { WireArtifactBootstrap } from "@semio-tech/framework-replication";
import { encodeDocumentArchiveBytes, decodeDocumentArchiveBytes, DEV_STREAM_ROUTES, DOCUMENT_ARCHIVE_MAXIMUM_BYTES } from "../../../../🟦️";
import type { StreamMuxEndpointV1 } from "@semio-tech/framework/io/stream-mux";
import { FOLDER_ENDPOINT_PATH, FOLDER_FETCH_TIMEOUT_MS, SANITY_POLL_MAX_MS, SANITY_POLL_MIN_MS } from "../⚙️constants/🟦️.ts";
import { type ArtifactState, emitEvent } from "../🧾️document-state/🟦️.ts";
import { executionTargetHex, ownedArrayBuffer } from "../📥️execution-target-read/🟦️.ts";
import { folderCanonicalBootstrapMirrorHeaders, folderCanonicalBootstrapMirrorUrl, retireFolderCanonicalBootstrapMirror } from "../🌉️wire-bridge/🟦️.ts";
import { setStatus } from "../🎭️actor-lifecycle/🟦️.ts";
import { equalByteArrays } from "../📮️hub-outbox/🟦️.ts";

/** 🌉️ `fetchWithTimeout` only declares the structural subset every OTHER call site in this file
 * needs (`ok`/`status`/`headers.get`/`json`/`text`) so its own module never requires the ambient
 * `Response` type — the folder/blob reads here are the only callers that also need raw bytes, so
 * that one extra method is added locally instead of widening the shared interface for everyone. */
export type BinaryFetchTimeoutResponse = FetchTimeoutResponse & { arrayBuffer(): Promise<ArrayBuffer> };

function folderEnvelopeUrl(binding: Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }>, documentId: string): string {
  return `${FOLDER_ENDPOINT_PATH}?uri=${encodeURIComponent(`folder://${binding.path}`)}&documentId=${encodeURIComponent(documentId)}`;
}

export async function reserveFolderCanonicalBootstrapMirror(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }>, bootstrap: WireArtifactBootstrap): Promise<FolderCanonicalBootstrapMirrorOwner> {
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

export async function stageFolderCanonicalBootstrapMirror(state: ArtifactState, owner: FolderCanonicalBootstrapMirrorOwner, pack: Uint8Array, spr: Uint8Array): Promise<void> {
  const archive = encodeDocumentArchiveBytes({ parent_pack: Array.from(pack), parent_spr: Array.from(spr), members: [] });
  const response = await fetchWithTimeout(
    folderCanonicalBootstrapMirrorUrl(owner, "stage"),
    { method: "PUT", headers: { ...folderCanonicalBootstrapMirrorHeaders(owner), "content-type": "application/octet-stream" }, body: ownedArrayBuffer(archive) },
    { timeoutMs: FOLDER_FETCH_TIMEOUT_MS, signal: state.docAbort.signal },
  );
  if (!response.ok) throw new Error(`folder canonical bootstrap stage failed (${response.status})`);
}

export async function publishFolderCanonicalBootstrapMirror(state: ArtifactState, owner: FolderCanonicalBootstrapMirrorOwner): Promise<void> {
  const response = await fetchWithTimeout(folderCanonicalBootstrapMirrorUrl(owner, "publish"), { method: "POST", headers: folderCanonicalBootstrapMirrorHeaders(owner) }, { timeoutMs: FOLDER_FETCH_TIMEOUT_MS, signal: state.docAbort.signal });
  if (!response.ok) throw new Error(`folder canonical bootstrap publish failed (${response.status})`);
}

export async function retireCurrentFolderCanonicalBootstrapMirror(state: ArtifactState): Promise<void> {
  const owner = state.canonicalFolderMirror;
  if (!owner) return;
  if (!(await retireFolderCanonicalBootstrapMirror(owner))) throw new Error("folder canonical bootstrap retirement failed");
  if (state.canonicalFolderMirror === owner) state.canonicalFolderMirror = null;
}

/** 📥️ One folder read — always routed through {@link ArtifactState.revalidateFolder}'s
 * `latestWins` wrapper by every caller (SSE wake, sanity-poll tick, `externalChanged`), never
 * called directly, so it can never overlap itself (finding 1). Aborts with the document
 * ({@link ArtifactState.docAbort}, finding 3); an abort is a clean shutdown, not a failure, so it
 * is swallowed without logging. The first answered read of an open that finds no archive, with none read or written
 * before, announces `documentArchiveAbsent` once; a later empty answer only forgets the archive the folder held, unless a
 * write landed while that read was in flight. */
export async function pollFolderOnce(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }>): Promise<void> {
  try {
    const held = state.folderArchive;
    const response = (await fetchWithTimeout(folderEnvelopeUrl(binding, state.config.documentId), undefined, {
      timeoutMs: FOLDER_FETCH_TIMEOUT_MS,
      signal: state.docAbort.signal,
    })) as BinaryFetchTimeoutResponse;
    if (response.status === 204 || response.status === 404) {
      if (state.folderArchive !== held) return;
      if (!state.folderBootstrapped) emitEvent(state, { kind: "documentArchiveAbsent" });
      state.folderBootstrapped = true;
      if (held !== null) {
        state.folderArchive = null;
        setStatus(state, { persisted: false });
      }
            seedConfigFacet(state, null);
return;
    }
    if (!response.ok) throw new Error(`folder backbone read failed (${response.status})`);
    const archive = new Uint8Array(await response.arrayBuffer());
    decodeDocumentArchiveBytes(archive);
    const echo = state.folderArchive !== null && equalByteArrays(state.folderArchive, archive);
    state.folderArchive = archive;
    state.folderBootstrapped = true;
    if (!echo) emitEvent(state, { kind: "documentArchiveReplaced", archive: Array.from(archive) });
    setStatus(state, { persisted: true });
      if (!echo || state.facetValue === undefined) seedConfigFacet(state, archive);
} catch (error) {
    if (state.docAbort.signal.aborted) return; // 🛑 closed mid-flight — not a real failure.
    console.error("[backbone-worker] folder poll failed", state.config.documentId, error);
  }
}

/** 🛟️ Slow, jittered sanity fallback (finding 1): reschedules itself with fresh jitter every tick
 * (a recursive `setTimeout`, not `setInterval`, since the delay must vary tick to tick) and only
 * actually revalidates when {@link ArtifactState.watchHealthy} is `false` — the change stream is the primary wake,
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
    if (!state.watchHealthy) void state.revalidateFolder();
    scheduleNext();
  };
  scheduleNext();
}

/** 🔀️ The page's stream channel as this worker reaches it (a port the shell transfers once, `semio-stream-mux-port`); `null`
 * until it arrives, in which case a folder falls back to its sanity poll. */
let streamMuxEndpoint: StreamMuxEndpointV1 | null = null;

/** 🧷️ Installs the endpoint every later folder watch opens its change stream on. */
export function installStreamMuxEndpoint(endpoint: StreamMuxEndpointV1 | null): void {
  streamMuxEndpoint = endpoint;
}

/** 👁️ External-change watch: an immediate bootstrap read, the folder's `backbone.folder` change stream on the page's stream
 * channel (every notice revalidates, and so does a fresh open — notices may have been missed while it was down; a resumed
 * open replays exactly the missed ones) and the slow sanity poll ({@link startSanityPolling}), which only works while no
 * stream is open. A notice's credit returns only when its revalidation settled, so a slow read is backpressure, not a queue. */
export function watchFolder(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }>): void {
  void state.revalidateFolder();
  startSanityPolling(state);
  streamMuxEndpoint?.open(
    DEV_STREAM_ROUTES.backboneFolder,
    `folder://${binding.path}`,
    {
      opened: (mode) => {
        state.watchHealthy = true;
        if (mode === "fresh") void state.revalidateFolder();
      },
      data: () => state.revalidateFolder(),
      end: () => {
        state.watchHealthy = false;
      },
    },
    { signal: state.docAbort.signal },
  );
}

export async function writeFolder(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }>, archive: readonly number[]): Promise<void> {
  await retireCurrentFolderCanonicalBootstrapMirror(state);
  const bytes = Uint8Array.from(archive);
  if (bytes.length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES) throw new Error("document archive exceeds its fixed byte authority");
  decodeDocumentArchiveBytes(bytes);
  if (state.folderArchive !== null && equalByteArrays(state.folderArchive, bytes)) return;
  const response = await fetchWithTimeout(
    folderEnvelopeUrl(binding, state.config.documentId),
    { method: "PUT", headers: { "content-type": "application/octet-stream" }, body: bytes },
    { timeoutMs: FOLDER_FETCH_TIMEOUT_MS, signal: state.docAbort.signal },
  );
  if (!response.ok) throw new Error(`folder backbone write failed (${response.status})`);
  state.folderArchive = bytes;
  state.folderBootstrapped = true;
  setStatus(state, { persisted: true });
}
