
use super::*;

#[test]
fn vec3_normalize_zero_stays_zero() {
    assert_eq!(Vec3::ZERO.normalize(), Vec3::ZERO);
}

#[test]
fn vec3_cross_is_perpendicular() {
    let a = Vec3::new(1.0, 0.0, 0.0);
    let b = Vec3::new(0.0, 1.0, 0.0);
    let c = a.cross(b);
    assert!((c.dot(a)).abs() < 1e-6);
    assert!((c.dot(b)).abs() < 1e-6);
    assert!((c.z - 1.0).abs() < 1e-6);
}

#[test]
fn mat4_identity_transforms_point_unchanged() {
    let p = Vec3::new(1.0, 2.0, 3.0);
    let out = Mat4::identity().transform_point(p);
    assert!((out.x - p.x).abs() < 1e-6 && (out.y - p.y).abs() < 1e-6 && (out.z - p.z).abs() < 1e-6);
}

#[test]
fn mat4_inverse_round_trips_translation() {
    let m = Mat4::translation(Vec3::new(3.0, -2.0, 5.0));
    let inv = m.inverse();
    let p = Vec3::new(1.0, 1.0, 1.0);
    let round = inv.transform_point(m.transform_point(p));
    assert!((round.x - p.x).abs() < 1e-4 && (round.y - p.y).abs() < 1e-4 && (round.z - p.z).abs() < 1e-4);
}

#[test]
fn vec3_array_round_trip() {
    let v = Vec3::from_array([1.0, 2.0, 3.0]);
    assert_eq!(v.to_array(), [1.0, 2.0, 3.0]);
}

#[test]
fn vec3_add_sub_scale_dot_length_match_hand_computation() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(0.5, 0.5, 0.5);
    assert_eq!(a.add(b), Vec3::new(1.5, 2.5, 3.5));
    assert_eq!(a.sub(b), Vec3::new(0.5, 1.5, 2.5));
    assert_eq!(a.scale(2.0), Vec3::new(2.0, 4.0, 6.0));
    assert!((a.dot(a) - 14.0).abs() < 1e-6);
    assert!((Vec3::new(3.0, 4.0, 0.0).length() - 5.0).abs() < 1e-6);
}

#[test]
fn mat4_perspective_maps_near_and_far_planes_to_depth_zero_and_one() {
    let m = Mat4::perspective(std::f32::consts::FRAC_PI_2, 1.0, 1.0, 10.0);
    let near = m.transform_point(Vec3::new(0.0, 0.0, -1.0));
    let far = m.transform_point(Vec3::new(0.0, 0.0, -10.0));
    assert!(near.z.abs() < 1e-5, "near plane depth was {}", near.z);
    assert!((far.z - 1.0).abs() < 1e-5, "far plane depth was {}", far.z);
}

#[test]
fn mat4_look_at_places_target_along_negative_z() {
    let m = Mat4::look_at(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::new(0.0, 1.0, 0.0));
    let cam_space = m.transform_point(Vec3::ZERO);
    assert!(cam_space.x.abs() < 1e-5);
    assert!(cam_space.y.abs() < 1e-5);
    assert!((cam_space.z + 5.0).abs() < 1e-5);
}

#[test]
fn mat4_mul_composes_transforms_in_matrix_order() {
    let t = Mat4::translation(Vec3::new(1.0, 0.0, 0.0));
    let s = Mat4::scale_vec(Vec3::new(2.0, 2.0, 2.0));
    let combined = t.mul(s);
    let out = combined.transform_point(Vec3::new(1.0, 1.0, 1.0));
    assert!((out.x - 3.0).abs() < 1e-6 && (out.y - 2.0).abs() < 1e-6 && (out.z - 2.0).abs() < 1e-6);
}

#[test]
fn mat4_transform_direction_ignores_translation_and_normalizes() {
    let m = Mat4::translation(Vec3::new(5.0, 5.0, 5.0));
    let dir = m.transform_direction(Vec3::new(2.0, 0.0, 0.0));
    assert!((dir.x - 1.0).abs() < 1e-6 && dir.y.abs() < 1e-6 && dir.z.abs() < 1e-6);
}

#[test]
fn mat4_inverse_of_singular_matrix_returns_identity() {
    let singular = Mat4 { cols: [[0.0; 4]; 4] };
    assert_eq!(singular.inverse().to_cols_array(), Mat4::identity().to_cols_array());
}

#[test]
fn mat4_scale_vec_scales_each_axis() {
    let m = Mat4::scale_vec(Vec3::new(2.0, 3.0, 4.0));
    let p = m.transform_point(Vec3::new(1.0, 1.0, 1.0));
    assert!((p.x - 2.0).abs() < 1e-6 && (p.y - 3.0).abs() < 1e-6 && (p.z - 4.0).abs() < 1e-6);
}

#[test]
fn mat4_from_quat_identity_is_identity() {
    let m = Mat4::from_quat(0.0, 0.0, 0.0, 1.0);
    let p = Vec3::new(1.0, 2.0, 3.0);
    let out = m.transform_point(p);
    assert!((out.x - p.x).abs() < 1e-6 && (out.y - p.y).abs() < 1e-6 && (out.z - p.z).abs() < 1e-6);
}

#[test]
fn mat4_from_quat_90_degrees_about_z_rotates_x_to_y() {
    let half = std::f32::consts::FRAC_PI_4;
    let m = Mat4::from_quat(0.0, 0.0, half.sin(), half.cos());
    let out = m.transform_point(Vec3::new(1.0, 0.0, 0.0));
    assert!(out.x.abs() < 1e-5);
    assert!((out.y - 1.0).abs() < 1e-5);
    assert!(out.z.abs() < 1e-5);
}

#[test]
fn mat4_to_cols_array_matches_column_major_layout() {
    let m = Mat4::translation(Vec3::new(1.0, 2.0, 3.0));
    let arr = m.to_cols_array();
    assert_eq!(arr[12], 1.0);
    assert_eq!(arr[13], 2.0);
    assert_eq!(arr[14], 3.0);
    assert_eq!(arr[15], 1.0);
}
