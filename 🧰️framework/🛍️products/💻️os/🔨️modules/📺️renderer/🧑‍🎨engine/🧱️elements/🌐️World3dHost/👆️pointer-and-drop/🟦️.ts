import type { WorldCore6 } from "../🎛️marquee-and-gumball/🟦️.ts";
import { useActiveUtilityState } from "./🔶️active-utility/🟦️.ts";
import { useHandlePointerUpState } from "./🔸️handle-pointer-up/🟦️.ts";
import { useCatalogueDragEnterState } from "./🟠️catalogue-drag-enter/🟦️.ts";

// #region World3dHost
/** 👆️ Wires the pointer handlers and the catalogue drag and drop. */
export function useWorldPointerAndDrop(core: WorldCore6) {
  const { gestureRecognizer, setMarqueePath, marqueeStartRef, endRelocateDrag, faceDragSession, setFaceDragSession, hostRef, cameraRef, dispatch, node, finalizeMarqueeSelection, marqueeDragActiveRef, paintMode, selection, activeUtility, toLocalPoint, instancesRef, meshesRef, handleInstancePointerDown, setVortexPointerArm, connectDropConsumedRef, handleConnectDragCancel, interactionDomainId, interactionGranularity, persistentSelectionMode, selectionMode, marqueeDown, wasMarqueeDragRef, contextMenu, dismissContextMenu, interaction, handleSuggestionClose, hoveredInstanceInteractionTargetRef, catalogueDragEncodedRef, catalogueDragDepthRef, gridSnapEnabled, gridFactor, pendingCelebrateCatalogueDropIdsRef, instances, scene, windowInstanceId, handlePointerDown, handlePointerMove } = useActiveUtilityState({ core });
  const { updateCatalogueDropPreviewAt, commitCatalogueDropAt, clearCatalogueDrop, handleEmptyClick, handlePointerUp } = useHandlePointerUpState({ gestureRecognizer, setMarqueePath, marqueeStartRef, endRelocateDrag, faceDragSession, setFaceDragSession, hostRef, cameraRef, dispatch, node, finalizeMarqueeSelection, marqueeDragActiveRef, paintMode, selection, activeUtility, toLocalPoint, instancesRef, meshesRef, handleInstancePointerDown, setVortexPointerArm, connectDropConsumedRef, handleConnectDragCancel, interactionDomainId, interactionGranularity, persistentSelectionMode, selectionMode, marqueeDown, wasMarqueeDragRef, contextMenu, dismissContextMenu, interaction, handleSuggestionClose, hoveredInstanceInteractionTargetRef, catalogueDragEncodedRef, catalogueDragDepthRef, gridSnapEnabled, gridFactor, pendingCelebrateCatalogueDropIdsRef, instances });
  const { onCatalogueDragEnter, onCatalogueDragLeave, onCatalogueDragOver, onCatalogueDrop } = useCatalogueDragEnterState({ scene, catalogueDragEncodedRef, catalogueDragDepthRef, updateCatalogueDropPreviewAt, commitCatalogueDropAt, clearCatalogueDrop, windowInstanceId, node, hostRef });
  return { handleEmptyClick, handlePointerDown, handlePointerMove, handlePointerUp, onCatalogueDragEnter, onCatalogueDragLeave, onCatalogueDragOver, onCatalogueDrop };
}

export type WorldCore7 = WorldCore6 & ReturnType<typeof useWorldPointerAndDrop>;
// #endregion World3dHost
