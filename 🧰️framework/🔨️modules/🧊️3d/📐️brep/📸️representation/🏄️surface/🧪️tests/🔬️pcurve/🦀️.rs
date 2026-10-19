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
    let n = Vec3::new(0.3, 0.1, 1.0).normalized().unwrap();
    let cos_theta = n.dot(Vec3::Z);
    let x = (Vec3::Z - n * cos_theta).normalized().unwrap();
    let y = n.cross(x);
    let curve_frame = Frame3 { origin: Pnt3::new(0.0, 0.0, 2.0), x, y, z: n };
    let e = Curve3::Ellipse { frame: curve_frame, major_radius: radius / cos_theta, minor_radius: radius };
    let tol = 1e-4;
    let domain = (0.0, 4.0);
    let pc = s.project_curve(&e, domain, tol);
    for i in 0..=40 {
        let s_param = pc.domain().0 + (pc.domain().1 - pc.domain().0) * i as f64 / 40.0;
        let traced = s.eval_pcurve(&pc, s_param);
        let nearest = curve_ops::closest_parameter(&e, (domain.0, domain.1), traced, 1e-12).distance;
        assert!(nearest < tol * 20.0, "traced point at s={s_param} is {nearest} from the true curve, exceeding bound");
    }
}

#[semio_framework_async_macros::async_test]
async fn project_curve_pieces_handles_a_seam_crossing_curve_on_a_cylinder() {
    let frame = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    let s = Surface::Cylinder { frame, radius: 2.0 };
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
