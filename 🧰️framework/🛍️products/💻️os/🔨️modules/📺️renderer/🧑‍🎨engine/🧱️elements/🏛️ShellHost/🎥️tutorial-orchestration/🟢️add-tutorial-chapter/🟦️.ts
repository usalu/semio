/** 🟢️ The add tutorial chapter phase of useTutorialOrchestration. */
import { useCallback, useEffect, useMemo } from "react";
import { getTutorialCameraDriver, type TutorialChapterMarker } from "@semio-tech/ui-react";
import { resolveManifestLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { sessionWindowInstances } from "../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import type { useScopeState } from "../🔶️scope/🟦️.ts";

/** 🔵 Builds addTutorialChapter, tutorialChapterMarkers of useTutorialOrchestration. */
export function useAddTutorialChapterState({ tutorialRecording, session, tutorialRecorderRef, extraWindowInstancesRef, activeTutorial, uiTerminology, uiLocale }: Pick<ReturnType<typeof useScopeState>, "tutorialRecording" | "session" | "tutorialRecorderRef" | "extraWindowInstancesRef" | "activeTutorial" | "uiTerminology" | "uiLocale">) {
  useEffect(() => {
    if (!tutorialRecording || !session || typeof window === "undefined") return;
    const interval = window.setInterval(() => {
      const recorder = tutorialRecorderRef.current;
      if (!recorder) return;
      for (const instance of sessionWindowInstances(session.app, extraWindowInstancesRef.current)) {
        const pose = getTutorialCameraDriver(instance.id)?.get();
        if (pose) recorder.sampleCamera(instance.id, pose);
      }
    }, 100);
    return () => window.clearInterval(interval);
  }, [tutorialRecording, session]);

  const addTutorialChapter = useCallback(() => {
    tutorialRecorderRef.current?.addChapter();
  }, []);

  const tutorialChapterMarkers = useMemo(
    (): readonly TutorialChapterMarker[] => (activeTutorial ? activeTutorial.chapters.map((chapter) => ({ id: chapter.id, title: resolveManifestLabel(chapter.title, uiTerminology, uiLocale), atMs: chapter.at })) : []),
    [activeTutorial, uiTerminology, uiLocale],
  );
  return { addTutorialChapter, tutorialChapterMarkers };
}
