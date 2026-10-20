/** 🔸️ The apply tutorial slice phase of useTutorialOrchestration. */
import { shellDialogOriginIsCurrentV1 } from "../../🗨️dialog-origin/🟦️.ts";
import { OwnedTutorialRunV1, runPausedTutorialSeekV1, tutorialHistoryStepInverseV1, type TutorialInverseV1 } from "../../🗨️dialog-origin/🎥️tutorial/🟦️.ts";
import { useCallback, useEffect } from "react";
import { type TutorialDocumentEventKind } from "@semio-tech/framework";
import { encodeMutationEnvelopesPack } from "@semio-tech/framework/packs";
import { type MutationEnvelope } from "@semio-tech/framework/replication";
import { CELEBRATE_STAMP_DURATION_MS, celebrateElements, composeTutorialUi, elementIdSelector, getTutorialCameraDriver, tutorialCameraAt, tutorialSlice, type TutorialSlice } from "@semio-tech/ui-react";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { applyTutorialUiChangeToShell, applyTutorialUiSnapshotToShell } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { EMPTY_SHELL_HISTORY_PROJECTION_V1 } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { programHistoryKeyV1, programHistoryProjectionV1 } from "../../🪟️spawned-program/🟦️.ts";
import type { useScopeState } from "../🔶️scope/🟦️.ts";

/** 🔹 Builds applyTutorialSliceToShell, seekTutorial of useTutorialOrchestration. */
export function useApplyTutorialSliceState({ tutorialRunRef, tutorialDrivenRef, captureDialogOrigin, dispatch, uiBridgeCtxRef, loadedPlugins, onActionRef, historyStore, scope, refreshUi, activeTutorial, session, tutorialClock, TUTORIAL_DIRECTOR_TICK_MS, tutorialLastAppliedMsRef, tutorialPlayingRef }: Pick<ReturnType<typeof useScopeState>, "tutorialRunRef" | "tutorialDrivenRef" | "captureDialogOrigin" | "dispatch" | "uiBridgeCtxRef" | "loadedPlugins" | "onActionRef" | "historyStore" | "scope" | "refreshUi" | "activeTutorial" | "session" | "tutorialClock" | "TUTORIAL_DIRECTOR_TICK_MS" | "tutorialLastAppliedMsRef" | "tutorialPlayingRef">) {
  /** 🎬️ Applies every entry of one `TutorialSlice` (a director tick or a seek span) onto the live
   * session — UI changes first, then document-track entries through the plugin bridge: `Edit` via
   * `applyMutations` (forward/backward per `slice.forward`), `Load` is DSL/JSON document text the
   * pack-only channel cannot replay (see the `kind.kind === "load"` branch below),
   * `Undo`/`Redo`/`Checkpoint`/`CheckoutCheckpoint`/`SwitchAlternative` via the SAME History-action
   * `onAction` funnel the app's own undo/redo buttons dispatch through (never a bespoke channel) — then
   * pulses any annotational event's target element via the existing `celebrateElements` vocabulary. */
  const applyTutorialSliceToShell = useCallback(
    async (slice: TutorialSlice, activeSession: ActiveSession, run: OwnedTutorialRunV1<TutorialInverseV1>, driveToken: number) => {
      if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken) || !shellDialogOriginIsCurrentV1(run.origin, captureDialogOrigin(activeSession))) return;
      for (const change of slice.uiChanges) applyTutorialUiChangeToShell(dispatch, change, uiBridgeCtxRef.current);
      const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === activeSession.pluginId)?.handle;
      let documentTouched = false;
      for (const documentEvent of slice.document) {
        if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken)) return;
        const kind: TutorialDocumentEventKind = documentEvent.kind;
        if (kind.kind === "edit") {
          documentTouched = true;
          const mutations = (slice.forward ? kind.forwards : kind.backwards) as readonly MutationEnvelope[];
          if (plugin?.applyMutations) {
            await plugin.applyMutations(activeSession.instanceId, encodeMutationEnvelopesPack(mutations));
            run.record(documentEvent, slice.forward, { kind: "mutations", envelopes: (slice.forward ? kind.backwards : kind.forwards) as readonly MutationEnvelope[] });
          }
        } else if (kind.kind === "load") {
          documentTouched = true;
        } else if (kind.kind === "undo") {
          await onActionRef.current({ controllerId: activeSession.app.controllerId, action: slice.forward ? "undo" : "redo" }, run.origin, true);
          run.record(documentEvent, slice.forward, { kind: "history", action: slice.forward ? "redo" : "undo" });
        } else if (kind.kind === "redo") {
          await onActionRef.current({ controllerId: activeSession.app.controllerId, action: slice.forward ? "redo" : "undo" }, run.origin, true);
          run.record(documentEvent, slice.forward, { kind: "history", action: slice.forward ? "undo" : "redo" });
        } else if (kind.kind === "checkpoint" || kind.kind === "checkoutCheckpoint" || kind.kind === "switchAlternative") {
          if (kind.kind === "checkpoint" && !slice.forward) continue;
          const stand = programHistoryProjectionV1(historyStore.get(), programHistoryKeyV1(activeSession), EMPTY_SHELL_HISTORY_PROJECTION_V1);
          const outcome = tutorialHistoryStepInverseV1(kind, { checkpointId: stand.currentCheckpointId, alternativeId: stand.activeAlternativeId });
          await onActionRef.current(
            kind.kind === "checkpoint"
              ? { controllerId: activeSession.app.controllerId, action: "commitCheckpoint" }
              : kind.kind === "checkoutCheckpoint"
                ? { controllerId: activeSession.app.controllerId, action: "checkoutCheckpoint", args: { checkpointId: kind.checkpointId } }
                : { controllerId: activeSession.app.controllerId, action: "switchAlternative", args: { alternativeId: kind.alternativeId } },
            run.origin,
            true,
          );
          if ("inverse" in outcome) run.record(documentEvent, true, outcome.inverse);
          else if ("unrevertable" in outcome) run.recordUnrevertable(outcome.unrevertable);
        }
      }
      for (const event of slice.events) {
        if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken)) return;
        const kind = event.kind;
        const targetId = kind.kind === "action" ? kind.action : kind.kind === "command" ? kind.command : undefined;
        if (targetId && scope.rootRef.current) celebrateElements(elementIdSelector(targetId), CELEBRATE_STAMP_DURATION_MS, scope.rootRef.current);
      }
      if (documentTouched && tutorialRunRef.current === run && run.ready && tutorialDrivenRef.current.accepts(driveToken)) await refreshUi(activeSession, { kind: "full" });
    },
    [historyStore, loadedPlugins, refreshUi],
  );

  useEffect(() => {
    const def = activeTutorial;
    if (!def || !session) return;
    const run = tutorialRunRef.current;
    if (run === null || run.tutorialId !== def.id) return;
    let lastHeavyTickAt = 0;
    const cameraWindowIds = new Set([...def.base.cameras, ...def.tracks.camera].map((keyframe) => keyframe.windowId));
    const unsubscribe = tutorialClock.subscribe(() => {
      if (tutorialRunRef.current !== run || !run.ready || tutorialDrivenRef.current.busy) return;
      const t = tutorialClock.getTimeMs();
      for (const windowId of cameraWindowIds) {
        const pose = tutorialCameraAt(def, windowId, t);
        if (pose) getTutorialCameraDriver(windowId)?.set(pose);
      }
      if (!tutorialClock.isPlaying()) return;
      const now = performance.now();
      if (now - lastHeavyTickAt < TUTORIAL_DIRECTOR_TICK_MS) return;
      lastHeavyTickAt = now;
      void tutorialDrivenRef.current.enqueue(() => tutorialRunRef.current === run && run.ready, async driveToken => {
        const from = tutorialLastAppliedMsRef.current;
        if (from === t) return;
        await applyTutorialSliceToShell(tutorialSlice(def, from, t), session, run, driveToken);
        if (tutorialRunRef.current === run && run.ready && tutorialDrivenRef.current.accepts(driveToken)) tutorialLastAppliedMsRef.current = t;
      }).catch((error) => {
        tutorialClock.pause();
      });
    });
    return unsubscribe;
  }, [activeTutorial, session, tutorialClock, applyTutorialSliceToShell]);

  /** ✂️ Seek/rebuild (design point 5): composes UI wholesale (never accumulates deltas across a seek —
   * mirrors the Rust `tutorial_slice` doc comment's own warning), applies the forward/backward document
   * span crossed since the last applied playhead, sets every camera exactly (no interpolation on a seek),
   * and moves the clock. */
  const seekTutorial = useCallback(
    (ms: number) => {
      const def = activeTutorial;
      if (!def || !session) return;
      const run = tutorialRunRef.current;
      if (run === null || run.tutorialId !== def.id || !run.ready) return;
      const clamped = Math.min(def.durationMs, Math.max(0, ms));
      void runPausedTutorialSeekV1(tutorialClock, tutorialDrivenRef.current, () => tutorialRunRef.current === run && run.ready, () => tutorialPlayingRef.current, async driveToken => {
        const from = tutorialLastAppliedMsRef.current;
        applyTutorialUiSnapshotToShell(dispatch, composeTutorialUi(def, clamped), uiBridgeCtxRef.current);
        const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
        const slice = tutorialSlice(def, from, clamped);
        let documentTouched = false;
        for (const documentEvent of slice.document) {
          if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken)) return;
          const kind: TutorialDocumentEventKind = documentEvent.kind;
          if (kind.kind === "edit") {
            documentTouched = true;
            const mutations = (slice.forward ? kind.forwards : kind.backwards) as readonly MutationEnvelope[];
            if (plugin?.applyMutations) {
              await plugin.applyMutations(session.instanceId, encodeMutationEnvelopesPack(mutations));
              run.record(documentEvent, slice.forward, { kind: "mutations", envelopes: (slice.forward ? kind.backwards : kind.forwards) as readonly MutationEnvelope[] });
            }
          } else if (kind.kind === "load") {
            documentTouched = true;
          }
        }
        if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken)) return;
        const cameraWindowIds = new Set([...def.base.cameras, ...def.tracks.camera].map((keyframe) => keyframe.windowId));
        for (const windowId of cameraWindowIds) {
          const pose = tutorialCameraAt(def, windowId, clamped);
          if (pose) getTutorialCameraDriver(windowId)?.set(pose);
        }
        tutorialLastAppliedMsRef.current = clamped;
        tutorialClock.seek(clamped);
        if (documentTouched) await refreshUi(session, { kind: "full" });
        if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken)) return;
      }).catch((error) => undefined);
    },
    [activeTutorial, session, loadedPlugins, tutorialClock, refreshUi],
  );
  return { seekTutorial };
}
