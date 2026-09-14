/** 📍 Spatial statistics: the TypeScript twin of `semio-viz-spatial`. Delaunay triangulation and its
 * Voronoi dual, the convex hull, hexagonal binning, marching-squares contours and two-dimensional
 * kernel density estimation.
 * @see ../../../🖋️latex/semio-viz-spatial.sty
 */
import { kernelDensity1d, silvermanBandwidth, thresholdSturges, vizKernel, type VizKernel } from "../🧮transform/🟦️.ts";
import { niceDomain, ticks } from "../📐scale/🟦️.ts";
import type { VizPoint } from "../🧬️schema/🟦️.ts";

//#region 🔖️Delaunay
/** 🔺️ A triangulation: triples of point indices, plus the hull in counter-clockwise order. */
export type VizTriangulation = { readonly triangles: readonly (readonly [number, number, number])[]; readonly hull: readonly number[]; readonly points: readonly VizPoint[] };

function circumcircle(a: VizPoint, b: VizPoint, c: VizPoint): { x: number; y: number; r2: number } | null {
  const ax = a[0];
  const ay = a[1];
  const bx = b[0];
  const by = b[1];
  const cx = c[0];
  const cy = c[1];
  const d = 2 * (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by));
  if (Math.abs(d) < 1e-12) return null;
  const a2 = ax * ax + ay * ay;
  const b2 = bx * bx + by * by;
  const c2 = cx * cx + cy * cy;
  const x = (a2 * (by - cy) + b2 * (cy - ay) + c2 * (ay - by)) / d;
  const y = (a2 * (cx - bx) + b2 * (ax - cx) + c2 * (bx - ax)) / d;
  return { x, y, r2: (ax - x) ** 2 + (ay - y) ** 2 };
}

/** 🔺️ The circumcentre of a triangle, the Voronoi vertex it induces. */
export function vizCircumcenter(a: VizPoint, b: VizPoint, c: VizPoint): VizPoint | null {
  const circle = circumcircle(a, b, c);
  return circle === null ? null : [circle.x, circle.y];
}

/** 🔺️ The Delaunay triangulation by Bowyer–Watson incremental insertion. For points in general
 * position the triangulation is unique, so it agrees with any other correct implementation. */
export function vizDelaunay(points: readonly VizPoint[]): VizTriangulation {
  const n = points.length;
  if (n < 3) return { triangles: [], hull: points.map((_, i) => i), points };
  let minX = Number.POSITIVE_INFINITY;
  let minY = Number.POSITIVE_INFINITY;
  let maxX = Number.NEGATIVE_INFINITY;
  let maxY = Number.NEGATIVE_INFINITY;
  for (const [x, y] of points) {
    if (x < minX) minX = x;
    if (x > maxX) maxX = x;
    if (y < minY) minY = y;
    if (y > maxY) maxY = y;
  }
  const dmax = Math.max(maxX - minX, maxY - minY) || 1;
  const midX = (minX + maxX) / 2;
  const midY = (minY + maxY) / 2;
  const all: VizPoint[] = [...points, [midX - 20 * dmax, midY - dmax], [midX, midY + 20 * dmax], [midX + 20 * dmax, midY - dmax]];
  let triangles: [number, number, number][] = [[n, n + 1, n + 2]];
  for (let i = 0; i < n; i += 1) {
    const p = all[i]!;
    const bad: [number, number, number][] = [];
    const kept: [number, number, number][] = [];
    for (const triangle of triangles) {
      const circle = circumcircle(all[triangle[0]]!, all[triangle[1]]!, all[triangle[2]]!);
      if (circle !== null && (p[0] - circle.x) ** 2 + (p[1] - circle.y) ** 2 <= circle.r2 * (1 + 1e-12)) bad.push(triangle);
      else kept.push(triangle);
    }
    const edgeCount = new Map<string, [number, number]>();
    const seen = new Map<string, number>();
    for (const [a, b, c] of bad) {
      for (const edge of [
        [a, b],
        [b, c],
        [c, a],
      ] as [number, number][]) {
        const key = edge[0] < edge[1] ? `${edge[0]}:${edge[1]}` : `${edge[1]}:${edge[0]}`;
        seen.set(key, (seen.get(key) ?? 0) + 1);
        edgeCount.set(key, edge);
      }
    }
    triangles = kept;
    for (const [key, count] of seen) if (count === 1) triangles.push([edgeCount.get(key)![0], edgeCount.get(key)![1], i]);
  }
  const real = triangles.filter((triangle) => triangle.every((index) => index < n)).map((triangle) => orientCcw(triangle, points));
  const normalized = real.map((triangle) => [...triangle].sort((a, b) => a - b) as [number, number, number]);
  const order = normalized.map((triangle, i) => ({ triangle: real[i]!, key: triangle }));
  order.sort((a, b) => a.key[0] - b.key[0] || a.key[1] - b.key[1] || a.key[2] - b.key[2]);
  return { triangles: order.map((entry) => entry.triangle), hull: vizConvexHull(points), points };
}

function orientCcw(triangle: [number, number, number], points: readonly VizPoint[]): [number, number, number] {
  const [a, b, c] = triangle;
  const cross = (points[b]![0] - points[a]![0]) * (points[c]![1] - points[a]![1]) - (points[c]![0] - points[a]![0]) * (points[b]![1] - points[a]![1]);
  return cross < 0 ? [a, c, b] : [a, b, c];
}
//#endregion 🔖️Delaunay

//#region 🔖️Hull
/** 🔷️ The convex hull as point indices in counter-clockwise order, by Andrew's monotone chain. */
export function vizConvexHull(points: readonly VizPoint[]): number[] {
  const n = points.length;
  if (n < 3) return points.map((_, i) => i);
  const order = points.map((_, i) => i).sort((a, b) => points[a]![0] - points[b]![0] || points[a]![1] - points[b]![1]);
  const cross = (o: number, a: number, b: number): number => (points[a]![0] - points[o]![0]) * (points[b]![1] - points[o]![1]) - (points[a]![1] - points[o]![1]) * (points[b]![0] - points[o]![0]);
  const lower: number[] = [];
  for (const i of order) {
    while (lower.length >= 2 && cross(lower[lower.length - 2]!, lower[lower.length - 1]!, i) <= 0) lower.pop();
    lower.push(i);
  }
  const upper: number[] = [];
  for (let k = order.length - 1; k >= 0; k -= 1) {
    const i = order[k]!;
    while (upper.length >= 2 && cross(upper[upper.length - 2]!, upper[upper.length - 1]!, i) <= 0) upper.pop();
    upper.push(i);
  }
  lower.pop();
  upper.pop();
  return [...lower, ...upper];
}
//#endregion 🔖️Hull

//#region 🔖️Voronoi
/** 🔶️ A Voronoi diagram clipped to a rectangle: one convex cell polygon per input point. */
export type VizVoronoi = { readonly cells: readonly (readonly VizPoint[])[]; readonly bounds: readonly [number, number, number, number] };

function clipHalfPlane(polygon: readonly VizPoint[], inside: (point: VizPoint) => number): VizPoint[] {
  const out: VizPoint[] = [];
  const n = polygon.length;
  for (let i = 0; i < n; i += 1) {
    const a = polygon[i]!;
    const b = polygon[(i + 1) % n]!;
    const da = inside(a);
    const db = inside(b);
    if (da >= 0) out.push(a);
    if ((da > 0 && db < 0) || (da < 0 && db > 0)) {
      const t = da / (da - db);
      out.push([a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])]);
    }
  }
  return out;
}

/** 🔶️ The Voronoi tessellation of a point set, clipped to `[xmin, ymin, xmax, ymax]`. */
export function vizVoronoi(points: readonly VizPoint[], bounds: readonly [number, number, number, number]): VizVoronoi {
  const [x0, y0, x1, y1] = bounds;
  const cells = points.map((site) => {
    let cell: VizPoint[] = [
      [x0, y0],
      [x1, y0],
      [x1, y1],
      [x0, y1],
    ];
    for (const other of points) {
      if (other === site) continue;
      const dx = other[0] - site[0];
      const dy = other[1] - site[1];
      if (dx === 0 && dy === 0) continue;
      const mx = (site[0] + other[0]) / 2;
      const my = (site[1] + other[1]) / 2;
      cell = clipHalfPlane(cell, (point) => -(dx * (point[0] - mx) + dy * (point[1] - my)));
      if (cell.length === 0) break;
    }
    return cell;
  });
  return { cells, bounds };
}

/** 🔶️ The signed area of a simple polygon; positive when it is counter-clockwise. */
export function vizPolygonArea(polygon: readonly VizPoint[]): number {
  let area = 0;
  for (let i = 0, n = polygon.length; i < n; i += 1) {
    const a = polygon[i]!;
    const b = polygon[(i + 1) % n]!;
    area += a[0] * b[1] - b[0] * a[1];
  }
  return area / 2;
}
//#endregion 🔖️Voronoi

//#region 🔖️Hexbin
/** 🐝 One hexagonal bin: its centre and the points that fell into it. */
export type VizHexBin<T> = { readonly x: number; readonly y: number; readonly values: readonly T[] };

const THIRD_PI = Math.PI / 3;
const HEX_ANGLES = [0, THIRD_PI, 2 * THIRD_PI, 3 * THIRD_PI, 4 * THIRD_PI, 5 * THIRD_PI];

/** 🐝 `\SemioVizLayout{hexbin}`: bins points onto a hexagonal lattice of the given radius. */
export function vizHexbin<T>(points: readonly T[], options: { radius?: number; x?: (row: T) => number; y?: (row: T) => number } = {}): VizHexBin<T>[] {
  const r = options.radius ?? 1;
  const dx = r * 2 * Math.sin(THIRD_PI);
  const dy = r * 1.5;
  const accessX = options.x ?? ((row: T) => (row as unknown as VizPoint)[0]);
  const accessY = options.y ?? ((row: T) => (row as unknown as VizPoint)[1]);
  const byId = new Map<string, { x: number; y: number; values: T[] }>();
  const bins: { x: number; y: number; values: T[] }[] = [];
  for (const point of points) {
    const rawX = +accessX(point);
    const rawY = +accessY(point);
    if (Number.isNaN(rawX) || Number.isNaN(rawY)) continue;
    const py = rawY / dy;
    let pj = Math.round(py);
    const px = rawX / dx - (pj & 1) / 2;
    let pi = Math.round(px);
    const py1 = py - pj;
    if (Math.abs(py1) * 3 > 1) {
      const px1 = px - pi;
      const pi2 = pi + (px < pi ? -1 : 1) / 2;
      const pj2 = pj + (py < pj ? -1 : 1);
      const px2 = px - pi2;
      const py2 = py - pj2;
      if (px1 * px1 + py1 * py1 > px2 * px2 + py2 * py2) {
        pi = pi2 + ((pj & 1) ? 1 : -1) / 2;
        pj = pj2;
      }
    }
    const id = `${pi}-${pj}`;
    const bin = byId.get(id);
    if (bin !== undefined) bin.values.push(point);
    else {
      const created = { x: (pi + (pj & 1) / 2) * dx, y: pj * dy, values: [point] };
      byId.set(id, created);
      bins.push(created);
    }
  }
  return bins;
}

/** 🐝 The hexagon outline of one bin, centred on the origin. */
export function vizHexagon(radius: number): VizPoint[] {
  return HEX_ANGLES.map((angle) => [radius * Math.sin(angle), -radius * Math.cos(angle)] as VizPoint);
}
//#endregion 🔖️Hexbin

//#region 🔖️Contour
const MARCHING_CASES: readonly (readonly (readonly [readonly [number, number], readonly [number, number]])[])[] = [
  [],
  [
    [
      [1, 1.5],
      [0.5, 1],
    ],
  ],
  [
    [
      [1.5, 1],
      [1, 1.5],
    ],
  ],
  [
    [
      [1.5, 1],
      [0.5, 1],
    ],
  ],
  [
    [
      [1, 0.5],
      [1.5, 1],
    ],
  ],
  [
    [
      [1, 1.5],
      [0.5, 1],
    ],
    [
      [1, 0.5],
      [1.5, 1],
    ],
  ],
  [
    [
      [1, 0.5],
      [1, 1.5],
    ],
  ],
  [
    [
      [1, 0.5],
      [0.5, 1],
    ],
  ],
  [
    [
      [0.5, 1],
      [1, 0.5],
    ],
  ],
  [
    [
      [1, 1.5],
      [1, 0.5],
    ],
  ],
  [
    [
      [0.5, 1],
      [1, 0.5],
    ],
    [
      [1.5, 1],
      [1, 1.5],
    ],
  ],
  [
    [
      [1.5, 1],
      [1, 0.5],
    ],
  ],
  [
    [
      [0.5, 1],
      [1.5, 1],
    ],
  ],
  [
    [
      [1, 1.5],
      [1.5, 1],
    ],
  ],
  [
    [
      [0.5, 1],
      [1, 1.5],
    ],
  ],
  [],
];

/** 🗺️ One iso-band: a `MultiPolygon` whose rings enclose the region above `value`. */
export type VizContour = { readonly type: "MultiPolygon"; readonly value: number; readonly coordinates: readonly (readonly (readonly VizPoint[])[])[] };

function ringArea(ring: readonly VizPoint[]): number {
  let area = 0;
  const n = ring.length;
  for (let i = 0; i < n; i += 1) {
    const a = ring[i]!;
    const b = ring[(i + 1) % n]!;
    area += a[1] * b[0] - a[0] * b[1];
  }
  return area / 2;
}

function ringContains(ring: readonly VizPoint[], point: VizPoint): number {
  let inside = -1;
  const n = ring.length;
  for (let i = 0, j = n - 1; i < n; j = i, i += 1) {
    const a = ring[j]!;
    const b = ring[i]!;
    if (segmentContains(a, b, point)) return 0;
    if (b[1] > point[1] !== a[1] > point[1] && point[0] < ((a[0] - b[0]) * (point[1] - b[1])) / (a[1] - b[1]) + b[0]) inside = -inside;
  }
  return inside;
}

function segmentContains(a: VizPoint, b: VizPoint, c: VizPoint): boolean {
  if (a[1] === b[1]) return collinear(a, b, c) && Math.min(a[0], b[0]) <= c[0] && c[0] <= Math.max(a[0], b[0]);
  return collinear(a, b, c) && Math.min(a[1], b[1]) <= c[1] && c[1] <= Math.max(a[1], b[1]);
}

function collinear(a: VizPoint, b: VizPoint, c: VizPoint): boolean {
  return Math.abs((b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1])) < 1e-9;
}

function polygonContains(polygon: readonly (readonly VizPoint[])[], hole: readonly VizPoint[]): boolean {
  for (const point of hole) {
    const state = ringContains(polygon[0]!, point);
    if (state !== 0) return state > 0;
  }
  return false;
}

function smooth1(x: number, v0: number, v1: number, value: number): number {
  const a = value - v0;
  const b = v1 - v0;
  const d = Number.isFinite(a) || Number.isFinite(b) ? a / b : Math.sign(a) / Math.sign(b);
  return Number.isNaN(d) ? x : x + d - 0.5;
}

/** 🗺️ Marching squares over a raster: one iso-contour per threshold. */
export function vizContours(values: readonly number[], size: readonly [number, number], thresholds?: number | readonly number[]): VizContour[] {
  const [dx, dy] = size;
  const finite = values.filter((value) => Number.isFinite(value));
  let tz: number[];
  if (Array.isArray(thresholds)) tz = [...(thresholds as readonly number[])].sort((a, b) => a - b);
  else {
    const count = typeof thresholds === "number" ? thresholds : thresholdSturges(finite.length);
    const lo = Math.min(...finite);
    const hi = Math.max(...finite);
    const [niceLo, niceHi] = niceDomain([lo, hi], count);
    tz = ticks(niceLo!, niceHi!, count);
    while (tz.length > 0 && tz[tz.length - 1]! >= hi) tz.pop();
    while (tz.length > 1 && tz[1]! < lo) tz.shift();
  }
  const above = (value: number | undefined, threshold: number): number => ((value ?? Number.NaN) >= threshold ? 1 : 0);
  return tz.map((value) => {
    const rings: VizPoint[][] = [];
    const fragmentByStart = new Map<number, { start: number; end: number; ring: VizPoint[] }>();
    const fragmentByEnd = new Map<number, { start: number; end: number; ring: VizPoint[] }>();
    const index = (point: VizPoint): number => point[0] * 2 + point[1] * (dx + 1) * 4;
    let x = -1;
    let y = -1;
    const stitch = (line: readonly [readonly [number, number], readonly [number, number]]): void => {
      const start: VizPoint = [line[0][0] + x, line[0][1] + y];
      const end: VizPoint = [line[1][0] + x, line[1][1] + y];
      const startIndex = index(start);
      const endIndex = index(end);
      const f = fragmentByEnd.get(startIndex);
      if (f !== undefined) {
        const g = fragmentByStart.get(endIndex);
        if (g !== undefined) {
          fragmentByEnd.delete(f.end);
          fragmentByStart.delete(g.start);
          if (f === g) {
            f.ring.push(end);
            rings.push(f.ring);
          } else {
            const merged = { start: f.start, end: g.end, ring: [...f.ring, ...g.ring] };
            fragmentByStart.set(merged.start, merged);
            fragmentByEnd.set(merged.end, merged);
          }
        } else {
          fragmentByEnd.delete(f.end);
          f.ring.push(end);
          f.end = endIndex;
          fragmentByEnd.set(endIndex, f);
        }
        return;
      }
      const g = fragmentByStart.get(endIndex);
      if (g !== undefined) {
        fragmentByStart.delete(g.start);
        g.ring.unshift(start);
        g.start = startIndex;
        fragmentByStart.set(startIndex, g);
        return;
      }
      const created = { start: startIndex, end: endIndex, ring: [start, end] };
      fragmentByStart.set(startIndex, created);
      fragmentByEnd.set(endIndex, created);
    };
    let t0: number;
    let t1 = above(values[0], value);
    let t2: number;
    let t3: number;
    for (const line of MARCHING_CASES[t1 << 1]!) stitch(line);
    while (++x < dx - 1) {
      t0 = t1;
      t1 = above(values[x + 1], value);
      for (const line of MARCHING_CASES[t0 | (t1 << 1)]!) stitch(line);
    }
    for (const line of MARCHING_CASES[t1]!) stitch(line);
    while (++y < dy - 1) {
      x = -1;
      t1 = above(values[y * dx + dx], value);
      t2 = above(values[y * dx], value);
      for (const line of MARCHING_CASES[(t1 << 1) | (t2 << 2)]!) stitch(line);
      while (++x < dx - 1) {
        t0 = t1;
        t1 = above(values[y * dx + dx + x + 1], value);
        t3 = t2;
        t2 = above(values[y * dx + x + 1], value);
        for (const line of MARCHING_CASES[t0 | (t1 << 1) | (t2 << 2) | (t3 << 3)]!) stitch(line);
      }
      for (const line of MARCHING_CASES[t1 | (t2 << 3)]!) stitch(line);
    }
    x = -1;
    t2 = above(values[y * dx], value);
    for (const line of MARCHING_CASES[t2 << 2]!) stitch(line);
    while (++x < dx - 1) {
      t3 = t2;
      t2 = above(values[y * dx + x + 1], value);
      for (const line of MARCHING_CASES[(t2 << 2) | (t3 << 3)]!) stitch(line);
    }
    for (const line of MARCHING_CASES[t2 << 3]!) stitch(line);
    const polygons: VizPoint[][][] = [];
    const holes: VizPoint[][] = [];
    for (const ring of rings) {
      for (const point of ring) {
        const px = point[0];
        const py = point[1];
        const xt = px | 0;
        const yt = py | 0;
        const v1 = values[yt * dx + xt] ?? Number.NaN;
        if (px > 0 && px < dx && xt === px) point[0] = smooth1(px, values[yt * dx + xt - 1] ?? Number.NaN, v1, value);
        if (py > 0 && py < dy && yt === py) point[1] = smooth1(py, values[(yt - 1) * dx + xt] ?? Number.NaN, v1, value);
      }
      if (ringArea(ring) > 0) polygons.push([ring]);
      else holes.push(ring);
    }
    for (const hole of holes) {
      for (const polygon of polygons) {
        if (polygonContains(polygon, hole)) {
          polygon.push(hole);
          break;
        }
      }
    }
    return { type: "MultiPolygon", value, coordinates: polygons };
  });
}
//#endregion 🔖️Contour

//#region 🔖️Density
/** 🌫️ A density raster: values on a `width × height` grid at the given cell size. */
export type VizDensityGrid = { readonly values: readonly number[]; readonly width: number; readonly height: number; readonly cellSize: number; readonly x0: number; readonly y0: number };

/** 🌫️ Two-dimensional kernel density estimate on a regular grid. */
export function vizDensity2d(points: readonly VizPoint[], options: { extent?: readonly [number, number, number, number]; cellSize?: number; bandwidth?: number; kernel?: VizKernel } = {}): VizDensityGrid {
  const xs = points.map((point) => point[0]);
  const ys = points.map((point) => point[1]);
  const [x0, y0, x1, y1] = options.extent ?? [Math.min(...xs), Math.min(...ys), Math.max(...xs), Math.max(...ys)];
  const cellSize = options.cellSize ?? Math.max((x1 - x0) / 40, 1e-9);
  const bandwidth = options.bandwidth ?? Math.max(silvermanBandwidth(xs), silvermanBandwidth(ys));
  const k = vizKernel(options.kernel ?? "gaussian");
  const width = Math.max(1, Math.ceil((x1 - x0) / cellSize) + 1);
  const height = Math.max(1, Math.ceil((y1 - y0) / cellSize) + 1);
  const values = new Array<number>(width * height).fill(0);
  const scale = 1 / (points.length * bandwidth * bandwidth);
  for (let j = 0; j < height; j += 1) {
    for (let i = 0; i < width; i += 1) {
      const gx = x0 + i * cellSize;
      const gy = y0 + j * cellSize;
      let total = 0;
      for (const point of points) total += k((gx - point[0]) / bandwidth) * k((gy - point[1]) / bandwidth);
      values[j * width + i] = total * scale;
    }
  }
  return { values, width, height, cellSize, x0, y0 };
}

/** 🌫️ Iso-contours of a density raster, in the raster's own coordinates. */
export function vizDensityContours(grid: VizDensityGrid, thresholds?: number | readonly number[]): VizContour[] {
  return vizContours(grid.values, [grid.width, grid.height], thresholds);
}

export { kernelDensity1d as vizKernelDensity1d };
//#endregion 🔖️Density
