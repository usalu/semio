import type { ShellCoreAfterDockAssembly } from "../🏗️dock-assembly/🟦️.ts";
import { useBrandState } from "./🔶️brand/🟦️.tsx";
import { useSearchItems } from "./🔸️search-items/🟦️.tsx";
import { useModeWindows } from "./🟠️mode-windows/🟦️.tsx";
import { useEffectiveModeLayoutState } from "./🟢️effective-mode-layout/🟦️.tsx";

export type ShellCoreAfterPanelHosting = ShellCoreAfterDockAssembly & ReturnType<typeof usePanelHosting>;

//#region FrameworkOsShell
export function usePanelHosting(core: ShellCoreAfterDockAssembly) {
  const { session, appLabelsOverlay, uiTerminology, uiLocale, onAction, dispatch, resolvedCommands, mobile, onCommand, hostMode, panel, spacePrograms, loadedPlugins, spawnProgram, hostControllerId, activeWindowId, actionPaneExpandedByWindowId, actionPaneStagedArgsByKey, activeUtilityByWindowId, introductionActionWindowSegment, actionPaneFoldedByWindowId, introductionUtilityWindowId, introductionMeasureWindowId, activeSpawnedEntry, activeSpawnedApp, focusedSpawnedId, localDocumentOwnerRegistry, world3dWindowViewRegistry, spawnedWindowEngagements, spawnedWindowMeasures, onActionStable, utilityGenerationFor, spawnedWindowActivityByWindowId, builtNodeStoreCacheRef, windowIconsById, timeTravelIndicator, introductionUtilityId, localPrograms, selectedSpaceArtifactKinds, onIntentStable, spawnedWindowFault, currentBrowserActorUi, builtNodeStoreFor, windowMeasuresByWindowId, windowEngagementsByWindowId, windowTitlesById, windowUiByWindowId, primaryWorld3dOwner, world3dWindowViews, instanceFault, windowTreeContext, currentLocalDocumentOwner, currentDocumentRuntimeKey, onBrowserActorIntent, extraWindowInstances, registerRevision, browserActorUiVersion, shellLayout, effectiveModeLayoutRef, mountedWindowIdsRef, sessionRef, unmountedSkippedWindowBodiesRef, refreshUi, dropForRetiredInstance, noteShellCommand, mobilePanel, initialExampleReady, buildPanelSelectionProps, navbarItems } = useBrandState({ core });
  const { searchItems } = useSearchItems({ session, appLabelsOverlay, uiTerminology, uiLocale, onAction, dispatch, resolvedCommands, mobile, onCommand, hostMode, panel, spacePrograms, loadedPlugins, spawnProgram, hostControllerId, activeWindowId });
  const { modeWindows } = useModeWindows({ session, loadedPlugins, actionPaneExpandedByWindowId, actionPaneStagedArgsByKey, activeUtilityByWindowId, introductionActionWindowSegment, actionPaneFoldedByWindowId, introductionUtilityWindowId, introductionMeasureWindowId, dispatch, hostMode, activeSpawnedEntry, activeSpawnedApp, focusedSpawnedId, localDocumentOwnerRegistry, world3dWindowViewRegistry, appLabelsOverlay, uiTerminology, uiLocale, spawnedWindowEngagements, spawnedWindowMeasures, onActionStable, utilityGenerationFor, spawnedWindowActivityByWindowId, builtNodeStoreCacheRef, windowIconsById, timeTravelIndicator, introductionUtilityId, localPrograms, selectedSpaceArtifactKinds, onIntentStable, spawnedWindowFault, currentBrowserActorUi, builtNodeStoreFor, windowMeasuresByWindowId, windowEngagementsByWindowId, windowTitlesById, windowUiByWindowId, primaryWorld3dOwner, world3dWindowViews, instanceFault, windowTreeContext, currentLocalDocumentOwner, currentDocumentRuntimeKey, onBrowserActorIntent, extraWindowInstances, registerRevision, browserActorUiVersion, panel });
  const { effectiveModeLayout, handleActiveWindowChange, handleModeLayoutChange } = useEffectiveModeLayoutState({ shellLayout, session, appLabelsOverlay, uiTerminology, uiLocale, effectiveModeLayoutRef, mountedWindowIdsRef, sessionRef, unmountedSkippedWindowBodiesRef, refreshUi, dropForRetiredInstance, dispatch, noteShellCommand });
  return {
    mobilePanel, initialExampleReady, buildPanelSelectionProps, navbarItems, searchItems, modeWindows,
    effectiveModeLayout, handleActiveWindowChange, handleModeLayoutChange
  };
}

//#endregion FrameworkOsShell
