/** 🧾️ The app channel client: the typed facade over one plugin instance's app channel, a thin composer over the collaborators that own its lifecycle, request correlation, outcome pump, document pack cache and command families.
 *
 * 🧱️ The children own the rest: `🏷️codec-primitives`, `🎛️command-codec`, `🎞️frame-codec`, `📋️report-decoders`, `🔢️request-sequence` (channel version admission and request sequence), `🗃️archive-load-host`, `🔌️channel-lifecycle`, `📮️reply-ledger`, `📦️document-pack-cache`, `📣️unsolicited-feed`, `🏠️local-interaction-query`, `🔁️outcome-pump`, `🧯️reply-faults`, `🪪️document-reads`, `🧳️archive-transfer`, `🎚️interaction-commands`, `🎬️media-commands`, `⚖️replication-commands` and `🎫️transaction-commands`.
 */

import type { ConflictResolution, MergePolicy, PluginDispatchHintV1 } from "@semio-tech/framework";
import type { ArtifactPresencePeer, LocalInteractionIdentity, LocalInteractionPage, MutationEnvelope } from "@semio-tech/framework-replication";
import type { AppDocumentIdentity, AppFrameValue, ChildHeadPackEntry, DocumentArchiveLoadStatus, DocumentArchivePack, MediaExportHandle, OperationCompletionV1, WindowConfigPackEntry } from "../🪢️wire-contracts/🟦️.ts";
import type { DocumentArchiveMergeV1 } from "./🗃️archive-load-host/🟦️.ts";
import { ChannelLifecycle } from "./🔌️channel-lifecycle/🟦️.ts";
import { DocumentPackCache } from "./📦️document-pack-cache/🟦️.ts";
import { ReplyLedger } from "./📮️reply-ledger/🟦️.ts";
import { type AppChannelEphemeralV1, type AppChannelOperationProgressV1, UnsolicitedFeed } from "./📣️unsolicited-feed/🟦️.ts";
import { LocalInteractionQuery } from "./🏠️local-interaction-query/🟦️.ts";
import { OutcomePump } from "./🔁️outcome-pump/🟦️.ts";
import type { AppChannelHandle, AppChannelRequestSequence } from "./🔢️request-sequence/🟦️.ts";
import { DocumentReads } from "./🪪️document-reads/🟦️.ts";
import { ArchiveTransfer } from "./🧳️archive-transfer/🟦️.ts";
import { InteractionCommands } from "./🎚️interaction-commands/🟦️.ts";
import { MediaCommands } from "./🎬️media-commands/🟦️.ts";
import { ReplicationCommands } from "./⚖️replication-commands/🟦️.ts";
import { TransactionCommands } from "./🎫️transaction-commands/🟦️.ts";

//#region 🔖️AppChannelClient
/**
 * 📡️ Typed facade over one plugin instance's app channel — encodes an {@link AppCommandValue}, queues
 * it via {@link PluginWasmHandle.enqueue}, and decodes every {@link AppFrameValue} the matching
 * {@link TurnOutcome} carries. This is the ONLY place `AppCommand`/`AppFrame` framing happens on the
 * host side; callers (a React renderer's dispatch/refresh loop, a headless workflow runner) work with
 * decoded frames and plain JS values, never raw bytes or wire tags. `seq` is a handle-owned monotonic
 * counter — the host has no other way to correlate a `Command`/`ConfigCommand`/
 * `ReadDocument`/`LoadConfig`/`ReadConfig` with the `Invocation`/`Document` frame(s) it produced
 * (`AppFrame.*.in_reply_to`). Channel v12 retired the `hello()`/`refreshUi()`/`attachBackbone()`/
 * `detachBackbone()`/`drain()` surface this class used to expose — the handshake, cache-probed UI
 * refresh, and empty-batch drain all disappeared with the reactor ABI (lifecycle now arrives via
 * `Event::InstanceOpen`/`InstanceClose`, UI updates are a `UiPatch` push, and guests wake on
 * events/timers/`next-wake` rather than a poll).
 *
 * `📌️important.md`'s "Replace, never wrap" list: the old handle's synchronous
 * per-call method used to hand this class its reply directly; the handle is now fire-and-forget
 * ({@link PluginWasmHandle.enqueue}) and replies arrive on the handle-wide {@link
 * PluginWasmHandle.outcomes} stream instead, so THIS class is what turns that stream back into the
 * one-reply-per-call shape every method below still returns. One background loop
 * ({@link OutcomePump}) owns the handle's async iterator for this instance's lifetime and routes
 * every explicit numeric receipt to its exact waiter. Empty outcomes and uncorrelated notifications
 * cannot complete a command. Transaction payloads accompany their numeric receipt and additionally
 * match the pending transaction identity. One outcome may complete several distinct commands.
 *
 * 🧩️ The class itself only wires its collaborators and keeps the retire and dispose ordering; every other member is a one-line
 * delegation that returns the owner's promise unchanged.
 */
export class AppChannelClient {
  private readonly lifecycle: ChannelLifecycle;
  private readonly ledger: ReplyLedger;
  private readonly cache: DocumentPackCache;
  private readonly feed: UnsolicitedFeed;
  private readonly localQuery: LocalInteractionQuery;
  private readonly pump: OutcomePump;
  private readonly documentReads: DocumentReads;
  private readonly archives: ArchiveTransfer;
  private readonly interactions: InteractionCommands;
  private readonly media: MediaCommands;
  private readonly replication: ReplicationCommands;
  private readonly transactions: TransactionCommands;

  constructor(handle: AppChannelHandle, sequenceOwner: AppChannelRequestSequence, instanceId: number, appId: string) {
    this.lifecycle = new ChannelLifecycle();
    this.ledger = new ReplyLedger(handle, sequenceOwner, instanceId, this.lifecycle);
    this.cache = new DocumentPackCache(this.lifecycle);
    this.feed = new UnsolicitedFeed(instanceId, this.lifecycle);
    this.localQuery = new LocalInteractionQuery(instanceId, this.ledger, this.lifecycle, () => this.pump.finishDisposal());
    this.pump = new OutcomePump(handle, instanceId, this.lifecycle, this.ledger, this.localQuery, this.cache, this.feed);
    this.documentReads = new DocumentReads(this.ledger, instanceId, appId);
    this.archives = new ArchiveTransfer(this.ledger, this.cache, appId);
    this.interactions = new InteractionCommands(this.ledger, appId);
    this.media = new MediaCommands(this.ledger);
    this.replication = new ReplicationCommands(this.ledger);
    this.transactions = new TransactionCommands(this.ledger);
    this.pump.start();
  }

  /** 🪦️ Closes this instance's COMMAND QUEUE and settles everything already in it, WITHOUT ending the
   * background outcome subscription — the first half of `destroyApp`
   * (`PluginRuntime/🟦️.tsx`), run before the actor's close ladder rather than after it.
   *
   * The ladder runs on the actor's own lifecycle lease and never speaks through this channel, so while
   * it ran the queue stayed open: a gesture dispatched into a dying instance was admitted here and only
   * refused one layer down by `requireActorId`, as `no actor for instance N` plus a stack. Retiring the
   * queue first turns every such arrival into this channel's own typed terminal. The subscription is
   * deliberately NOT ended here — {@link dispose} still owns that, so a close the native side REFUSES
   * leaves the iterator exactly where it was and the retry can still retire it. */
  retire(): void {
    if (!this.lifecycle.retire()) return;
    this.feed.clearCompletions();
    this.cache.forget();
    this.ledger.retireCommands();
    this.localQuery.retire();
  }

  /** 🔌️ Ends this client's background outcome subscription — call once from
   * `destroyApp` (`PluginRuntime/🟦️.tsx`) so a torn-down instance doesn't leak a live
   * subscriber against the handle-wide outcome stream for the rest of the handle's lifetime. */
  dispose(): void {
    this.retire();
    this.lifecycle.dispose();
    this.pump.finishDisposal();
  }

  /** 👥️ The last published presence pack, interaction slice and tool-run and history-edit summaries, `null` until the guest published one; see {@link UnsolicitedFeed.ephemeral}. */
  ephemeral(): AppChannelEphemeralV1 | null {
    return this.feed.ephemeral();
  }

  /** 🏁️ Subscribes to unsolicited typed-operation completions; see {@link UnsolicitedFeed.onCompleted}. */
  onOperationCompleted(listener: (completion: OperationCompletionV1) => void): () => void {
    return this.feed.onCompleted(listener);
  }

  /** 🎞️ Subscribes to unsolicited mid-operation progress; see {@link UnsolicitedFeed.onProgress}. */
  onOperationProgress(listener: (progress: AppChannelOperationProgressV1) => void): () => void {
    return this.feed.onProgress(listener);
  }

  /** 📦️ The cached `{pack, spr}` of this instance's document, or `null`; see {@link DocumentPackCache.documentPack}. */
  documentPack(): { readonly pack: Uint8Array; readonly spr: Uint8Array } | null {
    return this.cache.documentPack();
  }

  /** 📃️ Reads the local interaction slice page by page; see {@link LocalInteractionQuery.read}. */
  readLocalInteractionPages(consume: (page: LocalInteractionPage) => Promise<void>, signal?: AbortSignal): Promise<LocalInteractionIdentity> {
    return this.localQuery.read(consume, signal);
  }

  /** 🎛️ Forwards one opaque app-specific command; see {@link InteractionCommands.command}. */
  command(commandBytes: Uint8Array, viewState: unknown, dispatch?: PluginDispatchHintV1): Promise<AppFrameValue[]> {
    return this.interactions.command(commandBytes, viewState, dispatch);
  }

  /** 🔧️ Forwards one configuration command; see {@link InteractionCommands.configure}. */
  configure(config: unknown): Promise<AppFrameValue[]> {
    return this.interactions.configure(config);
  }

  /** 🖱️ On-demand context menu; see {@link InteractionCommands.contextMenu}. */
  contextMenu(request: unknown): Promise<unknown> {
    return this.interactions.contextMenu(request);
  }

  /** 📥️ Force-applies remote envelopes; see {@link InteractionCommands.applyEnvelopes}. */
  applyEnvelopes(envelopes: readonly MutationEnvelope[]): Promise<AppFrameValue[]> {
    return this.interactions.applyEnvelopes(envelopes);
  }

  /** 📂️ Opens an artifact in its viewer or editor surface; see {@link InteractionCommands.openArtifact}. */
  openArtifact(artifactRef: string, role: number, pluginId = "", appId = ""): Promise<AppFrameValue[]> {
    return this.interactions.openArtifact(artifactRef, role, pluginId, appId);
  }

  /** 📌️ Pins a viewer or editor default; see {@link InteractionCommands.setDefaultApp}. */
  setDefaultApp(artifactKind: string, standard: string, subset: string, role: number, pluginId: string, appId: string): Promise<AppFrameValue[]> {
    return this.interactions.setDefaultApp(artifactKind, standard, subset, role, pluginId, appId);
  }

  /** 🧽️ Clears a pinned default; see {@link InteractionCommands.clearDefaultApp}. */
  clearDefaultApp(artifactKind: string, standard: string, subset: string, role: number): Promise<AppFrameValue[]> {
    return this.interactions.clearDefaultApp(artifactKind, standard, subset, role);
  }

  /** 📄️ Reads the document; see {@link DocumentReads.readDocument}. */
  readDocument(): Promise<AppFrameValue[]> {
    return this.documentReads.readDocument();
  }

  /** 🆔️ Reads the trusted document envelope identity; see {@link DocumentReads.readDocumentIdentity}. */
  readDocumentIdentity(): Promise<AppDocumentIdentity> {
    return this.documentReads.readDocumentIdentity();
  }

  /** 🪆️ Reads every owned child's current head pack; see {@link DocumentReads.readChildHeads}. */
  readChildHeads(): Promise<readonly ChildHeadPackEntry[]> {
    return this.documentReads.readChildHeads();
  }

  /** 🧾️ Retrieves the complete history projection; see {@link DocumentReads.readHistory}. */
  readHistory(): Promise<AppFrameValue[]> {
    return this.documentReads.readHistory();
  }

  /** 🪟️ Restores one window's persisted-local config envelope; see {@link DocumentReads.loadWindowConfig}. */
  loadWindowConfig(entry: WindowConfigPackEntry): Promise<void> {
    return this.documentReads.loadWindowConfig(entry);
  }

  /** 🗂️ Reads every window config envelope; see {@link DocumentReads.readWindowConfigs}. */
  readWindowConfigs(): Promise<readonly WindowConfigPackEntry[]> {
    return this.documentReads.readWindowConfigs();
  }

  /** 🗃️ Restores a document archive atomically; see {@link ArchiveTransfer.load}. */
  loadDocumentArchive(archive: DocumentArchivePack, signal?: AbortSignal, progress?: (status: DocumentArchiveLoadStatus) => void): Promise<void> {
    return this.archives.load(archive, signal, progress);
  }

  /** 🔀️ Merges a read-back archive into the shown document; see {@link ArchiveTransfer.merge}. */
  mergeDocumentArchive(archive: DocumentArchivePack, signal?: AbortSignal, progress?: (status: DocumentArchiveLoadStatus) => void): Promise<DocumentArchiveMergeV1> {
    return this.archives.merge(archive, signal, progress);
  }

  /** 🧳️ Reads the generation-fenced document archive; see {@link ArchiveTransfer.read}. */
  readDocumentArchive(): Promise<DocumentArchivePack> {
    return this.archives.read();
  }

  /** 📤️ Produces one OUT port's current bytes; see {@link MediaCommands.mediaOut}. */
  mediaOut(port: string): Promise<AppFrameValue[]> {
    return this.media.mediaOut(port);
  }

  /** 🗳️ Delivers one typed media artifact; see {@link MediaCommands.mediaIn}. */
  mediaIn(port: string, descriptor: unknown, data: Uint8Array): Promise<AppFrameValue[]> {
    return this.media.mediaIn(port, descriptor, data);
  }

  /** 🚀️ Admits media work against an exact document and revision; see {@link MediaCommands.submitMediaExport}. */
  submitMediaExport(port: string, parentDocumentId: string, revision: bigint): Promise<AppFrameValue[]> {
    return this.media.submitMediaExport(port, parentDocumentId, revision);
  }

  /** ⏱️ Advances one bounded media-export slice; see {@link MediaCommands.pollMediaExport}. */
  pollMediaExport(handle: MediaExportHandle): Promise<AppFrameValue[]> {
    return this.media.pollMediaExport(handle);
  }

  /** 🛑️ Cancels the submitted media generation; see {@link MediaCommands.cancelMediaExport}. */
  cancelMediaExport(handle: MediaExportHandle): Promise<AppFrameValue[]> {
    return this.media.cancelMediaExport(handle);
  }

  /** 🧺️ Takes one bounded output chunk; see {@link MediaCommands.takeMediaExportChunk}. */
  takeMediaExportChunk(handle: MediaExportHandle): Promise<AppFrameValue[]> {
    return this.media.takeMediaExportChunk(handle);
  }

  //#region 🔖️Merge
  /** 🧭️ Sets the local merge-policy authority; see {@link ReplicationCommands.setMergePolicy}. */
  setMergePolicy(policy: MergePolicy): Promise<AppFrameValue[]> {
    return this.replication.setMergePolicy(policy);
  }

  /** ⚔️ Accepts or discards an open conflict; see {@link ReplicationCommands.resolveConflict}. */
  resolveConflict(conflictId: string, resolution: ConflictResolution): Promise<AppFrameValue[]> {
    return this.replication.resolveConflict(conflictId, resolution);
  }

  /** 📖️ Reads the open-conflict projection; see {@link ReplicationCommands.readConflicts}. */
  readConflicts(): Promise<AppFrameValue[]> {
    return this.replication.readConflicts();
  }
  //#endregion 🔖️Merge

  //#region 🔖️Presence
  /** 🫂️ Pushes the document-wide presence roster; see {@link ReplicationCommands.pushPresence}. */
  pushPresence(ownColor: number | null, peers: readonly ArtifactPresencePeer[]): Promise<AppFrameValue[]> {
    return this.replication.pushPresence(ownColor, peers);
  }
  //#endregion 🔖️Presence

  //#region 🔖️Transaction
  /** 🏷️ Prepares a transaction in owner-mutation form; see {@link TransactionCommands.prepareOwner}. */
  transactionPrepareOwner(txnId: string, mutationId: string, payload: Uint8Array): Promise<AppFrameValue[]> {
    return this.transactions.prepareOwner(txnId, mutationId, payload);
  }

  /** 🗓️ Prepares a transaction in pre-planned form; see {@link TransactionCommands.preparePlanned}. */
  transactionPreparePlanned(txnId: string, preparedOps: readonly Uint8Array[], origin: Uint8Array): Promise<AppFrameValue[]> {
    return this.transactions.preparePlanned(txnId, preparedOps, origin);
  }

  /** ✅️ Commits the prepared transaction; see {@link TransactionCommands.commit}. */
  transactionCommit(txnId: string): Promise<AppFrameValue[]> {
    return this.transactions.commit(txnId);
  }

  /** ↩️ Rolls the prepared transaction back; see {@link TransactionCommands.rollback}. */
  transactionRollback(txnId: string): Promise<AppFrameValue[]> {
    return this.transactions.rollback(txnId);
  }

  /** 🎁️ Group undo, one member's half; see {@link TransactionCommands.undo}. */
  transactionUndo(groupId: string): Promise<AppFrameValue[]> {
    return this.transactions.undo(groupId);
  }

  /** 🔁️ Group redo, one member's half; see {@link TransactionCommands.redo}. */
  transactionRedo(groupId: string): Promise<AppFrameValue[]> {
    return this.transactions.redo(groupId);
  }
  //#endregion 🔖️Transaction
}
//#endregion 🔖️AppChannelClient
