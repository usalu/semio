import type { ShellCoreAfterShellActions } from "../🖱️shell-actions/🟦️.ts";
import { useScopeState } from "./🔶️scope/🟦️.ts";
import { useApplyTutorialSliceState } from "./🔸️apply-tutorial-slice/🟦️.ts";
import { usePlayPauseTutorialState } from "./🟠️play-pause-tutorial/🟦️.ts";
import { useAddTutorialChapterState } from "./🟢️add-tutorial-chapter/🟦️.ts";

export type ShellCoreAfterTutorialOrchestration = ShellCoreAfterShellActions & ReturnType<typeof useTutorialOrchestration>;
export function useTutorialOrchestration(core: ShellCoreAfterShellActions) {
  const { tutorialRunRef, tutorialDrivenRef, captureDialogOrigin, dispatch, uiBridgeCtxRef, loadedPlugins, onActionRef, historyStore, scope, refreshUi, activeTutorial, session, tutorialClock, TUTORIAL_DIRECTOR_TICK_MS, tutorialLastAppliedMsRef, tutorialPlayingRef, tutorialPlaying, tutorialDeviated, activeTutorials, captureEffectOwner, isCurrentEffectOwner, tutorialTransitionEpochRef, documentArchivePorts, tutorialInitializingRunRef, tutorialRecorderRef, tutorialRecorderStartingRef, publishLocalInteraction, sessionRef, shellStateRef, localInteraction, startTutorialRef, stopTutorialRef, toggleTutorialRecordingRef, tutorialRecording, shellState, extraWindowInstancesRef, uiTerminology, uiLocale } = useScopeState({ core });
  const { seekTutorial } = useApplyTutorialSliceState({ tutorialRunRef, tutorialDrivenRef, captureDialogOrigin, dispatch, uiBridgeCtxRef, loadedPlugins, onActionRef, historyStore, scope, refreshUi, activeTutorial, session, tutorialClock, TUTORIAL_DIRECTOR_TICK_MS, tutorialLastAppliedMsRef, tutorialPlayingRef });
  const { playPauseTutorial, stopTutorial, toggleTutorialRecording } = usePlayPauseTutorialState({ activeTutorial, tutorialRunRef, tutorialDrivenRef, dispatch, tutorialPlaying, tutorialDeviated, session, tutorialClock, uiBridgeCtxRef, activeTutorials, captureDialogOrigin, captureEffectOwner, isCurrentEffectOwner, tutorialTransitionEpochRef, onActionRef, documentArchivePorts, tutorialInitializingRunRef, tutorialRecorderRef, tutorialRecorderStartingRef, loadedPlugins, publishLocalInteraction, sessionRef, shellStateRef, localInteraction, startTutorialRef, stopTutorialRef, toggleTutorialRecordingRef, tutorialRecording, shellState });
  const { addTutorialChapter, tutorialChapterMarkers } = useAddTutorialChapterState({ tutorialRecording, session, tutorialRecorderRef, extraWindowInstancesRef, activeTutorial, uiTerminology, uiLocale });
  return {
    activeTutorial, tutorialClock, seekTutorial, playPauseTutorial, stopTutorial, toggleTutorialRecording,
    addTutorialChapter, tutorialChapterMarkers
  };
}
