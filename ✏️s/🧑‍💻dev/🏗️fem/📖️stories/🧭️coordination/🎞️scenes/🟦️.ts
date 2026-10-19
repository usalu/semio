import type { Viewport2d } from "@semio-tech/framework-ui-viewport-ts/2d/schema";
import type { Viewport3dOrbit } from "@semio-tech/framework-ui-viewport-ts/3d/schema";
import type { Fem2dSnapshot, Fem3dSnapshot, FemVec2 } from "../📸️snapshot-projection/🟦️.ts";
import { encodeScopedIdentity, type SelectionSpec } from "@semio-tech/framework/interaction";
import type { DomainHover, DomainSelection, DomainTopology } from "../../../../../../🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🟦️.ts";

//#region 🔖️Fem2dScene
/** 📐️ `SCALE_2D` — model metres to screen pixels (`🪟️windows/🧱️model/🦀️.rs`). */
const FEM2D_SCALE = 20;

/** 📐️ `ORIGIN_2D` — screen-space offset so a structure anchored at (0,0) is not drawn at the canvas corner. */
const FEM2D_ORIGIN = 40;

/** 📐️ Port of `screen_2d`: model coordinates to the canvas' own screen space (y flipped). */
export function femScreen2d(x: number, y: number): FemVec2 {
  return [x * FEM2D_SCALE + FEM2D_ORIGIN, -y * FEM2D_SCALE + FEM2D_ORIGIN];
}

type FemLayer = Record<string, unknown>;

/** ➡️ Port of `vector_layer`: a two-point polyline from `origin` along `vector`, y negated. */
function femVectorLayer(id: string, origin: FemVec2, vector: FemVec2, color: string): FemLayer {
  return { kind: "polyline", id, points: [[origin[0], origin[1]], [origin[0] + vector[0], origin[1] - vector[1]]], color };
}

function femSign(value: number): number {
  return value < 0 ? -1 : 1;
}

/**
 * 🖼️ Port of `fem2d_structure_layers`: one circle per node, one line per member, one circle per
 * support and one red vector per load, in exactly the Rust order and with the Rust's own ids
 * (`node-<id>` / `el-<id>` / `support-<id>` / `load-<id>`). The three colors are the caller's, matching the
 * two Rust call sites: bright (`#38bdf8`/`#94a3b8`/`#f97316`) for the model window, a single muted
 * `#334155` for the results window's undeformed backdrop.
 */
export function fem2dStructureLayers(snapshot: Fem2dSnapshot, nodeColor: string, lineColor: string, supportColor: string): readonly FemLayer[] {
  const layers: FemLayer[] = [];
  const nodeById = new Map(snapshot.nodes.map((node) => [node.id, node]));
  for (const node of snapshot.nodes) {
    const [sx, sy] = femScreen2d(node.x, node.y);
    layers.push({ kind: "circle", id: `node-${node.id}`, x: sx - 4, y: sy - 4, width: 8, height: 8, color: nodeColor });
  }
  for (const element of snapshot.elements) {
    const start = nodeById.get(element.start);
    const end = nodeById.get(element.end);
    if (!start || !end) continue;
    const [x0, y0] = femScreen2d(start.x, start.y);
    const [x1, y1] = femScreen2d(end.x, end.y);
    layers.push({ kind: "line", id: `el-${element.id}`, x0, y0, x1, y1, color: lineColor });
  }
  for (const support of snapshot.supports) {
    const node = nodeById.get(support.nodeId);
    if (!node) continue;
    const [sx, sy] = femScreen2d(node.x, node.y);
    layers.push({ kind: "circle", id: `support-${support.id}`, x: sx - 5, y: sy - 5, width: 10, height: 10, color: supportColor });
  }
  for (const loadCase of snapshot.loadCases) {
    for (const load of loadCase.loads) {
      if (load.kind === "nodal") {
        const node = nodeById.get(load.nodeId ?? "");
        if (!node) continue;
        const value = load.value ?? 0;
        const vector: FemVec2 = load.dof === "Tx" ? [femSign(value) * 18, 0] : load.dof === "Ty" ? [0, femSign(value) * 18] : [0, -12];
        layers.push(femVectorLayer(`load-${load.id}`, femScreen2d(node.x, node.y), vector, "#ef4444"));
        continue;
      }
      if (load.kind === "memberUdl") {
        const element = snapshot.elements.find((entry) => entry.id === load.elementId);
        const start = element ? nodeById.get(element.start) : undefined;
        const end = element ? nodeById.get(element.end) : undefined;
        if (!start || !end) continue;
        layers.push(femVectorLayer(`load-${load.id}`, femScreen2d((start.x + end.x) * 0.5, (start.y + end.y) * 0.5), [femSign(load.wx ?? 0) * 18, femSign(load.wy ?? 0) * 18], "#ef4444"));
        continue;
      }
      if (load.kind === "area") {
        const region = snapshot.regions.find((entry) => entry.id === load.regionId);
        if (!region || region.outline.length === 0) continue;
        const sum = region.outline.reduce((accumulator, point) => [accumulator[0] + point[0], accumulator[1] + point[1]] as const, [0, 0] as FemVec2);
        const count = region.outline.length;
        layers.push(femVectorLayer(`load-${load.id}`, femScreen2d(sum[0] / count, sum[1] / count), [0, -femSign(load.pressure ?? 0) * 18], "#ef4444"));
      }
    }
  }
  return layers;
}

/** 🎥️ Initial navigation for a newly opened FEM 2D window. */
export const FEM2D_DEFAULT_CAMERA: Viewport2d = { x: 0, y: 0, zoom: 1 };

/** 📐️ The `canvas-2d` scene node a fem2d window renders, projected the way `crate::app_surface::canvas_2d_surface` encodes it — `surfaceId` is the window's own `BODY_KEY`. */
export function buildFem2dSceneNode(layers: readonly FemLayer[], camera: Viewport2d, bodyKey: string, controllerId: string) {
  return {
    type: "componentScene",
    surfaceId: bodyKey,
    controllerId,
    componentKind: "canvas-2d",
    canvas2d: { cameraX: camera.x, cameraY: camera.y, zoom: camera.zoom, layersJson: JSON.stringify(layers) },
  } as const;
}

//#endregion 🔖️Fem2dScene

//#region 🔖️Fem3dScene
/** 🧊️ `NODE_SIZE_3D` / `MEMBER_THICKNESS_3D` (`🗿️artifacts/🧊️3d/…/✏️editor/🦀️.rs`). */
const FEM3D_NODE_SIZE = 0.05;

const FEM3D_MEMBER_THICKNESS = 0.05;

type FemQuat = readonly [number, number, number, number];

type FemVec3 = readonly [number, number, number];

/** 🧭️ Port of `quat_mul`: Hamilton product `a * b`, both `[x,y,z,w]`. */
function quatMul(a: FemQuat, b: FemQuat): FemQuat {
  const [ax, ay, az, aw] = a;
  const [bx, by, bz, bw] = b;
  return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz];
}

/** 🧭️ Port of `quat_roll_z`: `roll` radians about the local +Z axis. */
function quatRollZ(roll: number): FemQuat {
  const half = roll / 2;
  return [0, 0, Math.sin(half), Math.cos(half)];
}

/** 🧭️ Port of `quat_z_to`: shortest-arc rotation taking local +Z (the `"box"` mesh's long axis) onto unit `dir`, with the antiparallel case flipped 180° about X. */
function quatZTo(dir: FemVec3): FemQuat {
  const dot = Math.min(1, Math.max(-1, dir[2]));
  if (dot > 0.999999) return [0, 0, 0, 1];
  if (dot < -0.999999) return [1, 0, 0, 0];
  const axisLength = Math.sqrt(dir[1] * dir[1] + dir[0] * dir[0]);
  const half = Math.acos(dot) / 2;
  const sin = Math.sin(half);
  return [(-dir[1] / axisLength) * sin, (dir[0] / axisLength) * sin, 0, Math.cos(half)];
}

/** 📦️ Port of `mesh_box(1,1,1)` + `compute_normals` (`🧰️framework/🔨️modules/🏗️mesh-engine/🦀️.rs`): six quads as twelve non-indexed triangles, so accumulated normals are flat per face. This is the `"box"` mesh `world3d_meshes_json_from_kinds(&["box"])` puts in every fem3d scene. */
function femUnitBoxMeshData(): { readonly positions: number[]; readonly normals: number[]; readonly indices: number[] } {
  const h = 0.5;
  const faces: readonly (readonly FemVec3[])[] = [
    [[-h, -h, h], [h, -h, h], [h, h, h], [-h, h, h]],
    [[h, -h, -h], [-h, -h, -h], [-h, h, -h], [h, h, -h]],
    [[-h, h, h], [h, h, h], [h, h, -h], [-h, h, -h]],
    [[-h, -h, -h], [h, -h, -h], [h, -h, h], [-h, -h, h]],
    [[h, -h, h], [h, -h, -h], [h, h, -h], [h, h, h]],
    [[-h, -h, -h], [-h, -h, h], [-h, h, h], [-h, h, -h]],
  ];
  const positions: number[] = [];
  const normals: number[] = [];
  const indices: number[] = [];
  const pushTriangle = (a: FemVec3, b: FemVec3, c: FemVec3): void => {
    const base = positions.length / 3;
    positions.push(a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]);
    const e0: FemVec3 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    const e1: FemVec3 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    const raw: FemVec3 = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
    const length = Math.max(Math.hypot(raw[0], raw[1], raw[2]), 1e-8);
    for (let corner = 0; corner < 3; corner += 1) normals.push(raw[0] / length, raw[1] / length, raw[2] / length);
    indices.push(base, base + 1, base + 2);
  };
  for (const [a, b, c, d] of faces) {
    pushTriangle(a!, b!, c!);
    pushTriangle(a!, c!, d!);
  }
  return { positions, normals, indices };
}

export const FEM3D_BOX_MESHES: readonly Record<string, unknown>[] = [{ id: "box", data: femUnitBoxMeshData() }];

/**
 * 🧊️ Port of `fem3d_structural_instances`: one small box per node, then one oriented box prism per
 * `Bar`/`Frame` member — positioned at the midpoint, scaled `[t,t,length]` so the mesh's long local Z axis
 * stretches along the member, rotated by `quat_z_to(dir) * quat_roll_z(roll)`. Ids use the native
 * scoped interaction identity. `displacements` are always absent here (no solver in the browser), so this is
 * always the undeformed structure.
 */
export function fem3dStructuralInstances(snapshot: Fem3dSnapshot): readonly Record<string, unknown>[] {
  const instances: Record<string, unknown>[] = [];
  const nodeById = new Map(snapshot.nodes.map((node) => [node.id, node]));
  for (const node of snapshot.nodes) {
    instances.push({ id: encodeScopedIdentity(["node", node.id]), interactionGranularityId: "node", meshId: "box", position: [node.x, node.y, node.z], rotation: [0, 0, 0, 1], scale: [FEM3D_NODE_SIZE, FEM3D_NODE_SIZE, FEM3D_NODE_SIZE], label: node.id });
  }
  for (const element of snapshot.elements) {
    const start = nodeById.get(element.start);
    const end = nodeById.get(element.end);
    if (!start || !end) continue;
    const delta: FemVec3 = [end.x - start.x, end.y - start.y, end.z - start.z];
    const length = Math.max(Math.hypot(delta[0], delta[1], delta[2]), 1e-9);
    const direction: FemVec3 = [delta[0] / length, delta[1] / length, delta[2] / length];
    const rotation = quatMul(quatZTo(direction), quatRollZ(element.kind === "frame" ? element.roll : 0));
    instances.push({
      id: encodeScopedIdentity(["element", element.id]),
      interactionGranularityId: "element",
      meshId: "box",
      position: [(start.x + end.x) / 2, (start.y + end.y) / 2, (start.z + end.z) / 2],
      rotation,
      scale: [FEM3D_MEMBER_THICKNESS, FEM3D_MEMBER_THICKNESS, length],
      label: element.id,
    });
  }
  return instances;
}

/** 🎥️ Initial navigation for a newly opened FEM 3D window. */
export const FEM3D_INITIAL_VIEWPORT: Viewport3dOrbit = { position: [4, -4, 3], target: [0, 0, 0], zoom: 1 };

/** 🕹️ The native FEM 3D interaction domain and its initial local selection. */
export const FEM3D_INTERACTION_DOMAIN = "fem3d";

export const FEM3D_INITIAL_SELECTION: DomainSelection = { granularity: "node", ids: [] };

export const FEM3D_INITIAL_HOVER: DomainHover = { channel: "pointer", ids: [] };

export const FEM3D_SELECTION_SPEC: SelectionSpec = { modes: ["multiple", "single"], methods: ["pick", "rectangle"], merges: ["replace", "additive", "subtractive", "invertive"], transitive: false, broadcast: true };

const FEM3D_STORY_TOPOLOGIES = new WeakMap<Fem3dSnapshot, { readonly topology: DomainTopology; readonly granularities: ReadonlyMap<string, string> }>();

/** 🪪️ Retains the shared node/member topology for one immutable story document snapshot. */
export function fem3dStoryTopology(snapshot: Fem3dSnapshot) {
  const existing = FEM3D_STORY_TOPOLOGIES.get(snapshot);
  if (existing) return existing;
  const ordered = [...snapshot.nodes.map((row) => ({ id: encodeScopedIdentity(["node", row.id]), granularity: "node" })), ...snapshot.elements.map((row) => ({ id: encodeScopedIdentity(["element", row.id]), granularity: "element" }))];
  const value = { topology: { ordered }, granularities: new Map(ordered.map((row) => [row.id, row.granularity])) };
  FEM3D_STORY_TOPOLOGIES.set(snapshot, value);
  return value;
}

/** 🌞️ `world3d_environment_json(&WorldSunConfig::default())`. */
const FEM3D_ENVIRONMENT_JSON = JSON.stringify({ sun: { enabled: false, azimuth: 45, elevation: 35, intensity: 0.85, color: "#ffffff" } });

/** 🌐️ The `world-3d` scene node a fem3d window renders, projected the way `crate::app_surface::world_3d_surface` encodes it — `surfaceId` is the window's own body key. */
export function buildFem3dSceneNode(instances: readonly Record<string, unknown>[], camera: Viewport3dOrbit, bodyKey: string, controllerId: string, selection: DomainSelection = FEM3D_INITIAL_SELECTION, hover: DomainHover = FEM3D_INITIAL_HOVER) {
  return {
    type: "componentScene",
    surfaceId: bodyKey,
    controllerId,
    componentKind: "world-3d",
    world3d: {
      cameraJson: JSON.stringify(camera),
      meshesJson: JSON.stringify(FEM3D_BOX_MESHES),
      instancesJson: JSON.stringify(instances),
      selectionJson: JSON.stringify({ method: "rectangle", mode: "replace", ids: selection.ids, hoveredId: hover.ids[0] ?? null }),
      domainId: FEM3D_INTERACTION_DOMAIN,
      domainGranularityId: "node",
      environmentJson: FEM3D_ENVIRONMENT_JSON,
      interactionJson: JSON.stringify({ activeUtility: "select" }),
    },
  } as const;
}

//#endregion 🔖️Fem3dScene
