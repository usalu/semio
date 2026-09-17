//! 🧪️ `createEdge` / `deleteEdge` laws: the compatibility gate, the occupied-handle refusal, and the
//! engagement line that drives the same arm.

use crate::editor::puzzle2d::unit_tests::context::*;
use crate::editor::puzzle2d::{fixture_edges, fixture_nodes};
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::InvocationResult;
use serde_json::{json, Value};

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

/// 🌲️ Two concrete-forest seeds side by side, and the `vN` handle ids of each.
fn two_seed_app() -> (Puzzle2dApp, String, String) {
    let mut app = concrete_forest_app();
    let source = first_node_id(&app);
    select_id(&mut app, crate::editor::puzzle2d::PUZZLE2D_GRANULARITY_NODE, &source).expect("select seed");
    dispatch(&mut app, "duplicateSelection", None, None).expect("duplicate");
    let clone = fixture_nodes(&fixture_of(&app))
        .iter()
        .filter_map(|node| node.get("id").and_then(Value::as_str))
        .find(|id| *id != source)
        .expect("clone id")
        .to_string();
    (app, source, clone)
}

/// 🔗️ A compatible pair of open handles connects, and the edge is a real document edit one undo takes back.
#[test]
fn create_edge_connects_two_compatible_open_handles() {
    let (mut app, source, clone) = two_seed_app();
    let before = fixture_edges(&fixture_of(&app)).len();
    let result = dispatch(&mut app, "createEdge", Some(&json!({ "source": format!("{source}:v0"), "target": format!("{clone}:v0") })), None).expect("createEdge");
    assert!(notices(&result).is_empty(), "a legal connect raises no notice: {:?}", result.requested_effects);
    let edges = fixture_edges(&fixture_of(&app)).to_vec();
    assert_eq!(edges.len(), before + 1, "createEdge splices exactly one edge");
    assert_eq!(edges.last().and_then(|edge| edge.get("source")).and_then(Value::as_str), Some(format!("{source}:v0").as_str()), "the edge keeps the requested direction");
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(fixture_edges(&fixture_of(&app)).len(), before, "ONE undo takes the connect back");
    close_app(&mut app);
}

/// 🚫️ `b-l` and `c-b` share no compatibility row, so the connect refuses with exactly one notice.
#[test]
fn create_edge_refuses_an_incompatible_handle_kind_pair() {
    let (mut app, source, clone) = two_seed_app();
    let before = fixture_of(&app);
    let refused = dispatch(&mut app, "createEdge", Some(&json!({ "source": format!("{source}:v0"), "target": format!("{clone}:v7") })), None).expect("createEdge");
    assert!(refused.mutations.is_empty(), "an incompatible connect emits no edit: {:?}", refused.mutations);
    assert_eq!(notices(&refused).len(), 1, "an incompatible connect raises exactly one notice: {:?}", refused.requested_effects);
    assert_eq!(fixture_of(&app), before, "an incompatible connect leaves the document byte-identical");
    close_app(&mut app);
}

/// 🔒️ A handle an edge already ends on refuses a second connection.
#[test]
fn create_edge_refuses_an_occupied_handle() {
    let (mut app, source, clone) = two_seed_app();
    dispatch(&mut app, "createEdge", Some(&json!({ "source": format!("{source}:v0"), "target": format!("{clone}:v0") })), None).expect("first connect");
    let before = fixture_of(&app);
    let refused = dispatch(&mut app, "createEdge", Some(&json!({ "source": format!("{source}:v0"), "target": format!("{clone}:v2") })), None).expect("second connect");
    assert!(refused.mutations.is_empty(), "an occupied connect emits no edit: {:?}", refused.mutations);
    assert_eq!(notices(&refused).len(), 1, "an occupied connect raises exactly one notice");
    assert_eq!(fixture_of(&app), before, "an occupied connect leaves the document byte-identical");
    close_app(&mut app);
}

/// 💔️ `deleteEdge` is the exact inverse route.
#[test]
fn delete_edge_drops_the_named_edge() {
    let (mut app, source, clone) = two_seed_app();
    dispatch(&mut app, "createEdge", Some(&json!({ "source": format!("{source}:v0"), "target": format!("{clone}:v0") })), None).expect("connect");
    let id = fixture_edges(&fixture_of(&app)).last().and_then(|edge| edge.get("id")).and_then(Value::as_str).expect("edge id").to_string();
    dispatch(&mut app, "deleteEdge", Some(&json!({ "id": id })), None).expect("deleteEdge");
    assert!(fixture_edges(&fixture_of(&app)).is_empty(), "deleteEdge removes the named edge");
    close_app(&mut app);
}

/// ⌨️ The engagement grammar's `connect <handle> <handle>` drives the same arm — and keeps the
/// operands' original case, which a lowercased line would have destroyed.
#[test]
fn engagement_connect_line_creates_the_same_edge() {
    let (mut app, source, clone) = two_seed_app();
    dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": format!("connect {source}:v0 {clone}:v0") })), None).expect("engagement connect");
    assert_eq!(fixture_edges(&fixture_of(&app)).len(), 1, "the engagement line connects the named handles");
    close_app(&mut app);
}
