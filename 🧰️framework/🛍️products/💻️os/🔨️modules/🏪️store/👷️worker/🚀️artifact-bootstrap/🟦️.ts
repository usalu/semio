import type { ArtifactBootstrapProgress, ArtifactBootstrapControl, WireArtifactBootstrap, WireFrontierSummary } from "@semio-tech/framework-replication";
import { ArtifactBootstrapAssembler, DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS } from "@semio-tech/framework-replication";
import { sameLeaseFieldsV1 } from "../../../📇️directory/🧬️schema/🔏️document-open/🟦️.ts";
import { encodeDocumentArchiveBytes } from "../../../../🟦️";
import { CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1, CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES, decodeCanonicalCheckpointPairV1 } from "../../../📇️directory/🚪️io/🧱️binary/🪢️checkpoint-pair/🟦️.ts";
import { admitCanonicalCheckpointPairV1 } from "@semio-tech/framework/directory/schema";
import { admitCheckpointSelectionV1 } from "../../../📇️directory/🚪️io/🧱️binary/🪢️checkpoint-pair/🟦️.ts";
import { type ArtifactState, artifactScope, artifacts, documentServices, dropVerifiedColdDocumentPair, emitEvent, post } from "../🧾️document-state/🟦️.ts";
import type { FolderCanonicalBootstrapMirrorOwner } from "../📁️folder/🏷️mirror-owner/🟦️.ts";
import type { DocumentArtifactBootstrapOwner, DocumentArtifactRebootstrapOwner } from "./🏷️owners/🟦️.ts";
import { publishFolderCanonicalBootstrapMirror, reserveFolderCanonicalBootstrapMirror, retireCurrentFolderCanonicalBootstrapMirror, stageFolderCanonicalBootstrapMirror } from "../📁️folder/🟦️.ts";
import { hubBinding } from "../🧩️actor-wire-helpers/🟦️.ts";
import { folderBinding, setRemote } from "../🎭️actor-lifecycle/🟦️.ts";
import { equalByteArrays, equalFrontiers, flushMutationsToHubIfReady, requeuePendingBatches } from "../📮️hub-outbox/🟦️.ts";
import { abortArtifactBootstrap, artifactBootstrapFailure, clearArtifactBootstrapDeadline, clearArtifactRebootstrapDeadline } from "../🛑️session-teardown/🟦️.ts";
import { ARTIFACT_BOOTSTRAP_DEADLINE_MS } from "../⚙️constants/🟦️.ts";
import { type ExecutionTargetReadControl, emitExecutionTargetStatus, executionTargetHex, forgetMountedDocument, readBoundedExecutionTargetBody } from "../📥️execution-target-read/🟦️.ts";
import { VerifiedColdDocumentPair, verifiedColdDocumentPairMintToken } from "../🧊️cold-pair/🟦️.ts";
import { documentBackboneMessageFromDomain, emitMutationEvent, retireFolderCanonicalBootstrapMirror } from "../🌉️wire-bridge/🟦️.ts";
import { hubSessionFetch } from "../🔐️hub-session/🟦️.ts";

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

export function bootstrapControl(state: ArtifactState, owner = state.artifactBootstrapOwner): ArtifactBootstrapControl {
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

export function rejectArtifactBootstrap(state: ArtifactState, error: unknown, owner = state.artifactBootstrapOwner): void {
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

/** 🔁️ Rebuilds this document from the hub's authoritative state (a remote fold over pending local operations, a hub
 * `RebootstrapRequired`): drops the local pair, frontier and resume token, closes the socket and reconnects at once with no
 * frontier. An actor-bound document is rebuilt exactly like a first open: the hub's `Welcome` carries only its tail (the hub
 * serves a canonical pair over the `active-checkpoint/pair` route, never on the socket), the new actor child is seeded from
 * that route once the `Session` admits it, and the tail follows. Requiring the pair inside the `Welcome` left every such
 * rebuild refused forever and the document empty (ticket 26/09/23 LD item 3, live on B3: 15 s cut → B's socket closed every
 * 15–20 s). */
export async function requireArtifactRebootstrap(state: ArtifactState): Promise<void> {
  state.remoteFoldedOverLocal = false;
  const owner = captureArtifactRebootstrapOwner(state);
  requeuePendingBatches(state);
  documentServices.documentRebootstrapped({ state });
  await retireCurrentFolderCanonicalBootstrapMirror(state);
  if (state.artifactRebootstrapOwner !== owner) return;
  owner.assertCurrent();
  abortArtifactBootstrap(state);
  dropVerifiedColdDocumentPair(state);
  forgetMountedDocument(state);
  setRemote(state, { kind: "connecting" });
  const scope = artifactScope(state);
  post({ kind: "artifact-rebootstrap-required", documentId: state.config.documentId, clientInstanceId: state.openClientInstanceId, ...(scope === undefined ? {} : { scope }), message: "rebootstrap-required", retryable: true });
  state.socket?.close();
}

function validateArtifactBootstrapIdentity(state: ArtifactState, bootstrap: WireArtifactBootstrap, serverFrontier: WireFrontierSummary): void {
  if (bootstrap.artifact_schema !== state.config.schema) throw new Error("artifact bootstrap schema mismatch");
  if (bootstrap.baseline_frontier.document_id !== state.config.documentId || bootstrap.required_tail_frontier.document_id !== state.config.documentId || serverFrontier.document_id !== state.config.documentId)
    throw new Error("artifact bootstrap document mismatch");
  // 🧬️ The pack schema hash's AUTHORITY is the verified open plan, mirrored onto the execution-target
  // lease; `ArtifactActorConfig.packSchemaHash` is the shell's OPTIONAL cross-check, supplied only by
  // the wasm renderer's `document_pack_schema_hash` export and absent in every React host — exactly
  // the rule `documentOpenPlanAuthority` already applies to the same pair of values. Demanding the
  // optional one here made a React host refuse every artifact bootstrap it could ever be offered.
  const packSchemaHash = state.config.packSchemaHash;
  const configuredPackSchemaHash = packSchemaHash !== undefined && packSchemaHash !== null && packSchemaHash.some((byte) => byte !== 0);
  if (bootstrap.pack_schema_hash.length !== 32 || bootstrap.pack_schema_hash.every((byte) => byte === 0)) throw new Error("artifact bootstrap pack schema mismatch");
  if (configuredPackSchemaHash && (packSchemaHash.length !== 32 || !equalByteArrays(bootstrap.pack_schema_hash, packSchemaHash))) throw new Error("artifact bootstrap pack schema mismatch");
  if (!configuredPackSchemaHash && state.executionTargetLease === null) throw new Error("artifact bootstrap pack schema unauthenticated");
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

export function finishCatchupIfReady(state: ArtifactState): void {
  if (!state.requiredTailFrontier || !state.frontier || !equalFrontiers(state.frontier, state.requiredTailFrontier)) return;
  state.requiredTailFrontier = null;
  if (state.pendingResumeToken !== null) state.resumeToken = state.pendingResumeToken;
  state.pendingResumeToken = null;
  setRemote(state, { kind: "live", peerCount: 0 });
  flushMutationsToHubIfReady(state);
}

export async function installArtifactBootstrap(state: ArtifactState, owner: DocumentArtifactBootstrapOwner, done: { readonly descriptor_hash: readonly number[]; readonly chunk_count: number } | null): Promise<void> {
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
      const reservation = state.browserActorReservation;
      if (reservation === null) throw new Error("artifact bootstrap missing browser actor reservation");
      const retainedBackbone = state.outbox.length === 0 ? [] : [documentBackboneMessageFromDomain(state, state.outbox)];
      await reservation.installColdPair(coldOwner, retainedBackbone);
      owner.assertCurrent();
      coldOwner.assertCurrent();
    }
    if (coldOwner === null) {
      const archive = encodeDocumentArchiveBytes({ parent_pack: Array.from(pair.pack), parent_spr: Array.from(pair.spr), members: [] });
      emitEvent(state, { kind: "documentArchiveReplaced", archive: Array.from(archive) });
    }
    owner.assertCurrent();
    if (state.outbox.length > 0 && coldOwner === null) {
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

const CANONICAL_CHECKPOINT_PAIR_REQUEST_TIMEOUT_MS = 30_000;

/** 🧯️ The hub did not hand over the document's canonical checkpoint pair. The pair is the ONLY source of the
 * document a browser actor renders and authors — for a door-created artifact it is the genesis the owning component
 * minted for its identity — so an actor never runs without it: before, a missing pair activated a child with no
 * document, which never rendered and refused every action (`action-owner-mismatch`), with nothing retrying. A
 * `transient` refusal (the hub is busy or unreachable) is retried while the lease lives; any other refusal is final. */
class CanonicalCheckpointPairUnavailableV1 extends Error {
  readonly transient: boolean;

  constructor(readonly status: number, detail = "status") {
    super(`canonical checkpoint pair: unavailable (${detail} ${status})`);
    this.transient = detail === "status" && CANONICAL_CHECKPOINT_PAIR_TRANSIENT_STATUSES.has(status);
  }
}

const CANONICAL_CHECKPOINT_PAIR_TRANSIENT_STATUSES: ReadonlySet<number> = new Set([408, 425, 429, 500, 502, 503, 504]);

/** 🔁️ Whether a failed canonical pair request may be repeated under the same lease. */
export function canonicalCheckpointPairRetryable(error: unknown): boolean {
  return (error instanceof CanonicalCheckpointPairUnavailableV1 && error.transient) || (error instanceof Error && error.message === "hub session unreachable");
}

/** 🪢️ Fetches, verifies and installs the document's canonical checkpoint pair.
 *
 * A hub's document socket NEVER seeds an artifact client: its `Welcome.bootstrap` is computed by the
 * database replay, whose only outcomes are `None`, `Tail` and the database-private `Snapshot` this
 * client refuses by contract — `Bootstrap.ArtifactBootstrap` has no producer on any hub. The pair a
 * cold client needs lives behind the hub's canonical-checkpoint-pair route, the same authenticated,
 * digest-bound transport the remote MCP workspace mounts a document with, and the open plan already
 * handed this client the checkpoint identity to verify it against.
 *
 * The fetched pair is then driven through the EXISTING inline-bootstrap path, so every check that a
 * socket-delivered pair would face still runs: {@link validateArtifactBootstrapIdentity} against the
 * lease checkpoint, the assembler's own digest verification, the folder mirror, the cold-pair mint
 * and `installColdPair`. `required_tail_frontier` is the pair's own baseline rather than the
 * welcome's frontier, because a hub stamps its INTERNAL database key into every frontier it derives
 * (`db_artifact_id`), which this client refuses as a document mismatch. */
export async function seedColdPairFromCanonicalCheckpoint(state: ArtifactState, resumeToken: string): Promise<boolean> {
  const binding = hubBinding(state.config);
  const lease = state.executionTargetLease;
  if (binding === null || lease === null || !lease.live) return false;
  const fields = lease.fields();
  if (fields.browserActor.kind !== "closed-browser-actor" || state.currentPack !== null || state.verifiedColdPair !== null) return false;
  const checkpoint = fields.checkpoint;
  const path = `/spaces/${encodeURIComponent(binding.spaceId)}/documents/${encodeURIComponent(state.config.documentId)}/active-checkpoint/pair`;
  if (!/^\/spaces\/[^/?#]+\/documents\/[^/?#]+\/active-checkpoint\/pair$/u.test(path)) throw new Error("canonical checkpoint pair: operation denied");
  const assertCurrent = (): void => {
    if (state.executionTargetLease !== lease || !lease.live) throw new Error("canonical checkpoint pair: lease retired");
  };
  emitExecutionTargetStatus(state, binding, "verifying", { stage: "canonical-pair", completedBytes: 0, totalBytes: 0 });
  const response = await hubSessionFetch(
    path,
    { method: "GET", headers: { accept: CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1 } },
    { timeoutMs: CANONICAL_CHECKPOINT_PAIR_REQUEST_TIMEOUT_MS, signal: lease.retirement, admit: () => state.executionTargetLease === lease && lease.live },
  );
  if (!response.ok) throw new CanonicalCheckpointPairUnavailableV1(response.status);
  if (response.headers.get("content-type") !== CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1) throw new CanonicalCheckpointPairUnavailableV1(response.status, "media type");
  const control: ExecutionTargetReadControl = { signal: lease.retirement, deadlineAtMs: Date.now() + CANONICAL_CHECKPOINT_PAIR_REQUEST_TIMEOUT_MS, assertCurrent };
  const body = await readBoundedExecutionTargetBody(response, null, CANONICAL_CHECKPOINT_PAIR_MAX_PAIR_BYTES, control, (completedBytes, totalBytes) =>
    emitExecutionTargetStatus(state, binding, "verifying", { stage: "canonical-pair", completedBytes, totalBytes }),
  );
  assertCurrent();
  const pair = decodeCanonicalCheckpointPairV1(body);
  admitCanonicalCheckpointPairV1(pair, { spaceId: binding.spaceId, documentId: state.config.documentId }, admitCheckpointSelectionV1(checkpoint));
  const frontier: WireFrontierSummary = {
    document_id: pair.baselineFrontier.documentId,
    head_edit_ordinal: pair.baselineFrontier.headEditOrdinal,
    head_edit_id: pair.baselineFrontier.headEditId,
    last_commit_seq: pair.baselineFrontier.lastCommitSeq,
    chain_hash: pair.baselineFrontier.chainHash,
  };
  const bootstrap: WireArtifactBootstrap = {
    format_version: 1,
    descriptor_hash: pair.descriptorDigestV1,
    artifact_schema: state.config.schema,
    artifact_kind: fields.artifact.kind,
    pack_schema_hash: Array.from({ length: 32 }, (_unused, index) => Number.parseInt(fields.artifact.packSchemaHash.slice(index * 2, index * 2 + 2), 16)),
    baseline_frontier: frontier,
    pack_hash: pair.pack.sha256,
    spr_hash: pair.spr.sha256,
    pack_length: pair.pack.byteLength,
    spr_length: pair.spr.byteLength,
    chunk_count: 0,
    aggregate_hash: pair.aggregateSha256,
    required_tail_frontier: frontier,
    inline: { pack: Array.from(pair.packBytes), spr: Array.from(pair.sprBytes) },
  };
  pair.packBytes.fill(0);
  pair.sprBytes.fill(0);
  await startArtifactBootstrap(state, bootstrap, resumeToken, frontier);
  return true;
}

export async function startArtifactBootstrap(state: ArtifactState, bootstrap: WireArtifactBootstrap, resumeToken: string, serverFrontier: WireFrontierSummary): Promise<void> {
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
