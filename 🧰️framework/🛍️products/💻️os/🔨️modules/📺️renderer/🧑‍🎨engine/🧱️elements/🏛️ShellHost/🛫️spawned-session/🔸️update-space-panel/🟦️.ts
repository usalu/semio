/** 🔸️ The update space panel phase of useSpawnedSession. */
import { useCallback, useEffect, useRef } from "react";
import { shellHumanChangeRecoveryV1, type ShellHumanV1 } from "../../🔀️surface-switch/🟦️.ts";
import { renameLayoutWindowIdsV1, spawnedLayoutRenameV1 } from "../../🪟️spawned-program/🟦️.ts";
import { type WindowLayoutNode } from "@semio-tech/ui-react";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { resolveFrameworkLayoutSeed, frameworkLayoutDeclaredInstances } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { panelJsonFromState, type SpacePanelState } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { windowFaultFromError } from "../../🩺️fault/🟦️.ts";
import { seededActiveWindowId } from "../../🎚️shell-model/🟦️.tsx";
import { SPAWNED_FOCUS_FRAME_BUDGET, withLocalizedWindowKindLabels } from "../../🧱️built-node-stores/🟦️.ts";
import type { useDocumentServicesState } from "../🔶️document-services/🟦️.ts";

/** 🔹 Builds updateSpacePanel, retireSessionInstance, sessionHumanRef of useSpawnedSession. */
export function useUpdateSpacePanelState({ session, seededFocusedProgramKeyRef, focusedProgramKey, activeToolIdRef, writeToolRegister, focusedSpawnedId, focusedApp, sessionLayoutBeforeSpawnRef, shellLayout, activeWindowIdRef, appLabelsOverlay, uiTerminology, uiLocale, dispatch, scope, loadedPluginsRef, openDocumentSessionsRef, closeDocumentRef, documentAttachmentLanesRef, identity, sessionRef, shellSessionLaneRef, refreshUi, reportRefreshFault, establishPrimaryWithShardRetry, pluginSupervisorByIdRef, sessionHumanKey }: Pick<ReturnType<typeof useDocumentServicesState>, "session" | "seededFocusedProgramKeyRef" | "focusedProgramKey" | "activeToolIdRef" | "writeToolRegister" | "focusedSpawnedId" | "focusedApp" | "sessionLayoutBeforeSpawnRef" | "shellLayout" | "activeWindowIdRef" | "appLabelsOverlay" | "uiTerminology" | "uiLocale" | "dispatch" | "scope" | "loadedPluginsRef" | "openDocumentSessionsRef" | "closeDocumentRef" | "documentAttachmentLanesRef" | "identity" | "sessionRef" | "shellSessionLaneRef" | "refreshUi" | "reportRefreshFault" | "establishPrimaryWithShardRetry" | "pluginSupervisorByIdRef" | "sessionHumanKey">) {
  useEffect(() => {
    if (!session) return;
    if (seededFocusedProgramKeyRef.current === focusedProgramKey) return;
    const previous = seededFocusedProgramKeyRef.current;
    seededFocusedProgramKeyRef.current = focusedProgramKey;
    if (previous !== "" && activeToolIdRef.current) writeToolRegister(null, null);
    if (focusedSpawnedId !== null && focusedApp !== null) {
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
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [focusedProgramKey, focusedSpawnedId, focusedApp, session, appLabelsOverlay, uiTerminology, uiLocale, writeToolRegister]);

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
  return { retireSessionInstance, updateSpacePanel };
}
