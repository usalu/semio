/** 🔀️ World3dHost paint and projection: the paint gesture, the camera change and navigate handlers, the projection window chrome and the projection content frame and fit. */
import { useCallback, useEffect, useMemo, useRef, useState } from "../../⚛️react-port/🟦️.ts";
import { type IconName } from "@semio-tech/ui-react";
import { worldProjectionDefaults, worldProjectionOrbitConstraints, worldProjectionSpecIconId, worldProjectionSpecLabel, type WorldCameraState, type WorldProjectionSpec } from "@semio-tech/framework/infinite/world/r3f";
import { world3dAutoFitKey, world3dFitProjectionContent, world3dProjectionContentFrameMounted, type WorldParsedCameraState } from "../🔭️scene-schema/🟦️.ts";
import { WORLD_PAINT_IDLE, type WorldPaintEvent, type WorldPaintGesture, type WorldPaintOrigin, worldPaintStep } from "../🧮️gumball-math/🟦️.ts";
import { peekPendingWorldProjection } from "../🪟️window-view-store/🟦️.tsx";
import type { WorldCore4 } from "../👇️hover-and-placement/🟦️.ts";

// #region World3dHost
/** 🔀️ Wires the paint gesture and the camera and projection handlers. */
export function useWorldPaintAndProjection(core: WorldCore4) {
  const { activeUtility, adoptViewportCamera, autoFitBounds, cameraNavigating, cameraState, dispatch, dispatchWorldCameraDebounced, externalPendingProjectionSpec, fit, instances, noteUserMovedCamera, pendingProjectionSpecRef, projectionContentFrameSeededRef, projectionFramePending, retainViewport, retainedPendingProjectionSpecRef, sceneCameraAttachJson, selection, setCameraNavigating, setDetachEpoch, setExternalPendingProjectionSpec, setProjectionFramePending, setViewportCamera, setViewportOwned, setWindowIcon, setWindowTitle, userMovedFitRevision, viewportOwned, windowInstanceId } = core;
  const paintMode = selection.interactionMode === "paint";
  /** 🖌️ The paint gesture, stepped by `worldPaintStep`; the release and the cancels listen window-wide, so a release
   * outside the canvas still commits and a blur or unmount mid-drag leaves zero trace. */
  const paintGestureRef = useRef<WorldPaintGesture>(WORLD_PAINT_IDLE);
  const stepPaintGesture = useCallback(
    (event: WorldPaintEvent) => {
      const next = worldPaintStep(paintGestureRef.current, event);
      paintGestureRef.current = next.gesture;
      if (next.dispatch) dispatch("paintAt", next.dispatch);
    },
    [dispatch],
  );
  const stepPaintGestureRef = useRef(stepPaintGesture);
  stepPaintGestureRef.current = stepPaintGesture;
  const handlePaintAt = useCallback((objectId: string, u: number, v: number, origin: WorldPaintOrigin) => stepPaintGesture({ kind: "dab", origin, objectId, u, v }), [stepPaintGesture]);
  useEffect(() => {
    const onRelease = () => stepPaintGestureRef.current({ kind: "release" });
    const onCancel = () => stepPaintGestureRef.current({ kind: "cancel", reason: "captureLost" });
    const onBlur = () => stepPaintGestureRef.current({ kind: "cancel", reason: "blur" });
    window.addEventListener("pointerup", onRelease);
    window.addEventListener("pointercancel", onCancel);
    window.addEventListener("blur", onBlur);
    return () => {
      window.removeEventListener("pointerup", onRelease);
      window.removeEventListener("pointercancel", onCancel);
      window.removeEventListener("blur", onBlur);
      stepPaintGestureRef.current({ kind: "cancel", reason: "captureLost" });
    };
  }, []);

  const handleCameraChange = useCallback(
    (state: WorldCameraState) => {
      adoptViewportCamera(state, false);
      noteUserMovedCamera();
      dispatchWorldCameraDebounced(state);
    },
    [adoptViewportCamera, noteUserMovedCamera, dispatchWorldCameraDebounced],
  );

  const handleCameraNavigate = useCallback(
    (active: boolean) => {
      setCameraNavigating((prev) => (prev === active ? prev : active));
      if (active) {
        setViewportOwned(true);
        noteUserMovedCamera();
      }
    },
    [noteUserMovedCamera],
  );

  const handleAutoFitCameraChange = useCallback(
    (state: WorldCameraState) => {
      adoptViewportCamera(state, false);
    },
    [adoptViewportCamera],
  );

  const syncProjectionWindowChrome = useCallback(
    (spec: WorldProjectionSpec) => {
      if (!windowInstanceId) return;
      setWindowTitle?.(windowInstanceId, worldProjectionSpecLabel(spec));
      setWindowIcon?.(windowInstanceId, worldProjectionSpecIconId(spec) as IconName);
    },
    [setWindowIcon, setWindowTitle, windowInstanceId],
  );

  useEffect(() => {
    const pending = peekPendingWorldProjection(windowInstanceId);
    if (pending && windowInstanceId) setWindowIcon?.(windowInstanceId, worldProjectionSpecIconId(pending) as IconName);
  }, [setWindowIcon, windowInstanceId]);

  const handleGizmoCameraChange = useCallback(
    (state: WorldCameraState) => {
      adoptViewportCamera(state, true);
      retainedPendingProjectionSpecRef.current = null;
      retainViewport({ pendingProjectionSpec: null });
      noteUserMovedCamera();
      if (state.projectionSpec) syncProjectionWindowChrome(state.projectionSpec);
      dispatchWorldCameraDebounced(state);
    },
    [adoptViewportCamera, noteUserMovedCamera, syncProjectionWindowChrome, dispatchWorldCameraDebounced],
  );

  const [pendingProjectionSpec, setPendingProjectionSpec] = useState<WorldProjectionSpec | null>(null);

  const handleProjectionKindChange = useCallback(
    (spec: WorldProjectionSpec) => {
      retainedPendingProjectionSpecRef.current = spec;
      setExternalPendingProjectionSpec(spec);
      retainViewport({ pendingProjectionSpec: spec });
      syncProjectionWindowChrome(spec);
    },
    [retainViewport, syncProjectionWindowChrome],
  );

  const handleProjectionContentFrame = useCallback((state: WorldParsedCameraState) => {
    setViewportCamera(state);
    if (!projectionContentFrameSeededRef.current) {
      projectionContentFrameSeededRef.current = true;
      retainViewport({ projectionContentFrameSeeded: true });
      setDetachEpoch((epoch) => epoch + 1);
    }
    retainViewport({ camera: state, viewportOwned: true });
    setViewportOwned(true);
    setProjectionFramePending(false);
  }, [retainViewport]);

  const worldProjectionSpec: WorldProjectionSpec = cameraState.projectionSpec ?? (cameraState.projection === "orthographic" ? worldProjectionDefaults("orthographic") : worldProjectionDefaults("threePoint"));
  /** 📷️ Keep fitting the live content bounds into seeded projection panes (esp. orthographic Top) until the
   * user takes ownership — otherwise tool-placed objects that expand the scene fall outside the one-shot
   * initial frustum and only remain visible in the wider perspective pane. */
  const localProjectionPending = pendingProjectionSpec !== null || externalPendingProjectionSpec !== null || pendingProjectionSpecRef.current !== null;
  const hasProjectionSeed = Boolean(pendingProjectionSpecRef.current ?? pendingProjectionSpec ?? externalPendingProjectionSpec ?? cameraState.projectionSpec);
  const lockProjectionContentFrame =
    activeUtility === "fill" || instances.some((instance) => instance.provisional) || projectionContentFrameSeededRef.current;
  const fitProjectionContent = world3dFitProjectionContent(viewportOwned, cameraNavigating, hasProjectionSeed, lockProjectionContentFrame, localProjectionPending ? "content" : cameraState.projectionFrame);
  const autoFitKey = world3dAutoFitKey(fit?.revision ?? 0, sceneCameraAttachJson, autoFitBounds);
  const projectionContentFrameMounted = world3dProjectionContentFrameMounted(fitProjectionContent, projectionFramePending, cameraNavigating);
  const autoFitUserMoved = userMovedFitRevision === (fit?.revision ?? 0);
  const worldOrbitConstraints = useMemo(() => worldProjectionOrbitConstraints(cameraState.projectionSpec), [cameraState.projectionSpec]);
  return { autoFitKey, autoFitUserMoved, fitProjectionContent, handleAutoFitCameraChange, handleCameraChange, handleCameraNavigate, handleGizmoCameraChange, handlePaintAt, handleProjectionContentFrame, handleProjectionKindChange, paintMode, pendingProjectionSpec, projectionContentFrameMounted, setPendingProjectionSpec, stepPaintGesture, worldOrbitConstraints, worldProjectionSpec };
}

export type WorldCore5 = WorldCore4 & ReturnType<typeof useWorldPaintAndProjection>;
// #endregion World3dHost
