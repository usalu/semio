/** 👆️ Board2dHost pointer and keyboard: the presence publishing, the canvas pointer effect and the keyboard effects. */
import { useCallback, useEffect, useRef } from "react";
import { type Board2dWasmSession } from "../../🪪️WasmSessionLoader/🟦️.tsx";
import { useLocalPresenceActorIdV1 } from "../../👕️canvas-presence/🟦️.tsx";
import { PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS, publishLocalPresenceWindowViewV1, clearLocalPresenceWindowViewV1, publishLocalActiveToolV1 } from "../../👕️canvas-presence/🟦️.ts";
import { board2dCameraActionArgs, board2dFloat32Decimal, parseBoardCamera } from "../🔬️board-parsing/🟦️.ts";
import { board2dPinchCamera, boardScreenToWorld } from "../📍️drop-and-camera/🟦️.ts";
import { beginBoardPeerGesture, endBoardPeerGesture, notifyBoardPeersGestureEnded } from "../🔄️session-sync/🟦️.ts";
import type { BoardCore3 } from "../🔁️board-lifecycle-and-sync/🟦️.ts";

// #region Board2dHost
/** 👆️ Wires the canvas pointer gestures, presence and keyboard of the board. */
export function useBoardPointerAndKeyboard(core: BoardCore3) {
  const { applyPendingSelectionIfReady, applyPendingSnapshotIfReady, beginCameraInteraction, boardStatusRef, canvasRef, containerRef, dispatch, dispatchBufferedEvents, drainAndMaybeFlush, drainIntoBuffer, flushBoardEvents, gestureRecognizer, hoverActiveRef, node, peerRef, peerScope, pendingCameraDispatchRef, pendingEventRowsRef, publishBoardVitals, readContainerSize, scene, sceneRef, scheduleRender, sessionRef, setLocalSelectionJson, settleGestureEnd, suggestionMenuOwnsThisWindowRef, windowInstanceId } = core;
  //#region Pointer

  const presencePublishAtRef = useRef(0);
  const presenceWindowId = windowInstanceId ?? node.surfaceId ?? "board";
  const localPresenceActor = useLocalPresenceActorIdV1("local");
  useEffect(() => () => clearLocalPresenceWindowViewV1(presenceWindowId), [presenceWindowId]);
  const publishBoardPresenceView = useCallback((screenPoint: { readonly x: number; readonly y: number } | null) => {
    const now = Date.now();
    if (now - presencePublishAtRef.current < PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS) return;
    presencePublishAtRef.current = now;
    const cameraJson = sceneRef.current?.cameraJson ?? "";
    const camera = parseBoardCamera(cameraJson) ?? { x: 0, y: 0, zoom: 1 };
    const size = readContainerSize();
    const world = screenPoint ? boardScreenToWorld(cameraJson, size, screenPoint) : null;
    publishLocalPresenceWindowViewV1("local", presenceWindowId, {
      windowId: presenceWindowId,
      space: "canvas",
      kind: { kind: "canvas", x: camera.x, y: camera.y, zoom: camera.zoom },
      size: [size.w, size.h],
      ...(world ? { pointer: [world.x, world.y, 0] as const } : {}),
    });
    publishLocalActiveToolV1("local", sceneRef.current?.activeUtility ?? null);
  }, [presenceWindowId, readContainerSize]);

  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container || !scene?.interactive) return undefined;

    const clientToLocal = (clientX: number, clientY: number): { x: number; y: number } => {
      const rect = canvas.getBoundingClientRect();
      return { x: board2dFloat32Decimal(clientX) - rect.left, y: board2dFloat32Decimal(clientY) - rect.top };
    };

    /** 🤏️ Abandons the single-pointer lane the moment a SECOND contact lands: the marquee/pick
     * gesture the first finger started must not keep growing under a pinch, and its pointer capture must
     * not swallow the second finger's moves. */
    const yieldToPinch = (session: Board2dWasmSession, point: { x: number; y: number }): void => {
      session.cancelAreaSelect?.();
      session.pointerUpScreen(point.x, point.y, false, false, false);
      for (const tracked of gestureRecognizer.pointers) {
        if (canvas.hasPointerCapture?.(tracked.pointerId)) canvas.releasePointerCapture(tracked.pointerId);
      }
    };

    const onPointerDown = (event: PointerEvent): void => {
      event.stopPropagation();
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      const verdict = gestureRecognizer.down({ pointerId: event.pointerId, x: point.x, y: point.y });
      if (verdict.kind === "pinchBegin") {
        yieldToPinch(session, point);
        beginCameraInteraction();
        return;
      }
      if (verdict.kind !== "single") return;
      if (event.button === 0 || event.button === 1) {
        canvas.setPointerCapture?.(event.pointerId);
      }
      beginBoardPeerGesture(peerScope, node.controllerId, node.surfaceId, peerRef.current);
      session.pointerDownScreen(point.x, point.y, event.button, event.shiftKey, event.metaKey || event.ctrlKey);
      scheduleRender();
    };

    const onPointerMove = (event: PointerEvent): void => {
      const session = sessionRef.current;
      if (!session) return;
      const point = clientToLocal(event.clientX, event.clientY);
      publishBoardPresenceView(point);
      const verdict = gestureRecognizer.move({ pointerId: event.pointerId, x: point.x, y: point.y });
      if (verdict.kind === "pinch") {
        beginCameraInteraction();
        const camera = board2dPinchCamera(session.cameraJson(), verdict.step, readContainerSize());
        if (!camera) return;
        if (session.setCameraSilent) session.setCameraSilent(camera.x, camera.y, camera.zoom);
        else session.setCamera(camera.x, camera.y, camera.zoom);
        pendingCameraDispatchRef.current = { camera };
        scheduleRender();
        return;
      }
      if (verdict.kind !== "single") return;
      session.pointerMoveScreen(point.x, point.y, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
      scheduleRender();
      drainAndMaybeFlush();
    };

    const onPointerUp = (event: PointerEvent): void => {
      const session = sessionRef.current;
      if (!session) return;
      const verdict = gestureRecognizer.up(event.pointerId);
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      if (verdict.kind !== "single") return;
      const point = clientToLocal(event.clientX, event.clientY);
      session.pointerUpScreen(point.x, point.y, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
      endBoardPeerGesture(peerScope, node.controllerId, node.surfaceId, peerRef.current);
      const flushed = settleGestureEnd(session);
      scheduleRender();
      dispatchBufferedEvents();
      notifyBoardPeersGestureEnded(peerScope, node.controllerId, node.surfaceId, flushed);
    };

    const onPointerCancel = (event: PointerEvent): void => {
      const session = sessionRef.current;
      const verdict = gestureRecognizer.up(event.pointerId);
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      if (!session || verdict.kind !== "single") return;
      session.pointerCancelScreen();
      endBoardPeerGesture(peerScope, node.controllerId, node.surfaceId, peerRef.current);
      pendingCameraDispatchRef.current = null;
      // ↩️ A cancelled gesture publishes nothing; its restore frames still mirror into the sibling panes.
      drainIntoBuffer();
      pendingEventRowsRef.current = [];
      boardStatusRef.current.pendingEvents = 0;
      setLocalSelectionJson(null);
      applyPendingSnapshotIfReady(session);
      applyPendingSelectionIfReady(session);
      publishBoardVitals();
      scheduleRender();
      notifyBoardPeersGestureEnded(peerScope, node.controllerId, node.surfaceId, false);
    };

    const onPointerEnter = (): void => {
      hoverActiveRef.current = true;
    };

    const onPointerLeave = (event: PointerEvent): void => {
      hoverActiveRef.current = false;
      const session = sessionRef.current;
      if (!session) return;
      session.pointerLeaveScreen?.(event.altKey);
      endBoardPeerGesture(peerScope, node.controllerId, node.surfaceId, peerRef.current);
      const flushed = settleGestureEnd(session);
      scheduleRender();
      dispatchBufferedEvents();
      notifyBoardPeersGestureEnded(peerScope, node.controllerId, node.surfaceId, flushed);
    };

    /** 🐁️ Wheel-zoom stays instant locally (WASM renders every tick via `scheduleRender`); only the React-visible camera echo and event flush are deferred until the gesture settles via `beginCameraInteraction`'s timeout. */
    const onWheel = (event: WheelEvent): void => {
      event.preventDefault();
      event.stopPropagation();
      const session = sessionRef.current;
      if (!session) return;
      beginCameraInteraction();
      const point = clientToLocal(event.clientX, event.clientY);
      const delta = event.deltaY * (event.deltaMode === WheelEvent.DOM_DELTA_LINE ? 16 : event.deltaMode === WheelEvent.DOM_DELTA_PAGE ? 400 : 1);
      session.wheelScreen(point.x, point.y, delta);
      scheduleRender();
      const cameraArgs = board2dCameraActionArgs(session.cameraJson());
      if (cameraArgs) pendingCameraDispatchRef.current = cameraArgs;
      drainIntoBuffer();
    };

    canvas.addEventListener("pointerdown", onPointerDown);
    canvas.addEventListener("pointerenter", onPointerEnter);
    canvas.addEventListener("pointerleave", onPointerLeave);
    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp);
    window.addEventListener("pointercancel", onPointerCancel);
    container.addEventListener("wheel", onWheel, { passive: false });
    return () => {
      canvas.removeEventListener("pointerdown", onPointerDown);
      canvas.removeEventListener("pointerenter", onPointerEnter);
      canvas.removeEventListener("pointerleave", onPointerLeave);
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp);
      window.removeEventListener("pointercancel", onPointerCancel);
      container.removeEventListener("wheel", onWheel);
    };
  }, [peerScope, applyPendingSnapshotIfReady, applyPendingSelectionIfReady, beginCameraInteraction, dispatch, dispatchBufferedEvents, drainAndMaybeFlush, drainIntoBuffer, node.controllerId, node.surfaceId, publishBoardVitals, readContainerSize, scheduleRender, scene?.activeUtility, scene?.interactive, settleGestureEnd, publishBoardPresenceView]);
  //#endregion Pointer

  //#region Keyboard
  useEffect(() => {
    if (!scene?.interactive) return undefined;
    const isEditableTarget = (target: EventTarget | null): boolean => {
      if (!(target instanceof HTMLElement)) return false;
      return target.tagName === "INPUT" || target.tagName === "TEXTAREA" || target.isContentEditable;
    };
    const onKeyDown = (event: globalThis.KeyboardEvent): void => {
      if (!hoverActiveRef.current || isEditableTarget(event.target)) return;
      const session = sessionRef.current;
      if (!session) return;
      if (event.key === "Escape") {
        if (session.cancelAreaSelect?.()) {
          event.preventDefault();
          endBoardPeerGesture(peerScope, node.controllerId, node.surfaceId, peerRef.current);
          const flushed = settleGestureEnd(session);
          try {
            session.renderFrame();
          } catch {
            /* gpu not ready */
          }
          dispatchBufferedEvents();
          notifyBoardPeersGestureEnded(peerScope, node.controllerId, node.surfaceId, flushed);
        }
        return;
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [peerScope, dispatch, dispatchBufferedEvents, node.controllerId, node.surfaceId, scene?.interactive, scene?.selectionJson, settleGestureEnd]);

  // 🔁️ `tab` walks the open slot's candidates forward, `shift+tab` back — the same two chords the app
  // binds to `cycleBrushCandidate`/`cycleBrushCandidateBack`. This listener runs in the CAPTURE phase so
  // its `preventDefault` reaches the shell's keybinding dispatcher (which bails on `defaultPrevented`)
  // BEFORE it fires: the slot must advance exactly one step, not two. The local engine cycles first so
  // the ghost moves this frame, and the flushed `brushCandidates` row carries the new index to the guest.
  // It is armed by the armed brush OR by an open popup, which is why the picker is reachable without the tool.
  useEffect(() => {
    if (!scene?.interactive) return undefined;
    const armed = scene.activeUtility === "brush";
    const onTabCapture = (event: globalThis.KeyboardEvent): void => {
      if (event.key !== "Tab" || !(armed ? hoverActiveRef.current : suggestionMenuOwnsThisWindowRef.current)) return;
      const session = sessionRef.current;
      if (!session) return;
      event.preventDefault();
      session.brushCycleCandidate?.(!event.shiftKey);
      try {
        session.renderFrame();
      } catch {
        /* gpu not ready */
      }
      flushBoardEvents();
    };
    window.addEventListener("keydown", onTabCapture, true);
    return () => window.removeEventListener("keydown", onTabCapture, true);
  }, [flushBoardEvents, scene?.activeUtility, scene?.interactive]);
  //#endregion Keyboard
  return { localPresenceActor, presenceWindowId };
}

export type BoardCore4 = BoardCore3 & ReturnType<typeof useBoardPointerAndKeyboard>;
// #endregion Board2dHost
