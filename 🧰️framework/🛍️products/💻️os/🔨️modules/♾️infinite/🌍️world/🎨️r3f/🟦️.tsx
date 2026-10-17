// #region 🧲️Header
/** 🌍️ `@semio-tech/infinite-world-r3f` — generic r3f infinite-world engine: layers, chunking, view radius, pooling, precision, LOD/grid, mesh borders. */
// #endregion 🧲️Header

import { sceneHostPort, ThreeOrbitControls } from "@semio-tech/ui-react";
import { tokenHex } from "@semio-tech/ui-styling";
import {
  BoxGeometry,
  HalfFloatType,
  LinearFilter,
  LinearSRGBColorSpace,
  LineBasicMaterial,
  Matrix4,
  Mesh,
  MOUSE,
  Object3D,
  ThreeOrthographicCamera,
  ThreePerspectiveCamera,
  Vector3
} from "./🔌️adapters/🟦️.ts";
import {
  applyWorldMeshEdgeBorders,
  classifyWorldNavigationGestures,
  floatingOriginRebase,
  resetWorldMeshBorderColorCache,
  WORLD_MESH_OUTLINE_USER_DATA_KEY,
  worldEntityInspectable,
  worldEntityRendered,
  worldEntityRenderMode,
  worldEntitySelectable,
  worldMeshBorderColor
} from "./🧷️r3f-foundation/🟦️.ts";
import {
  adaptiveOrbitCameraFar,
  applyOrbitProjectionToCameraState,
  computeOrbitCameraViewState,
  createOrbitCameraViewLayoutDescriptors,
  createOrbitCameraViewTemplates,
  ORBIT_CAMERA_VIEW_COMMAND,
  orbitCameraDistance,
  orbitCameraViewRigApplyToken,
  resolveOrbitCameraViewFromTemplateId,
  shouldApplyOrbitCameraViewRigSeed,
  WORLD_ORBIT_CAMERA_MIN_FAR,
  WorldOrbitCameraViewRigSeedApply,
  worldProjectionMatchedOrthoZoom,
  worldProjectionMatchedPerspectiveDistance
} from "./📷️camera-view/🟦️.tsx";
import {
  cameraGridFadeDistance,
  cameraGridVisibleRadius,
  chunkDistanceVisible,
  chunkKey,
  lodFromCameraDistance,
  lodGridStepWorld,
  lodOrbitDistanceForCamera,
  WORLD_LOD_GRID_COVERAGE_MARGIN,
  WORLD_LOD_REFERENCE_FOV_DEG
} from "./🧮️chunking-lod/🟦️.tsx";
import {
  applyWorldReferenceTransform,
  mountWorldReferencePose,
  patchWorldReferenceProps,
  WORLD_REFERENCE_SELECTED_CONTENT_OPACITY,
  worldReferenceAppearance
} from "./🖼️reference/🟦️.tsx";
import { applyWorldVolumeTransform, worldVolumesContainAabb } from "./🧊️volume/🟦️.tsx";
import { resolveOrbitGizmoViewFromDirection, WORLD_PROJECTION_KINDS, worldProjectionModeOptions, worldProjectionSwitchTreeItems, worldProjectionTemplateApplySpec, worldProjectionTemplateSelectionId } from "./📐️projection/🟦️.tsx";
import { computeWorldProjectionPose, frameWorldProjectionPose, orbitCameraViewGumballPlane, WORLD_PROJECTION_COMMAND, worldProjectionDefaults, worldProjectionFamily, worldProjectionGumballPlane, worldProjectionOrbitConstraints, worldProjectionPerspectiveFov, worldProjectionSnapZoom, worldSceneContentBounds, worldSceneContentBoundsKey } from "./📐️projection/🧮️projection-pose/🟦️.ts";
import { worldObliqueShearMatrix, worldProjectionGoalMatrix, worldProjectionMorphMatrix, worldProjectionTransitionPose } from "./📐️projection/🔄️projection-matrices/🟦️.ts";
import { orbitViewToWorldProjectionSpec, resolveProjectionGizmoSpec, worldProjectionKindSwitchSpec, worldProjectionSpecToOrbitView } from "./📐️projection/🧭️projection-gizmo-specs/🟦️.ts";
import { WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS, WORLD_CURVILINEAR_FRAGMENT_SHADER, worldCurvilinearUnproject } from "./📐️projection/🎬️projection-rig/🟦️.tsx";
import { createWorldProjectionTemplates, decodeWorldProjectionTemplateId, encodeWorldProjectionTemplateId, worldProjectionSpecIconId, worldProjectionSpecLabel } from "./📐️projection/🏷️projection-templates/🟦️.ts";
import {
  dispatchProjectionGizmoHit,
  projectionGizmoHeadFillColor,
  projectionGizmoHitVisualState,
  resolveProjectionGizmoVisualPalette
} from "./🧭️gizmo-viewport/🟦️.tsx";
import {
  applyWorldOrbitMouseButtonsIdle,
  resolveWorldOrbitMouseButtonsIdle,
  resolveWorldOrbitRightMouseAction,
  shouldAssignWorldOrbitRightMouse
} from "./🎬️canvas/🟦️.tsx";

export {
  type ProjectionGizmoHit,
  type WorldAxonometricHemisphere,
  type WorldAxonometricQuadrant,
  type WorldAxonometricVariant,
  type WorldCurvilinearMapping,
  type WorldObliqueVariant,
  type WorldOrthographicViewId,
  type WorldProjectionFamily,
  type WorldProjectionKind,
  type WorldProjectionMode,
  type WorldProjectionOrientation,
  type WorldProjectionSpec
} from "./🔭️scene-types/🟦️.ts";

export {
  applyWorldMeshEdgeBorders,
  cadObjectLocalDirectionToThreeGroupLocal,
  cadObjectLocalToThreeGroupLocal,
  cadQuatToThree,
  cadToThreeMatrix,
  cadVec3ToThree,
  classifyWorldNavigationGestures,
  createWorldMeshEdgeOutline,
  floatingOriginRebase,
  GLB_MESH_FRAME_ROTATION_X,
  type LodMeshEntry,
  type OrbitCameraProjection,
  type Quat,
  resetWorldMeshBorderColorCache,
  type SceneListenerTarget,
  threeQuatToCad,
  threeVec3ToCad,
  type Vec3,
  WORLD_LOCKED_DESATURATION,
  WORLD_LOCKED_OPACITY_SCALE,
  WORLD_MESH_BORDER_CSS,
  WORLD_MESH_OUTLINE_USER_DATA_KEY,
  type WorldCameraState,
  type WorldEntityFlags,
  worldEntityInspectable,
  worldEntityRendered,
  worldEntityRenderMode,
  type WorldEntityRenderMode,
  type WorldEntityRenderModeInput,
  worldEntitySelectable,
  WorldEventBindingController,
  worldMeshBorderColor,
  type WorldNavigationGesture,
  type WorldNavigationSnapshot,
  type WorldNavigationThresholds
} from "./🧷️r3f-foundation/🟦️.ts";

export {
  bodiesIntersect,
  type CollisionBody,
  collisionBodyFromObject,
  collisionBodyWorldBounds,
  type CollisionMeshPart,
  solidOverlapVolume,
  type SolidOverlapVolumeOptions
} from "./🧱️collision/🟦️.ts";

export {
  adaptiveOrbitCameraFar,
  applyOrbitProjectionToCameraState,
  type ComputeOrbitCameraViewOptions,
  computeOrbitCameraViewState,
  createOrbitCameraViewLayoutDescriptors,
  createOrbitCameraViewTemplates,
  type CreateOrbitCameraViewTemplatesConfig,
  ORBIT_CAMERA_VIEW_COMMAND,
  orbitCameraDistance,
  orbitCameraProjectionForView,
  type OrbitCameraViewId,
  type OrbitCameraViewLayoutArrangement,
  type OrbitCameraViewLayoutDescriptor,
  type OrbitCameraViewLayoutPane,
  orbitCameraViewRigApplyToken,
  OrbitCameraViewSeed,
  type OrbitCameraViewTemplateDescriptor,
  resolveOrbitCameraViewFromTemplateId,
  shouldApplyOrbitCameraViewRigSeed,
  stabilizeOrbitViewDirection,
  useWorldOrbitViewSnapGate,
  WORLD_ORBIT_CAMERA_FAR_DISTANCE_FACTOR,
  WORLD_ORBIT_CAMERA_MIN_FAR,
  worldCameraFov,
  worldCameraIsOrthographic,
  worldCameraZoom,
  WorldOrbitCameraViewApplier,
  WorldOrbitCameraViewRig,
  type WorldOrbitViewSnapGateContextValue,
  WorldOrbitViewSnapGateProvider,
  worldProjectionMatchedOrthoZoom,
  worldProjectionMatchedPerspectiveDistance,
  type WorldProjectionMorphState
} from "./📷️camera-view/🟦️.tsx";

export {
  type AssetPoolPort,
  cameraGridFadeDistance,
  cameraGridVisibleRadius,
  chunkBoundsRadius,
  chunkDistanceVisible,
  chunkKey,
  createRefCountPool,
  createTemplatePool,
  DEFAULT_GRID_PLANE_ANCHOR_CAD,
  DEFAULT_LOD_GRID_FACTOR,
  DEFAULT_LOD_RANGE,
  DEFAULT_MANUAL_LOD,
  formatLod,
  GridLayer,
  type LodContextValue,
  lodFromCameraDistance,
  lodFromSliderValue,
  lodGridStepWorld,
  lodOrbitDistanceForCamera,
  pickClosestLod,
  pickClosestMeshUrl,
  sliderValueFromLod,
  splitChunkedSceneChildren,
  useLod,
  useVisibleChunkKeys,
  ViewRadiusLayer,
  type ViewRadiusLayerProps,
  WORLD_LOD_EPSILON,
  WORLD_LOD_GRID_BASE_LOD,
  WORLD_LOD_GRID_COVERAGE_MARGIN,
  WORLD_LOD_GRID_FADE_HEIGHT_FACTOR,
  WORLD_LOD_GRID_FADE_STRENGTH,
  WORLD_LOD_GRID_MIN_FADE_CELLS,
  WORLD_LOD_REFERENCE_FOV_DEG,
  WORLD_LOD_SLIDER_MAX,
  WORLD_LOD_SLIDER_MIN,
  WorldChunkedSceneChildren,
  WorldChunks,
  WorldLodBridge,
  type WorldLodBridgeProps,
  WorldLodGridHelper
} from "./🧮️chunking-lod/🟦️.tsx";

export {
  WorldLayer,
  type WorldLayerProps,
  WorldLayerStack
} from "./🗂️layers/🟦️.tsx";

export {
  applyWorldReferencePose,
  applyWorldReferenceTransform,
  mountWorldReferencePose,
  patchWorldReferenceProps,
  WORLD_REFERENCE_DEFAULT_WIDTH,
  WORLD_REFERENCE_HOVER_BACKGROUND_CSS,
  WORLD_REFERENCE_SELECTED_BACKGROUND_CSS,
  WORLD_REFERENCE_SELECTED_CONTENT_OPACITY,
  WORLD_REFERENCE_SELECTED_OUTLINE_CSS,
  worldEulerDegreesToQuat,
  worldQuatToEulerDegrees,
  worldReferenceAppearance,
  type WorldReferenceAppearance,
  WorldReferenceLayer,
  worldReferenceOrientation,
  type WorldReferenceProps,
  type WorldReferenceRelocatePayload,
  worldReferenceScaleVec,
  type WorldReferenceSource
} from "./🖼️reference/🟦️.tsx";

export {
  applyWorldVolumePose,
  applyWorldVolumeTransform,
  WORLD_VOLUME_DEFAULT_COLOR,
  WORLD_VOLUME_DEFAULT_OPACITY,
  WorldVolumeLayer,
  type WorldVolumeProps,
  type WorldVolumeRelocatePayload,
  worldVolumesContainAabb
} from "./🧊️volume/🟦️.tsx";

export { resolveOrbitGizmoViewFromDirection, WORLD_PROJECTION_KINDS, WorldOrbitProjectionSwitch, type WorldOrbitProjectionSwitchProps, WorldProjectionKindSwitch, type WorldProjectionKindSwitchProps, type WorldProjectionModeOption, worldProjectionModeOptions, worldProjectionSwitchTreeItems, worldProjectionTemplateApplySpec, worldProjectionTemplateSelectionId } from "./📐️projection/🟦️.tsx";
export { applyWorldProjectionToCameraState, computeWorldProjectionPose, frameWorldProjectionPose, orbitCameraViewGumballPlane, WORLD_PROJECTION_COMMAND, worldProjectionCardinalLook, worldProjectionCornerLook, worldProjectionDefaults, worldProjectionFamily, worldProjectionGumballPlane, worldProjectionModeFov, worldProjectionModeKind, worldProjectionOrbitConstraints, worldProjectionOrientationLook, worldProjectionOrientationsEqual, worldProjectionOrthoZoom, worldProjectionPerspectiveFov, worldProjectionSnapZoom, worldProjectionViewHalfExtent, worldSceneContentBounds, type WorldSceneContentBounds, worldSceneContentBoundsKey } from "./📐️projection/🧮️projection-pose/🟦️.ts";
export { worldObliqueShearMatrix, worldProjectionGoalMatrix, worldProjectionMorphMatrix, worldProjectionTransitionPose } from "./📐️projection/🔄️projection-matrices/🟦️.ts";
export { orbitViewToWorldProjectionSpec, projectionGizmoFaceToOrthographicView, resolveProjectionGizmoHitFromDirection, resolveProjectionGizmoSpec, worldProjectionKindSwitchSpec, worldProjectionSpecToOrbitView } from "./📐️projection/🧭️projection-gizmo-specs/🟦️.ts";
export { WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS, WORLD_CURVILINEAR_FRAGMENT_SHADER, worldCurvilinearUnproject, WorldProjectionApplier, WorldProjectionRig } from "./📐️projection/🎬️projection-rig/🟦️.tsx";
export { createWorldProjectionTemplates, type CreateWorldProjectionTemplatesConfig, decodeWorldProjectionTemplateId, encodeWorldProjectionTemplateId, worldProjectionModeLabel, worldProjectionOrientationLabel, worldProjectionSpecIconId, worldProjectionSpecLabel, type WorldProjectionTemplateDescriptor } from "./📐️projection/🏷️projection-templates/🟦️.ts";

export {
  WorldOrbitViewControls,
  type WorldOrbitViewControlsProps,
  WorldOrbitViewGizmo,
  type WorldOrbitViewGizmoProps,
  WorldOrbitViewSnapDriver,
  type WorldOrbitViewSnapDriverProps,
  WorldProjectionSnapDriver,
  type WorldProjectionSnapDriverProps
} from "./🧭️gizmo-viewport/🟦️.tsx";

export {
  applyWorldOrbitMouseButtonsIdle,
  DemandFrameloopKick,
  resolveWorldOrbitMouseButtonsIdle,
  resolveWorldOrbitRightMouseAction,
  shouldAssignWorldOrbitRightMouse,
  useWorldOrbitRightMouseBindings,
  useWorldPointerTarget,
  WorldCameraInvalidator,
  worldCameraReportTargetV1,
  WorldCanvas,
  type WorldCanvasProps,
  type WorldOrbitControlsBinding,
  WorldOrbitGated,
  type WorldOrbitGatedProps,
  type WorldOrbitRightMouseBindingsOptions
} from "./🎬️canvas/🟦️.tsx";

export { Canvas, PerspectiveCamera, useFrame, useThree, Vector3 } from "./🔌️adapters/🟦️.ts";

// #region 🧪️Tests
/** 🧪️ Exact shape of the bag `🧪️tests/🧪️chunkkey` is handed — `typeof` of the live bindings, so a
 * renamed or dropped member is a compile error on both sides rather than a silent `any`. */
export type WorldR3fTestDependencies = {
  readonly BoxGeometry: typeof BoxGeometry;
  readonly HalfFloatType: typeof HalfFloatType;
  readonly LineBasicMaterial: typeof LineBasicMaterial;
  readonly LinearFilter: typeof LinearFilter;
  readonly LinearSRGBColorSpace: typeof LinearSRGBColorSpace;
  readonly MOUSE: typeof MOUSE;
  readonly Matrix4: typeof Matrix4;
  readonly Mesh: typeof Mesh;
  readonly ORBIT_CAMERA_VIEW_COMMAND: typeof ORBIT_CAMERA_VIEW_COMMAND;
  readonly Object3D: typeof Object3D;
  readonly ThreeOrbitControls: typeof ThreeOrbitControls;
  readonly ThreeOrthographicCamera: typeof ThreeOrthographicCamera;
  readonly ThreePerspectiveCamera: typeof ThreePerspectiveCamera;
  readonly Vector3: typeof Vector3;
  readonly WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS: typeof WORLD_CURVILINEAR_CAPTURE_TARGET_OPTIONS;
  readonly WORLD_CURVILINEAR_FRAGMENT_SHADER: typeof WORLD_CURVILINEAR_FRAGMENT_SHADER;
  readonly WORLD_LOD_GRID_COVERAGE_MARGIN: typeof WORLD_LOD_GRID_COVERAGE_MARGIN;
  readonly WORLD_LOD_REFERENCE_FOV_DEG: typeof WORLD_LOD_REFERENCE_FOV_DEG;
  readonly WORLD_MESH_OUTLINE_USER_DATA_KEY: typeof WORLD_MESH_OUTLINE_USER_DATA_KEY;
  readonly WORLD_ORBIT_CAMERA_MIN_FAR: typeof WORLD_ORBIT_CAMERA_MIN_FAR;
  readonly WORLD_PROJECTION_COMMAND: typeof WORLD_PROJECTION_COMMAND;
  readonly WORLD_PROJECTION_KINDS: typeof WORLD_PROJECTION_KINDS;
  readonly WORLD_REFERENCE_SELECTED_CONTENT_OPACITY: typeof WORLD_REFERENCE_SELECTED_CONTENT_OPACITY;
  readonly WorldOrbitCameraViewRigSeedApply: typeof WorldOrbitCameraViewRigSeedApply;
  readonly adaptiveOrbitCameraFar: typeof adaptiveOrbitCameraFar;
  readonly applyOrbitProjectionToCameraState: typeof applyOrbitProjectionToCameraState;
  readonly applyWorldMeshEdgeBorders: typeof applyWorldMeshEdgeBorders;
  readonly applyWorldOrbitMouseButtonsIdle: typeof applyWorldOrbitMouseButtonsIdle;
  readonly applyWorldReferenceTransform: typeof applyWorldReferenceTransform;
  readonly mountWorldReferencePose: typeof mountWorldReferencePose;
  readonly applyWorldVolumeTransform: typeof applyWorldVolumeTransform;
  readonly cameraGridFadeDistance: typeof cameraGridFadeDistance;
  readonly cameraGridVisibleRadius: typeof cameraGridVisibleRadius;
  readonly chunkDistanceVisible: typeof chunkDistanceVisible;
  readonly chunkKey: typeof chunkKey;
  readonly classifyWorldNavigationGestures: typeof classifyWorldNavigationGestures;
  readonly computeOrbitCameraViewState: typeof computeOrbitCameraViewState;
  readonly computeWorldProjectionPose: typeof computeWorldProjectionPose;
  readonly createOrbitCameraViewLayoutDescriptors: typeof createOrbitCameraViewLayoutDescriptors;
  readonly createOrbitCameraViewTemplates: typeof createOrbitCameraViewTemplates;
  readonly createWorldProjectionTemplates: typeof createWorldProjectionTemplates;
  readonly decodeWorldProjectionTemplateId: typeof decodeWorldProjectionTemplateId;
  readonly dispatchProjectionGizmoHit: typeof dispatchProjectionGizmoHit;
  readonly encodeWorldProjectionTemplateId: typeof encodeWorldProjectionTemplateId;
  readonly floatingOriginRebase: typeof floatingOriginRebase;
  readonly frameWorldProjectionPose: typeof frameWorldProjectionPose;
  readonly lodFromCameraDistance: typeof lodFromCameraDistance;
  readonly lodGridStepWorld: typeof lodGridStepWorld;
  readonly lodOrbitDistanceForCamera: typeof lodOrbitDistanceForCamera;
  readonly orbitCameraDistance: typeof orbitCameraDistance;
  readonly orbitCameraViewGumballPlane: typeof orbitCameraViewGumballPlane;
  readonly orbitCameraViewRigApplyToken: typeof orbitCameraViewRigApplyToken;
  readonly orbitViewToWorldProjectionSpec: typeof orbitViewToWorldProjectionSpec;
  readonly patchWorldReferenceProps: typeof patchWorldReferenceProps;
  readonly projectionGizmoHeadFillColor: typeof projectionGizmoHeadFillColor;
  readonly projectionGizmoHitVisualState: typeof projectionGizmoHitVisualState;
  readonly resetWorldMeshBorderColorCache: typeof resetWorldMeshBorderColorCache;
  readonly resolveOrbitCameraViewFromTemplateId: typeof resolveOrbitCameraViewFromTemplateId;
  readonly resolveOrbitGizmoViewFromDirection: typeof resolveOrbitGizmoViewFromDirection;
  readonly resolveProjectionGizmoSpec: typeof resolveProjectionGizmoSpec;
  readonly resolveProjectionGizmoVisualPalette: typeof resolveProjectionGizmoVisualPalette;
  readonly resolveWorldOrbitMouseButtonsIdle: typeof resolveWorldOrbitMouseButtonsIdle;
  readonly resolveWorldOrbitRightMouseAction: typeof resolveWorldOrbitRightMouseAction;
  readonly sceneHostPort: typeof sceneHostPort;
  readonly shouldApplyOrbitCameraViewRigSeed: typeof shouldApplyOrbitCameraViewRigSeed;
  readonly shouldAssignWorldOrbitRightMouse: typeof shouldAssignWorldOrbitRightMouse;
  readonly tokenHex: typeof tokenHex;
  readonly worldCurvilinearUnproject: typeof worldCurvilinearUnproject;
  readonly worldEntityInspectable: typeof worldEntityInspectable;
  readonly worldEntityRenderMode: typeof worldEntityRenderMode;
  readonly worldEntityRendered: typeof worldEntityRendered;
  readonly worldEntitySelectable: typeof worldEntitySelectable;
  readonly worldMeshBorderColor: typeof worldMeshBorderColor;
  readonly worldObliqueShearMatrix: typeof worldObliqueShearMatrix;
  readonly worldProjectionDefaults: typeof worldProjectionDefaults;
  readonly worldProjectionFamily: typeof worldProjectionFamily;
  readonly worldProjectionGoalMatrix: typeof worldProjectionGoalMatrix;
  readonly worldProjectionGumballPlane: typeof worldProjectionGumballPlane;
  readonly worldProjectionKindSwitchSpec: typeof worldProjectionKindSwitchSpec;
  readonly worldProjectionMatchedOrthoZoom: typeof worldProjectionMatchedOrthoZoom;
  readonly worldProjectionMatchedPerspectiveDistance: typeof worldProjectionMatchedPerspectiveDistance;
  readonly worldProjectionModeOptions: typeof worldProjectionModeOptions;
  readonly worldProjectionMorphMatrix: typeof worldProjectionMorphMatrix;
  readonly worldProjectionOrbitConstraints: typeof worldProjectionOrbitConstraints;
  readonly worldProjectionPerspectiveFov: typeof worldProjectionPerspectiveFov;
  readonly worldProjectionSnapZoom: typeof worldProjectionSnapZoom;
  readonly worldProjectionSpecIconId: typeof worldProjectionSpecIconId;
  readonly worldProjectionSpecLabel: typeof worldProjectionSpecLabel;
  readonly worldProjectionSpecToOrbitView: typeof worldProjectionSpecToOrbitView;
  readonly worldProjectionSwitchTreeItems: typeof worldProjectionSwitchTreeItems;
  readonly worldProjectionTemplateApplySpec: typeof worldProjectionTemplateApplySpec;
  readonly worldProjectionTemplateSelectionId: typeof worldProjectionTemplateSelectionId;
  readonly worldProjectionTransitionPose: typeof worldProjectionTransitionPose;
  readonly worldReferenceAppearance: typeof worldReferenceAppearance;
  readonly worldSceneContentBounds: typeof worldSceneContentBounds;
  readonly worldSceneContentBoundsKey: typeof worldSceneContentBoundsKey;
  readonly worldVolumesContainAabb: typeof worldVolumesContainAabb;
};

// #endregion 🧪️Tests
