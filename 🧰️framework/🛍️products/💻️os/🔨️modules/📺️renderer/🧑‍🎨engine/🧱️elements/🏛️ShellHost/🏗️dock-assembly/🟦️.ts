/** 🏗️ Default dock arrangement, dock persistence, tab and tree-unit drops, introduction overrides and the panel reveal effects.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useDockAssembly` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

// #region 🔌️Adapters
import type { ShellCoreAfterCommandBar } from "../🔎️command-bar/🟦️.tsx";
import { useDockModel, useDockPersistence } from "./🧱️dock-model-and-persistence/🟦️.ts";
import { useIntroductionRevealEffects, useOverrideAnchorsAndIntroductionIds } from "./🎓️override-anchors-and-introduction/🟦️.ts";
import { usePanelPathsAndOverrideReveals, useToolTabSelectionEffects } from "./🪟️panel-reveal-and-tool-tabs/🟦️.ts";
// #endregion 🔌️Adapters

export type ShellCoreAfterDockAssembly = ShellCoreAfterCommandBar & ReturnType<typeof useDockAssembly>;
export function useDockAssembly(core: ShellCoreAfterCommandBar) {
  //#region FrameworkOsShell
  //#region 🧭️DockAssembly — default four-corner arrangement (the two middle anchors start empty save the command palette in bottom-middle) + persisted-override reconciliation + drag-and-drop wiring.
  const { dock, defaultDock, dockLayoutStore, dispatch, dockUiStateStore, panels, panelPathMemory, treeOpenStates, noteShellCommand, hostMode, session, hostAppId, panel, hostCatalogueTabId, activeIntroduction, introductionStepIndex, appLabelsOverlay, uiTerminology, uiLocale, windowMeasuresByWindowId, extraWindowInstances, focusedToolMeasuresByToolId, activeToolIdRef, onActionStable, mobile, mobilePanelTabs, completeIntroductionInteraction, refreshHistorySnapshot, focusedProgram, mobilePanelPath, toolRunPanelNode, focusedTimeTravel, scope, applyAgentShellCommandRef, agentDeferredPlansRef, agentArtifactRouteRef, sessionRef, spawnedAppsRef, loadedPluginsRef, onActionRef, activeWindowIdRef, leftoverInspectionIdsRef, leftoverInspectionEpoch, uiRefreshCacheRef, refreshUi, reportRefreshFault, activeWindowId, spawnedIdsRef, activeToolId, toolGeneration } = useDockModel({ core });
  const { handleTabDockDrop, handleTreeUnitDockDrop } = useDockPersistence({ dock, defaultDock, dockLayoutStore, dispatch, dockUiStateStore, panels, panelPathMemory, treeOpenStates, noteShellCommand });
  const { introductionToolId, introductionToolPickIds, activeIntroductionStep, introductionPanelTabId, introductionPanelTabAnchor, hostOverrideTabId, studioOverrideAnchor, detailsOverrideTabId, detailsOverrideAnchor, introductionUtilityId, introductionActionWindowSegment, introductionUtilityWindowId, introductionMeasureWindowId } = useOverrideAnchorsAndIntroductionIds({ hostMode, session, hostAppId, panel, hostCatalogueTabId, dock, activeIntroduction, introductionStepIndex, appLabelsOverlay, uiTerminology, uiLocale, windowMeasuresByWindowId, extraWindowInstances, focusedToolMeasuresByToolId });
  useIntroductionRevealEffects({ introductionToolId, session, activeToolIdRef, onActionStable, introductionToolPickIds, activeIntroductionStep, mobile, mobilePanelTabs, dispatch, dock, introductionPanelTabId, introductionPanelTabAnchor, panels, completeIntroductionInteraction, treeOpenStates });
  const { panelActivePaths } = usePanelPathsAndOverrideReveals({ dock, panels, session, refreshHistorySnapshot, focusedProgram, hostOverrideTabId, studioOverrideAnchor, mobile, mobilePanelPath, mobilePanelTabs, dispatch, toolRunPanelNode, focusedTimeTravel, scope, applyAgentShellCommandRef });
  useToolTabSelectionEffects({ agentDeferredPlansRef, agentArtifactRouteRef, sessionRef, spawnedAppsRef, loadedPluginsRef, onActionRef, activeWindowIdRef, leftoverInspectionIdsRef, leftoverInspectionEpoch, dock, panels, dispatch, uiRefreshCacheRef, refreshUi, reportRefreshFault, activeWindowId, spawnedIdsRef, detailsOverrideTabId, detailsOverrideAnchor, studioOverrideAnchor, mobile, mobilePanelPath, mobilePanelTabs, session, panelActivePaths, activeToolId, onActionStable, toolGeneration });
  //#endregion 🧭️DockAssembly
  //#endregion FrameworkOsShell
  return {
    dock, mobilePanelTabs, handleTabDockDrop, handleTreeUnitDockDrop, introductionUtilityId,
    introductionActionWindowSegment, introductionUtilityWindowId, introductionMeasureWindowId, panelActivePaths
  };
}
