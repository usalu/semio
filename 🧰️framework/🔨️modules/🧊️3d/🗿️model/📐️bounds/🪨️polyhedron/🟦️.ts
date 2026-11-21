/** 📐️ Three physical spans along the principal directions of a vertex cloud. */
import type { Vec3 } from "../../🧬️schema/🟦️.ts";
import { cross, dot, sub, type V3 } from "../../🧮️math/🟦️.ts";

const unit = (p: V3): V3 => {
  const length = Math.hypot(...p);
  return length === 0 ? [0, 0, 0] : [p[0] / length, p[1] / length, p[2] / length];
};

/** 🧭 Measures an indexed solid in its own principal frame, including when its vertices already encode a rotation. */
export function polyhedronExtents(vertices: readonly Vec3[], first: readonly number[] | undefined): V3 {
  if (!first || !vertices[first[0]!] || !vertices[first[1]!] || !vertices[first[2]!]) return [0, 0, 0];
  const origin = vertices[first[0]!]!;
  const x = unit(sub(vertices[first[1]!]!, origin));
  const z = unit(cross(x, sub(vertices[first[2]!]!, origin)));
  const frame = [x, cross(z, x), z];
  const points = vertices.map((vertex) => {
    const offset = sub(vertex, origin);
    return frame.map((axis) => dot(axis, offset));
  });
  const mean = [0, 1, 2].map((i) => points.reduce((sum, p) => sum + p[i]!, 0) / points.length);
  const covariance = mean.map((_, i) => mean.map((__, j) => points.reduce((sum, p) => sum + (p[i]! - mean[i]!) * (p[j]! - mean[j]!), 0)));
  const axes = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
  const threshold = 1e-14 * (covariance[0]![0]! + covariance[1]![1]! + covariance[2]![2]!);
  for (let sweep = 0; sweep < 10; sweep++) {
    let changed = false;
    for (const [a, b] of [[0, 1], [0, 2], [1, 2]] as const) {
      const off = covariance[a]![b]!;
      if (Math.abs(off) <= threshold) continue;
      changed = true;
      const delta = (covariance[b]![b]! - covariance[a]![a]!) / (2 * off);
      const tangent = (delta < 0 ? -1 : 1) / (Math.abs(delta) + Math.hypot(1, delta));
      const cosine = 1 / Math.hypot(1, tangent), sine = tangent * cosine;
      covariance[a]![a]! -= tangent * off;
      covariance[b]![b]! += tangent * off;
      covariance[a]![b] = covariance[b]![a] = 0;
      const k = 3 - a - b;
      const u = covariance[k]![a]!, v = covariance[k]![b]!;
      covariance[a]![k] = covariance[k]![a] = cosine * u - sine * v;
      covariance[b]![k] = covariance[k]![b] = sine * u + cosine * v;
      for (let i = 0; i < 3; i++) {
        const u = axes[a]![i]!, v = axes[b]![i]!;
        axes[a]![i] = cosine * u - sine * v;
        axes[b]![i] = sine * u + cosine * v;
      }
    }
    if (!changed) break;
  }
  return axes.map((axis) => {
    let min = Infinity, max = -Infinity;
    for (const point of points) {
      const d = dot(axis, point);
      min = Math.min(min, d);
      max = Math.max(max, d);
    }
    return max - min;
  }).sort((a, b) => a - b) as V3;
}

