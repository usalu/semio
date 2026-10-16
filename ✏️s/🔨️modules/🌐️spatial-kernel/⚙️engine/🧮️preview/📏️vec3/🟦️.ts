/** 📏️ Vec3 arithmetic
 *
 * Component-wise `Vec3` arithmetic, normalisation and move-point constraining.
 */

// #region 🔌️Adapters
import { type Vec3 } from "@semio-tech/framework-3d-js";
// #endregion 🔌️Adapters

// #region 🧮️SpatialKernelMath
// #region 🧮️Vec
export function vec3Add(a: Vec3, b: Vec3): Vec3 {
  return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
}

/** 📏️ `a-b` component-wise for `Vec3`. */
export function vec3Sub(a: Vec3, b: Vec3): Vec3 {
  return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
}

/** 📏️ Scales a `Vec3` by scalar `s`. */
export function vec3Scale(a: Vec3, s: number): Vec3 {
  return [a[0] * s, a[1] * s, a[2] * s];
}

/** 📏️ Dot product of two `Vec3`. */
export function vec3Dot(a: Vec3, b: Vec3): number {
  return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

/** 📏️ Cross product `a×b`. */
export function vec3Cross(a: Vec3, b: Vec3): Vec3 {
  return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
}

/** 📏️ Euclidean length of `Vec3`. */
export function vec3Length(a: Vec3): number {
  return Math.hypot(a[0], a[1], a[2]);
}

/** 📏️ Euclidean distance between two `Vec3`. */
export function vec3Distance(a: Vec3, b: Vec3): number {
  return vec3Length(vec3Sub(b, a));
}

/** 📏️ Normalizes to unit length when non-zero; otherwise returns `[0,0,1]`. */
/** ↕️ Rhino Move constraint: free 3D, Vertical (CPlane Z), Normal (along `cplaneNormal`). */
export function constrainMovePoint(from: Vec3, to: Vec3, mode: string, cplaneNormal: Vec3 = [0, 0, 1]): Vec3 {
  const m = mode === "vertical" || mode === "normal" ? mode : "free";
  if (m === "free") return to;
  if (m === "vertical") return [from[0], from[1], to[2]];
  const n = vec3Normalize(cplaneNormal);
  const d = vec3Sub(to, from);
  const along = vec3Dot(d, n);
  return [from[0] + n[0] * along, from[1] + n[1] * along, from[2] + n[2] * along];
}

export function vec3Normalize(a: Vec3): Vec3 {
  const l = vec3Length(a);
  if (l < 1e-12) return [0, 0, 1];
  return [a[0] / l, a[1] / l, a[2] / l];
}
// #endregion 🧮️Vec
// #endregion 🧮️SpatialKernelMath
