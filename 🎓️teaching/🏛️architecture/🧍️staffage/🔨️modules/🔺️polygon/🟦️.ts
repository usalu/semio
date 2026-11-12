/** 🔺️ Polygon union without a runtime dependency: the boundary of the union of simple polygons as closed rings (outer rings counter-clockwise, holes clockwise), by splitting every edge at the crossings with the other edges and keeping the pieces that no other polygon covers.
 *
 * THE REFERENCE is shapely `unary_union` (see `🧪️tests/🧊️lateral-spread/🐍️.py`).
 *
 * @see ../🔗️contracts/🟦️.ts — `Point`
 */
import type { Point } from "../🔗️contracts/🟦️.ts";
import * as portable from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

const EPS = 1e-9;

const THIN_M = 1e-6;

const NEAR_M = 1e-5;

const SAME_M = 3e-7;

const area = (ring: readonly Point[]): number => ring.reduce((sum, p, i) => sum + (p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y), 0) / 2;

/** 📐️ The signed area of a ring (counter-clockwise positive). */
export const ringArea = area;

const perimeter = (ring: readonly Point[]): number => ring.reduce((sum, p, i) => sum + portable.hypot(p.x - ring[(i + 1) % ring.length]!.x, p.y - ring[(i + 1) % ring.length]!.y), 0);

/** 🪡️ Whether a ring has an area that survives the resolution of the chaining (nodes closer than 0.1 micrometre are one node): at least {@link EPS} square metres and a mean width `2 · area / perimeter` of at least a micrometre; a sliver of a few tens of nanometres (the stroke of a segment whose radius is zero) has no boundary the union could keep apart. */
const solid = (ring: readonly Point[]): boolean => Math.abs(area(ring)) > EPS && (2 * Math.abs(area(ring))) / perimeter(ring) >= THIN_M;

const key = (p: Point): string => `${Math.round(p.x / 1e-7)},${Math.round(p.y / 1e-7)}`;

function inside(ring: readonly Point[], p: Point): boolean {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const a = ring[i]!;
    const b = ring[j]!;
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) hit = !hit;
  }
  return hit;
}

/** 📏️ Whether a point lies within `NEAR_M` (10 micrometres) of the boundary of a ring: the polygons of one drawing that repeat each other with differences of a micrometre (a stroke drawn twice from two nearly equal frames) are one polygon, not two whose edges cross in slivers. */
function nearBoundary(ring: readonly Point[], p: Point): boolean {
  for (let i = 0; i < ring.length; i++) {
    const a = ring[i]!;
    const b = ring[(i + 1) % ring.length]!;
    const dx = b.x - a.x;
    const dy = b.y - a.y;
    const length2 = dx * dx + dy * dy;
    const t = length2 === 0 ? 0 : Math.min(1, Math.max(0, ((p.x - a.x) * dx + (p.y - a.y) * dy) / length2));
    if (portable.hypot(a.x + dx * t - p.x, a.y + dy * t - p.y) <= NEAR_M) return true;
  }
  return false;
}

function onEdge(ring: readonly Point[], p: Point): { readonly dx: number; readonly dy: number } | undefined {
  for (let i = 0; i < ring.length; i++) {
    const a = ring[i]!;
    const b = ring[(i + 1) % ring.length]!;
    const dx = b.x - a.x;
    const dy = b.y - a.y;
    const length = portable.hypot(dx, dy);
    if (length < EPS) continue;
    const t = ((p.x - a.x) * dx + (p.y - a.y) * dy) / (length * length);
    if (t < -EPS || t > 1 + EPS) continue;
    if (portable.hypot(a.x + dx * t - p.x, a.y + dy * t - p.y) < SAME_M) return { dx, dy };
  }
  return undefined;
}

function chainPieces(kept: readonly { readonly from: Point; readonly to: Point }[]): { readonly rings: Point[][]; readonly closed: boolean } {
  const cell = 1e-7;
  const grid = new Map<string, number[]>();
  const nodes: Point[] = [];
  const nodeOf = (p: Point): number => {
    const cx = Math.round(p.x / cell);
    const cy = Math.round(p.y / cell);
    for (let dx = -1; dx <= 1; dx++)
      for (let dy = -1; dy <= 1; dy++)
        for (const id of grid.get(`${cx + dx},${cy + dy}`) ?? []) if (portable.hypot(nodes[id]!.x - p.x, nodes[id]!.y - p.y) < cell) return id;
    nodes.push(p);
    grid.set(`${cx},${cy}`, [...(grid.get(`${cx},${cy}`) ?? []), nodes.length - 1]);
    return nodes.length - 1;
  };
  const edges = kept.map((piece) => ({ piece, from: nodeOf(piece.from), to: nodeOf(piece.to) })).filter((edge) => edge.from !== edge.to);
  const starts = new Map<number, number[]>();
  edges.forEach((edge, i) => starts.set(edge.from, [...(starts.get(edge.from) ?? []), i]));
  const used = new Set<number>();
  const rings: Point[][] = [];
  const heading = (i: number): number => portable.atan2(edges[i]!.piece.to.y - edges[i]!.piece.from.y, edges[i]!.piece.to.x - edges[i]!.piece.from.x);
  const turn = (a: number, b: number): number => portable.atan2(portable.sin(heading(b) - heading(a)), portable.cos(heading(b) - heading(a)));
  let closed = true;
  edges.forEach((_, i) => {
    if (used.has(i)) return;
    const ring: Point[] = [];
    let current: number | undefined = i;
    while (current !== undefined && !used.has(current)) {
      used.add(current);
      ring.push(nodes[edges[current]!.from]!);
      const from: number = current;
      const options: number[] = (starts.get(edges[from]!.to) ?? []).filter((candidate) => !used.has(candidate) || candidate === i);
      current = options.sort((p, q) => turn(from, p) - turn(from, q))[0];
      if (current === i) break;
    }
    if (current !== i || ring.length < 3) closed = false;
    if (ring.length >= 3) rings.push(ring);
  });
  return { rings, closed };
}

/** ✂️ Removes the vertices that lie on the straight line of their kept predecessor and their successor (within a nanometre) and do not turn back; a run of closely spaced vertices on a curve is measured against the last kept vertex, so it is never collapsed into its chord. */
const dropCollinear = (rings: readonly (readonly Point[])[]): Point[][] =>
  rings.map((ring) => {
    const straight = (a: Point, p: Point, b: Point): boolean => {
      const length = portable.hypot(b.x - a.x, b.y - a.y);
      return length > 0 && Math.abs((p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)) / length < 1e-9 && (p.x - a.x) * (b.x - p.x) + (p.y - a.y) * (b.y - p.y) >= 0;
    };
    const kept: Point[] = [];
    ring.forEach((p, i) => {
      const before = kept.length > 0 ? kept[kept.length - 1]! : ring[ring.length - 1]!;
      if (!straight(before, p, ring[(i + 1) % ring.length]!)) kept.push(p);
    });
    while (kept.length > 3 && straight(kept[kept.length - 1]!, kept[0]!, kept[1]!)) kept.shift();
    return kept;
  });

/** 🧩️ The edge-splitting union of polygons that already passed the input checks: every edge is cut at the crossings with the other polygons and the pieces no other polygon covers are chained into rings. */
function primaryUnion(polygons: Point[][]): Point[][] {
  const boxes = polygons.map((ring) => ({ minX: Math.min(...ring.map((p) => p.x)), maxX: Math.max(...ring.map((p) => p.x)), minY: Math.min(...ring.map((p) => p.y)), maxY: Math.max(...ring.map((p) => p.y)) }));
  const pieces: { from: Point; to: Point; owner: number }[] = [];
  polygons.forEach((ring, owner) => {
    ring.forEach((a, i) => {
      const b = ring[(i + 1) % ring.length]!;
      const dx = b.x - a.x;
      const dy = b.y - a.y;
      const length2 = dx * dx + dy * dy;
      if (length2 < EPS * EPS) return;
      const cuts = new Set<number>([0, 1]);
      polygons.forEach((other, index) => {
        const box = boxes[index]!;
        if (index === owner || box.maxX < Math.min(a.x, b.x) - SAME_M || box.minX > Math.max(a.x, b.x) + SAME_M || box.maxY < Math.min(a.y, b.y) - SAME_M || box.minY > Math.max(a.y, b.y) + SAME_M) return;
        other.forEach((c, k) => {
          const d = other[(k + 1) % other.length]!;
          const ex = d.x - c.x;
          const ey = d.y - c.y;
          const denominator = dx * ey - dy * ex;
          const along = (q: Point): number => ((q.x - a.x) * dx + (q.y - a.y) * dy) / length2;
          const distance = (q: Point): number => Math.abs((q.x - a.x) * dy - (q.y - a.y) * dx) / Math.sqrt(length2);
          if (Math.abs(denominator) < EPS * Math.sqrt(length2) * portable.hypot(ex, ey)) {
            for (const q of [c, d]) if (distance(q) < SAME_M) cuts.add(along(q));
            return;
          }
          const t = ((c.x - a.x) * ey - (c.y - a.y) * ex) / denominator;
          const u = ((c.x - a.x) * dy - (c.y - a.y) * dx) / denominator;
          if (u >= -EPS && u <= 1 + EPS) cuts.add(t);
        });
      });
      const sorted = [...cuts].filter((t) => t > -EPS && t < 1 + EPS).map((t) => Math.min(1, Math.max(0, t))).sort((p, q) => p - q);
      for (let n = 0; n + 1 < sorted.length; n++) {
        const [t0, t1] = [sorted[n]!, sorted[n + 1]!];
        if ((t1 - t0) * Math.sqrt(length2) < 1e-8) continue;
        pieces.push({ from: { x: a.x + dx * t0, y: a.y + dy * t0 }, to: { x: a.x + dx * t1, y: a.y + dy * t1 }, owner });
      }
    });
  });
  const kept = pieces.filter((piece) => {
    const mid = { x: (piece.from.x + piece.to.x) / 2, y: (piece.from.y + piece.to.y) / 2 };
    const direction = { x: piece.to.x - piece.from.x, y: piece.to.y - piece.from.y };
    for (let index = 0; index < polygons.length; index++) {
      const box = boxes[index]!;
      if (index === piece.owner || mid.x < box.minX - SAME_M || mid.x > box.maxX + SAME_M || mid.y < box.minY - SAME_M || mid.y > box.maxY + SAME_M) continue;
      const edge = onEdge(polygons[index]!, mid);
      if (edge !== undefined) {
        if (edge.dx * direction.x + edge.dy * direction.y < 0) return false;
        if (index < piece.owner) return false;
      } else if (inside(polygons[index]!, mid)) return false;
    }
    return true;
  });
  const chained = chainPieces(kept);
  return chained.closed ? dropCollinear(chained.rings) : [];
}

/** 🚫️ Thrown when the union of polygons that do have area cannot be built: the caller gets a named error, never an empty result. */
export class PolygonUnionError extends Error {
  readonly polygons: number;
  readonly input: readonly (readonly Point[])[];

  constructor(polygons: number, detail: string, input: readonly (readonly Point[])[] = []) {
    super(`the union of ${polygons} polygon(s) with area cannot be built: ${detail}`);
    this.name = "PolygonUnionError";
    this.polygons = polygons;
    this.input = input;
  }
}

type Edge = { readonly a: Point; readonly b: Point; readonly ring: number };

const yAt = (edge: Edge, x: number): number => edge.a.y + ((edge.b.y - edge.a.y) * (x - edge.a.x)) / (edge.b.x - edge.a.x);

/** 🧱️ Exact union by vertical slabs, independent of edge coincidences: between two consecutive event abscissae (vertices and edge crossings) no edges cross, so the union is a stack of trapezoids; the boundary is their top and bottom edges plus the vertical steps where coverage changes between two slabs. Returns the rings and the exact union area. */
function slabUnion(polygons: readonly (readonly Point[])[]): { readonly rings: Point[][]; readonly area: number } {
  const edges: Edge[] = [];
  polygons.forEach((ring, owner) => ring.forEach((a, i) => {
    const b = ring[(i + 1) % ring.length]!;
    if (Math.abs(b.x - a.x) > 1e-12) edges.push(a.x < b.x ? { a, b, ring: owner } : { a: b, b: a, ring: owner });
  }));
  const events = new Set<number>(polygons.flatMap((ring) => ring.map((p) => p.x)));
  for (let i = 0; i < edges.length; i++)
    for (let j = i + 1; j < edges.length; j++) {
      const [p, q] = [edges[i]!, edges[j]!];
      const [dx1, dy1, dx2, dy2] = [p.b.x - p.a.x, p.b.y - p.a.y, q.b.x - q.a.x, q.b.y - q.a.y];
      const denominator = dx1 * dy2 - dy1 * dx2;
      if (Math.abs(denominator) < 1e-18) continue;
      const t = ((q.a.x - p.a.x) * dy2 - (q.a.y - p.a.y) * dx2) / denominator;
      const u = ((q.a.x - p.a.x) * dy1 - (q.a.y - p.a.y) * dx1) / denominator;
      if (t > 0 && t < 1 && u > 0 && u < 1) events.add(p.a.x + dx1 * t);
    }
  const xs = [...events].sort((p, q) => p - q).filter((x, i, all) => i === 0 || x - all[i - 1]! > 1e-12);
  type Interval = { readonly bottom: Edge; readonly top: Edge };
  const slabs: { readonly x0: number; readonly x1: number; readonly intervals: Interval[] }[] = [];
  let total = 0;
  for (let k = 0; k + 1 < xs.length; k++) {
    const [x0, x1] = [xs[k]!, xs[k + 1]!];
    const xm = (x0 + x1) / 2;
    const found: Interval[] = [];
    for (let owner = 0; owner < polygons.length; owner++) {
      const crossing = edges.filter((e) => e.ring === owner && e.a.x < xm && e.b.x > xm).sort((p, q) => yAt(p, xm) - yAt(q, xm));
      for (let i = 0; i + 1 < crossing.length; i += 2) found.push({ bottom: crossing[i]!, top: crossing[i + 1]! });
    }
    found.sort((p, q) => yAt(p.bottom, xm) - yAt(q.bottom, xm));
    const merged: Interval[] = [];
    for (const interval of found) {
      const last = merged[merged.length - 1];
      if (last !== undefined && yAt(interval.bottom, xm) <= yAt(last.top, xm) + 1e-12) {
        if (yAt(interval.top, xm) > yAt(last.top, xm)) merged[merged.length - 1] = { bottom: last.bottom, top: interval.top };
      } else merged.push(interval);
    }
    slabs.push({ x0, x1, intervals: merged });
    for (const { bottom, top } of merged) total += ((yAt(top, x0) - yAt(bottom, x0) + yAt(top, x1) - yAt(bottom, x1)) / 2) * (x1 - x0);
  }
  const pieces: { from: Point; to: Point }[] = [];
  const ends = (slab: (typeof slabs)[number], x: number): [number, number][] => slab.intervals.map((i) => [yAt(i.bottom, x), yAt(i.top, x)]);
  const minus = (from: [number, number][], other: [number, number][]): [number, number][] =>
    from.flatMap(([lo, hi]) => {
      let parts: [number, number][] = [[lo, hi]];
      for (const [olo, ohi] of other) parts = parts.flatMap(([a, b]) => (ohi <= a || olo >= b ? [[a, b] as [number, number]] : [...(olo > a ? [[a, olo] as [number, number]] : []), ...(ohi < b ? [[ohi, b] as [number, number]] : [])]));
      return parts.filter(([a, b]) => b - a > 1e-9);
    });
  slabs.forEach((slab, k) => {
    for (const { bottom, top } of slab.intervals) {
      pieces.push({ from: { x: slab.x0, y: yAt(bottom, slab.x0) }, to: { x: slab.x1, y: yAt(bottom, slab.x1) } });
      pieces.push({ from: { x: slab.x1, y: yAt(top, slab.x1) }, to: { x: slab.x0, y: yAt(top, slab.x0) } });
    }
    const left = k === 0 ? [] : ends(slabs[k - 1]!, slab.x0);
    const right = ends(slab, slab.x0);
    for (const [lo, hi] of minus(left, right)) pieces.push({ from: { x: slab.x0, y: lo }, to: { x: slab.x0, y: hi } });
    for (const [lo, hi] of minus(right, left)) pieces.push({ from: { x: slab.x0, y: hi }, to: { x: slab.x0, y: lo } });
    if (k === slabs.length - 1) for (const [lo, hi] of ends(slab, slab.x1)) pieces.push({ from: { x: slab.x1, y: lo }, to: { x: slab.x1, y: hi } });
  });
  const chained = chainPieces(pieces);
  return { rings: chained.closed ? dropCollinear(chained.rings) : [], area: total };
}

const totalArea = (rings: readonly (readonly Point[])[]): number => rings.reduce((sum, ring) => sum + area(ring), 0);

/** 🧹️ The ring without the vertices that lie within {@link SAME_M} of the last kept one: a curve flattened into steps of nanometres (the stroke of a path of a hair-thin leaf) has no vertex the union could tell apart from its neighbour. */
const distinct = (ring: readonly Point[]): Point[] => ring.reduce<Point[]>((kept, p) => (kept.length === 0 || portable.hypot(p.x - kept[kept.length - 1]!.x, p.y - kept[kept.length - 1]!.y) > SAME_M ? [...kept, p] : kept), []);

/** 🌟️ The union of simple polygons (each at least three points, any orientation) as the rings of its boundary; polygons without area, or slivers thinner than a micrometre, are ignored (an empty result then means no polygon had area) and a single polygon is returned as it is (counter-clockwise, collinear points removed). Consecutive duplicate points are removed first, and a polygon that lies inside another one up to 10 micrometres is dropped. The fast edge-splitting union is checked against what a union can be (not smaller than its largest polygon, not larger than the sum); when it fails the check, the exact slab union takes over, and if that cannot close a boundary either a {@link PolygonUnionError} is thrown: polygons with area never silently produce nothing. */
export function unionPolygons(input: readonly (readonly Point[])[]): Point[][] {
  const clean = input.map(distinct).map((ring) => (ring.length > 1 && portable.hypot(ring[0]!.x - ring[ring.length - 1]!.x, ring[0]!.y - ring[ring.length - 1]!.y) <= SAME_M ? ring.slice(0, -1) : ring));
  const all = clean.filter((ring) => ring.length >= 3 && solid(ring)).map((ring) => (area(ring) < 0 ? [...ring].reverse() : [...ring]));
  const polygons: Point[][] = [];
  for (const ring of [...all].sort((p, q) => area(q) - area(p))) if (!polygons.some((big) => ring.every((p, i) => [p, { x: (p.x + ring[(i + 1) % ring.length]!.x) / 2, y: (p.y + ring[(i + 1) % ring.length]!.y) / 2 }].every((q) => inside(big, q) || nearBoundary(big, q))))) polygons.push(ring);
  if (polygons.length === 0) return [];
  const largest = Math.max(...polygons.map(area));
  const upper = polygons.reduce((sum, ring) => sum + area(ring), 0);
  const coherent = (rings: readonly (readonly Point[])[]): boolean => {
    const a = totalArea(rings);
    return rings.length > 0 && a >= largest * (1 - 1e-6) - 1e-12 && a <= upper * (1 + 1e-6) + 1e-12;
  };
  const fast = primaryUnion(polygons);
  if (coherent(fast)) return fast;
  const exact = slabUnion(polygons);
  if (exact.rings.length > 0 && Math.abs(totalArea(exact.rings) - exact.area) <= 1e-4 * exact.area + 1e-9) return exact.rings;
  throw new PolygonUnionError(polygons.length, "neither the edge-splitting nor the slab union closed a boundary of the expected area", polygons);
}
