/** 🪨️ Checks one connected outward triangular shell, including intersections between faces. */
import { meshIntegrity } from "../../🩻️integrity/🟦️.ts";
import { cross, dot, sub, type V3 } from "../../🧮️math/🟦️.ts";
import type { Pace } from "../../⏱️operation/🟦️.ts";

type Face = readonly [number, number, number];
const epsilon = 1e-10;
const axes = [0, 1, 2] as const;

function edges(points: V3[]): V3[] {
  return points.map((p, i) => sub(points[(i + 1) % 3]!, p));
}

function separated(pa: V3[], pb: V3[], axis: V3): boolean {
  const length = Math.hypot(...axis);
  if (length < 1e-16) return false;
  const a = pa.map((p) => dot(p, axis)), b = pb.map((p) => dot(p, axis));
  return Math.max(...a) < Math.min(...b) - epsilon * length || Math.max(...b) < Math.min(...a) - epsilon * length;
}

function side(a: V3, b: V3, p: V3, normal: V3): number {
  return dot(cross(sub(b, a), sub(p, a)), normal);
}

function crosses(a: Face, b: Face, points: readonly V3[]): boolean {
  const shared = a.filter((i) => b.includes(i));
  const pa = a.map((i) => points[i]!), pb = b.map((i) => points[i]!);
  const ea = edges(pa), eb = edges(pb);
  const na = cross(ea[0]!, ea[1]!), nb = cross(eb[0]!, eb[1]!);
  const lengthA = Math.hypot(...na), lengthB = Math.hypot(...nb);
  const coplanar = Math.hypot(...cross(na, nb)) <= epsilon * lengthA * lengthB && Math.abs(dot(na, sub(pb[0]!, pa[0]!))) <= epsilon * lengthA;
  if (shared.length === 2) return coplanar && dot(na, nb) < 0;
  if (shared.length === 0) {
    const directions = [na, nb, ...ea.flatMap((x) => eb.map((y) => cross(x, y)))];
    if (coplanar) directions.push(...ea.map((x) => cross(x, na)), ...eb.map((x) => cross(x, nb)));
    return !directions.some((axis) => separated(pa, pb, axis));
  }

  const origin = points[shared[0]!]!;
  function oppositeEdgeHits(ids: Face, face: V3[], normal: V3): boolean {
    const ends = ids.filter((i) => i !== shared[0]).map((i) => points[i]!);
    const p = ends[0]!, q = ends[1]!;
    const h = dot(sub(p, origin), normal), k = dot(sub(q, origin), normal);
    const inside = (v: V3) => face.every((x, i) => side(x, face[(i + 1) % 3]!, v, normal) >= -epsilon * dot(normal, normal));
    if (coplanar) {
      if (inside(p) || inside(q)) return true;
      return face.some((x, i) => {
        const y = face[(i + 1) % 3]!;
        return side(p, q, x, normal) * side(p, q, y, normal) < 0 && side(x, y, p, normal) * side(x, y, q, normal) < 0;
      });
    }
    if (h * k > 0 || Math.abs(h - k) < 1e-20) return false;
    const t = h / (h - k);
    return inside([p[0] + (q[0] - p[0]) * t, p[1] + (q[1] - p[1]) * t, p[2] + (q[2] - p[2]) * t]);
  }
  return oppositeEdgeHits(a, pb, nb) || oppositeEdgeHits(b, pa, na);
}

/** 🧭 Returns the invalid field, or null for a consistently oriented embedded boundary. */
export function polyhedronFault(vertices: unknown, triangles: unknown, pace?: Pace): "vertices" | "triangles" | null {
  if (!Array.isArray(vertices) || vertices.length < 4 || !vertices.every((p) => Array.isArray(p) && p.length === 3 && p.every(Number.isFinite))) return "vertices";
  if (!Array.isArray(triangles) || triangles.length < 4 || !triangles.every((t) => Array.isArray(t) && t.length === 3 && t.every((i) => Number.isInteger(i) && i >= 0 && i < vertices.length))) return "triangles";
  const verts = vertices as V3[], faces = triangles as Face[];
  if (new Set(faces.flat()).size !== verts.length) return "vertices";
  const low = [Infinity, Infinity, Infinity], high = [-Infinity, -Infinity, -Infinity];
  for (const p of verts) for (const i of axes) {
    low[i] = Math.min(low[i]!, p[i]);
    high[i] = Math.max(high[i]!, p[i]);
  }
  const span = Math.max(...axes.map((i) => high[i]! - low[i]!));
  if (!(span > 0)) return "vertices";
  const points = verts.map((p) => axes.map((i) => (p[i] - low[i]!) / span) as V3);
  const integrity = meshIntegrity(points.flat(), faces.flat(), { toleranceM: 1e-11 });
  if (integrity.vertices !== verts.length || integrity.degenerate || !integrity.closed || !integrity.manifold || !integrity.oriented || integrity.components !== 1 || !(integrity.signedVolume > 0)) return "triangles";
  const bounds = faces.map((ids) => {
    const p = ids.map((i) => points[i]!);
    return { ids, min: axes.map((i) => Math.min(...p.map((v) => v[i]))), max: axes.map((i) => Math.max(...p.map((v) => v[i]))) };
  }).sort((a, b) => a.min[0]! - b.min[0]!);
  for (let i = 0; i < bounds.length; i++) {
    const a = bounds[i]!;
    for (let j = i + 1; j < bounds.length; j++) {
      pace?.tick();
      const b = bounds[j]!;
      if (b.min[0]! > a.max[0]! + epsilon) break;
      if (b.min[1]! > a.max[1]! + epsilon || a.min[1]! > b.max[1]! + epsilon || b.min[2]! > a.max[2]! + epsilon || a.min[2]! > b.max[2]! + epsilon) continue;
      if (crosses(a.ids, b.ids, points)) return "triangles";
    }
  }
  return null;
}
