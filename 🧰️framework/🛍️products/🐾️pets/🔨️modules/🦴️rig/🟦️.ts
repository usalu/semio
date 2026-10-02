/** 🦴️ Forward kinematics of a species rig: 2×3 affine matrices in the SVG `matrix(a b c d e f)` convention (`x′ = a·x + c·y + e`, `y′ = b·x + d·y + f`), poses relative to the rest pose, and the gaze direction of an eye.
 *
 * Every expression is written once and in a fixed order (no reassociation), with sine and cosine from
 * `📐️trigonometry`, so the Rust twin yields the same bits. `solveRig` multiplies with the expressions of
 * {@link compose}, term for term, in place (no matrix is allocated per bone). {@link compose}, {@link invert} and
 * {@link IDENTITY} are API for whoever works with the solved matrices — a tool that places something relative to a
 * bone, a test that checks the solver against a third party —; the stage itself does not call them.
 *
 * @see https://www.w3.org/TR/css-transforms-1/#mathematical-description — the matrix convention
 * @see ../📐️trigonometry/🟦️.ts — `sinTurns`, `cosTurns`
 * @see ../../🧬️schema/🟦️.ts — `Species`, `Bone`, `Eye`, `Point`, `Degrees`
 * @see ./🦀️.rs — the Rust twin
 */

import type { Bone, Degrees, Eye, Point, Species } from "../../🧬️schema/🟦️.ts";
import { cosTurns, sinTurns } from "../📐️trigonometry/🟦️.ts";

const PUPIL_RIM = 0.25;

//#region 🔖️Affine
/** 🔳️ A 2×3 affine matrix `[a, b, c, d, e, f]`: the columns `(a, b)` and `(c, d)` carry rotation and scale, `(e, f)` the translation. */
export type Affine = readonly [number, number, number, number, number, number];

/** 🟰️ The matrix that changes nothing. */
export const IDENTITY: Affine = [1, 0, 0, 1, 0, 0];

/** ✖️ `parent × local`: the matrix that applies `local` first and `parent` after it. */
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

/** ↩️ The matrix that undoes `matrix` (adjugate ÷ determinant `a·d − b·c`); a matrix without an inverse (determinant 0) yields {@link IDENTITY}, so the result is always finite. */
export function invert(matrix: Affine): Affine {
  const determinant = matrix[0] * matrix[3] - matrix[1] * matrix[2];
  if (determinant === 0) return IDENTITY;
  return [
    matrix[3] / determinant,
    (0 - matrix[1]) / determinant,
    (0 - matrix[2]) / determinant,
    matrix[0] / determinant,
    (matrix[2] * matrix[5] - matrix[3] * matrix[4]) / determinant,
    (matrix[1] * matrix[4] - matrix[0] * matrix[5]) / determinant,
  ];
}

/** 📌️ The point `(x, y)` carried by `matrix`. */
export function transform(matrix: Affine, x: number, y: number): Point {
  return { x: matrix[0] * x + matrix[2] * y + matrix[4], y: matrix[1] * x + matrix[3] * y + matrix[5] };
}
//#endregion 🔖️Affine

//#region 🔖️Pose
/** 🤸️ What a bone adds to its rest transform: offsets in pixels, an offset in degrees and scale factors (rest = 0, 0, 0, 1, 1). */
export type BonePose = { readonly x: number; readonly y: number; readonly rotation: Degrees; readonly scaleX: number; readonly scaleY: number };

/** 🧍️ One {@link BonePose} per bone, in rig order. */
export type Pose = readonly BonePose[];

const REST: BonePose = { x: 0, y: 0, rotation: 0, scaleX: 1, scaleY: 1 };

/** 🛌️ The pose in which every bone stays at rest: no offset, no rotation, unit scale. */
export function restPose(species: Species): Pose {
  return species.bones.map(() => REST);
}
//#endregion 🔖️Pose

//#region 🔖️Solving
/** 👪️ The index of the bone's parent among the bones listed before it; −1 for a root (no parent, or none listed earlier). */
function parentIndex(bones: readonly Bone[], index: number): number {
  const parent = bones[index]!.parent;
  if (parent === undefined) return -1;
  for (let candidate = index - 1; candidate >= 0; candidate--) if (bones[candidate]!.id === parent) return candidate;
  return -1;
}

/** 🩻️ The world matrix of every bone relative to the feet origin, six numbers `a b c d e f` per bone in rig order.
 *
 * `local = translate(bone.x + pose.x, bone.y + pose.y) × rotate((bone.rotation + pose.rotation) ÷ 360 turns) × scale(pose.scaleX, pose.scaleY)`,
 * `world = parent world × local` (a root's world matrix is its local one). Bones the pose does not cover stay at rest.
 */
export function solveRig(species: Species, pose: Pose): number[] {
  const bones = species.bones;
  const world: number[] = [];
  for (let index = 0; index < bones.length; index++) {
    const bone = bones[index]!;
    const posed = index < pose.length ? pose[index]! : REST;
    const turns = ((bone.rotation === undefined ? 0 : bone.rotation) + posed.rotation) / 360;
    const sine = sinTurns(turns);
    const cosine = cosTurns(turns);
    const a = cosine * posed.scaleX;
    const b = sine * posed.scaleX;
    const c = (0 - sine) * posed.scaleY;
    const d = cosine * posed.scaleY;
    const e = bone.x + posed.x;
    const f = bone.y + posed.y;
    const parent = parentIndex(bones, index);
    if (parent < 0) {
      world.push(a, b, c, d, e, f);
      continue;
    }
    const offset = parent * 6;
    const pa = world[offset]!;
    const pb = world[offset + 1]!;
    const pc = world[offset + 2]!;
    const pd = world[offset + 3]!;
    const pe = world[offset + 4]!;
    const pf = world[offset + 5]!;
    world.push(pa * a + pc * b, pb * a + pd * b, pa * c + pc * d, pb * c + pd * d, pa * e + pc * f + pe, pb * e + pd * f + pf);
  }
  return world;
}
//#endregion 🔖️Solving

//#region 🔖️Gaze
/** 👀️ Where a pupil is drawn towards: the direction from `eye` to `target`, scaled to the length `d ÷ (d + reach)` for the distance `d`, so the result lies inside the unit disc (on its rim only when `reach` is not positive); `(0, 0)` when the points coincide. */
export function lookOffset(eye: Point, target: Point, reach: number): Point {
  const dx = target.x - eye.x;
  const dy = target.y - eye.y;
  const distance = Math.sqrt(dx * dx + dy * dy);
  if (distance === 0) return { x: 0, y: 0 };
  const span = distance + (reach > 0 ? reach : 0);
  return { x: dx / span, y: dy / span };
}

/** 👁️ How far the pupil of an eye travels from the centre of its white, in pixels: until it reaches the outline — `radius − pupil − 0.25`, the quarter pixel keeping it off the middle of the stroke —, never less than 0. A gaze between −1 and 1 times this is where a pupil is drawn. */
export function pupilReach(eye: Eye): number {
  const reach = eye.radius - eye.pupil - PUPIL_RIM;
  return reach > 0 ? reach : 0;
}
//#endregion 🔖️Gaze
