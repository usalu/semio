export * from "@semio-tech/schema-ts/artifact-reference";
export * from "@semio-tech/framework-io-schema-ts";
export{parseIntrinsicValue,parseIntrinsicValueControlled}from"@semio-tech/value-ts/schema/intrinsic";
export type{IntrinsicValue,IntrinsicMember}from"@semio-tech/value-ts/schema/intrinsic";
export { SqliteOperation, sqliteOperation, SqliteAllocationControl, exportSqliteDatabase, importSqliteDatabase, parseSqliteDatabaseSchema, validateSqliteDatabaseSchema, parseSqliteDatabaseSchemaControlled, validateSqliteDatabaseSchemaControlled } from "@semio-tech/framework-sqlite-snapshot-ts";
export type { SqliteValue, SqliteRow, SqliteTable, SqliteDatabase, SqliteDatabaseOptions, SqliteDatabaseProgress } from "@semio-tech/framework-sqlite-snapshot-ts";
export type { ArtifactSqliteOptions, ArtifactSqliteProgress } from "@semio-tech/framework-sqlite-snapshot-ts/artifact";
export {binary64,binary32,binary64Value,binary32Value,parseBinary64,parseBinary32} from "../../🔨️modules/🌱️value/📦️packages/🟦️typescript/🔢️ieee754/🟦️.ts";
export type {Binary64,Binary32} from "../../🔨️modules/🌱️value/📦️packages/🟦️typescript/🔢️ieee754/🟦️.ts";
export { createLeasePool } from "@semio-tech/framework-async/lease-pool";
export type { Lease, LeasePool, LeasePoolStats } from "@semio-tech/framework-async/lease-pool";
export { retryWithJitteredBackoff } from "@semio-tech/framework-async/jittered-backoff";
export type { JitteredBackoffOptions } from "@semio-tech/framework-async/jittered-backoff";
export { latestWins } from "@semio-tech/framework-async/latest-wins";
export { waitForEvent } from "@semio-tech/framework-async/event-wait";
export type { EventSubscribe, WaitForEventOptions } from "@semio-tech/framework-async/event-wait";
export { fetchWithTimeout } from "../../🔨️modules/🚪️io/🌐️fetch-timeout/🟦️.ts";
export type { FetchTimeoutOptions, FetchTimeoutResponse } from "../../🔨️modules/🚪️io/🌐️fetch-timeout/🟦️.ts";
/** 📦️ `@semio-tech/framework` — package glue (reexports + inline vitest). */
export * from "../../🔨️modules/🎯️action-bus/🟦️.ts";
export { blake3Hex, Blake3Hasher } from "@semio-tech/framework-hash";
export { TransientStore, defaultTransientStore, ephemeralBox, ephemeralMap, ephemeralSet, ephemeralWeakMap, type EphemeralBox } from "@semio-tech/framework-transient";
export * from "@semio-tech/framework-action-argument-resolution";
export * from "@semio-tech/schema-ts";
export * from "../../🔨️modules/🖥️platform/🟦️.ts";
export { createBrowserStoragePort, createMemoryStoragePort, createScopedStoragePort } from "@semio-tech/ui/modules/storage-port";
export type { StoragePort } from "@semio-tech/ui/modules/storage-port";
export { windowElementId, panelTabElementId, panelTabFirstDraggableElementId } from "@semio-tech/ui/modules/element-identity";
export type { DockSkeleton, DockTabSkeleton } from "@semio-tech/ui/modules/dock-skeleton";
export * from "@semio-tech/ui/scene";
export { CONTEXT_MENU_GROUP_ID_PREFIX, CONTEXT_MENU_OVERFLOW_CATEGORY, type ContextMenuItemSpec, type UiMenuRef, organizeContextMenu } from "@semio-tech/framework-manifest/context-menu";
export { DEFAULT_WORLD3D_PRESENTATION, type SceneViewportMask3d, type World3dComputeLabelV1, type World3dComputeStatusV1, type World3dPresentation, type World3dScene, parseWorld3dPresentation, world3dComputeStatusV1 } from "@semio-tech/ui/scene/world3d";
export { BOARD2D_SCENE_LANES, BOARD2D_SCENE_LANE_KEY_PREFIX, CANVAS2D_SCENE_LANES, CANVAS2D_SCENE_LANE_KEY_PREFIX, PAINT2D_SCENE_LANES, PAINT2D_SCENE_LANE_KEY_PREFIX, type SceneLane, type SceneLaneRef, TILEDMAP_SCENE_LANES, TILEDMAP_SCENE_LANE_KEY_PREFIX, WORLD3D_SCENE_LANES, WORLD3D_SCENE_LANE_KEY_PREFIX, type World3dSceneLane, board2dSceneFromLanes, board2dSceneLaneForBodyKey, canvas2dSceneFromLanes, canvas2dSceneLaneForBodyKey, paint2dSceneFromLanes, paint2dSceneLaneForBodyKey, sceneFromLanes, tiledMapSceneFromLanes, tiledMapSceneLaneForBodyKey, world3dSceneFromLanes, world3dSceneLaneForBodyKey } from "@semio-tech/ui/scene/lanes";
export { type AppCatalogue, type AppCatalogueItem, type AppCatalogueSection, NODE_GRAPH_SCENE_LANES, type NodeGraphEdgeRecord, type NodeGraphFindItem, type NodeGraphHover, type NodeGraphInteractionDomain, type NodeGraphNodeRecord, type NodeGraphOperatorChannelRecord, type NodeGraphOperatorRecord, type NodeGraphOperatorVariadicRecord, type NodeGraphPortRecord, type NodeGraphScene, type PresencePeer, TEXT_EDITOR_SCENE_LANES, type TextEditorScene, nodeGraphActions, nodeGraphSceneFromLanes, textEditorSceneFromLanes } from "@semio-tech/ui/scene/node-graph";
export { type ContinuousGestureAbortReason, type ContinuousGestureLane, type ContinuousGestureLanePorts, type ContinuousGesturePhase, continuousPressIdentity, createContinuousGestureLane, textEditorActions } from "@semio-tech/ui/scene/gestures";
export { type BlockPaletteEntry, type Board2dScene, type Board2dSuggestionCandidate, type Board2dSuggestionMenu, type DiffViewScene, type EventFeedEntry, type EventFeedScene, type GraphTimelineScene, type IconRenderScene, type InkCanvasInteractionDomain, type InkCanvasScene, type Paint2dScene, TABLE_SCENE_LANES, type TableScene, type TiledMapScene, type VirtualFileSystemScene, inkCanvasActions, tableSceneFromLanes } from "@semio-tech/ui/scene/documents";
export type { ActionDispatchLifecycleV1, ActionOperationControlV1, ComponentKind, ComponentSceneHostProps, UiComponentSceneNode, UiExternalSlotNode } from "@semio-tech/ui/scene/components";
export { parseViewport2d } from "@semio-tech/framework-ui-viewport-ts/2d/schema";
export type { Viewport2d } from "@semio-tech/framework-ui-viewport-ts/2d/schema";
export { parseViewport3dProjectionFramePolicy } from "@semio-tech/framework-ui-viewport-ts/3d/schema";
export type { Viewport3dProjectionFramePolicy } from "@semio-tech/framework-ui-viewport-ts/3d/schema";
export * from "@semio-tech/framework-manifest";
export * from "@semio-tech/framework-manifest/ui-contract-and-canvas-types";
export * from "@semio-tech/framework-manifest/action-types-and-semantics";
export * from "@semio-tech/ui-contract/action-arg-facets";
export * from "@semio-tech/framework-manifest/mutation-input-schema";
export * from "@semio-tech/framework-manifest/definitions-action-ids-and-shell-brand";
export * from "@semio-tech/framework-manifest/view-state-and-public-invocation";
export * from "@semio-tech/framework-manifest/contributions-and-examples";
export * from "@semio-tech/framework-manifest/host-effects-choices-and-fault-notices";
export * from "@semio-tech/framework-manifest/app-definition-refresh-and-context-menu";
export * from "@semio-tech/framework-time-travel-ts/actions";
export * from "@semio-tech/framework-tool-run-ts/actions";
export * from "@semio-tech/framework-onboarding";
export * from "@semio-tech/ui/modules/canvas-pick-targets";
export * from "@semio-tech/framework-trace/memory";
// 🕹️wave-2b: named (not `export *`) — the 🕹️interaction module's own `InteractionDefinition`/`MergeMode`/…
// family is already re-exported above via `🛂️manifest` (owned-schema-generated mirror of the same Rust types),
// so a second blanket export of the module root would collide; only its presence-broadcast leaf types,
// which nothing else exports yet, are pulled in here for `@semio-tech/framework` consumers like the OS Shell.
export type { InteractionTarget, PresenceDomain, PresenceInteraction } from "@semio-tech/framework-interaction/schema";
export {
  EMPTY_GESTURE_POINTERS,
  GESTURE_MULTI_TOUCH_POINTERS,
  IDENTITY_PINCH_STEP,
  PINCH_MIN_DISTANCE_PX,
  PINCH_NOTCH_LOG_EPSILON,
  PINCH_WHEEL_PIXELS_PER_DOUBLING,
  GestureRecognizer,
  IDLE_GESTURE_RECOGNIZER,
  applyPinchToCamera,
  applyPinchToOffsetCamera,
  applyPinchToOrbit,
  clampZoom,
  gestureIsMultiTouch,
  gesturePointerDown,
  gesturePointerMove,
  gesturePointerUp,
  pinchFrame,
  pinchStep,
  pinchWheelDelta,
  pinchZoomNotches,
  recognizeGesturePointerDown,
  recognizeGesturePointerMove,
  recognizeGesturePointerUp,
  shortestAngleDelta,
  zoomAboutPoint,
} from "@semio-tech/framework-interaction/gesture";
export type { GesturePointer, GesturePointers, GestureRecognizerState, GestureTransition, GestureVerdict, OrbitPose, OrbitProjection, OrbitVector, PinchFrame, PinchStep, ZoomBounds, ZoomNotchFactors } from "@semio-tech/framework-interaction/gesture";
export * from "../../🔨️modules/🎠️kernel/📦️packages/🟦️typescript/🟦️.ts";
export * from "@semio-tech/machine";
export { NumericIndex, NumericIndexEdit, NumericIndexReader, NumericIndexRetirement } from "../../🔨️modules/🌱️value/📦️packages/🟦️typescript/🗂️ordered/🔢️numeric/🟦️.ts";
export type { NumericIndexGrant, NumericIndexStep, NumericIndexReadStep, NumericIndexOrdinal } from "../../🔨️modules/🌱️value/📦️packages/🟦️typescript/🗂️ordered/🔢️numeric/🟦️.ts";
export { RetainedUiPatchCursor, RetainedUiSnapshotCursor, RetainedUiSurfaceOwner, RetainedUiTransaction } from "@semio-tech/ui-contract/retained";
export type { RetainedUiState, RetainedUiStep, RetainedUiResult, RetainedUiRejection, RetainedUiSurfaceIdentity, RetainedUiAcknowledgement } from "@semio-tech/ui-contract/retained";
