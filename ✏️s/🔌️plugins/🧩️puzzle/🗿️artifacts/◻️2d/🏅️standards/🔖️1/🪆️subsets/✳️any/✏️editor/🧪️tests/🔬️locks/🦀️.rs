//! 🔒️ Laws for the ONE lock promise: a locked node, handle, edge or target region refuses delete, the
//! board drag, the rotate ring, the three transform verbs and an inspector patch — each with exactly
//! one visible sentence, no document edit and no fault. The 2026-09-17 ◻️2d battery measured
//! `deleteSelection` erasing 12 entities whose inspector flag read `locked true` while the very same
//! node's drag was (silently) refused: one verb honoured the lock, the other did not, and neither
//! said anything.

use super::*;
use crate::editor::puzzle2d::unit_tests::context::*;
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

/// 🚀️ The three centroid transforms share one gate, so all three are asserted through the same law.
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

/// 🎲️ A pointer drag reaches the guest as `applyBoardEvents`, never as a transform verb — the rows the
/// board streams (`nodeMove` per tick, `nodeDragEnd` on release, `nodeRotate` on ring release) each
/// carry their own ids and each is refused on its own.
#[test]
fn board_drag_rows_refuse_a_locked_node_with_one_notice() {
    for (row, events) in [
        ("nodeMove", json!([{ "name": "nodeMove", "payload": { "id": "SEED", "x": 120.0, "y": 240.0 } }])),
        ("nodeDragEnd", json!([{ "name": "nodeDragEnd", "payload": { "moves": [{ "id": "SEED", "x": 120.0, "y": 240.0 }] } }])),
        ("nodeRotate", json!([{ "name": "nodeRotate", "payload": { "radians": 0.75, "ids": ["SEED"] } }])),
    ] {
        let (mut app, id) = locked_seed_app();
        let before = fixture_of(&app);
        let events_json = serde_json::to_string(&events).expect("serialize rows").replace("SEED", &id);
        let refused = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": events_json })), None).expect("drag a locked node");
        assert_refused(&app, &before, &refused, row);
        close_app(&mut app);
    }
}

/// 🐢️ A pointer drag streams one `nodeMove` per tick. The refusal is answered ONCE per batch, or the
/// board disappears behind a wall of toasts.
#[test]
fn a_whole_locked_drag_batch_raises_exactly_one_notice() {
    let (mut app, id) = locked_seed_app();
    let rows: Vec<Value> = (0..8).map(|step| json!({ "name": "nodeMove", "payload": { "id": id, "x": 10.0 * f64::from(step), "y": 0.0 } })).collect();
    let refused = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": serde_json::to_string(&rows).expect("serialize rows") })), None).expect("drag a locked node");
    assert_eq!(notices(&refused).len(), 1, "eight refused move rows raise one sentence, not eight: {:?}", refused.requested_effects);
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
    let node = fixture_nodes(&patched).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(id.as_str())).expect("the patched node").clone();
    assert_eq!(node.get("x").and_then(Value::as_f64), Some(999.0), "an unlocked patch commits its edit: {node}");
    close_app(&mut app);
}

/// 🔐️ The predicate itself: nodes, their handles, edges and target regions all answer the same lock.
#[test]
fn the_lock_predicate_answers_for_every_entity_kind() {
    let fixture = json!({
        "nodes": [
            { "id": "free", "x": 0.0, "y": 0.0, "handles": [{ "id": "free:link", "angle": 0.0 }] },
            { "id": "bound", "x": 0.0, "y": 0.0, "locked": true, "handles": [] },
            { "id": "host", "x": 0.0, "y": 0.0, "handles": [{ "id": "host:link", "angle": 0.0, "locked": true }] }
        ],
        "edges": [{ "id": "wire", "source": "free:link", "target": "host:link", "locked": true }],
        "targetRegions": [{ "id": "zone", "x": 0.0, "y": 0.0, "width": 1.0, "height": 1.0, "locked": true }]
    });
    assert!(!puzzle2d_addresses_locked_entity(&fixture, &[]), "an empty address list locks nothing");
    assert!(!puzzle2d_addresses_locked_entity(&fixture, &["free".into(), "free:link".into()]), "unlocked ids are not refused");
    for locked in ["bound", "host:link", "wire", "zone"] {
        assert!(puzzle2d_addresses_locked_entity(&fixture, &[locked.to_string()]), "{locked} must answer the lock");
    }
    assert!(puzzle2d_addresses_locked_entity(&fixture, &["free".into(), "zone".into()]), "one locked entity refuses the whole gesture");
}
