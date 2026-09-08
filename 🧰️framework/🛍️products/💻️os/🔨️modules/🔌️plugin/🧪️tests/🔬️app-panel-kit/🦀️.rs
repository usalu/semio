mod panel_kit_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn tree_item_builds_a_bare_item() {
        let item = tree_item("ns.kind.a", "A").expect("bounded fixture");
        assert_eq!(item.key.as_str(), "ns.kind.a");
        let Component::TreeItem(props) = &item.component else { panic!("expected a TreeItem") };
        assert_eq!(props.label.0.as_str(), "A");
        assert!(props.description.is_none());
        assert!(item.bindings.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn tree_item_with_action_draggable_maps_json_object_to_string_drag_data() {
        let action = ActionId::try_v1("app", "addWidget").expect("bounded fixture");
        let item =
            tree_item_with_action_draggable("ns.kind.a", "A", None, (action, None), &dsl::os_pack::json::object([("application/x-widget".to_string(), dsl::os_pack::json::Value::String("{\"kind\":\"a\"}".to_string()))])).expect("bounded fixture");
        let Component::TreeItem(props) = &item.component else { panic!("expected a TreeItem") };
        assert_eq!(props.draggable, Some(true));
        assert!(props.drag_data.as_ref().unwrap().iter().any(|(key, value)| key.as_str() == "application/x-widget" && value.as_str() == "{\"kind\":\"a\"}"));
    }

    #[semio_framework_async_macros::async_test]
    async fn selection_ids_reads_the_ids_array_arg() {
        let args = DslValue::from(&serde_json::json!({ "ids": ["a", "b"] }));
        assert_eq!(selection_ids(Some(&args)), vec!["a".to_string(), "b".to_string()]);
        assert!(selection_ids(None).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn panel_tree_builder_produces_a_namespaced_tree_with_placeholder() {
        let builder = PanelTreeBuilder::new("ns-play-document").expect("bounded fixture");
        let item_id = builder.item_id("widget", "w1").expect("bounded fixture");
        assert_eq!(item_id.as_str(), "ns-play-document.widget.w1");
        let mut items = UiFixedList::default();
        items.try_push(tree_item(item_id, "W1").expect("bounded fixture")).expect("bounded fixture");
        let builder = builder.section("ns-play-document.widgets", Some(Label::try_from("Widgets").expect("bounded fixture")), true, items).expect("bounded fixture");
        let builder = builder.section_or_placeholder("ns-play-document.synapses", Some(Label::try_from("Synapses").expect("bounded fixture")), false, UiFixedList::default(), "(none)").expect("bounded fixture");
        let builder = builder.selected(["ns-play-document.widget.w1".to_string()]).expect("bounded fixture");
        let builder = builder.interaction_domain("ns-play-document").expect("bounded fixture");
        let node = builder.build().expect("bounded fixture");
        assert_eq!(node.children.len(), 2);
        let Component::TreeSection(_) = &node.children[0].component else { panic!("expected a TreeSection") };
        assert_eq!(node.children[0].children.len(), 1);
        let Component::TreeItem(placeholder) = &node.children[1].children[0].component else { panic!("expected a TreeItem") };
        assert_eq!(placeholder.label.0.as_str(), "(none)");
        let Component::Tree(tree_props) = &node.component else { panic!("expected a Tree") };
        assert_eq!(tree_props.interaction_domain.as_deref(), Some("ns-play-document"));
    }
}
