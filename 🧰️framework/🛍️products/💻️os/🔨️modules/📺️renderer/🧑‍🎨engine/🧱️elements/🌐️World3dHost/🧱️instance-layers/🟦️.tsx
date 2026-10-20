/** 🧱️ The instances layer and the mesh visuals hook of {@link ../🟦️.tsx | World3dHost}.
 *
 * 🧱️ The scene gumball (`🎛️scene-gumball`), instance node (`🧊️instance-node`), point-cloud layer
 * (`☁️point-cloud-layer`), vortex markers (`🌪️vortex-markers`) and instance chrome store
 * (`🪟️instance-chrome-store`) live in the child folders beside this file; `World3dHost` re-exports
 * the public names from the module that owns each of them.
 */

import { useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import { hopTrace } from "@semio-tech/framework-trace";

// #region 🔌️Adapters
import { reactHostPort, type GumballHandleKind, type GumballPose, BufferGeometry, EdgesGeometry, Group } from "@semio-tech/ui-react";

// #endregion 🔌️Adapters
import { type World3dSubElementPaint } from "../📏️modelling/🟦️.tsx";
import { type MergeMode } from "@semio-tech/framework";
import { worldProjectionGumballPlane, WorldLayerStack, type WorldProjectionSpec } from "@semio-tech/framework/infinite/world/r3f";
import { type WorldEnvironmentRecord, type WorldHoverComponent, type WorldInstanceRecord, type WorldMeshData, type WorldMeshRecord, type WorldSelectionRecord } from "../🔭️scene-schema/🟦️.ts";
import { useThree } from "../🎨️mesh-render/🟦️.tsx";
import { type MeshStylePalette } from "../🎨️mesh-render/🖌️mesh-style-palette/🟦️.ts";
import { buildMeshVisuals, disposeMeshVisuals, type MeshVisuals, type VertexPickData } from "../🎨️mesh-render/🔺️mesh-geometry-and-appearance/🟦️.ts";
import { world3dComponentInteractionTarget } from "../🔗️instance-selection/🟦️.ts";
import { applyGumballLivePreviewDeltaToPose, applyGumballLivePreviewPoseToObject3D, clearWorldGumballTransformPreview, GUMBALL_TRANSFORM_EPSILON, gumballLivePreviewDeltaBetweenPoses, isWorldInstanceRootObject3D, isWorldTransformGumballMode, setWorldGumballTransformPreview, worldGumballConfigForProjection, type WorldGumballLivePose, type WorldGumballTransformPreview, type WorldPaintOrigin } from "../🧮️gumball-math/🟦️.ts";
import { componentPickMergeMode, resolveWorldMergeMode } from "../🎯️marquee-and-bridges/🟦️.tsx";
import { SceneGumball } from "./🎛️scene-gumball/🟦️.tsx";
import { WORLD_INSTANCE_UNIT_SCALE, WorldInstanceNode, worldInstanceQuaternion } from "./🧊️instance-node/🟦️.tsx";
import { createWorldInstanceChromeStore, worldHoverPaintIdV1, WorldInstanceChromeContext } from "./🪟️instance-chrome-store/🟦️.ts";

//#region 🔖️World3dHost
//#region WorldInstancesLayer
/** 🧊️ Retains the viewport's existing visual leases for both drawing and accessible publication status. */
export function useWorldMeshVisuals(meshes:readonly WorldMeshRecord[]):Map<string,MeshVisuals>{
  const meshVisualsRef = useRef(new Map<string, MeshVisuals>());
  const retiredMeshVisualsRef = useRef<MeshVisuals[]>([]);
  const meshVisuals = useMemo(() => {
    const held = meshVisualsRef.current;
    const next = new Map<string, MeshVisuals>();
    const closeSpan = hopTrace.open("mesh.decode", { meshes: meshes.length });
    let built = 0;
    for (const mesh of meshes) {
      const existing = held.get(mesh.id);
      if (existing && existing.record === mesh) {
        next.set(mesh.id, existing);
        continue;
      }
      if (existing) retiredMeshVisualsRef.current.push(existing);
      try{next.set(mesh.id, buildMeshVisuals(mesh));}catch(error){next.set(mesh.id,{record:mesh,geometry:null,border:null,vertexPick:null,edge:null,appearance:null,fault:error instanceof Error?error.message:String(error),closed:false});}
      built += 1;
    }
    for (const [meshId, existing] of held) if (!next.has(meshId)) retiredMeshVisualsRef.current.push(existing);
    meshVisualsRef.current = next;
    closeSpan({ built, kept: meshes.length - built });
    return next;
  }, [meshes]);
  useEffect(() => {
    const retired = retiredMeshVisualsRef.current;
    retiredMeshVisualsRef.current = [];
    for (const entry of retired) disposeMeshVisuals(entry);
  }, [meshVisuals]);
  useEffect(
    () => () => {
      for (const entry of meshVisualsRef.current.values()) disposeMeshVisuals(entry);
      meshVisualsRef.current = new Map();
      for (const entry of retiredMeshVisualsRef.current) disposeMeshVisuals(entry);
      retiredMeshVisualsRef.current = [];
    },
    [],
  );
  return meshVisuals;
}

export function WorldInstancesLayer({
  meshVisuals,
  instances,
  meshes,
  selection,
  persistentSelectionMode,
  palette,
  projectionSpec,
  onInstancePointerDown,
  onInstancePointerMove,
  onLocalHoverRelease,
  onHoverPaint,
  onWorldPick,
  onComponentHover,
  onPaintAt,
  gumballDragActive,
  controllerId,
  gumballPreviewSourceId,
  sharedGumballPreview,
  onGumballDraggingChanged,
  gumballLiveDispatch,
  onGumballDragStart,
  onGumballDrag,
  onGumballDragEnd,
  onFaceDragStart,
  mergedComponentIds,
  mergedInstanceIds,
  blockPick,
  environment,
  subElementPaint,
}: {
  readonly meshVisuals:ReadonlyMap<string,MeshVisuals>;
  readonly instances: readonly WorldInstanceRecord[];
  readonly meshes: readonly WorldMeshRecord[];
  readonly selection: WorldSelectionRecord;
  /** 🐚️ This shell's own `SelectionModeStore` value — see `resolveWorldMergeMode`'s doc. */
  readonly persistentSelectionMode: MergeMode;
  readonly palette: MeshStylePalette;
  readonly projectionSpec?: WorldProjectionSpec;
  readonly onInstancePointerDown: (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => void;
  readonly onInstancePointerMove: (id: string | null) => void;
  /** 🎯️ Subscribes a "the pointer left this pane" listener; returns the unsubscribe. The host fires it
   * from the canvas root's `pointerleave` so the layer withdraws its local hover claim. */
  readonly onLocalHoverRelease?: (listener: () => void) => () => void;
  /** 🩺️ What the hover chrome currently paints (raycast claim merged with the guest echo) — the host
   * mirrors it onto `data-hover-paint-id` so a probe reads the painted hover, not only the echoed one. */
  readonly onHoverPaint?: (id: string | null) => void;
  readonly onWorldPick: (args: { granularity: string; id: number; merge: string; objectId?: string }) => void;
  readonly onComponentHover: (args: { objectId: string; mode: string; id: number } | null) => void;
  readonly onPaintAt?: (objectId: string, u: number, v: number, origin: WorldPaintOrigin) => void;
  readonly gumballDragActive: boolean;
  readonly controllerId: string;
  readonly gumballPreviewSourceId: string;
  readonly sharedGumballPreview: WorldGumballTransformPreview | null;
  readonly onGumballDraggingChanged: (dragging: boolean) => void;
  /** ⚡️ The document owns the mid-drag pose (see `WorldSelectionRecord.gumballLiveDispatch`): no local
   * instance preview is written or shared, the instances draw whatever `instancesJson` says. */
  readonly gumballLiveDispatch: boolean;
  readonly onGumballDragStart?: (kind: GumballHandleKind, before: GumballPose) => void;
  readonly onGumballDrag?: (kind: GumballHandleKind, pose: GumballPose) => void;
  readonly onGumballDragEnd: (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => void;
  readonly onFaceDragStart?: (args: { objectId: string; faceId: number; normal: readonly [number, number, number]; point: readonly [number, number, number]; faceExtent?: readonly [number, number] }) => void;
  /** 🧬️ Live drag-preview merged component id set (null when no marquee drag is in progress). */
  readonly mergedComponentIds?: readonly number[] | null;
  /** 🤝️ Live drag-preview merged whole-instance id set (null when no marquee drag is in progress). */
  readonly mergedInstanceIds?: readonly string[] | null;
  /** 🔳️ Disables instance picking; passed for brush engagements so a click meant for a vortex marker can't fall through and select/gumball the underlying object instead. */
  readonly blockPick?: boolean;
  readonly environment?: WorldEnvironmentRecord | null;
  readonly subElementPaint: World3dSubElementPaint;
}) {
  const meshById = useMemo(() => new Map(meshes.map((mesh) => [mesh.id, mesh])), [meshes]);
  const [localHoveredComponent, setLocalHoveredComponent] = useState<WorldHoverComponent | null>(null);
  const onComponentHoverPainted = useCallback(
    (args: { objectId: string; mode: string; id: number } | null) => {
      setLocalHoveredComponent((prior) => {
        const next = args && args.mode !== "mesh" && args.mode !== "object" ? args : null;
        if (prior === next || (prior && next && prior.objectId === next.objectId && prior.mode === next.mode && prior.id === next.id)) return prior;
        return next;
      });
      onComponentHover(args);
    },
    [onComponentHover],
  );
  const hoveredComponent = localHoveredComponent ?? selection.hoveredComponent;
  const geometries = useMemo(() => {
    const map = new Map<string, BufferGeometry>();
    for (const [meshId, entry] of meshVisuals) if (entry.geometry) map.set(meshId, entry.geometry);
    return map;
  }, [meshVisuals]);
  /** 🎨️ Per-meshId border outline geometry, shared by every instance of that mesh — never rebuilt per instance. */
  const borderGeometries = useMemo(() => {
    const map = new Map<string, EdgesGeometry>();
    for (const [meshId, entry] of meshVisuals) if (entry.border) map.set(meshId, entry.border);
    return map;
  }, [meshVisuals]);
  const vertexPickByMeshId = useMemo(() => {
    const map = new Map<string, VertexPickData | null>();
    for (const [meshId, entry] of meshVisuals) if (entry.record.data) map.set(meshId, entry.vertexPick);
    return map;
  }, [meshVisuals]);
  const edgeGeometryByMeshId = useMemo(() => {
    const map = new Map<string, BufferGeometry | null>();
    for (const [meshId, entry] of meshVisuals) if (entry.record.data) map.set(meshId, entry.edge);
    return map;
  }, [meshVisuals]);
  const targets = useMemo(() => selection.targets ?? { mesh: true, vertex: false, edge: false, face: false }, [selection.targets]);
  const selectionMode = selection.selectionMode ?? selection.granularity ?? "mesh";
  const currentComponentIds = new Set(selection.componentIds ?? []);
  const mergedComponentIdsSet = mergedComponentIds ? new Set(mergedComponentIds) : null;
  const selectedComponentIds = useMemo(
    () => (mergedComponentIdsSet ? new Set([...currentComponentIds].filter((id) => mergedComponentIdsSet.has(id))) : currentComponentIds),
    [currentComponentIds, mergedComponentIdsSet],
  );
  const previewComponentIds = useMemo(
    () => (mergedComponentIdsSet ? new Set([...mergedComponentIdsSet].filter((id) => !currentComponentIds.has(id))) : new Set<number>()),
    [currentComponentIds, mergedComponentIdsSet],
  );
  const analyticComponentIdsByInstance = useMemo(() => {
    const byInstance = new Map<string, Set<number>>();
    const byTarget = new Map(instances.map(instance => [instance.interactionId ?? instance.id, instance]));
    for (const target of selection.gumballSelectionIds ?? []) {
      const address = /^(.+)\.(vertex|edge|face)\.(0|[1-9][0-9]*)~/.exec(target);
      if (!address || address[2] !== selectionMode) continue;
      const instance = byTarget.get(address[1]!);
      const group = Number(address[3]);
      if (!instance?.componentSource || !Number.isSafeInteger(group)) continue;
      const mesh = meshById.get(instance.meshId ?? instance.id);
      if (!mesh || world3dComponentInteractionTarget([instance], [mesh], instance.id, address[2]!, group)?.id !== target) continue;
      let groups = byInstance.get(instance.id);
      if (!groups) byInstance.set(instance.id, groups = new Set());
      groups.add(group);
    }
    return byInstance;
  }, [instances, meshById, selection.gumballSelectionIds, selectionMode]);
  const mergedInstanceIdsSet = mergedInstanceIds ? new Set(mergedInstanceIds) : null;
  const selectedIds = selection.ids ?? [];
  const instanceChromeStore = useMemo(() => createWorldInstanceChromeStore(), []);
  reactHostPort.useLayoutEffect(() => {
    instanceChromeStore.setSnapshot({
      selectedIds: new Set(selection.ids ?? []),
      hoveredId: selection.hoveredId ?? null,
      hoveredKindId: selection.hoveredKindId ?? null,
      previewInstanceIds: mergedInstanceIdsSet,
    });
  }, [instanceChromeStore, mergedInstanceIdsSet, selection.hoveredId, selection.hoveredKindId, selection.ids]);
  const handleLocalInstancePointerMove = useCallback(
    (id: string | null) => {
      instanceChromeStore.setLocalHover(id);
      onInstancePointerMove(id);
    },
    [instanceChromeStore, onInstancePointerMove],
  );
  reactHostPort.useEffect(() => {
    if (!onLocalHoverRelease) return undefined;
    return onLocalHoverRelease(() => instanceChromeStore.setLocalHover(undefined));
  }, [instanceChromeStore, onLocalHoverRelease]);
  reactHostPort.useEffect(() => {
    if (!onHoverPaint) return undefined;
    const publish = () => onHoverPaint(worldHoverPaintIdV1(instanceChromeStore.localHover(), instanceChromeStore.getSnapshot().hoveredId));
    publish();
    return instanceChromeStore.subscribe(publish);
  }, [instanceChromeStore, onHoverPaint]);
  const pickEnabled = !gumballDragActive && !onPaintAt && !blockPick && !mergedComponentIdsSet && !mergedInstanceIdsSet;
  const transformMode = selection.transformMode;
  const transformGumballMode = isWorldTransformGumballMode(transformMode);
  const gumballConfig = useMemo(() => {
    if (selection.gumballConfig) {
      const plane = worldProjectionGumballPlane(projectionSpec);
      return plane ? { ...selection.gumballConfig, plane } : selection.gumballConfig;
    }
    return worldGumballConfigForProjection(transformMode ?? "move", projectionSpec);
  }, [selection.gumballConfig, transformMode, projectionSpec]);
  const paintMode = selection.interactionMode === "paint";
  const gumballVisible = Boolean(selection.gumballActive) && transformGumballMode && !paintMode;
  const invalidate = useThree((state) => state.invalidate);
  const instanceRootsRef = useRef(new Map<string, Group>());
  const gumballLiveBasesRef = useRef(new Map<string, WorldGumballLivePose>());
  /** ⚡️ Final local poses held across the post-drag React frame(s) until `instancesJson` catches up — prevents a one-frame snap-back to the pre-drag pose. */
  const gumballCommitHoldRef = useRef(new Map<string, WorldGumballLivePose>());
  const gumballLiveStartPoseRef = useRef<GumballPose | null>(null);
  const gumballLivePoseRef = useRef<GumballPose | null>(null);
  const gumballLiveKindRef = useRef<GumballHandleKind | null>(null);
  const gumballPreviewPivotRef = useRef<readonly [number, number, number]>([0, 0, 0]);

  const publishGumballPreview = useCallback(
    (before: GumballPose, after: GumballPose, handleKind: GumballHandleKind | null, instanceIds: readonly string[]) => {
      if (!controllerId || instanceIds.length === 0) return;
      setWorldGumballTransformPreview(controllerId, {
        sourceId: gumballPreviewSourceId,
        transformMode,
        handleKind,
        before,
        after,
        instanceIds,
        pivot: gumballPreviewPivotRef.current,
      });
    },
    [controllerId, gumballPreviewSourceId, transformMode],
  );

  const registerInstanceRoot = useCallback((id: string, group: Group | null) => {
    if (isWorldInstanceRootObject3D(group)) instanceRootsRef.current.set(id, group);
    else instanceRootsRef.current.delete(id);
  }, []);

  const writeGumballPreviewPoses = useCallback(
    (poses: ReadonlyMap<string, WorldGumballLivePose>) => {
      for (const [id, pose] of poses) {
        const root = instanceRootsRef.current.get(id);
        if (!root) continue;
        applyGumballLivePreviewPoseToObject3D(root, pose);
      }
      invalidate();
    },
    [invalidate],
  );

  const applyGumballLivePreview = useCallback(
    (before: GumballPose, after: GumballPose, handleKind: GumballHandleKind | null) => {
      const delta = gumballLivePreviewDeltaBetweenPoses(transformMode, before, after, handleKind ?? undefined);
      const poses = new Map<string, WorldGumballLivePose>();
      for (const [id, base] of gumballLiveBasesRef.current) {
        poses.set(id, delta ? applyGumballLivePreviewDeltaToPose(base, delta) : base);
      }
      writeGumballPreviewPoses(poses);
      publishGumballPreview(before, after, handleKind, [...gumballLiveBasesRef.current.keys()]);
      return poses;
    },
    [publishGumballPreview, transformMode, writeGumballPreviewPoses],
  );

  const applySharedGumballPreview = useCallback(
    (preview: WorldGumballTransformPreview) => {
      const delta = gumballLivePreviewDeltaBetweenPoses(preview.transformMode, preview.before, preview.after, preview.handleKind ?? undefined);
      const poses = new Map<string, WorldGumballLivePose>();
      for (const id of preview.instanceIds) {
        const instance = instances.find((entry) => entry.id === id);
        if (!instance) continue;
        const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
        const scale = instance.scale ?? [1, 1, 1];
        const rotation = instance.rotation ?? [0, 0, 0, 1];
        const base: WorldGumballLivePose = {
          position: [position[0], position[1], position[2]],
          quaternion: [rotation[0], rotation[1], rotation[2], rotation[3]],
          scale: [scale[0], scale[1], scale[2]],
        };
        poses.set(id, delta ? applyGumballLivePreviewDeltaToPose(base, delta) : base);
      }
      writeGumballPreviewPoses(poses);
    },
    [instances, writeGumballPreviewPoses],
  );

  const resetGumballPreviewPosesFromInstances = useCallback(() => {
    for (const instance of instances) {
      const root = instanceRootsRef.current.get(instance.id);
      if (!root) continue;
      const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      const scale = instance.scale ?? [1, 1, 1];
      const rotation = instance.rotation ?? [0, 0, 0, 1];
      applyGumballLivePreviewPoseToObject3D(root, {
        position: [position[0], position[1], position[2]],
        quaternion: [rotation[0], rotation[1], rotation[2], rotation[3]],
        scale: [scale[0], scale[1], scale[2]],
      });
    }
    invalidate();
  }, [instances, invalidate]);

  useLayoutEffect(() => {
    const before = gumballLiveStartPoseRef.current;
    const after = gumballLivePoseRef.current;
    if (gumballDragActive && before && after) {
      applyGumballLivePreview(before, after, gumballLiveKindRef.current);
      return;
    }
    if (!gumballDragActive && sharedGumballPreview && sharedGumballPreview.sourceId !== gumballPreviewSourceId) {
      applySharedGumballPreview(sharedGumballPreview);
      return;
    }
    if (!gumballDragActive && !sharedGumballPreview && gumballCommitHoldRef.current.size === 0) {
      resetGumballPreviewPosesFromInstances();
      return;
    }
    const hold = gumballCommitHoldRef.current;
    if (hold.size === 0) return;
    writeGumballPreviewPoses(hold);
    let allMatch = true;
    for (const [id, pose] of hold) {
      const instance = instances.find((entry) => entry.id === id);
      if (!instance) {
        allMatch = false;
        continue;
      }
      const position = instance.position ?? [instance.x ?? 0, instance.y ?? 0, instance.z ?? 0];
      const scale = instance.scale ?? [1, 1, 1];
      const rotation = instance.rotation ?? [0, 0, 0, 1];
      if (
        Math.abs(position[0] - pose.position[0]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(position[1] - pose.position[1]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(position[2] - pose.position[2]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(scale[0] - pose.scale[0]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(scale[1] - pose.scale[1]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(scale[2] - pose.scale[2]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(rotation[0] - pose.quaternion[0]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(rotation[1] - pose.quaternion[1]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(rotation[2] - pose.quaternion[2]) > GUMBALL_TRANSFORM_EPSILON ||
        Math.abs(rotation[3] - pose.quaternion[3]) > GUMBALL_TRANSFORM_EPSILON
      ) {
        allMatch = false;
      }
    }
    if (allMatch) hold.clear();
  }, [applyGumballLivePreview, applySharedGumballPreview, gumballDragActive, gumballPreviewSourceId, instances, resetGumballPreviewPosesFromInstances, sharedGumballPreview, writeGumballPreviewPoses]);

  useEffect(
    () => () => {
      clearWorldGumballTransformPreview(controllerId, gumballPreviewSourceId);
    },
    [controllerId, gumballPreviewSourceId],
  );

  const handleGumballDraggingChanged = useCallback(
    (dragging: boolean) => {
      if (dragging) gumballCommitHoldRef.current.clear();
      onGumballDraggingChanged(dragging);
    },
    [onGumballDraggingChanged],
  );

  const handleGumballDragStart = useCallback(
    (kind: GumballHandleKind, before: GumballPose) => {
      if (gumballLiveDispatch) {
        gumballLiveBasesRef.current.clear();
        gumballCommitHoldRef.current.clear();
        onGumballDragStart?.(kind, before);
        return;
      }
      gumballPreviewPivotRef.current = selection.gumballTarget ?? [before.position[0], before.position[1], before.position[2]];
      const bases = new Map<string, WorldGumballLivePose>();
      for (const id of selectedIds) {
        const root = instanceRootsRef.current.get(id);
        if (!root) continue;
        bases.set(id, {
          position: [root.position.x, root.position.y, root.position.z],
          quaternion: [root.quaternion.x, root.quaternion.y, root.quaternion.z, root.quaternion.w],
          scale: [root.scale.x, root.scale.y, root.scale.z],
        });
      }
      gumballLiveBasesRef.current = bases;
      gumballCommitHoldRef.current.clear();
      gumballLiveStartPoseRef.current = before;
      gumballLivePoseRef.current = before;
      gumballLiveKindRef.current = kind;
      publishGumballPreview(before, before, kind, selectedIds);
      onGumballDragStart?.(kind, before);
    },
    [gumballLiveDispatch, onGumballDragStart, publishGumballPreview, selectedIds, selection.gumballTarget],
  );

  const handleGumballDrag = useCallback(
    (kind: GumballHandleKind, pose: GumballPose) => {
      if (gumballLiveDispatch) {
        onGumballDrag?.(kind, pose);
        return;
      }
      const before = gumballLiveStartPoseRef.current ?? pose;
      gumballLiveStartPoseRef.current = before;
      gumballLivePoseRef.current = pose;
      gumballLiveKindRef.current = kind;
      applyGumballLivePreview(before, pose, kind);
      onGumballDrag?.(kind, pose);
    },
    [applyGumballLivePreview, gumballLiveDispatch, onGumballDrag],
  );

  const handleGumballDragEnd = useCallback(
    (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => {
      if (gumballLiveDispatch) {
        onGumballDragEnd(kind, before, after);
        return;
      }
      const start = gumballLiveStartPoseRef.current ?? before;
      const finals = applyGumballLivePreview(start, after, kind);
      gumballCommitHoldRef.current = finals;
      gumballLiveBasesRef.current.clear();
      gumballLiveStartPoseRef.current = null;
      gumballLivePoseRef.current = null;
      gumballLiveKindRef.current = null;
      clearWorldGumballTransformPreview(controllerId, gumballPreviewSourceId);
      onGumballDragEnd(kind, before, after);
    },
    [applyGumballLivePreview, controllerId, gumballLiveDispatch, gumballPreviewSourceId, onGumballDragEnd],
  );

  const mergeMode = useCallback(
    (event: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => componentPickMergeMode(resolveWorldMergeMode(selection.selectionMergeMode, event, persistentSelectionMode)),
    [selection.selectionMergeMode, persistentSelectionMode],
  );

  const paintFromHit = useCallback((objectId: string, mesh: WorldMeshData, event: { faceIndex?: number | null; uv?: { x: number; y: number } }, origin: WorldPaintOrigin) => {
    if (!onPaintAt) return;
    let u = event.uv?.x;
    let v = event.uv?.y;
    if (u == null || v == null) {
      if (event.faceIndex == null || !mesh.indices.length) return;
      const i0 = mesh.indices[event.faceIndex * 3] ?? 0;
      const i1 = mesh.indices[event.faceIndex * 3 + 1] ?? 0;
      const i2 = mesh.indices[event.faceIndex * 3 + 2] ?? 0;
      if (!mesh.uvs || mesh.uvs.length < 6) return;
      u = (mesh.uvs[i0 * 2]! + mesh.uvs[i1 * 2]! + mesh.uvs[i2 * 2]!) / 3;
      v = (mesh.uvs[i0 * 2 + 1]! + mesh.uvs[i1 * 2 + 1]! + mesh.uvs[i2 * 2 + 1]!) / 3;
    }
    onPaintAt(objectId, u, v, origin);
  }, [onPaintAt]);

  return (
    <WorldInstanceChromeContext.Provider value={instanceChromeStore}>
    <WorldLayerStack>
      <group>
        {instances.map((instance, index) => {
          const meshId = instance.meshId ?? instance.id;
          const meshRecord = meshById.get(meshId);
          const meshData = meshRecord?.data;
          const geometry = geometries.get(meshId);
          const position = instance.position ?? [instance.x ?? index, instance.y ?? 0, instance.z ?? 0];
          const scale = instance.scale ?? WORLD_INSTANCE_UNIT_SCALE;
          const quaternion = worldInstanceQuaternion(instance);
          const isActiveObject = instance.id === selection.activeObjectId || (instance.interactionId != null && instance.interactionId === selection.activeObjectId);
          const componentIds = instance.componentSource ? analyticComponentIdsByInstance.get(instance.id) ?? new Set<number>() : isActiveObject ? selectedComponentIds : new Set<number>();
          const selectedIds = instance.componentSource && isActiveObject && mergedComponentIdsSet ? new Set([...componentIds].filter(id => mergedComponentIdsSet.has(id))) : componentIds;
          return (
            <WorldInstanceNode
              key={instance.id}
              instance={instance}
              previewInstanceSelected={undefined}
              index={index}
              meshRecord={meshRecord}
              meshData={meshData}
              geometry={geometry}
              appearance={meshVisuals.get(meshId)?.appearance}
              borderGeometry={borderGeometries.get(meshId)}
              palette={palette}
              vertexPick={vertexPickByMeshId.get(meshId) ?? null}
              edgeGeometry={edgeGeometryByMeshId.get(meshId) ?? null}
              paintTextureBase64={meshData?.paintTextureBase64}
              position={position as [number, number, number]}
              scale={scale as [number, number, number]}
              quaternion={quaternion}
              targets={targets}
              activeObjectId={selection.activeObjectId ?? undefined}
              selectionMode={selectionMode}
              selectedComponentIds={selectedIds}
              previewComponentIds={previewComponentIds}
              hoveredComponent={hoveredComponent}
              showEdges={selection.showEdges}
              pickEnabled={pickEnabled}
              onPaintAt={onPaintAt}
              paintFromHit={paintFromHit}
              flatShading={instance.smoothShading === false}
              onInstancePointerDown={onInstancePointerDown}
              onInstancePointerMove={handleLocalInstancePointerMove}
              onWorldPick={onWorldPick}
              onComponentHover={onComponentHoverPainted}
              mergeMode={mergeMode}
              faceDragActive={selection.faceDragActive === true}
              onFaceDragStart={onFaceDragStart}
              environmentMaterial={environment?.material}
              environmentShadowEnabled={environment?.shadow?.enabled === true}
              subElementPaint={subElementPaint}
              onRootRef={registerInstanceRoot}
            />
          );
        })}
      </group>
      <SceneGumball
        target={selection.gumballTarget}
        config={gumballConfig}
        active={gumballVisible}
        onDraggingChanged={handleGumballDraggingChanged}
        onDragStart={handleGumballDragStart}
        onDrag={handleGumballDrag}
        onDragEnd={handleGumballDragEnd}
      />
    </WorldLayerStack>
    </WorldInstanceChromeContext.Provider>
  );
}
//#endregion WorldInstancesLayer
//#endregion 🔖️World3dHost
