/** 🧷️ Vertices that differ only in a normal nobody reads are one vertex: inside every solid of a primitive whose materials are all unlit, equal positions share an index. The triangles, the lines, their order and the solid ranges stay as they are; only the vertex table and the indices into it shrink. */
import type { Mesh, Primitive } from "../../../🧬️schema/🟦️.ts";
import type { Pace } from "../../../⏱️operation/🟦️.ts";

type Span = { id: string; lo: number; hi: number };
type Use = { primitives: Primitive[]; ok: boolean };

function spansOf(primitive: Primitive, vertices: number, pace: Pace): Span[] | null {
  const spans: Span[] = [];
  let first = 0;
  for (const range of primitive.solids!) {
    let lo = vertices, hi = 0;
    for (let i = first; i < first + range.count; i++) {
      pace.tick();
      const v = primitive.indices[i]!;
      lo = Math.min(lo, v);
      hi = Math.max(hi, v + 1);
    }
    if (!(hi > lo)) return null;
    spans.push({ id: range.id, lo, hi });
    first += range.count;
  }
  return spans;
}

/** 🧷️ Welds the vertices of the primitives for which `reads` is false for every triangle primitive that shares their vertex table and that carry neither colours nor skin; every other primitive is returned as it was. */
export function weldUnread(meshes: readonly Mesh[], reads: (primitive: Primitive) => boolean, pace: Pace): readonly Mesh[] {
  const uses = new Map<readonly number[], Use>();
  for (const mesh of meshes) for (const p of mesh.primitives) {
    const use = uses.get(p.positions) ?? { primitives: [], ok: true };
    uses.set(p.positions, use);
    use.primitives.push(p);
    if (p.colors || p.joints || p.weights || !p.solids?.length || p.positions.length % 3 !== 0 || (p.mode === "triangles" && reads(p))) use.ok = false;
  }
  const replaced = new Map<Primitive, Primitive>();
  for (const [positions, use] of uses) {
    if (!use.ok) continue;
    const vertices = positions.length / 3;
    const triangles = use.primitives.filter((p) => p.mode === "triangles");
    if (triangles.length === 0) continue;
    const spans: Span[] = [];
    for (const p of triangles) {
      const own = spansOf(p, vertices, pace);
      if (own === null) { spans.length = 0; break; }
      spans.push(...own);
    }
    if (spans.length === 0) continue;
    spans.sort((a, b) => a.lo - b.lo);
    let cursor = 0;
    for (const span of spans) {
      if (span.lo !== cursor) { cursor = -1; break; }
      cursor = span.hi;
    }
    if (cursor !== vertices) continue;
    const byId = new Map(spans.map((s) => [s.id, s]));
    const contained = use.primitives.every((p) => {
      if (p.mode === "triangles") return true;
      let first = 0;
      for (const range of p.solids!) {
        const span = byId.get(range.id);
        if (span === undefined) return false;
        for (let i = first; i < first + range.count; i++) {
          pace.tick();
          const v = p.indices[i]!;
          if (v < span.lo || v >= span.hi) return false;
        }
        first += range.count;
      }
      return true;
    });
    if (!contained) continue;
    const map = new Int32Array(vertices);
    const welded: number[] = [];
    for (const span of spans) {
      const seen = new Map<string, number>();
      for (let v = span.lo; v < span.hi; v++) {
        pace.tick();
        const x = Math.fround(positions[3 * v]!), y = Math.fround(positions[3 * v + 1]!), z = Math.fround(positions[3 * v + 2]!);
        const key = `${x},${y},${z}`;
        let index = seen.get(key);
        if (index === undefined) {
          index = welded.length / 3;
          welded.push(positions[3 * v]!, positions[3 * v + 1]!, positions[3 * v + 2]!);
          seen.set(key, index);
        }
        map[v] = index;
      }
    }
    if (welded.length === positions.length) continue;
    for (const p of use.primitives) {
      const { normals: _normals, ...rest } = p;
      replaced.set(p, { ...rest, positions: welded, indices: p.indices.map((v) => { pace.tick(); return map[v]!; }) });
    }
  }
  if (replaced.size === 0) return meshes;
  return meshes.map((mesh) => ({ ...mesh, primitives: mesh.primitives.map((p) => replaced.get(p) ?? p) }));
}
