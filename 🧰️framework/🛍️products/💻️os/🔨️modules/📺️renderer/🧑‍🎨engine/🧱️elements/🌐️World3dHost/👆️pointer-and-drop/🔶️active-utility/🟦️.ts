/** 🔶️ The active utility phase of useWorldPointerAndDrop. */
import * as React from "../../../⚛️react-port/🟦️.ts";
import { useCallback, useEffect, useMemo } from "../../../⚛️react-port/🟦️.ts";
import { createCoalescingActionDispatcher } from "../../../🛠️ShellHelpers/🛑️extension-io/🟦️.ts";
import { world3dComponentInteractionTarget, world3dInstanceInteractionTargets, world3dLeftPressOrbits } from "../../🔗️instance-selection/🟦️.ts";
import { paneSuffixFromSurfaceId, raycastGroundPoint, world3dMarqueePointerCaptureArmed } from "../../🎯️marquee-and-bridges/🟦️.tsx";
import { world3dVolumeBrushCommits } from "../../🗂️catalogue-drop/🟦️.tsx";
import { world3dSelectionTargetsActionArgs } from "../../🖱️interaction-domain-args/🟦️.ts";
import type { WorldCore6 } from "../../🎛️marquee-and-gumball/🟦️.ts";

/** 🔷 Builds activeUtility, beginRelocateDrag, cameraRef, cameraState of useWorldPointerAndDrop. */
export function useActiveUtilityState({ core }: { readonly core: WorldCore6 }) {
  const { activeUtility, beginHandleDrag, beginRelocateDrag, cameraRef, cameraState, catalogueDragDepthRef, catalogueDragEncodedRef, connectDropConsumedRef, contextMenu, dismissContextMenu, dispatch, dispatchSettled, endHandleDrag, endRelocateDrag, faceDragSession, gestureRecognizer, gridFactor, gridSnapEnabled, handleConnectDragCancel, handleInstancePointerDown, handleSuggestionClose, handleVoxelPlace, hostRef, hoveredInstanceInteractionTargetRef, instances, instancesRef, interaction, interactionDomainId, interactionGranularity, marqueeDown, marqueeDragActiveRef, marqueeFinalizeOnceRef, marqueePreviewRef, marqueeStartRef, meshesRef, method, node, paintMode, pendingCelebrateCatalogueDropIdsRef, persistentSelectionMode, publishWorldPresenceView, relocateMode, scene, selection, selectionMode, setFaceDragSession, setMarqueeCommitHold, setMarqueeModifiers, setMarqueePath, setVortexPointerArm, setVoxelHoverOrigin, stepPaintGesture, toLocalPoint, updateHandleDrag, updateRelocateDrag, volumeBrushMode, voxelGroundOriginAt, wasMarqueeDragRef, windowInstanceId, yieldToPinch } = core;
  const sendEngagementPointerMove = useMemo(() => createCoalescingActionDispatcher<readonly [number, number, number]>(
    position => dispatchSettled("worldPointerMove", { pane: paneSuffixFromSurfaceId(node.surfaceId), position }),
    (left, right) => left[0] === right[0] && left[1] === right[1] && left[2] === right[2],
  ), [dispatchSettled, node.surfaceId]);
  useEffect(() => sendEngagementPointerMove.clearPending, [activeUtility, selection.engagementSessionActive, sendEngagementPointerMove]);
  const handlePointerDown = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      const verdict = gestureRecognizer.down({ pointerId: event.pointerId, x: event.clientX, y: event.clientY });
      if (verdict.kind === "pinchBegin") yieldToPinch(event);
      if (verdict.kind !== "single") return;
      if (event.button !== 0) return;
      if (beginHandleDrag(event)) return;
      if (relocateMode && beginRelocateDrag(event)) return;
      if (world3dVolumeBrushCommits(volumeBrushMode, event.altKey)) {
        const origin = voxelGroundOriginAt(event.clientX, event.clientY);
        if (origin) {
          setVoxelHoverOrigin(origin);
          handleVoxelPlace(origin);
          return;
        }
      }
      if (selection.engagementSessionActive && hostRef.current && cameraRef.current) {
        const rect = hostRef.current.getBoundingClientRect();
        const point = raycastGroundPoint(event.clientX, event.clientY, rect, cameraRef.current);
        if (point) {
          sendEngagementPointerMove.clearPending();
          dispatch("worldPointerDown", {
            pane: paneSuffixFromSurfaceId(node.surfaceId),
            position: point,
            shiftKey: event.shiftKey,
            ctrlKey: event.ctrlKey,
            metaKey: event.metaKey,
          });
          return;
        }
      }
      if (paintMode) stepPaintGesture({ kind: "press" });
      const start = toLocalPoint(event);
      marqueeStartRef.current = start;
      wasMarqueeDragRef.current = false;
      if (world3dLeftPressOrbits(event, { activeUtility: interaction.activeUtility, paintMode, engagementSessionActive: selection.engagementSessionActive })) return;
      setMarqueeModifiers({ shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
      marqueeFinalizeOnceRef.current = false;
      setMarqueePath([start]);
    },
    [interaction.activeUtility, wasMarqueeDragRef, beginHandleDrag, beginRelocateDrag, dispatch, gestureRecognizer, handleVoxelPlace, node.surfaceId, paintMode, relocateMode, selection.engagementSessionActive, sendEngagementPointerMove, stepPaintGesture, toLocalPoint, volumeBrushMode, voxelGroundOriginAt, yieldToPinch],
  );

  const handlePointerMove = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      if (hostRef.current && cameraRef.current) {
        const rect = hostRef.current.getBoundingClientRect();
        const hit = raycastGroundPoint(event.clientX, event.clientY, rect, cameraRef.current);
        const origin = cameraRef.current.position;
        publishWorldPresenceView(
          hit ?? (cameraState.target as [number, number, number]),
          [origin.x, origin.y, origin.z] as [number, number, number],
        );
      } else {
        publishWorldPresenceView(cameraState.target as [number, number, number], cameraState.position as [number, number, number]);
      }
      if (gestureRecognizer.move({ pointerId: event.pointerId, x: event.clientX, y: event.clientY }).kind !== "single") return;
      if (updateHandleDrag(event)) return;
      if (updateRelocateDrag(event)) return;
      if (volumeBrushMode) setVoxelHoverOrigin(voxelGroundOriginAt(event.clientX, event.clientY));
      if (selection.engagementSessionActive && hostRef.current && cameraRef.current) {
        const rect = hostRef.current.getBoundingClientRect();
        const point = raycastGroundPoint(event.clientX, event.clientY, rect, cameraRef.current);
        if (point) sendEngagementPointerMove(point);
        return;
      }
      if (!marqueeDown) return;
      const local = toLocalPoint(event);
      const start = marqueeStartRef.current;
      if (start && world3dMarqueePointerCaptureArmed(Math.hypot(local.x - start.x, local.y - start.y))) {
        if (!event.currentTarget.hasPointerCapture?.(event.pointerId)) {
          event.currentTarget.setPointerCapture?.(event.pointerId);
        }
      }
      setMarqueeModifiers({ shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
      setMarqueePath((path) => [...path, local]);
    },
    [gestureRecognizer, cameraState.position, cameraState.target, marqueeDown, publishWorldPresenceView, selection.engagementSessionActive, sendEngagementPointerMove, toLocalPoint, updateHandleDrag, updateRelocateDrag, volumeBrushMode, voxelGroundOriginAt],
  );

  const finalizeMarqueeSelection = useCallback(() => {
    if (!marqueeDragActiveRef.current || marqueeFinalizeOnceRef.current) return;
    const preview = marqueePreviewRef.current;
    if (!preview.mergedInstanceIds?.length && !preview.mergedComponentIds?.length) return;
    marqueeFinalizeOnceRef.current = true;
    if (preview.mergedInstanceIds?.length) {
      setMarqueeCommitHold({ mergedComponentIds: null, mergedInstanceIds: preview.mergedInstanceIds });
      const domainTargets = world3dInstanceInteractionTargets(instancesRef.current, preview.mergedInstanceIds, interactionGranularity);
      const marqueeSelect = interactionDomainId
        ? dispatch("interactionSelect", world3dSelectionTargetsActionArgs(interactionDomainId, domainTargets, "replace", method === "lasso" ? "lasso" : "rectangle"))
        : dispatch("worldSelect", { ids: preview.mergedInstanceIds, merge: "replace" });
      void Promise.resolve(marqueeSelect).finally(() => {
        window.setTimeout(() => setMarqueeCommitHold((hold) => (hold?.mergedInstanceIds === preview.mergedInstanceIds ? null : hold)), 250);
      });
    } else if (preview.mergedComponentIds?.length) {
      setMarqueeCommitHold({ mergedComponentIds: preview.mergedComponentIds, mergedInstanceIds: null });
      const activeObjectId = selection.activeObjectId;
      void Promise.resolve(
        interactionDomainId && activeObjectId
          ? dispatch(
              "interactionSelect",
              world3dSelectionTargetsActionArgs(
                interactionDomainId,
                preview.mergedComponentIds.flatMap((componentId) => { const target = world3dComponentInteractionTarget(instancesRef.current, meshesRef.current, activeObjectId, selectionMode, componentId); return target ? [target] : []; }),
                "replace",
                method === "lasso" ? "lasso" : "rectangle",
              ),
            )
          : dispatch("setSelection", {
              mode: selectionMode,
              ids: preview.mergedComponentIds,
              objectId: activeObjectId,
              merge: "replace",
            }),
      ).finally(() => {
        window.setTimeout(() => setMarqueeCommitHold((hold) => (hold?.mergedComponentIds === preview.mergedComponentIds ? null : hold)), 250);
      });
    }
    wasMarqueeDragRef.current = true;
    setMarqueePath([]);
  }, [dispatch, interactionDomainId, interactionGranularity, method, selection.activeObjectId, selectionMode]);
  return { gestureRecognizer, setMarqueePath, marqueeStartRef, endHandleDrag, endRelocateDrag, faceDragSession, setFaceDragSession, hostRef, cameraRef, dispatch, node, finalizeMarqueeSelection, marqueeDragActiveRef, paintMode, selection, activeUtility, toLocalPoint, instancesRef, meshesRef, handleInstancePointerDown, setVortexPointerArm, connectDropConsumedRef, handleConnectDragCancel, interactionDomainId, interactionGranularity, persistentSelectionMode, selectionMode, marqueeDown, wasMarqueeDragRef, contextMenu, dismissContextMenu, interaction, handleSuggestionClose, hoveredInstanceInteractionTargetRef, catalogueDragEncodedRef, catalogueDragDepthRef, gridSnapEnabled, gridFactor, pendingCelebrateCatalogueDropIdsRef, instances, scene, windowInstanceId, handlePointerDown, handlePointerMove };
}
