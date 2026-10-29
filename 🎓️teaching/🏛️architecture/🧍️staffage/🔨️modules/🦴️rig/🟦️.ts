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

/** 🔩️ The offset a joint actually realises: revolute clamps `angleDeg` to `minDeg..maxDeg`; prismatic projects (`dx`, `dy`) on `axisDeg`, clamps to `minM..maxM` and returns the axis components; fixed realises nothing. */
export function effectiveOffset(bone: Bone, offset: BoneOffset | undefined): { readonly angleDeg: number; readonly dx: number; readonly dy: number } {
  const joint = bone.joint;
  if (joint.type === "revolute") return { angleDeg: clamp(offset?.angleDeg ?? 0, joint.minDeg, joint.maxDeg), dx: 0, dy: 0 };
  if (joint.type === "prismatic") {
    const { sin, cos } = sinCosDeg(joint.axisDeg ?? 0);
    const s = clamp((offset?.dx ?? 0) * cos + (offset?.dy ?? 0) * sin, joint.minM, joint.maxM);
    return { angleDeg: 0, dx: s * cos, dy: s * sin };
  }
  return { angleDeg: 0, dx: 0, dy: 0 };
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

/** 🦴️ The world affine of every bone for the offsets and the root placement (`root.x`, `root.y`, `root.angleDeg`), parents first. Revolute offsets are clamped to the joint limits, prismatic offsets to their travel, fixed joints ignore offsets. */
export function solveRig(rig: Rig, offsets: Readonly<Record<Slug, BoneOffset>>, root: { readonly x: number; readonly y: number; readonly angleDeg: number }): Record<Slug, Affine> {
  const world: Record<Slug, Affine> = {};
  const origin = placement(root.x, root.y, root.angleDeg);
  for (const bone of parentFirstOrder(rig)) {
    const effective = effectiveOffset(bone, offsets[bone.id]);
    const local = placement(bone.x + effective.dx, bone.y + effective.dy, bone.angleDeg + effective.angleDeg);
    world[bone.id] = compose(bone.parent === null ? origin : world[bone.parent]!, local);
  }
  return world;
}
