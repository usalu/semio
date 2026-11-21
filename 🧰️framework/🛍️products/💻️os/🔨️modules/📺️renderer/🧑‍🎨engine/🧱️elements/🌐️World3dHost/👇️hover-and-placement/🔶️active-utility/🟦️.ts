/** 🔶️ The active utility phase of useWorldHoverAndPlacement. */
import { useCallback, useEffect, useLayoutEffect, useMemo, useRef } from "../../../⚛️react-port/🟦️.ts";
import { isContextMenuPointerTarget } from "@semio-tech/ui-react";
import { createCoalescingActionDispatcher } from "../../../🛠️ShellHelpers/🛑️extension-io/🟦️.ts";
import { leftoverBrushRetainGuestHoverV1, leftoverWorldWindowOverlayV1, WORLD3D_DEFAULT_MARKER_GRANULARITY, world3dComponentInteractionTarget, world3dInstanceHoverTargetsJson, world3dInstanceInteractionTarget, world3dMarkerInteractionTarget, world3dSuggestionsAltHeld, world3dSuggestionsGestureArmed, type World3dSuggestionsRightDownRoute, world3dSuggestionsRightDownRoutes } from "../../🔗️instance-selection/🟦️.ts";
import { world3dGumballSelectionArgsV1 } from "../../🧮️gumball-math/🟦️.ts";
import { resolveWorldMergeMode } from "../../🎯️marquee-and-bridges/🟦️.tsx";
import { world3dHoverActionArgs, world3dSelectionTargetsActionArgs } from "../../🖱️interaction-domain-args/🟦️.ts";
import type { WorldCore3 } from "../../🏗️brush-mesh-and-menus/🟦️.ts";

type WorldHoverPublication = {
  readonly action: "interactionHover" | "setHover" | "worldVortexHover";
  readonly granularity: string;
  readonly id: string | null;
  readonly protectClear?: boolean;
  readonly aliasesJson?: string;
};

/** 🔷 Builds activeUtility, altHeldRef, brushMode, cameraRef of useWorldHoverAndPlacement. */
export function useActiveUtilityState({ core }: { readonly core: WorldCore3 }) {
  const { activeUtility, altHeldRef, brushMode, cameraRef, connectDropConsumedRef, dispatch, dispatchSettled, geometryRevisionRef, gridFactor, hostRef, hoveredInstanceInteractionTargetRef, hoveredVortexFullIdRef, instances, instancesRef, interactionDomainId, interactionGranularity, meshesRef, persistentSelectionMode, selection, selectionMode, setConnectDragHoverPosition, setConnectDragSource, setVortexPointerArm, setVoxelHoverOrigin, suggestionMenuOpen, suggestionMenuOwnsThisWindow, volumeBrushMode, vortices, vorticesRef, windowInstanceId } = core;
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

  const retainGuestHover = useCallback(() => Boolean(interactionDomainId) && leftoverBrushRetainGuestHoverV1(activeUtility, leftoverWorldWindowOverlayV1(windowInstanceId)), [activeUtility, interactionDomainId, windowInstanceId]);
  const publishHover = useMemo(() => createCoalescingActionDispatcher<WorldHoverPublication>((mark) => {
    if (mark.protectClear && mark.id === null && retainGuestHover()) return undefined;
    if (mark.action === "interactionHover") {
      return dispatchSettled(mark.action, world3dHoverActionArgs(interactionDomainId ?? "", mark.granularity, mark.id, mark.aliasesJson));
    }
    if (mark.action === "worldVortexHover") return dispatchSettled(mark.action, mark.id === null ? {} : { fullId: mark.id });
    return dispatchSettled(mark.action, mark.id === null ? {} : { objectId: mark.id, mode: "mesh", id: 0 });
  }, (left, right) => left.action === right.action && (left.aliasesJson && right.aliasesJson && left.id !== null && right.id !== null
    ? left.aliasesJson === right.aliasesJson
    : left.id === right.id && left.aliasesJson === right.aliasesJson
      && (left.id === null ? left.protectClear === right.protectClear : left.granularity === right.granularity))), [dispatchSettled, interactionDomainId, retainGuestHover]);

  const localPointerHover = useRef<WorldHoverPublication | null>(null);
  useLayoutEffect(() => () => { localPointerHover.current = null; }, [interactionDomainId, windowInstanceId]);
  useLayoutEffect(() => () => publishHover.clearPending(), [publishHover, windowInstanceId, interactionGranularity, geometryRevisionRef.current.revision]);

  const offerHover = useCallback((mark: WorldHoverPublication) => {
    localPointerHover.current = mark.id === null ? null : mark;
    publishHover(mark);
  }, [publishHover]);

  const dispatchInstanceHover = useCallback((id: string | null) => {
    if (!interactionDomainId) {
      offerHover({ action: "setHover", granularity: "mesh", id });
      return;
    }
    const records = instancesRef.current;
    const target = id === null ? null : world3dInstanceInteractionTarget(records, id, interactionGranularity);
    const aliasesJson = id === null ? undefined : world3dInstanceHoverTargetsJson(records.find((record) => record.id === id));
    offerHover({ action: "interactionHover", granularity: target?.granularity ?? interactionGranularity, id: target?.id ?? null, protectClear: id === null, aliasesJson });
  }, [instancesRef, interactionDomainId, interactionGranularity, offerHover]);

  const dispatchVortexHover = useCallback((fullId: string | null) => {
    if (!interactionDomainId) {
      offerHover({ action: "worldVortexHover", granularity: WORLD3D_DEFAULT_MARKER_GRANULARITY.vortex, id: fullId });
      return;
    }
    const target = fullId === null ? null : world3dMarkerInteractionTarget("vortex", fullId, vorticesRef.current.find((entry) => entry.fullId === fullId));
    offerHover({ action: "interactionHover", granularity: target?.granularity ?? WORLD3D_DEFAULT_MARKER_GRANULARITY.vortex, id: target?.id ?? null, protectClear: fullId === null });
  }, [interactionDomainId, offerHover, vorticesRef]);

  const localHoverReleaseListeners = useRef(new Set<() => void>());
  const subscribeLocalHoverRelease = useCallback((listener: () => void) => {
    localHoverReleaseListeners.current.add(listener);
    return () => {
      localHoverReleaseListeners.current.delete(listener);
    };
  }, []);
  const releaseLocalHover = useCallback(() => {
    hoveredInstanceInteractionTargetRef.current = null;
    const last = localPointerHover.current;
    localPointerHover.current = null;
    if (last) publishHover({ action: last.action, granularity: last.granularity, id: null, protectClear: last.action === "interactionHover" || undefined });
    for (const listener of localHoverReleaseListeners.current) listener();
  }, [hoveredInstanceInteractionTargetRef, publishHover]);
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

  const dispatchComponentHover = useCallback((args: { objectId: string; mode: string; id: number }) => {
    const target = world3dComponentInteractionTarget(instancesRef.current, meshesRef.current, args.objectId, args.mode, args.id);
    offerHover({ action: "interactionHover", granularity: target?.granularity ?? args.mode, id: target?.id ?? null });
  }, [instancesRef, meshesRef, offerHover]);
  return { interactionDomainId, dispatchComponentHover, dispatchInstanceHover, dispatch, hoveredVortexFullIdRef, dispatchVortexHover, selection, persistentSelectionMode, vortices, setVortexPointerArm, setConnectDragSource, setConnectDragHoverPosition, connectDropConsumedRef, hostRef, cameraRef, gridFactor, volumeBrushMode, setVoxelHoverOrigin, instancesRef, meshesRef, handleInstancePointerDown, handleInstancePointerMove, handleSuggestionClose, handleWorldOrbitRightPointerDown, publishHoverPaint, releaseLocalHover, selectionArgs, subscribeLocalHoverRelease, suggestionsRightDownDispatchedAtRef };
}
