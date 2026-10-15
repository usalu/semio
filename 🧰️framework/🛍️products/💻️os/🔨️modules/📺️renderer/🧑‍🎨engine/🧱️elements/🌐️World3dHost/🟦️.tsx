// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/World3dHost/component.tsx
/** 🌐️ `World3dHost` — the 3D world viewport scene host: mesh/instance parsing, point-cloud and vortex-marker
 * layers, catalogue-drop and selection-preview stores, instance chrome, the transform gumball, and the full R3F
 * `World3dHost` component mounted inside a Mode window. The largest scene host in this package. */
// #endregion 🧲️Header

import React, {
  useCallback,
  useContext,
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  Suspense,
  useSyncExternalStore,
  type DragEvent
} from "react";
import type { ThreeCamera } from "@semio-tech/ui-react";
// #region 🔌️Adapters
import {
  CATALOGUE_DRAG_MIME,
  cn,
  ContextMenuController,
  getActiveCatalogueDragPayload,
  glassClass,
  IconShotFrame,
  isContextMenuPointerTarget,
  marqueeCoverageFromGesture,
  registerTutorialCameraDriver,
  SelectionMarquee,
  sunPositionFromAzimuthElevation,
  chromePanelSafeAreaStyle,
  useChromePanelSafeArea,
  useLabel,
  useShellScopeOptional,
  uiSpacingPx,
  windowChromeClearedTopOffset,
  type GumballHandleKind,
  type GumballPose,
  type IconName,
  type SelectionMarqueeCoverage,
  type SelectionMarqueePoint,
  type TutorialCameraDriver,
  Box3,
  BufferGeometry,
  Group,
  Vector3
} from "@semio-tech/ui-react";
import {
  ToolRunTraceLayer,
  toolRunTraceDataAttributes,
  useToolRunTraceCursorEcho,
  useToolRunTraceStore,
  type ToolRunTraceRecordStore
} from "./⏯️tool-run-trace/🟦️.tsx";
import {
  createWorld3dAnnotationStore,
  resolveWorld3dHighlightPaint,
  useStableWorld3dLane,
  useWorld3dScalarFieldMeshes,
  World3dAnnotationOverlay,
  World3dAnnotationProjector,
  World3dScalarLegendView,
  World3dSectionClip,
  world3dModellingDataAttributes,
  world3dModellingStrings,
  resolveWorld3dToneHex,
  world3dSelectionWithPickFilter,
  world3dSubElementPaint
} from "./📏️modelling/🟦️.tsx";
import {
  GestureRecognizer,
  parseWorld3dAnnotationLayer,
  parseWorld3dModellingOptions,
  parseWorld3dScalarField,
  world3dComputeStatusV1,
  type ComponentSceneHostProps,
  type MergeMode
} from "@semio-tech/framework";
import {
  DEFAULT_LOD_GRID_FACTOR,
  DEFAULT_MANUAL_LOD,
  worldProjectionDefaults,
  worldProjectionOrbitConstraints,
  worldProjectionSpecIconId,
  worldProjectionSpecLabel,
  worldSceneContentBounds,
  WorldCanvas,
  WorldLodBridge,
  WorldOrbitGated,
  WorldOrbitViewControls,
  WorldOrbitViewSnapGateProvider,
  WorldProjectionRig,
  WorldReferenceLayer,
  WorldVolumeLayer,
  type WorldCameraState,
  type WorldProjectionSpec,
  type WorldVolumeRelocatePayload
} from "@semio-tech/infinite-world-r3f";
import { CAMERA_SYNC_DEBOUNCE_MS } from "../📐️Canvas2dHost/🟦️.tsx";
import { WorldTerrainLayer } from "../🗺️WorldTerrainLayer/🟦️.tsx";
import {
  createCoalescingActionDispatcher,
  declareSurfaceCancelAction,
  world3dMarqueeOverlayShape,
  type Puzzle3dBrushMeshPage,
  puzzle3dAnnounceableBrushMeshUrls,
  puzzle3dBrushMeshDigest,
  puzzle3dBrushMeshPages,
  drainPuzzle3dBrushMeshQueue,
  PUZZLE3D_MESH_UPLOAD_QUEUE_PAGES,
  puzzle3dBrushMeshRegistry,
  NOTE_WORLD_NAVIGATION_ACTION_ID
} from "../🛠️ShellHelpers/🟦️.tsx";
import { SetWindowIconContext, SetWindowTitleContext } from "../🏛️ShellHost/🎚️shell-model/🟦️.tsx";
import {
  suggestionMenuOwnsWindow,
  suggestionPopupOwnsWindow,
  suggestionRowsWithFocus,
  suggestionSubmenuTarget,
  useSuggestionSubmenuSearch,
  withSuggestionSubmenu
} from "../🎣️suggestion-submenu/🟦️.ts";
import { CanvasPresenceOverlayV1, useLocalPresenceActorIdV1 } from "../👕️canvas-presence/🟦️.tsx";
// #endregion 🔌️Adapters
import {
  PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS,
  publishLocalPresenceWindowViewV1,
  clearLocalPresenceWindowViewV1,
  publishLocalActiveToolV1
} from "../👕️canvas-presence/🟦️.ts";
import {
  autofitCameraFromInstances,
  isTransparentWorldBackground,
  mergeWorldViewportCamera,
  parseCameraState,
  parseEnvironment,
  parseFit,
  parseFrame,
  seedPendingWorldProjectionCamera,
  shouldReattachWorldViewportCamera,
  WORLD3D_FRAME_BOUNDS_MARGIN,
  world3dAutoFitKey,
  world3dBoundsRadius,
  world3dCameraDomJson,
  world3dFitProjectionContent,
  world3dFrameCameraFromBounds,
  world3dFrameCameraFromInstances,
  world3dFramingInstances,
  world3dProjectionContentFrameMounted,
  world3dSunDomJson,
  world3dViewportCameraSeedKey,
  type WorldAttractionRecord,
  worldCameraSetCameraDispatchArgs,
  type WorldInstanceRecord,
  type WorldMeshRecord,
  type WorldParsedCameraState,
  type WorldReferenceRecord,
  type WorldTargetVolumeRecord,
  type WorldVortexRecord
} from "./🔭️scene-schema/🟦️.ts";
import {
  advanceWorldMeshResidency,
  celebrateWorldInstances,
  geometryFromMesh,
  semanticColorsFromPalette,
  useMeshStylePalette,
  WORLD_VERTEX_MARK_PX,
  WorldAcceptedFrameDiagnostics,
  WorldAutoFit,
  type WorldMeshResidencyV1,
  WorldProjectionContentFrame
} from "./🎨️mesh-render/🟦️.tsx";
import { BrushMeshRegistrar, ToolRunTraceGlbGeometry } from "./🧩️glb-surface/🟦️.tsx";
import {
  advanceWorldInstanceResidency,
  clientPointOverHost,
  hoveredInteractionTargetV1,
  leftoverBrushRetainGuestHoverV1,
  leftoverWorldWindowOverlayV1,
  mergeWorldInteractionWithLeftoverV1,
  mergeWorldSelectionWithLeftoverV1,
  parseEngagementPreview,
  parseInteraction,
  parseLod,
  parseSelection,
  resolveWorldContextMenuTarget,
  subscribeLeftoverWorldSelectionV1,
  WORLD3D_DEFAULT_INTERACTION_GRANULARITY,
  WORLD3D_DEFAULT_MARKER_GRANULARITY,
  world3dComponentInteractionTarget,
  world3dContextMenuSurfaceV1,
  world3dInstanceInteractionTarget,
  world3dInstanceInteractionTargets,
  world3dMarkerInteractionTarget,
  world3dRetainLocalVortexHover,
  world3dSuggestionsAltHeld,
  world3dSuggestionsGestureArmed,
  world3dSuggestionsGestureConsumesContextMenu,
  type World3dSuggestionsRightDownRoute,
  world3dSuggestionsRightDownRoutes,
  worldInstancePickBlocked,
  type WorldInstanceResidencyV1,
  worldSurfaceGuestSelectionDomV1,
  worldSurfaceSelectionDomV1
} from "./🔗️instance-selection/🟦️.ts";
import {
  getWorldGumballTransformPreview,
  getWorldGumballTransformPreviewServerSnapshot,
  gumballConfigForTransformMode,
  isWorldTransformGumballMode,
  subscribeWorldGumballTransformPreview,
  WORLD_GUMBALL_IDLE,
  WORLD_PAINT_IDLE,
  world3dGumballSelectionArgsV1,
  worldAttractionsWithGumballPreview,
  type WorldGumballEvent,
  type WorldGumballGesture,
  worldGumballOwnerPoses,
  worldGumballStep,
  type WorldPaintEvent,
  type WorldPaintGesture,
  type WorldPaintOrigin,
  worldPaintStep,
  worldVorticesWithGumballPreview
} from "./🧮️gumball-math/🟦️.ts";
import { useWorldMeshVisuals, WorldInstancesLayer, WorldPointCloudLayer, WorldVortexMarkers } from "./🧱️instance-layers/🟦️.tsx";
import {
  CameraRefBridge,
  EngagementPreviewLayer,
  IntroductionWorldResolverBridge,
  MARQUEE_DRAG_THRESHOLD_PX,
  mergeIdSet,
  paneSuffixFromSurfaceId,
  RaycasterPickTuning,
  raycastGroundPoint,
  resolveClickInstanceId,
  resolveMarqueeComponentIds,
  resolveMarqueeInstanceIds,
  resolveWorldMergeMode,
  suggestionMenuItems,
  world3dMarqueePointerCaptureArmed,
  WorldAttractionLines,
  WorldComputeStatusPane,
  WorldConnectRubberBand,
  WorldGumballHitStamp,
  WorldSurfaceStatusPane,
  WorldVortexHitStamp,
  WorldVoxelPreviewBox
} from "./🎯️marquee-and-bridges/🟦️.tsx";
import {
  CatalogueDropGhost,
  CatalogueDropPreviewInvalidate,
  clearWorldCatalogueDropPreview,
  clearWorldSelectionPreview,
  getWorldCatalogueDropPreview,
  getWorldCatalogueDropPreviewServerSnapshot,
  getWorldSelectionPreview,
  getWorldSelectionPreviewServerSnapshot,
  parsePuzzle3dCatalogueDragPayload,
  registerWorldCatalogueDropHost,
  resolveCatalogueDropOrigin,
  setWorldCatalogueDropPreview,
  setWorldSelectionPreview,
  subscribeWorldCatalogueDropPreview,
  subscribeWorldSelectionPreview,
  world3dRelocateDispatchArgsV1,
  world3dRelocateDragTargetV1,
  type World3dRelocateSession,
  world3dVolumeBrushCommits,
  world3dVolumeBrushOriginV1,
  worldCatalogueDropHostContainsPoint
} from "./🗂️catalogue-drop/🟦️.tsx";
import {
  axisDragParam,
  peekPendingWorldProjection,
  type World3dRetainedViewportV1,
  World3dWindowViewStoreContext,
  WorldOrbitProjectionSwitchPane
} from "./🪟️window-view-store/🟦️.tsx";
import { openSurfaceContextMenu, useShellContextMenuFallback, type SurfaceContextMenuResult, useMapContextMenuSpecs } from "../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { parseJsonArray, WindowInstanceIdContext } from "../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";

export {
  buildWorldCameraDispatchArgs,
  mergeWorldViewportCamera,
  parseCameraState,
  shouldReattachWorldViewportCamera,
  WORLD3D_FRAME_BOUNDS_MARGIN,
  world3dAutoFitKey,
  world3dAutoFitOwed,
  world3dBoundsRadius,
  world3dCameraDomJson,
  world3dFitProjectionContent,
  world3dFrameCameraFromBounds,
  world3dFrameCameraFromInstances,
  world3dFrameDistanceForRadius,
  world3dFrameVisibleOverlayOffered,
  world3dFramingInstances,
  type World3dMarkerInteractionFields,
  world3dProjectionContentFrameMounted,
  world3dSunDomJson,
  world3dViewportCameraSeedKey,
  worldCameraPoseApproxEqual,
  worldCameraSetCameraDispatchArgs,
  type WorldInstanceRecord,
  type WorldMeshAttribute,
  type WorldMeshData,
  type WorldParsedCameraState
} from "./🔭️scene-schema/🟦️.ts";

export {
  advanceWorldMeshResidency,
  buildMeshVisuals,
  celebrateWorldInstances,
  disposeMeshVisuals,
  geometryFromMesh,
  isCurveOnlyWorldMesh,
  isWorldInstanceCelebrating,
  meshBoundsCorners,
  meshDataFromKind,
  type MeshVisuals,
  resolveMeshSelectionPreviewStyle,
  resolveMeshStyle,
  type SemanticColors,
  semanticColorsFromPalette,
  splitJsonArrayElements,
  world3dInstanceLocalCorners,
  worldInstanceHighlighted,
  type WorldMeshAppearance,
  worldMeshMaterialRevision,
  type WorldMeshResidencyV1,
  worldSurfaceMaterial
} from "./🎨️mesh-render/🟦️.tsx";

export {
  cloneWorldGlbSurface,
  disposeWorldGlbSurface
} from "./🧩️glb-surface/🟦️.tsx";

export {
  advanceWorldInstanceResidency,
  applyWorldInstanceMeshRaycast,
  hoveredInteractionTargetV1,
  interactionTargetsForInstances,
  leftoverBrushRetainGuestHoverV1,
  leftoverHoveredVortexFullIdV1,
  leftoverOverlayArmedBrushUtilityV1,
  leftoverOverlayCarryingSelectionV1,
  leftoverOverlayCarryingUtilityV1,
  leftoverSelectIdsMustNameHoverPickV1,
  leftoverTreeItemSelectedV1,
  leftoverWorldArmedWindowIdV1,
  leftoverWorldArmedWindowOverlayV1,
  leftoverWorldOverlayAppliesV1,
  leftoverWorldOverlayIdsInDocumentV1,
  type LeftoverWorldOverlayScopeV1,
  leftoverWorldSelectionOverlayV1,
  type LeftoverWorldSelectionOverlayV1,
  leftoverWorldWindowOverlayV1,
  mergeWorldInteractionWithLeftoverV1,
  mergeWorldSelectionWithLeftoverV1,
  parseWorldInstanceDelta,
  publishLeftoverWorldSelectionV1,
  resolveVortexPointerDownIntent,
  resolveWorldContextMenuTarget,
  subscribeLeftoverWorldSelectionV1,
  WORLD3D_DEFAULT_INTERACTION_GRANULARITY,
  WORLD3D_DEFAULT_MARKER_GRANULARITY,
  world3dComponentInteractionTarget,
  world3dContextMenuSurfaceV1,
  world3dInstanceInteractionTarget,
  world3dInstanceInteractionTargets,
  world3dInstancePickUsesInteractionDomain,
  world3dMarkerInteractionTarget,
  type World3dMarkerLayer,
  world3dRetainLocalVortexHover,
  world3dSuggestionsAltHeld,
  world3dSuggestionsGestureArmed,
  world3dSuggestionsGestureConsumesContextMenu,
  world3dSuggestionsRightDownRoutesOnWindowCapture,
  worldActiveInstanceIdV1,
  type WorldInstanceDeltaV1,
  worldInstanceMeshRaycast,
  worldInstancePickBlocked,
  type WorldInstanceResidencyV1,
  worldSurfaceGuestSelectionDomV1,
  worldSurfaceSelectionDomV1,
  worldVortexHitProxy
} from "./🔗️instance-selection/🟦️.ts";

export {
  applyGumballLivePreviewDeltaToPose,
  applyGumballLivePreviewPoseToObject3D,
  clearWorldGumballTransformPreview,
  getWorldGumballTransformPreview,
  getWorldGumballTransformPreviewServerSnapshot,
  gumballConfigForTransformMode,
  gumballIdentityDelta,
  gumballLivePreviewDeltaBetweenPoses,
  gumballPreviewOwnedWorldDirection,
  gumballPreviewOwnedWorldPoint,
  gumballTransformDeltaBetweenPoses,
  isWorldInstanceRootObject3D,
  isWorldTransformGumballMode,
  setWorldGumballTransformPreview,
  subscribeWorldGumballTransformPreview,
  WORLD_GUMBALL_IDLE,
  WORLD_PAINT_IDLE,
  world3dGumballSelectionArgsV1,
  worldAttractionsWithGumballPreview,
  worldGumballConfigForProjection,
  type WorldGumballDispatch,
  type WorldGumballEvent,
  type WorldGumballGesture,
  type WorldGumballLivePreviewDelta,
  type WorldGumballOwnerPose,
  worldGumballOwnerPoses,
  worldGumballStep,
  type WorldGumballStepResult,
  type WorldGumballTargets,
  type WorldGumballTransformPreview,
  type WorldPaintEvent,
  type WorldPaintGesture,
  type WorldPaintOrigin,
  worldPaintStep,
  worldVorticesWithGumballPreview
} from "./🧮️gumball-math/🟦️.ts";

export {
  worldHoverPaintIdV1,
  type WorldLocalHoverV1,
  worldVortexMaterialRevision
} from "./🧱️instance-layers/🟦️.tsx";

export {
  componentPickMergeMode,
  raycastGroundPoint,
  resolveClickInstanceIdFromProjected,
  resolveWorldMergeMode,
  suggestionMenuItems,
  WORLD3D_MARQUEE_DRAG_THRESHOLD_PX,
  world3dMarqueePointerCaptureArmed,
  world3dProjectedAabbContainsClick,
  worldGhostMeshUrl,
  WorldSurfaceStatusPane
} from "./🎯️marquee-and-bridges/🟦️.tsx";

export {
  clearWorldCatalogueDropPreview,
  clearWorldSelectionPreview,
  getWorldCatalogueDropPreview,
  getWorldCatalogueDropPreviewServerSnapshot,
  getWorldSelectionPreview,
  getWorldSelectionPreviewServerSnapshot,
  parsePuzzle3dCatalogueDragPayload,
  registerWorldCatalogueDropHost,
  setWorldCatalogueDropPreview,
  setWorldSelectionPreview,
  snapWorldPointToGrid,
  subscribeWorldCatalogueDropPreview,
  subscribeWorldSelectionPreview,
  world3dRelocateDispatchArgsV1,
  world3dRelocateDragTargetV1,
  world3dVolumeBrushCommits,
  world3dVolumeBrushOriginV1,
  worldCatalogueDropHostContainsPoint,
  type WorldSelectionPreview
} from "./🗂️catalogue-drop/🟦️.tsx";

export {
  clearPendingWorldProjection,
  clearPendingWorldProjections,
  registerPendingWorldProjection,
  world3dProjectionPaneElementId,
  type World3dRetainedViewportV1,
  World3dWindowViewRegistryV1,
  World3dWindowViewStoreContext,
  World3dWindowViewStoreV1,
  WorldOrbitProjectionSwitchPane
} from "./🪟️window-view-store/🟦️.tsx";

//#region 🔖️World3dHost
//#region WorldSceneParsing
/** ⏯️ Mounts the scene's `toolRunTrace` lane for this window: one instanced mesh per `(mesh, verdict)` batch,
 * each drawn through `meshesJson[mesh]` — inline mesh data directly, a GLB once it has loaded, a unit box
 * until then. Only meshes a resident `instance3d` record names are loaded.
 * @see ./⏯️tool-run-trace/🟦️.tsx */
function WorldToolRunTrace({ lane, meshes, trace, focus }: { readonly lane: string | null | undefined; readonly meshes: readonly WorldMeshRecord[]; readonly trace: { readonly store: ToolRunTraceRecordStore; readonly version: number }; readonly focus?: bigint | null }) {
  const inline = useMemo(() => meshes.map((mesh) => (mesh.data ? geometryFromMesh(mesh.data) : null)), [meshes]);
  useEffect(() => () => inline.forEach((geometry) => geometry?.dispose()), [inline]);
  const [loaded, setLoaded] = useState<ReadonlyMap<string, BufferGeometry>>(() => new Map());
  const onGeometry = useCallback((url: string, geometry: BufferGeometry | null) => {
    setLoaded((previous) => {
      const next = new Map(previous);
      if (geometry) next.set(url, geometry);
      else next.delete(url);
      return next;
    });
  }, []);
  const urls = useMemo(() => {
    const named = new Set<string>();
    for (const batch of trace.store.batches()) {
      const mesh = batch.family === "instance3d" && batch.count > 0 ? meshes[batch.index] : undefined;
      if (mesh?.url && !mesh.data) named.add(mesh.url);
    }
    return [...named];
  }, [meshes, trace.store, trace.version]);
  const geometryForMesh = useCallback((index: number) => inline[index] ?? loaded.get(meshes[index]?.url ?? "") ?? null, [inline, loaded, meshes]);
  return (
    <>
      {urls.map((url) => (
        <Suspense key={url} fallback={null}>
          <ToolRunTraceGlbGeometry url={url} onGeometry={onGeometry} />
        </Suspense>
      ))}
      <ToolRunTraceLayer lane={lane} store={trace} geometryForMesh={geometryForMesh} focus={focus} />
    </>
  );
}
//#endregion WorldSceneParsing

//#region 🎯️WorldInteractionDomain
/** 🖱️ Mirrors `nodeGraphHoverActionArgs` (`NodeGraph/🟦️.tsx`) for a world window bound to a
 * framework interaction domain (`scene.domainId`), reporting the scene's own
 * `domainGranularityId` — the same shape a `domain_id`-aware `interactionHover` handler expects. */
export function world3dHoverActionArgs(domainId: string, granularity: string, id: string | null | undefined) {
  const targets = id ? [{ granularity, id }] : [];
  return { domainId, channel: "pointer", targets: JSON.stringify(targets) };
}

/** 🖱️ A selection is a SET of topology ids, so the target list this builds is one: the same id
 * can be handed in twice (several rendered instances of one channel, a pick batch the caller did not
 * run through {@link interactionTargetsForInstances}), and a `Select` must be idempotent per id —
 * the host never emits a duplicate target. Live defect 2026-09-12 (ticket
 * 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️selection-dedupe-2026-09-12.md`): the guest republished one
 * picked id twice, and only a set on BOTH sides makes that unrepresentable. */
/** 🎯️ `merge` is a `MergeMode` — the ONE vocabulary of `🕹️interaction/🧬️schema/🔣️.json`, put on the
 * wire verbatim. A host must NOT translate it: the framework's `parse_merge_mode` accepts exactly
 * those five words and faults on anything else. */
export function world3dSelectionActionArgs(domainId: string, granularity: string, ids: readonly string[], merge: MergeMode) {
  const targets = [...new Set(ids)].map((id) => ({ granularity, id }));
  return { domainId, targets: JSON.stringify(targets), merge, method: "pick" };
}

/** 🎯️ `interactionSelect` args over already-resolved `{ granularity, id }` targets. */
export function world3dSelectionTargetsActionArgs(domainId: string, targets: readonly { readonly granularity: string; readonly id: string }[], merge: MergeMode, method: "pick" | "rectangle" | "lasso" = "pick") {
  return { domainId, targets: JSON.stringify(targets), merge, method };
}
//#endregion 🎯️WorldInteractionDomain

//#region World3dHost
/** 🚚️ `node.world3d` arrives ASSEMBLED. Its payload fields (`instancesJson`, `vorticesJson`, … — see
 * `WORLD3D_SCENE_LANES`) no longer ride inside the surface doc's 32 KiB `UiFixedBytes` blob, which
 * cannot page and refused a 57 KB Nakagin world outright; each is published as its own retained,
 * individually paged text carrier beside the surface node and reattached by the Interpreter's
 * `PagedSurfaceView` before this host ever sees it (ticket 26/09/02 wave P). Every `*Json` field is
 * therefore still a plain string here, and the per-lane `useMemo`s below stay the incremental seam:
 * a lane whose content did not change keeps its identical string, so its parse never re-runs. The three
 * modelling lanes (`annotations`, `scalarField`, `modellingOptions`) arrive already typed (`encoding: "json"`)
 * and keep their identity by lane hash (`useStableWorld3dLane`). */
export function World3dHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.world3d;
  // 🪟️ Non-empty only — an empty-string `domainId` (never emitted by the Rust side, but defensive
  // against a hand-authored fixture) must fall back to the plugin-private action pathway below.
  const interactionDomainId = scene?.domainId ? scene.domainId : undefined;
  const interactionGranularity = scene?.domainGranularityId ? scene.domainGranularityId : WORLD3D_DEFAULT_INTERACTION_GRANULARITY;
  // 🐚️ Optional — this host is also unit-tested standalone, outside any `ShellScopeProvider`.
  const shellScope = useShellScopeOptional();
  // 🐚️ Read as reactive state (not `shellScope.selection.get()` inline at each use) because this value
  // is consumed inside `<Canvas>` r3f subtrees (`WorldInstancesLayer`) via a prop, not context — R3F
  // primitives aren't guaranteed to re-render just because an *outer* DOM component's context changed,
  // so the toolbar's mode toggle needs an explicit subscription to actually propagate.
  const [persistentSelectionMode, setPersistentSelectionMode] = useState<MergeMode>(() => shellScope?.selection.get() ?? "replace");
  useEffect(() => {
    if (!shellScope) return;
    return shellScope.selection.subscribe(() => setPersistentSelectionMode(shellScope.selection.get()));
  }, [shellScope]);
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const windowViewStore = useContext(World3dWindowViewStoreContext);
  const setWindowTitle = useContext(SetWindowTitleContext);
  const setWindowIcon = useContext(SetWindowIconContext);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const meshStylePalette = useMeshStylePalette();
  const colors = useMemo(() => semanticColorsFromPalette(meshStylePalette), [meshStylePalette]);
  const sceneCameraJson = scene?.cameraJson ?? "{}";
  const parsedCamera = useMemo(() => parseCameraState(sceneCameraJson), [sceneCameraJson]);
  // 🚚️ B44: the retained instance set advances by the producer's own per-object delta when it can, so a
  // pose edit on a 180-object document substitutes the moved records by id instead of re-parsing 55 KiB.
  const instanceResidencyRef = useRef<WorldInstanceResidencyV1 | null>(null);
  const instances = useMemo(() => {
    const advanced = advanceWorldInstanceResidency(instanceResidencyRef.current, scene?.instancesJson ?? "[]", scene?.instancesDeltaJson ?? null);
    instanceResidencyRef.current = advanced;
    return advanced.records as WorldInstanceRecord[];
  }, [scene?.instancesJson, scene?.instancesDeltaJson]);
  const references = useMemo(() => parseJsonArray<WorldReferenceRecord>(scene?.referencesJson), [scene?.referencesJson]);
  const sceneCamera = useMemo(() => {
    if (sceneCameraJson.includes('"position"')) return parsedCamera;
    return instances.length > 0 ? autofitCameraFromInstances(instances) : parsedCamera;
  }, [instances, parsedCamera, sceneCameraJson]);
  const contentBounds = useMemo(() => worldSceneContentBounds(world3dFramingInstances(instances), references), [instances, references]);
  const [retainedViewport] = useState(() => windowViewStore?.read(windowInstanceId) ?? null);
  const pendingProjectionSpecRef = useRef<WorldProjectionSpec | null>(null);
  const [viewportCamera, setViewportCamera] = useState<WorldParsedCameraState | null>(() => {
    if (retainedViewport) return retainedViewport.camera;
    const pendingSpec = peekPendingWorldProjection(windowInstanceId);
    if (!pendingSpec) return null;
    pendingProjectionSpecRef.current = pendingSpec;
    return seedPendingWorldProjectionCamera(pendingSpec, sceneCamera, instances, references);
  });
  const [projectionFramePending, setProjectionFramePending] = useState(() => retainedViewport === null && pendingProjectionSpecRef.current !== null);
  const [viewportOwned, setViewportOwned] = useState(retainedViewport?.viewportOwned ?? false);
  const [cameraNavigating, setCameraNavigating] = useState(false);
  /** 🔒️ The fit revision the USER last moved this camera on. While it names the live revision the auto-fit
   * lane stands down; a new revision (a document/example swap) is a new framing the user has not refused yet.
   * Deliberately not `viewportOwned`, which the auto-fit's own `onFitted` sets. */
  const [userMovedFitRevision, setUserMovedFitRevision] = useState<number | null>(retainedViewport?.userMovedFitRevision ?? null);
  const liveFitRevisionRef = useRef(0);
  const noteUserMovedCamera = useCallback(() => setUserMovedFitRevision(liveFitRevisionRef.current), []);
  const [detachEpoch, setDetachEpoch] = useState(retainedViewport?.detachEpoch ?? 0);
  /** 📷️ First content-frame remounts orbit controls; later tool-driven bound expansions only soft-update the camera. */
  const projectionContentFrameSeededRef = useRef(retainedViewport?.projectionContentFrameSeeded ?? false);
  const previousSceneCameraJsonRef = useRef(retainedViewport?.observedSceneCameraJson ?? sceneCameraJson);
  /** 🧭️ The last camera pose this component itself dispatched via debounced `setCamera` — lets the reattach
   * effect below recognize the plugin echoing it straight back (see `shouldReattachWorldViewportCamera`). */
  const lastDispatchedWorldCameraRef = useRef<WorldCameraState | null>(retainedViewport?.lastDispatchedCamera ?? null);
  /** 📷️ The scene-camera text the RIG is seeded from — advanced only by a genuine EXTERNAL camera change,
   * never by the plugin echoing this component's own `setCamera` straight back.
   *
   * ⚖️ Keying the rig on the raw `scene.cameraJson` made every completed gesture remount the camera: the
   * debounced `setCamera` reaches the guest, the guest republishes the pose on the window config lane, the
   * key changes, `WorldProjectionRig` mounts a fresh camera element, and `WorldOrbitControlsBridge`'s effect
   * then constructs a NEW `OrbitControls` — whose target starts at the world origin. Measured 2026-09-15 on
   * the generation3d preview: an orbit round-tripped with `target [1,1,1]`, and the very next wheel reported
   * `target [0,0,0]`, dragging the framed camera off the example
   * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`). The reattach
   * predicate already knows which changes are external; the seed key now reads its verdict instead of
   * re-deciding from the text. */
  const [sceneCameraAttachJson, setSceneCameraAttachJson] = useState(retainedViewport?.sceneCameraAttachJson ?? sceneCameraJson);
  const [externalPendingProjectionSpec, setExternalPendingProjectionSpec] = useState<WorldProjectionSpec | null>(retainedViewport?.pendingProjectionSpec ?? null);
  const retainedPendingProjectionSpecRef = useRef(retainedViewport?.pendingProjectionSpec ?? null);
  const retainedViewportSnapshotRef = useRef<World3dRetainedViewportV1>({
    camera: viewportCamera,
    viewportOwned,
    userMovedFitRevision,
    detachEpoch,
    projectionContentFrameSeeded: projectionContentFrameSeededRef.current,
    lastDispatchedCamera: lastDispatchedWorldCameraRef.current,
    observedSceneCameraJson: previousSceneCameraJsonRef.current,
    sceneCameraAttachJson,
    pendingProjectionSpec: retainedPendingProjectionSpecRef.current,
  });
  const retainViewport = useCallback(
    (patch: Partial<World3dRetainedViewportV1>) => {
      const next = { ...retainedViewportSnapshotRef.current, ...patch };
      retainedViewportSnapshotRef.current = next;
      windowViewStore?.write(windowInstanceId, next);
    },
    [windowInstanceId, windowViewStore],
  );
  useLayoutEffect(() => {
    retainViewport({
      camera: viewportCamera,
      viewportOwned,
      userMovedFitRevision,
      detachEpoch,
      projectionContentFrameSeeded: projectionContentFrameSeededRef.current,
      lastDispatchedCamera: lastDispatchedWorldCameraRef.current,
      observedSceneCameraJson: previousSceneCameraJsonRef.current,
      sceneCameraAttachJson,
      pendingProjectionSpec: retainedPendingProjectionSpecRef.current,
    });
  }, [detachEpoch, retainViewport, sceneCameraAttachJson, userMovedFitRevision, viewportCamera, viewportOwned]);
  useEffect(() => {
    // 🧭️ Always advance the tracking ref, even when we're about to suppress a reattach below — otherwise the
    // NEXT comparison would still diff against this stale value instead of the pose we just saw.
    const previousSceneCameraJson = previousSceneCameraJsonRef.current;
    previousSceneCameraJsonRef.current = sceneCameraJson;
    retainViewport({ observedSceneCameraJson: sceneCameraJson });
    if (!shouldReattachWorldViewportCamera(previousSceneCameraJson, sceneCameraJson, lastDispatchedWorldCameraRef.current)) return;
    setSceneCameraAttachJson(sceneCameraJson);
    setViewportCamera(null);
    setViewportOwned(false);
    // 🎯️ A reattach DISCARDS whatever pose the viewport held — the auto-fit's framing and the user's own
    // gesture alike — so the one-shot framing is re-armed with it. Measured 2026-09-15: the generation3d
    // editor framed `rectangle-extrude-volume`, the guest then republished its seed camera on the window
    // config lane, the reattach reset the viewport to that seed, and `WorldAutoFit` never fired again
    // because its key had already been consumed — leaving the example cut off on exactly the boot a user
    // sees (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
    setUserMovedFitRevision(null);
    setCameraNavigating(false);
    setDetachEpoch(0);
    setProjectionFramePending(Boolean(pendingProjectionSpecRef.current));
    projectionContentFrameSeededRef.current = false;
  }, [retainViewport, sceneCameraJson]);
  const cameraState = viewportCamera ?? sceneCamera;

  const cameraSeedKey = world3dViewportCameraSeedKey(sceneCameraAttachJson, detachEpoch);

  // 🎥️ Registers this window's live camera get/set for tutorial playback/recording (see
  // `registerTutorialCameraDriver` — modeled on `registerIntroductionSurfaceResolver`). `get` reads the
  // CURRENT pose via a ref (never a stale closure) for deviation-then-play convergence; `set` writes a
  // viewport-owned override and bumps `detachEpoch` so `OrbitControls` reseeds from the new pose instead
  // of fighting it, exactly like a programmatic view-preset apply already does.
  const cameraStateRef = useRef(cameraState);
  cameraStateRef.current = cameraState;
  useEffect(() => {
    if (!windowInstanceId) return undefined;
    const driver: TutorialCameraDriver = {
      get: () => {
        const live = cameraStateRef.current;
        return {
          kind: "orbit",
          position: [live.position[0], live.position[1], live.position[2]],
          target: [live.target[0], live.target[1], live.target[2]],
          up: live.up ? [live.up[0], live.up[1], live.up[2]] : [0, 0, 1],
          fov: live.fov,
        };
      },
      set: (pose) => {
        if (pose.kind !== "orbit") return;
        const live = cameraStateRef.current;
        const next = { ...live, position: pose.position, target: pose.target, up: pose.up ?? live.up, fov: pose.fov ?? live.fov };
        setViewportCamera(next);
        retainViewport({ camera: next });
        setDetachEpoch((epoch) => epoch + 1);
      },
    };
    return registerTutorialCameraDriver(windowInstanceId, driver);
  }, [retainViewport, windowInstanceId]);
  // 📏️ The typed modelling lanes keep their identity while their content hash is unchanged, so an unrelated refresh
  // that re-parses every JSON lane never re-validates a multi-megabyte scalar field.
  const laneHash = (lane: string) => scene?.lanes?.find((reference) => reference.lane === lane)?.hash;
  const rawAnnotations = useStableWorld3dLane(scene?.annotations, laneHash("annotations"));
  const rawScalarField = useStableWorld3dLane(scene?.scalarField, laneHash("scalarField"));
  const rawModellingOptions = useStableWorld3dLane(scene?.modellingOptions, laneHash("modellingOptions"));
  const annotationLayer = useMemo(() => (rawAnnotations === undefined ? undefined : parseWorld3dAnnotationLayer(rawAnnotations) ?? undefined), [rawAnnotations]);
  const scalarField = useMemo(() => (rawScalarField === undefined ? undefined : parseWorld3dScalarField(rawScalarField) ?? undefined), [rawScalarField]);
  const modellingOptions = useMemo(() => (rawModellingOptions === undefined ? undefined : parseWorld3dModellingOptions(rawModellingOptions) ?? undefined), [rawModellingOptions]);
  const annotationStore = useMemo(createWorld3dAnnotationStore, []);
  const highlightPaint = useMemo(() => resolveWorld3dHighlightPaint(modellingOptions?.highlight), [modellingOptions?.highlight, meshStylePalette]);
  const subElementPaint = useMemo(() => world3dSubElementPaint(colors, highlightPaint, { edgeWidth: 3, vertexMarkPx: WORLD_VERTEX_MARK_PX }), [colors, highlightPaint]);
  const sectionCapHex = useMemo(() => (modellingOptions?.section?.cap ? resolveWorld3dToneHex(modellingOptions.section.cap.tone) : undefined), [modellingOptions?.section?.cap, meshStylePalette]);
  // 🧊️ The retained mesh set advances per mesh, so a refresh that republishes an unchanged mesh keeps
  // that record's object identity and the instanced layer keeps its already-built buffers.
  const geometryRevisionRef = useRef({source: scene?.meshesJson, instances: scene?.instancesJson, revision: 1});
  if (geometryRevisionRef.current.source !== scene?.meshesJson || geometryRevisionRef.current.instances !== scene?.instancesJson) geometryRevisionRef.current = {source: scene?.meshesJson, instances: scene?.instancesJson, revision: geometryRevisionRef.current.revision + 1};
  const meshResidencyRef = useRef<WorldMeshResidencyV1 | null>(null);
  const meshes = useMemo(() => {
    const advanced = advanceWorldMeshResidency(meshResidencyRef.current, scene?.meshesJson ?? "[]");
    meshResidencyRef.current = advanced;
    return advanced.records as WorldMeshRecord[];
  }, [scene?.meshesJson]);
  const heatmap = useWorld3dScalarFieldMeshes(meshes, scalarField);
  const meshVisuals=useWorldMeshVisuals(heatmap.records as readonly WorldMeshRecord[]);
  const paneLeftover = useSyncExternalStore(subscribeLeftoverWorldSelectionV1, () => leftoverWorldWindowOverlayV1(windowInstanceId), () => leftoverWorldWindowOverlayV1(windowInstanceId));
  const mergedSelection = useMemo(() => mergeWorldSelectionWithLeftoverV1(parseSelection(scene?.selectionJson ?? "{}"), paneLeftover, instances), [paneLeftover, scene?.selectionJson, instances]);
  const selection = useMemo(() => world3dSelectionWithPickFilter(mergedSelection, modellingOptions?.pickFilter), [mergedSelection, modellingOptions?.pickFilter]);
  const vortices = useMemo(() => parseJsonArray<WorldVortexRecord>(scene?.vorticesJson), [scene?.vorticesJson]);
  const attractions = useMemo(() => parseJsonArray<WorldAttractionRecord>(scene?.attractionsJson), [scene?.attractionsJson]);
  const targetVolumes = useMemo(() => parseJsonArray<WorldTargetVolumeRecord>(scene?.targetVolumesJson), [scene?.targetVolumesJson]);
  const interaction = useMemo(() => mergeWorldInteractionWithLeftoverV1(parseInteraction(scene?.interactionJson), paneLeftover), [paneLeftover, scene?.interactionJson]);
  const lod = useMemo(() => parseLod(scene?.lodJson), [scene?.lodJson]);
  const engagementPreview = useMemo(() => parseEngagementPreview(scene?.engagementPreviewJson), [scene?.engagementPreviewJson]);
  const environment = useMemo(() => parseEnvironment(scene?.environmentJson), [scene?.environmentJson]);
  const frame = useMemo(() => parseFrame(scene?.frameJson), [scene?.frameJson]);
  const fit = useMemo(() => parseFit(scene?.fitJson), [scene?.fitJson]);
  /** 📦️ The producer's OWN delivered extent, when it published one — the single thing this pane frames,
   * for the boot framing AND for the `Frame visible` button.
   *
   * ⚖️ Measured 2026-09-15 on the served generation3d preview: `Frame visible` left the camera on its seed
   * pose because `Box3.setFromObject(instancesGroupRef)` came back empty for a payload whose geometry rides
   * inline `data` buffers, and the instance fallback then framed a table of instances that all sit at the
   * origin (the geometry is baked into the mesh, not into the instance transform). A pane can only frame
   * what it can measure — so the producer publishes what it delivered
   * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`). */
  const autoFitBounds = useMemo<readonly [readonly number[], readonly number[]] | null>(() => {
    const minimum = fit?.boundsMin;
    const maximum = fit?.boundsMax;
    if (!Array.isArray(minimum) || !Array.isArray(maximum) || minimum.length < 3 || maximum.length < 3) return null;
    if (!minimum.every((value) => Number.isFinite(value)) || !maximum.every((value) => Number.isFinite(value))) return null;
    if ((minimum as number[]).every((value, axis) => value === (maximum as number[])[axis])) return null;
    return [minimum, maximum];
  }, [fit?.boundsMin, fit?.boundsMax]);
  /** ✂️ Half the side of the section cap plane: twice the delivered extent's diagonal, so the cap covers the cut wherever the plane origin sits. */
  const sectionExtent = useMemo(() => {
    if (autoFitBounds) return Math.max(1, 2 * Math.hypot(autoFitBounds[1][0]! - autoFitBounds[0][0]!, autoFitBounds[1][1]! - autoFitBounds[0][1]!, autoFitBounds[1][2]! - autoFitBounds[0][2]!));
    if (contentBounds) return Math.max(1, 4 * Math.hypot(contentBounds.halfExtent[0], contentBounds.halfExtent[1], contentBounds.halfExtent[2]));
    return 100;
  }, [autoFitBounds, contentBounds]);
  liveFitRevisionRef.current = fit?.revision ?? 0;
  // 🧵️ Off-main-thread compute status (see `World3dScene.statusJson`) — the meshes above stay the
  // last-known-good (stale) cache while a plugin worker's `flowEvalTick` chain is still resolving.
  const computeStatus = useMemo(() => world3dComputeStatusV1(scene?.statusJson), [scene?.statusJson]);
  const computing = computeStatus.computing;
  const toolRunTrace = useToolRunTraceStore(scene?.toolRunTrace, useToolRunTraceCursorEcho(windowInstanceId));
  // 🛑️ While this surface offers a cancel affordance it DECLARES the action id to the shell, so the
  // shell's one action funnel can retire the requesting instance's in-flight extension requests
  // before forwarding the gesture — without the shell ever learning a domain verb from code
  // (`isDeclaredSurfaceCancelAction`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
  const cancelActionId = computeStatus.cancellable ? computeStatus.cancelAction : "";
  useEffect(() => declareSurfaceCancelAction(cancelActionId), [cancelActionId]);
  const activeUtility = interaction.activeUtility ?? "select";
  const brushMode = activeUtility === "brush";
  const volumeBrushMode = activeUtility === "volumeBrush";
  const relocateMode = activeUtility === "worldRelocate";
  const volumeLayersInteractive = !brushMode && !volumeBrushMode;
  const hostRef = useRef<HTMLDivElement | null>(null);
  const presencePublishAtRef = useRef(0);
  const presenceWindowId = windowInstanceId || node.surfaceId || "world";
  const localPresenceActor = useLocalPresenceActorIdV1("local");
  const presenceObjectWorldPositions = useMemo(() => {
    const map: Record<string, readonly [number, number, number]> = {};
    for (const instance of instances) {
      if (!instance.id) continue;
      const position =
        instance.position ??
        ([instance.x ?? 0, instance.y ?? 0, instance.z ?? 0] as [number, number, number]);
      map[instance.id] = position;
    }
    return map;
  }, [instances]);

  useEffect(() => () => clearLocalPresenceWindowViewV1(presenceWindowId), [presenceWindowId]);
  const publishWorldPresenceView = useCallback((pointer?: readonly [number, number, number], rayOrigin?: readonly [number, number, number]) => {
    const now = Date.now();
    if (now - presencePublishAtRef.current < PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS) return;
    presencePublishAtRef.current = now;
    const rect = hostRef.current?.getBoundingClientRect();
    const size: [number, number] = [Math.max(1, rect?.width ?? 1), Math.max(1, rect?.height ?? 1)];
    const tip = pointer ?? (cameraState.target as [number, number, number]);
    const origin = rayOrigin ?? (cameraState.position as [number, number, number]);
    publishLocalPresenceWindowViewV1("local", presenceWindowId, {
      windowId: presenceWindowId,
      space: "world",
      kind: {
        kind: "orbit",
        position: cameraState.position as [number, number, number],
        target: cameraState.target as [number, number, number],
        up: (cameraState.up as [number, number, number] | undefined) ?? [0, 0, 1],
        fov: cameraState.fov ?? 45,
      },
      size,
      pointer: tip,
      rayOrigin: origin,
    });
    publishLocalActiveToolV1("local", activeUtility);
  }, [presenceWindowId, cameraState, activeUtility]);

  useEffect(() => {
    publishLocalActiveToolV1("local", activeUtility);
  }, [activeUtility]);

  const overlayRailRef = useRef<HTMLDivElement | null>(null);
  const overlayRailSafeArea = useChromePanelSafeArea({ hostRef, affordanceRef: overlayRailRef, anchor: "top-right", yieldAxis: "either", gapPx: uiSpacingPx(1) });
  const instancesGroupRef = useRef<Group | null>(null);
  const lodRef = useRef(DEFAULT_MANUAL_LOD);
  const [marqueePath, setMarqueePath] = useState<readonly SelectionMarqueePoint[]>([]);
  const [marqueeModifiers, setMarqueeModifiers] = useState<{ readonly shiftKey: boolean; readonly ctrlKey: boolean; readonly metaKey: boolean }>({ shiftKey: false, ctrlKey: false, metaKey: false });
  const [gumballDragActive, setGumballDragActive] = useState(false);
  const [faceDragSession, setFaceDragSession] = useState<{
    readonly objectId: string;
    readonly faceId: number;
    readonly normal: readonly [number, number, number];
    readonly startPoint: readonly [number, number, number];
    readonly faceExtent?: readonly [number, number];
  } | null>(null);
  const [connectDragSource, setConnectDragSource] = useState<{
    readonly fullId: string;
    readonly position: readonly [number, number, number];
    readonly record?: WorldVortexRecord;
  } | null>(null);
  const [connectDragHoverPosition, setConnectDragHoverPosition] = useState<readonly [number, number, number] | null>(null);
  const displayVortices = useMemo(() => {
    if (!connectDragSource) return vortices;
    if (vortices.some((vortex) => vortex.fullId === connectDragSource.fullId)) return vortices;
    const retained =
      connectDragSource.record ??
      ({
        fullId: connectDragSource.fullId,
        position: connectDragSource.position,
      } satisfies WorldVortexRecord);
    return [...vortices, retained];
  }, [connectDragSource, vortices]);
  const [vortexPointerArm, setVortexPointerArm] = useState<{
    readonly fullId: string;
    readonly position: readonly [number, number, number];
    readonly clientX: number;
    readonly clientY: number;
    readonly event: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean };
  } | null>(null);
  const [voxelHoverOrigin, setVoxelHoverOrigin] = useState<readonly [number, number, number] | null>(null);
  const catalogueDropPreview = useSyncExternalStore(
    subscribeWorldCatalogueDropPreview,
    () => getWorldCatalogueDropPreview(node.controllerId),
    () => getWorldCatalogueDropPreviewServerSnapshot(node.controllerId),
  );
  const sharedSelectionPreview = useSyncExternalStore(
    subscribeWorldSelectionPreview,
    () => getWorldSelectionPreview(node.controllerId),
    () => getWorldSelectionPreviewServerSnapshot(node.controllerId),
  );
  const sharedGumballTransformPreview = useSyncExternalStore(
    subscribeWorldGumballTransformPreview,
    () => getWorldGumballTransformPreview(node.controllerId),
    () => getWorldGumballTransformPreviewServerSnapshot(node.controllerId),
  );
  const gumballOwnerPoses = useMemo(() => worldGumballOwnerPoses(instances), [instances]);
  const previewVortices = useMemo(
    () => worldVorticesWithGumballPreview(displayVortices, sharedGumballTransformPreview, gumballOwnerPoses),
    [displayVortices, sharedGumballTransformPreview, gumballOwnerPoses],
  );
  const previewAttractions = useMemo(
    () => worldAttractionsWithGumballPreview(attractions, displayVortices, sharedGumballTransformPreview, gumballOwnerPoses),
    [attractions, displayVortices, sharedGumballTransformPreview, gumballOwnerPoses],
  );
  const gumballTransformPreviewSourceId = windowInstanceId ?? node.surfaceId;
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number; readonly suggestTarget: string | null }) | null>(null);
  const contextMenuOpenEpochRef = useRef(0);
  const dismissContextMenu = useCallback(() => {
    contextMenuOpenEpochRef.current += 1;
    setContextMenu(null);
  }, []);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.scene");
  const cameraRef = useRef<ThreeCamera | null>(null);
  const catalogueDragDepthRef = useRef(0);
  const catalogueDragEncodedRef = useRef<string | null>(null);
  /** 🎉️ Pre-drop instance ids — when the scene gains new ids after {@link commitCatalogueDropAt}, those objects celebrate. */
  const pendingCelebrateCatalogueDropIdsRef = useRef<ReadonlySet<string> | null>(null);
  const instancesRef = useRef(instances);
  instancesRef.current = instances;
  const meshesRef = useRef(meshes);
  meshesRef.current = meshes;
  const marqueeStartRef = useRef<SelectionMarqueePoint | null>(null);
  /** 🤏️ The shared `👆️gesture` recognizer for this viewport. A second finger hands the surface to the
   * orbit rig's pinch (the same recognizer inside `WorldOrbitControlsBridge` drives the camera): the
   * host's single-pointer lane — marquee, pick, paint stroke, relocate, engagement — stays quiet until
   * the LAST finger lifts, so the finger left down never grows a marquee or replays a pick. */
  const [gestureRecognizer] = useState(() => new GestureRecognizer());
  const vorticesRef = useRef(vortices);
  vorticesRef.current = vortices;
  const referencesRef = useRef(references);
  referencesRef.current = references;
  const wasMarqueeDragRef = useRef(false);
  const [marqueeCommitHold, setMarqueeCommitHold] = useState<{
    readonly mergedComponentIds: readonly number[] | null;
    readonly mergedInstanceIds: readonly string[] | null;
  } | null>(null);
  const connectDropConsumedRef = useRef(false);
  const engagementPointerMoveInFlightRef = useRef(false);
  const engagementPointerMoveLastPointRef = useRef<readonly [number, number, number] | null>(null);
  /** 🚚️ Open Relocate-utility drag, or `null`. A ref, not state: mid-drag the only thing that changes is
   * the shared ghost origin, so the gesture never re-renders this host by itself (same discipline as the
   * gumball's local preview). */
  const relocateSessionRef = useRef<World3dRelocateSession | null>(null);
  /** 🕹️ The gumball gesture, stepped by `worldGumballStep`: its ids are the ones it GRABBED, pinned at drag start.
   * Resolving them per dispatch read whatever the selection happened to be when the delta was enqueued — and a gumball
   * drag routinely changes the selection under itself (a press that misses the handle is a canvas pick that clears it).
   * An id-less delta reaches the guest, matches no document object, and completes with a refusal notice instead of an
   * edit: the long-standing `gumball-scene-delta` red, where `data-instances-json` never moved. */
  const gumballGestureRef = useRef<WorldGumballGesture>(WORLD_GUMBALL_IDLE);
  /** 🧲️ Serialized gesture chain — mid-drag is local-only unless the selection record asks for live dispatch;
   * either way every delta, the live `commit` and a host `abort` ride this one FIFO. */
  const gumballDragChainRef = useRef(Promise.resolve());
  const gumballLiveDispatch = Boolean(selection.gumballLiveDispatch);
  const selectionMode = selection.selectionMode ?? selection.granularity ?? "mesh";
  const gridSnapEnabled = lod.gridSnapEnabled ?? false;
  const suggestionMenuOpen = Boolean(interaction.suggestionMenu?.open);
  const suggestionMenuOwnsThisWindow = suggestionMenuOwnsWindow(interaction.suggestionMenu, windowInstanceId ?? undefined);
  const suggestionPopupOwnsThisWindow = suggestionPopupOwnsWindow(interaction.suggestionMenu, windowInstanceId ?? undefined);
  /** 🔦️ The trace record of the suggestion row under the pointer — the viewport draws only that candidate. */
  const [suggestionFocus, setSuggestionFocus] = useState<bigint | null>(null);
  const suggestionMenuCheckingPlacementLabel = useLabel("ui.host.checkingPlacement");
  const suggestionMenuNoPlacementLabel = useLabel("ui.host.noPlacement");
  const suggestionMenuTitleLabel = useLabel("ui.surfaceContextMenu.placementSuggestions");
  const gridFactor = lod.gridFactor ?? interaction.gridFactor ?? DEFAULT_LOD_GRID_FACTOR;
  const marqueeDown = marqueePath.length > 0;
  const method = selection.method ?? "rectangle";
  const marqueeStart = marqueePath[0];
  const marqueeEnd = marqueePath[marqueePath.length - 1];
  const marqueeDragActive = marqueeDown && marqueePath.length > 1 && marqueeStart != null && marqueeEnd != null && Math.hypot(marqueeEnd.x - marqueeStart.x, marqueeEnd.y - marqueeStart.y) > MARQUEE_DRAG_THRESHOLD_PX;
  const marqueeMergeMode = useMemo(() => resolveWorldMergeMode(selection.selectionMergeMode, marqueeModifiers, persistentSelectionMode), [marqueeModifiers, selection.selectionMergeMode, persistentSelectionMode]);
  const marqueeCoverage: SelectionMarqueeCoverage = useMemo(() => {
    if (!marqueeDragActive || !marqueeStart || !marqueeEnd) return "full";
    return marqueeCoverageFromGesture({ method, startX: marqueeStart.x, endX: marqueeEnd.x, path: marqueePath });
  }, [marqueeDragActive, marqueeEnd, marqueePath, marqueeStart, method]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({
        controllerId: node.controllerId,
        action,
        args: { surfaceId: node.surfaceId, windowId: windowInstanceId ?? node.surfaceId, ...args },
      });
    },
    [node.controllerId, node.surfaceId, onAction, windowInstanceId],
  );

  /** 🏁️ `dispatch`'s awaitable twin for every SELF-GATING lane — the coalescing hover and brush target
   * dispatchers. `dispatch` discards `onAction`'s promise, and `onAction` is what settles
   * on the guest's `OperationCompleted` frame, so each gate built on "at most one round trip outstanding"
   * (`createCoalescingActionDispatcher`, `createInFlightSkippingInterval`) cleared itself on the next
   * microtask and gated NOTHING. Wave B33 measured one 70-move brush hover storm enqueuing 72
   * `interactionHover` + 85 background tick turns with 11/10 of them settled, after which the next user
   * action (`addTargetVolume`) waited behind the backlog past its 30 s budget
   * (`📓️2026-09-12-wave-B33-full-run-vs-fresh-lane.md` §3). Same envelope as `dispatch`, awaitable — the
   * shape `dispatchBrushMesh` already uses for the mesh page lane. */
  const dispatchSettled = useCallback(
    (action: string, args?: Record<string, unknown>) =>
      Promise.resolve(
        onAction({
          controllerId: node.controllerId,
          action,
          args: { surfaceId: node.surfaceId, windowId: windowInstanceId ?? node.surfaceId, ...args },
        }),
      ),
    [node.controllerId, node.surfaceId, onAction, windowInstanceId],
  );

  const adoptViewportCamera = useCallback(
    (next: WorldCameraState, applyToRig: boolean) => {
      setViewportCamera((prev) => {
        const merged = mergeWorldViewportCamera(prev ?? sceneCamera, next);
        retainViewport({ camera: merged, viewportOwned: true });
        return merged;
      });
      setViewportOwned(true);
      if (applyToRig) setDetachEpoch((epoch) => epoch + 1);
    },
    [retainViewport, sceneCamera],
  );

  // 🧭️ Syncs a completed user-driven camera gesture (orbit/pan/zoom end, gizmo view snap) to the plugin so
  // the guest owns one authoritative pose per window instance — the plugin re-derives every camera-dependent
  // projection (LOD, vortex markers, sun) from it, and a reattaching pane adopts it instead of snapping back.
  // `setCamera` is `ActionKind::View` publishing on the WindowConfig lane ONLY: it is per-window session view
  // state, never a document edit and never an artifact-history row (the plugin runtime's `dispatch_emit`
  // skips the command log for exactly this shape). Trailing-debounced like the 2D canvas' own camera sync
  // (`CAMERA_SYNC_DEBOUNCE_MS`) so a scroll/wheel burst doesn't spam one dispatch per tick. Never wired into
  // a programmatic camera change (auto-fit, scene-camera echo reattach) — only genuine user gestures call this.
  const cameraDispatchTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const dispatchWorldCameraDebounced = useCallback(
    (state: WorldCameraState) => {
      if (cameraDispatchTimeoutRef.current) clearTimeout(cameraDispatchTimeoutRef.current);
      cameraDispatchTimeoutRef.current = setTimeout(() => {
        cameraDispatchTimeoutRef.current = null;
        lastDispatchedWorldCameraRef.current = state;
        retainViewport({ lastDispatchedCamera: state });
        dispatch("setCamera", worldCameraSetCameraDispatchArgs(windowInstanceId ?? node.surfaceId, state));
      }, CAMERA_SYNC_DEBOUNCE_MS);
    },
    [dispatch, node.surfaceId, retainViewport, windowInstanceId],
  );
  useEffect(
    () => () => {
      if (cameraDispatchTimeoutRef.current) clearTimeout(cameraDispatchTimeoutRef.current);
    },
    [],
  );

  const referenceSelectedIds = useMemo(() => {
    if (!selection.referenceSelectedId) return new Set<string>();
    return new Set([selection.referenceSelectedId]);
  }, [selection.referenceSelectedId]);

  const targetVolumeSelectedIds = useMemo(() => new Set(selection.targetVolumeIds ?? []), [selection.targetVolumeIds]);

  const volumeGumballConfig = useMemo(() => {
    if (activeUtility !== "transform") return undefined;
    return selection.gumballConfig ?? gumballConfigForTransformMode("transform");
  }, [activeUtility, selection.gumballConfig]);

  const referenceHoveredId = useMemo(() => {
    const hovered = selection.hoveredId;
    if (!hovered?.startsWith("reference:")) return null;
    return hovered.slice("reference:".length);
  }, [selection.hoveredId]);

  const handleReferenceSelect = useCallback(
    (id: string) => {
      const reference = references.find((entry) => entry.id === id);
      if (interactionDomainId) {
        const target = world3dMarkerInteractionTarget("reference", id, reference);
        dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, target.granularity, reference?.locked ? [] : [target.id], "replace"));
        return;
      }
      if (reference?.locked) {
        dispatch("worldPick", {
          granularity: selectionMode,
          id: null,
          merge: "replace",
        });
        return;
      }
      dispatch("setReferenceSelection", {
        pane: paneSuffixFromSurfaceId(node.surfaceId),
        referenceId: id,
      });
    },
    [dispatch, interactionDomainId, node.surfaceId, references, selectionMode],
  );

  /** 🏁️ Reference-marker hover is a POINTERMOVE lane like the instance and vortex ones, so it coalesces onto
   * the latest marker and keeps one round trip outstanding — it used to dispatch one guest turn per move with
   * no gate at all, the same unbounded shape wave B33 measured on the vortex lane (§3). The record lookup
   * reads `referencesRef` so a republished scene cannot rebuild the dispatcher and drop its in-flight state. */
  const handleReferenceHover = useMemo(
    () =>
      createCoalescingActionDispatcher<string | null>((id) => {
        if (interactionDomainId) {
          const target = id ? world3dMarkerInteractionTarget("reference", id, referencesRef.current.find((entry) => entry.id === id)) : null;
          return dispatchSettled("interactionHover", world3dHoverActionArgs(interactionDomainId, target?.granularity ?? WORLD3D_DEFAULT_MARKER_GRANULARITY.reference, target?.id));
        }
        if (!id) return dispatchSettled("referenceHover", {});
        return dispatchSettled("referenceHover", { referenceId: id });
      }),
    [dispatchSettled, interactionDomainId],
  );

  const handleTargetVolumeSelect = useCallback(
    (id: string) => {
      const volume = targetVolumes.find((entry) => entry.id === id);
      if (interactionDomainId) {
        const target = world3dMarkerInteractionTarget("targetVolume", id, volume);
        dispatch("interactionSelect", world3dSelectionActionArgs(interactionDomainId, target.granularity, volume?.locked ? [] : [target.id], "replace"));
        return;
      }
      if (volume?.locked) {
        dispatch("worldPick", { granularity: selectionMode, id: null, merge: "replace" });
        return;
      }
      dispatch("setSelection", {
        selection: { objectIds: [], vortexIds: [], attractionIds: [], targetVolumeIds: [id], referenceIds: [] },
      });
    },
    [dispatch, interactionDomainId, selectionMode, targetVolumes],
  );

  const handleTargetVolumeRelocate = useCallback(
    (payload: WorldVolumeRelocatePayload) => {
      dispatch("relocateTargetVolume", {
        volumeId: payload.volumeId,
        mode: payload.mode,
        before: payload.before,
        after: payload.after,
      });
    },
    [dispatch],
  );

  // 🥽️ One GLB's collision geometry never fitted a single retained command: the shared puzzle contract
  // admits 8 192 raw JSON bytes and the Concrete Forest mesh alone encoded to 64 KB, so every upload is
  // a page run (`puzzle3dBrushMeshPages`) — the plugin stages the pages
  // under `(url, digest)` and installs the mesh when the last one lands. A mesh THIS GUEST already holds
  // is re-announced by `{url, digest}` alone, which it serves from its own content-addressed store
  // instead of taking the bytes again.
  //
  // 🚚️ Which meshes that is is `puzzle3dBrushMeshRegistry`'s to say, not this component's: the claim is
  // scoped to the guest instantiation that justified it, confirmed only when a run's LAST page goes out,
  // and voided by a fallen `meshResidency`. The claim used to live in a bare page-lifetime `Map` written
  // at enqueue time, so every activation after a guest restart re-announced seven identities the guest
  // held nothing for and the brush utility silently kept no collision geometry until a full reload.
  //
  // ⏳️ The run is a BACK-PRESSURED stream, never a burst. A macrotask drain (`setTimeout(drain, 0)`) put
  // every page of every mesh into the actor's one command queue within a couple of hundred milliseconds
  // and then left them there: browser-measured 2026-09-12 on wasm #47, 202 pages enqueued in 21 s, 40 of
  // them settled over the next 420 s, and ONE user click that landed mid-run waited 44.3 s behind 15
  // pages (`📓️2026-09-12-wave-B22-brush-mesh-upload.md`). Awaiting each page's own settle before the
  // next is queued bounds what a user action can ever queue behind at ONE page, whatever the run's
  // length — `onAction` settles on the dispatched action's typed-operation completion, which is exactly
  // the "this page is in" signal the old fire-and-forget shape threw away.
  //
  // 🪢️ And a run is only ever paged ONCE PER DIGEST. Every `dist/mesh/*.glb` in this repo is the same
  // 771 728-byte capsule, so a scene placing several object kinds paged byte-identical geometry once per
  // mesh id — 72 commands each. A digest this page already put into THIS guest is announced by
  // `{url, digest}` alone and the guest aliases its own derived page onto the new id
  // (`adopt_brush_mesh_by_digest`, `✏️editor/⏳️precompute/🦀️.rs`); a guest that cannot serve it answers
  // on `meshReuploadUrls` and the claim is re-driven, so an optimistic announcement is self-healing.
  const brushMeshQueueRef = useRef<Puzzle3dBrushMeshPage[]>([]);
  const brushMeshRunsRef = useRef(new Map<string, string>());
  const brushMeshDrainingRef = useRef(false);
  const brushMeshLiveRef = useRef(true);
  const [brushMeshRevisions, setBrushMeshRevisions] = useState<{ readonly generation: number; readonly urls: Readonly<Record<string, number>> }>({ generation: 0, urls: {} });
  /** 🥽️ `dispatch`'s awaitable twin for the one lane that needs back pressure — same envelope, but the
   * caller can wait for the page to be IN before it queues the next one. */
  const dispatchBrushMesh = useCallback(
    (args: Record<string, unknown>): Promise<unknown> =>
      Promise.resolve(
        onAction({
          controllerId: node.controllerId,
          action: "registerBrushMesh",
          args: { surfaceId: node.surfaceId, windowId: windowInstanceId ?? node.surfaceId, ...args },
        }),
      ),
    [node.controllerId, node.surfaceId, onAction, windowInstanceId],
  );
  const drainBrushMeshQueue = useCallback(async () => {
    if (brushMeshDrainingRef.current) return;
    brushMeshDrainingRef.current = true;
    try {
      await drainPuzzle3dBrushMeshQueue(
        brushMeshQueueRef.current,
        {
          holdsDigest: (digest) => puzzle3dBrushMeshRegistry.holdsDigest(digest),
          mayAlias: (url) => puzzle3dBrushMeshRegistry.mayAlias(url),
          alias: (url, digest) => {
            brushMeshRunsRef.current.delete(url);
            puzzle3dBrushMeshRegistry.alias(url, digest);
          },
          confirm: (url, digest) => {
            brushMeshRunsRef.current.delete(url);
            puzzle3dBrushMeshRegistry.confirm(url, digest);
          },
        },
        dispatchBrushMesh,
        () => brushMeshLiveRef.current,
      );
    } finally {
      brushMeshDrainingRef.current = false;
    }
  }, [dispatchBrushMesh]);
  const handleRegisterBrushMesh = useCallback(
    (url: string, positions: number[], indices: number[]) => {
      const digest = puzzle3dBrushMeshDigest(positions, indices);
      if (puzzle3dBrushMeshRegistry.holds(url, digest)) {
        void dispatchBrushMesh({ url, digest });
        return;
      }
      if (puzzle3dBrushMeshRegistry.mayAlias(url) && puzzle3dBrushMeshRegistry.holdsDigest(digest)) {
        puzzle3dBrushMeshRegistry.alias(url, digest);
        void dispatchBrushMesh({ url, digest });
        return;
      }
      if (brushMeshRunsRef.current.get(url) === digest) return;
      const pages = puzzle3dBrushMeshPages(url, node.surfaceId, positions, indices);
      if (pages.length === 0 || brushMeshQueueRef.current.length + pages.length > PUZZLE3D_MESH_UPLOAD_QUEUE_PAGES) return;
      brushMeshRunsRef.current.set(url, digest);
      brushMeshQueueRef.current.push(...pages);
      void drainBrushMeshQueue();
    },
    [dispatchBrushMesh, drainBrushMeshQueue, node.surfaceId],
  );
  useEffect(
    () => () => {
      brushMeshLiveRef.current = false;
      for (const page of brushMeshQueueRef.current.splice(0)) {
        brushMeshRunsRef.current.delete(page.url);
        puzzle3dBrushMeshRegistry.forget(page.url);
      }
    },
    [],
  );
  // 🚚️ The guest's two published facts about what it actually holds, folded in before anything is
  // announced: a residency that fell proves a restarted guest (its mesh store is not part of any
  // checkpoint), which voids EVERY claim this page holds, and `meshReuploadUrls` names identities an
  // announcement already sent is waiting on bytes for. Either way the answer is the same — bump the
  // revision the mounted `BrushMeshRegistrar`s carry, so they re-announce their already-loaded GLB
  // and, with the claim gone, take the page path instead of the id-only one.
  const meshResidency = interaction.meshResidency;
  const meshReuploadUrls = interaction.meshReuploadUrls;
  useEffect(() => {
    if (meshResidency === undefined) return;
    const restarted = puzzle3dBrushMeshRegistry.observeResidency(meshResidency);
    const claimed = (meshReuploadUrls ?? []).filter((url) => puzzle3dBrushMeshRegistry.claimReupload(url));
    if (!restarted && claimed.length === 0) return;
    setBrushMeshRevisions((previous) => {
      const urls: Record<string, number> = { ...previous.urls };
      for (const url of claimed) urls[url] = (previous.urls[url] ?? 0) + 1;
      return { generation: previous.generation + (restarted ? 1 : 0), urls };
    });
  }, [meshResidency, meshReuploadUrls]);

  // 🥽️ Every mesh the scene lane names — placed kinds and catalogue kinds alike — is uploaded for collision, so
  // the brush suggestions run tests a candidate against its real geometry.
  //
  // 🚪️ …but only for a guest that OWNS that lane — see `puzzle3dAnnounceableBrushMeshUrls`.
  const brushMeshUrls = useMemo(() => puzzle3dAnnounceableBrushMeshUrls(meshResidency, meshes), [meshResidency, meshes]);

  const handleFrameVisibleInstances = useCallback(() => {
    noteUserMovedCamera();
    const aspect = hostRef.current ? Math.max(hostRef.current.clientWidth, 1) / Math.max(hostRef.current.clientHeight, 1) : 1;
    if (autoFitBounds) {
      const [minimum, maximum] = autoFitBounds;
      const center: [number, number, number] = [((minimum[0] ?? 0) + (maximum[0] ?? 0)) * 0.5, ((minimum[1] ?? 0) + (maximum[1] ?? 0)) * 0.5, ((minimum[2] ?? 0) + (maximum[2] ?? 0)) * 0.5];
      adoptViewportCamera(world3dFrameCameraFromBounds(center, world3dBoundsRadius(minimum, maximum), cameraState, undefined, aspect), true);
      return;
    }
    const group = instancesGroupRef.current;
    if (group) {
      const box = new Box3().setFromObject(group);
      if (!box.isEmpty()) {
        const center = box.getCenter(new Vector3());
        const radius = world3dBoundsRadius([box.min.x, box.min.y, box.min.z], [box.max.x, box.max.y, box.max.z]);
        if (radius > 1e-6) {
          adoptViewportCamera(world3dFrameCameraFromBounds([center.x, center.y, center.z], radius, cameraState, undefined, aspect), true);
          return;
        }
      }
    }
    adoptViewportCamera(world3dFrameCameraFromInstances(instances, cameraState), true);
  }, [adoptViewportCamera, autoFitBounds, noteUserMovedCamera, cameraState, instances]);

  const handleZoomToSelection = useCallback(() => {
    const selectedIds = new Set(selection.ids ?? []);
    if (selectedIds.size === 0) {
      handleFrameVisibleInstances();
      return;
    }
    const selected = instances.filter((instance) => selectedIds.has(instance.id));
    if (selected.length === 0) {
      handleFrameVisibleInstances();
      return;
    }
    let centerX = 0;
    let centerY = 0;
    let centerZ = 0;
    for (const instance of selected) {
      const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      centerX += position[0];
      centerY += position[1];
      centerZ += position[2];
    }
    const count = selected.length;
    centerX /= count;
    centerY /= count;
    centerZ /= count;
    let maxDistance = 1;
    for (const instance of selected) {
      const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      const dx = position[0] - centerX;
      const dy = position[1] - centerY;
      const dz = position[2] - centerZ;
      maxDistance = Math.max(maxDistance, Math.hypot(dx, dy, dz));
    }
    const distance = maxDistance * 3 + 2;
    adoptViewportCamera(
      {
        position: [centerX + distance * 0.6, centerY - distance * 0.6, centerZ + distance * 0.5],
        target: [centerX, centerY, centerZ],
        zoom: cameraState.zoom,
        up: cameraState.up,
        projection: cameraState.projection,
      },
      true,
    );
  }, [adoptViewportCamera, cameraState.projection, cameraState.up, cameraState.zoom, handleFrameVisibleInstances, instances, selection.ids]);

  const handleWorldMenuDispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      if (action === "zoomToSelection") {
        handleZoomToSelection();
        return;
      }
      dispatch(action, args);
    },
    [dispatch, handleZoomToSelection],
  );

  const mapWorldContextMenuSpecs = useMapContextMenuSpecs(handleWorldMenuDispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();
  const mapSuggestionContextMenuSpecs = useMapContextMenuSpecs((action, args) => {
    if (action === "acceptSuggestion") {
      setVortexPointerArm(null);
      setConnectDragSource(null);
      setConnectDragHoverPosition(null);
    }
    dispatch(action, args);
  });
  useSuggestionSubmenuSearch(contextMenu?.suggestTarget ?? null, { x: contextMenu?.x ?? 0, y: contextMenu?.y ?? 0 }, windowInstanceId ?? undefined, dispatch, () => setSuggestionFocus(null));
  useEffect(() => {
    if (!suggestionMenuOwnsThisWindow) setSuggestionFocus(null);
  }, [suggestionMenuOwnsThisWindow]);
  /** 📂️ The candidate rows one suggestion menu lists — the context menu's "suggest" submenu and the floating
   * popup share them. A submenu whose search has not echoed back yet lists the pending row. */
  const suggestionRows = useMemo(() => {
    const menu = interaction.suggestionMenu?.open && suggestionMenuOwnsThisWindow ? interaction.suggestionMenu : null;
    const listed = menu ?? { pending: true, candidates: [] };
    const rows = mapSuggestionContextMenuSpecs(
      suggestionMenuItems(listed, interaction.brushCandidateIndex ?? 0, { checkingPlacement: suggestionMenuCheckingPlacementLabel, noPlacement: suggestionMenuNoPlacementLabel }),
    );
    return suggestionRowsWithFocus(rows, listed, setSuggestionFocus);
  }, [interaction.brushCandidateIndex, interaction.suggestionMenu, mapSuggestionContextMenuSpecs, suggestionMenuCheckingPlacementLabel, suggestionMenuNoPlacementLabel, suggestionMenuOwnsThisWindow]);

  const hoveredVortexFullIdRef = useRef<string | null>(null);
  /** 🎯️ The interaction id this pane's OWN raycast last resolved under the pointer — see
   * {@link hoveredInteractionTargetV1}. Null whenever the pointer left every instance. */
  const hoveredInstanceInteractionIdRef = useRef<string | null>(null);
  const altHeldRef = useRef(false);
  useEffect(() => {
    hoveredVortexFullIdRef.current = world3dRetainLocalVortexHover(hoveredVortexFullIdRef.current, interaction.hoveredVortexFullId);
  }, [interaction.hoveredVortexFullId]);
  /** 🎣️ The armed brush points the read-only brush suggestions run at the vortex under the pointer, and at
   * nothing once it leaves every vortex or the brush is disarmed. Only a CHANGE travels, through one
   * single-flight lane (at most one `targetBrushSuggestions` outstanding plus the latest follow-up), so a hover
   * storm never queues guest turns; the command only writes the target, and the run's own trace lane paints
   * every candidate it tests. */
  const brushSuggestionsTarget = brushMode ? interaction.hoveredVortexFullId ?? null : null;
  const brushSuggestionsTargetSentRef = useRef<string | null>(null);
  const sendBrushSuggestionsTarget = useMemo(() => createCoalescingActionDispatcher<string | null>((fullId) => dispatchSettled("targetBrushSuggestions", fullId ? { fullId } : {})), [dispatchSettled]);
  useEffect(() => {
    if (brushSuggestionsTargetSentRef.current === brushSuggestionsTarget) return;
    brushSuggestionsTargetSentRef.current = brushSuggestionsTarget;
    sendBrushSuggestionsTarget(brushSuggestionsTarget);
  }, [brushSuggestionsTarget, sendBrushSuggestionsTarget]);
  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Alt" || event.key === "AltGraph") altHeldRef.current = event.type === "keydown";
    };
    window.addEventListener("keydown", onKey, true);
    window.addEventListener("keyup", onKey, true);
    window.addEventListener("blur", () => {
      altHeldRef.current = false;
    });
    return () => {
      window.removeEventListener("keydown", onKey, true);
      window.removeEventListener("keyup", onKey, true);
    };
  }, []);

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

  // 🪟️ Sibling split panes do not mount the suggestion ContextMenuController, so they need their own
  // Escape / outside-dismiss path — otherwise `suggestionMenu.open` gates them with no way to clear it.
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

  // 🏁️ Both hover dispatchers RETURN their round trip: `createCoalescingActionDispatcher` keeps at most one
  // outstanding and coalesces the rest onto the latest target, and it can only do that for a `dispatch`
  // whose promise it can see — `dispatch` discards `onAction`'s, so the awaitable twin is the only shape
  // that arms the gate (wave B33 §3: 72 hover turns enqueued by one 70-move storm, 11 settled).
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

  // 🎯️ Listeners the instances layer registers to withdraw its local hover claim when the pointer
  // leaves this pane's canvas (`onPointerLeave` on the root below).
  const localHoverReleaseListeners = useRef(new Set<() => void>());
  const subscribeLocalHoverRelease = useCallback((listener: () => void) => {
    localHoverReleaseListeners.current.add(listener);
    return () => {
      localHoverReleaseListeners.current.delete(listener);
    };
  }, []);
  const releaseLocalHover = useCallback(() => {
    for (const listener of localHoverReleaseListeners.current) listener();
  }, []);
  // 🩺️ Imperative mirror of the painted hover — no React state, so a hover never re-renders the host.
  const publishHoverPaint = useCallback((id: string | null) => {
    const root = hostRef.current;
    if (!root) return;
    if (id === null) root.removeAttribute("data-hover-paint-id");
    else root.setAttribute("data-hover-paint-id", id);
  }, []);

  const handleInstancePointerMove = useCallback(
    (id: string | null) => {
      // 🎯️ What THIS pane's own raycast last resolved, kept locally. `selection.hoveredId` is the
      // guest's echo of it: it lags a round trip and any unrelated refresh can clear it, so a
      // background click that consulted it raced the refresh and cleared a live selection instead of
      // re-picking the object under the pointer (`📓️hot-swap-board-remount-2026-09-15.md` §3).
      hoveredInstanceInteractionIdRef.current = id == null ? null : (instancesRef.current.find((entry) => entry.id === id)?.interactionId ?? id);
      dispatchInstanceHover(id);
    },
    [dispatchInstanceHover],
  );

  // 🧭️ `setHover` is the DOMAINLESS hover lane. An app that declares an interaction domain declares
  // `interactionHover` instead and never `setHover`, so dispatching it there is an action no window
  // kind owns: the shell drops it and logs `dropped action "setHover" dispatched from window kind
  // "procedural-preview"` on every component hit, while the hover the user made is simply lost
  // (measured on the generation3d edit preview, ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The
  // instance dispatcher already carries the domain, the window's declared granularity and the
  // coalescing gate, so a component hit publishes through it on the id it names.
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

  // 🎯️ A component (face/edge/vertex) hit under a declared domain selects through `interactionSelect`
  // like an instance hit does; the plugin-private `worldPick` lane is for domainless scenes only.
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

  const paintMode = selection.interactionMode === "paint";
  /** 🖌️ The paint gesture, stepped by `worldPaintStep`; the release and the cancels listen window-wide, so a release
   * outside the canvas still commits and a blur or unmount mid-drag leaves zero trace. */
  const paintGestureRef = useRef<WorldPaintGesture>(WORLD_PAINT_IDLE);
  const stepPaintGesture = useCallback(
    (event: WorldPaintEvent) => {
      const next = worldPaintStep(paintGestureRef.current, event);
      paintGestureRef.current = next.gesture;
      if (next.dispatch) dispatch("paintAt", next.dispatch);
    },
    [dispatch],
  );
  const stepPaintGestureRef = useRef(stepPaintGesture);
  stepPaintGestureRef.current = stepPaintGesture;
  const handlePaintAt = useCallback((objectId: string, u: number, v: number, origin: WorldPaintOrigin) => stepPaintGesture({ kind: "dab", origin, objectId, u, v }), [stepPaintGesture]);
  useEffect(() => {
    const onRelease = () => stepPaintGestureRef.current({ kind: "release" });
    const onCancel = () => stepPaintGestureRef.current({ kind: "cancel", reason: "captureLost" });
    const onBlur = () => stepPaintGestureRef.current({ kind: "cancel", reason: "blur" });
    window.addEventListener("pointerup", onRelease);
    window.addEventListener("pointercancel", onCancel);
    window.addEventListener("blur", onBlur);
    return () => {
      window.removeEventListener("pointerup", onRelease);
      window.removeEventListener("pointercancel", onCancel);
      window.removeEventListener("blur", onBlur);
      stepPaintGestureRef.current({ kind: "cancel", reason: "captureLost" });
    };
  }, []);

  // 🧭️ Completed user orbit/pan/zoom gesture (`WorldOrbitGated.onCamera`, fired on gesture end) — the only
  // camera-change handler wired to a real user interaction, so it's the only one that also dispatches.
  const handleCameraChange = useCallback(
    (state: WorldCameraState) => {
      adoptViewportCamera(state, false);
      noteUserMovedCamera();
      dispatchWorldCameraDebounced(state);
    },
    [adoptViewportCamera, noteUserMovedCamera, dispatchWorldCameraDebounced],
  );

  const handleCameraNavigate = useCallback(
    (active: boolean) => {
      setCameraNavigating((prev) => (prev === active ? prev : active));
      if (active) {
        setViewportOwned(true);
        noteUserMovedCamera();
      }
    },
    [noteUserMovedCamera],
  );

  // 🧭️ Programmatic auto-fit-to-content camera change (`WorldAutoFit.onFitted`, runs after a document/scene
  // loads or content changes) — deliberately split from `handleCameraChange` so this path never dispatches;
  // only a genuine user gesture should sync a camera to the plugin.
  const handleAutoFitCameraChange = useCallback(
    (state: WorldCameraState) => {
      adoptViewportCamera(state, false);
    },
    [adoptViewportCamera],
  );

  const syncProjectionWindowChrome = useCallback(
    (spec: WorldProjectionSpec) => {
      if (!windowInstanceId) return;
      setWindowTitle?.(windowInstanceId, worldProjectionSpecLabel(spec));
      setWindowIcon?.(windowInstanceId, worldProjectionSpecIconId(spec) as IconName);
    },
    [setWindowIcon, setWindowTitle, windowInstanceId],
  );

  useEffect(() => {
    const pending = peekPendingWorldProjection(windowInstanceId);
    if (pending && windowInstanceId) setWindowIcon?.(windowInstanceId, worldProjectionSpecIconId(pending) as IconName);
  }, [setWindowIcon, windowInstanceId]);

  // 🧭️ Gizmo/axis-indicator camera snap — a discrete view change the user clicked, so (like
  // `handleCameraChange`) it also dispatches, debounced identically.
  const handleGizmoCameraChange = useCallback(
    (state: WorldCameraState) => {
      adoptViewportCamera(state, true);
      retainedPendingProjectionSpecRef.current = null;
      retainViewport({ pendingProjectionSpec: null });
      noteUserMovedCamera();
      if (state.projectionSpec) syncProjectionWindowChrome(state.projectionSpec);
      dispatchWorldCameraDebounced(state);
    },
    [adoptViewportCamera, noteUserMovedCamera, syncProjectionWindowChrome, dispatchWorldCameraDebounced],
  );

  const [pendingProjectionSpec, setPendingProjectionSpec] = useState<WorldProjectionSpec | null>(null);

  // 🧭️ User-driven projection-kind switch (`WorldOrbitProjectionSwitchPane`) — view-state only, never
  // dispatched: the only existing `setProjection` consumer (`apply_world3d_projection_action` in the
  // framework plugin's Rust) takes granular `{field, value}` pairs from window-measure Select/Slider
  // controls (e.g. `perspectiveKind`/`orthographicView`/`axonometricVariant`), a fundamentally different
  // contract than this pane's whole-`WorldProjectionSpec` (mode ⊗ orientation) template selection — there is
  // no lossless mapping from one to the other, so dispatching here would silently no-op against every real
  // handler instead of doing anything.
  const handleProjectionKindChange = useCallback(
    (spec: WorldProjectionSpec) => {
      retainedPendingProjectionSpecRef.current = spec;
      setExternalPendingProjectionSpec(spec);
      retainViewport({ pendingProjectionSpec: spec });
      syncProjectionWindowChrome(spec);
    },
    [retainViewport, syncProjectionWindowChrome],
  );

  const handleProjectionContentFrame = useCallback((state: WorldParsedCameraState) => {
    setViewportCamera(state);
    if (!projectionContentFrameSeededRef.current) {
      projectionContentFrameSeededRef.current = true;
      retainViewport({ projectionContentFrameSeeded: true });
      setDetachEpoch((epoch) => epoch + 1);
    }
    retainViewport({ camera: state, viewportOwned: true });
    setViewportOwned(true);
    setProjectionFramePending(false);
  }, [retainViewport]);

  const worldProjectionSpec: WorldProjectionSpec = cameraState.projectionSpec ?? (cameraState.projection === "orthographic" ? worldProjectionDefaults("orthographic") : worldProjectionDefaults("threePoint"));
  /** 📷️ Keep fitting the live content bounds into seeded projection panes (esp. orthographic Top) until the
   * user takes ownership — otherwise tool-placed objects that expand the scene fall outside the one-shot
   * initial frustum and only remain visible in the wider perspective pane. */
  const localProjectionPending = pendingProjectionSpec !== null || externalPendingProjectionSpec !== null || pendingProjectionSpecRef.current !== null;
  const hasProjectionSeed = Boolean(pendingProjectionSpecRef.current ?? pendingProjectionSpec ?? externalPendingProjectionSpec ?? cameraState.projectionSpec);
  const lockProjectionContentFrame =
    activeUtility === "fill" || instances.some((instance) => instance.provisional) || projectionContentFrameSeededRef.current;
  const fitProjectionContent = world3dFitProjectionContent(viewportOwned, cameraNavigating, hasProjectionSeed, lockProjectionContentFrame, localProjectionPending ? "content" : cameraState.projectionFrame);
  const autoFitKey = world3dAutoFitKey(fit?.revision ?? 0, sceneCameraAttachJson, autoFitBounds);
  const projectionContentFrameMounted = world3dProjectionContentFrameMounted(fitProjectionContent, projectionFramePending, cameraNavigating);
  const autoFitUserMoved = userMovedFitRevision === (fit?.revision ?? 0);
  const worldOrbitConstraints = useMemo(() => worldProjectionOrbitConstraints(cameraState.projectionSpec), [cameraState.projectionSpec]);

  const marqueePreview = useMemo<{ readonly mergedComponentIds: readonly number[] | null; readonly mergedInstanceIds: readonly string[] | null }>(() => {
    if (!marqueeDragActive || !hostRef.current || !cameraRef.current) return { mergedComponentIds: null, mergedInstanceIds: null };
    const rect = hostRef.current.getBoundingClientRect();
    const camera = cameraRef.current;
    if (selectionMode === "mesh" || selectionMode === "object") {
      const hits = resolveMarqueeInstanceIds(instances, meshes, marqueePath, rect, camera, method, marqueeCoverage);
      return { mergedComponentIds: null, mergedInstanceIds: mergeIdSet(marqueeMergeMode, selection.ids ?? [], hits) };
    }
    const hits = resolveMarqueeComponentIds(instances, meshes, selectionMode, selection.activeObjectId ?? undefined, marqueePath, rect, camera, method, marqueeCoverage);
    return { mergedComponentIds: mergeIdSet(marqueeMergeMode, selection.componentIds ?? [], hits), mergedInstanceIds: null };
  }, [instances, marqueeCoverage, marqueeDragActive, marqueeMergeMode, marqueePath, meshes, method, selection.activeObjectId, selection.componentIds, selection.ids, selectionMode]);
  const selectionPreviewSourceId = windowInstanceId ?? node.surfaceId;
  const localSelectionPreviewActive = marqueePreview.mergedComponentIds !== null || marqueePreview.mergedInstanceIds !== null;
  const heldSelectionPreviewActive = marqueeCommitHold !== null && (marqueeCommitHold.mergedComponentIds !== null || marqueeCommitHold.mergedInstanceIds !== null);
  useEffect(() => {
    if (!marqueeCommitHold) return;
    const selectionIdSet = new Set(selection.ids ?? []);
    const selectionComponentSet = new Set(selection.componentIds ?? []);
    const instanceMatch =
      marqueeCommitHold.mergedInstanceIds != null &&
      marqueeCommitHold.mergedInstanceIds.length > 0 &&
      marqueeCommitHold.mergedInstanceIds.length === selectionIdSet.size &&
      marqueeCommitHold.mergedInstanceIds.every((id) => selectionIdSet.has(id));
    const componentMatch =
      marqueeCommitHold.mergedComponentIds != null &&
      marqueeCommitHold.mergedComponentIds.length > 0 &&
      marqueeCommitHold.mergedComponentIds.length === selectionComponentSet.size &&
      marqueeCommitHold.mergedComponentIds.every((id) => selectionComponentSet.has(id));
    if (instanceMatch || componentMatch) setMarqueeCommitHold(null);
  }, [marqueeCommitHold, selection.componentIds, selection.ids]);
  useLayoutEffect(() => {
    if (localSelectionPreviewActive) {
      setWorldSelectionPreview(node.controllerId, { sourceId: selectionPreviewSourceId, ...marqueePreview });
    } else if (heldSelectionPreviewActive && marqueeCommitHold) {
      setWorldSelectionPreview(node.controllerId, { sourceId: selectionPreviewSourceId, ...marqueeCommitHold });
    } else {
      clearWorldSelectionPreview(node.controllerId, selectionPreviewSourceId);
    }
  }, [heldSelectionPreviewActive, localSelectionPreviewActive, marqueeCommitHold, marqueePreview, node.controllerId, selectionPreviewSourceId]);
  const marqueePreviewRef = useRef(marqueePreview);
  marqueePreviewRef.current = marqueePreview;
  const marqueeDragActiveRef = useRef(marqueeDragActive);
  marqueeDragActiveRef.current = marqueeDragActive;
  const marqueeFinalizeOnceRef = useRef(false);
  useEffect(
    () => () => {
      clearWorldSelectionPreview(node.controllerId, selectionPreviewSourceId);
    },
    [node.controllerId, selectionPreviewSourceId],
  );
  const visibleSelectionPreview = localSelectionPreviewActive
    ? marqueePreview
    : heldSelectionPreviewActive && marqueeCommitHold
      ? marqueeCommitHold
      : (sharedSelectionPreview ?? marqueePreview);

  /** 🛠️ Steps the gumball gesture by one `worldGumballStep` event and enqueues the dispatch it owes on the gesture chain;
   * a stream tick settles back into the gesture, so the newest pose that arrived meanwhile goes out as the next tick. A
   * skipped delta is logged as a PERMANENT record, not a `[TRACE]` trace: a drag whose pose never moved along its handle
   * commits NOTHING — it once synthesized a fixed 0.5 translate along the handle's axis instead, a document edit the user
   * never made (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B31) — so the cause is read off the gesture. */
  const stepGumballGestureRef = useRef<(event: WorldGumballEvent) => void>(() => undefined);
  stepGumballGestureRef.current = (event: WorldGumballEvent) => {
    const step = worldGumballStep(gumballGestureRef.current, event);
    gumballGestureRef.current = step.gesture;
    if (step.skipped && (event.kind === "release" || step.skipped === "no-selection-ids")) {
      console.info("gumball pose delta skipped", { reason: step.skipped, event: event.kind, transformMode: step.gesture.transformMode ?? selection.transformMode });
    }
    const sent = step.dispatch;
    if (!sent) return;
    gumballDragChainRef.current = gumballDragChainRef.current
      .then(async () => {
        await dispatchSettled(sent.action, sent.args);
      })
      .catch(() => undefined)
      .finally(() => {
        if (sent.args.phase === "stream") stepGumballGestureRef.current({ kind: "settled" });
      });
  };

  const handleGumballDragStart = useCallback(
    (kind: GumballHandleKind, before: GumballPose) => {
      (globalThis as { __gumballDragEntered?: boolean }).__gumballDragEntered = true;
      stepGumballGestureRef.current({ kind: "start", handle: kind, pose: before, targets: selectionArgs(), live: gumballLiveDispatch, transformMode: selection.transformMode });
    },
    [gumballLiveDispatch, selection.transformMode, selectionArgs],
  );

  /** 🧯️ Host aborts of a live gesture whose ticks opened the app's tool transaction: window blur and unmount. */
  useEffect(() => {
    const onBlur = () => stepGumballGestureRef.current({ kind: "cancel", reason: "blur" });
    window.addEventListener("blur", onBlur);
    return () => {
      window.removeEventListener("blur", onBlur);
      stepGumballGestureRef.current({ kind: "cancel", reason: "captureLost" });
    };
  }, []);

  const handleGumballDrag = useCallback((kind: GumballHandleKind, pose: GumballPose) => stepGumballGestureRef.current({ kind: "drag", handle: kind, pose }), []);

  const handleGumballDragEnd = useCallback(
    (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => stepGumballGestureRef.current({ kind: "release", handle: kind, before, after, targets: selectionArgs() }),
    [selectionArgs],
  );

  const handleFaceDragStart = useCallback((args: { objectId: string; faceId: number; normal: readonly [number, number, number]; point: readonly [number, number, number]; faceExtent?: readonly [number, number] }) => {
    setFaceDragSession({ objectId: args.objectId, faceId: args.faceId, normal: args.normal, startPoint: args.point, faceExtent: args.faceExtent });
  }, []);

  const toLocalPoint = useCallback((event: React.PointerEvent<HTMLDivElement>): SelectionMarqueePoint => {
    const rect = hostRef.current?.getBoundingClientRect();
    if (!rect) return { x: event.clientX, y: event.clientY };
    return { x: event.clientX - rect.left, y: event.clientY - rect.top };
  }, []);

  /** 🚚️ Takes a Relocate-utility press: grabs the object under it (gated by the selection, see
   * {@link world3dRelocateDragTargetV1}), pins the ground point the drag starts from and paints the shared
   * world ghost at the object's own origin. Answers whether it took the press, so the marquee never opens
   * underneath an in-progress relocate. A LOCKED object is grabbed on purpose — the guest answers the
   * commit with its `selection_locked` notice, which is a visible refusal instead of a dead gesture.
   * A press on empty ground with a live selection is the gesture's BASE POINT, not a miss — see
   * {@link world3dRelocateDragTargetV1} for why the base point never had to be on the object. */
  const beginRelocateDrag = useCallback(
    (event: React.PointerEvent<HTMLDivElement>): boolean => {
      const host = hostRef.current;
      const camera = cameraRef.current;
      if (!host || !camera) return false;
      const rect = host.getBoundingClientRect();
      const from = raycastGroundPoint(event.clientX, event.clientY, rect, camera);
      if (!from) return false;
      const pressed = resolveClickInstanceId(instancesRef.current, meshesRef.current, toLocalPoint(event), rect, camera);
      const objectId = world3dRelocateDragTargetV1(pressed, selection.ids ?? []);
      const instance = objectId ? instancesRef.current.find((entry) => entry.id === objectId) : undefined;
      if (!objectId || !instance) return false;
      const origin = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      const objectKind = instance.objectKind ?? "";
      const meshUrl = meshesRef.current.find((mesh) => mesh.id === (instance.meshId ?? instance.id))?.url;
      relocateSessionRef.current = { objectId, origin, from, objectKind, meshUrl };
      event.currentTarget.setPointerCapture?.(event.pointerId);
      setWorldCatalogueDropPreview(node.controllerId, { objectKind, meshUrl, origin });
      return true;
    },
    [node.controllerId, selection.ids, toLocalPoint],
  );

  /** 🚚️ Moves the live relocate ghost to the grabbed object's would-be origin — grid-snapped exactly like a
   * catalogue drop, zero dispatches until release. */
  const updateRelocateDrag = useCallback(
    (event: React.PointerEvent<HTMLDivElement>): boolean => {
      const session = relocateSessionRef.current;
      const host = hostRef.current;
      const camera = cameraRef.current;
      if (!session || !host || !camera) return false;
      const to = raycastGroundPoint(event.clientX, event.clientY, host.getBoundingClientRect(), camera);
      if (!to) return true;
      const args = world3dRelocateDispatchArgsV1(session.objectId, session.origin, session.from, to, { gridSnapEnabled, gridFactor });
      setWorldCatalogueDropPreview(node.controllerId, { objectKind: session.objectKind, meshUrl: session.meshUrl, origin: args?.position ?? session.origin });
      return true;
    },
    [gridFactor, gridSnapEnabled, node.controllerId],
  );

  /** 🚚️ Closes the relocate drag. A pointer-UP event commits ONE absolute `worldRelocate`; `null` (Escape) and
   * a `pointercancel` both drop the ghost and dispatch nothing — the host binds `onPointerCancel` to the same
   * pointer-up handler, so the event kind is the only thing separating a commit from an abort. */
  const endRelocateDrag = useCallback(
    (event: React.PointerEvent<HTMLDivElement> | null): boolean => {
      const session = relocateSessionRef.current;
      if (!session) return false;
      relocateSessionRef.current = null;
      clearWorldCatalogueDropPreview(node.controllerId);
      const host = hostRef.current;
      const camera = cameraRef.current;
      if (!event || event.type === "pointercancel" || !host || !camera) return true;
      const to = raycastGroundPoint(event.clientX, event.clientY, host.getBoundingClientRect(), camera);
      const args = to ? world3dRelocateDispatchArgsV1(session.objectId, session.origin, session.from, to, { gridSnapEnabled, gridFactor }) : null;
      if (args) void Promise.resolve(dispatch("worldRelocate", args));
      return true;
    },
    [dispatch, gridFactor, gridSnapEnabled, node.controllerId],
  );

  useEffect(() => {
    if (!relocateMode) return undefined;
    const onKeyDown = (event: KeyboardEvent): void => {
      if (event.key !== "Escape") return;
      if (endRelocateDrag(null)) event.stopPropagation();
    };
    window.addEventListener("keydown", onKeyDown, true);
    return () => window.removeEventListener("keydown", onKeyDown, true);
  }, [endRelocateDrag, relocateMode]);

  /** 🤏️ Abandons whatever single-pointer gesture is in flight and releases its capture, so the
   * orbit rig owns the surface alone for the rest of a two-finger pinch/pan. */
  const yieldToPinch = useCallback(
    (event: React.PointerEvent<HTMLDivElement>) => {
      for (const tracked of gestureRecognizer.pointers) {
        if (event.currentTarget.hasPointerCapture?.(tracked.pointerId)) event.currentTarget.releasePointerCapture(tracked.pointerId);
      }
      endRelocateDrag(null);
      setMarqueePath([]);
      marqueeStartRef.current = null;
      marqueeFinalizeOnceRef.current = true;
    },
    [endRelocateDrag, gestureRecognizer],
  );

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

  if (!scene) return <div className="semio-world-3d-empty">{emptySceneLabel}</div>;

  return (
    <div
      ref={hostRef}
      className="semio-world-3d-host relative h-full min-h-0 w-full touch-none overflow-hidden"
      data-surface-id={node.surfaceId}
      data-window-instance-id={windowInstanceId || undefined}
      data-selection-json={JSON.stringify(worldSurfaceSelectionDomV1(selection, interaction))}
      data-guest-selection-json={JSON.stringify(worldSurfaceGuestSelectionDomV1(scene.selectionJson))}
      data-orbit-view-gizmo=""
      data-puzzle3d-fixture-drag-active={catalogueDropPreview ? "" : undefined}
      data-meshes-json={scene.meshesJson ?? undefined}
      data-world-geometry-revision={geometryRevisionRef.current.revision}
      data-instances-json={scene.instancesJson ?? undefined}
      data-target-volumes-json={scene.targetVolumesJson ?? undefined}
      data-engagement-preview-json={scene.engagementPreviewJson ?? undefined}
      data-camera-json={world3dCameraDomJson(sceneCamera)}
      data-viewport-camera-json={world3dCameraDomJson(cameraState)}
      data-vortices-json={scene.vorticesJson ?? undefined}
      data-suggestion-menu-json={interaction.suggestionMenu ? JSON.stringify(interaction.suggestionMenu) : ""}
      data-suggestion-focus={suggestionFocus === null ? undefined : String(suggestionFocus)}
      data-interaction-json={JSON.stringify(interaction)}
      data-status-json={scene.statusJson ?? undefined}
      data-sun-json={world3dSunDomJson(environment)}
      {...world3dModellingDataAttributes({ pickFilter: modellingOptions?.pickFilter, section: modellingOptions?.section, scalarField, scalarStatus: heatmap.status, annotationCount: annotationLayer?.items.length })}
      {...toolRunTraceDataAttributes(toolRunTrace.store)}
      onContextMenu={(event) => {
        const alt = world3dSuggestionsAltHeld(event.altKey, altHeldRef.current);
        const brushArmed = Boolean(brushMode && hoveredVortexFullIdRef.current);
        if (world3dSuggestionsGestureConsumesContextMenu(alt) || brushArmed) {
          event.preventDefault();
          event.stopPropagation();
          if (performance.now() - suggestionsRightDownDispatchedAtRef.current < 80) return;
          if (world3dSuggestionsGestureArmed(alt, hoveredVortexFullIdRef.current) || brushArmed) {
            dispatch("openVortexSuggestions", { fullId: hoveredVortexFullIdRef.current, x: event.clientX, y: event.clientY, windowId: windowInstanceId ?? undefined });
          }
          return;
        }
        if (!requestContextMenu) return;
        const target = resolveWorldContextMenuTarget(interaction, selection);
        event.preventDefault();
        event.stopPropagation();
        const epoch = contextMenuOpenEpochRef.current + 1;
        contextMenuOpenEpochRef.current = epoch;
        void (async () => {
          const surface = world3dContextMenuSurfaceV1(target, selection);
          const suggest = { target: null as string | null };
          const menu = await openSurfaceContextMenu(
            requestContextMenu,
            {
              menu: { id: "world3d", args: null },
              surface: { surfaceId: node.surfaceId, kind: "world3d", hits: [...surface.hits], selection: surface.selection.map((group) => ({ domain: group.domain, ids: [...group.ids] })) },
              windowInstanceId: windowInstanceId ?? undefined,
              point: { x: event.clientX, y: event.clientY },
            },
            (specs) => {
              suggest.target = suggestionSubmenuTarget(specs);
              return mapWorldContextMenuSpecs(specs);
            },
            shellContextMenuFallback,
          );
          if (contextMenuOpenEpochRef.current !== epoch) return;
          setContextMenu({ x: event.clientX, y: event.clientY, ...menu, suggestTarget: suggest.target });
        })();
      }}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerCancel={handlePointerUp}
      onPointerLeave={releaseLocalHover}
      onDragEnter={onCatalogueDragEnter}
      onDragLeave={onCatalogueDragLeave}
      onDragOver={onCatalogueDragOver}
      onDrop={onCatalogueDrop}
    >
      <WorldCanvas
        className="h-full w-full"
        cameraUp={(cameraState.up as [number, number, number] | undefined) ?? [0, 0, 1]}
        cameraFov={cameraState.fov}
        background={environment && !isTransparentWorldBackground(environment.background) ? environment.background : undefined}
        gl={environment && isTransparentWorldBackground(environment.background) ? { antialias: true, alpha: true, stencil: true } : undefined}
        shadows={environment?.shadow?.enabled === true ? true : undefined}
        onPointerMissed={handleEmptyClick}
        overlay={
          <>
            {frame ? <IconShotFrame width={frame.width} height={frame.height} shape={frame.shape === "ellipse" ? "ellipse" : "rectangle"} badge={frame.badge !== false} background={frame.background} /> : null}
            {annotationLayer ? <World3dAnnotationOverlay layer={annotationLayer} store={annotationStore} locale={shellScope?.i18n.language} /> : null}
            {scalarField && heatmap.status === "applied" ? <World3dScalarLegendView field={scalarField} locale={shellScope?.i18n.language} className={cn("pointer-events-none absolute bottom-2 left-2 z-40 rounded px-single py-half text-xs shadow-sm", glassClass)} /> : null}
            {scalarField && heatmap.status === "mismatch" ? (
              <div role="status" data-slot="world-scalar-field-mismatch" className={cn("pointer-events-none absolute bottom-2 left-2 z-40 rounded px-single py-half text-xs shadow-sm", glassClass)}>
                {world3dModellingStrings(shellScope?.i18n.language).mismatch}
              </div>
            ) : null}
            <WorldOrbitProjectionSwitchPane spec={worldProjectionSpec} onSpecChange={handleProjectionKindChange} windowElementSegment={windowInstanceId ?? node.surfaceId} />
            {/* 🚧️ Scene overlays are window CONTENT, so they start below the window's own floating chrome
                control row ({@link windowChromeClearedTopOffset}) and stay out of the top-left corner the
                folded engagement's quick-action rail owns — a control painted into that band covers the
                pane toggles and, at `z-40`, swallows their pointer events outright. */}
            {/* 🛟️ The window's own chrome is only half the corner: an anchored chrome PANEL paints over
                this rail from the app root's stacking context (`z-panel` vs this `z-40` inside the
                window's `z-window`), so `Frame visible` and the compute `Cancel` were unreachable at
                every press. The rail reserves the panel's published box ({@link useChromePanelSafeArea})
                and steps clear on whichever axis costs less — the framework rule, not a restack. */}
            <div
              ref={overlayRailRef}
              data-slot="world-view-overlay-rail"
              data-safe-area-block={overlayRailSafeArea.blockPx || undefined}
              data-safe-area-inline={overlayRailSafeArea.inlinePx || undefined}
              className="pointer-events-none absolute z-40 flex flex-col items-end gap-single"
              style={chromePanelSafeAreaStyle("top-right", overlayRailSafeArea, { block: windowChromeClearedTopOffset })}
            >
              <WorldSurfaceStatusPane visuals={meshVisuals} locale={shellScope?.i18n.language} glassClass={glassClass} />
              <WorldComputeStatusPane status={computeStatus} glassClass={glassClass} locale={shellScope?.i18n.language} onCancel={() => dispatch(computeStatus.cancelAction, computeStatus.cancelArgs)} />
            </div>
            <CanvasPresenceOverlayV1
              runtimeKey="local"
              windowId={presenceWindowId}
              space="world"
              myActor={localPresenceActor ?? ""}
              locale={shellScope?.i18n.language}
              localOrbit={{
                position: cameraState.position as [number, number, number],
                target: cameraState.target as [number, number, number],
                up: (cameraState.up as [number, number, number] | undefined) ?? [0, 0, 1],
                fov: cameraState.fov ?? 45,
              }}
              localSizePx={hostRef.current ? [hostRef.current.clientWidth, hostRef.current.clientHeight] : [1, 1]}
              objectWorldPositions={presenceObjectWorldPositions}
              scenePath={`world/${presenceWindowId}`}
              domain={interactionDomainId}
              selectedIds={selection.ids}
            />
          </>
        }
      >
        <WorldOrbitViewSnapGateProvider>
          <WorldProjectionRig spec={worldProjectionSpec} state={cameraState} seedKey={cameraSeedKey} pendingSpec={pendingProjectionSpec} />
          {projectionContentFrameMounted ? (
            <WorldProjectionContentFrame
              enabled={fitProjectionContent || projectionFramePending}
              spec={pendingProjectionSpecRef.current ?? worldProjectionSpec}
              bounds={contentBounds}
              fov={cameraState.fov}
              onFramed={handleProjectionContentFrame}
            />
          ) : null}
          <WorldOrbitGated
            controlsGate={marqueeDown || gumballDragActive || connectDragSource !== null || faceDragSession !== null}
            onCamera={handleCameraChange}
            onCameraNavigate={handleCameraNavigate}
            zoom={cameraState.zoom}
            projection={cameraState.explicitProjection ? cameraState.projection : undefined}
            constraints={worldOrbitConstraints}
            onRightPointerDown={handleWorldOrbitRightPointerDown}
            onNavigationGestures={(gestures) => dispatch(NOTE_WORLD_NAVIGATION_ACTION_ID, { windowId: windowInstanceId ?? node.surfaceId, gestures })}
          />
          <WorldOrbitViewControls
            projectionSpec={worldProjectionSpec}
            externalPendingSpec={externalPendingProjectionSpec}
            onExternalPendingSpecClear={() => setExternalPendingProjectionSpec(null)}
            onCameraChange={handleGizmoCameraChange}
            onPendingSpecChange={setPendingProjectionSpec}
          />
          <WorldLodBridge
            lodRef={lodRef}
            distanceReference={100}
            gridFactor={lod.gridFactor ?? DEFAULT_LOD_GRID_FACTOR}
            gridSnapEnabled={lod.gridSnapEnabled ?? false}
            showLodGrid={lod.showLodGrid ?? true}
            automaticLod={lod.automaticLod ?? true}
            depthVariableLod={lod.depthVariableLod ?? false}
            manualLod={lod.manualLod ?? DEFAULT_MANUAL_LOD}
            gridDatum={[0, 0, 0]}
          >
            <WorldAcceptedFrameDiagnostics
              surfaceId={node.surfaceId}
              windowInstanceId={windowInstanceId}
              projection={worldProjectionSpec}
              cameraTarget={cameraState.target as [number, number, number]}
              contentBounds={contentBounds}
              references={references}
              lod={lod}
            />
            <ambientLight color={environment?.ambient?.color ?? "#ffffff"} intensity={environment?.ambient?.intensity ?? 1.15} />
            {environment?.sun?.enabled === true ? (
              <directionalLight
                color={environment.sun?.color ?? "#ffffff"}
                intensity={environment.sun?.intensity ?? 0.85}
                position={sunPositionFromAzimuthElevation(environment.sun?.azimuth ?? 45, environment.sun?.elevation ?? 35)}
                castShadow={environment.shadow?.enabled === true}
              />
            ) : (
              <>
                <hemisphereLight color="#ffffff" groundColor="#9aa0ab" intensity={1.35} position={[0, 0, 1]} />
                <directionalLight position={[12, 18, 10]} intensity={2.4} />
                <directionalLight position={[-14, -10, 6]} intensity={1.2} />
                <directionalLight position={[0, 0, -16]} intensity={0.75} />
              </>
            )}
            {fit?.enabled ? (
              <WorldAutoFit
                groupRef={instancesGroupRef}
                fitKey={autoFitKey}
                padding={fit.padding ?? WORLD3D_FRAME_BOUNDS_MARGIN}
                camera={cameraState}
                bounds={autoFitBounds}
                userMoved={autoFitUserMoved}
                onFitted={handleAutoFitCameraChange}
              />
            ) : null}
            <CameraRefBridge cameraRef={cameraRef} />
            <CatalogueDropPreviewInvalidate preview={catalogueDropPreview} />
            <RaycasterPickTuning />
            <WorldVortexHitStamp vortices={previewVortices} hostRef={hostRef} />
            <WorldGumballHitStamp target={selection.gumballTarget} active={Boolean(selection.gumballActive) && isWorldTransformGumballMode(selection.transformMode)} hostRef={hostRef} />
            {windowInstanceId ? (
              <IntroductionWorldResolverBridge windowInstanceId={windowInstanceId} vortices={vortices} instances={instances} attractions={attractions} />
            ) : null}
            {brushMeshUrls.map((url) => (
              <Suspense key={url} fallback={null}>
                <BrushMeshRegistrar url={url} revision={brushMeshRevisions.generation + (brushMeshRevisions.urls[url] ?? 0)} onRegister={handleRegisterBrushMesh} />
              </Suspense>
            ))}
            <WorldTerrainLayer terrainJson={scene?.terrainJson} cameraPosition={cameraState.position} cameraTarget={cameraState.target} />
            <WorldPointCloudLayer pointsJson={scene?.pointsJson} />
            <group ref={instancesGroupRef}>
              <WorldInstancesLayer
                meshVisuals={meshVisuals}
                instances={instances}
                meshes={meshes}
                selection={selection}
                persistentSelectionMode={persistentSelectionMode}
                palette={meshStylePalette}
                projectionSpec={worldProjectionSpec}
                onInstancePointerDown={handleInstancePointerDown}
                onInstancePointerMove={handleInstancePointerMove}
                onLocalHoverRelease={subscribeLocalHoverRelease}
                onHoverPaint={publishHoverPaint}
                onWorldPick={handleWorldPick}
                onComponentHover={handleComponentHover}
                onPaintAt={paintMode ? handlePaintAt : undefined}
                gumballDragActive={gumballDragActive}
                controllerId={node.controllerId}
                gumballPreviewSourceId={gumballTransformPreviewSourceId}
                sharedGumballPreview={sharedGumballTransformPreview}
                onGumballDraggingChanged={setGumballDragActive}
                gumballLiveDispatch={gumballLiveDispatch}
                onGumballDragStart={handleGumballDragStart}
                onGumballDrag={handleGumballDrag}
                onGumballDragEnd={handleGumballDragEnd}
                onFaceDragStart={handleFaceDragStart}
                mergedComponentIds={visibleSelectionPreview.mergedComponentIds}
                mergedInstanceIds={visibleSelectionPreview.mergedInstanceIds}
                blockPick={worldInstancePickBlocked(activeUtility)}
                environment={environment}
                subElementPaint={subElementPaint}
              />
            </group>
            {annotationLayer ? <World3dAnnotationProjector layer={annotationLayer} store={annotationStore} /> : null}
            {modellingOptions?.section ? <World3dSectionClip section={modellingOptions.section} groupRef={instancesGroupRef} capHex={sectionCapHex} extent={sectionExtent} /> : null}
            <WorldVortexMarkers
              vortices={previewVortices}
              palette={meshStylePalette}
              brushMode={brushMode}
              selectionMode={selectionMode}
              connectSourceFullId={connectDragSource?.fullId}
              onHover={handleVortexHover}
              onVortexSelect={handleVortexSelect}
              onBrushPlace={handleBrushPlace}
              onVortexPointerArm={handleVortexPointerArm}
              onVortexPointerMove={handleVortexPointerMove}
              onVortexPointerUp={handleVortexPointerUp}
              onConnectDragHover={handleConnectDragHover}
              onConnectDragDrop={handleConnectDragDrop}
            />
            {connectDragSource && connectDragHoverPosition ? <WorldConnectRubberBand from={connectDragSource.position} to={connectDragHoverPosition} /> : null}
            <WorldAttractionLines attractions={previewAttractions} />
            {catalogueDropPreview ? <CatalogueDropGhost preview={catalogueDropPreview} meshes={meshes} palette={meshStylePalette} /> : null}
            <WorldToolRunTrace lane={scene.toolRunTrace} meshes={meshes} trace={toolRunTrace} focus={suggestionFocus} />
            {engagementPreview.length > 0 ? <EngagementPreviewLayer items={engagementPreview} color={colors.hover} /> : null}
            <WorldVolumeLayer
              volumes={targetVolumes
                .filter((volume) => !volume.hidden)
                .map((volume) => ({
                  id: volume.id,
                  origin: volume.origin as [number, number, number],
                  orientation: volume.orientation as [number, number, number, number] | undefined,
                  scale: volume.scale,
                  color: volume.color,
                  hidden: volume.hidden,
                  locked: volume.locked,
                }))}
              selectedIds={targetVolumeSelectedIds}
              interactive={volumeLayersInteractive}
              gumballConfig={volumeGumballConfig}
              relocateActive={activeUtility === "transform"}
              translationSnap={gridSnapEnabled ? gridFactor : undefined}
              onSelect={handleTargetVolumeSelect}
              onRelocate={handleTargetVolumeRelocate}
            />
            {volumeBrushMode && voxelHoverOrigin ? <WorldVoxelPreviewBox origin={voxelHoverOrigin} dims={interaction.voxelDims ?? [1, 1, 1]} gridFactor={interaction.gridFactor ?? DEFAULT_LOD_GRID_FACTOR} /> : null}
            <WorldReferenceLayer
              references={references
                .filter((reference) => !reference.hidden)
                .map((reference) => ({
                  id: reference.id,
                  source: { url: reference.url, mediaKind: "image" as const },
                  origin: reference.origin as [number, number, number],
                  widthWorld: reference.widthWorld,
                  locked: reference.locked,
                  opacity: reference.opacity,
                }))}
              selectedIds={referenceSelectedIds}
              hoveredId={referenceHoveredId}
              onSelect={(id) => handleReferenceSelect(id)}
              onHover={handleReferenceHover}
            />
          </WorldLodBridge>
        </WorldOrbitViewSnapGateProvider>
      </WorldCanvas>
      {marqueeDragActive && marqueeStart && marqueeEnd && world3dMarqueeOverlayShape(method) === "polygon" ? (
        <SelectionMarquee className="z-50" coverage={marqueeCoverage} shape="polygon" points={marqueePath} />
      ) : marqueeDragActive && marqueeStart && marqueeEnd && world3dMarqueeOverlayShape(method) === "rect" ? (
          <SelectionMarquee
            className="z-50"
            coverage={marqueeCoverage}
            shape="rect"
            rect={{
              x: Math.min(marqueeStart.x, marqueeEnd.x),
              y: Math.min(marqueeStart.y, marqueeEnd.y),
              width: Math.abs(marqueeEnd.x - marqueeStart.x),
              height: Math.abs(marqueeEnd.y - marqueeStart.y),
            }}
          />
      ) : null}
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null && (contextMenu.items?.length ?? 0) > 0 && !suggestionPopupOwnsThisWindow}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.suggestTarget ? withSuggestionSubmenu(contextMenu.items, suggestionRows) : (contextMenu?.items ?? [])}
        onOpenChange={(open) => {
          if (!open) dismissContextMenu();
        }}
      />
      {suggestionPopupOwnsThisWindow ? (
        <ContextMenuController
          title={suggestionMenuTitleLabel}
          open
          closeOnSelect={false}
          position={{ x: interaction.suggestionMenu!.x, y: interaction.suggestionMenu!.y }}
          items={suggestionRows}
          onOpenChange={(open) => {
            if (!open) handleSuggestionClose();
          }}
        />
      ) : null}
    </div>
  );
}
//#endregion World3dHost
//#endregion 🔖️World3dHost
