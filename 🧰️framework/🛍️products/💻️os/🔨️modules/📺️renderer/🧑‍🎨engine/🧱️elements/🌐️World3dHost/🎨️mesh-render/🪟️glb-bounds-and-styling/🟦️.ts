/** 🪟️ GLB bounds and styling: the paint texture URL and tint constants, the GLB mesh frame corners, the instance local corners, the GLB edge borders and the mesh bounds corners. */
import { Box3, BufferGeometry, Color, EdgesGeometry, LineBasicMaterial, LineSegments, Mesh, Object3D, Vector3 } from "@semio-tech/ui-react";
import { GLB_MESH_FRAME_ROTATION_X, WORLD_MESH_OUTLINE_USER_DATA_KEY } from "@semio-tech/framework/infinite/world/r3f";
import { type WorldMeshData } from "../../🔭️scene-schema/🟦️.ts";

//#region 🔖️World3dHost
//#region WorldSceneParsing
export function paintTextureUrl(base64: string): string {
  return `data:image/png;base64,${base64}`;
}

/** 🌡️ How much of the hover or selection emissive tint a heatmap keeps, so the analysis colours stay readable under it. */
export const WORLD_HEATMAP_TINT_SHARE = 0.5;
/** ✂️ Marks the shaded solid of an instance: the section cap stencils exactly these meshes. */
export const WORLD_SOLID_MESH_USER_DATA = { world3dSolid: true } as const;

//#region GlbMeshBounds
/** 📦️ Local-space extents of every GLB the scene has loaded, keyed by the mesh record's own
 * `url` and already carrying the {@link GLB_MESH_FRAME_ROTATION_X} frame rotation the instance group
 * applies — so an entry is exactly the eight local corners a pick or a marquee has to project.
 *
 * 🎯️ A URL-backed mesh record carries NO inline `data` (the guest publishes `{id, url}` for every
 * catalogue representation), so every screen-space hit test over it used to fall back to a unit cube —
 * or, in the marquee's case, to the single point `[0,0,0]`. Measured on the 180-object Nakagin tower,
 * fully rendered: `candidates=180 containing=0 hit=none boxes=["28x30@427,394", …]` on 20 of 20 clicks
 * spread over the whole pane, while R3F's own raycast against the same GLB geometry resolved an
 * instance on 14 of them (26/09/02/PUZZLE-3D-END-TO-END wave B46). The loader is the only place that
 * knows a GLB's real size, so it is the place that records it. */
export const GLB_MESH_LOCAL_BOUNDS = new Map<string, readonly (readonly [number, number, number])[]>();

/** 📦️ The eight corners of one loaded GLB's frame-rotated AABB, in the instance group's local space. */
export function glbMeshFrameCorners(scene: Object3D): readonly (readonly [number, number, number])[] {
  const frame = new Object3D();
  frame.rotation.x = GLB_MESH_FRAME_ROTATION_X;
  frame.updateMatrixWorld(true);
  scene.updateMatrixWorld(true);
  const box = new Box3().setFromObject(scene);
  if (box.isEmpty()) return [];
  const scratch = new Vector3();
  const rotated = new Box3().makeEmpty();
  for (const corner of [
    [box.min.x, box.min.y, box.min.z],
    [box.max.x, box.min.y, box.min.z],
    [box.min.x, box.max.y, box.min.z],
    [box.max.x, box.max.y, box.min.z],
    [box.min.x, box.min.y, box.max.z],
    [box.max.x, box.min.y, box.max.z],
    [box.min.x, box.max.y, box.max.z],
    [box.max.x, box.max.y, box.max.z],
  ] as const) {
    rotated.expandByPoint(scratch.set(corner[0], corner[1], corner[2]).applyMatrix4(frame.matrixWorld));
  }
  return [
    [rotated.min.x, rotated.min.y, rotated.min.z],
    [rotated.max.x, rotated.min.y, rotated.min.z],
    [rotated.min.x, rotated.max.y, rotated.min.z],
    [rotated.max.x, rotated.max.y, rotated.min.z],
    [rotated.min.x, rotated.min.y, rotated.max.z],
    [rotated.max.x, rotated.min.y, rotated.max.z],
    [rotated.min.x, rotated.max.y, rotated.max.z],
    [rotated.max.x, rotated.max.y, rotated.max.z],
  ];
}

/** 📦️ The local corners a screen-space hit test must use for one instance's mesh: the mesh's own inline
 * geometry when it has some, else the loaded GLB's recorded extents, else — only while a GLB is still
 * loading — the unit cube. */
export function world3dInstanceLocalCorners(
  meshData: WorldMeshData | undefined,
  meshUrl: string | undefined,
  bounds: ReadonlyMap<string, readonly (readonly [number, number, number])[]> = GLB_MESH_LOCAL_BOUNDS,
): readonly (readonly [number, number, number])[] {
  if (meshData && (meshData.positions.length >= 3 || (meshData.edgePositions?.length ?? 0) >= 3)) return meshBoundsCorners(meshData);
  const recorded = meshUrl ? bounds.get(meshUrl) : undefined;
  if (recorded && recorded.length === 8) return recorded;
  return [
    [-0.5, -0.5, -0.5],
    [0.5, -0.5, -0.5],
    [-0.5, 0.5, -0.5],
    [0.5, 0.5, -0.5],
    [-0.5, -0.5, 0.5],
    [0.5, -0.5, 0.5],
    [-0.5, 0.5, 0.5],
    [0.5, 0.5, 0.5],
  ];
}

//#endregion GlbMeshBounds
//#region GlbMeshStyling
/** 🎨️ EdgesGeometry cache keyed by source BufferGeometry — `gltf.scene.clone(true)` shares geometries across every per-instance clone of the same GLB, so this dedupes edge computation across instances. */
const GLB_EDGE_GEOMETRY_CACHE = new WeakMap<BufferGeometry, EdgesGeometry>();

/** 🎨️ Adds a border-color {@link EdgesGeometry} outline to every mesh under `root` (idempotent), using the shared {@link GLB_EDGE_GEOMETRY_CACHE}. */
export function applyGlbMeshEdgeBorders(root: Object3D, borderColor: string): void {
  const targets: Mesh[] = [];
  root.traverse((object) => {
    if (!(object instanceof Mesh)) return;
    const geometry = object.geometry;
    if (!geometry || object.children.some((child) => child.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY])) return;
    targets.push(object);
  });
  for (const object of targets) {
    let edges = GLB_EDGE_GEOMETRY_CACHE.get(object.geometry);
    if (!edges) {
      edges = new EdgesGeometry(object.geometry);
      GLB_EDGE_GEOMETRY_CACHE.set(object.geometry, edges);
    }
    const outline = new LineSegments(edges, new LineBasicMaterial({ color: new Color(borderColor) }));
    outline.userData[WORLD_MESH_OUTLINE_USER_DATA_KEY] = true;
    outline.scale.setScalar(1.001);
    object.add(outline);
  }
}

//#endregion GlbMeshStyling
/** 📍️ Vertex-granularity pick dots and their hover/selection marks, in CSS pixels (screen-space). */
export const WORLD_VERTEX_DOT_PX = 6;

//#endregion WorldSceneParsing
export const WORLD_VERTEX_MARK_PX = 11;

/** 📦️ Local-space AABB corners of a mesh's vertex positions (or edge samples for curve-only meshes; fallback: origin). */
export function meshBoundsCorners(meshData: WorldMeshData): readonly (readonly [number, number, number])[] {
  let minX = Infinity;
  let minY = Infinity;
  let minZ = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  let maxZ = -Infinity;
  const source = meshData.positions.length >= 3 ? meshData.positions : (meshData.edgePositions ?? []);
  for (let index = 0; index < source.length; index += 3) {
    const x = source[index]!;
    const y = source[index + 1]!;
    const z = source[index + 2]!;
    if (x < minX) minX = x;
    if (x > maxX) maxX = x;
    if (y < minY) minY = y;
    if (y > maxY) maxY = y;
    if (z < minZ) minZ = z;
    if (z > maxZ) maxZ = z;
  }
  if (!Number.isFinite(minX)) return [[0, 0, 0]];
  return [
    [minX, minY, minZ],
    [maxX, minY, minZ],
    [minX, maxY, minZ],
    [maxX, maxY, minZ],
    [minX, minY, maxZ],
    [maxX, minY, maxZ],
    [minX, maxY, maxZ],
    [maxX, maxY, maxZ],
  ];
}
//#endregion 🔖️World3dHost
