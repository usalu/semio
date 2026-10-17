/** 🌍️ World3dHost scene lanes: the `node.world3d` scene and interaction domain, the shell selection mode, the window context, the parsed camera, instance, reference, annotation, scalar field and modelling lanes, the mesh residency and the interaction, lod, environment and frame lanes. */
import { useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import { registerTutorialCameraDriver, useLabel, useShellScopeOptional, type TutorialCameraDriver } from "@semio-tech/ui-react";
import { createWorld3dAnnotationStore, resolveWorld3dHighlightPaint, useStableWorld3dLane, useWorld3dScalarFieldMeshes, resolveWorld3dToneHex, world3dSelectionWithPickFilter, world3dSubElementPaint } from "../📏️modelling/🟦️.tsx";
import { parseWorld3dAnnotationLayer, parseWorld3dModellingOptions, parseWorld3dScalarField, type ComponentSceneHostProps, type MergeMode } from "@semio-tech/framework";
import { worldSceneContentBounds, type WorldCameraState, type WorldProjectionSpec } from "@semio-tech/infinite-world-r3f";
import { SetWindowIconContext, SetWindowTitleContext } from "../../🏛️ShellHost/🎚️shell-model/🟦️.tsx";
import { autofitCameraFromInstances, parseCameraState, parseEnvironment, parseFit, parseFrame, seedPendingWorldProjectionCamera, shouldReattachWorldViewportCamera, world3dFramingInstances, world3dViewportCameraSeedKey, type WorldAttractionRecord, type WorldInstanceRecord, type WorldMeshRecord, type WorldParsedCameraState, type WorldReferenceRecord, type WorldTargetVolumeRecord, type WorldVortexRecord } from "../🔭️scene-schema/🟦️.ts";
import { semanticColorsFromPalette, useMeshStylePalette } from "../🎨️mesh-render/🖌️mesh-style-palette/🟦️.ts";
import { advanceWorldMeshResidency, type WorldMeshResidencyV1 } from "../🎨️mesh-render/🧱️mesh-parsing-and-residency/🟦️.ts";
import { WORLD_VERTEX_MARK_PX } from "../🎨️mesh-render/🪟️glb-bounds-and-styling/🟦️.ts";
import { advanceWorldInstanceResidency, leftoverWorldWindowOverlayV1, mergeWorldInteractionWithLeftoverV1, mergeWorldSelectionWithLeftoverV1, parseEngagementPreview, parseInteraction, parseLod, parseSelection, subscribeLeftoverWorldSelectionV1, WORLD3D_DEFAULT_INTERACTION_GRANULARITY, type WorldInstanceResidencyV1 } from "../🔗️instance-selection/🟦️.ts";
import { useWorldMeshVisuals } from "../🧱️instance-layers/🟦️.tsx";
import { peekPendingWorldProjection, type World3dRetainedViewportV1, World3dWindowViewStoreContext } from "../🪟️window-view-store/🟦️.tsx";
import { parseJsonArray, WindowInstanceIdContext } from "../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";

// #region World3dHost
/** 🌍️ Reads every lane of the assembled `node.world3d` scene into memoised, identity-stable values. */
export function useWorldSceneLanes({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
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
  return { annotationLayer, annotationStore, attractions, cameraNavigating, cameraSeedKey, cameraState, colors, contentBounds, emptySceneLabel, engagementPreview, environment, externalPendingProjectionSpec, fit, frame, geometryRevisionRef, heatmap, instances, interaction, interactionDomainId, interactionGranularity, lastDispatchedWorldCameraRef, liveFitRevisionRef, lod, meshStylePalette, meshVisuals, meshes, modellingOptions, node, noteUserMovedCamera, onAction, pendingProjectionSpecRef, persistentSelectionMode, projectionContentFrameSeededRef, projectionFramePending, references, requestContextMenu, retainViewport, retainedPendingProjectionSpecRef, scalarField, scene, sceneCamera, sceneCameraAttachJson, sectionCapHex, selection, setCameraNavigating, setDetachEpoch, setExternalPendingProjectionSpec, setProjectionFramePending, setViewportCamera, setViewportOwned, setWindowIcon, setWindowTitle, shellScope, subElementPaint, targetVolumes, userMovedFitRevision, viewportOwned, vortices, windowInstanceId };
}

export type WorldCore1 = ReturnType<typeof useWorldSceneLanes>;
// #endregion World3dHost
