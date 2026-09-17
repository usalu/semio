use super::*;
use crate::schema::snapshot::ZipEntry;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_tree_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn render_lists_the_comment_root_and_one_leaf_per_entry() {
    let document = ZipSnapshot { entries: vec![ZipEntry { name: "a.txt".into(), data: b"hi".to_vec() }], comment: "an archive".into(), ..ZipSnapshot::default() };
    let node = render(&document, &semio_framework_plugin::TreeWindows::unhosted()).expect("render");
    let section = node.children.get(0).expect("tree section");
    let root = section.children.get(0).expect("tree root");
    assert_eq!(root.key.as_str(), COMMENT_NODE_ID);
    let children = &root.children;
    assert_eq!(children.len(), 1);
    assert_eq!(children.get(0).expect("child").key.as_str(), format!("{ENTRY_NODE_PREFIX}0"));
}

//#region 🪟️WindowLaws
use semio_framework_plugin::{TreeWindowRequest, TreeWindows, ViewModel};

/// 🪟 The viewport the host reports for these laws. Deliberately small: a first paint is priced in
/// real `UiValue` argument-arena credit, shared process-wide, and a law that materialised a full
/// 48-row viewport starved the sibling panel tests running beside it.
const MEASURED_VIEWPORT_ROWS: u32 = 4;

/// 🪟️ An ordinary archive — far past the 32 siblings this window used to refuse outright.
fn oversized_archive(entries: usize) -> ZipSnapshot {
    ZipSnapshot { entries: (0..entries).map(|index| ZipEntry { name: format!("file-{index}.txt"), data: vec![0; index] }).collect(), comment: "an archive".into(), ..ZipSnapshot::default() }
}

/// 🪟️ The window body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(document: &ZipSnapshot, requests: Vec<TreeWindowRequest>) -> String {
    let view = ViewModel { tree_windows: requests, tree_viewport_rows: Some(MEASURED_VIEWPORT_ROWS), ..Default::default() };
    let node = render(document, &TreeWindows::for_body(&view, BODY_KEY)).expect("render the archive tree");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project the archive tree")
}

/// 🪟️ Law (a): a 300-entry archive renders — the comment root stamps its full extent and no `+N` appears.
#[test]
fn oversized_archive_stamps_totals_and_never_a_continuation_row() {
    let json = window_body(&oversized_archive(300), Vec::new());
    assert!(json.contains("\"total\":300"), "the comment root stamps its full extent: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches(ENTRY_NODE_PREFIX).count() <= MEASURED_VIEWPORT_ROWS as usize, "first paint materialises the measured viewport and stops: {json}");
}

/// 🪟️ Law (b): a root the host closed stamps its total and materialises nothing.
#[test]
fn closed_root_stamps_total_and_materialises_no_entries() {
    let json = window_body(&oversized_archive(300), vec![TreeWindowRequest { body_key: BODY_KEY.into(), node_key: COMMENT_NODE_ID.into(), open: Some(false), offset: 0, rows: 0 }]);
    assert!(json.contains("\"total\":300"), "a closed root still stamps its extent: {json}");
    assert!(!json.contains(ENTRY_NODE_PREFIX), "a closed root materialises no entries: {json}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the raw entry id.
#[test]
fn host_window_materialises_exactly_its_slice() {
    let json = window_body(&oversized_archive(300), vec![TreeWindowRequest { body_key: BODY_KEY.into(), node_key: COMMENT_NODE_ID.into(), open: Some(true), offset: 100, rows: 10 }]);
    assert!(json.contains("\"offset\":100"), "the root reports its offset: {json}");
    for index in 100..110 {
        assert!(json.contains(&format!("\"{ENTRY_NODE_PREFIX}{index}\"")), "entry {index} is inside the window: {json}");
    }
    assert!(!json.contains(&format!("\"{ENTRY_NODE_PREFIX}99\"")), "the entry before the window stays out: {json}");
    assert!(!json.contains(&format!("\"{ENTRY_NODE_PREFIX}110\"")), "the entry after the window stays out: {json}");
}

/// 🪟️ Law (d) for a `TreeWindowKit` surface: it is a document structure, not a pick target — it binds no
/// interaction domain and stamps no granularity, so rows keep exactly the `set-node` editing they had.
#[test]
fn archive_tree_binds_no_interaction_domain() {
    let json = window_body(&oversized_archive(4), Vec::new());
    assert!(!json.contains("interactionDomain"), "the archive tree binds no domain: {json}");
    assert!(!json.contains("granularity"), "the archive tree stamps no pick granularity: {json}");
}
//#endregion 🪟️WindowLaws
