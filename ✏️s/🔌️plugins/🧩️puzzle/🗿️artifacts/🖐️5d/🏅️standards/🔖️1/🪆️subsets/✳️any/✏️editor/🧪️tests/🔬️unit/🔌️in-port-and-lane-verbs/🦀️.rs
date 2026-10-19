//! 🔌️ Kit in-port, clipboard maximum and window and config lane verb tests of the puzzle 5d editor. See [`super`].
use crate::editor::puzzle5d::catalog::puzzle5d_normalize_kind_weight_group;
use semio_framework_test_fixtures::fixture_text;

use super::*;

//#region 🔖️KitInPort
#[semio_framework_async_macros::async_test]
async fn kit_in_retained_import_media_dispatches_the_exact_factory_and_applies_canonical_output() {
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let fragment = serde_json::json!({
        "schema": "manifest",
        "objectKinds": [{
            "id": "retained-capsule",
            "name": "retained-capsule",
            "label": "Retained Capsule",
            "meshUrl": "/mesh/retained-capsule.glb",
            "vortices": [{ "id": "v0", "vortexKind": "retained-door", "position": [0.0, 0.0, 0.0], "direction": [0.0, 1.0, 0.0], "radius": 0.3 }],
        }],
        "vortexKinds": [{ "id": "retained-door", "name": "retained-door", "label": "Retained Door", "color": "#ff0000", "defaultCableKind": "" }],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [{ "source": "retained-door", "target": "retained-door", "bidirectional": true }],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };
    let result = app.0.import_media("kit:in", media, &meta("local"), app.1.cx()).await.expect("retained kit:in dispatch");
    assert!(!result.mutations.is_empty(), "exact retained import must publish document mutations");
    let after = projection_of(&app);
    assert_ne!(after, before, "exact retained import must apply its completion output");
    let snapshot: Puzzle5dSnapshot = semio_framework_pack_json::from_json_str(&after.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("retained projection deserializes");
    let catalogs = crate::standards::v1::subsets::any::schema::kind_catalogs::kind_catalogs_of(&snapshot.kind_catalogs, &snapshot.kind_catalogs_extra).expect("retained catalog replacement applied");
    let part = catalogs.parts.iter().find(|part| part.id == "retained-capsule").expect("retained part catalog row");
    assert_eq!(part.grips.first().and_then(|grip| grip.grip_kind.as_deref()), Some("retained-door"));
    assert!(catalogs.grips.iter().any(|grip| grip.id == "retained-door"));
}

#[semio_framework_async_macros::async_test]
async fn kit_in_retained_import_media_enforces_exact_media_max_plus_one_before_decode() {
    let prefix = r#"{"objectKinds":[],"vortexKinds":[{"id":"grip","name":"Grip","label":""#;
    let suffix = r##"","color":"#fff","defaultCableKind":""}],"kindCompatibility":[]}"##;
    let label = "x".repeat(PUZZLE5D_IMPORT_MEDIA_BYTES.checked_sub(prefix.len() + suffix.len()).expect("retained max fixture shell"));
    let maximum = format!("{prefix}{label}{suffix}");
    assert_eq!(maximum.len(), PUZZLE5D_IMPORT_MEDIA_BYTES);
    let mut app = app_with_registry();
    let exact = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: maximum.clone() } };
    app.0.import_media("kit:in", exact, &meta("local"), app.1.cx()).await.expect("exact media maximum retained dispatch");
    let after_exact = projection_of(&app);
    let plus_one = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: format!("{maximum} ") } };
    let error = app.0.import_media("kit:in", plus_one, &meta("local"), app.1.cx()).await.expect_err("media maximum plus one must fail before Serde");
    assert!(error.message.contains("predecode cap"), "the plus-one refusal must be the predecode cap, got {:?} {:?}", error.code, error.message);
    assert_eq!(projection_of(&app), after_exact, "rejected plus-one media must not mutate the document");
}

/// 🔌️ The flagship `kit:in` seam: feeding a `kit.catalog` fragment shaped exactly like
/// block3d's `puzzle3d_catalog_fragment` (`objectKinds`/`vortexKinds`, camelCase) through
/// `Puzzle5dPlayApp::import_media` must normalize `objectKinds` into the typed
/// `kindCatalogs.parts` (with each per-object `vortices[]` entry becoming a grip template) and
/// `vortexKinds` into `kindCatalogs.grips`, and land both after applying the returned operations.
#[semio_framework_async_macros::async_test]
async fn kit_in_retained_import_media_upserts_part_and_grip_kinds_into_kind_catalogs() {
    let mut app = app_with_registry();
    let fragment = serde_json::json!({
        "schema": "manifest",
        "objectKinds": [{
            "id": "capsule",
            "name": "capsule",
            "label": "Capsule",
            "meshUrl": "/mesh/capsule.glb",
            "vortices": [{ "id": "v0", "vortexKind": "door", "position": [0.0, 0.0, 0.0], "direction": [0.0, 1.0, 0.0], "radius": 0.3 }],
        }],
        "vortexKinds": [{ "id": "door", "name": "door", "label": "Door", "color": "#ff0000", "defaultCableKind": "" }],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [{ "source": "door", "target": "door", "bidirectional": true }],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

    let result = app.0.import_media("kit:in", media, &meta("local"), app.1.cx()).await.expect("retained kit:in import succeeds");
    assert!(!result.mutations.is_empty(), "importing a non-empty fragment must emit real operations");
    let next_projection = projection_of(&app);

    // 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W4d: `next_projection`'s raw
    // `kindCatalogs` key is now the composed `{childId,target}` handle, not the embedded
    // `{parts:[...],...}` shape a JSON pointer could probe directly — reassemble the full
    // `Puzzle5dKindCatalogs` through the typed snapshot + `kind_catalogs_of` accessor instead
    // (same pattern `sourcing`'s `stock_of` established for its own composed catalog field).
    let next_snapshot: Puzzle5dSnapshot = semio_framework_pack_json::from_json_str(&next_projection.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("next_projection deserializes as Puzzle5dSnapshot");
    let catalogs = crate::standards::v1::subsets::any::schema::kind_catalogs::kind_catalogs_of(&next_snapshot.kind_catalogs, &next_snapshot.kind_catalogs_extra).expect("parts catalog present");
    let capsule = catalogs.parts.iter().find(|entry| entry.id == "capsule").expect("the imported part kind must appear in kindCatalogs.parts");
    assert_eq!(capsule.representations.first().map(|representation| representation.url.as_str()), Some("/mesh/capsule.glb"));
    assert_eq!(capsule.grips.first().and_then(|grip| grip.grip_kind.as_deref()), Some("door"), "the per-part grip template keeps its gripKind after normalization");
    assert_eq!(capsule.grips.first().map(|grip| grip.point), Some([0.0, 0.0, 0.0]));
    assert_eq!(capsule.grips.first().map(|grip| grip.direction), Some([0.0, 1.0, 0.0]));
    assert_eq!(capsule.grips.first().and_then(|grip| grip.radius), Some(0.3));

    let door = catalogs.grips.iter().find(|entry| entry.id == "door").expect("the imported grip kind must appear in kindCatalogs.grips");
    assert_eq!(door.default_rope_kind.as_str(), "", "defaultCableKind maps onto defaultRopeKind (a naming judgment call — see import_media's doc comment)");

    let compatibility = next_projection.pointer("/kindCompatibility").and_then(Value::as_array).expect("kind compatibility present");
    assert!(compatibility.iter().any(|entry| entry.get("source").and_then(Value::as_str) == Some("door") && entry.get("target").and_then(Value::as_str) == Some("door")));
}

/// 🔁️ Re-importing the SAME fragment (simulating a second producer edge, or a redelivered
/// message on a `multiplicity: Many` port) must upsert idempotently — no duplicate rows.
#[semio_framework_async_macros::async_test]
async fn kit_in_retained_import_media_is_idempotent_on_repeated_delivery() {
    let mut app = app_with_registry();
    let fragment = serde_json::json!({
        "objectKinds": [{ "id": "capsule", "name": "capsule", "label": "Capsule", "meshUrl": "/mesh/capsule.glb", "vortices": [] }],
        "vortexKinds": [],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

    for _ in 0..2 {
        app.0.import_media("kit:in", media.clone(), &meta("local"), app.1.cx()).await.expect("retained kit:in import succeeds");
    }

    let current = projection_of(&app);
    let current_snapshot: Puzzle5dSnapshot = semio_framework_pack_json::from_json_str(&current.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("current deserializes as Puzzle5dSnapshot");
    let catalogs = crate::standards::v1::subsets::any::schema::kind_catalogs::kind_catalogs_of(&current_snapshot.kind_catalogs, &current_snapshot.kind_catalogs_extra).expect("parts catalog present");
    assert_eq!(catalogs.parts.iter().filter(|entry| entry.id == "capsule").count(), 1, "repeated delivery of the same fragment must upsert, never duplicate");
}

#[semio_framework_async_macros::async_test]
async fn kit_in_port_is_declared_on_the_app_io() {
    let io = Puzzle5dPlayApp::io().expect("puzzle5d declares an AppIo");
    let kit_in = io.ports.iter().find(|port| port.id == "kit:in").expect("kit:in port declared");
    assert_eq!(kit_in.kind_id.as_deref(), Some("kit.catalog"));
    assert_eq!(kit_in.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
    assert!(matches!(kit_in.multiplicity, PortMultiplicity::Many));
    let design_out = io.ports.iter().find(|port| port.id == "design:out").expect("design:out port declared");
    assert_eq!(design_out.kind_id.as_deref(), Some("5d.puzzle"));
    assert_eq!(design_out.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Design });
    assert!(matches!(design_out.multiplicity, PortMultiplicity::Many));
}
//#endregion 🔖️KitInPort

//#region 🔖️ClipboardMaximum
static CLIPBOARD_MAXIMUM_FIXTURE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../🪆️subsets/✳️any/🧪️tests/🧫️fixtures/📋️clipboard-maximum/🔣️.json"));

/// 📋️ LAW (language-neutral fixture): on the largest shipped example with every part selected, the copy fragment
/// carries every part, the cut removes every copied part and the paste of that fragment creates every part again.
/// Each verb runs as one step of its framework-reserved job (cost in `📓️wave-W2-B.md` §5); `import-media` is not
/// implemented by puzzle 5d (only the retained `kit:in` import is).
#[test]
fn clipboard_verbs_cover_every_part_of_the_largest_example() {
    let fixture: serde_json::Value = serde_json::from_str((*CLIPBOARD_MAXIMUM_FIXTURE)).expect("clipboard fixture parses");
    assert_eq!(fixture["example"], PUZZLE5D_EXAMPLE_CAPSULE_DREAM);
    let offset = fixture["pasteOffset"].as_array().expect("offset").iter().map(|axis| axis.as_f64().expect("axis")).collect::<Vec<_>>();
    let document = capsule_dream_example_document();
    assert_eq!(document.parts.len() as u64, fixture["parts"].as_u64().expect("parts"));
    let snapshot = Puzzle5dPlaySnapshot::new(crate::editor::puzzle5d::entities::puzzle5d_snapshot_from_document(&document).expect("typed host document admits"));
    let history = semio_framework_plugin::HistoryView::empty();
    let view = ArtifactView::new(&snapshot, &history);
    let part_ids: Vec<String> = document.parts.iter().map(|part| part.id.clone()).collect();
    let placement = PastePlacement { anchor: PasteAnchor::Original, position: Some([offset[0], offset[1], offset[2]]) };
    let fragment = puzzle5d_copy_fragment(&snapshot, &part_ids, &[]).expect("copy every part");
    let cut_operations = puzzle5d_cut_operations(&snapshot, &part_ids, &[]);
    let paste_operations = <Puzzle5dPlayApp as ArtifactEditor>::paste_operations(&view, &fragment, &placement).expect("paste every part");
    assert_eq!(fragment.label, format!("{} part(s)", document.parts.len()));
    assert_eq!(cut_operations.iter().filter(|operation| matches!(operation, Puzzle5dMutation::DeletePart(_))).count(), document.parts.len(), "the cut removes every copied part");
    assert_eq!(paste_operations.iter().filter(|operation| matches!(operation, Puzzle5dMutation::CreatePart(_))).count(), document.parts.len(), "the paste creates every fragment part");
}
//#endregion 🔖️ClipboardMaximum

//#region 🎚️WindowAndConfigLaneVerbs
/// 🪟️ The exact board pane partition `window_id` currently holds, projected typed out of the concrete
/// window registry — the same authority the retained route publishes into.
pub(super) async fn board_window_config(app: &mut Puzzle5dTestApp, window_id: &str) -> window_ownership::Puzzle5dBoardWindowConfig {
    let view = window_view(board2d::WINDOW_KIND_ID, window_id);
    semio_framework_plugin::artifact_app_laws::capture_fixture_window_config::<window_ownership::Puzzle5dBoardWindowConfigOwner, _, _>(app, &view)
        .await
        .expect("board window config capture")
        .expect("the addressed board pane owns a partition")
}

/// 🌍️ The exact world pane partition `window_id` currently holds.
pub(super) async fn world_window_config(app: &mut Puzzle5dTestApp, window_id: &str) -> window_ownership::Puzzle5dWorldWindowConfig {
    let view = window_view(world3d::WINDOW_KIND_ID, window_id);
    semio_framework_plugin::artifact_app_laws::capture_fixture_window_config::<window_ownership::Puzzle5dWorldWindowConfigOwner, _, _>(app, &view)
        .await
        .expect("world window config capture")
        .expect("the addressed world pane owns a partition")
}

/// 📜️ The declared lanes of one migrated tool id.
pub(super) fn declared_lanes(tool_id: &str) -> &'static [ArtifactToolPublicationLane] {
    <Puzzle5dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .find(|contract| contract.tool_id == tool_id)
        .unwrap_or_else(|| panic!("{tool_id} declares a publication contract"))
        .lanes
}

/// 🧵️ LAW: every verb this slice migrated is a live retained tool — registered in the id list, in the
/// bounded first-step proofs, in the publication contracts, and classified `Migrated`. A
/// `BatchOnlyPendingRewrite` row here is the repo's known hard-dead runtime pattern: the click reaches
/// nothing and no fault surfaces.
#[test]
fn window_and_config_lane_verbs_are_registered_migrated_retained_tools() {
    let source = editor_source();
    let source = source.as_str();
    let production = source.split_once("//#region 🧪️UnitTests").map(|(production, _)| production).expect("production prefix");
    for tool_id in MIGRATED_WINDOW_LANE_VERBS.iter().chain(MIGRATED_CONFIG_LANE_VERBS) {
        assert!(PUZZLE5D_RETAINED_TOOL_IDS.contains(tool_id), "{tool_id} must be a retained tool id");
        assert!(production.contains(&format!(r#".action_interactive_job("{tool_id}", InteractiveJobClassification::Migrated)"#)), "{tool_id} must be classified Migrated");
        assert!(!production.contains(&format!(r#".action_interactive_job("{tool_id}", InteractiveJobClassification::BatchOnlyPendingRewrite)"#)), "{tool_id} must carry no BatchOnlyPendingRewrite row");
        assert!(!declared_lanes(tool_id).is_empty(), "{tool_id} must declare its publication lanes");
    }
    for tool_id in MIGRATED_WINDOW_LANE_VERBS {
        assert_eq!(declared_lanes(tool_id), &[ArtifactToolPublicationLane::WindowConfig], "{tool_id} publishes the addressed pane's window config alone");
    }
    for tool_id in MIGRATED_CONFIG_LANE_VERBS {
        assert_eq!(declared_lanes(tool_id), &[ArtifactToolPublicationLane::Config], "{tool_id} publishes the shared app config alone");
    }
    // 🗡️ The pre-5A1 vocabulary must be gone everywhere, aliases included.
    assert!(!production.contains("setObjectKindWeight") && !production.contains("setVortexKindWeight"), "the object/vortex weight verb ids are renamed to the 5d part/grip vocabulary");
}

/// 🪟️ The window-config-lane verbs this slice migrated.
const MIGRATED_WINDOW_LANE_VERBS: &[&str] =
    &["setCamera", "setCamera2d", "setCamera3d", "setGridFactor", "setGridSnapEnabled", "setLodMode", "setSuggestionOffset", "toggleSun", "setSunAzimuth", "setSunElevation", "setSunIntensity"];

/// 🎚️ The shared-app-config-lane verbs this slice migrated.
const MIGRATED_CONFIG_LANE_VERBS: &[&str] = &["setBrushPlacementContactTolerance", "setPartKindWeight", "setGripKindWeight"];

/// 📜️ LAW: the wire band the retained factory admits is the one its bounded first-step proofs declare —
/// a proof advertising more than the factory admits is a contract the runtime refuses, and the shared
/// 8 KiB puzzle default is too narrow for a Nakagin-sized document's command wire.
#[test]
fn retained_factory_contract_matches_its_declared_proof_band() {
    let contract = ToolJobFactory::execution_contract(&Puzzle5dRetainedCommandJobFactory::new("s.puzzle.puzzle5d@1/*#editor"));
    assert_eq!(contract.max_raw_wire_bytes, PUZZLE5D_RETAINED_RAW_BYTES);
    assert_eq!(PUZZLE5D_RETAINED_RAW_BYTES, crate::retained_command::PUZZLE_IMPORT_RAW_BYTES, "the retained route admits one whole import");
    assert_eq!(PUZZLE5D_RETAINED_DECODED_ITEMS, 16_384);
    assert!(PUZZLE5D_RETAINED_RAW_BYTES > crate::retained_command::PUZZLE_COMMAND_RAW_BYTES, "the retained route must admit more than the shared puzzle default it replaced");
}

/// 🎥️ LAW (camera family): `setCamera` is scoped by the ADDRESSED WINDOW, not by the payload shape —
/// the board pane writes the flat camera, the world pane the volume camera — and each pane publishes
/// only its own `WindowConfig`, leaving the document and the shared app config untouched.
#[semio_framework_async_macros::async_test]
async fn camera_verbs_publish_only_the_addressed_window_config() {
    let mut app = Box::new(app_with_registry());
    let board = "puzzle5d-camera-board";
    let world = "puzzle5d-camera-world";
    let document_before = projection_of(&app);
    let config_before = app.config_pack().await.expect("app config before camera publications");

    let flat = dispatch(&mut app, "setCamera", Some(&semio_framework_pack_json::json!({ "windowId": board2d::WINDOW_KIND_ID, "camera": { "x": 4.5, "y": -2.5, "zoom": 3.0 } })), Some(board)).expect("setCamera in the board pane");
    assert!(flat.mutations.is_empty(), "a camera pose is never a document edit");
    assert_eq!(board_window_config(&mut app, board).await.camera2d, Puzzle5dCamera2d { x: 4.5, y: -2.5, zoom: 3.0 }, "the board pane's setCamera wrote its own flat camera");

    let volume = dispatch(&mut app, "setCamera", Some(&semio_framework_pack_json::json!({ "windowId": world3d::WINDOW_KIND_ID, "camera": { "position": [1.0, 2.0, 3.0], "target": [4.0, 5.0, 6.0], "zoom": 2.0 } })), Some(world)).expect("setCamera in the world pane");
    assert!(volume.mutations.is_empty());
    let volume_camera = world_window_config(&mut app, world).await.camera3d;
    assert_eq!((volume_camera.position, volume_camera.target, volume_camera.zoom), ([1.0, 2.0, 3.0], [4.0, 5.0, 6.0], 2.0), "the world pane's setCamera wrote its own volume camera");

    dispatch(&mut app, "setCamera2d", Some(&semio_framework_pack_json::json!({ "windowId": board2d::WINDOW_KIND_ID, "camera": { "x": -8.0, "y": 9.0, "zoom": 0.5 } })), Some(board)).expect("setCamera2d");
    assert_eq!(board_window_config(&mut app, board).await.camera2d, Puzzle5dCamera2d { x: -8.0, y: 9.0, zoom: 0.5 });
    dispatch(&mut app, "setCamera3d", Some(&semio_framework_pack_json::json!({ "windowId": world3d::WINDOW_KIND_ID, "camera": { "position": [7.0, 7.0, 7.0], "target": [0.0, 0.0, 0.0], "zoom": 1.25 } })), Some(world)).expect("setCamera3d");
    assert_eq!(world_window_config(&mut app, world).await.camera3d.position, [7.0, 7.0, 7.0]);

    assert_eq!(projection_of(&app), document_before, "no camera verb touches the document lane");
    let config_after = app.config_pack().await.expect("app config after camera publications");
    assert_eq!((config_after.pack, config_after.spr), (config_before.pack, config_before.spr), "no camera verb touches the shared app config lane");
    close_app(&mut app);
}

/// 🧲️ LAW (grid family): the snap toggle reads its `pressed` state and flips without one — reading an
/// absent argument as `false` turned every press into an unconditional "off" — and the factor takes an
/// absolute `value` or a `delta` nudge, clamped into the declared band the window schema states as
/// `exclusiveMinimum: 0`.
#[semio_framework_async_macros::async_test]
async fn grid_verbs_publish_the_clamped_board_window_config() {
    let mut app = Box::new(app_with_registry());
    let board = "puzzle5d-grid-board";
    assert!(window_ownership::Puzzle5dBoardWindowConfig::default().grid_snap_enabled, "snap starts on");

    dispatch(&mut app, "setGridSnapEnabled", Some(&semio_framework_pack_json::json!({ "pressed": false })), Some(board)).expect("press the snap toggle off");
    assert!(!board_window_config(&mut app, board).await.grid_snap_enabled, "the pressed state is what persists");
    dispatch(&mut app, "setGridSnapEnabled", None, Some(board)).expect("argument-less snap toggle");
    assert!(board_window_config(&mut app, board).await.grid_snap_enabled, "an argument-less invocation flips the current state");

    dispatch(&mut app, "setGridFactor", Some(&semio_framework_pack_json::json!({ "value": 4.0 })), Some(board)).expect("absolute grid factor");
    assert_eq!(board_window_config(&mut app, board).await.grid_factor, 4.0);
    dispatch(&mut app, "setGridFactor", Some(&semio_framework_pack_json::json!({ "delta": -1.0 })), Some(board)).expect("grid factor stepper nudge");
    assert_eq!(board_window_config(&mut app, board).await.grid_factor, 3.0, "a delta nudges the current factor");
    dispatch(&mut app, "setGridFactor", Some(&semio_framework_pack_json::json!({ "value": 0.0 })), Some(board)).expect("grid factor below the band");
    assert_eq!(board_window_config(&mut app, board).await.grid_factor, PUZZLE5D_GRID_FACTOR_MIN, "the schema's exclusive minimum is what the clamp protects");
    dispatch(&mut app, "setGridFactor", Some(&semio_framework_pack_json::json!({ "value": 1_000.0 })), Some(board)).expect("grid factor above the band");
    assert_eq!(board_window_config(&mut app, board).await.grid_factor, PUZZLE5D_GRID_FACTOR_MAX);
    close_app(&mut app);
}

/// 🔭️🧭️ LAW (LOD + suggestion offset): both persist into the addressed board pane's own partition, and
/// the offset clamps into the band its slider declares whether it arrives absolute or as a nudge.
#[semio_framework_async_macros::async_test]
async fn lod_and_suggestion_offset_publish_the_board_window_config() {
    let mut app = Box::new(app_with_registry());
    let board = "puzzle5d-lod-board";
    dispatch(&mut app, "setLodMode", Some(&semio_framework_pack_json::json!({ "value": "manual" })), Some(board)).expect("setLodMode");
    assert_eq!(board_window_config(&mut app, board).await.lod_mode, "manual");
    dispatch(&mut app, "setLodMode", Some(&semio_framework_pack_json::json!({ "value": PUZZLE5D_LOD_MODE_AUTOMATIC })), Some(board)).expect("setLodMode back to automatic");
    assert_eq!(board_window_config(&mut app, board).await.lod_mode, PUZZLE5D_LOD_MODE_AUTOMATIC);

    dispatch(&mut app, "setSuggestionOffset", Some(&semio_framework_pack_json::json!({ "value": PUZZLE5D_SUGGESTION_OFFSET_MAX + 1.0 })), Some(board)).expect("setSuggestionOffset above the band");
    assert_eq!(board_window_config(&mut app, board).await.suggestion_offset, PUZZLE5D_SUGGESTION_OFFSET_MAX);
    dispatch(&mut app, "setSuggestionOffset", Some(&semio_framework_pack_json::json!({ "delta": -1.0 })), Some(board)).expect("setSuggestionOffset stepper nudge");
    assert_eq!(board_window_config(&mut app, board).await.suggestion_offset, PUZZLE5D_SUGGESTION_OFFSET_MAX - 1.0, "the settings stepper's delta nudges the current offset");
    dispatch(&mut app, "setSuggestionOffset", Some(&semio_framework_pack_json::json!({ "value": PUZZLE5D_SUGGESTION_OFFSET_MIN - 1.0 })), Some(board)).expect("setSuggestionOffset below the band");
    assert_eq!(board_window_config(&mut app, board).await.suggestion_offset, PUZZLE5D_SUGGESTION_OFFSET_MIN);
    close_app(&mut app);
}

/// ☀️ LAW (sun family): the four sun verbs publish the world pane's own partition and nothing else.
#[semio_framework_async_macros::async_test]
async fn sun_verbs_publish_the_world_window_config() {
    let mut app = Box::new(app_with_registry());
    let world = "puzzle5d-sun-world";
    let document_before = projection_of(&app);
    let enabled_before = window_ownership::Puzzle5dWorldWindowConfig::default().sun.enabled;
    dispatch(&mut app, "toggleSun", None, Some(world)).expect("toggleSun");
    assert_eq!(world_window_config(&mut app, world).await.sun.enabled, !enabled_before, "toggling the sun flips the world pane's own environment");
    dispatch(&mut app, "setSunAzimuth", Some(&semio_framework_pack_json::json!({ "value": 123.0 })), Some(world)).expect("setSunAzimuth");
    dispatch(&mut app, "setSunElevation", Some(&semio_framework_pack_json::json!({ "value": 41.0 })), Some(world)).expect("setSunElevation");
    dispatch(&mut app, "setSunIntensity", Some(&semio_framework_pack_json::json!({ "value": 0.25 })), Some(world)).expect("setSunIntensity");
    let sun = world_window_config(&mut app, world).await.sun;
    assert_eq!((sun.azimuth, sun.elevation, sun.intensity), (123.0, 41.0, 0.25));
    assert_eq!(projection_of(&app), document_before, "no sun verb touches the document lane");
    close_app(&mut app);
}

/// ⚖️🚧️ LAW (Config-lane family): the contact tolerance and both renamed kind-weight verbs are shared
/// app configuration, not window state — dispatched through the real retained factory they publish the
/// `Config` lane, leave the addressed pane's window partition at generation zero and never touch the
/// document.
#[semio_framework_async_macros::async_test]
async fn config_lane_verbs_publish_the_shared_app_config_only() {
    let mut app = Box::new(app_with_registry());
    let pane = "puzzle5d-config-pane";
    let view = window_view(world3d::WINDOW_KIND_ID, pane);
    let document_before = projection_of(&app);
    let mut config_before = app.config_pack().await.expect("app config before");

    let part_kind = projection_of(&app).get("parts").and_then(Value::as_array).and_then(|parts| parts.first()).and_then(|part| part.get("partKind")).and_then(Value::as_str).expect("the boot document carries a part kind").to_string();
    let grip_kind = projection_of(&app)
        .get("parts")
        .and_then(Value::as_array)
        .and_then(|parts| parts.iter().find_map(|part| part.get("grips").and_then(Value::as_array).and_then(|grips| grips.first()).and_then(|grip| grip.get("gripKind")).and_then(Value::as_str)))
        .expect("the boot document carries a grip kind")
        .to_string();

    for (tool_id, args) in [
        ("setBrushPlacementContactTolerance", semio_framework_pack_json::json!({ "value": 0.02 })),
        ("setPartKindWeight", semio_framework_pack_json::json!({ "kindId": part_kind.as_str(), "value": 0.75 })),
        ("setGripKindWeight", semio_framework_pack_json::json!({ "kindId": grip_kind.as_str(), "value": 0.25 })),
    ] {
        let result = dispatch(&mut app, tool_id, Some(&args), Some(pane)).expect("Config-lane dispatch through the retained factory");
        assert!(result.mutations.is_empty(), "{tool_id} is never a document edit");
        let after = app.config_pack().await.expect("app config after a Config-lane publication");
        assert_ne!((&after.pack, &after.spr), (&config_before.pack, &config_before.spr), "{tool_id} must be a real shared-app-config publication");
        assert_eq!(app.window_config_generation(&view).await.expect("window config generation").unwrap_or(0), 0, "{tool_id} must publish no window partition");
        config_before = after;
    }

    assert_eq!(projection_of(&app), document_before, "no Config-lane verb touches the document lane");
    close_app(&mut app);
}

/// ⚖️ LAW: the group `setPartKindWeight`/`setGripKindWeight` re-normalize always sums to 1 and
/// redistributes the remainder across the siblings in proportion to what they already held — the pure
/// core both verbs share, so the retained route's law above only has to prove the lane.
#[test]
fn kind_weight_group_normalization_keeps_the_group_summing_to_one() {
    let ids = ["a".to_string(), "b".to_string(), "c".to_string()];
    let weights: HashMap<String, f64> = ids.iter().map(|id| (id.clone(), 1.0 / 3.0)).collect();
    let normalized = puzzle5d_normalize_kind_weight_group(&weights, &ids, "a", 0.5);
    assert!((normalized["a"] - 0.5).abs() < 1e-9, "the changed kind keeps exactly what was asked for");
    assert!((normalized.values().sum::<f64>() - 1.0).abs() < 1e-9, "the group sums to 1");
    assert!((normalized["b"] - normalized["c"]).abs() < 1e-9, "equal siblings stay equal");
    let skewed: HashMap<String, f64> = [("a", 0.2), ("b", 0.6), ("c", 0.2)].into_iter().map(|(id, weight)| (id.to_string(), weight)).collect();
    let normalized = puzzle5d_normalize_kind_weight_group(&skewed, &ids, "a", 0.5);
    assert!((normalized.values().sum::<f64>() - 1.0).abs() < 1e-9);
    assert!(normalized["b"] > normalized["c"], "the remainder is redistributed in proportion to what the siblings held");
    let single = [("only".to_string(), 1.0)].into_iter().collect::<HashMap<String, f64>>();
    let normalized = puzzle5d_normalize_kind_weight_group(&single, &["only".to_string()], "only", 0.0);
    assert!((normalized["only"] - 1.0).abs() < 1e-9, "a one-kind group is always the whole distribution");
}
//#endregion 🎚️WindowAndConfigLaneVerbs
