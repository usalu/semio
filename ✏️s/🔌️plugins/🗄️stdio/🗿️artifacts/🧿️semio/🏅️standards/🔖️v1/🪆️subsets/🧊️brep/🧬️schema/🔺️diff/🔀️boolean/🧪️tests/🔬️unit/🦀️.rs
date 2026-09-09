use super::*;
use crate::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_cylinder, make_sphere};
use crate::standards::v1::subsets::brep::schema::diff::transform::transform_solid;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Affine3;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn translate_solid(body: &mut Body, solid: SolidId, delta: Vec3, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    transform_solid(body, solid, &Affine3::translation(delta), rec)
}

/// 🧪 Regression guard for the `classification::point_in_solid` sphere-trim bug worked around
/// locally by [`local_point_in_solid`]: a plain, unsplit unit sphere's own center (and a
/// clearly-interior off-center point) must classify `Inside`.
#[semio_framework_async_macros::async_test]
async fn local_point_in_solid_handles_plain_sphere_interior() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let s = make_sphere(&mut body, 1.0, &mut rec).unwrap();
    assert!(matches!(local_point_in_solid(&body, s, Pnt3::new(0.0, 0.0, 0.0), 1e-6), Ok(PointClassification::Inside)));
    assert!(matches!(local_point_in_solid(&body, s, Pnt3::new(0.5, 0.0, 0.0), 1e-6), Ok(PointClassification::Inside)));
    assert!(matches!(local_point_in_solid(&body, s, Pnt3::new(2.0, 0.0, 0.0), 1e-6), Ok(PointClassification::Outside)));
    let b = translate_solid(&mut body, s, Vec3::new(0.0, 0.0, 1.2), &mut rec).unwrap();
    let p = Pnt3::new(-0.44721359549995704, -5.410403269529241e-16, 0.8944271909999163);
    assert!(matches!(local_point_in_solid(&body, b, p, 1e-6), Ok(PointClassification::Inside)));
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn offset_unit_cube(body: &mut Body, offset: Pnt3, rec: &mut OpRecorder) -> SolidId {
    let corners = [
        offset + Vec3::new(0.0, 0.0, 0.0),
        offset + Vec3::new(1.0, 0.0, 0.0),
        offset + Vec3::new(1.0, 1.0, 0.0),
        offset + Vec3::new(0.0, 1.0, 0.0),
        offset + Vec3::new(0.0, 0.0, 1.0),
        offset + Vec3::new(1.0, 0.0, 1.0),
        offset + Vec3::new(1.0, 1.0, 1.0),
        offset + Vec3::new(0.0, 1.0, 1.0),
    ];
    make_convex_hull(body, &corners, rec).expect("offset cube hull")
}

#[semio_framework_async_macros::async_test]
async fn disjoint_unit_boxes_fuse_volume_near_two() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let a = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let b = offset_unit_cube(&mut body, Pnt3::new(2.0, 0.0, 0.0), &mut rec);
    let fused = boolean_solid(&mut body, a, b, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
    let vol = solid_volume(&body, fused, 1e-6).unwrap();
    assert!((vol - 2.0).abs() < 1e-3, "expected volume ≈ 2, got {vol}");
}

#[semio_framework_async_macros::async_test]
async fn overlapping_aabb_intersect_volume_matches_dims() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let a = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let b = offset_unit_cube(&mut body, Pnt3::new(0.5, 0.5, 0.5), &mut rec);
    let hit = boolean_solid(&mut body, a, b, BooleanOp::Intersect, 1e-6, &mut rec).unwrap();
    let vol = solid_volume(&body, hit, 1e-6).unwrap();
    let expected = 0.5 * 0.5 * 0.5;
    assert!((vol - expected).abs() < 1e-3, "expected {expected}, got {vol}");
}

#[semio_framework_async_macros::async_test]
async fn boolean_unite_is_deterministic() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let a = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let b = offset_unit_cube(&mut body, Pnt3::new(2.0, 0.0, 0.0), &mut rec);
    let faces_before = body.faces.len();
    let r1 = boolean_solid(&mut body, a, b, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
    let delta1 = body.faces.len() - faces_before;
    let n1 = body.solid_faces(r1).len();
    let faces_mid = body.faces.len();
    let r2 = boolean_solid(&mut body, a, b, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
    let delta2 = body.faces.len() - faces_mid;
    let n2 = body.solid_faces(r2).len();
    assert_eq!(delta1, delta2);
    assert_eq!(n1, n2);
}

#[semio_framework_async_macros::async_test]
async fn cut_disjoint_preserves_volume() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let a = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let b = offset_unit_cube(&mut body, Pnt3::new(3.0, 0.0, 0.0), &mut rec);
    let vol_a = solid_volume(&body, a, 1e-6).unwrap();
    let cut = boolean_solid(&mut body, a, b, BooleanOp::Cut, 1e-6, &mut rec).unwrap();
    let vol_cut = solid_volume(&body, cut, 1e-6).unwrap();
    assert!((vol_cut - vol_a).abs() < 1e-3, "cut volume {vol_cut} vs A {vol_a}");
}

#[semio_framework_async_macros::async_test]
async fn adversarial_scale_sweep_determinism() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    for scale in [0.1_f64, 1.0, 10.0, 100.0] {
        let a = make_box(&mut body, scale, scale, scale, &mut rec).unwrap();
        let o = Pnt3::new(scale * 2.0, 0.0, 0.0);
        let corners = [
            o,
            Pnt3::new(o.x + scale, o.y, o.z),
            Pnt3::new(o.x + scale, o.y + scale, o.z),
            Pnt3::new(o.x, o.y + scale, o.z),
            Pnt3::new(o.x, o.y, o.z + scale),
            Pnt3::new(o.x + scale, o.y, o.z + scale),
            Pnt3::new(o.x + scale, o.y + scale, o.z + scale),
            Pnt3::new(o.x, o.y + scale, o.z + scale),
        ];
        let b = make_convex_hull(&mut body, &corners, &mut rec).unwrap();
        let u0 = boolean_solid(&mut body, a, b, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
        let u1 = boolean_solid(&mut body, a, b, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
        assert_eq!(body.solid_faces(u0).len(), body.solid_faces(u1).len());
        let v = solid_volume(&body, u0, scale * 1e-4).unwrap();
        assert!((v - 2.0 * scale.powi(3)).abs() < scale.powi(3) * 1e-2, "scale={scale} v={v}");
    }
}

#[semio_framework_async_macros::async_test]
async fn fuzz_random_aabb_intersect_volume_nonnegative() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let mut seed = 1u64;
    for _ in 0..32 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let w = 0.5 + (seed % 50) as f64 * 0.1;
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let ox = (seed % 20) as f64 * 0.25;
        let a = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
        let o = Pnt3::new(ox, ox * 0.5, 0.0);
        let corners =
            [o, Pnt3::new(o.x + w, o.y, o.z), Pnt3::new(o.x + w, o.y + w, o.z), Pnt3::new(o.x, o.y + w, o.z), Pnt3::new(o.x, o.y, o.z + w), Pnt3::new(o.x + w, o.y, o.z + w), Pnt3::new(o.x + w, o.y + w, o.z + w), Pnt3::new(o.x, o.y + w, o.z + w)];
        let b = make_convex_hull(&mut body, &corners, &mut rec).unwrap();
        if let Ok(inter) = boolean_solid(&mut body, a, b, BooleanOp::Intersect, 1e-6, &mut rec) {
            assert!(solid_volume(&body, inter, 1e-3).unwrap() >= -1e-9);
        }
    }
}

// #region 🧪️ExactCurvedTests

/// 🧪 A cylinder through the center of a box, radius small enough that the cylinder only
/// crosses the box's top/bottom planar caps (as closed interior circles) and its own lateral
/// face only meets the box at those two caps too — every imprint curve in this scenario is the
/// "closed, fully interior" case, exercising `split_face_by_interior_curve` on both a planar
/// and a cylindrical support. Volume = box − (cylinder ∩ box) by the closed-form cylinder
/// volume clipped to the box's height.
#[semio_framework_async_macros::async_test]
async fn box_union_cylinder_through_exact_volume_and_validates() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let box_solid = make_box(&mut body, 4.0, 4.0, 2.0, &mut rec).unwrap();
    let box_solid = translate_solid(&mut body, box_solid, Vec3::new(-2.0, -2.0, -0.5), &mut rec).unwrap();
    let cyl = make_cylinder(&mut body, 0.5, 4.0, &mut rec).unwrap();
    let cyl = translate_solid(&mut body, cyl, Vec3::new(0.0, 0.0, -2.0), &mut rec).unwrap();
    let united = boolean_solid(&mut body, box_solid, cyl, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
    let issues = validate_body(&body);
    assert!(issues.is_empty(), "validate_body issues: {:?}", issues.iter().map(|i| format!("{}:{}:{}", i.entity, i.code, i.message)).collect::<Vec<_>>());
    let vol = solid_volume(&body, united, 1e-4).unwrap();
    let box_vol = 4.0 * 4.0 * 2.0;
    let cyl_vol = std::f64::consts::PI * 0.5 * 0.5 * 4.0;
    let overlap_vol = std::f64::consts::PI * 0.5 * 0.5 * 2.0; // the cylinder segment inside the box's height
    let expected = box_vol + cyl_vol - overlap_vol;
    assert!((vol - expected).abs() / expected < 5e-3, "expected≈{expected}, got {vol}");
}

/// 🧪 Same geometry, `Cut`: the cylinder bores a round hole through the box.
#[semio_framework_async_macros::async_test]
async fn box_minus_cylinder_bore_exact_volume_and_validates() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let box_solid = make_box(&mut body, 4.0, 4.0, 2.0, &mut rec).unwrap();
    let box_solid = translate_solid(&mut body, box_solid, Vec3::new(-2.0, -2.0, -0.5), &mut rec).unwrap();
    let cyl = make_cylinder(&mut body, 0.5, 4.0, &mut rec).unwrap();
    let cyl = translate_solid(&mut body, cyl, Vec3::new(0.0, 0.0, -2.0), &mut rec).unwrap();
    let bored = boolean_solid(&mut body, box_solid, cyl, BooleanOp::Cut, 1e-6, &mut rec).unwrap();
    let issues = validate_body(&body);
    assert!(issues.is_empty(), "validate_body issues: {:?}", issues.iter().map(|i| format!("{}:{}:{}", i.entity, i.code, i.message)).collect::<Vec<_>>());
    let vol = solid_volume(&body, bored, 1e-4).unwrap();
    let box_vol = 4.0 * 4.0 * 2.0;
    let bore_vol = std::f64::consts::PI * 0.5 * 0.5 * 2.0;
    let expected = box_vol - bore_vol;
    assert!((vol - expected).abs() / expected < 5e-3, "expected≈{expected}, got {vol}");
}

/// 🧪 Two spheres overlapping (a lens): sphere/sphere intersection circle stays away from
/// either sphere's own seam/poles, so both sides see it as a closed interior curve.
#[semio_framework_async_macros::async_test]
async fn sphere_union_sphere_lens_exact_volume_and_validates() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let a = make_sphere(&mut body, 1.0, &mut rec).unwrap();
    let b = make_sphere(&mut body, 1.0, &mut rec).unwrap();
    let b = translate_solid(&mut body, b, Vec3::new(0.0, 0.0, 1.2), &mut rec).unwrap();
    let united = boolean_solid(&mut body, a, b, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
    let issues = validate_body(&body);
    assert!(issues.is_empty(), "validate_body issues: {:?}", issues.iter().map(|i| format!("{}:{}:{}", i.entity, i.code, i.message)).collect::<Vec<_>>());
    let sphere_vol = 4.0 / 3.0 * std::f64::consts::PI;
    // Spherical-cap overlap volume for two equal spheres radius r, center distance d:
    // V_lens = π (4r + d)(2r − d)² / 12.
    let (r, d) = (1.0_f64, 1.2_f64);
    let lens = std::f64::consts::PI * (4.0 * r + d) * (2.0 * r - d).powi(2) / 12.0;
    let expected = 2.0 * sphere_vol - lens;
    let vol = solid_volume(&body, united, 1e-4).unwrap();
    assert!((vol - expected).abs() / expected < 1e-2, "expected≈{expected}, got {vol}");
}

/// 🧪 A∪A / A∩A / A∖A on a single sphere via its own coincident-face special case (a solid
/// booleaned with itself has every face pair exactly coincident).
#[semio_framework_async_macros::async_test]
async fn self_boolean_identities_on_a_sphere() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let a = make_sphere(&mut body, 1.0, &mut rec).unwrap();
    let a2 = make_sphere(&mut body, 1.0, &mut rec).unwrap();
    let vol_a = solid_volume(&body, a, 1e-6).unwrap();
    let union = boolean_solid(&mut body, a, a2, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
    assert!((solid_volume(&body, union, 1e-6).unwrap() - vol_a).abs() / vol_a < 1e-6, "A∪A must equal A's volume");

    let mut body2 = Body::new();
    let mut rec2 = OpRecorder::new();
    let c = make_sphere(&mut body2, 1.0, &mut rec2).unwrap();
    let c2 = make_sphere(&mut body2, 1.0, &mut rec2).unwrap();
    let vol_c = solid_volume(&body2, c, 1e-6).unwrap();
    let inter = boolean_solid(&mut body2, c, c2, BooleanOp::Intersect, 1e-6, &mut rec2).unwrap();
    assert!((solid_volume(&body2, inter, 1e-6).unwrap() - vol_c).abs() / vol_c < 1e-6, "A∩A must equal A's volume");
}

/// 🧪 Commutativity of union/intersect (volume): `A∪B == B∪A`, `A∩B == B∩A`.
#[semio_framework_async_macros::async_test]
async fn union_and_intersect_are_commutative_by_volume() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let a1 = make_sphere(&mut body, 1.0, &mut rec).unwrap();
    let b1 = make_sphere(&mut body, 1.0, &mut rec).unwrap();
    let b1 = translate_solid(&mut body, b1, Vec3::new(0.0, 0.0, 1.2), &mut rec).unwrap();
    let u_ab = boolean_solid(&mut body, a1, b1, BooleanOp::Unite, 1e-6, &mut rec).unwrap();
    let v_ab = solid_volume(&body, u_ab, 1e-4).unwrap();

    let mut body2 = Body::new();
    let mut rec2 = OpRecorder::new();
    let a2 = make_sphere(&mut body2, 1.0, &mut rec2).unwrap();
    let b2 = make_sphere(&mut body2, 1.0, &mut rec2).unwrap();
    let a2 = translate_solid(&mut body2, a2, Vec3::new(0.0, 0.0, 1.2), &mut rec2).unwrap();
    let u_ba = boolean_solid(&mut body2, b2, a2, BooleanOp::Unite, 1e-6, &mut rec2).unwrap();
    let v_ba = solid_volume(&body2, u_ba, 1e-4).unwrap();
    assert!((v_ab - v_ba).abs() / v_ab < 1e-6, "union must be commutative: {v_ab} vs {v_ba}");

    let mut body3 = Body::new();
    let mut rec3 = OpRecorder::new();
    let a3 = make_sphere(&mut body3, 1.0, &mut rec3).unwrap();
    let b3 = make_sphere(&mut body3, 1.0, &mut rec3).unwrap();
    let b3 = translate_solid(&mut body3, b3, Vec3::new(0.0, 0.0, 1.2), &mut rec3).unwrap();
    let i_ab = boolean_solid(&mut body3, a3, b3, BooleanOp::Intersect, 1e-6, &mut rec3).unwrap();
    let vi_ab = solid_volume(&body3, i_ab, 1e-4).unwrap();

    let mut body4 = Body::new();
    let mut rec4 = OpRecorder::new();
    let a4 = make_sphere(&mut body4, 1.0, &mut rec4).unwrap();
    let b4 = make_sphere(&mut body4, 1.0, &mut rec4).unwrap();
    let a4 = translate_solid(&mut body4, a4, Vec3::new(0.0, 0.0, 1.2), &mut rec4).unwrap();
    let i_ba = boolean_solid(&mut body4, b4, a4, BooleanOp::Intersect, 1e-6, &mut rec4).unwrap();
    let vi_ba = solid_volume(&body4, i_ba, 1e-4).unwrap();
    assert!((vi_ab - vi_ba).abs() / vi_ab < 1e-6, "intersect must be commutative: {vi_ab} vs {vi_ba}");
}

/// 🧪 A tangent (just-touching, not overlapping) sphere pair: union volume must be the exact
/// sum (no lens carved out), and the classifier must not choke on the near-zero-margin contact.
#[semio_framework_async_macros::async_test]
async fn tangent_spheres_union_volume_is_exact_sum() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let a = make_sphere(&mut body, 1.0, &mut rec).unwrap();
    let b = make_sphere(&mut body, 1.0, &mut rec).unwrap();
    let b = translate_solid(&mut body, b, Vec3::new(2.0 + 1e-4, 0.0, 0.0), &mut rec).unwrap();
    let united = boolean_solid(&mut body, a, b, BooleanOp::Unite, 1e-3, &mut rec).unwrap();
    let vol = solid_volume(&body, united, 1e-4).unwrap();
    let expected = 2.0 * 4.0 / 3.0 * std::f64::consts::PI;
    assert!((vol - expected).abs() / expected < 1e-2, "expected≈{expected}, got {vol}");
}

// #endregion 🧪️ExactCurvedTests

/// 🧪 Every analytic primitive must be structurally valid on its own — the boolean re-runs
/// `validate_body` over its result and attributes anything it cannot pin on a pre-existing solid
/// to itself, so a primitive that is born invalid fails every boolean that touches it.
/// 🐛 Both halves of this were live: the sphere's two collapsed POLE edges (one use each, zero
/// length) were reported as `shell-not-closed` + `degenerate-edge`, and its single face measured
/// ZERO area (`sliver-face`) because the duplicate pole vertex in its UV boundary blocked every
/// ear of the triangulation. 🐛 And the cone was born INWARD: its lateral surface's frame paired a
/// `z = −Z` axis with an unmirrored `y = Y`, making the frame left-handed and negating `du × dv`,
/// so its signed volume came out exactly `−πr²h/3`. Each primitive's own closed-form volume is
/// asserted here WITH ITS SIGN, since `validate_body`'s orientation check is the only thing that
/// would otherwise catch it and a positive magnitude alone proves nothing.
#[semio_framework_async_macros::async_test]
async fn analytic_primitives_are_structurally_valid() {
    let pi = std::f64::consts::PI;
    for (name, expected) in [("sphere", 4.0 / 3.0 * pi), ("torus", 2.0 * pi * pi * 2.0 * 0.25), ("cylinder", 2.0 * pi), ("cone", pi / 3.0 * 2.0), ("box", 1.0)] {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = match name {
            "sphere" => make_sphere(&mut body, 1.0, &mut rec),
            "torus" => crate::standards::v1::subsets::brep::schema::diff::primitives::make_torus(&mut body, 2.0, 0.5, &mut rec),
            "cylinder" => make_cylinder(&mut body, 1.0, 2.0, &mut rec),
            "cone" => crate::standards::v1::subsets::brep::schema::diff::primitives::make_cone(&mut body, 1.0, 2.0, &mut rec),
            _ => make_box(&mut body, 1.0, 1.0, 1.0, &mut rec),
        }
        .unwrap();
        let issues = validate_body(&body);
        assert!(issues.is_empty(), "{name}: {:?}", issues.iter().map(|i| format!("{}:{}:{}", i.entity, i.code, i.message)).collect::<Vec<_>>());
        let signed = crate::standards::v1::subsets::brep::schema::inferences::mass_properties::solid_signed_volume(&body, solid, 1e-4).unwrap();
        assert!((signed - expected).abs() <= 5e-3 * expected, "{name}: signed volume {signed}, expected {expected}");
    }
}
