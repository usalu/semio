/** ⌨️ The utility and tool registers, the introduction callbacks, the browser actor view state and the context menu of the focus and input hook. */
import { shellDialogSessionIsCurrentV1 } from "../../🗨️dialog-origin/🟦️.ts";
import { type DocumentOpeningReceiptV1 } from "../../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { TutorialDriveV1 } from "../../🗨️dialog-origin/🎥️tutorial/🟦️.ts";
import { useCallback, useEffect, useMemo, useRef } from "react";
import { type ContextMenuItemSpec, type IntroductionInteraction, parseResolvedPluginViewState, type PluginContextMenuRequest, type PluginSourceEvent, retireContributorInstances, hostArmedViewContext, panelViewContext } from "@semio-tech/framework";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { type LocalInteractionState } from "@semio-tech/framework/replication";
import { CELEBRATE_STAMP_DURATION_MS, celebrateAllElements, celebrateElements, elementIdSelector, type WindowLayoutNode, writeStoredIntroductionSeen } from "@semio-tech/ui-react";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { ViewModel } from "../../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { leftoverWorldWindowOverlayV1, publishLeftoverWorldSelectionV1 } from "../../../🌐️World3dHost/🔗️instance-selection/🟦️.ts";
import { pluginAvailabilityRouteV1 } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { buildActiveUtilityByWindowId, sessionWindowInstances } from "../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { createLatestAsyncDispatcher } from "../../../🛠️ShellHelpers/🛑️extension-io/🟦️.ts";
import { shellLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { TutorialRecorder } from "../../🎬️tutorial-recorder/🟦️.tsx";
import { pluginInstallConcurrency } from "../../🔌️extension-runtime/🟦️.ts";
import type { ShellCoreAfterPluginLifecycle } from "../../🚀️plugin-lifecycle/🟦️.ts";
import type { OpenDocumentSession } from "../../🔒️identity-document-ports/🧰️identity-port-types/🟦️.ts";
import type { useFocusedProgramAndIntroductionState } from "../🎯️focused-program-and-introduction/🟦️.tsx";

/** 🧰 Builds the utility and tool register cells, the local interaction publishers and the introduction callbacks. */
export function useUtilityAndToolRegisters({ utilityRegisterCell, bumpRegisterRevision, activeUtilityByWindowIdRef, dispatch, toolRegisterRef, activeToolIdRef, focusedToolMeasuresByToolId, activeWindowId, actionPaneExpandedByWindowId, actionPaneStagedArgsByKey, introductionStepIndex, introductionCompletedInteractions, tutorialPlaying, tutorialRecording, shellState, core, overlayDialog, isCurrentDialogOrigin, closeOwnedDialog, session, sessionRef, loadedPluginsRef, localInteraction, dropForRetiredInstance, loadedPlugins, scope, dismissedIntroductionAppIdsRef, persistIntroductionSeen, introductionSeenKey, activeIntroductionRef, expandedCommandId, commandStagedArgsByCommandId, uiLocale, uiTerminology, identityRef, extraWindowInstancesRef }: Pick<ReturnType<typeof useFocusedProgramAndIntroductionState>, "utilityRegisterCell" | "bumpRegisterRevision" | "activeUtilityByWindowIdRef" | "dispatch" | "toolRegisterRef" | "activeToolIdRef" | "focusedToolMeasuresByToolId" | "activeWindowId" | "actionPaneExpandedByWindowId" | "actionPaneStagedArgsByKey" | "introductionStepIndex" | "introductionCompletedInteractions" | "tutorialPlaying" | "tutorialRecording" | "shellState" | "overlayDialog" | "isCurrentDialogOrigin" | "closeOwnedDialog" | "session" | "sessionRef" | "loadedPluginsRef" | "localInteraction" | "dropForRetiredInstance" | "loadedPlugins" | "scope" | "dismissedIntroductionAppIdsRef" | "persistIntroductionSeen" | "introductionSeenKey" | "activeIntroductionRef" | "expandedCommandId" | "commandStagedArgsByCommandId" | "uiLocale" | "uiTerminology" | "identityRef" | "extraWindowInstancesRef"> & { readonly core: ShellCoreAfterPluginLifecycle }) {
  //#endregion 🎯️InputLedger
  /** 🧰️ Dispatch + sync the ref immediately — `refreshUi` reads the ref before the next render, so a
   * bare `dispatch(SET_ACTIVE_UTILITY)` alone leaves the map stale and the gumball never appears. */
  const writeUtilityRegister = useCallback((windowId: string, utilityId: string | null, expectedGeneration: number | null) => {
    const write = utilityRegisterCell(windowId).write(utilityId, expectedGeneration);
    if (write.kind === "refused") return write;
    bumpRegisterRevision();
    activeUtilityByWindowIdRef.current = { ...activeUtilityByWindowIdRef.current, [windowId]: utilityId };
    dispatch({ type: "SET_ACTIVE_UTILITY", windowId, utilityId });
    const prior = leftoverWorldWindowOverlayV1(windowId);
    publishLeftoverWorldSelectionV1(
      {
        ids: prior?.ids ?? [],
        hoveredId: prior?.hoveredId ?? null,
        hoveredDomain: prior?.hoveredDomain,
        gumballActive: prior?.gumballActive ?? false,
        gumballAnchorId: prior?.gumballAnchorId ?? null,
        activeUtility: utilityId ?? "select",
        activeToolId: utilityId ? null : prior?.activeToolId ?? null,
      },
      { kind: "window", windowId },
    );
    return write;
  }, [utilityRegisterCell]);
  /** 🧰️ Clear every window's utility in the ref + store at once (tool/utility mutual exclusion) — a
   * shell-authoritative write, so every cleared window's register advances unconditionally. */
  const clearAllWindowUtilities = useCallback(() => {
    const next: Record<string, string | null> = { ...activeUtilityByWindowIdRef.current };
    let cleared = false;
    for (const windowId of Object.keys(next)) {
      if (next[windowId]) {
        next[windowId] = null;
        utilityRegisterCell(windowId).write(null, null);
        cleared = true;
        dispatch({ type: "SET_ACTIVE_UTILITY", windowId, utilityId: null });
      }
    }
    activeUtilityByWindowIdRef.current = next;
    if (cleared) bumpRegisterRevision();
  }, [utilityRegisterCell]);
  /** 🛠️ The tool register's compare-and-set twin of {@link writeUtilityRegister}: the ref `refreshUi`
   * reads and the store slice move together, and only when the write is admitted. */
  const writeToolRegister = useCallback((toolId: string | null, expectedGeneration: number | null) => {
    const write = toolRegisterRef.current.write(toolId, expectedGeneration);
    if (write.kind === "refused") return write;
    bumpRegisterRevision();
    activeToolIdRef.current = toolId;
    dispatch({ type: "SET_ACTIVE_TOOL", toolId });
    return write;
  }, []);
  /** 🛠️ The tool generation a shell-owned tool control stamps onto its `setActiveTool` press. */
  const toolGeneration = useCallback((): number => toolRegisterRef.current.read().generation, []);
  const toolMeasuresByToolIdRef = useRef(focusedToolMeasuresByToolId);
  toolMeasuresByToolIdRef.current = focusedToolMeasuresByToolId;
  const activeWindowIdRef = useRef(activeWindowId);
  activeWindowIdRef.current = activeWindowId;
  /** 🪟️ The layout the dock is actually rendering — the ONLY honest answer to "which windows does the
   * active mode mount", which is what scopes an app-wide chord to a window that exists right now.
   * Assigned from `effectiveModeLayout` further down (a `useMemo` declared after the keyboard loop). */
  const effectiveModeLayoutRef = useRef<WindowLayoutNode | null>(null);
  const actionPaneExpandedByWindowIdRef = useRef(actionPaneExpandedByWindowId);
  actionPaneExpandedByWindowIdRef.current = actionPaneExpandedByWindowId;
  const actionPaneStagedArgsByKeyRef = useRef(actionPaneStagedArgsByKey);
  actionPaneStagedArgsByKeyRef.current = actionPaneStagedArgsByKey;
  const introductionStepIndexRef = useRef(introductionStepIndex);
  introductionStepIndexRef.current = introductionStepIndex;
  const introductionCompletedInteractionsRef = useRef(introductionCompletedInteractions);
  introductionCompletedInteractionsRef.current = introductionCompletedInteractions;

  const startTutorialRef = useRef<(tutorialId: string) => void>(() => {});
  const stopTutorialRef = useRef<() => void>(() => {});
  const toggleTutorialRecordingRef = useRef<() => void>(() => {});
  /** 🧲️ True for the duration of any director/seek/converge-driven dispatch — `onAction`'s deviation
   * check below skips setting `deviated`/auto-pausing for anything stamped while this is true, mirroring
   * how the introduction mechanism's own interception distinguishes shell-originated from user-originated
   * activity. Never read during render, only inside event callbacks — a plain mutable ref is correct. */
  const tutorialDrivenRef = useRef(new TutorialDriveV1());
  const tutorialPlayingRef = useRef(tutorialPlaying);
  tutorialPlayingRef.current = tutorialPlaying;
  const tutorialRecordingRef = useRef(tutorialRecording);
  tutorialRecordingRef.current = tutorialRecording;
  /** ⏺️ Non-null while armed — mutated by `toggleTutorialRecording` (defined in the TutorialOrchestration block below), read/appended-to by `onAction`'s recorder tap right below. */
  const tutorialRecorderRef = useRef<TutorialRecorder | null>(null);
  const tutorialRecorderStartingRef = useRef(false);
  const shellStateRef = useRef(shellState);
  core.shellStateRef = shellStateRef;
  shellStateRef.current = shellState;
  useEffect(() => {
    if (overlayDialog !== null && !isCurrentDialogOrigin(overlayDialog.origin)) closeOwnedDialog(overlayDialog.openingId);
  }, [overlayDialog, session, closeOwnedDialog, isCurrentDialogOrigin]);
  const publishLocalInteraction = useCallback(async (plugin: PluginWasmHandle, instanceId: number): Promise<LocalInteractionState | null> => {
    const capture = await plugin.readLocalInteraction(instanceId);
    const active = sessionRef.current;
    const currentPlugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === active?.pluginId)?.handle;
    if (!active || active.instanceId !== instanceId || currentPlugin !== plugin) return null;
    localInteraction.observe({ ...capture.state, hover: localInteraction.get().hover });
    return capture.state;
  }, []);
  const observeLocalInteraction = useMemo(
    () => createLatestAsyncDispatcher(({ plugin, instanceId }: { readonly plugin: PluginWasmHandle; readonly instanceId: number }) => publishLocalInteraction(plugin, instanceId).catch((error) => {
      dropForRetiredInstance({ pluginId: plugin.pluginId, instanceId }, "local interaction observation", error);
    })),
    [dropForRetiredInstance, publishLocalInteraction],
  );
  useEffect(() => {
    if (!session) return;
    const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (plugin) observeLocalInteraction({ plugin, instanceId: session.instanceId });
  }, [loadedPlugins, observeLocalInteraction, session?.instanceId, session?.pluginId]);

  /** 🎓️ Ends the active introduction — records the answer for this shell session (see
   * {@link dismissedIntroductionAppIdsRef}), persists the device-local seen flag when configured, and on
   * successful completion (Done / last interaction) fires the tour-finale {@link celebrateAllElements}
   * stamp across every mounted UI element. Skip/escape passes `completed: false` and does not celebrate. */
  const dismissIntroduction = useCallback(
    (completed: boolean) => {
      if (completed && scope.rootRef.current) celebrateAllElements(CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
      const appId = sessionRef.current?.app.id;
      if (appId) dismissedIntroductionAppIdsRef.current.add(appId);
      dispatch({ type: "SET_INTRODUCTION_STEP", value: null });
      if (persistIntroductionSeen) writeStoredIntroductionSeen(scope.storage, introductionSeenKey);
    },
    [introductionSeenKey, persistIntroductionSeen],
  );

  /** 🎓️ Shared step-complete path: fires once every interaction-gated step's `interactions` are all done
   * (via `completeIntroductionInteraction` below), celebrating `introduce` on top of each interaction's
   * own celebration, then advances or finishes the tour. Finishing the last step celebrates every UI
   * element via {@link dismissIntroduction}(true) instead of only the introduce target. `celebrateOverride`
   * (threaded through from `completeIntroductionInteraction`) narrows this to the one element responsible
   * for the just-completed interaction — e.g. the specific 3D window pane that was orbited — instead of
   * every element aliased to the step's `introduce` kind (every open pane of that window kind). */
  const advanceIntroductionByDoing = useCallback(
    (celebrateOverride?: string) => {
      const stepIndex = introductionStepIndexRef.current;
      const introduction = activeIntroductionRef.current;
      if (stepIndex == null || !introduction) return;
      const step = introduction.steps[stepIndex];
      if (stepIndex >= introduction.steps.length - 1) {
        dismissIntroduction(true);
        return;
      }
      const celebrateId = celebrateOverride ?? step?.introduce;
      if (step && (step.interactions ?? []).length > 0 && celebrateId && scope.rootRef.current) celebrateElements(elementIdSelector(celebrateId), CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
      dispatch({ type: "SET_INTRODUCTION_STEP", value: stepIndex + 1 });
    },
    [dismissIntroduction],
  );

  /** ✅️ Completes the first not-yet-done interaction of the active step matching `matches` (respecting
   * `step.ordered` — only the next in-order interaction may complete), celebrates its target element, and
   * advances the step once every interaction is done. Mirrors the wgpu shell's
   * `chrome_tour_complete_interaction`. `celebrateOverride` — passed by callers that know exactly which
   * DOM element caused the completion (e.g. the gesture intercept knows the one window pane that was
   * actually orbited) — takes precedence over `interaction.celebrate ?? step.introduce`. Without it, a
   * window-kind `introduce`/`celebrate` id would celebrate every pane aliased to that kind, not just the
   * one that completed the interaction. */
  const completeIntroductionInteraction = useCallback(
    (matches: (interaction: IntroductionInteraction) => boolean, celebrateOverride?: string) => {
      const stepIndex = introductionStepIndexRef.current;
      const introduction = activeIntroductionRef.current;
      if (stepIndex == null || !introduction) return;
      const step = introduction.steps[stepIndex];
      if (!step || (step.interactions ?? []).length === 0) return;
      const completed = introductionCompletedInteractionsRef.current;
      const interactions = step.interactions ?? [];
      const index = interactions.findIndex((interaction, i) => !completed.includes(i) && matches(interaction));
      if (index < 0) return;
      if (step.ordered && index !== completed.length) return;
      const interaction=interactions[index];if(!interaction)return;
      const celebrateId = celebrateOverride ?? interaction.celebrate ?? step.introduce;
      if (celebrateId && scope.rootRef.current) celebrateElements(elementIdSelector(celebrateId), CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
      introductionCompletedInteractionsRef.current = [...completed, index];
      dispatch({ type: "COMPLETE_INTRODUCTION_INTERACTION", index });
      if (introductionCompletedInteractionsRef.current.length >= interactions.length) advanceIntroductionByDoing(celebrateOverride);
    },
    [advanceIntroductionByDoing],
  );
  const expandedCommandIdRef = useRef(expandedCommandId);
  expandedCommandIdRef.current = expandedCommandId;
  const commandStagedArgsByCommandIdRef = useRef(commandStagedArgsByCommandId);
  commandStagedArgsByCommandIdRef.current = commandStagedArgsByCommandId;
  const pendingDocumentOpeningPublicationRef = useRef<Readonly<{ receipt: DocumentOpeningReceiptV1; plugin: PluginWasmHandle; session: ActiveSession }> | null>(null);

  /** 🛠️ Overlays the mode-level host-owned `activeToolId` onto a view state at plugin-call time —
   * mirrors `injectActiveUtility` but is windowless (a tool is scoped to the active mode, not a window). */
  const injectActiveTool = useCallback((viewState: ViewModel): ViewModel => {
    const toolId = activeToolIdRef.current ?? undefined;
    return viewState.activeToolId === toolId ? viewState : { ...viewState, activeToolId: toolId };
  }, []);

  /** 🧰️ Overlays the active window's host-owned `activeUtilityId` (and the mode's `activeToolId`) onto a view state at plugin-call time. */
  const injectActiveUtility = useCallback((viewState: ViewModel, windowId?: string | null): ViewModel => {
    const key = windowId ?? activeWindowIdRef.current;
    const utilityId = key ? (activeUtilityByWindowIdRef.current[key] ?? undefined) : undefined;
    const withUtility = viewState.activeUtilityId === utilityId ? viewState : { ...viewState, activeUtilityId: utilityId };
    return injectActiveTool(withUtility);
  }, [injectActiveTool]);

  const resolvedTargetViewState = useCallback(
    (targetSession: ActiveSession) =>
      parseResolvedPluginViewState(
        injectActiveTool({
          ...targetSession.viewState,
          locale: uiLocale,
          terminology: uiTerminology,
          sessionIdentity: identityRef.current ? { userId: identityRef.current.userId, displayName: identityRef.current.displayName } : undefined,
          windowInstances: sessionWindowInstances(targetSession.app, extraWindowInstancesRef.current).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
          activeUtilityByWindowId: buildActiveUtilityByWindowId(activeUtilityByWindowIdRef.current),
          activeUtilityId: undefined,
        }),
      ),
    [injectActiveTool, uiLocale, uiTerminology],
  );
  return { resolvedTargetViewState, pendingDocumentOpeningPublicationRef, activeWindowIdRef, injectActiveTool, writeUtilityRegister, clearAllWindowUtilities, writeToolRegister, toolGeneration, toolMeasuresByToolIdRef, effectiveModeLayoutRef, actionPaneExpandedByWindowIdRef, actionPaneStagedArgsByKeyRef, startTutorialRef, stopTutorialRef, toggleTutorialRecordingRef, tutorialDrivenRef, tutorialPlayingRef, tutorialRecordingRef, tutorialRecorderRef, tutorialRecorderStartingRef, publishLocalInteraction, observeLocalInteraction, dismissIntroduction, completeIntroductionInteraction, expandedCommandIdRef, commandStagedArgsByCommandIdRef, injectActiveUtility };
}

/** 🪟 Posts the browser actor view state, runs the focus and input effects and builds the context menu request. */
export function useBrowserActorViewAndContextMenu({ resolvedTargetViewState, pendingDocumentOpeningPublicationRef, openDocumentSessionsRef, session, dispatch, panel, hostMode, backboneWorkerRef, spaceArtifactCreationOwnersRef, closeDocumentRef, browserActorUiByRuntimeKeyRef, backboneWorkerStreamsDetachRef, extensionFetchAbortRef, segmentedDownloadAbortRef, hubSessionPortRef, backgroundSpaceIndexSessionsRef, documentAttachmentLanesRef, directoryHomeOwnerRef, retireDirectoryHomeOwner, directoryHomeRetirementRef, sessionRef, loadedPluginsRef, spawnedAppsRef, contributorPassesRef, externalSlotNodesRef, contributorInstancesRef, scope, brand, activeAppTitle, primaryPluginId, installPlugin, registry, pluginArtifactRebuiltAtRef, reloadPlugin, pluginSource, loadedPlugins, uiLocaleRef, uiTerminologyRef, extraWindowInstancesRef, activeUtilityByWindowIdRef, activeWindowIdRef, activeToolIdRef, injectActiveTool }: Pick<ReturnType<typeof useUtilityAndToolRegisters>, "resolvedTargetViewState" | "pendingDocumentOpeningPublicationRef" | "activeWindowIdRef" | "injectActiveTool"> & Pick<ReturnType<typeof useFocusedProgramAndIntroductionState>, "openDocumentSessionsRef" | "session" | "dispatch" | "panel" | "hostMode" | "backboneWorkerRef" | "spaceArtifactCreationOwnersRef" | "closeDocumentRef" | "browserActorUiByRuntimeKeyRef" | "backboneWorkerStreamsDetachRef" | "extensionFetchAbortRef" | "segmentedDownloadAbortRef" | "hubSessionPortRef" | "backgroundSpaceIndexSessionsRef" | "documentAttachmentLanesRef" | "directoryHomeOwnerRef" | "retireDirectoryHomeOwner" | "directoryHomeRetirementRef" | "sessionRef" | "loadedPluginsRef" | "spawnedAppsRef" | "contributorPassesRef" | "externalSlotNodesRef" | "contributorInstancesRef" | "scope" | "brand" | "activeAppTitle" | "primaryPluginId" | "installPlugin" | "registry" | "pluginArtifactRebuiltAtRef" | "reloadPlugin" | "pluginSource" | "loadedPlugins" | "uiLocaleRef" | "uiTerminologyRef" | "extraWindowInstancesRef" | "activeUtilityByWindowIdRef" | "activeToolIdRef">) {
  const postBrowserActorViewState = useCallback(
    (worker: Worker, entry: OpenDocumentSession, targetSession: ActiveSession = entry.session) => {
      if (entry.scope === undefined) return;
      worker.postMessage({
        wire: encodeBackboneWorkerRequest({
          kind: "browser-actor-view-state",
          clientInstanceId: entry.clientInstanceId,
          scope: entry.scope,
          viewState: resolvedTargetViewState(targetSession),
        }),
      });
    },
    [resolvedTargetViewState],
  );

  useEffect(() => {
    const pending = pendingDocumentOpeningPublicationRef.current;
    pendingDocumentOpeningPublicationRef.current = null;
    const entry = pending === null ? undefined : openDocumentSessionsRef.current.get(pending.receipt.runtimeKey);
    const retainedRuntimeKey = pending !== null
      && entry?.clientInstanceId === pending.receipt.clientInstanceId
      && entry.plugin === pending.plugin
      && shellDialogSessionIsCurrentV1(pending.session, session)
      ? pending.receipt.runtimeKey
      : null;
    dispatch({ type: "SET_SYNC_BACKBONE_URI", value: retainedRuntimeKey === null ? null : `actor://${retainedRuntimeKey}` });
    dispatch({ type: "SET_SYNC_CARD_KIND", value: null });
  }, [panel?.activeSpawnedId, session, hostMode]);

  useEffect(() => {
    return () => {
      const worker = backboneWorkerRef.current;
      for (const owner of spaceArtifactCreationOwnersRef.current.values()) {
        worker?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-create-cancel", requestId: owner.requestId, spaceId: owner.spaceId }) });
      }
      spaceArtifactCreationOwnersRef.current.clear();
      for (const [runtimeKey, entry] of openDocumentSessionsRef.current) closeDocumentRef.current(runtimeKey, entry.clientInstanceId);
      for (const retained of browserActorUiByRuntimeKeyRef.current.values()) retained.actions.close("browser-actor-action: Shell unmounted");
      browserActorUiByRuntimeKeyRef.current.clear();
      worker?.terminate();
      backboneWorkerRef.current = null;
      backboneWorkerStreamsDetachRef.current?.();
      backboneWorkerStreamsDetachRef.current = null;
    };
  }, []);

  useEffect(() => {
    return () => {
      extensionFetchAbortRef.current.abort();
      segmentedDownloadAbortRef.current.abort(new Error("segmented-download-shell-unmounted"));
      hubSessionPortRef.current?.close();
      hubSessionPortRef.current = null;
      const retirements: Promise<unknown>[] = [];
      retirements.push(backgroundSpaceIndexSessionsRef.current.close());
      const destroyDetachedApp = async (plugin: PluginWasmHandle, instanceId: number) => {
        await documentAttachmentLanesRef.current.get(plugin)?.get(instanceId)?.drain();
        await plugin.destroyApp(instanceId);
      };
      const directoryHomeOwner = directoryHomeOwnerRef.current;
      directoryHomeOwnerRef.current = null;
      if (directoryHomeOwner) {
        const worker = backboneWorkerRef.current;
        retirements.push(retireDirectoryHomeOwner(directoryHomeOwner, worker));
      } else if (directoryHomeRetirementRef.current) {
        retirements.push(directoryHomeRetirementRef.current.promise);
      }
      const primary = sessionRef.current;
      if (primary) {
        const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === primary.pluginId)?.handle;
        if (plugin) retirements.push(destroyDetachedApp(plugin, primary.instanceId).catch(() => {}));
      }
      for (const spawned of spawnedAppsRef.current) {
        const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === spawned.pluginId)?.handle;
        if (plugin) retirements.push(destroyDetachedApp(plugin, spawned.instanceId).catch(() => {}));
      }
      contributorPassesRef.current.clear();
      externalSlotNodesRef.current.clear();
      retirements.push(retireContributorInstances(contributorInstancesRef.current));
      const handles = loadedPluginsRef.current.map((entry) => entry.handle);
      void Promise.allSettled(retirements).then(() => Promise.allSettled(handles.map(handle => handle.dispose()))).then(() => undefined);
    };
  }, [retireDirectoryHomeOwner]);

  useEffect(() => {
    if (!scope.ownsPage) return;
    if (brand) {
      document.title = brand.windowTitle;
    } else if (activeAppTitle) {
      document.title = activeAppTitle;
    }
  }, [activeAppTitle, brand, scope.ownsPage]);

  useEffect(() => {
    if (!primaryPluginId) return;
    if (loadedPluginsRef.current.some((entry) => entry.handle.pluginId === primaryPluginId)) return;
    void (async () => {
      const outcome = await installPlugin(primaryPluginId);
      if (outcome === "failed") {
        dispatch({ type: "SET_ERROR", value: shellLabel("ui.common.noPluginsLoaded") });
      }
    })();
  }, [primaryPluginId, installPlugin]);

  useEffect(() => {
    const registryIds = new Set(registry.map((entry) => entry.pluginId));
    let aborted = false;
    const pending: Array<{ readonly pluginId: string; readonly rebuiltAt: number | undefined }> = [];
    let activeWorkers = 0;
    const limit = pluginInstallConcurrency();

    const pump = (): void => {
      while (!aborted && activeWorkers < limit && pending.length > 0) {
        const next = pending.shift()!;
        const alreadyLoaded = loadedPluginsRef.current.some((entry) => entry.handle.pluginId === next.pluginId);
        const route = pluginAvailabilityRouteV1(alreadyLoaded, pluginArtifactRebuiltAtRef.current.get(next.pluginId), next.rebuiltAt);
        if (route === "drop") continue;
        activeWorkers += 1;
        void (route === "hot-swap" ? reloadPlugin(next.pluginId, next.rebuiltAt) : installPlugin(next.pluginId, next.rebuiltAt))
          .catch((error) => console.error("[os-shell] plugin install/reload failed", next.pluginId, error))
          .finally(() => {
            activeWorkers -= 1;
            pump();
          });
      }
    };

    const handlePluginAvailable = (pluginId: string, rebuiltAt: number | undefined) => {
      if (aborted || !registryIds.has(pluginId)) return;
      pending.push({ pluginId, rebuiltAt });
      pump();
    };
    const unsubscribe = pluginSource.subscribe((event: PluginSourceEvent) => {
      if (event.kind === "snapshot") {
        for (const plugin of event.plugins) handlePluginAvailable(plugin.pluginId, plugin.rebuiltAt);
        return;
      }
      handlePluginAvailable(event.pluginId, event.rebuiltAt);
    });
    return () => {
      aborted = true;
      pending.length = 0;
      unsubscribe();
    };
  }, [registry, pluginSource, installPlugin, reloadPlugin]);

  const requestContextMenu = useCallback(
    async (request: PluginContextMenuRequest): Promise<readonly ContextMenuItemSpec[]> => {
      if (!session) return [];
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
      if (!plugin?.contextMenu) return [];
      const baseViewState: ViewModel = {
        ...session.viewState,
        locale: uiLocaleRef.current,
        terminology: uiTerminologyRef.current,
        windowInstances: sessionWindowInstances(session.app, extraWindowInstancesRef.current).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
        activeUtilityByWindowId: buildActiveUtilityByWindowId(activeUtilityByWindowIdRef.current),
        focusedWindowId: activeWindowIdRef.current ?? undefined,
      };
      const viewState = request.windowInstanceId ? hostArmedViewContext(baseViewState, activeToolIdRef.current, request.windowInstanceId) : panelViewContext(injectActiveTool(baseViewState));
      if (!viewState) return [];
      return plugin.contextMenu(session.instanceId, request, viewState);
    },
    [injectActiveTool, loadedPlugins, session],
  );
  return { postBrowserActorViewState, requestContextMenu };
}
