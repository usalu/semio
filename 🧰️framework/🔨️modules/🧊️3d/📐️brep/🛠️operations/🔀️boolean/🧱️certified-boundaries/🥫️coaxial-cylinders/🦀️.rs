//! 🥫️ Analytic results for coaxial cylindrical primitives.

use super::{empty_result, unsupported, ValueError};
use crate::BooleanOp;
use semio_framework_3d_brep_construction::primitives::SolidPrimitive;
use semio_framework_3d_brep_representation::contract::AnnularRevolution;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::{matrix::Frame3, Vec3};

struct CoaxialPair {
    frame: Frame3,
    stock_radius: f64,
    stock_height: f64,
    tool_radius: f64,
    tool_span: [f64; 2],
}

fn coaxial_pair(a: SolidPrimitive, b: SolidPrimitive) -> Result<CoaxialPair, ValueError> {
    let (SolidPrimitive::Cylinder { frame: f, radius: r, height: h }, SolidPrimitive::Cylinder { frame: g, radius: s, height: k }) = (a, b) else {
        return Err(unsupported());
    };
    let axis = if g.z == f.z { 1.0 } else if g.z == -f.z { -1.0 } else { return Err(unsupported()); };
    let delta = g.origin - f.origin;
    let position = delta.dot(f.z);
    if !position.is_finite() || delta - f.z * position != Vec3::ZERO { return Err(unsupported()); }
    let (lo, hi) = if axis > 0.0 { (position, position + k) } else { (position - k, position) };
    if !lo.is_finite() || !hi.is_finite() { return Err(unsupported()); }
    Ok(CoaxialPair { frame: f, stock_radius: r, stock_height: h, tool_radius: s, tool_span: [lo, hi] })
}

/// 🕳️ Certifies a single smaller tool on the stock axis.
pub fn coaxial_cylinder_bore(a: SolidPrimitive, b: SolidPrimitive) -> Result<AnnularRevolution, ValueError> {
    let pair = coaxial_pair(a,b)?;
    AnnularRevolution::from_axial_cut(
        pair.frame,pair.tool_radius,pair.stock_radius,pair.stock_height,pair.tool_span,Tol::DEFAULT.value(),
    ).ok_or_else(unsupported)
}

/// 🔗️ Certifies connected stepped unions between coaxial cylinders of unequal radii.
pub fn coaxial_cylinder_step(a: SolidPrimitive,b: SolidPrimitive)->Result<AnnularRevolution,ValueError>{
    let mut pair=coaxial_pair(a,b)?;
    if pair.stock_radius<pair.tool_radius{pair=coaxial_pair(b,a)?;}
    AnnularRevolution::from_axial_union(
        pair.frame,pair.tool_radius,pair.stock_radius,pair.stock_height,pair.tool_span,Tol::DEFAULT.value(),
    ).ok_or_else(unsupported)
}

pub fn coaxial_cylinders(a: SolidPrimitive, b: SolidPrimitive, operation: BooleanOp) -> Result<SolidPrimitive, ValueError> {
    let CoaxialPair { frame: f, stock_radius: r, stock_height: h, tool_radius: s, tool_span: [lo, hi] } = coaxial_pair(a,b)?;
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
