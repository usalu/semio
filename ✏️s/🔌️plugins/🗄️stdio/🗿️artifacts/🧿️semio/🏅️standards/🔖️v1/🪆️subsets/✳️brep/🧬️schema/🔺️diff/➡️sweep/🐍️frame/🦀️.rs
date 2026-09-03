//! 🐍 Rotation-minimizing frames along a path `Wire`, by the double-reflection method (Wang et
//! al. 2008): adaptively samples each path edge (denser where curvature is higher, bounded by
//! `max_stations`), propagates an initial perpendicular axis frame-to-frame with zero unnecessary
//! twist, and — when a guide wire is supplied — overrides each station's own in-plane axis to
//! point at the guide's closest point instead, per the ticket's "frame's x-axis points at the
//! guide" instruction. Feeds `➡️sweep`'s general (non-line/non-arc) sweep/pipe/helix path.
//!
//! Mounted as a submodule of `➡️sweep` in ticket 26/09/03/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME wave
//! W2-C via `#[path]` from `➡️sweep/🦀️.rs`.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::Wire;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::curve_ops::closest_parameter;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::KernelError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

/// 🐍 One sampled path station: world point, unit tangent, and cumulative arc length from the
/// path's start (the natural, path-length "station scalar" every rail/lateral fit in the general
/// sweep chain is built against).
pub(super) struct Station {
    pub point: Pnt3,
    pub tangent: Vec3,
    pub length: f64,
}

/// 🐍 Samples `📡️wire`'s edges into stations: `min_per_edge` uniform samples, doubled wherever the
/// curve's own `curvature(t)` exceeds `curvature_gate` (a bounded stand-in for a fully certified
/// chordal-deviation refinement — see `📓️w2c-sweeps.md` §sweep for the scope this pass covers).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn sample_path(body: &Body, wire: &Wire, min_per_edge: usize, max_per_edge: usize) -> Result<Vec<Station>, KernelError> {
    let mut stations = Vec::new();
    let mut length = 0.0;
    let mut prev: Option<Pnt3> = None;
    for &(edge_id, forward) in &wire.members {
        let edge = body.edges.get(edge_id).ok_or_else(|| KernelError::MissingEntity(format!("edge {edge_id}")))?;
        let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
        let (a, b) = edge.range;
        let high_curvature = (0..5).any(|i| { let t = a + (b - a) * i as f64 / 4.0; curve.curvature(t) > 0.5 });
        let steps = if high_curvature { max_per_edge } else { min_per_edge };
        for i in 0..=steps {
            let s = i as f64 / steps as f64;
            let t = if forward { a + (b - a) * s } else { b + (a - b) * s };
            let point = curve.eval(t);
            let raw_tangent = curve.d1(t).normalized().unwrap_or(Vec3::X);
            let tangent = if forward { raw_tangent } else { -raw_tangent };
            if let Some(p) = prev {
                length += (point - p).norm();
            }
            if prev.map(|p| (point - p).norm() > 1e-12).unwrap_or(true) {
                stations.push(Station { point, tangent, length });
                prev = Some(point);
            }
        }
    }
    if stations.len() < 2 {
        return Err(KernelError::InvalidInput("sweep path needs at least two distinct stations".into()));
    }
    Ok(stations)
}

/// 🐍 One rotation-minimizing frame: `origin`, unit `tangent` (=`z`), unit `x` (perpendicular to
/// `tangent`, propagated with minimal twist), `y = tangent × x`.
pub(super) struct RmfFrame {
    pub origin: Pnt3,
    pub tangent: Vec3,
    pub x: Vec3,
}

impl RmfFrame {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn frame3(&self) -> Frame3 {
        Frame3 { origin: self.origin, x: self.x, y: self.tangent.cross(self.x), z: self.tangent }
    }
}

/// 🐍 Double-reflection rotation-minimizing propagation (Wang et al. 2008, Algorithm 1): `r0` is
/// any unit vector ⟂ the first station's tangent.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn propagate_rmf(stations: &[Station], r0: Vec3) -> Vec<RmfFrame> {
    let mut out = Vec::with_capacity(stations.len());
    out.push(RmfFrame { origin: stations[0].point, tangent: stations[0].tangent, x: r0 });
    for i in 1..stations.len() {
        let prev = &out[i - 1];
        let p0 = prev.origin;
        let p1 = stations[i].point;
        let t0 = prev.tangent;
        let t1 = stations[i].tangent;
        let v1 = p1 - p0;
        let c1 = v1.dot(v1);
        if c1 < 1e-20 {
            out.push(RmfFrame { origin: p1, tangent: t1, x: prev.x });
            continue;
        }
        let r_l = prev.x - v1 * (2.0 / c1 * v1.dot(prev.x));
        let t_l = t0 - v1 * (2.0 / c1 * v1.dot(t0));
        let v2 = t1 - t_l;
        let c2 = v2.dot(v2);
        let r1 = if c2 < 1e-20 { r_l } else { r_l - v2 * (2.0 / c2 * v2.dot(r_l)) };
        let r1 = (r1 - t1 * r1.dot(t1)).normalized().unwrap_or(r_l.normalized().unwrap_or(prev.x));
        out.push(RmfFrame { origin: p1, tangent: t1, x: r1 });
    }
    out
}

/// 🐍 Direct RMF frames from precomputed stations (no `Wire`/`Body` needed) — used by
/// `helical_sweep`'s analytic helix parametrization.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn stations_to_frames(stations: &[Station]) -> Vec<Frame3> {
    let r0 = stations[0].tangent.any_orthogonal();
    propagate_rmf(stations, r0).iter().map(RmfFrame::frame3).collect()
}

/// 🐍 Builds the station frames for a sweep/pipe along `path`: rotation-minimizing by default;
/// when `guide` is present, each station's `x` is instead re-pointed at the guide's own closest
/// point (Gram-Schmidt-orthogonalized against the tangent), per the ticket's guide-honouring
/// instruction — the RMF propagation still seeds the very first station's axis.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn frame_stations(body: &Body, path: &Wire, guide: Option<&Wire>, min_per_edge: usize, max_per_edge: usize) -> Result<Vec<Frame3>, KernelError> {
    let stations = sample_path(body, path, min_per_edge, max_per_edge)?;
    let r0 = stations[0].tangent.any_orthogonal();
    let frames = propagate_rmf(&stations, r0);
    let Some(guide) = guide else {
        return Ok(frames.iter().map(RmfFrame::frame3).collect());
    };
    let guide_edges: Vec<_> = guide.members.iter().map(|&(e, _)| e).collect();
    let mut out = Vec::with_capacity(frames.len());
    for f in &frames {
        let mut best: Option<Pnt3> = None;
        let mut best_d = f64::INFINITY;
        for &eid in &guide_edges {
            let edge = body.edges.get(eid).ok_or_else(|| KernelError::MissingEntity("guide edge".into()))?;
            let curve = body.curves3.get(edge.curve).ok_or_else(|| KernelError::MissingEntity("guide curve".into()))?;
            let cp = closest_parameter(curve, edge.range, f.origin, 1e-9);
            if cp.distance < best_d {
                best_d = cp.distance;
                best = Some(cp.point);
            }
        }
        let toward = best.map(|p| p - f.origin).unwrap_or(f.x);
        let projected = toward - f.tangent * toward.dot(f.tangent);
        let x = projected.normalized().unwrap_or(f.x);
        out.push(Frame3 { origin: f.origin, x, y: f.tangent.cross(x), z: f.tangent });
    }
    Ok(out)
}
