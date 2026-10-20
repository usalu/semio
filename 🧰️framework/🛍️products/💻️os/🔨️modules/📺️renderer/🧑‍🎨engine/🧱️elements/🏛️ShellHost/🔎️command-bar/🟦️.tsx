import type { ShellCoreAfterCheckInAndSync } from "../✅️check-in-sync/🟦️.tsx";
import { useScopeState } from "./🔶️scope/🟦️.tsx";
import { useCommandState } from "./🔸️command/🟦️.tsx";

export type ShellCoreAfterCommandBar = ShellCoreAfterCheckInAndSync & ReturnType<typeof useCommandBar>;

//#region FrameworkOsShell
export function useCommandBar(core: ShellCoreAfterCheckInAndSync) {
  const { resolvedCommands, expandedCommandIdRef, dispatch, commandStagedArgsByCommandIdRef, startTutorialRef, toggleTutorialRecordingRef, scope, navigateHistory, commitUiPreference, dockLayoutStore, dockUiStateStore, locks, noteShellCommand, uiTerminology, session, tutorialRecordingRef, tutorialDrivenRef, tutorialRecorderRef, loadedPlugins, showTransientNotice, uiLocale, injectActiveUtility, captureDialogOrigin, isCurrentDialogOrigin, directBrowserActorForSession, dispatchDirectBrowserActorCommand, captureEffectOwner, isCurrentEffectOwner, applyHistoryPatch, applyLeftoverInteractionView, applyHostEffects, isViewerReadOnlyFault, isMutationRejectedFault, showMutationRejectedNotice, historyStore, uiTerminologyRef, replayShellOwnedCommandRef, mobile, commandCategoryList, focusedApp, focusedModeId, toolMeasuresByToolIdRef, onActionStable, focusedPluginManifest, exampleOptions, dispatchActiveExample, exampleSelectElement, modeSwitcherElement, roleSwitcherElement } = useScopeState({ core });
  const { onCommand, commandCategoryTabs, toolTabs } = useCommandState({ resolvedCommands, expandedCommandIdRef, dispatch, commandStagedArgsByCommandIdRef, startTutorialRef, toggleTutorialRecordingRef, scope, navigateHistory, commitUiPreference, dockLayoutStore, dockUiStateStore, locks, noteShellCommand, uiTerminology, session, tutorialRecordingRef, tutorialDrivenRef, tutorialRecorderRef, loadedPlugins, showTransientNotice, uiLocale, injectActiveUtility, captureDialogOrigin, isCurrentDialogOrigin, directBrowserActorForSession, dispatchDirectBrowserActorCommand, captureEffectOwner, isCurrentEffectOwner, applyHistoryPatch, applyLeftoverInteractionView, applyHostEffects, isViewerReadOnlyFault, isMutationRejectedFault, showMutationRejectedNotice, historyStore, uiTerminologyRef, replayShellOwnedCommandRef, mobile, commandCategoryList, focusedApp, focusedModeId, toolMeasuresByToolIdRef, onActionStable });
  return {
    focusedPluginManifest, exampleOptions, dispatchActiveExample, exampleSelectElement, modeSwitcherElement,
    roleSwitcherElement, resolvedCommands, onCommand, commandCategoryTabs, toolTabs
  };
}

//#endregion FrameworkOsShell
