/** 🔸️ The handle pointer up phase of useWorldPointerAndDrop. */
import React, { useCallback, useEffect } from "react";
import { getActiveCatalogueDragPayload } from "@semio-tech/ui-react";
import { celebrateWorldInstances } from "../../🎨️mesh-render/🎊️instance-celebration/🟦️.ts";
import { clientPointOverHost, hoveredInteractionTargetV1, worldInstancePickBlocked } from "../../🔗️instance-selection/🟦️.ts";
import { paneSuffixFromSurfaceId, resolveClickInstanceId, resolveWorldMergeMode } from "../../🎯️marquee-and-bridges/🟦️.tsx";
import { clearWorldCatalogueDropPreview, parsePuzzle3dCatalogueDragPayload, resolveCatalogueDropOrigin, setWorldCatalogueDropPreview, worldCatalogueDropHostContainsPoint } from "../../🗂️catalogue-drop/🟦️.tsx";
import { axisDragParam } from "../../🪟️window-view-store/🟦️.tsx";
import { world3dSelectionActionArgs, world3dSelectionTargetsActionArgs } from "../../🖱️interaction-domain-args/🟦️.ts";
import type { useActiveUtilityState } from "../🔶️active-utility/🟦️.ts";

/** 🔹 Builds handlePointerUp, handleEmptyClick, clearCatalogueDrop, readCatalogueDragEncoded of useWorldPointerAndDrop. */
export function useHandlePointerUpState({ gestureRecognizer, setMarqueePath, marqueeStartRef, endRelocateDrag, faceDragSession, setFaceDragSession, hostRef, cameraRef, dispatch, node, finalizeMarqueeSelection, marqueeDragActiveRef, paintMode, selection, activeUtility, toLocalPoint, instancesRef, meshesRef, handleInstancePointerDown, setVortexPointerArm, connectDropConsumedRef, handleConnectDragCancel, interactionDomainId, interactionGranularity, persistentSelectionMode, selectionMode, marqueeDown, wasMarqueeDragRef, contextMenu, dismissContextMenu, interaction, handleSuggestionClose, hoveredInstanceInteractionTargetRef, catalogueDragEncodedRef, catalogueDragDepthRef, gridSnapEnabled, gridFactor, pendingCelebrateCatalogueDropIdsRef, instances }: Pick<ReturnType<typeof useActiveUtilityState>, "gestureRecognizer" | "setMarqueePath" | "marqueeStartRef" | "endRelocateDrag" | "faceDragSession" | "setFaceDragSession" | "hostRef" | "cameraRef" | "dispatch" | "node" | "finalizeMarqueeSelection" | "marqueeDragActiveRef" | "paintMode" | "selection" | "activeUtility" | "toLocalPoint" | "instancesRef" | "meshesRef" | "handleInstancePointerDown" | "setVortexPointerArm" | "connectDropConsumedRef" | "handleConnectDragCancel" | "interactionDomainId" | "interactionGranularity" | "persistentSelectionMode" | "selectionMode" | "marqueeDown" | "wasMarqueeDragRef" | "contextMenu" | "dismissContextMenu" | "interaction" | "handleSuggestionClose" | "hoveredInstanceInteractionTargetRef" | "catalogueDragEncodedRef" | "catalogueDragDepthRef" | "gridSnapEnabled" | "gridFactor" | "pendingCelebrateCatalogueDropIdsRef" | "instances">) {
  const handlePointerUp = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      const verdict = gestureRecognizer.up(event.pointerId);
      if (event.currentTarget.hasPointerCapture?.(event.pointerId)) {
        event.currentTarget.releasePointerCapture(event.pointerId);
      }
      if (verdict.kind !== "single") {
        setMarqueePath([]);
        marqueeStartRef.current = null;
        return;
      }
      if (endRelocateDrag(event)) return;
      if (faceDragSession) {
        const session = faceDragSession;
        setFaceDragSession(null);
        if (hostRef.current && cameraRef.current) {
          const rect = hostRef.current.getBoundingClientRect();
          const distance = axisDragParam(event.clientX, event.clientY, rect, cameraRef.current, session.startPoint, session.normal);
          if (distance != null && Math.abs(distance) > 1e-4) {
            dispatch("worldFaceDragEnd", {
              pane: paneSuffixFromSurfaceId(node.surfaceId),
              objectId: session.objectId,
              faceId: session.faceId,
              normal: session.normal,
              startPoint: session.startPoint,
              distance,
              faceExtent: session.faceExtent,
            });
          }
        }
        return;
      }
      finalizeMarqueeSelection();
      if (!marqueeDragActiveRef.current && !paintMode && !selection.engagementSessionActive && !worldInstancePickBlocked(activeUtility)) {
        const host = hostRef.current;
        const camera = cameraRef.current;
        if (host && camera) {
          const rect = host.getBoundingClientRect();
          const local = toLocalPoint(event);
          const id = resolveClickInstanceId(instancesRef.current, meshesRef.current, local, rect, camera);
          if (id) {
            const index = instancesRef.current.findIndex((entry) => entry.id === id);
            handleInstancePointerDown(id, index < 0 ? 0 : index, event);
          }
        }
      }
      setMarqueePath([]);
      marqueeStartRef.current = null;
      setVortexPointerArm(null);
      if (connectDropConsumedRef.current) {
        connectDropConsumedRef.current = false;
      } else {
        handleConnectDragCancel();
      }
    },
    [gestureRecognizer, activeUtility, dispatch, endRelocateDrag, faceDragSession, finalizeMarqueeSelection, handleConnectDragCancel, handleInstancePointerDown, interactionDomainId, interactionGranularity, node.surfaceId, paintMode, persistentSelectionMode, selection.engagementSessionActive, selection.selectionMergeMode, selectionMode, toLocalPoint],
  );

  useEffect(() => {
    if (!marqueeDown || selection.engagementSessionActive || paintMode) return undefined;
    const onWindowPointerUp = (event: PointerEvent): void => {
      if (event.button !== 0) return;
      finalizeMarqueeSelection();
    };
    window.addEventListener("pointerup", onWindowPointerUp, true);
    window.addEventListener("pointercancel", onWindowPointerUp, true);
    return () => {
      window.removeEventListener("pointerup", onWindowPointerUp, true);
      window.removeEventListener("pointercancel", onWindowPointerUp, true);
    };
  }, [finalizeMarqueeSelection, marqueeDown, paintMode, selection.engagementSessionActive]);

  const handleEmptyClick = useCallback(
    (event: globalThis.MouseEvent) => {
      if (wasMarqueeDragRef.current) {
        wasMarqueeDragRef.current = false;
        return;
      }
      if (selection.engagementSessionActive || paintMode) return;
      if (contextMenu) dismissContextMenu();
      if (interaction.suggestionMenu?.open) {
        handleSuggestionClose();
      }
      const merge = resolveWorldMergeMode(selection.selectionMergeMode, event, persistentSelectionMode);
      const hovered = hoveredInteractionTargetV1(hoveredInstanceInteractionTargetRef.current);
      if (interactionDomainId && hovered) {
        dispatch("interactionSelect", world3dSelectionTargetsActionArgs(interactionDomainId, [hovered], merge));
        return;
      }
      if (interactionDomainId) {
        dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, interactionGranularity, [], merge));
        return;
      }
      if (hovered) {
        dispatch("worldPick", { granularity: selectionMode, id: hovered.id, merge });
        return;
      }
      dispatch("worldPick", { granularity: selectionMode, id: null, merge });
    },
    [contextMenu, dismissContextMenu, dispatch, handleSuggestionClose, interaction.suggestionMenu?.open, interactionDomainId, interactionGranularity, paintMode, persistentSelectionMode, selection.engagementSessionActive, selection.selectionMergeMode, selectionMode],
  );

  const clearCatalogueDrop = useCallback(() => {
    catalogueDragEncodedRef.current = null;
    catalogueDragDepthRef.current = 0;
    clearWorldCatalogueDropPreview(node.controllerId);
  }, [node.controllerId]);

  const readCatalogueDragEncoded = useCallback((): string | null => {
    return getActiveCatalogueDragPayload() ?? catalogueDragEncodedRef.current;
  }, []);

  const updateCatalogueDropPreviewAt = useCallback(
    (clientX: number, clientY: number) => {
      const encoded = readCatalogueDragEncoded();
      const payload = parsePuzzle3dCatalogueDragPayload(encoded);
      if (!payload || !hostRef.current || !cameraRef.current) return;
      const rect = hostRef.current.getBoundingClientRect();
      if (!clientPointOverHost(clientX, clientY, rect)) {
        if (!worldCatalogueDropHostContainsPoint(node.controllerId, clientX, clientY)) clearWorldCatalogueDropPreview(node.controllerId);
        return;
      }
      const origin = resolveCatalogueDropOrigin(clientX, clientY, rect, cameraRef.current, gridSnapEnabled, gridFactor);
      if (!origin) return;
      if (encoded) catalogueDragEncodedRef.current = encoded;
      setWorldCatalogueDropPreview(node.controllerId, { ...payload, origin });
    },
    [gridFactor, gridSnapEnabled, node.controllerId, readCatalogueDragEncoded],
  );

  const commitCatalogueDropAt = useCallback(
    (clientX: number, clientY: number, encoded?: string | null) => {
      const payload = parsePuzzle3dCatalogueDragPayload(encoded ?? readCatalogueDragEncoded());
      if (!payload || !hostRef.current || !cameraRef.current) return;
      const rect = hostRef.current.getBoundingClientRect();
      const origin = resolveCatalogueDropOrigin(clientX, clientY, rect, cameraRef.current, gridSnapEnabled, gridFactor);
      if (!origin) return;
      pendingCelebrateCatalogueDropIdsRef.current = new Set(instancesRef.current.map((instance) => instance.id));
      dispatch("addObjectKind", { objectKind: payload.objectKind, origin });
    },
    [dispatch, gridFactor, gridSnapEnabled, readCatalogueDragEncoded],
  );

  useEffect(() => {
    const baseline = pendingCelebrateCatalogueDropIdsRef.current;
    if (!baseline) return;
    const added = instances.filter((instance) => !baseline.has(instance.id)).map((instance) => instance.id);
    if (added.length === 0) return;
    pendingCelebrateCatalogueDropIdsRef.current = null;
    celebrateWorldInstances(added);
  }, [instances]);
  return { updateCatalogueDropPreviewAt, commitCatalogueDropAt, clearCatalogueDrop, handleEmptyClick, handlePointerUp };
}
