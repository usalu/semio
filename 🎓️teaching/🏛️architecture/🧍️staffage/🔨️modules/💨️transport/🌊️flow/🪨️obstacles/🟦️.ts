/** 🪨️ The body of an asset as obstacles of the air: a small set of analytic superellipsoids in world metres, how air is deflected around them, how a point is kept out of them and what they hide from an eye.
 *
 * Shape. An obstacle is the superellipsoid `f(x) = (|q₁|ᵖ + |q₂|ᵖ + |q₃|ᵖ)^(1/p) = 1` with `q_i = a_i·(x − c)/r_i` (centre `c`, orthonormal axes `a_i`, radii `r_i`, `p ∈ {2, 4, 8}`): an ellipsoid, a rounded cylinder or a rounded box. `f` is a norm of `q`, so it is convex along every line, equals 1 on the surface and grows linearly with the distance from the centre; the outward normal is `n = ∇f/|∇f|`.
 * Derivation ({@link bodyClusters}, {@link obstaclesOf}). The solids of the `body` list of the finest authored volume (or of `scale`) are tessellated once by the neutral kernel in their bone frame; of each solid the support points in 74 directions (the 26 of a cube and 48 of a Fibonacci sphere), its volume and its surface area are kept. Solids are grouped at the rest pose: again and again the two neighbouring groups of one kind (their boxes, widened by a quarter of the smaller one, overlap; any two once no neighbours are left) whose common oriented box wastes the least volume, `V_ab − V_a − V_b`,
 * are merged, until at most {@link OBSTACLE_CAP} groups are left and no pair is left whose common box is smaller than its two boxes together: a trunk stays one shape, a limb gathers its branches, a hand its fingers. At an evaluated state the support points of a group are carried by the evaluated frames of their own bones and fitted again: the oriented box of least volume among the axes of the group's main bone and the principal axes of the points gives centre, axes and radii,
 * and for each exponent the radii are the half sizes of the box times the largest `f` of a support point, so the obstacle holds all of them and no stream cuts a corner of the body; the exponent is the one whose obstacle is smallest (the superellipsoids fill π/6, 0.81 and 0.94 of their box): the points of an ellipsoid lie on `p = 2`, the corners of a box ask for 8. Groups whose box at rest is smaller than {@link OBSTACLE_MIN_SHARE} of the largest one (a nose, a switch) are left out. A solid that is skinned to several bones moves with its own bone.
 * Kind and blockage `φ` (the share of the undisturbed normal flow an obstacle stops):
 * - `solid`: `φ = 1`, a wall. The solids cover at least {@link LATTICE_COVER} of the silhouette of the fitted shape: the cover is `S_solids/S_shape`, because the mean projected area of a convex solid is a quarter of its surface (Cauchy), with `S_shape` the ellipsoid area by Thomsen's formula times 1, 1.45 and 1.75 for the exponents 2, 4 and 8. A hollow casing covers its silhouette and is a wall.
 * - `lattice`: body solids that cover less of it (the limbs of a crown gathered into one shape): `φ` is the cover.
 * - `foliage`: solids whose material is the material of a thermal region of kind `foliage`. Leaves are a porous canopy with drag `c_d a u²` per volume, so air that crosses a chord `L` keeps `exp(−c_d a L)` of its speed (`c_d` = {@link FOLIAGE_DRAG}, leaf area density `a` = {@link FOLIAGE_AREA_DENSITY_M} m²/m³, `L = 4/3·∛(r₁r₂r₃)` the mean chord of the equal sphere): `φ = 1 − exp(−c_d a L)`.
 * An asset without an authored volume has no obstacles.
 *
 * Deflection ({@link deflect}). With `U` the undisturbed velocity at `x` and `s = f(x) ≥ 1`, every obstacle adds the correction of a doublet, `Δu = φ/(2s³)·(U − 3(U·n)n)`: for a sphere this is the exact potential flow past it (`u = U + R³/(2r³)(U − 3(U·r̂)r̂)`, stagnation points up- and downwind, 1.5 U at the equator); for every shape the normal velocity on the surface is `(1 − φ)U·n`, zero for a wall.
 * The flow of jets and plumes is not a flow from afar but starts on the body, so of it only the part that runs into an obstacle is removed: `Δu = −φ·min(0, U_j·n)·n/s³` (the jet slides along the wall it hits and leaves its nozzle freely). Inside a porous obstacle the air moves with `(1 − φ)U`; inside a wall there is no air: velocity and scalars are zero.
 * Every obstacle has a mirror image below the ground and on the ground itself the vertical velocity is left as it was, so no air crosses `y = 0`. Corrections of obstacles farther than `s =` {@link OBSTACLE_REACH} (below 0.2 % of `U`) are left out, and the sum of all corrections speeds the external flow up to at most {@link OBSTACLE_SPEED_UP} times its undisturbed speed. Where several obstacles meet, the corrections of the others leave a normal velocity on a wall; inside the shell `s < 1 +` {@link OBSTACLE_SHELL} this residual is removed (all of it on the surface, fading to the edge of the shell; the nearest wall last), so the normal velocity on every wall is zero.
 * Openings. Around the nozzle of an active emitter that blows or draws air (breath, vent, fan, exhaust, flue, an inhaling nose or mouth) the wall is open: within `ρ = max(1.25 d, depth + 0.75 d)` of the nozzle (diameter `d`, `depth` of the nozzle inside the fitted shape, measured along the ray from the centre of the obstacle and at most its smallest radius) nothing is corrected and the inside counts as air, fading to a closed wall at `2ρ`.
 * Keeping out ({@link pushOut}): a point inside a wall moves along the normal to `s = 1 +` {@link OBSTACLE_SKIN} (Newton steps on `f`).
 * Occlusion ({@link flowVisibility}): a point is hidden from an eye when the ray from it to the eye meets a wall (`min f < 1 −` {@link OCCLUSION_MARGIN} along the ray; closed form for `p = 2`, golden-section search of the convex `f` otherwise) and veiled by a porous obstacle to `1 −` {@link OBSTACLE_VEIL}`·φ` of its opacity.
 *
 * @see ../🟦️.ts — the field that is deflected
 * @see ../../../🔗️contracts/🟦️.ts — `FlowObstacle`, `FlowEye`
 * @see https://doi.org/10.1017/CBO9780511800955 — Batchelor 1967, An Introduction to Fluid Dynamics, § 6.8 (flow past a sphere)
 * @see https://doi.org/10.1023/B:BOUN.0000030653.71031.96 — Katul et al. 2004, One- and two-equation models for canopy turbulence (canopy drag)
 */
import type { Asset, AuthoredVolume, Slug } from "../../../../🧬️schema/🟦️.ts";
import { authoredScales, type EmitterState, type EvaluatedState, type FlowEye, type FlowObstacle, type Vec3 } from "../../../🔗️contracts/🟦️.ts";
import { restFrames } from "../../../🦴️rig/🟦️.ts";
import { authoredAt, solidOf } from "../../../🧊️volume/🧊️solids/🟦️.ts";
import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧱️solid/🟦️.ts";
import * as portable from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 🧮️ The most obstacles a body has. */
export const OBSTACLE_CAP = 16;
/** 🧮️ The share of the box volume of the largest group below which a group is no obstacle. */
export const OBSTACLE_MIN_SHARE = 1 / 500;
/** 🧮️ The share of the silhouette of its fitted shape a group of body solids covers from which it is a wall. */
export const LATTICE_COVER = 0.5;
/** 🍃️ Drag coefficient of foliage per leaf area. */
export const FOLIAGE_DRAG = 0.2;
/** 🍃️ Leaf area density of a crown (m² of leaf per m³). */
export const FOLIAGE_AREA_DENSITY_M = 1;
/** 🧮️ The level `s` beyond which the correction of an obstacle is left out. */
export const OBSTACLE_REACH = 8;
/** 🧮️ The most the corrections of all obstacles together speed the external flow up, as a multiple of its undisturbed speed (2 is the equator of a cylinder in cross flow, 1.5 that of a sphere): the sum of the doublets of obstacles that touch overshoots in the gaps between them. */
export const OBSTACLE_SPEED_UP = 2;
/** 🧮️ The thickness of the shell (in `s`) in which the residual normal velocity on a wall is removed. */
export const OBSTACLE_SHELL = 0.15;
/** 🧮️ How far outside a wall (in `s`) a point that was inside is put. */
export const OBSTACLE_SKIN = 0.01;
/** 👁️ How far inside a wall (in `s`) a ray must pass to be hidden, so a mark that slides along the near side of a wall stays visible. */
export const OCCLUSION_MARGIN = 0.03;
/** 👁️ The share of its blockage by which a porous obstacle dims what lies behind or inside it. */
export const OBSTACLE_VEIL = 0.75;
/** 🧮️ The smallest radius of an obstacle (m). */
export const MIN_RADIUS_M = 0.005;

const QUALITY = { chordM: 0.01, minSegments: 8, maxSegments: 16 } as const;
const DIRECTIONS: readonly (readonly [number, number, number])[] = [
  ...[-1, 0, 1].flatMap((x) => [-1, 0, 1].flatMap((y) => [-1, 0, 1].filter((z) => x !== 0 || y !== 0 || z !== 0).map((z) => [x, y, z] as const))),
  ...Array.from({ length: 48 }, (_, i) => {
    const y = 1 - (2 * i + 1) / 48;
    const r = Math.sqrt(1 - y * y);
    const turn = i * Math.PI * (3 - Math.sqrt(5));
    return [r * portable.cos(turn), y, r * portable.sin(turn)] as const;
  }),
];
const AREA = { 2: 1, 4: 1.45, 8: 1.75 } as const;
const FILL = { 2: Math.PI / 6, 4: 0.8102, 8: 0.9396 } as const;

type Cloud = { readonly bone: Slug; readonly points: Float64Array };
/** 🧩️ A group of body solids that becomes one obstacle: the support points of its solids per bone (bone-local `u, v, w`), their volume and surface area, whether they are foliage and the bone that carries most of the volume. */
export type BodyCluster = { readonly id: Slug; readonly foliage: boolean; readonly clouds: readonly Cloud[]; readonly volumeM3: number; readonly areaM2: number; readonly bone: Slug };
type Item = { id: Slug; foliage: boolean; clouds: Map<Slug, Float64Array>; shares: Map<Slug, number>; volume: number; area: number; rest: Float64Array; box: number; lo: [number, number, number]; hi: [number, number, number] };
type Triple = readonly [number, number, number];
type Fit = { readonly centre: [number, number, number]; readonly axes: [Triple, Triple, Triple]; readonly half: [number, number, number]; readonly volume: number };

function support(points: Float64Array): Float64Array {
  if (points.length <= 3 * DIRECTIONS.length) return points;
  const picked = new Set<number>();
  for (const [dx, dy, dz] of DIRECTIONS) {
    let best = 0, top = -Infinity;
    for (let i = 0; i < points.length; i += 3) {
      const d = points[i]! * dx + points[i + 1]! * dy + points[i + 2]! * dz;
      if (d > top) { top = d; best = i; }
    }
    picked.add(best);
  }
  const out = new Float64Array(picked.size * 3);
  let o = 0;
  for (const i of [...picked].sort((a, b) => a - b)) { out[o++] = points[i]!; out[o++] = points[i + 1]!; out[o++] = points[i + 2]!; }
  return out;
}

function boxAlong(points: Float64Array, axes: readonly [Triple, Triple, Triple]): Fit {
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < points.length; i += 3)
    for (let k = 0; k < 3; k++) {
      const d = points[i]! * axes[k]![0] + points[i + 1]! * axes[k]![1] + points[i + 2]! * axes[k]![2];
      if (d < lo[k]!) lo[k] = d;
      if (d > hi[k]!) hi[k] = d;
    }
  const mid = [0, 1, 2].map((k) => (lo[k]! + hi[k]!) / 2);
  const half = [0, 1, 2].map((k) => Math.max(MIN_RADIUS_M, (hi[k]! - lo[k]!) / 2)) as [number, number, number];
  return { centre: [0, 1, 2].map((j) => mid[0]! * axes[0][j]! + mid[1]! * axes[1][j]! + mid[2]! * axes[2][j]!) as [number, number, number], axes: [axes[0], axes[1], axes[2]], half, volume: 8 * half[0] * half[1] * half[2] };
}

const WORLD_AXES: [Triple, Triple, Triple] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];

/** 🧭️ The principal axes of a point set: the eigenvectors of its covariance by cyclic Jacobi rotations (square roots only, so the result is the same on every machine). */
export function principalAxes(points: Float64Array): [Triple, Triple, Triple] {
  const n = points.length / 3;
  if (n < 3) return WORLD_AXES;
  const mean = [0, 0, 0];
  for (let i = 0; i < points.length; i += 3) for (let k = 0; k < 3; k++) mean[k]! += points[i + k]! / n;
  const a = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
  for (let i = 0; i < points.length; i += 3) for (let r = 0; r < 3; r++) for (let c = 0; c < 3; c++) a[r]![c]! += (points[i + r]! - mean[r]!) * (points[i + c]! - mean[c]!);
  const v = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
  for (let sweep = 0; sweep < 12; sweep++) {
    const off = Math.abs(a[0]![1]!) + Math.abs(a[0]![2]!) + Math.abs(a[1]![2]!);
    if (off < 1e-14 * (Math.abs(a[0]![0]!) + Math.abs(a[1]![1]!) + Math.abs(a[2]![2]!) + 1e-300)) break;
    for (const [p, q] of [[0, 1], [0, 2], [1, 2]] as const) {
      if (Math.abs(a[p]![q]!) < 1e-300) continue;
      const theta = (a[q]![q]! - a[p]![p]!) / (2 * a[p]![q]!);
      const t = (theta >= 0 ? 1 : -1) / (Math.abs(theta) + Math.sqrt(theta * theta + 1));
      const c = 1 / Math.sqrt(t * t + 1), s = t * c;
      for (let k = 0; k < 3; k++) {
        const akp = a[k]![p]!, akq = a[k]![q]!;
        a[k]![p] = c * akp - s * akq;
        a[k]![q] = s * akp + c * akq;
      }
      for (let k = 0; k < 3; k++) {
        const apk = a[p]![k]!, aqk = a[q]![k]!;
        a[p]![k] = c * apk - s * aqk;
        a[q]![k] = s * apk + c * aqk;
      }
      for (let k = 0; k < 3; k++) {
        const vkp = v[k]![p]!, vkq = v[k]![q]!;
        v[k]![p] = c * vkp - s * vkq;
        v[k]![q] = s * vkp + c * vkq;
      }
    }
  }
  const x: Triple = [v[0]![0]!, v[1]![0]!, v[2]![0]!];
  const y: Triple = [v[0]![1]!, v[1]![1]!, v[2]![1]!];
  return [x, y, [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]]];
}

/** 📦️ The oriented box of a point set: the smaller of the box along `preferred` (kept unless the other is under 0.8 of its volume) and the box along the principal axes. */
function fit(points: Float64Array, preferred: [Triple, Triple, Triple]): Fit {
  const own = boxAlong(points, preferred);
  const principal = boxAlong(points, principalAxes(points));
  return principal.volume < 0.8 * own.volume ? principal : own;
}

const tripleOf = (p: Triple | Vec3): Triple => (Array.isArray(p) ? (p as Triple) : [(p as Vec3).x, (p as Vec3).y, (p as Vec3).z]);
type Frame = { readonly o: Triple | Vec3; readonly u: Triple | Vec3; readonly v: Triple | Vec3; readonly w: Triple | Vec3 };

function carry(points: Float64Array, frame: Frame, out: Float64Array, at: number): void {
  const o = tripleOf(frame.o), u = tripleOf(frame.u), v = tripleOf(frame.v), w = tripleOf(frame.w);
  for (let i = 0; i < points.length; i += 3) for (let k = 0; k < 3; k++) out[at + i + k] = o[k]! + points[i]! * u[k]! + points[i + 1]! * v[k]! + points[i + 2]! * w[k]!;
}

function measure(positions: readonly number[], indices: readonly number[]): { readonly volume: number; readonly area: number } {
  let six = 0, twice = 0;
  for (let t = 0; t < indices.length; t += 3) {
    const a = indices[t]! * 3, b = indices[t + 1]! * 3, c = indices[t + 2]! * 3;
    const ux = positions[b]! - positions[a]!, uy = positions[b + 1]! - positions[a + 1]!, uz = positions[b + 2]! - positions[a + 2]!, vx = positions[c]! - positions[a]!, vy = positions[c + 1]! - positions[a + 1]!, vz = positions[c + 2]! - positions[a + 2]!;
    const nx = uy * vz - uz * vy, ny = uz * vx - ux * vz, nz = ux * vy - uy * vx;
    six += positions[a]! * nx + positions[a + 1]! * ny + positions[a + 2]! * nz;
    twice += Math.sqrt(nx * nx + ny * ny + nz * nz);
  }
  return { volume: Math.abs(six) / 6, area: twice / 2 };
}

function boundsOf(points: Float64Array): { lo: [number, number, number]; hi: [number, number, number] } {
  const lo: [number, number, number] = [Infinity, Infinity, Infinity], hi: [number, number, number] = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < points.length; i += 3) for (let k = 0; k < 3; k++) {
    if (points[i + k]! < lo[k]!) lo[k] = points[i + k]!;
    if (points[i + k]! > hi[k]!) hi[k] = points[i + k]!;
  }
  return { lo, hi };
}

function concat(a: Float64Array, b: Float64Array): Float64Array {
  const out = new Float64Array(a.length + b.length);
  out.set(a);
  out.set(b, a.length);
  return out;
}

function merged(a: Item, b: Item): Item {
  const clouds = new Map(a.clouds);
  for (const [bone, points] of b.clouds) clouds.set(bone, clouds.has(bone) ? support(concat(clouds.get(bone)!, points)) : points);
  const shares = new Map(a.shares);
  for (const [bone, share] of b.shares) shares.set(bone, (shares.get(bone) ?? 0) + share);
  const rest = support(concat(a.rest, b.rest));
  return { id: a.volume >= b.volume ? a.id : b.id, foliage: a.foliage, clouds, shares, volume: a.volume + b.volume, area: a.area + b.area, rest, box: fit(rest, WORLD_AXES).volume, ...boundsOf(rest) };
}

function neighbours(a: Item, b: Item): boolean {
  const size = (item: Item): number => Math.max(item.hi[0] - item.lo[0], item.hi[1] - item.lo[1], item.hi[2] - item.lo[2]);
  const margin = 0.25 * Math.min(size(a), size(b));
  return [0, 1, 2].every((k) => a.lo[k]! - margin <= b.hi[k]! && b.lo[k]! - margin <= a.hi[k]!);
}

const CLUSTERS = new WeakMap<AuthoredVolume, readonly BodyCluster[]>();

/** 🧩️ The groups of body solids of an asset at an authored scale (default: the finest authored one) that become its obstacles; `[]` without an authored volume. Cached per volume. See the module text for the grouping. */
export function bodyClusters(asset: Asset, scale?: number): readonly BodyCluster[] {
  const denominator = scale ?? authoredScales(asset.volumes)[0];
  if (denominator === undefined || !authoredScales(asset.volumes).includes(denominator)) return [];
  const volume = authoredAt(asset.volumes, denominator, asset.id);
  const known = CLUSTERS.get(volume);
  if (known !== undefined) return known;
  const leaves = new Set(asset.thermal.regions.filter((region) => region.kind === "foliage").map((region) => region.material));
  const frames = restFrames(asset.rig);
  const items: Item[] = [];
  for (const solid of volume.body) {
    const frame = frames[solid.bone]?.frame;
    if (frame === undefined) continue;
    const geometry = tessellateSolid(solidOf(solid, solid.material), QUALITY);
    if (geometry.indices.length === 0) continue;
    const local = Float64Array.from(geometry.positions);
    for (let i = 2; i < local.length; i += 3) local[i] = -local[i]!;
    const points = support(local);
    const rest = new Float64Array(points.length);
    carry(points, frame, rest, 0);
    const { volume: volumeM3, area } = measure(geometry.positions, geometry.indices);
    items.push({ id: solid.id, foliage: leaves.has(solid.material), clouds: new Map([[solid.bone, points]]), shares: new Map([[solid.bone, volumeM3]]), volume: volumeM3, area, rest, box: fit(rest, WORLD_AXES).volume, ...boundsOf(rest) });
  }
  const groups: (Item | null)[] = [...items];
  const cost = (a: Item, b: Item): number => merged(a, b).box - a.box - b.box;
  const costs = new Map<number, Map<number, number>>();
  const link = (i: number, j: number): void => {
    const lo = Math.min(i, j), hi = Math.max(i, j);
    (costs.get(lo) ?? costs.set(lo, new Map()).get(lo)!).set(hi, cost(groups[lo]!, groups[hi]!));
  };
  const connect = (near: boolean): void => {
    for (let i = 0; i < groups.length; i++)
      for (let j = i + 1; j < groups.length; j++) {
        const a = groups[i], b = groups[j];
        if (a !== null && b !== null && a !== undefined && b !== undefined && a.foliage === b.foliage && costs.get(i)?.has(j) !== true && (!near || neighbours(a, b))) link(i, j);
      }
  };
  connect(true);
  let count = items.length;
  let all = false;
  for (;;) {
    let bi = -1, bj = -1, best = Infinity;
    for (const [i, row] of costs) for (const [j, c] of row) if (c < best) { best = c; bi = i; bj = j; }
    if (bi < 0) {
      if (count <= OBSTACLE_CAP || all) break;
      all = true;
      connect(false);
      continue;
    }
    if (best >= 0 && count <= OBSTACLE_CAP) break;
    const near = new Set<number>();
    for (const [i, row] of costs) {
      if (i === bi || i === bj) for (const j of row.keys()) near.add(j);
      else {
        if (row.has(bi) || row.has(bj)) near.add(i);
        row.delete(bi);
        row.delete(bj);
      }
    }
    costs.delete(bi);
    costs.delete(bj);
    groups[bi] = merged(groups[bi]!, groups[bj]!);
    groups[bj] = null;
    count--;
    near.delete(bi);
    near.delete(bj);
    for (const k of near) link(bi, k);
  }
  const largest = groups.reduce((most, item) => Math.max(most, item?.box ?? 0), 0);
  const made = groups.filter((item): item is Item => item !== null && item.box >= OBSTACLE_MIN_SHARE * largest).map((item): BodyCluster => ({ id: item.id, foliage: item.foliage, clouds: [...item.clouds].map(([bone, points]) => ({ bone, points })), volumeM3: item.volume, areaM2: item.area, bone: [...item.shares].reduce((best, entry) => (entry[1] > best[1] ? entry : best))[0] }));
  CLUSTERS.set(volume, made);
  return made;
}

const POSED = new WeakMap<EvaluatedState, Map<number, readonly FlowObstacle[]>>();
const vec = (t: Triple): Vec3 => ({ x: t[0], y: t[1], z: t[2] });

/** 🪨️ The obstacles of an evaluated state in world metres (ahead x, up y, left z): the groups of {@link bodyClusters} carried by the evaluated bone frames and fitted (see the module text). Cached per state and scale. */
export function obstaclesOf(state: EvaluatedState, scale?: number): readonly FlowObstacle[] {
  const clusters = bodyClusters(state.asset, scale);
  if (clusters.length === 0) return [];
  const byScale = POSED.get(state) ?? POSED.set(state, new Map()).get(state)!;
  const known = byScale.get(scale ?? 0);
  if (known !== undefined) return known;
  const made: FlowObstacle[] = [];
  for (const cluster of clusters) {
    const clouds = cluster.clouds.filter((cloud) => state.bones[cloud.bone] !== undefined);
    const main = state.bones[cluster.bone]?.frame;
    if (clouds.length === 0 || main === undefined) continue;
    const world = new Float64Array(clouds.reduce((sum, cloud) => sum + cloud.points.length, 0));
    let at = 0;
    for (const cloud of clouds) {
      carry(cloud.points, state.bones[cloud.bone]!.frame, world, at);
      at += cloud.points.length;
    }
    const f = fit(world, [tripleOf(main.u), tripleOf(main.v), tripleOf(main.w)]);
    let exponent: 2 | 4 | 8 = 2, grow = 1, least = Infinity;
    for (const p of [2, 4, 8] as const) {
      let most = 0;
      for (let k = 0; k < world.length; k += 3) {
        let sum = 0;
        for (let axis = 0; axis < 3; axis++) {
          const q = ((world[k]! - f.centre[0]) * f.axes[axis]![0] + (world[k + 1]! - f.centre[1]) * f.axes[axis]![1] + (world[k + 2]! - f.centre[2]) * f.axes[axis]![2]) / f.half[axis]!;
          sum += p === 2 ? q * q : p === 4 ? q * q * q * q : q * q * q * q * q * q * q * q;
        }
        if (sum > most) most = sum;
      }
      const g = Math.max(1, p === 2 ? Math.sqrt(most) : p === 4 ? Math.sqrt(Math.sqrt(most)) : Math.sqrt(Math.sqrt(Math.sqrt(most))));
      if (FILL[p] * g * g * g < least) {
        least = FILL[p] * g * g * g;
        exponent = p;
        grow = g;
      }
    }
    const radii: [number, number, number] = [f.half[0] * grow, f.half[1] * grow, f.half[2] * grow];
    const mean = (a: number, b: number): number => portable.pow(a * b, 1.6075);
    const surface = AREA[exponent] * 4 * Math.PI * portable.pow((mean(radii[0], radii[1]) + mean(radii[0], radii[2]) + mean(radii[1], radii[2])) / 3, 1 / 1.6075);
    const cover = Math.min(1, cluster.areaM2 / surface);
    const chord = (4 / 3) * portable.cbrt(radii[0] * radii[1] * radii[2]);
    const kind = cluster.foliage ? "foliage" : cover < LATTICE_COVER ? "lattice" : "solid";
    const blockage = kind === "foliage" ? 1 - portable.exp(-FOLIAGE_DRAG * FOLIAGE_AREA_DENSITY_M * chord) : kind === "lattice" ? cover : 1;
    made.push({ id: cluster.id, kind, centre: vec(f.centre), axes: [vec(f.axes[0]), vec(f.axes[1]), vec(f.axes[2])], radiiM: radii, exponent, blockage });
  }
  byScale.set(scale ?? 0, made);
  return made;
}

/** 🎞️ The obstacles between two evaluations of one body (`share` 0…1 from `from` to `to`): centres, radii and blockage are interpolated linearly, the axes linearly and made unit again; an obstacle whose fitted exponent or kind differs between the two, or lists that do not match, stay as in `from`. */
export function blendObstacles(from: readonly FlowObstacle[], to: readonly FlowObstacle[], share: number): readonly FlowObstacle[] {
  if (share <= 0 || from.length !== to.length) return from;
  const mix = (a: number, b: number): number => a + (b - a) * share;
  return from.map((a, i) => {
    const b = to[i]!;
    if (a.id !== b.id || a.exponent !== b.exponent || a.kind !== b.kind) return a;
    const axes = a.axes.map((axis, k) => {
      const other = b.axes[k]!;
      const sign = axis.x * other.x + axis.y * other.y + axis.z * other.z < 0 ? -1 : 1;
      const x = mix(axis.x, sign * other.x), y = mix(axis.y, sign * other.y), z = mix(axis.z, sign * other.z);
      const l = Math.sqrt(x * x + y * y + z * z) || 1;
      return { x: x / l, y: y / l, z: z / l };
    }) as [Vec3, Vec3, Vec3];
    return { ...a, centre: { x: mix(a.centre.x, b.centre.x), y: mix(a.centre.y, b.centre.y), z: mix(a.centre.z, b.centre.z) }, axes, radiiM: [mix(a.radiiM[0], b.radiiM[0]), mix(a.radiiM[1], b.radiiM[1]), mix(a.radiiM[2], b.radiiM[2])], blockage: mix(a.blockage, b.blockage) };
  });
}

const STRIDE = 16;
const CX = 0, A = 3, P = 12, PHI = 13, REACH2 = 14;

/** 🧱️ Obstacles and openings packed for the field: per obstacle the centre, the axes divided by their radii, the exponent, the blockage and the squared distance of its reach (`OBSTACLE_REACH` times the radius of the sphere that holds it: the largest radius, times √3 for the exponents 4 and 8); per opening the place, the radius `ρ` inside which the wall is open and `2ρ`. */
export type Walls = { readonly count: number; readonly data: Float64Array; readonly openings: Float64Array; readonly solid: boolean };

/** 🚪️ A nozzle that opens the wall around it: its place and diameter. */
export type Opening = { readonly at: Vec3; readonly diameterM: number };

const N = new Float64Array(4);

function levelAt(d: Float64Array, o: number, x: number, y: number, z: number): number {
  const dx = x - d[o + CX]!, dy = y - d[o + CX + 1]!, dz = z - d[o + CX + 2]!;
  const q0 = d[o + A]! * dx + d[o + A + 1]! * dy + d[o + A + 2]! * dz;
  const q1 = d[o + A + 3]! * dx + d[o + A + 4]! * dy + d[o + A + 5]! * dz;
  const q2 = d[o + A + 6]! * dx + d[o + A + 7]! * dy + d[o + A + 8]! * dz;
  const p = d[o + P]!;
  let s: number, g0: number, g1: number, g2: number;
  if (p === 2) {
    s = Math.sqrt(q0 * q0 + q1 * q1 + q2 * q2);
    g0 = q0; g1 = q1; g2 = q2;
  } else if (p === 4) {
    const a0 = q0 * q0, a1 = q1 * q1, a2 = q2 * q2;
    s = Math.sqrt(Math.sqrt(a0 * a0 + a1 * a1 + a2 * a2));
    g0 = a0 * q0; g1 = a1 * q1; g2 = a2 * q2;
  } else {
    const a0 = q0 * q0, a1 = q1 * q1, a2 = q2 * q2, b0 = a0 * a0, b1 = a1 * a1, b2 = a2 * a2;
    s = Math.sqrt(Math.sqrt(Math.sqrt(b0 * b0 + b1 * b1 + b2 * b2)));
    g0 = b0 * a0 * q0; g1 = b1 * a1 * q1; g2 = b2 * a2 * q2;
  }
  const nx = d[o + A]! * g0 + d[o + A + 3]! * g1 + d[o + A + 6]! * g2;
  const ny = d[o + A + 1]! * g0 + d[o + A + 4]! * g1 + d[o + A + 7]! * g2;
  const nz = d[o + A + 2]! * g0 + d[o + A + 5]! * g1 + d[o + A + 8]! * g2;
  const l = Math.sqrt(nx * nx + ny * ny + nz * nz);
  if (l > 0) {
    N[0] = nx / l; N[1] = ny / l; N[2] = nz / l;
    N[3] = p === 2 ? l / s : p === 4 ? l / (s * s * s) : l / (s * s * s * s * s * s * s);
  } else {
    N[0] = 0; N[1] = 1; N[2] = 0; N[3] = 0;
  }
  return s;
}

/** 📐️ The level `s = f(at)` of an obstacle at a point (below 1 inside, 1 on the surface), its outward unit normal there and `|∇f|` (1/m). */
export function obstacleLevel(obstacle: FlowObstacle, at: Vec3): { readonly s: number; readonly normal: Vec3; readonly gradientPerM: number } {
  const s = levelAt(packWalls([obstacle]).data, 0, at.x, at.y, at.z);
  return { s, normal: { x: N[0]!, y: N[1]!, z: N[2]! }, gradientPerM: N[3]! };
}

/** 🧱️ Packs obstacles and the openings of the instant ({@link openingsOf}); see {@link Walls}. */
export function packWalls(obstacles: readonly FlowObstacle[], openings: readonly Opening[] = []): Walls {
  const data = new Float64Array(obstacles.length * STRIDE);
  obstacles.forEach((ob, i) => {
    const o = i * STRIDE;
    data.set([ob.centre.x, ob.centre.y, ob.centre.z], o);
    ob.axes.forEach((axis, k) => data.set([axis.x / ob.radiiM[k]!, axis.y / ob.radiiM[k]!, axis.z / ob.radiiM[k]!], o + A + 3 * k));
    data[o + P] = ob.exponent;
    data[o + PHI] = ob.blockage;
    data[o + REACH2] = (ob.exponent === 2 ? 1 : 3) * (OBSTACLE_REACH * Math.max(...ob.radiiM)) ** 2;
  });
  const walls: Walls = { count: obstacles.length, data, openings: new Float64Array(0), solid: obstacles.some((ob) => ob.blockage === 1) };
  const holes = new Float64Array(openings.length * 5);
  openings.forEach((opening, i) => {
    let depth = 0;
    for (let k = 0; k < obstacles.length; k++) {
      if (obstacles[k]!.blockage !== 1) continue;
      const s = levelAt(data, k * STRIDE, opening.at.x, opening.at.y, opening.at.z);
      const c = obstacles[k]!.centre;
      if (s < 1 && s > 0) depth = Math.max(depth, Math.min((1 / s - 1) * Math.sqrt((opening.at.x - c.x) ** 2 + (opening.at.y - c.y) ** 2 + (opening.at.z - c.z) ** 2), Math.min(...obstacles[k]!.radiiM)));
    }
    const radius = Math.max(1.25 * opening.diameterM, depth + 0.75 * opening.diameterM);
    holes.set([opening.at.x, opening.at.y, opening.at.z, radius, 2 * radius], i * 5);
  });
  return { ...walls, openings: holes };
}

/** 🚪️ The openings of an instant: the nozzles of the active emitters that blow air (breath, vent, fan, exhaust, flue) and the inhaling openings. */
export function openingsOf(emitters: readonly EmitterState[], diameters: ReadonlyMap<Slug, number>, sinks: readonly Opening[] = []): readonly Opening[] {
  return [...emitters.filter((e) => e.active && e.kind !== "plume" && e.kind !== "transpiration").map((e) => ({ at: e.at, diameterM: diameters.get(e.emitter) ?? 0.02 })), ...sinks.map((s) => ({ at: s.at, diameterM: s.diameterM }))];
}

function wallShare(walls: Walls, x: number, y: number, z: number): number {
  const h = walls.openings;
  let share = 1;
  for (let i = 0; i < h.length; i += 5) {
    const dx = x - h[i]!, dy = y - h[i + 1]!, dz = z - h[i + 2]!;
    const r2 = dx * dx + dy * dy + dz * dz;
    if (r2 >= h[i + 4]! * h[i + 4]!) continue;
    const r = Math.sqrt(r2);
    if (r <= h[i + 3]!) return 0;
    const t = (r - h[i + 3]!) / (h[i + 4]! - h[i + 3]!);
    share *= t * t * (3 - 2 * t);
  }
  return share;
}

/** 🚧️ Whether a point lies inside a wall (a solid obstacle, away from every opening). */
export function insideWall(walls: Walls, x: number, y: number, z: number): boolean {
  if (!walls.solid) return false;
  const d = walls.data;
  for (let i = 0; i < walls.count; i++) {
    const o = i * STRIDE;
    if (d[o + PHI] !== 1) continue;
    const dx = x - d[o]!, dy = y - d[o + 1]!, dz = z - d[o + 2]!;
    if ((dx * dx + dy * dy + dz * dz) * (OBSTACLE_REACH * OBSTACLE_REACH) > d[o + REACH2]!) continue;
    if (levelAt(d, o, x, y, z) < 1) return wallShare(walls, x, y, z) === 1;
  }
  return false;
}

/** 🌫️ The share of the undisturbed wind that moves at a point inside porous obstacles: the product of `1 − φ` of every porous obstacle the point is in (1 outside all). */
export function porousShare(walls: Walls, x: number, y: number, z: number): number {
  const d = walls.data;
  let share = 1;
  for (let i = 0; i < walls.count; i++) {
    const o = i * STRIDE;
    if (d[o + PHI] === 1) continue;
    const dx = x - d[o]!, dy = y - d[o + 1]!, dz = z - d[o + 2]!;
    if ((dx * dx + dy * dy + dz * dz) * (OBSTACLE_REACH * OBSTACLE_REACH) > d[o + REACH2]!) continue;
    if (levelAt(d, o, x, y, z) < 1) share *= 1 - d[o + PHI]!;
  }
  return share;
}

/** 🧹️ Moves a point that lies inside a wall out along the normal to `s = 1 +` {@link OBSTACLE_SKIN} (in place, `out` = x, y, z; at most eight Newton steps over all walls) and, when `velocity` is given (offset `at` in it), removes the part of that velocity that points into the wall. With `limitM` the point moves at most that far towards where it would be put (a wall that moved over a parcel pushes it out over several steps). Returns false when the point is still inside a wall afterwards. A point within an opening is left alone. */
export function pushOut(walls: Walls, out: Float64Array | number[], velocity?: Float64Array | number[], at = 0, limitM = Infinity): boolean {
  if (!walls.solid) return true;
  const d = walls.data;
  const x0 = out[0]!, y0 = out[1]!, z0 = out[2]!;
  const limited = (free: boolean): boolean => {
    const dx = out[0]! - x0, dy = out[1]! - y0, dz = out[2]! - z0;
    const l = Math.sqrt(dx * dx + dy * dy + dz * dz);
    if (l <= limitM) return free;
    out[0] = x0 + (dx * limitM) / l;
    out[1] = y0 + (dy * limitM) / l;
    out[2] = z0 + (dz * limitM) / l;
    return false;
  };
  let moved = false;
  for (let round = 0; round < 8; round++) {
    let inside = false;
    for (let i = 0; i < walls.count; i++) {
      const o = i * STRIDE;
      if (d[o + PHI] !== 1) continue;
      const dx = out[0]! - d[o]!, dy = out[1]! - d[o + 1]!, dz = out[2]! - d[o + 2]!;
      if ((dx * dx + dy * dy + dz * dz) * (OBSTACLE_REACH * OBSTACLE_REACH) > d[o + REACH2]!) continue;
      const s = levelAt(d, o, out[0]!, out[1]!, out[2]!);
      if (s >= 1) continue;
      if (!moved && wallShare(walls, out[0]!, out[1]!, out[2]!) < 1) return true;
      inside = true;
      moved = true;
      const step = N[3]! > 0 ? (1 + OBSTACLE_SKIN - s) / N[3]! : MIN_RADIUS_M;
      out[0] = out[0]! + N[0]! * step;
      out[1] = Math.max(0, out[1]! + N[1]! * step);
      out[2] = out[2]! + N[2]! * step;
      if (velocity !== undefined) {
        const un = velocity[at]! * N[0]! + velocity[at + 1]! * N[1]! + velocity[at + 2]! * N[2]!;
        if (un < 0) {
          velocity[at] = velocity[at]! - un * N[0]!;
          velocity[at + 1] = velocity[at + 1]! - un * N[1]!;
          velocity[at + 2] = velocity[at + 2]! - un * N[2]!;
        }
      }
    }
    if (!inside) return limited(true);
  }
  return limited(!insideWall(walls, out[0]!, out[1]!, out[2]!));
}

const SHELL = new Float64Array(8 * 4);

/** 🌬️ Deflects the undisturbed velocity at a point around the obstacles (see the module text): `flow` holds the external part (wind, sinks) in 0…2 and the part of jets and plumes in 3…5 and is corrected in place. */
export function deflect(walls: Walls, x: number, y: number, z: number, flow: Float64Array): void {
  const share = wallShare(walls, x, y, z);
  const d = walls.data;
  const ex = flow[0]!, ey = flow[1]!, ez = flow[2]!, jx = flow[3]!, jy = flow[4]!, jz = flow[5]!;
  let dex = 0, dey = 0, dez = 0, djx = 0, djy = 0, djz = 0, near = 0, through = 1;
  for (let i = 0; i < walls.count; i++) {
    const o = i * STRIDE;
    const phi = d[o + PHI]!;
    const dx = x - d[o]!, dz = z - d[o + 2]!, dy = y - d[o + 1]!, my = -y - d[o + 1]!;
    const h2 = dx * dx + dz * dz;
    if (h2 + dy * dy <= d[o + REACH2]!) {
      const s = levelAt(d, o, x, y, z);
      if (s < 1 && phi < 1) through *= 1 - phi;
      else if (s <= OBSTACLE_REACH) {
        const sc = s < 1 ? 1 : s;
        const g = phi / (sc * sc * sc);
        const en = ex * N[0]! + ey * N[1]! + ez * N[2]!;
        dex += g * (0.5 * ex - 1.5 * en * N[0]!); dey += g * (0.5 * ey - 1.5 * en * N[1]!); dez += g * (0.5 * ez - 1.5 * en * N[2]!);
        const jn = jx * N[0]! + jy * N[1]! + jz * N[2]!;
        if (jn < 0) { djx -= g * jn * N[0]!; djy -= g * jn * N[1]!; djz -= g * jn * N[2]!; }
        if (phi === 1 && sc < 1 + OBSTACLE_SHELL && near < 8) {
          let at = near;
          while (at > 0 && SHELL[(at - 1) * 4 + 3]! < sc) {
            SHELL.copyWithin(at * 4, (at - 1) * 4, at * 4);
            at--;
          }
          SHELL[at * 4] = N[0]!; SHELL[at * 4 + 1] = N[1]!; SHELL[at * 4 + 2] = N[2]!; SHELL[at * 4 + 3] = sc;
          near++;
        }
      }
    }
    if (h2 + my * my <= d[o + REACH2]!) {
      const s = levelAt(d, o, x, -y, z);
      if (s <= OBSTACLE_REACH && !(s < 1 && phi < 1)) {
        const sc = s < 1 ? 1 : s;
        const g = phi / (sc * sc * sc);
        const nx = N[0]!, ny = -N[1]!, nz = N[2]!;
        const en = ex * nx + ey * ny + ez * nz;
        dex += g * (0.5 * ex - 1.5 * en * nx); dey += g * (0.5 * ey - 1.5 * en * ny); dez += g * (0.5 * ez - 1.5 * en * nz);
        const jn = jx * nx + jy * ny + jz * nz;
        if (jn < 0) { djx -= g * jn * nx; djy -= g * jn * ny; djz -= g * jn * nz; }
      }
    }
  }
  let ux = ex + share * dex, uy = ey + share * dey, uz = ez + share * dez;
  const fast = ux * ux + uy * uy + uz * uz, most = OBSTACLE_SPEED_UP * OBSTACLE_SPEED_UP * (ex * ex + ey * ey + ez * ez);
  const slow = (fast > most ? Math.sqrt(most / fast) : 1) * through;
  ux *= slow; uy *= slow; uz *= slow;
  let vx = (jx + share * djx) * through, vy = (jy + share * djy) * through, vz = (jz + share * djz) * through;
  for (let k = 0; k < near; k++) {
    const nx = SHELL[k * 4]!, ny = SHELL[k * 4 + 1]!, nz = SHELL[k * 4 + 2]!, s = SHELL[k * 4 + 3]!;
    const w = share * Math.min(1, (1 + OBSTACLE_SHELL - s) / OBSTACLE_SHELL);
    const own = 1 - 1 / (s * s * s);
    const residual = ux * nx + uy * ny + uz * nz - (ex * nx + ey * ny + ez * nz) * own * through;
    ux -= w * residual * nx; uy -= w * residual * ny; uz -= w * residual * nz;
    const jn = jx * nx + jy * ny + jz * nz;
    const left = vx * nx + vy * ny + vz * nz - (jn < 0 ? jn * own : jn) * through;
    if (left < 0) { vx -= w * left * nx; vy -= w * left * ny; vz -= w * left * nz; }
  }
  flow[0] = ux; flow[1] = y > 0 ? uy : ey * through; flow[2] = uz; flow[3] = vx; flow[4] = y > 0 ? vy : jy * through; flow[5] = vz;
}

const GOLDEN = (Math.sqrt(5) - 1) / 2;

/** 👁️ How much of a mark at a point an eye sees past the obstacles: 0 behind a wall, `1 − ` {@link OBSTACLE_VEIL}`·φ` per porous obstacle the ray to the eye crosses (or starts in), 1 in the open. The eye is a direction towards a far viewer (orthographic drawing) or a place (perspective camera). */
export function flowVisibility(obstacles: readonly FlowObstacle[], eye: FlowEye): (at: Vec3) => number {
  if (obstacles.length === 0) return () => 1;
  const walls = packWalls(obstacles);
  const d = walls.data;
  const order = obstacles.map((_, i) => i).sort((a, b) => obstacles[b]!.blockage - obstacles[a]!.blockage);
  const level = (o: number, x: number, y: number, z: number): number => {
    const dx = x - d[o]!, dy = y - d[o + 1]!, dz = z - d[o + 2]!;
    const q0 = d[o + A]! * dx + d[o + A + 1]! * dy + d[o + A + 2]! * dz;
    const q1 = d[o + A + 3]! * dx + d[o + A + 4]! * dy + d[o + A + 5]! * dz;
    const q2 = d[o + A + 6]! * dx + d[o + A + 7]! * dy + d[o + A + 8]! * dz;
    const a0 = q0 * q0, a1 = q1 * q1, a2 = q2 * q2;
    if (d[o + P] === 2) return Math.sqrt(a0 + a1 + a2);
    if (d[o + P] === 4) return Math.sqrt(Math.sqrt(a0 * a0 + a1 * a1 + a2 * a2));
    return Math.sqrt(Math.sqrt(Math.sqrt(a0 * a0 * a0 * a0 + a1 * a1 * a1 * a1 + a2 * a2 * a2 * a2)));
  };
  return (at) => {
    let ex: number, ey: number, ez: number, far = Infinity;
    if ("toward" in eye) { ex = eye.toward.x; ey = eye.toward.y; ez = eye.toward.z; }
    else { ex = eye.at.x - at.x; ey = eye.at.y - at.y; ez = eye.at.z - at.z; far = 1; }
    let seen = 1;
    for (const i of order) {
      const o = i * STRIDE;
      const dx = at.x - d[o]!, dy = at.y - d[o + 1]!, dz = at.z - d[o + 2]!;
      const q0 = d[o + A]! * dx + d[o + A + 1]! * dy + d[o + A + 2]! * dz, q1 = d[o + A + 3]! * dx + d[o + A + 4]! * dy + d[o + A + 5]! * dz, q2 = d[o + A + 6]! * dx + d[o + A + 7]! * dy + d[o + A + 8]! * dz;
      const e0 = d[o + A]! * ex + d[o + A + 1]! * ey + d[o + A + 2]! * ez, e1 = d[o + A + 3]! * ex + d[o + A + 4]! * ey + d[o + A + 5]! * ez, e2 = d[o + A + 6]! * ex + d[o + A + 7]! * ey + d[o + A + 8]! * ez;
      const ee = e0 * e0 + e1 * e1 + e2 * e2;
      if (ee === 0) continue;
      const t2 = Math.min(far, Math.max(0, -(q0 * e0 + q1 * e1 + q2 * e2) / ee));
      const c0 = q0 + t2 * e0, c1 = q1 + t2 * e1, c2 = q2 + t2 * e2;
      const nearest2 = c0 * c0 + c1 * c1 + c2 * c2;
      const limit = 1 - OCCLUSION_MARGIN;
      let hit: boolean;
      if (d[o + P] === 2) hit = nearest2 < limit * limit;
      else if (nearest2 >= 3) hit = false;
      else {
        const reach = 2 / Math.sqrt(ee);
        let lo = Math.max(0, t2 - reach), hi = Math.min(far, t2 + reach);
        const f = (t: number): number => level(o, at.x + t * ex, at.y + t * ey, at.z + t * ez);
        let m1 = hi - GOLDEN * (hi - lo), m2 = lo + GOLDEN * (hi - lo), f1 = f(m1), f2 = f(m2);
        hit = Math.min(f1, f2, f(t2)) < limit;
        for (let k = 0; k < 28 && !hit; k++) {
          if (f1 < f2) { hi = m2; m2 = m1; f2 = f1; m1 = hi - GOLDEN * (hi - lo); f1 = f(m1); }
          else { lo = m1; m1 = m2; f1 = f2; m2 = lo + GOLDEN * (hi - lo); f2 = f(m2); }
          hit = Math.min(f1, f2) < limit;
        }
      }
      if (!hit) continue;
      if (d[o + PHI] === 1) return 0;
      seen *= 1 - OBSTACLE_VEIL * d[o + PHI]!;
    }
    return seen;
  };
}
