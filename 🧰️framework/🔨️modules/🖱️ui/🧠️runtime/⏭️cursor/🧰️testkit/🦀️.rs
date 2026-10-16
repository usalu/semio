//! 🧰️ Shared fixtures of the reconcile suites — bounded tree builders, snapshot oracles and the process-wide resident-ledger guard that serialises the laws admitting into the shared ledgers. Exposed under the `testkit` feature so the authority crate's suites build their trees the same way.

use semio_framework_ui_runtime_presentation::{ComponentTree, TreeNode};
pub use semio_framework_ui_runtime_reconcile_primitives::testkit::{RegistryGuard, registry_guard};

//#region 🔖️Fixtures
pub fn ui_text(value: &str) -> ui_contract::UiText {
    ui_contract::UiText::try_from_str(value).expect("bounded fixture text")
}

pub fn leaf(key: &str) -> TreeNode {
    TreeNode::try_new(key, ui_contract::Component::Separator(ui_contract::SeparatorProps {})).expect("bounded fixture node")
}

pub fn text(key: &str, value: &str) -> TreeNode {
    TreeNode::try_new(key, ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::try_from(value).expect("bounded fixture label"), emphasize: None, data_attributes: None })).expect("bounded fixture node")
}

pub fn container(key: &str, children: Vec<TreeNode>) -> TreeNode {
    let node =
        TreeNode::try_new(key, ui_contract::Component::Container(ui_contract::ContainerProps { role: ui_contract::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None }))
            .expect("bounded fixture node");
    node.try_with_children(children).unwrap_or_else(|_| panic!("bounded fixture children"))
}

pub fn tree(root: TreeNode) -> ComponentTree {
    ComponentTree::new(root)
}

pub fn ui_list(values: impl IntoIterator<Item = ui_contract::UiValue>) -> ui_contract::UiList {
    let mut builder = ui_contract::UiListBuilder::try_new().expect("fixed list builder");
    for value in values {
        builder.push(value).expect("fixed list page");
    }
    builder.finish()
}

pub fn ui_map(entries: impl IntoIterator<Item = (String, ui_contract::UiValue)>) -> ui_contract::UiMap {
    let mut builder = ui_contract::UiMapBuilder::try_new().expect("fixed map builder");
    for (key, value) in entries {
        builder.push(key, value).expect("ascending fixed map page");
    }
    builder.finish()
}

pub fn styled(node: TreeNode, tone: ui_contract::Tone) -> TreeNode {
    TreeNode { style: ui_contract::StyleSpec { tone, ..Default::default() }, ..node }
}

pub fn with_shortcut(node: TreeNode, shortcut: &str) -> TreeNode {
    TreeNode { accessibility: ui_contract::AccessibilitySpec { shortcut: Some(ui_text(shortcut)), ..Default::default() }, ..node }
}

pub fn with_binding(mut node: TreeNode, scope: &str, name: &str) -> TreeNode {
    node.bindings.try_push(ui_contract::ActionBinding { trigger: ui_contract::Trigger::Activate, action: ui_contract::ActionId::try_v1(scope, name).expect("bounded fixture action"), args: None, capability: None }).expect("bounded fixture bindings");
    node
}

pub fn with_menu(node: TreeNode, menu_id: &str) -> TreeNode {
    TreeNode { menu: Some(ui_contract::MenuRef { id: ui_text(menu_id), args: None }), ..node }
}

pub fn id_of(snapshot: &ui_contract::UiSnapshot, key: &str) -> ui_contract::UiNodeId {
    snapshot.nodes.iter().find(|record| record.key.as_str() == key).unwrap_or_else(|| panic!("no node keyed {key:?} in snapshot")).id
}

pub fn assert_snapshot_matches_state(snapshot: &ui_contract::UiSnapshot, state: &ui_contract::UiSnapshotState) {
    assert_eq!(snapshot.revision, state.revision);
    assert_eq!(Some(snapshot.root), state.root);
    assert_eq!(snapshot.nodes.len(), state.nodes.len(), "snapshot/state node-count mismatch");
    for record in &snapshot.nodes {
        assert_eq!(state.nodes.get(&record.id), Some(record), "record {:?} diverges between snapshot and applied state", record.id);
    }
}
//#endregion 🔖️Fixtures
