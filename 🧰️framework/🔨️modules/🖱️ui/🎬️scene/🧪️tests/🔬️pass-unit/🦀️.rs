//! 🪟️ Unit proofs of frustum culling over the scene draw descriptors.

use super::*;
use semio_framework_ui_scene_camera::testkit::concrete_forest_camera;
use semio_framework_ui_scene_camera::Camera3d;

#[test]
fn frustum_contains_origin_box() {
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let planes = frustum_planes(view_proj);
    assert!(aabb_intersects_frustum(&planes, [-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]));
}

#[test]
fn frustum_culls_behind_camera_box() {
    let camera = Camera3d::default();
    let view_proj = camera.view_proj(800.0, 800.0);
    let planes = frustum_planes(view_proj);
    assert!(aabb_intersects_frustum(&planes, [-0.5, -0.5, -0.5], [0.5, 0.5, 0.5]));
    let behind = camera.position.add_m(camera.position.sub_m(camera.target).normalize_m().scale_m(2.0));
    let min = [behind.x - 0.1, behind.y - 0.1, behind.z - 0.1];
    let max = [behind.x + 0.1, behind.y + 0.1, behind.z + 0.1];
    assert!(!aabb_intersects_frustum(&planes, min, max));
}

#[test]
fn concrete_forest_frustum_contains_target_box() {
    let camera = concrete_forest_camera();
    let view_proj = camera.view_proj(800.0, 800.0);
    let planes = frustum_planes(view_proj);
    let target = camera.target;
    for plane in &planes {
        let distance = plane.normal.dot_m(target) + plane.distance;
        assert!(distance >= -1e-3, "look-at must be inside frustum, distance={distance}");
    }
    assert!(aabb_intersects_frustum(&planes, [6.0, -1.0, 2.0], [8.0, 1.0, 4.0]));
}

#[test]
fn concrete_forest_frustum_culls_off_axis_boxes() {
    let camera = concrete_forest_camera();
    let view_proj = camera.view_proj(800.0, 800.0);
    let planes = frustum_planes(view_proj);
    assert!(!aabb_intersects_frustum(&planes, [7.0, 0.0, 200.0], [8.0, 1.0, 201.0]));
    let behind = camera.position.add_m(camera.position.sub_m(camera.target).normalize_m().scale_m(4.0));
    let min = [behind.x - 0.25, behind.y - 0.25, behind.z - 0.25];
    let max = [behind.x + 0.25, behind.y + 0.25, behind.z + 0.25];
    assert!(!aabb_intersects_frustum(&planes, min, max));
}
