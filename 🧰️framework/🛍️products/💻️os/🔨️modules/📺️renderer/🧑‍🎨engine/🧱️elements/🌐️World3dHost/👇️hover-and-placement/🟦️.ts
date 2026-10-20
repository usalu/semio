import type { WorldCore3 } from "../🏗️brush-mesh-and-menus/🟦️.ts";
import { useActiveUtilityState } from "./🔶️active-utility/🟦️.ts";
import { useHandleComponentHoverState } from "./🔸️handle-component-hover/🟦️.ts";

// #region World3dHost
/** 👇️ Wires the hover dispatch, the connect drag and the placement and pick handlers. */
export function useWorldHoverAndPlacement(core: WorldCore3) {
  const { interactionDomainId, dispatchComponentHover, dispatchInstanceHover, dispatch, hoveredVortexFullIdRef, dispatchVortexHover, selection, persistentSelectionMode, vortices, setVortexPointerArm, setConnectDragSource, setConnectDragHoverPosition, connectDropConsumedRef, hostRef, cameraRef, gridFactor, volumeBrushMode, setVoxelHoverOrigin, instancesRef, meshesRef, handleInstancePointerDown, handleInstancePointerMove, handleSuggestionClose, handleWorldOrbitRightPointerDown, publishHoverPaint, releaseLocalHover, selectionArgs, subscribeLocalHoverRelease, suggestionsRightDownDispatchedAtRef } = useActiveUtilityState({ core });
  const { handleBrushPlace, handleComponentHover, handleConnectDragCancel, handleConnectDragDrop, handleConnectDragHover, handleVortexHover, handleVortexPointerArm, handleVortexPointerMove, handleVortexPointerUp, handleVortexSelect, handleVoxelPlace, handleWorldPick, voxelGroundOriginAt } = useHandleComponentHoverState({ interactionDomainId, dispatchComponentHover, dispatchInstanceHover, dispatch, hoveredVortexFullIdRef, dispatchVortexHover, selection, persistentSelectionMode, vortices, setVortexPointerArm, setConnectDragSource, setConnectDragHoverPosition, connectDropConsumedRef, hostRef, cameraRef, gridFactor, volumeBrushMode, setVoxelHoverOrigin, instancesRef, meshesRef });
  return { handleBrushPlace, handleComponentHover, handleConnectDragCancel, handleConnectDragDrop, handleConnectDragHover, handleInstancePointerDown, handleInstancePointerMove, handleSuggestionClose, handleVortexHover, handleVortexPointerArm, handleVortexPointerMove, handleVortexPointerUp, handleVortexSelect, handleVoxelPlace, handleWorldOrbitRightPointerDown, handleWorldPick, publishHoverPaint, releaseLocalHover, selectionArgs, subscribeLocalHoverRelease, suggestionsRightDownDispatchedAtRef, voxelGroundOriginAt };
}

export type WorldCore4 = WorldCore3 & ReturnType<typeof useWorldHoverAndPlacement>;
// #endregion World3dHost
