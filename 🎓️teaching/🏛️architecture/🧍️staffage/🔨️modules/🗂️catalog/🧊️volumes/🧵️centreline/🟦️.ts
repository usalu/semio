/** 🧵️ The centre line of a slender solid, curved or closed as it may be (a hose, a hoop, a ring): found from the triangles of its own mesh, so that slenderness is the length of the path over its largest cross extent and the silhouette comparisons draw the solid like the stroke it follows (amendment C24 refined).
 *
 * Every sample of the surface (the triangles subdivided to a spacing of a fortieth of the extent, and every vertex with the normal of its largest face) shoots a ray inward and then shrinks the ball that touches the surface there (Ma et al.) until it holds no other part of the surface: its centre is a point of the medial axis and its radius the half thickness there. In every cell of a grid of the cross extent the points whose radius is close to the largest one are averaged, the cells are joined by their minimum spanning tree and the tree edges that jump over a gap are dropped. A point is kept as coordinates in the frame of three vertices of the mesh, so that every copy of a `repeat` finds its own centre line from its own vertices.
 *
 * @see ../🌓️silhouette/🟦️.ts
 * @see ../🧱️rules/🟦️.ts */
export type Vec3 = readonly [number, number, number];

/** 🧵️ The centre line of one mesh: the `anchor` vertices whose frame carries the points, the `clusters` (centres as coordinates in that frame), the tree `edges` between them, the largest `thickness` across and the `length` of the path in metres. */
export type CentreLine = { readonly anchor: readonly [number, number, number]; readonly clusters: readonly Vec3[]; readonly edges: readonly (readonly [number, number])[]; readonly thickness: number; readonly length: number };

const CENTRE_LINES = new WeakMap<object, CentreLine | null>();
const MAX_SAMPLES = 1500;
const MAX_WORK = 4e7;
const MAX_DIVISIONS = 24;
const SPACING = 40;
const MAX_CLUSTERS = 400;
const SMOOTHING = 3;
const STRAIGHT = 3;
const KEEP_RADIUS = 0.8;

const sub = (a: Vec3, b: Vec3): Vec3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const dot = (a: Vec3, b: Vec3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const cross = (a: Vec3, b: Vec3): Vec3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const length3 = (a: Vec3): number => Math.hypot(a[0], a[1], a[2]);
const unit = (a: Vec3): Vec3 => {
  const l = length3(a) || 1;
  return [a[0] / l, a[1] / l, a[2] / l];
};

function vertex(p: ArrayLike<number>, base: number, v: number): Vec3 {
  const i = (base + v) * 3;
  return [p[i]!, p[i + 1]!, p[i + 2]!];
}

/** 🧭️ The rigid frame of three vertices of a copy: origin and the orthonormal axes. */
function frameOf(p: ArrayLike<number>, base: number, anchor: readonly [number, number, number]): { readonly o: Vec3; readonly e: readonly [Vec3, Vec3, Vec3] } {
  const [o, a, b] = anchor.map((v) => vertex(p, base, v)) as [Vec3, Vec3, Vec3];
  const e1 = unit(sub(a, o));
  const e3 = unit(cross(e1, sub(b, o)));
  return { o, e: [e1, cross(e3, e1), e3] };
}

/** 🎯️ The closest point of a triangle to a point (Ericson). */
function closest(p: Vec3, a: Vec3, b: Vec3, c: Vec3): Vec3 {
  const [ab, ac, ap] = [sub(b, a), sub(c, a), sub(p, a)];
  const [d1, d2] = [dot(ab, ap), dot(ac, ap)];
  if (d1 <= 0 && d2 <= 0) return a;
  const bp = sub(p, b);
  const [d3, d4] = [dot(ab, bp), dot(ac, bp)];
  if (d3 >= 0 && d4 <= d3) return b;
  const vc = d1 * d4 - d3 * d2;
  if (vc <= 0 && d1 >= 0 && d3 <= 0) {
    const v = d1 / (d1 - d3);
    return [a[0] + ab[0] * v, a[1] + ab[1] * v, a[2] + ab[2] * v];
  }
  const cp = sub(p, c);
  const [d5, d6] = [dot(ab, cp), dot(ac, cp)];
  if (d6 >= 0 && d5 <= d6) return c;
  const vb = d5 * d2 - d1 * d6;
  if (vb <= 0 && d2 >= 0 && d6 <= 0) {
    const w = d2 / (d2 - d6);
    return [a[0] + ac[0] * w, a[1] + ac[1] * w, a[2] + ac[2] * w];
  }
  const va = d3 * d6 - d5 * d4;
  if (va <= 0 && d4 - d3 >= 0 && d5 - d6 >= 0) {
    const w = (d4 - d3) / (d4 - d3 + (d5 - d6));
    return [b[0] + (c[0] - b[0]) * w, b[1] + (c[1] - b[1]) * w, b[2] + (c[2] - b[2]) * w];
  }
  const denominator = 1 / (va + vb + vc);
  const [v, w] = [vb * denominator, vc * denominator];
  return [a[0] + ab[0] * v + ac[0] * w, a[1] + ab[1] * v + ac[1] * w, a[2] + ab[2] * v + ac[2] * w];
}

/** 🎯️ The first hit of the ray `origin + t * direction` (t > `minimum`) on the triangles other than `skip`, and the number of hits; Moeller-Trumbore. */
function cast(p: ArrayLike<number>, indices: ArrayLike<number>, origin: Vec3, direction: Vec3, skip: number, minimum: number): { readonly hits: number; readonly t: number } {
  let [hits, best] = [0, Infinity];
  for (let f = 0; f < indices.length / 3; f++) {
    if (f === skip) continue;
    const [i, j, k] = [indices[f * 3]! * 3, indices[f * 3 + 1]! * 3, indices[f * 3 + 2]! * 3];
    const [e1x, e1y, e1z, e2x, e2y, e2z] = [p[j]! - p[i]!, p[j + 1]! - p[i + 1]!, p[j + 2]! - p[i + 2]!, p[k]! - p[i]!, p[k + 1]! - p[i + 1]!, p[k + 2]! - p[i + 2]!];
    const [px, py, pz] = [direction[1] * e2z - direction[2] * e2y, direction[2] * e2x - direction[0] * e2z, direction[0] * e2y - direction[1] * e2x];
    const det = e1x * px + e1y * py + e1z * pz;
    if (Math.abs(det) < 1e-18) continue;
    const [tx, ty, tz] = [origin[0] - p[i]!, origin[1] - p[i + 1]!, origin[2] - p[i + 2]!];
    const u = (tx * px + ty * py + tz * pz) / det;
    if (u < 0 || u > 1) continue;
    const [qx, qy, qz] = [ty * e1z - tz * e1y, tz * e1x - tx * e1z, tx * e1y - ty * e1x];
    const v = (direction[0] * qx + direction[1] * qy + direction[2] * qz) / det;
    if (v < 0 || u + v > 1) continue;
    const t = (e2x * qx + e2y * qy + e2z * qz) / det;
    if (t <= minimum) continue;
    hits++;
    if (t < best) best = t;
  }
  return { hits, t: best };
}

/** 🎈️ The radius of the largest ball that touches the surface at `s` (inward direction `d`) and holds no other part of it: starting from `start`, the ball shrinks to the part of the surface inside it that is met first along `d`, until nothing lies inside. */
function shrink(p: ArrayLike<number>, indices: ArrayLike<number>, s: Vec3, d: Vec3, start: number, size: number): number {
  let r = start;
  for (let iteration = 0; iteration < 8; iteration++) {
    const c: Vec3 = [s[0] + d[0] * r, s[1] + d[1] * r, s[2] + d[2] * r];
    let depth = r * (1 - 1e-3) - 1e-9 * size;
    let hit: Vec3 | undefined;
    for (let f = 0; f < indices.length / 3; f++) {
      const q = closest(c, vertex(p, 0, indices[f * 3]!), vertex(p, 0, indices[f * 3 + 1]!), vertex(p, 0, indices[f * 3 + 2]!));
      const distance = length3(sub(q, c));
      if (distance < depth) [depth, hit] = [distance, q];
    }
    if (hit === undefined) return r;
    const v = sub(hit, s);
    const along = dot(v, d);
    if (!(along > 1e-12)) return Math.max(r * 0.5, 1e-9);
    r = Math.min(r, dot(v, v) / (2 * along));
  }
  return r;
}

/** 🧵️ The centre line of the closed triangle mesh `positions`/`indices` of one copy (`key` identifies the mesh), or `undefined` when no inside direction is found (an open skin, a degenerate mesh); memoised per `key`. */
export function centreLineOf(key: object, positions: Float64Array, indices: ArrayLike<number>): CentreLine | undefined {
  const known = CENTRE_LINES.get(key);
  if (known !== undefined) return known ?? undefined;
  const found = build(positions, indices);
  CENTRE_LINES.set(key, found ?? null);
  return found;
}

function build(p: Float64Array, indices: ArrayLike<number>): CentreLine | undefined {
  const triangles = indices.length / 3;
  const count = p.length / 3;
  if (triangles < 4 || count < 4) return undefined;
  const lo: [number, number, number] = [Infinity, Infinity, Infinity];
  const hi: [number, number, number] = [-Infinity, -Infinity, -Infinity];
  for (let v = 0; v < p.length; v += 3) for (let k = 0; k < 3; k++) {
    if (p[v + k]! < lo[k]!) lo[k] = p[v + k]!;
    if (p[v + k]! > hi[k]!) hi[k] = p[v + k]!;
  }
  const size = Math.hypot(hi[0] - lo[0], hi[1] - lo[1], hi[2] - lo[2]);
  if (!(size > 0)) return undefined;
  const origin = vertex(p, 0, 0);
  let far = 1;
  for (let v = 1; v < count; v++) if (length3(sub(vertex(p, 0, v), origin)) > length3(sub(vertex(p, 0, far), origin))) far = v;
  let third = -1;
  let widest = 1e-12 * size * size;
  for (let v = 1; v < count; v++) {
    const a = length3(cross(sub(vertex(p, 0, far), origin), sub(vertex(p, 0, v), origin)));
    if (a > widest) [widest, third] = [a, v];
  }
  if (third < 0) return undefined;
  const anchor: [number, number, number] = [0, far, third];
  const [normal, area] = [new Float64Array(triangles * 3), new Float64Array(triangles)];
  const best = new Map<number, number>();
  for (let f = 0; f < triangles; f++) {
    const [a, b, c] = [vertex(p, 0, indices[f * 3]!), vertex(p, 0, indices[f * 3 + 1]!), vertex(p, 0, indices[f * 3 + 2]!)];
    const n = cross(sub(b, a), sub(c, a));
    const l = length3(n);
    area[f] = l / 2;
    if (l > 0) [normal[f * 3], normal[f * 3 + 1], normal[f * 3 + 2]] = [n[0] / l, n[1] / l, n[2] / l];
    for (let k = 0; k < 3; k++) {
      const held = best.get(indices[f * 3 + k]!);
      if (held === undefined || area[held]! < area[f]!) best.set(indices[f * 3 + k]!, f);
    }
  }
  const samples: { readonly point: Vec3; readonly f: number }[] = [];
  for (let f = 0; f < triangles; f++) {
    if (!(area[f]! > 0)) continue;
    const [a, b, c] = [vertex(p, 0, indices[f * 3]!), vertex(p, 0, indices[f * 3 + 1]!), vertex(p, 0, indices[f * 3 + 2]!)];
    const longest = Math.max(length3(sub(b, a)), length3(sub(c, b)), length3(sub(a, c)));
    const n = Math.min(MAX_DIVISIONS, Math.max(1, Math.ceil(longest / (size / SPACING))));
    const at = (u: number, w: number): Vec3 => [a[0] + (b[0] - a[0]) * u + (c[0] - a[0]) * w, a[1] + (b[1] - a[1]) * u + (c[1] - a[1]) * w, a[2] + (b[2] - a[2]) * u + (c[2] - a[2]) * w];
    for (let i = 0; i < n; i++)
      for (let j = 0; i + j < n; j++) {
        samples.push({ point: at((i + 1 / 3) / n, (j + 1 / 3) / n), f });
        if (i + j + 1 < n) samples.push({ point: at((i + 2 / 3) / n, (j + 2 / 3) / n), f });
      }
  }
  for (const [v, f] of best) samples.push({ point: vertex(p, 0, v), f });
  const stride = Math.max(1, Math.ceil(samples.length / MAX_SAMPLES), Math.ceil((samples.length * triangles * 6) / MAX_WORK));
  const balls: { point: Vec3; radius: number }[] = [];
  for (let n = 0; n < samples.length; n += stride) {
    const { point: s, f } = samples[n]!;
    const d: Vec3 = [normal[f * 3]!, normal[f * 3 + 1]!, normal[f * 3 + 2]!];
    const back: Vec3 = [-d[0], -d[1], -d[2]];
    const [up, down] = [cast(p, indices, s, d, f, 1e-9 * size), cast(p, indices, s, back, f, 1e-9 * size)];
    const inward = up.hits % 2 === 1 && down.hits % 2 === 0 ? { direction: d, t: up.t } : down.hits % 2 === 1 && up.hits % 2 === 0 ? { direction: back, t: down.t } : undefined;
    if (inward === undefined || !(inward.t < Infinity)) continue;
    const radius = shrink(p, indices, s, inward.direction, inward.t / 2, size);
    balls.push({ point: [s[0] + inward.direction[0] * radius, s[1] + inward.direction[1] * radius, s[2] + inward.direction[2] * radius], radius });
  }
  if (balls.length < 4) return undefined;
  const lateral = balls.filter((b) => b.radius > 0);
  if (lateral.length < 2) return undefined;
  const top = Math.max(...lateral.map((b) => b.radius));
  const thickness = 2 * top;
  const mean = [0, 1, 2].map((k) => lateral.reduce((t, b) => t + b.point[k]!, 0) / lateral.length) as unknown as Vec3;
  let axis: Vec3 = unit(sub(lateral.reduce((a, b) => (length3(sub(b.point, mean)) > length3(sub(a, mean)) ? b.point : a), lateral[0]!.point), mean));
  for (let n = 0; n < 24; n++) {
    const next = lateral.reduce<Vec3>((t, b) => {
      const [r, w] = [sub(b.point, mean), dot(sub(b.point, mean), axis)];
      return [t[0] + r[0] * w, t[1] + r[1] * w, t[2] + r[2] * w];
    }, [0, 0, 0]);
    if (length3(next) === 0) break;
    axis = unit(next);
  }
  const along = lateral.map((b) => dot(sub(b.point, mean), axis));
  const off = lateral.map((b, i) => length3(sub(sub(b.point, mean), [axis[0] * along[i]!, axis[1] * along[i]!, axis[2] * along[i]!])));
  if (Math.max(...off) <= STRAIGHT * thickness) {
    const [lowest, highest] = [Math.min(...along), Math.max(...along)];
    const ends = [lowest, highest].map((t) => [mean[0] + axis[0] * t, mean[1] + axis[1] * t, mean[2] + axis[2] * t] as unknown as Vec3);
    const frame = frameOf(p, 0, anchor);
    const local = ends.map((c) => {
      const r = sub(c, frame.o);
      return [dot(r, frame.e[0]), dot(r, frame.e[1]), dot(r, frame.e[2])] as unknown as Vec3;
    });
    return { anchor, clusters: local, edges: [[0, 1]], thickness, length: highest - lowest };
  }
  let cell = Math.max(2 * top, size / 20);
  let groups: { point: Vec3; radius: number }[][] = [];
  for (;;) {
    const grid = new Map<string, { point: Vec3; radius: number }[]>();
    for (const b of lateral) {
      const key = `${Math.floor(b.point[0] / cell)},${Math.floor(b.point[1] / cell)},${Math.floor(b.point[2] / cell)}`;
      const list = grid.get(key);
      if (list === undefined) grid.set(key, [b]);
      else list.push(b);
    }
    groups = [...grid.values()];
    if (groups.length <= MAX_CLUSTERS) break;
    cell *= 1.5;
  }
  const centres = groups.map((members) => {
    const largest = Math.max(...members.map((b) => b.radius));
    const near = members.filter((b) => b.radius >= KEEP_RADIUS * largest);
    return [0, 1, 2].map((k) => near.reduce((t, b) => t + b.point[k]!, 0) / near.length) as unknown as Vec3;
  });
  const reach = 4 * cell;
  const joined = new Uint8Array(centres.length);
  const near = new Float64Array(centres.length).fill(Infinity);
  const from = new Int32Array(centres.length).fill(-1);
  const edges: [number, number][] = [];
  let length = 0;
  for (let start = 0; start < centres.length; start++) {
    if (joined[start]) continue;
    near[start] = 0;
    for (;;) {
      let pick = -1;
      for (let n = 0; n < centres.length; n++) if (!joined[n] && near[n]! < Infinity && (pick < 0 || near[n]! < near[pick]!)) pick = n;
      if (pick < 0) break;
      joined[pick] = 1;
      if (from[pick]! >= 0) {
        edges.push([from[pick]!, pick]);
        length += near[pick]!;
      }
      for (let n = 0; n < centres.length; n++) {
        if (joined[n]) continue;
        const d = length3(sub(centres[n]!, centres[pick]!));
        if (d <= reach && d < near[n]!) [near[n], from[n]] = [d, pick];
      }
    }
  }
  const neighbours = centres.map(() => [] as number[]);
  for (const [i, j] of edges) {
    neighbours[i]!.push(j);
    neighbours[j]!.push(i);
  }
  let smooth = centres;
  for (let pass = 0; pass < SMOOTHING; pass++)
    smooth = smooth.map((c, i) => {
      const list = neighbours[i]!;
      if (list.length !== 2) return c;
      return [0, 1, 2].map((k) => 0.5 * c[k]! + 0.25 * (smooth[list[0]!]![k]! + smooth[list[1]!]![k]!)) as unknown as Vec3;
    });
  length = edges.reduce((t, [i, j]) => t + length3(sub(smooth[i]!, smooth[j]!)), 0);
  const frame = frameOf(p, 0, anchor);
  const local = smooth.map((c) => {
    const r = sub(c, frame.o);
    return [dot(r, frame.e[0]), dot(r, frame.e[1]), dot(r, frame.e[2])] as unknown as Vec3;
  });
  return { anchor, clusters: local, edges, thickness, length };
}

/** 📍️ The cluster centres of the copy of a solid that starts at vertex `base` of the world mesh, in world coordinates. */
export function clusterPoints(line: CentreLine, positions: ArrayLike<number>, base: number): readonly Vec3[] {
  const f = frameOf(positions, base, line.anchor);
  return line.clusters.map((c) => [0, 1, 2].map((k) => f.o[k]! + c[0] * f.e[0][k]! + c[1] * f.e[1][k]! + c[2] * f.e[2][k]!) as unknown as Vec3);
}
