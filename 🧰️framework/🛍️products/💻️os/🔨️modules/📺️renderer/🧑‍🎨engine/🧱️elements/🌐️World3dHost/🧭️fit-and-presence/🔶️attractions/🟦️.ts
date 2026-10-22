/** 🔶️ The attractions phase of useWorldFitAndPresence. */
import { useCallback, useEffect, useMemo, useRef, useState, useSyncExternalStore } from "../../../⚛️react-port/🟦️.ts";
import type { ThreeCamera } from "@semio-tech/ui-react";
import { useChromePanelSafeArea, useLabel, uiSpacingPx, type SelectionMarqueePoint, Group } from "@semio-tech/ui-react";
import { useToolRunTraceCursorEcho, useToolRunTraceStore } from "../../⏯️tool-run-trace/🟦️.tsx";
import { world3dComputeStatusV1 } from "@semio-tech/framework";
import { DEFAULT_MANUAL_LOD } from "@semio-tech/framework/infinite/world/r3f";
import { declareSurfaceCancelAction } from "../../../🛠️ShellHelpers/🛑️extension-io/🟦️.ts";
import { useLocalPresenceActorIdV1, usePresenceRuntimeKeyV1 } from "../../../👕️canvas-presence/🟦️.tsx";
import { PRESENCE_VIEW_PUBLISH_MIN_INTERVAL_MS, publishLocalPresenceWindowViewV1, clearLocalPresenceWindowViewV1, publishLocalActiveToolV1 } from "../../../👕️canvas-presence/🟦️.ts";
import { type WorldVortexRecord } from "../../🔭️scene-schema/🟦️.ts";
import { getWorldGumballTransformPreview, getWorldGumballTransformPreviewServerSnapshot, subscribeWorldGumballTransformPreview, worldAttractionsWithGumballPreview, worldGumballOwnerPoses, worldVorticesWithGumballPreview } from "../../🧮️gumball-math/🟦️.ts";
import { getWorldCatalogueDropPreview, getWorldCatalogueDropPreviewServerSnapshot, getWorldSelectionPreview, getWorldSelectionPreviewServerSnapshot, subscribeWorldCatalogueDropPreview, subscribeWorldSelectionPreview } from "../../🗂️catalogue-drop/🟦️.tsx";
import { type SurfaceContextMenuResult } from "../../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import type { WorldCore1 } from "../../🌍️scene-lanes/🟦️.ts";

/** 🔷 Builds attractions, cameraState, contentBounds, fit of useWorldFitAndPresence. */
export function useAttractionsState({ core }: { readonly core: WorldCore1 }) {
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
  const computeStatus = useMemo(() => world3dComputeStatusV1(scene?.statusJson), [scene?.statusJson]);
  const computing = computeStatus.computing;
  const toolRunTrace = useToolRunTraceStore(scene?.toolRunTrace, useToolRunTraceCursorEcho(windowInstanceId));
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
  return { instancesRef, instances, meshes, vortices, references, selection, lod, interaction, windowInstanceId, marqueePath, marqueeModifiers, persistentSelectionMode, onAction, node, setViewportCamera, sceneCamera, retainViewport, setViewportOwned, setDetachEpoch, lastDispatchedWorldCameraRef, activeUtility, interactionDomainId, targetVolumes, autoFitBounds, brushMode, cameraRef, catalogueDragDepthRef, catalogueDragEncodedRef, catalogueDropPreview, computeStatus, connectDragHoverPosition, connectDragSource, contextMenu, contextMenuOpenEpochRef, contextMenuTitleLabel, dismissContextMenu, faceDragSession, gumballDragActive, gumballTransformPreviewSourceId, hostRef, instancesGroupRef, localPresenceActor, lodRef, overlayRailRef, overlayRailSafeArea, pendingCelebrateCatalogueDropIdsRef, presenceObjectWorldPositions, presenceWindowId, previewAttractions, previewVortices, publishWorldPresenceView, relocateMode, sectionExtent, setConnectDragHoverPosition, setConnectDragSource, setContextMenu, setFaceDragSession, setGumballDragActive, setMarqueeModifiers, setMarqueePath, setVortexPointerArm, setVoxelHoverOrigin, sharedGumballTransformPreview, sharedSelectionPreview, toolRunTrace, volumeBrushMode, volumeLayersInteractive, voxelHoverOrigin, presenceRuntimeKey };
}
