/** 📐️ FBX Euler angles (rotation order XYZ, matrix `Rz * Ry * Rx`) of rotation quaternions, unwrapped to stay continuous; every other matrix and quaternion operation is the kernel's `🧮️math`.
 *
 * @see https://help.autodesk.com/view/FBX/2017/ENU/?guid=__cpp_ref_class_fbx_euler_html
 */
import type { Vec3 } from "../../../🧬️schema/🟦️.ts";
import { qNormalize } from "../../../🧮️math/🟦️.ts";
import * as portable from "../../../🧮️math/🎯️portable/🟦️.ts";

/** 📏️ Degrees per radian. */
export const DEGREES = 180 / Math.PI;

/** 🧭️ FBX Euler angles in degrees (order XYZ: the matrix is `Rz * Ry * Rx`) of a rotation quaternion. */
export function eulerDegrees(rotation: readonly number[]): Vec3 {
  const [x, y, z, w] = qNormalize(rotation);
  const r20 = 2 * (x * z - y * w);
  const r21 = 2 * (y * z + x * w);
  const r22 = 1 - 2 * (x * x + y * y);
  const r10 = 2 * (x * y + z * w);
  const r00 = 1 - 2 * (y * y + z * z);
  const b = portable.asin(Math.max(-1, Math.min(1, -r20)));
  if (Math.abs(r20) < 0.9999999) return [portable.atan2(r21, r22) * DEGREES, b * DEGREES, portable.atan2(r10, r00) * DEGREES];
  const r11 = 1 - 2 * (x * x + z * z);
  const r12 = 2 * (y * z - x * w);
  return [portable.atan2(-r12, r11) * DEGREES, b * DEGREES, 0];
}

/** 🔃️ The Euler angle triple closest to `previous`: of the triple and its equivalent `[x + 180, 180 - y, z + 180]`, each shifted by whole turns per axis, the nearer one. */
export function unwrapDegrees(angles: Vec3, previous: Vec3 | undefined): Vec3 {
  if (!previous) return angles;
  const near = (triple: readonly number[]): Vec3 => triple.map((angle, axis) => angle + 360 * Math.round((previous[axis] - angle) / 360)) as unknown as Vec3;
  const distance = (triple: Vec3): number => triple.reduce((sum, angle, axis) => sum + Math.abs(angle - previous[axis]), 0);
  const direct = near(angles);
  const alternate = near([angles[0] + 180, 180 - angles[1], angles[2] + 180]);
  return distance(alternate) < distance(direct) ? alternate : direct;
}
