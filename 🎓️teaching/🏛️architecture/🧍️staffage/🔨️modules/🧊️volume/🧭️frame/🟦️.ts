/** 🧭️ The one conversion between the staffage world and the neutral model frame.
 *
 * Staffage world: x ahead, y up, z left, a left-handed triple. Neutral model: right-handed, +Y up, +Z ahead, +X the asset's left, so a point `(x, y, z)` becomes `(z, y, x)`: one reflection, which turns every pseudo-vector (a moment, a rotation axis) into its negated, component-swapped image
 * and every triangle winding inside out. A bone frame `(u, v, w)` is a rotation in the staffage numbers, so its axes cannot be a neutral node basis; the neutral node frame of a bone is `(u, v, -w)` (right-handed again), and a volume solid, written in `(u, v, w)`, is carried into it by mirroring `w`
 * ({@link mirrorTransform}, {@link mirrorRepeat}), which the kernel tessellator turns into correctly wound triangles.
 *
 * @see ../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧬️schema/🟦️.ts — the neutral frame
 */
import type { Quaternion, Vector3, VolumeRepeat, VolumeTransform } from "../../../🧬️schema/🟦️.ts";
import * as portable from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 📐️ A triple in the neutral frame. */
export type Triple = readonly [number, number, number];
/** 🔄️ A unit quaternion `[x, y, z, w]` of the neutral frame. */
export type Quat = readonly [number, number, number, number];
/** 🧮️ A 3×3 rotation matrix of the neutral frame as three column vectors. */
export type Basis = readonly [Triple, Triple, Triple];
/** 🧊️ A world frame in the staffage world as numbers `(ahead, up, left)`. */
export type WorldFrame = { readonly o: Triple; readonly u: Triple; readonly v: Triple; readonly w: Triple };

/** 📍️ A staffage point or vector `(ahead, up, left)` in the neutral frame `(left, up, ahead)`. */
export const toNeutral = (p: Triple): Triple => [p[2], p[1], p[0]];

/** 🔃️ A staffage pseudo-vector (moment, rotation axis) in the neutral frame: the reflection negates it. */
export const pseudoToNeutral = (p: Triple): Triple => [-p[2], -p[1], -p[0]];

/** 🧱️ The neutral basis of a staffage bone frame: the node axes are `(u, v, -w)`, mapped into the neutral frame. */
export function basisOf(frame: WorldFrame): Basis {
  const w = toNeutral(frame.w);
  return [toNeutral(frame.u), toNeutral(frame.v), [-w[0], -w[1], -w[2]]];
}

const transpose = (b: Basis): Basis => [
  [b[0][0], b[1][0], b[2][0]],
  [b[0][1], b[1][1], b[2][1]],
  [b[0][2], b[1][2], b[2][2]],
];

/** ✖️ The basis `a · b` as column vectors. */
export function multiply(a: Basis, b: Basis): Basis {
  const apply = (v: Triple): Triple => [a[0][0] * v[0] + a[1][0] * v[1] + a[2][0] * v[2], a[0][1] * v[0] + a[1][1] * v[1] + a[2][1] * v[2], a[0][2] * v[0] + a[1][2] * v[1] + a[2][2] * v[2]];
  return [apply(b[0]), apply(b[1]), apply(b[2])];
}

/** 🔁️ The inverse of a rotation basis (its transpose). */
export const inverseBasis = (b: Basis): Basis => transpose(b);

/** 🧪️ The vector `v` rotated by the basis `b`. */
export const rotate = (b: Basis, v: Triple): Triple => [b[0][0] * v[0] + b[1][0] * v[1] + b[2][0] * v[2], b[0][1] * v[0] + b[1][1] * v[1] + b[2][1] * v[2], b[0][2] * v[0] + b[1][2] * v[1] + b[2][2] * v[2]];

/** 🌀️ The unit quaternion `[x, y, z, w]` (w ≥ 0) of a right-handed rotation basis (Shepperd's method). */
export function quaternionOf(b: Basis): Quat {
  const [m00, m10, m20] = b[0];
  const [m01, m11, m21] = b[1];
  const [m02, m12, m22] = b[2];
  const trace = m00 + m11 + m22;
  let x: number, y: number, z: number, w: number;
  if (trace > 0) {
    const s = Math.sqrt(trace + 1) * 2;
    w = s / 4;
    x = (m21 - m12) / s;
    y = (m02 - m20) / s;
    z = (m10 - m01) / s;
  } else if (m00 > m11 && m00 > m22) {
    const s = Math.sqrt(1 + m00 - m11 - m22) * 2;
    w = (m21 - m12) / s;
    x = s / 4;
    y = (m01 + m10) / s;
    z = (m02 + m20) / s;
  } else if (m11 > m22) {
    const s = Math.sqrt(1 + m11 - m00 - m22) * 2;
    w = (m02 - m20) / s;
    x = (m01 + m10) / s;
    y = s / 4;
    z = (m12 + m21) / s;
  } else {
    const s = Math.sqrt(1 + m22 - m00 - m11) * 2;
    w = (m10 - m01) / s;
    x = (m02 + m20) / s;
    y = (m12 + m21) / s;
    z = s / 4;
  }
  const n = portable.hypot(x, y, z, w) || 1;
  const sign = w < 0 ? -1 : 1;
  return [(x / n) * sign, (y / n) * sign, (z / n) * sign, (w / n) * sign];
}

/** 🎯️ The right-handed basis whose Y axis is `direction` (unit), its X axis chosen stably: the node frame of an arrow or an arc whose shape lies along +Y. */
export function basisAlongY(direction: Triple): Basis {
  const y = direction;
  const pick: Triple = Math.abs(y[0]) < 0.9 ? [1, 0, 0] : [0, 0, 1];
  const dot = pick[0] * y[0] + pick[1] * y[1] + pick[2] * y[2];
  const xRaw: Triple = [pick[0] - dot * y[0], pick[1] - dot * y[1], pick[2] - dot * y[2]];
  const n = portable.hypot(...xRaw) || 1;
  const x: Triple = [xRaw[0] / n, xRaw[1] / n, xRaw[2] / n];
  const z: Triple = [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]];
  return [x, y, z];
}

/** 🎡️ The product of two quaternions `a · b` (apply `b` first). */
export function multiplyQuaternions(a: Quat, b: Quat): Quat {
  return [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]];
}

/** 🪞️ A vector of the bone-local `(u, v, w)` frame in the node frame `(u, v, -w)`. */
export const mirrorW = (p: readonly number[]): Vector3 => [p[0]!, p[1]!, -p[2]!];

/** 📏️ The solid transform `Mirror · T` as a transform: translation `(t0, t1, -t2)`, rotation `(-qx, -qy, qz, qw)`, scale `(s0, s1, -s2)` (a negative scale; the kernel re-winds the triangles). */
export function mirrorTransform(transform: VolumeTransform | undefined): { readonly translation: Vector3; readonly rotation: Quaternion; readonly scale: Vector3 } {
  const t = transform?.translation ?? [0, 0, 0];
  const q = transform?.rotation ?? [0, 0, 0, 1];
  const s = transform?.scale ?? [1, 1, 1];
  return { translation: [t[0], t[1], -t[2]], rotation: [-q[0], -q[1], q[2], q[3]], scale: [s[0], s[1], -s[2]] };
}

/** 🦴️ The repeat step `Mirror · step · Mirror`: the translation and the rotation origin mirrored, the rotation axis mirrored and the angle negated. */
export function mirrorRepeat(repeat: VolumeRepeat | undefined): VolumeRepeat | undefined {
  if (repeat === undefined) return undefined;
  return {
    count: repeat.count,
    ...(repeat.translation === undefined ? {} : { translation: mirrorW(repeat.translation) }),
    ...(repeat.rotationAxis === undefined ? {} : { rotationAxis: mirrorW(repeat.rotationAxis) }),
    ...(repeat.rotationRad === undefined ? {} : { rotationRad: -repeat.rotationRad }),
    ...(repeat.origin === undefined ? {} : { origin: mirrorW(repeat.origin) }),
  };
}
