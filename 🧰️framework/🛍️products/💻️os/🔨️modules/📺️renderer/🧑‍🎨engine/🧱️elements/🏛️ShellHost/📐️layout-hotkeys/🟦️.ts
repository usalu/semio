import type { ShellCoreAfterTutorialOrchestration } from "../🎥️tutorial-orchestration/🟦️.ts";
import { useScopeState } from "./🔶️scope/🟦️.ts";
import { useApplyNamedLayoutState } from "./🔸️apply-named-layout/🟦️.ts";
import { useHandleTemplateDropState } from "./🟠️handle-template-drop/🟦️.ts";

export type ShellCoreAfterDirectoryLaneAndLayout = ShellCoreAfterTutorialOrchestration & ReturnType<typeof useDirectoryLaneAndLayout>;
export function useDirectoryLaneAndLayout(core: ShellCoreAfterTutorialOrchestration) {
  const { canGoForward, goForward, uiKeybindingOverrides, canGoUp, goUp, dispatch, session, appLabelsOverlay, uiTerminology, uiLocale, extraWindowInstancesRef, extraWindowCounterRef, refreshUi, reportRefreshFault, sessionRef, loadedPluginsRef, openDocumentSessionsRef, sharedSpaceAccessRef, showTransientNoticeRef, uiLocaleRef, switchToPluginApp, focusedSpawnedId, noteShellCommand, focusedApp, uiNamedLayouts, shellLayout, extraWindowInstances, commitUiPreferenceRef } = useScopeState({ core });
  const { applyNamedLayout, applyModeChange, switchToSessionRole } = useApplyNamedLayoutState({ canGoForward, goForward, uiKeybindingOverrides, canGoUp, goUp, dispatch, session, appLabelsOverlay, uiTerminology, uiLocale, extraWindowInstancesRef, extraWindowCounterRef, refreshUi, reportRefreshFault, sessionRef, loadedPluginsRef, openDocumentSessionsRef, sharedSpaceAccessRef, showTransientNoticeRef, uiLocaleRef, switchToPluginApp });
  const { handleTemplateDrop, displayHostRef, displayHost } = useHandleTemplateDropState({ session, focusedSpawnedId, extraWindowCounterRef, appLabelsOverlay, uiTerminology, uiLocale, extraWindowInstancesRef, dispatch, refreshUi, reportRefreshFault, noteShellCommand, focusedApp, uiNamedLayouts, shellLayout, extraWindowInstances, applyNamedLayout, commitUiPreferenceRef });
  return { applyModeChange, switchToSessionRole, handleTemplateDrop, displayHostRef, displayHost };
}
