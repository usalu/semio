/** 🎬️ Action dispatch, the context menu dispatch builder, gesture reasons, begin and end of a gesture, camera publication, the wheel gesture and the pointer-down gesture recognisers.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | FlowGraphCanvasHost}: `useFlowDispatchAndGestures` runs the original statements in order and returns the bindings later hooks and the render read; the component merges them into the shared core object.
 */

import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState
} from "react";
import { GestureRecognizer, nodeGraphActions } from "@semio-tech/framework";
import { useAppKeybindingsByActionId, useShellContextMenuFallback } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { nodeGraphViewportActionArgs, parseNodeGraphSessionViewport, resolveHostSnapshotWidgetInstanceId } from "../🧰️graph-core/🟦️.ts";
import { useGraphSliderLanes } from "../🖼️dag-overlays/🟦️.tsx";
import { observeFlowTask } from "../🔄️flow-session-sync/🟦️.ts";
import { createFlowCameraGesture } from "../🧷️flow-canvas-contract/🟦️.ts";
import type { FlowCoreAfterFlowSurfaceState } from "../🧱️flow-surface-state/🟦️.ts";
//#region 🔖️NodeGraphHost
//#region 🔖️flow-graph-canvas-host
//#region FlowGraphCanvasHost

export type FlowCoreAfterFlowDispatchAndGestures = FlowCoreAfterFlowSurfaceState & ReturnType<typeof useFlowDispatchAndGestures>;

export function useFlowDispatchAndGestures(core: FlowCoreAfterFlowSurfaceState) {
  const { scene, surfaceId, controllerId, onAction, sessionRef, schedulerRef, containerRef } = core;
  // 🔁️ Returns what `onAction` returns. `ComponentSceneHostProps.onAction` is declared
  // `void | Promise<void>` — a settling shell answers with the promise, and dropping it here is what
  // left every continuous gesture (a dragged slider) with no way to know when its last value had
  // landed, so it could only queue one round trip per tick.
  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => onAction({ controllerId, action, args: { surfaceId, ...args } }),
    [controllerId, onAction, surfaceId],
  );

  // 📮️ Effects that must NOT re-run per render (attach, scene sync) reach the dispatcher through this
  // ref: `dispatch`'s identity follows the `onAction` prop, and depending on it would re-attach the
  // canvas — and re-issue `synchronizeSnapshotJson` — on every parent render.
  const dispatchRef = useRef(dispatch);
  dispatchRef.current = dispatch;

  const flowMenuKeysByActionId = useAppKeybindingsByActionId();
  const shellContextMenuFallback = useShellContextMenuFallback();
  /** 🖱️ Builds the dispatch sink for one flow-graph context-menu opening — bound to that opening's own
   * `widgetId`/coordinates rather than closing over the (async, not-yet-committed) `contextMenu` state,
   * so `openInstance`'s fixture lookup always resolves against the widget actually right-clicked. */
  const buildFlowMenuDispatch = useCallback(
    (widgetId: string | undefined, x: number, y: number) => (action: string, args?: Record<string, unknown>) => {
      if (action === "openSpotlight") {
        const host = containerRef.current;
        if (host) core.openSpotlightAtClient(x, y, host);
        return;
      }
      dispatch(action, action === "openInstance" ? { ...args, instanceId: resolveHostSnapshotWidgetInstanceId(scene.hostSnapshotJson, widgetId) } : args);
    },
    [dispatch, scene.hostSnapshotJson],
  );

  /** 🤹 The gesture reasons holding this surface open right now. A surface can be under two at once
   * (a wheel gesture arriving mid-drag), so "is a gesture active" is a set membership rather than a
   * boolean that the second `end` would clear while the first still runs. */
  const gestureReasonsRef = useRef<Set<string>>(new Set());
  const cameraPanRef = useRef(false);
  const isGestureActive = useCallback(() => gestureReasonsRef.current.size > 0, []);

  /** 🎬️ Opens a gesture: the demand scheduler goes continuous for its duration, so every repaint the
   * gesture asks for is coalesced onto a rAF instead of issued per input event. */
  const beginGesture = useCallback((reason: string) => {
    gestureReasonsRef.current.add(reason);
    schedulerRef.current?.beginContinuous(reason);
    schedulerRef.current?.invalidate();
  }, []);

  /** 🧾️ A scene that arrived while a gesture held the session, and the camera-only gestures that owe
   * it an apply when they end.
   *
   * The scene effect skips its sync while a gesture is live — a drag holds live fixture edits the
   * session must not have overwritten — and it only ever runs again on the NEXT scene, so a scene
   * that arrived during a gesture was dropped for good. With gestures now covering wheel and pan as
   * well as slider drags, that window is wide enough to swallow a real edit: a `reorganize` issued
   * just after a zoom moved every widget in the plugin and none on the board
   * (`📓️flow-scroll-render-perf-2026-09-15.md` §7). A camera gesture holds no fixture edits, so its
   * end can simply apply what it deferred; a content gesture's own commit brings the next scene. */
  const deferredSceneRef = useRef(false);
  const cameraOnlyGestureEndRef = useRef(false);
  const applyDeferredSceneRef = useRef<() => void>(() => {});

  const endGesture = useCallback((reason: string) => {
    if (!gestureReasonsRef.current.delete(reason)) return;
    schedulerRef.current?.endContinuous(reason);
    if (gestureReasonsRef.current.size === 0 && deferredSceneRef.current && (reason === "wheel" || cameraOnlyGestureEndRef.current)) {
      deferredSceneRef.current = false;
      applyDeferredSceneRef.current();
    }
    schedulerRef.current?.invalidate();
  }, []);

  /** 🎥️ Publishes the live board camera to the plugin — ONCE, when a camera gesture has settled.
   *
   * The camera is board state, not plugin state: `wheelScreen`/`pointerMoveScreen` move it and the
   * board repaints from it with no guest involved. `nodeGraphViewport` exists so the NEXT open of
   * this graph honours where the user left the view, which is a per-gesture fact, not a per-tick one.
   * Dispatching it per wheel tick cost a `performInvocation` → `refreshUi` → React commit for every
   * notch of the wheel: measured at 30 ticks → 5–9 guest hops, 1 board paint, and a median
   * tick-to-paint of 653–1110 ms (`📓️flow-scroll-render-perf-2026-09-15.md` §2). */
  const publishCameraRef = useRef<() => void>(() => {});
  const publishCamera = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    observeFlowTask(session, "viewport:settle", session.viewport(), (value) => {
      dispatch(nodeGraphActions.viewport, nodeGraphViewportActionArgs(parseNodeGraphSessionViewport(value)));
    });
  }, [dispatch]);
  publishCameraRef.current = publishCamera;

  /** 🖱️ A wheel gesture has no release event, so its end is the absence of the next tick: every tick
   * restarts {@link FLOW_CAMERA_GESTURE_SETTLE_MS}, and the gesture settles once when they stop. */
  const wheelGesture = useMemo(
    () =>
      createFlowCameraGesture({
        begin: beginGesture,
        end: endGesture,
        invalidate: () => schedulerRef.current?.invalidate(),
        publish: () => publishCameraRef.current(),
        schedule: (run, delayMs) => setTimeout(run, delayMs),
        cancel: (handle) => clearTimeout(handle as ReturnType<typeof setTimeout>),
      }),
    [beginGesture, endGesture],
  );

  useEffect(() => () => wheelGesture.dispose(), [wheelGesture]);

  const handleGesturePointerDown = useCallback(() => {
    beginGesture("gesture");
  }, [beginGesture]);

  const { sliderLane } = useGraphSliderLanes(surfaceId, dispatchRef);
  const [gestureRecognizer] = useState(() => new GestureRecognizer());
  const pinchLogScaleRef = useRef(0);

  return {
    dispatch, dispatchRef, flowMenuKeysByActionId, shellContextMenuFallback, buildFlowMenuDispatch, cameraPanRef,
    isGestureActive, deferredSceneRef, cameraOnlyGestureEndRef, applyDeferredSceneRef, endGesture, publishCameraRef,
    wheelGesture, handleGesturePointerDown, sliderLane, gestureRecognizer, pinchLogScaleRef
  };
}
//#endregion FlowGraphCanvasHost
//#endregion 🔖️flow-graph-canvas-host
//#endregion 🔖️NodeGraphHost
