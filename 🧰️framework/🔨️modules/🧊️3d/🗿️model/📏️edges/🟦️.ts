/** 📏️ Feature edges of resolved meshes: boundary and non-manifold edges plus edges whose adjacent face normals differ by more than the angle, found on position-welded vertices so smooth seams never show.
 * One `lines` primitive per source triangle primitive is appended to its mesh, with the source material unless a material (or a function of the source primitive that names one) is given, and with joints and weights copied for skinned sources.
 * A source primitive with `solids` gives its lines `solids` too: an edge belongs to the solid of the first triangle that uses it (an edge shared by two solids is kept once, under the earlier one); solids without edges get no range.
 * @see ../🧬️schema/🟦️.ts */
import type { Mesh, ModelDocument, Operation, Primitive, PrimitiveSolid, Slug } from "../🧬️schema/🟦️.ts";
import { cross, dot, length, normalize, sub, type V3 } from "../🧮️math/🟦️.ts";
import { Pace, silentOperation } from "../⏱️operation/🟦️.ts";
import * as portable from "../🧮️math/🎯️portable/🟦️.ts";

const QUANTUM = 1e8;

/** 🔆️ Adds feature `lines` primitives; the input is not modified. Rejects with `CancelledError` when cancelled: checked between meshes, between primitives and every few thousand vertices, triangles and edges inside one primitive. */
export async function featureEdges(document: ModelDocument, angleRad: number, options: { readonly material?: Slug | ((source: Primitive) => Slug) } = {}, operation: Operation = silentOperation): Promise<ModelDocument> {
  const pace = new Pace(operation);
  const meshes: Mesh[] = [];
  let clock = Date.now();
  for (let m = 0; m < document.meshes.length; m++) {
    pace.step(m / document.meshes.length);
    const mesh = document.meshes[m]!;
    const lines: Primitive[] = [];
    for (const primitive of mesh.primitives) {
      if (Date.now() - clock > 16) {
        await new Promise<void>((resolve) => setTimeout(resolve, 0));
        clock = Date.now();
      }
      pace.check();
      if (primitive.mode !== "triangles") continue;
      const line = linesOf(primitive, angleRad, options.material, pace);
      if (line) lines.push(line);
    }
    meshes.push(lines.length ? { ...mesh, primitives: [...mesh.primitives, ...lines] } : mesh);
  }
  pace.step(1);
  return { ...document, meshes };
}

function linesOf(primitive: Primitive, angleRad: number, material: Slug | ((source: Primitive) => Slug) | undefined, pace: Pace): Primitive | null {
  const p = primitive.positions;
  const welded = new Map<string, number>();
  const group: number[] = [];
  for (let i = 0; i < p.length / 3; i++) {
    pace.tick();
    const key = `${Math.round(p[i * 3]! * QUANTUM)},${Math.round(p[i * 3 + 1]! * QUANTUM)},${Math.round(p[i * 3 + 2]! * QUANTUM)}`;
    if (!welded.has(key)) welded.set(key, i);
    group.push(welded.get(key)!);
  }
  const at = (i: number): V3 => [p[i * 3]!, p[i * 3 + 1]!, p[i * 3 + 2]!];
  const edges = new Map<string, { a: number; b: number; normals: V3[]; owner: number }>();
  const ranges = primitive.solids ?? [];
  let range = 0;
  for (let t = 0; t < primitive.indices.length; t += 3) {
    pace.tick();
    const [a, b, c] = [primitive.indices[t]!, primitive.indices[t + 1]!, primitive.indices[t + 2]!];
    const n = cross(sub(at(b), at(a)), sub(at(c), at(a)));
    if (length(n) < 1e-18) continue;
    const unit = normalize(n);
    while (range < ranges.length - 1 && t >= ranges[range]!.first + ranges[range]!.count) range++;
    for (const [u, v] of [[a, b], [b, c], [c, a]] as const) {
      const gu = group[u]!, gv = group[v]!;
      if (gu === gv) continue;
      const key = gu < gv ? `${gu}:${gv}` : `${gv}:${gu}`;
      const edge = edges.get(key) ?? { a: gu, b: gv, normals: [], owner: range };
      edges.set(key, edge);
      edge.normals.push(unit);
    }
  }
  const positions: number[] = [], indices: number[] = [], joints: number[] = [], weights: number[] = [], solids: PrimitiveSolid[] = [];
  const cos = portable.cos(angleRad);
  for (const e of edges.values()) {
    pace.tick();
    const sharp = e.normals.length !== 2 || dot(e.normals[0]!, e.normals[1]!) < cos - 1e-12;
    if (!sharp) continue;
    if (ranges.length > 0) {
      const id = ranges[e.owner]!.id, last = solids[solids.length - 1];
      if (last && last.id === id) solids[solids.length - 1] = { id, first: last.first, count: last.count + 2 };
      else solids.push({ id, first: indices.length, count: 2 });
    }
    for (const v of [e.a, e.b]) {
      indices.push(positions.length / 3);
      positions.push(p[v * 3]!, p[v * 3 + 1]!, p[v * 3 + 2]!);
      if (primitive.joints && primitive.weights) {
        joints.push(...primitive.joints.slice(v * 4, v * 4 + 4));
        weights.push(...primitive.weights.slice(v * 4, v * 4 + 4));
      }
    }
  }
  if (indices.length === 0) return null;
  return { mode: "lines", material: typeof material === "function" ? material(primitive) : (material ?? primitive.material), positions, indices, ...(primitive.joints ? { joints, weights } : {}), ...(ranges.length > 0 ? { solids } : {}) };
}
