/** 🏗️ Extrusion of a polygon with holes between two plane coordinates with optional taper: flat caps, side walls smooth across profile corners flatter than 35 degrees and hard otherwise.
 * Taper scales the profile about the area centroid of the outer ring.
 * @see ../🔺️polygon/🟦️.ts */
import type { ExtrudeSolid, Operation, Quality, Vec2 } from "../../🧬️schema/🟦️.ts";
import { add, cross, dot, mul, normalize, sub, type V3 } from "../../🧮️math/🟦️.ts";
import { Pace, silentOperation } from "../../⏱️operation/🟦️.ts";
import { CREASE_COS, MeshBuilder, type Geometry } from "../🧱️builder/🟦️.ts";
import { cleanRing, signedArea, triangulatePolygon } from "../🔺️polygon/🟦️.ts";

const centroid = (ring: readonly Vec2[]): Vec2 => {
  let a = 0, cx = 0, cy = 0;
  for (let i = 0; i < ring.length; i++) {
    const p = ring[i]!, q = ring[(i + 1) % ring.length]!;
    const w = p[0] * q[1] - q[0] * p[1];
    a += w;
    cx += (p[0] + q[0]) * w;
    cy += (p[1] + q[1]) * w;
  }
  return a === 0 ? ring[0]! : [cx / (3 * a), cy / (3 * a)];
};

export function tessellateExtrude(solid: ExtrudeSolid, _quality: Quality, operation: Operation = silentOperation): Geometry {
  let outer = cleanRing(solid.profile);
  if (signedArea(outer) < 0) outer.reverse();
  const holes = (solid.holes ?? []).map((h) => {
    const ring = cleanRing(h);
    if (signedArea(ring) > 0) ring.reverse();
    return ring;
  });
  const lo = Math.min(solid.from, solid.to), hi = Math.max(solid.from, solid.to);
  const [s0, s1] = solid.taperTo ?? [1, 1];
  const sLo = solid.from <= solid.to ? s0 : s1, sHi = solid.from <= solid.to ? s1 : s0;
  const c = centroid(outer);
  const map = (u: number, v: number, w: number): V3 => (solid.plane === "xy" ? [u, v, w] : solid.plane === "yz" ? [w, u, v] : [v, w, u]);
  const place = (p: Vec2, scale: number, w: number): V3 => map(c[0] + scale * (p[0] - c[0]), c[1] + scale * (p[1] - c[1]), w);
  const pace = new Pace(operation);
  const builder = new MeshBuilder(pace);
  for (const ring of [outer, ...holes]) {
    const n = ring.length;
    const faces: V3[] = [];
    for (let k = 0; k < n; k++) {
      const a = place(ring[k]!, sLo, lo), b = place(ring[(k + 1) % n]!, sLo, lo), t = place(ring[(k + 1) % n]!, sHi, hi);
      faces.push(normalize(cross(sub(b, a), sub(t, b)), map(0, 0, 1)));
    }
    const vertexNormal = (face: V3, other: V3): V3 => (dot(face, other) >= CREASE_COS ? normalize(add(face, other)) : face);
    for (let k = 0; k < n; k++) {
      const next = (k + 1) % n, prev = (k + n - 1) % n;
      const f = faces[k]!;
      const nStart = vertexNormal(f, faces[prev]!), nEnd = vertexNormal(f, faces[next]!);
      const a0 = builder.vertex(place(ring[k]!, sLo, lo), nStart), b0 = builder.vertex(place(ring[next]!, sLo, lo), nEnd);
      const a1 = builder.vertex(place(ring[k]!, sHi, hi), nStart), b1 = builder.vertex(place(ring[next]!, sHi, hi), nEnd);
      builder.triangle(a0, b0, b1);
      builder.triangle(a0, b1, a1);
    }
  }
  const all = [...outer, ...holes.flat()];
  const triangles = triangulatePolygon(outer, holes, pace);
  for (const [w, scale, up] of [[hi, sHi, 1], [lo, sLo, -1]] as const) {
    if (scale <= 1e-9) continue;
    const normal = mul(map(0, 0, 1), up);
    const ids = all.map((p) => builder.vertex(place(p, scale, w), normal));
    for (let t = 0; t < triangles.length; t += 3) {
      const [i, j, l] = [triangles[t]!, triangles[t + 1]!, triangles[t + 2]!];
      if (up > 0) builder.triangle(ids[i]!, ids[j]!, ids[l]!);
      else builder.triangle(ids[i]!, ids[l]!, ids[j]!);
    }
  }
  return builder.build();
}
