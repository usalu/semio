import type { ArtifactActorConfig, DocumentScope, DocumentLink, ArtifactSyncStatus, BackboneWorkerResponse, ArtifactEvent } from "../../../../🟦️";
import type { ResolvedPluginViewState } from "@semio-tech/framework/manifest";
import type { MutationEnvelope, ExactWireMutationEnvelope, WireFrontierSummary, ArtifactBootstrapProgress } from "@semio-tech/framework-replication";
import { ArtifactBootstrapAssembler } from "@semio-tech/framework-replication";
import type { DocumentActorRecoveryMemoryV1 } from "../🚑️actor-recovery/🟦️.ts";
import { documentRuntimeKeyV1, encodeBackboneWorkerResponse } from "../../../../🟦️";
import { InstalledServiceRegistryV1 } from "@semio-tech/framework/inference/service";
import type { DocumentArtifactBootstrapOwner, DocumentArtifactRebootstrapOwner } from "../🚀️artifact-bootstrap/🏷️owners/🟦️.ts";
import type { FolderCanonicalBootstrapMirrorOwner } from "../📁️folder/🏷️mirror-owner/🟦️.ts";
import { workerScope } from "../⚙️constants/🟦️.ts";
import { hubBinding } from "../🧩️actor-wire-helpers/🟦️.ts";

/** 🔌️ The slots of {@link ArtifactState} whose values are classes owned by the modules that fill them. Each owner declares its own slot by augmenting this interface, so the state never imports its owners and no import cycle can form between the state and the owners that read and write it. */
export interface ArtifactOwnerSlots {}

export type ArtifactState = {
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
  executionTargetLease: ArtifactOwnerSlots["executionTargetLease"] | null;
  browserActorReservation: ArtifactOwnerSlots["browserActorReservation"] | null;
  /** 📥️ Remote backbone messages of an actor-bound document that arrived before its actor child was reserved — the hub's
   * `Welcome` tail comes before the `Session` that admits the child — handed to the child in arrival order the moment it is
   * reserved. Bounded like the child's own pre-binding retention. */
  browserActorBackboneBeforeReservation: Uint8Array[];
  /** 🔀️ Another human's operations were folded in while this shell's own operations were still unacknowledged. The hub
   * orders those remote operations BEFORE the pending local ones, but the local fold applied them after, so once the local
   * ones are accepted the document is rebuilt from the hub's authoritative pair: measured as two humans each showing the
   * OTHER's value for the same field forever after a short link loss (🎫️ 26/09/23 C10). */
  remoteFoldedOverLocal: boolean;
  /** 🔌️ The mounted child's hub link — the kernel's one `DocumentLink` state machine (`documentLinkTransition`,
   * `🏪️store/🧫️fixtures/document-link-shortage-v1`): a short loss suspends it, past the bound it expires. */
  link: DocumentLink;
  /** 🔌️ Fires at the suspended link's expiry ({@link documentLinkExpiresAtMs}) and retires the child once it expired. */
  linkShortageTimer: ReturnType<typeof setTimeout> | null;
  /** ⏳️ The hub refused a batch for a transient reason ({@link hubTransientApplyRefusalV1}): the batch went back to the front of
   * the outbox, every later batch waits behind it, and the outbox drains one batch of at most `batchLimit` envelopes after
   * `backoffMs` (doubled per refusal, capped). Cleared by the next `Accepted`. */
  transientRefusal: { timer: ReturnType<typeof setTimeout> | null; backoffMs: number; batchLimit: number } | null;
  browserActorViewState: ResolvedPluginViewState | null;
  /** 🛟️ Handle for the recursive, jittered sanity-poll reschedule (finding 1) — a plain
   * `ReturnType<typeof setTimeout>`, not `setInterval`, because each tick schedules its OWN next
   * delay with fresh jitter rather than ticking on a fixed period. */
  sanityPollTimer: ReturnType<typeof setTimeout> | null;
  /** 📡️ Explicit "is the folder's change stream open" flag — set when the `backbone.folder` stream opens (fresh or
   * resumed), cleared when it ends; {@link startSanityPolling} reads it to decide whether a tick revalidates. */
  watchHealthy: boolean;
  /** 🥇️ Single-flight folder revalidation (finding 1) — built once per document with
   * {@link latestWins} over {@link pollFolderOnce}, so the SSE `onmessage` wake, the sanity-poll
   * tick, and an `externalChanged` local message all share the SAME in-flight guard and can never
   * stack overlapping reads. A no-op placeholder until {@link openArtifact} sees a folder binding. */
  revalidateFolder: () => Promise<void>;
  /** 🪞️ The archive bytes this document last wrote to, or applied from, its folder — the TypeScript twin of the Rust actor's
   * `last_written_hash`: a read that returns them is this document's own write echoing back through the watch, never an
   * external change, so it replaces nothing (a replace would reload the guest from a state older than every edit made
   * since the write). */
  folderArchive: Uint8Array | null;
  /** 🪹️ Whether this open's first folder read was answered, or an archive was read or written: no later read announces
   * an absent archive (`documentArchiveAbsent`). */
  folderBootstrapped: boolean;
    /** 🎚️ The value a {@link CONFIG_FACETS} document holds: the fold of its persisted archive and every diff the store admitted since.
   * `undefined` until the first folder read answered (or at once when the facet has no folder). */
  facetValue: unknown;
  /** ⏳️ Facet envelopes admitted before the first folder read answered; they fold onto the persisted value, never onto an empty one. */
  facetBacklog: MutationEnvelope[];
reconnectDelayMs: number;
  /** 🗃️ Outbound mutations not yet sent on a LIVE hub socket (finding 5) — distinct from
   * `pendingBatches`, which holds the one batch already sent and awaiting its `Ack`. Every relay
   * ({@link relayMutationsToHub}) queues here; a dead socket's `onclose` and a transient refusal put
   * their unapplied batch back at the FRONT; {@link flushMutationsToHubIfReady} drains it one bounded
   * batch at a time — this is the "flushed on reconnect" half of finding 5. */
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
  /** 🚑️ The document's actor-recovery memory (`🚑️actor-recovery`) and whether the socket closing now is a recovery close that
   * reconnects at once. */
  actorRecovery: DocumentActorRecoveryMemoryV1;
  actorRecoveryRequested: boolean;
  artifactBootstrapProgress: ArtifactBootstrapProgress[];
  canonicalFolderMirror: FolderCanonicalBootstrapMirrorOwner | null;
  verifiedColdPair: ArtifactOwnerSlots["verifiedColdPair"] | null;
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
  /** 🧿 Mutation ids already projected from remote Commands — duplicate frames are ignored. */
  ingestedMutationIds: Set<string>;
  /** ⏰️ Logical tick counter for {@link nextWireTimestamp} on every outbound wire envelope. */
  hlcCounter: number;
  closed: boolean;
};

export const artifacts = new Map<string, ArtifactState>();
export let workerPostTestSink: ((message: BackboneWorkerResponse) => void) | null = null;
/** 🧪️ Assigns `workerPostTestSink` for the test seam, because an imported binding cannot be assigned outside its declaring module. */
export function setWorkerPostTestSink(value: typeof workerPostTestSink): void {
  workerPostTestSink = value;
}

export function artifactRuntimeKey(documentId: string, spaceId?: string): string | null {
  if (spaceId !== undefined) return documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId, documentId });
  const localKey = documentRuntimeKeyV1({ kind: "local", documentId });
  if (artifacts.has(localKey)) return localKey;
  const matches = [...artifacts].filter(([, state]) => state.config.documentId === documentId);
  return matches.length === 1 ? matches[0]![0] : matches.length === 0 ? localKey : null;
}

export function artifactState(documentId: string, spaceId?: string): ArtifactState | undefined {
  const runtimeKey = artifactRuntimeKey(documentId, spaceId);
  return runtimeKey === null ? undefined : artifacts.get(runtimeKey);
}

export function post(message: BackboneWorkerResponse): void {
  if (workerPostTestSink !== null) {
    workerPostTestSink(message);
    return;
  }
  workerScope?.postMessage({ wire: encodeBackboneWorkerResponse(message) });
}

export function artifactScope(state: ArtifactState): DocumentScope | undefined {
  const binding = hubBinding(state.config);
  return binding === null ? undefined : { spaceId: binding.spaceId, documentId: state.config.documentId };
}

export function emitEvent(state: ArtifactState, event: ArtifactEvent): void {
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
export function dropVerifiedColdDocumentPair(state: ArtifactState): void {
  const owner = state.verifiedColdPair;
  state.verifiedColdPair = null;
  owner?.drop();
}

/** 🪪 Narrow authority port available to installed owner service contributions. */
export type DocumentServiceWorkerContextV1 = Readonly<{ state: ArtifactState; reservation?: NonNullable<ArtifactState["browserActorReservation"]>; pair?: NonNullable<ArtifactState["verifiedColdPair"]> }>;
export const documentServices = new InstalledServiceRegistryV1<DocumentServiceWorkerContextV1>();
