/** 🧮️ The owned geometry measures of the volume rules: union volume by vertical columns, the volume of the union eroded and dilated by a distance on a voxel grid, the exposed area of a set of meshes and the triangle count at one fixed tessellation quality.
 *
 * THE REFERENCES are trimesh and scipy (point containment, Euclidean distance transform) in `🧪️tests/🧊️volumes-rules/🐍️.py`.
 *
 * @see ../🟦️.ts — `checkVolumes`, which uses them
 * @see ../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts — the kernel tessellation */
import type { VolumeSolid } from "../../../../🧬️schema/🟦️.ts";
import { outwardOrientation } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { VOLUME_LIMITS } from "../🧱️rules/🟦️.ts";
import { copyMesh, edt1, extentOf, type World } from "../🌓️silhouette/🟦️.ts";

type Vec = readonly [number, number, number];

const JITTER_X = 1.2345e-3;
const JITTER_Z = 7.6543e-4;

type Columns = { readonly box: ReturnType<typeof extentOf>; readonly nx: number; readonly nz: number; readonly spans: Map<number, [number, number][]> };

function columns(solids: readonly World[], cell: number, boxOf?: ReturnType<typeof extentOf>): Columns {
  const box = boxOf ?? extentOf(solids);
  const nx = Math.max(1, Math.ceil((box.max[0] - box.min[0]) / cell));
  const nz = Math.max(1, Math.ceil((box.max[2] - box.min[2]) / cell));
  const raw = new Map<number, number[][]>();
  solids.forEach((solid, k) => {
    const p = solid.positions;
    const hits = new Map<number, number[]>();
    for (let t = 0; t < solid.indices.length; t += 3) {
      const [a, b, c] = [solid.indices[t]! * 3, solid.indices[t + 1]! * 3, solid.indices[t + 2]! * 3];
      const [x0, x1, x2, z0, z1, z2] = [p[a]!, p[b]!, p[c]!, p[a + 2]!, p[b + 2]!, p[c + 2]!];
      const det = (x1 - x0) * (z2 - z0) - (x2 - x0) * (z1 - z0);
      if (Math.abs(det) < 1e-18) continue;
      const i0 = Math.max(0, Math.ceil((Math.min(x0, x1, x2) - box.min[0]) / cell - 0.5));
      const i1 = Math.min(nx - 1, Math.floor((Math.max(x0, x1, x2) - box.min[0]) / cell - 0.5));
      const j0 = Math.max(0, Math.ceil((Math.min(z0, z1, z2) - box.min[2]) / cell - 0.5));
      const j1 = Math.min(nz - 1, Math.floor((Math.max(z0, z1, z2) - box.min[2]) / cell - 0.5));
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
      const own = raw.get(key) ?? [];
      for (let n = 0; n + 1 < ys.length; n += 2) own.push([ys[n]!, ys[n + 1]!, k]);
      raw.set(key, own);
    }
  });
  const spans = new Map<number, [number, number][]>();
  for (const [key, list] of raw) {
    list.sort((u, v) => u[0]! - v[0]!);
    const merged: [number, number][] = [];
    for (const [a, b] of list as [number, number][]) {
      const last = merged[merged.length - 1];
      if (last !== undefined && a <= last[1]) last[1] = Math.max(last[1], b);
      else merged.push([a, b]);
    }
    spans.set(key, merged);
  }
  return { box, nx, nz, spans };
}

/** 🧮️ The volume of the union of closed meshes in m³: along every vertical column of a grid of `cell` metres (columns sampled at cell centres nudged by different thousandths of a cell in x and z, so that no column lies on a shared diagonal edge) the union of the intervals between the entering and leaving crossings of each solid, exact in y and sampled in x and z. */
export function unionVolume(solids: readonly World[], cell: number): number {
  if (solids.length === 0 || !Number.isFinite(extentOf(solids).min[0])) return 0;
  let total = 0;
  for (const spans of columns(solids, cell).spans.values()) for (const [a, b] of spans) total += b - a;
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
  const [spansFine, spansCoarse] = [columns(fine, h, box).spans, columns(coarse, h, box).spans];
  const margin = Math.ceil(band / h) + 1;
  const limit = (band / h) ** 2;
  const yRange = (lo: number, hi: number): [number, number] => [Math.ceil((lo - grown.min[1]!) / h - 0.5), Math.floor((hi - grown.min[1]!) / h - 0.5)];
  let [volFine, volCoarse, eroded, dilated] = [0, 0, 0, 0];
  for (let k0 = 0; k0 < nz; k0 += BAND_BLOCK)
    for (let i0 = 0; i0 < nx; i0 += BAND_BLOCK) {
      const [i1, k1] = [Math.min(nx, i0 + BAND_BLOCK), Math.min(nz, k0 + BAND_BLOCK)];
      const [e0, f0, e1, f1] = [Math.max(0, i0 - margin), Math.max(0, k0 - margin), Math.min(nx, i1 + margin), Math.min(nz, k1 + margin)];
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
  const out = new Float32Array(grid.length);
  for (let i = 0; i < grid.length; i++) out[i] = grid[i] === wanted ? 0 : big;
  const n = Math.max(nx, ny, nz);
  const [f, d, z] = [new Float64Array(n), new Float64Array(n), new Float64Array(n + 1)];
  const v = new Int32Array(n);
  const pass = (length: number, stride: number, starts: readonly number[]): void => {
    for (const start of starts) {
      for (let q = 0; q < length; q++) f[q] = out[start + q * stride]!;
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

const ORIENTATIONS = new WeakMap<World, { readonly normals: Float64Array; readonly closed: boolean }>();

/** 🧭️ The outward unit normals (three numbers per triangle) and the closedness of a mesh, by the kernel's {@link outwardOrientation}: consistent winding by a walk over manifold edges, a closed piece turned by the sign of its volume, an open piece (a loft with a cap off) keeping the outward winding of the tessellator and turned when the bone frame is left-handed. Cached per mesh; the answer never depends on a solid that touches it. */
export function orientationOf(solid: World): { readonly normals: Float64Array; readonly closed: boolean } {
  let hit = ORIENTATIONS.get(solid);
  if (hit === undefined) {
    const f = solid.frame;
    const left = f !== undefined && f.u.x * (f.v.y * f.w.z - f.v.z * f.w.y) + f.u.y * (f.v.z * f.w.x - f.v.x * f.w.z) + f.u.z * (f.v.x * f.w.y - f.v.y * f.w.x) < 0;
    const { normals, closed } = outwardOrientation(solid.positions, solid.indices, { openWinding: left ? -1 : 1 });
    hit = { normals, closed };
    ORIENTATIONS.set(solid, hit);
  }
  return hit;
}

/** 🔒️ Whether the mesh is closed: every edge is used by exactly two triangles (the kernel's closedness). Only closed meshes have an inside; an open skin buries nothing. */
export const isClosed = (solid: World): boolean => orientationOf(solid).closed;

type Grid = { readonly min: Vec; readonly max: Vec; readonly n: number; readonly bins: Int32Array[]; readonly stamp: Int32Array; stampValue: number };

const GRIDS = new WeakMap<World, Grid>();

function gridOf(solid: World): Grid {
  let grid = GRIDS.get(solid);
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
  grid = { min, max, n, bins: lists.map((l) => Int32Array.from(l)), stamp: new Int32Array(indices.length / 3), stampValue: 0 };
  GRIDS.set(solid, grid);
  return grid;
}

const BOXES = new WeakMap<World, { readonly min: Vec; readonly max: Vec }>();

/** 📦️ The bounding box of a mesh, cached. */
function boxOf(solid: World): { readonly min: Vec; readonly max: Vec } {
  let hit = BOXES.get(solid);
  if (hit === undefined) {
    const [min, max]: [[number, number, number], [number, number, number]] = [[Infinity, Infinity, Infinity], [-Infinity, -Infinity, -Infinity]];
    const q = solid.positions;
    for (let i = 0; i < q.length; i += 3) for (let k = 0; k < 3; k++) {
      min[k] = Math.min(min[k]!, q[i + k]!);
      max[k] = Math.max(max[k]!, q[i + k]!);
    }
    BOXES.set(solid, (hit = { min, max }));
  }
  return hit;
}

const SKEW: Vec = [0.000123, 0.000317, 1];

/** 🔎️ Whether the point is inside any of the closed meshes that `skip` does not exclude: parity of the crossings of a slightly skewed ray along +y (Möller-Trumbore), candidates taken from a grid over x and z of each mesh. */
export function insideAny(solids: readonly World[], p: Vec, skip: (solid: World, index: number) => boolean = () => false): boolean {
  const d: Vec = [SKEW[0], SKEW[2], SKEW[1]];
  for (let k = 0; k < solids.length; k++) {
    const solid = solids[k]!;
    if (skip(solid, k) || !isClosed(solid)) continue;
    const g = gridOf(solid);
    const drift = 0.0004 * (g.max[1] - g.min[1]) + 1e-9;
    if (p[0] < g.min[0] - drift || p[0] > g.max[0] + drift || p[2] < g.min[2] - drift || p[2] > g.max[2] + drift || p[1] > g.max[1]) continue;
    const { positions: q, indices } = solid;
    const at = (v: number, lo: number, hi: number): number => Math.max(0, Math.min(g.n - 1, Math.floor(((v - lo) / Math.max(hi - lo, 1e-12)) * g.n)));
    const [i0, i1] = [at(p[0] - drift, g.min[0], g.max[0]), at(p[0] + drift, g.min[0], g.max[0])];
    const [j0, j1] = [at(p[2] - drift, g.min[2], g.max[2]), at(p[2] + drift, g.min[2], g.max[2])];
    g.stampValue++;
    let crossings = 0;
    for (let j = j0; j <= j1; j++)
      for (let i = i0; i <= i1; i++)
        for (const t of g.bins[j * g.n + i]!) {
          if (g.stamp[t / 3] === g.stampValue) continue;
          g.stamp[t / 3] = g.stampValue;
          const [a, b, c] = [indices[t]! * 3, indices[t + 1]! * 3, indices[t + 2]! * 3];
          const e1: Vec = [q[b]! - q[a]!, q[b + 1]! - q[a + 1]!, q[b + 2]! - q[a + 2]!];
          const e2: Vec = [q[c]! - q[a]!, q[c + 1]! - q[a + 1]!, q[c + 2]! - q[a + 2]!];
          const h: Vec = [d[1] * e2[2] - d[2] * e2[1], d[2] * e2[0] - d[0] * e2[2], d[0] * e2[1] - d[1] * e2[0]];
          const det = e1[0] * h[0] + e1[1] * h[1] + e1[2] * h[2];
          if (Math.abs(det) < 1e-18) continue;
          const s: Vec = [p[0] - q[a]!, p[1] - q[a + 1]!, p[2] - q[a + 2]!];
          const u = (s[0] * h[0] + s[1] * h[1] + s[2] * h[2]) / det;
          if (u < 0 || u > 1) continue;
          const r: Vec = [s[1] * e1[2] - s[2] * e1[1], s[2] * e1[0] - s[0] * e1[2], s[0] * e1[1] - s[1] * e1[0]];
          const v = (d[0] * r[0] + d[1] * r[1] + d[2] * r[2]) / det;
          if (v < 0 || u + v > 1) continue;
          if ((e2[0] * r[0] + e2[1] * r[1] + e2[2] * r[2]) / det > 0) crossings++;
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

/** 🧿️ The exposed area in m² of the triangles of `target`: faces that are not buried inside a mesh of `against` (a probe moved `probeM` out of the face, 0.5 mm by default and 0.02 smallest features in the rules, along its normal lies inside it; `ignore` excludes meshes; with `contactM` above the probe a face is also buried when a probe at half that distance or at that distance lies inside, so that two solids that meet within `contactM` of each other, as the facets of two tessellations of one curved surface do, count as touching and the seam between them is no surface), not lying in a contact plane and not within 45 degrees of a declared `sectionFaces` axis of their solid. */
export function exposedArea(target: readonly World[], against: readonly World[], planes: readonly ContactPlane[] = [], ignore: (solid: World) => boolean = () => false, probeM: number = BURIAL_PROBE_M, contactM = 0): number {
  let area = 0;
  const cos = Math.cos((VOLUME_LIMITS.sectionAngleDeg * Math.PI) / 180);
  for (const solid of target) {
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
      if (around.length === 0) {
        area += twice / 2;
        continue;
      }
      const longest = Math.max(Math.hypot(e1[0]!, e1[1]!, e1[2]!), Math.hypot(e2[0]!, e2[1]!, e2[2]!), Math.hypot(e2[0]! - e1[0]!, e2[1]! - e1[1]!, e2[2]! - e1[2]!));
      const weights = subdivision(Math.min(BURIAL_MAX_SPLITS, Math.max(1, Math.ceil(longest / (2 * (probeM / BURIAL_PROBE_FEATURE))))));
      let free = 0;
      const reach = contactM > probeM ? [probeM, contactM / 2, contactM] : [probeM];
      for (const [wa, wb, wc] of weights) {
        const base: Vec = [wa * q[a]! + wb * q[b]! + wc * q[c]!, wa * q[a + 1]! + wb * q[b + 1]! + wc * q[c + 1]!, wa * q[a + 2]! + wb * q[b + 2]! + wc * q[c + 2]!];
        if (!reach.some((r) => insideAny(around, [base[0] + n[0] * r, base[1] + n[1] * r, base[2] + n[2] * r]))) free++;
      }
      area += (twice / 2) * (free / weights.length);
    }
  }
  return area;
}

/** 🧿️ The outer area in m² of a set of meshes: {@link exposedArea} against each other, without planes. */
export const outerArea = (solids: readonly World[]): number => exposedArea(solids, solids);

/** 🔺️ The triangles of the solids at one fixed tessellation quality (chord 2 mm, the same number of segments for every radius), so that a count says how many solids and how much detail there are, not how large they are. */
export function fixedTriangleCount(solids: readonly VolumeSolid[]): number {
  const quality = { chordM: VOLUME_LIMITS.monotonicChordM, minSegments: VOLUME_LIMITS.monotonicSegments, maxSegments: VOLUME_LIMITS.monotonicSegments };
  return solids.reduce((sum, solid) => sum + (copyMesh(solid, quality).indices.length / 3) * (solid.repeat?.count ?? 1), 0);
}
