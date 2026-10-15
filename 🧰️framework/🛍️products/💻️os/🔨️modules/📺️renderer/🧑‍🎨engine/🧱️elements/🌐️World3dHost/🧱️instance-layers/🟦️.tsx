/** 🧱️ The instance chrome store, scene gumball, instance node, instances layer, point-cloud layer and vortex markers.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | World3dHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import {
  useCallback,
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  Suspense,
  type ComponentProps
} from "react";
import { hopTrace } from "../../../../../../../../🔨️modules/⏱️trace/🟦️.ts";
// #region 🔌️Adapters
import {
  reactHostPort,
  UnifiedGumball,
  useUiDriver,
  type GumballConfig,
  type GumballHandleKind,
  type GumballPose,
  BufferAttribute,
  BufferGeometry,
  DoubleSide,
  EdgesGeometry,
  Group,
  Object3D,
  PointsMaterial,
  Quaternion,
  Vector3
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { ToolRunProvisionalOutline, useToolRunProvisional } from "../⏯️tool-run-trace/🟦️.tsx";
import { world3dGlbHitFaceId, type World3dSubElementPaint } from "../📏️modelling/🟦️.tsx";
import { type MergeMode } from "@semio-tech/framework";
import { worldProjectionGumballPlane, WorldLayerStack, type WorldProjectionSpec } from "@semio-tech/infinite-world-r3f";
import { base64ToBytes } from "../../🖌️Paint2dHost/🟦️.tsx";
import { runtimeDiagnosticsEnabled } from "../../🏛️ShellHost/🔌️extension-runtime/🟦️.ts";
import {
  type WorldEnvironmentMaterialRecord,
  type WorldEnvironmentRecord,
  type WorldHoverComponent,
  type WorldInstanceRecord,
  type WorldMeshData,
  type WorldMeshRecord,
  type WorldPointCloudLayerRecord,
  type WorldSelectionRecord,
  type WorldSelectionTargets,
  type WorldVortexRecord
} from "../🔭️scene-schema/🟦️.ts";
import {
  buildEdgeOverlayGeometry,
  buildFaceOverlayGeometry,
  buildMeshVisuals,
  buildVertexOverlayGeometry,
  disposeMeshVisuals,
  faceExtentFromMesh,
  isCurveOnlyWorldMesh,
  type MeshStylePalette,
  type MeshVisuals,
  PaintTexturedMesh,
  resolveMeshSelectionPreviewStyle,
  semanticColorsFromPalette,
  useCelebratingWorldInstanceIds,
  useThree,
  type VertexPickData,
  WORLD_VERTEX_DOT_PX,
  worldInstanceHighlighted,
  type WorldMeshAppearance,
  worldMeshMaterialRevision
} from "../🎨️mesh-render/🟦️.tsx";
import { GlbInstanceMesh, GlbSubElementLayer } from "../🧩️glb-surface/🟦️.tsx";
import {
  resolveVortexPointerDownIntent,
  world3dComponentInteractionTarget,
  worldInstanceMeshRaycast,
  worldVortexHitProxy
} from "../🔗️instance-selection/🟦️.ts";
import { parseJsonArray } from "../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import {
  applyGumballLivePreviewDeltaToPose,
  applyGumballLivePreviewPoseToObject3D,
  clearWorldGumballTransformPreview,
  GUMBALL_TRANSFORM_EPSILON,
  gumballLivePreviewDeltaBetweenPoses,
  isWorldInstanceRootObject3D,
  isWorldTransformGumballMode,
  setWorldGumballTransformPreview,
  useUiCanvasHovered,
  worldGumballConfigForProjection,
  type WorldGumballLivePose,
  type WorldGumballTransformPreview,
  type WorldPaintOrigin
} from "../🧮️gumball-math/🟦️.ts";
import { componentPickMergeMode, resolveWorldMergeMode } from "../🎯️marquee-and-bridges/🟦️.tsx";

//#region 🔖️World3dHost
//#region WorldSceneParsing
function SceneGumball({
  target,
  config,
  active,
  onDraggingChanged,
  onDragStart,
  onDrag,
  onDragEnd,
}: {
  readonly target?: readonly [number, number, number];
  readonly config: GumballConfig;
  readonly active: boolean;
  readonly onDraggingChanged: (dragging: boolean) => void;
  readonly onDragStart?: (kind: GumballHandleKind, before: GumballPose) => void;
  readonly onDrag?: (kind: GumballHandleKind, pose: GumballPose) => void;
  readonly onDragEnd: (kind: GumballHandleKind, before: GumballPose, after: GumballPose) => void;
}) {
  const pivotRef = useRef<Object3D>(new Object3D());
  const draggingRef = useRef(false);
  const [ready, setReady] = useState(false);
  const driver = useUiDriver();
  const hovered = useUiCanvasHovered(driver.gumball === "hover");
  useEffect(() => {
    if (!target || draggingRef.current) return;
    pivotRef.current.position.set(target[0], target[1], target[2]);
    pivotRef.current.quaternion.set(0, 0, 0, 1);
    pivotRef.current.scale.set(1, 1, 1);
    pivotRef.current.updateMatrixWorld(true);
    setReady(true);
  }, [target]);
  if (!active || !target || !ready || !hovered) return null;
  return (
    <>
      <primitive object={pivotRef.current} />
      <UnifiedGumball
        target={pivotRef.current}
        config={config}
        onDraggingChanged={(dragging) => {
          draggingRef.current = dragging;
          onDraggingChanged(dragging);
        }}
        onDragStart={onDragStart}
        onDrag={onDrag}
        onDragEnd={(kind, before, after) => {
          onDragEnd(kind, before, after);
          pivotRef.current.position.set(after.position[0], after.position[1], after.position[2]);
          pivotRef.current.quaternion.set(after.quaternion[0], after.quaternion[1], after.quaternion[2], after.quaternion[3]);
          pivotRef.current.scale.set(after.scale[0], after.scale[1], after.scale[2]);
          pivotRef.current.updateMatrixWorld(true);
          draggingRef.current = false;
        }}
      />
    </>
  );
}

const WorldInstanceNode = reactHostPort.memo(function WorldInstanceNode({
  instance,
  index,
  meshRecord,
  meshData,
  geometry,
  appearance,
  borderGeometry,
  palette,
  vertexPick,
  edgeGeometry,
  paintTextureBase64,
  position,
  scale,
  quaternion,
  targets,
  activeObjectId,
  selectionMode,
  selectedComponentIds,
  previewComponentIds,
  hoveredComponent,
  showEdges,
  pickEnabled,
  onPaintAt,
  paintFromHit,
  flatShading,
  onInstancePointerDown,
  onInstancePointerMove,
  onWorldPick,
  onComponentHover,
  mergeMode,
  previewInstanceSelected,
  environmentMaterial,
  environmentShadowEnabled,
  faceDragActive,
  onFaceDragStart,
  subElementPaint,
  onRootRef,
}: {
  readonly instance: WorldInstanceRecord;
  readonly index: number;
  readonly meshRecord?: WorldMeshRecord;
  readonly meshData?: WorldMeshData;
  readonly geometry?: BufferGeometry;
  readonly appearance?: WorldMeshAppearance | null;
  /** 🎨️ Shared per-meshId edge outline geometry (see {@link WorldInstancesLayer}'s `geometries` memo); never rebuilt per instance. */
  readonly borderGeometry?: EdgesGeometry;
  readonly palette: MeshStylePalette;
  readonly vertexPick: VertexPickData | null;
  readonly edgeGeometry: BufferGeometry | null;
  readonly paintTextureBase64?: string;
  readonly position: readonly [number, number, number];
  readonly scale: readonly [number, number, number];
  readonly quaternion?: Quaternion;
  readonly targets: WorldSelectionTargets;
  readonly activeObjectId?: string;
  readonly selectionMode: string;
  readonly selectedComponentIds: ReadonlySet<number>;
  readonly previewComponentIds: ReadonlySet<number>;
  readonly hoveredComponent?: WorldHoverComponent;
  readonly showEdges?: boolean;
  readonly pickEnabled: boolean;
  readonly onPaintAt?: (objectId: string, u: number, v: number, origin: WorldPaintOrigin) => void;
  readonly paintFromHit: (objectId: string, mesh: WorldMeshData, event: { faceIndex?: number | null; uv?: { x: number; y: number } }, origin: WorldPaintOrigin) => void;
  readonly flatShading?: boolean;
  readonly onInstancePointerDown: (id: string, index: number, event: { shiftKey: boolean; ctrlKey: boolean; metaKey: boolean }) => void;
  readonly onInstancePointerMove: (id: string | null) => void;
  readonly onWorldPick: (args: { granularity: string; id: number; merge: string; objectId?: string }) => void;
  readonly onComponentHover: (args: { objectId: string; mode: string; id: number } | null) => void;
  readonly mergeMode: (event: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => string;
  /** 🖱️➡️ When true, pointer-down on an already-selected face starts a push/pull drag instead of falling through to selection/orbit. */
  readonly faceDragActive?: boolean;
  readonly onFaceDragStart?: (args: { objectId: string; faceId: number; normal: readonly [number, number, number]; point: readonly [number, number, number]; faceExtent?: readonly [number, number] }) => void;
  /** Live marquee-drag merged selection state for this instance; undefined when no drag is in progress. */
  readonly previewInstanceSelected?: boolean;
  readonly environmentMaterial?: WorldEnvironmentMaterialRecord;
  readonly environmentShadowEnabled?: boolean;
  /** 🖍️ Sub-element hover and selection paint: theme defaults unless the scene's highlight tokens override a granularity. */
  readonly subElementPaint: World3dSubElementPaint;
  /** ⚡️ Registers the instance root group for imperative mid-drag gumball live preview. */
  readonly onRootRef?: (id: string, group: Group | null) => void;
}) {
  const reportDraw = useMemo<ComponentProps<"mesh">["onAfterRender"]>(() => {
    let canvas: HTMLCanvasElement | null = null;
    let retainedRevision: string | null = null;
    let surface: HTMLElement | null = null;
    return (renderer, _scene, _camera, drawnGeometry) => {
      const currentCanvas = renderer.domElement;
      if (canvas !== currentCanvas || !surface?.isConnected) surface = currentCanvas.closest<HTMLElement>("[data-meshes-json]");
      const revision = surface?.dataset.worldGeometryRevision ?? null;
      if (!surface || !revision || (canvas === currentCanvas && retainedRevision === revision)) return;
      canvas = currentCanvas;
      retainedRevision = revision;
      currentCanvas.dataset.worldGeometryDrawn = instance.id;
      currentCanvas.dataset.worldGeometryMesh = instance.meshId ?? instance.id;
      currentCanvas.dataset.worldGeometryRevision = revision;
      currentCanvas.dataset.worldGeometryVertices = String(drawnGeometry.getAttribute("position")?.count ?? 0);
      if (runtimeDiagnosticsEnabled()) console.debug("[DEBUG] world geometry drawn", instance.id, instance.meshId ?? instance.id, drawnGeometry.getAttribute("position")?.count ?? 0);
    };
  }, [instance.id, instance.meshId, geometry]);
  const rootRef = useCallback(
    (group: Group | null) => {
      onRootRef?.(instance.id, group);
    },
    [instance.id, onRootRef],
  );
  const chrome = useWorldInstanceChrome(instance.id, instance.objectKind);
  const isActiveObject = instance.id === activeObjectId || (instance.interactionId != null && instance.interactionId === activeObjectId);
  const colors = semanticColorsFromPalette(palette);
  const celebratingIds = useCelebratingWorldInstanceIds();
  const provisional = useToolRunProvisional(instance) || instance.provisional === true;
  const styleKind = resolveMeshSelectionPreviewStyle(
    { ...instance, provisional, selected: chrome.selected, hovered: chrome.hovered, highlighted: worldInstanceHighlighted(chrome.highlighted, instance), celebrating: celebratingIds.has(instance.id) },
    chrome.previewSelected ?? previewInstanceSelected,
  );
  const style = palette[styleKind];
  const glbUsesEnvironmentColor = styleKind === "neutral" && environmentMaterial?.color != null;
  const glbColor = glbUsesEnvironmentColor ? environmentMaterial!.color! : style.meshColor;
  const glbEmissive = glbUsesEnvironmentColor && environmentMaterial?.emissive ? environmentMaterial.emissive : style.meshColor;
  const glbEmissiveIntensity = glbUsesEnvironmentColor && environmentMaterial?.emissive ? (environmentMaterial.emissiveIntensity ?? 1) : style.emissiveIntensity;
  const instancePickEnabled = pickEnabled && !instance.disabled && !provisional;
  const meshReachable = !targets.exclusive || targets.mesh === true;
  const wholePickEnabled = instancePickEnabled && meshReachable;
  const lockedClickClears = instance.disabled === true;
  const hoveredFaceId = hoveredComponent?.mode === "face" && hoveredComponent.objectId === instance.id ? hoveredComponent.id : undefined;
  const hoveredVertexId = hoveredComponent?.mode === "vertex" && hoveredComponent.objectId === instance.id ? hoveredComponent.id : undefined;
  const hoveredEdgeId = hoveredComponent?.mode === "edge" && hoveredComponent.objectId === instance.id ? hoveredComponent.id : undefined;
  const selectedFaceIds = selectionMode === "face" ? selectedComponentIds : new Set<number>();
  const selectedVertexIds = selectionMode === "vertex" ? selectedComponentIds : new Set<number>();
  const selectedEdgeIds = selectionMode === "edge" ? selectedComponentIds : new Set<number>();
  const previewFaceIds = isActiveObject && selectionMode === "face" ? previewComponentIds : new Set<number>();
  const previewVertexIds = isActiveObject && selectionMode === "vertex" ? previewComponentIds : new Set<number>();
  const previewEdgeIds = isActiveObject && selectionMode === "edge" ? previewComponentIds : new Set<number>();
  const facePreviewOverlay = meshData && previewFaceIds.size > 0 ? buildFaceOverlayGeometry(meshData, previewFaceIds) : null;
  const edgePreviewOverlay = meshData && previewEdgeIds.size > 0 ? buildEdgeOverlayGeometry(meshData, previewEdgeIds) : null;
  const vertexPreviewOverlay = meshData && previewVertexIds.size > 0 ? buildVertexOverlayGeometry(meshData, previewVertexIds) : null;
  const faceSelectedOverlay = meshData ? buildFaceOverlayGeometry(meshData, selectedFaceIds) : null;
  const faceHoveredOverlay = meshData && hoveredFaceId != null ? buildFaceOverlayGeometry(meshData, new Set([hoveredFaceId])) : null;
  const edgeSelectedOverlay = meshData ? buildEdgeOverlayGeometry(meshData, selectedEdgeIds) : null;
  const edgeHoveredOverlay = meshData && hoveredEdgeId != null ? buildEdgeOverlayGeometry(meshData, new Set([hoveredEdgeId])) : null;
  const vertexSelectedOverlay = meshData ? buildVertexOverlayGeometry(meshData, selectedVertexIds) : null;
  const vertexHoveredOverlay = meshData && hoveredVertexId != null ? buildVertexOverlayGeometry(meshData, new Set([hoveredVertexId])) : null;

  const hasShadedMesh = Boolean(meshData && geometry && meshData.indices.length > 0);
  const isCurveOnly = Boolean(meshData && isCurveOnlyWorldMesh(meshData));
  const isPointOnly = Boolean(meshData && !hasShadedMesh && !edgeGeometry && vertexPick);
  const curveLineWidth = styleKind === "neutral" ? 2 : 4;

  return (
    <group ref={rootRef} position={position as [number, number, number]} scale={scale as [number, number, number]} quaternion={quaternion} userData={WORLD_INSTANCE_ROOT_USER_DATA}>
      {meshData ? (
        <>
          {hasShadedMesh && geometry ? (
            <PaintTexturedMesh
            onAfterRender={reportDraw}
            geometry={geometry}
            appearance={appearance}
            style={style}
            styleKind={styleKind}
            textureBase64={paintTextureBase64}
            flatShading={flatShading}
            raycast={worldInstanceMeshRaycast(instancePickEnabled)}
            onPointerDown={(event) => {
              if (onPaintAt) {
                if (event.button === 0) paintFromHit(instance.id, meshData, event, "press");
                return;
              }
              if (!faceDragActive || !onFaceDragStart || !event.face) return;
              if (!(targets.face && event.faceIndex != null && meshData.faceIds?.[event.faceIndex] != null)) return;
              const faceId = meshData.faceIds[event.faceIndex]!;
              if (!(isActiveObject && selectionMode === "face" && selectedComponentIds.has(faceId))) return;
              event.stopPropagation();
              const normal = event.face.normal.clone().transformDirection(event.object.matrixWorld).normalize();
              onFaceDragStart({
                objectId: instance.id,
                faceId,
                normal: [normal.x, normal.y, normal.z],
                point: [event.point.x, event.point.y, event.point.z],
                faceExtent: faceExtentFromMesh(meshData, faceId),
              });
            }}
            onClick={(event) => {
              if (onPaintAt) {
                paintFromHit(instance.id, meshData, event, "click");
                return;
              }
              if (lockedClickClears) {
                event.stopPropagation();
                onInstancePointerDown(instance.id, index, event);
                return;
              }
              if (!instancePickEnabled) return;
              event.stopPropagation();
              if (targets.face && event.faceIndex != null && meshData.faceIds?.[event.faceIndex] != null) {
                onWorldPick({
                  granularity: "face",
                  id: meshData.faceIds[event.faceIndex]!,
                  merge: mergeMode(event),
                  objectId: instance.id,
                });
              } else if (targets.mesh) {
                onInstancePointerDown(instance.id, index, event);
              }
            }}
            onPointerMove={(event) => {
              if (onPaintAt) {
                if ((event.buttons & 1) !== 0) paintFromHit(instance.id, meshData, event, "drag");
                return;
              }
              if (!instancePickEnabled) return;
              event.stopPropagation();
              if (targets.face && event.faceIndex != null && meshData.faceIds?.[event.faceIndex] != null) {
                onComponentHover({
                  objectId: instance.id,
                  mode: "face",
                  id: meshData.faceIds[event.faceIndex]!,
                });
              } else if (meshReachable) {
                onInstancePointerMove(instance.id);
              }
            }}
            onPointerOut={() => {
              onInstancePointerMove(null);
              onComponentHover(null);
            }}
          ></PaintTexturedMesh>
          ) : null}
          {hasShadedMesh && borderGeometry && (showEdges ?? true) && !edgeGeometry && !provisional ? (
            <lineSegments geometry={borderGeometry} scale={1.001} raycast={() => null} renderOrder={2}>
              <lineBasicMaterial color={style.lineColor} depthTest={false} />
            </lineSegments>
          ) : null}
          {provisional && borderGeometry ? <ToolRunProvisionalOutline geometry={borderGeometry} /> : null}
          {(targets.edge || isCurveOnly || (showEdges ?? true) || (selectionMode === "mesh" && selectedComponentIds.size > 0)) && edgeGeometry ? (
            <lineSegments
              onAfterRender={reportDraw}
              geometry={edgeGeometry}
              renderOrder={2}
              onClick={(event) => {
                if (lockedClickClears) {
                  event.stopPropagation();
                  onInstancePointerDown(instance.id, index, event);
                  return;
                }
                if (!instancePickEnabled) return;
                // 🧵️ Displayed edges (`showEdges`) are not pick targets unless edge granularity is armed: let the
                // hit fall through to the face/instance under it instead of swallowing every click near a seam.
                if (!targets.edge && !isCurveOnly) return;
                event.stopPropagation();
                // 🧵️ Centerline/curve objects are the model-definition instances — never decompose into edge components.
                if (isCurveOnly && targets.mesh) {
                  onInstancePointerDown(instance.id, index, event);
                  return;
                }
                if (meshData.edgeIds?.length) {
                  const edgeIndex = Math.floor((event.index ?? 0) / 2);
                  const edgeId = meshData.edgeIds[edgeIndex];
                  if (edgeId != null) {
                    onWorldPick({ granularity: "edge", id: edgeId, merge: mergeMode(event), objectId: instance.id });
                    return;
                  }
                }
                if (targets.mesh) {
                  onInstancePointerDown(instance.id, index, event);
                }
              }}
              onPointerMove={(event) => {
                if (!instancePickEnabled) return;
                if (!targets.edge && !isCurveOnly) return;
                event.stopPropagation();
                if (isCurveOnly) {
                  onInstancePointerMove(instance.id);
                  return;
                }
                if (!meshData?.edgeIds?.length) return;
                const edgeIndex = Math.floor((event.index ?? 0) / 2);
                const edgeId = meshData.edgeIds[edgeIndex];
                if (edgeId == null) return;
                onComponentHover({ objectId: instance.id, mode: "edge", id: edgeId });
              }}
              onPointerOut={() => {
                if (isCurveOnly) onInstancePointerMove(null);
                onComponentHover(null);
              }}
            >
              <lineBasicMaterial color={style.lineColor} linewidth={isCurveOnly ? curveLineWidth : 1} depthTest={false} />
            </lineSegments>
          ) : null}
          {(targets.vertex || isPointOnly) && vertexPick ? (
            <points
              onAfterRender={reportDraw}
              geometry={vertexPick.geometry}
              onClick={(event) => {
                if (lockedClickClears) {
                  event.stopPropagation();
                  onInstancePointerDown(instance.id, index, event);
                  return;
                }
                if (!instancePickEnabled) return;
                event.stopPropagation();
                if (!targets.vertex) {
                  if (targets.mesh) onInstancePointerDown(instance.id, index, event);
                  return;
                }
                const idx = event.index ?? 0;
                const vertexId = vertexPick.vertexIds[idx];
                if (vertexId == null) return;
                onWorldPick({ granularity: "vertex", id: vertexId, merge: mergeMode(event), objectId: instance.id });
              }}
              onPointerMove={(event) => {
                if (!instancePickEnabled) return;
                event.stopPropagation();
                if (!targets.vertex) {
                  if (meshReachable) onInstancePointerMove(instance.id);
                  return;
                }
                const idx = event.index ?? 0;
                const vertexId = vertexPick.vertexIds[idx];
                if (vertexId == null) return;
                onComponentHover({ objectId: instance.id, mode: "vertex", id: vertexId });
              }}
              onPointerOut={() => { onInstancePointerMove(null); onComponentHover(null); }}
            >
              {/* 📍️ Screen-space dots: a world-unit size (0.05) is sub-pixel on a six-unit mesh seen from
                  thirty units away, so vertex mode showed nothing to hover or pick (lowpoly, 2026-09-18). */}
              <pointsMaterial color={isPointOnly ? style.lineColor : colors.edge} size={WORLD_VERTEX_DOT_PX} sizeAttenuation={false} />
            </points>
          ) : null}
          {faceSelectedOverlay ? (
            <mesh geometry={faceSelectedOverlay} raycast={() => null}>
              <meshBasicMaterial color={subElementPaint.faceSelect} transparent opacity={0.62} side={DoubleSide} depthWrite={false} polygonOffset polygonOffsetFactor={-2} />
            </mesh>
          ) : null}
          {faceHoveredOverlay ? (
            <mesh geometry={faceHoveredOverlay} raycast={() => null}>
              <meshBasicMaterial color={subElementPaint.faceHover} transparent opacity={0.48} side={DoubleSide} depthWrite={false} polygonOffset polygonOffsetFactor={-3} />
            </mesh>
          ) : null}
          {facePreviewOverlay ? (
            <mesh geometry={facePreviewOverlay} raycast={() => null}>
              <meshBasicMaterial color={subElementPaint.faceHover} transparent opacity={0.36} side={DoubleSide} depthWrite={false} polygonOffset polygonOffsetFactor={-4} />
            </mesh>
          ) : null}
          {edgeSelectedOverlay ? (
            <lineSegments geometry={edgeSelectedOverlay} raycast={() => null} renderOrder={3}>
              <lineBasicMaterial color={subElementPaint.edgeSelect} linewidth={subElementPaint.edgeWidth} depthTest={false} />
            </lineSegments>
          ) : null}
          {edgeHoveredOverlay ? (
            <lineSegments geometry={edgeHoveredOverlay} raycast={() => null} renderOrder={4}>
              <lineBasicMaterial color={subElementPaint.edgeHover} linewidth={subElementPaint.edgeWidth} depthTest={false} />
            </lineSegments>
          ) : null}
          {edgePreviewOverlay ? (
            <lineSegments geometry={edgePreviewOverlay} raycast={() => null} renderOrder={3}>
              <lineBasicMaterial color={subElementPaint.edgeHover} linewidth={Math.min(2, subElementPaint.edgeWidth)} depthTest={false} />
            </lineSegments>
          ) : null}
          {vertexSelectedOverlay ? (
            <points geometry={vertexSelectedOverlay} raycast={() => null}>
              <pointsMaterial color={subElementPaint.vertexSelect} size={subElementPaint.vertexMarkPx} sizeAttenuation={false} depthTest={false} />
            </points>
          ) : null}
          {vertexHoveredOverlay ? (
            <points geometry={vertexHoveredOverlay} raycast={() => null}>
              <pointsMaterial color={subElementPaint.vertexHover} size={subElementPaint.vertexMarkPx} sizeAttenuation={false} depthTest={false} />
            </points>
          ) : null}
          {vertexPreviewOverlay ? (
            <points geometry={vertexPreviewOverlay} raycast={() => null}>
              <pointsMaterial color={subElementPaint.vertexHover} size={subElementPaint.vertexMarkPx} sizeAttenuation={false} depthTest={false} />
            </points>
          ) : null}
        </>
      ) : meshRecord?.url ? (
        <group
          onPointerDown={(event) => {
            if (instancePickEnabled && targets.face && world3dGlbHitFaceId(event.object, event.faceIndex) !== undefined) return;
            if (!wholePickEnabled && !(lockedClickClears && meshReachable)) return;
            event.stopPropagation();
            onInstancePointerDown(instance.id, index, event);
          }}
          onClick={(event) => {
            if (!instancePickEnabled || !targets.face) return;
            const faceId = world3dGlbHitFaceId(event.object, event.faceIndex);
            if (faceId === undefined) return;
            event.stopPropagation();
            onWorldPick({ granularity: "face", id: faceId, merge: mergeMode(event), objectId: instance.id });
          }}
          onPointerMove={(event) => {
            if (instancePickEnabled && targets.face) {
              const faceId = world3dGlbHitFaceId(event.object, event.faceIndex);
              if (faceId !== undefined) {
                event.stopPropagation();
                onComponentHover({ objectId: instance.id, mode: "face", id: faceId });
                return;
              }
            }
            if (!wholePickEnabled) return;
            event.stopPropagation();
            onInstancePointerMove(instance.id);
          }}
          onPointerOut={() => {
            onInstancePointerMove(null);
            onComponentHover(null);
          }}
        >
          <Suspense fallback={null}>
            <GlbInstanceMesh
              key={worldMeshMaterialRevision(styleKind)}
              url={meshRecord.url}
              color={glbColor}
              emissive={glbEmissive}
              emissiveIntensity={glbEmissiveIntensity}
              opacity={style.opacity}
              borderColor={environmentMaterial?.stroke ?? palette.neutral.lineColor}
              material={environmentMaterial}
              shadowEnabled={environmentShadowEnabled}
              revision={styleKind}
              pickEnabled={instancePickEnabled && (meshReachable || targets.face === true)}
              subElements={
                targets.face || targets.vertex || selectedFaceIds.size > 0 || selectedVertexIds.size > 0 || hoveredFaceId !== undefined || hoveredVertexId !== undefined
                  ? (mesh) => (
                      <GlbSubElementLayer
                        mesh={mesh}
                        instanceId={instance.id}
                        targets={targets}
                        pickEnabled={instancePickEnabled}
                        meshReachable={meshReachable}
                        hoveredFaceId={hoveredFaceId}
                        hoveredVertexId={hoveredVertexId}
                        selectedFaceIds={selectedFaceIds}
                        selectedVertexIds={selectedVertexIds}
                        subElementPaint={subElementPaint}
                        mergeMode={mergeMode}
                        onWorldPick={onWorldPick}
                        onComponentHover={onComponentHover}
                        onInstancePointerMove={onInstancePointerMove}
                      />
                    )
                  : undefined
              }
            />
          </Suspense>
        </group>
      ) : (
        <mesh
          raycast={worldInstanceMeshRaycast(wholePickEnabled)}
          onPointerDown={(event) => {
            if (!wholePickEnabled && !(lockedClickClears && meshReachable)) return;
            event.stopPropagation();
            onInstancePointerDown(instance.id, index, event);
          }}
        >
          <boxGeometry args={[1, 1, 1]} />
          <meshStandardMaterial key={worldMeshMaterialRevision(styleKind)} color={style.meshColor} metalness={0} roughness={1} emissive={style.meshColor} emissiveIntensity={style.emissiveIntensity} transparent={style.opacity < 1} opacity={style.opacity} />
        </mesh>
      )}
    </group>
  );
});
//#endregion WorldSceneParsing

//#region WorldInstancesLayer
const WORLD_INSTANCE_UNIT_SCALE: readonly [number, number, number] = [1, 1, 1];
/** ✂️ Marks an instance root: the section plane clips the materials under these groups and nothing else. */
const WORLD_INSTANCE_ROOT_USER_DATA = { world3dInstanceRoot: true } as const;
const WORLD_INSTANCE_QUATERNION = new WeakMap<WorldInstanceRecord, Quaternion>();

/** 🪪️ One `Quaternion` per instance record object, so an unchanged record (kept by `advanceWorldInstanceResidency`) hands its
 * memoized `WorldInstanceNode` the same prop instead of a fresh object per render. */
function worldInstanceQuaternion(instance: WorldInstanceRecord): Quaternion | undefined {
  const rotation = instance.rotation;
  if (!rotation) return undefined;
  let quaternion = WORLD_INSTANCE_QUATERNION.get(instance);
  if (!quaternion) {
    quaternion = new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]);
    WORLD_INSTANCE_QUATERNION.set(instance, quaternion);
  }
  return quaternion;
}

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
  // 🧹️ Dispose-on-remove, after the commit that stopped referencing them: a geometry retired during
  // render is still in the mounted scene until React commits the new one.
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
  /** Live drag-preview merged component id set (null when no marquee drag is in progress). */
  readonly mergedComponentIds?: readonly number[] | null;
  /** Live drag-preview merged whole-instance id set (null when no marquee drag is in progress). */
  readonly mergedInstanceIds?: readonly string[] | null;
  /** Disables instance picking; passed for brush engagements so a click meant for a vortex marker can't fall through and select/gumball the underlying object instead. */
  readonly blockPick?: boolean;
  readonly environment?: WorldEnvironmentRecord | null;
  readonly subElementPaint: World3dSubElementPaint;
}) {
  const meshById = useMemo(() => new Map(meshes.map((mesh) => [mesh.id, mesh])), [meshes]);
  // 🖱️ The component (vertex/edge/face) under the pointer, painted from THIS pane's own raycast the
  // moment the pointer reaches it. The guest echoes the same hover back in its selection JSON
  // (`hoveredComponent`) a few turns later; until then — and for a guest that never echoes it — the
  // local hit is what highlights. Cleared on pointer-out and on every object-level hover, so a stale
  // face highlight never outlives the pointer leaving the mesh (lowpoly, 2026-09-18).
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
  // 🧊️ Per-mesh-id visuals, keyed on the mesh RECORD's identity (`advanceWorldMeshResidency` keeps
  // that identity across a refresh that did not touch the mesh), so an N-mesh surface with one moved
  // mesh allocates one `BufferGeometry` instead of N — and every retired one is disposed.
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
  // Still-selected (solid) = current ∩ merged when dragging; newly-added (preview tint) = merged − current.
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
  // 🎯️ The raycast result paints BEFORE it is dispatched: the hover chrome follows the pointer at
  // raycast speed and the guest's echo (`selection.hoveredId`, one round trip later) only confirms it
  // — see `worldHoverPaintIdV1`. The claim is withdrawn when the pointer leaves the canvas
  // (`onLocalHoverRelease`), so a hover another surface authors shows once the pointer is elsewhere.
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

  // 🪪️ Stable across renders, like every other `WorldInstanceNode` prop: an inline function here defeated the node's memo, so
  // every refresh re-rendered every instance of the document even when one instance changed.
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

//#region WorldPointCloudLayer
/** ☁️ Renders `World3dScene.pointsJson` layers as GPU point sprites — decodes each layer's base64
 * position/color buffers into a `BufferGeometry` and draws it with a `PointsMaterial`, mounted
 * alongside `🗺️WorldTerrainLayer` in the `World3dHost` scene tree. */
type WorldPointCloudLayerVisual = { readonly geometry: BufferGeometry; readonly material: PointsMaterial };

function pointCloudLayerVisual(layer: WorldPointCloudLayerRecord): WorldPointCloudLayerVisual {
  const geometry = new BufferGeometry();
  const positionBytes = base64ToBytes(layer.positionsB64);
  const positions = new Float32Array(positionBytes.buffer, positionBytes.byteOffset, positionBytes.byteLength / Float32Array.BYTES_PER_ELEMENT);
  geometry.setAttribute("position", new BufferAttribute(positions, 3));
  const hasColors = Boolean(layer.colorsB64);
  if (layer.colorsB64) geometry.setAttribute("color", new BufferAttribute(base64ToBytes(layer.colorsB64), 3, true));
  const material = new PointsMaterial({ size: layer.size, sizeAttenuation: layer.sizeAttenuation, vertexColors: hasColors });
  return { geometry, material };
}

export function WorldPointCloudLayer({ pointsJson }: { readonly pointsJson: string | undefined }) {
  const layers = useMemo(() => parseJsonArray<WorldPointCloudLayerRecord>(pointsJson), [pointsJson]);
  const visuals = useMemo(() => {
    const map = new Map<string, WorldPointCloudLayerVisual>();
    for (const layer of layers) map.set(layer.id, pointCloudLayerVisual(layer));
    return map;
  }, [layers]);

  useEffect(() => {
    return () => {
      for (const visual of visuals.values()) {
        visual.geometry.dispose();
        visual.material.dispose();
      }
    };
  }, [visuals]);

  if (layers.length === 0) return null;

  return (
    <group>
      {layers.map((layer) => {
        const visual = visuals.get(layer.id);
        if (!visual) return null;
        return <points key={layer.id} geometry={visual.geometry} material={visual.material} />;
      })}
    </group>
  );
}
//#endregion WorldPointCloudLayer

//#region WorldVortexMarkers
export function worldVortexMaterialRevision(selected?: boolean, hovered?: boolean): "selected" | "hovered" | "neutral" {
  return selected ? "selected" : hovered ? "hovered" : "neutral";
}

const WORLD_VORTEX_Y_AXIS = new Vector3(0, 1, 0);
const WORLD_VORTEX_DIRECTION_FALLBACK: readonly [number, number, number] = [0, 0, -1];

function worldVortexUnitDirection(direction?: readonly [number, number, number]): Vector3 {
  const vector = new Vector3(...(direction ?? WORLD_VORTEX_DIRECTION_FALLBACK));
  if (vector.lengthSq() < 1e-12) {
    return new Vector3(...WORLD_VORTEX_DIRECTION_FALLBACK);
  }
  return vector.normalize();
}

function worldVortexArrowLayout(
  position: readonly [number, number, number],
  direction: readonly [number, number, number] | undefined,
  radius: number,
  displayDirection: "outwards" | "inwards",
): {
  readonly pointRadius: number;
  readonly shaftRadius: number;
  readonly shaftLength: number;
  readonly headLength: number;
  readonly shaftCenter: [number, number, number];
  readonly headCenter: [number, number, number];
  readonly quaternion: Quaternion;
} {
  const dir = worldVortexUnitDirection(direction);
  const arrowLength = radius;
  const headLength = radius * 0.28;
  const shaftLength = Math.max(arrowLength - headLength, radius * 0.2);
  const shaftRadius = radius * 0.055;
  const pointRadius = radius * 0.18;
  const outward = displayDirection !== "inwards";
  const shaftCenter = new Vector3(...position).addScaledVector(dir, outward ? shaftLength * 0.5 : -(headLength + shaftLength * 0.5));
  const headCenter = new Vector3(...position).addScaledVector(dir, outward ? arrowLength - headLength * 0.5 : -headLength * 0.5);
  return {
    pointRadius,
    shaftRadius,
    shaftLength,
    headLength,
    shaftCenter: shaftCenter.toArray() as [number, number, number],
    headCenter: headCenter.toArray() as [number, number, number],
    quaternion: new Quaternion().setFromUnitVectors(WORLD_VORTEX_Y_AXIS, dir),
  };
}

export function WorldVortexMarkers({
  vortices,
  palette,
  brushMode,
  selectionMode,
  connectSourceFullId,
  onHover,
  onVortexSelect,
  onBrushPlace,
  onVortexPointerArm,
  onVortexPointerMove,
  onVortexPointerUp,
  onConnectDragHover,
  onConnectDragDrop,
}: {
  readonly vortices: readonly WorldVortexRecord[];
  readonly palette: MeshStylePalette;
  readonly brushMode: boolean;
  readonly selectionMode?: string;
  readonly connectSourceFullId?: string;
  readonly onHover: (fullId: string | null) => void;
  readonly onVortexSelect: (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => void;
  readonly onBrushPlace: () => void;
  readonly onVortexPointerArm: (args: {
    readonly fullId: string;
    readonly position: readonly [number, number, number];
    readonly clientX: number;
    readonly clientY: number;
    readonly event: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean };
  }) => void;
  readonly onVortexPointerMove: (fullId: string, clientX: number, clientY: number) => void;
  readonly onVortexPointerUp: (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => void;
  readonly onConnectDragHover: (position: readonly [number, number, number]) => void;
  readonly onConnectDragDrop: (fullId: string, event?: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => void;
}) {
  if (!vortices.length) return null;
  return (
    <group>
      {vortices.map((vortex) => {
        const radius = vortex.radius ?? 0.36;
        const displayDirection = vortex.displayDirection ?? "outwards";
        const layout = worldVortexArrowLayout(vortex.position, vortex.direction, radius, displayDirection);
        const isConnectSource = connectSourceFullId === vortex.fullId;
        const style = vortex.selected ? palette.selected : vortex.hovered ? palette.hovered : null;
        const color = isConnectSource ? "#f59e0b" : (style?.meshColor ?? vortex.color ?? "#38bdf8");
        const materialKey = worldVortexMaterialRevision(vortex.selected, vortex.hovered);
        const materialProps = {
          color,
          emissive: style?.meshColor ?? "#000000",
          emissiveIntensity: style?.emissiveIntensity ?? 0,
          transparent: true,
          opacity: 0.88,
        };
        const pointerHandlers = {
          onPointerOver: (event: { stopPropagation: () => void }) => {
            event.stopPropagation();
            onHover(vortex.fullId);
            if (connectSourceFullId) onConnectDragHover(vortex.position);
          },
          onPointerOut: (event: { stopPropagation: () => void }) => {
            event.stopPropagation();
            onHover(null);
          },
          onPointerDown: (event: { stopPropagation: () => void; button?: number; clientX: number; clientY: number; shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
            // 🖱️ Ignore right/middle — otherwise opening a context/suggestion menu arms connect-drag
            // and the portaled menu click never delivers host pointer-up to cancel it.
            if (event.button != null && event.button !== 0) return;
            event.stopPropagation();
            if (resolveVortexPointerDownIntent(brushMode, selectionMode) === "select") {
              onVortexSelect(vortex.fullId, event);
              if (brushMode) onBrushPlace();
              return;
            }
            onVortexPointerArm({
              fullId: vortex.fullId,
              position: vortex.position,
              clientX: event.clientX,
              clientY: event.clientY,
              event,
            });
          },
          onPointerMove: (event: { stopPropagation: () => void; clientX: number; clientY: number }) => {
            if (brushMode) return;
            event.stopPropagation();
            onVortexPointerMove(vortex.fullId, event.clientX, event.clientY);
            if (connectSourceFullId) onConnectDragHover(vortex.position);
          },
          onPointerUp: (event: { stopPropagation: () => void; shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => {
            if (brushMode) return;
            if (connectSourceFullId) {
              event.stopPropagation();
              onConnectDragDrop(vortex.fullId, event);
              return;
            }
            event.stopPropagation();
            onVortexPointerUp(vortex.fullId, event);
          },
          onClick: (event: { stopPropagation: () => void }) => {
            event.stopPropagation();
            if (brushMode) onBrushPlace();
          },
        };
        return (
          <group key={vortex.fullId}>
            <mesh position={vortex.position as [number, number, number]} visible={worldVortexHitProxy(radius).visible} {...pointerHandlers}>
              <sphereGeometry args={[worldVortexHitProxy(radius).radius, 16, 16]} />
              <meshBasicMaterial transparent opacity={0} depthWrite={false} depthTest={false} />
            </mesh>
            <mesh position={vortex.position as [number, number, number]}>
              <sphereGeometry args={[layout.pointRadius, 12, 12]} />
              <meshStandardMaterial key={materialKey} {...materialProps} />
            </mesh>
            <mesh position={layout.shaftCenter} quaternion={layout.quaternion}>
              <cylinderGeometry args={[layout.shaftRadius, layout.shaftRadius, layout.shaftLength, 10]} />
              <meshStandardMaterial key={materialKey} {...materialProps} />
            </mesh>
            <mesh position={layout.headCenter} quaternion={layout.quaternion}>
              <coneGeometry args={[layout.shaftRadius * 1.8, layout.headLength, 12]} />
              <meshStandardMaterial key={materialKey} {...materialProps} />
            </mesh>
          </group>
        );
      })}
    </group>
  );
}
//#endregion WorldVortexMarkers

//#region WorldInstanceChromeStore
type WorldInstanceChromeSnapshot = {
  readonly selectedIds: ReadonlySet<string>;
  readonly hoveredId: string | null;
  readonly hoveredKindId: string | null;
  readonly previewInstanceIds: ReadonlySet<string> | null;
};

/** 🎯️ What THIS pane's own raycast has under the pointer: an instance id, `null` for "inside the
 * pane, over nothing", `undefined` for "the pointer is not in this pane" (no local claim). */
export type WorldLocalHoverV1 = string | null | undefined;

/** 🎯️ The instance the hover chrome paints. The pane's own raycast LEADS while the pointer is inside
 * it — that is the value the guest is being told about, one round trip (hundreds of milliseconds
 * through `interactionHover` → reserved job → publication → refresh → commit) before the guest's
 * `hoveredId` echoes it back — so the highlight follows the pointer at raycast speed and the echo
 * only ever confirms it. The guest's lane leads when the pointer is elsewhere: a hover authored by
 * another surface (an outliner row, a remote presence) has nothing local to yield to.
 *
 * 🧭️ Nothing about the guest changes: the same `interactionHover` still reaches it, the same lane
 * comes back, the same selection semantics apply to a click. Only what is PAINTED between the raycast
 * and the echo differs, and it differs toward what the guest will answer. */
export function worldHoverPaintIdV1(local: WorldLocalHoverV1, guest: string | null): string | null {
  return local === undefined ? guest : local;
}

interface WorldInstanceChromeStore {
  subscribe: (listener: () => void) => () => void;
  getSnapshot: () => WorldInstanceChromeSnapshot;
  setSnapshot: (next: WorldInstanceChromeSnapshot) => void;
  /** 🎯️ The pane's raycast result, painted at once and reconciled against the guest echo by
   * {@link worldHoverPaintIdV1}. */
  setLocalHover: (local: WorldLocalHoverV1) => void;
  localHover: () => WorldLocalHoverV1;
  isSelected: (instanceId: string) => boolean;
  isHovered: (instanceId: string) => boolean;
  isHighlighted: (objectKind: string | undefined) => boolean;
  previewSelected: (instanceId: string) => boolean | undefined;
}

function chromeSnapshotsEqual(a: WorldInstanceChromeSnapshot, b: WorldInstanceChromeSnapshot): boolean {
  if (a.hoveredId !== b.hoveredId || a.hoveredKindId !== b.hoveredKindId) return false;
  if (a.previewInstanceIds !== b.previewInstanceIds) {
    if (!a.previewInstanceIds || !b.previewInstanceIds || a.previewInstanceIds.size !== b.previewInstanceIds.size) return false;
    for (const id of a.previewInstanceIds) {
      if (!b.previewInstanceIds.has(id)) return false;
    }
  }
  if (a.selectedIds.size !== b.selectedIds.size) return false;
  for (const id of a.selectedIds) {
    if (!b.selectedIds.has(id)) return false;
  }
  return true;
}

function createWorldInstanceChromeStore(): WorldInstanceChromeStore {
  let snapshot: WorldInstanceChromeSnapshot = {
    selectedIds: new Set(),
    hoveredId: null,
    hoveredKindId: null,
    previewInstanceIds: null,
  };
  let localHover: WorldLocalHoverV1 = undefined;
  const listeners = new Set<() => void>();
  const notify = (): void => {
    for (const listener of listeners) listener();
  };
  return {
    subscribe(listener) {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
    getSnapshot() {
      return snapshot;
    },
    setSnapshot(next) {
      if (chromeSnapshotsEqual(snapshot, next)) return;
      snapshot = next;
      notify();
    },
    setLocalHover(local) {
      if (local === localHover) return;
      const paintedBefore = worldHoverPaintIdV1(localHover, snapshot.hoveredId);
      localHover = local;
      if (worldHoverPaintIdV1(localHover, snapshot.hoveredId) === paintedBefore) return;
      notify();
    },
    localHover() {
      return localHover;
    },
    isSelected(instanceId) {
      return snapshot.selectedIds.has(instanceId);
    },
    isHovered(instanceId) {
      return worldHoverPaintIdV1(localHover, snapshot.hoveredId) === instanceId;
    },
    isHighlighted(objectKind) {
      return objectKind != null && snapshot.hoveredKindId != null && objectKind === snapshot.hoveredKindId;
    },
    previewSelected(instanceId) {
      if (!snapshot.previewInstanceIds) return undefined;
      return snapshot.previewInstanceIds.has(instanceId);
    },
  };
}

const WorldInstanceChromeContext = reactHostPort.createContext<WorldInstanceChromeStore | null>(null);

function useWorldInstanceChromeStore(): WorldInstanceChromeStore {
  const store = reactHostPort.useContext(WorldInstanceChromeContext);
  if (!store) throw new Error("World instance chrome hooks must render inside WorldInstancesLayer");
  return store;
}

function useWorldInstanceChrome(
  instanceId: string,
  objectKind: string | undefined,
): {
  readonly selected: boolean;
  readonly hovered: boolean;
  readonly highlighted: boolean;
  readonly previewSelected: boolean | undefined;
} {
  const store = useWorldInstanceChromeStore();
  const selected = reactHostPort.useSyncExternalStore(store.subscribe, () => store.isSelected(instanceId), () => false);
  const hovered = reactHostPort.useSyncExternalStore(store.subscribe, () => store.isHovered(instanceId), () => false);
  const highlighted = reactHostPort.useSyncExternalStore(store.subscribe, () => store.isHighlighted(objectKind), () => false);
  const previewSelected = reactHostPort.useSyncExternalStore(store.subscribe, () => store.previewSelected(instanceId), () => undefined);
  return { selected, hovered, highlighted, previewSelected };
}
//#endregion WorldInstanceChromeStore
//#endregion 🔖️World3dHost
