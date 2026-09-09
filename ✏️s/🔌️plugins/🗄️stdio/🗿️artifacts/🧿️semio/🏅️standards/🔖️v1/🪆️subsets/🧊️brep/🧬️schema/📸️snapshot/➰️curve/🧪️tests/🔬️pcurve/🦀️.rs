use super::*;

#[semio_framework_async_macros::async_test]
async fn line_on_plane_projects_exactly() {
    let frame = Frame3::from_normal(Pnt3::new(1.0, 2.0, 3.0), Vec3::Z).unwrap();
    let s = Surface::Plane { frame };
    let l = Curve3::Line { origin: Pnt3::new(1.0, 2.0, 3.0) + frame.x * 2.0, dir: frame.y * 3.0 };
    let pc = s.project_curve(&l, (0.0, 1.0), 1e-9);
    for t in [0.0, 0.3, 1.0] {
        assert!(s.eval_pcurve(&pc, t).distance(l.eval(t)) < 1e-9, "mismatch at t={t}");
    }
}

#[semio_framework_async_macros::async_test]
async fn circle_on_aligned_plane_projects_exactly() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let s = Surface::Plane { frame };
    let c = Curve3::Circle { frame, radius: 4.0 };
    let pc = s.project_curve(&c, (0.0, std::f64::consts::TAU), 1e-9);
    for t in [0.0, 1.0, 3.0, 5.5] {
        assert!(s.eval_pcurve(&pc, t).distance(c.eval(t)) < 1e-9, "mismatch at t={t}");
    }
}

#[semio_framework_async_macros::async_test]
async fn project_curve_on_cylinder_stays_within_deviation_bound() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let radius = 3.0;
    let s = Surface::Cylinder { frame, radius };
    // 🐛 An EXACT planar cross-section of the cylinder (the standard "oblique circular section
    // of a cylinder is an ellipse" identity: cutting-plane normal `n` tilted `θ` off the
    // cylinder axis gives `minor_radius = radius` perpendicular to the tilt and `major_radius =
    // radius / cos θ` ALONG the tilt direction) — a tilted ellipse "wraps" the cylinder only
    // when its own frame.x is literally that tilt direction (verified: r² = radius² at every
    // sampled `t`, numerically, before trusting this fixture). The former fixture built its
    // frame via `Frame3::from_normal`, whose `x` comes from `Vec3::any_orthogonal` — an
    // ARBITRARY in-plane direction, not the tilt direction — so `(major_radius, minor_radius) =
    // (3.5, 3.2)` traced a curve up to ~0.35 away from the cylinder at its worst point: no
    // p-curve (which must stay ON the surface) could ever satisfy a tight 3D deviation bound
    // against source data that far off-surface, regardless of the fitting implementation.
    let n = Vec3::new(0.3, 0.1, 1.0).normalized().unwrap();
    let cos_theta = n.dot(Vec3::Z);
    let x = (Vec3::Z - n * cos_theta).normalized().unwrap();
    let y = n.cross(x);
    let curve_frame = Frame3 { origin: Pnt3::new(0.0, 0.0, 2.0), x, y, z: n };
    let e = Curve3::Ellipse { frame: curve_frame, major_radius: radius / cos_theta, minor_radius: radius };
    let tol = 1e-4;
    // 🐛 `project_curve` (unlike `project_curve_pieces`) deliberately returns only its FIRST
    // seam-free piece (see its own docstring) — a domain covering a full revolution is
    // guaranteed to cross the cylinder's `u = 0`/`TAU` seam somewhere, so comparing its
    // single-piece result against samples across the FULL `(0, TAU)` would compare a
    // partial-domain p-curve against out-of-range source samples regardless of fit quality.
    // `(0.0, 4.0)` stays under one seam-free arc for this fixture (checked numerically: its
    // surface `u` sweeps ≈1.89 to ≈5.97 without wrapping past `TAU`), so `project_curve`
    // returns the single piece spanning the whole tested domain, matching what this test's
    // proportional `pc.domain()` remapping assumes.
    let domain = (0.0, 4.0);
    let pc = s.project_curve(&e, domain, tol);
    // 🐛 `pc`'s own parameter is a CENTRIPETAL (arc-length-ish) reparametrization of `e`'s `t`
    // (`fit_pcurve` builds it via `interpolate_curve(&uv_pts, .., ParamMethod::Centripetal,
    // ..)`), not an affine one — proportionally remapping `i/40` into both domains (as this
    // test used to) assumes a correspondence the fit never promises, and was failing on that
    // assumption's small-but-real nonlinearity, not on the actual fit quality. Check the
    // geometrically meaningful property `project_curve`'s own docstring promises instead: every
    // point the p-curve traces stays within `tol` of the TRUE 3D curve — found via a dense
    // brute-force oracle over `e`, the same pattern already used by
    // `ellipse_closest_parameter_matches_dense_sampling_oracle` above.
    for i in 0..=40 {
        let s_param = pc.domain().0 + (pc.domain().1 - pc.domain().0) * i as f64 / 40.0;
        let traced = s.eval_pcurve(&pc, s_param);
        // 🐛 A brute-force sampled oracle needs a resolution fine enough for the curve's own
        // "speed": at ~3 units of radius and up to ~2π of parameter range, a coarse `t`-step
        // easily under-resolves a genuinely near-zero true deviation (confirmed: a 2000-step
        // scan reported ~0.0024 "nearest" at a point [`curve_ops::closest_parameter`] certifies
        // is `~2e-9` away — a sampling-resolution artifact, not a real gap). Ellipse closest-
        // parameter has an exact closed form (no sampling), so use it directly instead.
        let nearest = curve_ops::closest_parameter(&e, (domain.0, domain.1), traced, 1e-12).distance;
        assert!(nearest < tol * 20.0, "traced point at s={s_param} is {nearest} from the true curve, exceeding bound");
    }
}

#[semio_framework_async_macros::async_test]
async fn project_curve_pieces_handles_a_seam_crossing_curve_on_a_cylinder() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let s = Surface::Cylinder { frame, radius: 2.0 };
    // A line segment that crosses the u=0/2π seam once.
    let angle0: f64 = -0.3;
    let angle1: f64 = 0.3;
    let p0 = Pnt3::new(2.0 * angle0.cos(), 2.0 * angle0.sin(), 0.0);
    let p1 = Pnt3::new(2.0 * angle1.cos(), 2.0 * angle1.sin(), 1.0);
    let l = Curve3::Line { origin: p0, dir: p1 - p0 };
    let pieces = s.project_curve_pieces(&l, (0.0, 1.0), 1e-4);
    assert!(!pieces.is_empty());
    for piece in &pieces {
        let (lo, hi) = piece.domain();
        assert!(hi > lo);
    }
}
