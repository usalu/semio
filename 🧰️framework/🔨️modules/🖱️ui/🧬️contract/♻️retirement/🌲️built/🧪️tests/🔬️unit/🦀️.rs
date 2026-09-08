use super::*;
use crate::{BuiltChildren, Component, TextProps};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixture/🔣️.json")).unwrap()
}

fn text_node(key: &str, value: &str) -> BuiltNode {
    BuiltNode::try_new(key, Component::Text(TextProps { value: crate::Label::try_from(value).unwrap(), emphasize: None, data_attributes: Default::default() })).unwrap()
}

fn close(owner: &mut BuiltTreeRetirement, grant: usize) -> usize {
    assert_eq!(owner.close_step(0, grant).unwrap(), UiValueRetirementStep::default());
    assert_eq!(owner.close_step(1, 0).unwrap(), UiValueRetirementStep::default());
    let mut bytes = 0;
    for _ in 0..2_000_000 {
        let step = owner.close_step(1, grant).unwrap();
        assert!(step.released_items <= 1 && step.released_bytes <= grant);
        bytes += step.released_bytes;
        if step.complete {
            assert!(owner.terminal_is_empty());
            return bytes;
        }
        if !step.progressed {
            std::thread::yield_now();
        }
    }
    panic!("built tree retirement did not finish");
}

#[test]
fn built_tree_retirement_closes_all_typed_fields_and_preserves_foreign_values() {
    let fixture = fixture();
    let components: serde_json::Value = serde_json::from_str(include_str!("../../../🌳️typed/🧩️components.json")).unwrap();
    let foreign: UiValue = serde_json::from_value(serde_json::json!({"foreign": ["Grüße"]})).unwrap();
    let foreign_handles = super::super::tests::descendants(&foreign);
    for grant in fixture["grants"].as_array().unwrap() {
        for row in components["cases"].as_array().unwrap() {
            let mut node = BuiltNode::try_new("K", serde_json::from_value(row["component"].clone()).unwrap()).unwrap();
            node.bindings.try_push(serde_json::from_value(fixture["binding"].clone()).unwrap()).unwrap();
            node.menu = Some(serde_json::from_value(fixture["menu"].clone()).unwrap());
            node.children.try_push(text_node("C", "child")).unwrap();
            node.rejected_children.try_push(text_node("R", "rejected")).unwrap();
            let mut owner = BuiltTreeRetirement::new(node);
            assert_eq!(close(&mut owner, grant.as_u64().unwrap() as usize), row["bytes"].as_u64().unwrap() as usize + fixture["extraPayloadBytes"].as_u64().unwrap() as usize);
            assert_eq!(serde_json::to_value(&foreign).unwrap(), serde_json::json!({"foreign": ["Grüße"]}));
            with_ui_value_arena(|arena| assert!(foreign_handles.iter().all(|handle| arena.collection(*handle).is_some())));
        }
    }
    let mut foreign = UiValueRetirement::new(foreign);
    for _ in 0..10_000 {
        if foreign.close_step(1, 4096).unwrap().complete {
            break;
        }
    }
    assert!(foreign.terminal_is_empty());
    let mut node = text_node("root", "V");
    node.children.try_push(text_node("ordinary", "V")).unwrap();
    node.rejected_children.try_push(text_node("rejected", "V")).unwrap();
    let mut owner = BuiltTreeRetirement::new(node);
    let mut visited = Vec::new();
    for _ in 0..10_000 {
        if owner.owned.field == 0 {
            if let Some(node) = owner.owned.node.as_ref() {
                visited.push(node.key.as_str().to_owned());
            }
        }
        if owner.close_step(1, 4096).unwrap().complete {
            break;
        }
    }
    assert!(owner.terminal_is_empty());
    assert_eq!(visited, ["root", "ordinary", "rejected"]);
    eprintln!("[DEBUG] built-tree retirement components=18 grants=3 ordinary+rejected=2 extraBytes=30 foreign=preserved");
}

#[test]
fn built_tree_retirement_closes_full_page_chain_beyond_observer_depth() {
    let fixture = fixture();
    let pages = fixture["chain"]["pages"].as_u64().unwrap() as usize;
    assert_eq!(pages, UI_BUILT_CHILD_RETIRE_SLOTS);
    let mut node = text_node("K", "V");
    for index in 0..pages {
        let mut parent = text_node("K", "V");
        let children: &mut BuiltChildren = if index % 2 == 0 { &mut parent.children } else { &mut parent.rejected_children };
        children.try_push(node).unwrap();
        node = parent;
    }
    let mut owner = BuiltTreeRetirement::new(node);
    assert_eq!(close(&mut owner, 1), fixture["chain"]["nodes"].as_u64().unwrap() as usize * 2);
    assert_eq!(owner.close_step(0, 0).unwrap(), UiValueRetirementStep { complete: true, ..Default::default() });
    eprintln!("[DEBUG] built-tree retirement fullPages={pages} nodes={} observerDepth=64 exactTerminal=true", pages + 1);
}
