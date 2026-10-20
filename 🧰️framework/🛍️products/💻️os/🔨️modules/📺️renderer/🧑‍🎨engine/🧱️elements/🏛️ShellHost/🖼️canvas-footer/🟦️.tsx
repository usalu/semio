import type { ShellCoreAfterPanelHosting } from "../📋️panel-hosting/🟦️.tsx";
import { buildPluginFilterState } from "./🔶️plugin-filter/🟦️.tsx";
import { useCanvas } from "./🔸️canvas/🟦️.tsx";
import { useFooterItemsState } from "./🟠️footer-items/🟦️.tsx";
import { useBuildShellContext } from "./🟢️build-shell-context/🟦️.tsx";

export type ShellCoreAfterCanvasAndFooter = ShellCoreAfterPanelHosting & ReturnType<typeof useCanvasAndFooter>;

//#region FrameworkOsShell
/**
 * 🧰️ Builds the canvas and footer items and exposes the readiness beacon, the dev catalog probes and the shell context menu.
 *
 * 🚦️ Deterministic DOM beacon for headless smoke tests (e.g. Storybook's OS-shell plugin-boot matrix)
 * to wait on instead of screenshots/timeouts — set once a session resolves or errors, cleared on unmount.
 * Mirrored onto this shell's own `[data-shell-id]` scope root (`data-shell-ready`/`data-shell-error`/
 * `data-shell-not-found`) because the `document.documentElement` slot is a single global value and
 * cannot distinguish several `FrameworkOsShell` instances mounted on one page (e.g. the demonstrator).
 *
 * 🔬️ Dev-only mirror of the two facts a catalog-wide smoke cannot otherwise read from the DOM: which
 * programs this session can spawn, and which installed plugins ended up `failed`/`crashed` (the shell
 * only console-logs a non-primary install failure — see the streaming install effect). Shape is
 * {@link ShellCatalogProbe}; consumed by
 * `framework-os-dev verify catalog`. Beside it, `__semioOsInstalledContributions(instanceId)` reads the contributions
 * closure the host last pushed into that instance through `setContributions` ({@link createContributionsPublisher}'s
 * `installedFor`) — the witness that a topic-only extension reached its parent. Never defined in a production build.
 *
 * 🔬️ Dev/test-only read seam over the actual acknowledged Shell store. The caller supplies only
 * a scope; the returned closed projection contains no capability or mutation surface.
 *
 * 🧯️ One console record per plugin the router excluded — permanent, not a `[TRACE]` trace: an
 * excluded plugin installs cleanly, so this is the only signal outside the dev probe that its
 * surfaces are unroutable.
 */
export function useCanvasAndFooter(core: ShellCoreAfterPanelHosting) {
  const { hostMode, notFoundPath, navigateShellUri, primaryPluginId, pluginSupervisorById, dispatch, reloadPlugin, uninstallPlugin, error, sessionFault, session, initialExampleReady, uiTerminology, hostAppId, panel, onAction, openSpaceIdRef, loadedPlugins, appLabelsOverlay, uiLocale, applyModeChange, mobile, modeWindows, effectiveModeLayout, activeWindowId, handleActiveWindowChange, handleModeLayoutChange, handleTemplateDrop, noteShellCommand, effectiveModeLayoutRef, updateSpacePanel, world3dWindowViewRegistry, writeUtilityRegister, extraWindowInstancesRef, spawnedIds, extraWindowCounterRef, refreshUi, reportRefreshFault, uiDevice, buildPanelSelectionProps, brand, presencePeers, timeTravelPeers, agentBridge, revealAgentApproval, hubConnectionStatuses, hubSessionPresence, hubLink, openHubWorkspace, panels, treeOpenStates, activeToolId, focusedToolMeasuresByToolId, actionPaneStagedArgsByKey, scope, pluginFilter, registry, pluginStatusById, appRouter, spacePrograms, contributionsPublisherRef, documentServices, browserActorUiByRuntimeKeyRef, routerFaultByPluginId, keysByActionId, hasOpenArtifactSurfaces, setShellContextMenu } = buildPluginFilterState({ core });
  const { canvas } = useCanvas({ hostMode, notFoundPath, navigateShellUri, primaryPluginId, pluginSupervisorById, dispatch, reloadPlugin, uninstallPlugin, error, sessionFault, session, initialExampleReady, uiTerminology, hostAppId, panel, onAction, openSpaceIdRef, loadedPlugins, appLabelsOverlay, uiLocale, applyModeChange, mobile, modeWindows, effectiveModeLayout, activeWindowId, handleActiveWindowChange, handleModeLayoutChange, handleTemplateDrop, noteShellCommand, effectiveModeLayoutRef, updateSpacePanel, world3dWindowViewRegistry, writeUtilityRegister, extraWindowInstancesRef, spawnedIds, extraWindowCounterRef, refreshUi, reportRefreshFault });
  const { dispatchShellMenuAction, footerItems, buildPanelProps } = useFooterItemsState({ uiDevice, mobile, buildPanelSelectionProps, brand, uiLocale, presencePeers, timeTravelPeers, agentBridge, revealAgentApproval, hubConnectionStatuses, hubSessionPresence, hubLink, openHubWorkspace, panels, dispatch, treeOpenStates, activeToolId, focusedToolMeasuresByToolId, actionPaneStagedArgsByKey, scope, pluginFilter, hostMode, notFoundPath, error, session, initialExampleReady, registry, pluginStatusById, appRouter, spacePrograms, panel, contributionsPublisherRef, documentServices, browserActorUiByRuntimeKeyRef, routerFaultByPluginId, activeWindowId, onAction });
  const { buildShellContextMenuItems } = useBuildShellContext({ session, activeWindowId, appLabelsOverlay, uiTerminology, uiLocale, keysByActionId, hasOpenArtifactSurfaces, dispatchShellMenuAction, setShellContextMenu });
  return { canvas, footerItems, buildPanelProps, buildShellContextMenuItems };
}

//#endregion FrameworkOsShell
