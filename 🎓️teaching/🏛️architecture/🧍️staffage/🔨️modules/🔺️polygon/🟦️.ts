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

/** 🚧️ The distance beyond which the boxes of a point and an edge are certainly farther apart than {@link NEAR_M}. */
const APART_M = 2 * NEAR_M;

/** ⚖️ How far (relative) a squared length may lie from a squared limit before the squares alone decide the comparison: far above the rounding of a square, so only a tie within it needs the correctly rounded length. */
const TIE = 1e-9;

/** 📏️ Whether the vector is shorter than the limit, decided as `portable.hypot(dx, dy) < limit` decides it: by the squares, and by the rounded length within {@link TIE} of the limit. */
const shorter = (dx: number, dy: number, limit: number): boolean => {
  const square = dx * dx + dy * dy;
  const bound = limit * limit;
  return square < bound * (1 - TIE) ? true : square > bound * (1 + TIE) ? false : portable.hypot(dx, dy) < limit;
};

/** 📐️ Whether the vector is at most as long as the limit, decided as {@link shorter} does. */
const within = (dx: number, dy: number, limit: number): boolean => {
  const square = dx * dx + dy * dy;
  const bound = limit * limit;
  return square < bound * (1 - TIE) ? true : square > bound * (1 + TIE) ? false : portable.hypot(dx, dy) <= limit;
};

function area(ring: readonly Point[]): number {
  let sum = 0;
  for (let i = 0; i < ring.length; i++) {
    const p = ring[i]!;
    const q = ring[i + 1 === ring.length ? 0 : i + 1]!;
    sum += p.x * q.y - q.x * p.y;
  }
  return sum / 2;
}

/** 📐️ The signed area of a ring (counter-clockwise positive). */
export const ringArea = area;

const perimeter = (ring: readonly Point[], length: (dx: number, dy: number) => number): number => {
  let sum = 0;
  for (let i = 0; i < ring.length; i++) {
    const p = ring[i]!;
    const q = ring[i + 1 === ring.length ? 0 : i + 1]!;
    sum += length(p.x - q.x, p.y - q.y);
  }
  return sum;
};

/** 🪡️ Whether a ring has an area that survives the resolution of the chaining (nodes closer than 0.1 micrometre are one node): at least {@link EPS} square metres and a mean width `2 · area / perimeter` of at least a micrometre; a sliver of a few tens of nanometres (the stroke of a segment whose radius is zero) has no boundary the union could keep apart. */
const solid = (ring: readonly Point[]): boolean => {
  const twice = 2 * Math.abs(area(ring));
  if (!(twice / 2 > EPS)) return false;
  const width = twice / perimeter(ring, (dx, dy) => Math.sqrt(dx * dx + dy * dy));
  return width > THIN_M * (1 + TIE) ? true : width < THIN_M * (1 - TIE) ? false : twice / perimeter(ring, portable.hypot) >= THIN_M;
};

/** 📦️ The bounding box of a ring. */
type Bounds = { readonly minX: number; readonly maxX: number; readonly minY: number; readonly maxY: number };

function boundsOf(ring: readonly Point[]): Bounds {
  let [minX, maxX, minY, maxY] = [Infinity, -Infinity, Infinity, -Infinity];
  for (const { x, y } of ring) {
    if (x < minX) minX = x;
    if (x > maxX) maxX = x;
    if (y < minY) minY = y;
    if (y > maxY) maxY = y;
  }
  return { minX, maxX, minY, maxY };
}

/** 🗂️ Intervals sorted into equal bins along one axis, each interval listed in every bin it comes within {@link APART_M} of, in ascending order: the intervals of the bin of a value are all that reach the value. */
type Bins = { readonly first: number; readonly per: number; readonly last: number; readonly start: Int32Array; readonly items: Int32Array };

const binAt = (bins: Bins, value: number): number => {
  const at = Math.floor((value - bins.first) / bins.per);
  return at > 0 ? (at < bins.last ? at : bins.last) : 0;
};

function binsOf(lo: Float64Array, hi: Float64Array): Bins {
  const count = lo.length;
  let least = Infinity;
  let most = -Infinity;
  for (let i = 0; i < count; i++) {
    if (lo[i]! < least) least = lo[i]!;
    if (hi[i]! > most) most = hi[i]!;
  }
  const total = most > least ? Math.max(1, Math.min(512, count >> 2)) : 1;
  const bins = { first: least, per: most > least ? (most - least) / total : 1, last: total - 1 };
  const start = new Int32Array(total + 1);
  const shell: Bins = { ...bins, start, items: start };
  for (let i = 0; i < count; i++) for (let bin = binAt(shell, lo[i]! - APART_M), to = binAt(shell, hi[i]! + APART_M); bin <= to; bin++) start[bin + 1]!++;
  for (let bin = 0; bin < total; bin++) start[bin + 1]! += start[bin]!;
  const items = new Int32Array(start[total]!);
  const fill = start.slice(0, total);
  for (let i = 0; i < count; i++) for (let bin = binAt(shell, lo[i]! - APART_M), to = binAt(shell, hi[i]! + APART_M); bin <= to; bin++) items[fill[bin]!++] = i;
  return { ...bins, start, items };
}

/** 🧵️ The edges of rings in flat tables, in ring order: start point, direction, box, and the bins of their ordinates. */
type Edges = { readonly starts: readonly number[]; readonly ax: Float64Array; readonly ay: Float64Array; readonly dx: Float64Array; readonly dy: Float64Array; readonly loX: Float64Array; readonly hiX: Float64Array; readonly loY: Float64Array; readonly hiY: Float64Array; readonly rows: Bins };

function edgesOf(rings: readonly (readonly Point[])[]): Edges {
  const starts = [0];
  for (const ring of rings) starts.push(starts[starts.length - 1]! + ring.length);
  const count = starts[rings.length]!;
  const ax = new Float64Array(count);
  const ay = new Float64Array(count);
  const dx = new Float64Array(count);
  const dy = new Float64Array(count);
  const loX = new Float64Array(count);
  const hiX = new Float64Array(count);
  const loY = new Float64Array(count);
  const hiY = new Float64Array(count);
  let k = 0;
  for (const ring of rings)
    for (let i = 0; i < ring.length; i++, k++) {
      const a = ring[i]!;
      const b = ring[i + 1 === ring.length ? 0 : i + 1]!;
      ax[k] = a.x;
      ay[k] = a.y;
      dx[k] = b.x - a.x;
      dy[k] = b.y - a.y;
      loX[k] = Math.min(a.x, b.x);
      hiX[k] = Math.max(a.x, b.x);
      loY[k] = Math.min(a.y, b.y);
      hiY[k] = Math.max(a.y, b.y);
    }
  return { starts, ax, ay, dx, dy, loX, hiX, loY, hiY, rows: binsOf(loY, hiY) };
}

/** 📏️ Whether a point lies inside a ring (even-odd) or within `NEAR_M` (10 micrometres) of its boundary: the polygons of one drawing that repeat each other with differences of a micrometre (a stroke drawn twice from two nearly equal frames) are one polygon, not two whose edges cross in slivers. */
function covers(edges: Edges, x: number, y: number): boolean {
  const { ax, ay, dx, dy, loX, hiX, loY, hiY, rows } = edges;
  const bin = binAt(rows, y);
  const count = ax.length;
  let hit = false;
  for (let n = rows.start[bin]!, end = rows.start[bin + 1]!; n < end; n++) {
    const m = rows.items[n]!;
    if (loY[m]! <= y && hiY[m]! > y) {
      const next = m + 1 === count ? 0 : m + 1;
      if (x < ((ax[m]! - ax[next]!) * (y - ay[next]!)) / (ay[m]! - ay[next]!) + ax[next]!) hit = !hit;
    }
  }
  if (hit) return true;
  for (let n = rows.start[bin]!, end = rows.start[bin + 1]!; n < end; n++) {
    const m = rows.items[n]!;
    if (loX[m]! > x + APART_M || hiX[m]! < x - APART_M || loY[m]! > y + APART_M || hiY[m]! < y - APART_M) continue;
    const length2 = dx[m]! * dx[m]! + dy[m]! * dy[m]!;
    const t = length2 === 0 ? 0 : Math.min(1, Math.max(0, ((x - ax[m]!) * dx[m]! + (y - ay[m]!) * dy[m]!) / length2));
    if (within(ax[m]! + dx[m]! * t - x, ay[m]! + dy[m]! * t - y, NEAR_M)) return true;
  }
  return false;
}

/** 🧲️ The polygons with every vertex that lies within {@link SAME_M} of another polygon moved onto it: onto the nearest vertex of another polygon, else onto the nearest point of one of its edges. The vertices are moved one after the other, each against the positions the earlier ones already took (two rounds), so two vertices that nearly meet end in one point instead of changing places. Edges that are coincident for the resolution of the union (0.3 micrometres) then are coincident in their coordinates, and no piece shorter than the resolution is left between a vertex and the edge it nearly touches; a vertex moves by less than {@link SAME_M} per round, a third of the thinnest polygon the union keeps. */
function snapped(polygons: readonly (readonly Point[])[]): Point[][] {
  const rings = polygons.map((ring) => [...ring]);
  const boxes = rings.map(boundsOf);
  for (let round = 0; round < 2; round++)
    rings.forEach((ring, owner) =>
      ring.forEach((p, at) => {
        const near = rings.filter((_, index) => index !== owner && p.x >= boxes[index]!.minX - APART_M && p.x <= boxes[index]!.maxX + APART_M && p.y >= boxes[index]!.minY - APART_M && p.y <= boxes[index]!.maxY + APART_M);
        if (near.length === 0) return;
        let best: Point | undefined;
        let nearest = SAME_M;
        const take = (q: Point): void => {
          if (!shorter(q.x - p.x, q.y - p.y, SAME_M)) return;
          const d = portable.hypot(q.x - p.x, q.y - p.y);
          if (d < nearest) [best, nearest] = [q, d];
        };
        for (const other of near) other.forEach(take);
        if (best === undefined)
          for (const other of near)
            other.forEach((a, i) => {
              const b = other[i + 1 === other.length ? 0 : i + 1]!;
              if ((a.x < b.x ? a.x : b.x) > p.x + APART_M || (a.x > b.x ? a.x : b.x) < p.x - APART_M || (a.y < b.y ? a.y : b.y) > p.y + APART_M || (a.y > b.y ? a.y : b.y) < p.y - APART_M) return;
              const dx = b.x - a.x;
              const dy = b.y - a.y;
              const length2 = dx * dx + dy * dy;
              if (length2 < EPS * EPS) return;
              const t = ((p.x - a.x) * dx + (p.y - a.y) * dy) / length2;
              if (t > 0 && t < 1) take({ x: a.x + dx * t, y: a.y + dy * t });
            });
        if (best !== undefined) ring[at] = best;
      }),
    );
  return rings.map(distinct).filter((ring) => ring.length >= 3 && solid(ring)).map((ring) => (area(ring) < 0 ? ring.reverse() : ring));
}

function chainPieces(kept: readonly { readonly from: Point; readonly to: Point }[], cell = 1e-7): { readonly rings: Point[][]; readonly closed: boolean } {
  const grid = new Map<number, Map<number, number[]>>();
  const nodes: Point[] = [];
  const nodeOf = (p: Point): number => {
    const cx = Math.round(p.x / cell);
    const cy = Math.round(p.y / cell);
    for (let dx = -1; dx <= 1; dx++) {
      const column = grid.get(cx + dx);
      if (column === undefined) continue;
      for (let dy = -1; dy <= 1; dy++) for (const id of column.get(cy + dy) ?? []) if (shorter(nodes[id]!.x - p.x, nodes[id]!.y - p.y, cell)) return id;
    }
    const column = grid.get(cx) ?? new Map<number, number[]>();
    grid.set(cx, column);
    const ids = column.get(cy);
    if (ids === undefined) column.set(cy, [nodes.length]);
    else ids.push(nodes.length);
    nodes.push(p);
    return nodes.length - 1;
  };
  const edges = kept.map((piece) => ({ piece, from: nodeOf(piece.from), to: nodeOf(piece.to) })).filter((edge) => edge.from !== edge.to);
  const starts = new Map<number, number[]>();
  edges.forEach((edge, i) => {
    const ids = starts.get(edge.from);
    if (ids === undefined) starts.set(edge.from, [i]);
    else ids.push(i);
  });
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
      if (!((p.x - a.x) * (b.x - p.x) + (p.y - a.y) * (b.y - p.y) >= 0)) return false;
      const cross = Math.abs((p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x));
      const off = cross / Math.sqrt((b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y));
      if (off > 1e-9 * (1 + TIE) && off < Infinity) return false;
      const length = portable.hypot(b.x - a.x, b.y - a.y);
      return length > 0 && cross / length < 1e-9;
    };
    const kept: Point[] = [];
    ring.forEach((p, i) => {
      const before = kept.length > 0 ? kept[kept.length - 1]! : ring[ring.length - 1]!;
      if (!straight(before, p, ring[(i + 1) % ring.length]!)) kept.push(p);
    });
    while (kept.length > 3 && straight(kept[kept.length - 1]!, kept[0]!, kept[1]!)) kept.shift();
    return kept;
  });

/** 🧩️ The edge-splitting union of polygons that already passed the input checks: every edge is cut at the crossings with the other polygons and the pieces no other polygon covers are chained into rings. The edges of all polygons lie in flat tables; a pair of edges, or a piece and an edge, is looked at only when their boxes come nearer than any cut or coincidence reaches.
 *
 * A piece lies on an edge when its two ends and its middle are within {@link SAME_M} of it. The middle alone does not say it: a piece shorter than a micrometre that crosses an edge at an angle has its middle on that edge and is still a piece of the boundary. */
function primaryUnion(polygons: Point[][]): Point[][] {
  const { starts, ax, ay, dx, dy, loX, hiX, loY, hiY, rows } = edgesOf(polygons);
  const count = ax.length;
  const boxes = polygons.map(boundsOf);
  const boxMinX = Float64Array.from(boxes, (box) => box.minX);
  const boxMaxX = Float64Array.from(boxes, (box) => box.maxX);
  const boxMinY = Float64Array.from(boxes, (box) => box.minY);
  const boxMaxY = Float64Array.from(boxes, (box) => box.maxY);
  const ownerOf = new Int32Array(count);
  for (let index = 0; index < polygons.length; index++) ownerOf.fill(index, starts[index]!, starts[index + 1]!);
  let extent = 0;
  for (let index = 0; index < polygons.length; index++) extent = Math.max(extent, Math.abs(boxMinX[index]!), Math.abs(boxMaxX[index]!), Math.abs(boxMinY[index]!), Math.abs(boxMaxY[index]!));
  const reach = 1e-5 * extent + APART_M;
  let spanX = 0;
  let spanY = 0;
  for (let k = 0; k < count; k++) {
    spanX += hiX[k]! - loX[k]!;
    spanY += hiY[k]! - loY[k]!;
  }
  const width = boxes.reduce((most, box) => Math.max(most, box.maxX), -Infinity) - boxes.reduce((least, box) => Math.min(least, box.minX), Infinity);
  const height = boxes.reduce((most, box) => Math.max(most, box.maxY), -Infinity) - boxes.reduce((least, box) => Math.min(least, box.minY), Infinity);
  const upright = spanX * height < spanY * width;
  const lanes = upright ? binsOf(loX, hiX) : rows;
  const seen = new Int32Array(count);
  const pieces: { from: Point; to: Point; owner: number }[] = [];
  const cuts: number[] = [];
  polygons.forEach((ring, owner) => {
    for (let k = starts[owner]!, i = 0; i < ring.length; i++, k++) {
      const a = ring[i]!;
      const ex = dx[k]!;
      const ey = dy[k]!;
      const length2 = ex * ex + ey * ey;
      if (length2 < EPS * EPS) continue;
      const length = Math.sqrt(length2);
      cuts.length = 0;
      cuts.push(0, 1);
      const x0 = loX[k]!;
      const x1 = hiX[k]!;
      const y0 = loY[k]!;
      const y1 = hiY[k]!;
      const farX0 = x0 - SAME_M;
      const farX1 = x1 + SAME_M;
      const farY0 = y0 - SAME_M;
      const farY1 = y1 + SAME_M;
      const outX0 = x0 - reach;
      const outX1 = x1 + reach;
      const outY0 = y0 - reach;
      const outY1 = y1 + reach;
      const first = lanes.start[binAt(lanes, upright ? outX0 : outY0)]!;
      const listed = lanes.start[binAt(lanes, upright ? outX1 : outY1) + 1]! - first;
      const every = listed >= count;
      for (let n = every ? 0 : first, end = every ? count : first + listed; n < end; n++) {
        const m = every ? n : lanes.items[n]!;
        if (seen[m] === k + 1 || hiX[m]! < outX0 || loX[m]! > outX1 || hiY[m]! < outY0 || loY[m]! > outY1) continue;
        seen[m] = k + 1;
        const index = ownerOf[m]!;
        if (index === owner || boxMaxX[index]! < farX0 || boxMinX[index]! > farX1 || boxMaxY[index]! < farY0 || boxMinY[index]! > farY1) continue;
        const cx = ax[m]!;
        const cy = ay[m]!;
        const fx = dx[m]!;
        const fy = dy[m]!;
        const next = m + 1 === starts[index + 1]! ? starts[index]! : m + 1;
        const qx = ax[next]!;
        const qy = ay[next]!;
        const denominator = ex * fy - ey * fx;
        const nearC = Math.abs((cx - a.x) * ey - (cy - a.y) * ex) / length < SAME_M;
        const nearD = Math.abs((qx - a.x) * ey - (qy - a.y) * ex) / length < SAME_M;
        const slope = Math.abs(denominator);
        const flat = EPS * length * Math.sqrt(fx * fx + fy * fy);
        if ((nearC && nearD) || (slope < flat * (1 + TIE) && (slope < flat * (1 - TIE) || slope < EPS * length * portable.hypot(fx, fy)))) {
          if (nearC) cuts.push(((cx - a.x) * ex + (cy - a.y) * ey) / length2);
          if (nearD) cuts.push(((qx - a.x) * ex + (qy - a.y) * ey) / length2);
          continue;
        }
        const t = ((cx - a.x) * fy - (cy - a.y) * fx) / denominator;
        const u = ((cx - a.x) * ey - (cy - a.y) * ex) / denominator;
        if (u >= -EPS && u <= 1 + EPS) cuts.push(t);
      }
      const sorted = cuts.filter((t) => t > -EPS && t < 1 + EPS).map((t) => Math.min(1, Math.max(0, t))).sort((p, q) => p - q);
      for (let n = 0; n + 1 < sorted.length; n++) {
        const t0 = sorted[n]!;
        const t1 = sorted[n + 1]!;
        if ((t1 - t0) * length < 1e-8) continue;
        pieces.push({ from: { x: a.x + ex * t0, y: a.y + ey * t0 }, to: { x: a.x + ex * t1, y: a.y + ey * t1 }, owner });
      }
    }
  });
  const kept = pieces.filter(({ from, to, owner }) => {
    const x = (from.x + to.x) / 2;
    const y = (from.y + to.y) / 2;
    const wayX = to.x - from.x;
    const wayY = to.y - from.y;
    const bin = binAt(rows, y);
    const end = rows.start[bin + 1]!;
    for (let n = rows.start[bin]!; n < end; ) {
      const index = ownerOf[rows.items[n]!]!;
      const stop = starts[index + 1]!;
      const from0 = n;
      while (n < end && rows.items[n]! < stop) n++;
      if (index === owner || x < boxMinX[index]! - SAME_M || x > boxMaxX[index]! + SAME_M || y < boxMinY[index]! - SAME_M || y > boxMaxY[index]! + SAME_M) continue;
      let on = -1;
      for (let i = from0; i < n; i++) {
        const m = rows.items[i]!;
        if (hiX[m]! < x - APART_M || loX[m]! > x + APART_M || hiY[m]! < y - APART_M || loY[m]! > y + APART_M) continue;
        const cx = ax[m]!;
        const cy = ay[m]!;
        const fx = dx[m]!;
        const fy = dy[m]!;
        const length = portable.hypot(fx, fy);
        if (length < EPS) continue;
        const slack = SAME_M / length;
        const lies = (px: number, py: number): boolean => {
          const t = ((px - cx) * fx + (py - cy) * fy) / (length * length);
          return t >= -slack && t <= 1 + slack && shorter(cx + fx * t - px, cy + fy * t - py, SAME_M);
        };
        if (lies(from.x, from.y) && lies(x, y) && lies(to.x, to.y)) {
          on = m;
          break;
        }
      }
      if (on >= 0) {
        if (dx[on]! * wayX + dy[on]! * wayY < 0) return false;
        if (index < owner) return false;
        continue;
      }
      let covered = false;
      for (let i = from0; i < n; i++) {
        const m = rows.items[i]!;
        if (loY[m]! > y || hiY[m]! <= y) continue;
        const next = m + 1 === stop ? starts[index]! : m + 1;
        if (x < ((ax[m]! - ax[next]!) * (y - ay[next]!)) / (ay[m]! - ay[next]!) + ax[next]!) covered = !covered;
      }
      if (covered) return false;
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
  const reach = 1e-5 * polygons.reduce((most, ring) => ring.reduce((far, p) => Math.max(far, Math.abs(p.x), Math.abs(p.y)), most), 0) + APART_M;
  const count = edges.length;
  const ax = Float64Array.from(edges, (edge) => edge.a.x);
  const ay = Float64Array.from(edges, (edge) => edge.a.y);
  const bx = Float64Array.from(edges, (edge) => edge.b.x);
  const by = Float64Array.from(edges, (edge) => edge.b.y);
  for (let i = 0; i < count; i++) {
    const dx1 = bx[i]! - ax[i]!;
    const dy1 = by[i]! - ay[i]!;
    const length1 = dx1 * dx1 + dy1 * dy1;
    const lowY = Math.min(ay[i]!, by[i]!) - reach;
    const highY = Math.max(ay[i]!, by[i]!) + reach;
    for (let j = i + 1; j < count; j++) {
      const dx2 = bx[j]! - ax[j]!;
      const dy2 = by[j]! - ay[j]!;
      const denominator = dx1 * dy2 - dy1 * dx2;
      if (Math.abs(denominator) < 1e-18) continue;
      const apart = ax[j]! > bx[i]! + reach || bx[j]! < ax[i]! - reach || (ay[j]! > highY && by[j]! > highY) || (ay[j]! < lowY && by[j]! < lowY);
      if (apart && denominator * denominator > EPS * EPS * length1 * (dx2 * dx2 + dy2 * dy2)) continue;
      const t = ((ax[j]! - ax[i]!) * dy2 - (ay[j]! - ay[i]!) * dx2) / denominator;
      const u = ((ax[j]! - ax[i]!) * dy1 - (ay[j]! - ay[i]!) * dx1) / denominator;
      if (t > 0 && t < 1 && u > 0 && u < 1) events.add(ax[i]! + dx1 * t);
    }
  }
  const xs = [...events].sort((p, q) => p - q).filter((x, i, all) => i === 0 || x - all[i - 1]! > 1e-12);
  type Interval = { readonly bottom: Edge; readonly top: Edge };
  const slabs: { readonly x0: number; readonly x1: number; readonly intervals: Interval[] }[] = [];
  let total = 0;
  const byStart = edges.map((_, index) => index).sort((p, q) => ax[p]! - ax[q]!);
  let entered = 0;
  let active: number[] = [];
  type Crossing = { readonly edge: Edge; readonly y: number };
  for (let k = 0; k + 1 < xs.length; k++) {
    const x0 = xs[k]!;
    const x1 = xs[k + 1]!;
    const xm = (x0 + x1) / 2;
    while (entered < count && ax[byStart[entered]!]! < xm) active.push(byStart[entered++]!);
    active = active.filter((index) => bx[index]! > xm).sort((p, q) => p - q);
    const spans: { readonly bottom: Edge; readonly top: Edge; readonly y: number }[] = [];
    for (let from = 0; from < active.length; ) {
      const owner = edges[active[from]!]!.ring;
      const crossing: Crossing[] = [];
      for (; from < active.length && edges[active[from]!]!.ring === owner; from++) crossing.push({ edge: edges[active[from]!]!, y: yAt(edges[active[from]!]!, xm) });
      crossing.sort((p, q) => p.y - q.y);
      for (let i = 0; i + 1 < crossing.length; i += 2) spans.push({ bottom: crossing[i]!.edge, top: crossing[i + 1]!.edge, y: crossing[i]!.y });
    }
    const found: Interval[] = spans.sort((p, q) => p.y - q.y);
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
  const chained = chainPieces(pieces, 1e-9);
  return { rings: chained.closed ? dropCollinear(chained.rings) : [], area: total };
}

/** 🧭️ The angles (radians) the slab union is repeated at when it does not close in the frame it was given. The slabs are vertical: an edge that stands within nanometres of the vertical (the face of a prism swept along the up axis of the view, a profile seen along itself) has a slope of 10⁸ and more, which the ordinates of a slab cannot carry; turned by one of these angles the same polygons have no such edge. */
const SLAB_TURNS: readonly number[] = [0.3, 0.7, 1.1, 1.9];

/** 🔄️ The {@link slabUnion} of the polygons turned by `angle` about the origin, with its rings turned back: the same union, built with slabs that stand at that angle to the polygons. */
function turnedSlabUnion(polygons: readonly (readonly Point[])[], angle: number): { readonly rings: Point[][]; readonly area: number } {
  const c = portable.cos(angle);
  const s = portable.sin(angle);
  const exact = slabUnion(polygons.map((ring) => ring.map((p) => ({ x: c * p.x - s * p.y, y: s * p.x + c * p.y }))));
  return { rings: exact.rings.map((ring) => ring.map((p) => ({ x: c * p.x + s * p.y, y: c * p.y - s * p.x }))), area: exact.area };
}

const totalArea = (rings: readonly (readonly Point[])[]): number => rings.reduce((sum, ring) => sum + area(ring), 0);

/** 🧹️ The ring without the vertices that lie within {@link SAME_M} of the last kept one: a curve flattened into steps of nanometres (the stroke of a path of a hair-thin leaf) has no vertex the union could tell apart from its neighbour. */
const distinct = (ring: readonly Point[]): Point[] => {
  const kept: Point[] = [];
  for (const p of ring) {
    const last = kept[kept.length - 1];
    if (last === undefined || !within(p.x - last.x, p.y - last.y, SAME_M)) kept.push(p);
  }
  return kept;
};

/** 🌟️ The union of simple polygons (each at least three points, any orientation) as the rings of its boundary; polygons without area, or slivers thinner than a micrometre, are ignored (an empty result then means no polygon had area) and a single polygon is returned as it is (counter-clockwise, collinear points removed). Consecutive duplicate points are removed first, and a polygon that lies inside another one up to 10 micrometres is dropped. The fast edge-splitting union is checked against what a union can be (not smaller than its largest polygon, not larger than the sum); when it fails the check, it is repeated on the polygons whose nearly touching vertices and edges are joined ({@link snapped}), then the exact slab union takes over (on the polygons as given, then on the joined ones, then on both turned by each of {@link SLAB_TURNS}, since edges within nanometres of the vertical defeat vertical slabs), and if that cannot close a boundary either a {@link PolygonUnionError} is thrown: polygons with area never silently produce nothing. */
export function unionPolygons(input: readonly (readonly Point[])[]): Point[][] {
  const clean = input.map(distinct).map((ring) => (ring.length > 1 && within(ring[0]!.x - ring[ring.length - 1]!.x, ring[0]!.y - ring[ring.length - 1]!.y, SAME_M) ? ring.slice(0, -1) : ring));
  const all = clean.filter((ring) => ring.length >= 3 && solid(ring)).map((ring) => (area(ring) < 0 ? [...ring].reverse() : [...ring]));
  const polygons: Point[][] = [];
  const boxes: Bounds[] = [];
  const tables: (Edges | undefined)[] = [];
  for (const { ring } of all.map((candidate) => ({ ring: candidate, size: area(candidate) })).sort((p, q) => q.size - p.size)) {
    const box = boundsOf(ring);
    const held = polygons.some((big, index) => {
      const outer = boxes[index]!;
      if (box.minX < outer.minX - APART_M || box.maxX > outer.maxX + APART_M || box.minY < outer.minY - APART_M || box.maxY > outer.maxY + APART_M) return false;
      const edges = (tables[index] ??= edgesOf([big]));
      return ring.every((p, i) => {
        const next = ring[i + 1 === ring.length ? 0 : i + 1]!;
        return covers(edges, p.x, p.y) && covers(edges, (p.x + next.x) / 2, (p.y + next.y) / 2);
      });
    });
    if (held) continue;
    polygons.push(ring);
    boxes.push(box);
  }
  if (polygons.length === 0) return [];
  const largest = Math.max(...polygons.map(area));
  const upper = polygons.reduce((sum, ring) => sum + area(ring), 0);
  const coherent = (rings: readonly (readonly Point[])[]): boolean => {
    const a = totalArea(rings);
    return rings.length > 0 && a >= largest * (1 - 1e-6) - 1e-12 && a <= upper * (1 + 1e-6) + 1e-12;
  };
  const fast = primaryUnion(polygons);
  if (coherent(fast)) return fast;
  const near = snapped(polygons);
  const joined = primaryUnion(near);
  if (coherent(joined)) return joined;
  const closes = (exact: { readonly rings: Point[][]; readonly area: number }): boolean => exact.rings.length > 0 && Math.abs(totalArea(exact.rings) - exact.area) <= 1e-4 * exact.area + 1e-9;
  for (const candidate of [polygons, near]) {
    const exact = slabUnion(candidate);
    if (closes(exact)) return exact.rings;
  }
  for (const angle of SLAB_TURNS)
    for (const candidate of [polygons, near]) {
      const exact = turnedSlabUnion(candidate, angle);
      if (closes(exact)) return exact.rings;
    }
  throw new PolygonUnionError(polygons.length, "neither the edge-splitting nor the slab union closed a boundary of the expected area", polygons);
}
