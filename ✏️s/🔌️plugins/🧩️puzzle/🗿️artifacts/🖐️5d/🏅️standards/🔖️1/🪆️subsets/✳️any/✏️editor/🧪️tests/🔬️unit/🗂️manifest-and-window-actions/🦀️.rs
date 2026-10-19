//! 🗂️ Command envelope, clipboard, manifest and window action contract tests of the puzzle 5d editor. See [`super`].
use crate::standards::v1::subsets::any::schema::mutations::PUZZLE5D_FLAT_TO_WORLD;

use super::*;

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `Puzzle5dMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s. Deliberately
/// dispatches through a standalone typed `Puzzle5dStore` — NOT through `Puzzle5dPlayApp`/
/// `Puzzle5dPlaySnapshot` (the `🔖️ValueBridge` `serde_json::Value` wrapper this app's real
/// `ArtifactApp` still uses) — since `Puzzle5dMutation`'s canonical `Mutation<Puzzle5dSnapshot>`
/// impl (not its `Mutation<Value>` bridge impl) is what the CW7 law is about.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::host::owned::{close_puzzle5d_store,puzzle5d_store};

    use crate::{PUZZLE_5D_SCHEMA};
use crate::standards::v1::subsets::any::schema::document_records::{Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::os_store::document_materialize::create_document_envelope;

    let mut store = puzzle5d_store(create_document_envelope(PUZZLE_5D_SCHEMA, "puzzle5d", Puzzle5dSnapshot::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("store");
    let part = Puzzle5dPart { id: "p1".into(), part_kind: None, anchor: Default::default(), part_2d: Puzzle5dPart2d::default(), part_3d: Puzzle5dPart3d::default(), grips: Vec::new() };
    semio_framework_plugin::with_authoring_identity!(|identity| ::semio_framework_async::poll::resolve_ready(store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_part(part, None)], transaction: None }, &mut identity))).expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle5dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    ::semio_framework_async::poll::resolve_ready(semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle5dSnapshot, Puzzle5dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())));
    close_puzzle5d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
}
//#endregion 🔖️CommandEnvelopeTests

//#region 🔖️Clipboard
#[semio_framework_async_macros::async_test]
async fn copy_emits_clipboard_fragment_for_the_closed_selection() {
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&semio_framework_pack_json::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
    let first_part_id = first_part_id(&app);
    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &first_part_id).expect("select");
    let result = dispatch(&mut app, "copy", None, None).expect("copy");
    assert!(result.mutations.is_empty(), "copy must not record an undo entry");
    assert_eq!(result.requested_effects.len(), 1);
    let Effect::ClipboardWrite { fragment } = &result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
    assert_eq!(fragment.source_app, PUZZLE5D_PLAY_APP_ID);
    let fragment_value: serde_json::Value = serde_json::from_str(&fragment.dsl_text).expect("fragment dsl_text is JSON");
    assert_eq!(fragment_value["parts"].as_array().expect("parts").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn copy_with_no_selection_is_a_benign_no_operation() {
    let mut app = app();
    let result = dispatch(&mut app, "copy", None, None).expect("copy");
    assert!(result.mutations.is_empty());
    assert!(result.requested_effects.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn cut_removes_selected_part_and_undo_restores_it() {
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&semio_framework_pack_json::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
    let before_count = part_count(&app);
    let first_part_id = first_part_id(&app);
    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &first_part_id).expect("select");
    let result = dispatch(&mut app, "cut", None, None).expect("cut");
    assert_eq!(result.requested_effects.len(), 1, "cut must also copy to the clipboard");
    assert_eq!(part_count(&app), before_count - 1);
    let after = projection_of(&app);
    assert!(!after["parts"].as_array().unwrap().iter().any(|part| part["id"] == first_part_id));
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(part_count(&app), before_count, "one undo restores the cut part as a single edit");
}

#[semio_framework_async_macros::async_test]
async fn paste_materializes_fragment_parts_at_original_anchor_with_fresh_ids() {
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&semio_framework_pack_json::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
    let projection = projection_of(&app);
    let first_part_id = first_part_id(&app);
    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &first_part_id).expect("select");
    let copy_result = dispatch(&mut app, "copy", None, None).expect("copy");
    let Effect::ClipboardWrite { fragment } = &copy_result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
    let before_count = part_count(&app);
    let before_ids: HashSet<String> = projection["parts"].as_array().unwrap().iter().map(|part| part["id"].as_str().unwrap_or_default().to_string()).collect();
    let paste_args = semio_framework_pack_json::from_dsl_value(&semio_framework_value::DslValue::from(serde_json::json!({ "fragment": fragment, "anchor": "original", "position": [10.0, 0.0, 0.0] })));
    dispatch(&mut app, "paste", Some(&paste_args), None).expect("paste");
    assert_eq!(part_count(&app), before_count + 1);
    let after = projection_of(&app);
    let pasted_parts: Vec<&Value> = after["parts"].as_array().unwrap().iter().filter(|part| !before_ids.contains(part["id"].as_str().unwrap_or_default())).collect();
    assert_eq!(pasted_parts.len(), 1);
    // "original" anchor uses the raw position override verbatim as the 2D delta.
    let original_x = projection["parts"][0]["2d"]["x"].as_f64().unwrap_or(0.0);
    assert_eq!(pasted_parts[0]["2d"]["x"].as_f64().unwrap(), original_x + 10.0);
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(part_count(&app), before_count, "one undo removes the whole pasted fragment");
}

#[semio_framework_async_macros::async_test]
async fn paste_with_no_fragment_arg_is_a_benign_no_operation() {
    let mut app = app();
    let before_count = part_count(&app);
    let result = dispatch(&mut app, "paste", None, None).expect("paste");
    assert!(result.mutations.is_empty());
    assert_eq!(part_count(&app), before_count);
}
//#endregion 🔖️Clipboard

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn app_definition_has_the_paired_windows() {
    let definition = create_puzzle5d_app();
    let ids: Vec<&str> = definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
    assert!(ids.contains(&board2d::WINDOW_KIND_ID) && ids.contains(&world3d::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn window_kind_actions_scope_transform_to_3d_only() {
    let definition = create_puzzle5d_app();
    let resolve = |window_id: &str| -> Vec<String> {
        let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
        semio_framework_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
    };
    let board = resolve(board2d::WINDOW_KIND_ID);
    let world = resolve(world3d::WINDOW_KIND_ID);
    for transform_operation in ["translateSelection", "rotateSelection", "scaleSelection", "worldRelocate", "setCamera3d"] {
        assert!(world.contains(&transform_operation.to_string()), "3D must expose {transform_operation}");
        assert!(!board.contains(&transform_operation.to_string()), "2D must NOT expose {transform_operation}");
    }
    assert!(board.contains(&"applyBoardEvents".to_string()), "2D must expose applyBoardEvents");
    assert!(!world.contains(&"applyBoardEvents".to_string()), "3D must NOT expose applyBoardEvents");
    for shared in ["addBrushPart", "deleteSelection"] {
        assert!(board.contains(&shared.to_string()) && world.contains(&shared.to_string()), "{shared} stays on both windows");
    }
}

/// 📑️ The three declared panel tabs must survive the `panel_tab_def` stitch. Asserts PRESENCE
/// only — the framework injects tabs of its own, so a total count would be brittle.
#[semio_framework_async_macros::async_test]
async fn app_definition_declares_its_three_panel_tabs() {
    let definition = create_puzzle5d_app();
    let body_keys: Vec<&str> = definition.panel_tabs.iter().filter_map(|tab| tab.body_key.as_deref()).collect();
    for body_key in [artifact_panel::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY] {
        assert!(body_keys.contains(&body_key), "panel tab {body_key} must be declared, got {body_keys:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn window_engagements_cover_both_windows() {
    let mut app = app();
    // 🪟️ A window engagement is collected per LIVE window INSTANCE, so an empty `ViewModel` names no
    // window and the map comes back empty — the same per-instance contract `window_measures` carries.
    let engagements = ::semio_framework_async::poll::resolve_ready(app.window_engagements(&window_view(world3d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID)));
    assert!(engagements.contains_key(board2d::WINDOW_KIND_ID));
    assert!(engagements.contains_key(world3d::WINDOW_KIND_ID));
}

/// 🎯️ Every action id `dispatch_puzzle5d_action` matches on must have a `Puzzle5dCommand` variant
/// under the SAME literal — the two lists are the whole app's dispatch contract and drift between
/// them is silent. (Deliberately NOT the framework's own
/// `assert_declared_actions_bridge_to_commands`, which probes `command_from_action`, the
/// string-dispatch path this app does not implement — its commands carry an opaque `args: Value`,
/// see the `🔖️Puzzle5dCommand` macro.)
#[semio_framework_async_macros::async_test]
async fn every_dispatched_action_bridges_to_a_command() {
    for action in [
        "exportSnapshot",
        "importSnapshot",
        "openImportSnapshot",
        "openAddPartDialog",
        "setActiveExample",
        "selectSameKindSelection",
        "addNode",
        "addPartKind",
        "deleteSelection",
        "duplicateSelection",
        "setSelectionFlag",
        "patchPart",
        "patchGrip",
        "patchFastener",
        "setCamera",
        "setCamera2d",
        "setCamera3d",
        "focusSelection",
        "toggleSun",
        "setSunAzimuth",
        "setSunElevation",
        "setSunIntensity",
        "setLodMode",
        "setGridSnapEnabled",
        "setGridFactor",
        "addBrushPart",
        "cycleBrushCandidate",
        "registerBrushMesh",
        "setBrushPlacementContactTolerance",
        "setPartKindWeight",
        "setGripKindWeight",
        "engagementControlSelect",
        "setSuggestionOffset",
        "setFillCount",
        "engagementInput",
        "engagementSubmit",
        "engagementAbort",
        "translateSelection",
        "rotateSelection",
        "scaleSelection",
        "worldRelocate",
        "applyBoardEvents",
        "worldPointerDown",
        "canvasPointerDown",
    ] {
        assert_eq!(Puzzle5dCommand::from_action(action, None, None).action_id(), action, "dispatched action {action} must have a Puzzle5dCommand variant");
    }
}
//#endregion 🔖️Manifest

//#region 🧰️ Window Actions & Utilities contract
/// 🗨️ Every `partKind` option a user can pick is a NAMED kind of a shipped document, never a raw
/// catalog UUID — and the select is built without materializing `capsule-dream`.
///
/// 🐛️ `capsule-dream`'s own `kindCatalogs.parts` is empty (its catalog is a composed child), so the
/// inference fallback used to take its 2 880 parts' `part-kind` column, which holds that child's
/// UUIDs. Two costs in one: the select offered unpickable ids inside a 64-row cap, and building the
/// `AppDefinition` parsed 3 035 200 B of DSL plus ~3.5 MB of JSON on the `describe` path — which is
/// what kept `🧩️puzzle` over its 1 800 s guest epoch (`📓️pz1-catalog-zero-diagnostics.md` §2.3).
#[semio_framework_async_macros::async_test]
async fn part_kind_options_are_named_kinds_and_never_catalog_uuids() {
    let options = puzzle5d_part_kind_options();
    assert!(!options.is_empty(), "the select must offer the shipped documents' real kinds");
    assert!(options.len() <= PUZZLE5D_PART_KIND_OPTIONS_MAX);
    let is_uuid = |value: &str| {
        let bytes = value.as_bytes();
        bytes.len() == 36 && [8usize, 13, 18, 23].iter().all(|index| bytes[*index] == b'-') && bytes.iter().all(|byte| byte.is_ascii_hexdigit() || *byte == b'-')
    };
    for option in &options {
        assert!(!is_uuid(&option.value), "partKind option {:?} is a raw catalog UUID, not a kind a user can pick", option.value);
    }
}

#[semio_framework_async_macros::async_test]
async fn add_part_kind_materializes_the_declared_kind_default() {
    // 📝️ P1 arg form: addPartKind with no args materializes the declared `partKind` default and adds a part.
    // 🗨️ The default is the FIRST LIVE catalog kind (`puzzle5d_default_part_kind`), never the literal
    // "Part" this select used to hardcode — a literal no catalog declares could not add a real kind.
    let expected_default = puzzle5d_default_part_kind(&puzzle5d_part_kind_options());
    assert!(!expected_default.is_empty() && expected_default != "Part", "the declared default is a real catalog kind, got {expected_default:?}");
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&semio_framework_pack_json::json!({ "exampleId": "" })), None).expect("empty");
    let before = part_count(&app);
    let result = dispatch(&mut app, "addPartKind", None, None).expect("addPartKind");
    // 🧾️ `addPartKind` is `InteractiveJobClassification::Migrated`, so its ADMISSION carries no
    // mutations — the edit travels on the retained operation and lands in the document below. The
    // "is a Mutation" half of this sentence is the manifest declaration, which is what is read here;
    // asserting it on the admission result was reading the pre-migration dispatch shape.
    assert!(result.mutations.is_empty(), "a Migrated verb's admission carries no mutations of its own");
    let declaration = declared_actions(&create_puzzle5d_app()).into_iter().find(|action| action.id == "addPartKind").expect("addPartKind is declared").kind;
    assert_eq!(declaration, semio_framework_plugin::ActionKind::Mutation, "addPartKind is declared a Mutation");
    assert_eq!(part_count(&app), before + 1, "the materialized default kind adds exactly one part");
    let projection = projection_of(&app);
    let kind = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.last()).and_then(|part| part.get("partKind")).and_then(Value::as_str);
    assert_eq!(kind, Some(expected_default.as_str()), "the declared partKind default was materialized host-side");
}

#[semio_framework_async_macros::async_test]
async fn set_active_utility_emits_no_ops_and_no_history_entry() {
    // 🧰️ Switching utilities is the framework View action: no document operations, no undo entry, no re-emitted effect.
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&semio_framework_pack_json::json!({ "utilityId": "brush" })), None).expect("switch utility");
    assert!(result.mutations.is_empty(), "utility switching never emits document operations");
    assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
    assert_eq!(projection_of(&app), before, "utility switching does not mutate the document");
}

#[semio_framework_async_macros::async_test]
async fn exact_window_cameras_isolate_render_and_reload_without_document_or_app_config_changes() {
    let mut app = Box::new(app_with_registry());
    let mut reopened = Box::new(app_with_registry());
    let window_a = "puzzle5d-board-a";
    let window_b = "puzzle5d-board-b";
    let view_a = window_view(board2d::WINDOW_KIND_ID, window_a);
    let view_b = window_view(board2d::WINDOW_KIND_ID, window_b);
    let before = projection_of(&app);
    let app_config_before = app.config_pack().await.expect("app config before window publications");
    let result_a = dispatch(&mut app, "setCamera2d", Some(&semio_framework_pack_json::json!({ "camera": { "x": 12.5, "y": -6.5, "zoom": 3.5 } })), Some(window_a)).expect("setCamera2d a");
    let result_b = dispatch(&mut app, "setCamera2d", Some(&semio_framework_pack_json::json!({ "camera": { "x": -42.5, "y": 7.5, "zoom": 1.5 } })), Some(window_b)).expect("setCamera2d b");
    assert!(result_a.mutations.is_empty() && result_b.mutations.is_empty());
    assert_eq!(projection_of(&app), before);
    let app_config_after = app.config_pack().await.expect("app config after window publications");
    assert_eq!((app_config_after.pack, app_config_after.spr), (app_config_before.pack, app_config_before.spr));
    let board_a = render_window(&mut app, board2d::BODY_KEY, window_a);
    let board_b = render_window(&mut app, board2d::BODY_KEY, window_b);
    assert!(board_a.contains("12.5") && board_a.contains("-6.5"));
    assert!(board_b.contains("-42.5") && board_b.contains("7.5"));
    assert_eq!(app.window_config_generation(&view_a).await.expect("window a generation"), Some(1));
    assert_eq!(app.window_config_generation(&view_b).await.expect("window b generation"), Some(1));
    let packs = app.window_config_packs().await.expect("two exact window packs");
    assert_eq!(packs.len(), 2);
    for pack in packs {
        reopened.load_window_config_pack(pack).await.expect("reload exact window pack");
    }
    assert_eq!(render_window(&mut reopened, board2d::BODY_KEY, window_a), board_a);
    assert_eq!(render_window(&mut reopened, board2d::BODY_KEY, window_b), board_b);
    close_app(&mut reopened);
    close_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn exact_window_transient_isolated_abort_and_reload_reset_through_registered_app() {
    let mut app = Box::new(app_with_registry());
    let mut reopened = Box::new(app_with_registry());
    let window_a = "puzzle5d-transient-a";
    let window_b = "puzzle5d-transient-b";
    let view_a = window_view(board2d::WINDOW_KIND_ID, window_a);
    let view_b = window_view(board2d::WINDOW_KIND_ID, window_b);
    dispatch(&mut app, "engagementInput", Some(&semio_framework_pack_json::json!({ "window": board2d::WINDOW_KIND_ID, "value": "fill" })), Some(window_a)).expect("window a engagement input");
    let transient_a = app.window_transient_snapshot(&view_a).expect("window a transient").expect("window a owner");
    let transient_b = app.window_transient_snapshot(&view_b).expect("window b transient").expect("window b owner");
    assert_eq!(transient_a.get::<window_ownership::Puzzle5dBoardWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some("fill"));
    assert_eq!(transient_b.get::<window_ownership::Puzzle5dBoardWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
    dispatch(&mut app, "engagementAbort", Some(&semio_framework_pack_json::json!({ "window": board2d::WINDOW_KIND_ID })), Some(window_a)).expect("abort window a engagement");
    let aborted = app.window_transient_snapshot(&view_a).expect("aborted transient").expect("aborted owner");
    assert_eq!(aborted.get::<window_ownership::Puzzle5dBoardWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
    dispatch(&mut app, "engagementInput", Some(&semio_framework_pack_json::json!({ "window": board2d::WINDOW_KIND_ID, "value": "brush" })), Some(window_a)).expect("window a second engagement input");
    let submitted = dispatch(&mut app, "engagementSubmit", Some(&semio_framework_pack_json::json!({ "window": board2d::WINDOW_KIND_ID, "value": "brush" })), Some(window_a)).expect("submit window a engagement");
    assert!(submitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { window_id, .. } if window_id == window_a)));
    dispatch(&mut app, "engagementInput", Some(&semio_framework_pack_json::json!({ "window": board2d::WINDOW_KIND_ID, "value": "draft" })), Some(window_a)).expect("window a third engagement input");
    let reset = reopened.window_transient_snapshot(&view_a).expect("reopened transient").expect("reopened owner");
    assert_eq!(reset.get::<window_ownership::Puzzle5dBoardWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
    assert_eq!(app.window_transient_generation(&view_a).expect("window a transient generation"), Some(5));
    assert_eq!(app.window_transient_generation(&view_b).expect("window b transient generation"), Some(0));
    drop((transient_a, transient_b, aborted, reset));
    close_app(&mut reopened);
    close_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn engagements_expose_no_utility_switch_options_for_either_window() {
    // 🧰️ select/brush/fill switching lives only on the framework utility bar; neither the 2D nor the 3D
    // engagement HUD may duplicate it as options.
    let mut app = app();
    let engagements = ::semio_framework_async::poll::resolve_ready(app.window_engagements(&window_view(world3d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID)));
    for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
        assert!(engagements.get(window).expect("engagement").options.is_none(), "the {window} engagement must not re-expose utility switching as options");
    }
}

/// 🎯️ D-3 follow-up, re-based on the fill TOOL: the brush placement picker is a `"brush"`-tagged
/// `WindowMeasure::Group` in each window's `window_measures` (surfaced by `partition_window_measures` only for
/// its active utility) and the fill count is a MODE-LEVEL tool measure (`tool_measures`), never a
/// `WindowEngagementControl` on the HUD and never a window rail group — for both the 2D and 3D windows.
#[semio_framework_async_macros::async_test]
async fn fill_is_tool_options_and_brush_is_utility_options_never_engagement_controls() {
    let labels = puzzle5d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("admitted host axis");
    // 🪣️ Fill TOOL: the count entry and the distribution trees live in the tool options rail, NOT in either
    // window's measures and NOT on the engagement HUD.
    let fill_runtime = Puzzle5dRuntime { fill_count: 3, ..Default::default() };
    let fill_scene = Puzzle5dScene { document: default_document(), runtime: fill_runtime, active_utility: fill_tool::TOOL_ID.into(), interaction: Default::default() };
    let tool_measures = fill_tool::measures(&fill_scene, labels, None);
    assert!(matches!(find_measure_number(&tool_measures, "puzzle5d-fill-count"), Some(WindowMeasure::Number { value, .. }) if *value == 3.0), "the fill tool options carry the count entry");
    for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
        let measures = if window == board2d::WINDOW_KIND_ID { board2d::window_measures(&fill_scene, labels) } else { world3d::window_measures(&fill_scene, labels) };
        assert!(find_measure_number(&measures, "puzzle5d-fill-count").is_none(), "{window} must not duplicate the fill count as a window measure");
        assert_eq!(measure_group_tag(&measures, "puzzle5d-play-utility-options-fill"), None, "{window} must no longer carry a fill Utility Options rail group");
        let fill_hud = edit::puzzle5d_engagement(&fill_scene, window, labels, None);
        assert!(fill_hud.control.is_none() && fill_hud.controls.is_none(), "{window} fill engagement HUD must no longer carry the relocated control");
    }
    // 🖌️ Brush utility: with no candidates to place, the "brush"-tagged group still surfaces (matching the
    // old gate), and the engagement HUD is likewise bare.
    let brush_scene = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_utility: "brush".into(), interaction: Default::default() };
    for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
        let measures = if window == board2d::WINDOW_KIND_ID { board2d::window_measures(&brush_scene, labels) } else { world3d::window_measures(&brush_scene, labels) };
        assert_eq!(measure_group_tag(&measures, "puzzle5d-play-utility-options-brush"), Some(Some("brush".into())), "{window} brush Utility Options surfaces even without candidates");
        let brush_hud = edit::puzzle5d_engagement(&brush_scene, window, labels, None);
        assert!(brush_hud.control.is_none() && brush_hud.controls.is_none(), "{window} brush engagement HUD must no longer carry the relocated control");
    }
}

/// ♾️ The fill count is an unbounded `Number` defaulting to 100 — the deleted `PUZZLE5D_FILL_COUNT_MAX`
/// pin must not come back anywhere on the path from runtime default to rendered tool measure.
#[semio_framework_async_macros::async_test]
async fn fill_count_entry_is_unbounded_and_defaults_to_one_hundred() {
    let labels = puzzle5d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("admitted host axis");
    assert_eq!(PUZZLE5D_DEFAULT_FILL_COUNT, 100);
    assert_eq!(Puzzle5dRuntime::default().fill_count, 100);
    assert_eq!(Puzzle5dConfig::default().fill_count, 100);

    let default_scene = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_utility: fill_tool::TOOL_ID.into(), interaction: Default::default() };
    let measures = fill_tool::measures(&default_scene, labels, None);
    assert!(matches!(find_measure_number(&measures, "puzzle5d-fill-count"), Some(WindowMeasure::Number { value, .. }) if *value == 100.0));

    let large_scene = Puzzle5dScene { runtime: Puzzle5dRuntime { fill_count: 5_000, ..Default::default() }, ..default_scene };
    let measures = fill_tool::measures(&large_scene, labels, None);
    let Some(WindowMeasure::Number { value, min, max, step, .. }) = find_measure_number(&measures, "puzzle5d-fill-count") else { panic!("fill count entry") };
    assert_eq!(*value, 5_000.0);
    assert_eq!(*min, Some(0.0));
    assert_eq!(*max, None, "the fill count must carry no ceiling");
    assert_eq!(*step, Some(1.0));
}

/// 🛠️ LAW: fill is a first-class framework TOOL in the manifest — one `.tool(...)` registration carrying its
/// `ToolRunDefinition`, listed in the edit mode's tools, and NO fill utility rail left behind in either window.
#[semio_framework_async_macros::async_test]
async fn fill_is_registered_as_a_mode_tool_and_no_longer_as_a_utility() {
    let definition = create_puzzle5d_app();
    let tool = definition.tools.iter().find(|tool| tool.id == fill_tool::TOOL_ID).expect("the manifest registers a fill tool");
    assert!(tool.run.is_some(), "the fill tool carries its tool run declaration");
    assert!(!definition.utilities.iter().any(|utility| utility.id == fill_tool::TOOL_ID), "fill is no longer a utility");
    let mode = definition.modes.iter().find(|mode| mode.id == edit::PUZZLE5D_PLAY_MODE_EDIT).expect("the edit mode");
    assert!(mode.tools.iter().any(|tool| tool.as_str() == fill_tool::TOOL_ID), "the edit mode lists the fill tool");
    for window in definition.window_kinds.iter() {
        assert!(!window.utilities.iter().any(|utility| utility.as_str() == fill_tool::TOOL_ID), "{} must not bind fill as a utility", window.id);
    }
}

/// ⌨️ LAW: the engagement placeholder advertises EXACTLY the verbs `engagement_submit` parses — an advertised
/// verb no arm implements is a dead promise (the defect 3d still carries with `pick`/`rectangle`/`lasso`).
#[semio_framework_async_macros::async_test]
async fn engagement_placeholder_advertises_exactly_the_parsed_verbs() {
    let labels = puzzle5d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("admitted host axis");
    let scene = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_utility: "select".into(), interaction: Default::default() };
    for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
        let hud = edit::puzzle5d_engagement(&scene, window, labels, None);
        let placeholder = hud.input.as_ref().expect("engagement input").placeholder.clone().expect("placeholder");
        assert_eq!(placeholder, crate::editor::puzzle5d::commands::engagement_submit::PUZZLE5D_ENGAGEMENT_VERBS.join(", "), "{window} placeholder is derived from the parsed verb list");
        assert!(hud.input.as_ref().and_then(|input| input.on_repeat_last.clone()).is_some(), "{window} offers repeat-last");
    }
}

#[semio_framework_async_macros::async_test]
async fn engagement_submit_switches_utility_via_host_effect_for_both_windows() {
    // 🧰️ The engagement token drives the host-owned utility switch of the window the line was typed in,
    // and of that window ALONE. The active utility is per window INSTANCE in the host (a split pane keeps
    // its own), so pushing `brush` into the sibling pane as well would silently retool a window nobody
    // addressed. This law used to demand both windows, which is the pre-per-window contract.
    for window in [world3d::WINDOW_KIND_ID, board2d::WINDOW_KIND_ID] {
        let mut app = app();
        let result = dispatch(&mut app, "engagementSubmit", Some(&semio_framework_pack_json::json!({ "window": window, "value": "brush" })), Some(window)).expect("submit");
        let windows: Vec<&str> = result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::SetActiveUtility { window_id, utility_id } if utility_id == "brush" => Some(window_id.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(windows, vec![window], "the brush switch is pushed to the addressed window and to no other, got {windows:?}");
    }
}

/// 🧾️ The applied history rows one dispatch upserted that carry document ops.
fn edit_rows(result: &semio_framework_plugin::InvocationResult) -> Vec<semio_framework::kernel::HistoryEntry> {
    result.history_patch.iter().flat_map(|patch| patch.upserts.iter()).filter(|entry| entry.applied && !entry.op_lines.is_empty()).cloned().collect()
}

fn english(entry: &semio_framework::kernel::HistoryEntry) -> String {
    entry.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string()
}

fn german(entry: &semio_framework::kernel::HistoryEntry) -> String {
    entry.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De).to_string()
}

/// 📍️ One part's `(flat x, flat y, world origin)`.
fn part_poses(app: &Puzzle5dTestApp, id: &str) -> (f64, f64, [f64; 3]) {
    let projection = projection_of(app);
    let row = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.iter().find(|part| part.get("id").and_then(Value::as_str) == Some(id))).expect("part row").clone();
    let flat = |key: &str| row.get("2d").and_then(|flat| flat.get(key)).and_then(Value::as_f64).unwrap_or_default();
    (flat("x"), flat("y"), row.get("3d").and_then(|part| part.get("origin")).and_then(puzzle5d_value_as_f64_3).unwrap_or_default())
}

/// 🛠️ One gumball drag — the ONE pose delta the host dispatches on release — is ONE tool transaction: one edit,
/// one history row stamped with the transform tool's `TransactionRef`, whose op is the parametric
/// `drag-selection3d` over the gesture's literal ids and offset, labelled from the leaf in English and German.
/// One undo restores both poses exactly.
#[semio_framework_async_macros::async_test]
async fn one_gumball_translate_is_one_edit_one_row_and_one_transaction() {
    let mut app = Box::new(app_with_registry());
    let part_id = seeded_parts(&mut app, 1).remove(0);
    let start = part_poses(&app, &part_id);
    let result = dispatch(&mut app, "translateSelection", Some(&semio_framework_pack_json::json!({ "ids": [part_id.as_str()], "dx": 6.0, "dy": 0.0, "dz": 0.0 })), None).expect("the release delta commits");
    let rows = edit_rows(&result);
    assert_eq!(rows.len(), 1, "one gesture, one history row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.puzzle.puzzle5d@1/*#editor#translateSelection", "the transform tool authored it: {transaction:?}");
    assert!(rows[0].op_lines.len() == 1 && rows[0].op_lines[0].starts_with("drag-selection3d"), "the one op is the parametric leaf: {:?}", rows[0].op_lines);
    assert_eq!((english(&rows[0]), german(&rows[0])), ("Drag 1 item by (6, 0, 0)".to_string(), "1 Element um (6; 0; 0) ziehen".to_string()), "the row is labelled from the leaf");
    let moved = part_poses(&app, &part_id);
    assert!((moved.2[0] - start.2[0] - 6.0).abs() < 1e-9 && (moved.0 - start.0 - 6.0 / PUZZLE5D_FLAT_TO_WORLD).abs() < 1e-9, "the offset lands verbatim on the world origin and the board pin follows");
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(part_poses(&app, &part_id), start, "one undo restores the whole gesture");
    close_app(&mut app);
}

/// 🧮️ Two gestures are two transactions — two rows with distinct refs — and one undo takes back only the second:
/// gestures never coalesce into one another.
#[semio_framework_async_macros::async_test]
async fn two_gumball_gestures_are_two_transactions() {
    let mut app = Box::new(app_with_registry());
    let part_id = seeded_parts(&mut app, 1).remove(0);
    let start = part_poses(&app, &part_id);
    let first = dispatch(&mut app, "translateSelection", Some(&semio_framework_pack_json::json!({ "ids": [part_id.as_str()], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), None).expect("first gesture");
    let second = dispatch(&mut app, "rotateSelection", Some(&semio_framework_pack_json::json!({ "ids": [part_id.as_str()], "ax": 0.0, "ay": 0.0, "az": 1.0, "angle": 0.5 })), None).expect("second gesture");
    let (first, second) = (edit_rows(&first), edit_rows(&second));
    assert_eq!((first.len(), second.len()), (1, 1), "each gesture is its own row");
    assert!(second[0].op_lines[0].starts_with("rotate-selection3d"), "the turn is the parametric rotate leaf: {:?}", second[0].op_lines);
    assert_ne!(first[0].transaction.as_ref().map(|transaction| transaction.id.clone()), second[0].transaction.as_ref().map(|transaction| transaction.id.clone()), "two gestures mint two transactions");
    dispatch(&mut app, "undo", None, None).expect("undo");
    let undone = part_poses(&app, &part_id);
    assert!((undone.2[0] - start.2[0] - 1.0).abs() < 1e-9, "one undo takes back only the second gesture");
    close_app(&mut app);
}

/// 🎲️ A board drag is ONE tool transaction of `drag-selection2d` leaves: the flat pin moves, the world origin
/// stays — a plan edit — and the row is labelled as a board drag.
#[semio_framework_async_macros::async_test]
async fn a_board_drag_is_one_board_leaf_in_one_transaction() {
    let mut app = Box::new(app_with_registry());
    let part_id = seeded_parts(&mut app, 1).remove(0);
    let start = part_poses(&app, &part_id);
    let events = format!("[{{\"name\":\"gesture\",\"payload\":{{\"kind\":\"drag\",\"dx\":5,\"dy\":-2.5,\"targets\":[\"{part_id}\",\"{part_id}\"]}}}}]");
    let result = dispatch(&mut app, "applyBoardEvents", Some(&semio_framework_pack_json::json!({ "windowId": board2d::WINDOW_KIND_ID, "eventsJson": events })), Some(board2d::WINDOW_KIND_ID)).expect("applyBoardEvents drag");
    let rows = edit_rows(&result);
    assert_eq!(rows.len(), 1, "one batch, one row: {rows:?}");
    assert!(rows[0].op_lines.len() == 1 && rows[0].op_lines[0].starts_with("drag-selection2d"), "the one op is the board leaf: {:?}", rows[0].op_lines);
    assert_eq!(rows[0].transaction.as_ref().map(|transaction| transaction.tool.as_str()), Some("s.puzzle.puzzle5d@1/*#editor#applyBoardEvents"), "the transform tool authored it");
    assert_eq!((english(&rows[0]), german(&rows[0])), ("Drag 1 item by (5, -2.5) on the board".to_string(), "1 Element auf dem Brett um (5; -2,5) ziehen".to_string()));
    let moved = part_poses(&app, &part_id);
    assert_eq!((moved.0, moved.1, moved.2), (start.0 + 5.0, start.1 - 2.5, start.2), "the board drag moves the flat pin only");
    close_app(&mut app);
}

/// 🩹️ An inspector stepper nudge of the board `x` is a gesture: ONE transaction of `drag-selection2d`, while an
/// absolute `value` stays an absolute edit.
#[semio_framework_async_macros::async_test]
async fn an_inspector_nudge_is_one_relative_transaction() {
    let mut app = Box::new(app_with_registry());
    let part_id = seeded_parts(&mut app, 1).remove(0);
    let start = part_poses(&app, &part_id);
    let result = dispatch(&mut app, "patchPart", Some(&semio_framework_pack_json::json!({ "partId": part_id.as_str(), "field": "x", "delta": 4.0 })), None).expect("patchPart x nudge");
    let rows = edit_rows(&result);
    assert!(rows.len() == 1 && rows[0].op_lines[0].starts_with("drag-selection2d") && rows[0].transaction.as_ref().is_some_and(|transaction| transaction.tool.ends_with("#patchPart")), "one nudge, one transaction: {rows:?}");
    assert_eq!(part_poses(&app, &part_id).0, start.0 + 4.0, "the nudge lands relative to the committed pin");
    let absolute = dispatch(&mut app, "patchPart", Some(&semio_framework_pack_json::json!({ "partId": part_id.as_str(), "field": "x", "value": 42.0 })), None).expect("patchPart x value");
    let absolute = edit_rows(&absolute);
    assert!(absolute.len() == 1 && !absolute[0].op_lines[0].starts_with("drag-selection"), "an absolute edit is no gesture: {absolute:?}");
    assert_eq!(part_poses(&app, &part_id).0, 42.0, "the absolute edit lands verbatim");
    close_app(&mut app);
}

/// 🧯️ A motionless release — the drag that went nowhere — leaves zero trace: no edit, no row, no notice.
#[semio_framework_async_macros::async_test]
async fn a_motionless_gumball_release_leaves_zero_trace() {
    let mut app = Box::new(app_with_registry());
    let part_id = seeded_parts(&mut app, 1).remove(0);
    let result = dispatch(&mut app, "translateSelection", Some(&semio_framework_pack_json::json!({ "ids": [part_id.as_str()], "dx": 0.0, "dy": 0.0, "dz": 0.0 })), None).expect("a motionless release completes");
    let noticed = result.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. }));
    assert!(edit_rows(&result).is_empty() && result.mutations.is_empty() && !noticed, "nothing moved, nothing recorded");
    close_app(&mut app);
}
//#endregion 🧰️ Window Actions & Utilities contract
