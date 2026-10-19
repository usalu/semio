/** 🔁️ The host effects that continue program work: re-dispatch, media frame requests and extension invocation.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useApplyHostEffects}: each function is one effect arm of the `applyHostEffects` pass, moved verbatim. */
import { type Effect, resolveUiDirtyScope, type UiDirtyScope } from "@semio-tech/framework";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { makeEffectDispatchOne } from "../../../🛠️ShellHelpers/🔩️plumbing/🚀️effect-dispatch/🟦️.ts";
import { requestMediaFramesSourceV1, runMediaFramesV1, scheduleDispatchAction } from "../../../🛠️ShellHelpers/🎞️media-import/🟦️.ts";
import { parsePanelState } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { isPluginInstanceRetiredV1 } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { dispatchInvokeExtensionEffect } from "../../🔌️extension-runtime/🟦️.ts";
import type { ShellCoreAfterSpawnedSession } from "../../🛫️spawned-session/🟦️.ts";

/** 🔂️ Schedules the program's self re-dispatch through the host continuation scheduler. */
export function applyDispatchActionEffect({ loadedPluginsRef, isCurrentEffectOwner, resolvedTargetViewState, effect, baseSession, applyHostEffects, effectOwner }: Pick<ShellCoreAfterSpawnedSession, "loadedPluginsRef" | "isCurrentEffectOwner" | "resolvedTargetViewState"> & {
  readonly effect: Extract<Effect, Readonly<Record<"dispatchAction", unknown>>>;
  readonly baseSession: ActiveSession;
  readonly applyHostEffects: (effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>) => Promise<void>;
  readonly effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>;
}) {
  const { action: dispatchActionId, args: dispatchArgs, delayMs } = effect.dispatchAction;
  const pluginEntry = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === baseSession.pluginId);
  if (!pluginEntry) {
    console.error(`[os-shell] dispatchAction "${dispatchActionId}" dropped: no loaded program for "${baseSession.pluginId}" (loaded: ${loadedPluginsRef.current.map((entry) => entry.handle.pluginId).join(", ") || "none"})`);
    return;
  }
  scheduleDispatchAction(dispatchActionId, dispatchArgs as Record<string, unknown> | undefined, delayMs, makeEffectDispatchOne(pluginEntry, baseSession, (effects, target, scope) => applyHostEffects(effects, target, scope, effectOwner), () => isCurrentEffectOwner(effectOwner), resolvedTargetViewState, { causedBy: effectOwner.inputSeq, windowId: baseSession.viewState.windowId ?? null }));
}

/** 🎥️ Answers a media frame request through the program's media transport. */
export async function applyRequestMediaFramesEffect({ loadedPlugins, isCurrentEffectOwner, resolvedTargetViewState, effect, baseSession, core, applyHostEffects, effectOwner }: Pick<ShellCoreAfterSpawnedSession, "loadedPlugins" | "isCurrentEffectOwner" | "resolvedTargetViewState"> & {
  readonly effect: Extract<Effect, Readonly<Record<"requestMediaFrames", unknown>>>;
  readonly baseSession: ActiveSession;
  readonly core: ShellCoreAfterSpawnedSession;
  readonly applyHostEffects: (effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>) => Promise<void>;
  readonly effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>;
}) {
  const { accept, payload, frameAction, doneAction, fallbackAction, sampleStride, maxFrames, maxLongEdgePx, fpsHint, args } = effect.requestMediaFrames;
  const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
  const source = pluginEntry ? await requestMediaFramesSourceV1(accept, payload) : null;
  if (pluginEntry && source) {
    const task = core.documentTransferRef.current.trackDocumentTransfer(source.name, baseSession.pluginId);
    try {
      const dispatchOne = makeEffectDispatchOne(pluginEntry, baseSession, (effects, target, scope) => applyHostEffects(effects, target, scope, effectOwner), () => isCurrentEffectOwner(effectOwner), resolvedTargetViewState, { causedBy: effectOwner.inputSeq, windowId: baseSession.viewState.windowId ?? null });
      const outcome = await runMediaFramesV1({ frameAction, doneAction, fallbackAction, sampleStride: sampleStride ?? 0, maxFrames: maxFrames ?? 0, maxLongEdgePx: maxLongEdgePx ?? 0, fpsHint: fpsHint ?? 0, args: args as Record<string, unknown> | undefined }, source, dispatchOne, baseSession.app, task.signal, task.progress);
      if (outcome === "cancelled") core.documentTransferRef.current.notifyDocumentTransfer("import-cancelled", source.name, "info");
    } finally {
      task.finish();
    }
  }
}

/** 🧩️ Invokes an extension capability and feeds its completion back through the effect pass. */
export function applyInvokeExtensionEffect({ loadedPluginsRef, sessionRef, sessionWorkRef, dropForSealedInstance, isCurrentEffectOwner, applyHistoryPatch, applyLeftoverInteractionView, sealedInstancesRef, baseSession, effect, effectOwner, applyHostEffects }: Pick<ShellCoreAfterSpawnedSession, "loadedPluginsRef" | "sessionRef" | "sessionWorkRef" | "dropForSealedInstance" | "isCurrentEffectOwner" | "applyHistoryPatch" | "applyLeftoverInteractionView" | "sealedInstancesRef"> & {
  readonly baseSession: ActiveSession;
  readonly effect: Extract<Effect, Readonly<Record<"invokeExtension", unknown>>>;
  readonly effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>;
  readonly applyHostEffects: (effects: readonly Effect[], baseSession: ActiveSession, uiScope: UiDirtyScope | undefined, effectOwner: ReturnType<ShellCoreAfterSpawnedSession["captureEffectOwner"]>) => Promise<void>;
}) {
  const pluginsNow = loadedPluginsRef.current;
  const requesterId = pluginsNow.some((entry) => entry.handle.pluginId === baseSession.pluginId) ? baseSession.pluginId : sessionRef.current?.pluginId;
  const requester = requesterId && pluginsNow.some((entry) => entry.handle.pluginId === requesterId) ? { pluginId: requesterId, instanceId: baseSession.instanceId } : baseSession;
  const releaseExtensionWork = sessionWorkRef.current.begin(requester.pluginId, requester.instanceId, "extension-invocation");
  void dispatchInvokeExtensionEffect(pluginsNow, requester, effect.invokeExtension, async (requestingPlugin, response) => {
    if (dropForSealedInstance(requester, "extension answer", effect.invokeExtension.extensionId)) throw new Error("extension.requester-retired");
    if (!isCurrentEffectOwner(effectOwner)) throw new Error("extension.requester-retired");
    const current = sessionRef.current;
    const handle = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === requester.pluginId)?.handle;
    if (!current || handle !== requestingPlugin.handle) throw new Error("extension.requester-retired");
    const primary = current.pluginId === baseSession.pluginId && current.instanceId === baseSession.instanceId;
    const spawned = parsePanelState(current.viewState)?.spawnedApps.some((entry) => entry.pluginId === baseSession.pluginId && entry.instanceId === baseSession.instanceId);
    if (!primary && !spawned) throw new Error("extension.requester-retired");
    applyHistoryPatch(response.historyPatch, false, { pluginId: baseSession.pluginId, instanceId: baseSession.instanceId });
    applyLeftoverInteractionView(response.output);
    await applyHostEffects(response.requestedEffects ?? [], primary ? current : { ...baseSession, viewState: current.viewState }, resolveUiDirtyScope(response.uiScope), effectOwner);
  }).catch((error) => {
    const { extensionId, capability, req } = effect.invokeExtension;
    if (sealedInstancesRef.current.sealed(requester.pluginId, requester.instanceId)) return;
    if (isPluginInstanceRetiredV1(error)) {
      return;
    }
  }).finally(releaseExtensionWork);
}
