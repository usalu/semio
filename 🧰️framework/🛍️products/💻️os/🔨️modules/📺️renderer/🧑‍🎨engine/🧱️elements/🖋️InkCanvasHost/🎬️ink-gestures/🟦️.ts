/** 🎬️ InkCanvasHost gestures: the action dispatcher, the interaction selection and hover publishing and the begin, live, commit, abort and atomic gesture lifecycle of the ink events. */
import { useCallback, useMemo } from "react";
import { inkCanvasActions } from "@semio-tech/framework";
import { useShellContextMenuFallback, useMapContextMenuSpecs } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { type InkCanvasEvent, type InkDragGesture, type InkItem, type InkPendingLive } from "../🧾️ink-model/🟦️.ts";
import { inkSelectionBounds } from "../📐️ink-geometry/🟦️.ts";
import { applyEventsLocal } from "../🌳️ink-document-ops/🟦️.ts";
import type { InkCore1 } from "../🧰️ink-session-state/🟦️.ts";

// #region InkCanvasHost
/** 🎬️ Publishes the interaction lanes and runs the gesture lifecycle that sends ink events. */
export function useInkGestures(core: InkCore1) {
  const { doc, gestureActiveRef, interactionDomain, isNavigator, node, onAction, pendingLiveRef, rafRef, scene, sceneDoc, selectedIds, setDraftDoc } = core;
  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      if (!node.controllerId) return;
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );
  const publishInteractionSelection = useCallback(
    (blocks: readonly InkItem[], merge: "replace" | "additive", method: "pick" | "rectangle"): boolean => {
      if (!interactionDomain) return false;
      const targets: { readonly granularity: string; readonly id: string }[] = [];
      for (const block of blocks) {
        if (!block.interactionId) {
          if (method === "pick") return false;
          continue;
        }
        targets.push({ granularity: interactionDomain.granularity, id: block.interactionId });
      }
      dispatch("interactionSelect", { domainId: interactionDomain.domainId, targets: JSON.stringify(targets), merge, method });
      return true;
    },
    [dispatch, interactionDomain],
  );
  const publishInteractionHover = useCallback(
    (block: InkItem | null): boolean => {
      if (!interactionDomain || (block && !block.interactionId)) return false;
      const targets = block ? [{ granularity: interactionDomain.granularity, id: block.interactionId! }] : [];
      dispatch("interactionHover", { domainId: interactionDomain.domainId, channel: "pointer", targets: JSON.stringify(targets) });
      return true;
    },
    [dispatch, interactionDomain],
  );
  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();

  /** 🧺️ Cancels the scheduled live frame and hands back what it would have published. */
  const takePendingLive = useCallback((): InkPendingLive | null => {
    if (rafRef.current != null) {
      cancelAnimationFrame(rafRef.current);
      rafRef.current = null;
    }
    const pending = pendingLiveRef.current;
    pendingLiveRef.current = null;
    return pending;
  }, []);

  /** 📨️ One `inkApplyEvents` dispatch: a `stream` tick into the plugin's open ink transaction, the `commit` that
   * publishes it as ONE edit, or (no phase) a one-shot. */
  const dispatchInk = useCallback(
    (phase: "stream" | "commit" | null, events: readonly InkCanvasEvent[], gesture?: InkDragGesture, selectIds?: readonly string[]) => {
      dispatch(inkCanvasActions.applyEvents, { eventsJson: JSON.stringify(events), ...(phase ? { phase } : {}), ...(gesture ? { gestureJson: JSON.stringify(gesture) } : {}), ...(selectIds ? { selectIds: [...selectIds] } : {}) });
    },
    [dispatch],
  );

  const beginGesture = useCallback(
    (events: readonly InkCanvasEvent[], selectIds?: readonly string[]) => {
      gestureActiveRef.current = true;
      setDraftDoc((current) => applyEventsLocal(current ?? sceneDoc ?? { schema: "ink.document", id: "empty", camera: { x: 0, y: 0, zoom: 1 }, blocks: [] }, events));
      dispatchInk("stream", events, undefined, selectIds);
    },
    [dispatchInk, sceneDoc],
  );

  const liveGesture = useCallback(
    (events: readonly InkCanvasEvent[], gesture?: InkDragGesture) => {
      gestureActiveRef.current = true;
      setDraftDoc((current) => applyEventsLocal(current ?? sceneDoc ?? { schema: "ink.document", id: "empty", camera: { x: 0, y: 0, zoom: 1 }, blocks: [] }, events));
      const pending = pendingLiveRef.current;
      pendingLiveRef.current = { events: gesture ? (pending?.events ?? []) : [...(pending?.events ?? []), ...events], gesture: gesture ?? pending?.gesture };
      if (rafRef.current == null) {
        rafRef.current = requestAnimationFrame(() => {
          rafRef.current = null;
          const live = pendingLiveRef.current;
          pendingLiveRef.current = null;
          if (live) dispatchInk("stream", live.events, live.gesture);
        });
      }
    },
    [dispatchInk, sceneDoc],
  );

  const commitGesture = useCallback(
    (events: readonly InkCanvasEvent[], gesture?: InkDragGesture, selectIds?: readonly string[]) => {
      const pending = takePendingLive();
      gestureActiveRef.current = false;
      dispatchInk("commit", [...(pending?.events ?? []), ...events], gesture ?? pending?.gesture, selectIds);
    },
    [dispatchInk, takePendingLive],
  );

  /** 🧯️ A host cancel (lost capture, a second finger's pinch): the plugin drops the open ink transaction with zero
   * trace; a gesture that never streamed has nothing to drop. */
  const abortGesture = useCallback(() => {
    const active = gestureActiveRef.current;
    takePendingLive();
    gestureActiveRef.current = false;
    if (active) dispatch(inkCanvasActions.applyEvents, { eventsJson: "[]", phase: "abort", reason: "captureLost" });
  }, [dispatch, takePendingLive]);

  const atomicGesture = useCallback(
    (events: readonly InkCanvasEvent[], selectIds?: readonly string[]) => {
      dispatchInk(null, events, undefined, selectIds);
    },
    [dispatchInk],
  );

  const selectionBounds = useMemo(() => (doc ? inkSelectionBounds(doc.blocks, selectedIds) : null), [doc, selectedIds]);
  const utility = scene?.activeUtility || "selectDirect";
  const showResizeHandles = !isNavigator && (utility === "selectDirect" || utility === "selectMarquee") && Boolean(selectionBounds) && selectedIds.length > 0;
  return { abortGesture, atomicGesture, beginGesture, commitGesture, dispatch, liveGesture, mapContextMenu, publishInteractionHover, publishInteractionSelection, selectionBounds, shellContextMenuFallback, showResizeHandles, takePendingLive, utility };
}

export type InkCore2 = InkCore1 & ReturnType<typeof useInkGestures>;
// #endregion InkCanvasHost
