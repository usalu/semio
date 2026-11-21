/** 🔶️ The abort gesture phase of useInkPointer. */
import * as React from "../../../⚛️react-port/🟦️.ts";
import { useCallback } from "../../../⚛️react-port/🟦️.ts";
import { type InkItem, type InkResizeHandle } from "../../🧾️ink-model/🟦️.ts";
import { eraseInkStrokeEventsAtPoint, eraseInkStrokePointEventsNearPoint, findInkItem, inkItemsAtPoint, inkMaybeSnapWorldPoint, screenToWorld } from "../../📐️ink-geometry/🟦️.ts";
import { createInkItemByKind } from "../../🌳️ink-document-ops/🟦️.ts";
import type { InkCore2 } from "../../🎬️ink-gestures/🟦️.ts";

function isInkEditorTarget(target: EventTarget): boolean {
  return target instanceof Element && target.closest("[data-ink-editor-overlay]") !== null;
}

/** 🔷 Builds abortGesture, atomicGesture, beginGesture, commitGesture of useInkPointer. */
export function useAbortGestureState({ core }: { readonly core: InkCore2 }) {
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

  const pickAndBeginMove = useCallback(
    (event: React.PointerEvent, block: InkItem) => {
      if (event.shiftKey || selectedIds.length < 2 || !selectedSet.has(block.id)) {
        publishInteractionSelection([block], event.shiftKey ? "additive" : "replace", "pick");
      }
      beginMove(event, block.id);
    },
    [beginMove, publishInteractionSelection, selectedIds.length, selectedSet],
  );

  /** 🤏️ Every contact reaches the shared recognizer in the CAPTURE phase — before a block's own
   * `pointerdown` can stop it — so a second finger landing on a block still starts the pinch. The pinch
   * cancels whatever the first finger began (its live stroke/move preview is dropped, never committed). */
  const handlePointerDownCapture = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (!rootRef.current || !doc || isInkEditorTarget(event.target)) return;
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
      if (!rootRef.current || !doc || isNavigator || !interactive || isInkEditorTarget(event.target)) return;
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
      if (utility === "selectDirect") pickAndBeginMove(event, top);
    },
    [atomicGesture, beginGesture, doc, interactive, isNavigator, pickAndBeginMove, publishInteractionSelection, utility],
  );

  const handleBlockPointerDown = useCallback(
    (event: React.PointerEvent, blockId: string) => {
      if (!rootRef.current || !doc || !interactive || isNavigator) return;
      if ((utility !== "selectDirect" && utility !== "selectMarquee") || event.button === 1 || (utility === "selectDirect" && event.altKey)) return;
      event.stopPropagation();
      rootRef.current.focus();
      const block = findInkItem(doc, blockId);
      if (block && !block.locked) pickAndBeginMove(event, block);
    },
    [doc, interactive, isNavigator, pickAndBeginMove, utility],
  );

  const handleResizePointerDown = useCallback(
    (handle: InkResizeHandle, event: React.PointerEvent) => {
      event.stopPropagation();
      if (!rootRef.current || !doc || !selectionBounds) return;
      const rect = rootRef.current.getBoundingClientRect();
      const origins = selectedIds.map((id) => findInkItem(doc, id)).filter((block) => block !== null);
      setDragState({ kind: "resize", handle, fromBounds: selectionBounds, startX: event.clientX - rect.left, startY: event.clientY - rect.top, origins });
    },
    [doc, selectedIds, selectionBounds],
  );
  return { rootRef, doc, presencePointerRef, publishInkPresenceView, gestureRecognizer, pinchCameraRef, setDraftDoc, dragState, interactive, publishInteractionHover, dispatch, liveGesture, setMarqueePoints, setDragState, commitGesture, takePendingLive, gestureActiveRef, marqueePoints, publishInteractionSelection, abortGesture, isNavigator, setTableEdit, setTextEdit, atomicGesture, handleBlockPointerDown, handlePointerDown, handlePointerDownCapture, handleResizePointerDown };
}
