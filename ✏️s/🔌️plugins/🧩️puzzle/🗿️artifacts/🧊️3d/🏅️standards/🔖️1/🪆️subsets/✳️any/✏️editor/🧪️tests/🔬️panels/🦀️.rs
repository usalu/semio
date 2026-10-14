//! 🪟️ Puzzle3d editor window laws — window options, fill, distribution, UI scope, utilities, world selection, gumball, history references, kit in-port, convergence and close.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_puzzle_3d_editor::*;
use semio_s_artifact_puzzle_3d_retained_action::{PUZZLE3D_RETAINED_PAYLOAD_SCHEMA, Puzzle3dAcceptSuggestionWork, Puzzle3dAddBrushObjectWork, Puzzle3dFocusSelectionWork, Puzzle3dInstanceBinding, Puzzle3dPrecomputeCommandWork, Puzzle3dTransformWork, Puzzle3dWindowCommandWork, puzzle3d_retained_extent, puzzle3d_retained_reduce};
use {semio_s_artifact_puzzle_3d_retained_document::Puzzle3dAddObjectKindWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dCreateAttractionWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dKindWeightWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dPatchInspectorWork, semio_s_artifact_puzzle_3d_retained_document::Puzzle3dSetActiveExampleWork};
use semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_handle_action;
use {semio_s_artifact_puzzle_3d_editor_session::PUZZLE3D_INTERACTION_TOPOLOGY_BUILDS, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dInstanceOperationOwner, semio_s_artifact_puzzle_3d_editor_session::with_puzzle3d_session_for, semio_s_artifact_puzzle_3d_editor_session::puzzle3d_view_session_key};
use {semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_DEFAULT_UTILITY, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_ATTRACTION, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_KIND, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_OBJECT, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_REFERENCE, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_TARGET_VOLUME, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_VORTEX, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_HOVER_CHANNEL, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_INTERACTION_DOMAIN, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dInteractionSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dScene, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneSnapshot, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dScopeClass, semio_s_artifact_puzzle_3d_editor_snapshot::default_scene_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_addressed_window_id, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_copy_fragment, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_copy_fragment_from, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_entity_label, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_fault_notices, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_flag_value_action, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_flag_value_argument, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_inspector_origin_nudge, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_kind_ids, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_paste_operations, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_active_utility, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_document, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_play_snapshot, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_selected_objects_from, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_upsert_catalog_rows, semio_s_artifact_puzzle_3d_editor_snapshot::scene_from_snapshot};
use semio_s_artifact_puzzle_3d_commands_document::world_relocate;
use {semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfig, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigMutation, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dRuntime};
use semio_s_artifact_puzzle_3d_editor_modes as edit;
use semio_s_artifact_puzzle_3d_editor_modes::tools::fill as fill_tool;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main::utilities;
use semio_s_artifact_puzzle_3d_editor_panels::{catalogue, artifact, inspection, settings as settings_panel};
use semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsOwner;
use semio_s_artifact_puzzle_3d_editor_presence as presence;
use semio_s_artifact_puzzle_3d_editor_presence::{Puzzle3dPresence, Puzzle3dPresenceMutation};
use semio_s_artifact_puzzle_3d_editor_terminology::{puzzle3d_labels, puzzle3d_localized, puzzle3d_localized_phrase, Puzzle3dLabels};
use semio_s_artifact_puzzle_3d_editor_window as window_ownership;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;
use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, Effect, PastePlacement};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, StepContext};
use semio_framework_plugin::panel_tab_element_id;
use semio_framework_plugin::panel_tab_first_draggable_element_id;
use semio_framework_plugin::window_element_id;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ActionRef;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DialogDefinition;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::ArtifactReservedJob;
use semio_framework_plugin::ArtifactReservedToolInput;
use semio_framework_plugin::ArtifactReservedToolJob;
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionVerb;
use semio_framework_plugin::InteractionWrite;
use semio_framework_plugin::IntroductionDefinition;
use semio_framework_plugin::IntroductionInteraction;
use semio_framework_plugin::IntroductionPlacement;
use semio_framework_plugin::IntroductionStepDefinition;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPortDirection;
use semio_framework_plugin::MediaPortSpec;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PortMultiplicity;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactory;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_2d::compute::EngineHandles;
use semio_framework_value::FromValue;
use {semio_framework_pack_json::json, semio_framework_pack_json::object, semio_framework_pack_json::parse, semio_framework_pack_json::to_json_string, semio_framework_pack_json::to_string, semio_framework_pack_json::Value};
use semio_framework_plugin::app::{EphemeralEmit, InteractionView};
use std::collections::{HashMap, HashSet};
use std::collections::{BTreeMap};
use semio_s_artifact_puzzle_3d_commands_document::delete_selection;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;

#[path = "../🧰️context/🦀️.rs"]
pub mod context;
use context::*;
use semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID;
use semio_s_artifact_puzzle_3d_editor_config::Puzzle3dCamera;

//#region 🔖️WindowOptions
#[semio_framework_async_macros::async_test]
async fn grid_window_options_control_one_visible_grid_spacing() {
    let mut app = app().await;
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), None).await.expect("setGridVisible");
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 7.5 })), None).await.expect("setGridSpacing");
    let lod = lod_of(&render_composite(&mut app).await);
    assert_eq!(lod.get("showLodGrid").and_then(Value::as_bool), Some(false));
    assert_eq!(lod.get("gridFactor").and_then(Value::as_f64), Some(7.5));
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(measure_group_tag(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid")), Some(None));
    assert_eq!(find_measure_slider(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing")), Some(7.5));
}

/// 🪟️ Two window instances of the same kind (a split top/perspective pane pair) must never share
/// window options — toggling grid visibility in one instance must leave every other instance's
/// grid untouched, both in its measures chrome and in its own rendered scene.
#[semio_framework_async_macros::async_test]
async fn window_options_are_local_to_the_window_instance_not_shared_across_split_panes() {
    let mut reopened = app().await;
    let mut app = app().await;
    let second_window = "puzzle3d-main-2";
    let toggle_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible");
    let base_view = app.window_view(main::WINDOW_KIND_ID);
    let second_view = app.window_view(second_window);
    let document_before = projection_of(&app);
    let app_config_before = app.config_pack().await.expect("app config before exact window publications");

    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 2.5 })), Some(main::WINDOW_KIND_ID)).await.expect("setGridSpacing on base window");
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(second_window)).await.expect("setGridVisible on second window");

    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_after = app.window_measures(&view).await;
    assert_eq!(find_measure_toggle(measures_after.get(main::WINDOW_KIND_ID).expect("base measures"), &toggle_id), Some(true), "the base window instance's grid must stay visible");
    assert_eq!(find_measure_toggle(measures_after.get(second_window).expect("second measures"), &toggle_id), Some(false), "only the targeted window instance's grid toggles off");
    let base_render = render_window(&mut app, main::WINDOW_KIND_ID).await;
    let second_render = render_window(&mut app, second_window).await;
    assert_eq!(lod_of(&base_render).get("showLodGrid").and_then(Value::as_bool), Some(true));
    assert_eq!(lod_of(&base_render).get("gridFactor").and_then(Value::as_f64), Some(2.5));
    assert_eq!(lod_of(&second_render).get("showLodGrid").and_then(Value::as_bool), Some(false));
    assert_eq!(projection_of(&app), document_before);
    let app_config_after = app.config_pack().await.expect("app config after exact window publications");
    assert_eq!((app_config_after.pack, app_config_after.spr), (app_config_before.pack, app_config_before.spr));
    assert_eq!(app.window_config_generation(&base_view).await.expect("base generation"), Some(1));
    assert_eq!(app.window_config_generation(&second_view).await.expect("second generation"), Some(1));
    let packs = app.window_config_packs().await.expect("exact window packs");
    assert_eq!(packs.len(), 2);
    for pack in packs {
        reopened.load_window_config_pack(pack).await.expect("reload exact window pack");
    }
    assert_eq!(render_window(&mut reopened, main::WINDOW_KIND_ID).await, base_render);
    assert_eq!(render_window(&mut reopened, second_window).await, second_render);
    assert!(close_witness(reopened).expect("reopened app close"));
    assert!(close_witness(app).expect("source app close"));
}

/// 📏 Wave B12: grid / spacing / LOD / vortex-show must publish into Puzzle3dWindowConfig and the
/// measures rail must read the new values. A silent `.ok().into_iter()` drop would leave generation
/// unchanged and the rail bound to the previous snapshot.
#[semio_framework_async_macros::async_test]
async fn window_option_rail_round_trips_grid_lod_and_vortex_into_published_config() {
    let mut app = app().await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let before = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("setGridVisible");
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 25.0 })), Some(main::WINDOW_KIND_ID)).await.expect("setGridSpacing");
    dispatch(&mut app, "setLodAutomatic", Some(&json!({ "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("setLodAutomatic");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(main::WINDOW_KIND_ID)).await.expect("setVortexShow");
    assert!(!app.has_pending_typed_operations(), "window-option gestures must quiesce");
    let after = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    assert_eq!(after, before + 4, "each window-option change must publish a WindowConfig generation, never drop");
    let measures = app.window_measures(&view).await;
    let rail = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_toggle(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible")), Some(false));
    assert_eq!(find_measure_slider(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing")), Some(25.0));
    assert_eq!(find_measure_toggle(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-auto")), Some(false));
    assert_eq!(find_measure_select(rail, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_ALWAYS));
}

/// 🪟️ The whole per-window option round trip, on the two instances the default layout actually opens
/// (`puzzle3d-main-top` / `puzzle3d-main-perspective`), driven exactly as the measures rail drives it:
/// the rail's own `WindowMeasure::Toggle` args (`{"pressed": …}`) tagged with the instance the rail
/// belongs to. One toggle must (a) advance THAT instance's window-config generation by exactly one,
/// (b) come back through the same instance's measures rail as the new value, (c) reach the world body
/// the instance renders, and (d) leave the sibling instance's rail, body and generation untouched.
///
/// 🕹️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12: the browser symptom was a checkbox that never
/// moved. Every hop below is green, which is what pinned the defect to the two host-side halves — the
/// control rendering the published value with no state of its own while the round trip is in flight,
/// and `Puzzle3dScopeClass::Chrome` making that round trip a full-shell repaint.
#[semio_framework_async_macros::async_test]
async fn window_option_toggle_round_trips_on_its_own_instance_and_leaves_the_sibling_alone() {
    let mut app = app().await;
    let toggle_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible");
    let top = app.window_view(main::WINDOW_INSTANCE_TOP);
    let perspective = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": true })), Some(main::WINDOW_INSTANCE_TOP)).await.expect("register the top pane's own config lane");
    let top_generation_before = app.window_config_generation(&top).await.expect("top generation");

    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("setGridVisible on the perspective pane");

    let perspective_generation = app.window_config_generation(&perspective).await.expect("perspective generation");
    assert_eq!(perspective_generation, Some(1), "one rail toggle publishes exactly one generation on the instance it was dispatched from");
    assert_eq!(app.window_config_generation(&top).await.expect("top generation"), top_generation_before, "the sibling instance's own config lane must not advance");

    let view = app.window_view(main::WINDOW_INSTANCE_PERSPECTIVE);
    let measures = app.window_measures(&view).await;
    assert_eq!(find_measure_toggle(measures.get(main::WINDOW_INSTANCE_PERSPECTIVE).expect("perspective measures"), &toggle_id), Some(false), "the rail re-reads the value its own toggle just published");
    assert_eq!(find_measure_toggle(measures.get(main::WINDOW_INSTANCE_TOP).expect("top measures"), &toggle_id), Some(true), "the sibling rail keeps the value IT published");
    assert_eq!(lod_of(&render_window(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await).get("showLodGrid").and_then(Value::as_bool), Some(false), "the world body the instance renders consumes the published option");
    assert_eq!(lod_of(&render_window(&mut app, main::WINDOW_INSTANCE_TOP).await).get("showLodGrid").and_then(Value::as_bool), Some(true), "the sibling's world body keeps its own grid");
}

/// ⚙️ The Settings panel's four steppers are addressable, carry the value they configure, and declare
/// the trigger the control they are built from actually fires.
///
/// 🔢️ `uniform` is the load-bearing half: a `NumberStepper` declared `uniform: false` renders MIXED —
/// a blank box with a placeholder instead of the value, and a +/− bump computed from the widget's own
/// `defaultValue` (0) rather than from the setting. These are one window's own scalars, never a
/// multi-selection aggregate, so the honest declaration is `true` (ticket
/// 26/09/02/PUZZLE-3D-END-TO-END wave B12, browser-measured: all four boxes rendered empty).
#[semio_framework_async_macros::async_test]
async fn settings_panel_steppers_carry_their_value_and_the_trigger_they_dispatch_on() {
    fn stepper(node: &Value, id: &str) -> Option<Value> {
        if node.get("key").and_then(Value::as_str) == Some(id) {
            return Some(node.clone());
        }
        match node {
            Value::Object(fields) => fields.iter().find_map(|(_, child)| stepper(child, id)),
            Value::Array(items) => items.iter().find_map(|item| stepper(item, id)),
            _ => None,
        }
    }
    let mut app = app().await;
    let panel: Value = from_json_str(&to_json_string(&render_body(&mut app, settings_panel::BODY_KEY).await), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the settings panel renders parseable ui json");
    for (field, action) in [
        ("contact-tolerance", "setBrushPlacementContactTolerance"),
        ("proximity-radius", "setProximityRadius"),
        ("chunk-size", "setChunkSize"),
        ("grid-spacing", "setGridSpacing"),
    ] {
        let row = stepper(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.{field}")).unwrap_or_else(|| panic!("the settings panel must declare a {field} field row: {panel}"));
        let control = stepper(&panel, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-settings.{field}.control")).unwrap_or_else(|| panic!("the {field} row must own the stepper control it labels: {row}"));
        let props = control.get("component").cloned().unwrap_or_else(|| panic!("{field} control is a number stepper: {control}"));
        assert_eq!(props.get("type").and_then(Value::as_str), Some("numberStepper"), "{field} control is a number stepper: {control}");
        assert_eq!(props.get("uniform").and_then(Value::as_bool), Some(true), "{field} configures ONE window's own scalar, so it can never render as a mixed selection: {props}");
        assert!(props.get("value").and_then(Value::as_f64).is_some(), "{field} must carry the value it configures: {props}");
        let bindings = control.get("bindings").and_then(Value::as_array).cloned().unwrap_or_default();
        assert_eq!(bindings.len(), 1, "{field} declares exactly the one trigger it handles: {bindings:?}");
        assert_eq!(bindings[0].get("trigger").and_then(Value::as_str), Some("change"), "{field} handles absolute values, so it binds Change and NOT Delta: {bindings:?}");
        assert_eq!(bindings[0].pointer("/action/name").and_then(Value::as_str), Some(action), "{field} dispatches {action}: {bindings:?}");
        assert_eq!(bindings[0].pointer("/args/windowId").and_then(Value::as_str), Some(main::WINDOW_KIND_ID), "{field} is scoped at the window the panel was rendered for, so the value it publishes reaches THAT window's rail: {bindings:?}");
    }
}

/// 🐢️ Every control the measures rail and the Settings panel bind to a per-window option resolves to
/// [`Puzzle3dScopeClass::WindowOption`], and that class paints exactly the three surfaces those options
/// can move: the world body that re-derives from them, the measures rail that renders them, and the
/// Settings panel that mirrors four of the same fields. Never the outliner, catalogue, inspector,
/// history, labels, utilities or tool measures — and never `Full`, which is what these verbs used to
/// fall through to (one grid toggle re-rendered the whole shell before its own checkbox could move).
#[semio_framework_async_macros::async_test]
async fn window_option_verbs_paint_only_their_window_their_rail_and_the_settings_panel() {
    use {semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_command_scope_class, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dScopeClass};
    for action in [
        "setGridVisible",
        "setGridSnapEnabled",
        "setGridSpacing",
        "setLodAutomatic",
        "setLodDepthVariable",
        "setLodManual",
        "setVortexShow",
        "setVortexDirection",
        "toggleSun",
        "setSunAzimuth",
        "setSunElevation",
        "setSunIntensity",
        "setSelectableKind",
        "setTransformGumballFlag",
        "setProximityRadius",
        "setChunkSize",
        "setVoxelDims",
        "setProjection",
        "setProjectionParam",
    ] {
        assert_eq!(puzzle3d_command_scope_class(action), Puzzle3dScopeClass::WindowOption, "{action} is a per-window option and must be classified as one");
    }
    let UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } = puzzle3d_scope(Puzzle3dScopeClass::WindowOption) else {
        panic!("the window-option class is a narrowed scope, never Full");
    };
    assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
    assert_eq!(panel_bodies, vec![settings_panel::BODY_KEY.to_string()], "the Settings panel mirrors four window options and must never disagree with the rail");
    assert!(measures, "the rail's own controls are bound to the value this publishes");
    assert!(!utilities && !tools && !engagements && !labels, "a window option moves no other chrome");
    for body in [inspection::BODY_KEY, artifact::BODY_KEY, catalogue::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
        assert!(!panel_bodies.iter().any(|named| named == body), "a window option cannot move the {body} panel");
    }
}

#[semio_framework_async_macros::async_test]
async fn window_transient_is_exact_instance_local_and_resets_on_reload() {
    let mut reopened = app().await;
    let mut app = app().await;
    let window_a = "puzzle3d-transient-a";
    let window_b = "puzzle3d-transient-b";
    let view_a = app.window_view(window_a);
    let view_b = app.window_view(window_b);
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 10.0, "y": 20.0 })), Some(window_a)).await.expect("open suggestions in window a");
    let transient_a = app.window_transient_snapshot(&view_a).expect("window a transient").expect("window a owner");
    let transient_b = app.window_transient_snapshot(&view_b).expect("window b transient").expect("window b owner");
    assert_eq!(transient_a.get::<window_ownership::Puzzle3dWindowTransientOwner>().and_then(|value| value.suggestion_menu.as_ref()).map(|menu| menu.window_id.as_str()), Some(window_a));
    assert!(transient_b.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    dispatch(&mut app, "closeVortexSuggestions", None, Some(window_a)).await.expect("close suggestions in window a");
    let closed = app.window_transient_snapshot(&view_a).expect("closed transient").expect("closed owner");
    assert!(closed.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    let vortex_again = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex_again })), Some(window_a)).await.expect("open suggestions in window a again");
    let reset = reopened.window_transient_snapshot(&view_a).expect("reopened transient").expect("reopened owner");
    assert!(reset.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    assert_eq!(app.window_transient_generation(&view_a).expect("window a transient generation"), Some(3));
    assert_eq!(app.window_transient_generation(&view_b).expect("window b transient generation"), Some(0));
    // 🧷️ Every `window_transient_snapshot` above is a live READ of the partition store, and close is
    // blocked — by design — while one is outstanding. The reads have to be surrendered before the
    // close witness, exactly as a host surrenders a rendered body before retiring its app.
    drop((transient_a, transient_b, closed));
    drop(reset);
    assert!(close_witness(reopened).expect("reopened app close"));
    assert!(close_witness(app).expect("source app close"));
}

/// 🧹️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave S. A mounted worker job session is admitted out of a
/// FIXED process-wide array of `semio_framework_job::WORKER_JOB_SESSION_SLOTS` retirement slots, and an
/// app dropped while an operation is still mounted parks its node there instead of freeing it. ONLY a
/// live app's cooperative-maintenance round robin hands that slot back. Without that pump the array
/// filled up for the life of the process: from then on EVERY typed operation had its worker session
/// refused, and a refused session never runs the job, so the operation reached publication with no
/// completion value and the host's continuation loop spun on it forever — which is exactly how one
/// abandoned command turned a whole test binary into `pending typed operations outlived the settle
/// budget` for every later command, including plain view actions.
#[semio_framework_async_macros::async_test]
async fn an_abandoned_app_hands_its_worker_session_admission_back_to_the_next_app() {
    let mut abandoned = app().await;
    dispatch_unsettled(&mut abandoned, "setCamera", Some(&json!({ "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0] })), None).await.expect("setCamera mounts one typed operation");
    drop(abandoned);
    let mut live = app().await;
    for _ in 0..RETIREMENT_RECLAIM_TURNS {
        if !semio_framework_job::worker_job_retirements_are_parked() {
            break;
        }
        live.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("one cooperative maintenance unit");
    }
    assert!(
        !semio_framework_job::worker_job_retirements_are_parked(),
        "a live app's maintenance round robin must reclaim every parked worker-job retirement slot, or the process runs out of session admissions"
    );
    dispatch(&mut live, "setCamera", Some(&json!({ "position": [4.0, 5.0, 6.0], "target": [0.0, 0.0, 0.0] })), None).await.expect("the next app still quiesces on a plain view action");
}

/// 🔁️ Maintenance units the reclaim law grants: every stage of the round robin, many times over, so the
/// law fails on a missing pump rather than on a tight budget.
const RETIREMENT_RECLAIM_TURNS: usize = semio_framework_plugin::MAINTENANCE_STAGES as usize * 64;

/// 🎥️ `setCamera`/`setProjection`/`setProjectionParam`/`focusSelection` moved off the document —
/// they are View-kind and must never emit VCS operations, no matter what they mutate.
#[semio_framework_async_macros::async_test]
async fn camera_actions_are_view_actions_that_emit_no_artifact_mutations() {
    let app_definition = create_puzzle3d_app();
    for action_id in ["setCamera", "setProjection", "setProjectionParam", "focusSelection"] {
        let def = dispatchable_actions(&app_definition).into_iter().find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("{action_id} declared"));
        assert_eq!(def.kind, ActionKind::View, "{action_id} must be a View action — camera is session-only, never a VCS edit");
    }
    let mut live = app().await;
    let before = projection_of(&live);
    let result = dispatch(&mut live, "setCamera", Some(&json!({ "camera": { "position": [1.0, 2.0, 3.0], "target": [4.0, 5.0, 6.0], "zoom": 2.5 } })), None).await.expect("setCamera");
    assert!(result.mutations.is_empty(), "setCamera must not emit document operations");
    assert_eq!(projection_of(&live), before, "setCamera must not mutate the document");
}

/// 🎥️ Wave B11 (checklist §22, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 14): `f` with
/// nothing selected completed with a bare `Emit::default()` — the camera never moved, no notice was
/// raised, and the key read as dead on every boot before the user had picked anything. A camera verb
/// with no selection has an obvious subject (the whole document), so focus must FRAME it — while still
/// writing only the per-window camera, never a document operation.
#[semio_framework_async_macros::async_test]
async fn focus_selection_frames_the_whole_document_when_nothing_is_selected() {
    let mut app = app().await;
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [900.0, 900.0, 900.0], "target": [900.0, 900.0, 900.0], "zoom": 1.0 } })), Some(main::WINDOW_KIND_ID)).await.expect("park the camera far away");
    let parked = camera_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    let document_before = projection_of(&app);
    let result = dispatch(&mut app, "focusSelection", None, Some(main::WINDOW_KIND_ID)).await.expect("focusSelection with an empty selection must complete");
    let framed = camera_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_ne!(framed, parked, "focus with nothing selected must frame the document instead of leaving the camera parked");
    assert!(result.mutations.is_empty(), "focus is camera-only — it must emit no artifact history entry: {:?}", result.mutations);
    assert_eq!(projection_of(&app), document_before, "focus must not touch the document");
    let objects = document_before.get("objects").and_then(Value::as_array).expect("the example carries objects");
    let first = objects.first().and_then(|object| object.get("origin")).cloned().expect("an object origin");
    assert_eq!(framed.pointer("/target/0").and_then(Value::as_f64), first.pointer("/0").and_then(Value::as_f64), "a one-object document frames that object");
}

/// 🪟️ ONE per-window setting is ONE window-config publication that QUIESCES. The lane's own bounded
/// preparation used to demand the owner's whole declared publication ceiling
/// (`Puzzle3dWindowConfigOwner::MAXIMUM_PUBLICATION_BYTES`, 65 536) out of the publisher's per-turn
/// byte grant (`TYPED_OPERATION_RESULT_PAGE_BYTES`, 4 096) and answered `Blocked` on every single
/// turn, so `setCamera` — and every projection/grid/vortex/window option behind the same lane — sat
/// in stage `Publishing` forever and nothing queued behind it could ever run. The law is exactly:
/// one mutation, one generation, no pending operation left over.
#[semio_framework_async_macros::async_test]
async fn one_window_config_mutation_publishes_exactly_one_generation_and_quiesces() {
    let mut app = app().await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let before = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [7.0, 8.0, 9.0], "target": [0.0, 0.0, 0.0], "zoom": 1.5 } })), None).await.expect("setCamera");
    assert!(!app.has_pending_typed_operations(), "one window-config gesture must leave no typed operation pending");
    let after = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    assert_eq!(after, before + 1, "one window-config mutation publishes exactly one store generation, never zero and never a spin");
    assert_eq!(camera_of(&render_window(&mut app, main::WINDOW_KIND_ID).await).pointer("/position/0").and_then(Value::as_f64), Some(7.0), "the published window config is what the window renders");
}

/// 🛰️📷️ Wave B50: `setCamera` must move the pose the REFRESH route publishes, not only the one the
/// `<body>:<windowInstanceId>` surface route does. `plugin_refresh_ui` renders a window section as
/// `app.render(body_key, None, view.for_window_instance(key))` (`🔌️plugin/🦀️.rs:33790`) — the
/// instance reaches the guest through the VIEW alone, with no instance-suffixed body key — and the
/// section hash it answers is what `uiRefreshSectionUnchanged` compares. Every pre-existing camera
/// law measured [`render_window`] (the surface route), so a refresh route that publishes the opening
/// pose forever read green while the browser froze: wasm #59's battery reported
/// `camera settle puzzle3d-main-perspective moved=false waitedMs=30184` for orbit, pan AND zoom with
/// `refreshUi sections asked=["puzzle3d-main","puzzle3d-main-top","puzzle3d-main-perspective"]
/// changed=[] hashes={…"puzzle3d-main-perspective":"aba169d7:1"}` — the same hash after every
/// `setCamera`, on a `full` scope included. Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B50.
#[semio_framework_async_macros::async_test]
async fn set_camera_moves_the_pose_the_window_refresh_route_publishes() {
    let mut app = app().await;
    let window = main::WINDOW_INSTANCE_PERSPECTIVE;
    let before = camera_of(&render_window_refresh(&mut app, main::BODY_KEY, window).await);
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [11.0, 22.0, 33.0], "target": [1.0, 2.0, 3.0], "zoom": 4.0 } })), Some(window)).await.expect("setCamera on the perspective instance");
    let after = camera_of(&render_window_refresh(&mut app, main::BODY_KEY, window).await);
    assert_ne!(after, before, "the refresh route must publish the gestured pose, not the opening one: before={before} after={after}");
    assert_eq!(after.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "the refresh route's published camera is the one `setCamera` wrote: {after}");
    let sibling = camera_of(&render_window_refresh(&mut app, main::BODY_KEY, main::WINDOW_INSTANCE_TOP).await);
    assert_ne!(sibling.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "one instance's gesture must not move its sibling's refresh pose: {sibling}");
}

/// 🪟️📷️ Orbiting one window instance's camera must never move any sibling instance's camera, and
/// must never touch the shared document.
#[semio_framework_async_macros::async_test]
async fn set_camera_is_per_window_and_leaves_sibling_windows_and_the_document_untouched() {
    let mut app = app().await;
    let window_a = "puzzle3d-main-a";
    let window_b = "puzzle3d-main-b";
    dispatch(&mut app, "worldPointerDown", None, Some(window_a)).await.expect("register a");
    dispatch(&mut app, "worldPointerDown", None, Some(window_b)).await.expect("register b");

    let before_document = projection_of(&app);
    let camera_b_before = camera_of(&render_window(&mut app, window_b).await);

    let result = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [11.0, 22.0, 33.0], "target": [1.0, 2.0, 3.0], "zoom": 4.0 } })), Some(window_a)).await.expect("setCamera on window A");
    assert!(result.mutations.is_empty(), "setCamera must not emit document operations");
    assert_eq!(projection_of(&app), before_document, "setCamera must never mutate the shared document");

    let camera_a_after = camera_of(&render_window(&mut app, window_a).await);
    assert_eq!(camera_a_after.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "window A's own rendered camera picks up the new pose");
    assert_eq!(camera_of(&render_window(&mut app, window_b).await), camera_b_before, "window B's rendered camera must be unaffected by window A's setCamera");
}

#[semio_framework_async_macros::async_test]
async fn vortex_show_window_option_defaults_to_selected_and_switches_to_always() {
    let mut app = app().await;
    let all_vortex_ids = vortex_full_ids(&app);
    assert!(!all_vortex_ids.is_empty(), "scene_snapshot must expose vortices");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_SELECTED));

    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "Selected mode must hide vortices while idle");

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow always");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_always = app.window_measures(&view).await;
    let window_measures_always = measures_always.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures_always, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_ALWAYS));
    assert_eq!(vortices_of(&render_composite(&mut app).await).len(), all_vortex_ids.len(), "Always mode must emit every vortex while idle");

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_SELECTED })), None).await.expect("setVortexShow selected");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "switching back to Selected must hide idle vortices");
}

#[semio_framework_async_macros::async_test]
async fn vortex_direction_window_option_defaults_to_outwards_and_switches_to_inwards() {
    let mut app = app().await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction")).as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS));

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow always");
    let outwards_vortices = vortices_of(&render_composite(&mut app).await);
    assert!(!outwards_vortices.is_empty(), "scene_snapshot must expose vortices");
    assert!(outwards_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

    dispatch(&mut app, "setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), None).await.expect("setVortexDirection inwards");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_inwards = app.window_measures(&view).await;
    let window_measures_inwards = measures_inwards.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures_inwards, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction")).as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS));
    assert!(vortices_of(&render_composite(&mut app).await).iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
}

#[semio_framework_async_macros::async_test]
async fn vortex_direction_option_is_local_to_the_window_instance() {
    let mut app = app().await;
    let second_window = "puzzle3d-main-2";
    dispatch(&mut app, "worldPointerDown", None, Some(main::WINDOW_KIND_ID)).await.expect("register base window");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(main::WINDOW_KIND_ID)).await.expect("setVortexShow always on base");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(second_window)).await.expect("setVortexShow always on second");
    dispatch(&mut app, "setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), Some(second_window)).await.expect("setVortexDirection inwards on second window");

    let base_vortices = vortices_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert!(!base_vortices.is_empty(), "the base window must still emit vortices");
    assert!(base_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

    let second_vortices = vortices_of(&render_window(&mut app, second_window).await);
    assert!(second_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
}
//#endregion 🔖️WindowOptions

//#region 🔖️Fill
/// 🛠️ Wave B14: Fill is a mode-level tool. A leftover/window `select` utility must not take the world lane —
/// the guest world lane publishes `fill` while `active_tool_id` is fill.
#[test]
fn fill_tool_wins_the_world_lane_over_a_select_window_utility() {
    let mut runtime = Puzzle3dRuntime::default();
    runtime.active_tool_id = Some(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID.into());
    let mut view = semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    view.active_utility_id = Some("select".into());
    view.active_utility_by_window_id.insert("pane".into(), "select".into());
    assert!(puzzle3d_fill_tool_active(&runtime), "the Fill tab arms the mode tool, not a window utility");
    assert_eq!(puzzle3d_scene_active_utility(&runtime, Some(&view), Some("pane")), semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID, "guest world lane must stay fill while the tool is armed");
    runtime.active_tool_id = None;
    assert!(!puzzle3d_fill_tool_active(&runtime));
    assert_eq!(puzzle3d_scene_active_utility(&runtime, Some(&view), Some("pane")), "select");
}

/// ⏯️ Fill is a framework tool run declared through the manifest (`📋️tool-run-contract.md` §2.4): a mutating
/// `instance3d` run with revalidate rebase and resume reconfigure, run and revalidate jobs, and one reason per
/// schema reason spelling that reason's own code, id and verdict. The framework injects the seven run actions;
/// no plugin tick or cancel verb survives.
#[semio_framework_async_macros::async_test]
async fn the_fill_tool_declares_its_tool_run_through_the_manifest() {
    use semio_s_artifact_puzzle_3d_model::FillRunReason;
    use semio_framework_tool_run::{ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunTraceKind, TOOL_RUN_ACTION_IDS};
    let definition = create_puzzle3d_app();
    let fill = definition.tools.iter().find(|tool| tool.id == semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID).expect("fill tool");
    let run = fill.run.as_ref().expect("the fill tool declares its ToolRunDefinition");
    run.validate().expect("the fill run definition validates");
    assert!(run.mutating, "fill writes the document, so its run is transactional");
    assert_eq!((run.rebase, run.reconfigure, run.trace), (ToolRunRebasePolicy::Revalidate, ToolRunReconfigurePolicy::Resume, ToolRunTraceKind::Instance3d));
    assert_eq!((run.run_job.as_str(), run.revalidate_job.as_ref().map(|job| job.as_str())), (fill_tool::RUN_JOB_KIND, Some(fill_tool::REVALIDATE_JOB_KIND)));
    assert_eq!(run.reasons.len(), FillRunReason::ALL.len());
    for (reason, declared) in FillRunReason::ALL.iter().zip(&run.reasons) {
        assert_eq!((declared.code, declared.id.as_str(), declared.verdict), (reason.code(), reason.id(), reason.verdict()), "{} spells the schema's own code, id and verdict", reason.id());
    }
    let actions: Vec<&str> = dispatchable_actions(&definition).into_iter().map(|action| action.id.as_str()).collect();
    for retired in ["fillBuildTick", "cancelFillBuild"] {
        assert!(!actions.contains(&retired), "{retired} is a per-plugin run verb the framework tool run replaces");
    }
    for id in TOOL_RUN_ACTION_IDS {
        assert!(actions.contains(&id), "declaring a run injects {id}");
    }
}

/// ⏯️ Red→green successor of the lock-is-commit tick law (`📋️tool-run-contract.md` §5 W1-F): a fill run holds
/// every placement provisionally in the instance-owned tool run ledger — the committed document never moves
/// while it runs and the viewport renders committed ⊕ provisional — and finalize publishes ONE edit carrying
/// every provisional placement, which one undo removes and one redo restores. No process-global run state
/// exists, so the law holds at any test thread count.
#[semio_framework_async_macros::async_test]
async fn fill_run_finalize_publishes_one_edit_with_every_provisional_placement() {
    use semio_framework_tool_run::TOOL_RUN_FINALIZE_ACTION_ID;
    let mut app = app().await;
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 3 })), None).await.expect("setFillCount");
    let committed = committed_document(&app);
    let objects = object_count(&app);
    assert_eq!(start_fill_run(&mut app).await["toolRun"], json!("spawnJob"));
    pump_fill_run(&mut app, "the run completes", |presence| presence.state == protocol::PresenceToolRunState::Complete).await;
    let placed = fill_run_presence(&app).completed as usize;
    assert_eq!(placed, 3, "the run places exactly what was requested");
    assert_eq!(committed_document(&app), committed, "a complete run has committed nothing yet");
    assert_eq!(rendered_object_count(&mut app).await, objects + placed, "the viewport renders committed ⊕ provisional");
    let provisional_instances = |node: &Value| instances_of(node).iter().filter(|instance| instance["provisional"] == json!(true)).count();
    assert_eq!(provisional_instances(&render_composite(&mut app).await), placed, "exactly the provisional placements carry the provisional instance style");
    let identity = fill_run_identity(&mut app).await.expect("a complete run is live until it is finalized");
    assert_eq!(tool_run_action(&mut app, TOOL_RUN_FINALIZE_ACTION_ID, identity).await["toolRun"], json!("beginFinalize"));
    pump_fill_run(&mut app, "finalize publishes", |presence| presence.state == protocol::PresenceToolRunState::Finalized).await;
    assert_eq!(object_count(&app), objects + placed, "finalize publishes every provisional placement");
    assert_eq!(rendered_object_count(&mut app).await, objects + placed, "the overlay is released onto the committed document");
    assert_eq!(provisional_instances(&render_composite(&mut app).await), 0, "finalized placements render as committed objects");
    assert!(fill_run_identity(&mut app).await.is_none(), "a finalized run is terminal, so Escape no longer aborts it");
    let finalized = committed_document(&app);
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(committed_document(&app), committed, "one undo removes every placement: start → complete → finalize is ONE undo entry");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(committed_document(&app), finalized, "one redo restores all of them");
}

/// 🛑 Red→green (`📋️tool-run-contract.md` §5 W1-F): the run's abort is offered from `starting` onwards with the
/// run's current identity, so it exists whenever provisional pieces are on screen. Aborting leaves zero
/// provisional placements and a committed document byte-identical to the one at start — no edit, no undo entry.
#[semio_framework_async_macros::async_test]
async fn cancelling_a_stepping_fill_job_never_panics_and_reports_a_cancelled_run() {
    use semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID;
    let mut app = app().await;
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 40 })), None).await.expect("setFillCount");
    let committed = committed_document(&app);
    let objects = object_count(&app);
    start_fill_run(&mut app).await;
    assert!(fill_run_identity(&mut app).await.is_some(), "abort is enabled while the run is still starting");
    pump_fill_run(&mut app, "a provisional placement", |presence| presence.completed > 0).await;
    assert!(rendered_object_count(&mut app).await > objects, "provisional pieces are on screen before the abort");
    let identity = fill_run_identity(&mut app).await.expect("abort carries the run's current identity in every non-terminal state");
    assert_eq!(tool_run_action(&mut app, TOOL_RUN_ABORT_ACTION_ID, identity).await["toolRun"], json!("closeJob"));
    pump_fill_run(&mut app, "the abort settles", |presence| presence.state == protocol::PresenceToolRunState::Aborted).await;
    assert_eq!(committed_document(&app), committed, "an aborted run leaves the committed document byte-identical");
    assert_eq!(rendered_object_count(&mut app).await, objects, "an aborted run holds zero provisional placements");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(committed_document(&app), committed, "and left no undo entry behind");
}

/// 🛑 Red→green (`📋️tool-run-contract.md` §5 W1-F): Escape (`engagementAbort`) maps to the framework-reserved
/// `toolRunAbort` of the run this window shows while it is non-terminal, and disarms the tool. The abort only
/// BEGINS inside that call — the run is `aborting` — and the teardown is spent in the driver's bounded close
/// turns, never inside one turn. Nothing was ever written, so no artifact lane is involved.
#[semio_framework_async_macros::async_test]
async fn engagement_abort_tears_the_fill_plan_down_across_turns_and_never_inside_one() {
    let mut app = app().await;
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 40 })), None).await.expect("setFillCount");
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID })), None).await.expect("arm fill");
    let committed = committed_document(&app);
    start_fill_run(&mut app).await;
    pump_fill_run(&mut app, "a provisional placement", |presence| presence.completed > 0).await;
    let identity = fill_run_identity(&mut app).await.expect("a live fill run");
    let run = identity[semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID].as_str().and_then(|run| run.parse().ok()).expect("run id");
    let generation = identity[semio_framework_tool_run::TOOL_RUN_ARG_GENERATION].as_u64().and_then(|generation| u32::try_from(generation).ok()).expect("generation");
    app.echo_tool_run_trace_cursor(main::WINDOW_KIND_ID, semio_framework_tool_run::ToolRunTraceCursor { run, generation, page: 1 });
    let escape = dispatch_unsettled(&mut app, "engagementAbort", None, None).await.expect("Escape");
    let settled = settle(&mut app).await;
    let effects: Vec<&Effect> = escape.requested_effects.iter().chain(&settled.effects).collect();
    let abort = effects
        .iter()
        .find_map(|effect| match effect {
            Effect::DispatchAction { action, args, .. } if action == semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID => args.clone(),
            _ => None,
        })
        .expect("Escape dispatches toolRunAbort while a fill run is non-terminal");
    assert!(effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty())), "Escape also disarms the Fill tool");
    let abort = semio_framework_pack_json::from_dsl_value(&abort);
    assert_eq!(abort[semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID], identity[semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID], "the abort names the run this window shows");
    tool_run_action(&mut app, semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID, abort).await;
    assert_eq!(fill_run_presence(&app).state, protocol::PresenceToolRunState::Aborting, "the abort only begins inside the call");
    let turns = pump_fill_run(&mut app, "the teardown", |presence| presence.state == protocol::PresenceToolRunState::Aborted).await;
    assert!(turns >= 1, "the run tears down in the driver's close turns, never inside the abort's own turn");
    assert_eq!(committed_document(&app), committed, "nothing was ever written, so nothing is rolled back");
}

/// 🎚️ Red→green (`📋️tool-run-contract.md` §5 W1-F): a fresh document asks for the default of 100, and
/// `setFillCount` is configuration only — it never emits an artifact mutation. During a run it reconfigures
/// that run (`reconfigure: resume`): the same run id continues under a newer generation.
#[semio_framework_async_macros::async_test]
async fn set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count() {
    async fn fill_count_entry(app: &mut Puzzle3dApp) -> Option<f64> {
        let view = app.window_view(main::WINDOW_KIND_ID);
        let measures = app.tool_measures(&view).await;
        find_measure_number(measures.get(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID).expect("fill tool measures"), "puzzle3d-fill-count")
    }
    let mut app = app().await;
    assert_eq!(fill_count_entry(&mut app).await, Some(100.0), "a fresh document asks for the default of 100");
    let committed = committed_document(&app);
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 3 })), None).await.expect("setFillCount through the migrated tool-job path");
    assert_eq!(fill_count_entry(&mut app).await, Some(3.0), "setFillCount updates the requested count");
    assert_eq!(committed_document(&app), committed, "the count is configuration: no artifact mutation");
    start_fill_run(&mut app).await;
    pump_fill_run(&mut app, "the first run completes", |presence| presence.state == protocol::PresenceToolRunState::Complete).await;
    let before = fill_run_identity(&mut app).await.expect("a live run");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 5 })), None).await.expect("setFillCount during a run");
    pump_fill_run(&mut app, "the reconfigured run completes", |presence| presence.state == protocol::PresenceToolRunState::Complete && presence.completed == 5).await;
    let after = fill_run_identity(&mut app).await.expect("the same run is still live");
    assert_eq!(after["runId"], before["runId"], "a count change resumes the SAME run");
    assert!(after["generation"].as_u64() > before["generation"].as_u64(), "under a newer generation");
    assert_eq!(committed_document(&app), committed, "a reconfigured run still committed nothing");
}

/// 🔁️ A count raised during a run continues the same deterministic sequence, a lowered one retracts its tail:
/// the provisional placements at 2 ⊂ at 4 ⊂ at 6, all under one run id, and finalize commits exactly the
/// lowered count.
#[semio_framework_async_macros::async_test]
async fn raising_and_lowering_the_fill_count_during_a_run_continues_and_retracts_the_same_sequence() {
    use semio_framework_tool_run::TOOL_RUN_FINALIZE_ACTION_ID;
    async fn provisional_ids(app: &mut Puzzle3dApp, committed: &std::collections::BTreeSet<String>) -> std::collections::BTreeSet<String> {
        instances_of(&render_composite(app).await).iter().filter_map(|instance| instance.get("id").and_then(Value::as_str)).filter(|id| !committed.contains(*id)).map(str::to_string).collect()
    }
    let mut app = app().await;
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 4 })), None).await.expect("setFillCount 4");
    let committed: std::collections::BTreeSet<String> = instances_of(&render_composite(&mut app).await).iter().filter_map(|instance| instance.get("id").and_then(Value::as_str).map(str::to_string)).collect();
    let objects = object_count(&app);
    start_fill_run(&mut app).await;
    pump_fill_run(&mut app, "four placements", |presence| presence.state == protocol::PresenceToolRunState::Complete).await;
    let run = fill_run_identity(&mut app).await.expect("live run")["runId"].clone();
    let four = provisional_ids(&mut app, &committed).await;
    assert_eq!(four.len(), 4);
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 6 })), None).await.expect("raise to 6");
    pump_fill_run(&mut app, "six placements", |presence| presence.state == protocol::PresenceToolRunState::Complete && presence.completed == 6).await;
    let six = provisional_ids(&mut app, &committed).await;
    assert!(four.is_subset(&six) && six.len() == 6, "a raise continues the sequence: {four:?} ⊂ {six:?}");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 2 })), None).await.expect("lower to 2");
    pump_fill_run(&mut app, "two placements", |presence| presence.state == protocol::PresenceToolRunState::Complete && presence.completed == 2).await;
    let two = provisional_ids(&mut app, &committed).await;
    assert!(two.is_subset(&four) && two.len() == 2, "a lower retracts the tail: {two:?} ⊂ {four:?}");
    let identity = fill_run_identity(&mut app).await.expect("live run");
    assert_eq!(identity["runId"], run, "raise and lower both resume the same run");
    tool_run_action(&mut app, TOOL_RUN_FINALIZE_ACTION_ID, identity).await;
    pump_fill_run(&mut app, "finalize", |presence| presence.state == protocol::PresenceToolRunState::Finalized).await;
    assert_eq!(object_count(&app), objects + 2, "finalize commits exactly the lowered count");
}

/// ♾️ Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS §1 decisions 1 and 4 — the count is any `u32`.
/// Nothing clamps a request against a planner ceiling; a document that cannot hold the ask reports a
/// stall step instead.
#[semio_framework_async_macros::async_test]
async fn the_fill_count_has_no_ceiling_and_the_entry_declares_none() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID })), None).await.expect("select fill tool");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 5_000 })), None).await.expect("setFillCount");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.tool_measures(&view).await;
    let tool_measures = measures.get(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID).expect("fill tool measures");
    assert_eq!(find_measure_number(tool_measures, "puzzle3d-fill-count"), Some(5_000.0), "a request far above anything planned is kept verbatim — no clamp");
    assert_eq!(find_measure_number_max(tool_measures, "puzzle3d-fill-count"), Some(None), "the count entry declares no maximum");
    assert_eq!(semio_s_artifact_puzzle_3d_commands_runtime::set_fill_count::parse_count(Some(&json!({ "value": 4_000_000_000u32 }))), 4_000_000_000, "parse_count carries the whole u32 range");
}

/// 🪣️ The count is shared configuration: a count set in one split pane is the count every pane shows.
#[semio_framework_async_macros::async_test]
async fn fill_count_is_shared_across_split_panes() {
    let mut app = app().await;
    let top = main::WINDOW_INSTANCE_TOP;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    dispatch(&mut app, "worldPointerDown", None, Some(perspective)).await.expect("register perspective");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 6 })), Some(top)).await.expect("setFillCount in the top pane");
    for window in [top, perspective] {
        let view = app.window_view(window);
        let measures = app.tool_measures(&view).await;
        assert_eq!(find_measure_number(measures.get(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID).expect("fill tool measures"), "puzzle3d-fill-count"), Some(6.0), "{window} shows the shared count");
    }
}

/// 🔢️ The count control is an unbounded `Number` entry showing the REQUESTED count; progress and placements
/// are the framework ToolRun panel's, so the entry carries no `ready` extent and loads only while a fill run
/// is live.
#[test]
fn fill_count_measure_is_an_unbounded_number_entry_reporting_the_requested_count() {
    let scene = Puzzle3dScene { scene_snapshot: nakagin_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID.into() };
    match fill_tool::count_measure(&scene, &Puzzle3dLabels::NATIVE_EN, None) {
        WindowMeasure::Number { label: Some(label), value, min, max, ready, loading, .. } => {
            assert_eq!(label, Puzzle3dLabels::NATIVE_EN.count.as_str());
            assert_eq!(value, scene.runtime.fill_count as f64, "the entry shows the REQUESTED count, verbatim");
            assert_eq!((min, max, ready, loading), (Some(0.0), None, None, None), "no ceiling, no locked extent, and no loading ring without a run");
        }
        other => panic!("expected a Number measure, got {other:?}"),
    }
}

/// 🔁️ A committed `setFillCount` survives the renders that follow it: the config mutation IS the count.
#[semio_framework_async_macros::async_test]
async fn a_committed_fill_count_survives_the_renders_that_follow_it() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID })), None).await.expect("select fill tool");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 250 })), None).await.expect("setFillCount");
    for _ in 0..3 {
        render_composite(&mut app).await;
    }
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.tool_measures(&view).await;
    let tool_measures = measures.get(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID).expect("fill tool measures");
    assert_eq!(find_measure_number(tool_measures, "puzzle3d-fill-count"), Some(250.0), "three renders later the requested count is still what the user asked for");
}
//#endregion 🔖️Fill

//#region 🔖️Distribution
#[semio_framework_async_macros::async_test]
async fn puzzle3d_normalize_kind_weight_group_redistributes_siblings_proportionally() {
    let ids = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let mut weights = HashMap::from([("a".to_string(), 0.2), ("b".to_string(), 0.3), ("c".to_string(), 0.5)]);
    weights = puzzle3d_normalize_kind_weight_group(&weights, &ids, "a", 0.5);
    let sum: f64 = ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum();
    assert!((sum - 1.0).abs() < 1e-9, "simplex must stay at 1, got {sum}");
    assert!((weights.get("a").copied().unwrap_or(0.0) - 0.5).abs() < 1e-9);
    // b:c were 0.3:0.5 — remainder 0.5 splits 0.3/0.8 and 0.5/0.8
    assert!((weights.get("b").copied().unwrap_or(0.0) - 0.5 * 0.3 / 0.8).abs() < 1e-9);
    assert!((weights.get("c").copied().unwrap_or(0.0) - 0.5 * 0.5 / 0.8).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_vortex_measure_exposes_joint_weight_scaled_by_object() {
    let object_ids = vec!["Object".to_string(), "Placed".to_string()];
    let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
    let mut object_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let mut joint_weights = HashMap::new();
    puzzle3d_ensure_joint_distribution(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids);
    let object_weight = puzzle3d_object_joint_total(&joint_weights, &object_weights, "Object", &vortex_ids);
    let measures = puzzle3d_joint_vortex_measures("Object", object_weight, &vortex_ids, &object_weights, &joint_weights);
    let expected_joint = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "c-b");
    match &measures[0] {
        WindowMeasure::Slider { value, max, step, disabled, .. } => {
            assert!((*value - expected_joint).abs() < 1e-9, "slider must show the joint probability, got {value}");
            assert!((*max - 1.0).abs() < 1e-9, "joint range spans the global simplex");
            assert_eq!(*step, Some(0.01));
            assert_eq!(*disabled, None);
        }
        other => panic!("expected vortex slider, got {other:?}"),
    }
    puzzle3d_set_object_kind_distribution_weight(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids, "Object", 0.8);
    let raised_weight = puzzle3d_object_joint_total(&joint_weights, &object_weights, "Object", &vortex_ids);
    let raised_measures = puzzle3d_joint_vortex_measures("Object", raised_weight, &vortex_ids, &object_weights, &joint_weights);
    match (&measures[0], &raised_measures[0]) {
        (WindowMeasure::Slider { value: before, .. }, WindowMeasure::Slider { value: after, .. }) => {
            assert!(*after > *before, "raising P(object) must raise joint vortex percentages");
            let expected_after = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "c-b");
            assert!((*after - expected_after).abs() < 1e-9);
        }
        _ => panic!("expected vortex sliders"),
    }
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_distribution_lists_global_vortices_and_joints_sum_to_one() {
    let scene_snapshot = nakagin_scene_snapshot();
    let object_ids = puzzle3d_kind_ids(&scene_snapshot, "objects");
    let vortex_ids = puzzle3d_kind_ids(&scene_snapshot, "vortices");
    assert!(object_ids.len() >= 2, "default scene_snapshot needs multiple object kinds");
    assert!(vortex_ids.len() >= 2, "default scene_snapshot needs multiple vortex kinds");
    let object_kind_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let vortex_kind_weights = puzzle3d_uniform_kind_weights(&vortex_ids);
    let scene = Puzzle3dScene { scene_snapshot, runtime: Puzzle3dRuntime { object_kind_weights, vortex_kind_weights, ..Puzzle3dRuntime::default() }, active_utility: semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID.into() };
    let distribution_children = puzzle3d_distribution_children(&scene, Some(true));
    assert_eq!(distribution_children.len(), object_ids.len());
    let mut joint_sum = 0.0;
    for measure in &distribution_children {
        let WindowMeasure::Group { children, value: Some(object_weight), .. } = measure else {
            panic!("expected object-kind group");
        };
        assert_eq!(children.len(), vortex_ids.len(), "each object must list the full global vortex catalog");
        let local_sum: f64 = children
            .iter()
            .map(|child| match child {
                WindowMeasure::Slider { value, .. } => *value,
                _ => panic!("expected vortex slider"),
            })
            .sum();
        assert!((local_sum - object_weight).abs() < 1e-6, "under one object joints sum to P(object), not 1");
        joint_sum += local_sum;
    }
    assert!((joint_sum - 1.0).abs() < 1e-6, "all nested joint percentages across objects must sum to 1, got {joint_sum}");
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_object_weight_change_scales_joint_sampling_product() {
    let object_ids = vec!["Object".to_string(), "Placed".to_string()];
    let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
    let mut object_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let mut joint_weights = HashMap::new();
    puzzle3d_ensure_joint_distribution(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids);
    let joint_before = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "c-b");
    puzzle3d_set_object_kind_distribution_weight(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids, "Object", 0.6);
    let joint_after = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "c-b");
    assert!(joint_after > joint_before);
    let global = puzzle3d_global_joint_total(&joint_weights, &object_weights, &object_ids, &vortex_ids);
    assert!((global - 1.0).abs() < 1e-6, "global joints must stay normalized, got {global}");
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_vortex_joint_edit_redistributes_siblings_and_preserves_object_total() {
    let object_ids = vec!["Object".to_string(), "Placed".to_string()];
    let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
    let mut object_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let mut joint_weights = HashMap::new();
    puzzle3d_ensure_joint_distribution(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids);
    let object_total_before = puzzle3d_object_joint_total(&joint_weights, &object_weights, "Object", &vortex_ids);
    let sibling_before = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "b-s");
    let new_joint = object_total_before * 0.7;
    puzzle3d_set_vortex_joint_weight(&mut object_weights, &mut joint_weights, &object_ids, &vortex_ids, "Object", "c-b", new_joint);
    let object_total_after = puzzle3d_object_joint_total(&joint_weights, &object_weights, "Object", &vortex_ids);
    assert!((object_total_after - object_total_before).abs() < 1e-6);
    let sibling_after = puzzle3d_read_joint_weight(&joint_weights, &object_weights, "Object", "b-s");
    assert!(sibling_after < sibling_before, "raising one vortex must lower its siblings");
    let global = puzzle3d_global_joint_total(&joint_weights, &object_weights, &object_ids, &vortex_ids);
    assert!((global - 1.0).abs() < 1e-6);
}

/// 🚫️ Zero object-kind weight disables every vortex slider under that kind — anything × 0 is 0.
#[semio_framework_async_macros::async_test]
async fn zero_object_kind_weight_disables_joint_vortex_sliders() {
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("admitted host axis");
    let scene_snapshot = nakagin_scene_snapshot();
    let object_ids = puzzle3d_kind_ids(&scene_snapshot, "objects");
    assert!(!object_ids.is_empty(), "default scene_snapshot must expose object kinds");
    let zeroed_id = object_ids[0].clone();
    let mut object_kind_weights = puzzle3d_uniform_kind_weights(&object_ids);
    object_kind_weights = puzzle3d_normalize_kind_weight_group(&object_kind_weights, &object_ids, &zeroed_id, 0.0);
    assert!(object_kind_weights.get(&zeroed_id).copied().unwrap_or(1.0) <= f64::EPSILON);
    let scene = Puzzle3dScene { scene_snapshot, runtime: Puzzle3dRuntime { object_kind_weights, ..Puzzle3dRuntime::default() }, active_utility: semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID.into() };
    let fill_measures = fill_tool::measures(&scene, labels, None);
    let distribution_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution");
    let distribution_children = fill_measures
        .iter()
        .find_map(|measure| match measure {
            WindowMeasure::Group { id, children, .. } if id == &distribution_id => Some(children.as_slice()),
            _ => None,
        })
        .expect("fill must expose a Distribution group");
    let zeroed_group_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution-object-{zeroed_id}");
    let zeroed_group = distribution_children.iter().find(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == &zeroed_group_id)).expect("zeroed object kind must appear in distribution");
    match zeroed_group {
        WindowMeasure::Group { value: Some(value), children, .. } => {
            assert!(*value <= f64::EPSILON, "object-kind header must read 0%");
            assert!(!children.is_empty(), "object kind must still list vortex sliders");
            assert!(children.iter().all(|child| matches!(child, WindowMeasure::Slider { disabled: Some(true), value, .. } if *value <= f64::EPSILON)), "every joint vortex slider under a 0% object kind must be disabled at 0%");
        }
        other => panic!("expected object-kind group, got {other:?}"),
    }
    let live_group = distribution_children.iter().find(|measure| match measure {
        WindowMeasure::Group { id, value: Some(value), .. } if id != &zeroed_group_id => *value > f64::EPSILON,
        _ => false,
    });
    if let Some(WindowMeasure::Group { children, .. }) = live_group {
        assert!(children.iter().all(|child| matches!(child, WindowMeasure::Slider { disabled: None | Some(false), .. })), "joint vortex sliders under a non-zero object kind must stay enabled");
    }
}

/// 🎯️ Fill tool measures expose count + nested distribution tree under the Fill toggle; the Volume
/// Brush voxel dims live in a utility-options group in the window's own measures.
#[semio_framework_async_macros::async_test]
async fn fill_and_brush_params_are_tagged_utility_options_not_engagement_controls() {
    {
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("admitted host axis");
    let fill_scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID.into() };
    let fill_measures = fill_tool::measures(&fill_scene, labels, None);
    let distribution_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution");
    assert!(!fill_measures.iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "puzzle3d-play-tool-options-fill")), "fill must not wrap its options in a nested Fill group — the tool toggle already owns that row");
    assert_eq!(measure_group_tag(&fill_measures, &distribution_id), Some(None));
    let distribution_children = fill_measures
        .iter()
        .find_map(|measure| match measure {
            WindowMeasure::Group { id, children, .. } if id == &distribution_id => Some(children.as_slice()),
            _ => None,
        })
        .expect("fill must expose a Distribution group");
    assert!(!distribution_children.is_empty(), "distribution must list object-kind groups");
    assert!(distribution_children.iter().all(|measure| matches!(measure, WindowMeasure::Group { value: Some(_), on_change: Some(_), .. })), "each object-kind group must carry a header weight slider");
    assert!(
        distribution_children.iter().all(|measure| match measure {
            WindowMeasure::Group { label, value: Some(_), on_change: Some(_), .. } => !label.contains('%'),
            _ => false,
        }),
        "object-kind group labels must not embed percentages — the header slider owns the value readout"
    );
    assert!(
        distribution_children.iter().any(|measure| match measure {
            WindowMeasure::Group { children, .. } => children.iter().any(|child| matches!(child, WindowMeasure::Slider { label: Some(label), .. } if !label.contains('%'))),
            _ => false,
        }),
        "vortex joint sliders must label kinds without embedding percentages"
    );
    assert!(find_measure_toggle(&fill_measures, "puzzle3d-edit-volumes").is_none(), "fill must not carry edit-volumes toggle");
    assert!(find_measure_slider(&fill_measures, "puzzle3d-voxel-w").is_none(), "fill must not carry voxel-dimension sliders");
    assert!(find_measure_number(&fill_measures, "puzzle3d-fill-count").is_some(), "the fill-count entry always lives in the fill tool measures");
    assert!(
        !main::window_measures(&fill_scene, labels).iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id.contains("fill"))),
        "fill must no longer surface in window_measures — it is a mode-level tool, not a window utility"
    );
    let volume_brush_scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::volume_brush::UTILITY_ID.into() };
    let volume_brush_measures = main::window_measures(&volume_brush_scene, labels);
    assert_eq!(measure_group_tag(&volume_brush_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-volume-brush")), Some(Some(utilities::volume_brush::UTILITY_ID.into())));
    assert!(find_measure_slider(&volume_brush_measures, "puzzle3d-voxel-w").is_some(), "volume brush utility exposes voxel width slider");
    let fill_engagement = main::engagement(&fill_scene, &Puzzle3dLabels::NATIVE_EN, None);
    assert!(fill_engagement.control.is_none() && fill_engagement.controls.is_none(), "fill engagement HUD must no longer carry the relocated controls");
    let brush_scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::brush::UTILITY_ID.into() };
    assert_eq!(measure_group_tag(&main::window_measures(&brush_scene, labels), &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush")), Some(Some(utilities::brush::UTILITY_ID.into())));
    let brush_engagement = main::engagement(&brush_scene, &Puzzle3dLabels::NATIVE_EN, None);
    assert!(brush_engagement.control.is_none() && brush_engagement.controls.is_none(), "brush engagement HUD must no longer carry the relocated control");
    }
    assert_brush_options_after_open_vortex_suggestions().await;
}

#[inline(never)]
async fn assert_brush_options_after_open_vortex_suggestions() {
    let mut app = app().await;
    activate_window_utility(&mut app, utilities::brush::UTILITY_ID);
    let view = app.window_view(main::WINDOW_KIND_ID);
    let brush_app_measures = app.window_measures(&view).await;
    let window_measures = brush_app_measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(measure_group_tag(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush")), Some(Some(utilities::brush::UTILITY_ID.into())), "the brush Utility Options group surfaces on the live window-measures path when brush is the active utility");
}
//#endregion 🔖️Distribution

//#region 🔖️UiScope
/// 🐢️ THE scope law: every declared MUTATING command's `UiDirtyScope` names the panels its
/// mutation changes. A document edit moves the artifact outliner (the object roster), the field
/// inspector (the selected entity's own fields) and the framework history panel (one command row per
/// dispatch) — so a mutation whose scope is `Partial` and omits any of those three leaves that panel
/// showing pre-edit content until some unrelated later action happens to repaint it.
///
/// 🧯️ Why this is a law and not a review note: before wave N every hand-written `Partial` in this app
/// carried `panel_bodies: Vec::new()` (`📓️2026-09-09-wave-L-…md` §5.3) and the shared dispatcher's
/// default was `Full`, so the only two states a command could be in were "repaint the whole shell" and
/// "silently stale panels". `puzzle3d_command_scope_class` is now the one place that decides, and this
/// law is what stops a narrower class from being handed to a verb that edits the document.
///
/// 🈳️ `Chrome` (i.e. `Full`) always satisfies this law — it is the widest, always-correct answer, and a
/// newly declared command that nobody has classified yet is slow rather than wrong.
#[semio_framework_async_macros::async_test]
async fn command_scope_classes_name_the_panels_they_change() {
    use {semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_command_scope_class, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope};
    let definition = create_puzzle3d_app();
    let mut mutating = 0_usize;
    let mut narrowed = 0_usize;
    for action in dispatchable_actions(&definition) {
        let class = puzzle3d_command_scope_class(&action.id);
        let scope = puzzle3d_scope(class);
        // 🧾️ The declared `ActionKind` is a HISTORY/undo classification, not a paint one. Both
        // `relocateTargetVolume` and `worldRelocate` are Mutations (they emit artifact edits), so this
        // law requires them to name inspector, outliner and history. A `View` verb that happens to carry
        // the document class is over-painting at worst, never stale.
        if action.kind != ActionKind::Mutation {
            continue;
        }
        mutating += 1;
        match scope {
            UiDirtyScope::Full => {}
            UiDirtyScope::None => panic!("{} is a declared Mutation, so it cannot paint nothing", action.id),
            UiDirtyScope::Partial { ref panel_bodies, ref window_bodies, .. } => {
                narrowed += 1;
                assert_eq!(window_bodies, &vec![main::BODY_KEY.to_string()], "{} edits the document, so the world body is dirty", action.id);
                for body in [inspection::BODY_KEY, artifact::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
                    assert!(panel_bodies.iter().any(|named| named == body), "{} is a declared Mutation with a narrowed scope, so it must name the {body} panel body: {panel_bodies:?}", action.id);
                }
            }
        }
    }
    assert!(mutating > 0, "the app declares mutating commands");
    assert!(narrowed >= 10, "the scope table must actually narrow the mutating verbs, not fall back to Full for all of them: {narrowed}/{mutating}");
}

/// 🕹️ A pure SELECTION change repaints the field inspector — the panel that renders the selected
/// entity's own fields — and the outliner that marks the selected rows, but never the catalogue, which
/// lists object KINDS and cannot move when only the selection moves.
#[semio_framework_async_macros::async_test]
async fn selection_scope_names_the_inspector_and_not_the_catalogue() {
    use {semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dScopeClass};
    let UiDirtyScope::Partial { panel_bodies, measures, utilities, engagements, labels, .. } = puzzle3d_scope(Puzzle3dScopeClass::Selection) else {
        panic!("the selection class is a narrowed scope");
    };
    assert!(panel_bodies.iter().any(|body| body == inspection::BODY_KEY), "{panel_bodies:?}");
    assert!(panel_bodies.iter().any(|body| body == artifact::BODY_KEY), "{panel_bodies:?}");
    assert!(panel_bodies.iter().any(|body| body == FRAMEWORK_HISTORY_BODY_KEY), "{panel_bodies:?}");
    assert!(!panel_bodies.iter().any(|body| body == catalogue::BODY_KEY), "a selection cannot change the kind roster: {panel_bodies:?}");
    assert!(measures, "selection-dependent window measures are re-read");
    assert!(!utilities);
    assert!(!engagements);
    assert!(!labels);
}

/// 🎮️ Wave B34: the six framework interaction verbs are declared out of this app's own scope table
/// through `ArtifactEditor::interaction_scope`, and each one names EXACTLY the lanes it moves. Before
/// this wave the framework answered `UiDirtyScope::Full` for all six — including `interactionHover`,
/// which fires on pointer motion — so every mouse move over the viewport repainted every window body,
/// every panel body, the utilities/tools/engagements rails, the labels and the measures
/// (`📓️2026-09-12-wave-B32-world-lane-after-completion.md` §3.2: 24 interaction ingresses and 3
/// whole-shell completions per 80 s lane, which is what a document mutation's own refresh pass then
/// queued behind for 10–14 s).
#[semio_framework_async_macros::async_test]
async fn interaction_verbs_declare_exactly_the_lanes_they_move() {
    use {semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_interaction_chrome_scope, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_selection_scope, semio_s_artifact_puzzle_3d_editor_modes::windows::main::puzzle3d_viewport_scope, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_INTERACTION_DOMAIN};
    use semio_framework_plugin::InteractionVerb;
    let domain = [PUZZLE3D_INTERACTION_DOMAIN];
    assert_eq!(Puzzle3dPlayApp::interaction_scope(InteractionVerb::Hover, &domain), Some(puzzle3d_viewport_scope()), "a hover moves the world body's hover lane and nothing else");
    for verb in [InteractionVerb::Select, InteractionVerb::ClearSelection, InteractionVerb::SelectAll] {
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &domain), Some(puzzle3d_selection_scope()), "{verb:?} moves the declared selection lanes");
    }
    for verb in [InteractionVerb::SetSelectionMode, InteractionVerb::SetGranularity] {
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &domain), Some(puzzle3d_interaction_chrome_scope()), "{verb:?} moves the window's Select chrome");
    }
    for verb in InteractionVerb::ALL {
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &["mesh"]), None, "{verb:?} on a domain this app never declared must fall back to the framework scope");
        assert_eq!(Puzzle3dPlayApp::interaction_scope(verb, &[]), None, "{verb:?} touching no domain must fall back to the framework scope");
    }
}

/// 🐁️ The hover half, pinned field by field: a pointer move paints the world body ALONE — no panel, no
/// rail, no label, not even the window measures. The pick half names exactly the three selection panels
/// (`selection_scope_names_the_inspector_and_not_the_catalogue` pins their identity) and no fourth one,
/// so widening either answer has to widen this law with it.
#[semio_framework_async_macros::async_test]
async fn a_hover_paints_only_the_world_body_and_a_pick_adds_exactly_the_selection_panels() {
    use {semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_interaction_chrome_scope, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_selection_scope, semio_s_artifact_puzzle_3d_editor_modes::windows::main::puzzle3d_viewport_scope};
    let UiDirtyScope::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels } = puzzle3d_viewport_scope() else {
        panic!("the hover scope is a narrowed scope");
    };
    assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
    assert!(panel_bodies.is_empty(), "a hover cannot move a panel: {panel_bodies:?}");
    assert!(!utilities && !tools && !engagements && !measures && !labels, "a hover cannot move the shell chrome");
    let UiDirtyScope::Partial { panel_bodies, .. } = puzzle3d_selection_scope() else {
        panic!("the selection scope is a narrowed scope");
    };
    assert_eq!(panel_bodies.len(), 3, "a pick moves exactly the inspector, the outliner and the history: {panel_bodies:?}");
    for body in [inspection::BODY_KEY, artifact::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
        assert!(panel_bodies.iter().any(|named| named == body), "a pick moves {body}: {panel_bodies:?}");
    }
    let UiDirtyScope::Partial { panel_bodies, measures, .. } = puzzle3d_interaction_chrome_scope() else {
        panic!("the interaction chrome scope is a narrowed scope");
    };
    assert!(panel_bodies.is_empty(), "no panel renders the active selection mode or granularity: {panel_bodies:?}");
    assert!(measures, "the measures rail binds both Select controls");
}

/// 🈳️ The two narrow non-document classes deliberately name NO panel body — a camera move and a fill
/// count or distribution slider each touch the world body (and at most the tool/window measures) and
/// nothing else; pointing the brush suggestions run at a vortex paints nothing at all. Pins the claim their doc comments make, so widening one
/// of them has to widen this law too.
#[semio_framework_async_macros::async_test]
async fn viewport_and_fill_option_scope_classes_name_no_panel_body() {
    use {semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dScopeClass};
    for class in [Puzzle3dScopeClass::Viewport, Puzzle3dScopeClass::FillOptions] {
        let UiDirtyScope::Partial { panel_bodies, window_bodies, .. } = puzzle3d_scope(class) else {
            panic!("{class:?} is a narrowed scope");
        };
        assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()], "{class:?}");
        assert!(panel_bodies.is_empty(), "{class:?} claims to paint no panel: {panel_bodies:?}");
    }
    assert!(matches!(puzzle3d_scope(Puzzle3dScopeClass::Quiet), UiDirtyScope::None));
    assert_eq!(semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_command_scope_class("targetBrushSuggestions"), Puzzle3dScopeClass::Quiet);
    assert!(matches!(puzzle3d_scope(Puzzle3dScopeClass::Chrome), UiDirtyScope::Full));
}

/// 🪣️ `setFillCount` is configuration only — a live fill run holds its placements in the framework ledger —
/// so it declares the fill-options scope: the world body plus the tool and window measures, and no panel.
#[semio_framework_async_macros::async_test]
async fn set_fill_count_declares_the_fill_options_ui_scope() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID })), None).await.expect("select fill tool");
    let result = dispatch(&mut app, "setFillCount", Some(&json!({ "value": 1 })), None).await.expect("setFillCount");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
            assert!(panel_bodies.is_empty() || panel_bodies == [FRAMEWORK_HISTORY_BODY_KEY], "the count moves no document panel: {panel_bodies:?}");
            assert!(tools, "the fill-count entry lives in the fill tool's measures");
            assert!(measures);
            assert!(!engagements);
            assert!(!utilities);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for setFillCount, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn set_object_kind_weight_declares_fill_options_ui_scope() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID })), None).await.expect("select fill tool");
    let object_ids = puzzle3d_kind_ids(&nakagin_scene_snapshot(), "objects");
    let kind_id = object_ids.first().expect("object kind");
    let result = dispatch(&mut app, "setObjectKindWeight", Some(&json!({ "kindId": kind_id.as_str(), "value": 0.75 })), None).await.expect("setObjectKindWeight");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
            assert!(panel_bodies.is_empty());
            assert!(tools);
            assert!(measures, "distribution sliders live in tool + window measures");
            assert!(!engagements);
            assert!(!utilities);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for setObjectKindWeight, got {other:?}"),
    }
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `set_hover_is_a_view_action_with_no_ops_after_document_mutation`
// and `world_pick_declares_selection_ui_scope` deleted — both dispatched the now-deleted
// `setHover`/`worldPick` actions and asserted on the deleted `Effect::PatchWorld3dChrome`
// push-setter effect (selection/hover are framework-owned actions now, dispatched exclusively
// through the six reserved `interactionSelect`-family verbs; see `select_id`/`hover_id`).
//#endregion 🔖️UiScope

//#region 🔖️Utilities
#[semio_framework_async_macros::async_test]
async fn add_object_kind_honors_drop_origin() {
    let mut app = app().await;
    let before = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.5, 3.5, 0.0] })), None).await.expect("addObjectKind");
    assert_eq!(object_count(&app), before + 1);
    let projection = projection_of(&app);
    let object = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).expect("added object");
    let origin = object.get("origin").and_then(Value::as_array).expect("origin array");
    assert_eq!(origin.first().and_then(Value::as_f64), Some(2.5));
    assert_eq!(origin.get(1).and_then(Value::as_f64), Some(3.5));
    assert_eq!(origin.get(2).and_then(Value::as_f64), Some(0.0));
}

#[semio_framework_async_macros::async_test]
async fn add_object_kind_materializes_the_declared_kind_default() {
    // 📝️ P1 arg form: firing addObjectKind with no args on a document whose catalogs were cleared must
    // materialize the declared `objectKind` default — the catalog row AND the object referencing it —
    // in ONE gesture. A retained tool job never publishes inline (`settle`'s own docstring), so the
    // settled document, not the dispatch's `mutations` vector, is where the gesture is observed.
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    let before = object_count(&app);
    dispatch(&mut app, "addObjectKind", None, None).await.expect("addObjectKind");
    assert_eq!(object_count(&app), before + 1, "the materialized default kind adds exactly one object");
    let projection = projection_of(&app);
    let kind = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("objectKind")).and_then(Value::as_str);
    assert_eq!(kind, Some("Object"), "the declared objectKind default was materialized host-side");
    let catalog = projection.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("the declared default kind catalog was materialized");
    assert!(catalog.iter().any(|row| row.get("id").and_then(Value::as_str) == Some("Object")), "the created object references a catalogued kind, never a dangling one: {catalog:?}");
}

/// 🌲️ The catalogue's own gesture on the DEFAULT document: pressing (or dropping) a `concrete-forest`
/// kind row must place an instance that carries that kind's `representations[].url` as its own
/// `meshUrl`. Every other `addObjectKind` law fires `"Object"`, a kind no shipped catalog declares, so
/// none of them ever read a representation — which is how the `meshUrl`-only read in
/// `🎮️commands/🌱️add-object-kind` survived: a placed object with no mesh identity renders nothing and
/// its fill/brush candidates read `mesh-unavailable`.
#[semio_framework_async_macros::async_test]
async fn adding_a_catalogued_concrete_forest_kind_places_an_object_carrying_its_mesh_url() {
    let scene_snapshot = semio_s_artifact_puzzle_3d_editor_snapshot::default_scene_snapshot();
    let entry = semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_catalog_entries(&scene_snapshot, "objects").first().cloned().expect("the default document catalogues object kinds");
    let kind_id = entry.get("id").and_then(semio_framework_value::DslValue::as_str).expect("catalogued kind id").to_string();
    let expected = entry
        .get("representations")
        .and_then(semio_framework_value::DslValue::as_array)
        .into_iter()
        .flatten()
        .filter_map(|representation| representation.get("url").and_then(semio_framework_value::DslValue::as_str))
        .find(|url| !url.is_empty())
        .expect("the catalogued kind names a representation url")
        .to_string();
    let mut app = app().await;
    let before = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": kind_id.clone(), "origin": [4.0, 5.0, 0.0] })), None).await.expect("addObjectKind");
    assert_eq!(object_count(&app), before + 1, "a catalogued kind places exactly one instance");
    let projection = projection_of(&app);
    let object = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).expect("the placed object").clone();
    assert_eq!(object.get("objectKind").and_then(Value::as_str), Some(kind_id.as_str()), "the placed instance references the catalogued kind");
    assert_eq!(object.get("meshUrl").and_then(Value::as_str), Some(expected.as_str()), "the placed instance must carry its kind's representation url as its mesh identity");
}

/// 🌱️ A catalog TEMPLATE seats its vortex at `point`; a PLACED vortex carries `position`. The
/// non-migrated `addObjectKind` route seeded from `position` alone, so every seat of every catalogued
/// kind collapsed onto the object origin and the object it placed had a degenerate rim no brush could
/// use. Pins the real seats of a real shipped kind.
#[test]
fn catalogued_kind_templates_seed_vortices_at_their_catalog_points() {
    let scene_snapshot = semio_s_artifact_puzzle_3d_editor_snapshot::default_scene_snapshot();
    let entry = semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_catalog_entries(&scene_snapshot, "objects").first().cloned().expect("the default document catalogues object kinds");
    let seats = semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_vortices_from_kind_template(&entry);
    let templates = entry.get("vortices").and_then(semio_framework_value::DslValue::as_array).expect("the catalogued kind declares vortex templates");
    assert_eq!(seats.len(), templates.len(), "one seeded vortex per declared template");
    assert!(seats.iter().any(|seat| seat.position != [0.0, 0.0, 0.0]), "a catalogued kind's seats must not all collapse onto the object origin: {seats:?}");
    for (seat, template) in seats.iter().zip(templates) {
        let point: [f64; 3] = template.get("point").and_then(|value| semio_framework_value::FromValue::from_value(value.clone()).ok()).expect("every template declares its point");
        assert_eq!(seat.position, point, "seeded vortex {} must sit on its template point", seat.id);
    }
}

/// 🛍️ Both doors onto the default document carry the same catalogue: the INITIAL snapshot the app
/// boots with, and `setActiveExample("concrete-forest")` — which replaces the catalogs through their
/// own `replace_kind_catalogs` mutation and could therefore drop them independently of the scene_snapshot.
#[semio_framework_async_macros::async_test]
async fn the_initial_snapshot_and_set_active_example_both_carry_the_concrete_forest_catalogue() {
    let declared: Vec<String> = semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_catalog_entries(&semio_s_artifact_puzzle_3d_editor_snapshot::default_scene_snapshot(), "objects")
        .iter()
        .filter_map(|entry| entry.get("id").and_then(semio_framework_value::DslValue::as_str).map(str::to_string))
        .collect();
    assert!(!declared.is_empty(), "the concrete-forest scene_snapshot catalogues object kinds");
    let catalogued = |app: &Puzzle3dApp| -> Vec<String> {
        projection_of(app).pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).map(|rows| rows.iter().filter_map(|row| row.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default()
    };
    let mut app = app().await;
    assert_eq!(catalogued(&app), declared, "the app boots on the concrete-forest catalogue");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_CONCRETE_FOREST })), None).await.expect("concrete-forest");
    assert_eq!(catalogued(&app), declared, "switching back to concrete-forest restores its catalogue");
}

/// 🧊️ Wave B11 (checklist §10, `📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 12): the Volume
/// Brush armed and its W/D/H sliders moved, but Alt+click added nothing. This is the guest half of that
/// gesture — an `addTargetVolume` carrying the host's grid-snapped ground origin must land ONE volume
/// there, sized by the utility's own voxel dimensions times the grid spacing.
#[semio_framework_async_macros::async_test]
async fn alt_click_adds_one_target_volume_sized_by_the_utility_voxel_dims() {
    let volumes = |app: &Puzzle3dApp| projection_of(app).get("targetVolumes").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::volume_brush::UTILITY_ID })), None).await.expect("arm the volume brush");
    for (axis, value) in [("w", 2.0), ("d", 3.0), ("h", 4.0)] {
        dispatch(&mut app, "setVoxelDims", Some(&json!({ "axis": axis, "value": value })), Some(main::WINDOW_KIND_ID)).await.expect("voxel dimension");
    }
    let before = volumes(&app);
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [3.0, -2.0, 0.0] })), Some(main::WINDOW_KIND_ID)).await.expect("addTargetVolume");
    assert_eq!(volumes(&app), before + 1, "one Alt+click places exactly one target volume");
    let projection = projection_of(&app);
    let volume = projection.get("targetVolumes").and_then(Value::as_array).and_then(|entries| entries.last()).expect("the placed volume");
    let scale: Vec<f64> = volume.get("scale").and_then(Value::as_array).expect("the volume carries a scale").iter().filter_map(Value::as_f64).collect();
    assert_eq!(scale.len(), 3, "a voxel volume is a three-axis box: {scale:?}");
    assert!(scale[1] > scale[0] && scale[2] > scale[1], "the W/D/H steppers must size the placed box in their own order: {scale:?}");
    let origin: Vec<f64> = volume.get("origin").and_then(Value::as_array).expect("the volume carries an origin").iter().filter_map(Value::as_f64).collect();
    assert_eq!(origin.len(), 3, "the volume lands at the pointed ground point: {origin:?}");
    // 🔢️ A grid-snapped ground point is USUALLY whole (`[0,10,0]` was the live browser payload), and JSON
    // serializes a whole f64 as an integer — so the decode must admit integer carriers, not only floats.
    let whole = volumes(&app);
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [0, 10, 0] })), Some(main::WINDOW_KIND_ID)).await.expect("addTargetVolume with integer coordinates");
    assert_eq!(volumes(&app), whole + 1, "an integer-valued ground point must place a volume exactly like a fractional one");
}

/// 🧯️ …and a dispatch that carries no usable ground point answers with ONE localized notice instead of
/// returning silently, so a miss is distinguishable from a dead gesture.
#[semio_framework_async_macros::async_test]
async fn add_target_volume_without_a_ground_point_raises_one_notice_and_places_nothing() {
    let volumes = |app: &Puzzle3dApp| projection_of(app).get("targetVolumes").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    let mut app = app().await;
    let before = volumes(&app);
    let result = dispatch(&mut app, "addTargetVolume", None, Some(main::WINDOW_KIND_ID)).await.expect("a missing origin is a refusal, not a fault");
    let notices: Vec<&String> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message),
            _ => None,
        })
        .collect();
    assert_eq!(notices.len(), 1, "an origin-less placement must say why: {:?}", result.requested_effects);
    assert_ne!(notices[0].as_str(), PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so the refusal must be real prose");
    assert_eq!(volumes(&app), before, "a refused placement adds nothing");
}

#[semio_framework_async_macros::async_test]
async fn set_active_utility_emits_no_ops_and_no_history_entry() {
    // 🧰️ Switching utilities is the framework-injected View action: no document operations, no undo
    // entry, no re-emitted utility-switch effect (the command IS the direct switch).
    let mut app = app().await;
    let before = projection_of(&app);
    let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), None).await.expect("switch utility");
    assert!(result.mutations.is_empty(), "utility switching never emits document operations");
    assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
    assert_eq!(projection_of(&app), before, "utility switching does not mutate the document");
}

#[test]
fn set_active_utility_dirties_the_world_body() {
    use {semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_command_scope_class, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dScopeClass};
    assert_eq!(puzzle3d_command_scope_class(SET_ACTIVE_UTILITY_ACTION_ID), Puzzle3dScopeClass::Viewport, "utility switch must republish the world body (interaction + vortices), not Full chrome and not None");
    match puzzle3d_scope(Puzzle3dScopeClass::Viewport) {
        UiDirtyScope::Partial { window_bodies, .. } => assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]),
        other => panic!("utility switch scope must be the world body, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn engagement_exposes_no_utility_switch_options() {
    // 🧰️ select/brush/fill switching lives only on the framework utility bar; the engagement HUD
    // must not duplicate it as options. Object placement is catalogue drag-and-drop plus the shell
    // menu row `openAddObjectDialog`, not a per-viewport quick-action chip.
    let scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let engagement = main::engagement(&scene, &Puzzle3dLabels::NATIVE_EN, None);
    let options = engagement.options.as_deref().unwrap_or(&[]);
    assert!(options.is_empty(), "the puzzle3d engagement must not publish window quick-action options");
    assert!(options.iter().all(|option| !matches!(option.id.as_str(), "select" | "brush" | "fill" | "volumeBrush" | "worldRelocate" | "shell-menu.action.openAddObjectDialog")), "the puzzle3d engagement must not re-expose utilities or add-object chrome as options");
}

#[semio_framework_async_macros::async_test]
async fn transform_engagement_does_not_block_background_deselect() {
    let scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::transform::UTILITY_ID.into() };
    assert_eq!(main::engagement(&scene, &Puzzle3dLabels::NATIVE_EN, None).session_active, Some(false));
}

//#endregion 🔖️Utilities

//#region 🔖️WorldSelection
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the app-owned `worldSelect`
/// command is deleted — selection now goes exclusively through the framework's `interactionSelect`
/// verb (`select_id`), which is view-only by construction (`dispatch_interaction_action` never
/// touches `self.store`). Proves the `vortex` domain wiring reaches that same guarantee.
#[semio_framework_async_macros::async_test]
async fn world_select_emits_no_artifact_mutations() {
    let mut app = app().await;
    let before = projection_of(&app);
    let object_id = first_object_id(&app);
    let result = select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    assert!(result.mutations.is_empty(), "interactionSelect is framework-owned and view-only, must not diff the document");
    assert_eq!(projection_of(&app), before);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: selection paint rides on
/// `selectionJson` only — instance GEOMETRY stays byte-stable across a pick, so the host never has to
/// re-upload meshes just because something was selected.
#[semio_framework_async_macros::async_test]
async fn world_pick_keeps_instances_geometry_json_stable() {
    let mut app = app().await;
    let instances_before = instances_of(&render_composite(&mut app).await);
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let after = render_composite(&mut app).await;
    assert_eq!(instances_of(&after), instances_before, "picking must never perturb instance geometry");
    assert_eq!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()), Some(vec![object_id.clone()]));
    assert_eq!(
        selection_of(&after).get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()),
        Some(vec![object_id]),
        "`selectionJson.ids` is what World3dHost paints instance selection from"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selectionJson` carries the exact
/// field names `World3dHost`'s `parseSelection`/`WorldSelectionRecord` reads — `ids` for object
/// instances, `targetVolumeIds`, `referenceSelectedId`, `hoveredId`. Vortex marks deliberately do NOT
/// appear here (that record has no vortex field; see `world_selection_json`'s doc comment).
#[semio_framework_async_macros::async_test]
async fn world_selection_json_carries_the_host_field_names_per_granularity() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, Some(&object_id)).await.expect("hover object");
    let selection = selection_of(&render_composite(&mut app).await);
    assert_eq!(selection.get("hoveredId").and_then(Value::as_str), Some(object_id.as_str()));
    assert_eq!(selection.get("targetVolumeIds").and_then(Value::as_array).map(Vec::len), Some(0));
    assert!(selection.get("vortexIds").is_none(), "the host's WorldSelectionRecord has no vortexIds field");

    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [0.0, 0.0, 0.0] })), Some(main::WINDOW_KIND_ID)).await.expect("addTargetVolume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.last().cloned()).and_then(|volume| volume.get("id").and_then(Value::as_str).map(str::to_string)).expect("target volume id");
    select_id(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id).await.expect("select target volume");
    let selection = selection_of(&render_composite(&mut app).await);
    assert_eq!(
        selection.get("targetVolumeIds").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()),
        Some(vec![volume_id]),
        "a target-volume granularity selection paints through `targetVolumeIds`"
    );
    assert_eq!(selection.get("ids").and_then(Value::as_array).map(Vec::len), Some(0), "object `ids` stay empty while a target volume is the live granularity");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the world scene binds `domainId` (and
/// its own `domainGranularityId`) so `World3dHost` dispatches the generic `interactionSelect`/
/// `interactionHover` verbs — this crate has no handler for the legacy `worldPick`/`worldSelect`/
/// `setHover` fallbacks the host would otherwise use.
#[semio_framework_async_macros::async_test]
async fn world_scene_binds_the_vortex_interaction_domain_and_its_granularity() {
    let mut app = app().await;
    let scene = render_composite(&mut app).await.get("world3d").cloned().unwrap_or(Value::Null);
    assert_eq!(scene.get("domainId").and_then(Value::as_str), Some(PUZZLE3D_INTERACTION_DOMAIN));
    assert_eq!(scene.get("domainGranularityId").and_then(Value::as_str), Some(PUZZLE3D_GRANULARITY_OBJECT));
}

#[semio_framework_async_macros::async_test]
async fn world_pick_null_clears_without_reselecting_first_object() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_some_and(|selection| !selection.ids.is_empty()));
    dispatch(
        &mut app,
        "interactionSelect",
        Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": "[]", "merge": "replace", "method": "pick" })),
        None,
    )
    .await
    .expect("empty-target interactionSelect");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()), "clicking empty background must clear, never fall back to reselecting the first object");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove that
/// world-picking a LOCKED object clears the selection instead of selecting it — the app-owned
/// `worldPick` command, which could read `object.locked` before deciding, is deleted.
/// `interactionSelect` is a framework-generic id/merge verb blind to app data, and this app's
/// `interaction_topology` does not filter locked ids out of the `vortex` domain either (see that
/// function's doc) — "never select a locked object" is now entirely a host click→pick
/// translation concern, not reachable from this crate. Proves the Rust-side floor instead: the
/// `vortex` domain has no lock awareness, so selecting a locked object's id still succeeds.
#[semio_framework_async_macros::async_test]
async fn world_pick_locked_object_clears_like_background() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "flag": "locked", "value": true })), None).await.expect("lock");
    let instances = instances_of(&render_composite(&mut app).await);
    assert_eq!(instances.first().and_then(|entry| entry.get("disabled")).and_then(Value::as_bool), Some(true));
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select locked object");
    assert_eq!(
        app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()),
        Some(vec![object_id]),
        "the vortex domain has no lock awareness — this now succeeds, the host must gate locked picks itself"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `PUZZLE3D_VORTEX_SHOW_SELECTED`
/// reveals a marker only while its own object (or one of its markers) is selected/hovered — the
/// live read `render_with_request_context` threads in as a `Puzzle3dInteractionSnapshot`.
#[semio_framework_async_macros::async_test]
async fn world_vortices_reveal_in_selected_mode_only_for_the_selected_object() {
    let mut app = app().await;
    let all_vortex_ids = vortex_full_ids(&app);
    assert!(!all_vortex_ids.is_empty(), "scene_snapshot must expose vortices");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "Selected mode with nothing selected reveals no vortex marker");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let revealed = vortices_of(&render_composite(&mut app).await);
    assert!(!revealed.is_empty(), "Selected mode must reveal the selected object's own vortex markers");
    assert!(revealed.iter().all(|vortex| vortex.get("objectId").and_then(Value::as_str) == Some(object_id.as_str())), "Selected mode must reveal ONLY the selected object's markers");
    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "clearing the selection hides the markers again");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow");
    assert!(!vortices_of(&render_composite(&mut app).await).is_empty(), "Always mode must still reveal every vortex marker");
}

/// 🕹️ A selected vortex marker carries its own `selected` flag on `vorticesJson` — the host's
/// `WorldVortexMarkers` reads it off each record (`WorldSelectionRecord` has no vortex field), and a
/// hovered marker carries `hovered` the same way.
#[semio_framework_async_macros::async_test]
async fn world_vortices_carry_their_own_selected_and_hovered_flags() {
    let mut app = app().await;
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow");
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let selected = vortices_of(&render_composite(&mut app).await);
    let record = selected.iter().find(|entry| entry.get("fullId").and_then(Value::as_str) == Some(vortex.as_str())).expect("selected vortex record");
    assert_eq!(record.get("selected").and_then(Value::as_bool), Some(true));
    assert!(selected.iter().filter(|entry| entry.get("fullId").and_then(Value::as_str) != Some(vortex.as_str())).all(|entry| entry.get("selected").and_then(Value::as_bool) == Some(false)), "only the picked marker is selected");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover vortex");
    let hovered = vortices_of(&render_composite(&mut app).await);
    let record = hovered.iter().find(|entry| entry.get("fullId").and_then(Value::as_str) == Some(vortex.as_str())).expect("hovered vortex record");
    assert_eq!(record.get("hovered").and_then(Value::as_bool), Some(true));
    assert_eq!(interaction_of(&render_composite(&mut app).await).get("hoveredVortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `worldVortexSelect`/`worldPick`
/// are deleted; a `vortex`-domain `DomainSelection` only ever carries one granularity at a time
/// (see `Puzzle3dActionCtx::selected_ids`'s doc), so a `merge: "replace"` pick at a different
/// granularity inherently replaces the whole prior selection — verified against
/// `interaction_state()` (the render-time `selectionJson`/`vorticesJson` fields carry no live ids
/// any more, per `world_selection_json`'s known-gap doc comment).
#[semio_framework_async_macros::async_test]
async fn world_pick_object_replaces_vortex_selection() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    assert_eq!(selection.ids, vec![object_id]);
}

#[semio_framework_async_macros::async_test]
async fn world_vortex_select_clears_object_selection() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_VORTEX);
    assert_eq!(selection.ids, vec![vortex]);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove a
/// PERSISTED "default merge mode" (`selection_mode_default`, a config field — on this crate's
/// DELETE list) flips `worldVortexSelect`'s implicit merge between replace/invertive. The
/// framework has no equivalent persisted concept: `interactionSelect`'s `merge` arg is supplied
/// explicitly on every dispatch (the host decides per click — e.g. a held modifier key — never
/// defaulted from stored state). Proves the underlying `merge: "invertive"` primitive still
/// toggles a second target back into the selection instead.
#[semio_framework_async_macros::async_test]
async fn world_vortex_click_replaces_until_invertive_mode_is_selected() {
    let mut app = app().await;
    let vortices = vortex_full_ids(&app);
    assert!(vortices.len() >= 2, "scene_snapshot must expose two vortices");
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortices[0]).await.expect("select first vortex");
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortices[1]).await.expect("replace with second vortex");
    let replaced = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(replaced.ids, vec![vortices[1].clone()]);

    let targets = to_json_string(&vec![InteractionTarget { granularity: PUZZLE3D_GRANULARITY_VORTEX.into(), id: vortices[0].clone() }]);
    dispatch(&mut app, "interactionSelect", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "invertive", "method": "pick" })), None).await.expect("invertive toggle");
    let invertive = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(invertive.ids.len(), 2, "invertive merge toggles the first vortex back into the selection alongside the second");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: "Select Same Kind" emits a real
/// `Emit.interaction_writes` entry, which `VcsArtifactApp` applies through the same `next_selection`
/// machine the reserved `interactionSelect` verb uses — so the widened selection is observable on
/// `interaction_state()` and painted into `selectionJson` on the next render.
#[semio_framework_async_macros::async_test]
async fn select_same_kind_widens_the_selection_to_every_object_of_that_kind() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    for _ in 0..3 {
        dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    }
    let ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).map(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default();
    assert_eq!(ids.len(), 3, "three same-kind objects must exist");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &ids[0]).await.expect("select one");
    dispatch(&mut app, "selectSameKindSelection", None, None).await.expect("selectSameKindSelection");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    let mut widened = selection.ids.clone();
    widened.sort();
    let mut expected = ids.clone();
    expected.sort();
    assert_eq!(widened, expected, "every object of the clicked object's kind is selected");
    let painted = selection_of(&render_composite(&mut app).await).get("ids").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()).unwrap_or_default();
    let mut painted_sorted = painted.clone();
    painted_sorted.sort();
    assert_eq!(painted_sorted, expected, "the widened selection reaches the host through selectionJson");
}

/// 🕹️ "Select Same Kind" with nothing selected still aborts (nothing to widen from) and leaves the
/// selection untouched.
#[semio_framework_async_macros::async_test]
async fn select_same_kind_with_no_selection_leaves_the_selection_untouched() {
    let mut app = app().await;
    dispatch(&mut app, "selectSameKindSelection", None, None).await.expect("selectSameKindSelection");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()), "widening from nothing must not invent a selection");
}

/// 🎯️ A selection-scoped command dispatched with NOTHING selected must refuse VISIBLY: exactly one
/// `Effect::Notify` and no document edit. This is the browser defect measured
/// 2026-09-09 21:05 — "Duplicate Selection" from the context menu completed with no clone, no command
/// row and no notice of any kind, which is indistinguishable from a dead menu entry. Every arm that
/// reads the framework-owned selection is covered here, so the refusal cannot be reintroduced one
/// command at a time.
///
/// 🧾️ A command-log ROW is deliberately not asserted absent: `dispatch_emit` records one row per
/// dispatch by design, mutation-kind actions that produced zero operations included (see its own doc
/// in `🔌️plugin/🦀️.rs`), so a refused `Mutation` still shows up in history with no edit id. What must
/// never happen is a silent nothing — that is what the notice below pins.
#[semio_framework_async_macros::async_test]
async fn selection_scoped_commands_with_no_selection_refuse_with_exactly_one_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Notify { message } => Some(message.clone()),
                _ => None,
            })
            .collect()
    };
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    // 🌱️ The seed gesture SELECTS what it adds (`add_object_kind_selects_the_object_it_adds`), so the
    // empty precondition this law is about has to be established explicitly — otherwise every arm below
    // measures a live selection and refuses nothing (wave B36).
    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear the seed gesture's own selection");
    let objects_before = projection_of(&app).get("objects").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    for action in ["duplicateSelection", "deleteSelection", "selectSameKindSelection"] {
        let result = dispatch(&mut app, action, None, None).await.unwrap_or_else(|error| panic!("{action} must complete, not fault: {error:?}"));
        let raised = notices(&result);
        assert_eq!(raised.len(), 1, "{action} with nothing selected must raise exactly one notice: {:?}", result.requested_effects);
        assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so {action}'s refusal must be real prose");
        assert!(result.mutations.is_empty(), "{action} refused, so it must emit no document mutation: {:?}", result.mutations);
        assert!(!matches!(result.ui_scope, UiDirtyScope::Full), "{action} painted nothing, so it must not force a full refresh");
    }
    // 🧲️ The three gumball verbs run the transform tool (`Puzzle3dTransformWork`), whose whole outcome
    // (notice, scope) lives on the completion lane that `context::settle` drains into `InvocationResult`: a
    // gesture addressing nothing refuses with one notice and paints nothing.
    for (action, args) in [
        ("translateSelection", json!({ "dx": 1.0, "dy": 0.0, "dz": 0.0 })),
        ("rotateSelection", json!({ "ax": 0.0, "ay": 0.0, "az": 1.0, "angle": 1.0 })),
        ("scaleSelection", json!({ "sx": 2.0, "sy": 2.0, "sz": 2.0 })),
    ] {
        let result = dispatch(&mut app, action, Some(&args), None).await.unwrap_or_else(|error| panic!("{action} must complete, not fault: {error:?}"));
        let raised = notices(&result);
        assert_eq!(raised.len(), 1, "{action} with nothing selected must raise exactly one notice on its completion lane: {:?}", result.requested_effects);
        assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so {action}'s refusal must be real prose");
        assert!(result.mutations.is_empty(), "{action} refused, so it must emit no document mutation: {:?}", result.mutations);
        assert!(matches!(result.ui_scope, UiDirtyScope::None), "{action} painted nothing, so its completion must carry UiDirtyScope::None, got {:?}", result.ui_scope);
    }
    let objects_after = projection_of(&app).get("objects").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    assert_eq!(objects_after, objects_before, "a refused selection command must leave the document untouched");
    assert!(
        app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()),
        "a refused selection command must not invent a selection either"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `duplicateSelection` re-selects the
/// clones it created — the interaction write is applied AFTER the document mutations land, so the new
/// ids are already in `interaction_topology` and survive `validate_state`'s pruning.
#[semio_framework_async_macros::async_test]
async fn duplicate_selection_reselects_the_created_clones() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "duplicateSelection", None, None).await.expect("duplicateSelection");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    assert_eq!(selection.ids.len(), 1, "exactly the one clone is selected");
    assert_ne!(selection.ids.first().map(String::as_str), Some(object_id.as_str()), "the CLONE is selected, not the original");
    let live_ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).map(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default();
    assert!(live_ids.contains(selection.ids.first().expect("clone id")), "the re-selected id must exist in the document");
}

/// 🌱️ `addObjectKind` SELECTS the object it adds — the same `Emit.interaction_writes` contract
/// `duplicateSelection` and `addBrushObject` already honour, on both of its implementations: the
/// cursorized `Puzzle3dAddObjectKindWork` the interactive job runs and the direct reducer arm. Adding an
/// object the user must then hunt for is half a gesture: the catalogue row, the drag-drop and the Add
/// Object dialog all end with the new object nowhere in the inspector, the gumball or the outliner
/// (battery #53 `catalogue-add-selects-new-object added=1`, ticket 26/09/02/PUZZLE-3D-END-TO-END wave
/// B36). Every id is asserted against the live document, so a write naming a phantom fails here.
#[semio_framework_async_macros::async_test]
async fn add_object_kind_selects_the_object_it_adds() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("first object");
    let first = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(first.granularity, PUZZLE3D_GRANULARITY_OBJECT, "the add selects at OBJECT granularity: {first:?}");
    assert_eq!(first.ids.len(), 1, "exactly the added object is selected: {first:?}");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [4.0, 0.0, 0.0] })), None).await.expect("second object");
    let second = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(second.ids.len(), 1, "the second add REPLACES the selection with its own object, it does not widen it: {second:?}");
    assert_ne!(second.ids, first.ids, "the second add selects the object IT created: {second:?} vs {first:?}");
    let live_ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).map(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default();
    assert_eq!(live_ids.len(), 2, "both adds landed in the document: {live_ids:?}");
    assert!(live_ids.contains(second.ids.first().expect("added id")), "the selected id is the one the document carries: {live_ids:?}");
}

/// 🗑️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B31: one `deleteSelection` on a live object selection
/// removes that object from the RENDERED world census within the same settle, records exactly one
/// command-log row, and leaves the deleted id out of the framework-owned selection — so pressing
/// `Delete` twice cannot land a second, empty edit on an id the document no longer carries. The
/// browser red this states was `before=2 after=2 waitedMs=30299` with
/// `history patch applied labels=["delete-object id=object-1"]` in the same run: the document moved
/// and the selection did not, so every later delete was a no-op against a phantom.
#[semio_framework_async_macros::async_test]
async fn delete_selection_shrinks_the_world_census_and_drops_the_deleted_id() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("first object");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("second object");
    let census_before = instance_count(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(census_before, 2, "two added objects must both be in the rendered world census");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    let (deleted, settled) = dispatch_reporting(&mut app, "deleteSelection", None, None).await;
    deleted.expect("deleteSelection");
    assert_eq!(history_rows(&settled), 1, "a delete that removed an object must publish exactly one command-log row");
    assert_eq!(object_count(&app), 1, "the document must carry one object fewer");
    assert_eq!(instance_count(&render_window(&mut app, main::WINDOW_KIND_ID).await), census_before - 1, "the rendered world census must shrink by one within the same settle");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert!(!selection.ids.contains(&object_id), "the deleted id must be gone from the selection, else a second Delete is a silent no-op: {:?}", selection.ids);
    let (again, settled_again) = dispatch_reporting(&mut app, "deleteSelection", None, None).await;
    again.expect("second deleteSelection");
    // 🧾️ A Mutation that emits nothing still logs one row (`edit_id: None`), on THIS settle — the same
    // empty-emit row `dispatch_emit` files under "Without Operations". What must not happen is an edit.
    let again_rows = history_row_labels(&settled_again);
    assert_eq!(history_rows(&settled_again), 1, "a delete with nothing selected logs the command and no edit: {again_rows:?}");
    assert!(again_rows.iter().all(|row| row.contains("deleteSelection") && row.contains("applied=false") && row.contains("ops=0")), "{again_rows:?}");
    assert_eq!(object_count(&app), 1, "and must not touch the document");
}
//#endregion 🔖️WorldSelection

//#region 🔖️Gumball
/// 🕹️ Framework leftover `interactionView` must not arm the gumball on a bare pick — only the guest
/// `selectionJson` lane may set `gumballActive`, and only when the transform utility is active.
#[semio_framework_async_macros::async_test]
async fn interaction_select_leftover_does_not_arm_gumball_without_transform_utility() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    let targets = to_json_string(&vec![InteractionTarget { granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), id: object_id.clone() }]);
    let admitted = dispatch_reserved_unsettled(
        &mut app,
        "interactionSelect",
        Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })),
        Some(main::WINDOW_KIND_ID),
    )
    .await
    .expect("interactionSelect admit");
    let settled = settle_reserved(&mut app, admitted).await.expect("interactionSelect settle");
    let gumball = settled
        .output
        .get("interactionView")
        .and_then(|view| view.get("gumball"))
        .expect("leftover gumball");
    assert_eq!(gumball.get("active").and_then(semio_framework_value::DslValue::as_bool), Some(false));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `gumballActive` requires BOTH the
/// transform utility active AND a live object (or target-volume) selection — the live read
/// `render_with_request_context` threads in. `transformMode`/`gumballConfig` depend only on the
/// active utility, per window.
#[semio_framework_async_macros::async_test]
async fn gumball_active_only_for_transform_utilities_with_object_selection() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let idle_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(idle_selection.get("gumballActive").and_then(Value::as_bool), Some(false), "selection alone must not show the gumball");
    assert!(idle_selection.get("transformMode").is_none(), "non-transform utility must not emit transformMode");

    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    let transform_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(transform_selection.get("gumballActive").and_then(Value::as_bool), Some(true), "transform utility plus a live object selection shows the gumball");
    assert_eq!(transform_selection.get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()), Some(vec![object_id.clone()]));
    assert_eq!(transform_selection.get("activeObjectId").and_then(Value::as_str), Some(object_id.as_str()));
    assert_eq!(transform_selection.get("transformMode").and_then(Value::as_str), Some("transform"));
    assert_eq!(transform_selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
    assert_eq!(transform_selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(true));

    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear");
    assert_eq!(selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await).get("gumballActive").and_then(Value::as_bool), Some(false), "an unattached gumball must never render");

    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("reselect");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
    let brush_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(brush_selection.get("gumballActive").and_then(Value::as_bool), Some(false));
    assert!(brush_selection.get("transformMode").is_none());
}

/// 🕹️ Both handle flags off leaves nothing to grab, so the gumball must not render even with a live
/// selection and the transform utility active (`setTransformGumballFlag` is what the user toggles).
#[semio_framework_async_macros::async_test]
async fn gumball_inactive_when_every_handle_flag_is_off() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "move", "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("no move");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("no rotate");
    let selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(selection.get("gumballActive").and_then(Value::as_bool), Some(false), "a gumball with no handles must not render");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `transformMode` depends only on
/// each window instance's host-owned `ViewModel` utility map, so it is the per-window-isolation proof.
#[semio_framework_async_macros::async_test]
async fn transform_utility_is_local_to_the_window_instance_not_shared_across_split_panes() {
    let mut app = app().await;
    let top = main::WINDOW_INSTANCE_TOP;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    dispatch(&mut app, "worldPointerDown", None, Some(perspective)).await.expect("register perspective");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(top)).await.expect("transform on top");
    let top_selection = selection_of(&render_window(&mut app, top).await);
    assert_eq!(top_selection.get("transformMode").and_then(Value::as_str), Some("transform"), "transform on top pane must switch that pane's own scene mode");
    let perspective_selection = selection_of(&render_window(&mut app, perspective).await);
    assert!(perspective_selection.get("transformMode").is_none(), "perspective pane must not inherit top pane's transform utility");
}

#[semio_framework_async_macros::async_test]
async fn transform_utility_options_expose_move_and_rotate_flags() {
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).expect("admitted host axis");
    let scene = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::transform::UTILITY_ID.into() };
    let measures = main::window_measures(&scene, labels);
    assert_eq!(measure_group_tag(&measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-transform")), Some(Some(utilities::transform::UTILITY_ID.into())));
    assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-move"), Some(true));
    assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-rotate"), Some(true));
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("disable rotate");
    let selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
    assert_eq!(selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(false));
    let view = app.window_view(main::WINDOW_KIND_ID);
    let app_measures = app.window_measures(&view).await;
    let window_measures = app_measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_toggle(window_measures, "puzzle3d-transform-rotate"), Some(false));
}

fn object_orientation(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("orientation").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

fn object_scale(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("scale").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

/// 🧾️ The applied history rows one settle upserted that carry document ops.

fn english(entry: &semio_framework::kernel::HistoryEntry) -> String {
    entry.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string()
}

fn german(entry: &semio_framework::kernel::HistoryEntry) -> String {
    entry.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De).to_string()
}

/// 🌱️ An empty scene with one fresh object, the object selected and the Transform utility active.
async fn gumball_app() -> (Puzzle3dApp, String) {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    (app, object_id)
}

/// 🛠️ One gumball drag — the ONE pose delta both hosts dispatch on release — is ONE tool transaction: one
/// edit, one history row stamped with the transform tool's `TransactionRef`, whose op is the parametric
/// `drag-selection` over the gesture's literal ids and offset, labelled from the leaf in English and German.
/// One undo restores the pose exactly.
#[semio_framework_async_macros::async_test]
async fn one_gumball_translate_is_one_edit_one_row_and_one_transaction() {
    let (mut app, object_id) = gumball_app().await;
    let start = object_origin(&app, &object_id);
    let (result, settled) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 6.0, "dy": 0.0, "dz": 0.0 })), None).await;
    result.expect("the release delta commits");
    let rows = edit_rows(&settled);
    assert_eq!(rows.len(), 1, "one gesture, one history row: {}", history_row_labels(&settled).join(" | "));
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.puzzle.puzzle3d@1/*#editor#translateSelection", "the transform tool authored it: {transaction:?}");
    assert!(rows[0].op_lines.len() == 1 && rows[0].op_lines[0].starts_with("drag-selection"), "the one op is the parametric leaf: {:?}", rows[0].op_lines);
    assert_eq!((english(&rows[0]), german(&rows[0])), ("Drag 1 item by (6, 0, 0)".to_string(), "1 Element um (6; 0; 0) ziehen".to_string()), "the row is labelled from the leaf");
    assert!((object_origin(&app, &object_id)[0] - start[0] - 6.0).abs() < 1e-9, "the offset lands verbatim on the document");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_origin(&app, &object_id), start, "one undo restores the whole gesture");
}

/// 🧮️ Two gestures are two transactions — two rows with distinct refs — and one undo takes back only the
/// second: gestures never coalesce into one another.
#[semio_framework_async_macros::async_test]
async fn two_gumball_gestures_are_two_transactions() {
    let (mut app, object_id) = gumball_app().await;
    let start = object_origin(&app, &object_id);
    let (_, first) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), None).await;
    let (_, second) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 2.0, "dy": 0.0, "dz": 0.0 })), None).await;
    let (first, second) = (edit_rows(&first), edit_rows(&second));
    assert_eq!((first.len(), second.len()), (1, 1), "each gesture is its own row");
    assert_ne!(first[0].transaction.as_ref().map(|transaction| transaction.id.clone()), second[0].transaction.as_ref().map(|transaction| transaction.id.clone()), "two gestures mint two transactions");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert!((object_origin(&app, &object_id)[0] - start[0] - 1.0).abs() < 1e-9, "one undo takes back only the second gesture");
}

/// 🔄️ A gumball turn and a gumball scaling are ONE transaction each, yielding `rotate-selection` and
/// `scale-selection`, and one undo restores the exact base pose (absolute setters, never a negated angle).
#[semio_framework_async_macros::async_test]
async fn gumball_rotate_and_scale_are_one_transaction_each() {
    let (mut app, object_id) = gumball_app().await;
    let (orientation, scale) = (object_orientation(&app, &object_id), object_scale(&app, &object_id));
    let (_, turned) = dispatch_reporting(&mut app, "rotateSelection", Some(&json!({ "ids": [object_id.as_str()], "ax": 0.0, "ay": 0.0, "az": 1.0, "angle": 0.75 })), None).await;
    let rows = edit_rows(&turned);
    assert!(rows.len() == 1 && rows[0].op_lines[0].starts_with("rotate-selection") && rows[0].transaction.is_some(), "one turn, one transaction: {rows:?}");
    assert_ne!(object_orientation(&app, &object_id), orientation, "the turn moves the pose");
    dispatch(&mut app, "undo", None, None).await.expect("undo turn");
    assert_eq!(object_orientation(&app, &object_id), orientation, "one undo restores the orientation");
    let (_, grown) = dispatch_reporting(&mut app, "scaleSelection", Some(&json!({ "ids": [object_id.as_str()], "sx": 1.5, "sy": 1.0, "sz": 1.0 })), None).await;
    let rows = edit_rows(&grown);
    assert!(rows.len() == 1 && rows[0].op_lines[0].starts_with("scale-selection") && rows[0].transaction.is_some(), "one scaling, one transaction: {rows:?}");
    dispatch(&mut app, "undo", None, None).await.expect("undo scaling");
    assert_eq!(object_scale(&app, &object_id), scale, "one undo restores the scale");
}

/// 🧲️ A gumball drag of an attracting object re-solves the attraction graph inside the SAME transaction: on
/// Nakagin (which ships no attraction, so the law first attracts the compatible door pair `25b0dba0-…:link` →
/// `5f0266bc-…:sl0_d0`) the object the attraction hangs off the dragged one is re-placed from it — one row, one
/// `drag-selection` op whose leaf carries the follower — and one undo restores both poses exactly.
#[semio_framework_async_macros::async_test]
async fn a_gumball_drag_carries_its_attracted_objects_in_the_same_transaction() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin");
    dispatch(&mut app, "createAttraction", Some(&json!({ "attracting": "25b0dba0-8f81-423a-94a1-b911a6031010:link", "attracted": "5f0266bc-856b-4ef2-9eb0-16ef5e1fb952:sl0_d0" })), None).await.expect("attract the compatible door pair");
    let scene_snapshot = puzzle3d_scene_snapshot_from_document(app.snapshot().expect("typed app snapshot").typed());
    let owner = |full_id: &str| full_id.split(':').next().unwrap_or_default().to_string();
    let (parent, child) = scene_snapshot
        .attractions
        .iter()
        .map(|attraction| (owner(&attraction.attracting), owner(&attraction.attracted)))
        .find(|(parent, child)| parent != child && scene_snapshot.objects.iter().any(|object| &object.id == child && !object.locked) && scene_snapshot.objects.iter().any(|object| &object.id == parent && !object.locked))
        .expect("Nakagin carries an attraction between two unlocked objects");
    let (parent_start, child_start) = (object_origin(&app, &parent), object_origin(&app, &child));
    let (result, settled) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [parent.as_str()], "dx": 0.0, "dy": 0.0, "dz": 2.0 })), None).await;
    result.expect("the release delta commits");
    let rows = edit_rows(&settled);
    assert!(rows.len() == 1 && rows[0].transaction.is_some() && rows[0].op_lines.len() == 1 && rows[0].op_lines[0].starts_with("drag-selection"), "one gesture, one row, one parametric op: {rows:?}");
    assert!((object_origin(&app, &parent)[2] - parent_start[2] - 2.0).abs() < 1e-9, "the dragged object lifts by the offset");
    assert_ne!(object_origin(&app, &child), child_start, "the attracted object follows its moved parent");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!((object_origin(&app, &parent), object_origin(&app, &child)), (parent_start, child_start), "one undo restores the dragged object AND its follower");
}

/// 🧯️ A motionless release — the drag that went nowhere — leaves zero trace: no edit, no row, no notice.
#[semio_framework_async_macros::async_test]
async fn a_motionless_gumball_release_leaves_zero_trace() {
    let (mut app, object_id) = gumball_app().await;
    let (result, settled) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 0.0, "dy": 0.0, "dz": 0.0 })), None).await;
    let result = result.expect("a motionless release completes");
    let noticed = result.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. }));
    assert!(edit_rows(&settled).is_empty() && result.mutations.is_empty() && !noticed, "nothing moved, nothing recorded: {}", history_row_labels(&settled).join(" | "));
}

/// 🛠️ LAW (audit Z4) — the World3d host corpus' LOCAL one-shot cases (`🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json`,
/// `live: false`, the protocol puzzle 3d speaks: one net delta on release, no stream phase): every verb dispatch the host
/// owes for a scripted gesture, sent with its exact wire args (the targets swapped for the scene_snapshot's own object), publishes
/// exactly the case's guest edits and moves the object by its offset; a gesture that dispatches nothing publishes nothing.
#[semio_framework_async_macros::async_test]
async fn the_world3d_local_gumball_cases_land_as_their_guest_edits() {
    let corpus = parse(std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🛠️gumball-live-protocol.json")).expect("committed test data reads").as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the protocol corpus parses");
    let cases: Vec<&Value> = corpus.get("cases").and_then(Value::as_array).expect("cases").iter().filter(|case| case.get("live").and_then(Value::as_bool) == Some(false)).collect();
    assert_eq!(cases.len(), 4, "the corpus carries the four local one-shot cases");
    for case in cases {
        let name = case.get("name").and_then(Value::as_str).expect("name");
        let (mut app, object_id) = gumball_app().await;
        let before = object_origin(&app, &object_id);
        let mut edits = 0;
        for dispatch in case.get("steps").and_then(Value::as_array).expect("steps").iter().filter_map(|step| step.get("dispatch")).filter(|dispatch| !dispatch.is_null()) {
            let action = dispatch.get("action").and_then(Value::as_str).expect("action");
            let mut args = dispatch.get("args").cloned().expect("args");
            args.as_object_mut().expect("object args").insert("ids", json!([object_id.as_str()]));
            let (result, settled) = dispatch_reporting(&mut app, action, Some(&args), None).await;
            result.unwrap_or_else(|fault| panic!("{name}: {action} dispatches from the host's wire args: {fault:?}"));
            edits += edit_rows(&settled).len();
        }
        let guest = case.get("guest");
        let expected_edits = guest.and_then(|guest| guest.get("edits")).and_then(Value::as_u64).unwrap_or(0);
        let offset: Vec<f64> = guest.and_then(|guest| guest.get("offset")).and_then(Value::as_array).map_or_else(|| vec![0.0; 3], |values| values.iter().filter_map(Value::as_f64).collect());
        let after = object_origin(&app, &object_id);
        assert_eq!(edits as u64, expected_edits, "{name}: the published edits");
        assert!((0..3).all(|axis| (after[axis] - before[axis] - offset[axis]).abs() < 1e-9), "{name}: the object's offset {:?} -> {:?}, expected {offset:?}", before, after);
    }
}

//#endregion 🔖️Gumball

//#region 🔖️HistoryEditReferences
/// 🪧️ N3 (design §16.4): a history-edit reference chip names a puzzle 3d entity as the outliner does — an unlabelled
/// object by its kind's catalog label, a vortex as `<object> · <vortex>`, an attraction as `<object> → <object>` — and
/// leaves target volumes, undeclared kinds and unknown ids to the framework's generic `<Kind> <short id>`.
#[test]
fn history_edit_reference_chips_name_entities_as_the_outliner_does() {
    use semio_s_artifact_puzzle_3d_model::io::text::snapshot::parse_dsl;
    let mut snapshot = parse_dsl(&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio")).expect("committed example reads")).expect("the example parses");
    let object = snapshot.objects[0].id.clone();
    let vortex = puzzle3d_vortex_full_id(&object, &snapshot.objects[0].vortices[0].id);
    let vortex_label = snapshot.objects[0].vortices[0].label.clone().expect("the example labels its vortices");
    let kind = snapshot.objects[0].object_kind.clone().expect("the seed names its kind");
    snapshot.objects[0].label = None;
    snapshot.meta.kind_catalogs.as_mut().expect("the example ships its kind catalogs").objects.iter_mut().find(|entry| entry.id == kind).expect("the seed's kind is catalogued").label = "Forest Left".into();
    snapshot.attractions.push(semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction { id: "attraction-chip".into(), attracting: vortex.clone(), attracted: vortex.clone(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
    let chip = |kinds: &[&str], id: &str| puzzle3d_entity_label(&snapshot, &kinds.iter().map(|kind| kind.to_string()).collect::<Vec<_>>(), id).map(|label| label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string());
    assert_eq!(chip(&[PUZZLE3D_GRANULARITY_OBJECT, PUZZLE3D_GRANULARITY_TARGET_VOLUME], &object).as_deref(), Some("Forest Left"), "an unlabelled object reads its kind's catalog label");
    assert_eq!(chip(&[PUZZLE3D_GRANULARITY_VORTEX], &vortex), Some(format!("Forest Left \u{b7} {vortex_label}")), "a vortex reads its object and its own label");
    assert_eq!(chip(&[PUZZLE3D_GRANULARITY_ATTRACTION], "attraction-chip").as_deref(), Some("Forest Left \u{2192} Forest Left"), "an attraction reads its two objects");
    assert_eq!(chip(&[PUZZLE3D_GRANULARITY_TARGET_VOLUME], &object), None, "an id outside the reference's declared kinds keeps the generic label");
    assert_eq!(chip(&[], "ghost"), None, "an unknown id keeps the generic label");
}

/// 🔗️ N3 (design §16.4): while a history edit of a gesture is open, the world paints exactly the objects its draft
/// references highlighted (one changed instance record, so the delta lane carries it) — and none once the edit exits.
#[semio_framework_async_macros::async_test]
async fn a_history_edit_draft_highlights_exactly_the_objects_it_references() {
    let (mut app, object_id) = gumball_app().await;
    let (result, settled) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), None).await;
    result.expect("the drag commits");
    let mutation_id = edit_rows(&settled).first().and_then(|row| row.mutations.first()).map(|mutation| mutation.mutation_id.clone()).expect("the drag is one editable mutation");
    let highlighted = |node: &Value| instances_of(node).into_iter().filter(|instance| instance.get("highlighted").and_then(Value::as_bool) == Some(true)).filter_map(|instance| instance.get("id").and_then(Value::as_str).map(str::to_string)).collect::<Vec<_>>();
    assert!(highlighted(&render_composite(&mut app).await).is_empty(), "nothing is highlighted before a draft opens");
    let admitted = dispatch_reserved_unsettled(&mut app, semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID, Some(&json!({ "mutationId": mutation_id })), None).await.expect("historyEditBegin admits");
    settle_reserved(&mut app, admitted).await.expect("historyEditBegin settles");
    assert_eq!(highlighted(&render_composite(&mut app).await), vec![object_id.clone()], "the draft's target is painted highlighted");
    let admitted = dispatch_reserved_unsettled(&mut app, semio_framework::HISTORY_EDIT_EXIT_ACTION_ID, None, None).await.expect("historyEditExit admits");
    settle_reserved(&mut app, admitted).await.expect("historyEditExit settles");
    assert!(highlighted(&render_composite(&mut app).await).is_empty(), "closing the edit clears the highlight");
}
//#endregion 🔖️HistoryEditReferences

//#region 🔖️KitInPort
/// 🔌️ The flagship `kit:in` seam: feeding a `kit.catalog` fragment shaped exactly like block3d's
/// `puzzle3d_catalog_fragment` (`objectKinds`/`vortexKinds`, camelCase) through
/// `Puzzle3dPlayApp::import_media` must normalize `objectKinds` → `objects` / `vortexKinds` →
/// `vortices` and, after applying the returned operations, land that object kind inside
/// `meta.kind_catalogs.objects` (and the vortex kind inside `.vortices`).
#[semio_framework_async_macros::async_test]
async fn kit_in_import_media_upserts_object_and_vortex_kinds_into_meta_kind_catalogs() {
    let projection = Puzzle3dPlayApp::initial_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);

    let fragment = json!({
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

    let emit = Puzzle3dPlayApp::import_media("kit:in", &media, &doc).expect("kit:in import_media succeeds");
    assert!(!emit.artifact_mutations.is_empty(), "importing a non-empty fragment must emit real operations");

    let mut next_projection = projection.typed().clone();
    for operation in &emit.artifact_mutations {
        next_projection = protocol::apply_diff(protocol::Mutation::<semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot>::diff(operation, &next_projection).diff(), &next_projection).expect("valid mutation diff");
    }

    let next_projection = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&next_projection));
    let objects = next_projection.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("objects catalog present");
    assert!(objects.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")), "the imported object kind must appear in meta.kind_catalogs.objects");
    let capsule = objects.iter().find(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).unwrap();
    assert_eq!(capsule.pointer("/representations/0/url").and_then(Value::as_str), Some("/mesh/capsule.glb"));
    assert_eq!(capsule.pointer("/vortices/0/vortexKind").and_then(Value::as_str), Some("door"), "the per-object vortex template keeps its vortexKind after normalization");

    let vortices = next_projection.pointer("/meta/kindCatalogs/vortices").and_then(Value::as_array).expect("vortices catalog present");
    assert!(vortices.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some("door")), "the imported vortex kind must appear in meta.kind_catalogs.vortices");

    let compatibility = next_projection.pointer("/meta/kindCompatibility").and_then(Value::as_array).expect("kind compatibility present");
    assert!(compatibility.iter().any(|entry| entry.get("source").and_then(Value::as_str) == Some("door") && entry.get("target").and_then(Value::as_str) == Some("door")));
}

/// 🔁️ Re-importing the SAME fragment (a second producer edge, or a redelivered message on a
/// `multiplicity: Many` port) must upsert idempotently — no duplicate rows.
#[semio_framework_async_macros::async_test]
async fn kit_in_import_media_is_idempotent_on_repeated_delivery() {
    let projection = Puzzle3dPlayApp::initial_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let mut current = projection.typed().clone();

    let fragment = json!({
        "objectKinds": [{ "id": "capsule", "name": "capsule", "label": "Capsule", "meshUrl": "/mesh/capsule.glb", "vortices": [] }],
        "vortexKinds": [],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

    for _ in 0..2 {
        let doc_projection = Puzzle3dPlaySnapshot::new(current.clone());
        let doc = ArtifactView::new(&doc_projection, &history);
        let emit = Puzzle3dPlayApp::import_media("kit:in", &media, &doc).expect("kit:in import_media succeeds");
        for operation in &emit.artifact_mutations {
            current = protocol::apply_diff(protocol::Mutation::<semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot>::diff(operation, &current).diff(), &current).expect("valid mutation diff");
        }
    }

    let current = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&current));
    let objects = current.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("objects catalog present");
    assert_eq!(objects.iter().filter(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).count(), 1, "repeated delivery of the same fragment must upsert, never duplicate");
}

#[semio_framework_async_macros::async_test]
async fn kit_in_port_is_declared_on_the_app_io() {
    let io = Puzzle3dPlayApp::io().expect("puzzle3d declares an AppIo");
    let port = io.ports.iter().find(|port| port.id == "kit:in").expect("kit:in port declared");
    assert_eq!(port.kind_id.as_deref(), Some("kit.catalog"));
    assert_eq!(port.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
    assert!(matches!(port.multiplicity, PortMultiplicity::Many));
}
//#endregion 🔖️KitInPort

//#region 🔖️Convergence
/// 🧪️ Definitional convergence proof: two instances on one backbone make DISJOINT object edits and,
/// after exchanging operations, both converge to contain BOTH objects — impossible under
/// whole-document `setSnapshot` snapshots, which would clobber one side.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_object_edits_via_backbone() {
    use store::os_store::backbone::MemoryBackbone;
    let mut instance_a = app().await;
    let mut instance_b = app().await;
    let seeded = object_count(&instance_a);
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://puzzle3d-convergence", "mem://puzzle3d-convergence").await;
    instance_a.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_b)).await.expect("attach b");

    dispatch(&mut instance_a, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("a adds object");
    dispatch(&mut instance_b, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("b adds object");

    // A neutral history action always calls store.dispatch(), which pumps inbound operations first.
    dispatch(&mut instance_a, "commitCheckpoint", None, None).await.expect("pump a");
    dispatch(&mut instance_b, "commitCheckpoint", None, None).await.expect("pump b");

    assert_eq!(object_count(&instance_a), seeded + 2, "instance A must contain both objects");
    assert_eq!(object_count(&instance_b), seeded + 2, "instance B must contain both objects");
    let _ = Puzzle3dCamera::default();
}

//#endregion 🔖️Convergence

//#region 🔖️Close
/// 🧹️ ticket 26/09/02/PUZZLE-3D-END-TO-END: the app-close contract, stated exactly once and asserted
/// by a test rather than by a destructor. `VcsArtifactApp::close_step` walks seven owned lanes in a
/// fixed order (`document-store`, `config-store`, `draft-store`, `presence-store`, `transient-store`,
/// window-transient, `interaction-store`) and every lane is mandatory: `drive_artifact_owned_disposer`
/// faults `interactive-job.close-owned-disposer-missing` the moment `A::build_<lane>_disposer()`
/// returned `None`. Until that holds for every lane, no puzzle3d instance can ever reach its
/// terminal-empty witness — in a test process OR in a host, where the same `close_step` runs on
/// shutdown and the framework-installed `ArtifactStoreCursorDisposer` members then panic in `Drop`.
#[semio_framework_async_macros::async_test]
async fn fixture_app_reaches_its_terminal_empty_close_witness() {
    match close_witness(app().await) {
        Ok(true) => {}
        Ok(false) => panic!("close_step reported Complete without reaching terminal-empty ownership"),
        Err(fault) => panic!("every owned close lane must supply its bounded disposer, got {fault:?}"),
    }
}
//#endregion 🔖️Close
