/** 🔺️ Ear-clipping triangulation of a counter-clockwise polygon with clockwise holes joined by visibility bridges; used for extrusion caps and partial sweep caps. Collinear and near-duplicate vertices stay vertices of the triangulation (no T-junctions): an ear must be convex beyond a relative flatness tolerance and may not have any other vertex inside or on it within that tolerance, with a second, exact pass for the rest.
 * Also the simple-polygon predicates the validator uses ({@link ringFault}, {@link holesFault}) and {@link cleanRing}.
 * @see ../🧱️builder/🟦️.ts */
import type { Vec2 } from "../../🧬️schema/🟦️.ts";
import { Pace } from "../../⏱️operation/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

export const signedArea = (ring: readonly Vec2[]): number => {
  let s = 0;
  for (let i = 0; i < ring.length; i++) {
    const a = ring[i]!, b = ring[(i + 1) % ring.length]!;
    s += a[0] * b[1] - b[0] * a[1];
  }
  return s / 2;
};

const cross2 = (o: Vec2, a: Vec2, b: Vec2): number => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);

const inTriangle = (p: Vec2, a: Vec2, b: Vec2, c: Vec2, slack = 0): boolean => cross2(a, b, p) >= -slack && cross2(b, c, p) >= -slack && cross2(c, a, p) >= -slack;

/** 🧭️ Whether `target` lies in the interior wedge at vertex `v` of a counter-clockwise polygon whose neighbours are `before` and `after`: a vertex id that appears twice (where an earlier bridge touched it) has two wedges, and a new bridge must enter the one that faces it. */
const wedgeContains = (before: Vec2, v: Vec2, after: Vec2, target: Vec2): boolean =>
  cross2(before, v, after) > 0 ? cross2(before, v, target) >= 0 && cross2(v, after, target) >= 0 : cross2(before, v, target) > 0 || cross2(v, after, target) > 0;

/** 📐️ Triangle index triples into `[...outer, ...holes]`, counter-clockwise; holes must be clockwise and the outer ring counter-clockwise. */
export function triangulatePolygon(outer: readonly Vec2[], holes: readonly (readonly Vec2[])[], pace: Pace = new Pace()): number[] {
  const points: Vec2[] = [...outer, ...holes.flat()];
  let poly: number[] = outer.map((_, i) => i);
  const ranges: number[][] = [];
  let offset = outer.length;
  for (const hole of holes) {
    ranges.push(hole.map((_, i) => offset + i));
    offset += hole.length;
  }
  ranges.sort((a, b) => Math.max(...b.map((i) => points[i]![0])) - Math.max(...a.map((i) => points[i]![0])));
  for (const hole of ranges) {
    pace.tick(poly.length);
    let m = hole[0]!;
    for (const i of hole) if (points[i]![0] > points[m]![0]) m = i;
    const M = points[m]!;
    let best = Infinity, bridge = -1, hitX = 0;
    for (let k = 0; k < poly.length; k++) {
      const a = points[poly[k]!]!, b = points[poly[(k + 1) % poly.length]!]!;
      if ((a[1] - M[1]) * (b[1] - M[1]) > 0 || a[1] === b[1]) continue;
      const x = a[0] + ((M[1] - a[1]) / (b[1] - a[1])) * (b[0] - a[0]);
      if (x >= M[0] && x < best) {
        best = x;
        hitX = x;
        bridge = a[0] > b[0] ? k : (k + 1) % poly.length;
      }
    }
    if (bridge < 0) continue;
    const P = points[poly[bridge]!]!;
    const I: Vec2 = [hitX, M[1]];
    let bestAngle = Infinity;
    for (let k = 0; k < poly.length; k++) {
      const q = points[poly[k]!]!;
      if (poly[k] === poly[bridge] || q[0] < M[0] || (q[1] === M[1] && q[0] > hitX)) continue;
      const [ta, tb, tc]: [Vec2, Vec2, Vec2] = I[1] < P[1] || (I[1] === P[1] && I[0] < P[0]) ? [M, I, P] : [M, P, I];
      if (!inTriangle(q, ta, tb, tc)) continue;
      const angle = Math.abs(portable.atan2(q[1] - M[1], q[0] - M[0]));
      if (angle < bestAngle) {
        bestAngle = angle;
        bridge = k;
      }
    }
    const facing = poly.findIndex((id, k) => id === poly[bridge] && wedgeContains(points[poly[(k + poly.length - 1) % poly.length]!]!, points[id]!, points[poly[(k + 1) % poly.length]!]!, M));
    if (facing >= 0) bridge = facing;
    const at = hole.indexOf(m);
    const ring = [...hole.slice(at), ...hole.slice(0, at)];
    poly = [...poly.slice(0, bridge + 1), ...ring, m, poly[bridge]!, ...poly.slice(bridge + 1)];
  }
  const out: number[] = [];
  const extent = Math.max(1e-12, ...points.flatMap((q) => [Math.abs(q[0]), Math.abs(q[1])]));
  const flat = 1e-14 * extent * extent;
  const same = (p: Vec2, q: Vec2): boolean => p[0] === q[0] && p[1] === q[1];
  const clip = (strict: boolean): boolean => {
    for (let k = 0; k < poly.length; k++) {
      const ia = poly[(k + poly.length - 1) % poly.length]!, ib = poly[k]!, ic = poly[(k + 1) % poly.length]!;
      const a = points[ia]!, b = points[ib]!, c = points[ic]!;
      const area = cross2(a, b, c);
      if (area <= (strict ? flat : 1e-15)) continue;
      let ear = true;
      pace.tick(poly.length);
      for (const other of poly) {
        if (other === ia || other === ib || other === ic) continue;
        const q = points[other]!;
        if (same(q, a) || same(q, b) || same(q, c)) continue;
        if (inTriangle(q, a, b, c, strict ? flat : 0)) { ear = false; break; }
      }
      if (!ear) continue;
      out.push(ia, ib, ic);
      poly.splice(k, 1);
      return true;
    }
    return false;
  };
  let guard = poly.length * poly.length + 16;
  while (poly.length > 3 && guard-- > 0) {
    if (clip(true) || clip(false)) continue;
    const ia = poly[poly.length - 1]!, ib = poly[0]!, ic = poly[1]!;
    if (cross2(points[ia]!, points[ib]!, points[ic]!) > 0) out.push(ia, ib, ic);
    poly.splice(0, 1);
  }
  if (poly.length === 3 && cross2(points[poly[0]!]!, points[poly[1]!]!, points[poly[2]!]!) > 0) out.push(poly[0]!, poly[1]!, poly[2]!);
  return out;
}

/** 🧹️ A ring without consecutive duplicate points and without a repeated closing point (a first point equal to the last closes the loop). */
export const cleanRing = (ring: readonly Vec2[]): Vec2[] => {
  const out: Vec2[] = [];
  for (const p of ring) {
    const last = out[out.length - 1];
    if (!last || portable.hypot(p[0] - last[0], p[1] - last[1]) > 1e-12) out.push([p[0], p[1]]);
  }
  while (out.length > 1 && portable.hypot(out[0]![0] - out[out.length - 1]![0], out[0]![1] - out[out.length - 1]![1]) <= 1e-12) out.pop();
  return out;
};

const scaleOf = (rings: readonly (readonly Vec2[])[]): number => Math.max(1e-9, ...rings.flatMap((r) => r.flatMap((p) => [Math.abs(p[0]), Math.abs(p[1])])));
const onSegment = (p: Vec2, a: Vec2, b: Vec2, tol: number): boolean => {
  const length = portable.hypot(b[0] - a[0], b[1] - a[1]);
  return Math.abs(cross2(a, b, p)) <= tol * (length || 1) && (p[0] - a[0]) * (p[0] - b[0]) + (p[1] - a[1]) * (p[1] - b[1]) <= tol * (length + tol);
};

/** 🔗️ The contact points of segments ab and cd (a proper crossing, an end touching the other segment, or the ends inside a collinear overlap), or null when they are apart by more than `tol`. */
export function contact(a: Vec2, b: Vec2, c: Vec2, d: Vec2, tol: number): Vec2[] | null {
  const ab = portable.hypot(b[0] - a[0], b[1] - a[1]) || 1, cd = portable.hypot(d[0] - c[0], d[1] - c[1]) || 1;
  const side = (x: number): number => (Math.abs(x) <= tol ? 0 : Math.sign(x));
  const s1 = side(cross2(a, b, c) / ab), s2 = side(cross2(a, b, d) / ab), s3 = side(cross2(c, d, a) / cd), s4 = side(cross2(c, d, b) / cd);
  if (s1 * s2 < 0 && s3 * s4 < 0) {
    const t = cross2(c, d, a) / (cross2(c, d, a) - cross2(c, d, b));
    return [[a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])]];
  }
  const touching = ([[c, a, b], [d, a, b], [a, c, d], [b, c, d]] as [Vec2, Vec2, Vec2][]).filter(([p, u, v]) => onSegment(p, u, v, tol)).map(([p]) => p);
  return touching.length > 0 ? touching : null;
}

/** 🧪️ Whether a closed ring or an open polyline is faulty: a ring without area, two non-adjacent edges that touch or cross, or adjacent edges that fold back onto each other. `exempt` lets contacts at such points pass (the revolve axis). */
export function ringFault(ring: readonly Vec2[], closed: boolean, exempt: (p: Vec2) => boolean = () => false, pace: Pace = new Pace()): boolean {
  const scale = scaleOf([ring]), tol = 1e-12 * scale;
  if (closed && (ring.length < 3 || Math.abs(signedArea(ring)) <= tol * scale)) return true;
  const edges: (readonly [Vec2, Vec2])[] = [];
  for (let i = 0; i + (closed ? 0 : 1) < ring.length; i++) edges.push([ring[i]!, ring[(i + 1) % ring.length]!]);
  const foldsBack = (shared: Vec2, u: Vec2, v: Vec2): boolean => {
    const lu = portable.hypot(u[0] - shared[0], u[1] - shared[1]), lv = portable.hypot(v[0] - shared[0], v[1] - shared[1]);
    return Math.abs(cross2(shared, u, v)) <= 1e-9 * lu * lv && (u[0] - shared[0]) * (v[0] - shared[0]) + (u[1] - shared[1]) * (v[1] - shared[1]) > 0;
  };
  for (let i = 0; i < edges.length; i++) {
    pace.tick(edges.length - i);
    for (let j = i + 1; j < edges.length; j++) {
      const [a, b] = edges[i]!, [c, d] = edges[j]!;
      if (j === i + 1) {
        if (foldsBack(b, a, d)) return true;
        continue;
      }
      if (closed && i === 0 && j === edges.length - 1) {
        if (foldsBack(a, b, c)) return true;
        continue;
      }
      const hit = contact(a, b, c, d, tol);
      if (hit && !hit.every(exempt)) return true;
    }
  }
  return false;
}

const inside = (p: Vec2, ring: readonly Vec2[]): boolean => {
  let odd = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const a = ring[i]!, b = ring[j]!;
    if ((a[1] > p[1]) !== (b[1] > p[1]) && p[0] < ((b[0] - a[0]) * (p[1] - a[1])) / (b[1] - a[1]) + a[0]) odd = !odd;
  }
  return odd;
};

/** 🕳️ Whether the holes are faulty: a hole touching or crossing the outline or another hole, lying outside the outline or inside another hole. */
export function holesFault(outer: readonly Vec2[], holes: readonly (readonly Vec2[])[], pace: Pace = new Pace()): boolean {
  const tol = 1e-12 * scaleOf([outer, ...holes]);
  const rings = [outer, ...holes];
  const loop = (ring: readonly Vec2[]) => ring.map((p, i) => [p, ring[(i + 1) % ring.length]!] as const);
  for (let a = 0; a < rings.length; a++) for (let b = a + 1; b < rings.length; b++)
    for (const [p, q] of loop(rings[a]!)) {
      pace.tick(rings[b]!.length);
      for (const [r, s] of loop(rings[b]!)) if (contact(p, q, r, s, tol)) return true;
    }
  return holes.some((hole) => !inside(hole[0]!, outer) || holes.some((other) => other !== hole && inside(hole[0]!, other)));
}
