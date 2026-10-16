//! ➕️ Unit proofs of the synchronous `Vec3`/`Mat4` algebra.

use super::*;

#[test]
fn mat4_inverse_round_trips_to_identity() {
    let m = mat4_translation_m(vec3_new_m(3.0, -2.0, 5.0)).mul_m(mat4_from_quat_m(0.1, 0.2, 0.05, 0.9701425)).mul_m(mat4_scale_vec_m(vec3_new_m(2.0, 1.5, 0.5)));
    let round_trip = m.mul_m(m.inverse_m());
    let identity = mat4_identity_m();
    for col in 0..4 {
        for row in 0..4 {
            assert!((round_trip.cols[col][row] - identity.cols[col][row]).abs() < 1e-4, "mismatch at col={col} row={row}: {} vs {}", round_trip.cols[col][row], identity.cols[col][row]);
        }
    }
}

#[test]
fn perspective_maps_depth_to_wgpu_ndc() {
    let near = 0.1_f32;
    let far = 100.0_f32;
    let proj = mat4_perspective_m(45.0_f32.to_radians(), 1.0, near, far);
    let near_pt = proj.transform_point_m(vec3_new_m(0.0, 0.0, -near));
    let far_pt = proj.transform_point_m(vec3_new_m(0.0, 0.0, -far));
    assert!((near_pt.z - 0.0).abs() < 1e-4, "near z={}", near_pt.z);
    assert!((far_pt.z - 1.0).abs() < 1e-3, "far z={}", far_pt.z);
}
