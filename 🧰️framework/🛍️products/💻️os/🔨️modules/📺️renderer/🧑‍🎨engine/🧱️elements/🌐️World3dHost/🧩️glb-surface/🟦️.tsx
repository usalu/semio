/** 🧩️ GLB instance and sub-element layers, collision mesh extraction, the brush mesh registrar and the tool-run trace layer.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | World3dHost}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { publishedPageUrl } from "@semio-tech/framework/assets/published-page";
import React, { useEffect, useLayoutEffect, useMemo } from "react";
import { meshAssetTransportUrl } from "@semio-tech/framework/assets/mesh";
import { MESH_DELIVERY_CATALOG } from "@semio-tech/framework/assets/mesh";
// #region 🔌️Adapters
import {
  BufferAttribute,
  BufferGeometry,
  Color,
  DoubleSide,
  LineSegments,
  Mesh,
  MeshStandardMaterial,
  Object3D,
  ShaderMaterial,
  Texture,
  SRGBColorSpace,
  NoColorSpace,
  Vector3,
  GLTFLoader
} from "@semio-tech/ui-react";
// #endregion 🔌️Adapters
import { world3dGlbSubElementMeshData, type World3dSubElementPaint } from "../📏️modelling/🟦️.tsx";
import { GLB_MESH_FRAME_ROTATION_X, WORLD_MESH_OUTLINE_USER_DATA_KEY } from "@semio-tech/framework/infinite/world/r3f";
import { type WorldEnvironmentMaterialRecord, type WorldMeshData, type WorldSelectionTargets } from "../🔭️scene-schema/🟦️.ts";
import { useFrame, useLoader, useThree } from "../🎨️mesh-render/🟦️.tsx";
import { type MeshStyleKind } from "../🎨️mesh-render/🖌️mesh-style-palette/🟦️.ts";
import { celebrateConicAngleRadians, createCelebratingConicMaterial } from "../🎨️mesh-render/🎊️instance-celebration/🟦️.ts";
import { buildFaceOverlayGeometry, buildVertexOverlayGeometry, buildVertexPickData } from "../🎨️mesh-render/🔺️mesh-geometry-and-appearance/🟦️.ts";
import { applyGlbMeshEdgeBorders, GLB_MESH_LOCAL_BOUNDS, glbMeshFrameCorners, WORLD_VERTEX_DOT_PX } from "../🎨️mesh-render/🪟️glb-bounds-and-styling/🟦️.ts";
import { applyWorldInstanceMeshRaycast } from "../🔗️instance-selection/🟦️.ts";

//#region 🔖️World3dHost
//#region WorldSceneParsing
/** 🎭️ Clones authored materials for neutral styles and applies reversible active selection paint. */
export function cloneWorldGlbSurface(source:Object3D,revision:MeshStyleKind,style:{meshColor:string;emissiveIntensity:number;opacity:number},emissive=style.meshColor):Object3D {
  const cloned=source.clone(true);const owned=new Set<MeshStandardMaterial|ShaderMaterial>();const retained=new Map<object,MeshStandardMaterial>();const textures=new Map<Texture,Map<string,Texture>>();const geometries=new Set<BufferGeometry>();
  cloned.userData.semioOwnedSurfaceMaterials=owned;cloned.userData.semioOwnedSurfaceGeometries=geometries;
  try {cloned.traverse(child=>{
    if(!(child instanceof Mesh))return;
    if(revision === "neutral" || revision === "disabled"){
      const clone=(material:MeshStandardMaterial)=>{
        let result=retained.get(material);
        if(!result){
          result=material.clone();retained.set(material,result);owned.add(result);
          for(const role of ["map","metalnessMap","roughnessMap","normalMap","aoMap","emissiveMap"] as const){
            const texture=material[role];if(!texture)continue;
            const colorSpace=role === "map" || role === "emissiveMap" ? SRGBColorSpace : NoColorSpace;
            let roles=textures.get(texture);if(!roles){roles=new Map();textures.set(texture,roles);}
            let mapped=roles.get(colorSpace);if(!mapped){mapped=texture.clone();mapped.colorSpace=colorSpace;mapped.needsUpdate=true;roles.set(colorSpace,mapped);}result[role]=mapped;
          }
          const extra=[...new Set(["map","metalnessMap","roughnessMap","normalMap","aoMap","emissiveMap"].map(role=>(result as unknown as Record<string,Texture|null>)[role]?.channel ?? 0).filter(set=>set>3))].sort((a,b)=>a-b);
          if(extra.length){result.onBeforeCompile=shader=>{shader.vertexShader=extra.map(set=>`attribute vec2 uv${set};`).join("\n")+"\n"+shader.vertexShader;};result.customProgramCacheKey=()=>`surface-uv:${extra.join(",")}`;}
        }
        return result;
      };
      child.material=Array.isArray(child.material) ? child.material.map(clone) : clone(child.material as MeshStandardMaterial);
      const coordinates=[...new Set((Array.isArray(child.material)?child.material:[child.material]).flatMap(material=>["map","metalnessMap","roughnessMap","normalMap","aoMap","emissiveMap"].flatMap(role=>{const texture=(material as unknown as Record<string,Texture|null>)[role];return texture?[texture.channel]:[];})))];
      const missing=coordinates.filter(set=>!child.geometry.hasAttribute(set?`uv${set}`:"uv"));
      if(missing.length){const geometry=child.geometry.clone();for(const set of missing){const attribute=geometry.getAttribute(`texcoord_${set}`);if(!attribute){geometry.dispose();throw new Error("GLB texture references an absent UV set");}geometry.setAttribute(set?`uv${set}`:"uv",attribute);}child.geometry=geometry;geometries.add(geometry);}

    }else{
      const material=revision === "celebrated" ? createCelebratingConicMaterial(style.opacity) : new MeshStandardMaterial({color:new Color(style.meshColor),emissive:new Color(emissive),emissiveIntensity:style.emissiveIntensity,metalness:0,roughness:1,transparent:style.opacity<1,opacity:style.opacity});
      child.material=material;owned.add(material);
    }
  });}catch(error){cloned.userData.semioOwnedSurfaceTextures=new Set([...textures.values()].flatMap(roles=>[...roles.values()]));disposeWorldGlbSurface(cloned);throw error;}
  cloned.userData.semioOwnedSurfaceTextures=new Set([...textures.values()].flatMap(roles=>[...roles.values()]));
  return cloned;
}

/** 🧹️ Releases clone-owned material/chrome resources while preserving loader-owned geometry and textures. */
export function disposeWorldGlbSurface(scene:Object3D):void {
  const materials=scene.userData.semioOwnedSurfaceMaterials as Set<MeshStandardMaterial|ShaderMaterial>|undefined;
  if(!materials)return;for(const material of materials)material.dispose();materials.clear();delete scene.userData.semioOwnedSurfaceMaterials;
  const geometries=scene.userData.semioOwnedSurfaceGeometries as Set<BufferGeometry>|undefined;for(const geometry of geometries??[])geometry.dispose();geometries?.clear();delete scene.userData.semioOwnedSurfaceGeometries;
  const textures=scene.userData.semioOwnedSurfaceTextures as Set<Texture>|undefined;for(const texture of textures??[])texture.dispose();textures?.clear();delete scene.userData.semioOwnedSurfaceTextures;
  scene.traverse(child=>{if(child instanceof LineSegments && child.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY]){const material=child.material;for(const owned of Array.isArray(material)?material:[material])owned.dispose();}});
}

export function GlbInstanceMesh({
  url,
  color,
  emissive,
  emissiveIntensity,
  opacity,
  borderColor,
  material,
  shadowEnabled,
  revision,
  pickEnabled,
  subElements,
}: {
  readonly url: string;
  readonly color: string;
  readonly emissive: string;
  readonly emissiveIntensity: number;
  readonly opacity: number;
  readonly borderColor: string;
  readonly material?: WorldEnvironmentMaterialRecord;
  readonly shadowEnabled?: boolean;
  readonly revision: MeshStyleKind;
  readonly pickEnabled: boolean;
  /** 🪪️ Receives the merged picking mesh with the GLB's own `_FACE_ID`/`_VERTEX_ID` tables (`null` without triangles) and renders sub-element overlays in the GLB frame. */
  readonly subElements?: (mesh: WorldMeshData | null) => React.ReactNode;
}) {
  const gltf = useLoader(GLTFLoader, publishedPageUrl(meshAssetTransportUrl(url, MESH_DELIVERY_CATALOG)));
  const subMesh = useMemo(() => {
    if (!subElements) return null;
    const data = world3dGlbSubElementMeshData(gltf.scene);
    return data ? ({ normals: [], ...data } as WorldMeshData) : null;
  }, [gltf.scene, subElements !== undefined]);
  const invalidate = useThree((state) => state.invalidate);
  if (!GLB_MESH_LOCAL_BOUNDS.has(url)) {
    const corners = glbMeshFrameCorners(gltf.scene);
    if (corners.length === 8) GLB_MESH_LOCAL_BOUNDS.set(url, corners);
  }
  const celebrating = revision === "celebrated";
  const scene = useMemo(() => {
    const cloned=cloneWorldGlbSurface(gltf.scene,revision,{meshColor:color,emissiveIntensity,opacity},emissive);
    cloned.traverse(child=>{if(child instanceof Mesh){child.castShadow=shadowEnabled===true;child.receiveShadow=shadowEnabled===true;}});
    applyGlbMeshEdgeBorders(cloned, borderColor);
    return cloned;
  }, [gltf.scene, material?.metalness, material?.roughness, shadowEnabled, color, emissive, emissiveIntensity, opacity, borderColor, revision, celebrating]);
  useEffect(()=>()=>disposeWorldGlbSurface(scene),[scene]);
  useFrame(() => {
    if (!celebrating) return;
    const angle = celebrateConicAngleRadians();
    scene.traverse((child) => {
      if (!(child instanceof Mesh)) return;
      const mat = child.material;
      if (!(mat instanceof ShaderMaterial) || !mat.uniforms?.uAngle) return;
      mat.uniforms.uAngle.value = angle;
    });
    invalidate();
  });
  useLayoutEffect(() => {
    invalidate();
  }, [invalidate, scene]);
  useLayoutEffect(() => {
    applyWorldInstanceMeshRaycast(scene, pickEnabled, Mesh.prototype.raycast);
  }, [scene, pickEnabled]);

  return (
    <group rotation={[GLB_MESH_FRAME_ROTATION_X, 0, 0]}>
      <primitive object={scene} />
      {subElements?.(subMesh)}
    </group>
  );
}

/** 🪪️ Hover, selection and vertex picking of the topology ids a GLB carries (`_FACE_ID`, `_VERTEX_ID`), drawn in the GLB frame with the same paint table as inline meshes. */
export function GlbSubElementLayer({
  mesh,
  instanceId,
  targets,
  pickEnabled,
  meshReachable,
  hoveredFaceId,
  hoveredVertexId,
  selectedFaceIds,
  selectedVertexIds,
  subElementPaint,
  mergeMode,
  onWorldPick,
  onComponentHover,
  onInstancePointerMove,
}: {
  readonly mesh: WorldMeshData | null;
  readonly instanceId: string;
  readonly targets: WorldSelectionTargets;
  readonly pickEnabled: boolean;
  readonly meshReachable: boolean;
  readonly hoveredFaceId?: number;
  readonly hoveredVertexId?: number;
  readonly selectedFaceIds: ReadonlySet<number>;
  readonly selectedVertexIds: ReadonlySet<number>;
  readonly subElementPaint: World3dSubElementPaint;
  readonly mergeMode: (event: { shiftKey?: boolean; ctrlKey?: boolean; metaKey?: boolean }) => string;
  readonly onWorldPick: (args: { granularity: string; id: number; merge: string; objectId?: string }) => void;
  readonly onComponentHover: (args: { objectId: string; mode: string; id: number } | null) => void;
  readonly onInstancePointerMove: (id: string | null) => void;
}) {
  const vertexPick = useMemo(() => (mesh?.vertexIds ? buildVertexPickData(mesh) : null), [mesh]);
  useEffect(() => () => vertexPick?.geometry.dispose(), [vertexPick]);
  const hoveredFaces = useMemo(() => (hoveredFaceId === undefined ? null : new Set([hoveredFaceId])), [hoveredFaceId]);
  const hoveredVertices = useMemo(() => (hoveredVertexId === undefined ? null : new Set([hoveredVertexId])), [hoveredVertexId]);
  const faceSelected = useMemo(() => (mesh ? buildFaceOverlayGeometry(mesh, selectedFaceIds) : null), [mesh, selectedFaceIds]);
  const faceHovered = useMemo(() => (mesh && hoveredFaces ? buildFaceOverlayGeometry(mesh, hoveredFaces) : null), [mesh, hoveredFaces]);
  const vertexSelected = useMemo(() => (mesh ? buildVertexOverlayGeometry(mesh, selectedVertexIds) : null), [mesh, selectedVertexIds]);
  const vertexHovered = useMemo(() => (mesh && hoveredVertices ? buildVertexOverlayGeometry(mesh, hoveredVertices) : null), [mesh, hoveredVertices]);
  useEffect(() => () => faceSelected?.dispose(), [faceSelected]);
  useEffect(() => () => faceHovered?.dispose(), [faceHovered]);
  useEffect(() => () => vertexSelected?.dispose(), [vertexSelected]);
  useEffect(() => () => vertexHovered?.dispose(), [vertexHovered]);
  return (
    <>
      {targets.vertex && vertexPick ? (
        <points
          geometry={vertexPick.geometry}
          onClick={(event) => {
            if (!pickEnabled) return;
            event.stopPropagation();
            const vertexId = vertexPick.vertexIds[event.index ?? 0];
            if (vertexId != null) onWorldPick({ granularity: "vertex", id: vertexId, merge: mergeMode(event), objectId: instanceId });
          }}
          onPointerMove={(event) => {
            if (!pickEnabled) return;
            event.stopPropagation();
            const vertexId = vertexPick.vertexIds[event.index ?? 0];
            if (vertexId != null) onComponentHover({ objectId: instanceId, mode: "vertex", id: vertexId });
            else if (meshReachable) onInstancePointerMove(instanceId);
          }}
          onPointerOut={() => {
            onInstancePointerMove(null);
            onComponentHover(null);
          }}
        >
          <pointsMaterial color={subElementPaint.vertexHover} size={WORLD_VERTEX_DOT_PX} sizeAttenuation={false} />
        </points>
      ) : null}
      {faceSelected ? (
        <mesh geometry={faceSelected} raycast={() => null}>
          <meshBasicMaterial color={subElementPaint.faceSelect} transparent opacity={0.62} side={DoubleSide} depthWrite={false} polygonOffset polygonOffsetFactor={-2} />
        </mesh>
      ) : null}
      {faceHovered ? (
        <mesh geometry={faceHovered} raycast={() => null}>
          <meshBasicMaterial color={subElementPaint.faceHover} transparent opacity={0.48} side={DoubleSide} depthWrite={false} polygonOffset polygonOffsetFactor={-3} />
        </mesh>
      ) : null}
      {vertexSelected ? (
        <points geometry={vertexSelected} raycast={() => null}>
          <pointsMaterial color={subElementPaint.vertexSelect} size={subElementPaint.vertexMarkPx} sizeAttenuation={false} depthTest={false} />
        </points>
      ) : null}
      {vertexHovered ? (
        <points geometry={vertexHovered} raycast={() => null}>
          <pointsMaterial color={subElementPaint.vertexHover} size={subElementPaint.vertexMarkPx} sizeAttenuation={false} depthTest={false} />
        </points>
      ) : null}
    </>
  );
}

function extractGlbCollisionMesh(gltf: Awaited<ReturnType<GLTFLoader["loadAsync"]>>): {
  readonly positions: number[];
  readonly indices: number[];
} {
  const frame = new Object3D();
  frame.rotation.x = GLB_MESH_FRAME_ROTATION_X;
  frame.updateMatrixWorld(true);
  const positions: number[] = [];
  const indices: number[] = [];
  let vertexOffset = 0;
  const scratch = new Vector3();
  gltf.scene.updateMatrixWorld(true);
  gltf.scene.traverse((child) => {
    if (!(child instanceof Mesh)) return;
    const geometry = child.geometry;
    const positionAttr = geometry.getAttribute("position");
    if (!positionAttr) return;
    const worldMatrix = frame.matrixWorld.clone().multiply(child.matrixWorld);
    for (let index = 0; index < positionAttr.count; index += 1) {
      scratch.fromBufferAttribute(positionAttr, index).applyMatrix4(worldMatrix);
      positions.push(scratch.x, scratch.y, scratch.z);
    }
    const indexAttr = geometry.index;
    if (indexAttr) {
      for (let index = 0; index < indexAttr.count; index += 1) {
        indices.push(indexAttr.getX(index) + vertexOffset);
      }
    } else {
      for (let index = 0; index < positionAttr.count; index += 3) {
        indices.push(vertexOffset + index, vertexOffset + index + 1, vertexOffset + index + 2);
      }
    }
    vertexOffset += positionAttr.count;
  });
  return { positions, indices };
}

/** 🥽️ Announces one loaded GLB's collision geometry to the guest. `revision` is the ONLY thing that
 * re-announces an already-loaded mesh: the GLB is `useLoader`-cached for the page's lifetime, so without
 * it the announcement happens exactly once per mount and a guest that lost the geometry (a restored
 * actor, whose mesh store is not part of any checkpoint) could never be told again — see
 * {@link Puzzle3dBrushMeshRegistry}. */
export function BrushMeshRegistrar({ url, revision, onRegister }: { readonly url: string; readonly revision: number; readonly onRegister: (url: string, positions: number[], indices: number[]) => void }) {
  const gltf = useLoader(GLTFLoader, publishedPageUrl(meshAssetTransportUrl(url, MESH_DELIVERY_CATALOG)));
  useEffect(() => {
    const mesh = extractGlbCollisionMesh(gltf);
    if (mesh.positions.length === 0 || mesh.indices.length === 0) return;
    onRegister(url, mesh.positions, mesh.indices);
  }, [gltf, onRegister, revision, url]);
  return null;
}

/** 🥽️ One loaded GLB as the single welded geometry a tool run trace instance draws through, already in the
 * {@link GLB_MESH_FRAME_ROTATION_X} frame the instance group applies. */
export function ToolRunTraceGlbGeometry({ url, onGeometry }: { readonly url: string; readonly onGeometry: (url: string, geometry: BufferGeometry | null) => void }) {
  const gltf = useLoader(GLTFLoader, publishedPageUrl(meshAssetTransportUrl(url, MESH_DELIVERY_CATALOG)));
  useEffect(() => {
    const mesh = extractGlbCollisionMesh(gltf);
    if (mesh.positions.length === 0 || mesh.indices.length === 0) return;
    const geometry = new BufferGeometry();
    geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.positions), 3));
    geometry.setIndex(mesh.indices);
    geometry.computeVertexNormals();
    onGeometry(url, geometry);
    return () => {
      onGeometry(url, null);
      geometry.dispose();
    };
  }, [gltf, onGeometry, url]);
  return null;
}
//#endregion WorldSceneParsing
//#endregion 🔖️World3dHost
