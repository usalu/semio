use crate::editor::note::unit_tests::context::{note_app, render as render_body};
use crate::editor::note::NOTE_PLAY_BODY_ARTIFACT as BODY_ARTIFACT;
use semio_framework_plugin::PluginApp;

/// 🩹️ Pre-existing bug fixed here (confirmed via `git log --date=iso`: `SetActiveExample`'s
/// `reset_document_effect`/`Effect::LoadDocument` conversion — and this very test — both
/// predate this ticket's dispatch to note, unrelated to composition). Dispatching a command only
/// ever RETURNS a `Effect::LoadDocument` as data for a real host to re-apply; `dispatch_typed`
/// never loops it back into the same app instance, so `app.snapshot()`/subsequent `render()` never
/// reflected it — this assertion could never have passed as originally written, on ANY content.
/// Fixed the same way writer's own `app_with_jack()` and cad's `two_instances_converge_…` tests
/// already do: call `PluginApp::load_document_pack` directly, the same technique a real host uses
/// when it receives the effect.
#[semio_framework_async_macros::async_test]
async fn renders_document_tree() {
    let mut app = note_app().await;
    let document = crate::schema::semio_example_snapshot();
    // ♻️ The seed envelope is printed from inside the owner-installing store guard
    // (`🚪️io/📸️snapshot/💾️binary`), which walks the bounded close loop on drop. A bare
    // `create_document_envelope` handed straight to `print_document_pack` and then dropped asserts
    // `artifact envelope terminal shell reached Drop before its app-owned bounded retirement
    // authority detached every nested owner`, which is what this law used to die of.
    let files = {
        let seed = crate::standards::v1::subsets::any::io::snapshot::binary::new_note_store(store::create_document_envelope::<crate::NoteSnapshot, crate::NoteMutation>(&document.schema.clone(), &document.id.clone(), document, None))
            .await
            .expect("seed store for the semio example");
        store::print_document_pack(seed.envelope()).await.expect("print semio example document pack")
    };
    app.load_document_pack(&files).await.expect("load semio example");
    let json = render_body(&mut app, BODY_ARTIFACT).await;
    assert!(json.contains("\"type\":\"tree\""));
    assert!(json.contains("Welcome"));
}

#[semio_framework_async_macros::async_test]
async fn note_labels_resolve_native_by_default() {
    let mut app = note_app().await;
    let document_json = render_body(&mut app, BODY_ARTIFACT).await;
    assert!(document_json.contains("Add Text"));
    assert!(document_json.contains("Add Table"));
    assert!(document_json.contains("Add Math"));
    assert!(document_json.contains("Add Image"));
    assert!(document_json.contains("Add Group"));
    assert!(document_json.contains("Drop blocks here"));
}

//#region 🪟️WindowLaws
// 🪟️ ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §8.4 — the four window laws that replace the
// old `+N` paging laws. (a) an oversized document stamps every container's real `total` and
// materialises at most its slice, (b) a closed container stamps `total` and builds no children,
// (c) a host `TreeWindowRequest{offset, rows}` materialises exactly `[offset, offset + rows)` keyed by
// the row's own target id, (d) pick rows carry a granularity and no binding of their own while the
// tree root carries exactly one `interactionSelect`.
use super::render;
use crate::editor::note::terminology::note_play_labels;
use crate::NoteBlockNode;
use semio_framework_plugin::{BuiltNode, Component, TreeWindowRequest, TreeWindows, ViewModel, INTERACTION_SELECT_ACTION_ID};

const BLOCKS_SECTION: &str = "note-play-blocks.blocks";
const ADD_SECTION: &str = "note-play-blocks.add";
const OVERSIZED: usize = 200;

fn note_group(id: String, children: Vec<NoteBlockNode>) -> NoteBlockNode {
    NoteBlockNode::Group { id: id.clone(), name: id, x: 0.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0, visible: true, locked: false, children }
}

/// 🗒️ 200 top-level groups, each nesting 200 more — every level independently outgrows one node's
/// fixed child capacity, which is exactly the shape that used to fail the whole render.
fn oversized_note_document(count: usize) -> crate::NoteSnapshot {
    let blocks = (0..count).map(|index| note_group(format!("block-{index:03}"), (0..count).map(|child| note_group(format!("block-{index:03}-{child:03}"), Vec::new())).collect())).collect();
    crate::NoteSnapshot { blocks, ..crate::NoteSnapshot::default() }
}

fn window_law_request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }
}

fn window_law_node<'a>(root: &'a BuiltNode, key: &str) -> &'a BuiltNode {
    fn walk<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
        if node.key.as_str() == key {
            return Some(node);
        }
        node.children.iter().find_map(|child| walk(child, key))
    }
    walk(root, key).unwrap_or_else(|| panic!("no node keyed {key}"))
}

fn window_law_extent(node: &BuiltNode) -> (u32, u32) {
    let window = match &node.component {
        Component::TreeSection(props) => props.window,
        Component::TreeItem(props) => props.window,
        _ => None,
    };
    let window = window.unwrap_or_else(|| panic!("{} stamps no window", node.key.as_str()));
    (window.total, window.offset)
}

fn window_law_keys(node: &BuiltNode) -> Vec<String> {
    node.children.iter().map(|child| child.key.as_str().to_string()).collect()
}

fn window_law_no_continuation(node: &BuiltNode) {
    assert!(!node.key.as_str().ends_with(".more"), "{} is a continuation row", node.key.as_str());
    if let Component::TreeItem(props) = &node.component {
        assert!(!props.label.0.as_str().starts_with('+'), "{} carries a +N label", node.key.as_str());
    }
    for child in node.children.iter() {
        window_law_no_continuation(child);
    }
}

fn window_law_view(requests: Vec<TreeWindowRequest>) -> ViewModel {
    ViewModel { tree_windows: requests, ..Default::default() }
}

#[semio_framework_async_macros::async_test]
async fn an_oversized_document_stamps_the_full_total_at_every_level() {
    let document = oversized_note_document(OVERSIZED);
    let tree = render(&document, note_play_labels(&ViewModel::default()), &TreeWindows::unhosted()).expect("the document tree builds");
    let blocks = window_law_node(&tree, BLOCKS_SECTION);
    assert_eq!(window_law_extent(blocks), (OVERSIZED as u32, 0));
    assert!(blocks.children.len() < OVERSIZED, "only the first-paint slice is materialised: {}", blocks.children.len());
    let first_group = blocks.children.iter().next().expect("at least one group row");
    assert_eq!(window_law_extent(first_group), (OVERSIZED as u32, 0), "a nested group stamps its own total");
    window_law_no_continuation(&tree);
}

/// 🧰️ The five quick-add rows are panel chrome in their own fixed section — they never enter the
/// windowed content section and so never cost it a slot.
#[semio_framework_async_macros::async_test]
async fn the_quick_add_rows_live_in_their_own_fixed_section() {
    let document = oversized_note_document(4);
    let tree = render(&document, note_play_labels(&ViewModel::default()), &TreeWindows::unhosted()).expect("the document tree builds");
    assert_eq!(window_law_node(&tree, ADD_SECTION).children.len(), 5);
    assert!(window_law_keys(window_law_node(&tree, BLOCKS_SECTION)).iter().all(|key| key.starts_with("note-play-block")), "content rows only");
}

#[semio_framework_async_macros::async_test]
async fn a_closed_section_stamps_its_total_and_builds_no_children() {
    let document = oversized_note_document(OVERSIZED);
    let view = window_law_view(vec![window_law_request(BLOCKS_SECTION, Some(false), 0, 32)]);
    let tree = render(&document, note_play_labels(&ViewModel::default()), &TreeWindows::for_body(&view, BODY_ARTIFACT)).expect("the document tree builds");
    let blocks = window_law_node(&tree, BLOCKS_SECTION);
    assert_eq!(window_law_extent(blocks), (OVERSIZED as u32, 0));
    assert_eq!(blocks.children.len(), 0);
}

#[semio_framework_async_macros::async_test]
async fn a_window_request_materialises_exactly_its_slice_keyed_by_the_block_row_id() {
    let document = oversized_note_document(OVERSIZED);
    let view = window_law_view(vec![window_law_request(BLOCKS_SECTION, Some(true), 60, 6)]);
    let tree = render(&document, note_play_labels(&ViewModel::default()), &TreeWindows::for_body(&view, BODY_ARTIFACT)).expect("the document tree builds");
    let blocks = window_law_node(&tree, BLOCKS_SECTION);
    assert_eq!(window_law_extent(blocks), (OVERSIZED as u32, 60));
    let expected: Vec<String> = (60..66).map(|index| crate::schema::block_tree_row_id(&note_group(format!("block-{index:03}"), Vec::new()))).collect();
    assert_eq!(window_law_keys(blocks), expected);
}

#[semio_framework_async_macros::async_test]
async fn pick_rows_carry_a_granularity_and_the_tree_root_carries_the_one_interaction_select() {
    let document = oversized_note_document(3);
    let tree = render(&document, note_play_labels(&ViewModel::default()), &TreeWindows::unhosted()).expect("the document tree builds");
    assert_eq!(tree.bindings.len(), 1);
    assert_eq!(tree.bindings.iter().next().expect("the tree binding").action.name.as_str(), INTERACTION_SELECT_ACTION_ID);
    for row in window_law_node(&tree, BLOCKS_SECTION).children.iter() {
        let Component::TreeItem(props) = &row.component else { panic!("{} is not a tree row", row.key.as_str()) };
        assert_eq!(props.granularity.as_ref().map(semio_framework_plugin::UiText::as_str), Some("block"));
        assert!(row.bindings.is_empty(), "{} must not pay for its own activate binding", row.key.as_str());
    }
}

//#endregion 🪟️WindowLaws
