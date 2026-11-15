/** 🩻️ Integrity of a triangle mesh: is it a closed 2-manifold, how big is it signed, and which way does every face look. The queries the validators of solid models need (volume, exposed area, burial by another solid) and the guard of the tessellation tests.
 *
 * Vertices are welded by position (`toleranceM`, default 0.1 µm) because the tessellator duplicates vertices along creases. Terms:
 * - an edge is *open* when one triangle uses it, *non-manifold* when more than two do; a mesh is *closed* when it has neither;
 * - it is *oriented* when every edge shared by two triangles is walked in opposite directions by them (consistent winding);
 * - a *pinch vertex* is a vertex whose triangles do not form one fan (two cones touching in a point); a mesh is a *closed 2-manifold* when it is closed and has no pinch vertex;
 * - the *signed volume* is the sum of the tetrahedra of the triangles to the origin: positive when a closed, oriented mesh is wound counter-clockwise seen from outside.
 *
 * `outwardNormals` orients the faces consistently (a breadth-first walk over manifold edges) and, per closed component, towards the outside by the sign of its volume; an open component keeps the winding of its seed face, which for the tessellator's solids (open lofts, open sweeps) already is outward. It does not depend on any other mesh, so solids that touch exactly are judged one by one.
 * @see ../🔺️tessellation/🟦️.ts */
import type { Geometry } from "../🔺️tessellation/🧱️builder/🟦️.ts";
import type { Quality, Solid } from "../🧬️schema/🟦️.ts";
import { tessellateSolid } from "../🔺️tessellation/🟦️.ts";
import * as portable from "../🧮️math/🎯️portable/🟦️.ts";

/** 🪡️ The topology and size of a mesh, see the module notes for the terms. */
export type MeshIntegrity = {
  readonly triangles: number;
  readonly degenerate: number;
  readonly vertices: number;
  readonly edges: number;
  readonly openEdges: number;
  readonly nonManifoldEdges: number;
  readonly inconsistentEdges: number;
  readonly pinchVertices: number;
  readonly components: number;
  readonly closed: boolean;
  readonly oriented: boolean;
  readonly manifold: boolean;
  readonly signedVolume: number;
  readonly area: number;
};

/** 🎚️ Welding tolerance in metres; `openWinding` is `-1` when the triangles of the mesh were mirrored after tessellation (a left-handed frame turned the winding of an open piece inside-out, which no volume can tell), default `1`. */
export type IntegrityOptions = { readonly toleranceM?: number; readonly openWinding?: 1 | -1 };

type Topology = {
  readonly welded: Int32Array;
  readonly vertexCount: number;
  readonly live: Uint8Array;
  readonly corners: Int32Array;
  readonly edgeTriangles: Map<number, number[]>;
  readonly degenerate: number;
};

const DEFAULT_TOLERANCE_M = 1e-7;

function topologyOf(positions: ArrayLike<number>, indices: ArrayLike<number>, toleranceM: number): Topology {
  const count = positions.length / 3;
  const welded = new Int32Array(count);
  const seen = new Map<string, number>();
  const q = 1 / toleranceM;
  for (let i = 0; i < count; i++) {
    const key = `${Math.round(positions[i * 3]! * q)},${Math.round(positions[i * 3 + 1]! * q)},${Math.round(positions[i * 3 + 2]! * q)}`;
    let id = seen.get(key);
    if (id === undefined) seen.set(key, (id = seen.size));
    welded[i] = id;
  }
  const triangles = indices.length / 3;
  const live = new Uint8Array(triangles);
  const corners = new Int32Array(triangles * 3);
  const edgeTriangles = new Map<number, number[]>();
  const span = seen.size + 1;
  let degenerate = 0;
  for (let t = 0; t < triangles; t++) {
    const [a, b, c] = [welded[indices[t * 3]!]!, welded[indices[t * 3 + 1]!]!, welded[indices[t * 3 + 2]!]!];
    corners[t * 3] = a;
    corners[t * 3 + 1] = b;
    corners[t * 3 + 2] = c;
    const [pa, pb, pc] = [indices[t * 3]! * 3, indices[t * 3 + 1]! * 3, indices[t * 3 + 2]! * 3];
    const e1 = [positions[pb]! - positions[pa]!, positions[pb + 1]! - positions[pa + 1]!, positions[pb + 2]! - positions[pa + 2]!];
    const e2 = [positions[pc]! - positions[pa]!, positions[pc + 1]! - positions[pa + 1]!, positions[pc + 2]! - positions[pa + 2]!];
    const twice = portable.hypot(e1[1]! * e2[2]! - e1[2]! * e2[1]!, e1[2]! * e2[0]! - e1[0]! * e2[2]!, e1[0]! * e2[1]! - e1[1]! * e2[0]!);
    if (a === b || b === c || a === c || twice < 1e-18) {
      degenerate++;
      continue;
    }
    live[t] = 1;
    for (let k = 0; k < 3; k++) {
      const [u, v] = [corners[t * 3 + k]!, corners[t * 3 + ((k + 1) % 3)]!];
      const key = Math.min(u, v) * span + Math.max(u, v);
      const entry = t * 2 + (u < v ? 0 : 1);
      const list = edgeTriangles.get(key);
      if (list) list.push(entry);
      else edgeTriangles.set(key, [entry]);
    }
  }
  return { welded, vertexCount: seen.size, live, corners, edgeTriangles, degenerate };
}

function signedVolumeOf(positions: ArrayLike<number>, indices: ArrayLike<number>, t: number): number {
  const [a, b, c] = [indices[t * 3]! * 3, indices[t * 3 + 1]! * 3, indices[t * 3 + 2]! * 3];
  const [ax, ay, az, bx, by, bz, cx, cy, cz] = [positions[a]!, positions[a + 1]!, positions[a + 2]!, positions[b]!, positions[b + 1]!, positions[b + 2]!, positions[c]!, positions[c + 1]!, positions[c + 2]!];
  return (ax * (by * cz - bz * cy) - ay * (bx * cz - bz * cx) + az * (bx * cy - by * cx)) / 6;
}

function areaOf(positions: ArrayLike<number>, indices: ArrayLike<number>, t: number): number {
  const [a, b, c] = [indices[t * 3]! * 3, indices[t * 3 + 1]! * 3, indices[t * 3 + 2]! * 3];
  const e1 = [positions[b]! - positions[a]!, positions[b + 1]! - positions[a + 1]!, positions[b + 2]! - positions[a + 2]!];
  const e2 = [positions[c]! - positions[a]!, positions[c + 1]! - positions[a + 1]!, positions[c + 2]! - positions[a + 2]!];
  return portable.hypot(e1[1]! * e2[2]! - e1[2]! * e2[1]!, e1[2]! * e2[0]! - e1[0]! * e2[2]!, e1[0]! * e2[1]! - e1[1]! * e2[0]!) / 2;
}

type Walk = { readonly flipped: Uint8Array; readonly component: Int32Array; readonly components: number; readonly conflicts: number };

function walk(topology: Topology, triangleCount: number): Walk {
  const flipped = new Uint8Array(triangleCount);
  const component = new Int32Array(triangleCount).fill(-1);
  const links = new Int32Array(triangleCount * 9);
  const degree = new Uint8Array(triangleCount);
  for (const list of topology.edgeTriangles.values()) {
    if (list.length !== 2) continue;
    const [t, u] = [list[0]! >> 1, list[1]! >> 1];
    if (t === u || degree[t]! >= 3 || degree[u]! >= 3) continue;
    const [dt, du] = [list[0]! & 1, list[1]! & 1];
    links.set([u, dt, du], t * 9 + degree[t]!++ * 3);
    links.set([t, du, dt], u * 9 + degree[u]!++ * 3);
  }
  let components = 0, conflicts = 0;
  const stack: number[] = [];
  for (let seed = 0; seed < triangleCount; seed++) {
    if (!topology.live[seed] || component[seed] !== -1) continue;
    component[seed] = components;
    stack.push(seed);
    while (stack.length > 0) {
      const t = stack.pop()!;
      for (let k = 0; k < degree[t]!; k++) {
        const [other, mine, theirs] = [links[t * 9 + k * 3]!, links[t * 9 + k * 3 + 1]!, links[t * 9 + k * 3 + 2]!];
        const wanted = mine ^ flipped[t]! ^ 1 ^ theirs;
        if (component[other] === -1) {
          component[other] = components;
          flipped[other] = wanted;
          stack.push(other);
        } else if (flipped[other] !== wanted) conflicts++;
      }
    }
    components++;
  }
  return { flipped, component, components, conflicts };
}

/** 🔬️ Closedness, manifoldness, winding consistency, signed volume and area of an indexed triangle mesh (positions xyz, indices three per triangle), see the module notes. */
export function meshIntegrity(positions: ArrayLike<number>, indices: ArrayLike<number>, options: IntegrityOptions = {}): MeshIntegrity {
  const triangleCount = Math.floor(indices.length / 3);
  const topology = topologyOf(positions, indices, options.toleranceM ?? DEFAULT_TOLERANCE_M);
  let openEdges = 0, nonManifoldEdges = 0, inconsistentEdges = 0;
  for (const list of topology.edgeTriangles.values()) {
    if (list.length === 1) openEdges++;
    else if (list.length > 2) nonManifoldEdges++;
    else if ((list[0]! & 1) === (list[1]! & 1)) inconsistentEdges++;
  }
  const parent = new Int32Array(triangleCount * 3).map((_, i) => i);
  const find = (i: number): number => {
    while (parent[i] !== i) i = parent[i] = parent[parent[i]!]!;
    return i;
  };
  const cornerOf = (t: number, vertex: number): number => {
    for (let k = 0; k < 3; k++) if (topology.corners[t * 3 + k] === vertex) return t * 3 + k;
    return -1;
  };
  const span = topology.vertexCount + 1;
  for (const [key, list] of topology.edgeTriangles) {
    if (list.length !== 2) continue;
    const [u, v] = [Math.floor(key / span), key % span];
    const [t, s] = [list[0]! >> 1, list[1]! >> 1];
    for (const vertex of [u, v]) parent[find(cornerOf(t, vertex))] = find(cornerOf(s, vertex));
  }
  const fans = new Map<number, Set<number>>();
  for (let t = 0; t < triangleCount; t++) {
    if (!topology.live[t]) continue;
    for (let k = 0; k < 3; k++) {
      const vertex = topology.corners[t * 3 + k]!;
      const roots = fans.get(vertex);
      if (roots) roots.add(find(t * 3 + k));
      else fans.set(vertex, new Set([find(t * 3 + k)]));
    }
  }
  let pinchVertices = 0;
  for (const roots of fans.values()) if (roots.size > 1) pinchVertices++;
  const { components } = walk(topology, triangleCount);
  let signedVolume = 0, area = 0;
  for (let t = 0; t < triangleCount; t++) {
    if (!topology.live[t]) continue;
    signedVolume += signedVolumeOf(positions, indices, t);
    area += areaOf(positions, indices, t);
  }
  const closed = openEdges === 0 && nonManifoldEdges === 0 && triangleCount - topology.degenerate > 0;
  return { triangles: triangleCount, degenerate: topology.degenerate, vertices: topology.vertexCount, edges: topology.edgeTriangles.size, openEdges, nonManifoldEdges, inconsistentEdges, pinchVertices, components, closed, oriented: inconsistentEdges === 0, manifold: closed && pinchVertices === 0, signedVolume, area };
}

/** 🧭️ The outward orientation of a mesh: `normals` (three numbers per triangle, unit, zeros for a degenerate triangle) point out of the solid, `closed` says that no edge is open or non-manifold (a closed mesh has an inside, an open one buries nothing), `signedVolume` is the volume after the orientation (positive for a closed mesh). The faces of a connected piece are oriented consistently, a closed piece is turned so that its signed volume is positive, an open piece keeps the winding of its first face (times `options.openWinding`); faces of a closed piece never point into it, so a probe moved out of a face along its normal leaves the solid even when the winding of the mesh was inside-out. The answer depends on this mesh alone, never on a solid it touches. */
export function outwardOrientation(positions: ArrayLike<number>, indices: ArrayLike<number>, options: IntegrityOptions = {}): { readonly normals: Float64Array; readonly closed: boolean; readonly signedVolume: number } {
  const triangleCount = Math.floor(indices.length / 3);
  const topology = topologyOf(positions, indices, options.toleranceM ?? DEFAULT_TOLERANCE_M);
  const { flipped, component, components } = walk(topology, triangleCount);
  const volume = new Float64Array(components);
  const open = new Uint8Array(components);
  for (let t = 0; t < triangleCount; t++) if (topology.live[t]) volume[component[t]!]! += (flipped[t] ? -1 : 1) * signedVolumeOf(positions, indices, t);
  let closed = triangleCount - topology.degenerate > 0;
  for (const list of topology.edgeTriangles.values()) if (list.length !== 2) {
    closed = false;
    for (const entry of list) open[component[entry >> 1]!] = 1;
  }
  const normals = new Float64Array(triangleCount * 3);
  let signedVolume = 0;
  for (let t = 0; t < triangleCount; t++) {
    if (!topology.live[t]) continue;
    const c = component[t]!;
    const turned = !open[c] && volume[c]! < 0 ? -1 : 1;
    const sign = (flipped[t] ? -1 : 1) * turned * (open[c] ? (options.openWinding ?? 1) : 1);
    signedVolume += sign * signedVolumeOf(positions, indices, t);
    const [a, b, d] = [indices[t * 3]! * 3, indices[t * 3 + 1]! * 3, indices[t * 3 + 2]! * 3];
    const e1 = [positions[b]! - positions[a]!, positions[b + 1]! - positions[a + 1]!, positions[b + 2]! - positions[a + 2]!];
    const e2 = [positions[d]! - positions[a]!, positions[d + 1]! - positions[a + 1]!, positions[d + 2]! - positions[a + 2]!];
    const n = [e1[1]! * e2[2]! - e1[2]! * e2[1]!, e1[2]! * e2[0]! - e1[0]! * e2[2]!, e1[0]! * e2[1]! - e1[1]! * e2[0]!];
    const l = portable.hypot(n[0]!, n[1]!, n[2]!) || 1;
    normals[t * 3] = (sign * n[0]!) / l;
    normals[t * 3 + 1] = (sign * n[1]!) / l;
    normals[t * 3 + 2] = (sign * n[2]!) / l;
  }
  return { normals, closed, signedVolume };
}

/** 🧭️ The outward unit normals of {@link outwardOrientation} alone. */
export function outwardNormals(positions: ArrayLike<number>, indices: ArrayLike<number>, options: IntegrityOptions = {}): Float64Array {
  return outwardOrientation(positions, indices, options).normals;
}

/** 🧊️ {@link meshIntegrity} of one solid tessellated at `quality` in its node frame (transform and repeat applied). */
export function solidIntegrity(solid: Solid, quality: Quality, options: IntegrityOptions = {}): MeshIntegrity {
  const g: Geometry = tessellateSolid(solid, quality);
  return meshIntegrity(g.positions, g.indices, options);
}
