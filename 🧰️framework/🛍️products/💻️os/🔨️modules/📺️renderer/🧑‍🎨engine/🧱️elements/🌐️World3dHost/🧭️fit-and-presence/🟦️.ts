/** 🧭️ World3dHost fit and presence: the auto-fit bounds and revision, the compute status and tool-run trace, the presence publishing, the drag, marquee and gumball state, the shared previews, the context menu state, the action dispatchers, the camera debounce and the reference and target volume handlers. */
import { useCallback, useEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import type { ThreeCamera } from "@semio-tech/ui-react";
import { marqueeCoverageFromGesture, useChromePanelSafeArea, useLabel, uiSpacingPx, type SelectionMarqueeCoverage, type SelectionMarqueePoint, Group } from "@semio-tech/ui-react";
import { useToolRunTraceCursorEcho, useToolRunTraceStore } from "../⏯️tool-run-trace/🟦️.tsx";
import { GestureRecognizer, world3dComputeStatusV1 } from "@semio-tech/framework";
import { DEFAULT_LOD_GRID_FACTOR, DEFAULT_MANUAL_LOD, type WorldCameraState, type WorldVolumeRelocatePayload } from "@semio-tech/infinite-world-r3f";
import { CAMERA_SYNC_DEBOUNCE_MS } from "../../📐️Canvas2dHost/🟦️.tsx";
import { createCoalescingActionDispatcher, declareSurfaceCancelAction } from "../../🛠️ShellHelpers/🛑️extension-io/🟦️.ts";
import { suggestionMenuOwnsWindow, suggestionPopupOwnsWindow } from "../../🎣️suggestion-submenu/🟦️.ts";
import { useLocalPresenceActorIdV1 , usePresenceRuntimeKeyV1} from "../../👕️canvas-presence/🟦️.tsx";
import { PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS, publishLocalPresenceWindowViewV1, clearLocalPresenceWindowViewV1, publishLocalActiveToolV1 } from "../../👕️canvas-presence/🟦️.ts";
import { mergeWorldViewportCamera, worldCameraSetCameraDispatchArgs, type WorldVortexRecord } from "../🔭️scene-schema/🟦️.ts";
import { WORLD3D_DEFAULT_MARKER_GRANULARITY, world3dMarkerInteractionTarget } from "../🔗️instance-selection/🟦️.ts";
import { getWorldGumballTransformPreview, getWorldGumballTransformPreviewServerSnapshot, gumballConfigForTransformMode, subscribeWorldGumballTransformPreview, WORLD_GUMBALL_IDLE, worldAttractionsWithGumballPreview, type WorldGumballGesture, worldGumballOwnerPoses, worldVorticesWithGumballPreview } from "../🧮️gumball-math/🟦️.ts";
import { MARQUEE_DRAG_THRESHOLD_PX, paneSuffixFromSurfaceId, resolveWorldMergeMode } from "../🎯️marquee-and-bridges/🟦️.tsx";
import { getWorldCatalogueDropPreview, getWorldCatalogueDropPreviewServerSnapshot, getWorldSelectionPreview, getWorldSelectionPreviewServerSnapshot, subscribeWorldCatalogueDropPreview, subscribeWorldSelectionPreview, type World3dRelocateSession } from "../🗂️catalogue-drop/🟦️.tsx";
import { type SurfaceContextMenuResult } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { world3dHoverActionArgs, world3dSelectionActionArgs } from "../🖱️interaction-domain-args/🟦️.ts";
import type { WorldCore1 } from "../🌍️scene-lanes/🟦️.ts";

// #region World3dHost
/** 🧭️ Holds the fit, presence, gesture and menu state and the action dispatchers of the world window. */
export function useWorldFitAndPresence(core: WorldCore1) {
  const { attractions, cameraState, contentBounds, fit, instances, interaction, interactionDomainId, lastDispatchedWorldCameraRef, liveFitRevisionRef, lod, meshes, node, onAction, persistentSelectionMode, references, retainViewport, scene, sceneCamera, selection, setDetachEpoch, setViewportCamera, setViewportOwned, targetVolumes, vortices, windowInstanceId } = core;
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
  const presencePointerRef = useRef<{ point: readonly [number, number, number]; ray: readonly [number, number, number] } | null>(null);
  const presenceRuntimeKey = usePresenceRuntimeKeyV1();
  const presenceWindowId = windowInstanceId || node.surfaceId || "world";
  const localPresenceActor = useLocalPresenceActorIdV1(presenceRuntimeKey);
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

  useEffect(() => presenceRuntimeKey ? () => clearLocalPresenceWindowViewV1(presenceWindowId, presenceRuntimeKey) : undefined, [presenceWindowId, presenceRuntimeKey]);
  const publishWorldPresenceView = useCallback((pointer?: readonly [number, number, number] | null, rayOrigin?: readonly [number, number, number]) => {
    if (pointer !== undefined) presencePointerRef.current = pointer === null ? null : { point: pointer, ray: rayOrigin ?? cameraState.position as [number, number, number] };
    const now = Date.now();
    if (pointer !== null && now - presencePublishAtRef.current < PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS) return;
    presencePublishAtRef.current = now;
    const rect = hostRef.current?.getBoundingClientRect();
    const size: [number, number] = [Math.max(1, rect?.width ?? 1), Math.max(1, rect?.height ?? 1)];
    const cursor = presencePointerRef.current;
    if (!presenceRuntimeKey) return;
    publishLocalPresenceWindowViewV1(presenceRuntimeKey, presenceWindowId, {
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
      ...(cursor ? { pointer: cursor.point, rayOrigin: cursor.ray } : {}),
    });
    publishLocalActiveToolV1(presenceRuntimeKey, activeUtility);
  }, [presenceWindowId, presenceRuntimeKey, cameraState, activeUtility]);

  useEffect(() => {
    const timer = setTimeout(() => publishWorldPresenceView(), PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS);
    return () => clearTimeout(timer);
  }, [publishWorldPresenceView]);

  useEffect(() => {
    if (presenceRuntimeKey) publishLocalActiveToolV1(presenceRuntimeKey, activeUtility);
  }, [activeUtility, presenceRuntimeKey]);

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
  return { activeUtility, adoptViewportCamera, autoFitBounds, brushMode, cameraRef, catalogueDragDepthRef, catalogueDragEncodedRef, catalogueDropPreview, computeStatus, connectDragHoverPosition, connectDragSource, connectDropConsumedRef, contextMenu, contextMenuOpenEpochRef, contextMenuTitleLabel, dismissContextMenu, dispatch, dispatchSettled, dispatchWorldCameraDebounced, engagementPointerMoveInFlightRef, engagementPointerMoveLastPointRef, faceDragSession, gestureRecognizer, gridFactor, gridSnapEnabled, gumballDragActive, gumballDragChainRef, gumballGestureRef, gumballLiveDispatch, gumballTransformPreviewSourceId, handleReferenceHover, handleReferenceSelect, handleTargetVolumeRelocate, handleTargetVolumeSelect, hostRef, instancesGroupRef, instancesRef, localPresenceActor, lodRef, marqueeCommitHold, marqueeCoverage, marqueeDown, marqueeDragActive, marqueeEnd, marqueeMergeMode, marqueePath, marqueeStart, marqueeStartRef, meshesRef, method, overlayRailRef, overlayRailSafeArea, pendingCelebrateCatalogueDropIdsRef, presenceObjectWorldPositions, presenceWindowId, previewAttractions, previewVortices, publishWorldPresenceView, referenceHoveredId, referenceSelectedIds, relocateMode, relocateSessionRef, sectionExtent, selectionMode, setConnectDragHoverPosition, setConnectDragSource, setContextMenu, setFaceDragSession, setGumballDragActive, setMarqueeCommitHold, setMarqueeModifiers, setMarqueePath, setSuggestionFocus, setVortexPointerArm, setVoxelHoverOrigin, sharedGumballTransformPreview, sharedSelectionPreview, suggestionFocus, suggestionMenuCheckingPlacementLabel, suggestionMenuNoPlacementLabel, suggestionMenuOpen, suggestionMenuOwnsThisWindow, suggestionMenuTitleLabel, suggestionPopupOwnsThisWindow, targetVolumeSelectedIds, toolRunTrace, volumeBrushMode, volumeGumballConfig, volumeLayersInteractive, vorticesRef, voxelHoverOrigin, wasMarqueeDragRef , presenceRuntimeKey };
}

export type WorldCore2 = WorldCore1 & ReturnType<typeof useWorldFitAndPresence>;
// #endregion World3dHost
