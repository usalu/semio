//! 📋️ Laws for the `Puzzle2dClipboardJob` route: the copy→paste round trip on both shipped examples,
//! cut as one history edit, and the locked refusal.

use crate::editor::puzzle2d::snapshot::{PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, PUZZLE2D_GRANULARITY_NODE, PUZZLE2D_INTERACTION_DOMAIN, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID};
use crate::editor::puzzle2d::recorder::puzzle2d_paste_operations_on;
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;
use semio_framework_plugin::kernel::{Effect, PastePlacement};
use semio_framework_plugin::InteractionTarget;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use semio_framework_pack_json::{json, Value};
use std::collections::{HashMap, HashSet};
#[allow(unused_imports)]
use crate::editor::puzzle2d::snapshot::{puzzle2d_scene_for, puzzle2d_scene_with, PUZZLE2D_GENERIC_TOOL_IDS, PUZZLE2D_RETAINED_TOOL_IDS, PUZZLE2D_ADD_NODE_DIALOG_ID, PUZZLE2D_GRANULARITY_EDGE, PUZZLE2D_GRANULARITY_HANDLE, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, Puzzle2dInteractionSnapshot, Puzzle2dScene, puzzle2d_active_utility, puzzle2d_addresses_locked_entity, puzzle2d_entity_hidden, puzzle2d_entity_label, puzzle2d_interaction_definition, puzzle2d_node_kind_arg, puzzle2d_select_scope, puzzle2d_selected_node_ids, puzzle2d_selection_is_locked, puzzle2d_window_and_engagements_scope, puzzle2d_window_only_scope};
#[allow(unused_imports)]
use crate::editor::puzzle2d::engine::board_host::drain_board_events_json;
#[allow(unused_imports)]
use crate::editor::puzzle2d::recorder::{puzzle2d_copy_fragment_from, puzzle2d_cut_operations_from};
#[allow(unused_imports)]
use crate::editor::puzzle2d::commands::set_active_example;
#[allow(unused_imports)]
use crate::editor::puzzle2d::config::{Puzzle2dConfig, Puzzle2dConfigMutation};
#[allow(unused_imports)]
use crate::editor::puzzle2d::engine::{BoardHost, Point};
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit;
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit::tools::fill;
#[allow(unused_imports)]
use crate::editor::puzzle2d::precompute::fill as fill_run;
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::{area_brush as area_brush_utility, brush as brush_utility, select as select_utility};
#[allow(unused_imports)]
use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
#[allow(unused_imports)]
use crate::editor::puzzle2d::panels::{artifact, catalogue, inspection, settings};
#[allow(unused_imports)]
use crate::editor::puzzle2d::presence::{Puzzle2dPresence, Puzzle2dPresenceMutation};
#[allow(unused_imports)]
use crate::editor::puzzle2d::window::{self, Puzzle2dWindowConfig, Puzzle2dWindowTransient};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
#[allow(unused_imports)]
use semio_framework::kernel::UiDirtyScope;
#[allow(unused_imports)]
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment};
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgDef;
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgOption;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ActionKind;
#[allow(unused_imports)]
use semio_framework_plugin::ActionRef;
#[allow(unused_imports)]
use semio_framework_plugin::AppIo;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactEditor;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactPresentation;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolFactoryRegistry;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationContract;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationLane;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::DialogDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::DraftView;
#[allow(unused_imports)]
use semio_framework_plugin::Editor;
#[allow(unused_imports)]
use semio_framework_plugin::EditorApp;
#[allow(unused_imports)]
use semio_framework_plugin::Emit;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_plugin::HierarchyProvider;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionRef;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionVerb;
#[allow(unused_imports)]
use semio_framework_plugin::InteractiveJobClassification;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::MediaPortDirection;
#[allow(unused_imports)]
use semio_framework_plugin::MediaPortSpec;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraft;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_framework_plugin::PortMultiplicity;
#[allow(unused_imports)]
use semio_framework_plugin::ToolFactoryKey;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactory;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactoryError;
#[allow(unused_imports)]
use semio_framework_plugin::WindowEngagement;
#[allow(unused_imports)]
use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use semio_framework_plugin::app::InteractionView;
#[allow(unused_imports)]
use std::collections::BTreeSet;
#[allow(unused_imports)]
use semio_framework_2d::compute::EngineHandles;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest};
#[allow(unused_imports)]
use crate::editor::puzzle2d::kit::*;
#[allow(unused_imports)]
use crate::editor::puzzle2d::retained::*;
#[allow(unused_imports)]
use crate::editor::puzzle2d::session::*;
use crate::editor::puzzle2d::*;
use crate::editor::puzzle2d::testkit::*;
use semio_framework_plugin::InvocationResult;

fn node_ids(app: &Puzzle2dApp) -> Vec<String> {
    board_snapshot_nodes(&fixture_of(app)).iter().filter_map(|node| node.get("id").and_then(Value::as_str).map(str::to_string)).collect()
}

fn edge_count(app: &Puzzle2dApp) -> usize {
    board_snapshot_edges(&fixture_of(app)).len()
}

/// 🕹️ Selects exactly these ids at node granularity — the multi-select `select_id` has no shape for.
fn select_ids(app: &mut Puzzle2dApp, ids: &[String]) {
    let targets: Vec<InteractionTarget> = ids.iter().map(|id| InteractionTarget { granularity: PUZZLE2D_GRANULARITY_NODE.into(), id: id.clone() }).collect();
    let targets = semio_framework_pack_json::to_json_string(&targets);
    dispatch(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE2D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None).expect("select ids");
}

fn clipboard_fragment(result: &InvocationResult) -> semio_framework_plugin::kernel::ClipboardFragment {
    result
        .requested_effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ClipboardWrite { fragment } => Some(fragment.clone()),
            _ => None,
        })
        .expect("copy must emit exactly one ClipboardWrite")
}

fn paste_args(fragment: &semio_framework_plugin::kernel::ClipboardFragment) -> Value {
    json!({ "fragment": semio_framework_pack_json::from_dsl_value(semio_framework_value::ToValue::to_value(&(fragment))) })
}

fn notices(result: &InvocationResult) -> Vec<String> {
    result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        })
        .collect()
}

/// 📋️ Concrete forest round trip: copy the seed node, paste it, and the clone carries fresh node AND
/// handle ids at the default offset while the source stays exactly where it was.
#[test]
fn copy_then_paste_round_trips_the_concrete_forest_seed() {
    let mut app = concrete_forest_app();
    let source_id = first_node_id(&app);
    let before = fixture_of(&app);
    let source_node = board_snapshot_nodes(&before)[0].clone();
    select_ids(&mut app, &[source_id.clone()]);
    let fragment = clipboard_fragment(&dispatch(&mut app, "copy", None, None).expect("copy"));
    assert_eq!(fragment.schema, PUZZLE2D_CLIPBOARD_SCHEMA, "the fragment must carry this artifact's own clipboard schema");
    let pasted = dispatch(&mut app, "paste", Some(&paste_args(&fragment)), None).expect("paste");
    assert!(committed_edits(&pasted) > 0, "paste must commit one document edit");
    let after = fixture_of(&app);
    let nodes = board_snapshot_nodes(&after);
    assert_eq!(nodes.len(), 2, "paste clones the copied node");
    let clone = nodes.iter().find(|node| node.get("id").and_then(Value::as_str) != Some(source_id.as_str())).expect("clone node");
    let clone_id = clone.get("id").and_then(Value::as_str).expect("clone id");
    assert_ne!(clone_id, source_id, "the clone mints a fresh id");
    assert_eq!(clone.get("x").and_then(Value::as_f64), source_node.get("x").and_then(Value::as_f64).map(|x| x + PUZZLE2D_PASTE_OFFSET), "the clone lands at the default paste offset");
    let source_handles = source_node.get("handles").and_then(Value::as_array).expect("source handles").len();
    let clone_handles = clone.get("handles").and_then(Value::as_array).expect("clone handles");
    assert_eq!(clone_handles.len(), source_handles, "the clone carries every handle of its source");
    assert!(clone_handles.iter().all(|handle| handle.get("id").and_then(Value::as_str).is_some_and(|id| id.starts_with(clone_id))), "every cloned handle id is re-minted under the clone: {clone_handles:?}");
    close_app(&mut app);
}

/// 🏗️ A 12-node Nakagin subgraph round trip: the copy carries the edges BETWEEN the copied nodes and
/// the paste restores that exact topology alongside the originals.
#[test]
fn copy_then_paste_restores_a_twelve_node_nakagin_subgraph() {
    let mut app = app();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    let snapshot = fixture_of(&app);
    // 🔎️ A connected dozen: walk the edge list and take the first twelve distinct endpoint owners, so
    // the subset is guaranteed to carry internal edges rather than twelve isolated nodes.
    let owner_of: HashMap<String, String> = board_snapshot_nodes(&snapshot)
        .iter()
        .flat_map(|node| {
            let node_id = node.get("id").and_then(Value::as_str).unwrap_or_default().to_string();
            node.get("handles").and_then(Value::as_array).into_iter().flatten().filter_map(move |handle| handle.get("id").and_then(Value::as_str).map(|id| (id.to_string(), node_id.clone())))
        })
        .collect();
    let mut subset: Vec<String> = Vec::new();
    for edge in board_snapshot_edges(&snapshot) {
        for key in ["source", "target"] {
            let Some(owner) = edge.get(key).and_then(Value::as_str).and_then(|id| owner_of.get(id)) else { continue };
            if subset.len() < 12 && !subset.contains(owner) {
                subset.push(owner.clone());
            }
        }
    }
    assert_eq!(subset.len(), 12, "nakagin must offer twelve connected nodes to copy");
    let subset_handles: HashSet<&str> = owner_of.iter().filter(|(_, owner)| subset.contains(owner)).map(|(handle, _)| handle.as_str()).collect();
    let internal_edges = board_snapshot_edges(&snapshot)
        .iter()
        .filter(|edge| {
            let endpoint = |key: &str| edge.get(key).and_then(Value::as_str).is_some_and(|id| subset_handles.contains(id));
            endpoint("source") && endpoint("target")
        })
        .count();
    assert!(internal_edges > 0, "the copied dozen must carry internal edges");
    let (nodes_before, edges_before) = (board_snapshot_nodes(&snapshot).len(), board_snapshot_edges(&snapshot).len());
    select_ids(&mut app, &subset);
    let fragment = clipboard_fragment(&dispatch(&mut app, "copy", None, None).expect("copy"));
    dispatch(&mut app, "paste", Some(&paste_args(&fragment)), None).expect("paste");
    let after = fixture_of(&app);
    assert_eq!(board_snapshot_nodes(&after).len(), nodes_before + 12, "paste restores every copied node");
    assert_eq!(board_snapshot_edges(&after).len(), edges_before + internal_edges, "paste restores exactly the edges between the copied nodes");
    close_app(&mut app);
}

/// ✂️ Cut removes the selection and its incident edges as ONE history edit — one undo restores the
/// whole topology.
#[test]
fn cut_removes_and_one_undo_restores_the_topology() {
    let mut app = app();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    let ids = node_ids(&app);
    let (nodes_before, edges_before) = (ids.len(), edge_count(&app));
    let victim = ids.first().cloned().expect("a nakagin node");
    select_ids(&mut app, &[victim.clone()]);
    let cut = dispatch(&mut app, "cut", None, None).expect("cut");
    assert!(!clipboard_fragment(&cut).dsl_text.is_empty(), "cut writes the clipboard before it deletes");
    let after_cut = fixture_of(&app);
    assert_eq!(board_snapshot_nodes(&after_cut).len(), nodes_before - 1, "cut removes the selected node");
    assert!(board_snapshot_edges(&after_cut).len() <= edges_before, "cut removes the incident edges too");
    dispatch(&mut app, "undo", None, None).expect("undo");
    let restored = fixture_of(&app);
    assert_eq!(board_snapshot_nodes(&restored).len(), nodes_before, "ONE undo restores every cut node");
    assert_eq!(board_snapshot_edges(&restored).len(), edges_before, "ONE undo restores every cut edge");
    close_app(&mut app);
}

/// 🔒️ A locked node refuses the cut with a real sentence and leaves the document untouched.
#[test]
fn cut_refuses_a_locked_node_with_a_notice() {
    let mut app = concrete_forest_app();
    let victim = first_node_id(&app);
    select_ids(&mut app, &[victim.clone()]);
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "flag": "locked", "value": true })), None).expect("lock");
    let before = fixture_of(&app);
    let refused = dispatch(&mut app, "cut", None, None).expect("cut a locked node");
    assert_eq!(committed_edits(&refused), 0, "a locked cut must emit no edit");
    assert_eq!(notices(&refused).len(), 1, "a locked cut raises exactly one notice: {:?}", refused.requested_effects);
    assert_eq!(fixture_of(&app), before, "a locked cut leaves the document byte-identical");
    close_app(&mut app);
}

/// 🚪️ A fragment from another app's media class is refused rather than pasted.
#[test]
fn paste_refuses_a_foreign_media_type() {
    let snapshot = Puzzle2dPlaySnapshot::new(semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(&json!({ "schema": PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, "nodes": [], "edges": [] }))).expect("typed fixture admits"));
    let foreign = semio_framework_plugin::kernel::ClipboardFragment {
        schema: PUZZLE2D_CLIPBOARD_SCHEMA.into(),
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Design },
        dsl_text: json!({ "nodes": [{ "id": "n", "x": 0.0, "y": 0.0 }], "edges": [] }).to_string(),
        pack_bytes: None,
        source_app: PUZZLE2D_PLAY_CONTROLLER_ID.into(),
        label: "1 nodes".into(),
    };
    assert!(puzzle2d_paste_operations_on(&snapshot, &foreign, &PastePlacement::default()).is_err(), "a 3d fragment must never paste into a 2d board");
}
