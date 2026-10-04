/** 📐️ Deterministic trigonometry in turns and the scalar blends every pets module shares: built from `+ − × ÷`, `abs`, `floor` and comparisons only, so the Rust twin reproduces every result bit for bit.
 *
 * Reduction is exact: the magnitude of the angle loses its whole turns (`m − floor(m)`), the remaining phase is
 * measured from the nearest quarter turn (a residue in [−⅛, ⅛)), and only then one rounding multiplies it by 2π.
 * The kernels are the fdlibm polynomials on [−π/4, π/4]; the quarter picks the kernel and its sign. Sine is odd and
 * cosine even by construction: `sinTurns(−t) = −sinTurns(t)`, `cosTurns(−t) = cosTurns(t)`. No result is a negative zero.
 *
 * The way back is coarser on purpose: {@link atanTurns} reduces a direction to the first octant by exact operations
 * (magnitudes, one comparison, one division) and evaluates a fifth-degree odd polynomial there, which is right to
 * 2e-6 turns (0.0007°) — enough for a lean, a rope or a ladder, and the same bits in every language. {@link fastNegExp}
 * is the rational stand-in for `e⁻ˣ` wherever something saturates or dies out.
 *
 * @see https://www.netlib.org/fdlibm/k_sin.c — the sine kernel and its coefficients
 * @see https://www.netlib.org/fdlibm/k_cos.c — the cosine kernel and its coefficients
 * @see Abramowitz and Stegun, Handbook of Mathematical Functions, 4.4.49 — the arctangent polynomial and its coefficients
 * @see https://theorangeduck.com/page/spring-roll-call#exactdamper — Holden's `fast_negexp`, the rational decay
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
const ARC_1 = 0.999866;
const ARC_3 = -0.3302995;
const ARC_5 = 0.180141;
const ARC_7 = -0.085133;
const ARC_9 = 0.0208351;
const DECAY_2 = 0.48;
const DECAY_3 = 0.235;
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

/** 🏹️ The arctangent of `ratio` in radians for 0 ≤ ratio ≤ 1: `z·(A1 + z²·(A3 + z²·(A5 + z²·(A7 + z²·A9))))`, off by at most 1.15e-5 and rising all the way. */
function arcKernel(ratio: number): number {
  const square = ratio * ratio;
  return ratio * (ARC_1 + square * (ARC_3 + square * (ARC_5 + square * (ARC_7 + square * ARC_9))));
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

/** 🎯️ The direction of the point `(x, y)` seen from the origin, in turns from the positive x axis towards the positive y axis: a value in (−½, ½], off by at most 2e-6 turns; 0 for the origin itself.
 *
 * The smaller magnitude is divided by the larger one, the polynomial of that ratio is the angle to the nearer axis
 * (`¼ − …` when `|y|` is the larger), a negative `x` mirrors it (`½ − …`) and a negative `y` negates it; a result
 * that rounds to −½ is given as ½, the same direction. The direction is exactly odd in `y` wherever it is not ½
 * and exactly unchanged when both coordinates are doubled; a negative zero counts as zero, no result is a negative
 * zero, and the four axes give exactly 0, ¼, ½ and −¼. Across the four diagonals the value steps by 3.6e-6 turns,
 * which is inside the stated error. Both coordinates must be finite.
 */
export function atanTurns(y: number, x: number): Turns {
  const rise = Math.abs(y);
  const run = Math.abs(x);
  if (rise === 0 && run === 0) return 0;
  const octant = rise <= run ? arcKernel(rise / run) / TAU : 0.25 - arcKernel(run / rise) / TAU;
  const half = x < 0 ? 0.5 - octant : octant;
  const turns = y < 0 ? 0 - half : half;
  return turns <= -0.5 ? 0.5 : turns;
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

//#region 🔖️Decay
/** 📉️ A stand-in for `e⁻ˣ` without the exponential: `1 ÷ (1 + x·(1 + x·(0.48 + 0.235·x)))` for `x ≥ 0`, and 1 for everything else.
 *
 * It is 1 at 0, falls all the way and never reaches 0, so `1 − fastNegExp(x)` rises and saturates like the real
 * thing. It lies within 1.9e-2 of `e⁻ˣ` everywhere (the widest gap is near `x = 3.3`) and within 6e-4 up to
 * `x = 1`.
 */
export function fastNegExp(x: number): number {
  const held = x > 0 ? x : 0;
  return 1 / (1 + held * (1 + held * (DECAY_2 + DECAY_3 * held)));
}
//#endregion 🔖️Decay
