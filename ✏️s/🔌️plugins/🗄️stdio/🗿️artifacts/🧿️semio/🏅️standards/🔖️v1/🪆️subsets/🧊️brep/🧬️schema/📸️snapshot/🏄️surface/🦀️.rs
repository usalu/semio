//! 🗺️ Analytic and free-form parametric surfaces. Every variant supports position, first/second
//! partial derivatives, normal, and Gaussian/mean/principal curvature via the standard first- and
//! second-fundamental-form formulas — the common surface interface every face in the topology
//! layer evaluates through, regardless of whether it's a `Plane` or a full `Nurbs` patch.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🏄️surface` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, together with
//! its `🪡️surface-ops` sibling as a local child, mirroring the `⚙️engine` → `🟫️step`/`📦️mesh-io`
//! local-mount pattern from wave PEEL3.

// #region 🔖️Submodules

#[path = "🪡️surface-ops/🦀️.rs"]
pub mod surface_ops;

// #endregion 🔖️Submodules

use crate::standards::v1::subsets::brep::schema::snapshot::curve::bspline::{basis_function_derivatives, surface_derivatives_rational, KnotVector};
use crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

// #region 🔖️Surface

/// 🗺️ A parametric surface `S(u, v)`. Domain and periodicity are documented per variant; as with
/// [`crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3`], a face's *used* trim domain is stored by the topology layer, not here.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Surface {
    /// 🗺️ `frame.origin + u·frame.x + v·frame.y`. Domain `(-∞, ∞) × (-∞, ∞)`.
    Plane { frame: Frame3 },
    /// 🗺️ Axis along `frame.z`. `u` = angle around the axis (periodic `[0, 2π)`), `v` = height
    /// along the axis (`(-∞, ∞)`).
    Cylinder { frame: Frame3, radius: f64 },
    /// 🗺️ Apex at `frame.origin`, axis `frame.z`. `u` = angle (periodic), `v` = signed distance
    /// along the axis from the apex; the radius at `v` is `v · tan(half_angle)`.
    Cone { frame: Frame3, half_angle: f64 },
    /// 🗺️ `u` = azimuth around `frame.z` (periodic `[0, 2π)`), `v` = elevation from the equator
    /// (`[-π/2, π/2]`).
    Sphere { frame: Frame3, radius: f64 },
    /// 🗺️ `u` = azimuth around the main axis `frame.z` (periodic), `v` = angle around the tube
    /// (periodic). `major_radius` is the distance from the axis to the tube center.
    Torus { frame: Frame3, major_radius: f64, minor_radius: f64 },
    /// 🗺️ A rational tensor-product B-spline surface; `controls[i][j]`/`weights[i][j]` indexed by
    /// `(u, v)` control-net position.
    Nurbs { u_knots: KnotVector, v_knots: KnotVector, controls: Vec<Vec<Pnt3>>, weights: Vec<Vec<f64>> },
}

/// 🗺️ First/second partial derivatives at a surface point, the common input to normal and
/// curvature computations.
pub struct SurfaceDerivatives {
    pub point: Pnt3,
    pub du: Vec3,
    pub dv: Vec3,
    pub duu: Vec3,
    pub duv: Vec3,
    pub dvv: Vec3,
}

impl Surface {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn domain(&self) -> ((f64, f64), (f64, f64)) {
        match self {
            Surface::Plane { .. } => ((f64::NEG_INFINITY, f64::INFINITY), (f64::NEG_INFINITY, f64::INFINITY)),
            Surface::Cylinder { .. } => ((0.0, std::f64::consts::TAU), (f64::NEG_INFINITY, f64::INFINITY)),
            Surface::Cone { .. } => ((0.0, std::f64::consts::TAU), (0.0, f64::INFINITY)),
            Surface::Sphere { .. } => ((0.0, std::f64::consts::TAU), (-std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2)),
            Surface::Torus { .. } => ((0.0, std::f64::consts::TAU), (0.0, std::f64::consts::TAU)),
            Surface::Nurbs { u_knots, v_knots, .. } => (u_knots.domain(), v_knots.domain()),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_u_periodic(&self) -> bool {
        matches!(self, Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } | Surface::Torus { .. })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_v_periodic(&self) -> bool {
        matches!(self, Surface::Torus { .. })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn eval(&self, u: f64, v: f64) -> Pnt3 {
        match self {
            Surface::Plane { frame } => frame.to_world(Pnt3::new(u, v, 0.0)),
            Surface::Cylinder { frame, radius } => frame.to_world(Pnt3::new(radius * u.cos(), radius * u.sin(), v)),
            Surface::Cone { frame, half_angle } => {
                let r = v * half_angle.tan();
                frame.to_world(Pnt3::new(r * u.cos(), r * u.sin(), v))
            }
            Surface::Sphere { frame, radius } => frame.to_world(Pnt3::new(radius * v.cos() * u.cos(), radius * v.cos() * u.sin(), radius * v.sin())),
            Surface::Torus { frame, major_radius, minor_radius } => {
                let r = major_radius + minor_radius * v.cos();
                frame.to_world(Pnt3::new(r * u.cos(), r * u.sin(), minor_radius * v.sin()))
            }
            Surface::Nurbs { u_knots, v_knots, controls, weights } => eval_nurbs_point(u_knots, v_knots, controls, weights, u, v),
        }
    }
    /// 🗺️ First and second partial derivatives at `(u, v)`. Analytic surfaces use closed forms;
    /// [`Surface::Nurbs`] uses the exact rational `RatSurfaceDerivs` recurrence
    /// ([`surface_derivatives_rational`]) — safe for tight Newton iteration, unlike the
    /// finite-difference stand-in this used to be.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn derivatives(&self, u: f64, v: f64) -> SurfaceDerivatives {
        match self {
            Surface::Plane { frame } => SurfaceDerivatives { point: self.eval(u, v), du: frame.x, dv: frame.y, duu: Vec3::ZERO, duv: Vec3::ZERO, dvv: Vec3::ZERO },
            Surface::Cylinder { frame, radius } => {
                let du = frame.to_world_vector(Vec3::new(-radius * u.sin(), radius * u.cos(), 0.0));
                let duu = frame.to_world_vector(Vec3::new(-radius * u.cos(), -radius * u.sin(), 0.0));
                SurfaceDerivatives { point: self.eval(u, v), du, dv: frame.z, duu, duv: Vec3::ZERO, dvv: Vec3::ZERO }
            }
            Surface::Cone { frame, half_angle } => {
                let r = v * half_angle.tan();
                let tan_a = half_angle.tan();
                let du = frame.to_world_vector(Vec3::new(-r * u.sin(), r * u.cos(), 0.0));
                let dv = frame.to_world_vector(Vec3::new(tan_a * u.cos(), tan_a * u.sin(), 1.0));
                let duu = frame.to_world_vector(Vec3::new(-r * u.cos(), -r * u.sin(), 0.0));
                let duv = frame.to_world_vector(Vec3::new(-tan_a * u.sin(), tan_a * u.cos(), 0.0));
                SurfaceDerivatives { point: self.eval(u, v), du, dv, duu, duv, dvv: Vec3::ZERO }
            }
            Surface::Sphere { frame, radius } => {
                let du = frame.to_world_vector(Vec3::new(-radius * v.cos() * u.sin(), radius * v.cos() * u.cos(), 0.0));
                let dv = frame.to_world_vector(Vec3::new(-radius * v.sin() * u.cos(), -radius * v.sin() * u.sin(), radius * v.cos()));
                let duu = frame.to_world_vector(Vec3::new(-radius * v.cos() * u.cos(), -radius * v.cos() * u.sin(), 0.0));
                let duv = frame.to_world_vector(Vec3::new(radius * v.sin() * u.sin(), -radius * v.sin() * u.cos(), 0.0));
                let dvv = frame.to_world_vector(Vec3::new(-radius * v.cos() * u.cos(), -radius * v.cos() * u.sin(), -radius * v.sin()));
                SurfaceDerivatives { point: self.eval(u, v), du, dv, duu, duv, dvv }
            }
            Surface::Torus { frame, major_radius, minor_radius } => {
                let r = major_radius + minor_radius * v.cos();
                let du = frame.to_world_vector(Vec3::new(-r * u.sin(), r * u.cos(), 0.0));
                let dv = frame.to_world_vector(Vec3::new(-minor_radius * v.sin() * u.cos(), -minor_radius * v.sin() * u.sin(), minor_radius * v.cos()));
                let duu = frame.to_world_vector(Vec3::new(-r * u.cos(), -r * u.sin(), 0.0));
                let duv = frame.to_world_vector(Vec3::new(minor_radius * v.sin() * u.sin(), -minor_radius * v.sin() * u.cos(), 0.0));
                let dvv = frame.to_world_vector(Vec3::new(-minor_radius * v.cos() * u.cos(), -minor_radius * v.cos() * u.sin(), -minor_radius * v.sin()));
                SurfaceDerivatives { point: self.eval(u, v), du, dv, duu, duv, dvv }
            }
            Surface::Nurbs { u_knots, v_knots, controls, weights } => {
                let controls_h: Vec<Vec<Vec<f64>>> = controls.iter().zip(weights).map(|(row, wrow)| row.iter().zip(wrow).map(|(p, &w)| vec![p.x * w, p.y * w, p.z * w, w]).collect()).collect();
                let s = surface_derivatives_rational(u_knots, v_knots, &controls_h, u, v, 2);
                let vec_of = |k: usize, l: usize| Vec3::new(s[k][l][0], s[k][l][1], s[k][l][2]);
                let p0 = vec_of(0, 0);
                SurfaceDerivatives { point: Pnt3::new(p0.x, p0.y, p0.z), du: vec_of(1, 0), dv: vec_of(0, 1), duu: vec_of(2, 0), duv: vec_of(1, 1), dvv: vec_of(0, 2) }
            }
        }
    }
    /// 🗺️ Unit surface normal `du × dv` (falls back to `None` at a singular point, e.g. a sphere
    /// pole or a cone apex, where `du` degenerates to zero).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn normal(&self, u: f64, v: f64) -> Option<Vec3> {
        let d = self.derivatives(u, v);
        d.du.cross(d.dv).normalized()
    }
    /// 🗺️ Gaussian curvature `K = (LN - M²) / (EG - F²)` and mean curvature `H = (EN - 2FM + GL) /
    /// (2(EG - F²))`, from the first fundamental form `(E, F, G)` and second `(L, M, N)`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curvature(&self, u: f64, v: f64) -> Option<(f64, f64)> {
        let d = self.derivatives(u, v);
        let n = d.du.cross(d.dv).normalized()?;
        let e = d.du.dot(d.du);
        let f = d.du.dot(d.dv);
        let g = d.dv.dot(d.dv);
        let l = d.duu.dot(n);
        let m = d.duv.dot(n);
        let nn = d.dvv.dot(n);
        let denom = e * g - f * f;
        if denom.abs() <= 1e-300 {
            return None;
        }
        let gaussian = (l * nn - m * m) / denom;
        let mean = (e * nn - 2.0 * f * m + g * l) / (2.0 * denom);
        Some((gaussian, mean))
    }
    /// 🗺️ Principal curvatures `(κ1, κ2)` derived from Gaussian `K` and mean `H` curvature via
    /// `κ = H ± √(H² - K)`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn principal_curvatures(&self, u: f64, v: f64) -> Option<(f64, f64)> {
        let (gaussian, mean) = self.curvature(u, v)?;
        let disc = (mean * mean - gaussian).max(0.0).sqrt();
        Some((mean + disc, mean - disc))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_planar(&self) -> bool {
        matches!(self, Surface::Plane { .. })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn eval_nurbs_point(u_knots: &KnotVector, v_knots: &KnotVector, controls: &[Vec<Pnt3>], weights: &[Vec<f64>], u: f64, v: f64) -> Pnt3 {
    let u_span = u_knots.find_span(u);
    let v_span = v_knots.find_span(v);
    let nu = basis_function_derivatives(u_knots, u_span, u, 0);
    let nv = basis_function_derivatives(v_knots, v_span, v, 0);
    let up = u_knots.degree;
    let vp = v_knots.degree;
    let mut hx = 0.0;
    let mut hy = 0.0;
    let mut hz = 0.0;
    let mut hw = 0.0;
    #[allow(clippy::needless_range_loop)]
    for i in 0..=up {
        for j in 0..=vp {
            let ci = u_span - up + i;
            let cj = v_span - vp + j;
            let b = nu[0][i] * nv[0][j];
            let w = weights[ci][cj];
            hx += b * w * controls[ci][cj].x;
            hy += b * w * controls[ci][cj].y;
            hz += b * w * controls[ci][cj].z;
            hw += b * w;
        }
    }
    Pnt3::new(hx / hw, hy / hw, hz / hw)
}

// #endregion 🔖️Surface

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region 🔁️Transform

use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Affine3;

/// 🗺️ A generous, explicitly finite stand-in for the mathematically unbounded `v`-extent of a
/// `Cylinder`/`Cone` when a non-similarity map forces a NURBS conversion (a clamped B-spline knot
/// vector cannot represent a literal infinite domain — the same reason [`crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3::to_nurbs`]
/// requires an explicit `domain` for `Line`). Every double-precision kernel already operates within
/// some finite practical range; `1e6` is far beyond any plausible model extent while staying exact
/// (not tessellated/approximated) within that range.
const PRACTICAL_UNBOUNDED_EXTENT: f64 = 1.0e6;

/// 🗺️ The largest per-span half-angle keeping [`circular_profile_with_span`]'s rational-quadratic
/// parametrization within `1e-9` of the true `radius·(cos t, sin t)` point at every `t`, mirroring
/// [`crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3`]'s
/// identical fix for [`crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3::transformed`] —
/// same leading-order bound `peak ≈ 0.0321·radius·half_span³` (a rational quadratic Bezier's
/// parameter is a Möbius, not linear, reparametrization of angle), same derivation, same 2×
/// margin against the series' next term. [`revolve_to_nurbs`]'s non-similarity transform path
/// needs the `u`-direction sweep to reproduce `Surface::eval`'s pointwise pushforward exactly
/// (same contract as `Curve3::transformed`), so every [`circular_profile_with_span`] call site
/// sizes its span with this instead of a fixed 120° cap.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn refined_max_span(radius: f64) -> f64 {
    let tol = 1e-9;
    let r = radius.abs().max(1e-9);
    let half_span = (tol / (0.0321 * r)).cbrt() * 0.5;
    (2.0 * half_span).min(std::f64::consts::TAU / 3.0)
}

/// 🗺️ Exact rational-quadratic NURBS control points for a circular arc of `radius` centered at
/// `center` in a generic 2D `(radial, height)` half-plane, split every `max_span` — the same
/// per-span construction [`crate::standards::v1::subsets::brep::schema::snapshot::curve::Curve3::to_nurbs`] uses for `Circle`/`Ellipse`,
/// generalized to an off-origin circle (a torus's meridian) and reused, at `center = (0,0), radius
/// = 1`, as the shared angular sweep for every surface of revolution built below. Callers that
/// only need the SHAPE (not a pointwise angle-to-parameter correspondence) can pass a coarse span
/// like `TAU / 3.0`; [`refined_max_span`] gives the tighter span an exact pointwise pushforward
/// (`Curve3::transformed`/`Surface::transformed`'s non-similarity paths) needs instead.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn circular_profile_with_span(center: (f64, f64), radius: f64, domain: (f64, f64), max_span: f64) -> (KnotVector, Vec<(f64, f64)>, Vec<f64>) {
    let span = domain.1 - domain.0;
    let n_spans = (span.abs() / max_span).ceil().max(1.0) as usize;
    let step = span / n_spans as f64;
    let mut points = Vec::with_capacity(2 * n_spans + 1);
    let mut weights = Vec::with_capacity(2 * n_spans + 1);
    let local = |angle: f64, scale: f64| (center.0 + radius * angle.cos() * scale, center.1 + radius * angle.sin() * scale);
    for span_i in 0..n_spans {
        let a0 = domain.0 + step * span_i as f64;
        let a1 = a0 + step;
        let half = (a1 - a0) * 0.5;
        let mid = (a0 + a1) * 0.5;
        let w1 = half.cos();
        if span_i == 0 {
            points.push(local(a0, 1.0));
            weights.push(1.0);
        }
        points.push(local(mid, 1.0 / w1));
        weights.push(w1);
        points.push(local(a1, 1.0));
        weights.push(1.0);
    }
    let mut knots = vec![domain.0, domain.0, domain.0];
    for span_i in 1..n_spans {
        let knot = domain.0 + step * span_i as f64;
        knots.push(knot);
        knots.push(knot);
    }
    knots.push(domain.1);
    knots.push(domain.1);
    knots.push(domain.1);
    (KnotVector::new(knots, 2, points.len()).unwrap(), points, weights)
}

/// 🗺️ Builds the exact (already `map`-transformed) NURBS surface of revolution swept from a
/// `(radial, height)` meridian `profile` (its own `profile_knots`/`profile_weights`, in `frame`'s
/// local plane) around `frame.z` — the shared tensor-product construction behind every analytic
/// surface kind's non-similarity fallback below: `u` comes from [`circular_profile_with_span`]'s
/// unit-circle sweep, `v` is the caller's own meridian, and each control point's weight is the
/// product of its `u`- and `v`-direction weights (the standard NURBS revolution rule). `u`'s span
/// is refined via [`refined_max_span`] against `u_radius_scale` — the physical radius the caller's
/// unit-circle `(cx, cy)` gets multiplied by (see each call site) — so a unit-circle angular error
/// of `angular_error` turns into a WORLD positional error of only `u_radius_scale · angular_error`,
/// keeping `Surface::eval`'s pointwise pushforward contract (`transformed.eval(u,v) ==
/// map.apply_point(self.eval(u,v))`) intact for every `(u, v)`, not just at span boundaries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn revolve_to_nurbs(frame: &Frame3, map: &Affine3, profile_knots: KnotVector, profile: &[(f64, f64)], profile_weights: &[f64], u_radius_scale: f64) -> Surface {
    let (u_knots, u_nodes, u_weights) = circular_profile_with_span((0.0, 0.0), 1.0, (0.0, std::f64::consts::TAU), refined_max_span(u_radius_scale));
    let mut controls: Vec<Vec<Pnt3>> = Vec::with_capacity(u_nodes.len());
    let mut weights: Vec<Vec<f64>> = Vec::with_capacity(u_nodes.len());
    for (&(cx, cy), &uw) in u_nodes.iter().zip(u_weights.iter()) {
        let mut row_ctrl = Vec::with_capacity(profile.len());
        let mut row_w = Vec::with_capacity(profile.len());
        for (&(r, h), &pw) in profile.iter().zip(profile_weights.iter()) {
            let local = frame.origin + frame.x * (r * cx) + frame.y * (r * cy) + frame.z * h;
            row_ctrl.push(map.apply_point(local));
            row_w.push(pw * uw);
        }
        controls.push(row_ctrl);
        weights.push(row_w);
    }
    Surface::Nurbs { u_knots, v_knots: profile_knots, controls, weights }
}

/// 🗺️ [`Frame3::transformed`] divides EVERY axis by `scale` so the frame stays orthonormal — right
/// for `Sphere`/`Torus`, whose `eval` multiplies all three axes by an already-`scale`-compensated
/// radius. `Cylinder`/`Cone::eval` instead use `frame.z` as a raw multiplier on `v` itself (no
/// radius in that term), so dividing `z` by `scale` there would make `transformed.eval(u, v)`
/// drift from `map.apply_point(self.eval(u, v))` by a `v`-proportional error — `x`/`y` still need
/// the `1/scale` (they pair with `radius * scale`), only `z` must stay the raw, un-divided
/// `map.apply_vector(frame.z)` (magnitude `scale`, matching the correct `d(eval)/dv` under `map`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn axial_frame_transformed(frame: &Frame3, map: &Affine3, scale: f64) -> Frame3 {
    let radial = frame.transformed(map, scale);
    Frame3 { origin: radial.origin, x: radial.x, y: radial.y, z: map.apply_vector(frame.z) }
}

impl Surface {
    /// 🗺️ Exact affine transform. `Plane` stays exact under ANY invertible affine map — its `x`/`y`
    /// axes are mapped directly (not renormalized: `Plane` places no orthonormality requirement on
    /// its own axes, only linear independence, which any invertible linear map preserves) and `z`
    /// is re-derived as `x × y` (deliberately, not itself mapped — see [`Frame3::transformed`]'s
    /// docstring on why re-deriving `z` would be wrong for the OTHER analytic kinds but is exactly
    /// what a general, non-orthonormal frame needs here). `Cylinder`/`Cone`/`Sphere`/`Torus` stay
    /// analytic under a similarity (frame mapped via [`Frame3::transformed`], radii scaled
    /// uniformly, `half_angle` invariant); under a non-similarity map they convert to the exact
    /// NURBS surface of revolution ([`revolve_to_nurbs`]) already expressed in `map`'s image.
    /// `Nurbs` always just transforms its control points (rational weights are affine-invariant).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn transformed(&self, map: &Affine3) -> Surface {
        match self {
            Surface::Plane { frame } => {
                let x = map.apply_vector(frame.x);
                let y = map.apply_vector(frame.y);
                let z = x.cross(y);
                Surface::Plane { frame: Frame3 { origin: map.apply_point(frame.origin), x, y, z } }
            }
            Surface::Nurbs { u_knots, v_knots, controls, weights } => {
                Surface::Nurbs { u_knots: u_knots.clone(), v_knots: v_knots.clone(), controls: controls.iter().map(|row| row.iter().map(|p| map.apply_point(*p)).collect()).collect(), weights: weights.clone() }
            }
            Surface::Cylinder { frame, radius } => match map.is_similarity() {
                Some((_, scale, _)) => Surface::Cylinder { frame: axial_frame_transformed(frame, map, scale), radius: radius * scale },
                None => {
                    let knots = KnotVector::new(vec![-PRACTICAL_UNBOUNDED_EXTENT, -PRACTICAL_UNBOUNDED_EXTENT, PRACTICAL_UNBOUNDED_EXTENT, PRACTICAL_UNBOUNDED_EXTENT], 1, 2).unwrap();
                    let profile = [(*radius, -PRACTICAL_UNBOUNDED_EXTENT), (*radius, PRACTICAL_UNBOUNDED_EXTENT)];
                    revolve_to_nurbs(frame, map, knots, &profile, &[1.0, 1.0], *radius)
                }
            },
            Surface::Cone { frame, half_angle } => match map.is_similarity() {
                Some((_, scale, _)) => Surface::Cone { frame: axial_frame_transformed(frame, map, scale), half_angle: *half_angle },
                None => {
                    let knots = KnotVector::new(vec![0.0, 0.0, PRACTICAL_UNBOUNDED_EXTENT, PRACTICAL_UNBOUNDED_EXTENT], 1, 2).unwrap();
                    let profile = [(0.0, 0.0), (PRACTICAL_UNBOUNDED_EXTENT * half_angle.tan(), PRACTICAL_UNBOUNDED_EXTENT)];
                    // 🧭️ The cone's radius grows unboundedly with `v` (`r(v) = v·tan(half_angle)`), so the
                    // u-sweep's positional error at any given `v` is exactly `v·tan(half_angle)·angular_error`
                    // (the `v`-degree-1 blend between the apex, r=0, and the far profile point is affine, so
                    // it scales the u-error by the SAME `r(v)` a direct construction at that `v` would use —
                    // verified numerically). Sizing the refinement off `half_angle.tan()` (r at v=1, not the
                    // impractical `PRACTICAL_UNBOUNDED_EXTENT` edge) keeps span count sane while still giving
                    // sub-`1e-6` error out to `v` in the hundreds — ample for any query in the surface's own
                    // "practical" domain.
                    revolve_to_nurbs(frame, map, knots, &profile, &[1.0, 1.0], half_angle.tan())
                }
            },
            Surface::Sphere { frame, radius } => match map.is_similarity() {
                Some((_, scale, _)) => Surface::Sphere { frame: frame.transformed(map, scale), radius: radius * scale },
                None => {
                    // 🧭️ Both directions are angle-parametrized (`u` AND `v`), so both need
                    // [`refined_max_span`], not just the shared `u`-sweep inside `revolve_to_nurbs`.
                    let (knots, profile, weights) = circular_profile_with_span((0.0, 0.0), *radius, (-std::f64::consts::FRAC_PI_2, std::f64::consts::FRAC_PI_2), refined_max_span(*radius));
                    revolve_to_nurbs(frame, map, knots, &profile, &weights, *radius)
                }
            },
            Surface::Torus { frame, major_radius, minor_radius } => match map.is_similarity() {
                Some((_, scale, _)) => Surface::Torus { frame: frame.transformed(map, scale), major_radius: major_radius * scale, minor_radius: minor_radius * scale },
                None => {
                    let (knots, profile, weights) = circular_profile_with_span((*major_radius, 0.0), *minor_radius, (0.0, std::f64::consts::TAU), refined_max_span(*minor_radius));
                    revolve_to_nurbs(frame, map, knots, &profile, &weights, major_radius + minor_radius)
                }
            },
        }
    }
}

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️transform/🦀️.rs"]
mod transform_tests;
// #endregion 🔖️Tests

// #endregion 🔁️Transform

// #region 🧭️Isocurve

/// 🧭️ The two directions an isoparametric curve can hold fixed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsoDirection {
    U,
    V,
}

impl Surface {
    /// 🧭️ The curve traced by holding `dir` fixed at `at` and letting the other parameter vary —
    /// exact for every analytic kind (each reduces to a [`Curve3::Line`] or [`Curve3::Circle`]
    /// whose own evaluation reproduces `self.eval` pointwise, not merely approximates it — see the
    /// per-arm derivations below) and exact for [`Surface::Nurbs`] via the standard isoparametric
    /// curve-extraction formula (fixed-direction basis functions folded into new, real control
    /// points on the other direction's own knot vector — no new knots, no approximation).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn isocurve(&self, dir: IsoDirection, at: f64) -> Curve3 {
        match self {
            Surface::Plane { frame } => match dir {
                IsoDirection::U => Curve3::Line { origin: frame.origin + frame.x * at, dir: frame.y },
                IsoDirection::V => Curve3::Line { origin: frame.origin + frame.y * at, dir: frame.x },
            },
            Surface::Cylinder { frame, radius } => match dir {
                IsoDirection::U => Curve3::Line { origin: frame.origin + frame.x * (radius * at.cos()) + frame.y * (radius * at.sin()), dir: frame.z },
                IsoDirection::V => Curve3::Circle { frame: Frame3 { origin: frame.origin + frame.z * at, x: frame.x, y: frame.y, z: frame.z }, radius: *radius },
            },
            Surface::Cone { frame, half_angle } => match dir {
                IsoDirection::U => {
                    let tan_a = half_angle.tan();
                    Curve3::Line { origin: frame.origin, dir: frame.to_world_vector(Vec3::new(tan_a * at.cos(), tan_a * at.sin(), 1.0)) }
                }
                IsoDirection::V => Curve3::Circle { frame: Frame3 { origin: frame.origin + frame.z * at, x: frame.x, y: frame.y, z: frame.z }, radius: at * half_angle.tan() },
            },
            Surface::Sphere { frame, radius } => match dir {
                IsoDirection::U => {
                    let radial = frame.x * at.cos() + frame.y * at.sin();
                    let up = frame.z;
                    let side = radial.cross(up);
                    Curve3::Circle { frame: Frame3 { origin: frame.origin, x: radial, y: up, z: side }, radius: *radius }
                }
                IsoDirection::V => Curve3::Circle { frame: Frame3 { origin: frame.origin + frame.z * (radius * at.sin()), x: frame.x, y: frame.y, z: frame.z }, radius: radius * at.cos() },
            },
            Surface::Torus { frame, major_radius, minor_radius } => match dir {
                IsoDirection::U => {
                    let radial = frame.x * at.cos() + frame.y * at.sin();
                    let up = frame.z;
                    let side = radial.cross(up);
                    Curve3::Circle { frame: Frame3 { origin: frame.origin + radial * *major_radius, x: radial, y: up, z: side }, radius: *minor_radius }
                }
                IsoDirection::V => {
                    let radius = major_radius + minor_radius * at.cos();
                    Curve3::Circle { frame: Frame3 { origin: frame.origin + frame.z * (minor_radius * at.sin()), x: frame.x, y: frame.y, z: frame.z }, radius }
                }
            },
            Surface::Nurbs { u_knots, v_knots, controls, weights } => nurbs_isocurve(u_knots, v_knots, controls, weights, dir, at),
        }
    }
}

/// 🧭️ Standard isoparametric curve extraction for a tensor-product rational B-spline: folds the
/// fixed direction's basis functions into new (real, not approximated) control points on the
/// other direction's own knot vector — `C(t) = Σ_j [Σ_i N_i(fixed) w_ij P_ij] M_j(t) / Σ_j[...] M_j(t)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn nurbs_isocurve(u_knots: &KnotVector, v_knots: &KnotVector, controls: &[Vec<Pnt3>], weights: &[Vec<f64>], dir: IsoDirection, at: f64) -> Curve3 {
    match dir {
        IsoDirection::U => {
            let span = u_knots.find_span(at);
            let basis = &basis_function_derivatives(u_knots, span, at, 0)[0];
            let degree = u_knots.degree;
            let nv = controls[0].len();
            let mut new_controls = Vec::with_capacity(nv);
            let mut new_weights = Vec::with_capacity(nv);
            for j in 0..nv {
                let (mut hx, mut hy, mut hz, mut hw) = (0.0, 0.0, 0.0, 0.0);
                for (k, &weight) in basis[..=degree].iter().enumerate() {
                    let i = span - degree + k;
                    let b = weight * weights[i][j];
                    hx += b * controls[i][j].x;
                    hy += b * controls[i][j].y;
                    hz += b * controls[i][j].z;
                    hw += b;
                }
                new_controls.push(Pnt3::new(hx / hw, hy / hw, hz / hw));
                new_weights.push(hw);
            }
            Curve3::Nurbs { knots: v_knots.clone(), controls: new_controls, weights: new_weights }
        }
        IsoDirection::V => {
            let span = v_knots.find_span(at);
            let basis = &basis_function_derivatives(v_knots, span, at, 0)[0];
            let degree = v_knots.degree;
            let nu = controls.len();
            let mut new_controls = Vec::with_capacity(nu);
            let mut new_weights = Vec::with_capacity(nu);
            for i in 0..nu {
                let (mut hx, mut hy, mut hz, mut hw) = (0.0, 0.0, 0.0, 0.0);
                for (k, &weight) in basis[..=degree].iter().enumerate() {
                    let j = span - degree + k;
                    let b = weight * weights[i][j];
                    hx += b * controls[i][j].x;
                    hy += b * controls[i][j].y;
                    hz += b * controls[i][j].z;
                    hw += b;
                }
                new_controls.push(Pnt3::new(hx / hw, hy / hw, hz / hw));
                new_weights.push(hw);
            }
            Curve3::Nurbs { knots: u_knots.clone(), controls: new_controls, weights: new_weights }
        }
    }
}

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️isocurve/🦀️.rs"]
mod isocurve_tests;
// #endregion 🔖️Tests

// #endregion 🧭️Isocurve
