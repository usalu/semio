/** 📥️ The host effects that bring a document in: archive loading and file open requests.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useApplyHostEffects}: each function is one effect arm of the `applyHostEffects` pass, moved verbatim. */
import { type Effect, type UiDirtyScope } from "@semio-tech/framework";
import { programHistoryKeyV1 } from "../../🪟️spawned-program/🟦️.ts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { documentTransferNoticeCodeV1, documentLoadCancelledV1 } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { requestFileOpen } from "../../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { makeEffectDispatchOne } from "../../../🛠️ShellHelpers/🔩️plumbing/🚀️effect-dispatch/🟦️.ts";
import { programHistoryProjectionsWithLoadV1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { importOpenedFilesV1 } from "../../../🛠️ShellHelpers/🎞️media-import/🟦️.ts";
import { resolveManifestLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { coerceWireBytes } from "../../../🔌️PluginRuntime/🟦️.tsx";
import type { ShellCoreAfterSpawnedSession } from "../../🛫️spawned-session/🟦️.ts";

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
    const showLoad = (load: { readonly completed: number; readonly total: number } | null) => historyStore.update((projections) => programHistoryProjectionsWithLoadV1(projections, programHistoryKeyV1(baseSession), load));
    try {
      if (await loadDocumentPair(pluginEntry.handle, baseSession.instanceId, packBytes, sprBytes, () => isCurrentEffectOwner(effectOwner), { signal: task.signal, progress: (status) => { task.progress(status.completed, status.total > 0 ? status.total : null); showLoad(status); } })) await refreshHistorySnapshot(baseSession);
    } catch (error) {
      const cancelled = documentLoadCancelledV1(error, task.signal);
      console.warn(`[os-shell] ${documentTransferNoticeCodeV1(cancelled ? "load-cancelled" : "load-failed")} ${baseSession.pluginId}#${baseSession.instanceId}`, error);
      core.documentTransferRef.current.notifyDocumentTransfer(cancelled ? "load-cancelled" : "load-failed", file, cancelled ? "info" : "error");
    } finally {
      showLoad(null);
      task.finish();
    }
  } else {
    console.error("[os-shell] loadDocument: program has no archive loader", baseSession.pluginId, Object.keys(payload));
  }
}

/** 📂️ Reads the files the user picks and imports them through the program's import action. */
export async function applyRequestFileOpenEffect({ loadedPlugins, isCurrentEffectOwner, resolvedTargetViewState, effect, baseSession, core, applyHostEffects, effectOwner }: Pick<ShellCoreAfterSpawnedSession, "loadedPlugins" | "isCurrentEffectOwner" | "resolvedTargetViewState"> & {
  readonly effect: Extract<Effect, Readonly<Record<"requestFileOpen", unknown>>>;
  readonly baseSession: ActiveSession;
  readonly core: ShellCoreAfterSpawnedSession;
  readonly applyHostEffects: (effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>) => Promise<void>;
  readonly effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>;
}) {
  const { accept, readAs, importAction, multiple, args } = effect.requestFileOpen;
  const resolvedImport = importAction || "importSnapshot";
  const opened = await requestFileOpen(accept || ".spk,.dsl,.ops,application/octet-stream", readAs, multiple);
  if (opened.length > 0) {
    const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
    if (pluginEntry) {
      const task = core.documentTransferRef.current.trackDocumentTransfer(opened.map((file) => file.name).join(", "), baseSession.pluginId);
      try {
        const dispatchOne = makeEffectDispatchOne(pluginEntry, baseSession, (effects, target, scope) => applyHostEffects(effects, target, scope, effectOwner), () => isCurrentEffectOwner(effectOwner), resolvedTargetViewState, { causedBy: effectOwner.inputSeq, windowId: baseSession.viewState.windowId ?? null });
        if ((await importOpenedFilesV1(baseSession.app, opened, resolvedImport, Boolean(multiple), dispatchOne, task.signal, task.progress, args as Readonly<Record<string, unknown>> | undefined)) === "cancelled") core.documentTransferRef.current.notifyDocumentTransfer("import-cancelled", opened[0]!.name, "info");
      } finally {
        task.finish();
      }
    }
  }
}
