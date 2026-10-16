use super::*;

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn built_fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../../../../🧬️contract/♻️retirement/🌲️built/🧪️tests/🧫️fixtures/🔣️.json")).unwrap() }
fn text_node(key: &str, value: &str) -> crate::TreeNode {
    ui_contract::BuiltNode::try_new(key, ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label::try_from(value).unwrap(), emphasize: None, data_attributes: Default::default() })).unwrap()
}
fn populated(component: ui_contract::Component) -> crate::TreeNode {
    let fixture = built_fixture();
    let mut node = crate::TreeNode::try_new("K", component).unwrap();
    node.bindings.try_push(serde_json::from_value(fixture["binding"].clone()).unwrap()).unwrap();
    node.menu = Some(serde_json::from_value(fixture["menu"].clone()).unwrap());
    node.children.try_push(text_node("C", "child")).unwrap();
    node.rejected_children.try_push(text_node("R", "rejected")).unwrap();
    node
}
fn close(cursor: &mut SurfaceTreeRetireCursor, grant: usize) -> usize {
    let mut bytes = 0;
    for _ in 0..1_000_000 {
        let step = cursor.close_step(1, grant).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= grant);
        bytes += step.released_bytes;
        if step.complete { assert!(cursor.is_empty()); return bytes; }
        if !step.progressed { std::thread::yield_now(); }
    }
    panic!("runtime exact tree owner did not reach terminal");
}
fn chain(pages: usize) -> crate::TreeNode {
    let mut node = text_node("K", "V");
    for index in 0..pages {
        let mut parent = text_node("K", "V");
        let children = if index % 2 == 0 { &mut parent.children } else { &mut parent.rejected_children };
        children.try_push(node).unwrap();
        node = parent;
    }
    node
}
#[test]
fn runtime_tree_retirement_preserves_occupied_sources_and_closes_exact_payloads() {
    let _guard = testkit::registry_guard();
    let fixture = fixture();
    let components: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧬️contract/♻️retirement/🌳️typed/🧩️components.json")).unwrap();
    let foreign: ui_contract::UiValue = serde_json::from_value(fixture["foreign"].clone()).unwrap();
    for grant in fixture["grants"].as_array().unwrap() {
        for row in components["cases"].as_array().unwrap() {
            let mut cursor = SurfaceTreeRetireCursor::default();
            let mut source = Some(populated(serde_json::from_value(row["component"].clone()).unwrap()));
            assert!(cursor.try_begin_node(&mut source)); assert!(source.is_none());
            assert_eq!(cursor.close_step(0, 4096).unwrap(), Default::default());
            assert_eq!(cursor.close_step(1, 0).unwrap(), Default::default());
            let mut blocked = Some(crate::ComponentTree::new(text_node("blocked", "🌊")));
            let mut blocked_node = Some(text_node("blocked-node", "🌊"));
            let mut blocked_held = Some((Some(9), text_node("blocked-held", "🌊")));
            let source_identity = blocked.as_ref().unwrap() as *const crate::ComponentTree;
            let node_identity = blocked_node.as_ref().unwrap() as *const crate::TreeNode;
            let held_identity = blocked_held.as_ref().unwrap() as *const (Option<usize>, crate::TreeNode);
            let owner_identity = cursor.owner.as_ref().unwrap() as *const ui_contract::BuiltTreeRetirement;
            assert!(!cursor.try_begin_tree(&mut blocked));
            assert!(!cursor.try_begin_node(&mut blocked_node));
            assert!(!cursor.try_begin_held(&mut blocked_held));
            assert_eq!(blocked.as_ref().unwrap() as *const crate::ComponentTree, source_identity);
            assert_eq!(blocked_node.as_ref().unwrap() as *const crate::TreeNode, node_identity);
            assert_eq!(blocked_held.as_ref().unwrap() as *const (Option<usize>, crate::TreeNode), held_identity);
            assert_eq!(cursor.owner.as_ref().unwrap() as *const ui_contract::BuiltTreeRetirement, owner_identity);
            assert_eq!(close(&mut cursor, grant.as_u64().unwrap() as usize), row["bytes"].as_u64().unwrap() as usize + built_fixture()["extraPayloadBytes"].as_u64().unwrap() as usize);
            assert!(cursor.try_begin_tree(&mut blocked)); assert!(blocked.is_none());
            assert_eq!(close(&mut cursor, 1), "blocked🌊".len());
            assert!(cursor.try_begin_node(&mut blocked_node)); assert!(blocked_node.is_none());
            assert_eq!(close(&mut cursor, 1), "blocked-node🌊".len());
            assert!(cursor.try_begin_held(&mut blocked_held)); assert!(blocked_held.is_none());
            assert_eq!(close(&mut cursor, 1), "blocked-held🌊".len());
            assert_eq!(serde_json::to_value(&foreign).unwrap(), fixture["foreign"]);
        }
    }
    let mut foreign = ui_contract::UiValueRetirement::new(foreign);
    for _ in 0..10_000 { if foreign.close_step(1, 4096).unwrap().complete { break; } }
    assert!(foreign.terminal_is_empty());
    eprintln!("runtime tree exact owner:18 components*3 grants; occupied node/tree/held sources preserved; bytes matched; foreign untouched");
}
