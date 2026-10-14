//! 🗺️ Analytic and free-form parametric surfaces. Every variant supports position, first/second
//! partial derivatives, normal, and Gaussian/mean/principal curvature via the standard first- and
//! second-fundamental-form formulas — the common surface interface every face in the topology
//! layer evaluates through, regardless of whether it's a `Plane` or a full `Nurbs` patch.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🏄️surface` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4, together with
//! its `🪡️surface-ops` sibling as a local child, mirroring the `⚙️engine` → `🟫️step`/`📦️mesh-io`
//! local-mount pattern from wave PEEL3.
//!
//! Crate cut of `semio-framework-3d-brep-representation` (wave E4 source admission ceiling, `📓️2026-10-14-wave-e4-3d-architect-bim.md`).

extern crate semio_framework_value_derive as value_derive;

// #region 🔖️Submodules

#[path = "🪡️surface-ops/🦀️.rs"]
pub mod surface_ops;

// #endregion 🔖️Submodules

use semio_framework_3d_brep_curve::bspline::{basis_function_derivatives, surface_derivatives_rational, KnotVector};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::{Pnt3, Vec3};

// #region 🔖️Surface

/// 🗺️ A parametric surface `S(u, v)`. Domain and periodicity are documented per variant; as with
/// [`semio_framework_3d_brep_curve::Curve3`], a face's *used* trim domain is stored by the topology layer, not here.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
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

semio_framework_3d_brep_vector::retire_geometry_variants!(Surface {Plane {frame},Cylinder {frame,radius},Cone {frame,half_angle},Sphere {frame,radius},Torus {frame,major_radius,minor_radius},Nurbs {u_knots,v_knots,controls,weights}});

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
    /// 🗺️ `true` where the parametrization COLLAPSES — a sphere's poles, a cone's apex — i.e. one
    /// partial vanishes against the other and `(u, v)` no longer names a distinct point.
    ///
    /// Asking [`Self::normal`] instead (`is_none()`) answers a different question with an ABSOLUTE
    /// threshold: `Vec3::normalized` rejects at `f64::EPSILON`, so a pole is detected on a unit
    /// sphere (`|du × dv| ≈ 6e-17`) and MISSED on a radius-2.2 one (`≈ 3e-16`) — the same geometry,
    /// decided by scale. Everything that has to know where a face's parametric boundary is
    /// degenerate (UV boundary polygons, tessellation's pole fan) needs the scale-free test, since
    /// a missed pole makes the two `u` branches meeting there collapse onto one and the face
    /// measures zero area.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_degenerate_uv(&self, u: f64, v: f64) -> bool {
        let d = self.derivatives(u, v);
        let (du, dv) = (d.du.norm(), d.dv.norm());
        let (lo, hi) = (du.min(dv), du.max(dv));
        hi <= f64::EPSILON || lo <= 1e-9 * hi
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

use semio_framework_3d_brep_vector::matrix::Affine3;

/// 🗺️ A generous, explicitly finite stand-in for the mathematically unbounded `v`-extent of a
/// `Cylinder`/`Cone` when a non-similarity map forces a NURBS conversion (a clamped B-spline knot
/// vector cannot represent a literal infinite domain — the same reason [`semio_framework_3d_brep_curve::Curve3::to_nurbs`]
/// requires an explicit `domain` for `Line`). Every double-precision kernel already operates within
/// some finite practical range; `1e6` is far beyond any plausible model extent while staying exact
/// (not tessellated/approximated) within that range.
const PRACTICAL_UNBOUNDED_EXTENT: f64 = 1.0e6;

/// 🗺️ The largest per-span half-angle keeping [`circular_profile_with_span`]'s rational-quadratic
/// parametrization within `1e-9` of the true `radius·(cos t, sin t)` point at every `t`, mirroring
/// [`semio_framework_3d_brep_curve::Curve3`]'s
/// identical fix for [`semio_framework_3d_brep_curve::Curve3::transformed`] —
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
/// per-span construction [`semio_framework_3d_brep_curve::Curve3::to_nurbs`] uses for `Circle`/`Ellipse`,
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

// #region 🎯️Pcurve

use semio_framework_3d_brep_curve::{curve_ops, Curve2, NurbsCurve3};
use semio_framework_3d_brep_vector::{Pnt2, Vec2};
use surface_ops::closest_uv;

/// 🎯️ Fits a p-curve for `curve` on `surface` restricted to `domain` — the [`Surface::project_curve`]
/// worker, split out so the seam-splitting entry point ([`Surface::project_curve_pieces`]) can call
/// it once per seam-free sub-domain.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fit_pcurve(surface: &Surface, curve: &Curve3, domain: (f64, f64), tol: f64) -> Curve2 {
    use curve_ops::{interpolate_curve, parameterize, ParamMethod};
    let surf_domain = surface.domain();
    let mut n_samples = 8usize;
    loop {
        let ts: Vec<f64> = (0..=n_samples).map(|i| domain.0 + (domain.1 - domain.0) * i as f64 / n_samples as f64).collect();
        let mut uv_pts: Vec<Pnt3> = Vec::with_capacity(ts.len());
        let mut prev: Option<(f64, f64)> = None;
        for &t in &ts {
            let target = curve.eval(t);
            let closest = closest_uv(surface, surf_domain, target, tol.min(1e-9));
            let (mut u, mut v) = (closest.u, closest.v);
            if let Some((pu, pv)) = prev {
                if surface.is_u_periodic() {
                    u = unwrap_near(u, pu, std::f64::consts::TAU);
                }
                if surface.is_v_periodic() {
                    v = unwrap_near(v, pv, std::f64::consts::TAU);
                }
            }
            prev = Some((u, v));
            uv_pts.push(Pnt3::new(u, v, 0.0));
        }
        let params = parameterize(&uv_pts, ParamMethod::Centripetal);
        if let Some(fitted3) = interpolate_curve(&uv_pts, 3, ParamMethod::Centripetal, None, false) {
            let pcurve = nurbs3_to_curve2(&fitted3);
            // 🐛 A SINGLE midpoint per inter-sample interval under-resolves the actual worst-case
            // deviation: it can converge (report `max_dev <= tol`) while a DIFFERENT point inside
            // the same interval — not the exact midpoint — is still off by 10-40× `tol` (confirmed:
            // this loop used to report a converged `6e-5` at `n_samples=128` while a point at
            // `s=0.1` was independently measured `0.0024` from the true curve). 4 evenly-spaced
            // interior probes per interval catch that without assuming any particular worst-point
            // location.
            let mut max_dev = 0.0f64;
            for (i, w) in ts.windows(2).enumerate() {
                for k in 1..=4 {
                    let frac = k as f64 / 5.0;
                    let t_probe = w[0] + (w[1] - w[0]) * frac;
                    let real = curve.eval(t_probe);
                    let s_probe = params[i] + (params[i + 1] - params[i]) * frac;
                    let uv = pcurve.eval(s_probe);
                    let approx = surface.eval(uv.x, uv.y);
                    max_dev = max_dev.max(real.distance(approx));
                }
            }
            if max_dev <= tol || n_samples >= 1024 {
                return pcurve;
            }
        }
        n_samples *= 2;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unwrap_near(x: f64, near: f64, period: f64) -> f64 {
    let mut y = x;
    while y - near > period * 0.5 {
        y -= period;
    }
    while near - y > period * 0.5 {
        y += period;
    }
    y
}

/// 🎯️ Drops the (always-zero) z-coordinate a `(u, v, 0)`-embedded [`NurbsCurve3`] fit carries,
/// producing the equivalent [`Curve2`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn nurbs3_to_curve2(curve: &NurbsCurve3) -> Curve2 {
    let controls = curve.controls.iter().map(|p| Pnt2::new(p.x, p.y)).collect();
    Curve2::Nurbs { knots: curve.knots.clone(), controls, weights: curve.weights.clone() }
}

/// 🎯️ Detects whether `fit_pcurve`'s naive single-shot fit crossed a periodic seam in a way that
/// needs an explicit domain split (the unwrapped raw samples jumped by close to a full period
/// between two adjacent low-density samples, rather than smoothly tracking the surface) and, if
/// so, bisects `domain` at the crossing and fits each seam-free side independently.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn seam_crossings(surface: &Surface, curve: &Curve3, domain: (f64, f64)) -> Vec<f64> {
    let probes = 16usize;
    let ts: Vec<f64> = (0..=probes).map(|i| domain.0 + (domain.1 - domain.0) * i as f64 / probes as f64).collect();
    let mut raw: Vec<(f64, f64)> = Vec::with_capacity(ts.len());
    for &t in &ts {
        let closest = closest_uv(surface, surface.domain(), curve.eval(t), 1e-6);
        raw.push((closest.u, closest.v));
    }
    let mut crossings = Vec::new();
    let period = std::f64::consts::TAU;
    for i in 0..probes {
        let (u0, v0) = raw[i];
        let (u1, v1) = raw[i + 1];
        let jumps = (surface.is_u_periodic() && (u1 - u0).abs() > period * 0.4) || (surface.is_v_periodic() && (v1 - v0).abs() > period * 0.4);
        if jumps {
            let mut lo = ts[i];
            let mut hi = ts[i + 1];
            for _ in 0..30 {
                let mid = 0.5 * (lo + hi);
                let a = closest_uv(surface, surface.domain(), curve.eval(lo), 1e-6);
                let m = closest_uv(surface, surface.domain(), curve.eval(mid), 1e-6);
                let far_from_a = (surface.is_u_periodic() && (m.u - a.u).abs() > period * 0.4) || (surface.is_v_periodic() && (m.v - a.v).abs() > period * 0.4);
                if far_from_a {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            crossings.push(0.5 * (lo + hi));
        }
    }
    crossings
}

impl Surface {
    /// 🎯️ Evaluates a p-curve at its own parameter `t`, mapping through `surface.eval` — the
    /// trivial half of the p-curve contract; [`Surface::project_curve`] is the fitting half.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn eval_pcurve(&self, pcurve: &Curve2, t: f64) -> Pnt3 {
        let uv = pcurve.eval(t);
        self.eval(uv.x, uv.y)
    }

    /// 🎯️ Fits a `Curve2` p-curve approximating `curve` (restricted to `domain`) on `self`, to
    /// within `tol` 3D deviation. Analytic shortcuts for a [`Curve3::Line`]/[`Curve3::Circle`] on
    /// a [`Surface::Plane`] whose in-plane axes already match the curve's own frame exactly (the
    /// common case for edges constructed directly in a face's frame); otherwise samples by
    /// (adaptively densified) subdivision, projects each sample through the certified
    /// [`crate::surface_ops::closest_uv`], unwraps periodic directions for continuity, and interpolates
    /// via [`curve_ops::interpolate_curve`], refining the sample density until the actual 3D
    /// deviation (checked at inter-sample midpoints, not just at the fitted points themselves) is
    /// within `tol`. Delegates to [`Surface::project_curve_pieces`] when the curve crosses a
    /// periodic seam, returning only its first piece — callers that need every piece (e.g. a
    /// seam-crossing trim edge that must become several coedges) should call
    /// [`Surface::project_curve_pieces`] directly.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn project_curve(&self, curve: &Curve3, domain: (f64, f64), tol: f64) -> Curve2 {
        if let Some(shortcut) = analytic_pcurve_shortcut(self, curve) {
            return shortcut;
        }
        self.project_curve_pieces(curve, domain, tol).into_iter().next().unwrap_or(Curve2::Line { origin: Pnt2::new(0.0, 0.0), dir: Vec2::new(1.0, 0.0) })
    }

    /// 🎯️ [`Surface::project_curve`], but split at every periodic-seam crossing so each returned
    /// piece stays within one seam-free stretch of `self`'s parameter domain.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn project_curve_pieces(&self, curve: &Curve3, domain: (f64, f64), tol: f64) -> Vec<Curve2> {
        if !self.is_u_periodic() && !self.is_v_periodic() {
            return vec![fit_pcurve(self, curve, domain, tol)];
        }
        let mut breaks = seam_crossings(self, curve, domain);
        breaks.retain(|&b| b > domain.0 + 1e-9 && b < domain.1 - 1e-9);
        breaks.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut bounds = vec![domain.0];
        bounds.extend(breaks);
        bounds.push(domain.1);
        bounds.windows(2).map(|w| fit_pcurve(self, curve, (w[0], w[1]), tol)).collect()
    }
}

/// 🎯️ Exact p-curve for the narrow (but common) case a curve's own frame is already aligned with
/// the plane's frame — a line always projects to a 2D line exactly; a circle/ellipse whose frame
/// shares the plane's `x`/`y`/`z` axes (not merely coplanar — [`semio_framework_3d_brep_curve::Curve2::Circle`] has no
/// independent rotation of its own) projects to the matching 2D conic exactly, parameter for
/// parameter. Any other in-plane rotation, or any non-planar surface, falls through to the
/// general numeric fit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn analytic_pcurve_shortcut(surface: &Surface, curve: &Curve3) -> Option<Curve2> {
    let Surface::Plane { frame } = surface else { return None };
    match curve {
        Curve3::Line { origin, dir } => {
            let lo = frame.to_local(*origin);
            let dv = frame.to_local_vector(*dir);
            Some(Curve2::Line { origin: Pnt2::new(lo.x, lo.y), dir: Vec2::new(dv.x, dv.y) })
        }
        Curve3::Circle { frame: cf, radius } if frames_aligned(frame, cf) => {
            let lo = frame.to_local(cf.origin);
            Some(Curve2::Circle { center: Pnt2::new(lo.x, lo.y), radius: *radius })
        }
        Curve3::Ellipse { frame: cf, major_radius, minor_radius } if frames_aligned(frame, cf) => {
            let lo = frame.to_local(cf.origin);
            Some(Curve2::Ellipse { center: Pnt2::new(lo.x, lo.y), x_axis: Vec2::new(1.0, 0.0), major_radius: *major_radius, minor_radius: *minor_radius })
        }
        _ => None,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn frames_aligned(a: &Frame3, b: &Frame3) -> bool {
    (a.x - b.x).norm() < 1e-9 && (a.y - b.y).norm() < 1e-9 && (a.z - b.z).norm() < 1e-9 && (a.to_local(b.origin).z).abs() < 1e-9
}

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️pcurve/🦀️.rs"]
mod pcurve_tests;
// #endregion 🔖️Tests

// #endregion 🎯️Pcurve
