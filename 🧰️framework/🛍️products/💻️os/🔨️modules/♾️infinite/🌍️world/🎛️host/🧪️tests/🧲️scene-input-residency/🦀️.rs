//! 🧪️ host tests (🧲️scene-input-residency): compiled against the `testkit` surface of the world crates.

use semio_os_infinite_world_components::{World3dRelocatePlane, World3dRelocateSession};
use semio_os_infinite_world_scene::{reserve_world3d_asset_request, retire_cancelled_world3d_asset_step, return_world3d_asset, take_next_world3d_asset, world3d_asset_cancellation_requested, WorldAssetRequestKind};
use semio_os_infinite_world_bridge::sync_world3d_scene_document_lanes;
use semio_os_infinite_world_interaction::{apply_reference_image_bytes, world3d_begin_relocate_drag, world3d_cancel_relocate_drag, world3d_catalogue_drop_origin, world3d_end_relocate_drag, world3d_update_catalogue_drop_preview, world3d_update_relocate_drag};
use semio_os_infinite_world_host::World3dState;
use semio_framework_test_fixtures::fixture_text;
use ui_scene::math::{Camera3d, CameraProjection3d, Instance3d, Mat4, OrbitController, SceneDraw3d, Vec3};
use semio_ui_theme::geometry::Rect;
use semio_ui_theme::Rgba;
use semio_ui_viewport::{Viewport3dOrthographicView, Viewport3dProjectionMode, Viewport3dProjectionOrientation, Viewport3dProjectionSpec};
use serde_json::json;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(fixture_text!(path));
    }
    Box::leak(joined.into_boxed_str())
}

#[allow(non_snake_case)]
fn FIXTURE() -> &'static str {
    fixture_text!("../../../🧪️tests/🧫️fixtures/🧲️scene-input-residency/🔣️.json")
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(FIXTURE()).expect("scene input residency fixture")
}

fn point(value: &serde_json::Value) -> [f64; 3] {
    let values = value.as_array().expect("point array");
    [values[0].as_f64().expect("x"), values[1].as_f64().expect("y"), values[2].as_f64().expect("z")]
}

fn scene(references: serde_json::Value) -> semio_ui_declaration::ui::World3dScene {
    let mut scene = semio_ui_declaration::ui::World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
    scene.references_json = Some(references.to_string());
    scene
}

fn pointer_for(state: &World3dState, world: [f64; 3]) -> (f32, f32) {
    let camera = state.camera.orbit.to_camera();
    let local = ui_scene::math::project_point(ui_scene::math::projection_spec_view_proj(&camera, state.camera.projection_spec, state.session.bounds.w, state.session.bounds.h), Vec3::new(world[0] as f32, world[1] as f32, world[2] as f32), state.session.bounds.w, state.session.bounds.h).expect("point projects into the fixture camera");
    (state.session.bounds.x + local[0], state.session.bounds.y + local[1])
}

fn assert_point(actual: [f64; 3], expected: [f64; 3]) {
    for axis in 0..3 {
        assert!((actual[axis] - expected[axis]).abs() < 1e-4, "axis {axis}: {actual:?} != {expected:?}");
    }
}

fn one_pixel_png() -> Vec<u8> {
    let image = image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(1, 1, image::Rgba([17, 34, 51, 255])));
    let mut bytes = std::io::Cursor::new(Vec::new());
    image.write_to(&mut bytes, image::ImageFormat::Png).expect("PNG oracle encodes");
    bytes.into_inner()
}

#[test]
fn pointer_relocate_and_catalogue_drop_honor_the_document_snap_boolean() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let fixture = fixture();
    let target = point(&fixture["snap"]["point"]);
    let factor = fixture["snap"]["factor"].as_f64().expect("factor");
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.bounds = Rect { x: 10.0, y: 20.0, w: 400.0, h: 400.0 };
    state.camera.orbit = OrbitController::from_camera(&Camera3d {
        position: Vec3::new(0.0, 0.0, 10.0),
        target: Vec3::ZERO,
        up: Vec3::new(0.0, 1.0, 0.0),
        fov_y: 45.0_f32.to_radians(),
        near: ui_scene::math::WORLD_ORBIT_CAMERA_NEAR,
        far: ui_scene::math::WORLD_ORBIT_CAMERA_MIN_FAR,
        projection: CameraProjection3d::Orthographic,
        zoom: 20.0,
    });
    state.camera.projection_spec = semio_ui_viewport::Viewport3dProjectionSpec { mode: semio_ui_viewport::Viewport3dProjectionMode::Orthographic {}, orientation: semio_ui_viewport::Viewport3dProjectionOrientation::Cardinal { view: semio_ui_viewport::Viewport3dOrthographicView::Top } };
    state.scene.lod.grid_factor = factor;
    let (x, y) = pointer_for(&state, target);

    for (enabled, expected_key) in [(false, "disabled"), (true, "enabled")] {
        let expected = point(&fixture["snap"][expected_key]);
        state.scene.lod.grid_snap_enabled = enabled;
        state.gumball.relocate = Some(World3dRelocateSession { object_id: "object".into(), origin: [0.0; 3], from: [0.0; 3], plane: World3dRelocatePlane::Z, mesh_key: "mesh".into(), preview_model: Mat4::identity() });
        assert!(world3d_update_relocate_drag(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene, x, y));
        assert_point(translation(state.gumball.relocate.as_ref().expect("relocate ghost").preview_model), expected);
        let (_, committed) = world3d_end_relocate_drag(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene, Some((x, y))).expect("relocate commit args");
        assert_point(committed, expected);

        world3d_update_catalogue_drop_preview(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene, x, y, "catalogue-object", None);
        assert_point(state.gumball.catalogue_drop_preview.as_ref().expect("catalogue ghost").origin, expected);
        assert_point(world3d_catalogue_drop_origin(&state.session, &state.camera, &state.scene, x, y).expect("catalogue drop args"), expected);
    }
}

fn translation(model: Mat4) -> [f64; 3] {
    let [x, y, z, _] = model.cols[3];
    [f64::from(x), f64::from(y), f64::from(z)]
}

#[test]
fn cardinal_relocate_retains_the_grabbed_plane_and_model_through_release_or_cancel() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let relocate: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🚚️relocate-gesture.json")).expect("shared relocate fixture");
    let rotation = serde_json::from_value(relocate["rotation"].clone()).expect("rotation quaternion");
    let scale = serde_json::from_value(relocate["scale"].clone()).expect("nonuniform scale");
    for row in relocate["cardinal"].as_array().expect("cardinal views") {
        let view = match row["view"].as_str() {
            Some("front") => Viewport3dOrthographicView::Front,
            Some("right") => Viewport3dOrthographicView::Right,
            _ => panic!("unexpected relocate fixture view"),
        };
        let [x, y, z] = point(&row["eye"]).map(|value| value as f32);
        let mut state = World3dState::new("surface".into(), "controller".into());
        state.session.bounds = Rect { x: 10.0, y: 20.0, w: 400.0, h: 400.0 };
        state.session.active_utility = "worldRelocate".into();
        state.camera.projection_spec = Viewport3dProjectionSpec { mode: Viewport3dProjectionMode::Orthographic {}, orientation: Viewport3dProjectionOrientation::Cardinal { view } };
        state.camera.orbit = OrbitController::from_camera(&Camera3d {
            position: Vec3::new(x, y, z), target: Vec3::ZERO, up: Vec3::new(0.0, 0.0, 1.0), fov_y: 45.0_f32.to_radians(),
            near: ui_scene::math::WORLD_ORBIT_CAMERA_NEAR, far: ui_scene::math::WORLD_ORBIT_CAMERA_MIN_FAR,
            projection: CameraProjection3d::Orthographic, zoom: 20.0,
        });
        state.scene.lod.grid_snap_enabled = row["snap"].as_bool().expect("snap policy");
        state.scene.lod.grid_factor = 1.0;
        let model = Instance3d::model_from_trs(point(&row["origin"]).map(|value| value as f32), rotation, scale);
        assert!(state.scene.draws.push(SceneDraw3d { mesh_key: "source-mesh".into(), mesh_version: 1, instances: vec![
            Instance3d { component_source: None, id: "object".into(), model, color: [1.0; 4], selected: false, hovered: false, material: Default::default() },
        ], shadow_role: Default::default() }).is_ok());
        state.selection.local_hover_id = Some("object".into());
        let (grab_x, grab_y) = pointer_for(&state, point(&row["grab"]));
        let (move_x, move_y) = pointer_for(&state, point(&row["preview"]));
        let (release_x, release_y) = pointer_for(&state, point(&row["release"]));
        let projection = state.camera.projection_spec;
        for commit in [false, true] {
            state.camera.projection_spec = projection;
            assert!(world3d_begin_relocate_drag(&mut state.session, &mut state.camera, &mut state.selection, &mut state.gumball, &mut state.scene, grab_x, grab_y));
            let grabbed = state.gumball.relocate.as_ref().expect("captured relocate source");
            assert_eq!(grabbed.plane.axis(), row["axis"].as_u64().unwrap() as usize);
            assert_eq!(grabbed.preview_model.cols, model.cols);
            let mesh_storage = grabbed.mesh_key.as_ptr();
            let object_storage = grabbed.object_id.as_ptr();
            state.camera.projection_spec.orientation = Viewport3dProjectionOrientation::Free {};
            assert!(world3d_update_relocate_drag(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene, move_x, move_y));
            let preview = state.gumball.relocate.as_ref().expect("translated preview");
            assert_eq!(preview.mesh_key, "source-mesh");
            assert_eq!(preview.mesh_key.as_ptr(), mesh_storage);
            assert_eq!(preview.object_id.as_ptr(), object_storage);
            assert_eq!(preview.preview_model.cols[..3], model.cols[..3]);
            assert_point(translation(preview.preview_model), point(&row["expectedPreview"]));
            if commit {
                let (object, released) = world3d_end_relocate_drag(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene, Some((release_x, release_y))).expect("release sample commits");
                assert_eq!(object, "object");
                assert_point(released, point(&row["expectedRelease"]));
            } else {
                assert!(world3d_cancel_relocate_drag(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene));
                assert!(world3d_end_relocate_drag(&mut state.session, &mut state.camera, &mut state.gumball, &mut state.scene, Some((release_x, release_y))).is_none());
            }
            assert!(state.gumball.relocate.is_none());
            assert_eq!(state.scene.draws[0].instances[0].model.cols, model.cols);
        }
    }
}

#[test]
fn replacing_a_reference_cancels_its_claim_and_rejects_its_late_bytes() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let fixture = fixture();
    let a = fixture["referenceResidency"]["removedUrl"].as_str().expect("A");
    let b = fixture["referenceResidency"]["replacementUrl"].as_str().expect("B");
    let mut state = World3dState::new("surface".into(), "controller".into());
    sync_world3d_scene_document_lanes(&mut state.session, &mut state.selection, &mut state.scene, &mut state.bridge, &mut state.assets, &scene(serde_json::json!([{ "url": a, "origin": [0, 0, 0], "widthWorld": 1 }] )));
    assert!(apply_reference_image_bytes(&mut state.session, &mut state.authority, &mut state.scene, &mut state.assets, a, &one_pixel_png()));
    let token = reserve_world3d_asset_request(&mut state.session, &mut state.assets, WorldAssetRequestKind::ReferenceImage, a).expect("A request");
    let owner = take_next_world3d_asset(&mut state.assets).expect("A fetch owner");

    sync_world3d_scene_document_lanes(&mut state.session, &mut state.selection, &mut state.scene, &mut state.bridge, &mut state.assets, &scene(serde_json::json!([{ "url": b, "origin": [0, 0, 0], "widthWorld": 1 }] )));
    assert!(!state.scene.reference_pixels.contains_key(a), "removed A pixels retire before render");
    assert!(state.assets.asset_io.slots[usize::from(token.slot)].as_ref().is_some_and(|claim| claim.cancelled), "A's in-flight claim is URL-cancelled while the pane generation remains live");
    assert!(world3d_asset_cancellation_requested(&state.assets, token));
    return_world3d_asset(&mut state.assets, owner).unwrap_or_else(|_| panic!("cancelled A owner returns to bounded retirement"));
    while retire_cancelled_world3d_asset_step(&mut state.assets) {}

    let png = one_pixel_png();
    assert!(!apply_reference_image_bytes(&mut state.session, &mut state.authority, &mut state.scene, &mut state.assets, a, &png), "late A is rejected by active-document ownership");
    assert!(!state.scene.reference_pixels.contains_key(a));
    assert!(apply_reference_image_bytes(&mut state.session, &mut state.authority, &mut state.scene, &mut state.assets, b, &png), "current B is decoded");
    assert!(state.scene.reference_pixels.contains_key(b));
}

#[test]
fn more_than_256_distinct_reference_replacements_keep_one_resident_entry() {
    let _gate = semio_os_infinite_world_host::world_test_gate();
    let fixture = fixture();
    let count = fixture["referenceResidency"]["distinctReplacementCount"].as_u64().expect("replacement count");
    let maximum = fixture["referenceResidency"]["maximumResidentUrls"].as_u64().expect("maximum resident URLs") as usize;
    let png = one_pixel_png();
    let mut state = World3dState::new("surface".into(), "controller".into());
    for index in 0..count {
        let url = format!("/reference/replacement-{index}.png");
        sync_world3d_scene_document_lanes(&mut state.session, &mut state.selection, &mut state.scene, &mut state.bridge, &mut state.assets, &scene(serde_json::json!([{ "url": url, "origin": [0, 0, 0], "widthWorld": 1 }] )));
        assert!(apply_reference_image_bytes(&mut state.session, &mut state.authority, &mut state.scene, &mut state.assets, &url, &png), "replacement {index} is current");
        assert!(state.scene.reference_pixels.len() <= maximum, "replacement {index} retains only the active URL");
        assert!(state.session.snapshot_fault.is_none(), "replacement {index} does not report dynamic capacity");
    }
}
