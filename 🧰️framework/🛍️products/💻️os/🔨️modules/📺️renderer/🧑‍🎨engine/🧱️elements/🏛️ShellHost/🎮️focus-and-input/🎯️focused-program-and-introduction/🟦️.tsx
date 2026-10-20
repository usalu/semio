/** 🎯️ The focused program, history, time travel, introduction and tutorial state of the focus and input hook. */
import { shouldStartIntroduction } from "@semio-tech/ui/introduction";
import { useCallback, useEffect, useMemo, useReducer, useRef, useSyncExternalStore } from "react";
import { FRAMEWORK_PANEL_TAB_TOOL_RUN_ID, type TutorialDefinition } from "@semio-tech/framework";
import { focusedProgramKeyV1, focusedProgramV1, programEntriesV1, programHistoryKeyV1, programHistoryProjectionV1, programHistoryProjectionsRetainedV1, spawnedBridgeCensusV1, spawnedProgramViewStateV1, spawnedProgramWindowInstancesV1, type FocusedProgramV1 } from "../../🪟️spawned-program/🟦️.ts";
import { createHubIdentitySettleV1, createInputLedgerV1, createRefusalNoticeThrottleV1, createVersionedRegisterV1, inputAwaitsHubIdentityV1, type HubIdentityGateV1, type VersionedRegisterCellV1 } from "../../🎯️input-ledger/🟦️.ts";
import { readStoredIntroductionSeen } from "@semio-tech/ui-react";
import { resolveAppSurfaceSessionFactory } from "../../../🪪️WasmSessionLoader/🟦️.tsx";
import { shouldPersistIntroductionSeen, shouldReplayIntroductionOnLoad } from "../../../🐚️Shell/🔒️boot-and-locks/🟦️.ts";
import type { ViewModel } from "../../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { appBreadcrumb, resolveAppBreadcrumb, resolveArtifactByAppId, frameworkLayoutDeclaredInstances } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { sessionWindowInstances } from "../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { EMPTY_SHELL_HISTORY_PROJECTION_V1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { parsePanelState, type SpawnedAppEntry } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { timeTravelIndicatorTextV1, TimeTravelWindowIndicator } from "../../../🛠️ShellHelpers/⏪️time-travel/🟦️.tsx";
import type { BridgeInstanceRef } from "@semio-tech/framework/mcp/bridge";
import type { ShellCoreAfterPluginLifecycle } from "../../🚀️plugin-lifecycle/🟦️.ts";

/**
 * 🧭 Binds the focused program, its history and time travel indicator and the introduction and tutorial state with their effects.
 *
 * 🧾️ A spawned program's full history, read once when it takes the canvas. Its completions patch
 * the projection incrementally from then on; without this first snapshot its ledger would start
 * empty even for a document that already carries entries (a re-opened artifact).
 *
 * 📇️🪟️ Publishes the agent census — the live session PLUS every spawned program, the FOCUSED one
 * first ({@link spawnedBridgeCensusV1}). Keyed on the spawned ROSTER, never on `panel` identity (a
 * panel object churns on every action), on the loaded-plugin count (a spawned program's window
 * kinds come from its manifest and a roster entry can be written before its plugin finished
 * loading), and on the focused program, because focus is what entry 0 means.
 *
 * ⏺️ Dev/studio-only probe surface (mirrors `__semioCrossingCensus` in `🔌️PluginRuntime`).
 */
export function useFocusedProgramAndIntroductionState({ core }: { readonly core: ShellCoreAfterPluginLifecycle }) {
  const {
    surfaceSessionFactories, brand, suppressAutoIntroduction, scope, hostMode, shellState, dispatch, localInteraction,
    historyStore, historyOrderByProgramRef, loadedPlugins, session, uiLocale, uiTerminology, refreshHistorySnapshot,
    toolMeasuresByToolId, panelBodyStoreByKey, actionPaneExpandedByWindowId, actionPaneStagedArgsByKey,
    activeUtilityByWindowId, activeToolId, expandedCommandId, commandStagedArgsByCommandId, activeWindowId,
    extraWindowInstances, introductionStepIndex, introductionCompletedInteractions, overlayDialog, tutorialPlaying,
    tutorialRecording, contributorInstancesRef, contributorPassesRef, externalSlotNodesRef, extraWindowInstancesRef,
    dropForRetiredInstance, sessionRef, builtNodeStoreCacheRef, backboneWorkerRef, backboneWorkerStreamsDetachRef,
    setAgentBridgeCensus, identity, identityRef, identityOffline, hubSessionRefused, hubSessionPortRef,
    hubSessionCapability, directoryHomeOwnerRef, directoryHomeRetirementRef, uiLocaleRef, uiTerminologyRef,
    retireDirectoryHomeOwner, backgroundSpaceIndexSessionsRef, closeDocumentRef, documentAttachmentLanesRef,
    spaceArtifactCreationOwnersRef, extensionFetchAbortRef, segmentedDownloadAbortRef, openDocumentSessionsRef,
    isCurrentDialogOrigin, closeOwnedDialog, browserActorUiByRuntimeKeyRef, loadedPluginsRef, pluginArtifactRebuiltAtRef,
    registry, primaryPluginId, pluginSource, installPlugin, reloadPlugin
  } = core;
  const panel = useMemo(() => (session ? parsePanelState(session.viewState) : null), [session?.viewState.panelJson]);
  /** 🐚️ Mirrors `panel?.spawnedApps` for the unmount-cleanup effect below — same rationale as
   * `loadedPluginsRef`: needs the latest value at teardown time without depending on it. */
  const spawnedAppsRef = useRef<readonly SpawnedAppEntry[]>([]);
  core.spawnedAppsRef = spawnedAppsRef;
  spawnedAppsRef.current = panel?.spawnedApps ?? [];
  const activeSpawnedEntry = panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
  const activeAppTitle = appBreadcrumb(activeSpawnedEntry ? resolveArtifactByAppId(loadedPlugins, activeSpawnedEntry.appId, activeSpawnedEntry.breadcrumb, uiTerminology) : session ? resolveAppBreadcrumb(session.app, uiTerminology) : []);
  /** 🪟️ Every live spawned instance id, for the host↔guest window-id translation. A ref too, because
   * `onAction` and the keydown lane read it at dispatch time from stable callbacks. */
  const spawnedIds = useMemo(() => (panel?.spawnedApps ?? []).map((entry) => entry.id), [panel?.spawnedApps]);
  const spawnedIdsRef = useRef<readonly string[]>([]);
  core.spawnedIdsRef = spawnedIdsRef;
  spawnedIdsRef.current = spawnedIds;
  /** 🏁️ Identity of the live spawned ROSTER — what a per-instance subscription must re-run on. */
  const spawnedRosterKey = (panel?.spawnedApps ?? []).map((entry) => `${entry.id}:${entry.pluginId}:${entry.instanceId}`).join("|");
  /** 🪟️ The focused spawned instance's own `AppDefinition` — the manifest the canvas, the Display
   * menu, the Actions rail and the chord loop must read once a foreign program owns the canvas. */
  const activeSpawnedApp = useMemo(
    () => (activeSpawnedEntry ? loadedPlugins.find((entry) => entry.handle.pluginId === activeSpawnedEntry.pluginId)?.manifest.apps.find((candidate) => candidate.id === activeSpawnedEntry.appId) : undefined),
    [activeSpawnedEntry, loadedPlugins],
  );
  /** 🪟️ Which program owns the canvas right now (`🪟️spawned-program`). A spawned program is a member
   * of this session, not a picture inside the host app's panel: from here on every window-scoped
   * decision — the layout seed, the Display menu, the active window, per-window utilities, the
   * Actions rail and the chord loop — is taken against THIS app, not `session.app`. */
  const focusedProgram = useMemo(
    (): FocusedProgramV1 | null =>
      focusedProgramV1(
        session ? { pluginId: session.pluginId, instanceId: session.instanceId, appId: session.app.id } : null,
        hostMode && activeSpawnedEntry && activeSpawnedApp ? activeSpawnedEntry : null,
      ),
    [session, hostMode, activeSpawnedEntry, activeSpawnedApp],
  );
  const boardSessionFactory = useMemo(() => resolveAppSurfaceSessionFactory(surfaceSessionFactories ?? [], focusedProgram), [surfaceSessionFactories, focusedProgram?.pluginId, focusedProgram?.appId, focusedProgram?.instanceId]);
  const focusedApp = hostMode && activeSpawnedApp ? activeSpawnedApp : (session?.app ?? null);
  const focusedSpawnedId = focusedProgram?.spawnedId ?? null;
  const focusedPanelBodyStores = useMemo(() => programEntriesV1(panelBodyStoreByKey, focusedSpawnedId, spawnedIds), [panelBodyStoreByKey, focusedSpawnedId, spawnedIds]);
  /** ⏯️ The focused program's ToolRun panel body — the task feed and the reveal follow THIS store, not every panel body. */
  const toolRunPanelStore = focusedPanelBodyStores[FRAMEWORK_PANEL_TAB_TOOL_RUN_ID] ?? null;
  const subscribeToolRunPanel = useCallback((listener: () => void) => toolRunPanelStore?.subscribeRevision(listener) ?? (() => undefined), [toolRunPanelStore]);
  const toolRunPanelNode = useSyncExternalStore(subscribeToolRunPanel, () => (toolRunPanelStore === null ? null : builtNodeStoreCacheRef.current.nodeOf(toolRunPanelStore)), () => null) ?? undefined;
  const focusedToolMeasuresByToolId = useMemo(() => programEntriesV1(toolMeasuresByToolId, focusedSpawnedId, spawnedIds), [toolMeasuresByToolId, focusedSpawnedId, spawnedIds]);
  const focusedModeId = focusedSpawnedId !== null && focusedApp !== null ? (spawnedProgramViewStateV1<ViewModel>({}, focusedApp).activeModeId ?? focusedApp.id) : (session?.viewState.activeModeId ?? session?.app.modes[0]?.id ?? session?.app.id ?? "");
  /** 🧾️ The history the History panel, the undo/redo controls, the uncommitted-edit count and
   * `data-history-json` all read: the FOCUSED program's own, because that is the document the user
   * is editing. While a spawned editor owns the canvas the session's ledger is still kept — it is
   * just not what is shown — so coming back to it shows its own rows, not the editor's. */
  const focusedHistoryKey = programHistoryKeyV1(focusedProgram);
  const focusedHistoryV1 = useCallback(() => programHistoryProjectionV1(historyStore.get(), focusedHistoryKey, EMPTY_SHELL_HISTORY_PROJECTION_V1), [historyStore, focusedHistoryKey]);
  const historyCanUndo = useSyncExternalStore(historyStore.subscribe, () => focusedHistoryV1().canUndo, () => false);
  const historyCheckpointId = useSyncExternalStore(historyStore.subscribe, () => focusedHistoryV1().currentCheckpointId, () => undefined);
  /** ⏪️ The focused program's live history-edit session, `null` while none is open: the band shows it whichever panel
   * is open, and every window of that program wears the indicator. The indicator reads only the stage and its own
   * text, so a replay progress step re-renders the band and never the window descriptors. */
  const focusedTimeTravel = useSyncExternalStore(historyStore.subscribe, () => focusedHistoryV1().timeTravel, () => null);
  /** 📡️ The focused program's history change replaying before adoption (remote, its own step, a document load), `null` while
   * none waits: the shell announces it outside the History panel whichever panel is open (audit W1E-3). */
  const focusedReprojection = useSyncExternalStore(historyStore.subscribe, () => focusedHistoryV1().reprojection, () => null);
  const focusedHistoryEntries = useSyncExternalStore(historyStore.subscribe, () => focusedHistoryV1().entries, () => EMPTY_SHELL_HISTORY_PROJECTION_V1.entries);
  const timeTravelIndicatorStage = focusedTimeTravel?.stage ?? null;
  const timeTravelIndicatorText = focusedTimeTravel === null ? null : timeTravelIndicatorTextV1(focusedTimeTravel, { terminology: uiTerminology, locale: uiLocale });
  const timeTravelIndicator = useMemo(() => (timeTravelIndicatorStage === null || timeTravelIndicatorText === null ? undefined : <TimeTravelWindowIndicator stage={timeTravelIndicatorStage} description={timeTravelIndicatorText} />), [timeTravelIndicatorStage, timeTravelIndicatorText]);
  useEffect(() => {
    if (!hostMode || focusedProgram === null || focusedProgram.spawnedId === null) return;
    refreshHistorySnapshot({ pluginId: focusedProgram.pluginId, instanceId: focusedProgram.instanceId });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [hostMode, focusedProgram?.pluginId, focusedProgram?.instanceId, focusedProgram?.spawnedId, refreshHistorySnapshot]);
  useEffect(() => {
    const spawned = (spawnedAppsRef.current ?? []).flatMap((entry) => {
      const app = loadedPluginsRef.current.find((candidate) => candidate.handle.pluginId === entry.pluginId)?.manifest.apps.find((candidate) => candidate.id === entry.appId);
      return app ? [{ id: entry.id, pluginId: entry.pluginId, appId: app.id, instanceId: entry.instanceId, windowIds: spawnedProgramWindowInstancesV1(entry.id, app.windowKinds.map((kind) => ({ id: kind.id, bodyKey: kind.bodyKey })), frameworkLayoutDeclaredInstances(app.defaultLayout, app.windowKinds)).map((instance) => instance.id) }] : [];
    });
    const census = spawnedBridgeCensusV1(
      session ? { pluginId: session.pluginId, appId: session.app.id, instanceId: session.instanceId, windowIds: sessionWindowInstances(session.app, extraWindowInstancesRef.current).map((instance) => instance.id) } : null,
      spawned,
      focusedProgram,
    );
    const next: BridgeInstanceRef[] = [...census].sort((left, right) => Number(right.focused) - Number(left.focused)).map(({ focused: _focused, ...ref }) => ({ ...ref, windowIds: [...ref.windowIds] }));
    setAgentBridgeCensus((current) => (JSON.stringify(current) === JSON.stringify(next) ? current : next));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [spawnedRosterKey, loadedPlugins.length, session?.pluginId, session?.app.id, session?.instanceId, focusedProgramKeyV1(focusedProgram)]);
  useEffect(() => {
    const live = [programHistoryKeyV1(session), ...spawnedAppsRef.current.map((entry) => programHistoryKeyV1(entry))];
    historyStore.update((projections) => programHistoryProjectionsRetainedV1(projections, live));
    for (const key of [...historyOrderByProgramRef.current.keys()]) if (!live.includes(key)) historyOrderByProgramRef.current.delete(key);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [spawnedRosterKey, session?.pluginId, session?.instanceId]);
  /** 🪟️ The focused program's live window instances, in the ONE shape every consumer reads: a spawned
   * program contributes one per declared window kind under its own `${spawnedId}::` namespace; the
   * session's own app contributes its base kinds plus the user's split/extra instances. */
  const focusedWindowInstances = useMemo(
    () =>
      focusedApp === null
        ? []
        : focusedSpawnedId !== null
          ? spawnedProgramWindowInstancesV1(focusedSpawnedId, focusedApp.windowKinds, frameworkLayoutDeclaredInstances(focusedApp.defaultLayout, focusedApp.windowKinds))
          : sessionWindowInstances(focusedApp, extraWindowInstances),
    [focusedApp, focusedSpawnedId, extraWindowInstances],
  );
  const focusedWindowInstancesRef = useRef<readonly { readonly id: string; readonly bodyKey: string; readonly windowKindId: string }[]>([]);
  focusedWindowInstancesRef.current = focusedWindowInstances;

  useEffect(() => {
    sessionRef.current = session;
  }, [session]);

  const activeIntroduction = brand?.introduction ?? session?.app.introduction;
  const introductionSeenKey = session ? (brand ? `${brand.id}:${session.app.id}` : session.app.id) : "";
  const replayIntroductionOnLoad = shouldReplayIntroductionOnLoad(brand);
  const persistIntroductionSeen = shouldPersistIntroductionSeen(brand);
  const activeIntroductionRef = useRef(activeIntroduction);
  activeIntroductionRef.current = activeIntroduction;
  /** 🎓️ App ids whose tour this shell session has already offered AND had answered (Skip or Done) — the
   * session-durable half of {@link shouldStartIntroduction}, see its own reasoning for why the
   * device-local seen flag alone cannot carry this. Lives for the shell mount, exactly the lifetime of
   * "this session", so a `replayIntroductionOnLoad` brand still replays on the next load. */
  const dismissedIntroductionAppIdsRef = useRef<Set<string>>(new Set());

  useEffect(() => {
    if (!session) return;
    if (typeof window !== "undefined" && window.self !== window.top) return;
    if (suppressAutoIntroduction) {
      dispatch({ type: "SET_INTRODUCTION_STEP", value: null });
      return;
    }
    const armable = shouldStartIntroduction({
      appId: session.app.id,
      hasIntroduction: activeIntroduction != null,
      tutorialActive: shellState.tutorial.activeTutorialId != null,
      suppressed: suppressAutoIntroduction,
      replayOnLoad: replayIntroductionOnLoad,
      seenOnDevice: readStoredIntroductionSeen(scope.storage, introductionSeenKey),
      dismissedInSession: dismissedIntroductionAppIdsRef.current.has(session.app.id),
    });
    if (!armable) return;
    dispatch({ type: "AUTO_START_INTRODUCTION", key: introductionSeenKey });
  }, [session?.app.id, activeIntroduction, introductionSeenKey, replayIntroductionOnLoad, shellState.tutorial.activeTutorialId, suppressAutoIntroduction]);

  const activeTutorials = useMemo((): readonly TutorialDefinition[] => [...(brand?.tutorials ?? []), ...(session?.app.tutorials ?? [])], [brand?.tutorials, session?.app.tutorials]);
  /** ⏺️ The recorder is dev/studio-only — Vite always defines `import.meta.env.DEV`; guarded for non-Vite (e.g. `bun test`) evaluation. */
  const tutorialRecorderAvailable = useMemo(() => {
    try {
      return Boolean((import.meta as unknown as { readonly env?: { readonly DEV?: boolean } }).env?.DEV);
    } catch {
      return false;
    }
  }, []);

  const activeUtilityByWindowIdRef = useRef(activeUtilityByWindowId);
  core.activeUtilityByWindowIdRef = activeUtilityByWindowIdRef;
  activeUtilityByWindowIdRef.current = activeUtilityByWindowId;
  const activeToolIdRef = useRef(activeToolId);
  activeToolIdRef.current = activeToolId;
  //#region 🎯️InputLedger
  /** 🎯️ The Input Causality Ledger (ticket 26/09/16/INPUT-CAUSALITY-LEDGER): `onAction` is its one funnel —
   * every input becomes an entry and reaches `applied | refused(reason) | superseded`, never a silent
   * `return`. `refusalNoticeThrottleRef` folds a burst of one refusal reason into one notice (§G). */
  const inputLedgerRef = useRef(createInputLedgerV1());
  const refusalNoticeThrottleRef = useRef(createRefusalNoticeThrottleV1());
  /** 🔐️ A user input that arrives while a just-granted hub session is still being resolved to a human waits for it
   * (bounded, {@link inputAwaitsHubIdentityV1}) instead of reaching the guest identity-less and being refused. */
  const hubIdentityGateRef = useRef<HubIdentityGateV1>({ capabilityHeld: false, identityKnown: false, refused: false, offline: false });
  hubIdentityGateRef.current = { capabilityHeld: hubSessionCapability !== null, identityKnown: identity !== null, refused: hubSessionRefused, offline: identityOffline };
  const hubIdentitySettleRef = useRef(createHubIdentitySettleV1());
  useEffect(() => {
    if (!inputAwaitsHubIdentityV1(hubIdentityGateRef.current)) hubIdentitySettleRef.current.settle();
  }, [identity, hubSessionCapability, hubSessionRefused, identityOffline]);
  /** 🔢️ L3 versioned registers — the host-owned interaction state a widget renders AND stamps back
   * (`expectedGeneration`) so a stale press is refused by compare-and-set, never by a wall clock. The
   * store slices (`activeUtilityByWindowId`, `activeToolId`) stay the RENDERED value; these cells carry
   * the generation. One cell per window instance, minted lazily off the current value. */
  const utilityGenerationByWindowRef = useRef(new Map<string, VersionedRegisterCellV1<string | null>>());
  const toolRegisterRef = useRef(createVersionedRegisterV1<string | null>(activeToolId ?? null));
  /** 🔢️ Bumped after every APPLIED register write — a write that leaves the VALUE unchanged (the picker's
   * `utilityId: ""` resync while nothing is armed) moves the generation without moving the store, and the
   * widgets that stamp `expectedGeneration` must still re-render to read the new one. */
  const [registerRevision, bumpRegisterRevision] = useReducer((revision: number) => revision + 1, 0);
  const utilityRegisterCell = useCallback((windowId: string): VersionedRegisterCellV1<string | null> => {
    const cells = utilityGenerationByWindowRef.current;
    let cell = cells.get(windowId);
    if (cell === undefined) {
      cell = createVersionedRegisterV1<string | null>(activeUtilityByWindowIdRef.current[windowId] ?? null);
      cells.set(windowId, cell);
    }
    return cell;
  }, []);
  /** 🔢️ The generation a utility bar / method picker renders for `windowId` (read at render time; the
   * `registerRevision` dependency is what re-runs the readers after a write). */
  const utilityGenerationFor = useCallback((windowId: string): number => utilityRegisterCell(windowId).read().generation, [utilityRegisterCell]);
  useEffect(() => {
    if (!tutorialRecorderAvailable) return;
    Object.defineProperty(globalThis, "__semioInputLedger", { configurable: true, get: () => ({ ...inputLedgerRef.current.census(), recent: inputLedgerRef.current.recent() }), set: () => {} });
    return () => {
      delete (globalThis as { __semioInputLedger?: unknown }).__semioInputLedger;
    };
  }, [tutorialRecorderAvailable]);
  return { utilityRegisterCell, bumpRegisterRevision, activeUtilityByWindowIdRef, dispatch, toolRegisterRef, activeToolIdRef, focusedToolMeasuresByToolId, activeWindowId, actionPaneExpandedByWindowId, actionPaneStagedArgsByKey, introductionStepIndex, introductionCompletedInteractions, tutorialPlaying, tutorialRecording, shellState, overlayDialog, isCurrentDialogOrigin, closeOwnedDialog, session, sessionRef, loadedPluginsRef, localInteraction, dropForRetiredInstance, loadedPlugins, scope, dismissedIntroductionAppIdsRef, persistIntroductionSeen, introductionSeenKey, activeIntroductionRef, expandedCommandId, commandStagedArgsByCommandId, uiLocale, uiTerminology, identityRef, extraWindowInstancesRef, openDocumentSessionsRef, panel, hostMode, backboneWorkerRef, spaceArtifactCreationOwnersRef, closeDocumentRef, browserActorUiByRuntimeKeyRef, backboneWorkerStreamsDetachRef, extensionFetchAbortRef, segmentedDownloadAbortRef, hubSessionPortRef, backgroundSpaceIndexSessionsRef, documentAttachmentLanesRef, directoryHomeOwnerRef, retireDirectoryHomeOwner, directoryHomeRetirementRef, spawnedAppsRef, contributorPassesRef, externalSlotNodesRef, contributorInstancesRef, brand, activeAppTitle, primaryPluginId, installPlugin, registry, pluginArtifactRebuiltAtRef, reloadPlugin, pluginSource, uiLocaleRef, uiTerminologyRef, activeSpawnedEntry, spawnedIds, spawnedRosterKey, activeSpawnedApp, focusedProgram, boardSessionFactory, focusedApp, focusedSpawnedId, focusedPanelBodyStores, toolRunPanelNode, focusedModeId, focusedHistoryV1, historyCanUndo, historyCheckpointId, focusedTimeTravel, focusedReprojection, focusedHistoryEntries, timeTravelIndicator, focusedWindowInstancesRef, activeIntroduction, activeTutorials, tutorialRecorderAvailable, inputLedgerRef, refusalNoticeThrottleRef, hubIdentityGateRef, hubIdentitySettleRef, registerRevision, utilityGenerationFor };
}
