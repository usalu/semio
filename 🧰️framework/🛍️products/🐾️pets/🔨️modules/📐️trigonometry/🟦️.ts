/** 📐️ Deterministic trigonometry in turns and the scalar blends every pets module shares: built from `+ − × ÷`, `abs`, `floor` and comparisons only, so the Rust twin reproduces every result bit for bit.
 *
 * Reduction is exact: the magnitude of the angle loses its whole turns (`m − floor(m)`), the remaining phase is
 * measured from the nearest quarter turn (a residue in [−⅛, ⅛)), and only then one rounding multiplies it by 2π.
 * The kernels are the fdlibm polynomials on [−π/4, π/4]; the quarter picks the kernel and its sign. Sine is odd and
 * cosine even by construction: `sinTurns(−t) = −sinTurns(t)`, `cosTurns(−t) = cosTurns(t)`. No result is a negative zero.
 *
 * @see https://www.netlib.org/fdlibm/k_sin.c — the sine kernel and its coefficients
 * @see https://www.netlib.org/fdlibm/k_cos.c — the cosine kernel and its coefficients
 * @see ../../🧬️schema/🟦️.ts — `Turns`
 * @see ./🦀️.rs — the Rust twin
 */

import type { Turns } from "../../🧬️schema/🟦️.ts";

//#region 🔖️Constants
const TAU = 6.283185307179586;
const SINE_1 = -1.66666666666666324348e-1;
const SINE_2 = 8.33333333332248946124e-3;
const SINE_3 = -1.98412698298579493134e-4;
const SINE_4 = 2.75573137070700676789e-6;
const SINE_5 = -2.50507602534068634195e-8;
const SINE_6 = 1.58969099521155010221e-10;
const COSINE_1 = 4.16666666666666019037e-2;
const COSINE_2 = -1.38888888888741095749e-3;
const COSINE_3 = 2.48015872894767294178e-5;
const COSINE_4 = -2.75573143513906633035e-7;
const COSINE_5 = 2.0875723212981748279e-9;
const COSINE_6 = -1.13596475577881948265e-11;
//#endregion 🔖️Constants

//#region 🔖️Kernels
/** 🌊️ The sine of `angle` radians for |angle| ≤ π/4: `x + x³·(S1 + z·(S2 + z·(S3 + z·(S4 + z·(S5 + z·S6)))))` with `z = x²`. */
function sineKernel(angle: number): number {
  const square = angle * angle;
  const tail = SINE_2 + square * (SINE_3 + square * (SINE_4 + square * (SINE_5 + square * SINE_6)));
  return angle + square * angle * (SINE_1 + square * tail);
}

/** 🏔️ The cosine of `angle` radians for |angle| ≤ π/4: `1 − (z/2 − z²·(C1 + z·(C2 + z·(C3 + z·(C4 + z·(C5 + z·C6))))))` with `z = x²`. */
function cosineKernel(angle: number): number {
  const square = angle * angle;
  const tail = COSINE_1 + square * (COSINE_2 + square * (COSINE_3 + square * (COSINE_4 + square * (COSINE_5 + square * COSINE_6))));
  return 1 - (0.5 * square - square * (square * tail));
}
//#endregion 🔖️Kernels

//#region 🔖️Turns
/** 🌀️ The sine of an angle in turns (1 = a full revolution); absolute error below 1e-15 for every finite angle. */
export function sinTurns(turns: Turns): number {
  const magnitude = Math.abs(turns);
  const phase = magnitude - Math.floor(magnitude);
  const quarter = Math.floor((Math.floor(phase * 8) + 1) * 0.5);
  const angle = (phase - quarter * 0.25) * TAU;
  const value = quarter === 1 ? cosineKernel(angle) : quarter === 2 ? 0 - sineKernel(angle) : quarter === 3 ? 0 - cosineKernel(angle) : sineKernel(angle);
  return turns < 0 ? 0 - value : value;
}

/** 🧭️ The cosine of an angle in turns (1 = a full revolution); absolute error below 1e-15 for every finite angle. */
export function cosTurns(turns: Turns): number {
  const magnitude = Math.abs(turns);
  const phase = magnitude - Math.floor(magnitude);
  const quarter = Math.floor((Math.floor(phase * 8) + 1) * 0.5);
  const angle = (phase - quarter * 0.25) * TAU;
  return quarter === 1 ? 0 - sineKernel(angle) : quarter === 2 ? 0 - cosineKernel(angle) : quarter === 3 ? sineKernel(angle) : cosineKernel(angle);
}
//#endregion 🔖️Turns

//#region 🔖️Blends
/** 🗜️ `value` held inside `[low, high]`: raised to `low`, then lowered to `high`, so `high` wins when the bounds cross. */
export function clamp(value: number, low: number, high: number): number {
  const raised = value < low ? low : value;
  return raised > high ? high : raised;
}

/** ↔️ The point `amount` of the way from `from` to `to`: `from + (to − from) × amount`, not clamped. */
export function lerp(from: number, to: number, amount: number): number {
  return from + (to - from) * amount;
}

/** 🛝️ The Hermite ease `t²·(3 − 2t)` of `amount` clamped to [0, 1]: 0 at 0, 1 at 1, flat at both ends. */
export function smoothstep(amount: number): number {
  const held = clamp(amount, 0, 1);
  return held * held * (3 - 2 * held);
}
//#endregion 🔖️Blends
