import { OperationDiagnosticObservers } from "@semio-tech/framework/actor/wire-turn/diagnostic/delivery";
import { operationDiagnosticFanouts, type PluginOperationDiagnostic } from "../📬️typed-result/🟦️.ts";
/** 🔗️ `adaptPluginHandle`, the `AppChannelClient` adapter over the kernel handle, with the bounded history-read retry and the shell actor identity.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { mediaTransportPort } from "@semio-tech/framework/os/modules/renderer/engine/media/channel";
import { type Effect, type HistoryPatch, type InvocationResponse } from "@semio-tech/framework";
import {
  decodePackValue
} from "@semio-tech/framework/packs";
import {
  AppChannelClient,
  AppChannelRequestSequence,
  decodeConflictsFromWire,
  decodeMergeReportFromWire,
  faultDisplayMessage
} from "@semio-tech/framework/app-channel";
import {
  type AppFrameValue
} from "@semio-tech/framework/wire-contracts";
import {
  decodeMutationEnvelopesPack,
  decodePackWire
} from "@semio-tech/framework/packs";
import { decodeLocalInteractionCaptureJson, LOCAL_INTERACTION_CAPTURE_MAX_BYTES, localInteractionIdentityEquals } from "@semio-tech/framework/replication";
import { type PluginWasmHandle as KernelPluginWasmHandle } from "@semio-tech/framework";
import { type PluginManifest, type PluginOperationCompletion, type PluginWasmHandle } from "../🧷️handle-contract/🟦️.ts";
import { wireEffectToFriendly } from "../🪟️retained-ui-patch/🟦️.ts";
import { TYPED_OPERATION_PENDING_OUTPUT, TYPED_OPERATION_TERMINAL_OUTPUT, TYPED_OPERATION_TERMINAL_SEEN } from "../📬️typed-result/🟦️.ts";
import { completionFanouts, pendingCompletionEffects, takeTypedOperationTerminalOutputV1 } from "../🏁️operation-completion/🟦️.ts";
import {
  forgetPluginInstanceRetirementV1,
  markPluginInstanceRetiredForPluginV1,
  markPluginInstanceRetiredV1,
  pluginInstanceWasRetiredV1
} from "../🪦️instance-retirement/🟦️.ts";
import { noteGuestIngressV1, performContextMenu, performInvocation } from "../📥️invocation-frames/🟦️.ts";

//#region 🔖️plugin-runtime
//#region 🔖️ChannelAdapter
//#region 🔖️ActorIdentity
/** 🪪️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0/§C3 — the shell's
 * current actor id (`user:{userId}#{shellSessionId}` once identity resolves, `client-<random>` before
 * that), stamped onto every plugin instance opened from here on. Module-level rather than an
 * `adaptPluginHandle` parameter because `PluginWasmHandle` is the kernel's frozen shape (no setter
 * method to add) and every real call site (`loadPluginModuleResilient`, `🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx`)
 * lives outside this ticket's lease — this is the smallest surface that reaches every future
 * `createApp` call without touching a foreign-leased signature. Known limitation: a single JS realm
 * hosting more than one `🏛️ShellHost` (e.g. the multi-pane demonstrator) shares one actor id across
 * panes — out of scope for this lane, flagged in `📓️w2-c-report.md`. */
export let currentPluginRuntimeActor = "local";

/** 🪪️ Sets the actor id every subsequently-opened plugin instance is stamped with — call once the
 * shell mints/resolves its `user:{userId}#{shellSessionId}` identity (or reverts it on sign-out). */
export function setPluginRuntimeActor(actor: string): void {
  currentPluginRuntimeActor = actor;
}
//#endregion 🔖️ActorIdentity

//#region 🔖️HistorySnapshotRetry
/** 🧮️ Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice C1 — how many times a history
 * snapshot read may be re-issued against the one guest refusal that is transient by construction.
 * `plugin_exchange`'s `ReadHistory` arm reaches the instance cell through `try_lock`
 * (`🔨️modules/🔌️plugin/🦀️.rs:35395`/`:35681`), so a snapshot read that lands while the SAME instance
 * is already mid-turn is refused with `instance busy or poisoned: N` rather than queued behind it. A
 * refusal therefore carries no `HistorySnapshot` frame at all, which is the literal condition the old
 * code reported as `missing HistorySnapshot frame` — a sentence that named the symptom and discarded
 * the guest's own fault text, so every `📓️final-summary.md` reading of it blamed the hub's
 * `document_ws_v1` bootstrap for a frame the hub never sends. Four attempts covers one guest turn. */
export const PLUGIN_INSTANCE_BUSY_MAX_ATTEMPTS = 4;

/** ⏱️ The first backoff step of the ladder above; each further attempt doubles it (25, 50, 100 ms),
 * so an exhausted ladder has waited 175 ms — longer than a guest turn's 8 ms ceiling by two orders of
 * magnitude, and still bounded, which is the whole point: `🏛️ShellHost`'s history effects are
 * edge-triggered and never re-armed by their own callers, so an exhausted ladder must surface the real
 * fault instead of spinning. */
export const PLUGIN_INSTANCE_BUSY_BACKOFF_MS = 25;

/** 🚦️ True only for the transient `try_lock` refusal, never for a real app fault. Anything else — a
 * poisoned document, a rejected projection, a guest trap — is terminal and must be raised on the first
 * reply rather than hidden behind three more round trips. */
export function isPluginInstanceBusyFaultV1(message: string): boolean {
  return /instance busy or poisoned/u.test(message);
}

/** 🧾️ Reads one instance's whole history projection, retrying a bounded number of times against a
 * transient `instance busy` refusal and raising the guest's own fault text on anything else. `delay`
 * is injectable so the renderer's own test drives the ladder without real timers. */
export async function readHistoryWithBoundedRetryV1(
  channel: AppChannelClient,
  instanceId: number,
  delay: (ms: number) => Promise<void> = (ms) => new Promise<void>((resolve) => { setTimeout(resolve, ms); }),
): Promise<HistoryPatch> {
  let refusal = "";
  for (let attempt = 0; attempt < PLUGIN_INSTANCE_BUSY_MAX_ATTEMPTS; attempt += 1) {
    const frames = await channel.readHistory();
    const snapshot = frames.find((candidate): candidate is Extract<AppFrameValue, { readonly HistorySnapshot: unknown }> => "HistorySnapshot" in candidate);
    if (snapshot) return decodePackWire(new Uint8Array(snapshot.HistorySnapshot.history_patch), "$.historyPatch") as HistoryPatch;
    const errorFrame = frames.find((candidate): candidate is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in candidate);
    refusal = errorFrame ? faultDisplayMessage(errorFrame.Error.fault, decodePackValue) : `no HistorySnapshot and no Error frame among ${frames.length} reply frame(s)`;
    if (!errorFrame || !isPluginInstanceBusyFaultV1(refusal)) break;
    if (attempt + 1 < PLUGIN_INSTANCE_BUSY_MAX_ATTEMPTS) await delay(PLUGIN_INSTANCE_BUSY_BACKOFF_MS * 2 ** attempt);
  }
  throw new Error(`readHistory(${instanceId}) failed: ${refusal}`);
}
//#endregion 🔖️HistorySnapshotRetry

/** 📡️ Wraps the framework-core `PluginWasmHandle` (the `enqueue`/`outcomes` turn ABI) behind the
 * SAME method surface the rest of this file already calls — the compatibility adapter for
 * `HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS`'s ABI flip. One `AppChannelClient` per
 * live instance id (created in `createApp`, dropped in `destroyApp`) frames every call through
 * `AppCommand`/`AppFrame`; no `AppCommand::Hello` handshake is sent — `plugin_exchange` already
 * defaults an un-`Hello`'d instance's actor to `"local"` (see `instance_actor`'s doc), so skipping it
 * avoids the alternative (sending a real `Hello.config`, which would run every migrated app's
 * `apply_config_bytes` against an arbitrary empty/placeholder config — wrong for an app like shooting
 * whose `ShootingConfig` fields have no `#[serde(default)]` and would reject `{}`). */
export async function adaptPluginHandle(pluginId: string, lease: { readonly handle: KernelPluginWasmHandle; readonly release: () => Promise<void> }): Promise<PluginWasmHandle> {
  const handle = lease.handle;
  const manifest = handle.manifest as unknown as PluginManifest;
  const channels = new Map<number, AppChannelClient>();
  const channelRequests = new AppChannelRequestSequence();
  let disposal: Promise<void> | null = null;
  let disposing = false;
  const requireChannel = (instanceId: number): AppChannelClient => {
    if (disposing) throw new Error("plugin-handle.closed");
    const client = channels.get(instanceId);
    if (!client) {
      const missing = new Error(`program ${pluginId}: no channel for instance ${instanceId} (createApp not called, or already destroyed)`);
      throw pluginInstanceWasRetiredV1(pluginId, instanceId) ? markPluginInstanceRetiredV1(missing) : missing;
    }
    return client;
  };
  return {
    pluginId,
    manifest,
    createApp: async (appId) => {
      if (disposing) throw new Error("plugin-handle.closed");
      const instanceId = await handle.createApp(appId);
      if (disposing) { await handle.destroyApp(instanceId); throw new Error("plugin-handle.closed"); }
      forgetPluginInstanceRetirementV1(pluginId, instanceId);
      channels.set(instanceId, new AppChannelClient(handle, channelRequests, instanceId, appId));
      return instanceId;
    },
    /** 🪦️ Retires this instance's DISPATCH QUEUE before its actor is destroyed, never after.
     *
     * The close ladder runs on the actor's own lifecycle lease, not on this channel, so the channel
     * used to stay open and reachable for the whole of it: `requireChannel` answered, the command
     * crossed into `AppChannelClient.sendCommand`, and only `requireActorId` — one layer down, with the
     * actor already revoked — refused it. A late gesture therefore surfaced as a stack instead of a
     * drop (measured on generation3d 6018: a node-graph host remount republished
     * `interactionSelect targets:[]` at +68 s onto the instance a hot-swap had destroyed,
     * `🗑️generated/react-verify/journey/console.txt`). Closing the queue first makes every such arrival
     * a typed {@link isPluginInstanceRetiredV1} refusal raised by the layer that OWNS the queue, and
     * settles the pending gestures that were already in it with the same terminal. */
    destroyApp: async (instanceId) => {
      const channel = channels.get(instanceId);
      markPluginInstanceRetiredForPluginV1(pluginId, instanceId);
      channel?.retire();
      await handle.destroyApp(instanceId);
      channel?.dispose();
      if (channels.get(instanceId) === channel) channels.delete(instanceId);
    },
    takeSegmentedDownloadChunk: (instanceId, operationId) => handle.takeSegmentedDownloadChunk(instanceId, operationId),
    handleAction: async (instanceId, actionJson, viewState, dispatch) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(actionJson), "action", viewState, dispatch),
    handleCommand: async (instanceId, commandJson, viewState, dispatch) => performInvocation(requireChannel(instanceId), instanceId, JSON.parse(commandJson), "command", viewState, dispatch),
    refreshUi: async () => ({}),
    contextMenu: async (instanceId, request, viewState) => performContextMenu(requireChannel(instanceId), request, viewState),
    readHistory: async (instanceId) => readHistoryWithBoundedRetryV1(requireChannel(instanceId), instanceId),
    applyMutations: async (instanceId, mutationsPack) => {
      const envelopes = decodeMutationEnvelopesPack(mutationsPack);
      const frames = await requireChannel(instanceId).applyEnvelopes(envelopes);
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`applyMutations failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
      const mergeFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly MergeReport: unknown }> => "MergeReport" in frame);
      const conflictsFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Conflicts: unknown }> => "Conflicts" in frame);
      return {
        mergeReport: mergeFrame ? decodeMergeReportFromWire(mergeFrame.MergeReport.report, decodePackValue) : null,
        conflicts: conflictsFrame ? decodeConflictsFromWire(conflictsFrame.Conflicts.conflicts, decodePackValue) : null,
      };
    },
    readAppDocumentPack: async (instanceId) => {
      const frames = await requireChannel(instanceId).readDocument();
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`readAppDocumentPack failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
      const documentFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Document: unknown }> => "Document" in frame);
      return documentFrame
        ? { pack: new Uint8Array(documentFrame.Document.pack), spr: new Uint8Array(documentFrame.Document.spr), ops: documentFrame.Document.ops }
        : null;
    },
    exportAppMedia: async (instanceId, port) => {
      const frames = await requireChannel(instanceId).mediaOut(port);
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`exportAppMedia failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
      const mediaFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Media: unknown }> => "Media" in frame);
      return mediaFrame ? { port: mediaFrame.Media.port, descriptor: new Uint8Array(mediaFrame.Media.descriptor), data: new Uint8Array(mediaFrame.Media.data) } : null;
    },
    importAppMedia: async (instanceId, port, descriptor, data) => {
      const frames = await requireChannel(instanceId).mediaIn(port, descriptor, data);
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`importAppMedia failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
      if (!frames.some((frame) => "Done" in frame)) throw new Error("importAppMedia failed: missing completion receipt");
    },
    ...mediaTransportPort(requireChannel),
    readAppDocumentArchive: (instanceId) => requireChannel(instanceId).readDocumentArchive(),
    readAppDocumentIdentity: (instanceId) => requireChannel(instanceId).readDocumentIdentity(),
    loadAppDocumentArchive: (instanceId, archive, signal, progress) => requireChannel(instanceId).loadDocumentArchive(archive, signal, progress),
    mergeAppDocumentArchive: (instanceId, archive, signal, progress) => requireChannel(instanceId).mergeDocumentArchive(archive, signal, progress),
    readWindowConfigPacks: (instanceId) => requireChannel(instanceId).readWindowConfigs(),
    loadWindowConfigPack: (instanceId, entry) => requireChannel(instanceId).loadWindowConfig(entry),
    ephemeralSnapshot: async (instanceId) => requireChannel(instanceId).ephemeral(),
    pushPresence: async (instanceId, ownColor, peers) => {
      await requireChannel(instanceId).pushPresence(ownColor, peers);
    },
    documentPack: (instanceId) => requireChannel(instanceId).documentPack(),
    transactionPrepare: async (instanceId, txnId, request) => {
      const frames =
        request.form === "owner"
          ? await requireChannel(instanceId).transactionPrepareOwner(txnId, request.mutationId, request.payload)
          : await requireChannel(instanceId).transactionPreparePlanned(txnId, request.preparedOps, request.origin);
      const frame = frames.find((candidate): candidate is Extract<AppFrameValue, { readonly transactionPrepared: unknown }> => "transactionPrepared" in candidate);
      if (!frame) throw new Error(`program ${pluginId}: transactionPrepare(${instanceId}): missing transactionPrepared frame`);
      return {
        foreign: frame.transactionPrepared.foreign.map((bytes) => new Uint8Array(bytes)),
        rejection: frame.transactionPrepared.rejection.length > 0 ? new Uint8Array(frame.transactionPrepared.rejection) : null,
      };
    },
    transactionCommit: async (instanceId, txnId) => {
      const frames = await requireChannel(instanceId).transactionCommit(txnId);
      const committed = frames.find((candidate): candidate is Extract<AppFrameValue, { readonly transactionCommitted: unknown }> => "transactionCommitted" in candidate);
      if (committed) return { editId: committed.transactionCommitted.edit_id };
      const errorFrame = frames.find((candidate): candidate is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in candidate);
      if (errorFrame) return { rejection: new Uint8Array(errorFrame.Error.fault) };
      throw new Error(`program ${pluginId}: transactionCommit(${instanceId}): missing transactionCommitted/Error frame`);
    },
    transactionRollback: async (instanceId, txnId) => {
      await requireChannel(instanceId).transactionRollback(txnId);
    },
    transactionUndo: async (instanceId, groupId) => {
      await requireChannel(instanceId).transactionUndo(groupId);
    },
    transactionRedo: async (instanceId, groupId) => {
      await requireChannel(instanceId).transactionRedo(groupId);
    },
    //#region 🔖️Merge
    setMergePolicy: async (instanceId, policy) => {
      const frames = await requireChannel(instanceId).setMergePolicy(policy);
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`program ${pluginId}: setMergePolicy failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
    },
    resolveConflict: async (instanceId, conflictId, resolution) => {
      const frames = await requireChannel(instanceId).resolveConflict(conflictId, resolution);
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`program ${pluginId}: resolveConflict failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
      const mergeFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly MergeReport: unknown }> => "MergeReport" in frame);
      const conflictsFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Conflicts: unknown }> => "Conflicts" in frame);
      return {
        mergeReport: mergeFrame ? decodeMergeReportFromWire(mergeFrame.MergeReport.report, decodePackValue) : null,
        conflicts: conflictsFrame ? decodeConflictsFromWire(conflictsFrame.Conflicts.conflicts, decodePackValue) : null,
      };
    },
    readConflicts: async (instanceId) => {
      const frames = await requireChannel(instanceId).readConflicts();
      const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
      if (errorFrame) throw new Error(`program ${pluginId}: readConflicts failed: ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
      const conflictsFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Conflicts: unknown }> => "Conflicts" in frame);
      return conflictsFrame ? decodeConflictsFromWire(conflictsFrame.Conflicts.conflicts, decodePackValue) : [];
    },
    //#endregion 🔖️Merge
    readLocalInteraction: async (instanceId, signal) => {
      const pages: Uint8Array[] = [];
      let length = 0;
      const identity = await requireChannel(instanceId).readLocalInteractionPages(async (page) => {
        length += page.bytes.length;
        if (length > LOCAL_INTERACTION_CAPTURE_MAX_BYTES) throw new Error("local-interaction.capture-length");
        pages.push(Uint8Array.from(page.bytes));
      }, signal);
      const bytes = new Uint8Array(length);
      let offset = 0;
      for (const page of pages) {
        bytes.set(page, offset);
        offset += page.length;
      }
      const capture = decodeLocalInteractionCaptureJson(bytes);
      if (!localInteractionIdentityEquals(capture.identity, identity)) throw new Error("local-interaction.capture-authority");
      return capture;
    },
    subscribeOperationDiagnostics: (instanceId, listener) => {
      requireChannel(instanceId);
      let listeners = operationDiagnosticFanouts.get(instanceId);
      if (!listeners) { listeners = new OperationDiagnosticObservers<PluginOperationDiagnostic>(64, error => console.error("plugin.diagnostic-observer-failed", error)); operationDiagnosticFanouts.set(instanceId, listeners); }
      const returned = listeners.subscribe(listener);
      const original = listeners;
      return () => { returned(); if (original.size === 0 && operationDiagnosticFanouts.get(instanceId) === original) operationDiagnosticFanouts.delete(instanceId); };
    },
    subscribeOperationCompletions: (instanceId, listener) => {
      const channel = requireChannel(instanceId);
      let fanout = completionFanouts.get(instanceId);
      if (!fanout) {
        const listeners = new Set<(completion: PluginOperationCompletion) => void>();
        const dispose = channel.onOperationCompleted((completion) => {
          const terminalOutput = takeTypedOperationTerminalOutputV1(instanceId);
          const leftover = pendingCompletionEffects.get(instanceId) ?? [];
          pendingCompletionEffects.delete(instanceId);
          const published: PluginOperationCompletion = {
            instanceId,
            operation: completion.operation,
            revision: completion.revision,
            uiScope: completion.uiScope as InvocationResponse["uiScope"],
            historyPatch: completion.historyPatch as HistoryPatch | undefined,
            requestedEffects: leftover
              .filter((effect) => effect.tag !== TYPED_OPERATION_TERMINAL_OUTPUT && effect.tag !== TYPED_OPERATION_PENDING_OUTPUT && effect.tag !== TYPED_OPERATION_TERMINAL_SEEN)
              .map(wireEffectToFriendly)
              .filter((effect): effect is Effect => effect !== null),
            terminalOutput,
          };
          for (const subscriber of [...listeners]) {
            try { subscriber(published); }
            catch (error) { console.error("operation-completion subscriber failed", error); }
          }
        });
        fanout = { listeners, dispose };
        completionFanouts.set(instanceId, fanout);
      }
      const bound = fanout;
      bound.listeners.add(listener);
      return () => {
        if (!bound.listeners.delete(listener) || bound.listeners.size > 0) return;
        if (completionFanouts.get(instanceId) === bound) completionFanouts.delete(instanceId);
        bound.dispose();
      };
    },
    subscribeOperationProgress: (instanceId, listener) =>
      requireChannel(instanceId).onOperationProgress((progress) => {
        noteGuestIngressV1(instanceId);
        listener(progress as { readonly uiScope?: InvocationResponse["uiScope"]; readonly historyPatch?: HistoryPatch });
      }),
    dispose: () => {
      if (disposal) return disposal;
      disposing = true;
      disposal = lease.release().then(() => {
        for (const channel of channels.values()) channel.dispose();
        channels.clear();
      });
      return disposal;
    },
  };
}
//#endregion 🔖️ChannelAdapter
//#endregion 🔖️plugin-runtime
