/** 👇️ World3dHost hover and placement: the suggestions right-press route, the instance, vortex and component hover dispatch, the vortex arm, the connect drag, the voxel and brush placement and the world pick. */
import { useCallback, useEffect, useMemo, useRef } from "react";
import { isContextMenuPointerTarget } from "@semio-tech/ui-react";
import { type MergeMode } from "@semio-tech/framework";
import { createCoalescingActionDispatcher } from "../../🛠️ShellHelpers/🛑️extension-io/🟦️.ts";
import { leftoverBrushRetainGuestHoverV1, leftoverWorldWindowOverlayV1, WORLD3D_DEFAULT_MARKER_GRANULARITY, world3dComponentInteractionTarget, world3dInstanceInteractionTarget, world3dMarkerInteractionTarget, world3dSuggestionsAltHeld, world3dSuggestionsGestureArmed, type World3dSuggestionsRightDownRoute, world3dSuggestionsRightDownRoutes } from "../🔗️instance-selection/🟦️.ts";
import { world3dGumballSelectionArgsV1 } from "../🧮️gumball-math/🟦️.ts";
import { MARQUEE_DRAG_THRESHOLD_PX, raycastGroundPoint, resolveWorldMergeMode } from "../🎯️marquee-and-bridges/🟦️.tsx";
import { world3dVolumeBrushOriginV1 } from "../🗂️catalogue-drop/🟦️.tsx";
import { world3dHoverActionArgs, world3dSelectionActionArgs, world3dSelectionTargetsActionArgs } from "../🖱️interaction-domain-args/🟦️.ts";
import type { WorldCore3 } from "../🏗️brush-mesh-and-menus/🟦️.ts";

// #region World3dHost
/** 👇️ Wires the hover dispatch, the connect drag and the placement and pick handlers. */
export function useWorldHoverAndPlacement(core: WorldCore3) {
  const { activeUtility, altHeldRef, brushMode, cameraRef, connectDropConsumedRef, dispatch, dispatchSettled, gridFactor, hostRef, hoveredInstanceInteractionTargetRef, hoveredVortexFullIdRef, instances, instancesRef, interactionDomainId, interactionGranularity, meshesRef, persistentSelectionMode, selection, selectionMode, setConnectDragHoverPosition, setConnectDragSource, setVortexPointerArm, setVoxelHoverOrigin, suggestionMenuOpen, suggestionMenuOwnsThisWindow, volumeBrushMode, vortices, vorticesRef, windowInstanceId } = core;
  const handleWorldOrbitRightPointerDown = useCallback(
    (event: PointerEvent) => {
      const alt = world3dSuggestionsAltHeld(event.altKey, altHeldRef.current);
      const brushArmed = Boolean(brushMode && hoveredVortexFullIdRef.current);
      if (world3dSuggestionsGestureArmed(alt, hoveredVortexFullIdRef.current) || brushArmed) {
        setVortexPointerArm(null);
        setConnectDragSource(null);
        setConnectDragHoverPosition(null);
        dispatch("openVortexSuggestions", { fullId: hoveredVortexFullIdRef.current, x: event.clientX, y: event.clientY, windowId: windowInstanceId ?? undefined });
        return false;
      }
      return true;
    },
    [brushMode, dispatch, windowInstanceId],
  );
  const suggestionsRightDownDispatchedAtRef = useRef(0);
  useEffect(() => {
    const route: World3dSuggestionsRightDownRoute = {
      host: () => hostRef.current,
      handle: (event) => {
        const allowOrbit = handleWorldOrbitRightPointerDown(event);
        if (allowOrbit === false) suggestionsRightDownDispatchedAtRef.current = performance.now();
        return allowOrbit;
      },
    };
    world3dSuggestionsRightDownRoutes.add(route);
    return () => {
      world3dSuggestionsRightDownRoutes.delete(route);
    };
  }, [handleWorldOrbitRightPointerDown]);

  const handleSuggestionClose = useCallback(() => {
    setVortexPointerArm(null);
    setConnectDragSource(null);
    setConnectDragHoverPosition(null);
    dispatch("closeVortexSuggestions");
  }, [dispatch]);

  useEffect(() => {
    if (!suggestionMenuOpen || suggestionMenuOwnsThisWindow) return undefined;
    const handlePointerDown = (event: globalThis.PointerEvent): void => {
      if (isContextMenuPointerTarget(event.target)) return;
      handleSuggestionClose();
    };
    const handleKeyDown = (event: globalThis.KeyboardEvent): void => {
      if (event.key === "Escape") handleSuggestionClose();
    };
    window.addEventListener("pointerdown", handlePointerDown);
    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("pointerdown", handlePointerDown);
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [handleSuggestionClose, suggestionMenuOpen, suggestionMenuOwnsThisWindow]);

  const selectionArgs = useCallback(() => world3dGumballSelectionArgsV1(selection), [selection]);

  const handleInstancePointerDown = useCallback(
    (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => {
      const merge = resolveWorldMergeMode(selection.selectionMergeMode, event, persistentSelectionMode);
      const record = instances.find((entry) => entry.id === id);
      if (record?.disabled) {
        dispatch("worldPick", { granularity: selectionMode, id: null, merge });
        return;
      }
      if (interactionDomainId) {
        dispatch("interactionSelect", world3dSelectionTargetsActionArgs(interactionDomainId, [world3dInstanceInteractionTarget(instances, id, interactionGranularity)], merge));
        return;
      }
      if (selectionMode === "mesh" || selectionMode === "object") {
        dispatch("worldPick", { granularity: "mesh", id: index, merge });
        return;
      }
      dispatch("worldSelect", {
        ids: [id],
        merge,
      });
    },
    [dispatch, instances, interactionDomainId, interactionGranularity, persistentSelectionMode, selection.selectionMergeMode, selectionMode],
  );

  const dispatchInstanceHover = useMemo(
    () =>
      createCoalescingActionDispatcher<string | null>((id) => {
        if (interactionDomainId) {
          if (id == null && leftoverBrushRetainGuestHoverV1(activeUtility, leftoverWorldWindowOverlayV1(windowInstanceId))) return undefined;
          if (id == null) return dispatchSettled("interactionHover", world3dHoverActionArgs(interactionDomainId, interactionGranularity, null));
          const target = world3dInstanceInteractionTarget(instancesRef.current, id, interactionGranularity);
          return dispatchSettled("interactionHover", world3dHoverActionArgs(interactionDomainId, target.granularity, target.id));
        }
        if (id == null) return dispatchSettled("setHover", {});
        return dispatchSettled("setHover", { objectId: id, mode: "mesh", id: 0 });
      }),
    [activeUtility, dispatchSettled, interactionDomainId, interactionGranularity, windowInstanceId],
  );

  const dispatchVortexHover = useMemo(
    () =>
      createCoalescingActionDispatcher<string | null>((fullId) => {
        if (interactionDomainId) {
          if (!fullId && leftoverBrushRetainGuestHoverV1(activeUtility, leftoverWorldWindowOverlayV1(windowInstanceId))) return undefined;
          const target = fullId ? world3dMarkerInteractionTarget("vortex", fullId, vorticesRef.current.find((entry) => entry.fullId === fullId)) : null;
          const args = world3dHoverActionArgs(interactionDomainId, target?.granularity ?? WORLD3D_DEFAULT_MARKER_GRANULARITY.vortex, target?.id);
          return dispatchSettled("interactionHover", args);
        }
        if (!fullId) return dispatchSettled("worldVortexHover", {});
        return dispatchSettled("worldVortexHover", { fullId });
      }),
    [activeUtility, dispatchSettled, interactionDomainId, windowInstanceId],
  );

  const localHoverReleaseListeners = useRef(new Set<() => void>());
  const subscribeLocalHoverRelease = useCallback((listener: () => void) => {
    localHoverReleaseListeners.current.add(listener);
    return () => {
      localHoverReleaseListeners.current.delete(listener);
    };
  }, []);
  const releaseLocalHover = useCallback(() => {
    hoveredInstanceInteractionTargetRef.current = null;
    for (const listener of localHoverReleaseListeners.current) listener();
  }, []);
  const publishHoverPaint = useCallback((id: string | null) => {
    const root = hostRef.current;
    if (!root) return;
    if (id === null) root.removeAttribute("data-hover-paint-id");
    else root.setAttribute("data-hover-paint-id", id);
  }, []);

  const handleInstancePointerMove = useCallback(
    (id: string | null) => {
      hoveredInstanceInteractionTargetRef.current = id == null ? null : world3dInstanceInteractionTarget(instancesRef.current, id, interactionGranularity);
      dispatchInstanceHover(id);
    },
    [dispatchInstanceHover, interactionGranularity],
  );

  const dispatchComponentHover = useMemo(
    () =>
      createCoalescingActionDispatcher<{ objectId: string; mode: string; id: number }>((args) => {
        const target = world3dComponentInteractionTarget(instancesRef.current, meshesRef.current, args.objectId, args.mode, args.id);
        return dispatchSettled("interactionHover", world3dHoverActionArgs(interactionDomainId ?? "", target?.granularity ?? args.mode, target?.id ?? null));
      }),
    [dispatchSettled, interactionDomainId],
  );
  const handleComponentHover = useCallback(
    (args: { objectId: string; mode: string; id: number } | null) => {
      if (interactionDomainId) {
        if (args && args.mode !== "mesh" && args.mode !== "object") dispatchComponentHover(args);
        else dispatchInstanceHover(args?.objectId ?? null);
        return;
      }
      if (!args) {
        dispatch("setHover", {});
        return;
      }
      dispatch("setHover", args);
    },
    [dispatch, dispatchComponentHover, dispatchInstanceHover, interactionDomainId],
  );

  const handleVortexHover = useCallback(
    (fullId: string | null) => {
      if (fullId) hoveredVortexFullIdRef.current = fullId;
      if (fullId) dispatchVortexHover(fullId);
    },
    [dispatchVortexHover],
  );

  const handleVortexSelect = useCallback(
    (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      const merge = resolveWorldMergeMode(selection.selectionMergeMode, event ?? {}, persistentSelectionMode);
      if (interactionDomainId) {
        const target = world3dMarkerInteractionTarget("vortex", fullId, vortices.find((entry) => entry.fullId === fullId));
        dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, target.granularity, [target.id], merge));
        return;
      }
      dispatch("worldVortexSelect", { fullId, merge });
    },
    [dispatch, interactionDomainId, persistentSelectionMode, selection.selectionMergeMode, vortices],
  );

  const handleConnectDragStart = useCallback(
    (fullId: string, position: readonly [number, number, number]) => {
      setVortexPointerArm(null);
      const record = vortices.find((vortex) => vortex.fullId === fullId);
      setConnectDragSource({ fullId, position, record });
      setConnectDragHoverPosition(position);
    },
    [vortices],
  );

  const handleVortexPointerArm = useCallback(
    (arm: {
      readonly fullId: string;
      readonly position: readonly [number, number, number];
      readonly clientX: number;
      readonly clientY: number;
      readonly event: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean };
    }) => {
      setVortexPointerArm(arm);
    },
    [],
  );

  const handleVortexPointerMove = useCallback(
    (fullId: string, clientX: number, clientY: number) => {
      setVortexPointerArm((arm) => {
        if (!arm || arm.fullId !== fullId) return arm;
        const distance = Math.hypot(clientX - arm.clientX, clientY - arm.clientY);
        if (distance > MARQUEE_DRAG_THRESHOLD_PX) {
          handleConnectDragStart(arm.fullId, arm.position);
          return null;
        }
        return arm;
      });
    },
    [handleConnectDragStart],
  );

  const handleVortexPointerUp = useCallback(
    (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      setVortexPointerArm((arm) => {
        if (arm && arm.fullId === fullId) {
          handleVortexSelect(fullId, arm.event ?? event);
        }
        return null;
      });
    },
    [handleVortexSelect],
  );

  const handleConnectDragHover = useCallback((position: readonly [number, number, number]) => {
    setConnectDragHoverPosition(position);
  }, []);

  const handleConnectDragDrop = useCallback(
    (targetFullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
      connectDropConsumedRef.current = true;
      setConnectDragSource((source) => {
        if (source) {
          if (source.fullId === targetFullId) {
            handleVortexSelect(targetFullId, event);
          } else {
            dispatch("createAttraction", { attracting: source.fullId, attracted: targetFullId });
          }
        }
        return null;
      });
      setConnectDragHoverPosition(null);
    },
    [dispatch, handleVortexSelect],
  );

  const handleConnectDragCancel = useCallback(() => {
    setVortexPointerArm(null);
    setConnectDragSource(null);
    setConnectDragHoverPosition(null);
  }, []);

  const handleVoxelPlace = useCallback(
    (origin: readonly [number, number, number]) => {
      dispatch("addTargetVolume", { origin });
    },
    [dispatch],
  );

  /** 🧊️ The Volume Brush's ground read, from the host's own pointer coordinates — one raycast shared by
   * the hover ghost and the Alt+click commit, so the box the user sees is exactly the volume that lands. */
  const voxelGroundOriginAt = useCallback(
    (clientX: number, clientY: number): [number, number, number] | null => {
      const host = hostRef.current;
      const camera = cameraRef.current;
      if (!host || !camera) return null;
      return world3dVolumeBrushOriginV1(raycastGroundPoint(clientX, clientY, host.getBoundingClientRect(), camera), gridFactor);
    },
    [gridFactor],
  );

  useEffect(() => {
    if (!volumeBrushMode) setVoxelHoverOrigin(null);
  }, [volumeBrushMode]);

  /** ✅️ A brush click places the current free candidate the brush suggestions run found for the vortex under the
   * pointer — the one-shot `acceptSuggestion`, which answers with a notice when nothing free is known yet. */
  const handleBrushPlace = useCallback(() => {
    const fullId = hoveredVortexFullIdRef.current;
    if (fullId) dispatch("acceptSuggestion", { fullId });
  }, [dispatch]);

  const handleWorldPick = useCallback(
    (args: { granularity: string; id: number; merge: string; objectId?: string }) => {
      if (interactionDomainId && args.objectId != null && args.id != null) {
        const target = world3dComponentInteractionTarget(instancesRef.current, meshesRef.current, args.objectId, args.granularity, args.id);
        dispatch("interactionSelect", world3dSelectionTargetsActionArgs(interactionDomainId, target ? [target] : [], args.merge as MergeMode));
        return;
      }
      dispatch("worldPick", args);
    },
    [dispatch, interactionDomainId],
  );
  return { handleBrushPlace, handleComponentHover, handleConnectDragCancel, handleConnectDragDrop, handleConnectDragHover, handleInstancePointerDown, handleInstancePointerMove, handleSuggestionClose, handleVortexHover, handleVortexPointerArm, handleVortexPointerMove, handleVortexPointerUp, handleVortexSelect, handleVoxelPlace, handleWorldOrbitRightPointerDown, handleWorldPick, publishHoverPaint, releaseLocalHover, selectionArgs, subscribeLocalHoverRelease, suggestionsRightDownDispatchedAtRef, voxelGroundOriginAt };
}

export type WorldCore4 = WorldCore3 & ReturnType<typeof useWorldHoverAndPlacement>;
// #endregion World3dHost
