/** 🛫️ Spawned ui refresh, session keys, layout seeding before a spawn, space panel updates, instance retirement, plugin app switching, service operations and direct browser actor dispatch.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useSpawnedSession` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { DOCUMENT_SERVICE_TOPIC_V1 } from "@semio-tech/framework/inference/service";
import { admitDocumentServiceWireDeclarationV1 } from "@semio-tech/framework/inference/service/io";
import { prepareDocumentSurfaceV1 } from "../🔀️surface-switch/📄️document/🟦️.ts";
import { createAdmittedShellInstanceV1, shellDialogSessionIsCurrentV1 } from "../🗨️dialog-origin/🟦️.ts";
import { type DocumentOpeningReceiptV1 } from "../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { directorySessionAuthorityIsCurrentV1 } from "@semio-tech/framework/directory/session-refresh";
import { useCallback, useEffect, useRef } from "react";
// #region 🔌️Adapters
import {
  type ActionInvocation,
  type AppDefinition,
  type CommandInvocation,
  type Effect,
  type HistoryPatch,
  panelTabKindId,
  pendingWindowUiNode,
  type BuiltNode,
  type UiDirtyScope,
  type WindowEngagement,
  type WindowMeasure
} from "@semio-tech/framework";
import { decodePackWire } from "@semio-tech/framework/packs";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { quiesceSessionWorkV1, shellHumanChangeRecoveryV1, surfaceSwitchBusyTextV1, type ShellHumanV1 } from "../🔀️surface-switch/🟦️.ts";
import {
  focusedProgramKeyV1,
  programKeyedEntriesV1,
  withProgramEntriesV1,
  spawnedProgramViewStateV1,
  guestActiveUtilityByWindowIdV1,
  guestWindowIdV1,
  renameLayoutWindowIdsV1,
  spawnedGuestWindowInstancesV1,
  spawnedLayoutRenameV1,
  spawnedWindowInstanceIdV1
} from "../🪟️spawned-program/🟦️.ts";
import { type WindowLayoutNode } from "@semio-tech/ui-react";
import { type SpacePanelState, type SpawnedAppEntry } from "../../🐚️Shell/🟦️.tsx";
import { type ActiveSession, sessionProgramsV1, type SpaceProgramEntry } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { ViewModel } from "../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import {
  applyFrameworkLayoutSeed,
  flattenPanelTabLeaves,
  resolveCanvasBodyKey,
  resolveFrameworkLayoutSeed,
  frameworkLayoutDeclaredInstances
} from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import {
  applyUiRefreshResponseToCache,
  browserActorWindowConfigDispatchUiScopeV1,
  buildUiRefreshRequest,
  mergeRecordPreservingIdentity
} from "../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import {
  buildSpacePanelState,
  panelJsonFromState,
  parsePanelState,
  studioPanelFocusingSpawned
} from "../../🛠️ShellHelpers/📌️panel/🟦️.ts";
// #endregion 🔌️Adapters
import { isPluginInstanceRetiredV1, type PluginWasmHandle } from "../../🔌️PluginRuntime/🟦️.tsx";
import { BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION } from "@semio-tech/framework/browser-bundle/action-handoff";
import { publishBrowserActorHostEffectsV1 } from "@semio-tech/framework/browser-bundle/action-handoff/publication";
import { InferencePortOpeningMailboxV1 } from "@semio-tech/framework/inference/opening";
import { liveInstanceWindowFaultV1, windowFaultFromError } from "../🩺️fault/🟦️.ts";
import { seededActiveWindowId } from "../🎚️shell-model/🟦️.tsx";
import { encodeWindowActionInvocation } from "../📨️window-invocation/🟦️.ts";
import { canonicalSurfaceId } from "../🔐️identity-binding/🟦️.ts";
import {
  publishBuiltNodesV1,
  publishPanelBodiesV1,
  requiredHostPanelLeafId,
  SPAWNED_FOCUS_FRAME_BUDGET,
  type SpawnedRefreshSlotV1,
  withLocalizedWindowKindLabels
} from "../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterContributionsAndRefresh } from "../🔁️contributions-refresh/🟦️.ts";

export type ShellCoreAfterSpawnedSession = ShellCoreAfterContributionsAndRefresh & ReturnType<typeof useSpawnedSession>;

//#region FrameworkOsShell
export function useSpawnedSession(core: ShellCoreAfterContributionsAndRefresh) {
  const {
    documentServices, scope, hostMode, dispatch, loadedPlugins, session, uiLocale, uiTerminology, applyHistoryPatch,
    leftoverReplaceRefreshBodiesV1, applyLeftoverInteractionView, hostApp, landingAppId, appLabelsOverlay, shellLayout,
    spawnedRefreshSlotsRef, extraWindowCounterRef, extraWindowInstancesRef, sessionWorkRef, sealedInstancesRef,
    sessionSwitchGateRef, setSurfaceSwitchBusy, spawnedUiRefreshCacheRef, spawnedLayoutSeedRef, rememberOpenSpaceId,
    openInstanceIdRef, sessionRef, shellSessionLaneRef, builtNodeStoreCacheRef, backboneWorkerRef, identity, identityRef,
    verifiedSessionAuthorityRef, showTransientNoticeRef, openDocumentRef, closeDocumentRef, documentAttachmentLanesRef,
    inferencePortEpochRef, inferencePortOwnerRef, inferencePortAuthorityRef, retiredSessionDocumentOwnersRef,
    inferencePortOpeningRef, openDocumentSessionsRef, browserActorUiByRuntimeKeyRef, loadedPluginsRef,
    pluginSupervisorByIdRef, reportRefreshFault, documentServiceInventoryRef, ensureBackboneWorker,
    establishPrimaryWithShardRetry, panel, spawnedIdsRef, focusedProgram, focusedApp, focusedSpawnedId,
    activeUtilityByWindowIdRef, activeToolIdRef, writeToolRegister, activeWindowIdRef, shellStateRef,
    injectActiveUtility, publishContributions, refreshUi
  } = core;
  const refreshSpawnedUiPass = useCallback(
    async (spawned: SpawnedAppEntry, viewState: ViewModel) => {
      // 🩹️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END lane 5-E — same fix as `refreshUi`
      // above: reads `loadedPluginsRef.current` instead of closing over `loadedPlugins`, so this
      // callback's identity (and every effect that lists it in a deps array) stops churning on every
      // unrelated background catalogue plugin load.
      const loadedPlugins = loadedPluginsRef.current;
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId);
      const plugin = pluginEntry?.handle;
      const app = pluginEntry?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
      if (!plugin || !app) {
        console.warn("[os-shell] refreshSpawnedUi: plugin/app unavailable", { pluginId: spawned.pluginId, appId: spawned.appId });
        const unavailableWindowId = spawnedWindowInstanceIdV1(spawned.id, "main");
        publishBuiltNodesV1(builtNodeStoreCacheRef.current, [[`spawned:${unavailableWindowId}`, { ...pendingWindowUiNode(), activity: "idle", component: { type: "text", value: `Plugin unavailable: ${spawned.pluginId}/${spawned.appId}`, emphasize: null, dataAttributes: null } } satisfies BuiltNode]]);
        dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: { [unavailableWindowId]: "idle" } });
        dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
        dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
        return;
      }
      const spawnedSeed = `${spawned.pluginId}:${spawned.appId}:${spawned.instanceId}`;
      if (spawnedLayoutSeedRef.current !== spawnedSeed) {
        spawnedLayoutSeedRef.current = spawnedSeed;
        spawnedUiRefreshCacheRef.current = new Map();
      }
      const cache = spawnedUiRefreshCacheRef.current;
      // 🪟️ EVERY window the spawned app declares, not just its canvas body: a spawned program is a
      // member of this session and owns all of its own windows. The guest is addressed by its OWN
      // window instance ids (its window kind ids — it has exactly one instance), while the shell keys
      // them under `${spawnedId}::` so two spawned instances of one app cannot collide
      // (`🪟️spawned-program`).
      const windowKinds = spawnedGuestWindowInstancesV1(app.windowKinds, frameworkLayoutDeclaredInstances(app.defaultLayout, app.windowKinds));
      const guestActiveWindowId = guestWindowIdV1(activeWindowIdRef.current, spawnedIdsRef.current) ?? resolveCanvasBodyKey(app);
      // 🧩️ A spawned instance reads contributions from the registry the paged `setContributions` run
      // installed, exactly like a primary session — the view context carries none.
      const fullViewState: ViewModel = injectActiveUtility(
        {
          ...spawnedProgramViewStateV1(viewState, app),
          locale: uiLocale,
          terminology: uiTerminology,
          windowId: guestActiveWindowId,
          windowInstances: windowKinds.map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
          activeUtilityByWindowId: guestActiveUtilityByWindowIdV1(activeUtilityByWindowIdRef.current, spawned.id, spawnedIdsRef.current),
        },
        spawnedWindowInstanceIdV1(spawned.id, guestActiveWindowId),
      );
      const panelTabLeaves = flattenPanelTabLeaves(app.panelTabs);
      const contributions = await publishContributions({ pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: spawnedProgramViewStateV1(viewState, app) }, sessionProgramsV1(loadedPlugins, spawned.pluginId), true);
      if (contributions.status === "failed") console.error(`[os-shell] contributions push into spawned "${spawned.pluginId}" #${spawned.instanceId} failed: ${contributions.reason}`);
      if (spawnedLayoutSeedRef.current !== spawnedSeed) return;
      const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, fullViewState, cache);
      if (request) {
        const response = await plugin.refreshUi(spawned.instanceId, request);
        if (spawnedLayoutSeedRef.current !== spawnedSeed) return;
        applyUiRefreshResponseToCache(cache, response);
      }
      const uiByWindowId = Object.fromEntries(windowKinds.map((kind) => [spawnedWindowInstanceIdV1(spawned.id, kind.id), (cache.get(`window:${kind.id}`)?.value as BuiltNode | undefined) ?? pendingWindowUiNode()] as const));
      const spawnedKeyed = <T,>(byGuestWindowId: Readonly<Record<string, T>>): Readonly<Record<string, T>> =>
        Object.fromEntries(Object.entries(byGuestWindowId).map(([windowId, value]) => [spawnedWindowInstanceIdV1(spawned.id, windowId), value] as const));
      const dynamicEngagements = spawnedKeyed((cache.get("engagements")?.value as Readonly<Record<string, WindowEngagement>> | undefined) ?? {});
      const dynamicMeasures = spawnedKeyed((cache.get("measures")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {});
      publishBuiltNodesV1(builtNodeStoreCacheRef.current, Object.entries(uiByWindowId).map(([windowId, node]) => [`spawned:${windowId}`, node] as const));
      dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: (current) => mergeRecordPreservingIdentity(current, Object.entries(uiByWindowId).map(([windowId, node]) => [windowId, node.activity] as const)) });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicEngagements)) });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicMeasures)) });
      const panelBodies = programKeyedEntriesV1(
        spawned.id,
        Object.fromEntries(panelTabLeaves.filter((tab) => tab.bodyKey).map((tab) => [panelTabKindId(tab.kind), cache.get(`panel:${panelTabKindId(tab.kind)}`)?.value as BuiltNode | undefined] as const)),
      );
      const panelStores = publishPanelBodiesV1(builtNodeStoreCacheRef.current, Object.entries(panelBodies));
      dispatch({ type: "SET_PANEL_BODY_STORE_BY_KEY", value: (current) => mergeRecordPreservingIdentity(current, withProgramEntriesV1(current, spawned.id, spawnedIdsRef.current, panelStores)) });
      const toolMeasures = programKeyedEntriesV1(spawned.id, (cache.get("tools")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {});
      dispatch({ type: "SET_TOOL_MEASURES_BY_TOOL_ID", value: (current) => mergeRecordPreservingIdentity(current, withProgramEntriesV1(current, spawned.id, spawnedIdsRef.current, Object.entries(toolMeasures))) });
    },
    [injectActiveUtility, publishContributions, uiLocale, uiTerminology],
  );
  const refreshSpawnedUiPassRef = useRef(refreshSpawnedUiPass);
  refreshSpawnedUiPassRef.current = refreshSpawnedUiPass;
  const refreshSpawnedUi = useCallback(
    (spawned: SpawnedAppEntry, viewState: ViewModel, scopeArg: UiDirtyScope = { kind: "full" }): Promise<void> => {
      if (scopeArg.kind === "none") return Promise.resolve();
      const slots = spawnedRefreshSlotsRef.current;
      const running = slots.get(spawned.id);
      if (running) {
        running.viewState = viewState;
        running.again = true;
        return running.done;
      }
      const slot: SpawnedRefreshSlotV1 = { viewState, again: false, done: Promise.resolve() };
      slot.done = (async () => {
        try {
          do {
            slot.again = false;
            await refreshSpawnedUiPassRef.current(spawned, slot.viewState);
          } while (slot.again);
        } finally {
          slots.delete(spawned.id);
        }
      })();
      slots.set(spawned.id, slot);
      return slot.done;
    },
    [],
  );

  // 🐢️ Keyed on the pluginId/app/instance triple (not `session` object identity) so this only fires on
  // a genuine session switch (app open/spawn/instance change) — every other action already calls
  // `refreshUi` explicitly via `applyHostEffects`, and re-running it here too on every `session` object
  // churn was a second, redundant full-shell refresh cascade per interaction.
  // 🩹️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END lane 5-E — `loadedPlugins` dropped
  // from this effect's own deps (mirrors `refreshUi`'s own fix above, and lane 5-A's `readHistory`
  // effect fix, line ~992): a session can only exist for an already-loaded plugin (nothing sets
  // `session`/`sessionIdentityKey` for a plugin still mid-load), so this effect never needed to refire
  // on unrelated background catalogue loads in the first place — `refreshUi` itself now reads
  // `loadedPluginsRef.current` at call time regardless. Before this fix, `loadedPlugins` getting a new
  // array reference on every one of the ~50+ sequential background plugin loads during boot re-ran this
  // effect that many times for the SAME already-open session, each dispatching a fresh top-level
  // `refreshUi` against the wasm guest's single-flight `InstanceGuard` — live-confirmed as the dominant
  // contributor to the `plugin.internal: plugin instance busy` storm blocking collab-e2e STEP 2.
  // 🪪️ The signed-in human is tracked beside the app/instance triple, not inside it: a changed triple is a live session that
  // only needs re-rendering, while a changed HUMAN is recovered by the human-change effect below
  // ({@link shellHumanChangeRecoveryV1}) — re-rendered for a first human or a new display name, established again for
  // another human. `resolvedTargetViewState` stamps `sessionIdentity` from `identityRef.current` at plugin-call time.
  const sessionTripleKey = session ? `${session.pluginId}:${session.app.id}:${session.instanceId}` : null;
  const sessionHumanKey = `${identity?.userId ?? ""}:${identity?.displayName ?? ""}`;
  useEffect(() => {
    const current = sessionRef.current;
    if (!current) return;
    void refreshUi(current).catch((renderError) => reportRefreshFault(current.pluginId, renderError));
  }, [refreshUi, reportRefreshFault, sessionTripleKey]);

  useEffect(() => {
    if (!hostMode || !session) {
      dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
      return;
    }
    const activeSpawned = panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
    if (!activeSpawned) {
      dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
      return;
    }
    void refreshSpawnedUi(activeSpawned, session.viewState).catch((renderError) => {
      const fault = liveInstanceWindowFaultV1(renderError, pluginSupervisorByIdRef.current[activeSpawned.pluginId], isPluginInstanceRetiredV1);
      if (fault !== null) dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: {}, fault });
    });
    // 🩹️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END lane 5-E — `loadedPlugins` dropped
    // (same reasoning as the session-refresh effect above): `refreshSpawnedUi` now reads
    // `loadedPluginsRef.current` at call time, and a spawned instance can only exist for an
    // already-loaded plugin, so this never needed to refire on unrelated background catalogue loads.
  }, [panel, refreshSpawnedUi, refreshSpawnedUiPass, session, hostMode]);

  /** 🪟️ The session layout the shell was showing when a spawned program took the canvas — restored
   * verbatim when focus comes back, so opening a foreign editor and closing it again does not cost
   * the user the arrangement they built. */
  const sessionLayoutBeforeSpawnRef = useRef<{ readonly layout: WindowLayoutNode | null; readonly activeWindowId: string | null } | null>(null);
  const focusedProgramKey = focusedProgramKeyV1(focusedProgram);
  const seededFocusedProgramKeyRef = useRef<string>("");
  /**
   * 🪟️ Seeds the canvas for whichever program owns it. THIS is what makes a spawned program a member
   * of the session rather than a breadcrumb: `Mode` prunes every layout leaf whose id is absent from
   * the window list it is handed (`resolveModeLayout` → `reconcileWindows`), so a spawned program
   * whose windows were never put into `shellLayout` reconciled the host app's leaves away and painted
   * the empty-shell notice — measured verbatim on the real `s` host as "Drag windows from Display in
   * the navbar, or restore a saved layout" with `[data-window-id] = []`
   * (ticket 26/09/18, S4 §5.2). The spawned app's OWN declared default layout is used, renamed into
   * that instance's window-id namespace, so a two-window foreign editor opens as its author declared.
   *
   * 🚦️ Keyed on the focused PROGRAM, never on `session` identity: an ordinary refresh must not
   * re-seed, or every action would throw the live arrangement away.

   */
  useEffect(() => {
    if (!session) return;
    if (seededFocusedProgramKeyRef.current === focusedProgramKey) return;
    const previous = seededFocusedProgramKeyRef.current;
    seededFocusedProgramKeyRef.current = focusedProgramKey;
    if (previous !== "" && activeToolIdRef.current) writeToolRegister(null, null);
    if (focusedSpawnedId !== null && focusedApp !== null) {
      // 🪟️ Leaving the session's own canvas — remember it before the spawned seed replaces it.
      if (sessionLayoutBeforeSpawnRef.current === null) sessionLayoutBeforeSpawnRef.current = { layout: shellLayout ?? null, activeWindowId: activeWindowIdRef.current };
      const seed = resolveFrameworkLayoutSeed(focusedApp.defaultLayout, withLocalizedWindowKindLabels(focusedApp.windowKinds), appLabelsOverlay, uiTerminology, uiLocale);
      const modeLayout = renameLayoutWindowIdsV1(seed.modeLayout as WindowLayoutNode & { kind: "window" | "stack" | "row" | "column" }, spawnedLayoutRenameV1(focusedSpawnedId, focusedApp.windowKinds, frameworkLayoutDeclaredInstances(focusedApp.defaultLayout, focusedApp.windowKinds))) as WindowLayoutNode;
      dispatch({ type: "SET_SHELL_LAYOUT", value: modeLayout });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: seededActiveWindowId(modeLayout) });
      return;
    }
    const restored = sessionLayoutBeforeSpawnRef.current;
    sessionLayoutBeforeSpawnRef.current = null;
    if (restored?.layout) {
      dispatch({ type: "SET_SHELL_LAYOUT", value: restored.layout });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: restored.activeWindowId ?? seededActiveWindowId(restored.layout) });
      return;
    }
    if (previous === "") return;
    const seed = resolveFrameworkLayoutSeed(session.app.defaultLayout, withLocalizedWindowKindLabels(session.app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale);
    dispatch({ type: "SET_SHELL_LAYOUT", value: seed.modeLayout });
    dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: seededActiveWindowId(seed.modeLayout) });
    // 🐢️ `shellLayout` is read, not depended on: this effect fires on a focused-PROGRAM change only,
    // and listing the live layout would re-enter it on every drag the user makes.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [focusedProgramKey, focusedSpawnedId, focusedApp, session, appLabelsOverlay, uiTerminology, uiLocale, writeToolRegister]);

  /** ⌨️ A program the user just opened takes keyboard focus once its active window has mounted — its dock tabpanel,
   * the same stop a pointer click lands on — but only while nothing else holds focus. Opening a program from the
   * command palette used to leave focus on `<body>`, so the next Tab restarted at the navbar instead of inside the
   * program the user asked for (WCAG 2.2 SC 2.4.3, measured inside `s`, ticket 26/09/23 S15). */
  useEffect(() => {
    if (focusedSpawnedId === null) return;
    let frames = 0;
    let handle = 0;
    const attempt = (): void => {
      const holder = document.activeElement;
      if (holder instanceof HTMLElement && holder !== document.body) return;
      const windowId = activeWindowIdRef.current;
      const panel = windowId === null ? null : scope.rootRef.current?.querySelector<HTMLElement>(`[data-slot="window"][id="${CSS.escape(windowId)}"]`)?.closest<HTMLElement>('[role="tabpanel"]');
      if (panel) panel.focus({ preventScroll: true });
      else if (++frames < SPAWNED_FOCUS_FRAME_BUDGET) handle = requestAnimationFrame(attempt);
    };
    handle = requestAnimationFrame(attempt);
    return () => cancelAnimationFrame(handle);
  }, [focusedProgramKey, focusedSpawnedId]);

  const updateSpacePanel = useCallback((panelState: SpacePanelState) => {
    dispatch({
      type: "SET_SESSION",
      value: (current) => {
        if (!current) return current;
        return { ...current, viewState: { ...current.viewState, panelJson: panelJsonFromState(panelState) } };
      },
    });
  }, []);

  /** 🪦️ The session-instance close ladder, lifted out of the shell-unmount teardown effect so an
   * in-place app switch retires its predecessor the same way unmounting does: every document session
   * still bound to that instance is closed (which retires its attachment), then the attachment lane is
   * drained, then the instance is destroyed. Without it every `switchToPluginApp` leaked one live
   * `createApp` — tolerable while only two host apps ever swapped, not for a user-facing toggle
   * (`📓️viewer-eval-chain-2026-09-12.md` §6 item 2). Never throws: a predecessor that refuses to
   * retire is a leak to report, never a reason to block the successor. */
  const retireSessionInstance = useCallback(async (retired: ActiveSession): Promise<void> => {
    const plugin = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === retired.pluginId)?.handle;
    if (!plugin) return;
    for (const [runtimeKey, entry] of [...openDocumentSessionsRef.current]) {
      if (entry.plugin !== plugin || entry.session.instanceId !== retired.instanceId) continue;
      closeDocumentRef.current(runtimeKey, entry.clientInstanceId);
    }
    await documentAttachmentLanesRef.current.get(plugin)?.get(retired.instanceId)?.drain();
    await plugin.destroyApp(retired.instanceId);
  }, []);

  /** 🪪️ A change of the signed-in human is recovered by what it invalidates ({@link shellHumanChangeRecoveryV1}): a first
   * human for a session assembled signed-out, or a new display name, only re-renders it — signed out is a state every
   * surface renders, and the directory owner binds the visible Home as soon as the hub confirms the human, so Home fills in
   * place and whatever the human opened on it (a dialog) stays. Before, every sign-in re-established Home 5–8 s later and
   * closed that dialog (C11 run b3-3: `event-page?after=0` for instance 2 behind a socket closed on instance 1). A session
   * assembled for ANOTHER human — a switch of accounts, or a sign-out — is established again with the same
   * `establishPrimaryWithShardRetry` the killed-boot and worker-loss paths use, and the superseded instance is retired once
   * the new one is mounted.
   *
   * The first run only records the human: mounting is not a change. Both recoveries run on the shell session lane, after
   * whatever route application is in flight; after a re-establishment the route effect re-applies the session route behind
   * it — the new session is the landing app. A space route is held until the identity it is opened for resolves
   * ({@link shellRouteAdmissionV1}), so a hard load of `/spaces/<id>` mounts the space once, and a sign-in under the hub
   * overlay returns to the space the overlay sits over (ticket 26/09/23 U5; C10 and G10 S4 relays). */
  const sessionHumanRef = useRef<ShellHumanV1 | null | undefined>(undefined);
  useEffect(() => {
    const previous = sessionHumanRef.current;
    const current: ShellHumanV1 | null = identity ? { userId: identity.userId, displayName: identity.displayName } : null;
    sessionHumanRef.current = current;
    if (!sessionRef.current || previous === undefined) return;
    const recovery = shellHumanChangeRecoveryV1(previous, current);
    if (recovery === "keep") return;
    void shellSessionLaneRef.current.run(async () => {
      const active = sessionRef.current;
      if (!active) return;
      if (recovery === "refresh") {
        await refreshUi(active).catch((renderError) => reportRefreshFault(active.pluginId, renderError));
        return;
      }
      const handle = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === active.pluginId)?.handle;
      if (!handle) return;
      try {
        await establishPrimaryWithShardRetry(handle);
      } catch (error) {
        const fault = windowFaultFromError(error, pluginSupervisorByIdRef.current[active.pluginId]);
        dispatch({ type: "SET_ERROR", value: fault.message, fault });
        return;
      }
      await retireSessionInstance(active).catch(() => undefined);
    });
  }, [sessionHumanKey, establishPrimaryWithShardRetry, retireSessionInstance, refreshUi, reportRefreshFault]);

  // 🏠️🧳️👁️✏️ Generalised from the host-only `switchToManagedApp`: switches the mounted session to ANY
  // loaded plugin's app by `(pluginId, appId)`, so the navbar role group can swap a playground between
  // `…#editor` and `…#viewer` the same way studio mode swaps its landing/host apps. The host-only
  // bookkeeping (`openSpaceIdRef`/`openInstanceIdRef` reset on the landing app) stays behind an
  // explicit `hostMode` guard rather than being reachable from a non-host switch.
  //
  // 🪦️ TRANSACTIONAL since 26/09/09/PROCEDURAL-3D-END-TO-END: the whole switch runs through
  // `sessionSwitchGateRef` (one at a time) and starts with a `quiesce` pass over `sessionWorkRef`, so
  // the predecessor is never torn down under its own in-flight typed operations and extension
  // invocations — the mid-chain tear-down measured on 6018, where `⌘️⌥️V` pressed during an
  // `interactionSelect`/`flow-extension-brep evaluate` round trip revoked instance 1's activation and
  // left every later action failing with `no actor for instance 1`
  // (`🗑️generated/journey-3/console.txt`). `seal` runs before the close ladder because the ladder's own
  // first step already revokes the actor.
  const switchToPluginApp = useCallback(
    async (pluginId: string, appId: string, viewState?: ViewModel, documentSource?: ActiveSession): Promise<ActiveSession | null> => {
      const defaultViewState = (app: AppDefinition): ViewModel => ({ activeModeId: app.defaultModeId ?? app.modes[0]?.id, ...(hostMode ? { panelJson: panelJsonFromState(buildSpacePanelState([], requiredHostPanelLeafId(hostApp))) } : {}) });
      setSurfaceSwitchBusy(true);
      try {
        const outcome = await sessionSwitchGateRef.current.run(
          {
            session,
            resolveApp: (targetPluginId, targetAppId) => loadedPlugins.find((entry) => entry.handle.pluginId === targetPluginId)?.manifest.apps.find((candidate) => candidate.id === targetAppId) ?? null,
            appId: (app) => app.id,
            quiesce: async (retiring) => {
              const quiet = await quiesceSessionWorkV1(() => sessionWorkRef.current.pending(retiring.pluginId, retiring.instanceId));
              // ⏳️ A refusal has to name what it waited on, or the next person reading the console sees
              // only a number and cannot tell a stuck guest turn from a stuck extension round trip.
              if (!quiet.settled) console.warn(`surface-switch draining ${retiring.pluginId}#${retiring.instanceId} ${sessionWorkRef.current.outstanding(retiring.pluginId, retiring.instanceId)}`);
              return quiet;
            },
            seal: (retiring) => sealedInstancesRef.current.seal(retiring.pluginId, retiring.instanceId),
            unseal: (kept) => sealedInstancesRef.current.unseal(kept.pluginId, kept.instanceId),
            createInstance: async (targetPluginId, app) => {
              const handle = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === targetPluginId)?.handle;
              if (!handle) throw new Error(`switchToPluginApp: ${targetPluginId} is no longer loaded`);
              if (documentSource === undefined) return handle.createApp(app.id);
              const current = () => sessionRef.current?.pluginId === documentSource.pluginId && sessionRef.current?.instanceId === documentSource.instanceId;
              if (targetPluginId !== documentSource.pluginId || !current() || !app.dialect) throw new Error("surface-document.predecessor-stale");
              const readArchive = handle.readAppDocumentArchive;
              const loadArchive = handle.loadAppDocumentArchive;
              if (!readArchive || !loadArchive) throw new Error("surface-document.archive-port-unavailable");
              const owners = [...openDocumentSessionsRef.current.values()].filter(entry => entry.plugin === handle && entry.session.instanceId === documentSource.instanceId);
              if (owners.length > 1) throw new Error("surface-document.owner-ambiguous");
              const owner = owners[0];
              const surfaceId = canonicalSurfaceId(app.dialect,app.role);
              let receipt: DocumentOpeningReceiptV1 | null = null;
              return prepareDocumentSurfaceV1({
                current,
                capture: () => readArchive(documentSource.instanceId),
                create: () => handle.createApp(app.id),
                restore: (instanceId,archive) => loadArchive(instanceId,archive),
                attach: async instanceId => {
                  if (owner === undefined) return;
                  const nextSession: ActiveSession = { pluginId: targetPluginId,instanceId,app,viewState: viewState ?? defaultViewState(app),documentInitialized: true };
                  const bindings = owner.persistenceBindings.map(binding => binding.kind === "hub" ? { ...binding,requestedSurfaceId: surfaceId } : binding);
                  receipt = await openDocumentRef.current(owner.openingReference,bindings,{ plugin: handle,session: nextSession });
                  if (receipt === null) throw new Error("surface-document.attachment-refused");
                },
                release: async instanceId => {
                  if (receipt !== null) closeDocumentRef.current(receipt.runtimeKey,receipt.clientInstanceId);
                  await documentAttachmentLanesRef.current.get(handle)?.get(instanceId)?.drain();
                  await handle.destroyApp(instanceId);
                },
              });
            },
            retire: retireSessionInstance,
            // 🪶️ The empty studio panel belongs to `hostMode` only: a non-host playground's boot session
            // carries no `panelJson` at all, so a role switch must not invent one either.
            defaultViewState,
            publish: (next) => dispatch({ type: "SET_SESSION", value: { ...next,documentInitialized: documentSource !== undefined } }),
            seedLayout: (app) => {
              const seeded = applyFrameworkLayoutSeed(app.defaultLayout, withLocalizedWindowKindLabels(app.windowKinds), appLabelsOverlay, uiTerminology, uiLocale);
              extraWindowInstancesRef.current = seeded.extraInstances;
              extraWindowCounterRef.current = seeded.extraInstances.length;
              dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
              dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
              dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: seededActiveWindowId(seeded.modeLayout) });
              if (hostMode && app.id === landingAppId) {
                rememberOpenSpaceId(null);
                openInstanceIdRef.current = null;
              }
            },
            refresh: refreshUi,
          },
          { pluginId, appId, viewState },
          (retired, retireError) => console.warn(`switchToPluginApp: predecessor ${retired.pluginId}/${retired.app.id} retirement failed`, retireError),
        );
        // 🚦️ `draining` and `busy` both mean "the switch did NOT happen": the predecessor kept its
        // instance and the shell kept its surface. Telling the user so is the whole difference between
        // a refusal and a silent no-op — the alternative the fixture rules out is tearing the
        // predecessor down under its own in-flight work.
        if (outcome.status === "draining" || outcome.status === "busy") {
          showTransientNoticeRef.current(surfaceSwitchBusyTextV1(uiLocale), "info");
          return null;
        }
        return outcome.session === null || documentSource === undefined ? outcome.session : { ...outcome.session,documentInitialized: true };
      } finally {
        setSurfaceSwitchBusy(false);
      }
    },
    [loadedPlugins, refreshUi, retireSessionInstance, session, appLabelsOverlay, hostMode, hostApp, landingAppId, uiTerminology, uiLocale, rememberOpenSpaceId],
  );

  const syncSpawnedPluginDocument = useCallback(async (plugin: PluginWasmHandle, app: AppDefinition, pluginInstanceId: number, artifactJson: string, viewState: ViewModel) => {
    try {
      const document = JSON.parse(artifactJson) as Record<string, unknown>;
      const targetSession: ActiveSession = { pluginId: plugin.pluginId, instanceId: pluginInstanceId, app, viewState: spawnedProgramViewStateV1(viewState, app) };
      await plugin.handleAction(pluginInstanceId, encodeWindowActionInvocation(targetSession, { controllerId: app.controllerId, action: "setDocument", args: { document } }), viewState);
    } catch (syncError) {
    }
  }, []);

  const ensureSpawnedPlugin = useCallback(
    async (program: SpaceProgramEntry, isCurrent: () => boolean, label?: string, osInstanceId?: string, artifactJson?: string, sourceViewState?: ViewModel): Promise<SpacePanelState | null> => {
      if (!isCurrent()) return null;
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === program.pluginId);
      if (!pluginEntry || !session) return null;
      const app = pluginEntry.manifest.apps.find((candidate) => candidate.id === program.appId);
      const currentPanel = parsePanelState(sourceViewState ?? session.viewState) ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
      const existing = osInstanceId ? currentPanel.spawnedApps.find((entry) => entry.id === osInstanceId) : currentPanel.spawnedApps.find((entry) => entry.appId === program.appId && entry.pluginId === program.pluginId);
      if (existing) {
        if (artifactJson && app) {
          await syncSpawnedPluginDocument(pluginEntry.handle, app, existing.instanceId, artifactJson, sourceViewState ?? session.viewState);
        }
        return isCurrent() ? studioPanelFocusingSpawned(currentPanel, existing) : null;
      }
      const instanceId = await createAdmittedShellInstanceV1(isCurrent, () => pluginEntry.handle.createApp(program.appId), (id) => pluginEntry.handle.destroyApp(id));
      if (instanceId === null) return null;
      if (artifactJson && app) {
        await syncSpawnedPluginDocument(pluginEntry.handle, app, instanceId, artifactJson, sourceViewState ?? session.viewState);
      }
      if (!isCurrent()) {
        await pluginEntry.handle.destroyApp(instanceId);
        return null;
      }
      const spawnedId = osInstanceId ?? `${program.pluginId}-${instanceId}`;
      return studioPanelFocusingSpawned(currentPanel, {
        id: spawnedId,
        pluginId: program.pluginId,
        instanceId,
        appId: program.appId,
        label: label ?? program.label,
        breadcrumb: program.breadcrumb,
      });
    },
    [loadedPlugins, session, syncSpawnedPluginDocument, hostApp],
  );

  /**
   * 🐚️ Consumes a plugin action's typed `requestedEffects: Effect[]` (WS-D's `InvocationResponse`) —
   * replaces the deleted `processPluginOperations` string-matching. The legacy `setDocument`-mirror
   * backbone-write block is gone entirely: document content sync now flows through
   * `openDocument`/`closeDocument`'s worker-backed `DocumentHost` lifecycle, not a per-operation JS mirror.
   */
  useEffect(() => {
    const services = documentServices.filter((presentation) => loadedPlugins.some((entry) => entry.handle.pluginId === presentation.owner && (entry.manifest.topicContributions ?? []).some((topic) => {
      if (topic.topic !== DOCUMENT_SERVICE_TOPIC_V1) return false;
      try { return admitDocumentServiceWireDeclarationV1(presentation.owner, topic.payload).serviceId === presentation.serviceId; } catch { return false; }
    }))).map(({owner, serviceId}) => ({owner, serviceId}));
    documentServiceInventoryRef.current=services;
    backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "service-contributions", services }) });
  }, [loadedPlugins, documentServices]);

  const requestServiceOperation = useCallback(async (baseSession: ActiveSession, operation: Extract<Effect, { readonly requestServiceOperation: unknown }>["requestServiceOperation"], admit: () => boolean) => {
    if (!admit()) throw new Error("inference-opening: owner retired");
    if (operation.owner !== baseSession.pluginId) throw new Error("installed-service.foreign-owner");
    const presentations = documentServices.filter((entry) => entry.owner === operation.owner && entry.serviceId === operation.serviceId);
    if (presentations.length !== 1) throw new Error("installed-service.unavailable");
    const program = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === baseSession.pluginId);
    const declarations = (program?.manifest.topicContributions ?? []).filter((entry) => entry.topic === DOCUMENT_SERVICE_TOPIC_V1).map((entry) => admitDocumentServiceWireDeclarationV1(operation.owner, entry.payload));
    if (declarations.filter((entry) => entry.serviceId === operation.serviceId).length !== 1) throw new Error("installed-service.unavailable");
    if (inferencePortOwnerRef.current !== null || inferencePortOpeningRef.current !== null) throw new Error("inference.capacity");
    const owners = [...openDocumentSessionsRef.current.entries()].filter(([, entry]) => entry.session.pluginId === baseSession.pluginId && entry.session.instanceId === baseSession.instanceId && entry.scope !== undefined);
    const owner = owners.length === 1 ? owners[0] : undefined;
    const scope = owner?.[1].scope;
    if (!owner || !scope) throw new Error("inference-proposal: exact document owner required");
    const authority = verifiedSessionAuthorityRef.current;
    if (!directorySessionAuthorityIsCurrentV1(authority, authority) || authority?.userId !== identityRef.current?.userId || retiredSessionDocumentOwnersRef.current.has(owner[1])) throw new Error("inference-proposal: authenticated authority required");
    if (inferencePortEpochRef.current === Number.MAX_SAFE_INTEGER) throw new Error("inference-proposal: epoch exhausted");
    const worker = ensureBackboneWorker();
    const operationEpoch = ++inferencePortEpochRef.current;
    const [runtimeKey] = owner;
    const mailbox = new InferencePortOpeningMailboxV1((request) => worker.postMessage({ wire: encodeBackboneWorkerRequest(request) }));
    const opening = { owner: { owner: operation.owner, serviceId: operation.serviceId, operationEpoch, runtimeKey, scope }, clientInstanceId: owner[1].clientInstanceId, sessionInstanceId: baseSession.instanceId, mailbox };
    inferencePortOpeningRef.current = opening;
    try {
      await mailbox.open({ kind: "service-operation", owner: operation.owner, serviceId: operation.serviceId, action: "open", operationEpoch, payload: { scope } });
      const entry = openDocumentSessionsRef.current.get(runtimeKey);
      if (inferencePortOpeningRef.current !== opening || entry?.clientInstanceId !== opening.clientInstanceId || entry.session.instanceId !== opening.sessionInstanceId || retiredSessionDocumentOwnersRef.current.has(entry) || !directorySessionAuthorityIsCurrentV1(authority, verifiedSessionAuthorityRef.current) || !admit()) throw new Error("inference-opening: owner retired");
      inferencePortOwnerRef.current = opening.owner;
      inferencePortAuthorityRef.current = authority;
      dispatch({ type: "OPEN_INFERENCE_PORT", runtimeKey, operationEpoch });
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "service-operation", owner: operation.owner, serviceId: operation.serviceId, action: operation.action, operationEpoch, payload: operation.payload }) });
    } catch (error) {
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "service-operation", owner: operation.owner, serviceId: operation.serviceId, action: "close", operationEpoch, payload: {} }) });
      throw error;
    } finally {
      if (inferencePortOpeningRef.current === opening) inferencePortOpeningRef.current = null;
      mailbox.close("inference-opening: settled");
    }
  }, [ensureBackboneWorker]);

  const directBrowserActorForSession = useCallback((baseSession: ActiveSession) => {
    const matches = [...browserActorUiByRuntimeKeyRef.current.entries()].flatMap(([runtimeKey, retained]) => {
      const entry = openDocumentSessionsRef.current.get(runtimeKey);
      return retained.identity !== null && entry?.clientInstanceId === retained.clientInstanceId && entry.session.pluginId === baseSession.pluginId && entry.session.instanceId === baseSession.instanceId && retained.sessionInstanceId === baseSession.instanceId && shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, entry.session)
        ? [{ runtimeKey, retained, entry, identity: retained.identity }]
        : [];
    });
    if (matches.length > 1) throw new Error("browser-actor-action: ambiguous document owner");
    return matches[0] ?? null;
  }, []);

  const dispatchDirectBrowserActorCommand = useCallback(async (
    owned: NonNullable<ReturnType<typeof directBrowserActorForSession>>,
    invocation: ActionInvocation | CommandInvocation,
    viewState: ViewModel,
  ) => {
    const { runtimeKey, retained, entry, identity } = owned;
    const current = (): boolean => {
      const live = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
      return live?.actions === retained.actions && live.identity === identity && openDocumentSessionsRef.current.get(runtimeKey) === entry && shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, entry.session);
    };
    if (!current()) throw new Error("browser-actor-action: owner retired");
    const windowKindId = "windowKindId" in invocation.address ? invocation.address.windowKindId : typeof viewState.activeWindowKindId === "string" ? viewState.activeWindowKindId : identity.windowKindId;
    const store = retained.windows.get(windowKindId);
    if (store === undefined) throw new Error("browser-actor-action: window unpainted");
    const result = await retained.actions.dispatchCommand({
      scope: retained.scope,
      verifiedSurfaceId: retained.verifiedSurfaceId,
      activationGeneration: retained.activationGeneration,
      appChannelVersion: BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION,
      instanceId: identity.instanceId,
      surfaceRevision: store.getRevisionSnapshot(),
    }, invocation, viewState);
    for (const patch of result.historyPatches) if (current()) applyHistoryPatch(decodePackWire(Uint8Array.from(patch), "$.historyPatch") as HistoryPatch, false, { pluginId: entry.session.pluginId, instanceId: entry.session.instanceId });
    await publishBrowserActorHostEffectsV1(result.hostEffects, current, async (effect) => {
      if ("requestServiceOperation" in effect) await requestServiceOperation(entry.session, effect.requestServiceOperation, current);
      else window.open(effect.openExternalUrl.url, "_blank", "noopener,noreferrer");
    });
    // 🖼️ Wave B9 lane 4: this is the ONE dispatch route that carries no `UiDirtyScope` of its own and
    // publishes no `OperationCompleted` frame for the verbs that commit inline, so without this the
    // document changes and nothing repaints until some later, unrelated action refreshes — `paste`
    // reached the world lane at +12 s in the browser (wave B6 §3). `browserActorDispatchUiScopeV1`
    // answers `none` for a refusal or a zero-mutation action, and `refreshUi` returns immediately on it.
    const actionId = "actionId" in invocation.address ? invocation.address.actionId : undefined;
    applyLeftoverInteractionView(("output" in result ? (result as { output?: unknown }).output : undefined) ?? null);
    const dirty = browserActorWindowConfigDispatchUiScopeV1(result, actionId);
    if (dirty.kind !== "none" && current()) {
      await refreshUi({ ...entry.session, viewState }, dirty, undefined, leftoverReplaceRefreshBodiesV1());
    }
    return result;
  }, [applyHistoryPatch, directBrowserActorForSession, refreshUi, requestServiceOperation]);

  return {
    refreshSpawnedUi, updateSpacePanel, switchToPluginApp, ensureSpawnedPlugin, requestServiceOperation,
    directBrowserActorForSession, dispatchDirectBrowserActorCommand
  };
}
//#endregion FrameworkOsShell
