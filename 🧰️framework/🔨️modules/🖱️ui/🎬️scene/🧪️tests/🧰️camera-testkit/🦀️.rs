//! 🎦️ Shared test kit of the camera crate: the verbatim wire cameras reused by the culling and picking suites.

use super::*;

pub fn concrete_forest_camera() -> Camera3d {
    Camera3d { position: vec3_new_m(30.0, -30.0, 20.0), target: vec3_new_m(7.0, 0.0, 3.0), up: vec3_new_m(0.0, 0.0, 1.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 1000.0, projection: CameraProjection3d::Perspective, zoom: 1.0 }
}

/// 🎥️ The puzzle 3d `Top` pane's camera, verbatim off the wire
/// (`🗑️generated/w7b-diag/dumps.json`): an orthographic plan view whose `up` is `+Y`.
pub fn puzzle3d_top_camera(zoom: f32) -> Camera3d {
    Camera3d {
        position: vec3_new_m(3.5, 0.0, 9.455),
        target: vec3_new_m(3.5, 0.0, 0.005),
        up: vec3_new_m(0.0, 1.0, 0.0),
        fov_y: 45.0_f32.to_radians(),
        near: WORLD_ORBIT_CAMERA_NEAR,
        far: WORLD_ORBIT_CAMERA_MIN_FAR,
        projection: CameraProjection3d::Orthographic,
        zoom,
    }
}

/// 🧩️ The puzzle 3d `Perspective` pane's camera, verbatim off the same wire.
pub fn puzzle3d_perspective_camera() -> Camera3d {
    Camera3d {
        position: vec3_new_m(9.17, -5.67, 4.2575),
        target: vec3_new_m(3.5, 0.0, 0.005),
        up: vec3_new_m(0.0, 0.0, 1.0),
        fov_y: 50.0_f32.to_radians(),
        near: WORLD_ORBIT_CAMERA_NEAR,
        far: WORLD_ORBIT_CAMERA_MIN_FAR,
        projection: CameraProjection3d::Perspective,
        zoom: 1.0,
    }
}
