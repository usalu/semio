//! 🥫️ Analytic results for coaxial cylindrical primitives.

use super::{empty_result, unsupported, ValueError};
use crate::BooleanOp;
use semio_framework_3d_brep_construction::primitives::SolidPrimitive;
use semio_framework_3d_brep_vector::{matrix::Frame3, Vec3};

pub fn coaxial_cylinders(a: SolidPrimitive, b: SolidPrimitive, operation: BooleanOp) -> Result<SolidPrimitive, ValueError> {
    let (SolidPrimitive::Cylinder { frame: f, radius: r, height: h }, SolidPrimitive::Cylinder { frame: g, radius: s, height: k }) = (a, b) else {
        return Err(unsupported());
    };
    let axis = if g.z == f.z { 1.0 } else if g.z == -f.z { -1.0 } else { return Err(unsupported()); };
    let delta = g.origin - f.origin;
    let position = delta.dot(f.z);
    if !position.is_finite() || delta - f.z * position != Vec3::ZERO { return Err(unsupported()); }
    let (lo, hi) = if axis > 0.0 { (position, position + k) } else { (position - k, position) };
    if !lo.is_finite() || !hi.is_finite() { return Err(unsupported()); }
    let overlap_lo = lo.max(0.0);
    let overlap_hi = hi.min(h);
    let cylinder = |start, end, radius| SolidPrimitive::Cylinder {
        frame: Frame3 { origin: f.origin + f.z * start, ..f }, radius, height: end - start,
    };
    match operation {
        BooleanOp::Intersect => {
            if overlap_hi <= overlap_lo { return Err(empty_result()); }
            Ok(cylinder(overlap_lo, overlap_hi, r.min(s)))
        }
        BooleanOp::Unite => {
            if lo >= 0.0 && hi <= h && s <= r { return Ok(a); }
            if lo <= 0.0 && hi >= h && s >= r { return Ok(b); }
            if r == s && overlap_hi >= overlap_lo { return Ok(cylinder(lo.min(0.0), hi.max(h), r)); }
            Err(unsupported())
        }
        BooleanOp::Cut => {
            if overlap_hi <= overlap_lo { return Ok(a); }
            if s < r { return Err(unsupported()); }
            if overlap_lo == 0.0 && overlap_hi == h { return Err(empty_result()); }
            if overlap_lo == 0.0 { return Ok(cylinder(overlap_hi, h, r)); }
            if overlap_hi == h { return Ok(cylinder(0.0, overlap_lo, r)); }
            Err(unsupported())
        }
    }
}
