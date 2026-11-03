/** 🦴️ Forward kinematics of an elevation rig: 2D affine helpers, joint limits and the parent-first world solve.
 *
 * Matrices follow `[a, b, c, d, e, f]` with `x' = a·x + c·y + e`, `y' = b·x + d·y + f`. A bone's local frame has its origin at the joint and +x along the bone;
 * its local transform is `translate(rest + prismatic) · rotate(restAngle + clampedOffset)` in the parent frame. Angles are degrees, counter-clockwise.
 *
 * @see ../../🧬️schema/🟦️.ts — `Rig`, `Bone`, `Joint`, `BoneOffset`
 * @see ../🔗️contracts/🟦️.ts — `Affine`, `Point`
 * @see https://www.w3.org/TR/css-transforms-1/#mathematical-description — the matrix convention
 */
import type { Bone, BoneOffset, Rig, Slug } from "../../🧬️schema/🟦️.ts";
import type { Affine, Point } from "../🔗️contracts/🟦️.ts";

/** 🟰️ The transform that changes nothing. */
export const IDENTITY: Affine = [1, 0, 0, 1, 0, 0];

/** 🔄️ Sine and cosine of an angle in degrees, exact at multiples of 90°. */
export function sinCosDeg(deg: number): { readonly sin: number; readonly cos: number } {
  const quarter = deg / 90;
  if (Number.isInteger(quarter)) {
    const k = ((quarter % 4) + 4) % 4;
    return k === 0 ? { sin: 0, cos: 1 } : k === 1 ? { sin: 1, cos: 0 } : k === 2 ? { sin: 0, cos: -1 } : { sin: -1, cos: 0 };
  }
  const rad = (deg * Math.PI) / 180;
  return { sin: Math.sin(rad), cos: Math.cos(rad) };
}

/** ↗️ A translation by (`x`, `y`) followed by a counter-clockwise rotation of `angleDeg` about the translated origin. */
export function placement(x: number, y: number, angleDeg: number): Affine {
  const { sin, cos } = sinCosDeg(angleDeg);
  return [cos, sin, -sin, cos, x, y];
}

/** ✖️ `parent × local`: applies `local` first and `parent` after it. */
export function compose(parent: Affine, local: Affine): Affine {
  return [
    parent[0] * local[0] + parent[2] * local[1],
    parent[1] * local[0] + parent[3] * local[1],
    parent[0] * local[2] + parent[2] * local[3],
    parent[1] * local[2] + parent[3] * local[3],
    parent[0] * local[4] + parent[2] * local[5] + parent[4],
    parent[1] * local[4] + parent[3] * local[5] + parent[5],
  ];
}

/** 📌️ The point (`x`, `y`) carried by the matrix. */
export function apply(matrix: Affine, x: number, y: number): Point {
  return { x: matrix[0] * x + matrix[2] * y + matrix[4], y: matrix[1] * x + matrix[3] * y + matrix[5] };
}

/** ↩️ The matrix that undoes `matrix`; throws on a singular matrix. */
export function invert(matrix: Affine): Affine {
  const det = matrix[0] * matrix[3] - matrix[1] * matrix[2];
  if (det === 0 || !Number.isFinite(det)) throw new Error("affine matrix is singular and cannot be inverted");
  return [
    matrix[3] / det,
    -matrix[1] / det,
    -matrix[2] / det,
    matrix[0] / det,
    (matrix[2] * matrix[5] - matrix[3] * matrix[4]) / det,
    (matrix[1] * matrix[4] - matrix[0] * matrix[5]) / det,
  ];
}

/** 🧭️ The rotation of the matrix in radians (`atan2(b, a)`). */
export function rotationRad(matrix: Affine): number {
  return Math.atan2(matrix[1], matrix[0]);
}

/** 🔒️ `value` limited to the optional bounds. */
export function clamp(value: number, min?: number, max?: number): number {
  const low = min === undefined ? value : Math.max(value, min);
  return max === undefined ? low : Math.min(low, max);
}

/** 🔩️ The offset a joint actually realises: revolute clamps `angleDeg` to `minDeg..maxDeg` and `abductionDeg` to ±180° and `twistDeg` to `twistMinDeg..twistMaxDeg` (default ±180°, whole turns where the joint says so); prismatic projects (`dx`, `dy`) on `axisDeg`, clamps to `minM..maxM` and returns the axis components; fixed realises nothing. */
export function effectiveOffset(bone: Bone, offset: BoneOffset | undefined): { readonly angleDeg: number; readonly abductionDeg: number; readonly twistDeg: number; readonly dx: number; readonly dy: number } {
  const joint = bone.joint;
  if (joint.type === "revolute") return { angleDeg: clamp(offset?.angleDeg ?? 0, joint.minDeg, joint.maxDeg), abductionDeg: clamp(offset?.abductionDeg ?? 0, -180, 180), twistDeg: clamp(offset?.twistDeg ?? 0, joint.twistMinDeg ?? -180, joint.twistMaxDeg ?? 180), dx: 0, dy: 0 };
  if (joint.type === "prismatic") {
    const { sin, cos } = sinCosDeg(joint.axisDeg ?? 0);
    const s = clamp((offset?.dx ?? 0) * cos + (offset?.dy ?? 0) * sin, joint.minM, joint.maxM);
    return { angleDeg: 0, abductionDeg: 0, twistDeg: 0, dx: s * cos, dy: s * sin };
  }
  return { angleDeg: 0, abductionDeg: 0, twistDeg: 0, dx: 0, dy: 0 };
}

/** 🧱️ The bone ids of the rig, parents before children; throws on a missing parent, a cycle, a missing or non-unique root, or a duplicate id. */
export function parentFirstOrder(rig: Rig): readonly Bone[] {
  const byId = new Map<Slug, Bone>();
  for (const bone of rig.bones) {
    if (byId.has(bone.id)) throw new Error(`rig has duplicate bone id "${bone.id}"`);
    byId.set(bone.id, bone);
  }
  const root = byId.get(rig.root);
  if (root === undefined) throw new Error(`rig root "${rig.root}" is not a bone`);
  if (root.parent !== null) throw new Error(`rig root "${rig.root}" must not have a parent`);
  for (const bone of rig.bones) {
    if (bone.parent === null && bone.id !== rig.root) throw new Error(`bone "${bone.id}" has no parent but is not the rig root "${rig.root}"`);
    if (bone.parent !== null && !byId.has(bone.parent)) throw new Error(`bone "${bone.id}" has missing parent "${bone.parent}"`);
  }
  const ordered: Bone[] = [];
  const state = new Map<Slug, 1 | 2>();
  const visit = (bone: Bone): void => {
    const seen = state.get(bone.id);
    if (seen === 2) return;
    if (seen === 1) throw new Error(`rig has a parent cycle through bone "${bone.id}"`);
    state.set(bone.id, 1);
    if (bone.parent !== null) visit(byId.get(bone.parent)!);
    state.set(bone.id, 2);
    ordered.push(bone);
  };
  for (const bone of rig.bones) visit(bone);
  return ordered;
}

/** 🦴️ The world affine of every bone for the offsets and the root placement (`root.x`, `root.y`, `root.angleDeg`), parents first: the orthographic side projection of the 3D chain. A lateral offset (`Bone.z`) never changes an affine, so the plain 2D solve is exact unless a bone is spread or twisted, and then the chain of {@link solveFrames} is projected. The lateral position of every bone, `Bone.z` included, is read from {@link solveChain}. Revolute offsets are clamped to the joint limits, prismatic offsets to their travel, fixed joints ignore offsets. */
export function solveRig(rig: Rig, offsets: Readonly<Record<Slug, BoneOffset>>, root: { readonly x: number; readonly y: number; readonly angleDeg: number }): Record<Slug, Affine> {
  const spread = solveFrames(rig, offsets, root);
  if (spread !== null) return Object.fromEntries(Object.entries(spread).map(([id, frame]) => [id, frame.affine]));
  const world: Record<Slug, Affine> = {};
  const origin = placement(root.x, root.y, root.angleDeg);
  for (const bone of parentFirstOrder(rig)) {
    const effective = effectiveOffset(bone, offsets[bone.id]);
    const local = placement(bone.x + effective.dx, bone.y + effective.dy, bone.angleDeg + effective.angleDeg);
    world[bone.id] = compose(bone.parent === null ? origin : world[bone.parent]!, local);
  }
  return world;
}

/** 🪽️ The lateral (left is +z) position of the origin and of the tip of a spread or twisted bone, metres. */
export type BoneSpread = { readonly originZ: number; readonly tipZ: number };

type Vec3 = readonly [number, number, number];

/** 🧊️ A bone frame in three dimensions: origin `o` and the axes `u` (along the bone), `v` (its normal in the sagittal plane at rest) and `w` (lateral, left is +), each as `[ahead, up, left]`. */
export type Frame3 = { readonly o: Vec3; readonly u: Vec3; readonly v: Vec3; readonly w: Vec3 };

/** 🪽️ The solved frame of a bone: the projected side affine, the lateral position of origin and tip, the 3D frame and whether the bone or an ancestor is spread or twisted. */
export type SolvedFrame = { readonly affine: Affine; readonly frame: Frame3; readonly affected: boolean } & BoneSpread;

const combine = (a: Vec3, s: number, b: Vec3, t: number): Vec3 => [a[0] * s + b[0] * t, a[1] * s + b[1] * t, a[2] * s + b[2] * t];

function chain(rig: Rig, offsets: Readonly<Record<Slug, BoneOffset>>, root: { readonly x: number; readonly y: number; readonly angleDeg: number }): Record<Slug, SolvedFrame> {
  const { sin: rs, cos: rc } = sinCosDeg(root.angleDeg);
  const base: Frame3 = { o: [root.x, root.y, 0], u: [rc, rs, 0], v: [-rs, rc, 0], w: [0, 0, 1] };
  const result: Record<Slug, SolvedFrame> = {};
  for (const bone of parentFirstOrder(rig)) {
    const parentSolved = bone.parent === null ? undefined : result[bone.parent]!;
    const parent = parentSolved?.frame ?? base;
    const effective = effectiveOffset(bone, offsets[bone.id]);
    const along = combine(parent.u, bone.x + effective.dx, parent.v, bone.y + effective.dy);
    const side = bone.z ?? 0;
    const origin: Vec3 = [parent.o[0] + along[0] + parent.w[0] * side, parent.o[1] + along[1] + parent.w[1] * side, parent.o[2] + along[2] + parent.w[2] * side];
    const { sin: ts, cos: tc } = sinCosDeg(bone.angleDeg + effective.angleDeg);
    const u1 = combine(parent.u, tc, parent.v, ts);
    const v1 = combine(parent.u, -ts, parent.v, tc);
    const { sin: ps, cos: pc } = sinCosDeg(effective.abductionDeg);
    const u2 = combine(u1, pc, parent.w, ps);
    const w2 = combine(u1, -ps, parent.w, pc);
    const { sin: ws, cos: wc } = sinCosDeg(effective.twistDeg);
    const frame: Frame3 = { o: origin, u: u2, v: combine(v1, wc, w2, -ws), w: combine(v1, ws, w2, wc) };
    result[bone.id] = { affine: [frame.u[0], frame.u[1], frame.v[0], frame.v[1], origin[0], origin[1]], frame, originZ: origin[2], tipZ: origin[2] + frame.u[2] * bone.lengthM, affected: (parentSolved?.affected ?? false) || effective.abductionDeg !== 0 || effective.twistDeg !== 0 };
  }
  return result;
}

/** 🪽️ The three-dimensional chain of a spread or twisted rig, or `null` when no bone is spread or twisted.
 *
 * After its own sagittal rotation about the lateral axis `w` a bone turns about its normal `v` by `abductionDeg` and then about its own axis `u` by `twistDeg` (revolute joints only; positive twist turns the `y` axis of the bone towards the right of the asset, so a vertical bone turns counter-clockwise seen from above),
 * so a point at distance `s` along it is `s·sin φ` to the left (+z) and keeps `s·cos φ` of its reach; children are placed in the spread and twisted frame of the parent (a bone may also sit beside the symmetry plane: `Bone.z`). The side view (elevation) is the orthographic projection of that chain:
 * the affine of a bone carries the projected axes `u` and `v` and the projected origin, so elevation shapes foreshorten and every consumer of {@link solveRig} (placement, mass, thermal, structure, kinematics, grounding) reads the same chain.
 * Front and plan shapes of an affected bone are carried by the full 3D transform from the rest frame ({@link restFrames}) to the current frame ({@link carry}) and projected to the view.
 *
 * @see ../../🧬️schema/🟦️.ts — `BoneOffset.abductionDeg`, `BoneOffset.twistDeg`, `Bone.z`
 */
export function solveFrames(rig: Rig, offsets: Readonly<Record<Slug, BoneOffset>>, root: { readonly x: number; readonly y: number; readonly angleDeg: number }): Record<Slug, SolvedFrame> | null {
  const moved = parentFirstOrder(rig).some((bone) => {
    const e = effectiveOffset(bone, offsets[bone.id]);
    return e.abductionDeg !== 0 || e.twistDeg !== 0;
  });
  return moved ? chain(rig, offsets, root) : null;
}

/** 🧊️ The three-dimensional chain of every rig, spread or not: the reference of the single front and plan placement. */
export function solveChain(rig: Rig, offsets: Readonly<Record<Slug, BoneOffset>>, root: { readonly x: number; readonly y: number; readonly angleDeg: number }): Record<Slug, SolvedFrame> {
  return chain(rig, offsets, root);
}

/** 🧊️ The 3D frames of the rest pose with the root at the origin: the reference of the front and plan geometry. */
export function restFrames(rig: Rig): Record<Slug, SolvedFrame> {
  return chain(rig, {}, { x: 0, y: 0, angleDeg: 0 });
}

/** ➡️ A point carried from the rest frame to the current frame of a bone: `now.o + now.M · rest.M⁻¹ · (point − rest.o)`. */
export function carry(rest: Frame3, now: Frame3, point: Vec3): [number, number, number] {
  const d: Vec3 = [point[0] - rest.o[0], point[1] - rest.o[1], point[2] - rest.o[2]];
  const a = d[0] * rest.u[0] + d[1] * rest.u[1] + d[2] * rest.u[2];
  const b = d[0] * rest.v[0] + d[1] * rest.v[1] + d[2] * rest.v[2];
  const c = d[0] * rest.w[0] + d[1] * rest.w[1] + d[2] * rest.w[2];
  return [now.o[0] + a * now.u[0] + b * now.v[0] + c * now.w[0], now.o[1] + a * now.u[1] + b * now.v[1] + c * now.w[1], now.o[2] + a * now.u[2] + b * now.v[2] + c * now.w[2]];
}
