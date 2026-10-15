/** 🎮️ Spawned apps, the focused program and its history and time travel, introductions and tutorial availability, the input ledger, utility and tool registers, intro callbacks, browser actor view state and the context menu request.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useFocusAndInput` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { shouldStartIntroduction } from "../../../../../../../../🔨️modules/🖱️ui/🎓️introduction/🟦️.ts";
import { shellDialogSessionIsCurrentV1 } from "../🗨️dialog-origin/🟦️.ts";
import { type DocumentOpeningReceiptV1 } from "../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { TutorialDriveV1 } from "../🗨️dialog-origin/🎥️tutorial/🟦️.ts";
// #region 🔌️Adapters
import {
  useCallback,
  useEffect,
  useMemo,
  useReducer,
  useRef,
  useSyncExternalStore
} from "react";
import {
  type ContextMenuItemSpec,
  FRAMEWORK_PANEL_TAB_TOOL_RUN_ID,
  type IntroductionInteraction,
  parseResolvedPluginViewState,
  type PluginContextMenuRequest,
  type PluginSourceEvent,
  retireContributorInstances,
  type TutorialDefinition,
  hostArmedViewContext,
  panelViewContext
} from "@semio-tech/framework";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework-os";
import { type LocalInteractionState } from "@semio-tech/framework-replication";
import {
  focusedProgramKeyV1,
  focusedProgramV1,
  programEntriesV1,
  programHistoryKeyV1,
  programHistoryProjectionV1,
  programHistoryProjectionsRetainedV1,
  spawnedBridgeCensusV1,
  spawnedProgramViewStateV1,
  spawnedProgramWindowInstancesV1,
  type FocusedProgramV1
} from "../🪟️spawned-program/🟦️.ts";
import {
  createHubIdentitySettleV1,
  createInputLedgerV1,
  createRefusalNoticeThrottleV1,
  createVersionedRegisterV1,
  inputAwaitsHubIdentityV1,
  type HubIdentityGateV1,
  type VersionedRegisterCellV1
} from "../🎯️input-ledger/🟦️.ts";
import {
  CELEBRATE_STAMP_DURATION_MS,
  celebrateAllElements,
  celebrateElements,
  elementIdSelector,
  readStoredIntroductionSeen,
  type WindowLayoutNode,
  writeStoredIntroductionSeen
} from "@semio-tech/ui-react";
import { resolveAppSurfaceSessionFactory } from "../../🪪️WasmSessionLoader/🟦️.tsx";
import {
  type ActiveSession,
  shouldPersistIntroductionSeen,
  shouldReplayIntroductionOnLoad,
  type SpawnedAppEntry
} from "../../🐚️Shell/🟦️.tsx";
// #endregion 🔌️Adapters
import type { ViewModel } from "../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { leftoverWorldWindowOverlayV1, publishLeftoverWorldSelectionV1 } from "../../🌐️World3dHost/🟦️.tsx";
import {
  appBreadcrumb,
  buildActiveUtilityByWindowId,
  EMPTY_SHELL_HISTORY_PROJECTION_V1,
  parsePanelState,
  pluginAvailabilityRouteV1,
  createLatestAsyncDispatcher,
  resolveAppBreadcrumb,
  resolveArtifactByAppId,
  frameworkLayoutDeclaredInstances,
  sessionWindowInstances,
  shellLabel
} from "../../🛠️ShellHelpers/🟦️.tsx";
import { timeTravelIndicatorTextV1, TimeTravelWindowIndicator } from "../../🛠️ShellHelpers/⏪️time-travel/🟦️.tsx";
import { type PluginWasmHandle } from "../../🔌️PluginRuntime/🟦️.tsx";
import type { BridgeInstanceRef } from "../../../../../🌉️mcp/🧵️bridge/🟦️.ts";
import { TutorialRecorder } from "../🎬️tutorial-recorder/🟦️.tsx";
import { pluginInstallConcurrency } from "../🔌️extension-runtime/🟦️.ts";
import type { ShellCoreAfterPluginLifecycle } from "../🚀️plugin-lifecycle/🟦️.ts";
import type { OpenDocumentSession } from "../🔒️identity-document-ports/🟦️.ts";

export type ShellCoreAfterFocusAndInput = ShellCoreAfterPluginLifecycle & ReturnType<typeof useFocusAndInput>;

//#region FrameworkOsShell
export function useFocusAndInput(core: ShellCoreAfterPluginLifecycle) {
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
  // 🐢️ Memoized on the raw `panelJson` string (not `session` object identity, which churns every
  // action) so a `session` refresh that leaves `panelJson` untouched reuses the same parsed `panel`
  // object — a prerequisite for any downstream `useMemo`/`React.memo` keyed on `panel` to bail.
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
  /** 🧾️ A spawned program's full history, read once when it takes the canvas. Its completions patch
   * the projection incrementally from then on; without this first snapshot its ledger would start
   * empty even for a document that already carries entries (a re-opened artifact). */
  useEffect(() => {
    if (!hostMode || focusedProgram === null || focusedProgram.spawnedId === null) return;
    refreshHistorySnapshot({ pluginId: focusedProgram.pluginId, instanceId: focusedProgram.instanceId });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [hostMode, focusedProgram?.pluginId, focusedProgram?.instanceId, focusedProgram?.spawnedId, refreshHistorySnapshot]);
  /** 📇️🪟️ Publishes the agent census — the live session PLUS every spawned program, the FOCUSED one
   * first ({@link spawnedBridgeCensusV1}). Keyed on the spawned ROSTER, never on `panel` identity (a
   * panel object churns on every action), on the loaded-plugin count (a spawned program's window
   * kinds come from its manifest and a roster entry can be written before its plugin finished
   * loading), and on the focused program, because focus is what entry 0 means. */
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
  /** 🧾️ Drops the projections of programs that are no longer open. Keyed on the spawned roster and
   * the session's instance, which is exactly when a program appears or disappears. */
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

  // 🎓️ A brand-owned introduction fully replaces the app's own (already localized, rendered verbatim);
  // its first-run-seen flag is brand-scoped so the branded tour plays even on a device that saw the
  // unbranded one. Brands with `replayIntroductionOnLoad` skip persistence and auto-start every load.
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

  // 🎓️ Auto-starts an app's introduction the first time it launches on this device (or every load when
  // the brand opts in); replaying stays available afterward via the shell-owned Introduce App command.
  // 🎥️ Never auto-starts while a tutorial is active (mutual exclusivity) — `activeTutorialId` is declared
  // just below (the TutorialOrchestration block's state resolution); read via `shellState.tutorial`
  // directly here rather than the not-yet-declared local to avoid a definition-order dependency.
  useEffect(() => {
    if (!session) return;
    if (typeof window !== "undefined" && window.self !== window.top) return;
    // Embedded multi-shell hosts (demonstrator grid) pass suppressAutoIntroduction while a pane is
    // backgrounded. That must both block auto-start AND tear down an already-running tour — otherwise the
    // unfocused shell keeps mounting UIIntroduction (veil/hotkeys/ghost cursor) and steals step chrome
    // from the focused pane.
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

  // 🎥️ Zero per-app work: any app/brand that declares `tutorials` gets shell support automatically.
  // Brand-owned tutorials are shown ALONGSIDE the app's own (never replacing them, unlike `introduction`).
  const activeTutorials = useMemo((): readonly TutorialDefinition[] => [...(brand?.tutorials ?? []), ...(session?.app.tutorials ?? [])], [brand?.tutorials, session?.app.tutorials]);
  /** ⏺️ The recorder is dev/studio-only — Vite always defines `import.meta.env.DEV`; guarded for non-Vite (e.g. `bun test`) evaluation. */
  const tutorialRecorderAvailable = useMemo(() => {
    try {
      return Boolean((import.meta as unknown as { readonly env?: { readonly DEV?: boolean } }).env?.DEV);
    } catch {
      return false;
    }
  }, []);

  // 🧰️ Refs so `refreshUi`/`onAction`/`applyHostEffects` can read the current host-owned active utility and
  // active window without re-creating those callbacks on every utility switch.
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
  /** ⏺️ Dev/studio-only probe surface (mirrors `__semioCrossingCensus` in `🔌️PluginRuntime`). */
  useEffect(() => {
    if (!tutorialRecorderAvailable) return;
    Object.defineProperty(globalThis, "__semioInputLedger", { configurable: true, get: () => ({ ...inputLedgerRef.current.census(), recent: inputLedgerRef.current.recent() }), set: () => {} });
    return () => {
      delete (globalThis as { __semioInputLedger?: unknown }).__semioInputLedger;
    };
  }, [tutorialRecorderAvailable]);
  //#endregion 🎯️InputLedger
  /** 🧰️ Dispatch + sync the ref immediately — `refreshUi` reads the ref before the next render, so a
   * bare `dispatch(SET_ACTIVE_UTILITY)` alone leaves the map stale and the gumball never appears. */
  // 🧰️ The ONE place one window's arm changes — the utility-bar action AND a program's own
  // `setActiveUtility` effect (an engagement verb's `brush`) both come through here. The pane's
  // leftover overlay is published from here too, so an arm the guest raised itself is not masked by
  // the pane's stale `select` overlay until some later refresh
  // (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B39).
  // 🔢️ It is a compare-and-set over the window's register cell: `expectedGeneration === null` is the
  // shell-authoritative unconditional write (a guest `setActiveUtility` effect, a tool activation
  // clearing every pane); a widget press passes the generation it rendered and is REFUSED when the
  // register moved under it — the `🎯️input-ledger` L3 replacement for the deleted 8 s echo-off window.
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

  // 🎥️ Forward-declared refs so `onAction` (defined below, before the full tutorial orchestration further
  // down this component) can shell-intercept `START_TUTORIAL_ACTION_ID`/`RECORD_TUTORIAL_ACTION_ID`
  // without a definition-order cycle — mirrors the `onActionRef` pattern used the other way around.
  // Populated by the TutorialOrchestration block's effect once the real callbacks exist.
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
      const celebrateId = celebrateOverride ?? interactions[index].celebrate ?? step.introduce;
      if (celebrateId && scope.rootRef.current) celebrateElements(elementIdSelector(celebrateId), CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
      introductionCompletedInteractionsRef.current = [...completed, index];
      dispatch({ type: "COMPLETE_INTRODUCTION_INTERACTION", index });
      if (introductionCompletedInteractionsRef.current.length >= interactions.length) advanceIntroductionByDoing(celebrateOverride);
    },
    [advanceIntroductionByDoing],
  );
  // 🎛️ So the command-category leaves' lazily-resolved tree content (built once per resolved-commands
  // change, not per keystroke — see `buildCommandCategoryTabs`) can read the latest expand/staged-arg
  // state without becoming a `defaultDock` memo dependency, which would otherwise persist-write the dock
  // skeleton on every keystroke while staging a command argument.
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
      // 🌐️ terra-web-shellhost (finding 4) — aborts any in-flight extension install fetch at real
      // component unmount; see `extensionFetchAbortRef`'s own doc a few hundred lines up.
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
      // 🪶️ Closes the previously-documented Wave-1 gap: studio-mode spawned apps (`panel.spawnedApps`)
      // and external-slot contributor instances (`contributorInstancesRef`) each hold a live plugin
      // instance too — leaving them running past shell unmount was pure leaked memory (see
      // REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT). Best-effort: an instance the guest already dropped,
      // or whose plugin already disposed, just rejects harmlessly via the same `.catch(() => {})`
      // pattern the primary session's own destroy already used above.
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
    // 🐚️ Only the page-owning shell may write the browser tab title — an embedded shell (e.g. one
    // demonstrator pane) sharing the page with others must not fight them over it.
    if (!scope.ownsPage) return;
    if (brand) {
      document.title = brand.windowTitle;
    } else if (activeAppTitle) {
      document.title = activeAppTitle;
    }
  }, [activeAppTitle, brand, scope.ownsPage]);

  // 🔌️ Boot gates on the primary/host plugin ONLY — every other registry entry streams in via the
  // subscription effect below as its build lands, instead of the whole shell waiting on all ~37 crates
  // (see `buildPluginsStreaming` in the dev runner). A primary that fails to load (timeout/error) is
  // still fatal, mirroring the old `noPluginsLoaded`/"host program missing landing app" boot failures.
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

  // 🔌️ Streams every registry entry in independently of boot: one connect-time `snapshot` (whatever's
  // already built, including a dev server that was already fully built before this shell mounted) plus
  // a `built` event per crate as `buildPluginsStreaming`/the folded-in watch loop finishes it. An event
  // for an already-loaded plugin routes to `reloadPlugin` (hot-swap) instead of `installPlugin`.
  // 🧮️ terra-web-shellhost (finding 2) — a cold boot's `snapshot` event can carry ~20 already-built
  // plugins at once; before this packet each one fired an unbounded, uncancellable
  // `void installPlugin(...)`, all instantiating real wasm modules through the shard pool
  // simultaneously. `pending`/`pump` below is a plain worker-pool queue (mirrors `PluginRuntime`'s own
  // `runBounded` shape): at most `pluginInstallConcurrency()` installs/reloads run at once, extras wait
  // their turn, and `installPlugin`/`reloadPlugin` are still called with a FRESH `alreadyLoaded` read
  // at dispatch time (not at enqueue time) so a plugin that finished loading while queued still routes
  // correctly. `aborted` stops handing out new work on unmount — an install already in flight (already
  // called) settles on its own, same "stop starting, let in-flight finish" contract
  // `loadPluginModulesInDependencyOrder`'s own `signal` documents.
  // 🔁️ An availability event for an artifact this shell ALREADY runs is dropped unless its `rebuiltAt`
  // is strictly newer (`pluginAvailabilityRouteV1`). A `PluginSource` streams availability, not
  // commands: `subscribe` opens a fresh `EventSource` and the dev endpoint answers every connect with a
  // full `snapshot`, so the same builds arrive again on every reconnect — and answering a replay with a
  // hot-swap destroys the session-owning plugin's live instance and its document
  // (`actor-activation.revoked`, then `no channel for instance N`; ticket 26/09/02 wave B40 §1).
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

  return {
    panel, activeSpawnedEntry, spawnedIds, spawnedRosterKey, activeSpawnedApp, focusedProgram, boardSessionFactory,
    focusedApp, focusedSpawnedId, focusedPanelBodyStores, toolRunPanelNode, focusedToolMeasuresByToolId, focusedModeId,
    focusedHistoryV1, historyCanUndo, historyCheckpointId, focusedTimeTravel, focusedReprojection, focusedHistoryEntries,
    timeTravelIndicator, focusedWindowInstancesRef, activeIntroduction, activeTutorials, tutorialRecorderAvailable,
    activeToolIdRef, inputLedgerRef, refusalNoticeThrottleRef, hubIdentityGateRef, hubIdentitySettleRef,
    registerRevision, utilityGenerationFor, writeUtilityRegister, clearAllWindowUtilities, writeToolRegister,
    toolGeneration, toolMeasuresByToolIdRef, activeWindowIdRef, effectiveModeLayoutRef, actionPaneExpandedByWindowIdRef,
    actionPaneStagedArgsByKeyRef, startTutorialRef, stopTutorialRef, toggleTutorialRecordingRef, tutorialDrivenRef,
    tutorialPlayingRef, tutorialRecordingRef, tutorialRecorderRef, tutorialRecorderStartingRef, publishLocalInteraction,
    observeLocalInteraction, dismissIntroduction, completeIntroductionInteraction, expandedCommandIdRef,
    commandStagedArgsByCommandIdRef, pendingDocumentOpeningPublicationRef, injectActiveTool, injectActiveUtility,
    resolvedTargetViewState, postBrowserActorViewState, requestContextMenu
  };
}
//#endregion FrameworkOsShell
