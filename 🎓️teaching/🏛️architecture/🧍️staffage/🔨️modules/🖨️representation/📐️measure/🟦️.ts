/** 📐️ Measurements on paper geometry (millimetres) that decide print suitability: the white gap between the outlines of separate shapes and the minimum local width of a filled shape.
 *
 * **Gap.** For two outlines of different shapes, the ink gap is the smallest distance between their outline centrelines minus half of each pen. Outlines that touch, cross or whose
 * inked strokes overlap (ink gap ≤ 0) are joined on purpose and are not gaps; a gap is only reported for `0 < ink gap` (the caller compares it with the profile). The outlines measured
 * are the visible ink (hidden-line removed runs, see the shapes module), so a part covered by another part has no outline to merge with its neighbours.
 *
 * **Feature.** The thickness at a point of a filled shape is the diameter of the largest disk inside the shape that covers it. The feature width of a shape is the smallest width `w`
 * for which the area thinner than `w` (the part of the shape removed by a morphological opening with a disk of diameter `w`) exceeds the tolerance
 * `max(1 % of the area, (minFeature / 2)²) + 4·(1 − π/4)·(w/2)²` (the last term is the area four sharp corners lose to any opening), so a sharp tip or corner does not count, but a finger, spoke or sliver does. It is computed on a raster of pitch
 * `h ≤ minFeature / 6` with exact Euclidean distance transforms and is accurate to about one pixel; a shape with no thin part reports twice its largest inscribed radius.
 *
 * @see ../../🎨️depiction/📐️geometry/🟦️.ts — planar helpers
 * @see https://doi.org/10.1007/978-3-642-33191-6_4 — Felzenszwalb & Huttenlocher, distance transforms of sampled functions
 */
import type { Ring, Vec } from "../../🎨️depiction/📐️geometry/🟦️.ts";

/** ✏️ One drawn shape on paper: its rings (closed polygons or open polylines, y up or down alike), whether the rings are closed and the pen of its outline. */
export type PaperShape = { readonly id: string; readonly rings: readonly Ring[]; readonly closed: boolean; readonly penMm: number };

/** 📏️ A measured gap between two shapes of one drawing. */
export type Gap = { readonly a: string; readonly b: string; readonly gapMm: number; readonly centreMm: number; readonly at: Vec };

const TOUCH_MM = 1e-4;

function segmentDistance(p: Vec, q: Vec, r: Vec, s: Vec): { readonly d: number; readonly a: Vec; readonly b: Vec } {
  const d1x = q.x - p.x;
  const d1y = q.y - p.y;
  const d2x = s.x - r.x;
  const d2y = s.y - r.y;
  const denom = d1x * d2y - d1y * d2x;
  if (Math.abs(denom) > 1e-14) {
    const t = ((r.x - p.x) * d2y - (r.y - p.y) * d2x) / denom;
    const u = ((r.x - p.x) * d1y - (r.y - p.y) * d1x) / denom;
    if (t >= 0 && t <= 1 && u >= 0 && u <= 1) {
      const hit = { x: p.x + t * d1x, y: p.y + t * d1y };
      return { d: 0, a: hit, b: hit };
    }
  }
  const candidates: { d: number; a: Vec; b: Vec }[] = [];
  const onSegment = (x: Vec, a: Vec, b: Vec): Vec => {
    const dx = b.x - a.x;
    const dy = b.y - a.y;
    const l2 = dx * dx + dy * dy;
    const t = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((x.x - a.x) * dx + (x.y - a.y) * dy) / l2));
    return { x: a.x + t * dx, y: a.y + t * dy };
  };
  for (const [x, a, b, flip] of [[p, r, s, false], [q, r, s, false], [r, p, q, true], [s, p, q, true]] as const) {
    const c = onSegment(x, a, b);
    candidates.push(flip ? { d: Math.hypot(x.x - c.x, x.y - c.y), a: c, b: x } : { d: Math.hypot(x.x - c.x, x.y - c.y), a: x, b: c });
  }
  return candidates.reduce((best, c) => (c.d < best.d ? c : best));
}

function edgesOf(shape: PaperShape): { readonly a: Vec; readonly b: Vec }[] {
  const edges: { a: Vec; b: Vec }[] = [];
  for (const ring of shape.rings) {
    const n = ring.length;
    for (let i = 0; i + 1 < n; i++) edges.push({ a: ring[i]!, b: ring[i + 1]! });
    if (shape.closed && n > 2) edges.push({ a: ring[n - 1]!, b: ring[0]! });
  }
  return edges;
}

type Boxed = { readonly shape: PaperShape; readonly edges: { readonly a: Vec; readonly b: Vec }[]; readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number };

function boxed(shape: PaperShape): Boxed | null {
  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  for (const ring of shape.rings) for (const p of ring) {
    if (p.x < minX) minX = p.x;
    if (p.x > maxX) maxX = p.x;
    if (p.y < minY) minY = p.y;
    if (p.y > maxY) maxY = p.y;
  }
  return minX === Infinity ? null : { shape, edges: edgesOf(shape), minX, minY, maxX, maxY };
}

/** 🕳️ The ink gaps between every pair of separate shapes of one drawing that lie within `searchMm` of each other (outline centreline distance), see the module description. */
export function gapsBetween(shapes: readonly PaperShape[], searchMm: number): Gap[] {
  const items = shapes.flatMap((shape) => {
    const b = boxed(shape);
    return b === null ? [] : [b];
  });
  const gaps: Gap[] = [];
  for (let i = 0; i < items.length; i++) {
    for (let j = i + 1; j < items.length; j++) {
      const a = items[i]!;
      const b = items[j]!;
      if (a.shape.id === b.shape.id) continue;
      const reach = searchMm + (a.shape.penMm + b.shape.penMm) / 2;
      if (a.minX > b.maxX + reach || b.minX > a.maxX + reach || a.minY > b.maxY + reach || b.minY > a.maxY + reach) continue;
      let best = Infinity;
      let at: Vec = a.edges[0]?.a ?? { x: 0, y: 0 };
      for (const ea of a.edges) {
        const eaMinX = Math.min(ea.a.x, ea.b.x);
        const eaMaxX = Math.max(ea.a.x, ea.b.x);
        const eaMinY = Math.min(ea.a.y, ea.b.y);
        const eaMaxY = Math.max(ea.a.y, ea.b.y);
        const cap = Math.min(best, reach);
        if (eaMaxX + cap < b.minX || b.maxX + cap < eaMinX || eaMaxY + cap < b.minY || b.maxY + cap < eaMinY) continue;
        for (const eb of b.edges) {
          const limit = Math.min(best, reach);
          if (eaMaxX + limit < Math.min(eb.a.x, eb.b.x) || Math.max(eb.a.x, eb.b.x) + limit < eaMinX || eaMaxY + limit < Math.min(eb.a.y, eb.b.y) || Math.max(eb.a.y, eb.b.y) + limit < eaMinY) continue;
          const s = segmentDistance(ea.a, ea.b, eb.a, eb.b);
          if (s.d < best) {
            best = s.d;
            at = { x: (s.a.x + s.b.x) / 2, y: (s.a.y + s.b.y) / 2 };
            if (best <= TOUCH_MM) break;
          }
        }
        if (best <= TOUCH_MM) break;
      }
      if (best <= TOUCH_MM || best > reach) continue;
      const ink = best - (a.shape.penMm + b.shape.penMm) / 2;
      if (ink <= 0) continue;
      gaps.push({ a: a.shape.id, b: b.shape.id, gapMm: ink, centreMm: best, at });
    }
  }
  return gaps;
}

const INF = 1e20;

function distance1d(f: Float64Array, n: number, d: Float64Array, v: Int32Array, z: Float64Array): void {
  let k = 0;
  v[0] = 0;
  z[0] = -INF;
  z[1] = INF;
  for (let q = 1; q < n; q++) {
    let s = (f[q]! + q * q - (f[v[k]!]! + v[k]! * v[k]!)) / (2 * q - 2 * v[k]!);
    while (s <= z[k]!) {
      k--;
      s = (f[q]! + q * q - (f[v[k]!]! + v[k]! * v[k]!)) / (2 * q - 2 * v[k]!);
    }
    k++;
    v[k] = q;
    z[k] = s;
    z[k + 1] = INF;
  }
  k = 0;
  for (let q = 0; q < n; q++) {
    while (z[k + 1]! < q) k++;
    d[q] = (q - v[k]!) * (q - v[k]!) + f[v[k]!]!;
  }
}

type DistanceScratch = { readonly grid: Float64Array; readonly f: Float64Array; readonly d: Float64Array; readonly v: Int32Array; readonly z: Float64Array };

function makeScratch(nx: number, ny: number): DistanceScratch {
  const size = Math.max(nx, ny);
  return { grid: new Float64Array(nx * ny), f: new Float64Array(size), d: new Float64Array(size), v: new Int32Array(size), z: new Float64Array(size + 1) };
}

/** 📐️ The squared Euclidean distance (in pixels) of every cell to the nearest cell where `source` is 1 (Felzenszwalb–Huttenlocher, exact, O(n)). */
export function squaredDistanceTransform(source: Uint8Array, nx: number, ny: number, scratch?: DistanceScratch): Float64Array {
  const { grid, f, d, v, z } = scratch ?? makeScratch(nx, ny);
  for (let i = 0; i < grid.length; i++) grid[i] = source[i] === 1 ? 0 : INF;
  for (let x = 0; x < nx; x++) {
    for (let y = 0; y < ny; y++) f[y] = grid[y * nx + x]!;
    distance1d(f, ny, d, v, z);
    for (let y = 0; y < ny; y++) grid[y * nx + x] = d[y]!;
  }
  for (let y = 0; y < ny; y++) {
    for (let x = 0; x < nx; x++) f[x] = grid[y * nx + x]!;
    distance1d(f, nx, d, v, z);
    for (let x = 0; x < nx; x++) grid[y * nx + x] = d[x]!;
  }
  return grid;
}

function rasterize(rings: readonly Ring[], pitch: number, minX: number, minY: number, nx: number, ny: number): Uint8Array {
  const mask = new Uint8Array(nx * ny);
  for (let row = 0; row < ny; row++) {
    const y = minY + (row + 0.5) * pitch;
    const xs: number[] = [];
    for (const ring of rings)
      for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
        const a = ring[i]!;
        const b = ring[j]!;
        if (a.y > y !== b.y > y) xs.push(a.x + ((y - a.y) / (b.y - a.y)) * (b.x - a.x));
      }
    xs.sort((p, q) => p - q);
    for (let k = 0; k + 1 < xs.length; k += 2) {
      const from = Math.max(0, Math.ceil((xs[k]! - minX) / pitch - 0.5));
      const to = Math.min(nx - 1, Math.floor((xs[k + 1]! - minX) / pitch - 0.5));
      for (let col = from; col <= to; col++) mask[row * nx + col] = 1;
    }
  }
  return mask;
}

/** 📏️ The feature width in millimetres of a filled shape (closed rings, even-odd), see the module description; `minFeatureMm` sets the raster pitch and the tip tolerance. */
export function featureWidthMm(rings: readonly Ring[], minFeatureMm: number): number {
  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  for (const ring of rings) for (const p of ring) {
    minX = Math.min(minX, p.x);
    maxX = Math.max(maxX, p.x);
    minY = Math.min(minY, p.y);
    maxY = Math.max(maxY, p.y);
  }
  if (minX === Infinity || maxX - minX <= 0 || maxY - minY <= 0) return 0;
  const pitch = Math.max(minFeatureMm / 6, Math.sqrt(((maxX - minX) * (maxY - minY)) / 2.5e6), 0.002);
  const nx = Math.ceil((maxX - minX) / pitch) + 4;
  const ny = Math.ceil((maxY - minY) / pitch) + 4;
  const x0 = minX - 2 * pitch;
  const y0 = minY - 2 * pitch;
  const mask = rasterize(rings, pitch, x0, y0, nx, ny);
  let inside = 0;
  const outside = new Uint8Array(mask.length);
  for (let i = 0; i < mask.length; i++) {
    if (mask[i] === 1) inside++;
    else outside[i] = 1;
  }
  if (inside === 0) return 0;
  const scratch = makeScratch(nx, ny);
  const toOutside = squaredDistanceTransform(outside, nx, ny, scratch);
  const radius = new Float64Array(mask.length);
  let rMax = 0;
  for (let i = 0; i < mask.length; i++)
    if (mask[i] === 1) {
      radius[i] = Math.sqrt(toOutside[i]!) * pitch;
      if (radius[i]! > rMax) rMax = radius[i]!;
    }
  const tolerance = Math.max(0.01 * inside * pitch * pitch, (minFeatureMm / 2) ** 2);
  const core = new Uint8Array(mask.length);
  const thinMask = new Uint8Array(mask.length);
  const queue = new Int32Array(mask.length);
  const thin = (width: number): number => {
    const r = width / 2;
    for (let i = 0; i < mask.length; i++) core[i] = mask[i] === 1 && radius[i]! >= r - 1e-9 ? 1 : 0;
    const reach = squaredDistanceTransform(core, nx, ny, scratch);
    const limit = (r / pitch) ** 2;
    for (let i = 0; i < mask.length; i++) thinMask[i] = mask[i] === 1 && radius[i]! > pitch && reach[i]! > limit ? 1 : 0;
    let best = 0;
    for (let i = 0; i < mask.length; i++) {
      if (thinMask[i] !== 1) continue;
      thinMask[i] = 0;
      let head = 0;
      let tail = 0;
      queue[tail++] = i;
      let count = 0;
      while (head < tail) {
        const cur = queue[head++]!;
        const cx = cur % nx;
        const cy = (cur - cx) / nx;
        count++;
        for (let dy = -1; dy <= 1; dy++) {
          const ny0 = cy + dy;
          if (ny0 < 0 || ny0 >= ny) continue;
          for (let dx = -1; dx <= 1; dx++) {
            const nx0 = cx + dx;
            if (nx0 < 0 || nx0 >= nx) continue;
            const nb = ny0 * nx + nx0;
            if (thinMask[nb] === 1) {
              thinMask[nb] = 0;
              queue[tail++] = nb;
            }
          }
        }
      }
      if (count > best) best = count;
    }
    return best * pitch * pitch;
  };
  const top = 2 * rMax;
  const allowed = (width: number): number => tolerance + 4 * (1 - Math.PI / 4) * (width / 2) ** 2;
  if (thin(top) <= allowed(top)) return top;
  let low = 0;
  let high = top;
  for (let step = 0; step < 12; step++) {
    const mid = (low + high) / 2;
    if (thin(mid) > allowed(mid)) high = mid;
    else low = mid;
  }
  return high;
}


/** 🕳️ A white sliver: negative space narrower than the minimum gap, with its width and the centroid of the offending area. */
export type Sliver = { readonly widthMm: number; readonly at: Vec };

/** 🖋️ A visible stroke on paper: polyline (closed or not) and its pen. */
export type Stroke = { readonly points: Ring; readonly closed: boolean; readonly penMm: number };

function segmentDistanceToPoint(px: number, py: number, a: Vec, b: Vec): number {
  const dx = b.x - a.x;
  const dy = b.y - a.y;
  const l2 = dx * dx + dy * dy;
  const t = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((px - a.x) * dx + (py - a.y) * dy) / l2));
  return Math.hypot(px - (a.x + t * dx), py - (a.y + t * dy));
}

/** 🕳️ The narrowest negative space of a drawing: the inked area is the union of the filled shapes and the strokes with their pens, the white space is its complement. A white region is a sliver where it is
 * thinner than `minGapMm` (the part not covered by any disk of that diameter inside the white space exceeds `3·w²`, which lets a tapering notch such as the crotch between legs end in a point, but not a long narrow channel).
 * Returns the width at which the criterion fails and the centroid of the uncovered area, or `null` when no negative space is narrower than `minGapMm`. Raster pitch is at most `minGapMm / 6`.
 * @see ../../../🖨️representation/🟦️.ts — reported as a `gap` violation of `negative-space` */
export function whiteSliver(fills: readonly (readonly Ring[])[], strokes: readonly Stroke[], minGapMm: number): Sliver | null {
  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  let pen = 0;
  for (const ring of fills.flat()) for (const p of ring) {
    minX = Math.min(minX, p.x); maxX = Math.max(maxX, p.x); minY = Math.min(minY, p.y); maxY = Math.max(maxY, p.y);
  }
  for (const stroke of strokes) {
    pen = Math.max(pen, stroke.penMm);
    for (const p of stroke.points) {
      minX = Math.min(minX, p.x); maxX = Math.max(maxX, p.x); minY = Math.min(minY, p.y); maxY = Math.max(maxY, p.y);
    }
  }
  if (minX === Infinity || !(minGapMm > 0)) return null;
  const margin = 2 * minGapMm + pen;
  const pitch = Math.max(minGapMm / 6, Math.sqrt(((maxX - minX + 2 * margin) * (maxY - minY + 2 * margin)) / 2.5e6));
  const x0 = minX - margin;
  const y0 = minY - margin;
  const nx = Math.ceil((maxX - minX + 2 * margin) / pitch) + 1;
  const ny = Math.ceil((maxY - minY + 2 * margin) / pitch) + 1;
  const ink = new Uint8Array(nx * ny);
  for (const rings of fills) {
    const mask = rasterize(rings, pitch, x0, y0, nx, ny);
    for (let i = 0; i < ink.length; i++) if (mask[i] === 1) ink[i] = 1;
  }
  for (const stroke of strokes) {
    const half = stroke.penMm / 2;
    const n = stroke.points.length;
    const count = stroke.closed && n > 2 ? n : n - 1;
    for (let i = 0; i < count; i++) {
      const a = stroke.points[i]!;
      const b = stroke.points[(i + 1) % n]!;
      const cx0 = Math.max(0, Math.floor((Math.min(a.x, b.x) - half - x0) / pitch - 0.5));
      const cx1 = Math.min(nx - 1, Math.ceil((Math.max(a.x, b.x) + half - x0) / pitch - 0.5));
      const cy0 = Math.max(0, Math.floor((Math.min(a.y, b.y) - half - y0) / pitch - 0.5));
      const cy1 = Math.min(ny - 1, Math.ceil((Math.max(a.y, b.y) + half - y0) / pitch - 0.5));
      for (let cy = cy0; cy <= cy1; cy++) for (let cx = cx0; cx <= cx1; cx++)
        if (segmentDistanceToPoint(x0 + (cx + 0.5) * pitch, y0 + (cy + 0.5) * pitch, a, b) <= half) ink[cy * nx + cx] = 1;
    }
  }
  const scratch = makeScratch(nx, ny);
  const toInk = squaredDistanceTransform(ink, nx, ny, scratch);
  const radius = new Float64Array(ink.length);
  let rMax = 0;
  for (let i = 0; i < ink.length; i++)
    if (ink[i] === 0) {
      radius[i] = Math.max(0, (Math.sqrt(toInk[i]!) - 0.5) * pitch);
      if (radius[i]! > rMax) rMax = radius[i]!;
    }
  const core = new Uint8Array(ink.length);
  const thinMask = new Uint8Array(ink.length);
  const queue = new Int32Array(ink.length);
  const uncovered = (width: number): { readonly area: number; readonly x: number; readonly y: number } => {
    const r = width / 2;
    for (let i = 0; i < ink.length; i++) core[i] = ink[i] === 0 && radius[i]! >= r ? 1 : 0;
    const reach = squaredDistanceTransform(core, nx, ny, scratch);
    const limit = ((r + pitch * 0.5) / pitch) ** 2;
    for (let i = 0; i < ink.length; i++) thinMask[i] = ink[i] === 0 && reach[i]! > limit ? 1 : 0;
    let bestCount = 0;
    let bestSx = 0;
    let bestSy = 0;
    for (let i = 0; i < ink.length; i++) {
      if (thinMask[i] !== 1) continue;
      thinMask[i] = 0;
      let head = 0;
      let tail = 0;
      queue[tail++] = i;
      let count = 0;
      let sx = 0;
      let sy = 0;
      while (head < tail) {
        const cur = queue[head++]!;
        const cx = cur % nx;
        const cy = (cur - cx) / nx;
        count++;
        sx += x0 + (cx + 0.5) * pitch;
        sy += y0 + (cy + 0.5) * pitch;
        for (let dy = -1; dy <= 1; dy++) {
          const ny0 = cy + dy;
          if (ny0 < 0 || ny0 >= ny) continue;
          for (let dx = -1; dx <= 1; dx++) {
            const nx0 = cx + dx;
            if (nx0 < 0 || nx0 >= nx) continue;
            const nb = ny0 * nx + nx0;
            if (thinMask[nb] === 1) {
              thinMask[nb] = 0;
              queue[tail++] = nb;
            }
          }
        }
      }
      if (count > bestCount) {
        bestCount = count;
        bestSx = sx;
        bestSy = sy;
      }
    }
    return { area: bestCount * pitch * pitch, x: bestCount === 0 ? 0 : bestSx / bestCount, y: bestCount === 0 ? 0 : bestSy / bestCount };
  };
  const allowed = (width: number): number => 3 * width * width + 4 * pitch * pitch;
  if (rMax <= 0 || uncovered(minGapMm).area <= allowed(minGapMm)) return null;
  let low = 0;
  let high = minGapMm;
  for (let step = 0; step < 10; step++) {
    const mid = (low + high) / 2;
    if (uncovered(mid).area > allowed(mid)) high = mid;
    else low = mid;
  }
  const at = uncovered(minGapMm);
  return { widthMm: high, at: { x: at.x, y: at.y } };
}
