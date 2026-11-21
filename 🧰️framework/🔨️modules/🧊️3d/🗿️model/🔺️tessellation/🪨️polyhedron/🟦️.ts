/** 🪨️ Indexed botanical and sculptural surfaces retain their authored facets at every quality. */
import type { Operation, PolyhedronSolid } from "../../🧬️schema/🟦️.ts";
import { cross, normalize, sub } from "../../🧮️math/🟦️.ts";
import { Pace, silentOperation } from "../../⏱️operation/🟦️.ts";
import type { Geometry } from "../🧱️builder/🟦️.ts";

/** 🔺️ Smooth area-weighted vertex normals over the solid's outward triangular shell. */
export function tessellatePolyhedron(s: PolyhedronSolid, operation: Operation = silentOperation): Geometry {
  const pace = new Pace(operation);
  const normals = new Array<number>(s.vertices.length * 3).fill(0);
  const indices: number[] = [];
  for (const [a, b, c] of s.triangles) {
    pace.tick();
    const n = cross(sub(s.vertices[b]!, s.vertices[a]!), sub(s.vertices[c]!, s.vertices[a]!));
    for (const i of [a, b, c]) for (let j = 0; j < 3; j++) normals[i * 3 + j] = normals[i * 3 + j]! + n[j]!;
    indices.push(a, b, c);
  }
  const positions: number[] = [];
  for (let i = 0; i < s.vertices.length; i++) {
    pace.tick();
    positions.push(...s.vertices[i]!);
    const j = i * 3;
    const x = normals[j]!, y = normals[j + 1]!, z = normals[j + 2]!;
    const scale = Math.max(Math.abs(x), Math.abs(y), Math.abs(z));
    if (scale > 0) [normals[j], normals[j + 1], normals[j + 2]] = normalize([x / scale, y / scale, z / scale]);
  }
  return { positions, normals, indices };
}
