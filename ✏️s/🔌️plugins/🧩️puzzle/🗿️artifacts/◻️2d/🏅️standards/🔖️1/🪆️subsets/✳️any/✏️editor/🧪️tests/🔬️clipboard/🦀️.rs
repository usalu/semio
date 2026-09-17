//! 📋️ Laws for the `Puzzle2dClipboardJob` route: the copy→paste round trip on both shipped examples,
//! cut as one history edit, and the locked refusal.

use super::*;
use crate::editor::puzzle2d::unit_tests::context::*;
use semio_framework_plugin::InvocationResult;

fn node_ids(app: &Puzzle2dApp) -> Vec<String> {
    fixture_nodes(&fixture_of(app)).iter().filter_map(|node| node.get("id").and_then(Value::as_str).map(str::to_string)).collect()
}

fn edge_count(app: &Puzzle2dApp) -> usize {
    fixture_edges(&fixture_of(app)).len()
}

/// 🕹️ Selects exactly these ids at node granularity — the multi-select `select_id` has no shape for.
fn select_ids(app: &mut Puzzle2dApp, ids: &[String]) {
    let targets: Vec<InteractionTarget> = ids.iter().map(|id| InteractionTarget { granularity: PUZZLE2D_GRANULARITY_NODE.into(), id: id.clone() }).collect();
    let targets = serde_json::to_string(&targets).expect("serialize targets");
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
    json!({ "fragment": serde_json::to_value(fragment).expect("serialize fragment") })
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
    let source_node = fixture_nodes(&before)[0].clone();
    select_ids(&mut app, &[source_id.clone()]);
    let fragment = clipboard_fragment(&dispatch(&mut app, "copy", None, None).expect("copy"));
    assert_eq!(fragment.schema, PUZZLE2D_CLIPBOARD_SCHEMA, "the fragment must carry this artifact's own clipboard schema");
    let pasted = dispatch(&mut app, "paste", Some(&paste_args(&fragment)), None).expect("paste");
    assert!(!pasted.mutations.is_empty(), "paste must commit one document edit");
    let after = fixture_of(&app);
    let nodes = fixture_nodes(&after);
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
    let fixture = fixture_of(&app);
    // 🔎️ A connected dozen: walk the edge list and take the first twelve distinct endpoint owners, so
    // the subset is guaranteed to carry internal edges rather than twelve isolated nodes.
    let owner_of: HashMap<String, String> = fixture_nodes(&fixture)
        .iter()
        .flat_map(|node| {
            let node_id = node.get("id").and_then(Value::as_str).unwrap_or_default().to_string();
            node.get("handles").and_then(Value::as_array).into_iter().flatten().filter_map(move |handle| handle.get("id").and_then(Value::as_str).map(|id| (id.to_string(), node_id.clone())))
        })
        .collect();
    let mut subset: Vec<String> = Vec::new();
    for edge in fixture_edges(&fixture) {
        for key in ["source", "target"] {
            let Some(owner) = edge.get(key).and_then(Value::as_str).and_then(|id| owner_of.get(id)) else { continue };
            if subset.len() < 12 && !subset.contains(owner) {
                subset.push(owner.clone());
            }
        }
    }
    assert_eq!(subset.len(), 12, "nakagin must offer twelve connected nodes to copy");
    let subset_handles: HashSet<&str> = owner_of.iter().filter(|(_, owner)| subset.contains(owner)).map(|(handle, _)| handle.as_str()).collect();
    let internal_edges = fixture_edges(&fixture)
        .iter()
        .filter(|edge| {
            let endpoint = |key: &str| edge.get(key).and_then(Value::as_str).is_some_and(|id| subset_handles.contains(id));
            endpoint("source") && endpoint("target")
        })
        .count();
    assert!(internal_edges > 0, "the copied dozen must carry internal edges");
    let (nodes_before, edges_before) = (fixture_nodes(&fixture).len(), fixture_edges(&fixture).len());
    select_ids(&mut app, &subset);
    let fragment = clipboard_fragment(&dispatch(&mut app, "copy", None, None).expect("copy"));
    dispatch(&mut app, "paste", Some(&paste_args(&fragment)), None).expect("paste");
    let after = fixture_of(&app);
    assert_eq!(fixture_nodes(&after).len(), nodes_before + 12, "paste restores every copied node");
    assert_eq!(fixture_edges(&after).len(), edges_before + internal_edges, "paste restores exactly the edges between the copied nodes");
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
    assert_eq!(fixture_nodes(&after_cut).len(), nodes_before - 1, "cut removes the selected node");
    assert!(fixture_edges(&after_cut).len() <= edges_before, "cut removes the incident edges too");
    dispatch(&mut app, "undo", None, None).expect("undo");
    let restored = fixture_of(&app);
    assert_eq!(fixture_nodes(&restored).len(), nodes_before, "ONE undo restores every cut node");
    assert_eq!(fixture_edges(&restored).len(), edges_before, "ONE undo restores every cut edge");
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
    assert!(refused.mutations.is_empty(), "a locked cut must emit no edit: {:?}", refused.mutations);
    assert_eq!(notices(&refused).len(), 1, "a locked cut raises exactly one notice: {:?}", refused.requested_effects);
    assert_eq!(fixture_of(&app), before, "a locked cut leaves the document byte-identical");
    close_app(&mut app);
}

/// 🚪️ A fragment from another app's media class is refused rather than pasted.
#[test]
fn paste_refuses_a_foreign_media_type() {
    let fixture = json!({ "schema": PUZZLE2D_FIXTURE_SCHEMA, "nodes": [], "edges": [] });
    let foreign = semio_framework_plugin::kernel::ClipboardFragment {
        schema: PUZZLE2D_CLIPBOARD_SCHEMA.into(),
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Design },
        dsl_text: json!({ "nodes": [{ "id": "n", "x": 0.0, "y": 0.0 }], "edges": [] }).to_string(),
        pack_bytes: None,
        source_app: PUZZLE2D_PLAY_CONTROLLER_ID.into(),
        label: "1 nodes".into(),
    };
    assert!(puzzle2d_paste_operations_on(&fixture, &foreign, &PastePlacement::default()).is_err(), "a 3d fragment must never paste into a 2d board");
}
