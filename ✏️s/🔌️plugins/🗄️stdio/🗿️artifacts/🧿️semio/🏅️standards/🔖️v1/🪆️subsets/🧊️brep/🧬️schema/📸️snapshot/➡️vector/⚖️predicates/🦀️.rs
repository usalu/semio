//! 🎯️ Robust geometric predicates: a cheap `f64` evaluation plus a conservative forward
//! error bound decides the sign whenever possible; only when the true value could be smaller
//! than the accumulated roundoff does the predicate escalate to exact [`semio_framework_number::Rational`]
//! arithmetic (lossless for any finite `f64`, per `Rational::from_f64`). This is deliberately
//! simpler than Shewchuk-style adaptive expansions — the exact path is cold, so raw simplicity
//! beats squeezing out its last microsecond. The hard invariant: a predicate here never returns a
//! wrong sign, only (rarely) pays for a certain one.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚖️predicates` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, mounted locally
//! under `➡️vector` (its sole dependency) since no target stub was pre-mounted for it.

use super::{Pnt2, Pnt3, Vec3};
use semio_framework_number::Rational;
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn to_rational(v: f64) -> Rational {
    Rational::from_f64(v).expect("finite f64 is always exactly representable as a Rational")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rational_sign(v: &Rational) -> Orient {
    Orient::from(v.cmp(&Rational::zero()))
}

// #endregion 🔖️Filtered

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
    let (ax, ay) = (to_rational(a.x), to_rational(a.y));
    let (bx, by) = (to_rational(b.x), to_rational(b.y));
    let (cx, cy) = (to_rational(c.x), to_rational(c.y));
    let acx = bx.sub(&ax);
    let acy = by.sub(&ay);
    let bcx = cx.sub(&ax);
    let bcy = cy.sub(&ay);
    let det = acx.mul(&bcy).sub(&acy.mul(&bcx));
    rational_sign(&det)
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
    let ax = to_rational(a.x);
    let ay = to_rational(a.y);
    let az = to_rational(a.z);
    let ux = to_rational(b.x).sub(&ax);
    let uy = to_rational(b.y).sub(&ay);
    let uz = to_rational(b.z).sub(&az);
    let vx = to_rational(c.x).sub(&ax);
    let vy = to_rational(c.y).sub(&ay);
    let vz = to_rational(c.z).sub(&az);
    let wx = to_rational(d.x).sub(&ax);
    let wy = to_rational(d.y).sub(&ay);
    let wz = to_rational(d.z).sub(&az);
    let t1 = ux.mul(&vy).mul(&wz);
    let t2 = ux.mul(&vz).mul(&wy);
    let t3 = uy.mul(&vz).mul(&wx);
    let t4 = uy.mul(&vx).mul(&wz);
    let t5 = uz.mul(&vx).mul(&wy);
    let t6 = uz.mul(&vy).mul(&wx);
    let det = t1.sub(&t2).add(&t3).sub(&t4).add(&t5).sub(&t6);
    rational_sign(&det)
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
    let dx = to_rational(d.x);
    let dy = to_rational(d.y);
    let adx = to_rational(a.x).sub(&dx);
    let ady = to_rational(a.y).sub(&dy);
    let bdx = to_rational(b.x).sub(&dx);
    let bdy = to_rational(b.y).sub(&dy);
    let cdx = to_rational(c.x).sub(&dx);
    let cdy = to_rational(c.y).sub(&dy);
    let ad2 = adx.mul(&adx).add(&ady.mul(&ady));
    let bd2 = bdx.mul(&bdx).add(&bdy.mul(&bdy));
    let cd2 = cdx.mul(&cdx).add(&cdy.mul(&cdy));
    let t1 = adx.mul(&bdy.mul(&cd2).sub(&cdy.mul(&bd2)));
    let t2 = ady.mul(&bdx.mul(&cd2).sub(&cdx.mul(&bd2)));
    let t3 = ad2.mul(&bdx.mul(&cdy).sub(&cdx.mul(&bdy)));
    let det = t1.sub(&t2).add(&t3);
    rational_sign(&det)
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
    let tx = to_rational(u.x).mul(&to_rational(v.x));
    let ty = to_rational(u.y).mul(&to_rational(v.y));
    let tz = to_rational(u.z).mul(&to_rational(v.z));
    let dot = tx.add(&ty).add(&tz);
    rational_sign(&dot)
}

// #endregion 🔖️Exact

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
