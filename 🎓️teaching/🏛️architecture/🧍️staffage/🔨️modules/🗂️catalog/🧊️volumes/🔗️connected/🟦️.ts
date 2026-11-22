/** 🔗️ Which solids of a list touch: the exact smallest distance between the triangles of two posed meshes, the groups of parts that touch within a tolerance and what each loose group rests on.
 *
 * A part is one copy of a solid (a `repeat` gives one part per copy, so a row of slats is not connected by being one entry). Two parts touch when their surfaces come within the tolerance of each other (triangle against triangle, exact: vertex to face, edge to edge and edge through face) or when one lies inside the other (a vertex inside the closed mesh). No bounding box decides anything; boxes only discard pairs that cannot touch.
 *
 * THE REFERENCES are trimesh (`proximity.closest_point`, `contains`) and scipy (`sparse.csgraph.connected_components`) in `🧪️tests/🧊️volumes-connected/🐍️.py`.
 *
 * @see ../🟦️.ts — `checkVolumes`, rule `volumes-connected`
 * @see ../🧮️measures/🟦️.ts — `insideAny`, `SEAM_FEATURES`
 * @see https://en.wikipedia.org/wiki/Disjoint-set_data_structure */
import { insideAny } from "../🧮️measures/🟦️.ts";
import type { World } from "../🌓️silhouette/🟦️.ts";

/** 🧩️ One copy of a solid as a mesh of its own: `solid` is the index in the list, `copy` the index of the `repeat` copy. */
export type Part = { readonly solid: number; readonly copy: number; readonly id: string; readonly positions: Float64Array; readonly indices: Uint32Array; readonly min: readonly [number, number, number]; readonly max: readonly [number, number, number] };

/** 🧮️ The groups of touching parts: `group[k]` is the group of part k (0 is the group with the largest summed box volume; equal volumes go to the group that reaches lowest, then to the smallest solid id, so the order of the list never matters), `count` the number of groups. */
export type Groups = { readonly group: Int32Array; readonly count: number };

type Cells = { readonly cell: number; readonly n: readonly [number, number, number]; readonly start: Int32Array; readonly items: Int32Array; readonly stamp: Int32Array; mark: number };
const CELLS = new WeakMap<Float64Array, Cells>();

/** 🧩️ The parts of posed solids: `copies[k]` is the `repeat` count of solid k, whose mesh holds that many equal runs of vertices and triangles. */
export function partsOf(worlds: readonly World[], copies: readonly number[] = []): readonly Part[] {
  const out: Part[] = [];
  worlds.forEach((w, solid) => {
    const n = Math.max(1, copies[solid] ?? 1);
    const [pv, pi] = [w.positions.length / n, w.indices.length / n];
    const whole = n === 1 || !Number.isInteger(pv / 3) || !Number.isInteger(pi / 3);
    for (let copy = 0; copy < (whole ? 1 : n); copy++) {
      const positions = whole ? w.positions : w.positions.subarray(copy * pv, (copy + 1) * pv);
      const indices = whole ? w.indices : w.indices.subarray(0, pi);
      const min: [number, number, number] = [Infinity, Infinity, Infinity];
      const max: [number, number, number] = [-Infinity, -Infinity, -Infinity];
      for (let i = 0; i < positions.length; i += 3) for (let k = 0; k < 3; k++) {
        const v = positions[i + k]!;
        if (v < min[k]!) min[k] = v;
        if (v > max[k]!) max[k] = v;
      }
      if (indices.length >= 3) out.push({ solid, copy, id: w.id, positions, indices, min, max });
    }
  });
  return out;
}

function cellsOf(part: Part): Cells {
  let hit = CELLS.get(part.positions);
  if (hit !== undefined) return hit;
  const { positions: q, indices, min, max } = part;
  const triangles = indices.length / 3;
  const size = [max[0] - min[0], max[1] - min[1], max[2] - min[2]] as const;
  const long = Math.max(size[0], size[1], size[2], 1e-9);
  const cell = Math.max(long / 48, Math.cbrt((Math.max(size[0], long / 48) * Math.max(size[1], long / 48) * Math.max(size[2], long / 48)) / Math.max(1, triangles / 2)));
  const n = [Math.max(1, Math.ceil(size[0] / cell)), Math.max(1, Math.ceil(size[1] / cell)), Math.max(1, Math.ceil(size[2] / cell))] as const;
  const at = (v: number, k: number): number => Math.max(0, Math.min(n[k]! - 1, Math.floor((v - min[k]!) / cell)));
  const count = new Int32Array(n[0] * n[1] * n[2] + 1);
  const range = (t: number): number[] => {
    const r: number[] = [];
    for (let k = 0; k < 3; k++) {
      const [a, b, c] = [q[indices[t]! * 3 + k]!, q[indices[t + 1]! * 3 + k]!, q[indices[t + 2]! * 3 + k]!];
      r.push(at(Math.min(a, b, c), k), at(Math.max(a, b, c), k));
    }
    return r;
  };
  for (let t = 0; t < indices.length; t += 3) {
    const r = range(t);
    for (let z = r[4]!; z <= r[5]!; z++) for (let y = r[2]!; y <= r[3]!; y++) for (let x = r[0]!; x <= r[1]!; x++) count[(z * n[1] + y) * n[0] + x + 1]!++;
  }
  for (let i = 1; i < count.length; i++) count[i]! += count[i - 1]!;
  const items = new Int32Array(count[count.length - 1]!);
  const fill = count.slice(0, -1);
  for (let t = 0; t < indices.length; t += 3) {
    const r = range(t);
    for (let z = r[4]!; z <= r[5]!; z++) for (let y = r[2]!; y <= r[3]!; y++) for (let x = r[0]!; x <= r[1]!; x++) items[fill[(z * n[1] + y) * n[0] + x]!++] = t;
  }
  hit = { cell, n, start: count, items, stamp: new Int32Array(triangles), mark: 0 };
  CELLS.set(part.positions, hit);
  return hit;
}

const P = new Float64Array(18);

function segments2(a: number, b: number, c: number, d: number): number {
  const [d1x, d1y, d1z] = [P[b]! - P[a]!, P[b + 1]! - P[a + 1]!, P[b + 2]! - P[a + 2]!];
  const [d2x, d2y, d2z] = [P[d]! - P[c]!, P[d + 1]! - P[c + 1]!, P[d + 2]! - P[c + 2]!];
  const [rx, ry, rz] = [P[a]! - P[c]!, P[a + 1]! - P[c + 1]!, P[a + 2]! - P[c + 2]!];
  const [aa, e, f] = [d1x * d1x + d1y * d1y + d1z * d1z, d2x * d2x + d2y * d2y + d2z * d2z, d2x * rx + d2y * ry + d2z * rz];
  let [s, t] = [0, 0];
  if (aa <= 1e-24 && e <= 1e-24) return rx * rx + ry * ry + rz * rz;
  if (aa <= 1e-24) t = Math.max(0, Math.min(1, f / e));
  else {
    const c1 = d1x * rx + d1y * ry + d1z * rz;
    if (e <= 1e-24) s = Math.max(0, Math.min(1, -c1 / aa));
    else {
      const bb = d1x * d2x + d1y * d2y + d1z * d2z;
      const denominator = aa * e - bb * bb;
      s = denominator > 1e-24 * aa * e ? Math.max(0, Math.min(1, (bb * f - c1 * e) / denominator)) : 0;
      t = (bb * s + f) / e;
      if (t < 0) [t, s] = [0, Math.max(0, Math.min(1, -c1 / aa))];
      else if (t > 1) [t, s] = [1, Math.max(0, Math.min(1, (bb - c1) / aa))];
    }
  }
  const [x, y, z] = [rx + d1x * s - d2x * t, ry + d1y * s - d2y * t, rz + d1z * s - d2z * t];
  return x * x + y * y + z * z;
}

function pointTriangle2(p: number, a: number, b: number, c: number): number {
  const [abx, aby, abz] = [P[b]! - P[a]!, P[b + 1]! - P[a + 1]!, P[b + 2]! - P[a + 2]!];
  const [acx, acy, acz] = [P[c]! - P[a]!, P[c + 1]! - P[a + 1]!, P[c + 2]! - P[a + 2]!];
  const [apx, apy, apz] = [P[p]! - P[a]!, P[p + 1]! - P[a + 1]!, P[p + 2]! - P[a + 2]!];
  const [d1, d2] = [abx * apx + aby * apy + abz * apz, acx * apx + acy * apy + acz * apz];
  const norm = (x: number, y: number, z: number): number => x * x + y * y + z * z;
  if (d1 <= 0 && d2 <= 0) return norm(apx, apy, apz);
  const [bpx, bpy, bpz] = [P[p]! - P[b]!, P[p + 1]! - P[b + 1]!, P[p + 2]! - P[b + 2]!];
  const [d3, d4] = [abx * bpx + aby * bpy + abz * bpz, acx * bpx + acy * bpy + acz * bpz];
  if (d3 >= 0 && d4 <= d3) return norm(bpx, bpy, bpz);
  const vc = d1 * d4 - d3 * d2;
  if (vc <= 0 && d1 >= 0 && d3 <= 0) {
    const v = d1 / (d1 - d3 || 1);
    return norm(apx - v * abx, apy - v * aby, apz - v * abz);
  }
  const [cpx, cpy, cpz] = [P[p]! - P[c]!, P[p + 1]! - P[c + 1]!, P[p + 2]! - P[c + 2]!];
  const [d5, d6] = [abx * cpx + aby * cpy + abz * cpz, acx * cpx + acy * cpy + acz * cpz];
  if (d6 >= 0 && d5 <= d6) return norm(cpx, cpy, cpz);
  const vb = d5 * d2 - d1 * d6;
  if (vb <= 0 && d2 >= 0 && d6 <= 0) {
    const w = d2 / (d2 - d6 || 1);
    return norm(apx - w * acx, apy - w * acy, apz - w * acz);
  }
  const va = d3 * d6 - d5 * d4;
  if (va <= 0 && d4 - d3 >= 0 && d5 - d6 >= 0) {
    const w = (d4 - d3) / (d4 - d3 + (d5 - d6) || 1);
    return norm(bpx - w * (P[c]! - P[b]!), bpy - w * (P[c + 1]! - P[b + 1]!), bpz - w * (P[c + 2]! - P[b + 2]!));
  }
  const [nx, ny, nz] = [aby * acz - abz * acy, abz * acx - abx * acz, abx * acy - aby * acx];
  const n2 = norm(nx, ny, nz);
  if (n2 <= 1e-300) return Math.min(segments2(p, p, a, b), segments2(p, p, a, c));
  const h = apx * nx + apy * ny + apz * nz;
  return (h * h) / n2;
}

function pierces(p: number, q: number, a: number, b: number, c: number): boolean {
  const [dx, dy, dz] = [P[q]! - P[p]!, P[q + 1]! - P[p + 1]!, P[q + 2]! - P[p + 2]!];
  const [e1x, e1y, e1z] = [P[b]! - P[a]!, P[b + 1]! - P[a + 1]!, P[b + 2]! - P[a + 2]!];
  const [e2x, e2y, e2z] = [P[c]! - P[a]!, P[c + 1]! - P[a + 1]!, P[c + 2]! - P[a + 2]!];
  const [hx, hy, hz] = [dy * e2z - dz * e2y, dz * e2x - dx * e2z, dx * e2y - dy * e2x];
  const det = e1x * hx + e1y * hy + e1z * hz;
  if (Math.abs(det) < 1e-300) return false;
  const [sx, sy, sz] = [P[p]! - P[a]!, P[p + 1]! - P[a + 1]!, P[p + 2]! - P[a + 2]!];
  const u = (sx * hx + sy * hy + sz * hz) / det;
  if (u < 0 || u > 1) return false;
  const [rx, ry, rz] = [sy * e1z - sz * e1y, sz * e1x - sx * e1z, sx * e1y - sy * e1x];
  const v = (dx * rx + dy * ry + dz * rz) / det;
  if (v < 0 || u + v > 1) return false;
  const t = (e2x * rx + e2y * ry + e2z * rz) / det;
  return t >= 0 && t <= 1;
}

/** 📏️ The squared smallest distance between two triangles given as nine coordinates each: 0 when they cross or touch, else the least of the six vertex to face and nine edge to edge distances. */
export function triangleDistance2(first: ArrayLike<number>, second: ArrayLike<number>): number {
  for (let i = 0; i < 9; i++) {
    P[i] = first[i]!;
    P[9 + i] = second[i]!;
  }
  let best = Infinity;
  for (let i = 0; i < 9; i += 3) {
    best = Math.min(best, pointTriangle2(i, 9, 12, 15), pointTriangle2(9 + i, 0, 3, 6));
    for (let j = 0; j < 9; j += 3) best = Math.min(best, segments2(i, (i + 3) % 9, 9 + j, 9 + ((j + 3) % 9)));
  }
  if (best === 0) return 0;
  for (let i = 0; i < 9; i += 3) if (pierces(i, (i + 3) % 9, 9, 12, 15) || pierces(9 + i, 9 + ((i + 3) % 9), 0, 3, 6)) return 0;
  return best;
}

const boxGap = (a: Part, b: Part): number => Math.hypot(...[0, 1, 2].map((k) => Math.max(0, a.min[k]! - b.max[k]!, b.min[k]! - a.max[k]!)));
const within = (inner: Part, outer: Part): boolean => [0, 1, 2].every((k) => inner.min[k]! >= outer.min[k]! && inner.max[k]! <= outer.max[k]!);
const WORLDS = new WeakMap<Float64Array, World>();
const asWorld = (part: Part): World => {
  let hit = WORLDS.get(part.positions);
  if (hit === undefined) WORLDS.set(part.positions, (hit = { id: part.id, positions: part.positions, indices: part.indices }));
  return hit;
};
const vertexInside = (inner: Part, outer: Part): boolean => within(inner, outer) && insideAny([asWorld(outer)], [inner.positions[0]!, inner.positions[1]!, inner.positions[2]!]);

/** 📏️ The smallest distance between the surfaces of two parts in metres when it is at most `cutoffM`, else `Infinity`; 0 when the surfaces cross or one part lies inside the other. With `first` the search stops at the first distance at most `cutoffM` and returns that distance, which is then an upper bound of the smallest. */
export function partGap(a: Part, b: Part, cutoffM: number, first = false): number {
  if (boxGap(a, b) > cutoffM) return Infinity;
  const [small, large] = a.indices.length <= b.indices.length ? [a, b] : [b, a];
  const grid = cellsOf(large);
  const { positions: p, indices: pi } = small;
  const { positions: q, indices: qi } = large;
  const [ta, tb] = [new Float64Array(9), new Float64Array(9)];
  let best2 = cutoffM * cutoffM;
  let found = Infinity;
  if (!first) {
    const [sp, sq] = [Math.max(1, Math.floor(p.length / 3 / 48)) * 3, Math.max(1, Math.floor(q.length / 3 / 48)) * 3];
    let near2 = Infinity;
    for (let i = 0; i < p.length; i += sp) for (let j = 0; j < q.length; j += sq) near2 = Math.min(near2, (p[i]! - q[j]!) ** 2 + (p[i + 1]! - q[j + 1]!) ** 2 + (p[i + 2]! - q[j + 2]!) ** 2);
    if (near2 < best2) best2 = near2 * (1 + 1e-12) + 1e-24;
  }
  for (let t = 0; t < pi.length; t += 3) {
    const reach = Math.sqrt(best2);
    const r = [0, 0, 0, 0, 0, 0];
    let skip = false;
    for (let k = 0; k < 3; k++) {
      const [x, y, z] = [p[pi[t]! * 3 + k]!, p[pi[t + 1]! * 3 + k]!, p[pi[t + 2]! * 3 + k]!];
      [ta[k], ta[3 + k], ta[6 + k]] = [x, y, z];
      const [lo, hi] = [Math.min(x, y, z) - reach, Math.max(x, y, z) + reach];
      if (lo > large.max[k]! || hi < large.min[k]!) skip = true;
      r[2 * k] = Math.max(0, Math.min(grid.n[k]! - 1, Math.floor((lo - large.min[k]!) / grid.cell)));
      r[2 * k + 1] = Math.max(0, Math.min(grid.n[k]! - 1, Math.floor((hi - large.min[k]!) / grid.cell)));
    }
    if (skip) continue;
    grid.mark++;
    for (let z = r[4]!; z <= r[5]!; z++) for (let y = r[2]!; y <= r[3]!; y++) for (let x = r[0]!; x <= r[1]!; x++) {
      const c = (z * grid.n[1] + y) * grid.n[0] + x;
      for (let i = grid.start[c]!; i < grid.start[c + 1]!; i++) {
        const u = grid.items[i]!;
        if (grid.stamp[u / 3] === grid.mark) continue;
        grid.stamp[u / 3] = grid.mark;
        for (let k = 0; k < 3; k++) [tb[k], tb[3 + k], tb[6 + k]] = [q[qi[u]! * 3 + k]!, q[qi[u + 1]! * 3 + k]!, q[qi[u + 2]! * 3 + k]!];
        const d2 = triangleDistance2(ta, tb);
        if (d2 <= best2) {
          best2 = d2;
          found = Math.sqrt(d2);
          if (first || d2 === 0) return found;
        }
      }
    }
  }
  if (found === Infinity && (vertexInside(a, b) || vertexInside(b, a))) return 0;
  return found;
}

/** 🧮️ The groups of parts that touch within `toleranceM`, directly or through other parts (union-find over the pairs whose boxes come that close; a pair already in one group is not measured). */
export function touchingGroups(parts: readonly Part[], toleranceM: number, tick: () => void = () => {}): Groups {
  const parent = Int32Array.from(parts, (_, i) => i);
  const find = (i: number): number => {
    while (parent[i] !== i) i = parent[i] = parent[parent[i]!]!;
    return i;
  };
  const order = parts.map((_, i) => i).sort((i, j) => parts[i]!.min[0] - parts[j]!.min[0] || i - j);
  for (let a = 0; a < order.length; a++) {
    tick();
    const i = order[a]!;
    for (let b = a + 1; b < order.length; b++) {
      const j = order[b]!;
      if (parts[j]!.min[0] > parts[i]!.max[0] + toleranceM) break;
      if (find(i) === find(j)) continue;
      if (partGap(parts[i]!, parts[j]!, toleranceM, true) <= toleranceM) parent[find(Math.max(i, j))] = find(Math.min(i, j));
    }
  }
  const facts = new Map<number, { weight: number; low: number; id: string }>();
  parts.forEach((part, i) => {
    const fact = facts.get(find(i)) ?? { weight: 0, low: Infinity, id: part.id };
    fact.weight += (part.max[0] - part.min[0]) * (part.max[1] - part.min[1]) * (part.max[2] - part.min[2]);
    fact.low = Math.min(fact.low, part.min[1]);
    if (part.id < fact.id) fact.id = part.id;
    facts.set(find(i), fact);
  });
  const roots = [...facts.keys()].sort((p, q) => {
    const [a, b] = [facts.get(p)!, facts.get(q)!];
    if (Math.abs(a.weight - b.weight) > 1e-9 * Math.max(a.weight, b.weight)) return b.weight - a.weight;
    if (Math.abs(a.low - b.low) > 1e-9) return a.low - b.low;
    return a.id < b.id ? -1 : a.id > b.id ? 1 : p - q;
  });
  const rank = new Map(roots.map((root, k) => [root, k] as const));
  return { group: Int32Array.from(parts, (_, i) => rank.get(find(i))!), count: roots.length };
}

/** 📏️ How far the parts of group `g` are from the nearest part of another group: the exact distance when it is at most `capM`, else `capM` (a group farther away than the cap is reported at the cap). Only the parts that `counts` accepts are measured, on both sides: in a structure list the `mass` markers are left out, because a mass is no load path and never what a member should have met. */
export function groupGap(parts: readonly Part[], groups: Groups, g: number, capM: number, counts: (part: Part) => boolean = () => true): { readonly gapM: number; readonly nearest: number } {
  let [best, nearest] = [capM, -1];
  const others = parts.map((_, j) => j).filter((j) => groups.group[j] !== g && counts(parts[j]!));
  parts.forEach((own, i) => {
    if (groups.group[i] !== g || !counts(own)) return;
    const order = others.map((j) => [boxGap(own, parts[j]!), j] as const).filter(([d]) => d <= best).sort((p, q) => p[0] - q[0] || p[1] - q[1]);
    for (const [low, j] of order) {
      if (low > best) break;
      const d = partGap(own, parts[j]!, best);
      if (d < best || (d === best && nearest < 0 && d < capM)) [best, nearest] = [d, j];
    }
  });
  return { gapM: best, nearest };
}

/** 🏷️ What a part is for the judgement of a list: a `mass` marker sits at a centre of mass and may hang in the air, a `support` closes a load path. */
export type PartKind = "mass" | "support" | undefined;

/** 🛑️ One group of parts that the list does not hold together: `floating` touches neither the largest group nor the ground nor an anchor, `near` rests on its own but lies within the clearance of another group, `detached` rests on its own but is tied to another group (a movable joint of the rig holds the two together) without touching it, `unsupported` is a group of a structure list that reaches neither a support solid nor the ground nor an anchor. `ids` are the solids of the group in list order, `gapM` the distance to the nearest part of another group (the cap when farther), `nearest` the id of that part; in a structure list neither is ever taken from a `mass` marker, which is no load path. */
export type Loose = { readonly reason: "floating" | "near" | "detached" | "unsupported"; readonly ids: readonly string[]; readonly parts: number; readonly gapM: number; readonly nearest?: string; readonly tie?: string };

/** ⚖️ What {@link looseGroups} is asked: the contact tolerance, the clearance below which two resting groups are neither joined nor apart, the cap of the measured gap, whether a part rests on something outside the list (the ground, an anchor) and, for a structure list, the kind of each part (`structure: true` then asks every group that is not only masses for a support instead of asking for one body); `tie` names what ties a resting group to the parts outside it although they do not touch (the joint of the rig between their bones), or nothing. */
export type ConnectedOptions = { readonly tie?: (own: readonly Part[], rest: readonly Part[], groupOf: (part: Part) => number) => string | undefined; readonly toleranceM: number; readonly apartM: number; readonly capM: number; readonly rests: (part: Part) => boolean; readonly structure?: boolean; readonly kind?: (part: Part) => PartKind; readonly tick?: () => void };

/** 🔗️ The groups of touching parts of one list and those that fail: in a body every group apart from the largest must rest on the ground or at an anchor, keep the clearance and not be tied to another group; in a structure every group that is not only mass markers must hold a support solid or rest on the ground or at an anchor (in a list without any support solid the largest group is the structure itself and the others must join it), and such a group apart from the largest must keep the clearance to the members, joints and supports of the other groups (a mass marker beside a member is no gap in a load path and is never the nearest solid). Sorted by reason, then the largest gap first, then the first id. */
export function looseGroups(parts: readonly Part[], options: ConnectedOptions): { readonly groups: number; readonly loose: readonly Loose[] } {
  const groups = touchingGroups(parts, options.toleranceM, options.tick);
  const kind = options.kind ?? (() => undefined);
  const loose: Loose[] = [];
  const index = new Map(parts.map((part, i) => [part, i] as const));
  const supported = parts.some((part) => kind(part) === "support");
  for (let g = 0; g < groups.count; g++) {
    options.tick?.();
    const own = parts.filter((_, i) => groups.group[i] === g);
    const kinds = own.map(kind);
    if (options.structure === true && kinds.every((k) => k === "mass")) continue;
    const held = options.structure === true ? kinds.includes("support") || (g === 0 && !supported) || own.some(options.rests) : g === 0 || own.some(options.rests);
    if (held && g === 0) continue;
    const gap = groups.count === 1 ? { gapM: options.capM, nearest: -1 } : groupGap(parts, groups, g, options.capM, options.structure === true ? (part) => kind(part) !== "mass" : undefined);
    const tie = held && options.tie !== undefined ? options.tie(own, parts.filter((_, i) => groups.group[i] !== g), (part) => groups.group[index.get(part)!]!) : undefined;
    if (held && gap.gapM > options.apartM && tie === undefined) continue;
    loose.push({ reason: !held ? (options.structure === true ? "unsupported" : "floating") : tie !== undefined ? "detached" : "near", ids: [...new Set(own.map((p) => p.id))], parts: own.length, gapM: gap.gapM, ...(gap.nearest < 0 ? {} : { nearest: parts[gap.nearest]!.id }), ...(tie === undefined ? {} : { tie }) });
  }
  const order = { floating: 0, unsupported: 0, detached: 1, near: 2 } as const;
  return { groups: groups.count, loose: loose.sort((p, q) => order[p.reason] - order[q.reason] || q.gapM - p.gapM || (p.ids[0]! < q.ids[0]! ? -1 : p.ids[0]! > q.ids[0]! ? 1 : 0)) };
}
