/** 🧊️ World3d instance layers, instance node: the per-instance node that paints a mesh or primitive with its pose, paint ids and highlight, and the cached orientation quaternion of an instance. */
import { useCallback, useMemo, Suspense, type ComponentProps } from "react";
import { reactHostPort, BufferGeometry, DoubleSide, EdgesGeometry, Group, Quaternion } from "@semio-tech/ui-react";
import { ToolRunProvisionalOutline, useToolRunProvisional } from "../../⏯️tool-run-trace/🟦️.tsx";
import { world3dGlbHitFaceId, type World3dSubElementPaint } from "../../📏️modelling/🟦️.tsx";
import { runtimeDiagnosticsEnabled } from "../../../🏛️ShellHost/🔌️extension-runtime/🟦️.ts";
import { type WorldEnvironmentMaterialRecord, type WorldHoverComponent, type WorldInstanceRecord, type WorldMeshData, type WorldMeshRecord, type WorldSelectionTargets } from "../../🔭️scene-schema/🟦️.ts";
import { PaintTexturedMesh } from "../../🎨️mesh-render/🟦️.tsx";
import { type MeshStylePalette, resolveMeshSelectionPreviewStyle, semanticColorsFromPalette, worldInstanceHighlighted, worldMeshMaterialRevision } from "../../🎨️mesh-render/🖌️mesh-style-palette/🟦️.ts";
import { useCelebratingWorldInstanceIds } from "../../🎨️mesh-render/🎊️instance-celebration/🟦️.ts";
import { buildEdgeOverlayGeometry, buildFaceOverlayGeometry, buildVertexOverlayGeometry, faceExtentFromMesh, isCurveOnlyWorldMesh, type VertexPickData, type WorldMeshAppearance } from "../../🎨️mesh-render/🔺️mesh-geometry-and-appearance/🟦️.ts";
import { WORLD_VERTEX_DOT_PX } from "../../🎨️mesh-render/🪟️glb-bounds-and-styling/🟦️.ts";
import { GlbInstanceMesh, GlbSubElementLayer } from "../../🧩️glb-surface/🟦️.tsx";
import { worldInstanceMeshRaycast } from "../../🔗️instance-selection/🟦️.ts";
import { type WorldPaintOrigin } from "../../🧮️gumball-math/🟦️.ts";
import { useWorldInstanceChrome } from "../🪟️instance-chrome-store/🟦️.ts";

//#region 🔖️World3dHost
//#region WorldSceneParsing
export const WorldInstanceNode = reactHostPort.memo(function WorldInstanceNode({
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
      if (runtimeDiagnosticsEnabled()) {}
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
export const WORLD_INSTANCE_UNIT_SCALE: readonly [number, number, number] = [1, 1, 1];
/** ✂️ Marks an instance root: the section plane clips the materials under these groups and nothing else. */
const WORLD_INSTANCE_ROOT_USER_DATA = { world3dInstanceRoot: true } as const;
const WORLD_INSTANCE_QUATERNION = new WeakMap<WorldInstanceRecord, Quaternion>();

/** 🪪️ One `Quaternion` per instance record object, so an unchanged record (kept by `advanceWorldInstanceResidency`) hands its
 * memoized `WorldInstanceNode` the same prop instead of a fresh object per render. */
export function worldInstanceQuaternion(instance: WorldInstanceRecord): Quaternion | undefined {
  const rotation = instance.rotation;
  if (!rotation) return undefined;
  let quaternion = WORLD_INSTANCE_QUATERNION.get(instance);
  if (!quaternion) {
    quaternion = new Quaternion(rotation[0], rotation[1], rotation[2], rotation[3]);
    WORLD_INSTANCE_QUATERNION.set(instance, quaternion);
  }
  return quaternion;
}
//#endregion WorldInstancesLayer
//#endregion 🔖️World3dHost
