/** 🧮️ Vector, quaternion and column-major 4x4 matrix helpers on plain number tuples; quaternions are `[x, y, z, w]`. No dependencies.
 * @see ../🧬️schema/🟦️.ts */
import * as portable from "./🎯️portable/🟦️.ts";
export type V3 = [number, number, number];
export type Q4 = [number, number, number, number];
export type M4 = number[];

export const EPSILON = 1e-12;
export const TAU = Math.PI * 2;

export const add = (a: readonly number[], b: readonly number[]): V3 => [a[0]! + b[0]!, a[1]! + b[1]!, a[2]! + b[2]!];
export const sub = (a: readonly number[], b: readonly number[]): V3 => [a[0]! - b[0]!, a[1]! - b[1]!, a[2]! - b[2]!];
export const mul = (a: readonly number[], s: number): V3 => [a[0]! * s, a[1]! * s, a[2]! * s];
export const dot = (a: readonly number[], b: readonly number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
export const cross = (a: readonly number[], b: readonly number[]): V3 => [a[1]! * b[2]! - a[2]! * b[1]!, a[2]! * b[0]! - a[0]! * b[2]!, a[0]! * b[1]! - a[1]! * b[0]!];
export const length = (a: readonly number[]): number => portable.hypot(a[0]!, a[1]!, a[2]!);
export const normalize = (a: readonly number[], fallback: V3 = [0, 1, 0]): V3 => {
  const l = length(a);
  return l < EPSILON ? [...fallback] : [a[0]! / l, a[1]! / l, a[2]! / l];
};
export const lerp3 = (a: readonly number[], b: readonly number[], t: number): V3 => [a[0]! + (b[0]! - a[0]!) * t, a[1]! + (b[1]! - a[1]!) * t, a[2]! + (b[2]! - a[2]!) * t];

export const qIdentity = (): Q4 => [0, 0, 0, 1];
export const qMul = (a: readonly number[], b: readonly number[]): Q4 => [
  a[3]! * b[0]! + a[0]! * b[3]! + a[1]! * b[2]! - a[2]! * b[1]!,
  a[3]! * b[1]! - a[0]! * b[2]! + a[1]! * b[3]! + a[2]! * b[0]!,
  a[3]! * b[2]! + a[0]! * b[1]! - a[1]! * b[0]! + a[2]! * b[3]!,
  a[3]! * b[3]! - a[0]! * b[0]! - a[1]! * b[1]! - a[2]! * b[2]!,
];
export const qConj = (a: readonly number[]): Q4 => [-a[0]!, -a[1]!, -a[2]!, a[3]!];
export const qDot = (a: readonly number[], b: readonly number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]! + a[3]! * b[3]!;
export const qNormalize = (a: readonly number[]): Q4 => {
  const l = portable.hypot(a[0]!, a[1]!, a[2]!, a[3]!);
  return l < EPSILON ? [0, 0, 0, 1] : [a[0]! / l, a[1]! / l, a[2]! / l, a[3]! / l];
};
export const qAxisAngle = (axis: readonly number[], angle: number): Q4 => {
  const n = normalize(axis, [0, 1, 0]);
  const s = portable.sin(angle / 2);
  return [n[0] * s, n[1] * s, n[2] * s, portable.cos(angle / 2)];
};
export const qRotate = (q: readonly number[], v: readonly number[]): V3 => {
  const u: V3 = [q[0]!, q[1]!, q[2]!];
  const t = mul(cross(u, v), 2);
  return add(add(v as V3, mul(t, q[3]!)), cross(u, t));
};
export const qSlerp = (a: readonly number[], b: readonly number[], t: number): Q4 => {
  let c = qDot(a, b);
  const sign = c < 0 ? -1 : 1;
  c *= sign;
  const bb = [b[0]! * sign, b[1]! * sign, b[2]! * sign, b[3]! * sign];
  if (c > 0.9995) return qNormalize([a[0]! + (bb[0]! - a[0]!) * t, a[1]! + (bb[1]! - a[1]!) * t, a[2]! + (bb[2]! - a[2]!) * t, a[3]! + (bb[3]! - a[3]!) * t]);
  const theta = portable.acos(Math.min(1, c));
  const s = portable.sin(theta);
  const wa = portable.sin((1 - t) * theta) / s;
  const wb = portable.sin(t * theta) / s;
  return [a[0]! * wa + bb[0]! * wb, a[1]! * wa + bb[1]! * wb, a[2]! * wa + bb[2]! * wb, a[3]! * wa + bb[3]! * wb];
};

export const identity4 = (): M4 => [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];
export const compose = (t: readonly number[], q: readonly number[], s: readonly number[]): M4 => {
  const [x, y, z, w] = [q[0]!, q[1]!, q[2]!, q[3]!];
  const [xx, yy, zz, xy, xz, yz, wx, wy, wz] = [x * x, y * y, z * z, x * y, x * z, y * z, w * x, w * y, w * z];
  return [
    (1 - 2 * (yy + zz)) * s[0]!, 2 * (xy + wz) * s[0]!, 2 * (xz - wy) * s[0]!, 0,
    2 * (xy - wz) * s[1]!, (1 - 2 * (xx + zz)) * s[1]!, 2 * (yz + wx) * s[1]!, 0,
    2 * (xz + wy) * s[2]!, 2 * (yz - wx) * s[2]!, (1 - 2 * (xx + yy)) * s[2]!, 0,
    t[0]!, t[1]!, t[2]!, 1,
  ];
};
export const multiply4 = (a: readonly number[], b: readonly number[]): M4 => {
  const out = new Array<number>(16).fill(0);
  for (let c = 0; c < 4; c++) for (let r = 0; r < 4; r++) {
    let s = 0;
    for (let k = 0; k < 4; k++) s += a[k * 4 + r]! * b[c * 4 + k]!;
    out[c * 4 + r] = s;
  }
  return out;
};
export const invert4 = (m: readonly number[]): M4 => {
  const a = m as number[];
  const b00 = a[0]! * a[5]! - a[1]! * a[4]!, b01 = a[0]! * a[6]! - a[2]! * a[4]!, b02 = a[0]! * a[7]! - a[3]! * a[4]!;
  const b03 = a[1]! * a[6]! - a[2]! * a[5]!, b04 = a[1]! * a[7]! - a[3]! * a[5]!, b05 = a[2]! * a[7]! - a[3]! * a[6]!;
  const b06 = a[8]! * a[13]! - a[9]! * a[12]!, b07 = a[8]! * a[14]! - a[10]! * a[12]!, b08 = a[8]! * a[15]! - a[11]! * a[12]!;
  const b09 = a[9]! * a[14]! - a[10]! * a[13]!, b10 = a[9]! * a[15]! - a[11]! * a[13]!, b11 = a[10]! * a[15]! - a[11]! * a[14]!;
  let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
  if (Math.abs(det) < 1e-300) return identity4();
  det = 1 / det;
  return [
    (a[5]! * b11 - a[6]! * b10 + a[7]! * b09) * det, (a[2]! * b10 - a[1]! * b11 - a[3]! * b09) * det, (a[13]! * b05 - a[14]! * b04 + a[15]! * b03) * det, (a[10]! * b04 - a[9]! * b05 - a[11]! * b03) * det,
    (a[6]! * b08 - a[4]! * b11 - a[7]! * b07) * det, (a[0]! * b11 - a[2]! * b08 + a[3]! * b07) * det, (a[14]! * b02 - a[12]! * b05 - a[15]! * b01) * det, (a[8]! * b05 - a[10]! * b02 + a[11]! * b01) * det,
    (a[4]! * b10 - a[5]! * b08 + a[7]! * b06) * det, (a[1]! * b08 - a[0]! * b10 - a[3]! * b06) * det, (a[12]! * b04 - a[13]! * b02 + a[15]! * b00) * det, (a[9]! * b02 - a[8]! * b04 - a[11]! * b00) * det,
    (a[5]! * b07 - a[4]! * b09 - a[6]! * b06) * det, (a[0]! * b09 - a[1]! * b07 + a[2]! * b06) * det, (a[13]! * b01 - a[12]! * b03 - a[14]! * b00) * det, (a[8]! * b03 - a[9]! * b01 + a[10]! * b00) * det,
  ];
};
export const transformPoint = (m: readonly number[], p: readonly number[]): V3 => [
  m[0]! * p[0]! + m[4]! * p[1]! + m[8]! * p[2]! + m[12]!,
  m[1]! * p[0]! + m[5]! * p[1]! + m[9]! * p[2]! + m[13]!,
  m[2]! * p[0]! + m[6]! * p[1]! + m[10]! * p[2]! + m[14]!,
];
export const transformDirection = (m: readonly number[], d: readonly number[]): V3 => [
  m[0]! * d[0]! + m[4]! * d[1]! + m[8]! * d[2]!,
  m[1]! * d[0]! + m[5]! * d[1]! + m[9]! * d[2]!,
  m[2]! * d[0]! + m[6]! * d[1]! + m[10]! * d[2]!,
];
export const determinant3 = (m: readonly number[]): number =>
  m[0]! * (m[5]! * m[10]! - m[9]! * m[6]!) - m[4]! * (m[1]! * m[10]! - m[9]! * m[2]!) + m[8]! * (m[1]! * m[6]! - m[5]! * m[2]!);
export const normalMatrix = (m: readonly number[]): number[] => {
  const i = invert4(m);
  return [i[0]!, i[4]!, i[8]!, i[1]!, i[5]!, i[9]!, i[2]!, i[6]!, i[10]!];
};
export const transformNormal = (nm: readonly number[], n: readonly number[]): V3 =>
  normalize([nm[0]! * n[0]! + nm[3]! * n[1]! + nm[6]! * n[2]!, nm[1]! * n[0]! + nm[4]! * n[1]! + nm[7]! * n[2]!, nm[2]! * n[0]! + nm[5]! * n[1]! + nm[8]! * n[2]!]);

/** 🌳️ World matrices of nodes keyed by id: `local(node)` composed with the world matrix of its parent (any node order; a missing parent or a chain deeper than 4096 makes the node a root). */
export const worldMatrices = <N extends { readonly id: string; readonly parent: string | null }>(nodes: readonly N[], local: (node: N) => M4): Map<string, M4> => {
  const byId = new Map(nodes.map((n) => [n.id, n]));
  const world = new Map<string, M4>();
  const resolve = (node: N, depth: number): M4 => {
    const cached = world.get(node.id);
    if (cached) return cached;
    const own = local(node);
    const parent = node.parent === null ? undefined : byId.get(node.parent);
    const result = parent && depth < 4096 ? multiply4(resolve(parent, depth + 1), own) : own;
    world.set(node.id, result);
    return result;
  };
  for (const n of nodes) resolve(n, 0);
  return world;
};

export const clamp = (x: number, lo: number, hi: number): number => Math.min(hi, Math.max(lo, x));
