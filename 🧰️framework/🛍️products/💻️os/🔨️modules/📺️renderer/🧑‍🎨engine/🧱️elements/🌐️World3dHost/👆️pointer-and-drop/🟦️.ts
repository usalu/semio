/** 👆️ World3dHost pointer and drop: the pointer down, move and up handlers, the marquee finalisation, the empty click, the catalogue drop and its effects. */
import React, { useCallback, useEffect, type DragEvent } from "react";
import { CATALOGUE_DRAG_MIME, getActiveCatalogueDragPayload } from "@semio-tech/ui-react";
import { celebrateWorldInstances } from "../🎨️mesh-render/🎊️instance-celebration/🟦️.ts";
import { clientPointOverHost, hoveredInteractionTargetV1, world3dComponentInteractionTarget, world3dInstanceInteractionTargets, worldInstancePickBlocked } from "../🔗️instance-selection/🟦️.ts";
import { paneSuffixFromSurfaceId, raycastGroundPoint, resolveClickInstanceId, resolveWorldMergeMode, world3dMarqueePointerCaptureArmed } from "../🎯️marquee-and-bridges/🟦️.tsx";
import { clearWorldCatalogueDropPreview, getWorldCatalogueDropPreview, parsePuzzle3dCatalogueDragPayload, registerWorldCatalogueDropHost, resolveCatalogueDropOrigin, setWorldCatalogueDropPreview, world3dVolumeBrushCommits, worldCatalogueDropHostContainsPoint } from "../🗂️catalogue-drop/🟦️.tsx";
import { axisDragParam } from "../🪟️window-view-store/🟦️.tsx";
import { world3dSelectionActionArgs, world3dSelectionTargetsActionArgs } from "../🖱️interaction-domain-args/🟦️.ts";
import type { WorldCore6 } from "../🎛️marquee-and-gumball/🟦️.ts";

// #region World3dHost
/** 👆️ Wires the pointer handlers and the catalogue drag and drop. */
export function useWorldPointerAndDrop(core: WorldCore6) {
  const { activeUtility, beginRelocateDrag, cameraRef, cameraState, catalogueDragDepthRef, catalogueDragEncodedRef, connectDropConsumedRef, contextMenu, dismissContextMenu, dispatch, endRelocateDrag, engagementPointerMoveInFlightRef, engagementPointerMoveLastPointRef, faceDragSession, gestureRecognizer, gridFactor, gridSnapEnabled, handleConnectDragCancel, handleInstancePointerDown, handleSuggestionClose, handleVoxelPlace, hostRef, hoveredInstanceInteractionIdRef, instances, instancesRef, interaction, interactionDomainId, interactionGranularity, marqueeDown, marqueeDragActiveRef, marqueeFinalizeOnceRef, marqueePreviewRef, marqueeStartRef, meshesRef, method, node, paintMode, pendingCelebrateCatalogueDropIdsRef, persistentSelectionMode, publishWorldPresenceView, relocateMode, scene, selection, selectionMode, setFaceDragSession, setMarqueeCommitHold, setMarqueeModifiers, setMarqueePath, setVortexPointerArm, setVoxelHoverOrigin, stepPaintGesture, toLocalPoint, updateRelocateDrag, volumeBrushMode, voxelGroundOriginAt, wasMarqueeDragRef, windowInstanceId, yieldToPinch } = core;
  const handlePointerDown = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      const verdict = gestureRecognizer.down({ pointerId: event.pointerId, x: event.clientX, y: event.clientY });
      if (verdict.kind === "pinchBegin") yieldToPinch(event);
      if (verdict.kind !== "single") return;
      if (event.button !== 0) return;
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
      setMarqueeModifiers({ shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey });
      marqueeFinalizeOnceRef.current = false;
      const start = toLocalPoint(event);
      marqueeStartRef.current = start;
      setMarqueePath([start]);
    },
    [beginRelocateDrag, dispatch, gestureRecognizer, handleVoxelPlace, node.surfaceId, paintMode, relocateMode, selection.engagementSessionActive, stepPaintGesture, toLocalPoint, volumeBrushMode, voxelGroundOriginAt, yieldToPinch],
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
      if (updateRelocateDrag(event)) return;
      if (volumeBrushMode) setVoxelHoverOrigin(voxelGroundOriginAt(event.clientX, event.clientY));
      if (selection.engagementSessionActive && hostRef.current && cameraRef.current) {
        const rect = hostRef.current.getBoundingClientRect();
        const point = raycastGroundPoint(event.clientX, event.clientY, rect, cameraRef.current);
        const last = engagementPointerMoveLastPointRef.current;
        const unchanged = point && last && point[0] === last[0] && point[1] === last[1] && point[2] === last[2];
        if (point && !unchanged && !engagementPointerMoveInFlightRef.current) {
          engagementPointerMoveInFlightRef.current = true;
          engagementPointerMoveLastPointRef.current = point;
          requestAnimationFrame(() => {
            engagementPointerMoveInFlightRef.current = false;
            dispatch("worldPointerMove", { pane: paneSuffixFromSurfaceId(node.surfaceId), position: point });
          });
        }
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
    [gestureRecognizer, cameraState.position, cameraState.target, dispatch, marqueeDown, node.surfaceId, publishWorldPresenceView, selection.engagementSessionActive, toLocalPoint, updateRelocateDrag, volumeBrushMode, voxelGroundOriginAt],
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
      // 🧹️ Consume the post-marquee suppress flag so a stale `true` cannot permanently no-operation background deselect.
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
      // 🎯️ A pane must not CLEAR its selection at a point where it is publishing a hover target. The
      // two hit tests are not the same code — hover rides R3F's raycast against the real geometry, the
      // click's "did I hit anything" is R3F's own `onPointerMissed` bookkeeping — and on the
      // generation3d preview they disagree: sweeping the pointer resolves `extrude@solid` and clicking
      // that same point ten milliseconds later arrives here, so the pick dispatched
      // `interactionSelect targets:[]` and the object was never selectable at all
      // (measured 26/09/15 on :6023, `📓️hot-swap-board-remount-2026-09-15.md` §3;
      // `🐍️world-pick-recon.mjs` records both halves). The hover this pane published IS the target under
      // the pointer, so a click there is a pick of it; a click on genuinely empty canvas has no hover
      // and still clears.
      const hovered = hoveredInteractionTargetV1(hoveredInstanceInteractionIdRef.current ?? selection.hoveredId);
      if (interactionDomainId && hovered) {
        dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, hovered.granularity, [hovered.id], merge));
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
    [contextMenu, dismissContextMenu, dispatch, handleSuggestionClose, interaction.suggestionMenu?.open, interactionDomainId, interactionGranularity, paintMode, persistentSelectionMode, selection.engagementSessionActive, selection.hoveredId, selection.selectionMergeMode, selectionMode],
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
      // 🎉️ Stamp the pre-drop instance set so the next scene update can celebrate the newly placed object(s).
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

  const onCatalogueDragEnter = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      if (!event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME) && !getActiveCatalogueDragPayload()) return;
      event.preventDefault();
      const encoded = getActiveCatalogueDragPayload();
      if (encoded) catalogueDragEncodedRef.current = encoded;
      catalogueDragDepthRef.current += 1;
      updateCatalogueDropPreviewAt(event.clientX, event.clientY);
    },
    [scene, updateCatalogueDropPreviewAt],
  );

  const onCatalogueDragLeave = useCallback(
    (_event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      catalogueDragDepthRef.current = Math.max(0, catalogueDragDepthRef.current - 1);
    },
    [scene],
  );

  const onCatalogueDragOver = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      if (!event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME) && !getActiveCatalogueDragPayload()) return;
      const encoded = getActiveCatalogueDragPayload();
      if (!parsePuzzle3dCatalogueDragPayload(encoded) && !event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) return;
      event.preventDefault();
      event.dataTransfer.dropEffect = "copy";
      if (encoded) catalogueDragEncodedRef.current = encoded;
      updateCatalogueDropPreviewAt(event.clientX, event.clientY);
    },
    [scene, updateCatalogueDropPreviewAt],
  );

  const onCatalogueDrop = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!scene) return;
      event.preventDefault();
      const encoded = event.dataTransfer.getData(CATALOGUE_DRAG_MIME) || getActiveCatalogueDragPayload() || catalogueDragEncodedRef.current;
      commitCatalogueDropAt(event.clientX, event.clientY, encoded);
      clearCatalogueDrop();
    },
    [clearCatalogueDrop, commitCatalogueDropAt, scene],
  );

  useEffect(() => {
    const hostId = windowInstanceId ?? node.surfaceId;
    return registerWorldCatalogueDropHost(node.controllerId, hostId, (clientX, clientY) => {
      const host = hostRef.current;
      if (!host) return false;
      return clientPointOverHost(clientX, clientY, host.getBoundingClientRect());
    });
  }, [node.controllerId, node.surfaceId, windowInstanceId]);

  useEffect(() => {
    // 🧭️ The DOM `DragEvent`, not React's synthetic one (imported above for JSX props): this listener
    // is installed on `window`, which delivers the browser's own event.
    const onWindowDragOver = (event: globalThis.DragEvent) => {
      if (!getActiveCatalogueDragPayload() && !event.dataTransfer?.types.includes(CATALOGUE_DRAG_MIME)) return;
      if (!parsePuzzle3dCatalogueDragPayload(getActiveCatalogueDragPayload()) && !event.dataTransfer?.types.includes(CATALOGUE_DRAG_MIME)) return;
      event.preventDefault();
      const encoded = getActiveCatalogueDragPayload();
      if (encoded) catalogueDragEncodedRef.current = encoded;
      updateCatalogueDropPreviewAt(event.clientX, event.clientY);
    };

    const onPointerMove = (event: PointerEvent) => {
      const encoded = getActiveCatalogueDragPayload();
      if (!encoded) {
        if (getWorldCatalogueDropPreview(node.controllerId)) clearWorldCatalogueDropPreview(node.controllerId);
        return;
      }
      if (!parsePuzzle3dCatalogueDragPayload(encoded)) return;
      catalogueDragEncodedRef.current = encoded;
      updateCatalogueDropPreviewAt(event.clientX, event.clientY);
    };

    const onPointerUp = (event: PointerEvent) => {
      const encoded = getActiveCatalogueDragPayload() ?? catalogueDragEncodedRef.current;
      const payload = parsePuzzle3dCatalogueDragPayload(encoded);
      if (!payload) return;
      const host = hostRef.current;
      if (!host) return;
      const rect = host.getBoundingClientRect();
      if (clientPointOverHost(event.clientX, event.clientY, rect)) {
        commitCatalogueDropAt(event.clientX, event.clientY, encoded);
        clearCatalogueDrop();
        return;
      }
      if (!worldCatalogueDropHostContainsPoint(node.controllerId, event.clientX, event.clientY)) clearCatalogueDrop();
    };

    const onDragEnd = () => {
      queueMicrotask(() => {
        if (!getActiveCatalogueDragPayload()) clearCatalogueDrop();
      });
    };

    window.addEventListener("dragover", onWindowDragOver, true);
    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp, true);
    window.addEventListener("dragend", onDragEnd);
    return () => {
      window.removeEventListener("dragover", onWindowDragOver, true);
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp, true);
      window.removeEventListener("dragend", onDragEnd);
    };
  }, [clearCatalogueDrop, commitCatalogueDropAt, node.controllerId, updateCatalogueDropPreviewAt]);
  return { handleEmptyClick, handlePointerDown, handlePointerMove, handlePointerUp, onCatalogueDragEnter, onCatalogueDragLeave, onCatalogueDragOver, onCatalogueDrop };
}

export type WorldCore7 = WorldCore6 & ReturnType<typeof useWorldPointerAndDrop>;
// #endregion World3dHost
