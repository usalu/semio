//! 🔭️ Unit proofs of the orbit and parallel cameras, projection matrices, gumball and level-of-detail grid math.

use super::testkit::{puzzle3d_perspective_camera, puzzle3d_top_camera};
use super::*;
use semio_framework_ui_scene_algebra::mat4_from_quat_m;

#[test]
fn orbit_round_trip() {
    let camera = Camera3d::default();
    let orbit = OrbitController::from_camera(&camera);
    let next = orbit.to_camera();
    assert!((next.position.x - camera.position.x).abs() < 0.5);
}

#[test]
fn mat4_inverse_undoes_view_projection() {
    let camera = Camera3d::default();
    let view = mat4_look_at_m(camera.position, camera.target, camera.up);
    let proj = mat4_perspective_m(camera.fov_y, 1.5, camera.near, camera.far);
    let view_proj = proj.mul_m(view);
    let inv = view_proj.inverse_m();
    let world_point = vec3_new_m(1.0, 2.0, 0.5);
    let clip_point = view_proj.transform_point_m(world_point);
    let unprojected = inv.transform_point_m(clip_point);
    assert!((unprojected.x - world_point.x).abs() < 1e-3, "x mismatch: {}", unprojected.x);
    assert!((unprojected.y - world_point.y).abs() < 1e-3, "y mismatch: {}", unprojected.y);
    assert!((unprojected.z - world_point.z).abs() < 1e-3, "z mismatch: {}", unprojected.z);
}

#[test]
fn ray_from_screen_center_points_at_target() {
    let camera = Camera3d::default();
    let (origin, dir) = camera.ray_from_screen(400.0, 300.0, 800.0, 600.0);
    assert!((origin.x - camera.position.x).abs() < 1e-4);
    let to_target = camera.target.sub_m(camera.position).normalize_m();
    let dot = dir.dot_m(to_target);
    assert!(dot > 0.999, "ray from screen center should point at target, dot={dot}");
}

#[test]
fn lod_from_camera_distance_scales() {
    assert!((lod_from_camera_distance(100.0, 100.0) - 1.0).abs() < 1e-6);
    assert!((lod_from_camera_distance(20000.0, 100.0) - 200.0).abs() < 1e-6);
}

/// 📐️ LAW: the grid's fade radius follows the camera and stays inside the far plane it is drawn
/// against, so it is neither clipped away nor drawn with a hard square edge — React's
/// `cameraGridFadeDistance` (`🎨️r3f/🟦️.tsx`), viewport branch and all, now that the orbit camera
/// grows its own far plane the way React's does
/// (`📓️w7b-presenter-one-frame-per-boot.md` §5, `📓️w8b-orthographic-camera-and-3d-parity.md` §2).
#[test]
fn the_grid_fade_radius_follows_the_camera_and_stays_inside_the_far_plane() {
    let orbit = OrbitController::default();
    let camera = orbit.to_camera();
    assert_eq!(camera.far, adaptive_orbit_camera_far(orbit.distance));
    assert_eq!(camera.far, WORLD_ORBIT_CAMERA_MIN_FAR, "a near orbit sits on React's floor");
    assert_eq!(adaptive_orbit_camera_far(4_096.0), 2.0_f32.powf((4_096.0 * WORLD_ORBIT_CAMERA_FAR_DISTANCE_FACTOR).log2().ceil()), "and a far one follows the distance, quantized to a power of two");

    let fade = camera_grid_fade_distance(&camera, 0.0, 10.0, 800.0, 600.0);
    assert!(fade > 0.0, "the grid always covers something");
    assert!(fade <= f32::from(camera.far) * 0.25 + 1e-3, "and never reaches past a quarter of the far plane while the coverage floor allows it");
    let higher = Camera3d { position: vec3_new_m(camera.position.x, camera.position.y, camera.position.z * 20.0), ..camera.clone() };
    assert!(camera_grid_fade_distance(&higher, 0.0, 10.0, 800.0, 600.0) >= fade, "a camera further above the plane covers at least as much");
    assert_eq!(camera_grid_fade_distance(&camera, 0.0, 0.0, 800.0, 600.0), 0.0, "a degenerate step draws nothing");

    assert_eq!(lod_grid_fade_alpha(0.0, 100.0), 1.0, "the grid is opaque at its centre");
    assert_eq!(lod_grid_fade_alpha(100.0, 100.0), 0.0, "and gone at its rim");
    assert!(lod_grid_fade_alpha(50.0, 100.0) < 1.0 && lod_grid_fade_alpha(50.0, 100.0) > 0.0, "with a monotone fade between");
    assert_eq!(lod_grid_fade_alpha(10.0, 0.0), 0.0, "a zero fade radius draws nothing");
}

/// 🎯️ LAW: React's `WorldAutoFit` uses the sphere-fit eye distance in both camera families and
/// preserves parallel zoom from the independent projection-content frame.
#[test]
fn auto_fit_moves_both_camera_families_eye_distance_and_preserves_parallel_zoom() {
    let (width, height) = (478.0_f32, 814.0_f32);
    let minimum = [0.0_f32, -2.0, 0.0];
    let maximum = [7.0_f32, 2.0, 1.0];

    let perspective = OrbitController::default();
    let framed = frame_orbit_to_bounds(&perspective, minimum, maximum, width, height, WORLD_FRAME_BOUNDS_MARGIN);
    assert_eq!(framed.target, vec3_new_m(3.5, 0.0, 0.5), "both families centre on the box");
    assert!((framed.distance - perspective.distance).abs() > 1e-3, "a perspective fit dollies");
    assert!((framed.zoom - 1.0).abs() < 1e-6, "and leaves the identity zoom alone");

    let parallel = OrbitController { projection: CameraProjection3d::Orthographic, zoom: 7.7595, ..OrbitController::default() };
    let parallel_framed = frame_orbit_to_bounds(&parallel, minimum, maximum, width, height, WORLD_FRAME_BOUNDS_MARGIN);
    assert_eq!(parallel_framed.target, vec3_new_m(3.5, 0.0, 0.5));
    assert!((parallel_framed.distance - 11.888316).abs() < 1e-5, "Three OrthographicCamera has no aspect property, so WorldAutoFit uses one");
    let parallel_wide = frame_orbit_to_bounds(&parallel, minimum, maximum, height, width, WORLD_FRAME_BOUNDS_MARGIN);
    assert!((parallel_wide.distance - parallel_framed.distance).abs() < 1e-6, "parallel eye distance is independent of viewport aspect");
    assert!((parallel_framed.zoom - parallel.zoom).abs() < 1e-6, "and preserves projection-content zoom");
}

/// 🖼️ LAW: a projection pane frames on React's CARDINAL half-extent, not on the box's projected
/// screen extent — `worldProjectionViewHalfExtent` reads `(hx, hy)` for a plan view, `(hx, hz)` for
/// front/back, `(hy, hz)` for left/right, `(hx, hz)` for a free oblique, and the ISOTROPIC span
/// `max(hx, hy, hz)` for everything else (`🎨️r3f/🟦️.tsx`).
///
/// 🧪️ The numbers are React's own, ported from `🎨️r3f/🧪️tests/🧪️chunkkey/🟦️.tsx`: a 50-wide
/// reference at `[7, 0, 0.01]` gives `halfExtent [25, 25, 0.5]`, and a `400 × 800` top pane frames
/// it at `zoom = 200 / (25 · 1.35)`. There was no Rust mirror of that fixture, which is why the
/// cardinal-versus-projected divergence went unpinned.
#[test]
fn a_projection_frame_reads_reacts_cardinal_half_extent() {
    let half = [25.0_f32, 25.0, 0.5];
    let cardinal = |view| world_projection_view_half_extent(WorldProjectionOrientation::Cardinal(view), false, half);
    assert_eq!(cardinal(WorldCardinalView::Top), (25.0, 25.0), "📐️ a plan view spans (hx, hy)");
    assert_eq!(cardinal(WorldCardinalView::Bottom), (25.0, 25.0));
    assert_eq!(cardinal(WorldCardinalView::Front), (25.0, 0.5), "📐️ front/back span (hx, hz)");
    assert_eq!(cardinal(WorldCardinalView::Back), (25.0, 0.5));
    assert_eq!(cardinal(WorldCardinalView::Left), (25.0, 0.5), "📐️ left/right span (hy, hz)");
    assert_eq!(cardinal(WorldCardinalView::Right), (25.0, 0.5));
    assert_eq!(world_projection_view_half_extent(WorldProjectionOrientation::Free, true, half), (25.0, 0.5), "📐️ a free oblique measures in (x, z)");
    assert_eq!(world_projection_view_half_extent(WorldProjectionOrientation::Free, false, half), (25.0, 25.0), "📐️ and every other free orientation takes the isotropic span");
    assert_eq!(world_projection_view_half_extent(WorldProjectionOrientation::Free, false, [1.0, 2.0, 9.0]), (9.0, 9.0), "📐️ isotropic means the LARGEST axis on both, React's `max(hx, hy, hz)`");

    let parallel = OrbitController { projection: CameraProjection3d::Orthographic, zoom: 1.0, ..OrbitController::default() };
    let (minimum, maximum) = ([7.0_f32 - 25.0, -25.0, 0.01 - 0.5], [7.0_f32 + 25.0, 25.0, 0.01 + 0.5]);
    let framed = frame_projection_orbit_to_bounds(&parallel, WorldProjectionOrientation::Cardinal(WorldCardinalView::Top), false, minimum, maximum, 400.0, 800.0, WORLD_PROJECTION_FRAME_PADDING);
    assert!((framed.target.x - 7.0).abs() < 1e-5 && framed.target.y.abs() < 1e-5 && (framed.target.z - 0.01).abs() < 1e-5, "📐️ the frame centres on the content box, as React's `computeWorldProjectionPose` does: {:?}", framed.target);
    assert!((framed.zoom - 200.0 / (25.0 * WORLD_PROJECTION_FRAME_PADDING)).abs() < 1e-3, "📐️ React's own chunkkey number, to the digit: {}", framed.zoom);
    assert!((framed.distance - parallel.distance).abs() < 1e-6, "📐️ a parallel frame never dollies");

    let corner = OrbitController { projection: CameraProjection3d::Orthographic, zoom: 1.0, yaw: 0.7, pitch: 0.6, ..OrbitController::default() };
    let (thin_min, thin_max) = ([-8.0_f32, -1.0, -1.0], [8.0_f32, 1.0, 1.0]);
    let isotropic = frame_projection_orbit_to_bounds(&corner, WorldProjectionOrientation::Free, false, thin_min, thin_max, 400.0, 800.0, WORLD_PROJECTION_FRAME_PADDING);
    let centered = OrbitController { target: vec3_new_m(0.0, 0.0, 0.0), ..corner };
    let (projected_half_width, projected_half_height) = screen_half_extent(&centered.to_camera(), thin_min, thin_max);
    let projected_zoom = world_projection_ortho_zoom(projected_half_width, projected_half_height, 400.0, 800.0, WORLD_PROJECTION_FRAME_PADDING);
    assert!(isotropic.zoom < projected_zoom, "📐️ React's isotropic span frames WIDER than the projected box extent: {} vs {}", isotropic.zoom, projected_zoom);
}

/// 🔎️ LAW: the wheel moves whichever number the family owns — three's `OrbitControls` dollies a
/// perspective camera and scales an orthographic one's `zoom` (`captureNavigationSnapshot` reports
/// exactly that split).
#[test]
fn the_wheel_dollies_a_perspective_orbit_and_zooms_a_parallel_one() {
    let mut perspective = OrbitController::default();
    let (distance, zoom) = (perspective.distance, perspective.zoom);
    perspective.zoom(-200.0);
    assert!(perspective.distance > distance, "a perspective wheel dollies");
    assert!((perspective.zoom - zoom).abs() < 1e-6, "and never touches the zoom");

    let mut parallel = OrbitController { projection: CameraProjection3d::Orthographic, zoom: WORLD_ORBIT_PARALLEL_DEFAULT_ZOOM, ..OrbitController::default() };
    let distance = parallel.distance;
    parallel.zoom(200.0);
    assert!(parallel.zoom > WORLD_ORBIT_PARALLEL_DEFAULT_ZOOM, "a parallel wheel scales the zoom");
    assert!((parallel.distance - distance).abs() < 1e-6, "and never dollies");
    parallel.zoom(-1e9);
    assert!(parallel.zoom >= WORLD_ORBIT_PARALLEL_ZOOM_MIN, "the frustum never inverts");
}

/// 🎥️ LAW: the two puzzle 3d panes' delivered cameras survive the orbit round-trip intact — this is
/// the fixture the whole packet is measured against. The `Top` pane's `up` is `+Y`, and the Z-up the
/// orbit used to hard-code made its `look_at` cross product degenerate, which is why that pane
/// rendered nothing useful (`📓️w8b-orthographic-camera-and-3d-parity.md` §1).
#[test]
fn the_puzzle3d_pane_cameras_survive_the_orbit_round_trip() {
    let top = puzzle3d_top_camera(1.0);
    let orbit = OrbitController::from_camera(&top);
    assert_eq!(orbit.projection, CameraProjection3d::Orthographic);
    assert_eq!(orbit.up, vec3_new_m(0.0, 1.0, 0.0));
    assert!((orbit.zoom - 1.0).abs() < 1e-6);
    let round_tripped = orbit.to_camera();
    assert!(round_tripped.position.sub_m(top.position).length_m() < 1e-3, "the plan eye survives: {:?}", round_tripped.position);
    assert_eq!(round_tripped.target, top.target);
    assert_eq!(round_tripped.up, top.up);
    let matrix = round_tripped.view_proj(478.0, 814.0);
    assert!(matrix.cols.iter().flatten().all(|value| value.is_finite()), "and the plan view-projection is finite, not NaN");

    let perspective = puzzle3d_perspective_camera();
    let orbit = OrbitController::from_camera(&perspective);
    assert_eq!(orbit.projection, CameraProjection3d::Perspective);
    assert!((orbit.distance - 9.0764).abs() < 1e-2, "distance={}", orbit.distance);
    assert!((orbit.fov_y.to_degrees() - 50.0).abs() < 1e-3);
    assert!(orbit.to_camera().position.sub_m(perspective.position).length_m() < 1e-3);
}

/// ↔️ LAW: the two `zoom` ↔ frustum mappings React uses are exact inverses, so a projection switch
/// never jumps scale (`worldProjectionMatchedOrthoZoom` / `…MatchedPerspectiveDistance`).
#[test]
fn the_matched_zoom_and_distance_mappings_invert_each_other() {
    let zoom = world_projection_matched_ortho_zoom(WORLD_LOD_REFERENCE_FOV_DEG, 9.45, 814.0);
    let distance = world_projection_matched_perspective_distance(WORLD_LOD_REFERENCE_FOV_DEG, zoom, 814.0);
    assert!((distance - 9.45).abs() < 1e-3, "distance={distance}");
    let camera = Camera3d { projection: CameraProjection3d::Orthographic, zoom, ..puzzle3d_perspective_camera() };
    assert!((lod_orbit_distance_for_camera(&camera, 0.0, 814.0) - 9.45).abs() < 1e-3, "a parallel pane bands its LOD on the matched distance");
    let perspective = puzzle3d_perspective_camera();
    assert!((lod_orbit_distance_for_camera(&perspective, 9.45, 814.0) - 9.45).abs() < 1e-6, "and a perspective one on its own");
}

/// 🔳️ LAW: a parallel camera's visible radius is its own zoomed pixel extent, never the
/// height-above-plane a perspective one uses — React's `cameraGridVisibleRadius`.
#[test]
fn the_grid_visible_radius_reads_the_cameras_own_family() {
    let perspective = OrbitController::default().to_camera();
    let radius = camera_grid_visible_radius(&perspective, 0.0, 960.0, 800.0);
    assert!(radius > 0.0);

    let parallel = Camera3d { projection: CameraProjection3d::Orthographic, zoom: 1.0, ..perspective.clone() };
    let half = (960.0_f32 * 0.5).hypot(800.0 * 0.5);
    assert!((camera_grid_visible_radius(&parallel, 0.0, 960.0, 800.0) - half).abs() < 1e-3, "zoom 1 shows exactly the viewport in world units");
    let zoomed = Camera3d { zoom: 4.0, ..parallel.clone() };
    assert!((camera_grid_visible_radius(&zoomed, 0.0, 960.0, 800.0) - half / 4.0).abs() < 1e-3, "and four times the zoom shows a quarter of it");
}

#[test]
fn lod_progressive_grid_layers_adds_bands() {
    assert!(lod_progressive_grid_layers(5000.0, 10.0).is_empty());
    assert_eq!(lod_progressive_grid_layers(500.0, 10.0).iter().map(|(step, _)| *step).collect::<Vec<_>>(), vec![100.0]);
    assert_eq!(lod_progressive_grid_layers(50.0, 10.0).iter().map(|(step, _)| *step).collect::<Vec<_>>(), vec![100.0, 25.0]);
    assert_eq!(lod_progressive_grid_layers(2.0, 10.0).iter().map(|(step, _)| *step).collect::<Vec<_>>(), vec![100.0, 25.0, 5.0, 1.0]);
}

#[test]
fn lod_progressive_grid_layer_key_stable_within_band() {
    let key_a = lod_progressive_grid_layer_key(50.0, 10.0);
    let key_b = lod_progressive_grid_layer_key(49.2, 10.0);
    let key_c = lod_progressive_grid_layer_key(11.4, 10.0);
    assert_eq!(key_a, key_b);
    assert_eq!(key_a, key_c);
    assert_eq!(lod_progressive_grid_layer_key(9.8, 10.0), "100|25|5");
}

#[test]
fn pick_closest_lod_prefers_more_detailed_on_tie() {
    let picked = pick_closest_lod(&[1.0, 2.0, 4.0], 2.0).unwrap();
    assert!((picked - 2.0).abs() < 1e-6);
}

#[test]
fn floating_origin_rebase_subtracts_anchor() {
    let rebased = floating_origin_rebase(vec3_new_m(10.0, 20.0, 30.0), vec3_new_m(1.0, 2.0, 3.0));
    assert_eq!(rebased, vec3_new_m(9.0, 18.0, 27.0));
}

#[test]
fn orbit_controller_orbit_clamps_pitch() {
    let mut orbit = OrbitController { pitch: 1.49, ..Default::default() };
    orbit.orbit(0.0, 1000.0);
    assert!((orbit.pitch - 1.5).abs() < 1e-5, "pitch={}", orbit.pitch);
    orbit.pitch = -1.49;
    orbit.orbit(0.0, -1000.0);
    assert!((orbit.pitch + 1.5).abs() < 1e-5, "pitch={}", orbit.pitch);
}

#[test]
fn orbit_controller_pan_moves_target_away_from_origin() {
    let mut orbit = OrbitController::default();
    let start = orbit.target;
    orbit.pan(50.0, 0.0);
    assert!(orbit.target.sub_m(start).length_m() > 0.0);
}

#[test]
fn orbit_controller_zoom_clamps_distance_bounds() {
    let mut orbit = OrbitController { distance: 1.0, ..Default::default() };
    orbit.zoom(100_000.0);
    assert!((orbit.distance - 0.5).abs() < 1e-4, "distance={}", orbit.distance);
    orbit.distance = 400.0;
    orbit.zoom(-100_000.0);
    assert!((orbit.distance - 500.0).abs() < 1e-4, "distance={}", orbit.distance);
}

#[test]
fn vec3_from_f64_converts_components() {
    let v = vec3_from_f64([1.5, -2.5, 3.5]);
    assert_eq!(v, vec3_new_m(1.5, -2.5, 3.5));
}

#[test]
fn gumball_extent_clamps_to_bounds() {
    assert!((gumball_extent(0.0) - 0.25).abs() < 1e-6);
    assert!((gumball_extent(1000.0) - 2.5).abs() < 1e-6);
    assert!((gumball_extent(10.0) - 1.5).abs() < 1e-6);
}

#[test]
fn gumball_eye_points_from_pivot_to_camera() {
    let camera = Camera3d { position: vec3_new_m(0.0, 0.0, 10.0), target: Vec3::ZERO, up: vec3_new_m(0.0, 1.0, 0.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 100.0, projection: CameraProjection3d::Perspective, zoom: 1.0 };
    let eye = gumball_eye(&camera, Vec3::ZERO);
    assert!((eye.length_m() - 1.0).abs() < 1e-5);
    assert!(eye.z > 0.99);
}

#[test]
fn ray_plane_point_hits_plane_ahead_and_misses_parallel() {
    let hit = ray_plane_point(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), Vec3::ZERO, vec3_new_m(0.0, 0.0, 1.0)).expect("hit");
    assert!((hit.z - 0.0).abs() < 1e-5, "z={}", hit.z);
    assert!(ray_plane_point(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(1.0, 0.0, 0.0), Vec3::ZERO, vec3_new_m(0.0, 0.0, 1.0)).is_none());
}

#[test]
fn ray_plane_point_rejects_intersection_behind_origin() {
    assert!(ray_plane_point(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, -1.0), Vec3::ZERO, vec3_new_m(0.0, 0.0, 1.0)).is_none());
}

#[test]
fn gumball_axis_drag_plane_normal_is_perpendicular_to_axis() {
    let axis = vec3_new_m(1.0, 0.0, 0.0);
    let eye = vec3_new_m(0.0, 0.0, 1.0);
    let normal = gumball_axis_drag_plane_normal(axis, eye);
    assert!(normal.dot_m(axis).abs() < 1e-5, "dot={}", normal.dot_m(axis));
}

#[test]
fn gumball_axis_drag_plane_normal_handles_axis_aligned_with_eye() {
    let axis = vec3_new_m(0.0, 0.0, 1.0);
    let eye = vec3_new_m(0.0, 0.0, 1.0);
    let normal = gumball_axis_drag_plane_normal(axis, eye);
    assert!(normal.dot_m(axis).abs() < 1e-5, "dot={}", normal.dot_m(axis));
    assert!((normal.length_m() - 1.0).abs() < 1e-5);
}

#[test]
fn gumball_project_ray_onto_axis_measures_signed_offset() {
    let pivot = Vec3::ZERO;
    let axis = vec3_new_m(1.0, 0.0, 0.0);
    let eye = vec3_new_m(0.0, 0.0, 1.0);
    let offset = gumball_project_ray_onto_axis(vec3_new_m(3.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), pivot, axis, eye).expect("offset");
    assert!((offset - 3.0).abs() < 1e-4, "offset={offset}");
}

#[test]
fn original_ray_segment_projection_clips_depth_with_independent_three_witnesses() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let law = &fixture["gumball"]["projectedSegmentCases"];
    let vector = |row: &serde_json::Value| vec3_new_m(row[0].as_f64().unwrap() as f32, row[1].as_f64().unwrap() as f32, row[2].as_f64().unwrap() as f32);
    let width = law["viewport"][0].as_f64().unwrap() as f32;
    let height = law["viewport"][1].as_f64().unwrap() as f32;
    let camera = Camera3d { position: vector(&law["camera"]["position"]), target: vector(&law["camera"]["target"]), up: vector(&law["camera"]["up"]), fov_y: (law["camera"]["fov"].as_f64().unwrap() as f32).to_radians(), near: law["camera"]["near"].as_f64().unwrap() as f32, far: law["camera"]["far"].as_f64().unwrap() as f32, projection: CameraProjection3d::Perspective, zoom: 1.0 };
    let spec = default_projection_spec();
    let view_projection = projection_spec_view_proj(&camera, spec, width, height);
    for row in law["cases"].as_array().unwrap() {
        let segment = projection_spec_project_segment(view_projection, spec, vector(&row["a"]), vector(&row["b"]), width, height);
        assert_eq!(segment.is_some(), row["visible"].as_bool().unwrap(), "{}", row["name"]);
        if let Some(segment) = segment { for index in 0..2 { let expected = vector(&row["points"][index]); assert!(segment.points[index].sub_m(expected).length_m() < 2e-4, "{} endpoint {index}: {:?}", row["name"], segment.points[index]); let projected = [row["screen"][index][0].as_f64().unwrap() as f32, row["screen"][index][1].as_f64().unwrap() as f32]; assert!((segment.screen[index][0] - projected[0]).abs() < 1e-3 && (segment.screen[index][1] - projected[1]).abs() < 1e-3, "{} screen {index}", row["name"]); } }
    }
    assert!(projection_spec_project_segment(view_projection, spec, vec3_new_m(f32::NAN, 0.0, -1.0), vec3_new_m(0.0, 0.0, -1.0), width, height).is_none());
}

#[test]
fn original_ray_segment_closest_points_match_independent_three_witnesses() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let vector = |row: &serde_json::Value| vec3_new_m(row[0].as_f64().unwrap() as f32, row[1].as_f64().unwrap() as f32, row[2].as_f64().unwrap() as f32);
    for row in fixture["gumball"]["raySegmentClosestCases"].as_array().unwrap() {
        let closest = ray_segment_closest(vector(&row["origin"]), vector(&row["direction"]), vector(&row["a"]), vector(&row["b"])).unwrap();
        assert!((closest.distance - row["distance"].as_f64().unwrap() as f32).abs() < 1e-5, "{}", row["name"]);
        assert!(closest.ray_point.sub_m(vector(&row["rayPoint"])).length_m() < 1e-5, "{}", row["name"]);
        assert!(closest.segment_point.sub_m(vector(&row["segmentPoint"])).length_m() < 1e-5, "{}", row["name"]);
        assert_eq!(ray_segment_distance(vector(&row["origin"]), vector(&row["direction"]), vector(&row["a"]), vector(&row["b"])), Some(closest.distance));
    }
    assert!(ray_segment_closest(Vec3::ZERO, Vec3::ZERO, vec3_new_m(0.0, 0.0, -1.0), vec3_new_m(1.0, 0.0, -1.0)).is_none());
    assert!(ray_segment_closest(Vec3::ZERO, vec3_new_m(0.0, 0.0, -1.0), Vec3::ZERO, Vec3::ZERO).is_none());
}

#[test]
fn ray_segment_distance_measures_perpendicular_gap() {
    let dist = ray_segment_distance(vec3_new_m(3.0, 0.0, 0.0), vec3_new_m(0.0, 0.0, 1.0), vec3_new_m(0.0, -5.0, 0.0), vec3_new_m(0.0, 5.0, 0.0)).expect("distance");
    assert!((dist - 3.0).abs() < 1e-4, "dist={dist}");
}

#[test]
fn ray_segment_distance_none_for_degenerate_segment() {
    assert!(ray_segment_distance(Vec3::ZERO, vec3_new_m(1.0, 0.0, 0.0), vec3_new_m(5.0, 5.0, 5.0), vec3_new_m(5.0, 5.0, 5.0)).is_none());
}

#[test]
fn quat_from_basis_identity_axes_yields_identity_quaternion() {
    let q = quat_from_basis(vec3_new_m(1.0, 0.0, 0.0), vec3_new_m(0.0, 1.0, 0.0), vec3_new_m(0.0, 0.0, 1.0));
    assert!((q[0]).abs() < 1e-5 && (q[1]).abs() < 1e-5 && (q[2]).abs() < 1e-5, "q={q:?}");
    assert!((q[3] - 1.0).abs() < 1e-5, "q={q:?}");
}

#[test]
fn quat_from_basis_round_trips_through_mat4_from_quat() {
    let q = quat_from_basis(vec3_new_m(0.0, 1.0, 0.0), vec3_new_m(-1.0, 0.0, 0.0), vec3_new_m(0.0, 0.0, 1.0));
    let m = mat4_from_quat_m(q[0], q[1], q[2], q[3]);
    let rotated = m.transform_point_m(vec3_new_m(1.0, 0.0, 0.0));
    assert!((rotated.x - 0.0).abs() < 1e-4 && (rotated.y - 1.0).abs() < 1e-4, "rotated={rotated:?}");
}

#[test]
fn rotate_vector_by_90_degrees_around_z_axis() {
    let rotated = rotate_vector(vec3_new_m(1.0, 0.0, 0.0), vec3_new_m(0.0, 0.0, 1.0), std::f32::consts::FRAC_PI_2);
    assert!((rotated.x - 0.0).abs() < 1e-4, "x={}", rotated.x);
    assert!((rotated.y - 1.0).abs() < 1e-4, "y={}", rotated.y);
}

#[test]
fn axis_rotate_angle_measures_signed_rotation() {
    let axis = vec3_new_m(0.0, 0.0, 1.0);
    let start = vec3_new_m(1.0, 0.0, 0.0);
    let current = vec3_new_m(0.0, 1.0, 0.0);
    let angle = axis_rotate_angle(start, current, axis);
    assert!((angle - std::f32::consts::FRAC_PI_2).abs() < 1e-3, "angle={angle}");
    let angle_reverse = axis_rotate_angle(start, vec3_new_m(0.0, -1.0, 0.0), axis);
    assert!((angle_reverse + std::f32::consts::FRAC_PI_2).abs() < 1e-3, "angle={angle_reverse}");
}

#[test]
fn pick_closest_mesh_url_returns_fallback_when_no_entries() {
    let entries: [(f64, &str); 0] = [];
    assert_eq!(pick_closest_mesh_url(&entries, 5.0, Some("fallback.glb")), Some("fallback.glb"));
}

#[test]
fn pick_closest_mesh_url_selects_nearest_lod_entry() {
    let entries = [(1.0, "hi.glb"), (10.0, "mid.glb"), (100.0, "lo.glb")];
    assert_eq!(pick_closest_mesh_url(&entries, 8.0, None), Some("mid.glb"));
}

#[test]
fn pick_closest_mesh_url_filters_out_non_finite_and_negative_lods() {
    let entries = [(-1.0, "bad.glb"), (f64::NAN, "nan.glb"), (5.0, "good.glb")];
    assert_eq!(pick_closest_mesh_url(&entries, 3.0, None), Some("good.glb"));
}

#[test]
fn lod_grid_step_world_quantizes_like_reacts_helper() {
    assert_eq!(lod_grid_step_world(0.0, 10.0), None, "a degenerate LOD draws nothing");
    assert_eq!(lod_grid_step_world(-1.0, 10.0), None);
    assert_eq!(lod_grid_step_world(1.0, 0.0), None, "and so does a degenerate factor");
    let step = lod_grid_step_world(1.0, 10.0).expect("step");
    assert!((step - 10.0).abs() < 1e-9, "one factor covers everything up to the base LOD: step={step}");
    assert!((lod_grid_step_world(2.0, 10.0).expect("step") - 10.0).abs() < 1e-9);
    assert!((lod_grid_step_world(5.0, 10.0).expect("step") - 25.0).abs() < 1e-9, "2.5 quantum");
    assert!((lod_grid_step_world(9.0, 10.0).expect("step") - 50.0).abs() < 1e-9, "5 quantum");
    assert!((lod_grid_step_world(25.0, 10.0).expect("step") - 250.0).abs() < 1e-9, "and the next decade");
}

#[test]
fn grid_placement_anchor_uses_orbit_xy_and_datum_z() {
    let anchor = grid_placement_anchor(vec3_new_m(3.0, 4.0, 999.0), [0.0, 0.0, 12.5]);
    assert_eq!(anchor, vec3_new_m(3.0, 4.0, 12.5));
}

/// 📏️ Original ray-to-segment distances retain finite endpoints and the forward ray domain.
#[test]
fn ray_segment_distance_matches_neutral_three_cases() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    for row in fixture["gumball"]["raySegmentCases"].as_array().unwrap() {
        let vector = |key: &str| vec3_new_m(row[key][0].as_f64().unwrap() as f32, row[key][1].as_f64().unwrap() as f32, row[key][2].as_f64().unwrap() as f32);
        let distance = ray_segment_distance(vector("origin"), vector("direction").normalize_m(), vector("a"), vector("b")).expect("nondegenerate original segment");
        assert!((f64::from(distance) - row["distance"].as_f64().unwrap()).abs() < 1e-5, "{}: {distance}", row["name"]);
    }
}
