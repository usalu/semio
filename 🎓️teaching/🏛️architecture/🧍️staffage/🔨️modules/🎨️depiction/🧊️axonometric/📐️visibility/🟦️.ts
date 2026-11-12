/** 📐️ Exact hidden-surface and hidden-line removal of an orthographic view of triangles, in vector form: no raster, no z-buffer.
 *
 * View space is right-handed with `x` to the right, `y` up and `z` towards the viewer (larger is nearer); a triangle is front-facing when its counter-clockwise winding in `(x, y)` is positive, which is the outward winding of the kernel tessellator.
 * Every opaque front-facing triangle is an occluder; glass triangles hide nothing. The depth of a triangle is an affine function of `(x, y)`, so everything below is exact up to the tolerance `eps`:
 *
 * - {@link Visibility.fragments}: the visible part of a triangle as convex polygons. The triangle is cut against every nearer overlapping occluder in order of nearness; where the depth difference of the two planes changes sign inside the overlap (interpenetration) the overlap is split along the intersection line of the planes, so interpenetrating and cyclically overlapping solids come out right without any ordering assumption. Coplanar overlaps are decided by index, so exactly one triangle stays.
 * - {@link Visibility.hidden}: the stretches of a 3D segment hidden by occluders as parameter intervals; the visible rest is the stroke (Appel/Galimberti-Montanari hidden-line removal with exact interval arithmetic along the segment).
 *
 * A uniform grid over the projected bounding boxes bounds the candidates of each query; the order of everything is by index, so the result is deterministic.
 *
 * @see https://doi.org/10.1145/1468075.1468082 — Appel, "The notion of quantitative invisibility and the machine rendering of solids"
 */
import * as portable from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 🔺️ Triangles in view space: `vertices` holds `x, y, z` per vertex, `indices` three vertices per triangle (counter-clockwise outward), `opaque` is 1 for an occluder and 0 for glass. */
export type Triangles = { readonly vertices: Float64Array; readonly indices: Uint32Array; readonly opaque: Uint8Array };

/** 📏️ The tolerances of one visibility computation, in the unit of the view space (metres). */
export type Tolerance = { readonly depth: number; readonly area: number };

/** 🔬️ The default tolerance for a scene of `extent` metres: depth 1e-9 of the extent (at least 1e-9 m) and area 1e-12 of its square. */
export const toleranceOf = (extent: number): Tolerance => ({ depth: Math.max(1e-9, 1e-9 * extent), area: Math.max(1e-14, 1e-12 * extent * extent) });

const STRIDE_BOX = 6;

function areaOf(poly: readonly number[]): number {
  let s = 0;
  const n = poly.length / 2;
  for (let i = 0, j = n - 1; i < n; j = i++) s += poly[2 * j]! * poly[2 * i + 1]! - poly[2 * i]! * poly[2 * j + 1]!;
  return s / 2;
}

/** ✂️ Splits a convex polygon (flat `x, y` pairs) by the line `a x + b y + c = 0` into the part with value ≥ 0 and the part with value ≤ 0. */
function split(poly: readonly number[], a: number, b: number, c: number): [number[], number[]] {
  const n = poly.length / 2;
  const inside: number[] = [];
  const outside: number[] = [];
  let px = poly[2 * (n - 1)]!;
  let py = poly[2 * (n - 1) + 1]!;
  let pv = a * px + b * py + c;
  for (let i = 0; i < n; i++) {
    const x = poly[2 * i]!;
    const y = poly[2 * i + 1]!;
    const v = a * x + b * y + c;
    if ((v > 0 && pv < 0) || (v < 0 && pv > 0)) {
      const t = pv / (pv - v);
      const ix = px + (x - px) * t;
      const iy = py + (y - py) * t;
      inside.push(ix, iy);
      outside.push(ix, iy);
    }
    if (v >= 0) inside.push(x, y);
    if (v <= 0) outside.push(x, y);
    px = x;
    py = y;
    pv = v;
  }
  return [inside, outside];
}

/** 🧊️ The visibility structure of a set of triangles in view space. */
export class Visibility {
  private readonly count: number;
  private readonly live: Uint8Array;
  private readonly tri: Float64Array;
  private readonly plane: Float64Array;
  private readonly box: Float64Array;
  private readonly cellX: number;
  private readonly cellY: number;
  private readonly size: number;
  private readonly originX: number;
  private readonly originY: number;
  private readonly start: Uint32Array;
  private readonly items: Uint32Array;
  private readonly stamp: Int32Array;
  private clock = 0;

  constructor(readonly triangles: Triangles, readonly tolerance: Tolerance) {
    const n = triangles.indices.length / 3;
    this.count = n;
    this.live = new Uint8Array(n);
    this.tri = new Float64Array(6 * n);
    this.plane = new Float64Array(3 * n);
    this.box = new Float64Array(STRIDE_BOX * n);
    this.stamp = new Int32Array(n);
    const v = triangles.vertices;
    let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity, sizeSum = 0, lives = 0;
    for (let t = 0; t < n; t++) {
      const i0 = triangles.indices[3 * t]! * 3, i1 = triangles.indices[3 * t + 1]! * 3, i2 = triangles.indices[3 * t + 2]! * 3;
      const x0 = v[i0]!, y0 = v[i0 + 1]!, z0 = v[i0 + 2]!, x1 = v[i1]!, y1 = v[i1 + 1]!, z1 = v[i1 + 2]!, x2 = v[i2]!, y2 = v[i2 + 1]!, z2 = v[i2 + 2]!;
      const ux = x1 - x0, uy = y1 - y0, uz = z1 - z0, wx = x2 - x0, wy = y2 - y0, wz = z2 - z0;
      const nz = ux * wy - uy * wx;
      const nx = uy * wz - uz * wy;
      const ny = uz * wx - ux * wz;
      const length = portable.hypot(nx, ny, nz);
      if (!(nz > tolerance.area * 2) || nz < 1e-7 * length) continue;
      this.live[t] = 1;
      this.tri.set([x0, y0, x1, y1, x2, y2], 6 * t);
      const a = -nx / nz;
      const b = -ny / nz;
      this.plane.set([a, b, z0 - a * x0 - b * y0], 3 * t);
      const bx0 = Math.min(x0, x1, x2), bx1 = Math.max(x0, x1, x2), by0 = Math.min(y0, y1, y2), by1 = Math.max(y0, y1, y2);
      this.box.set([bx0, by0, bx1, by1, Math.min(z0, z1, z2), Math.max(z0, z1, z2)], STRIDE_BOX * t);
      minX = Math.min(minX, bx0); minY = Math.min(minY, by0); maxX = Math.max(maxX, bx1); maxY = Math.max(maxY, by1);
      sizeSum += Math.max(bx1 - bx0, by1 - by0);
      lives++;
    }
    const mean = lives === 0 ? 1 : sizeSum / lives;
    const spanX = Math.max(maxX - minX, 1e-9), spanY = Math.max(maxY - minY, 1e-9);
    const cell = Math.max(mean * 2, Math.sqrt((spanX * spanY) / 1_000_000), 1e-9);
    this.size = cell;
    this.originX = Number.isFinite(minX) ? minX : 0;
    this.originY = Number.isFinite(minY) ? minY : 0;
    this.cellX = Math.max(1, Math.ceil(spanX / cell) + 1);
    this.cellY = Math.max(1, Math.ceil(spanY / cell) + 1);
    const cells = this.cellX * this.cellY;
    const counts = new Uint32Array(cells + 1);
    const each = (t: number, visit: (cell: number) => void): void => {
      const c0 = Math.floor((this.box[STRIDE_BOX * t]! - this.originX) / cell), c1 = Math.floor((this.box[STRIDE_BOX * t + 2]! - this.originX) / cell);
      const r0 = Math.floor((this.box[STRIDE_BOX * t + 1]! - this.originY) / cell), r1 = Math.floor((this.box[STRIDE_BOX * t + 3]! - this.originY) / cell);
      for (let r = r0; r <= r1; r++) for (let c = c0; c <= c1; c++) visit(r * this.cellX + c);
    };
    for (let t = 0; t < n; t++) if (this.live[t] === 1 && triangles.opaque[t] === 1) each(t, (c) => void counts[c + 1]!++);
    for (let c = 0; c < cells; c++) counts[c + 1]! += counts[c]!;
    this.start = counts;
    this.items = new Uint32Array(counts[cells]!);
    const fill = counts.slice(0, cells);
    for (let t = 0; t < n; t++) if (this.live[t] === 1 && triangles.opaque[t] === 1) each(t, (c) => void (this.items[fill[c]!++] = t));
  }

  /** 👁️ Whether triangle `t` is front-facing and big enough to be drawn. */
  isLive(t: number): boolean {
    return this.live[t] === 1;
  }

  private candidates(x0: number, y0: number, x1: number, y1: number, accept: (t: number) => boolean): number[] {
    const out: number[] = [];
    const stamp = ++this.clock;
    const c0 = Math.max(0, Math.floor((x0 - this.originX) / this.size)), c1 = Math.min(this.cellX - 1, Math.floor((x1 - this.originX) / this.size));
    const r0 = Math.max(0, Math.floor((y0 - this.originY) / this.size)), r1 = Math.min(this.cellY - 1, Math.floor((y1 - this.originY) / this.size));
    for (let r = r0; r <= r1; r++)
      for (let c = c0; c <= c1; c++) {
        const cell = r * this.cellX + c;
        for (let k = this.start[cell]!; k < this.start[cell + 1]!; k++) {
          const t = this.items[k]!;
          if (this.stamp[t] === stamp) continue;
          this.stamp[t] = stamp;
          if (accept(t)) out.push(t);
        }
      }
    return out;
  }

  /** 🧩️ The visible part of triangle `t` as convex polygons (flat counter-clockwise `x, y` pairs), empty when it is hidden or not live. */
  fragments(t: number): number[][] {
    if (this.live[t] !== 1) return [];
    const { depth: eps, area: minArea } = this.tolerance;
    const T = this.tri.subarray(6 * t, 6 * t + 6);
    const pa = this.plane[3 * t]!, pb = this.plane[3 * t + 1]!, pc = this.plane[3 * t + 2]!;
    const minZ = this.box[STRIDE_BOX * t + 4]!;
    const bx0 = this.box[STRIDE_BOX * t]!, by0 = this.box[STRIDE_BOX * t + 1]!, bx1 = this.box[STRIDE_BOX * t + 2]!, by1 = this.box[STRIDE_BOX * t + 3]!;
    const near = this.candidates(bx0, by0, bx1, by1, (c) => c !== t && this.box[STRIDE_BOX * c + 5]! > minZ - eps && this.box[STRIDE_BOX * c]! < bx1 && this.box[STRIDE_BOX * c + 2]! > bx0 && this.box[STRIDE_BOX * c + 1]! < by1 && this.box[STRIDE_BOX * c + 3]! > by0);
    near.sort((p, q) => this.box[STRIDE_BOX * q + 5]! - this.box[STRIDE_BOX * p + 5]! || p - q);
    let pieces: number[][] = [Array.from(T)];
    for (const c of near) {
      if (pieces.length === 0) break;
      const C = this.tri.subarray(6 * c, 6 * c + 6);
      const ca = this.plane[3 * c]!, cb = this.plane[3 * c + 1]!, cc = this.plane[3 * c + 2]!;
      const da = ca - pa, db = cb - pb, dc = cc - pc;
      let nearest = -Infinity, farthest = Infinity;
      for (let k = 0; k < 3; k++) {
        const d = da * C[2 * k]! + db * C[2 * k + 1]! + dc;
        if (d > nearest) nearest = d;
        if (d < farthest) farthest = d;
      }
      const coplanar = nearest <= eps && farthest >= -eps;
      if (nearest <= eps && !(coplanar && c < t)) continue;
      const hideAll = coplanar || farthest > eps;
      const next: number[][] = [];
      for (const piece of pieces) {
        let rem: number[] = piece;
        const outside: number[][] = [];
        let any = true;
        for (let k = 0; k < 3 && any; k++) {
          const px = C[2 * k]!, py = C[2 * k + 1]!, qx = C[2 * ((k + 1) % 3)]!, qy = C[2 * ((k + 1) % 3) + 1]!;
          const a = -(qy - py), b = qx - px;
          const [inner, outer] = split(rem, a, b, -(a * px + b * py));
          if (outer.length >= 6 && Math.abs(areaOf(outer)) > minArea) outside.push(outer);
          rem = inner;
          any = rem.length >= 6;
        }
        if (!any || Math.abs(areaOf(rem)) <= minArea) {
          next.push(piece);
          continue;
        }
        next.push(...outside);
        if (hideAll) continue;
        const keeping = split(rem, da, db, dc)[1];
        if (keeping.length >= 6 && Math.abs(areaOf(keeping)) > minArea) next.push(keeping);
      }
      pieces = next;
    }
    return pieces;
  }

  /** 🙈️ The parameter intervals `[t0, t1]` of the segment `a → b` (view-space points) hidden by occluders, merged and sorted. `except` lists triangles that never hide it (the triangles the segment belongs to). */
  hidden(a: readonly [number, number, number], b: readonly [number, number, number], except: ReadonlySet<number> = new Set()): [number, number][] {
    const { depth: eps } = this.tolerance;
    const dx = b[0] - a[0], dy = b[1] - a[1], dz = b[2] - a[2];
    const x0 = Math.min(a[0], b[0]), x1 = Math.max(a[0], b[0]), y0 = Math.min(a[1], b[1]), y1 = Math.max(a[1], b[1]), z0 = Math.min(a[2], b[2]);
    const near = this.candidates(x0, y0, x1, y1, (c) => !except.has(c) && this.box[STRIDE_BOX * c + 5]! > z0 + eps && this.box[STRIDE_BOX * c]! <= x1 && this.box[STRIDE_BOX * c + 2]! >= x0 && this.box[STRIDE_BOX * c + 1]! <= y1 && this.box[STRIDE_BOX * c + 3]! >= y0);
    const out: [number, number][] = [];
    for (const c of near) {
      const C = this.tri.subarray(6 * c, 6 * c + 6);
      let t0 = 0, t1 = 1;
      for (let k = 0; k < 3 && t0 < t1; k++) {
        const px = C[2 * k]!, py = C[2 * k + 1]!, qx = C[2 * ((k + 1) % 3)]!, qy = C[2 * ((k + 1) % 3) + 1]!;
        const ha = -(qy - py), hb = qx - px, hc = -(ha * px + hb * py);
        const v0 = ha * a[0] + hb * a[1] + hc;
        const rate = ha * dx + hb * dy;
        if (Math.abs(rate) < 1e-18) {
          if (v0 < 0) t1 = t0;
          continue;
        }
        const t = -v0 / rate;
        if (rate > 0) t0 = Math.max(t0, t);
        else t1 = Math.min(t1, t);
      }
      if (!(t1 - t0 > 1e-9)) continue;
      const ca = this.plane[3 * c]!, cb = this.plane[3 * c + 1]!, cc = this.plane[3 * c + 2]!;
      const f = (t: number): number => ca * (a[0] + dx * t) + cb * (a[1] + dy * t) + cc - (a[2] + dz * t);
      const f0 = f(t0), f1 = f(t1);
      if (f0 <= eps && f1 <= eps) continue;
      if (f0 > eps && f1 > eps) out.push([t0, t1]);
      else {
        const cross = t0 + (-f0 / (f1 - f0)) * (t1 - t0);
        if (f0 > eps) out.push([t0, cross]);
        else out.push([cross, t1]);
      }
    }
    out.sort((p, q) => p[0] - q[0]);
    const merged: [number, number][] = [];
    for (const interval of out) {
      const last = merged[merged.length - 1];
      if (last !== undefined && interval[0] <= last[1] + 1e-9) last[1] = Math.max(last[1], interval[1]);
      else merged.push([interval[0], interval[1]]);
    }
    return merged;
  }

  /** 🦄 The visible parameter intervals of a segment: the complement of {@link Visibility.hidden} in `[0, 1]`, without slivers shorter than `minLength` (in the unit of the segment parameter times its length). */
  visible(a: readonly [number, number, number], b: readonly [number, number, number], except: ReadonlySet<number> = new Set()): [number, number][] {
    const hidden = this.hidden(a, b, except);
    const out: [number, number][] = [];
    let cursor = 0;
    for (const [h0, h1] of hidden) {
      if (h0 - cursor > 1e-9) out.push([cursor, h0]);
      cursor = Math.max(cursor, h1);
    }
    if (1 - cursor > 1e-9) out.push([cursor, 1]);
    return out;
  }
}
