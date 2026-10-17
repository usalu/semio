/** 👆️ InkCanvasHost pointer: the move start, the pointer down, move, up and cancel handlers, the resize handle and block handlers, the wheel and the double click. */
import React, { useCallback } from "react";
import { applyPinchToOffsetCamera, inkCanvasActions } from "@semio-tech/framework";
import { screenRectFromPoints } from "@semio-tech/ui-react";
import { type InkCanvasEvent, type InkItem, type InkResizeHandle } from "../🧾️ink-model/🟦️.ts";
import { eraseInkStrokeEventsAtPoint, eraseInkStrokePointEventsNearPoint, findInkItem, inkItemsAtPoint, inkItemsIntersectingRect, inkMaybeSnapWorldPoint, inkResizeBounds, inkScaleItemWithinGroup, inkSnapWorldPoint, inkTableCellAtPoint, screenToWorld } from "../📐️ink-geometry/🟦️.ts";
import { createInkItemByKind } from "../🌳️ink-document-ops/🟦️.ts";
import { INK_CAMERA_ZOOM_BOUNDS } from "../🧲️ink-host-state/🟦️.ts";
import type { InkCore2 } from "../🎬️ink-gestures/🟦️.ts";

// #region InkCanvasHost
/** 👆️ Builds the pointer, wheel and double click handlers of the ink canvas. */
export function useInkPointer(core: InkCore2) {
  const { abortGesture, atomicGesture, beginGesture, commitGesture, dispatch, doc, dragState, gestureActiveRef, gestureRecognizer, interactive, isNavigator, liveGesture, marqueePoints, pinchCameraRef, presencePointerRef, publishInkPresenceView, publishInteractionHover, publishInteractionSelection, rootRef, sceneDoc, selectedIds, selectedSet, selectionBounds, setDraftDoc, setDragState, setMarqueePoints, setTableEdit, setTextEdit, takePendingLive, utility } = core;
  const beginMove = useCallback(
    (event: React.PointerEvent, blockId: string) => {
      if (!rootRef.current || !doc) return;
      const block = findInkItem(doc, blockId);
      if (!block || block.locked) return;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const moveIds = selectedSet.has(blockId) ? selectedIds : [blockId];
      const origins: Record<string, { x: number; y: number }> = {};
      for (const id of moveIds) {
        const entry = findInkItem(doc, id);
        if (entry) origins[id] = { x: entry.x, y: entry.y };
      }
      setDragState({ kind: "move", origins, startX: screenX, startY: screenY });
    },
    [doc, selectedIds, selectedSet],
  );

  /** 🤏️ Every contact reaches the shared recognizer in the CAPTURE phase — before a block's own
   * `pointerdown` can stop it — so a second finger landing on a block still starts the pinch. The pinch
   * cancels whatever the first finger began (its live stroke/move preview is dropped, never committed). */
  const handlePointerDownCapture = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc) return;
      const rect = rootRef.current.getBoundingClientRect();
      const verdict = gestureRecognizer.down({ pointerId: event.pointerId, x: event.clientX - rect.left, y: event.clientY - rect.top });
      if (verdict.kind === "single") return;
      event.stopPropagation();
      if (verdict.kind !== "pinchBegin") return;
      abortGesture();
      setDragState(null);
      setMarqueePoints([]);
      pinchCameraRef.current = sceneDoc?.camera ?? doc.camera;
      setDraftDoc(sceneDoc ? { ...sceneDoc, camera: pinchCameraRef.current } : null);
    },
    [abortGesture, doc, gestureRecognizer, sceneDoc],
  );

  const handlePointerDown = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isNavigator || !interactive) return;
      rootRef.current.focus();
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
      if (utility === "pan" || event.button === 1 || (utility === "selectDirect" && event.altKey)) {
        setDragState({ kind: "pan", startX: screenX, startY: screenY, camera });
        return;
      }
      if (utility === "eraserStroke" || utility === "eraserPoint") {
        setDragState({ kind: "eraser", mode: utility });
        const events = utility === "eraserStroke" ? eraseInkStrokeEventsAtPoint(doc, worldX, worldY) : eraseInkStrokePointEventsNearPoint(doc, worldX, worldY, doc.eraserRadius ?? 12);
        if (events.length) beginGesture(events);
        return;
      }
      if (utility === "selectMarquee") {
        setDragState({ kind: "marquee", start: { x: screenX, y: screenY } });
        setMarqueePoints([{ x: screenX, y: screenY }]);
        return;
      }
      if (utility === "pencil") {
        const block = createInkItemByKind("stroke", worldX, worldY);
        beginGesture([{ operation: "addBlock", block }], [block.id]);
        setDragState({ kind: "stroke", blockId: block.id });
        return;
      }
      if (utility === "text" || utility === "image" || utility === "table" || utility === "math") {
        const [placeX, placeY] = inkMaybeSnapWorldPoint(doc, worldX, worldY);
        const block = createInkItemByKind(utility, placeX, placeY);
        atomicGesture([{ operation: "addBlock", block }], [block.id]);
        if (utility === "text") setTextEdit({ blockId: block.id, created: true });
        return;
      }
      const hits = inkItemsAtPoint(doc.blocks, worldX, worldY);
      const top = hits[0];
      if (!top || top.locked) {
        if (utility === "selectDirect" && !top) publishInteractionSelection([], "replace", "pick");
        return;
      }
      if (utility === "selectDirect") {
        publishInteractionSelection([top], event.shiftKey ? "additive" : "replace", "pick");
        beginMove(event, top.id);
      }
    },
    [atomicGesture, beginGesture, beginMove, doc, interactive, isNavigator, publishInteractionSelection, utility],
  );

  const handleBlockPointerDown = useCallback(
    (event: React.PointerEvent, blockId: string) => {
      event.stopPropagation();
      if (!rootRef.current || !doc || !interactive) return;
      const block = findInkItem(doc, blockId);
      if (!block || block.locked) return;
      publishInteractionSelection([block], event.shiftKey ? "additive" : "replace", "pick");
      if (utility === "selectDirect" || utility === "selectMarquee") beginMove(event, blockId);
    },
    [beginMove, doc, interactive, publishInteractionSelection, utility],
  );

  const handleResizePointerDown = useCallback(
    (handle: InkResizeHandle, event: React.PointerEvent) => {
      event.stopPropagation();
      if (!rootRef.current || !selectionBounds) return;
      const rect = rootRef.current.getBoundingClientRect();
      setDragState({ kind: "resize", handle, fromBounds: selectionBounds, startX: event.clientX - rect.left, startY: event.clientY - rect.top, selectedIds: [...selectedIds] });
    },
    [selectedIds, selectionBounds],
  );

  const handlePointerMove = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc) return;
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      presencePointerRef.current = screenToWorld(camera, screenX, screenY);
      publishInkPresenceView(false);
      const verdict = gestureRecognizer.move({ pointerId: event.pointerId, x: screenX, y: screenY });
      if (verdict.kind === "pinch") {
        const nextCamera = applyPinchToOffsetCamera(pinchCameraRef.current ?? camera, verdict.step, INK_CAMERA_ZOOM_BOUNDS);
        pinchCameraRef.current = nextCamera;
        setDraftDoc((current) => ({ ...(current ?? doc), camera: nextCamera }));
        return;
      }
      if (verdict.kind !== "single") return;
      const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
      if (!dragState) {
        if (!interactive) return;
        const hits = inkItemsAtPoint(doc.blocks, worldX, worldY);
        const top = hits[0] ?? null;
        publishInteractionHover(top);
        return;
      }
      if (dragState.kind === "pan") {
        const nextCamera = { ...dragState.camera, x: dragState.camera.x + (screenX - dragState.startX), y: dragState.camera.y + (screenY - dragState.startY) };
        setDraftDoc((current) => ({ ...(current ?? doc), camera: nextCamera }));
        dispatch(inkCanvasActions.setCamera, { camera: nextCamera });
        return;
      }
      if (dragState.kind === "move") {
        const dx = (screenX - dragState.startX) / camera.zoom;
        const dy = (screenY - dragState.startY) / camera.zoom;
        const events: InkCanvasEvent[] = [];
        for (const [blockId, origin] of Object.entries(dragState.origins)) {
          const block = findInkItem(doc, blockId);
          if (!block) continue;
          events.push({ operation: "updateBlock", blockId, block: { ...block, x: origin.x + dx, y: origin.y + dy } });
        }
        if (events.length) liveGesture(events, { kind: "drag", ids: Object.keys(dragState.origins), dx, dy });
        return;
      }
      if (dragState.kind === "marquee") {
        setMarqueePoints([dragState.start, { x: screenX, y: screenY }]);
        return;
      }
      if (dragState.kind === "stroke") {
        const block = findInkItem(doc, dragState.blockId);
        if (!block || block.kind !== "stroke") return;
        const localX = worldX - block.x;
        const localY = worldY - block.y;
        liveGesture([{ operation: "updateBlock", blockId: block.id, block: { ...block, points: [...block.points, [localX, localY]] } }]);
        return;
      }
      if (dragState.kind === "eraser") {
        const events = dragState.mode === "eraserStroke" ? eraseInkStrokeEventsAtPoint(doc, worldX, worldY) : eraseInkStrokePointEventsNearPoint(doc, worldX, worldY, doc.eraserRadius ?? 12);
        if (events.length) liveGesture(events);
        return;
      }
      if (dragState.kind === "resize") {
        const dx = (screenX - dragState.startX) / camera.zoom;
        const dy = (screenY - dragState.startY) / camera.zoom;
        const toBounds = inkResizeBounds(dragState.fromBounds, dragState.handle, dx, dy);
        const events: InkCanvasEvent[] = [];
        for (const blockId of dragState.selectedIds) {
          const block = findInkItem(doc, blockId);
          if (!block) continue;
          events.push({ operation: "updateBlock", blockId, block: inkScaleItemWithinGroup(block, dragState.fromBounds, toBounds) });
        }
        if (events.length) liveGesture(events);
      }
    },
    [dispatch, doc, dragState, gestureRecognizer, interactive, liveGesture, publishInteractionHover, publishInkPresenceView],
  );

  /** 🤏️ Routes a lifted/cancelled contact through the shared recognizer: the LAST finger of a pinch
   * commits its camera exactly once; any other contact of a latched gesture is swallowed. Answers whether
   * the single-pointer lane may handle the release. */
  const releaseContact = useCallback(
    (pointerId: number): boolean => {
      const verdict = gestureRecognizer.up(pointerId);
      if (verdict.kind === "pinchEnd" && pinchCameraRef.current) {
        dispatch(inkCanvasActions.setCamera, { camera: pinchCameraRef.current });
        pinchCameraRef.current = null;
      }
      return verdict.kind === "single";
    },
    [dispatch, gestureRecognizer],
  );

  const handlePointerUp = useCallback(() => {
    if (!doc) {
      setDragState(null);
      setMarqueePoints([]);
      return;
    }
    if (dragState?.kind === "move") {
      const ids = Object.keys(dragState.origins);
      const anchor = ids.map((id) => ({ origin: dragState.origins[id]!, block: findInkItem(doc, id) })).find(({ block }) => block !== null);
      if (anchor?.block) {
        const [x, y] = doc.snapEnabled ? inkSnapWorldPoint(anchor.block.x, anchor.block.y, doc.snapGridSpacing ?? 8) : [anchor.block.x, anchor.block.y];
        commitGesture([], { kind: "drag", ids, dx: x - anchor.origin.x, dy: y - anchor.origin.y });
      } else commitGesture([]);
    } else if (dragState?.kind === "stroke") {
      const block = findInkItem(doc, dragState.blockId);
      if (block) commitGesture([{ operation: "updateBlock", blockId: block.id, block }]);
      else commitGesture([]);
    } else if (dragState?.kind === "resize") {
      const events: InkCanvasEvent[] = [];
      for (const blockId of dragState.selectedIds) {
        const block = findInkItem(doc, blockId);
        if (block) events.push({ operation: "updateBlock", blockId, block });
      }
      commitGesture(events);
    } else if (dragState?.kind === "eraser") {
      commitGesture([]);
    } else if (dragState?.kind === "pan") {
      takePendingLive();
      gestureActiveRef.current = false;
    }
    if (dragState?.kind === "marquee" && marqueePoints.length >= 2 && rootRef.current) {
      const screenRect = screenRectFromPoints(marqueePoints);
      if (screenRect) {
        const camera = doc.camera;
        const worldRect = { x: (screenRect.x - camera.x) / camera.zoom, y: (screenRect.y - camera.y) / camera.zoom, width: screenRect.width / camera.zoom, height: screenRect.height / camera.zoom };
        const blocks = inkItemsIntersectingRect(doc.blocks, worldRect).map((id) => findInkItem(doc, id)).filter((block): block is InkItem => block !== null);
        publishInteractionSelection(blocks, "replace", "rectangle");
      }
    }
    setDragState(null);
    setMarqueePoints([]);
  }, [commitGesture, doc, dragState, marqueePoints, publishInteractionSelection, takePendingLive]);

  const handlePointerCancel = useCallback(() => {
    abortGesture();
    setDraftDoc(null);
    setDragState(null);
    setMarqueePoints([]);
  }, [abortGesture]);

  const handleWheel = useCallback(
    (event: React.WheelEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isNavigator) return;
      event.preventDefault();
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const zoomFactor = event.deltaY < 0 ? 1.08 : 0.92;
      const nextZoom = Math.min(INK_CAMERA_ZOOM_BOUNDS.max, Math.max(INK_CAMERA_ZOOM_BOUNDS.min, camera.zoom * zoomFactor));
      const worldX = (screenX - camera.x) / camera.zoom;
      const worldY = (screenY - camera.y) / camera.zoom;
      const nextCamera = { x: screenX - worldX * nextZoom, y: screenY - worldY * nextZoom, zoom: nextZoom };
      setDraftDoc((current) => ({ ...(current ?? doc), camera: nextCamera }));
      dispatch(inkCanvasActions.setCamera, { camera: nextCamera });
    },
    [dispatch, doc, isNavigator],
  );

  const handleDoubleClick = useCallback(
    (event: React.MouseEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isNavigator || !interactive) return;
      const camera = doc.camera;
      const rect = rootRef.current.getBoundingClientRect();
      const screenX = event.clientX - rect.left;
      const screenY = event.clientY - rect.top;
      const [worldX, worldY] = screenToWorld(camera, screenX, screenY);
      const hits = inkItemsAtPoint(doc.blocks, worldX, worldY);
      const top = hits[0];
      if (top?.kind === "text" && !top.locked) {
        setTableEdit(null);
        setTextEdit({ blockId: top.id });
        publishInteractionSelection([top], "replace", "pick");
        return;
      }
      if (top?.kind === "table" && !top.locked) {
        const cell = inkTableCellAtPoint(top, worldX - top.x, worldY - top.y);
        if (!cell) return;
        setTextEdit(null);
        setTableEdit({ blockId: top.id, row: cell.row, col: cell.col });
        publishInteractionSelection([top], "replace", "pick");
        return;
      }
      if (top) return;
      const [placeX, placeY] = inkMaybeSnapWorldPoint(doc, worldX, worldY);
      const block = createInkItemByKind("text", placeX, placeY);
      atomicGesture([{ operation: "addBlock", block }], [block.id]);
      setTextEdit({ blockId: block.id, created: true });
    },
    [atomicGesture, doc, interactive, isNavigator, publishInteractionSelection],
  );
  return { handleBlockPointerDown, handleDoubleClick, handlePointerCancel, handlePointerDown, handlePointerDownCapture, handlePointerMove, handlePointerUp, handleResizePointerDown, handleWheel, releaseContact };
}

export type InkCore3 = InkCore2 & ReturnType<typeof useInkPointer>;
// #endregion InkCanvasHost
