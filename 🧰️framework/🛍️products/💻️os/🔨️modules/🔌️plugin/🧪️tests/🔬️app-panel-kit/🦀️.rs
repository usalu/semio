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
        let builder = builder.interaction_domain("ctrl", "ns-play-document").expect("bounded fixture");
        let node = builder.build().expect("bounded fixture");
        assert_eq!(node.children.len(), 2);
        let Component::TreeSection(_) = &node.children[0].component else { panic!("expected a TreeSection") };
        assert_eq!(node.children[0].children.len(), 1);
        let Component::TreeItem(placeholder) = &node.children[1].children[0].component else { panic!("expected a TreeItem") };
        assert_eq!(placeholder.label.0.as_str(), "(none)");
        let Component::Tree(tree_props) = &node.component else { panic!("expected a Tree") };
        assert_eq!(tree_props.interaction_domain.as_deref(), Some("ns-play-document"));
    }
    fn window_entries(count: usize) -> Vec<usize> {
        (0..count).collect()
    }

    fn window_row(entry: &usize) -> UiAssemblyResult<BuiltNode> {
        tree_item(format!("ns.row.{entry}"), format!("Row {entry}"))
    }

    fn window_label() -> Label {
        Label::try_from("Rows").expect("bounded fixture")
    }

    fn section_window(node: &BuiltNode) -> Option<TreeWindow> {
        let Component::TreeSection(props) = &node.component else { panic!("expected a TreeSection") };
        props.window
    }

    #[semio_framework_async_macros::async_test]
    async fn a_closed_container_materialises_no_rows_and_still_stamps_its_total() {
        let entries = window_entries(20);
        let section = tree_window_section(&TreeWindows::unhosted(), "ns.rows", window_label(), false, &entries, window_row).expect("bounded fixture");
        assert!(section.children.is_empty(), "a closed container materialises nothing");
        assert_eq!(section_window(&section), Some(TreeWindow { total: 20, offset: 0 }), "its full extent is still published");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_host_request_materialises_exactly_its_slice_and_opens_a_closed_author_default() {
        let entries = window_entries(20);
        let view = ViewModel {
            tree_windows: vec![TreeWindowRequest { body_key: "body".to_string(), node_key: "ns.rows".to_string(), open: Some(true), offset: 5, rows: 3 }],
            ..ViewModel::default()
        };
        let section = tree_window_section(&TreeWindows::for_body(&view, "body"), "ns.rows", window_label(), false, &entries, window_row).expect("bounded fixture");
        assert_eq!(section.children.len(), 3);
        assert_eq!(section.children[0].key.as_str(), "ns.row.5");
        assert_eq!(section.children[2].key.as_str(), "ns.row.7");
        assert_eq!(section_window(&section), Some(TreeWindow { total: 20, offset: 5 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn a_first_paint_spends_one_shared_viewport_budget_in_document_order() {
        let entries = window_entries(40);
        let view = ViewModel { tree_viewport_rows: Some(48), ..ViewModel::default() };
        let windows = TreeWindows::for_body(&view, "body");
        for (id, expected) in [("ns.a", 40usize), ("ns.b", 8), ("ns.c", 0)] {
            let section = tree_window_section(&windows, id, window_label(), true, &entries, window_row).expect("bounded fixture");
            assert_eq!(section.children.len(), expected, "{id} materialises its share of the first-paint budget");
            assert_eq!(section_window(&section), Some(TreeWindow { total: 40, offset: 0 }), "{id} publishes its full extent regardless");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn a_refused_row_shortens_the_window_instead_of_faulting() {
        let entries = window_entries(20);
        let mut built = 0usize;
        let section = tree_window_section(&TreeWindows::unhosted(), "ns.rows", window_label(), true, &entries, |entry| {
            if built == 5 {
                return Err(ui_assembly_error("test.arena-spent"));
            }
            built += 1;
            window_row(entry)
        })
        .expect("a fixed-capacity refusal must end the window, never the render");
        assert_eq!(section.children.len(), 5);
        assert_eq!(section_window(&section), Some(TreeWindow { total: 20, offset: 0 }));
    }

    #[semio_framework_async_macros::async_test]
    async fn interaction_domain_stamps_one_tree_level_activate_binding() {
        let node = PanelTreeBuilder::new("ns").expect("bounded fixture").interaction_domain("ctrl", "ns.domain").expect("bounded fixture").build().expect("bounded fixture");
        assert_eq!(node.bindings.len(), 1, "exactly one tree-level pick binding, so rows cost no arena");
        let binding = &node.bindings[0];
        assert_eq!(binding.trigger, Trigger::Activate);
        assert_eq!(binding.action.scope.as_str(), "ctrl");
        assert_eq!(binding.action.name.as_str(), INTERACTION_SELECT_ACTION_ID);
        let Some(UiValue::Map(args)) = &binding.args else { panic!("expected a domainId argument map") };
        assert!(args.iter().any(|(key, value)| key.as_str() == "domainId" && matches!(value, UiValue::Text(text) if text.as_str() == "ns.domain")));
        let Component::Tree(props) = &node.component else { panic!("expected a Tree") };
        assert_eq!(props.interaction_domain.as_deref(), Some("ns.domain"));
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_windows_its_commands_without_a_continuation_row() {
        let entry = |seq: u64| CommandView {
            seq,
            action_id: "translateSelection".into(),
            label: format!("Move {seq}"),
            kind: ActionKind::Mutation,
            timestamp: "0".into(),
            edit_id: Some(format!("edit-{seq}")),
            config_edit_id: None,
            child_edit_ids: Vec::new(),
            op_lines: Vec::new(),
            applied: true,
            revertible: true,
            count: 1,
            inverse: None,
        };
        let history = HistoryView {
            columns: Vec::new(),
            can_undo: true,
            can_redo: false,
            active_alternative_id: None,
            current_checkpoint_id: None,
            commands: (1..=100).map(entry).collect(),
            command_filter: HistoryCommandFilter::All,
        };
        let panel = ui_history_panel(&history, "ctrl", false, false, &ViewModel::default()).await.expect("bounded fixture");
        let commands = &panel.children[1];
        assert_eq!(commands.children.len(), TREE_WINDOW_DEFAULT_ROWS as usize, "a cold paint materialises one viewport of commands");
        assert_eq!(section_window(commands), Some(TreeWindow { total: 100, offset: 0 }), "the scrollbar spans the whole log");
        let json = serde_json::to_string(&panel).expect("panel JSON");
        assert!(!json.contains(".more"), "no continuation row key survives: {json}");
        assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    }
}
