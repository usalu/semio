import type { ShellCoreAfterContributionsAndRefresh } from "../🔁️contributions-refresh/🟦️.ts";
import { useDocumentServicesState } from "./🔶️document-services/🟦️.ts";
import { useUpdateSpacePanelState } from "./🔸️update-space-panel/🟦️.ts";
import { useSwitchPluginAppState } from "./🟠️switch-plugin-app/🟦️.ts";
import { useRequestServiceOperationState } from "./🟢️request-service-operation/🟦️.ts";

export type ShellCoreAfterSpawnedSession = ShellCoreAfterContributionsAndRefresh & ReturnType<typeof useSpawnedSession>;

//#region FrameworkOsShell
/**
 * 🌱️ Seeds the canvas of the focused program, grabs focus for newly opened programs and publishes the service contributions of spawned sessions.
 *
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
 *
 * ⌨️ A program the user just opened takes keyboard focus once its active window has mounted — its dock tabpanel,
 * the same stop a pointer click lands on — but only while nothing else holds focus. Opening a program from the
 * command palette used to leave focus on `<body>`, so the next Tab restarted at the navbar instead of inside the
 * program the user asked for (WCAG 2.2 SC 2.4.3, measured inside `s`, ticket 26/09/23 S15).
 */
export function useSpawnedSession(core: ShellCoreAfterContributionsAndRefresh) {
  const { session, seededFocusedProgramKeyRef, focusedProgramKey, activeToolIdRef, writeToolRegister, focusedSpawnedId, focusedApp, sessionLayoutBeforeSpawnRef, shellLayout, activeWindowIdRef, appLabelsOverlay, uiTerminology, uiLocale, dispatch, scope, loadedPluginsRef, openDocumentSessionsRef, closeDocumentRef, documentAttachmentLanesRef, identity, sessionRef, shellSessionLaneRef, refreshUi, reportRefreshFault, establishPrimaryWithShardRetry, pluginSupervisorByIdRef, sessionHumanKey, hostMode, hostApp, setSurfaceSwitchBusy, sessionSwitchGateRef, loadedPlugins, sessionWorkRef, sealedInstancesRef, openDocumentRef, extraWindowInstancesRef, extraWindowCounterRef, landingAppId, rememberOpenSpaceId, openInstanceIdRef, showTransientNoticeRef, documentServices, documentServiceInventoryRef, backboneWorkerRef, inferencePortOwnerRef, inferencePortOpeningRef, verifiedSessionAuthorityRef, identityRef, retiredSessionDocumentOwnersRef, inferencePortEpochRef, ensureBackboneWorker, inferencePortAuthorityRef, browserActorUiByRuntimeKeyRef, shellStateRef, applyHistoryPatch, applyLeftoverInteractionView, leftoverReplaceRefreshBodiesV1, refreshSpawnedUi } = useDocumentServicesState({ core });
  const { retireSessionInstance, updateSpacePanel } = useUpdateSpacePanelState({ session, seededFocusedProgramKeyRef, focusedProgramKey, activeToolIdRef, writeToolRegister, focusedSpawnedId, focusedApp, sessionLayoutBeforeSpawnRef, shellLayout, activeWindowIdRef, appLabelsOverlay, uiTerminology, uiLocale, dispatch, scope, loadedPluginsRef, openDocumentSessionsRef, closeDocumentRef, documentAttachmentLanesRef, identity, sessionRef, shellSessionLaneRef, refreshUi, reportRefreshFault, establishPrimaryWithShardRetry, pluginSupervisorByIdRef, sessionHumanKey });
  const { switchToPluginApp, ensureSpawnedPlugin } = useSwitchPluginAppState({ hostMode, hostApp, setSurfaceSwitchBusy, sessionSwitchGateRef, session, loadedPlugins, sessionWorkRef, sealedInstancesRef, loadedPluginsRef, sessionRef, openDocumentSessionsRef, openDocumentRef, closeDocumentRef, documentAttachmentLanesRef, retireSessionInstance, dispatch, appLabelsOverlay, uiTerminology, uiLocale, extraWindowInstancesRef, extraWindowCounterRef, landingAppId, rememberOpenSpaceId, openInstanceIdRef, refreshUi, showTransientNoticeRef, documentServices, documentServiceInventoryRef, backboneWorkerRef });
  const { requestServiceOperation, directBrowserActorForSession, dispatchDirectBrowserActorCommand } = useRequestServiceOperationState({ documentServices, loadedPluginsRef, inferencePortOwnerRef, inferencePortOpeningRef, openDocumentSessionsRef, verifiedSessionAuthorityRef, identityRef, retiredSessionDocumentOwnersRef, inferencePortEpochRef, ensureBackboneWorker, inferencePortAuthorityRef, dispatch, browserActorUiByRuntimeKeyRef, shellStateRef, applyHistoryPatch, applyLeftoverInteractionView, refreshUi, leftoverReplaceRefreshBodiesV1 });
  return {
    refreshSpawnedUi, updateSpacePanel, switchToPluginApp, ensureSpawnedPlugin, requestServiceOperation,
    directBrowserActorForSession, dispatchDirectBrowserActorCommand
  };
}

//#endregion FrameworkOsShell
