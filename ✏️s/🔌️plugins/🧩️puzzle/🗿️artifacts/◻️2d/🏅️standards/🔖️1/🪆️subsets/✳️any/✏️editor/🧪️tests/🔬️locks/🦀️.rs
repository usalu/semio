//! 🔒️ Laws for the ONE lock promise: a locked node, handle, edge or target region refuses delete, the
//! board drag, the rotate ring, the three transform verbs and an inspector patch — each with exactly
//! one visible sentence, no document edit and no fault. A selection transform refuses at the select tool
//! only when EVERY target is locked; a mixed one commits and its leaf reports the locked rest. The 2026-09-17 ◻️2d battery measured
//! `deleteSelection` erasing 12 entities whose inspector flag read `locked true` while the very same
//! node's drag was (silently) refused: one verb honoured the lock, the other did not, and neither
//! said anything.

use crate::editor::puzzle2d::snapshot::{PUZZLE2D_GRANULARITY_NODE, puzzle2d_addresses_locked_entity};
use semio_framework_plugin::kernel::Effect;
use semio_framework_pack_json::{json, Value};
#[allow(unused_imports)]
use crate::editor::puzzle2d::snapshot::{puzzle2d_scene_for, puzzle2d_scene_with, PUZZLE2D_GENERIC_TOOL_IDS, PUZZLE2D_RETAINED_TOOL_IDS, PUZZLE2D_ADD_NODE_DIALOG_ID, PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, PUZZLE2D_GRANULARITY_EDGE, PUZZLE2D_GRANULARITY_HANDLE, PUZZLE2D_INTERACTION_DOMAIN, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, Puzzle2dInteractionSnapshot, Puzzle2dScene, puzzle2d_active_utility, puzzle2d_entity_hidden, puzzle2d_entity_label, puzzle2d_interaction_definition, puzzle2d_node_kind_arg, puzzle2d_select_scope, puzzle2d_selected_node_ids, puzzle2d_selection_is_locked, puzzle2d_window_and_engagements_scope, puzzle2d_window_only_scope};
#[allow(unused_imports)]
use crate::editor::puzzle2d::engine::board_host::drain_board_events_json;
#[allow(unused_imports)]
use crate::editor::puzzle2d::recorder::{puzzle2d_copy_fragment_from, puzzle2d_cut_operations_from, puzzle2d_paste_operations_on};
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
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
#[allow(unused_imports)]
use semio_framework::kernel::UiDirtyScope;
#[allow(unused_imports)]
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, PastePlacement};
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
use semio_framework_plugin::InteractionTarget;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionVerb;
#[allow(unused_imports)]
use semio_framework_plugin::InteractiveJobClassification;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::MediaClass;
#[allow(unused_imports)]
use semio_framework_plugin::MediaForm;
#[allow(unused_imports)]
use semio_framework_plugin::MediaPortDirection;
#[allow(unused_imports)]
use semio_framework_plugin::MediaPortSpec;
#[allow(unused_imports)]
use semio_framework_plugin::MediaType;
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
use std::collections::{BTreeSet, HashMap, HashSet};
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

/// 🔒️ Loads the concrete forest, selects its seed node and locks it — the shared precondition of
/// every law below, answering the locked node's id.
fn locked_seed_app() -> (Puzzle2dApp, String) {
    let mut app = concrete_forest_app();
    let id = first_node_id(&app);
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &id).expect("select the seed node");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "flag": "locked", "value": true })), None).expect("lock the seed node");
    assert!(puzzle2d_addresses_locked_entity(&fixture_of(&app), &[id.clone()]), "the precondition must really be locked");
    (app, id)
}

/// 🔒️ Asserts the shape of a refusal: exactly one sentence, and a byte-identical document. The edit is
/// measured on the DOCUMENT, not on `InvocationResult.mutations` — these verbs run as owned tool jobs
/// whose committed operations reach the store through the job's completion, so `mutations` is empty for
/// a successful edit too and an assertion on it could never fail.
fn assert_refused(app: &Puzzle2dApp, before: &Value, result: &InvocationResult, verb: &str) {
    assert_eq!(notices(result).len(), 1, "{verb} on a locked entity raises exactly one notice: {:?}", result.requested_effects);
    assert_eq!(&fixture_of(app), before, "{verb} on a locked entity leaves the document byte-identical");
}

/// 🗑️ The battery's red: `deleteSelection` used to erase a locked node outright.
#[test]
fn delete_selection_refuses_a_locked_node_with_one_notice() {
    let (mut app, _id) = locked_seed_app();
    let before = fixture_of(&app);
    let refused = dispatch(&mut app, "deleteSelection", None, None).expect("delete a locked node");
    assert_refused(&app, &before, &refused, "deleteSelection");
    close_app(&mut app);
}

/// 🚀️ The three transforms share the select tool's one refusal — every target locked — so all three are
/// asserted through the same law.
#[test]
fn transform_verbs_refuse_a_locked_selection_with_one_notice() {
    for (verb, args) in [
        ("translateSelection", json!({ "dx": 10.0, "dy": 0.0 })),
        ("rotateSelection", json!({ "radians": 0.5 })),
        ("scaleSelection", json!({ "factor": 2.0 })),
    ] {
        let (mut app, _id) = locked_seed_app();
        let before = fixture_of(&app);
        let refused = dispatch(&mut app, verb, Some(&args), None).expect("transform a locked node");
        assert_refused(&app, &before, &refused, verb);
        close_app(&mut app);
    }
}

/// 🎲️ A pointer drag reaches the guest as `applyBoardEvents`, never as a transform verb: ONE gesture record per
/// release (a `drag`, a ring `rotate`), each refused at the select tool when every target it names is locked.
#[test]
fn board_gesture_records_refuse_a_locked_node_with_one_notice() {
    for (kind, record) in [
        ("drag", json!({ "gestureId": "gesture-1", "kind": "drag", "targets": ["SEED"], "dx": 120.0, "dy": 240.0, "proximity": [] })),
        ("rotate", json!({ "gestureId": "gesture-2", "kind": "rotate", "targets": ["SEED"], "pivotX": 0.0, "pivotY": 0.0, "angle": 0.75, "proximity": [] })),
    ] {
        let (mut app, id) = locked_seed_app();
        let before = fixture_of(&app);
        let events_json = semio_framework_pack_json::to_json_string(&json!([{ "name": "gesture", "payload": record }])).replace("SEED", &id);
        let refused = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": events_json })), None).expect("gesture on a locked node");
        assert_refused(&app, &before, &refused, kind);
        close_app(&mut app);
    }
}

/// 🐢️ However many locked records one flush carries, the refusal is answered ONCE, or the board disappears
/// behind a wall of toasts.
#[test]
fn a_whole_locked_gesture_batch_raises_exactly_one_notice() {
    let (mut app, id) = locked_seed_app();
    let rows: Vec<Value> = (0..3).map(|step| json!({ "name": "gesture", "payload": { "gestureId": format!("gesture-{step}"), "kind": "drag", "targets": [id], "dx": 10.0, "dy": 0.0, "proximity": [] } })).collect();
    let refused = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": semio_framework_pack_json::to_json_string(&rows) })), None).expect("drag a locked node");
    assert_eq!(notices(&refused).len(), 1, "three refused records raise one sentence, not three: {:?}", refused.requested_effects);
    close_app(&mut app);
}

/// 🩹️ An inspector stepper on a locked node refuses — but the lock row itself must stay pressable, or
/// a locked node could never be unlocked again.
#[test]
fn inspector_patch_refuses_a_locked_node_yet_the_lock_row_still_answers() {
    let (mut app, id) = locked_seed_app();
    let before = fixture_of(&app);
    let refused = dispatch(&mut app, "patchInspectorNodes", Some(&json!({ "ids": [id.clone()], "field": "x", "value": 999.0 })), None).expect("patch a locked node");
    assert_refused(&app, &before, &refused, "patchInspectorNodes");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "flag": "locked", "value": false })), None).expect("unlock the seed node");
    assert!(!puzzle2d_addresses_locked_entity(&fixture_of(&app), &[id.clone()]), "the lock row unlocks a locked node");
    let allowed = dispatch(&mut app, "patchInspectorNodes", Some(&json!({ "ids": [id.clone()], "field": "x", "value": 999.0 })), None).expect("patch an unlocked node");
    assert!(notices(&allowed).is_empty(), "an unlocked patch raises no refusal: {:?}", allowed.requested_effects);
    let patched = fixture_of(&app);
    let node = board_snapshot_nodes(&patched).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(id.as_str())).expect("the patched node").clone();
    assert_eq!(node.get("x").and_then(Value::as_f64), Some(999.0), "an unlocked patch commits its edit: {node}");
    close_app(&mut app);
}

/// 🔐️ The predicate itself: nodes, their handles, edges and target regions all answer the same lock.
#[test]
fn the_lock_predicate_answers_for_every_entity_kind() {
    let snapshot = json!({
        "nodes": [
            { "id": "free", "x": 0.0, "y": 0.0, "handles": [{ "id": "free:link", "angle": 0.0 }] },
            { "id": "bound", "x": 0.0, "y": 0.0, "locked": true, "handles": [] },
            { "id": "host", "x": 0.0, "y": 0.0, "handles": [{ "id": "host:link", "angle": 0.0, "locked": true }] }
        ],
        "edges": [{ "id": "wire", "source": "free:link", "target": "host:link", "locked": true }],
        "targetRegions": [{ "id": "zone", "x": 0.0, "y": 0.0, "width": 1.0, "height": 1.0, "locked": true }]
    });
    assert!(!puzzle2d_addresses_locked_entity(&snapshot, &[]), "an empty address list locks nothing");
    assert!(!puzzle2d_addresses_locked_entity(&snapshot, &["free".into(), "free:link".into()]), "unlocked ids are not refused");
    for locked in ["bound", "host:link", "wire", "zone"] {
        assert!(puzzle2d_addresses_locked_entity(&snapshot, &[locked.to_string()]), "{locked} must answer the lock");
    }
    assert!(puzzle2d_addresses_locked_entity(&snapshot, &["free".into(), "zone".into()]), "one locked entity refuses the whole gesture");
}
