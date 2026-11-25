//! 🌳️ Reads a built UI tree node by node because populated `BuiltChildren` only travel through retained page transport.
use semio_ui_contract::{BuiltNode,Component};

/// 🔎️ Concatenates every node's key, component, accessibility, bindings and menu serialization, depth first.
pub(crate) fn built_tree_serialization(root:&BuiltNode)->String{
    let mut out=String::new();
    let mut frontier=vec![root];
    while let Some(node)=frontier.pop(){
        out.push_str(node.key.as_str());
        out.push_str(&serde_json::to_string(&node.component).unwrap());
        out.push_str(&serde_json::to_string(&node.accessibility).unwrap());
        out.push_str(&serde_json::to_string(&node.bindings).unwrap());
        out.push_str(&serde_json::to_string(&node.menu).unwrap());
        if let Component::Text(text)=&node.component{out.push_str(&text.packed_payload());}
        frontier.extend(node.children.iter().rev());
    }
    out
}

/// 🧹️ Observes one built tree through [`built_tree_serialization`], then retires exactly that tree before any outcome.
pub(crate) fn retired_tree_serialization(root:BuiltNode)->String{
    let text=built_tree_serialization(&root);
    let mut retirement=semio_ui_contract::BuiltTreeRetirement::new(root);
    while !retirement.terminal_is_empty(){
        let step=retirement.close_step(1,4096).expect("built-tree retirement remains valid");
        if !step.progressed{std::thread::yield_now();}
    }
    while !semio_ui_contract::close_built_node_page_one(){}
    text
}

/// 🔥️ Publishes the built-child and UI-value retirement authorities, as a host does before its first tree.
pub(crate) fn warm_ui() {
    use semio_ui_contract::{UiMapBuilder, UiValue, UiValueRetirement};
    semio_ui_contract::built_node_pages_are_terminal_empty();
    let mut warm = UiMapBuilder::try_new().unwrap();
    warm.try_insert("warm".into(), UiValue::Null).unwrap();
    let mut value = UiValueRetirement::new(UiValue::Map(warm.finish()));
    for _ in 0..16384 {
        if value.terminal_is_empty() { break; }
        value.close_step(1, 32768).unwrap();
    }
    assert!(value.terminal_is_empty());
}
