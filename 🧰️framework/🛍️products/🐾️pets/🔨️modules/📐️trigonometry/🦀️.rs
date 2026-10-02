//! 📐️ Deterministic trigonometry in turns and the scalar blends every pets module shares: built from `+ − × ÷`, `abs`, `floor` and comparisons only, in the order of the TypeScript twin, so both cores yield the same bits.
//!
//! Reduction is exact: the magnitude of the angle loses its whole turns (`m − floor(m)`), the remaining phase is
//! measured from the nearest quarter turn (a residue in [−⅛, ⅛)), and only then one rounding multiplies it by 2π.
//! The kernels are the fdlibm polynomials on [−π/4, π/4]; the quarter picks the kernel and its sign. Sine is odd and
//! cosine even by construction: `sin_turns(−t) = −sin_turns(t)`, `cos_turns(−t) = cos_turns(t)`. No result is a
//! negative zero (`0.0 − x`, never `−x`). No `mul_add`, no libm call: every product and sum rounds once, as written.
//!
//! The constants are fdlibm's published decimal literals, which carry more digits than a double holds, and `TAU` is
//! spelled out like in the twin; the unit tests hold each literal to its bit pattern.
//!
//! @see <https://www.netlib.org/fdlibm/k_sin.c> — the sine kernel and its coefficients
//! @see <https://www.netlib.org/fdlibm/k_cos.c> — the cosine kernel and its coefficients
//! @see ../../🧬️schema/🦀️.rs — `Turns`
//! @see ../📐️trigonometry/🟦️.ts — the TypeScript twin
#![allow(clippy::excessive_precision, clippy::approx_constant)]

use crate::schema::Turns;

//#region 🔖️Constants
const TAU: f64 = 6.283185307179586;
const SINE_1: f64 = -1.66666666666666324348e-1;
const SINE_2: f64 = 8.33333333332248946124e-3;
const SINE_3: f64 = -1.98412698298579493134e-4;
const SINE_4: f64 = 2.75573137070700676789e-6;
const SINE_5: f64 = -2.50507602534068634195e-8;
const SINE_6: f64 = 1.58969099521155010221e-10;
const COSINE_1: f64 = 4.16666666666666019037e-2;
const COSINE_2: f64 = -1.38888888888741095749e-3;
const COSINE_3: f64 = 2.48015872894767294178e-5;
const COSINE_4: f64 = -2.75573143513906633035e-7;
const COSINE_5: f64 = 2.0875723212981748279e-9;
const COSINE_6: f64 = -1.13596475577881948265e-11;
//#endregion 🔖️Constants

//#region 🔖️Kernels
/// 🌊️ The sine of `angle` radians for |angle| ≤ π/4: `x + x³·(S1 + z·(S2 + z·(S3 + z·(S4 + z·(S5 + z·S6)))))` with `z = x²`.
fn sine_kernel(angle: f64) -> f64 {
    let square = angle * angle;
    let tail = SINE_2 + square * (SINE_3 + square * (SINE_4 + square * (SINE_5 + square * SINE_6)));
    angle + square * angle * (SINE_1 + square * tail)
}

/// 🏔️ The cosine of `angle` radians for |angle| ≤ π/4: `1 − (z/2 − z²·(C1 + z·(C2 + z·(C3 + z·(C4 + z·(C5 + z·C6))))))` with `z = x²`.
fn cosine_kernel(angle: f64) -> f64 {
    let square = angle * angle;
    let tail = COSINE_1 + square * (COSINE_2 + square * (COSINE_3 + square * (COSINE_4 + square * (COSINE_5 + square * COSINE_6))));
    1.0 - (0.5 * square - square * (square * tail))
}
//#endregion 🔖️Kernels

//#region 🔖️Turns
/// 🌀️ The sine of an angle in turns (1 = a full revolution); absolute error below 1e-15 for every finite angle.
pub fn sin_turns(turns: Turns) -> f64 {
    let magnitude = turns.abs();
    let phase = magnitude - magnitude.floor();
    let quarter = (((phase * 8.0).floor() + 1.0) * 0.5).floor();
    let angle = (phase - quarter * 0.25) * TAU;
    let value = if quarter == 1.0 {
        cosine_kernel(angle)
    } else if quarter == 2.0 {
        0.0 - sine_kernel(angle)
    } else if quarter == 3.0 {
        0.0 - cosine_kernel(angle)
    } else {
        sine_kernel(angle)
    };
    if turns < 0.0 {
        0.0 - value
    } else {
        value
    }
}

/// 🧭️ The cosine of an angle in turns (1 = a full revolution); absolute error below 1e-15 for every finite angle.
pub fn cos_turns(turns: Turns) -> f64 {
    let magnitude = turns.abs();
    let phase = magnitude - magnitude.floor();
    let quarter = (((phase * 8.0).floor() + 1.0) * 0.5).floor();
    let angle = (phase - quarter * 0.25) * TAU;
    if quarter == 1.0 {
        0.0 - sine_kernel(angle)
    } else if quarter == 2.0 {
        0.0 - cosine_kernel(angle)
    } else if quarter == 3.0 {
        sine_kernel(angle)
    } else {
        cosine_kernel(angle)
    }
}
//#endregion 🔖️Turns

//#region 🔖️Blends
/// 🗜️ `value` held inside `[low, high]`: raised to `low`, then lowered to `high`, so `high` wins when the bounds cross.
pub fn clamp(value: f64, low: f64, high: f64) -> f64 {
    let raised = if value < low { low } else { value };
    if raised > high {
        high
    } else {
        raised
    }
}

/// ↔️ The point `amount` of the way from `from` to `to`: `from + (to − from) × amount`, not clamped.
pub fn lerp(from: f64, to: f64, amount: f64) -> f64 {
    from + (to - from) * amount
}

/// 🛝️ The Hermite ease `t²·(3 − 2t)` of `amount` clamped to [0, 1]: 0 at 0, 1 at 1, flat at both ends.
pub fn smoothstep(amount: f64) -> f64 {
    let held = clamp(amount, 0.0, 1.0);
    held * held * (3.0 - 2.0 * held)
}
//#endregion 🔖️Blends

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
