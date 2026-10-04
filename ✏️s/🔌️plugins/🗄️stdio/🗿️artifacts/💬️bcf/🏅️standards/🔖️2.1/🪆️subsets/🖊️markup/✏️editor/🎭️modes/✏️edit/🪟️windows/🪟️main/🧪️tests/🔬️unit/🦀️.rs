use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_editable_table_window_kit() {
    let def = definition();
    assert_eq!(def.id, TableWindowKit::KIND_ID);
    assert!(def.actions.iter().any(|action| action.id == "set-cell"));
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_table_node_for_the_default_document() {
    let document = BcfSnapshot::default();
    for locale in [Locale::En, Locale::De] {
        render(&document, locale).expect("default document table renders");
    }
}

#[semio_framework_async_macros::async_test]
async fn render_binds_each_topic_cell_to_its_address_and_snapshot_revision() {
    use semio_framework_plugin::{Component, UiValue};
    fn node_by_key<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
        if node.key.as_str() == key {
            return Some(node);
        }
        node.children.iter().find_map(|child| node_by_key(child, key))
    }
    let mut document = BcfSnapshot::default();
    document.topics.push(Default::default());
    document.topics[0].title = "Issue".into();
    let node = render_revisioned(&document, "store-revision", semio_framework_plugin::UiPublicationRevision(23), Locale::En, &TreeWindows::unhosted()).expect("render table");
    let row = node_by_key(&node, "topic-0").expect("topic row");
    let cell = node_by_key(row, "cell-1").expect("title cell");
    let Component::Input(props) = &cell.component else { panic!("the title cell is an editable input") };
    assert_eq!(props.value.as_str(), "Issue");
    let binding = cell.bindings.iter().find(|binding| binding.action.name.as_str() == "set-cell").expect("set-cell binding");
    let Some(UiValue::Map(arguments)) = &binding.args else { panic!("set-cell carries its cell address") };
    let arguments: Vec<_> = arguments.iter().collect();
    let argument = |key: &str| arguments.iter().find(|(name, _)| name.as_str() == key).map(|(_, value)| value).unwrap_or_else(|| panic!("{key} argument"));
    assert_eq!(argument("row"), &UiValue::Number(0.0));
    assert_eq!(argument("column"), &UiValue::Number(1.0));
    assert!(matches!(argument("revision"), UiValue::Text(revision) if revision.as_str() == "store-revision"));
}
