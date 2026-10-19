/** 🎥️ Tutorial clock, run and recorder state, applying slices to the shell, seeking, play, pause, start, stop, recording and chapter markers.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useTutorialOrchestration` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { shellDialogOriginIsCurrentV1 } from "../🗨️dialog-origin/🟦️.ts";
import { OwnedTutorialRunV1, runPausedTutorialSeekV1, tutorialHistoryStepInverseV1, type TutorialInverseV1 } from "../🗨️dialog-origin/🎥️tutorial/🟦️.ts";
import { useCallback, useEffect, useMemo, useRef } from "react";
import { TUTORIAL_CONVERGE_MS, type TutorialCameraState, type TutorialDocumentEventKind } from "@semio-tech/framework";
import { encodeMutationEnvelopesPack } from "@semio-tech/framework/packs";
import { type DocumentArchivePack } from "@semio-tech/framework/wire-contracts";
import { type LocalInteractionState, type MutationEnvelope } from "@semio-tech/framework/replication";
// #region 🔌️Adapters
import {
  CELEBRATE_STAMP_DURATION_MS,
  celebrateElements,
  composeTutorialUi,
  createTutorialClock,
  elementIdSelector,
  getTutorialCameraDriver,
  interpolateTutorialCamera,
  tutorialCameraAt,
  type TutorialChapterMarker,
  type TutorialClock,
  tutorialSlice,
  type TutorialSlice,
  validateTutorial
} from "@semio-tech/ui-react";
import { type ActiveSession } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";
import {
  applyTutorialUiChangeToShell,
  applyTutorialUiSnapshotToShell,
  EMPTY_SHELL_HISTORY_PROJECTION_V1,
  captureTutorialUiSnapshot,
  type TutorialUiBridgeContext
} from "../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { downloadMediaExport } from "../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { resolveManifestLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { sessionWindowInstances } from "../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
// #endregion 🔌️Adapters
import { tutorialInteractionSelectionActions, TutorialRecorder } from "../🎬️tutorial-recorder/🟦️.tsx";
import { programHistoryKeyV1, programHistoryProjectionV1 } from "../🪟️spawned-program/🟦️.ts";
import type { ShellCoreAfterShellActions } from "../🖱️shell-actions/🟦️.ts";

export type ShellCoreAfterTutorialOrchestration = ShellCoreAfterShellActions & ReturnType<typeof useTutorialOrchestration>;

export function useTutorialOrchestration(core: ShellCoreAfterShellActions) {
  const {
    scope, shellState, dispatch, localInteraction, loadedPlugins, session, uiLocale, uiTerminology, appLabelsOverlay,
    activeTutorialId, tutorialPlaying, tutorialRate, tutorialRecording, tutorialDeviated, extraWindowInstancesRef,
    sessionRef, captureDialogOrigin, captureEffectOwner, isCurrentEffectOwner, documentArchivePorts, historyStore, makeOwnedDialog,
    activeTutorials, startTutorialRef, stopTutorialRef, toggleTutorialRecordingRef, tutorialDrivenRef,
    tutorialPlayingRef, tutorialRecorderRef, tutorialRecorderStartingRef, shellStateRef, publishLocalInteraction,
    refreshUi, onActionRef
  } = core;
  //#region FrameworkOsShell
  //#region 🎥️TutorialOrchestration
  /** ⏱️ Real-time throttle for the director's UI/document/event application (~10Hz) — camera stays
   * smooth every clock tick regardless (see the `subscribe` callback below). */
  const TUTORIAL_DIRECTOR_TICK_MS = 90;

  const activeTutorial = useMemo(() => activeTutorials.find((tutorial) => tutorial.id === activeTutorialId) ?? null, [activeTutorials, activeTutorialId]);

  const tutorialClockRef = useRef<TutorialClock | null>(null);
  core.tutorialClockRef = tutorialClockRef;
  if (!tutorialClockRef.current) tutorialClockRef.current = createTutorialClock(activeTutorial?.durationMs ?? 0);
  const tutorialClock = tutorialClockRef.current;
  useEffect(() => () => {
    const run = tutorialRunRef.current;
    tutorialRunRef.current = null;
    tutorialTransitionEpochRef.current += 1;
    tutorialDrivenRef.current.retire();
    void run?.stop().catch((error) => undefined);
    tutorialClockRef.current?.dispose();
  }, []);
  useEffect(() => {
    tutorialClock.setDurationMs(activeTutorial?.durationMs ?? 0);
  }, [activeTutorial?.durationMs, tutorialClock]);
  useEffect(() => {
    tutorialClock.setRate(tutorialRate);
  }, [tutorialRate, tutorialClock]);
  useEffect(() => {
    if (tutorialPlaying && !tutorialDrivenRef.current.busy) tutorialClock.play();
    else tutorialClock.pause();
  }, [tutorialPlaying, tutorialClock]);

  const publishTutorialInteractionSelection = useCallback((selection: LocalInteractionState["selection"]) => {
    const active = sessionRef.current;
    if (!active || !tutorialRunRef.current?.ready) return;
    for (const action of tutorialInteractionSelectionActions(active.app.controllerId, selection)) onActionRef.current(action);
  }, []);
  const tutorialRunRef = useRef<OwnedTutorialRunV1<TutorialInverseV1> | null>(null);
  core.tutorialRunRef = tutorialRunRef;
  const tutorialTransitionEpochRef = useRef(0);
  core.tutorialTransitionEpochRef = tutorialTransitionEpochRef;
  const tutorialInitializingRunRef = useRef<OwnedTutorialRunV1<TutorialInverseV1> | null>(null);
  const uiBridgeCtxRef = useRef<TutorialUiBridgeContext>({
    session,
    restoreDialog: (dialogId, seedArgs) => {
      const run = tutorialRunRef.current;
      return run?.ready ? makeOwnedDialog(dialogId, run.origin, seedArgs) : null;
    },
    appLabelsOverlay,
    terminology: uiTerminology,
    locale: uiLocale,
    interactionSelection: () => localInteraction.get().selection,
    publishInteractionSelection: publishTutorialInteractionSelection,
  });
  uiBridgeCtxRef.current = {
    session,
    restoreDialog: (dialogId, seedArgs) => {
      const run = tutorialRunRef.current;
      return run?.ready ? makeOwnedDialog(dialogId, run.origin, seedArgs) : null;
    },
    appLabelsOverlay,
    terminology: uiTerminology,
    locale: uiLocale,
    interactionSelection: () => localInteraction.get().selection,
    publishInteractionSelection: publishTutorialInteractionSelection,
  };

  /** ⏱️ Playhead (ms) the director/seek last applied document/UI tracks up to — the "from" side of the
   * next `tutorialSlice(def, from, to)` call. Reset to 0 on sandbox (re)start. */
  const tutorialLastAppliedMsRef = useRef(0);
  useEffect(() => {
    const run = tutorialRunRef.current;
    if (run === null) {
      if (activeTutorialId !== null) dispatch({ type: "SET_TUTORIAL", value: null });
      return;
    }
    if (!run.isCurrent() || activeTutorialId !== run.tutorialId || session === null) {
      tutorialClock.pause();
      tutorialDrivenRef.current.retire();
      if (activeTutorialId !== null) dispatch({ type: "SET_TUTORIAL", value: null });
      void run.stop().catch((error) => undefined).finally(() => {
        if (tutorialRunRef.current !== run) return;
        tutorialRunRef.current = null;
      });
      return;
    }
    if (tutorialInitializingRunRef.current === run) return;
    const def = activeTutorials.find((tutorial) => tutorial.id === run.tutorialId);
    if (def === undefined) {
      tutorialDrivenRef.current.retire();
      void run.stop().catch((error) => undefined);
      dispatch({ type: "SET_TUTORIAL", value: null });
      return;
    }
    tutorialInitializingRunRef.current = run;
    const driveToken = tutorialDrivenRef.current.claim();
    void (async () => {
      if (!await run.start() || tutorialRunRef.current !== run || !run.isCurrent() || !tutorialDrivenRef.current.accepts(driveToken)) return;
      if (def.base.exampleId) dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: def.base.exampleId });
      applyTutorialUiSnapshotToShell(dispatch, def.base.ui, uiBridgeCtxRef.current);
      for (const cameraKeyframe of def.base.cameras) getTutorialCameraDriver(cameraKeyframe.windowId)?.set(cameraKeyframe.camera);
      tutorialLastAppliedMsRef.current = 0;
      tutorialClock.seek(0);
      await refreshUi(session, { kind: "full" });
    })().catch((error) => {
      if (tutorialRunRef.current === run) dispatch({ type: "SET_TUTORIAL", value: null });
    }).finally(() => {
      tutorialDrivenRef.current.release(driveToken);
    });
  }, [activeTutorialId, activeTutorials, session, loadedPlugins, tutorialClock, refreshUi]);

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
          // 🚧️ `kind.documentDsl`/`kind.previousDsl` is authored DSL/JSON document TEXT — the channel
          // only carries binary pack/spr containers (`AppCommand::LoadDocumentArchive`), and no TS-side
          // text→pack encoder exists (a separate, much larger work package), so a `Load` history entry
          // cannot replay through the plugin bridge; the UI/camera/event tracks alongside it still do.
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

  // 🎬️ Director: one subscription to the clock's rAF-driven ticks. Camera interpolation applies every
  // tick (smooth); UI/document/event application throttles to `TUTORIAL_DIRECTOR_TICK_MS`.
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
            // 🚧️ `kind.documentDsl`/`kind.previousDsl` is authored DSL/JSON document TEXT — the channel
            // only carries binary pack/spr containers (`AppCommand::LoadDocumentArchive`), and no TS-side
            // text→pack encoder exists (a separate, much larger work package), so a `Load` history
            // entry cannot replay through the plugin bridge on a seek either.
            documentTouched = true;
          }
          // 🚧️ Undo/Redo/Checkpoint/CheckoutCheckpoint/SwitchAlternative crossings mid-seek are an honest
          // scope cut here (replaying a crossed history op out of its natural live-dispatch order is
          // ambiguous without more VCS-side infrastructure) — the director's per-tick forward playback
          // above still applies them correctly; only a large scrub jumping OVER one of these entries misses it.
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

  /** ▶️ Play/pause toggle — the deviation-converge path (design point 6): snaps document+UI to the
   * composed target at the current playhead, tweens the camera over `TUTORIAL_CONVERGE_MS` (real-time,
   * rate-independent) from each window's LIVE pose to its target pose, then resumes the clock. */
  const playPauseTutorial = useCallback(() => {
    if (!activeTutorial) return;
    const run = tutorialRunRef.current;
    if (run === null || run.tutorialId !== activeTutorial.id || !run.ready) return;
    if (tutorialDrivenRef.current.busy) {
      dispatch({ type: "SET_TUTORIAL_PLAYING", value: !tutorialPlaying });
      return;
    }
    if (tutorialPlaying) {
      dispatch({ type: "SET_TUTORIAL_PLAYING", value: false });
      return;
    }
    if (tutorialDeviated && session) {
      const def = activeTutorial;
      const atMs = tutorialClock.getTimeMs();
      const driveToken = tutorialDrivenRef.current.claim();
      applyTutorialUiSnapshotToShell(dispatch, composeTutorialUi(def, atMs), uiBridgeCtxRef.current);
      const cameraWindowIds = new Set([...def.base.cameras, ...def.tracks.camera].map((keyframe) => keyframe.windowId));
      const startPoseByWindow = new Map<string, TutorialCameraState>();
      for (const windowId of cameraWindowIds) {
        const live = getTutorialCameraDriver(windowId)?.get();
        if (live) startPoseByWindow.set(windowId, live);
      }
      const startedAt = performance.now();
      const tween = (now: number) => {
        if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken)) {
          tutorialDrivenRef.current.release(driveToken);
          return;
        }
        const progress = Math.min(1, (now - startedAt) / TUTORIAL_CONVERGE_MS);
        for (const windowId of cameraWindowIds) {
          const targetPose = tutorialCameraAt(def, windowId, atMs);
          if (!targetPose) continue;
          const driver = getTutorialCameraDriver(windowId);
          if (!driver) continue;
          const startPose = startPoseByWindow.get(windowId);
          if (startPose && startPose.kind === targetPose.kind) {
            driver.set(interpolateTutorialCamera({ at: 0, windowId, camera: startPose, easing: "linear" }, { at: TUTORIAL_CONVERGE_MS, windowId, camera: targetPose, easing: "linear" }, progress * TUTORIAL_CONVERGE_MS));
          } else {
            driver.set(targetPose);
          }
        }
        if (progress < 1) requestAnimationFrame(tween);
        else {
          tutorialDrivenRef.current.release(driveToken);
          dispatch({ type: "SET_TUTORIAL_DEVIATED", value: false });
          dispatch({ type: "SET_TUTORIAL_PLAYING", value: true });
        }
      };
      requestAnimationFrame(tween);
      return;
    }
    dispatch({ type: "SET_TUTORIAL_PLAYING", value: true });
  }, [activeTutorial, tutorialPlaying, tutorialDeviated, session, tutorialClock]);

  const startTutorial = useCallback(
    (tutorialId: string) => {
      if (!session || !activeTutorials.some((tutorial) => tutorial.id === tutorialId)) return;
      const origin = captureDialogOrigin(session);
      const owner = captureEffectOwner(session, origin);
      if (origin === null || !isCurrentEffectOwner(owner) || owner.plugin === null) return;
      const plugin = owner.plugin;
      const epoch = ++tutorialTransitionEpochRef.current;
      tutorialClock.pause();
      tutorialDrivenRef.current.retire();
      dispatch({ type: "SET_TUTORIAL", value: null });
      void (async () => {
        const previous = tutorialRunRef.current;
        await previous?.stop().catch((error) => undefined);
        if (tutorialRunRef.current === previous) tutorialRunRef.current = null;
        if (epoch !== tutorialTransitionEpochRef.current || !isCurrentEffectOwner(owner)) return;
        const run: OwnedTutorialRunV1<TutorialInverseV1> = new OwnedTutorialRunV1(tutorialId, origin, (): boolean => tutorialRunRef.current === run && isCurrentEffectOwner(owner), {
          drain: () => tutorialDrivenRef.current.drain(),
          revert: async (inverse) => {
            if (!isCurrentEffectOwner(owner)) return;
            if (inverse.kind === "mutations") {
              if (plugin.applyMutations) await plugin.applyMutations(session.instanceId, encodeMutationEnvelopesPack(inverse.envelopes));
            } else if (inverse.kind === "history") await onActionRef.current({ controllerId: session.app.controllerId, action: inverse.action, ...(inverse.args === undefined ? {} : { args: inverse.args }) }, run.origin, true);
          },
          settle: async (unreverted) => {
            if (!isCurrentEffectOwner(owner)) return;
            for (const reason of unreverted) console.warn(`[tutorial] left in place: ${reason}`);
            const ports = documentArchivePorts(plugin, session, () => isCurrentEffectOwner(owner));
            await ports.history();
            await ports.refresh();
          },
        });
        tutorialRunRef.current = run;
        tutorialInitializingRunRef.current = null;
        dispatch({ type: "SET_TUTORIAL", value: tutorialId });
      })().catch((error) => undefined);
    },
    [activeTutorials, session, captureDialogOrigin, captureEffectOwner, isCurrentEffectOwner, documentArchivePorts, tutorialClock],
  );
  const stopTutorial = useCallback(() => {
    tutorialTransitionEpochRef.current += 1;
    tutorialClock.pause();
    tutorialDrivenRef.current.retire();
    void tutorialRunRef.current?.stop().catch((error) => undefined);
    dispatch({ type: "SET_TUTORIAL", value: null });
  }, [tutorialClock]);

  /** ⏺️ Arms/disarms `TutorialRecorder` against the LIVE (never sandboxed) document — a recording IS the
   * user's work. On stop: light `validateTutorial` sanity check, then serialize + trigger a browser
   * download, matching the repo's existing media-export download pattern. */
  const toggleTutorialRecording = useCallback(() => {
    if (!session) return;
    const recorder = tutorialRecorderRef.current;
    if (recorder) {
      tutorialRecorderRef.current = null;
      const id = `recorded-${session.app.id}-${Date.now()}`;
      const def = recorder.build(id, `${session.app.id} recording`);
      const validationError = validateTutorial(def);
      const json = JSON.stringify(def, null, 2);
      downloadMediaExport(`tutorial-${session.app.id}-${Date.now()}.ops`, "text/plain", json);
      dispatch({ type: "SET_TUTORIAL_RECORDING", value: false });
      return;
    }
    if (tutorialRecorderStartingRef.current) return;
    // 🚧️ `TutorialRecorder`'s base document is authored DSL/JSON TEXT — the channel only carries
    // binary pack/spr containers, and no TS-side pack→text decoder exists (deliberately out of scope
    // for `🔖️PackValueCodec`), so a recording's `base.documentDsl` fixture can't be captured from the
    // live channel; its UI/camera/event tracks still capture faithfully.
    const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    tutorialRecorderStartingRef.current = true;
    void publishLocalInteraction(plugin, session.instanceId).then((interaction) => {
      const active = sessionRef.current;
      if (!interaction || !active || active.pluginId !== session.pluginId || active.instanceId !== session.instanceId) return;
      tutorialRecorderRef.current = new TutorialRecorder(captureTutorialUiSnapshot(shellStateRef.current, localInteraction.get(), active), null);
      dispatch({ type: "SET_TUTORIAL_RECORDING", value: true });
    }).catch((captureError) => undefined).finally(() => {
      tutorialRecorderStartingRef.current = false;
    });
  }, [loadedPlugins, publishLocalInteraction, session]);

  useEffect(() => {
    startTutorialRef.current = startTutorial;
    stopTutorialRef.current = stopTutorial;
    toggleTutorialRecordingRef.current = toggleTutorialRecording;
  }, [startTutorial, stopTutorial, toggleTutorialRecording]);

  // ⏺️ Recorder: UI-state diff on every `ShellState` change (catches panel-tab clicks/tree expands/etc.
  // that bypass `onAction`), a periodic full-snapshot keyframe every 5s, and a 10Hz epsilon-filtered
  // camera sampler per registered driver (world drags bypass `onAction` entirely).
  useEffect(() => {
    if (!tutorialRecording) return;
    const record = (): void => {
      tutorialRecorderRef.current?.recordUiDiff(captureTutorialUiSnapshot(shellState, localInteraction.get(), session));
    };
    record();
    return localInteraction.subscribe(record);
  }, [tutorialRecording, shellState, session, localInteraction]);

  useEffect(() => {
    if (!tutorialRecording || !session || typeof window === "undefined") return;
    const interval = window.setInterval(() => {
      tutorialRecorderRef.current?.recordSnapshot(captureTutorialUiSnapshot(shellStateRef.current, localInteraction.get(), session));
    }, 5000);
    return () => window.clearInterval(interval);
  }, [tutorialRecording, session]);

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
  //#endregion 🎥️TutorialOrchestration
  //#endregion FrameworkOsShell

  return {
    activeTutorial, tutorialClock, seekTutorial, playPauseTutorial, stopTutorial, toggleTutorialRecording,
    addTutorialChapter, tutorialChapterMarkers
  };
}
