/** 🧮️ The owned geometry measures of the volume rules: union volume by vertical columns, the volume of the union eroded and dilated by a distance on a voxel grid, the exposed area of a set of meshes and the triangle count at one fixed tessellation quality.
 *
 * THE REFERENCES are trimesh and scipy (point containment, Euclidean distance transform) in `🧪️tests/🧊️volumes-rules/🐍️.py`.
 *
 * @see ../🟦️.ts — `checkVolumes`, which uses them
 * @see ../🌊️flood/🟦️.ts — the flood from outside behind `outsideOf` (C31, E3)
 * @see ../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts — the kernel tessellation */
import type { VolumeSolid } from "../../../../🧬️schema/🟦️.ts";
import { outwardOrientation } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { FLOOD_BLOCK, floodOutside } from "../🌊️flood/🟦️.ts";
import { VOLUME_LIMITS } from "../🧱️rules/🟦️.ts";
import { copyMesh, edt1, extentOf, type World } from "../🌓️silhouette/🟦️.ts";

type Vec = readonly [number, number, number];

const JITTER_X = 1.2345e-3;
const JITTER_Z = 7.6543e-4;

type ColumnWindow = { readonly i0: number; readonly i1: number; readonly j0: number; readonly j1: number };
type ColumnMesh = { readonly solid: World; readonly range: ColumnWindow };

function columnMeshes(solids: readonly World[], box: ReturnType<typeof extentOf>, cell: number): readonly ColumnMesh[] {
  return solids.map(solid => {
    const { min, max } = extentOf([solid]);
    return { solid, range: {
      i0: Math.ceil((min[0] - box.min[0]) / cell - 0.5), i1: Math.floor((max[0] - box.min[0]) / cell - 0.5) + 1,
      j0: Math.ceil((min[2] - box.min[2]) / cell - 0.5), j1: Math.floor((max[2] - box.min[2]) / cell - 0.5) + 1,
    } };
  });
}

function columns(meshes: readonly ColumnMesh[], cell: number, box: ReturnType<typeof extentOf>, nx: number, tile: ColumnWindow) {
  const spans = new Map<number, [number, number][]>();
  for (const {solid, range} of meshes) {
    if (range.i1 <= tile.i0 || range.i0 >= tile.i1 || range.j1 <= tile.j0 || range.j0 >= tile.j1) continue;
    const p = solid.positions;
    const hits = new Map<number, number[]>();
    for (let t = 0; t < solid.indices.length; t += 3) {
      const [a, b, c] = [solid.indices[t]! * 3, solid.indices[t + 1]! * 3, solid.indices[t + 2]! * 3];
      const [x0, x1, x2, z0, z1, z2] = [p[a]!, p[b]!, p[c]!, p[a + 2]!, p[b + 2]!, p[c + 2]!];
      const det = (x1 - x0) * (z2 - z0) - (x2 - x0) * (z1 - z0);
      if (Math.abs(det) < 1e-18) continue;
      const i0 = Math.max(tile.i0, Math.ceil((Math.min(x0, x1, x2) - box.min[0]) / cell - 0.5));
      const i1 = Math.min(tile.i1 - 1, Math.floor((Math.max(x0, x1, x2) - box.min[0]) / cell - 0.5));
      const j0 = Math.max(tile.j0, Math.ceil((Math.min(z0, z1, z2) - box.min[2]) / cell - 0.5));
      const j1 = Math.min(tile.j1 - 1, Math.floor((Math.max(z0, z1, z2) - box.min[2]) / cell - 0.5));
      for (let j = j0; j <= j1; j++)
        for (let i = i0; i <= i1; i++) {
          const [x, z] = [box.min[0] + (i + 0.5 + JITTER_X) * cell, box.min[2] + (j + 0.5 + JITTER_Z) * cell];
          const l1 = ((x - x0) * (z2 - z0) - (x2 - x0) * (z - z0)) / det;
          const l2 = ((x1 - x0) * (z - z0) - (x - x0) * (z1 - z0)) / det;
          if (l1 < 0 || l2 < 0 || l1 + l2 > 1) continue;
          const y = p[a + 1]! + l1 * (p[b + 1]! - p[a + 1]!) + l2 * (p[c + 1]! - p[a + 1]!);
          const key = j * nx + i;
          const list = hits.get(key);
          if (list === undefined) hits.set(key, [y]);
          else list.push(y);
        }
    }
    for (const [key, ys] of hits) {
      ys.sort((u, v) => u - v);
      const own = spans.get(key) ?? [];
      for (let n = 0; n + 1 < ys.length; n += 2) own.push([ys[n]!, ys[n + 1]!]);
      spans.set(key, own);
    }
  }
  for (const intervals of spans.values()) {
    intervals.sort((a, b) => a[0] - b[0]);
    let count = 0;
    for (const interval of intervals) {
      const last = intervals[count - 1];
      if (last !== undefined && interval[0] <= last[1]) last[1] = Math.max(last[1], interval[1]);
      else intervals[count++] = interval;
    }
    intervals.length = count;
  }
  return spans;
}

/** 🧮️ The volume of the union of closed meshes in m³: along every vertical column of a grid of `cell` metres (columns sampled at cell centres nudged by different thousandths of a cell in x and z, so that no column lies on a shared diagonal edge) the union of the intervals between the entering and leaving crossings of each solid, exact in y and sampled in x and z. */
export function unionVolume(solids: readonly World[], cell: number): number {
  const box = extentOf(solids);
  if (!Number.isFinite(box.min[0])) return 0;
  const nx = Math.max(1, Math.ceil((box.max[0] - box.min[0]) / cell)), nz = Math.max(1, Math.ceil((box.max[2] - box.min[2]) / cell));
  const meshes = columnMeshes(solids, box, cell);
  const side = 128;
  let total = 0;
  for (let j = 0; j < nz; j += side) for (let i = 0; i < nx; i += side) {
    const spans = columns(meshes, cell, box, nx, { i0: i, i1: Math.min(nx, i + side), j0: j, j1: Math.min(nz, j + side) });
    for (const intervals of spans.values()) for (const [a, b] of intervals) total += b - a;
  }
  return total * cell * cell;
}

/** 🧊️ The volumes in m³ of two unions of closed meshes and of the first one eroded and dilated by `distanceM`, on one voxel grid of `cellM` metres: a quarter of the distance, raised only until the grid has at most `maxVoxels` cells and, for objects so large that this leaves the cell above the distance, the band is the cell (`bandM`). `coarse` is the second union, `fine` the first, `eroded` the cells of `fine` farther than the band from every empty cell and `dilated` the cells within the band of a cell of `fine`. The grid is never allocated whole: blocks of `BAND_BLOCK` columns with a margin of the band are transformed one at a time and empty blocks cost nothing, so memory stays bounded for a 70 m crane and a 10 m excavator keeps a band of two cells. */
export function bandVolumes(fine: readonly World[], coarse: readonly World[], distanceM: number, maxVoxels = 150_000_000): { readonly fine: number; readonly coarse: number; readonly eroded: number; readonly dilated: number; readonly cellM: number; readonly bandM: number } {
  const all = extentOf([...fine, ...coarse]);
  if (!Number.isFinite(all.min[0])) return { fine: 0, coarse: 0, eroded: 0, dilated: 0, cellM: distanceM, bandM: distanceM };
  const reach = distanceM;
  const grown = { min: all.min.map((v) => v - reach) as unknown as Vec, max: all.max.map((v) => v + reach) as unknown as Vec };
  const extent = [0, 1, 2].map((k) => grown.max[k]! - grown.min[k]!);
  const h = Math.max(distanceM / 4, Math.cbrt((extent[0]! * extent[1]! * extent[2]!) / maxVoxels));
  const band = Math.max(distanceM, h);
  const [nx, ny, nz] = extent.map((e) => Math.max(1, Math.ceil(e / h))) as [number, number, number];
  const box = { min: grown.min, max: [grown.min[0]! + nx * h, grown.min[1]! + ny * h, grown.min[2]! + nz * h] as unknown as Vec };
  const fineMeshes = columnMeshes(fine, box, h), coarseMeshes = columnMeshes(coarse, box, h);
  const margin = Math.ceil(band / h) + 1;
  const limit = (band / h) ** 2;
  const yRange = (lo: number, hi: number): [number, number] => [Math.ceil((lo - grown.min[1]!) / h - 0.5), Math.floor((hi - grown.min[1]!) / h - 0.5)];
  let [volFine, volCoarse, eroded, dilated] = [0, 0, 0, 0];
  for (let k0 = 0; k0 < nz; k0 += BAND_BLOCK)
    for (let i0 = 0; i0 < nx; i0 += BAND_BLOCK) {
      const [i1, k1] = [Math.min(nx, i0 + BAND_BLOCK), Math.min(nz, k0 + BAND_BLOCK)];
      const [e0, f0, e1, f1] = [Math.max(0, i0 - margin), Math.max(0, k0 - margin), Math.min(nx, i1 + margin), Math.min(nz, k1 + margin)];
      const tile = { i0: e0, i1: e1, j0: f0, j1: f1 };
      const spansFine = columns(fineMeshes, h, box, nx, tile);
      const spansCoarse = columns(coarseMeshes, h, box, nx, tile);
      let [lo, hi] = [Infinity, -Infinity];
      for (let k = f0; k < f1; k++)
        for (let i = e0; i < e1; i++) {
          for (const spans of [spansFine.get(k * nx + i), spansCoarse.get(k * nx + i)])
            if (spans !== undefined) for (const [u, v] of spans) [lo, hi] = [Math.min(lo, u), Math.max(hi, v)];
        }
      if (!Number.isFinite(lo)) continue;
      const [y0, y1] = [Math.max(0, yRange(lo, lo)[0] - margin), Math.min(ny - 1, yRange(hi, hi)[1] + margin)];
      const [bx, bz, by] = [e1 - e0, f1 - f0, y1 - y0 + 1];
      if (by <= 0) continue;
      const a = new Uint8Array(bx * by * bz);
      const b = new Uint8Array(bx * by * bz);
      const fill = (grid: Uint8Array, source: Map<number, [number, number][]>): void => {
        for (let k = f0; k < f1; k++)
          for (let i = e0; i < e1; i++) {
            const spans = source.get(k * nx + i);
            if (spans === undefined) continue;
            for (const [u, v] of spans) for (let y = Math.max(y0, yRange(u, v)[0]); y <= Math.min(y1, yRange(u, v)[1]); y++) grid[((y - y0) * bz + (k - f0)) * bx + (i - e0)] = 1;
          }
      };
      fill(a, spansFine);
      fill(b, spansCoarse);
      const near = distance3(a, bx, by, bz, 1);
      const far = distance3(a, bx, by, bz, 0);
      for (let y = 0; y < by; y++)
        for (let k = k0; k < k1; k++)
          for (let i = i0; i < i1; i++) {
            const at = (y * bz + (k - f0)) * bx + (i - e0);
            if (a[at]) volFine++;
            if (b[at]) volCoarse++;
            if (near[at]! <= limit) dilated++;
            if (a[at] === 1 && far[at]! > limit) eroded++;
          }
    }
  const cube = h * h * h;
  return { fine: volFine * cube, coarse: volCoarse * cube, eroded: eroded * cube, dilated: dilated * cube, cellM: h, bandM: band };
}

/** 🧱️ How many columns a block of {@link bandVolumes} spans along x and z. */
export const BAND_BLOCK = 48;

/** 📏️ The squared distance in cells from every cell to the nearest cell whose value is `wanted` (Felzenszwalb-Huttenlocher in three passes). */
export function distance3(grid: Uint8Array, nx: number, ny: number, nz: number, wanted: 0 | 1): Float32Array {
  const big = 1e20;
  const unset = Math.fround(big);
  const out = new Float32Array(grid.length);
  for (let i = 0; i < grid.length; i++) out[i] = grid[i] === wanted ? 0 : big;
  const n = Math.max(nx, ny, nz);
  const [f, d, z] = [new Float64Array(n), new Float64Array(n), new Float64Array(n + 1)];
  const v = new Int32Array(n);
  const pass = (length: number, stride: number, starts: readonly number[]): void => {
    for (const start of starts) {
      let [lo, hi] = [Infinity, -Infinity];
      for (let q = 0; q < length; q++) {
        const value = out[start + q * stride]!;
        f[q] = value;
        if (value < lo) lo = value;
        if (value > hi) hi = value;
      }
      if (lo === hi && (lo === 0 || lo === unset)) continue;
      edt1(f, length, d, v, z);
      for (let q = 0; q < length; q++) out[start + q * stride] = d[q]!;
    }
  };
  const xs: number[] = [];
  for (let y = 0; y < ny; y++) for (let k = 0; k < nz; k++) xs.push((y * nz + k) * nx);
  pass(nx, 1, xs);
  const zs: number[] = [];
  for (let y = 0; y < ny; y++) for (let i = 0; i < nx; i++) zs.push(y * nz * nx + i);
  pass(nz, nx, zs);
  const ys: number[] = [];
  for (let k = 0; k < nz; k++) for (let i = 0; i < nx; i++) ys.push(k * nx + i);
  pass(ny, nz * nx, ys);
  return out;
}

type Orientation = { readonly normals: Float64Array; readonly closed: boolean };
const ORIENTATIONS = [new WeakMap<World, Orientation>(), new WeakMap<World, Orientation>()];

/** 🧭️ Outward normals and closedness of a posed mesh, shared by exact geometry references. Translated copies with disjoint bounds are welded separately and retain the same triangle order. */
export function orientationOf(solid: World): Orientation {
  const source = solid.geometrySource ?? solid;
  const f = solid.frame;
  const left = f !== undefined && f.u.x * (f.v.y * f.w.z - f.v.z * f.w.y) + f.u.y * (f.v.z * f.w.x - f.v.x * f.w.z) + f.u.z * (f.v.x * f.w.y - f.v.y * f.w.x) < 0;
  const cache = ORIENTATIONS[left ? 1 : 0]!;
  const cached = cache.get(source);
  if (cached) return cached;
  const options = { openWinding: left ? -1 as const : 1 as const };
  const copies = source.separateCopies;
  let result: Orientation;
  if (copies) {
    const normals = new Float64Array(source.indices.length);
    let hasClosedCopy = false, hasOpenFaces = false;
    for (let p = 0, n = 0; p < source.positions.length; p += copies.coordinatesPerCopy, n += copies.indices.length) {
      const copy = outwardOrientation(source.positions.subarray(p, p + copies.coordinatesPerCopy), copies.indices, options);
      normals.set(copy.normals, n);
      if (copy.closed) hasClosedCopy = true;
      else if (!hasOpenFaces) hasOpenFaces = copy.normals.some(value => value !== 0);
    }
    result = { normals, closed: hasClosedCopy && !hasOpenFaces };
  } else {
    const { normals, closed } = outwardOrientation(source.positions, source.indices, options);
    result = { normals, closed };
  }
  cache.set(source, result);
  return result;
}

/** 🔒️ Whether the mesh is closed: every edge is used by exactly two triangles (the kernel's closedness). Only closed meshes have an inside; an open skin buries nothing. */
export const isClosed = (solid: World): boolean => orientationOf(solid).closed;

type Grid = { readonly min: Vec; readonly max: Vec; readonly n: number; readonly bins: Int32Array[]; readonly stamp: Int32Array; stampValue: number; readonly rays: Float64Array };

/** 🎯️ The numbers kept per triangle for the ray of {@link insideAny} (`rays` of the grid): its first corner, its two edges, the cross product of the ray direction with the second edge and the determinant, computed once per mesh with the expressions the test used per query. */
const RAY_STRIDE = 13;

const GRIDS = new WeakMap<World, Grid>();

function gridOf(solid: World): Grid {
  const source = solid.geometrySource ?? solid;
  let grid = GRIDS.get(source);
  if (grid !== undefined) return grid;
  const { positions: q, indices } = solid;
  const min: [number, number, number] = [Infinity, Infinity, Infinity];
  const max: [number, number, number] = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < q.length; i += 3) for (let k = 0; k < 3; k++) {
    min[k] = Math.min(min[k]!, q[i + k]!);
    max[k] = Math.max(max[k]!, q[i + k]!);
  }
  const n = Math.max(1, Math.min(64, Math.round(Math.sqrt(indices.length / 3 / 4))));
  const lists: number[][] = Array.from({ length: n * n }, () => []);
  const at = (v: number, lo: number, hi: number): number => Math.max(0, Math.min(n - 1, Math.floor(((v - lo) / Math.max(hi - lo, 1e-12)) * n)));
  for (let t = 0; t < indices.length; t += 3) {
    const xs = [q[indices[t]! * 3]!, q[indices[t + 1]! * 3]!, q[indices[t + 2]! * 3]!];
    const zs = [q[indices[t]! * 3 + 2]!, q[indices[t + 1]! * 3 + 2]!, q[indices[t + 2]! * 3 + 2]!];
    for (let j = at(Math.min(...zs), min[2], max[2]); j <= at(Math.max(...zs), min[2], max[2]); j++) for (let i = at(Math.min(...xs), min[0], max[0]); i <= at(Math.max(...xs), min[0], max[0]); i++) lists[j * n + i]!.push(t);
  }
  const rays = new Float64Array((indices.length / 3) * RAY_STRIDE);
  const d: Vec = [SKEW[0], SKEW[2], SKEW[1]];
  for (let t = 0, o = 0; t < indices.length; t += 3, o += RAY_STRIDE) {
    const [a, b, c] = [indices[t]! * 3, indices[t + 1]! * 3, indices[t + 2]! * 3];
    const e1: Vec = [q[b]! - q[a]!, q[b + 1]! - q[a + 1]!, q[b + 2]! - q[a + 2]!];
    const e2: Vec = [q[c]! - q[a]!, q[c + 1]! - q[a + 1]!, q[c + 2]! - q[a + 2]!];
    const h: Vec = [d[1] * e2[2] - d[2] * e2[1], d[2] * e2[0] - d[0] * e2[2], d[0] * e2[1] - d[1] * e2[0]];
    rays.set([q[a]!, q[a + 1]!, q[a + 2]!, e1[0], e1[1], e1[2], e2[0], e2[1], e2[2], h[0], h[1], h[2], e1[0] * h[0] + e1[1] * h[1] + e1[2] * h[2]], o);
  }
  grid = { min, max, n, bins: lists.map((l) => Int32Array.from(l)), stamp: new Int32Array(indices.length / 3), stampValue: 0, rays };
  GRIDS.set(source, grid);
  return grid;
}

const BOXES = new WeakMap<World, { readonly min: Vec; readonly max: Vec }>();

/** 📦️ The bounding box of a mesh, cached. */
function boxOf(solid: World): { readonly min: Vec; readonly max: Vec } {
  const source = solid.geometrySource ?? solid;
  let hit = BOXES.get(source);
  if (hit === undefined) {
    const [min, max]: [[number, number, number], [number, number, number]] = [[Infinity, Infinity, Infinity], [-Infinity, -Infinity, -Infinity]];
    const q = solid.positions;
    for (let i = 0; i < q.length; i += 3) for (let k = 0; k < 3; k++) {
      min[k] = Math.min(min[k]!, q[i + k]!);
      max[k] = Math.max(max[k]!, q[i + k]!);
    }
    BOXES.set(source, (hit = { min, max }));
  }
  return hit;
}

const DISJOINT_PARTS = new WeakMap<World, readonly World[]>();

/** 🪜 Closed translated parts retain their own bounds and local triangle index. */
function disjointParts(solid: World): readonly World[] {
  const source = solid.geometrySource ?? solid;
  const known = DISJOINT_PARTS.get(source);
  if (known) return known;
  const { coordinatesPerCopy, indices } = source.separateCopies!;
  const parts: World[] = [];
  for (let start = 0; start < source.positions.length; start += coordinatesPerCopy)
    parts.push({ id: source.id, positions: source.positions.subarray(start, start + coordinatesPerCopy), indices });
  DISJOINT_PARTS.set(source, parts);
  return parts;
}

const SKEW: Vec = [0.000123, 0.000317, 1];

/** 🗃️ The bin of the coordinate `v` in a grid of `n` bins from `lo` to `hi`, clamped. */
const binOf =(v: number, lo: number, hi: number, n: number): number => Math.max(0, Math.min(n - 1, Math.floor(((v - lo) / Math.max(hi - lo, 1e-12)) * n)));

/** 🔎️ Whether the point is inside any of the closed meshes that `skip` does not exclude: parity of the crossings of a slightly skewed ray along +y (Möller-Trumbore), candidates taken from a grid over x and z of each mesh. */
export function insideAny(solids: readonly World[], p: Vec, skip: (solid: World, index: number) => boolean = () => false): boolean {
  const [d0, d1, d2] = [SKEW[0], SKEW[2], SKEW[1]];
  const [px, py, pz] = [p[0], p[1], p[2]];
  for (let k = 0; k < solids.length; k++) {
    const solid = solids[k]!;
    const { min, max } = boxOf(solid);
    const drift = 0.0004 * (max[1] - min[1]) + 1e-9;
    if (px < min[0] - drift || px > max[0] + drift || pz < min[2] - drift || pz > max[2] + drift || py < min[1] || py > max[1] || skip(solid, k) || !isClosed(solid)) continue;
    if (solid.separateCopies) {
      if (insideAny(disjointParts(solid), p)) return true;
      continue;
    }
    const g = gridOf(solid);
    const { n, bins, stamp, rays } = g;
    const [i0, i1] = [binOf(px - drift, g.min[0], g.max[0], n), binOf(px + drift, g.min[0], g.max[0], n)];
    const [j0, j1] = [binOf(pz - drift, g.min[2], g.max[2], n), binOf(pz + drift, g.min[2], g.max[2], n)];
    const mark = ++g.stampValue;
    let crossings = 0;
    for (let j = j0; j <= j1; j++)
      for (let i = i0; i <= i1; i++) {
        const bin = bins[j * n + i]!;
        for (let m = 0; m < bin.length; m++) {
          const face = bin[m]! / 3;
          if (stamp[face] === mark) continue;
          stamp[face] = mark;
          const o = face * RAY_STRIDE;
          const det = rays[o + 12]!;
          if (Math.abs(det) < 1e-18) continue;
          const s0 = px - rays[o]!;
          const s1 = py - rays[o + 1]!;
          const s2 = pz - rays[o + 2]!;
          const u = (s0 * rays[o + 9]! + s1 * rays[o + 10]! + s2 * rays[o + 11]!) / det;
          if (u < 0 || u > 1) continue;
          const r0 = s1 * rays[o + 5]! - s2 * rays[o + 4]!;
          const r1 = s2 * rays[o + 3]! - s0 * rays[o + 5]!;
          const r2 = s0 * rays[o + 4]! - s1 * rays[o + 3]!;
          const v = (d0 * r0 + d1 * r1 + d2 * r2) / det;
          if (v < 0 || u + v > 1) continue;
          if ((rays[o + 6]! * r0 + rays[o + 7]! * r1 + rays[o + 8]! * r2) / det > 0) crossings++;
        }
      }
    if (crossings % 2 === 1) return true;
  }
  return false;
}

/** 🧱️ A contact plane through `point` with the unit `normal`: faces lying in it within `toleranceM` and facing along it do not count as exposed (the ground under a foot, the wall behind a casing). */
export type ContactPlane = { readonly point: Vec; readonly normal: Vec; readonly toleranceM: number };

/** 🧵️ How far apart in smallest features two solids may lie and still count as touching along a seam (a fifth of a feature): the facets of two tessellations of one curved surface differ by about that much. */
export const SEAM_FEATURES = 0.2;

/** 🧭️ How far a probe is moved out of a face before it is tested for burial, in metres. */
export const BURIAL_PROBE_M = 5e-4;

const AXES = { u: "u", v: "v", w: "w" } as const;

const SUBDIVISIONS = new Map<number, readonly (readonly [number, number, number])[]>();
/** 🔹️ The barycentric centres of the m² congruent triangles a triangle splits into by m cuts per edge: the sample points of an area-weighted burial test (m = 1 is the centroid). */
function subdivision(m: number): readonly (readonly [number, number, number])[] {
  let hit = SUBDIVISIONS.get(m);
  if (hit === undefined) {
    const out: [number, number, number][] = [];
    for (let i = 0; i < m; i++)
      for (let j = 0; j < m - i; j++) {
        out.push([(i + 1 / 3) / m, (j + 1 / 3) / m, 1 - (i + j + 2 / 3) / m]);
        if (i + j < m - 1) out.push([(i + 2 / 3) / m, (j + 2 / 3) / m, 1 - (i + j + 4 / 3) / m]);
      }
    SUBDIVISIONS.set(m, (hit = out));
  }
  return hit;
}
/** 📏️ The probe distance in smallest features (0.02, {@link BURIAL_PROBE_M} is that times the feature of the scale) and the most cuts per edge of a face tested at several points: samples at most two features apart, so a face half under another solid is half buried, not wholly one or the other, and no author needs to tile a mesh to make that so. */
const BURIAL_PROBE_FEATURE = 0.02;
const BURIAL_MAX_SPLITS = 24;

/** 🔢️ The step between the sample numbers of two faces: {@link exposedShares} hands `outside` the number `(face of the target) · SAMPLES_PER_FACE + sample of the face`, the same for the same target whatever is buried, so that a caller that measures one target twice can remember the reach of a sample ({@link rememberedReach}). */
export const SAMPLES_PER_FACE = BURIAL_MAX_SPLITS ** 2;

/** 🧠️ `reach` with its answers remembered by sample number: the outer shares and the exposed area of one region ask the flood for the same samples of the same target. */
export function rememberedReach(reach: (base: Vec, normal: Vec) => Reach | undefined): (base: Vec, normal: Vec, sample: number) => Reach | undefined {
  const known = new Map<number, Int8Array>();
  return (base, normal, sample) => {
    const face = Math.floor(sample / SAMPLES_PER_FACE);
    const at = sample - face * SAMPLES_PER_FACE;
    let answers = known.get(face);
    if (answers === undefined || answers.length <= at) {
      const grown = new Int8Array(2 ** Math.ceil(Math.log2(at + 1)));
      if (answers !== undefined) grown.set(answers);
      known.set(face, (answers = grown));
    }
    if (answers[at] === 0) answers[at] = 2 + REACHES.indexOf(reach(base, normal) as Reach);
    return answers[at] === 1 ? undefined : REACHES[answers[at]! - 2];
  };
}

/** 🧭️ Why a face sample may count as outer surface (C31, E3): `air` the air from outside reaches it, `cover` a solid of another region that the air reaches covers it (C29), `cavity` it looks into a closed cavity and the region's own wall behind it does not separate that cavity from the air, so it exchanges with the enclosed air when the region is contents of the cavity ({@link outerOfShares}). */
export type Reach = "air" | "cover" | "cavity";

/** 🥫️ Whether a region is contents of a closed cavity: its `cavity` share outweighs its `air` and `cover` shares (E3). */
export const isContents = (shares: Readonly<Record<Reach, number>>): boolean => shares.cavity > shares.air + shares.cover;

/** 🫙️ The outer area of a region from its {@link exposedShares} (E3): a region whose `cavity` share outweighs its `air` and `cover` shares is contents of a closed cavity (the absorber and headers under the glass of a collector, a seat in a closed cabin) and counts its whole surface, which exchanges with the enclosed air; any other region is part of the closure or lies outside, and its `cavity` faces (the edge of a cover under a clamping lip, a tray floor over a rail of another region) do not count. */
export const outerOfShares = (shares: Readonly<Record<Reach, number>>): number => shares.air + shares.cover + (isContents(shares) ? shares.cavity : 0);

/** 🧿️ The exposed area in m² of the triangles of `target`, split by {@link Reach}: faces that are not buried inside a mesh of `against` (a probe moved `probeM` out of the face, 0.5 mm by default and 0.02 smallest features in the rules, along its normal lies inside it; `ignore` excludes meshes; with `contactM` above the probe a face is also buried when a probe at half that distance or at that distance lies inside, so that two solids that meet within `contactM` of each other, as the facets of two tessellations of one curved surface do, count as touching and the seam between them is no surface), not lying in a contact plane and not within 45 degrees of a declared `sectionFaces` axis of their solid. Without `outside` every such face is `air`; with it every face is sampled and a sample counts with the reach `outside` gives for its foot and outward normal, and not at all when it gives none ({@link outsideOf}). */
export function exposedShares(target: readonly World[], against: readonly World[], planes: readonly ContactPlane[] = [], ignore: (solid: World) => boolean = () => false, probeM: number = BURIAL_PROBE_M, contactM = 0, outside?: (base: Vec, normal: Vec, sample: number) => Reach | undefined): Readonly<Record<Reach, number>> {
  const shares: Record<Reach, number> = { air: 0, cover: 0, cavity: 0 };
  const cos = Math.cos((VOLUME_LIMITS.sectionAngleDeg * Math.PI) / 180);
  let faces = 0;
  for (const solid of target) {
    const first = faces;
    faces += solid.indices.length / 3;
    const q = solid.positions;
    const outward = orientationOf(solid).normals;
    const reachM = Math.max(probeM, contactM) + 1e-6;
    const [lo, hi] = [boxOf(solid).min, boxOf(solid).max];
    const near = against.filter((other) => {
      if (other === solid || ignore(other)) return false;
      const box = boxOf(other);
      return [0, 1, 2].every((k) => box.min[k]! - reachM <= hi[k]! && box.max[k]! + reachM >= lo[k]!) && isClosed(other);
    });
    for (let t = 0; t < solid.indices.length; t += 3) {
      const [a, b, c] = [solid.indices[t]! * 3, solid.indices[t + 1]! * 3, solid.indices[t + 2]! * 3];
      const e1 = [q[b]! - q[a]!, q[b + 1]! - q[a + 1]!, q[b + 2]! - q[a + 2]!];
      const e2 = [q[c]! - q[a]!, q[c + 1]! - q[a + 1]!, q[c + 2]! - q[a + 2]!];
      const cr = [e1[1]! * e2[2]! - e1[2]! * e2[1]!, e1[2]! * e2[0]! - e1[0]! * e2[2]!, e1[0]! * e2[1]! - e1[1]! * e2[0]!];
      const twice = Math.hypot(cr[0]!, cr[1]!, cr[2]!);
      if (twice < 1e-18) continue;
      const n: Vec = [outward[t]!, outward[t + 1]!, outward[t + 2]!];
      const centroid: Vec = [(q[a]! + q[b]! + q[c]!) / 3, (q[a + 1]! + q[b + 1]! + q[c + 1]!) / 3, (q[a + 2]! + q[b + 2]! + q[c + 2]!) / 3];
      if (solid.sectionFaces !== undefined && solid.frame !== undefined && solid.sectionFaces.some((face) => {
        const axis = solid.frame![AXES[face.slice(1) as "u" | "v" | "w"]];
        return (face[0] === "-" ? -1 : 1) * (n[0] * axis.x + n[1] * axis.y + n[2] * axis.z) >= cos;
      })) continue;
      if (planes.some((plane) => Math.abs(n[0] * plane.normal[0] + n[1] * plane.normal[1] + n[2] * plane.normal[2]) >= 0.9 && Math.abs((centroid[0] - plane.point[0]) * plane.normal[0] + (centroid[1] - plane.point[1]) * plane.normal[1] + (centroid[2] - plane.point[2]) * plane.normal[2]) <= plane.toleranceM)) continue;
      const around = near.filter((other) => {
        const box = boxOf(other);
        return [0, 1, 2].every((k) => box.min[k]! - reachM <= Math.max(q[a + k]!, q[b + k]!, q[c + k]!) && box.max[k]! + reachM >= Math.min(q[a + k]!, q[b + k]!, q[c + k]!));
      });
      if (around.length === 0 && outside === undefined) {
        shares.air += twice / 2;
        continue;
      }
      const longest = Math.max(Math.hypot(e1[0]!, e1[1]!, e1[2]!), Math.hypot(e2[0]!, e2[1]!, e2[2]!), Math.hypot(e2[0]! - e1[0]!, e2[1]! - e1[1]!, e2[2]! - e1[2]!));
      const weights = subdivision(Math.min(BURIAL_MAX_SPLITS, Math.max(1, Math.ceil(longest / (2 * (probeM / BURIAL_PROBE_FEATURE))))));
      const free: Record<Reach, number> = { air: 0, cover: 0, cavity: 0 };
      const reach = contactM > probeM ? [probeM, contactM / 2, contactM] : [probeM];
      for (let s = 0; s < weights.length; s++) {
        const [wa, wb, wc] = weights[s]!;
        const base: Vec = [wa * q[a]! + wb * q[b]! + wc * q[c]!, wa * q[a + 1]! + wb * q[b + 1]! + wc * q[c + 1]!, wa * q[a + 2]! + wb * q[b + 2]! + wc * q[c + 2]!];
        if (around.length > 0 && reach.some((r) => insideAny(around, [base[0] + n[0] * r, base[1] + n[1] * r, base[2] + n[2] * r]))) continue;
        const why = outside === undefined ? "air" : outside(base, n, (first + t / 3) * SAMPLES_PER_FACE + s);
        if (why !== undefined) free[why]++;
      }
      for (const why of REACHES) shares[why] += (twice / 2) * (free[why] / weights.length);
    }
  }
  return shares;
}

const REACHES: readonly Reach[] = ["air", "cover", "cavity"];

/** 🧿️ The exposed area in m² of the triangles of `target`: the sum of {@link exposedShares}. */
export function exposedArea(target: readonly World[], against: readonly World[], planes: readonly ContactPlane[] = [], ignore: (solid: World) => boolean = () => false, probeM: number = BURIAL_PROBE_M, contactM = 0, outside?: (base: Vec, normal: Vec, sample: number) => Reach | undefined): number {
  const shares = exposedShares(target, against, planes, ignore, probeM, contactM, outside);
  return shares.air + shares.cover + shares.cavity;
}

/** 🌊️ The cell of the flood from outside in smallest features (a quarter: a gap of at most that is closed, a gap of half a feature is open), and how far in front of a face, in smallest features, a solid of another region still covers it (half a feature). */
export const FLOOD_FEATURES = 0.25;
export const COVER_FEATURES = 0.5;

/** 🪟 Three voxels across a measured opening preserve a connected air path between its two boundary cells. */
function airCell(featureM: number, passageM?: number): number {
  return Math.min(FLOOD_FEATURES * featureM, passageM !== undefined && passageM > 0 ? passageM / 3 : Infinity);
}

/** 🌊️ What is outer surface among `walls`, the solids of one scale (C31, E3): `reach` says why a face sample with the foot `base` and the outward unit normal `normal` is outer for a region whose solids `own` names, or nothing when it is not. It is `air` when the air flooded from outside through the walls reaches the probe `probeM` in front of it (an open tube end, a cup or a duct mouth lets the air in); `cover` when a solid of another region covers it (C29: the probe lies inside that solid, or its surface is the first one within {@link COVER_FEATURES} features in front of the face), that solid itself has surface the air reaches and it stands between the face and the air: walking on along the normal through the solids, the first free space (free for {@link COVER_FEATURES} features) is reached by the air, so a neck, a spigot or a sensor that passes through a closed shell covers nothing of the inside of that shell; `cavity` when it looks into a closed cavity and the region's own wall behind it does not separate that cavity from the air: walking against the normal through the region's own solids (gaps of at most {@link SEAM_FEATURES} features between them are seams), the first free point beyond them is not reached by the air. So the inside of a closure (the bore of a capped tube, the inside of a casing, the underside of a glass cover, the face of a kettle against its sealed water) is never outer, while the faces of the contents of a cavity (the absorber and headers under the glass of a collector, a seat in a closed cabin) are `cavity` and count by {@link outerOfShares}. `cellM` is the cell the flood used: {@link FLOOD_FEATURES} features unless the budget of the flood raised it. `tick` is called regularly and may throw to cancel. */
export type Outside = { readonly cellM: number; readonly reach: (base: Vec, normal: Vec, own: (solid: World) => boolean) => Reach | undefined };
export function outsideOf(walls: readonly World[], featureM: number, probeM: number = BURIAL_PROBE_M, tick: () => void = () => {}, minimumAirPassageM?: number): Outside {
  const flood = floodOutside(walls, airCell(featureM,minimumAirPassageM), tick);
  const open = new Map<World, boolean>();
  const isOpen = (solid: World): boolean => {
    let hit = open.get(solid);
    if (hit === undefined) {
      hit = false;
      const { positions: q, indices } = solid;
      const outward = orientationOf(solid).normals;
      for (let t = 0; t < indices.length && !hit; t += 3) {
        const [a, b, c] = [indices[t]! * 3, indices[t + 1]! * 3, indices[t + 2]! * 3];
        const n: Vec = [outward[t]!, outward[t + 1]!, outward[t + 2]!];
        if (n[0] === 0 && n[1] === 0 && n[2] === 0) continue;
        hit = flood.reaches([(q[a]! + q[b]! + q[c]!) / 3 + n[0] * probeM, (q[a + 1]! + q[b + 1]! + q[c + 1]!) / 3 + n[1] * probeM, (q[a + 2]! + q[b + 2]! + q[c + 2]!) / 3 + n[2] * probeM], n);
      }
      open.set(solid, hit);
    }
    return hit;
  };
  const coverM = COVER_FEATURES * featureM;
  const seamM = SEAM_FEATURES * featureM;
  const stepM = FLOOD_BLOCK * flood.cellM;
  const box = walls.length === 0 ? { min: [0, 0, 0], max: [0, 0, 0] } : extentOf(walls);
  const limitM = Math.hypot(box.max[0]! - box.min[0]!, box.max[1]! - box.min[1]!, box.max[2]! - box.min[2]!) + stepM;
  const closure = (base: Vec, n: Vec, own: (solid: World) => boolean): boolean => {
    const at = (d: number): Vec => [base[0] - n[0] * d, base[1] - n[1] * d, base[2] - n[2] * d];
    const back: Vec = [-n[0], -n[1], -n[2]];
    const keep = (index: number): boolean => own(walls[index]!);
    const hits: number[] = [0];
    let marched = 0;
    const march = (untilM: number): void => {
      while (marched < untilM && marched < limitM) {
        for (const along of flood.crossings(at(marched), at(marched + stepM), keep)) if (marched + along * stepM > probeM * 1e-3) hits.push(marched + along * stepM);
        marched += stepM;
      }
      hits.sort((u, v) => u - v);
    };
    for (let k = 0; ; k++) {
      while (k >= hits.length && marched < limitM) march(marched + stepM);
      if (k >= hits.length) return true;
      const d = hits[k]!;
      march(d + seamM + probeM);
      const x = at(d + probeM);
      if (insideAny(walls, x, (solid) => !own(solid))) continue;
      if (hits.some((h) => h > d + probeM && h <= d + seamM)) continue;
      return flood.reaches(x, back);
    }
  };
  const beyond = (p: Vec, n: Vec): boolean => {
    const at = (d: number): Vec => [p[0] + n[0] * d, p[1] + n[1] * d, p[2] + n[2] * d];
    const hits: number[] = [];
    let marched = 0;
    const march = (untilM: number): void => {
      while (marched < untilM && marched < limitM) {
        for (const along of flood.crossings(at(marched), at(marched + stepM), () => true)) hits.push(marched + along * stepM);
        marched += stepM;
      }
      hits.sort((u, v) => u - v);
    };
    for (let k = 0; ; k++) {
      while (k >= hits.length && marched < limitM) march(marched + stepM);
      if (k >= hits.length) return true;
      const d = hits[k]!;
      march(d + coverM + probeM);
      const x = at(d + probeM);
      if (insideAny(walls, x)) continue;
      if (hits.some((h) => h > d + probeM && h <= d + coverM)) continue;
      return flood.reaches(x, n);
    }
  };
  return {
    cellM: flood.cellM,
    reach: (base, n, own) => {
      const p: Vec = [base[0] + n[0] * probeM, base[1] + n[1] * probeM, base[2] + n[2] * probeM];
      if (flood.reaches(p, n)) return "air";
      const hit = insideAny(walls, p, (solid) => own(solid) || !isOpen(solid)) ? -2 : flood.firstHit(p, [base[0] + n[0] * coverM, base[1] + n[1] * coverM, base[2] + n[2] * coverM]);
      if ((hit === -2 || (hit >= 0 && !own(walls[hit]!) && isOpen(walls[hit]!))) && beyond(p, n)) return "cover";
      return closure(base, n, own) ? undefined : "cavity";
    },
  };
}

/** 🔭️ Whether a face sample can be seen from outside when the solids that `clear` accepts are transparent (E3: the glazing regions): the probe `probeM` in front of it is reached by a flood through the other walls with cells of {@link FLOOD_FEATURES} features. A region that lies wholly in a closed cavity is declared `hidden` exactly when no face of it can be seen. */
export function seenFromOutside(walls: readonly World[], clear: (solid: World) => boolean, featureM: number, probeM: number = BURIAL_PROBE_M, tick: () => void = () => {}, minimumAirPassageM?: number): (base: Vec, normal: Vec) => Reach | undefined {
  const flood = floodOutside(walls.filter((solid) => !clear(solid)), airCell(featureM,minimumAirPassageM), tick);
  return (base, n) => (flood.reaches([base[0] + n[0] * probeM, base[1] + n[1] * probeM, base[2] + n[2] * probeM], n) ? "air" : undefined);
}

/** 🧿️ The outer area in m² of a set of meshes: {@link exposedArea} against each other, without planes. */
export const outerArea = (solids: readonly World[]): number => exposedArea(solids, solids);

/** 🔺️ The triangles of the solids at one fixed tessellation quality (chord 2 mm, the same number of segments for every radius), so that a count says how many solids and how much detail there are, not how large they are. */
export function fixedTriangleCount(solids: readonly VolumeSolid[]): number {
  const quality = { chordM: VOLUME_LIMITS.monotonicChordM, minSegments: VOLUME_LIMITS.monotonicSegments, maxSegments: VOLUME_LIMITS.monotonicSegments };
  return solids.reduce((sum, solid) => sum + (copyMesh(solid, quality).indices.length / 3) * (solid.repeat?.count ?? 1), 0);
}
