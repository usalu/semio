/** 🧱️ Rigid solids with large matching vertex payloads can share GLB accessors across materials and model scenes. */
import type { Mesh, Primitive, PrimitiveSolid } from "../../../🧬️schema/🟦️.ts";
import type { Pace } from "../../../⏱️operation/🟦️.ts";

type Span = { lo: number; hi: number };
type Part = Span & { range: PrimitiveSolid; keys: string[] };
type Group = Span & { parts: Part[] };
const MIN_VERTEX_BYTES = 4096;

function partsOf(p: Primitive, pace: Pace): Part[] {
  const vertices = p.positions.length / 3;
  if (!p.solids?.length || p.joints || p.weights || !Number.isInteger(vertices) || (p.colors && p.colors.length !== vertices * 4) || (p.normals && p.normals.length !== p.positions.length)) return [];
  const parts: Part[] = [];
  const arity = p.mode === "triangles" ? 3 : 2;
  let first = 0;
  for (const range of p.solids) {
    if (range.first !== first || range.count < arity || range.count % arity || first + range.count > p.indices.length) return [];
    let lo = vertices, hi = 0;
    for (let i = first; i < first + range.count; i++) {
      pace.tick();
      const v = p.indices[i]!;
      if (!Number.isInteger(v) || v < 0 || v >= vertices) return [];
      lo = Math.min(lo, v);
      hi = Math.max(hi, v + 1);
    }
    parts.push({ range, lo, hi, keys: [] });
    first += range.count;
  }
  return first === p.indices.length ? parts : [];
}

function groupOf(parts: Part[]): Group {
  let lo = Infinity, hi = 0;
  for (const span of parts) {
    lo = Math.min(lo, span.lo);
    hi = Math.max(hi, span.hi);
  }
  return { parts, lo, hi };
}

function split(p: Primitive, group: Group, pace: Pace): Primitive {
  const { lo, hi, parts } = group;
  const first = parts[0]!.range.first, last = parts[parts.length - 1]!.range;
  const indices = p.indices.slice(first, last.first + last.count).map((v) => { pace.tick(); return v - lo; });
  return {
    ...p, positions: p.positions.slice(3 * lo, 3 * hi), indices,
    ...(p.normals && { normals: p.normals.slice(3 * lo, 3 * hi) }), ...(p.colors && { colors: p.colors.slice(4 * lo, 4 * hi) }),
    solids: parts.map(({ range }) => ({ ...range, first: range.first - first })),
  };
}

/** ♊️ Retains the original order of every index, solid, material and mesh. A hash nominates a candidate partition; BufferBuilder verifies every encoded component before sharing an accessor. Feature lines can use the matching triangle span, including its unused boundary vertices. */
export function partitionRigidParts(meshes: readonly Mesh[], pace: Pace): readonly Mesh[] {
  if (!meshes.some((mesh) => mesh.primitives.some((p) => p.mode === "triangles" && !p.joints && !p.weights && (p.solids?.length ?? 0) > 1 && p.positions.length * 4 >= MIN_VERTEX_BYTES))) return meshes;
  const rows = new Map<Primitive, Part[]>();
  const owners = new Map<string, string>();
  const repeated = new Set<string>();
  const parentHashes = new Map<readonly number[], number>();
  const float = new Float32Array(1), bits = new Uint32Array(float.buffer);
  const fingerprint = (values: readonly number[], start: number, end: number): number => {
    let hash = 2166136261;
    for (let i = start; i < end; i++) {
      pace.tick();
      if (!Number.isFinite(values[i])) throw new Error(`non-finite vertex attribute at ${i}`);
      float[0] = values[i]!;
      hash = Math.imul(hash ^ bits[0]!, 16777619);
    }
    return hash;
  };
  for (const mesh of meshes) for (const p of mesh.primitives) {
    if (p.mode !== "triangles" || p.positions.length * 4 < MIN_VERTEX_BYTES) continue;
    const parts = partsOf(p, pace);
    for (const part of parts) {
      if ((part.hi - part.lo) * 12 < MIN_VERTEX_BYTES) continue;
      for (const [name, values] of [["p", p.positions], ["n", p.normals]] as const) {
        if (!values) continue;
        const hash = fingerprint(values, 3 * part.lo, 3 * part.hi);
        const key = `${name}/${part.hi - part.lo}/${hash}`;
        let parent = parentHashes.get(values);
        if (parent === undefined) {
          parent = fingerprint(values, 0, values.length);
          parentHashes.set(values, parent);
        }
        const location = `${parent}/${values.length}/${part.lo}`;
        const first = owners.get(key);
        if (first === undefined) owners.set(key, location);
        else if (first !== location) repeated.add(key);
        part.keys.push(key);
      }
    }
    if (parts.length > 1) rows.set(p, parts);
  }
  const partitions = new Map<Primitive, Group[]>();
  const lineOwners = new Map<readonly number[], Map<string, Group>>();
  for (const [p, parts] of rows) {
    const groups: Group[] = [];
    let other: Part[] = [];
    for (const part of parts) {
      if (part.keys.some((key) => repeated.has(key))) {
        if (other.length) groups.push(groupOf(other));
        other = [];
        groups.push(groupOf([part]));
      } else other.push(part);
    }
    if (other.length) groups.push(groupOf(other));
    if (groups.length < 2) continue;
    if (p.colors) fingerprint(p.colors, 0, p.colors.length);
    partitions.set(p, groups);
    const bounds = lineOwners.get(p.positions) ?? new Map<string, Group>();
    for (const group of groups) for (const part of group.parts) bounds.set(part.range.id, group);
    lineOwners.set(p.positions, bounds);
  }
  if (!partitions.size) return meshes;
  return meshes.map((mesh) => ({
    ...mesh, primitives: mesh.primitives.flatMap((p) => {
      const triangles = partitions.get(p);
      if (triangles) return triangles.map((group) => split(p, group, pace));
      const owners = lineOwners.get(p.positions);
      if (!owners || p.mode !== "lines") return [p];
      const parts = partsOf(p, pace);
      if (!parts.length || parts.some((part) => !owners.has(part.range.id))) return [p];
      const groups: Group[] = [];
      let previous: Group | undefined;
      for (const part of parts) {
        const owner = owners.get(part.range.id)!;
        if (owner !== previous) groups.push({ lo: owner.lo, hi: owner.hi, parts: [] });
        const group = groups[groups.length - 1]!;
        group.lo = Math.min(group.lo, part.lo);
        group.hi = Math.max(group.hi, part.hi);
        group.parts.push(part);
        previous = owner;
      }
      return groups.map((group) => split(p, group, pace));
    })
  }));
}
