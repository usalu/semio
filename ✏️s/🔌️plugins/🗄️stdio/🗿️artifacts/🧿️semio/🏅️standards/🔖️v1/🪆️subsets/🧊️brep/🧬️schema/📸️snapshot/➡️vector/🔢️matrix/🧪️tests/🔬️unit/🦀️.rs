use super::*;

#[semio_framework_async_macros::async_test]
async fn quat_from_axis_angle_rotates_correctly() {
    let q = Quat::from_axis_angle(Vec3::Z, std::f64::consts::FRAC_PI_2);
    let r = q.rotate(Vec3::X);
    assert!((r - Vec3::Y).norm() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn quat_conjugate_is_inverse_rotation() {
    let q = Quat::from_axis_angle(Vec3::new(1.0, 2.0, 3.0), 0.7);
    let v = Vec3::new(4.0, -1.0, 2.0);
    let round_trip = q.conjugate().rotate(q.rotate(v));
    assert!((round_trip - v).norm() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn quat_to_mat3_matches_direct_rotation() {
    let q = Quat::from_axis_angle(Vec3::new(0.3, 0.7, -0.2), 1.1);
    let v = Vec3::new(1.0, 0.0, 0.0);
    let via_quat = q.rotate(v);
    let via_mat = q.to_mat3().transform(v);
    assert!((via_quat - via_mat).norm() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn slerp_endpoints_match_inputs() {
    let a = Quat::from_axis_angle(Vec3::Z, 0.0);
    let b = Quat::from_axis_angle(Vec3::Z, 1.5);
    let s0 = a.slerp(b, 0.0);
    let s1 = a.slerp(b, 1.0);
    assert!((s0.rotate(Vec3::X) - a.rotate(Vec3::X)).norm() < 1e-9);
    assert!((s1.rotate(Vec3::X) - b.rotate(Vec3::X)).norm() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn trsf_inverse_round_trips_a_point() {
    let t = Trsf { rotation: Quat::from_axis_angle(Vec3::new(1.0, 1.0, 0.0), 0.9), translation: Vec3::new(5.0, -2.0, 3.0), scale: 2.5 };
    let p = Pnt3::new(1.0, 2.0, 3.0);
    let round_trip = t.inverse().apply_point(t.apply_point(p));
    assert!(round_trip.distance(p) < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn trsf_compose_matches_sequential_application() {
    let a = Trsf { rotation: Quat::from_axis_angle(Vec3::Z, 0.4), translation: Vec3::new(1.0, 0.0, 0.0), scale: 1.0 };
    let b = Trsf { rotation: Quat::from_axis_angle(Vec3::X, 0.9), translation: Vec3::new(0.0, 2.0, 0.0), scale: 1.5 };
    let p = Pnt3::new(3.0, -1.0, 2.0);
    let composed = a.semio_compose_rs(&b).apply_point(p);
    let sequential = a.apply_point(b.apply_point(p));
    assert!(composed.distance(sequential) < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn frame_from_normal_round_trips_local_world() {
    let f = Frame3::from_normal(Pnt3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 5.0)).unwrap();
    let local = Pnt3::new(2.0, -1.0, 0.5);
    let round_trip = f.to_local(f.to_world(local));
    assert!(round_trip.distance(local) < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn frame_from_normal_is_right_handed() {
    let f = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z).unwrap();
    assert!((f.x.cross(f.y) - f.z).norm() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn frame_from_normal_deterministic_for_axis_aligned_normals() {
    let f1 = Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::X).unwrap();
    let f2 = Frame3::from_normal(Pnt3::new(5.0, 5.0, 5.0), Vec3::X).unwrap();
    assert_eq!(f1.x, f2.x);
    assert_eq!(f1.y, f2.y);
}

#[semio_framework_async_macros::async_test]
async fn mat3_determinant_of_identity_is_one() {
    assert!((Mat3::IDENTITY.determinant() - 1.0).abs() < 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn mat3_from_axis_angle_matches_quat_rotation() {
    let axis = Vec3::new(0.2, -0.4, 0.9);
    let angle = 1.3;
    let m = Mat3::from_axis_angle(axis, angle);
    let q = Quat::from_axis_angle(axis, angle);
    let v = Vec3::new(1.0, 2.0, -3.0);
    assert!((m.transform(v) - q.rotate(v)).norm() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn affine_translation_round_trips_via_inverse() {
    let a = Affine3::translation(Vec3::new(3.0, -2.0, 5.0));
    let p = Pnt3::new(1.0, 2.0, 3.0);
    let round_trip = a.inverse().unwrap().apply_point(a.apply_point(p));
    assert!(round_trip.distance(p) < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn affine_from_trsf_matches_trsf_apply_point() {
    let t = Trsf { rotation: Quat::from_axis_angle(Vec3::new(1.0, 1.0, 0.0), 0.9), translation: Vec3::new(5.0, -2.0, 3.0), scale: 2.5 };
    let a = Affine3::from_trsf(&t);
    let p = Pnt3::new(1.0, 2.0, 3.0);
    assert!(a.apply_point(p).distance(t.apply_point(p)) < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn affine_rotation_about_fixes_its_own_origin() {
    let origin = Pnt3::new(4.0, -1.0, 2.0);
    let a = Affine3::rotation_about(origin, Vec3::Z, 1.234);
    assert!(a.apply_point(origin).distance(origin) < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn affine_scaling_fixes_center_and_scales_offset() {
    let center = Pnt3::new(1.0, 1.0, 1.0);
    let a = Affine3::scaling(center, Vec3::new(2.0, 3.0, 4.0));
    assert!(a.apply_point(center).distance(center) < 1e-9);
    let p = center + Vec3::new(1.0, 1.0, 1.0);
    let mapped = a.apply_point(p);
    assert!(mapped.distance(center + Vec3::new(2.0, 3.0, 4.0)) < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn affine_mirror_fixes_plane_and_flips_normal_side() {
    let origin = Pnt3::new(0.0, 0.0, 0.0);
    let a = Affine3::mirror(origin, Vec3::Z);
    assert!(a.apply_point(origin).distance(origin) < 1e-9);
    assert!((a.determinant() + 1.0).abs() < 1e-9);
    let above = Pnt3::new(1.0, 2.0, 3.0);
    let mapped = a.apply_point(above);
    assert!((mapped.z + 3.0).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn affine_compose_matches_sequential_application() {
    let a = Affine3::rotation_axis_angle(Vec3::Z, 0.4).compose(&Affine3::translation(Vec3::new(1.0, 0.0, 0.0)));
    let b = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(1.5, 1.5, 1.5)).compose(&Affine3::translation(Vec3::new(0.0, 2.0, 0.0)));
    let p = Pnt3::new(3.0, -1.0, 2.0);
    let composed = a.compose(&b).apply_point(p);
    let sequential = a.apply_point(b.apply_point(p));
    assert!(composed.distance(sequential) < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn affine_is_similarity_recognizes_rotation_translation_uniform_scale() {
    let t = Trsf { rotation: Quat::from_axis_angle(Vec3::new(0.2, 0.7, -0.3), 1.1), translation: Vec3::new(1.0, 2.0, 3.0), scale: 3.5 };
    let a = Affine3::from_trsf(&t);
    let (rotation, scale, is_reflection) = a.is_similarity().expect("rigid+uniform-scale must be recognized as a similarity");
    assert!((scale - 3.5).abs() < 1e-9);
    assert!(!is_reflection);
    let v = Vec3::new(1.0, 0.0, 0.0);
    assert!((rotation.rotate(v) - t.rotation.rotate(v)).norm() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn affine_is_similarity_recognizes_reflection() {
    let a = Affine3::mirror(Pnt3::new(0.0, 0.0, 0.0), Vec3::Z);
    let (_, scale, is_reflection) = a.is_similarity().expect("a mirror must be recognized as a similarity with reflection");
    assert!((scale - 1.0).abs() < 1e-9);
    assert!(is_reflection);
}

#[semio_framework_async_macros::async_test]
async fn affine_is_similarity_rejects_non_uniform_scale() {
    let a = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0));
    assert!(a.is_similarity().is_none());
}

#[semio_framework_async_macros::async_test]
async fn affine_apply_normal_matches_rotation_for_similarity() {
    let a = Affine3::rotation_axis_angle(Vec3::new(0.3, 0.6, 0.1), 0.8);
    let n = Vec3::new(0.0, 0.0, 1.0).normalized().unwrap();
    let via_normal = a.apply_normal(n).normalized().unwrap();
    let via_vector = a.apply_vector(n).normalized().unwrap();
    assert!((via_normal - via_vector).norm() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn affine_max_singular_value_matches_scale_for_similarity() {
    let a = Affine3::from_trsf(&Trsf { rotation: Quat::from_axis_angle(Vec3::new(0.1, 1.0, 0.2), 0.5), translation: Vec3::ZERO, scale: 4.2 });
    assert!((a.max_singular_value() - 4.2).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn affine_max_singular_value_matches_largest_scale_factor_for_diagonal_scale() {
    let a = Affine3::scaling(Pnt3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 5.0, 3.0));
    assert!((a.max_singular_value() - 5.0).abs() < 1e-9);
}

mod quick {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn trsf_inverse_round_trips_random_points() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(7);
        for _ in 0..200 {
            let axis = Vec3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5);
            let angle = rng.next_f64() * std::f64::consts::TAU;
            let t = Trsf { rotation: Quat::from_axis_angle(axis, angle), translation: Vec3::new(rng.next_f64() * 10.0, rng.next_f64() * 10.0, rng.next_f64() * 10.0), scale: 0.1 + rng.next_f64() * 5.0 };
            let p = Pnt3::new(rng.next_f64() * 10.0 - 5.0, rng.next_f64() * 10.0 - 5.0, rng.next_f64() * 10.0 - 5.0);
            let round_trip = t.inverse().apply_point(t.apply_point(p));
            assert!(round_trip.distance(p) < 1e-7, "round trip drifted: {round_trip:?} vs {p:?}");
        }
    }
}
