mod utility_node_wire_format_tests {
    use super::super::layout::ActionDescriptor;
    use super::*;

    const GOLDEN_UTILITY_NODE_JSON: &str = "[{\"kind\":\"separator\",\"id\":\"sep1\",\"order\":1},{\"kind\":\"button\",\"id\":\"btn1\",\"iconId\":\"wrench\",\"label\":\"Utility\",\"title\":\"Utility\",\"category\":\"history\",\"onPress\":{\"controllerId\":\"ctrl\",\"action\":\"runUtility\"}},{\"kind\":\"toggle\",\"id\":\"tog1\",\"iconId\":\"panel-left\",\"label\":\"Toggle\",\"title\":\"Toggle\",\"pressed\":true,\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"toggleUtility\"}},{\"kind\":\"collection\",\"id\":\"col1\",\"iconId\":\"folder\",\"label\":\"Group\",\"title\":\"Group\",\"children\":[{\"kind\":\"separator\",\"id\":\"sep2\"}]}]";

    #[semio_framework_async_macros::async_test]
    async fn utility_node_serializes_to_golden_json() {
        let nodes = vec![
            UtilityNode::Separator { id: "sep1".into(), order: Some(1), disabled: None },
            utility_button("btn1", IconName::Wrench, "Utility", ActionDescriptor { controller_id: "ctrl".into(), action: "runUtility".into(), args: None }).with_category(UtilityCategory::History),
            utility_toggle("tog1", IconName::PanelLeft, "Toggle", true, ActionDescriptor { controller_id: "ctrl".into(), action: "toggleUtility".into(), args: None }),
            utility_collection("col1", IconName::Folder, "Group", vec![utility_separator("sep2")]),
        ];
        let json = serde_json::to_string(&nodes).unwrap();
        assert_eq!(json, GOLDEN_UTILITY_NODE_JSON);
        let roundtripped: Vec<UtilityNode> = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, nodes);
    }

    fn spec(id: &str, group: Option<&str>) -> DerivedUtilitySpec {
        DerivedUtilitySpec { id: id.into(), label: id.to_uppercase(), icon_id: IconName::CircleDot, group: group.map(str::to_string), category: None }
    }

    #[semio_framework_async_macros::async_test]
    async fn derive_utility_nodes_marks_the_active_utility_pressed() {
        let nodes = derive_utility_nodes("ctrl", &[spec("select", None), spec("brush", None)], Some("brush"));
        assert_eq!(nodes.len(), 2);
        match &nodes[0] {
            UtilityNode::Toggle { id, pressed, on_change, .. } => {
                assert_eq!(id, "select");
                assert_eq!(*pressed, Some(false));
                assert_eq!(on_change.action, "setActiveUtility");
                assert_eq!(on_change.args, Some(DslValue::Object(vec![("utilityId".into(), DslValue::String("select".into()))])));
            }
            other => panic!("expected toggle, got {other:?}"),
        }
        match &nodes[1] {
            UtilityNode::Toggle { id, pressed, .. } => {
                assert_eq!(id, "brush");
                assert_eq!(*pressed, Some(true));
            }
            other => panic!("expected toggle, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn derive_utility_nodes_groups_shared_group_into_one_collection() {
        let nodes = derive_utility_nodes("ctrl", &[spec("select", None), spec("line", Some("shapes")), spec("rect", Some("shapes"))], None);
        assert_eq!(nodes.len(), 2, "one ungrouped toggle + one shapes collection");
        assert!(matches!(&nodes[0], UtilityNode::Toggle { id, .. } if id == "select"));
        match &nodes[1] {
            UtilityNode::Collection { id, children, .. } => {
                assert_eq!(id, "group:shapes");
                assert_eq!(children.len(), 2);
                assert!(matches!(&children[0], UtilityNode::Toggle { id, .. } if id == "line"));
                assert!(matches!(&children[1], UtilityNode::Toggle { id, .. } if id == "rect"));
            }
            other => panic!("expected collection, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn derive_utility_nodes_hoists_single_child_groups() {
        let nodes = derive_utility_nodes("ctrl", &[spec("transform", Some("transform")), spec("brush", None)], Some("transform"));
        assert_eq!(nodes.len(), 2, "lone group child is hoisted — no nested Transform/Transform pair");
        match &nodes[0] {
            UtilityNode::Toggle { id, pressed, .. } => {
                assert_eq!(id, "transform");
                assert_eq!(*pressed, Some(true));
            }
            other => panic!("expected hoisted toggle, got {other:?}"),
        }
        assert!(matches!(&nodes[1], UtilityNode::Toggle { id, .. } if id == "brush"));
    }

    // 🌱️ Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS: round-trip
    // coverage for `UtilityNode` — internally tagged (`tag = "kind"`) with `rename_all_fields`.
    #[semio_framework_async_macros::async_test]
    async fn utility_node_round_trips() {
        let values = [
            UtilityNode::Separator { id: "sep-1".into(), order: Some(1), disabled: None },
            UtilityNode::Button {
                id: "btn-1".into(),
                icon_id: IconName::AppWindow,
                label: Some("Label".into()),
                text: None,
                title: None,
                order: None,
                disabled: Some(false),
                category: Some(UtilityCategory::Selection),
                on_press: ActionDescriptor { controller_id: "ctrl".into(), action: "press".into(), args: None },
            },
            UtilityNode::Collection {
                id: "col-1".into(),
                icon_id: IconName::AppWindow,
                label: None,
                text: None,
                title: None,
                order: None,
                disabled: None,
                category: None,
                children: vec![UtilityNode::Toggle {
                    id: "tog-1".into(),
                    icon_id: IconName::AppWindow,
                    label: None,
                    text: None,
                    title: None,
                    order: None,
                    pressed: Some(true),
                    disabled: None,
                    category: None,
                    on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "toggle".into(), args: None },
                }],
            },
        ];
        for value in values {
            assert_eq!(UtilityNode::from_value(value.to_value()).expect("valid DslValue decodes"), value);
        }
    }
}
