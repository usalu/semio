
use super::*;
use crate::editor::puzzle5d::unit_tests::context::*;

#[test]
fn renders_the_world_scene() {
    let mut app = app();
    assert!(render_body(&mut app, BODY_KEY).contains("world-3d"));
}

//#region 🕹️Interaction
fn document_with_one_gripped_part() -> Puzzle5dDocument {
    let projection = serde_json::json!({
        "schema": "puzzle.5d",
        "parts": [{ "id": "teil-ä", "partKind": "Part", "2d": { "x": 1.0, "y": 2.0 }, "3d": { "origin": [0.0, 0.0, 0.0] }, "grips": [{ "id": "g1", "gripKind": "griff-ü", "2d": {}, "3d": { "position": [1.0, 0.0, 0.0] } }] }],
        "fasteners": []
    });
    <Puzzle5dDocument as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(&dsl::os_pack::json::parse(&projection.to_string()).expect("projection"))).expect("document")
}

fn scene_with(interaction: Puzzle5dInteractionSnapshot, runtime: Puzzle5dRuntime, active_utility: &str) -> Puzzle5dScene {
    Puzzle5dScene { document: document_with_one_gripped_part(), runtime, active_utility: active_utility.into(), interaction }
}

fn part_selection() -> Puzzle5dInteractionSnapshot {
    Puzzle5dInteractionSnapshot { granularity: crate::editor::puzzle5d::PUZZLE5D_GRANULARITY_PART.into(), selected: vec!["teil-ä".into()], hovered: Vec::new() }
}

/// 🕹️ Law: a part the framework `vortex` domain holds selected paints as `selected` on its world
/// instance, and a hovered one as `hovered` — the world half of the cross-pane law.
#[test]
fn world_instances_paint_the_live_selection_and_hover() {
    let document = document_with_one_gripped_part();
    let marked = Puzzle5dInteractionSnapshot { granularity: crate::editor::puzzle5d::PUZZLE5D_GRANULARITY_PART.into(), selected: vec!["teil-ä".into()], hovered: vec!["teil-ä".into()] };
    let instances: Value = serde_json::from_str::<serde_json::Value>(&world_instances_json(&document, &marked, None)).expect("instancesJson");
    assert_eq!(instances[0]["selected"], serde_json::json!(true));
    assert_eq!(instances[0]["hovered"], serde_json::json!(true));
    let idle: Value = serde_json::from_str(&world_instances_json(&document, &Puzzle5dInteractionSnapshot::default(), None)).expect("instancesJson");
    assert_eq!(idle[0]["selected"], serde_json::json!(false));
    assert_eq!(idle[0]["hovered"], serde_json::json!(false));
}

/// 🎯️ Law: the world selection payload carries the live ids and the hovered part, so `World3dHost`
/// highlights what the board pane picked.
#[test]
fn world_selection_json_carries_the_live_ids() {
    let mut interaction = part_selection();
    interaction.hovered = vec!["teil-ä".into()];
    let envelope = scene_with(interaction, Puzzle5dRuntime::default(), "select");
    let value: Value = serde_json::from_str(&world_selection_json_ex(&envelope)).expect("selectionJson");
    assert_eq!(value["ids"], serde_json::json!(["teil-ä"]));
    assert_eq!(value["activeObjectId"], serde_json::json!("teil-ä"));
    assert_eq!(value["hoveredId"], serde_json::json!("teil-ä"));
}

/// 🤏️ Law: `PUZZLE5D_GRIP_SHOW_SELECTED` hides grip markers until the interaction touches the part;
/// `…_ALWAYS` emits them regardless.
#[test]
fn grip_markers_follow_the_show_mode_and_the_live_marks() {
    let document = document_with_one_gripped_part();
    let selected_mode = Puzzle5dRuntime::default();
    let idle: Vec<Value> = serde_json::from_str(&world_grips_json(&document, &selected_mode, &Puzzle5dInteractionSnapshot::default(), "select")).expect("vorticesJson");
    assert!(idle.is_empty(), "selected mode emits no markers for an untouched part");
    let touched: Vec<Value> = serde_json::from_str(&world_grips_json(&document, &selected_mode, &part_selection(), "select")).expect("vorticesJson");
    assert_eq!(touched.len(), 1);
    assert_eq!(touched[0]["displayDirection"], serde_json::json!(crate::editor::puzzle5d::PUZZLE5D_GRIP_DIRECTION_OUTWARDS));
    let always = Puzzle5dRuntime { grip_show: PUZZLE5D_GRIP_SHOW_ALWAYS.into(), ..Puzzle5dRuntime::default() };
    let shown: Vec<Value> = serde_json::from_str(&world_grips_json(&document, &always, &Puzzle5dInteractionSnapshot::default(), "select")).expect("vorticesJson");
    assert_eq!(shown.len(), 1);
}

/// 🎛️ Law: the gumball renders only with a transform utility armed, a live part selection AND at
/// least one handle flag on — all three off is a gumball nobody could grab.
#[test]
fn gumball_needs_a_transform_utility_a_selection_and_a_flag() {
    let runtime = Puzzle5dRuntime::default();
    assert!(crate::editor::puzzle5d::puzzle5d_gumball_active(&runtime, "move", &part_selection()));
    assert!(!crate::editor::puzzle5d::puzzle5d_gumball_active(&runtime, "select", &part_selection()));
    assert!(!crate::editor::puzzle5d::puzzle5d_gumball_active(&runtime, "move", &Puzzle5dInteractionSnapshot::default()));
    let flagless = Puzzle5dRuntime { transform_move: false, transform_rotate: false, ..Puzzle5dRuntime::default() };
    assert!(!crate::editor::puzzle5d::puzzle5d_gumball_active(&flagless, "move", &part_selection()));
}
//#endregion 🕹️Interaction

//#region ☑️Options
/// 🎚️ Law: every option group this pane owns is present, and each one reflects the runtime it was
/// built from — the render half of "dispatch → config changes → measure reflects it".
#[test]
fn window_measures_expose_every_world_option_group() {
    let labels = &crate::editor::puzzle5d::terminology::Puzzle5dLabels::NATIVE_EN;
    let runtime = Puzzle5dRuntime { grid_visible: false, grid_spacing: 4.0, lod_automatic: false, lod_manual: 250.0, ..Puzzle5dRuntime::default() };
    let envelope = scene_with(Puzzle5dInteractionSnapshot::default(), runtime, "select");
    let measures = window_measures(&envelope, labels);
    let ids = measure_ids(&measures);
    for expected in [
        "puzzle5d-play-world-grid",
        "puzzle5d-play-world-grid-visible",
        "puzzle5d-play-world-grid-snap",
        "puzzle5d-play-world-grid-spacing",
        "puzzle5d-play-world-lod",
        "puzzle5d-play-world-lod-auto",
        "puzzle5d-play-world-lod-depth-variable",
        "puzzle5d-play-world-lod-value",
        "puzzle5d-play-world-select",
        "puzzle5d-play-world-select-parts",
        "puzzle5d-play-world-select-grips",
        "puzzle5d-play-world-select-fasteners",
        "puzzle5d-play-world-grip-show",
        "puzzle5d-play-world-grip-direction",
        "puzzle5d-measure-projection-orthographic",
        "puzzle5d-play-utility-options-move-move",
        "puzzle5d-play-utility-options-rotate-rotate",
    ] {
        assert!(ids.iter().any(|id| id == expected), "missing measure {expected} in {ids:?}");
    }
    assert_eq!(toggle_pressed(&measures, "puzzle5d-play-world-grid-visible"), Some(false));
    assert_eq!(toggle_pressed(&measures, "puzzle5d-play-world-lod-auto"), Some(false));
    assert_eq!(slider_value(&measures, "puzzle5d-play-world-grid-spacing"), Some(4.0));
    assert_eq!(slider_value(&measures, "puzzle5d-play-world-lod-value"), Some(250.0));
}

/// 🎥️ Law: the camera payload is derived from this pane's projection, not a fixed 45° perspective —
/// switching to an orthographic view changes what the host reads.
#[test]
fn camera_json_follows_the_projection() {
    let mut camera = Puzzle5dCamera3d::default();
    camera.position = [8.0, -8.0, 8.0];
    camera.zoom = 1.0;
    let perspective: Value = serde_json::from_str(&camera3d_json(&camera)).expect("cameraJson");
    camera.projection.kind = "orthographic".into();
    camera.projection.orthographic_view = "front".into();
    let orthographic: Value = serde_json::from_str(&camera3d_json(&camera)).expect("cameraJson");
    assert_ne!(perspective, orthographic, "the projection must reach the host camera payload");
}
//#endregion ☑️Options

//#region 🧊️MeshLane
/// 🧊️ LAW: every instance this window publishes names a mesh the SAME scene publishes, and a document
/// whose parts carry `/mesh/…` urls publishes at least one real GLB row.
///
/// 🐛️ An instance whose `meshId` no `meshesJson` row declares draws NOTHING — the renderer has no
/// geometry to attach to it and never even fetches a GLB. The 2026-09-21 play-grid audit saw exactly
/// that on the `puzzle5d` pane: booted ready, a curated example named in the picker, an empty 3D
/// viewport and not one `/mesh/*.glb` request on the wire. The window builds `meshesJson` from a mesh
/// LANE handed in by `puzzle5d_mesh_lane` while `instancesJson` resolves each part's url itself, so the
/// two can disagree without any error anywhere — this law is what ties them together.
#[test]
fn every_world_instance_names_a_mesh_this_scene_publishes() {
    let mut app = app();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": crate::editor::puzzle5d::PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
    let rendered = render_body(&mut app, BODY_KEY);
    close_app(&mut app);
    let scene: serde_json::Value = serde_json::from_str(&rendered).expect("world scene json");
    let meshes: serde_json::Value = serde_json::from_str(scene["scene"]["meshesJson"].as_str().expect("the world scene carries meshesJson")).expect("meshesJson parses");
    let instances: serde_json::Value = serde_json::from_str(scene["scene"]["instancesJson"].as_str().expect("the world scene carries instancesJson")).expect("instancesJson parses");
    let published: Vec<&str> = meshes.as_array().expect("mesh rows").iter().filter_map(|mesh| mesh["id"].as_str()).collect();
    let rows = instances.as_array().expect("instance rows");
    assert!(!rows.is_empty(), "the loaded example must place world instances");
    for instance in rows {
        let mesh_id = instance["meshId"].as_str().unwrap_or_default();
        assert!(published.contains(&mesh_id), "instance {} names mesh {mesh_id:?}, which this scene never publishes: {published:?}", instance["id"]);
    }
    assert!(
        meshes.as_array().expect("mesh rows").iter().any(|mesh| mesh["url"].as_str().is_some_and(|url| url.starts_with("/mesh/"))),
        "a document whose parts carry mesh urls must publish at least one real GLB row: {meshes}"
    );
}
//#endregion 🧊️MeshLane

//#region 🎯️Fit
/// 🎯️ ticket 26/09/19 (play grid, visual audit): the `puzzle5d` pane booted ready with its curated
/// example named in the picker, the board pane drew the assembled ring of capsules and the 3D
/// viewport drew nothing but the camera gizmo and the grid. A world window that publishes no `fit`
/// lane can be framed by nothing but the camera the document authored, so a document whose parts sit
/// anywhere else is outside that frustum. Law: this window always stages the one-shot fit, and it
/// stages it WITHOUT bounds — this crate knows mesh urls, never delivered mesh extents.
#[test]
fn the_world_scene_stages_a_one_shot_fit_for_its_document() {
    let scene = scene_with(part_selection(), Puzzle5dRuntime::default(), "select");
    let node = render(&scene, None, &[]).expect("the world surface assembles");
    let built: World3dScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("the world surface decodes as a 3d scene");
    let fit = built.fit_json.as_deref().expect("the world window must publish a fit lane");
    assert!(fit.contains("\"enabled\":true"), "the fit lane must be enabled, else the authored camera is the only framing: {fit}");
    assert!(fit.contains(&format!("\"revision\":{}", world_fit_revision(&scene.document))), "the lane must carry this document's own identity revision: {fit}");
    assert!(!fit.contains("boundsMin"), "puzzle5d knows mesh urls, never mesh extents — the host measures what it loaded: {fit}");
}

/// 🎯️ The fit revision is a document IDENTITY, not a geometry digest: loading another example refits,
/// dragging a part must not yank the camera out from under the hand that is dragging it.
#[test]
fn the_fit_revision_tracks_document_identity_not_part_edits() {
    let base = document_with_one_gripped_part();
    let mut moved = base.clone();
    moved.parts[0].part_3d.origin = [120.0, -40.0, 7.5];
    assert_eq!(world_fit_revision(&base), world_fit_revision(&moved), "moving a part is not a document swap");

    let mut relabelled = base.clone();
    relabelled.label = Some("Capsule Dream".into());
    assert_ne!(world_fit_revision(&base), world_fit_revision(&relabelled), "another example is another document and gets its own framing");

    let mut recatalogued = base.clone();
    recatalogued.kind_catalogs = Some(serde_json::json!({ "parts": [{ "id": "other-kind" }] }));
    assert_ne!(world_fit_revision(&base), world_fit_revision(&recatalogued), "a different kind catalog resolves different meshes, so it is framed anew");
}
//#endregion 🎯️Fit

