/** 📥️ The host effects that bring a document in: archive loading and file open requests.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useApplyHostEffects}: each function is one effect arm of the `applyHostEffects` pass, moved verbatim. */
import type { Effect } from "@semio-tech/framework";
import { programHistoryKeyV1 } from "../../🪟️spawned-program/🟦️.ts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { documentTransferNoticeCodeV1, documentLoadCancelledV1 } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { FileOpenSizeRefusalV1, readSelectedFilesBoundedV1, requestFileOpen, requestFileSelectionV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { makeEffectDispatchOne, ownedEffectInvocationV1, type EffectDispatchOne, type EffectInvocationSink } from "../../../🛠️ShellHelpers/🔩️plumbing/🚀️effect-dispatch/🟦️.ts";
import { programHistoryProjectionsWithLoadV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { importOpenedFilesV1 } from "../../../🛠️ShellHelpers/🎞️media-import/🟦️.ts";
import { resolveManifestLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { pickOriginalMedia } from "../../../🔌️PluginRuntime/🚚️handle-turn-drive/original-media-pick.ts";
import { originalMediaRequest } from "../../../🔌️PluginRuntime/🚚️handle-turn-drive/original-media-wire.ts";
import { coerceWireBytes } from "../../../🔌️PluginRuntime/🟦️.tsx";
import type { ShellCoreAfterSpawnedSession } from "../../🛫️spawned-session/🟦️.ts";
import { documentLoadRunningV1, noteDocumentLoadEndV1 } from "../../⏳️document-load-gate/🟦️.ts";
import { exampleRestoredAfterUnappliedLoadV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts";

/**
 * 📦️ Loads a pack and spr pair through the program's stepped archive loader with progress and cancellation.
 *
 * 🚧️ `Effect::LoadDocument` is pack+spr bytes only (no JSON-text fallback exists on the wire) and loads through the
 * stepped archive load — a program without `loadAppDocumentArchive` simply cannot receive this effect.
 */
export async function applyLoadDocumentEffect({ loadedPlugins, uiTerminologyRef, uiLocaleRef, historyStore, loadDocumentPair, isCurrentEffectOwner, refreshHistorySnapshot, baseSession, effect, core, effectOwner }: Pick<ShellCoreAfterSpawnedSession, "loadedPlugins" | "uiTerminologyRef" | "uiLocaleRef" | "historyStore" | "loadDocumentPair" | "isCurrentEffectOwner" | "refreshHistorySnapshot"> & {
  readonly baseSession: ActiveSession;
  readonly effect: Extract<Effect, Readonly<Record<"loadDocument", unknown>>>;
  readonly core: ShellCoreAfterSpawnedSession;
  readonly effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>;
}) {
  const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
  const payload = effect.loadDocument;
  if (payload.pack && payload.spr && pluginEntry?.handle.loadAppDocumentArchive) {
    const packBytes = coerceWireBytes(payload.pack);
    const sprBytes = coerceWireBytes(payload.spr);
    const file = resolveManifestLabel(baseSession.app.label, uiTerminologyRef.current, uiLocaleRef.current) || baseSession.app.id;
    const task = core.documentTransferRef.current.trackDocumentTransfer(file, baseSession.pluginId);
    task.signal.addEventListener("abort", () => core.documentLoadGateRef.current.markCancelling(baseSession.pluginId, baseSession.instanceId), { once: true });
    const exampleKey = programHistoryKeyV1(baseSession);
    let restored = false;
    const restoreExample = () => {
      if (restored || core.focusedExampleProgramRef.current !== exampleKey) return;
      restored = true;
      const restoration = exampleRestoredAfterUnappliedLoadV1(core.exampleBeforePickByProgramRef.current.get(exampleKey), core.committedExamplesByProgramRef.current.get(exampleKey));
      if (restoration === undefined) return;
      core.committedExamplesByProgramRef.current.set(exampleKey, restoration.memory);
      core.exampleBeforePickByProgramRef.current.delete(exampleKey);
      core.dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: restoration.visibleExampleId });
    };
    const showLoad = (load: { readonly completed: number; readonly total: number } | null) => historyStore.update((projections) => programHistoryProjectionsWithLoadV1(projections, programHistoryKeyV1(baseSession), load));
    try {
      const loaded = await loadDocumentPair(pluginEntry.handle, baseSession.instanceId, packBytes, sprBytes, () => isCurrentEffectOwner(effectOwner), { signal: task.signal, progress: (status) => { task.progress(status.completed, status.total > 0 ? status.total : null); const stopping = core.documentLoadGateRef.current.cancelling(baseSession.pluginId, baseSession.instanceId) || status.state === "cancelled"; showLoad(documentLoadRunningV1(status.state) && !stopping ? status : null); if (stopping) restoreExample(); } });
      if (loaded) { core.exampleBeforePickByProgramRef.current.delete(exampleKey); await refreshHistorySnapshot(baseSession); }
      return loaded;
    } catch (error) {
      const cancelled = documentLoadCancelledV1(error, task.signal);
      console.warn(`[os-shell] ${documentTransferNoticeCodeV1(cancelled ? "load-cancelled" : "load-failed")} ${baseSession.pluginId}#${baseSession.instanceId}`, error);
      core.documentTransferRef.current.notifyDocumentTransfer(cancelled ? "load-cancelled" : "load-failed", file, cancelled ? "info" : "error");
      noteDocumentLoadEndV1(effectOwner, cancelled);
      if (!cancelled) restoreExample();
    } finally {
      showLoad(null);
      task.finish();
    }
  } else {
    console.error("[os-shell] loadDocument: program has no archive loader", baseSession.pluginId, Object.keys(payload));
  }
  return false;
}

/** 📂️ Reads the files the user picks and imports them through the program's import action. */
export async function applyRequestFileOpenEffect({ loadedPlugins, isCurrentEffectOwner, resolvedTargetViewState, effect, baseSession, core, applyDeferredResponse, effectOwner }: Pick<ShellCoreAfterSpawnedSession, "loadedPlugins" | "isCurrentEffectOwner" | "resolvedTargetViewState"> & {
  readonly effect: Extract<Effect, Readonly<Record<"requestFileOpen", unknown>>>;
  readonly baseSession: ActiveSession;
  readonly core: ShellCoreAfterSpawnedSession;
  readonly applyDeferredResponse: EffectInvocationSink;
  readonly effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>;
}) {
  const { accept, readAs, importAction, multiple, args, maxFileBytes } = effect.requestFileOpen;
  const filter = accept || ".spk,.dsl,.ops,application/octet-stream";
  const actor = effectOwner.browserActor === undefined ? null : core.directBrowserActorForSession(baseSession);
  if (effectOwner.browserActor !== undefined && (actor === null || !isCurrentEffectOwner(effectOwner))) return;
  if (typeof args === "object" && args !== null && "originalMedia" in args) {
    if (actor !== null) throw new Error("original-media.signed-ingress-unavailable");
    const issued = args.originalMedia;
    const current = () => isCurrentEffectOwner(effectOwner);
    const host = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId)?.handle;
    if (!host?.openOriginalMediaIngress) throw new Error("original-media.host-unavailable");
    if (multiple) throw new Error("original-media.selection-count");
    const request = originalMediaRequest(issued, baseSession.instanceId);
    const exchange = host.openOriginalMediaIngress(baseSession.instanceId, request);
    const transfers = core.documentTransferRef.current;
    let pickedName = "";
    try {
      const status = await pickOriginalMedia(request, exchange, () => requestFileSelectionV1(filter), current, (file) => {
        pickedName = file.name;
        return transfers.trackDocumentTransfer(file.name, baseSession.pluginId);
      });
      if (status === "cancelled" && pickedName) transfers.notifyDocumentTransfer("import-cancelled", pickedName, "info");
    } catch (error) {
      if (!pickedName) throw error;
      const code = error instanceof Error && error.message === "The selected media exceeds its admitted source extent" ? "import-size-limit" : "import-failed";
      transfers.notifyDocumentTransfer(code, pickedName, "error");
    }
    return;
  }
  const selected = maxFileBytes === undefined ? undefined : await requestFileSelectionV1(filter, multiple);
  const unbounded = selected === undefined ? await requestFileOpen(filter, readAs, multiple) : [];
  const picked = selected ?? unbounded;
  if (picked.length === 0 || !isCurrentEffectOwner(effectOwner)) return;
  const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
  if (!pluginEntry) return;

  const transfers = core.documentTransferRef.current;
  const task = transfers.trackDocumentTransfer(picked.map((file) => file.name).join(", "), baseSession.pluginId);
  const phaseProgress = (start: number) => (completed: number, total: number | null) =>
    task.progress(start + (total ? Math.round(50 * completed / total) : 0), 100);
  try {
    const opened = selected === undefined || maxFileBytes === undefined ? unbounded :
      await readSelectedFilesBoundedV1(selected, readAs, maxFileBytes, { signal: task.signal, progress: phaseProgress(0) });
    const dispatchOne: EffectDispatchOne = actor === null
      ? makeEffectDispatchOne(pluginEntry, baseSession, applyDeferredResponse, () => isCurrentEffectOwner(effectOwner), resolvedTargetViewState, { causedBy: effectOwner.inputSeq, windowId: baseSession.viewState.windowId ?? null })
      : async (action, args) => {
        if (!isCurrentEffectOwner(effectOwner)) throw new Error("action-owner-mismatch: selected file owner retired");
        const viewState = resolvedTargetViewState(baseSession);
        const invocation = ownedEffectInvocationV1({ ...baseSession, viewState }, action, args);
        await core.dispatchDirectBrowserActorCommand(actor, invocation, viewState, effectOwner.inputSeq);
      };
    const status = await importOpenedFilesV1(baseSession.app, opened, importAction || "importSnapshot", Boolean(multiple), dispatchOne, task.signal, selected === undefined ? task.progress : phaseProgress(50), args as Readonly<Record<string, unknown>> | undefined);
    if (status === "cancelled") transfers.notifyDocumentTransfer("import-cancelled", picked[0]!.name, "info");
  } catch (error) {
    if (selected === undefined) throw error;
    if (error instanceof FileOpenSizeRefusalV1) {
      transfers.notifyDocumentTransfer("import-size-limit", error.fileName, "error");
    } else {
      const cancelled = documentLoadCancelledV1(error, task.signal);
      transfers.notifyDocumentTransfer(cancelled ? "import-cancelled" : "import-failed", picked[0]!.name, cancelled ? "info" : "error");
    }
  } finally {
    task.finish();
  }
}
