/** 🧭️ The candidate lines of an orthographic view of a triangle soup: silhouette, crease and boundary edges of every body and the curves where two bodies interpenetrate. Which parts of them are visible is decided by `../📐️visibility`.
 *
 * Everything is in view space (`x` right, `y` up, `z` towards the viewer, counter-clockwise outward triangles). Vertices are welded per part on a grid of `1e-7` m, so the hard-normal duplicates of a tessellated crease meet.
 * - a **silhouette** edge joins a front-facing and a back-facing triangle of one part;
 * - a **crease** edge joins two front-facing triangles whose normals differ by more than the crease angle (30°, the angle of the model's feature edges);
 * - a **boundary** edge belongs to one front-facing triangle only (an open surface);
 * - a **join** is the intersection of a triangle of one part with a triangle of another, kept when at least one of them is front-facing; coplanar overlaps have no curve.
 */
import * as portable from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 🧵️ A candidate 3D line segment in view space and what it belongs to: `part` is the owner (for a join the first part) and `other` the second part of a join; `triangles` the triangles it lies in, which never hide it. */
export type Candidate = { readonly a: readonly [number, number, number]; readonly b: readonly [number, number, number]; readonly kind: "silhouette" | "crease" | "boundary" | "join"; readonly part: number; readonly other: number; readonly triangles: readonly number[] };

/** 📐️ The crease angle in radians above which two front-facing neighbours form an edge. */
export const CREASE_RAD = Math.PI / 6;

const WELD = 1e-7;

type Frame = { readonly vertices: Float64Array; readonly indices: Uint32Array; readonly part: Uint32Array };

const normalOf = (f: Frame, t: number): [number, number, number] => {
  const v = f.vertices;
  const a = f.indices[3 * t]! * 3, b = f.indices[3 * t + 1]! * 3, c = f.indices[3 * t + 2]! * 3;
  const ux = v[b]! - v[a]!, uy = v[b + 1]! - v[a + 1]!, uz = v[b + 2]! - v[a + 2]!, wx = v[c]! - v[a]!, wy = v[c + 1]! - v[a + 1]!, wz = v[c + 2]! - v[a + 2]!;
  return [uy * wz - uz * wy, uz * wx - ux * wz, ux * wy - uy * wx];
};

const pointOf = (f: Frame, vertex: number): [number, number, number] => [f.vertices[3 * vertex]!, f.vertices[3 * vertex + 1]!, f.vertices[3 * vertex + 2]!];

/** 🦄 The silhouette, crease and boundary edges of the soup, in order of triangle index. `include(part)` selects the parts that draw lines. */
export function edgesOf(f: Frame, include: (part: number) => boolean): Candidate[] {
  const n = f.indices.length / 3;
  const welded = new Map<string, number>();
  const canon = (part: number, vertex: number): number => {
    const k = `${part}|${Math.round(f.vertices[3 * vertex]! / WELD)}|${Math.round(f.vertices[3 * vertex + 1]! / WELD)}|${Math.round(f.vertices[3 * vertex + 2]! / WELD)}`;
    let id = welded.get(k);
    if (id === undefined) welded.set(k, (id = vertex));
    return id;
  };
  const normals = new Float64Array(3 * n);
  const front = new Uint8Array(n);
  const edges = new Map<string, { a: number; b: number; tris: number[] }>();
  for (let t = 0; t < n; t++) {
    const part = f.part[t]!;
    if (!include(part)) continue;
    const nrm = normalOf(f, t);
    const length = portable.hypot(...nrm);
    if (length < 1e-18) continue;
    normals.set(nrm, 3 * t);
    front[t] = nrm[2] > 1e-9 * length ? 1 : 0;
    const ids = [canon(part, f.indices[3 * t]!), canon(part, f.indices[3 * t + 1]!), canon(part, f.indices[3 * t + 2]!)];
    for (let e = 0; e < 3; e++) {
      const a = ids[e]!, b = ids[(e + 1) % 3]!;
      if (a === b) continue;
      const key = a < b ? `${a}|${b}` : `${b}|${a}`;
      const entry = edges.get(key);
      if (entry === undefined) edges.set(key, { a, b, tris: [t] });
      else if (entry.tris.length < 4) entry.tris.push(t);
    }
  }
  const out: Candidate[] = [];
  const cosCrease = portable.cos(CREASE_RAD);
  for (const { a, b, tris } of edges.values()) {
    const first = tris[0]!;
    const part = f.part[first]!;
    const make = (kind: Candidate["kind"]): void => void out.push({ a: pointOf(f, a), b: pointOf(f, b), kind, part, other: part, triangles: tris });
    if (tris.length === 1) {
      if (front[first] === 1) make("boundary");
      continue;
    }
    const second = tris[1]!;
    if (front[first] !== front[second]) make("silhouette");
    else if (front[first] === 1) {
      const n1 = normals.subarray(3 * first, 3 * first + 3), n2 = normals.subarray(3 * second, 3 * second + 3);
      const cos = (n1[0]! * n2[0]! + n1[1]! * n2[1]! + n1[2]! * n2[2]!) / (portable.hypot(n1[0]!, n1[1]!, n1[2]!) * portable.hypot(n2[0]!, n2[1]!, n2[2]!));
      if (cos < cosCrease) make("crease");
    }
  }
  out.sort((p, q) => p.triangles[0]! - q.triangles[0]! || p.a[0] - q.a[0] || p.a[1] - q.a[1] || p.a[2] - q.a[2]);
  return out;
}

type Box = { readonly min: [number, number, number]; readonly max: [number, number, number] };

/** 🔀️ The interpenetration curves of different parts as segments in view space, at least one of the two triangles front-facing. `include(part)` selects the parts that draw lines. */
export function joinsOf(f: Frame, include: (part: number) => boolean, partCount: number): Candidate[] {
  const n = f.indices.length / 3;
  const boxes: Box[] = Array.from({ length: partCount }, () => ({ min: [Infinity, Infinity, Infinity], max: [-Infinity, -Infinity, -Infinity] }));
  const triangles: number[][] = Array.from({ length: partCount }, () => []);
  const tbox = new Float64Array(6 * n);
  const front = new Uint8Array(n);
  for (let t = 0; t < n; t++) {
    const part = f.part[t]!;
    if (!include(part)) continue;
    triangles[part]!.push(t);
    const nrm = normalOf(f, t);
    front[t] = nrm[2] > 1e-9 * portable.hypot(...nrm) ? 1 : 0;
    for (let k = 0; k < 3; k++) {
      const p = pointOf(f, f.indices[3 * t + k]!);
      for (let d = 0; d < 3; d++) {
        const lo = k === 0 ? Infinity : tbox[6 * t + d]!;
        const hi = k === 0 ? -Infinity : tbox[6 * t + 3 + d]!;
        tbox[6 * t + d] = Math.min(lo, p[d]!);
        tbox[6 * t + 3 + d] = Math.max(hi, p[d]!);
        boxes[part]!.min[d] = Math.min(boxes[part]!.min[d]!, p[d]!);
        boxes[part]!.max[d] = Math.max(boxes[part]!.max[d]!, p[d]!);
      }
    }
  }
  const out: Candidate[] = [];
  const eps = 1e-9;
  for (let i = 0; i < partCount; i++) {
    if (triangles[i]!.length === 0) continue;
    for (let j = i + 1; j < partCount; j++) {
      if (triangles[j]!.length === 0) continue;
      const bi = boxes[i]!, bj = boxes[j]!;
      let overlap = true;
      const lo: number[] = [], hi: number[] = [];
      for (let d = 0; d < 3; d++) {
        lo.push(Math.max(bi.min[d]!, bj.min[d]!) - eps);
        hi.push(Math.min(bi.max[d]!, bj.max[d]!) + eps);
        if (lo[d]! > hi[d]!) overlap = false;
      }
      if (!overlap) continue;
      const inside = (t: number): boolean => tbox[6 * t]! <= hi[0]! && tbox[6 * t + 3]! >= lo[0]! && tbox[6 * t + 1]! <= hi[1]! && tbox[6 * t + 4]! >= lo[1]! && tbox[6 * t + 2]! <= hi[2]! && tbox[6 * t + 5]! >= lo[2]!;
      const as = triangles[i]!.filter(inside);
      const bs = triangles[j]!.filter(inside).sort((p, q) => tbox[6 * p]! - tbox[6 * q]! || p - q);
      for (const a of as) {
        let from = 0, to = bs.length;
        while (from < to) {
          const mid = (from + to) >> 1;
          if (tbox[6 * bs[mid]!]! < tbox[6 * a + 3]! + eps) from = mid + 1;
          else to = mid;
        }
        for (let k = 0; k < from; k++) {
          const b = bs[k]!;
          if (tbox[6 * b + 3]! < tbox[6 * a]! - eps) continue;
          if (tbox[6 * b + 1]! > tbox[6 * a + 4]! + eps || tbox[6 * b + 4]! < tbox[6 * a + 1]! - eps || tbox[6 * b + 2]! > tbox[6 * a + 5]! + eps || tbox[6 * b + 5]! < tbox[6 * a + 2]! - eps) continue;
          if (front[a] === 0 && front[b] === 0) continue;
          const segment = intersect(f, a, b);
          if (segment !== null) out.push({ a: segment[0], b: segment[1], kind: "join", part: i, other: j, triangles: [a, b] });
        }
      }
    }
  }
  out.sort((p, q) => p.part - q.part || p.other - q.other || p.triangles[0]! - q.triangles[0]! || p.triangles[1]! - q.triangles[1]!);
  return out;
}

type V = [number, number, number];

const sub = (a: V, b: V): V => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const dot = (a: V, b: V): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const cross = (a: V, b: V): V => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];

/** ✂️ The segment where triangle `b` crosses the plane of triangle `a`, clipped to `a`; `null` when they do not cross in a segment (disjoint, touching in a point, coplanar). */
function intersect(f: Frame, a: number, b: number): [V, V] | null {
  const A = [0, 1, 2].map((k) => pointOf(f, f.indices[3 * a + k]!)) as [V, V, V];
  const B = [0, 1, 2].map((k) => pointOf(f, f.indices[3 * b + k]!)) as [V, V, V];
  const n = cross(sub(A[1], A[0]), sub(A[2], A[0]));
  const length = portable.hypot(...n);
  if (length < 1e-18) return null;
  const eps = 1e-9;
  const d = B.map((p) => dot(n, sub(p, A[0])) / length);
  if (d.every((x) => Math.abs(x) <= eps) || d.every((x) => x > eps) || d.every((x) => x < -eps)) return null;
  const points: V[] = [];
  for (let k = 0; k < 3; k++) {
    const p = B[k]!, q = B[(k + 1) % 3]!, dp = d[k]!, dq = d[(k + 1) % 3]!;
    if (Math.abs(dp) <= eps) points.push(p);
    if ((dp > eps && dq < -eps) || (dp < -eps && dq > eps)) {
      const t = dp / (dp - dq);
      points.push([p[0] + (q[0] - p[0]) * t, p[1] + (q[1] - p[1]) * t, p[2] + (q[2] - p[2]) * t]);
    }
  }
  if (points.length < 2) return null;
  let p = points[0]!, q = points[1]!;
  let best = -1;
  for (let i = 0; i < points.length; i++) for (let j = i + 1; j < points.length; j++) {
    const len = portable.hypot(...sub(points[i]!, points[j]!));
    if (len > best) {
      best = len;
      p = points[i]!;
      q = points[j]!;
    }
  }
  let t0 = 0, t1 = 1;
  const dir = sub(q, p);
  for (let k = 0; k < 3 && t0 < t1; k++) {
    const m = cross(n, sub(A[(k + 1) % 3]!, A[k]!));
    const scale = portable.hypot(...m) || 1;
    const v0 = dot(m, sub(p, A[k]!)) / scale;
    const rate = dot(m, dir) / scale;
    if (Math.abs(rate) < 1e-18) {
      if (v0 < -1e-12) return null;
      continue;
    }
    const t = -v0 / rate;
    if (rate > 0) t0 = Math.max(t0, t);
    else t1 = Math.min(t1, t);
  }
  if (!(t1 - t0 > 1e-9)) return null;
  return [[p[0] + dir[0] * t0, p[1] + dir[1] * t0, p[2] + dir[2] * t0], [p[0] + dir[0] * t1, p[1] + dir[1] * t1, p[2] + dir[2] * t1]];
}
