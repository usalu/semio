/** 📐️ Clips projected mesh edges and classifies their spans against opaque faces. */

export type ClipPoint = readonly [number, number, number, number];
export type Point = readonly [number, number, number];
export type Edge = readonly [Point, Point];
type Triangle = readonly [Point, Point, Point];
type Interval = [number, number];
type Bounds = readonly [number, number, number, number];

const DEPTH_TOLERANCE = 4 * 2 ** -23;
const PARAMETER_TOLERANCE = 1e-12;
const TILE_LEVELS = 6;
const FACES_PER_STEP = 1024;

function planeDistance(point: ClipPoint, plane: number): number {
  return point[3] + (plane % 2 === 0 ? 1 : -1) * point[Math.floor(plane / 2)]!;
}

function interpolate(a: ClipPoint, b: ClipPoint, t: number): ClipPoint {
  return [a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1]), a[2] + t * (b[2] - a[2]), a[3] + t * (b[3] - a[3])];
}

export function clipEdge([a, b]: readonly [ClipPoint, ClipPoint]): [ClipPoint, ClipPoint] | undefined {
  for (let plane = 0; plane < 6; plane++) {
    const da = planeDistance(a, plane), db = planeDistance(b, plane);
    if (da < 0 && db < 0) return undefined;
    if (da < 0) a = interpolate(a, b, da / (da - db));
    else if (db < 0) b = interpolate(a, b, da / (da - db));
  }
  return [a, b];
}

export function clipTriangle(points: readonly [ClipPoint, ClipPoint, ClipPoint]): ClipPoint[] {
  let polygon = [...points];
  for (let plane = 0; plane < 6 && polygon.length > 0; plane++) {
    const output: ClipPoint[] = [];
    for (let index = 0; index < polygon.length; index++) {
      const a = polygon[index]!, b = polygon[(index + 1) % polygon.length]!;
      const da = planeDistance(a, plane), db = planeDistance(b, plane);
      if (da >= 0) output.push(a);
      if ((da < 0 && db > 0) || (da > 0 && db < 0)) output.push(interpolate(a, b, da / (da - db)));
    }
    polygon = output;
  }
  return polygon;
}

export function projectPoint(point: ClipPoint, width: number, height: number): Point {
  return [(point[0] / point[3]) * width * 0.5, (-point[1] / point[3]) * height * 0.5, point[2] / point[3]];
}

export function signedArea(a: Point, b: Point, point: Point): number {
  return (b[0] - a[0]) * (point[1] - a[1]) - (b[1] - a[1]) * (point[0] - a[0]);
}

function intersectPositive(interval: Interval, a: number, b: number): boolean {
  if (a < 0 && b < 0) return false;
  if (a < 0) interval[0] = Math.max(interval[0], a / (a - b));
  if (b < 0) interval[1] = Math.min(interval[1], a / (a - b));
  return interval[1] - interval[0] > PARAMETER_TOLERANCE;
}

function hiddenInterval(edge: Edge, [a, b, c]: Triangle, area: number): Interval | undefined {
  const weights = ([p, q]: Edge, u: Point, v: Point) => [signedArea(u, v, p) / area, signedArea(u, v, q) / area] as const;
  const wa = weights(edge, b, c), wb = weights(edge, c, a), wc = weights(edge, a, b);
  const interval: Interval = [0, 1];
  if (!intersectPositive(interval, ...wa) || !intersectPositive(interval, ...wb) || !intersectPositive(interval, ...wc)) return undefined;
  const near = (i: 0 | 1) => edge[i][2] - (wa[i] * a[2] + wb[i] * b[2] + wc[i] * c[2]);
  const start = near(0), end = near(1);
  const maxGap = Math.max(...interval.map((t) => start + t * (end - start)));
  return maxGap > DEPTH_TOLERANCE && intersectPositive(interval, start, end) ? interval : undefined;
}

function bounds(points: readonly Point[]): Bounds {
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  for (const [x, y] of points) {
    minX = Math.min(minX, x); minY = Math.min(minY, y);
    maxX = Math.max(maxX, x); maxY = Math.max(maxY, y);
  }
  return [minX, minY, maxX, maxY];
}

function overlaps(a: Bounds, b: Bounds): boolean {
  return a[0] <= b[2] && b[0] <= a[2] && a[1] <= b[3] && b[1] <= a[3];
}

function contains(a: Bounds, b: Bounds): boolean {
  return a[0] <= b[0] && a[1] <= b[1] && b[2] <= a[2] && b[3] <= a[3];
}

function tiles([minX, minY, maxX, maxY]: Bounds): Bounds[] {
  const midX = (minX + maxX) * 0.5, midY = (minY + maxY) * 0.5;
  return [[minX, minY, midX, midY], [midX, minY, maxX, midY], [minX, midY, midX, maxY], [midX, midY, maxX, maxY]];
}

export class Occluders {
  private readonly buckets = new Map<number, { points: Triangle; box: Bounds; area: number }[]>();
  private readonly frame: Bounds;

  constructor(width: number, height: number) {
    this.frame = [-width * 0.5, -height * 0.5, width * 0.5, height * 0.5];
  }

  add(points: Triangle) {
    const area = signedArea(...points);
    if (!Number.isFinite(area) || area === 0) return;
    const box = bounds(points);
    let tile = this.frame, node = 0;
    for (let level = 0; level < TILE_LEVELS; level++) {
      const children = tiles(tile);
      const child = children.findIndex((candidate) => contains(candidate, box));
      if (child < 0) break;
      if (!this.buckets.has(node)) this.buckets.set(node, []);
      node = node * 4 + child + 1;
      tile = children[child]!;
    }
    const bucket = this.buckets.get(node) ?? [];
    bucket.push({ points, box, area });
    this.buckets.set(node, bucket);
  }

  *classify(edge: Edge): Generator<void, { range: Interval; hidden: boolean }[]> {
    const box = bounds(edge);
    const events = new Map([[1, 0]]);
    let work = 0;
    const visit: { node: number; tile: Bounds }[] = [{ node: 0, tile: this.frame }];
    while (visit.length > 0) {
      const { node, tile } = visit.pop()!;
      const bucket = this.buckets.get(node);
      if (!bucket) continue;
      for (const face of bucket) {
        if (++work % FACES_PER_STEP === 0) yield;
        if (!overlaps(box, face.box)) continue;
        const interval = hiddenInterval(edge, face.points, face.area);
        if (!interval) continue;
        const [start, end] = interval;
        events.set(start, (events.get(start) ?? 0) + 1);
        events.set(end, (events.get(end) ?? 0) - 1);
      }
      for (const [index, child] of tiles(tile).entries()) {
        const childNode = node * 4 + index + 1;
        if (this.buckets.has(childNode) && overlaps(box, child)) visit.push({ node: childNode, tile: child });
      }
    }
    const spans: { range: Interval; hidden: boolean }[] = [];
    let start = 0, covering = 0;
    for (const [end, delta] of [...events].sort(([a], [b]) => a - b)) {
      const hidden = covering > 0;
      covering += delta;
      if (end !== 1 && hidden === (covering > 0)) continue;
      if (end - start > PARAMETER_TOLERANCE) spans.push({ range: [start, end], hidden });
      start = end;
    }
    return spans;
  }
}
