import { Window, type WindowConfig } from "../../../🧱️elements/🪟️Window/🟦️.tsx";
import { applyNodeChanges, Background, BackgroundVariant, BaseEdge, getBezierPath, Handle, Position, ReactFlow, ReactFlowProvider, SelectionMode, useInternalNode, useReactFlow, useStoreApi, ViewportPortal } from "@xyflow/react";
import type { Connection, ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, Node as FlowNode, NodeProps, NodeTypes, OnSelectionChangeParams, ReactFlowInstance } from "@xyflow/react";
import { DIAGRAM_UNIT, type DiagramLayoutDirection, type DiagramLayoutOptions, Diagram, DiagramSkeleton } from "../../../🧱️elements/🕸️Diagram/🟦️.tsx";
import { type DiagramForceConfig, defaultDiagramForceConfig } from "../../../🧱️elements/🕸️Diagram/📋️force-model/🟦️.ts";
import type { DiagramProps } from "../../../🧱️elements/🕸️Diagram/🧩️inner/🟦️.tsx";
import { useDiagramLayout } from "../../../🧱️elements/🕸️Diagram/🪢️layout-hook/🟦️.ts";
import { DIAGRAM_KEYBOARD_FOCUS_CLASS, DiagramLiveRegion, diagramKeyboardAnnouncement, diagramKeyboardStep, diagramReadingOrderPosition, useDiagramTranslate, type DiagramKeyboardStep, type DiagramNavigableNode, type DiagramTranslate } from "../../../🧱️elements/🕸️Diagram/⌨️keyboard/🟦️.tsx";
import { sceneFrameControlRef, planeFromPointAndDirection, getPlanePosition, hasValidPlane, isGeometryFocusable, Geometry, type SceneGeometry, type TransformableGeometry, type PlaneTransformDelta, type OnPlaneUpdate, type OnMultiPlaneUpdate } from "../../../🧱️elements/🎬️Scene/🗿️objects/🟦️.tsx";
import { SCENE_GIZMO_LABELS, resolveSceneGizmoSnapTarget, resolveSceneGizmoViewportPlacement } from "../../../🧱️elements/🎬️Scene/🧭️gizmo-snap/🟦️.ts";
import { GUMBALL_PLANE_OFFSET, GUMBALL_PLANE_SIZE, GUMBALL_RING_RADIUS, GUMBALL_PREVIEW_MIN_EXTENT, gumballScalePlaneAxisIndices, gumballPlaneScaleCorner, gumballPlaneScaleFactors, gumballScaleAxisOffset } from "../../../🧱️elements/🎬️Scene/🔩️gumball-handles/🟦️.tsx";
import { GUMBALL_PREVIEW_RING_RADIUS, GUMBALL_PREVIEW_DISK_RADIUS, gumballPreviewWorldExtent, resolveGumballVisualPalette, gumballHandleVisualState, gumballResolveHandleVisual, type GumballVisualPalette, type GumballHandleVisualState } from "../../../🧱️elements/🎬️Scene/🎨️gumball-visual/🟦️.tsx";
import { gumballHandleRaycast, gumballRaycastOwnedAtClientPoint, gumballRayFromNdc, gumballPointerConsumesCanvasEventRef, gumballKindFromRaycastObject, gumballRaycastAtClientPoint, gumballRaycastKindAtClientPoint } from "../../../🧱️elements/🎬️Scene/🎯️gumball-pick/🟦️.ts";
import { gumballApplyHandleVisualMaterial, UnifiedGumball, type UnifiedGumballProps } from "../../../🧱️elements/🎬️Scene/🕹️gumball/🟦️.tsx";
import { type GumballVec3, GUMBALL_PLANE_HANDLES, DEFAULT_GUMBALL_CONFIG, resolveGumballConfig, gumballHandleAllowedByPlane, gumballHandleGroupEnabled, gumballHandleEnabled, gumballConfigVisible, gumballHandleKindToTransformMode, gumballPoseFromObject3D, applyGumballPose, gumballRayAxisParameter, gumballEyeFromPivot, gumballAxisDragPlaneNormal, gumballProjectRayOntoAxis, gumballRayPlanePoint, gumballAxisRotateAngle, gumballAxisScaleFactor, gumballSnapScalar, GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP, GUMBALL_DEFAULT_SHIFT_SCALE_SNAP, gumballEffectiveSnapValue, gumballResolveDragSnaps, type GumballPose, type GumballHandleKind, type GumballPlaneId, type GumballConfig } from "../../../🧱️elements/🎬️Scene/🧿️gumball-model/🟦️.ts";
import { Scene, SceneSkeleton } from "../../../🧱️elements/🎬️Scene/🟦️.tsx";
import { type SortDirection, type TableColumn, type HierarchicalRowData, type DragDropConfig, type TableProps, Table, type TableSkeletonProps, TableSkeleton } from "../../../🧱️elements/📊️Table/🟦️.tsx";
import { type HistoryColumnAuthor, type HistoryColumn, type HistoryTableProps, HistoryTable } from "../../../🧱️elements/🕰️HistoryTable/🟦️.tsx";
import { VIRTUAL_FILE_SYSTEM_DEMO_DESCRIPTOR_KINDS, VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS, VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, getVirtualFileSystemOrderedRowIds, normalizeVirtualFileSystemSelectedRowIds, getVirtualFileSystemNextSelectionState, resolveVirtualFileSystemFileNodeKind, resolveVirtualFileSystemDescriptorKind, resolveVirtualFileSystemDescriptorBinding, buildVirtualFileSystemDescriptorValues, formatVirtualFileSystemTime, renderVirtualFileSystemDescriptorCell, buildVirtualFileSystemDescriptorColumns, resolveVirtualFileSystemSchemaIcon, virtualFileSystemKindIcon, isVirtualFileSystemRemoteIcon, buildVirtualFileSystemSceneRows, buildVirtualFileSystemVisibleRows, VirtualFileSystem, type DescriptorKind, type FileNodeDescriptor, type FileNodeKind, type FileNodeDescriptorValue, type VirtualFileSystemSchema, type FileNode, type VirtualFileSystemNode, type VirtualFileSystemRow, type VirtualFileSystemProps } from "../../../🧱️elements/⚙️VirtualFileSystem/🟦️.tsx";
import { Canvas, HorizontalWindows, VerticalWindows, App, Ui, type AppModeDescriptor, type AppProps, type UiAppDescriptor, type UiProps } from "../../../🧱️elements/🎨️Canvas/🟦️.tsx";
import { panelGhostSessionBridge, setPanelGhostSessionBridge } from "../../../🧱️elements/👻️PanelGhost/🟦️.tsx";
import { MODE_CANVAS_INSET_CLASS, Mode, type ModeProps } from "../../../🧱️elements/🎨️Canvas/🟦️.tsx";
import { modeCollectWindowIds, removeWindowFromLayout, resolveStackPathForWindowId, reconcileWindows, normalizeLayoutToStacks, collapseLayout } from "../../../🧱️elements/🎨️Canvas/🗺️layout/🟦️.ts";
import { COMPOSE_WINDOW_TEMPLATE_MIME, MODE_TEMPLATE_PREVIEW_WINDOW_ID, beginWindowTemplateDrag, endWindowTemplateDrag, readActiveWindowTemplateDragSession, windowTemplatePointerDragRef, beginWindowTemplatePointerDrag, cancelWindowTemplatePointerDrag, windowTemplatePaletteTreeDragController, type ModeWindowDescriptor, type WindowTemplateDragSession, type ModeCanvasDropTarget, type WindowTemplateDropPayload } from "../../../🧱️elements/🎨️Canvas/🪟️template-drag/🟦️.ts";
import { modeAxisIsPerpendicularChild, modePerpendicularJoinSeparators, modeJoinCornerSpecsForSeparator, modeJoinCornerSpecsForCrossSeparator, MODE_JOIN_CORNER_TOUCH_EPS, resolveJoinCornerPeerCrossAxes, applyAxisResizeDelta, resolveJoinCornerResizeDeltas, applyAxisGroupLayoutDelta, modeAxisGroupLayout, applyModeJoinCornerResize, type ModeJoinCornerCrossAxis } from "../../../🧱️elements/🎨️Canvas/🔗️join-corners/🟦️.ts";
import { insertWindowAtDropZone, applyModeDrop, computeModeDropZone, computeModeSplitPreviewInBody, resolveModeSplitSideInBody, computeTabInsertPreview, modeDockOutLayout, modeDockTabsWithInsertPreview, modeDockDragInsertTabs, resolveModeTabInsertPreview } from "../../../🧱️elements/🎨️Canvas/🎯️drop/🟦️.ts";
import { splitWithWindow, splitWithStack, extractStackFromLayout, mergeStackTabsIntoStack, WINDOW_STACK_CORNERS, resolveWindowCorner, modeStackTabsByCorner, insertWindowAsTabAtCorner, setWindowCornerInLayout } from "../../../🧱️elements/🎨️Canvas/📐️corners/🟦️.ts";
import { Edges } from "@react-three/drei/core/Edges.js";
import { GizmoHelper } from "@react-three/drei/core/GizmoHelper.js";
import { GizmoViewport } from "@react-three/drei/core/GizmoViewport.js";
import { Grid } from "@react-three/drei/core/Grid.js";
import { OrbitControls } from "@react-three/drei/core/OrbitControls.js";
import { useGLTF } from "@react-three/drei/core/Gltf.js";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { useTranslation } from "react-i18next";
import { useHotkeys } from "../../../🔨️modules/🕹️control-keybinding-context/🟦️.tsx";
import { rankFuzzyItems, type FuzzySearchField, type FuzzySearchOptions, type FuzzySearchResult } from "../../../🔨️modules/🔎️fuzzy-ranking/🟦️.ts";

export { Window, type WindowConfig };

// #region 🧫️Diagram
export { applyNodeChanges, Background, BackgroundVariant, BaseEdge, getBezierPath, Handle, Position, ReactFlow, ReactFlowProvider, SelectionMode, useInternalNode, useReactFlow, useStoreApi, ViewportPortal };
export type { Connection, ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, FlowNode as Node, NodeProps, NodeTypes, OnSelectionChangeParams, ReactFlowInstance, Connection as RFConnection };
export { DIAGRAM_UNIT, type DiagramLayoutDirection, type DiagramLayoutOptions, type DiagramForceConfig, defaultDiagramForceConfig, type DiagramProps, Diagram, useDiagramLayout, DiagramSkeleton };
export { DIAGRAM_KEYBOARD_FOCUS_CLASS, DiagramLiveRegion, diagramKeyboardAnnouncement, diagramKeyboardStep, diagramReadingOrderPosition, useDiagramTranslate, type DiagramKeyboardStep, type DiagramNavigableNode, type DiagramTranslate };
export {
  sceneFrameControlRef,
  SCENE_GIZMO_LABELS,
  GUMBALL_PLANE_OFFSET,
  GUMBALL_PLANE_SIZE,
  GUMBALL_RING_RADIUS,
  GUMBALL_PREVIEW_MIN_EXTENT,
  GUMBALL_PREVIEW_RING_RADIUS,
  GUMBALL_PREVIEW_DISK_RADIUS,
  gumballHandleRaycast,
  gumballRaycastOwnedAtClientPoint,
  gumballApplyHandleVisualMaterial,
  type GumballVec3,
  planeFromPointAndDirection,
  getPlanePosition,
  hasValidPlane,
  isGeometryFocusable,
  Geometry,
  resolveSceneGizmoSnapTarget,
  resolveSceneGizmoViewportPlacement,
  GUMBALL_PLANE_HANDLES,
  DEFAULT_GUMBALL_CONFIG,
  resolveGumballConfig,
  gumballHandleAllowedByPlane,
  gumballHandleGroupEnabled,
  gumballHandleEnabled,
  gumballConfigVisible,
  gumballHandleKindToTransformMode,
  gumballPoseFromObject3D,
  applyGumballPose,
  gumballRayAxisParameter,
  gumballEyeFromPivot,
  gumballAxisDragPlaneNormal,
  gumballProjectRayOntoAxis,
  gumballRayPlanePoint,
  gumballAxisRotateAngle,
  gumballAxisScaleFactor,
  gumballSnapScalar,
  GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP,
  GUMBALL_DEFAULT_SHIFT_SCALE_SNAP,
  gumballEffectiveSnapValue,
  gumballResolveDragSnaps,
  gumballScalePlaneAxisIndices,
  gumballPlaneScaleCorner,
  gumballPlaneScaleFactors,
  gumballScaleAxisOffset,
  gumballPreviewWorldExtent,
  resolveGumballVisualPalette,
  gumballHandleVisualState,
  gumballResolveHandleVisual,
  gumballRayFromNdc,
  gumballPointerConsumesCanvasEventRef,
  gumballKindFromRaycastObject,
  gumballRaycastAtClientPoint,
  gumballRaycastKindAtClientPoint,
  UnifiedGumball,
  Scene,
  SceneSkeleton,
  type SceneGeometry,
  type TransformableGeometry,
  type PlaneTransformDelta,
  type OnPlaneUpdate,
  type OnMultiPlaneUpdate,
  type GumballPose,
  type GumballHandleKind,
  type GumballPlaneId,
  type GumballConfig,
  type GumballVisualPalette,
  type GumballHandleVisualState,
  type UnifiedGumballProps,
};
export { type SortDirection, type TableColumn, type HierarchicalRowData, type DragDropConfig, type TableProps, Table, type TableSkeletonProps, TableSkeleton };
export { type HistoryColumnAuthor, type HistoryColumn, type HistoryTableProps, HistoryTable };
export {
  VIRTUAL_FILE_SYSTEM_DEMO_DESCRIPTOR_KINDS,
  VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS,
  VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA,
  getVirtualFileSystemOrderedRowIds,
  normalizeVirtualFileSystemSelectedRowIds,
  getVirtualFileSystemNextSelectionState,
  resolveVirtualFileSystemFileNodeKind,
  resolveVirtualFileSystemDescriptorKind,
  resolveVirtualFileSystemDescriptorBinding,
  buildVirtualFileSystemDescriptorValues,
  formatVirtualFileSystemTime,
  renderVirtualFileSystemDescriptorCell,
  buildVirtualFileSystemDescriptorColumns,
  resolveVirtualFileSystemSchemaIcon,
  virtualFileSystemKindIcon,
  isVirtualFileSystemRemoteIcon,
  buildVirtualFileSystemSceneRows,
  buildVirtualFileSystemVisibleRows,
  VirtualFileSystem,
  type DescriptorKind,
  type FileNodeDescriptor,
  type FileNodeKind,
  type FileNodeDescriptorValue,
  type VirtualFileSystemSchema,
  type FileNode,
  type VirtualFileSystemNode,
  type VirtualFileSystemRow,
  type VirtualFileSystemProps,
};
export { type PaneHostProps, PaneHost, type PaneProps, Pane } from "../../../🧱️elements/📌️WindowPane/🟦️.tsx";
export { type EngagementOption, type EngagementStatus, type EngagementRingOption, type EngagementSliderControl, type EngagementStepperControl, type EngagementRingControl, type EngagementToggleGroupOption, type EngagementToggleGroupControl, type EngagementSelectItem, type EngagementSelectControl, type EngagementControl, UI_ENGAGEMENT, ENGAGEMENT_USER, type EngagementSpec, type EngagementProps, Engagement } from "../../../🧱️elements/🎮️Engagement/🟦️.tsx";
export { type WindowStackCorner, type WindowLayoutWindowNode, type WindowLayoutStackNode, type WindowLayoutAxisNode, type WindowLayoutNode, createEvenWindowLayout } from "../../../🔨️modules/🗺️window-layout/🟦️.ts";
export { UI_WINDOW_SEARCH, type SearchInput, type SearchPossible, filterSearchPossibles, type SearchInlineCompletion, searchInlineCompletion, searchActiveInlineCompletion, searchHighlightedLabel, isUiTypingTarget, isWindowSearchTypingTarget, shouldRouteKeysToWindowSearch, queryWindowSearchInput, focusActiveSearchInput, type SearchLineEdit, SEARCH_LINE_EDIT_SENT_LIMIT, searchControlledLineV1, shouldActivateSearchPossibleOnConfirm, searchSpaceConfirmsLine, applySearchSpaceAction, routeWindowSearchSpace, routeWindowSearchKeydown, routeWindowSearchEscape, type SearchSpec, type SearchProps, normalizeEngagementActionText, Search } from "../../../🧱️elements/🔍️WindowSearch/🟦️.tsx";
export { ContextMenuChrome } from "../../../🧱️elements/🖱️ContextMenu/🥣️chrome/🟦️.tsx";
export { windowMeasuresDefaultWidthPx, windowMeasuresMinWidthPx, windowMeasuresMaxWidthPx, windowChromeClearedTopOffset, WINDOW_PANE_MEASURES_ICON, WINDOW_PANE_ACTIONS_ICON, WINDOW_PANE_SEARCH_ICON, WINDOW_PANE_UTILITIES_ICON } from "../../../🔨️modules/📐️window-measures/🟦️.ts";
export { shellNavbarTrailingEndWidthByRoot, useShellNavbarTrailingEndWidthPx, shellNavbarTrailingEndReserveStyle, chromeHostedPanelCapRowStyle, type SafeAreaBox, type ChromePanelOccupancy, type SafeAreaYield, type ChromePanelSafeArea, safeAreaBoxFromRect, publishShellChromePanelBox, useShellChromePanelBoxes, chromePanelSafeArea, chromePanelSafeAreaStyle, useChromePanelSafeArea } from "../../../🔨️modules/🛟️chrome-safe-area/🟦️.tsx";
export { type PanelTabDockTarget, type PanelTreeUnitDockTarget, type PanelTreeUnitDockMove, PANEL_TREE_UNIT_MIME, TREE_SECTION_REORDER_MIME, type PanelTreeUnitDragSession, beginPanelTreeUnitDrag, endPanelTreeUnitDrag, readActivePanelTreeUnitDrag, usePanelTreeUnitDragActive, type PanelDockContextValue, PanelDockContext, usePanelDockContext } from "../../../🔨️modules/🧲️panel-dock/🟦️.tsx";
export { ANCHORS, type Anchor, anchorVertical, anchorHorizontal, flowFromAnchor, anchorPositionStyle, chromeHostedOpenPanelPositionStyle, nearestAnchor } from "../../../🔨️modules/⚓️panel-anchor/🟦️.ts";
export { windowMeasureTreeLeafLabelClass, windowMeasureTreeGroupLabelClass, panelTabLabelClass, panelTabIconSlotClass, dropZoneReadyTextClass, dropZoneReadyFillClass, dropZoneReadyClass, GLASS_OVERLAY_BOX_CLASS, borderNormalTopClass, shellChromeFrameLayerClass, floatingMenuItemClass, panelTabBarClass, panelTabButtonDividerClass, panelTabButtonClass, panelAnchorTabBarClass, panelAnchorTabButtonClass, mobilePanelTabBarClass, mobilePanelTabButtonClass, panelResizeEdgeAccentClass, windowBodyFrameClass, modeDockInactiveTabClass, modeDockInactiveTabBeforeGapClass, panelWindowInactiveTabClass, modeDockTabLabelClassName, modeDockActiveTabFillClass, modeDockActiveTabClass, windowEngagementBodyClass, utilityBarBodyClass, windowMeasuresBodyClass } from "../../../🔨️modules/🎨️chrome-class-names/🟦️.ts";
export { type Point, type Vector, type Plane, type Camera } from "../../../🧱️elements/🎬️Scene/📐️geometry/🟦️.ts";
export { UiMobileProvider, useUiMobile } from "../../../🔨️modules/📱️ui-mobile-context/🟦️.tsx";
export { UI_TERMINOLOGY_NATIVE, UI_CHROME_TERMINOLOGY_STORAGE_KEY, readStoredUiChromeTerminology, writeStoredUiChromeTerminology, resolveUiLocalizedText, useUiTerminology } from "../../../🔨️modules/🗣️ui-terminology/🟦️.tsx";
export { interactionMergeFromModifiers, type PanelGhostValue, usePanelGhost, useInteractionCommands, shouldBeginAutomaticGhostInteraction, GhostProvider, GhostRegionShell, PanelGhostRoot } from "../../../🧱️elements/👻️PanelGhost/🟦️.tsx";
export {
  Canvas,
  panelGhostSessionBridge,
  setPanelGhostSessionBridge,
  MODE_CANVAS_INSET_CLASS,
  modeCollectWindowIds,
  HorizontalWindows,
  VerticalWindows,
  COMPOSE_WINDOW_TEMPLATE_MIME,
  MODE_TEMPLATE_PREVIEW_WINDOW_ID,
  beginWindowTemplateDrag,
  endWindowTemplateDrag,
  readActiveWindowTemplateDragSession,
  windowTemplatePointerDragRef,
  beginWindowTemplatePointerDrag,
  cancelWindowTemplatePointerDrag,
  windowTemplatePaletteTreeDragController,
  modeAxisIsPerpendicularChild,
  modePerpendicularJoinSeparators,
  modeJoinCornerSpecsForSeparator,
  modeJoinCornerSpecsForCrossSeparator,
  MODE_JOIN_CORNER_TOUCH_EPS,
  resolveJoinCornerPeerCrossAxes,
  applyAxisResizeDelta,
  resolveJoinCornerResizeDeltas,
  applyAxisGroupLayoutDelta,
  modeAxisGroupLayout,
  applyModeJoinCornerResize,
  insertWindowAtDropZone,
  Mode,
  removeWindowFromLayout,
  resolveStackPathForWindowId,
  splitWithWindow,
  splitWithStack,
  extractStackFromLayout,
  applyModeDrop,
  reconcileWindows,
  normalizeLayoutToStacks,
  collapseLayout,
  computeModeDropZone,
  computeModeSplitPreviewInBody,
  resolveModeSplitSideInBody,
  computeTabInsertPreview,
  modeDockOutLayout,
  modeDockTabsWithInsertPreview,
  modeDockDragInsertTabs,
  mergeStackTabsIntoStack,
  resolveModeTabInsertPreview,
  WINDOW_STACK_CORNERS,
  resolveWindowCorner,
  modeStackTabsByCorner,
  insertWindowAsTabAtCorner,
  setWindowCornerInLayout,
  App,
  Ui,
  type ModeWindowDescriptor,
  type WindowTemplateDragSession,
  type ModeCanvasDropTarget,
  type WindowTemplateDropPayload,
  type ModeProps,
  type ModeJoinCornerCrossAxis,
  type AppModeDescriptor,
  type AppProps,
  type UiAppDescriptor,
  type UiProps,
};
// #endregion ⚙️Canvas

// Re-exports of common libraries used alongside UI primitives.
// Workbench shell types and chrome live in `@semio-tech/framework-platform-core` / `@semio-tech/framework-platform-renderer-react`.

// #region 🌩️DnD Kit
export { closestCenter, DndContext, DragOverlay, PointerSensor, pointerWithin, rectIntersection, useDraggable, useDroppable, useSensor, useSensors } from "@dnd-kit/core";
export type { DragEndEvent, DragOverEvent, DragStartEvent } from "@dnd-kit/core";
export { arrayMove, SortableContext, useSortable, verticalListSortingStrategy } from "@dnd-kit/sortable";
export { CSS as DndCSS } from "@dnd-kit/utilities";
// #endregion 🌩️DnD Kit

// #region 📰️Three.js
export { Select as DreiSelect } from "@react-three/drei/web/Select.js";
export { Edges, GizmoHelper, GizmoViewport, Grid, OrbitControls, useGLTF };
export { Line } from "@react-three/drei/core/Line.js";
export { Sphere } from "@react-three/drei/core/shapes.js";
export { useFBX } from "@react-three/drei/core/Fbx.js";
export { Canvas as ThreeCanvas, useFrame, useLoader, useThree } from "@react-three/fiber";
export {
  Box3,
  BoxGeometry,
  BufferAttribute,
  BufferGeometry,
  CanvasTexture,
  ClampToEdgeWrapping,
  RepeatWrapping,
  MirroredRepeatWrapping,
  NearestFilter,
  LinearFilter,
  NearestMipmapNearestFilter,
  LinearMipmapNearestFilter,
  NearestMipmapLinearFilter,
  LinearMipmapLinearFilter,
  Color,
  ConeGeometry,
  CylinderGeometry,
  DoubleSide,
  DynamicDrawUsage,
  EdgesGeometry,
  Group,
  IcosahedronGeometry,
  InstancedBufferAttribute,
  InstancedMesh,
  LineBasicMaterial,
  LineDashedMaterial,
  LineSegments,
  Mesh,
  MeshStandardMaterial,
  Object3D,
  OrthographicCamera,
  PlaneGeometry,
  PointsMaterial,
  Quaternion,
  ShaderMaterial,
  SphereGeometry,
  Texture,
  TextureLoader,
  FrontSide,
  SRGBColorSpace,
  NoColorSpace,
  TorusGeometry,
  Vector3,
  Vector2,
  Matrix4,
  Plane as ThreePlane,
  BackSide,
  MeshBasicMaterial,
  AlwaysStencilFunc,
  NotEqualStencilFunc,
  IncrementWrapStencilOp,
  DecrementWrapStencilOp,
  ReplaceStencilOp,
} from "three";
export { GLTFLoader };
export { OBJLoader } from "three/addons/loaders/OBJLoader.js";
export { OrbitControls as ThreeOrbitControls } from "three/addons/controls/OrbitControls.js";
// #endregion 📰️Three.js

// #region 🎽️XY Flow (additions not already exported inline)
export { ConnectionMode, MiniMap } from "@xyflow/react";
// #endregion 🎽️XY Flow

// #region 🖋️State Management
export { assign, createActor, fromCallback, setup, type ActorRefFrom, type AnyActorRef, type SnapshotFrom } from "xstate";
// #endregion 🖋️State Management

// #region 🗿️I18n
export { useTranslation };
// #endregion 🗿️I18n

// #region 🌙️Hotkeys
export { useHotkeys };
// #endregion 🌙️Hotkeys

// #region 🔔️Search
export { rankFuzzyItems };
export type { FuzzySearchField, FuzzySearchOptions, FuzzySearchResult };
// #endregion 🔔️Search

// #region 🌨️Styling
export { styleVariants } from "../../../🔨️modules/🧬️style-variants/🟦️.ts";
export type { StyleCompoundVariant, StyleVariantCompiler, StyleVariantConfiguration, StyleVariantProps, StyleVariantSchema, StyleVariantSelection } from "../../../🔨️modules/🧬️style-variants/🟦️.ts";
// #endregion 🌨️Styling

export * as ResizablePrimitive from "react-resizable-panels";
