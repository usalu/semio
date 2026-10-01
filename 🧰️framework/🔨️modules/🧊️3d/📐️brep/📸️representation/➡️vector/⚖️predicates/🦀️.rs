//! 🎯️ Robust geometric predicates: a cheap `f64` evaluation plus a conservative forward
//! error bound decides the sign whenever possible; only when the true value could be smaller
//! than the accumulated roundoff does the predicate escalate to the EXACT path below. The hard
//! invariant: a predicate here never returns a wrong sign, only (rarely) pays for a certain one.
//!
//! ⚡️ The exact path is nonoverlapping floating-point EXPANSION arithmetic (Shewchuk's error-free
//! transformations), not arbitrary-precision rationals. It was rationals until ticket
//! 26/09/09/PROCEDURAL-3D-END-TO-END profiled `🍩️sphere-cut-with-torus`: 99% of that example's
//! 61 s native evaluate sat under `mass_properties::ear_clip`, and nearly all of that self time was
//! `semio_framework_number::Natural`'s `gcd`/`normalize`/`checked_sub` plus the malloc traffic of
//! the limb vectors each `Rational` op allocates. The premise the rational path rested on — "the
//! exact path is cold" — is FALSE for any polygon produced by a dense curve discretisation, because
//! every exactly-collinear sample triple drives the filter's `None` branch deterministically, so the
//! filter never fires there and the exact path runs on EVERY call. Expansions answer the identical
//! sign (both are exact) with zero allocation and no bignum normalisation.
//!
//! @see https://www.cs.cmu.edu/~quake/robust.html — Shewchuk, "Adaptive Precision Floating-Point
//! Arithmetic and Fast Robust Geometric Predicates", the source of `two_sum`/`two_product`/
//! `fast_expansion_sum_zeroelim`/`scale_expansion_zeroelim`.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚖️predicates` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, mounted locally
//! under `➡️vector` (its sole dependency) since no target stub was pre-mounted for it.

use super::{Pnt2, Pnt3, Vec3};
use std::cmp::Ordering;

// #region 🔖️Filtered

/// 🎯️ The exact sign of a geometric test — kept distinct from [`Ordering`] so call sites read as
/// geometry (`Orient::Positive`) rather than arithmetic (`Ordering::Greater`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orient {
    Positive,
    Negative,
    Zero,
}

impl From<Ordering> for Orient {
    fn from(o: Ordering) -> Self {
        match o {
            Ordering::Greater => Orient::Positive,
            Ordering::Less => Orient::Negative,
            Ordering::Equal => Orient::Zero,
        }
    }
}

/// 🎯️ Decides the sign of `value` (a sum of `terms`) certainly, or returns `None` when
/// accumulated floating-point roundoff across `terms.len()` operations could plausibly have
/// flipped the sign — the caller must then escalate to exact arithmetic.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn filtered_sign(value: f64, terms: &[f64]) -> Option<Orient> {
    let magnitude: f64 = terms.iter().map(|t| t.abs()).sum();
    let bound = (terms.len() as f64 + 1.0) * f64::EPSILON * magnitude;
    if value > bound {
        Some(Orient::Positive)
    } else if value < -bound {
        Some(Orient::Negative)
    } else {
        None
    }
}

// #endregion 🔖️Filtered

// #region 🔖️Expansion

/// ➗️ Dekker's splitter `2^27 + 1`, which cuts a 53-bit significand into two 26-bit halves whose
/// pairwise products are each exactly representable.
const SPLITTER: f64 = 134_217_729.0;

/// ✂️ Splits `value` into a high and a low half with no rounding error.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split(value: f64) -> (f64, f64) {
    let c = SPLITTER * value;
    let big = c - value;
    let high = c - big;
    (high, value - high)
}

/// ➕️ `a + b` as an exact two-term expansion — the rounded sum and the roundoff it discarded.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let x = a + b;
    let b_virtual = x - a;
    let a_virtual = x - b_virtual;
    (x, (a - a_virtual) + (b - b_virtual))
}

/// ➕️ `a + b` when `|a| >= |b|` is already known, which saves the two virtual reconstructions.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fast_two_sum(a: f64, b: f64) -> (f64, f64) {
    let x = a + b;
    (x, b - (x - a))
}

/// ➖️ `a - b` as an exact two-term expansion.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn two_diff(a: f64, b: f64) -> [f64; 2] {
    let x = a - b;
    let b_virtual = a - x;
    let a_virtual = x + b_virtual;
    [(a - a_virtual) + (b_virtual - b), x]
}

/// ✖️ `a * b` against a pre-split `b`, as an exact two-term expansion.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn two_product_presplit(a: f64, b: f64, b_high: f64, b_low: f64) -> (f64, f64) {
    let x = a * b;
    let (a_high, a_low) = split(a);
    let error = x - a_high * b_high;
    let error = error - a_low * b_high;
    let error = error - a_high * b_low;
    (x, a_low * b_low - error)
}

/// ➕️ Exact sum of two nonoverlapping increasing-magnitude expansions into `out`, dropping zero
/// components; returns the component count written.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expansion_sum(left: &[f64], right: &[f64], out: &mut [f64]) -> usize {
    let (mut li, mut ri, mut len) = (0usize, 0usize, 0usize);
    let mut carry = 0.0;
    let mut started = false;
    while li < left.len() || ri < right.len() {
        let take_left = ri >= right.len() || (li < left.len() && (right[ri] > left[li]) == (right[ri] > -left[li]));
        let value = if take_left { left[li] } else { right[ri] };
        if take_left {
            li += 1;
        } else {
            ri += 1;
        }
        if !started {
            carry = value;
            started = true;
            continue;
        }
        let (sum, remainder) = two_sum(carry, value);
        carry = sum;
        if remainder != 0.0 {
            out[len] = remainder;
            len += 1;
        }
    }
    if carry != 0.0 || len == 0 {
        out[len] = carry;
        len += 1;
    }
    len
}

/// ✖️ Exact product of an expansion by a single `f64` into `out`, dropping zero components;
/// returns the component count written (at most `2 * expansion.len()`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn scale_expansion(expansion: &[f64], factor: f64, out: &mut [f64]) -> usize {
    let (factor_high, factor_low) = split(factor);
    let (mut carry, first) = two_product_presplit(expansion[0], factor, factor_high, factor_low);
    let mut len = 0usize;
    if first != 0.0 {
        out[len] = first;
        len += 1;
    }
    for &term in &expansion[1..] {
        let (product_high, product_low) = two_product_presplit(term, factor, factor_high, factor_low);
        let (sum, remainder) = two_sum(carry, product_low);
        if remainder != 0.0 {
            out[len] = remainder;
            len += 1;
        }
        let (next, remainder) = fast_two_sum(product_high, sum);
        carry = next;
        if remainder != 0.0 {
            out[len] = remainder;
            len += 1;
        }
    }
    if carry != 0.0 || len == 0 {
        out[len] = carry;
        len += 1;
    }
    len
}

/// ✖️ Exact product of two expansions into `out`; returns the component count written (at most
/// `2 * left.len() * right.len()`). Scratch is taken from `out`'s own tail, so the caller sizes one
/// array rather than three.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn multiply_expansion<const N: usize>(left: &[f64], right: &[f64]) -> ([f64; N], usize) {
    let mut total = [0.0; N];
    let mut total_len = 0usize;
    let mut partial = [0.0; N];
    let mut accumulator = [0.0; N];
    for &factor in right {
        let partial_len = scale_expansion(left, factor, &mut partial);
        if total_len == 0 {
            total[..partial_len].copy_from_slice(&partial[..partial_len]);
            total_len = partial_len;
            continue;
        }
        let merged = expansion_sum(&total[..total_len], &partial[..partial_len], &mut accumulator);
        total[..merged].copy_from_slice(&accumulator[..merged]);
        total_len = merged;
    }
    (total, total_len)
}

/// ➖️ Component-wise negation, which is exact and preserves the nonoverlapping ordering.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn negate_expansion<const N: usize>(expansion: &[f64]) -> [f64; N] {
    let mut out = [0.0; N];
    for (slot, &term) in out.iter_mut().zip(expansion) {
        *slot = -term;
    }
    out
}

/// 🎯️ The certain sign of a nonoverlapping increasing-magnitude expansion: its largest component
/// dominates every other, so that component's sign IS the expansion's. A non-finite leading
/// component means a caller handed a predicate a non-finite coordinate, which is a defect rather
/// than a geometric answer, so it fails loudly here instead of silently reporting `Zero`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expansion_sign(expansion: &[f64]) -> Orient {
    let leading = *expansion.last().expect("a zero-eliminated expansion always keeps one component");
    if leading > 0.0 {
        Orient::Positive
    } else if leading < 0.0 {
        Orient::Negative
    } else {
        assert!(leading == 0.0, "an exact predicate requires finite coordinates, got {leading}");
        Orient::Zero
    }
}

// #endregion 🔖️Expansion

// #region 🔖️Exact

/// 🎯️ Orientation of three 2D points: [`Orient::Positive`] when `a → b → c` turns counterclockwise,
/// [`Orient::Negative`] clockwise, [`Orient::Zero`] when collinear.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn orient2d(a: Pnt2, b: Pnt2, c: Pnt2) -> Orient {
    let acx = b.x - a.x;
    let acy = b.y - a.y;
    let bcx = c.x - a.x;
    let bcy = c.y - a.y;
    let det_left = acx * bcy;
    let det_right = acy * bcx;
    let det = det_left - det_right;
    filtered_sign(det, &[det_left, det_right]).unwrap_or_else(|| orient2d_exact(a, b, c))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn orient2d_exact(a: Pnt2, b: Pnt2, c: Pnt2) -> Orient {
    let acx = two_diff(b.x, a.x);
    let acy = two_diff(b.y, a.y);
    let bcx = two_diff(c.x, a.x);
    let bcy = two_diff(c.y, a.y);
    let (left, left_len) = multiply_expansion::<8>(&acx, &bcy);
    let (right, right_len) = multiply_expansion::<8>(&acy, &bcx);
    let negated = negate_expansion::<8>(&right[..right_len]);
    let mut det = [0.0; 16];
    let len = expansion_sum(&left[..left_len], &negated[..right_len], &mut det);
    expansion_sign(&det[..len])
}

/// 🎯️ Orientation of four 3D points via the signed volume of tetrahedron `(a,b,c,d)`, computed as
/// the scalar triple product `(b-a) · ((c-a) × (d-a))`. [`Orient::Positive`] when `(b-a,c-a,d-a)`
/// form a right-handed frame; [`Orient::Zero`] when the four points are coplanar.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn orient3d(a: Pnt3, b: Pnt3, c: Pnt3, d: Pnt3) -> Orient {
    let u = b - a;
    let v = c - a;
    let w = d - a;
    let t1 = u.x * v.y * w.z;
    let t2 = u.x * v.z * w.y;
    let t3 = u.y * v.z * w.x;
    let t4 = u.y * v.x * w.z;
    let t5 = u.z * v.x * w.y;
    let t6 = u.z * v.y * w.x;
    let det = t1 - t2 + t3 - t4 + t5 - t6;
    filtered_sign(det, &[t1, t2, t3, t4, t5, t6]).unwrap_or_else(|| orient3d_exact(a, b, c, d))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn orient3d_exact(a: Pnt3, b: Pnt3, c: Pnt3, d: Pnt3) -> Orient {
    let ux = two_diff(b.x, a.x);
    let uy = two_diff(b.y, a.y);
    let uz = two_diff(b.z, a.z);
    let vx = two_diff(c.x, a.x);
    let vy = two_diff(c.y, a.y);
    let vz = two_diff(c.z, a.z);
    let wx = two_diff(d.x, a.x);
    let wy = two_diff(d.y, a.y);
    let wz = two_diff(d.z, a.z);
    let triple = |p: &[f64; 2], q: &[f64; 2], r: &[f64; 2]| {
        let (pair, pair_len) = multiply_expansion::<8>(p, q);
        multiply_expansion::<32>(&pair[..pair_len], r)
    };
    let (t1, t1_len) = triple(&ux, &vy, &wz);
    let (t2, t2_len) = triple(&ux, &vz, &wy);
    let (t3, t3_len) = triple(&uy, &vz, &wx);
    let (t4, t4_len) = triple(&uy, &vx, &wz);
    let (t5, t5_len) = triple(&uz, &vx, &wy);
    let (t6, t6_len) = triple(&uz, &vy, &wx);
    let mut det = [0.0; 192];
    let mut scratch = [0.0; 192];
    let mut len = expansion_sum(&t1[..t1_len], &negate_expansion::<32>(&t2[..t2_len])[..t2_len], &mut det);
    for (term, term_len) in [(t3, t3_len), (negate_expansion::<32>(&t4[..t4_len]), t4_len), (t5, t5_len), (negate_expansion::<32>(&t6[..t6_len]), t6_len)] {
        let merged = expansion_sum(&det[..len], &term[..term_len], &mut scratch);
        det[..merged].copy_from_slice(&scratch[..merged]);
        len = merged;
    }
    expansion_sign(&det[..len])
}

/// 🎯️ The incircle test: [`Orient::Positive`] when `d` lies strictly inside the circle through
/// `a, b, c` (assuming `a, b, c` are given counterclockwise), [`Orient::Negative`] outside,
/// [`Orient::Zero`] on the circle.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn in_circle2d(a: Pnt2, b: Pnt2, c: Pnt2, d: Pnt2) -> Orient {
    let adx = a.x - d.x;
    let ady = a.y - d.y;
    let bdx = b.x - d.x;
    let bdy = b.y - d.y;
    let cdx = c.x - d.x;
    let cdy = c.y - d.y;
    let ad2 = adx * adx + ady * ady;
    let bd2 = bdx * bdx + bdy * bdy;
    let cd2 = cdx * cdx + cdy * cdy;
    let t1 = adx * (bdy * cd2 - cdy * bd2);
    let t2 = ady * (bdx * cd2 - cdx * bd2);
    let t3 = ad2 * (bdx * cdy - cdx * bdy);
    let det = t1 - t2 + t3;
    filtered_sign(det, &[t1, t2, t3]).unwrap_or_else(|| in_circle2d_exact(a, b, c, d))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn in_circle2d_exact(a: Pnt2, b: Pnt2, c: Pnt2, d: Pnt2) -> Orient {
    let adx = two_diff(a.x, d.x);
    let ady = two_diff(a.y, d.y);
    let bdx = two_diff(b.x, d.x);
    let bdy = two_diff(b.y, d.y);
    let cdx = two_diff(c.x, d.x);
    let cdy = two_diff(c.y, d.y);
    let square_sum = |u: &[f64; 2], v: &[f64; 2]| {
        let (uu, uu_len) = multiply_expansion::<8>(u, u);
        let (vv, vv_len) = multiply_expansion::<8>(v, v);
        let mut out = [0.0; 16];
        let len = expansion_sum(&uu[..uu_len], &vv[..vv_len], &mut out);
        (out, len)
    };
    let (ad2, ad2_len) = square_sum(&adx, &ady);
    let (bd2, bd2_len) = square_sum(&bdx, &bdy);
    let (cd2, cd2_len) = square_sum(&cdx, &cdy);
    let cross = |p: &[f64], p_len: usize, q: &[f64; 2], r: &[f64], r_len: usize, s: &[f64; 2]| {
        let (left, left_len) = multiply_expansion::<64>(&p[..p_len], q);
        let (right, right_len) = multiply_expansion::<64>(&r[..r_len], s);
        let mut out = [0.0; 128];
        let len = expansion_sum(&left[..left_len], &negate_expansion::<64>(&right[..right_len])[..right_len], &mut out);
        (out, len)
    };
    let (bc, bc_len) = cross(&cd2, cd2_len, &bdy, &bd2, bd2_len, &cdy);
    let (cb, cb_len) = cross(&cd2, cd2_len, &bdx, &bd2, bd2_len, &cdx);
    let (bcx, bcx_len) = multiply_expansion::<8>(&bdx, &cdy);
    let (cbx, cbx_len) = multiply_expansion::<8>(&cdx, &bdy);
    let mut area = [0.0; 16];
    let area_len = expansion_sum(&bcx[..bcx_len], &negate_expansion::<8>(&cbx[..cbx_len])[..cbx_len], &mut area);
    let (t1, t1_len) = multiply_expansion::<512>(&bc[..bc_len], &adx);
    let (t2, t2_len) = multiply_expansion::<512>(&cb[..cb_len], &ady);
    let (t3, t3_len) = multiply_expansion::<512>(&area[..area_len], &ad2[..ad2_len]);
    let mut partial = [0.0; 1024];
    let partial_len = expansion_sum(&t1[..t1_len], &negate_expansion::<512>(&t2[..t2_len])[..t2_len], &mut partial);
    let mut det = [0.0; 1536];
    let len = expansion_sum(&partial[..partial_len], &t3[..t3_len], &mut det);
    expansion_sign(&det[..len])
}

/// 🎯️ True when `a, b, c` are collinear within the exact predicate (i.e. `orient2d` is exactly zero).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn collinear2d(a: Pnt2, b: Pnt2, c: Pnt2) -> bool {
    orient2d(a, b, c) == Orient::Zero
}

/// 🎯️ True when `a, b, c, d` are coplanar within the exact predicate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn coplanar3d(a: Pnt3, b: Pnt3, c: Pnt3, d: Pnt3) -> bool {
    orient3d(a, b, c, d) == Orient::Zero
}

/// 🎯️ The certified sign of `u · v` — used to classify angles as acute/obtuse/right without a
/// raw `f64` comparison.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sign_of_dot(u: Vec3, v: Vec3) -> Orient {
    let tx = u.x * v.x;
    let ty = u.y * v.y;
    let tz = u.z * v.z;
    let dot = tx + ty + tz;
    filtered_sign(dot, &[tx, ty, tz]).unwrap_or_else(|| sign_of_dot_exact(u, v))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sign_of_dot_exact(u: Vec3, v: Vec3) -> Orient {
    let (tx, tx_len) = multiply_expansion::<2>(&[u.x], &[v.x]);
    let (ty, ty_len) = multiply_expansion::<2>(&[u.y], &[v.y]);
    let (tz, tz_len) = multiply_expansion::<2>(&[u.z], &[v.z]);
    let mut partial = [0.0; 4];
    let partial_len = expansion_sum(&tx[..tx_len], &ty[..ty_len], &mut partial);
    let mut dot = [0.0; 6];
    let len = expansion_sum(&partial[..partial_len], &tz[..tz_len], &mut dot);
    expansion_sign(&dot[..len])
}

// #endregion 🔖️Exact

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
