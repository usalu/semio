/** 🔶️ The plugin filter phase of useCanvasAndFooter. */
import type { ShellCoreAfterPanelHosting } from "../../📋️panel-hosting/🟦️.tsx";

/** 🔷 Builds pluginFilter, documentServices, brand, scope of useCanvasAndFooter. */
export function buildPluginFilterState({ core }: { readonly core: ShellCoreAfterPanelHosting }) {
  const {
    pluginFilter, documentServices, brand, scope, hostMode, mobile, dispatch, loadedPlugins, pluginStatusById,
    pluginSupervisorById, session, error, sessionFault, uiLocale, uiTerminology, hostAppId, spacePrograms,
    appLabelsOverlay, actionPaneStagedArgsByKey, activeToolId, panels, treeOpenStates, activeWindowId,
    world3dWindowViewRegistry, extraWindowCounterRef, setShellContextMenu, extraWindowInstancesRef, openSpaceIdRef,
    uiDevice, agentBridge, browserActorUiByRuntimeKeyRef, reportRefreshFault, notFoundPath, navigateShellUri, registry,
    primaryPluginId, reloadPlugin, uninstallPlugin, panel, spawnedIds, focusedToolMeasuresByToolId, writeUtilityRegister,
    effectiveModeLayoutRef, contributionsPublisherRef, refreshUi, updateSpacePanel, onAction, noteShellCommand,
    applyModeChange, handleTemplateDrop, appRouter, routerFaultByPluginId, hasOpenArtifactSurfaces, keysByActionId,
    revealAgentApproval, presencePeers, timeTravelPeers, hubConnectionStatuses, hubSessionPresence, hubLink,
    openHubWorkspace, initialExampleReady, buildPanelSelectionProps, modeWindows, effectiveModeLayout,
    handleActiveWindowChange, handleModeLayoutChange
  } = core;
  return { hostMode, notFoundPath, navigateShellUri, primaryPluginId, pluginSupervisorById, dispatch, reloadPlugin, uninstallPlugin, error, sessionFault, session, initialExampleReady, uiTerminology, hostAppId, panel, onAction, openSpaceIdRef, loadedPlugins, appLabelsOverlay, uiLocale, applyModeChange, mobile, modeWindows, effectiveModeLayout, activeWindowId, handleActiveWindowChange, handleModeLayoutChange, handleTemplateDrop, noteShellCommand, effectiveModeLayoutRef, updateSpacePanel, world3dWindowViewRegistry, writeUtilityRegister, extraWindowInstancesRef, spawnedIds, extraWindowCounterRef, refreshUi, reportRefreshFault, uiDevice, buildPanelSelectionProps, brand, presencePeers, timeTravelPeers, agentBridge, revealAgentApproval, hubConnectionStatuses, hubSessionPresence, hubLink, openHubWorkspace, panels, treeOpenStates, activeToolId, focusedToolMeasuresByToolId, actionPaneStagedArgsByKey, scope, pluginFilter, registry, pluginStatusById, appRouter, spacePrograms, contributionsPublisherRef, documentServices, browserActorUiByRuntimeKeyRef, routerFaultByPluginId, keysByActionId, hasOpenArtifactSurfaces, setShellContextMenu };
}
