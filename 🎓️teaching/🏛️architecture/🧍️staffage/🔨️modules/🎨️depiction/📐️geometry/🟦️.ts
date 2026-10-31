/** 📐️ Planar geometry of the depiction: polyline simplification, exact hidden-line clipping against filled occluders, arrow, arc and tapered-strip outlines.
 *
 * Everything works in one plane with y down (paper millimetres) but is unit-free; tolerances are passed in.
 *
 * @see ../../🔗️contracts/🟦️.ts — {@link PathCommand}
 */
import type { PathCommand } from "../../🔗️contracts/🟦️.ts";

/** 📍️ A planar point. */
export type Vec = { readonly x: number; readonly y: number };

/** ⭕️ A polygon or polyline ring. */
export type Ring = readonly Vec[];

/** 📦️ An axis-aligned box. */
export type Box = { readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number };

/** 🧱️ A filled shape that hides everything of lower z behind it (even-odd across its rings). */
export type Occluder = { readonly rings: readonly Ring[]; readonly box: Box };

/** 🖍️ A visible stretch of an outline after hidden-line removal. */
export type Run = { readonly points: readonly Vec[]; readonly closed: boolean };

/** ➕️ The sum of two vectors. */
export const add = (a: Vec, b: Vec): Vec => ({ x: a.x + b.x, y: a.y + b.y });

/** ➖️ The difference `a − b`. */
export const sub = (a: Vec, b: Vec): Vec => ({ x: a.x - b.x, y: a.y - b.y });

/** ✖️ A vector scaled by a factor. */
export const scale = (a: Vec, factor: number): Vec => ({ x: a.x * factor, y: a.y * factor });

/** 📏️ The Euclidean length. */
export const length = (a: Vec): number => Math.hypot(a.x, a.y);

/** 🧭️ The unit vector, or the zero vector for a zero input. */
export function unit(a: Vec): Vec {
  const l = length(a);
  return l < 1e-12 ? { x: 0, y: 0 } : { x: a.x / l, y: a.y / l };
}

/** ↺️ The vector rotated a quarter turn clockwise on screen (y down): `(x, y) → (−y, x)`. */
export const perp = (a: Vec): Vec => ({ x: -a.y, y: a.x });

/** 🔀️ The point at fraction `t` between two points. */
export const lerp = (a: Vec, b: Vec, t: number): Vec => ({ x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t });

/** 📦️ The box of all rings, `null` when there are no points. */
export function boxOf(rings: readonly Ring[]): Box | null {
  let box: Box | null = null;
  for (const ring of rings)
    for (const p of ring) box = box === null ? { minX: p.x, minY: p.y, maxX: p.x, maxY: p.y } : { minX: Math.min(box.minX, p.x), minY: Math.min(box.minY, p.y), maxX: Math.max(box.maxX, p.x), maxY: Math.max(box.maxY, p.y) };
  return box;
}

function distanceToSegment(p: Vec, a: Vec, b: Vec): number {
  const d = sub(b, a);
  const l2 = d.x * d.x + d.y * d.y;
  const t = l2 === 0 ? 0 : Math.max(0, Math.min(1, ((p.x - a.x) * d.x + (p.y - a.y) * d.y) / l2));
  return Math.hypot(p.x - (a.x + d.x * t), p.y - (a.y + d.y * t));
}

function simplifyOpen(points: readonly Vec[], tolerance: number): Vec[] {
  if (points.length < 3) return [...points];
  const keep = new Uint8Array(points.length);
  keep[0] = keep[points.length - 1] = 1;
  const stack: [number, number][] = [[0, points.length - 1]];
  while (stack.length) {
    const [from, to] = stack.pop()!;
    let worst = -1;
    let at = -1;
    for (let i = from + 1; i < to; i++) {
      const d = distanceToSegment(points[i]!, points[from]!, points[to]!);
      if (d > worst) {
        worst = d;
        at = i;
      }
    }
    if (worst > tolerance) {
      keep[at] = 1;
      stack.push([from, at], [at, to]);
    }
  }
  return points.filter((_, i) => keep[i] === 1);
}

/** ✂️ Douglas–Peucker simplification; closed rings are split at their farthest vertex pair so extreme points survive. */
export function simplify(points: Ring, closed: boolean, tolerance: number): Vec[] {
  if (!closed || points.length < 5) return simplifyOpen(points, tolerance);
  let far = 0;
  let best = -1;
  for (let i = 1; i < points.length; i++) {
    const d = length(sub(points[i]!, points[0]!));
    if (d > best) {
      best = d;
      far = i;
    }
  }
  const first = simplifyOpen(points.slice(0, far + 1), tolerance);
  const second = simplifyOpen([...points.slice(far), points[0]!], tolerance);
  return [...first, ...second.slice(1, -1)];
}

function pointInRings(p: Vec, rings: readonly Ring[], eps: number): boolean {
  let inside = false;
  for (const ring of rings) {
    for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
      const a = ring[i]!;
      const b = ring[j]!;
      if (p.x >= Math.min(a.x, b.x) - eps && p.x <= Math.max(a.x, b.x) + eps && p.y >= Math.min(a.y, b.y) - eps && p.y <= Math.max(a.y, b.y) + eps && distanceToSegment(p, a, b) <= eps) return false;
      if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) inside = !inside;
    }
  }
  return inside;
}

/** 🎯️ Whether a point lies inside the rings (even-odd; on an edge counts as outside). */
export const pointInside = (p: Vec, rings: readonly Ring[]): boolean => pointInRings(p, rings, 0);

function crossings(a: Vec, b: Vec, occluder: Occluder): number[] {
  const minX = Math.min(a.x, b.x);
  const maxX = Math.max(a.x, b.x);
  const minY = Math.min(a.y, b.y);
  const maxY = Math.max(a.y, b.y);
  if (maxX < occluder.box.minX || minX > occluder.box.maxX || maxY < occluder.box.minY || minY > occluder.box.maxY) return [];
  const out: number[] = [];
  const d1 = sub(b, a);
  for (const ring of occluder.rings)
    for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
      const c = ring[j]!;
      const r = ring[i]!;
      if (Math.max(c.x, r.x) < minX || Math.min(c.x, r.x) > maxX || Math.max(c.y, r.y) < minY || Math.min(c.y, r.y) > maxY) continue;
      const d2 = sub(r, c);
      const denom = d1.x * d2.y - d1.y * d2.x;
      if (Math.abs(denom) < 1e-12) continue;
      const ca = sub(c, a);
      const t = (ca.x * d2.y - ca.y * d2.x) / denom;
      const u = (ca.x * d1.y - ca.y * d1.x) / denom;
      if (t > 1e-9 && t < 1 - 1e-9 && u >= 0 && u <= 1) out.push(t);
    }
  return out;
}

function overlaps(box: Box, other: Box, pad: number): boolean {
  return box.minX <= other.maxX + pad && box.maxX >= other.minX - pad && box.minY <= other.maxY + pad && box.maxY >= other.minY - pad;
}

/** 🕳️ The visible stretches of one outline after removing every part inside an occluder; an untouched closed ring comes back closed. */
export function visibleRuns(points: Ring, closed: boolean, occluders: readonly Occluder[], eps: number): Run[] {
  const box = boxOf([points]);
  if (box === null || points.length < 2) return [];
  const relevant = occluders.filter((o) => overlaps(box, o.box, eps));
  if (relevant.length === 0) return [{ points, closed }];
  const n = points.length;
  const edges = closed ? n : n - 1;
  const runs: Vec[][] = [];
  let current: Vec[] = [];
  let firstVisible = false;
  let lastVisible = false;
  for (let i = 0; i < edges; i++) {
    const a = points[i]!;
    const b = points[(i + 1) % n]!;
    const ts = [0, ...relevant.flatMap((o) => crossings(a, b, o)).sort((x, y) => x - y), 1];
    for (let k = 0; k + 1 < ts.length; k++) {
      const t0 = ts[k]!;
      const t1 = ts[k + 1]!;
      if (t1 - t0 < 1e-9) continue;
      const mid = lerp(a, b, (t0 + t1) / 2);
      const hidden = relevant.some((o) => mid.x >= o.box.minX - eps && mid.x <= o.box.maxX + eps && mid.y >= o.box.minY - eps && mid.y <= o.box.maxY + eps && pointInRings(mid, o.rings, eps));
      if (i === 0 && k === 0) firstVisible = !hidden;
      lastVisible = !hidden;
      if (hidden) {
        if (current.length > 1) runs.push(current);
        current = [];
      } else {
        if (current.length === 0) current.push(lerp(a, b, t0));
        current.push(lerp(a, b, t1));
      }
    }
  }
  if (current.length > 1) runs.push(current);
  if (closed && runs.length === 1 && firstVisible && lastVisible && runs[0]!.length >= n) return [{ points: runs[0]!.slice(0, -1), closed: true }];
  if (closed && runs.length > 1 && firstVisible && lastVisible) {
    const last = runs.pop()!;
    runs[0] = [...last, ...runs[0]!.slice(1)];
  }
  return runs.map((r) => ({ points: r, closed: false }));
}

/** ✂️ The stretches of the segment `a → b` that lie inside the rings (even-odd across all rings), in order along the segment. */
export function clipSegmentToRings(a: Vec, b: Vec, rings: readonly Ring[]): [Vec, Vec][] {
  const d1 = sub(b, a);
  const ts: number[] = [0, 1];
  for (const ring of rings)
    for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
      const c = ring[j]!;
      const d2 = sub(ring[i]!, c);
      const denom = d1.x * d2.y - d1.y * d2.x;
      if (Math.abs(denom) < 1e-12) continue;
      const ca = sub(c, a);
      const t = (ca.x * d2.y - ca.y * d2.x) / denom;
      const u = (ca.x * d1.y - ca.y * d1.x) / denom;
      if (t > 0 && t < 1 && u >= 0 && u < 1) ts.push(t);
    }
  ts.sort((p, q) => p - q);
  const out: [Vec, Vec][] = [];
  for (let k = 0; k + 1 < ts.length; k++) {
    const t0 = ts[k]!;
    const t1 = ts[k + 1]!;
    if (t1 - t0 < 1e-9) continue;
    const mid = lerp(a, b, (t0 + t1) / 2);
    if (pointInRings(mid, rings, 0)) out.push([lerp(a, b, t0), lerp(a, b, t1)]);
  }
  return out;
}

/** ✒️ Move/line commands through the points of a ring, closed with `Z` when requested. */
export function commandsOf(points: Ring, closed: boolean): PathCommand[] {
  const out: PathCommand[] = points.map((p, i) => ({ op: i === 0 ? "M" : "L", x: p.x, y: p.y }));
  if (closed && points.length > 2) out.push({ op: "Z" });
  return out;
}

/** ✒️ Commands of several rings as subpaths of one path. */
export const ringsCommands = (rings: readonly Ring[], closed: boolean): PathCommand[] => rings.flatMap((r) => commandsOf(r, closed));

/** ⭕️ A circle as four cubic Béziers. */
export function circleCommands(c: Vec, r: number): PathCommand[] {
  const k = 0.5522847498 * r;
  return [
    { op: "M", x: c.x + r, y: c.y },
    { op: "C", x1: c.x + r, y1: c.y + k, x2: c.x + k, y2: c.y + r, x: c.x, y: c.y + r },
    { op: "C", x1: c.x - k, y1: c.y + r, x2: c.x - r, y2: c.y + k, x: c.x - r, y: c.y },
    { op: "C", x1: c.x - r, y1: c.y - k, x2: c.x - k, y2: c.y - r, x: c.x, y: c.y - r },
    { op: "C", x1: c.x + k, y1: c.y - r, x2: c.x + r, y2: c.y - k, x: c.x + r, y: c.y },
    { op: "Z" },
  ];
}

/** 🥧️ A closed pie sector of a circle between two screen angles (radians, y down). */
export function sectorCommands(c: Vec, r: number, from: number, to: number): PathCommand[] {
  return [{ op: "M", x: c.x, y: c.y }, { op: "L", x: c.x + r * Math.cos(from), y: c.y + r * Math.sin(from) }, ...arcSegments(c, r, from, to - from), { op: "Z" }];
}

/** 🌀️ Cubic Bézier segments of an arc around a centre from a screen angle through a signed sweep (radians, y down). */
export function arcSegments(c: Vec, r: number, from: number, sweep: number): PathCommand[] {
  const count = Math.max(1, Math.ceil(Math.abs(sweep) / (Math.PI / 2) - 1e-9));
  const step = sweep / count;
  const h = (4 / 3) * Math.tan(step / 4) * r;
  const out: PathCommand[] = [];
  for (let i = 0; i < count; i++) {
    const a0 = from + i * step;
    const a1 = a0 + step;
    out.push({ op: "C", x1: c.x + r * Math.cos(a0) - h * Math.sin(a0), y1: c.y + r * Math.sin(a0) + h * Math.cos(a0), x2: c.x + r * Math.cos(a1) + h * Math.sin(a1), y2: c.y + r * Math.sin(a1) - h * Math.cos(a1), x: c.x + r * Math.cos(a1), y: c.y + r * Math.sin(a1) });
  }
  return out;
}

/** 🌀️ An open arc path starting at its first point. */
export function arcCommands(c: Vec, r: number, from: number, sweep: number): PathCommand[] {
  return [{ op: "M", x: c.x + r * Math.cos(from), y: c.y + r * Math.sin(from) }, ...arcSegments(c, r, from, sweep)];
}

/** ➡️ The filled outline of an arrow from tail to head: a thin shaft and a triangular head whose tip is the head point. */
export function arrowOutline(tail: Vec, head: Vec, shaft: number, headLength: number, headWidth: number): Vec[] {
  const d = sub(head, tail);
  const l = length(d);
  const u = unit(d);
  const n = perp(u);
  const hl = Math.min(headLength, l * 0.6);
  const base = sub(head, scale(u, hl));
  const hw = headWidth * (hl / headLength);
  return [add(tail, scale(n, shaft / 2)), add(base, scale(n, shaft / 2)), add(base, scale(n, hw / 2)), head, sub(base, scale(n, hw / 2)), sub(base, scale(n, shaft / 2)), sub(tail, scale(n, shaft / 2))];
}

/** 🔺️ A triangular arrow head with its tip at `tip` pointing along `direction`. */
export function headOutline(tip: Vec, direction: Vec, headLength: number, headWidth: number): Vec[] {
  const u = unit(direction);
  const base = sub(tip, scale(u, headLength));
  const n = perp(u);
  return [tip, add(base, scale(n, headWidth / 2)), sub(base, scale(n, headWidth / 2))];
}

/** 〰️ The two offset boundaries of a polyline with per-point widths, for tapered strips; returns `[left, right]`. */
export function strip(points: Ring, widths: readonly number[]): [Vec[], Vec[]] {
  const left: Vec[] = [];
  const right: Vec[] = [];
  for (let i = 0; i < points.length; i++) {
    const a = points[Math.max(0, i - 1)]!;
    const b = points[Math.min(points.length - 1, i + 1)]!;
    const n = perp(unit(sub(b, a)));
    const w = (widths[i] ?? 0) / 2;
    left.push(add(points[i]!, scale(n, w)));
    right.push(sub(points[i]!, scale(n, w)));
  }
  return [left, right];
}
