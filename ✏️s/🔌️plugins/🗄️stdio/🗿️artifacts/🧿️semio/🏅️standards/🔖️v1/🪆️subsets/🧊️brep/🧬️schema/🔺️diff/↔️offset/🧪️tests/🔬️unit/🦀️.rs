
use super::*;
use crate::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_cylinder, make_planar_face_from_points, make_sphere};
use crate::standards::v1::subsets::brep::schema::inferences::mass_properties::solid_volume;
use std::f64::consts::PI;

#[semio_framework_async_macros::async_test]
async fn debug_plain_cylinder_volume() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_cylinder(&mut body, 1.0, 2.0, &mut rec).unwrap();
    let v = solid_volume(&body, solid, 1e-4).unwrap();
    let expected = PI * 1.0 * 1.0 * 2.0;
    println!("[DEBUG] plain cylinder v={v} expected={expected}");
}

#[semio_framework_async_macros::async_test]
async fn offset_solid_box_sharp_matches_closed_form() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (a, b, c, d) = (2.0, 3.0, 1.5, 0.3);
    let solid = make_box(&mut body, a, b, c, &mut rec).unwrap();
    let grown = offset_solid_with_corner(&mut body, solid, d, OffsetCorner::Sharp, &mut rec).unwrap();
    let v = solid_volume(&body, grown, 1e-6).unwrap();
    let closed_form = (a + 2.0 * d) * (b + 2.0 * d) * (c + 2.0 * d);
    assert!((v - closed_form).abs() < 1e-6, "v={v} expected={closed_form}");
}

#[semio_framework_async_macros::async_test]
async fn offset_solid_box_round_matches_minkowski_closed_form() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (a, b, c, r) = (2.0, 3.0, 1.5, 0.3);
    let solid = make_box(&mut body, a, b, c, &mut rec).unwrap();
    let grown = offset_solid_with_corner(&mut body, solid, r, OffsetCorner::Round, &mut rec).unwrap();
    let v = solid_volume(&body, grown, 1e-4).unwrap();
    let closed_form = a * b * c + 2.0 * r * (a * b + b * c + c * a) + PI * r * r * (a + b + c) + (4.0 / 3.0) * PI * r * r * r;
    assert!((v - closed_form).abs() < 1e-2 * closed_form, "v={v} expected={closed_form}");
}

#[semio_framework_async_macros::async_test]
async fn offset_cylinder_matches_closed_form() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (r, h, d) = (1.0, 2.0, 0.25);
    let solid = make_cylinder(&mut body, r, h, &mut rec).unwrap();
    let grown = offset_solid_with_corner(&mut body, solid, d, OffsetCorner::Sharp, &mut rec).unwrap();
    let v = solid_volume(&body, grown, 1e-4).unwrap();
    let closed_form = PI * (r + d) * (r + d) * h;
    assert!((v - closed_form).abs() < 1e-2 * closed_form, "v={v} expected={closed_form}");
}

#[semio_framework_async_macros::async_test]
async fn offset_sphere_matches_closed_form() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (r, d) = (1.0, 0.4);
    let solid = make_sphere(&mut body, r, &mut rec).unwrap();
    let grown = offset_solid_with_corner(&mut body, solid, d, OffsetCorner::Sharp, &mut rec).unwrap();
    let v = solid_volume(&body, grown, 1e-4).unwrap();
    let closed_form = (4.0 / 3.0) * PI * (r + d).powi(3);
    assert!((v - closed_form).abs() < 1e-2 * closed_form, "v={v} expected={closed_form}");
}

#[semio_framework_async_macros::async_test]
async fn offset_nurbs_surface_within_bound() {
    use crate::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector;
    let u_knots = KnotVector::clamped_uniform(3, 2);
    let v_knots = KnotVector::clamped_uniform(3, 2);
    let controls = vec![
        vec![Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(0.0, 1.0, 0.1), Pnt3::new(0.0, 2.0, 0.0)],
        vec![Pnt3::new(1.0, 0.0, 0.1), Pnt3::new(1.0, 1.0, 0.4), Pnt3::new(1.0, 2.0, 0.1)],
        vec![Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 1.0, 0.1), Pnt3::new(2.0, 2.0, 0.0)],
    ];
    let weights = vec![vec![1.0; 3]; 3];
    let surface = Surface::Nurbs { u_knots, v_knots, controls, weights };
    let tol = 1e-4;
    let offset = offset_surface(&surface, 0.15, tol).unwrap();
    let (u0, u1) = surface.domain().0;
    let (v0, v1) = surface.domain().1;
    for i in 0..6 {
        for j in 0..6 {
            let u = u0 + (u1 - u0) * (i as f64 + 0.5) / 6.0;
            let v = v0 + (v1 - v0) * (j as f64 + 0.5) / 6.0;
            let n = surface.normal(u, v).unwrap();
            let truth = surface.eval(u, v) + n * 0.15;
            let got = offset.eval(u, v);
            assert!(truth.distance(got) <= tol * 4.0, "deviation {} at ({u},{v})", truth.distance(got));
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn thicken_planar_face_matches_box_volume() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let face = make_planar_face_from_points(&mut body, &[Pnt3::new(0.0, 0.0, 0.0), Pnt3::new(2.0, 0.0, 0.0), Pnt3::new(2.0, 1.0, 0.0), Pnt3::new(0.0, 1.0, 0.0)], &mut rec).unwrap();
    let solid = thicken_face(&mut body, face, 0.5, &mut rec).unwrap();
    let v = solid_volume(&body, solid, 1e-6).unwrap();
    assert!((v - 1.0).abs() < 1e-6, "volume {v}");
}

#[semio_framework_async_macros::async_test]
async fn shell_box_one_open_face_matches_closed_form() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (a, b, c, t) = (2.0, 2.0, 2.0, 0.2);
    let solid = make_box(&mut body, a, b, c, &mut rec).unwrap();
    let top = *body.solid_faces(solid).iter().find(|&&f| matches!(body.surfaces.get(body.faces.get(f).unwrap().surface).unwrap(), Surface::Plane { frame } if (frame.origin.z - c).abs() < 1e-9)).unwrap();
    let shelled = shell_solid_with_open_faces(&mut body, solid, t, &[top], &mut rec).unwrap();
    let v = solid_volume(&body, shelled, 1e-6).unwrap();
    let outer = a * b * c;
    let inner = (a - 2.0 * t) * (b - 2.0 * t) * (c - t);
    let closed_form = outer - inner;
    assert!((v - closed_form).abs() < 1e-3 * closed_form.max(1.0), "v={v} expected={closed_form}");
}

#[semio_framework_async_macros::async_test]
async fn shell_box_fully_closed_matches_closed_form() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (a, b, c, t) = (2.0, 3.0, 1.5, 0.2);
    let solid = make_box(&mut body, a, b, c, &mut rec).unwrap();
    let shelled = shell_solid(&mut body, solid, t, &mut rec).unwrap();
    let v = solid_volume(&body, shelled, 1e-6).unwrap();
    let closed_form = a * b * c - (a - 2.0 * t) * (b - 2.0 * t) * (c - 2.0 * t);
    assert!((v - closed_form).abs() < 1e-6, "v={v} expected={closed_form}");
}

#[semio_framework_async_macros::async_test]
async fn draft_box_side_face_matches_trapezoid_magnitude() {
    let (a, b, c, angle) = (1.0, 1.0, 1.0, 0.2_f64);
    let mut body_plus = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body_plus, a, b, c, &mut rec).unwrap();
    let right = *body_plus.solid_faces(solid).iter().find(|&&f| matches!(body_plus.surfaces.get(body_plus.faces.get(f).unwrap().surface).unwrap(), Surface::Plane { frame } if (frame.origin.x - a).abs() < 1e-9)).unwrap();
    let v0 = solid_volume(&body_plus, solid, 1e-6).unwrap();
    let drafted_plus = draft_angle(&mut body_plus, solid, &[right], Vec3::Z, (Pnt3::new(0.0, 0.0, 0.0), Vec3::Z), angle, &mut rec).unwrap();
    let v_plus = solid_volume(&body_plus, drafted_plus, 1e-6).unwrap();

    let mut body_minus = Body::new();
    let mut rec2 = OpRecorder::new();
    let solid2 = make_box(&mut body_minus, a, b, c, &mut rec2).unwrap();
    let right2 = *body_minus.solid_faces(solid2).iter().find(|&&f| matches!(body_minus.surfaces.get(body_minus.faces.get(f).unwrap().surface).unwrap(), Surface::Plane { frame } if (frame.origin.x - a).abs() < 1e-9)).unwrap();
    let drafted_minus = draft_angle(&mut body_minus, solid2, &[right2], Vec3::Z, (Pnt3::new(0.0, 0.0, 0.0), Vec3::Z), -angle, &mut rec2).unwrap();
    let v_minus = solid_volume(&body_minus, drafted_minus, 1e-6).unwrap();

    let expected_delta = b * c * c * angle.tan() / 2.0;
    assert!(v_plus != v_minus, "draft should change volume asymmetrically: {v_plus} vs {v_minus}");
    assert!(((v_plus - v0).abs() - expected_delta).abs() < 1e-4, "v_plus={v_plus} v0={v0} expected_delta={expected_delta}");
    assert!(((v_minus - v0).abs() - expected_delta).abs() < 1e-4, "v_minus={v_minus} v0={v0} expected_delta={expected_delta}");
    assert!((v_plus + v_minus - 2.0 * v0).abs() < 1e-6, "draft should be symmetric around the undrafted volume");
}

#[semio_framework_async_macros::async_test]
async fn draft_zero_angle_errors() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let face = body.solid_faces(solid)[0];
    let err = draft_angle(&mut body, solid, &[face], Vec3::Z, (Pnt3::new(0.0, 0.0, 0.0), Vec3::Z), 0.0, &mut rec).unwrap_err();
    assert!(matches!(err, KernelError::Operation(_)));
}

#[semio_framework_async_macros::async_test]
async fn debug_offset_cylinder2() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let (r, h, d) = (1.0, 2.0, 0.25);
    let solid = make_cylinder(&mut body, r, h, &mut rec).unwrap();
    let grown = offset_solid_with_corner(&mut body, solid, d, OffsetCorner::Sharp, &mut rec).unwrap();
    use crate::standards::v1::subsets::brep::schema::inferences::mass_properties::face_area;
    for f in body.solid_faces(grown) {
        let fd = body.faces.get(f).unwrap();
        println!("[DEBUG] face {f:?} flipped={} area={:?} surface={:?}", fd.flipped, face_area(&body, f, 1e-4), body.surfaces.get(fd.surface));
        if let Some(o) = fd.outer {
            for cid in body.loop_coedges(o) {
                let c = body.coedges.get(cid).unwrap();
                let e = body.edges.get(c.edge).unwrap();
                println!("[DEBUG]   coedge edge={:?} forward={} range={:?} v0={:?} v1={:?} curve={:?}", c.edge, c.forward, e.range, e.v0, e.v1, body.curves3.get(e.curve));
            }
        }
    }
    let v = solid_volume(&body, grown, 1e-4);
    println!("[DEBUG] volume={v:?}");
}

#[semio_framework_async_macros::async_test]
async fn offset_determinism_face_count_and_volume() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let a = offset_solid(&mut body, solid, 0.1, &mut rec).unwrap();
    let b = offset_solid(&mut body, solid, 0.1, &mut rec).unwrap();
    assert_eq!(body.solid_faces(a).len(), body.solid_faces(b).len());
    let va = solid_volume(&body, a, 1e-6).unwrap();
    let vb = solid_volume(&body, b, 1e-6).unwrap();
    assert!((va - vb).abs() < 1e-9, "va={va} vb={vb}");
}
