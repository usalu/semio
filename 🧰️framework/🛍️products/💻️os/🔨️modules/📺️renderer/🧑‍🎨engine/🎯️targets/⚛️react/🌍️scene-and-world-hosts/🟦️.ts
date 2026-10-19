/** 🌍️ Scene and world host exports of the renderer entry: the terrain layer, the canvas 2d and world 3d hosts, the node graph host, the text editor, table, paint 2d and tiled map hosts. */
import { WorldTerrainLayer, parseWorldTerrainStyle, type WorldTerrainStyle } from "@semio-tech/framework-os/modules/renderer/engine/elements/WorldTerrainLayer";
import { CAMERA_SYNC_DEBOUNCE_MS, Canvas2dHost } from "@semio-tech/framework-os/modules/renderer/engine/elements/Canvas2dHost";
import { CANVAS_CAMERA_ZOOM_MIN, CANVAS_CAMERA_ZOOM_MAX, type CanvasCamera, clampCanvasZoom, worldToScreenLogical, screenToWorldLogical, wheelCameraAtScreen } from "@semio-tech/framework-os/modules/renderer/engine/elements/Canvas2dHost/camera-math";
import { readCanvas2dSurfaceColors, canvasLayerDisplayLabel, type CanvasLayerLabelInput } from "@semio-tech/framework-os/modules/renderer/engine/elements/Canvas2dHost/layer-paint";
import { World3dHost } from "@semio-tech/framework-os/modules/renderer/engine/elements/World3dHost";
import { worldMeshMaterialRevision, resolveMeshStyle, resolveMeshSelectionPreviewStyle, type SemanticColors, semanticColorsFromPalette } from "@semio-tech/framework-os/modules/renderer/engine/elements/World3dHost/mesh-render/mesh-style-palette";
import { celebrateWorldInstances, isWorldInstanceCelebrating } from "@semio-tech/framework-os/modules/renderer/engine/elements/World3dHost/mesh-render/instance-celebration";
import { isCurveOnlyWorldMesh } from "@semio-tech/framework-os/modules/renderer/engine/elements/World3dHost/mesh-render/mesh-geometry-and-appearance";
import { meshBoundsCorners } from "@semio-tech/framework-os/modules/renderer/engine/elements/World3dHost/mesh-render/glb-bounds-and-styling";
import { worldGhostMeshUrl, suggestionMenuItems, resolveWorldMergeMode, raycastGroundPoint } from "@semio-tech/framework-os/modules/renderer/engine/elements/World3dHost/marquee-and-bridges";
import { interactionTargetsForInstances, WORLD3D_DEFAULT_INTERACTION_GRANULARITY, WORLD3D_DEFAULT_MARKER_GRANULARITY, world3dMarkerInteractionTarget, world3dInstanceInteractionTarget, world3dInstanceInteractionTargets, type World3dMarkerLayer, resolveWorldContextMenuTarget, worldInstancePickBlocked, resolveVortexPointerDownIntent, world3dContextMenuSurfaceV1 } from "@semio-tech/framework-os/modules/renderer/engine/elements/World3dHost/instance-selection";
import { type World3dMarkerInteractionFields, type WorldInstanceRecord, mergeWorldViewportCamera, world3dViewportCameraSeedKey, world3dFitProjectionContent, world3dFramingInstances, world3dProjectionContentFrameMounted, world3dFrameVisibleOverlayOffered, buildWorldCameraDispatchArgs, worldCameraSetCameraDispatchArgs, worldCameraPoseApproxEqual, shouldReattachWorldViewportCamera, world3dCameraDomJson } from "@semio-tech/framework-os/modules/renderer/engine/elements/World3dHost/scene-schema";
import { world3dRelocateDragTargetV1, world3dRelocateDispatchArgsV1, world3dVolumeBrushOriginV1, world3dVolumeBrushCommits, parsePuzzle3dCatalogueDragPayload, snapWorldPointToGrid, subscribeWorldCatalogueDropPreview, getWorldCatalogueDropPreview, getWorldCatalogueDropPreviewServerSnapshot, setWorldCatalogueDropPreview, clearWorldCatalogueDropPreview, registerWorldCatalogueDropHost, worldCatalogueDropHostContainsPoint, type WorldSelectionPreview, subscribeWorldSelectionPreview, getWorldSelectionPreview, getWorldSelectionPreviewServerSnapshot, setWorldSelectionPreview, clearWorldSelectionPreview } from "@semio-tech/framework-os/modules/renderer/engine/elements/World3dHost/catalogue-drop";
import { isWorldTransformGumballMode, gumballTransformDeltaBetweenPoses, gumballIdentityDelta, worldPaintStep, WORLD_PAINT_IDLE, type WorldPaintGesture, type WorldPaintEvent, type WorldPaintOrigin, world3dGumballSelectionArgsV1, type WorldGumballLivePreviewDelta, gumballLivePreviewDeltaBetweenPoses, applyGumballLivePreviewDeltaToPose, applyGumballLivePreviewPoseToObject3D, gumballConfigForTransformMode, worldGumballConfigForProjection, type WorldGumballTransformPreview, subscribeWorldGumballTransformPreview, getWorldGumballTransformPreview, getWorldGumballTransformPreviewServerSnapshot, setWorldGumballTransformPreview, clearWorldGumballTransformPreview, worldVorticesWithGumballPreview, worldAttractionsWithGumballPreview } from "@semio-tech/framework-os/modules/renderer/engine/elements/World3dHost/gumball-math";
import { worldVortexMaterialRevision } from "@semio-tech/framework-os/modules/renderer/engine/elements/World3dHost/instance-layers/vortex-markers";
import { registerPendingWorldProjection, clearPendingWorldProjection, world3dProjectionPaneElementId, WorldOrbitProjectionSwitchPane } from "@semio-tech/framework-os/modules/renderer/engine/elements/World3dHost/window-view-store";
import { world3dHoverActionArgs, world3dSelectionActionArgs, world3dSelectionTargetsActionArgs } from "@semio-tech/framework-os/modules/renderer/engine/elements/World3dHost/interaction-domain-args";
import { mapContextMenuSpecs } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/surface-menu";
import { parseJsonArray, parseSelectionDomainsFromSession, selectionGroupsFromDomains, WindowInstanceIdContext } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/scene-host-contract";
import { alignModeToDag, catalogueGhostDescriptorJson, type CatalogueAppDragPayload, computeDagMarqueeOverlay, dagElementInteractionChrome, dagOverlayLabelFill, dagOverlayLabelFillHex, dagContentBounds, dagContentCoverage, dagEllipsizeByMeasure, dagFitCamera, dagScreenToWorld, dagStartupCamera, dagWorldToScreen, nodeGraphContentSignature, DAG_CONTENT_FIT_PADDING_PX, DAG_CONTENT_FRAMED_MIN_COVERAGE, DAG_CONTENT_REFIT_MAX_COVERAGE, DAG_LABEL_ELLIPSIS, type DagCameraState, type DagContentBounds, type DagLabelOverlayInteraction, type DagLabelOverlayRow, type DagMarqueeOverlay, type DagPreselectSnapshot, type DagSelectionBounds, type DagSliderOverlayRow, flowCatalogueItemDescriptor, type FlowCatalogueGroup, type FlowCatalogueItem, type FlowCatalogueSection, FlowGraphCanvasHost, flowSurfaceRenderAllowed, flowRankCatalogueSuggestions, flowSpotlightSuggestionListScrollClass, GraphSliderOverlays, NodeGraphHost, nodeGraphHoverActionArgs, nodeGraphSelectionActionArgs, nodeGraphViewportActionArgs, parseNodeGraphSessionViewport, nodeGraphPickChannel, nodeGraphConnectionIsValid, portValueTypes, portValueTypesCompatible, wireRefusalLabelOptions, paintDagLabelOverlays, resizeCanvasBackingStore, parseCatalogueAppDragPayload, parseDagCameraState, parseDagLabelRows, parseDagMinimapWidgetCursor, parseDagMinimapWidgetOccluder, parseDagNodeIdArray, parseDagOverlayCamera, parseDagPreselectJson, parseDagSelectionUnionBoundsScreen, parseDagSliderOverlays, dagSliderValueText, resolveHostSnapshotWidgetInstanceId, sceneToSyncJson, sceneToSyncPack, SelectionAlignChrome, useClient } from "@semio-tech/framework-os-node-graph-rs/index";
import { TextEditorHost } from "@semio-tech/framework-os/modules/renderer/engine/elements/TextEditor";
import { hostLabel, lineRangeAt, multiSpanReplace } from "@semio-tech/framework-os/modules/renderer/engine/elements/TextEditor/buffer-and-edit-helpers";
import { TableHost, tableStepperClampedDelta, tableStepperKeyDelta } from "@semio-tech/framework-os/modules/renderer/engine/elements/Table";
import { base64ToBytes, Paint2dHost } from "@semio-tech/framework-os/modules/renderer/engine/elements/Paint2dHost";
import { TiledMapHost } from "@semio-tech/framework-os/modules/renderer/engine/elements/TiledMapHost";
import { resolveMapInteractionSync } from "@semio-tech/framework-os/modules/renderer/engine/elements/TiledMapHost/map-parsing";

//#region 🔖️WorldTerrainLayerHost
export { WorldTerrainLayer, parseWorldTerrainStyle, type WorldTerrainStyle };
//#endregion 🔖️WorldTerrainLayerHost

//#region 🔖️Canvas2dHost
export { CANVAS_CAMERA_ZOOM_MIN, CANVAS_CAMERA_ZOOM_MAX, type CanvasCamera, clampCanvasZoom, worldToScreenLogical, screenToWorldLogical, wheelCameraAtScreen, readCanvas2dSurfaceColors, CAMERA_SYNC_DEBOUNCE_MS, Canvas2dHost, canvasLayerDisplayLabel, type CanvasLayerLabelInput };
//#endregion 🔖️Canvas2dHost

//#region 🔖️World3dHost
export { world3dHoverActionArgs, world3dSelectionActionArgs, interactionTargetsForInstances, WORLD3D_DEFAULT_INTERACTION_GRANULARITY, WORLD3D_DEFAULT_MARKER_GRANULARITY, world3dMarkerInteractionTarget, world3dInstanceInteractionTarget, world3dInstanceInteractionTargets, world3dSelectionTargetsActionArgs, type World3dMarkerLayer, type World3dMarkerInteractionFields, type WorldInstanceRecord, worldMeshMaterialRevision, resolveMeshStyle, resolveMeshSelectionPreviewStyle, type SemanticColors, semanticColorsFromPalette, celebrateWorldInstances, isWorldInstanceCelebrating, mergeWorldViewportCamera, world3dViewportCameraSeedKey, world3dFitProjectionContent, world3dFramingInstances, world3dProjectionContentFrameMounted, world3dFrameVisibleOverlayOffered, buildWorldCameraDispatchArgs, worldCameraSetCameraDispatchArgs, worldCameraPoseApproxEqual, shouldReattachWorldViewportCamera, world3dCameraDomJson, parseJsonArray, parseSelectionDomainsFromSession, selectionGroupsFromDomains, mapContextMenuSpecs, resolveWorldContextMenuTarget, worldInstancePickBlocked, resolveVortexPointerDownIntent, isCurveOnlyWorldMesh, isWorldTransformGumballMode, gumballTransformDeltaBetweenPoses, gumballIdentityDelta, worldPaintStep, WORLD_PAINT_IDLE, type WorldPaintGesture, type WorldPaintEvent, type WorldPaintOrigin, world3dGumballSelectionArgsV1, world3dRelocateDragTargetV1, world3dRelocateDispatchArgsV1, world3dVolumeBrushOriginV1, world3dVolumeBrushCommits, type WorldGumballLivePreviewDelta, gumballLivePreviewDeltaBetweenPoses, applyGumballLivePreviewDeltaToPose, applyGumballLivePreviewPoseToObject3D, gumballConfigForTransformMode, worldGumballConfigForProjection, worldVortexMaterialRevision, worldGhostMeshUrl, suggestionMenuItems, resolveWorldMergeMode, world3dContextMenuSurfaceV1, meshBoundsCorners, raycastGroundPoint, parsePuzzle3dCatalogueDragPayload, snapWorldPointToGrid, subscribeWorldCatalogueDropPreview, getWorldCatalogueDropPreview, getWorldCatalogueDropPreviewServerSnapshot, setWorldCatalogueDropPreview, clearWorldCatalogueDropPreview, registerWorldCatalogueDropHost, worldCatalogueDropHostContainsPoint, type WorldSelectionPreview, subscribeWorldSelectionPreview, getWorldSelectionPreview, getWorldSelectionPreviewServerSnapshot, setWorldSelectionPreview, clearWorldSelectionPreview, type WorldGumballTransformPreview, subscribeWorldGumballTransformPreview, getWorldGumballTransformPreview, getWorldGumballTransformPreviewServerSnapshot, setWorldGumballTransformPreview, clearWorldGumballTransformPreview, worldVorticesWithGumballPreview, worldAttractionsWithGumballPreview, WindowInstanceIdContext, registerPendingWorldProjection, clearPendingWorldProjection, world3dProjectionPaneElementId, WorldOrbitProjectionSwitchPane, World3dHost };
//#endregion 🔖️World3dHost

//#region 🔖️NodeGraphHost
export {
  alignModeToDag,
  catalogueGhostDescriptorJson,
  type CatalogueAppDragPayload,
  computeDagMarqueeOverlay,
  dagElementInteractionChrome,
  dagOverlayLabelFill,
  dagOverlayLabelFillHex,
  dagContentBounds,
  dagContentCoverage,
  dagEllipsizeByMeasure,
  dagFitCamera,
  dagScreenToWorld,
  dagStartupCamera,
  dagWorldToScreen,
  nodeGraphContentSignature,
  DAG_CONTENT_FIT_PADDING_PX,
  DAG_CONTENT_FRAMED_MIN_COVERAGE,
  DAG_CONTENT_REFIT_MAX_COVERAGE,
  DAG_LABEL_ELLIPSIS,
  type DagCameraState,
  type DagContentBounds,
  type DagLabelOverlayInteraction,
  type DagLabelOverlayRow,
  type DagMarqueeOverlay,
  type DagPreselectSnapshot,
  type DagSelectionBounds,
  type DagSliderOverlayRow,
  flowCatalogueItemDescriptor,
  type FlowCatalogueGroup,
  type FlowCatalogueItem,
  type FlowCatalogueSection,
  FlowGraphCanvasHost,
  flowSurfaceRenderAllowed,
  flowRankCatalogueSuggestions,
  flowSpotlightSuggestionListScrollClass,
  GraphSliderOverlays,
  NodeGraphHost,
  nodeGraphHoverActionArgs,
  nodeGraphSelectionActionArgs,
  nodeGraphViewportActionArgs,
  parseNodeGraphSessionViewport,
  nodeGraphPickChannel,
  nodeGraphConnectionIsValid,
  portValueTypes,
  portValueTypesCompatible,
  wireRefusalLabelOptions,
  paintDagLabelOverlays,
  resizeCanvasBackingStore,
  parseCatalogueAppDragPayload,
  parseDagCameraState,
  parseDagLabelRows,
  parseDagMinimapWidgetCursor,
  parseDagMinimapWidgetOccluder,
  parseDagNodeIdArray,
  parseDagOverlayCamera,
  parseDagPreselectJson,
  parseDagSelectionUnionBoundsScreen,
  parseDagSliderOverlays,
  dagSliderValueText,
  resolveHostSnapshotWidgetInstanceId,
  sceneToSyncJson,
  sceneToSyncPack,
  SelectionAlignChrome,
  useClient,
};
//#endregion 🔖️NodeGraphHost

//#region 🔖️TextEditorHost
export { hostLabel, lineRangeAt, multiSpanReplace, TextEditorHost };
//#endregion 🔖️TextEditorHost

//#region 🔖️TableHost
export { TableHost, tableStepperClampedDelta, tableStepperKeyDelta };
//#endregion 🔖️TableHost

//#region 🔖️Paint2dHost
export { base64ToBytes, Paint2dHost };
//#endregion 🔖️Paint2dHost

//#region 🔖️TiledMapHost
export { TiledMapHost, resolveMapInteractionSync };
//#endregion 🔖️TiledMapHost
