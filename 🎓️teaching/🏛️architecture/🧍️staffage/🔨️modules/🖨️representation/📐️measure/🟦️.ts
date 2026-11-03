/** 📐️ Measurements on paper geometry (millimetres) that decide print suitability: the white gap between the outlines of separate shapes and the minimum local width of a filled shape.
 *
 * **Gap.** For two outlines of different shapes, the ink gap is the smallest distance between their outline centrelines minus half of each pen. Outlines that touch, cross or whose
 * inked strokes overlap (ink gap ≤ 0) are joined on purpose and are not gaps; a gap is only reported for `0 < ink gap` (the caller compares it with the profile). The outlines measured
 * are the visible ink (hidden-line removed runs, see the shapes module), so a part covered by another part has no outline to merge with its neighbours.
 *
 * **Feature.** The thickness at a point of a filled shape is the diameter of the largest disk inside the shape that covers it. The feature width of a shape is the smallest width `w`
 * for which the area thinner than `w` (the part of the shape removed by a morphological opening with a disk of diameter `w`) exceeds the tolerance
 * `max(1 % of the area, (minFeature / 2)²) + 4·(1 − π/4)·(w/2)²` (the last term is the area four sharp corners lose to any opening), so a sharp tip or corner does not count, but a finger, spoke or sliver does. It is computed on a raster of pitch
 * `h ≤ minFeature / 6` (refined for small shapes, see {@link REFINE_CELLS}; the radius of a cell is its distance to the nearest outside cell centre less half a pixel, the distance to the boundary) with exact Euclidean distance transforms (linear time, visiting only the cells that can matter) and is accurate to about one pixel; a shape with no thin part reports twice its largest inscribed radius.
 *
 * @see ../../🎨️depiction/📐️geometry/🟦️.ts — planar helpers
 * @see https://doi.org/10.1007/0-306-47025-X_36 — Meijster, Roerdink & Hesselink, linear-time exact distance transforms
 */
import { type Ring, type Vec, pointInside } from "../../🎨️depiction/📐️geometry/🟦️.ts";

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

type Buffer = Uint8Array | Int32Array | Float64Array;
const arena = new Map<string, Buffer>();

/** ♻️ A zeroed view of `length` cells on the retained buffer of `slot`, grown only when it is too small: the rasters of one suitability check are sized by the shape box, so leasing replaces one allocation of up to 2.5 million cells per array and shape with one buffer per slot.
 * Slots never overlap in lifetime within one call; {@link releaseMeasureScratch} drops them all. */
function lease<T extends Buffer>(slot: string, kind: new (length: number) => T, length: number): T {
  let held = arena.get(slot) as T | undefined;
  if (held === undefined || held.length < length) arena.set(slot, (held = new kind(length)));
  const view = held.subarray(0, length) as T;
  view.fill(0);
  return view;
}

/** 🧹️ Drops every retained raster buffer, so a long-lived process keeps no megabytes of scratch after the measurement that needed them. */
export function releaseMeasureScratch(): void {
  arena.clear();
}

function leaseScratch(nx: number, ny: number, tag: string): DistanceScratch {
  return { grid: lease(`${tag}:grid`, Float64Array, nx * ny), vertical: lease(`${tag}:vertical`, Int32Array, nx * ny), stack: lease(`${tag}:stack`, Int32Array, nx), bounds: lease(`${tag}:bounds`, Int32Array, nx) };
}

export type DistanceScratch = { readonly grid: Float64Array; readonly vertical: Int32Array; readonly stack: Int32Array; readonly bounds: Int32Array };

/** 🧭️ Where a distance transform has to be right: `from[y]..to[y]` is the span of the cells read in row `y` (`to < from` for none), `cap` the largest vertical distance and `margin` the largest horizontal distance (pixels) of a source that can still matter.
 * Cells outside the spans, and values beyond the cap, are not defined. `sparse` promises that every source lies inside the spans, so only the spans (and their margins) are visited at all. */
export type DistanceWindow = { readonly from: Int32Array; readonly to: Int32Array; readonly margin: number; readonly cap: number; readonly sparse?: boolean };

function sparseVertical(source: Uint8Array, nx: number, ny: number, g: Int32Array, edge: Int32Array, window: DistanceWindow): boolean {
  const { from, to, margin } = window;
  const big = nx + ny;
  let any = false;
  edge.fill(ny + 2 * big);
  for (let y = ny - 1; y >= 0; y--) {
    const xa = from[y]!;
    const xb = to[y]!;
    if (xa > xb) continue;
    const row = y * nx;
    for (let x = xa; x <= xb; x++)
      if (source[row + x] === 1) {
        edge[x] = y;
        any = true;
      }
    const hi = Math.min(nx - 1, xb + margin);
    for (let x = Math.max(0, xa - margin); x <= hi; x++) {
      const d = edge[x]! - y;
      g[row + x] = d > big ? big : d;
    }
  }
  if (!any) return false;
  edge.fill(-ny - 2 * big);
  for (let y = 0; y < ny; y++) {
    const xa = from[y]!;
    const xb = to[y]!;
    if (xa > xb) continue;
    const row = y * nx;
    for (let x = xa; x <= xb; x++) if (source[row + x] === 1) edge[x] = y;
    const hi = Math.min(nx - 1, xb + margin);
    for (let x = Math.max(0, xa - margin); x <= hi; x++) {
      const d = y - edge[x]!;
      if (d < g[row + x]!) g[row + x] = d;
    }
  }
  return true;
}

export function makeScratch(nx: number, ny: number): DistanceScratch {
  return { grid: new Float64Array(nx * ny), vertical: new Int32Array(nx * ny), stack: new Int32Array(nx), bounds: new Int32Array(nx) };
}

/** 🔢️ The squared Euclidean distance (in pixels) of every cell to the nearest cell where `source` is 1: exact and linear in the number of cells (Meijster, Roerdink & Hesselink), so the values equal those of any other exact transform;
 * `1e20` everywhere when there is no source. The vertical phase sweeps whole rows (sequential memory), the horizontal phase keeps the lower envelope of the column parabolas on a stack.
 * With a `window` only the cells that are read, and the sources that can still be their nearest within the cap, are visited; every value that is read and not beyond the cap is exact.
 * @see https://doi.org/10.1007/0-306-47025-X_36 — Meijster, Roerdink & Hesselink, a general algorithm for computing distance transforms in linear time */
export function squaredDistanceTransform(source: Uint8Array, nx: number, ny: number, scratch?: DistanceScratch, window?: DistanceWindow): Float64Array {
  const { grid, vertical: g, stack: s, bounds: t } = scratch ?? makeScratch(nx, ny);
  const big = nx + ny;
  if (window?.sparse) {
    if (!sparseVertical(source, nx, ny, g, s, window)) {
      for (let y = 0; y < ny; y++) if (window.from[y]! <= window.to[y]!) grid.fill(INF, y * nx + window.from[y]!, y * nx + window.to[y]! + 1);
      return grid;
    }
  } else {
    let any = false;
    for (let x = 0; x < nx; x++) {
      if (source[x] === 1) any = true;
      g[x] = source[x] === 1 ? 0 : big;
    }
    for (let y = 1; y < ny; y++) {
      const row = y * nx;
      for (let x = 0; x < nx; x++) {
        if (source[row + x] === 1) {
          g[row + x] = 0;
          any = true;
        } else {
          const above = g[row - nx + x]!;
          g[row + x] = above >= big ? big : above + 1;
        }
      }
    }
    if (!any) {
      grid.fill(INF);
      return grid;
    }
    for (let y = ny - 2; y >= 0; y--) {
      const row = y * nx;
      for (let x = 0; x < nx; x++) {
        const below = g[row + nx + x]! + 1;
        if (below < g[row + x]!) g[row + x] = below;
      }
    }
  }
  const margin = window?.margin ?? nx;
  const cap = window?.cap ?? big;
  for (let y = 0; y < ny; y++) {
    const row = y * nx;
    const xa = window ? window.from[y]! : 0;
    const xb = window ? window.to[y]! : nx - 1;
    if (xa > xb) continue;
    const lo = Math.max(0, xa - margin);
    const hi = Math.min(nx - 1, xb + margin);
    let q = -1;
    let gs = 0;
    for (let u = lo; u <= hi; u++) {
      const gu = g[row + u]!;
      if (gu > cap) continue;
      while (q >= 0) {
        const i = s[q]!;
        const left = t[q]! - i;
        const right = t[q]! - u;
        if (left * left + gs * gs > right * right + gu * gu) {
          q--;
          if (q >= 0) gs = g[row + s[q]!]!;
        } else break;
      }
      if (q < 0) {
        q = 0;
        s[0] = u;
        t[0] = 0;
        gs = gu;
      } else {
        const i = s[q]!;
        const w = 1 + Math.floor((u * u + gu * gu - i * i - gs * gs) / (2 * (u - i)));
        if (w < nx) {
          q++;
          s[q] = u;
          t[q] = w;
          gs = gu;
        }
      }
    }
    if (q < 0) {
      grid.fill(INF, row + xa, row + xb + 1);
      continue;
    }
    while (q > 0 && t[q]! > xb) q--;
    for (let u = xb; u >= xa; u--) {
      const i = s[q]!;
      const gi = g[row + i]!;
      grid[row + u] = (u - i) * (u - i) + gi * gi;
      if (u === t[q]) q--;
    }
  }
  return grid;
}

function rasterize(rings: readonly Ring[], pitch: number, minX: number, minY: number, nx: number, ny: number, mask: Uint8Array = new Uint8Array(nx * ny)): Uint8Array {
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

/** 🔬️ The raster of a small shape is refined to about this many cells, so the width of a shape that is only a few pitches of `minFeature / 6` across is not rounded to whole pixels (down to a pitch of 2 µm). */
const REFINE_CELLS = 1.2e5;

const FEATURE_MEMO_LIMIT = 1024;
const MEMO_KEY_LIMIT = 20000;
const featureMemo = new Map<string, number>();
const visibleMemo = new Map<string, VisibleFeature | null>();

/** 🪡️ The feature width in millimetres of a filled shape (closed rings, even-odd), see the module description; `minFeatureMm` sets the raster pitch and the tip tolerance.
 * Pure in its arguments: a bounded memo (1024 shapes) answers the shapes that repeat unchanged across poses, casts and frames.
 * With `atLeastMm` the bisection stops as soon as it has proven the width is not below it and returns that proven lower bound (at least `atLeastMm`, at most the exact width, not memoised). */
export function featureWidthMm(rings: readonly Ring[], minFeatureMm: number, atLeastMm = Infinity): number {
  const key = `${minFeatureMm}|${JSON.stringify(rings)}`;
  const known = featureMemo.get(key);
  if (known !== undefined) return known;
  const { widthMm, exact } = measureFeatureWidthMm(rings, minFeatureMm, atLeastMm, []);
  if (!exact) return widthMm;
  if (featureMemo.size >= FEATURE_MEMO_LIMIT) featureMemo.delete(featureMemo.keys().next().value!);
  featureMemo.set(key, widthMm);
  return widthMm;
}

/** 🧱️ A nearer filled shape painted over another: its rings (even-odd) and the pen of its outline, which inks the paper `penMm / 2` beyond the edge. */
export type Cover = { readonly rings: readonly Ring[]; readonly penMm: number };

/** 👁️ The feature of what is visible of a filled shape: its width and the shorter side of the box of the visible part (both millimetres). */
export type VisibleFeature = { readonly widthMm: number; readonly boxMinMm: number };

/** 🙈️ The feature width of a filled shape after the nearer filled `covers` are painted over it: the shape is measured on its visible part only (the shape minus the covers grown by half their outline pen, which is inked), so a shape fully covered contributes nothing (`null`)
 * and a partly covered one is measured on what remains. Pieces of the visible part with an area below the tolerance `(minFeature / 2)²` are residue of coincident edges, not drawn parts, and are ignored. Same raster and criterion as {@link featureWidthMm}; with no occluder over the shape it equals it.
 * @see https://shapely.readthedocs.io/en/stable/manual.html#object.difference — shapely `difference` is the oracle of the visible part */
export function visibleFeatureWidthMm(rings: readonly Ring[], covers: readonly Cover[], minFeatureMm: number, atLeastMm = Infinity): VisibleFeature | null {
  const key = `${minFeatureMm}|${JSON.stringify(rings)}|${JSON.stringify(covers)}`;
  if (visibleMemo.has(key)) return visibleMemo.get(key)!;
  const { widthMm, exact, boxMinMm } = measureFeatureWidthMm(rings, minFeatureMm, atLeastMm, covers);
  const result = boxMinMm === undefined ? null : { widthMm, boxMinMm };
  if (!exact || key.length > MEMO_KEY_LIMIT) return result;
  if (visibleMemo.size >= FEATURE_MEMO_LIMIT) visibleMemo.delete(visibleMemo.keys().next().value!);
  visibleMemo.set(key, result);
  return result;
}

function dropSmallPieces(mask: Uint8Array, nx: number, ny: number, minCells: number): void {
  const seen = lease("drop:seen", Uint8Array, mask.length);
  const queue = lease("drop:queue", Int32Array, mask.length);
  for (let start = 0; start < mask.length; start++) {
    if (mask[start] !== 1 || seen[start] === 1) continue;
    let head = 0;
    let tail = 0;
    queue[tail++] = start;
    seen[start] = 1;
    while (head < tail) {
      const cur = queue[head++]!;
      const cx = cur % nx;
      const cy = (cur - cx) / nx;
      for (let dy = -1; dy <= 1; dy++) for (let dx = -1; dx <= 1; dx++) {
        const x = cx + dx;
        const y = cy + dy;
        if (x < 0 || y < 0 || x >= nx || y >= ny) continue;
        const nb = y * nx + x;
        if (mask[nb] === 1 && seen[nb] === 0) {
          seen[nb] = 1;
          queue[tail++] = nb;
        }
      }
    }
    if (tail < minCells) for (let k = 0; k < tail; k++) mask[queue[k]!] = 0;
  }
}

function lineVisible(rings: readonly Ring[], covers: readonly Cover[]): boolean {
  const hidden = (p: Vec): boolean =>
    covers.some((cover) => {
      if (pointInside(p, cover.rings)) return true;
      if (!(cover.penMm > 0)) return false;
      return cover.rings.some((ring) => ring.some((a, i) => segmentDistanceToPoint(p.x, p.y, a, ring[(i + 1) % ring.length]!) <= cover.penMm / 2));
    });
  for (const ring of rings)
    for (let i = 0; i + 1 < ring.length; i++) {
      const a = ring[i]!;
      const b = ring[i + 1]!;
      const steps = Math.max(1, Math.ceil(Math.hypot(b.x - a.x, b.y - a.y) / 0.01));
      for (let k = 0; k <= steps; k++) if (!hidden({ x: a.x + ((b.x - a.x) * k) / steps, y: a.y + ((b.y - a.y) * k) / steps })) return true;
    }
  return false;
}

function measureFeatureWidthMm(rings: readonly Ring[], minFeatureMm: number, atLeastMm: number, covers: readonly Cover[]): { readonly widthMm: number; readonly exact: boolean; readonly boxMinMm?: number } {
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
  if (minX === Infinity || maxX - minX <= 0 || maxY - minY <= 0) return minX !== Infinity && covers.length > 0 && !lineVisible(rings, covers) ? { widthMm: 0, exact: true } : { widthMm: 0, exact: true, boxMinMm: 0 };
  const boxArea = (maxX - minX) * (maxY - minY);
  const pitch = Math.max(Math.min(minFeatureMm / 6, Math.sqrt(boxArea / REFINE_CELLS)), Math.sqrt(boxArea / 2.5e6), 0.002);
  const nx = Math.ceil((maxX - minX) / pitch) + 4;
  const ny = Math.ceil((maxY - minY) / pitch) + 4;
  const x0 = minX - 2 * pitch;
  const y0 = minY - 2 * pitch;
  const mask = rasterize(rings, pitch, x0, y0, nx, ny, lease("mask", Uint8Array, nx * ny));
  const cut = covers.length > 0;
  if (cut) {
    const cutScratch = leaseScratch(nx, ny, "dt");
    const byPen = new Map<number, Uint8Array>();
    for (const cover of covers) {
      let union = byPen.get(cover.penMm);
      if (union === undefined) byPen.set(cover.penMm, (union = lease(`union:${byPen.size}`, Uint8Array, nx * ny)));
      rasterize(cover.rings, pitch, x0, y0, nx, ny, union);
    }
    for (const [penMm, union] of byPen) {
      const reach = penMm > 0 ? squaredDistanceTransform(union, nx, ny, cutScratch) : union;
      const limit = penMm > 0 ? ((penMm / 2) / pitch + 0.5) ** 2 : 0;
      for (let i = 0; i < mask.length; i++) if (penMm > 0 ? reach[i]! <= limit : union[i] === 1) mask[i] = 0;
    }
    dropSmallPieces(mask, nx, ny, Math.ceil((minFeatureMm / 2) ** 2 / (pitch * pitch)));
  }
  let inside = 0;
  const outside = lease("outside", Uint8Array, mask.length);
  for (let i = 0; i < mask.length; i++) {
    if (mask[i] === 1) inside++;
    else outside[i] = 1;
  }
  if (inside === 0) return { widthMm: 0, exact: true };
  const cells = lease("cells", Int32Array, inside);
  let cellMinX = nx;
  let cellMaxX = -1;
  let cellMinY = ny;
  let cellMaxY = -1;
  const from = lease("from", Int32Array, ny).fill(nx);
  const to = lease("to", Int32Array, ny).fill(-1);
  for (let i = 0, k = 0; i < mask.length; i++)
    if (mask[i] === 1) {
      cells[k++] = i;
      const y = Math.floor(i / nx);
      const x = i - y * nx;
      if (x < from[y]!) from[y] = x;
      if (x > to[y]!) to[y] = x;
      if (x < cellMinX) cellMinX = x;
      if (x > cellMaxX) cellMaxX = x;
      if (y < cellMinY) cellMinY = y;
      if (y > cellMaxY) cellMaxY = y;
    }
  const boxMinMm = Math.min(cellMaxX - cellMinX + 1, cellMaxY - cellMinY + 1) * pitch;
  const scratch = leaseScratch(nx, ny, "dt");
  const toOutside = squaredDistanceTransform(outside, nx, ny, scratch, { from, to, margin: 1, cap: nx + ny });
  const radius = lease("radius", Float64Array, mask.length);
  let rMax = 0;
  for (let c = 0; c < cells.length; c++) {
    const i = cells[c]!;
    radius[i] = (Math.sqrt(toOutside[i]!) - 0.5) * pitch;
    if (radius[i]! > rMax) rMax = radius[i]!;
  }
  const tolerance = Math.max(0.01 * inside * pitch * pitch, (minFeatureMm / 2) ** 2);
  const core = lease("core", Uint8Array, mask.length);
  const thinMask = lease("thin", Uint8Array, mask.length);
  const queue = lease("queue", Int32Array, mask.length);
  const exceeds = (width: number, allowedMm2: number): boolean => {
    const r = width / 2;
    for (let c = 0; c < cells.length; c++) core[cells[c]!] = radius[cells[c]!]! >= r - 1e-9 ? 1 : 0;
    const limit = (r / pitch) ** 2;
    const span = Math.ceil(r / pitch) + 1;
    const reach = squaredDistanceTransform(core, nx, ny, scratch, { from, to, margin: span, cap: span, sparse: true });
    let total = 0;
    for (let c = 0; c < cells.length; c++) {
      const on = radius[cells[c]!]! > pitch && reach[cells[c]!]! > limit ? 1 : 0;
      thinMask[cells[c]!] = on;
      total += on;
    }
    let found = false;
    if (total * pitch * pitch > allowedMm2)
      for (let c = 0; c < cells.length && !found; c++) {
        const i = cells[c]!;
        if (thinMask[i] !== 1) continue;
        thinMask[i] = 0;
        let head = 0;
        let tail = 0;
        queue[tail++] = i;
        let count = 0;
        while (head < tail && !found) {
          const cur = queue[head++]!;
          const cx = cur % nx;
          const cy = (cur - cx) / nx;
          count++;
          if (count * pitch * pitch > allowedMm2) found = true;
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
      }
    thinMask.fill(0);
    return found;
  };
  const top = 2 * rMax;
  const allowed = (width: number): number => tolerance + 4 * (1 - Math.PI / 4) * (width / 2) ** 2;
  const tested = new Map<number, boolean>();
  const thin = (width: number): boolean => {
    let known = tested.get(width);
    if (known === undefined) tested.set(width, (known = exceeds(width, allowed(width))));
    return known;
  };
  let probe = top;
  while (atLeastMm > 0 && probe / 2 >= atLeastMm) probe /= 2;
  if (probe < top && !thin(probe)) return { widthMm: probe, exact: false, boxMinMm };
  if (!thin(top)) return { widthMm: top, exact: true, boxMinMm };
  let low = 0;
  let high = top;
  for (let step = 0; step < 12; step++) {
    const mid = (low + high) / 2;
    if (thin(mid)) high = mid;
    else low = mid;
    if (low >= atLeastMm) return { widthMm: low, exact: false, boxMinMm };
  }
  return { widthMm: high, exact: true, boxMinMm };
}

/** 🫧️ A white sliver: negative space narrower than the minimum gap, with its width and the centroid of the offending area. */
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

/** 🥖️ The narrowest negative space of a drawing: the inked area is the union of the filled shapes and the strokes with their pens, the white space is its complement. A white region is a sliver where it is
 * thinner than `minGapMm` (the part not covered by any disk of that diameter inside the white space exceeds `3·w²`, which lets a tapering notch such as the crotch between legs end in a point, but not a long narrow channel).
 * Returns the width at which the criterion fails and the centroid of the uncovered area, or `null` when no negative space is narrower than `minGapMm`. Raster pitch is at most `minGapMm / 6`.
 * @see ../🟦️.ts — reported as a `gap` violation of `negative-space` */
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
  const ink = lease("ink", Uint8Array, nx * ny);
  for (const rings of fills) {
    const mask = rasterize(rings, pitch, x0, y0, nx, ny, lease("mask", Uint8Array, nx * ny));
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
  const scratch = leaseScratch(nx, ny, "dt");
  const toInk = squaredDistanceTransform(ink, nx, ny, scratch);
  const radius = lease("radius", Float64Array, ink.length);
  let rMax = 0;
  for (let i = 0; i < ink.length; i++)
    if (ink[i] === 0) {
      radius[i] = Math.max(0, (Math.sqrt(toInk[i]!) - 0.5) * pitch);
      if (radius[i]! > rMax) rMax = radius[i]!;
    }
  const core = lease("core", Uint8Array, ink.length);
  const thinMask = lease("thin", Uint8Array, ink.length);
  const queue = lease("queue", Int32Array, ink.length);
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
