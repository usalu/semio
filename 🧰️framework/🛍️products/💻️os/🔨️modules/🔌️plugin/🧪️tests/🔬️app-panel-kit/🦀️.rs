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

    /// 🧾️ The body as the host reads it — `BuiltChildren` refuses a direct serde walk (it is retained
    /// page transport), so the tree is projected node by node.
    fn panel_body_json(node: &BuiltNode) -> serde_json::Value {
        serde_json::json!({
            "key": node.key.as_str(),
            "component": serde_json::to_value(&node.component).expect("component JSON"),
            "children": node.children.iter().map(panel_body_json).collect::<Vec<_>>(),
        })
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
        let entries = window_entries(30);
        let view = ViewModel { tree_viewport_rows: Some(48), ..ViewModel::default() };
        let windows = TreeWindows::for_body(&view, "body");
        for (id, expected) in [("ns.a", 30usize), ("ns.b", 18), ("ns.c", 0)] {
            let section = tree_window_section(&windows, id, window_label(), true, &entries, window_row).expect("bounded fixture");
            assert_eq!(section.children.len(), expected, "{id} materialises its share of the one shared first-paint budget");
            assert_eq!(section_window(&section), Some(TreeWindow { total: 30, offset: 0 }), "{id} publishes its full extent regardless");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn a_window_never_exceeds_one_built_children_page() {
        let entries = window_entries(UI_BUILT_CHILDREN_MAX * 4);
        let view = ViewModel { tree_viewport_rows: Some(u32::MAX), ..ViewModel::default() };
        let windows = TreeWindows::for_body(&view, "body");
        let section = tree_window_section(&windows, "ns.rows", window_label(), true, &entries, window_row).expect("bounded fixture");
        assert!(section.children.len() <= UI_BUILT_CHILDREN_MAX, "one built node fans out at most one host child page");
        assert_eq!(section.children.len(), LEDGER - 1, "and the body-wide node ledger — root, headroom, this section node — is the tighter ceiling");
        assert_eq!(windows.nodes_remaining(), 0);
        assert_eq!(section_window(&section), Some(TreeWindow { total: entries.len() as u32, offset: 0 }));
    }

    /// 🧾️ The ledger a body starts a render with — the ONE budget both sides of the wire spend.
    const LEDGER: usize = TREE_WINDOW_BODY_NODE_BUDGET;

    /// 🧾️ Records the reconciler charges for a presented body: every node counts once.
    fn body_nodes(node: &BuiltNode) -> usize {
        1 + node.children.iter().map(body_nodes).sum::<usize>()
    }

    /// 🧾️ Nine containers, each asking for 30 of its 63 entries, sum to 270 rows — more than twice the
    /// surface's record arena, and exactly the shape that faulted fem3d House at `nodes: 129 >
    /// max_nodes: 128`. The body-wide ledger seats them in document order, starves the tail, and every
    /// container still publishes its full extent so the host can scroll into it.
    #[semio_framework_async_macros::async_test]
    async fn nine_windowed_sections_share_one_body_wide_node_ledger() {
        let entries = window_entries(63);
        let keys: Vec<String> = (0..9).map(|index| format!("ns.s{index}")).collect();
        let view = ViewModel {
            tree_windows: keys.iter().map(|node_key| TreeWindowRequest { body_key: "body".to_string(), node_key: node_key.clone(), open: Some(true), offset: 0, rows: 30 }).collect(),
            ..ViewModel::default()
        };
        let windows = TreeWindows::for_body(&view, "body");
        assert_eq!(windows.nodes_remaining(), LEDGER, "a render opens on the full ledger");
        let mut builder = PanelTreeBuilder::new("ns").expect("bounded fixture");
        for node_key in &keys {
            builder = builder.window_section(&windows, node_key, Some(window_label()), true, &entries, window_row).expect("bounded fixture");
        }
        let body = builder.build().expect("bounded fixture");
        assert_eq!(body.children.len(), keys.len());
        for section in body.children.iter() {
            assert_eq!(section_window(section), Some(TreeWindow { total: 63, offset: 0 }), "every container publishes its full extent, including one the ledger could not seat");
        }
        let rows: usize = body.children.iter().map(|section| section.children.len()).sum();
        println!("[DEBUG] tree-window-ledger sections={} rows={rows} body_nodes={} remaining={}", keys.len(), body_nodes(&body), windows.nodes_remaining());
        assert_eq!(windows.nodes_remaining(), 0, "nine containers of 30 rows exhaust the ledger");
        assert!(rows <= LEDGER, "the rows a body materialises never exceed its ledger: {rows} > {LEDGER}");
        assert!(body_nodes(&body) <= UI_DOCUMENT_NODES, "and the presented body fits the reconciler's record arena: {} > {UI_DOCUMENT_NODES}", body_nodes(&body));
        assert!(body_nodes(&body) <= LEDGER + 1 + TREE_WINDOW_FIXED_NODE_HEADROOM, "the headroom absorbs the container nodes the exhausted ledger could not charge");
    }

    /// 🧾️ A nested group item is a node record like any other: the item and every row it materialises
    /// come off the same body-wide ledger as the sections around it.
    #[semio_framework_async_macros::async_test]
    async fn a_nested_group_item_charges_the_body_wide_node_ledger() {
        let entries = window_entries(10);
        let windows = TreeWindows::unhosted();
        let Ok(item) = ui::tree_item(window_label()).try_id("ns.group") else { panic!("bounded fixture") };
        let group = tree_window_item(&windows, item, "ns.group", true, &entries, window_row).expect("bounded fixture");
        assert_eq!(group.children.len(), 10);
        assert_eq!(windows.nodes_remaining(), LEDGER - 11, "the group item and its ten rows each cost one record");

        let nested = TreeWindows::unhosted();
        let leaves = window_entries(8);
        let groups = window_entries(LEDGER);
        let Ok(outer) = ui::tree_item(window_label()).try_id("ns.deep") else { panic!("bounded fixture") };
        let deep = tree_window_item(&nested, outer, "ns.deep", true, &groups, |entry| {
            let key = format!("ns.deep.{entry}");
            let inner = ui::tree_item(window_label()).try_id(&key).map_err(|_| ui_assembly_error("test.inner-id"))?;
            tree_window_item(&nested, inner, &key, true, &leaves, window_row)
        })
        .expect("bounded fixture");
        println!("[DEBUG] tree-window-ledger nested body_nodes={} remaining={}", body_nodes(&deep), nested.nodes_remaining());
        assert!(body_nodes(&deep) <= UI_DOCUMENT_NODES, "a nested window still fits the record arena: {}", body_nodes(&deep));
    }

    /// 🧾️ The first-paint path is ledgered too: a host that reports a 500-row viewport still cannot
    /// make a body outgrow the surface document.
    #[semio_framework_async_macros::async_test]
    async fn a_first_paint_never_outspends_the_node_ledger() {
        let entries = window_entries(200);
        let view = ViewModel { tree_viewport_rows: Some(500), ..ViewModel::default() };
        let windows = TreeWindows::for_body(&view, "body");
        let mut builder = PanelTreeBuilder::new("ns").expect("bounded fixture");
        for node_key in ["ns.a", "ns.b", "ns.c"] {
            builder = builder.window_section(&windows, node_key, Some(window_label()), true, &entries, window_row).expect("bounded fixture");
        }
        let body = builder.build().expect("bounded fixture");
        let rows: usize = body.children.iter().map(|section| section.children.len()).sum();
        println!("[DEBUG] tree-window-ledger first-paint viewport=500 rows={rows} body_nodes={}", body_nodes(&body));
        assert!(rows <= LEDGER, "a 500-row viewport cannot outspend the ledger: {rows} > {LEDGER}");
        assert!(body_nodes(&body) <= UI_DOCUMENT_NODES, "{} > {UI_DOCUMENT_NODES}", body_nodes(&body));
        assert_eq!(windows.nodes_remaining(), 0);
        for section in body.children.iter() {
            assert_eq!(section_window(section), Some(TreeWindow { total: 200, offset: 0 }));
        }
    }

    /// 🧾️ The framework's own history body obeys the ledger: the Actions section is un-ledgered fixed
    /// rows paid for by [`TREE_WINDOW_FIXED_NODE_HEADROOM`], and the windowed Commands section takes
    /// what is left, never the 300 live commands.
    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_stays_inside_the_body_node_ledger() {
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
            revertible: false,
            count: 1,
            inverse: None,
        };
        let history = HistoryView {
            columns: Vec::new(),
            can_undo: true,
            can_redo: false,
            active_alternative_id: None,
            current_checkpoint_id: None,
            commands: (1..=300).map(entry).collect(),
            command_filter: HistoryCommandFilter::All,
        };
        let view = ViewModel { tree_viewport_rows: Some(500), ..ViewModel::default() };
        let panel = ui_history_panel(&history, "ctrl", false, false, &view).await.expect("a log of any length must assemble");
        let commands = &panel.children[1];
        let fixed = body_nodes(&panel) - commands.children.len() - 1;
        println!("[DEBUG] tree-window-ledger history commands={} fixed={fixed} body_nodes={}", commands.children.len(), body_nodes(&panel));
        assert_eq!(section_window(commands), Some(TreeWindow { total: 300, offset: 0 }), "the scrollbar spans the whole log");
        assert!(commands.children.len() <= LEDGER, "{} > {LEDGER}", commands.children.len());
        assert!(fixed <= TREE_WINDOW_FIXED_NODE_HEADROOM, "the un-ledgered Actions rows fit the reserve: {fixed} > {TREE_WINDOW_FIXED_NODE_HEADROOM}");
        assert!(body_nodes(&panel) <= UI_DOCUMENT_NODES, "{} > {UI_DOCUMENT_NODES}", body_nodes(&panel));
    }

    /// 🧾️ The budget is derived once, in the contract, from the reconciler's own record arena.
    #[semio_framework_async_macros::async_test]
    async fn the_body_node_budget_is_the_record_arena_less_the_root_and_the_fixed_reserve() {
        assert_eq!(TREE_WINDOW_BODY_NODE_BUDGET, UI_DOCUMENT_NODES - 1 - TREE_WINDOW_FIXED_NODE_HEADROOM);
        assert_eq!(TreeWindows::unhosted().nodes_remaining(), TREE_WINDOW_BODY_NODE_BUDGET, "a render opens on the whole budget");
    }

    /// 🧾️ One budget, one cost model, both sides of the wire: the React host caps `Σ(1 + rows)` with
    /// the same number this SDK's ledger spends. The generated contract TS carries no constants, so
    /// the parity is pinned here, against the host element's source.
    #[semio_framework_async_macros::async_test]
    async fn the_host_tree_element_declares_the_same_body_node_budget() {
        let relative = std::path::Path::new("🧰️framework").join("🔨️modules").join("🖱️ui").join("🧱️elements").join("🌳️Tree").join("🟦️.tsx");
        let tsx = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().map(|root| root.join(&relative)).find(|path| path.exists()).expect("the host Tree element lives in this repo");
        let source = std::fs::read_to_string(&tsx).expect("the host Tree element is readable");
        let spelling = format!("TREE_WINDOW_BODY_NODE_BUDGET = {TREE_WINDOW_BODY_NODE_BUDGET};");
        assert!(source.contains(&spelling), "{} must declare `{spelling}` on one line — the host's request cap and this ledger are one budget", tsx.display());
        let separator = format!("TREE_WINDOW_PATH_SEPARATOR = \"{TREE_WINDOW_PATH_SEPARATOR}\";");
        assert!(
            source.contains(&separator) || source.contains("TREE_WINDOW_PATH_SEPARATOR = \"\\u001f\";"),
            "{} must declare TREE_WINDOW_PATH_SEPARATOR as U+001F on one line — a path the host joins is the string the guest addresses containers by",
            tsx.display()
        );
    }

    /// 🧾️ The cost model, exactly: a body costs `1 + rows` per windowed container, every node charged
    /// ONCE. A nested container is one row of its parent, so it must not charge its own node again —
    /// what the ledger spent equals what the body actually built.
    #[semio_framework_async_macros::async_test]
    async fn a_nested_container_is_charged_exactly_once() {
        let groups = window_entries(3);
        let leaves = window_entries(4);
        let windows = TreeWindows::unhosted();
        let section = tree_window_section(&windows, "ns.groups", window_label(), true, &groups, |group| {
            let key = format!("ns.groups.{group}");
            let item = ui::tree_item(window_label()).try_id(&key).map_err(|_| ui_assembly_error("test.group-id"))?;
            tree_window_item(&windows, item, &key, true, &leaves, window_row)
        })
        .expect("bounded fixture");
        let spent = LEDGER - windows.nodes_remaining();
        println!("[DEBUG] tree-window-ledger exactly-once spent={spent} body_nodes={}", body_nodes(&section));
        assert_eq!(body_nodes(&section), 1 + 3 + 3 * 4, "one section, three group rows, four leaves each");
        assert_eq!(spent, body_nodes(&section), "the ledger spends exactly one record per node the body actually built");
    }

    /// 🥇️ `rows: 0` with `open: true` is the host saying "this container is open but off screen":
    /// build its node, stamp its extent, materialise nothing. It must NOT fall back to the first-paint
    /// default, which is what made off-screen containers compete with visible ones.
    #[semio_framework_async_macros::async_test]
    async fn an_open_request_for_zero_rows_materialises_nothing_and_keeps_its_offset() {
        let entries = window_entries(60);
        let view = ViewModel {
            tree_windows: vec![TreeWindowRequest { body_key: "body".to_string(), node_key: "ns.offscreen".to_string(), open: Some(true), offset: 40, rows: 0 }],
            ..ViewModel::default()
        };
        let windows = TreeWindows::for_body(&view, "body");
        assert_eq!(windows.nodes_reserved(), 1, "an off-screen open container reserves its own node and no rows");
        let section = tree_window_section(&windows, "ns.offscreen", window_label(), true, &entries, window_row).expect("bounded fixture");
        assert!(section.children.is_empty(), "a zero-row request materialises nothing, never the first-paint default");
        assert_eq!(section_window(&section), Some(TreeWindow { total: 60, offset: 40 }), "and the host keeps the scroll position it reported");
        assert_eq!(windows.nodes_remaining(), LEDGER - 1);
    }

    /// 🥇️ A stale offset — the document shrank under a request in flight — clamps to the LAST window,
    /// never to an empty slice of a container that has rows to show.
    #[semio_framework_async_macros::async_test]
    async fn a_stale_offset_clamps_to_the_last_window() {
        let entries = window_entries(12);
        let view = ViewModel {
            tree_windows: vec![TreeWindowRequest { body_key: "body".to_string(), node_key: "ns.rows".to_string(), open: Some(true), offset: 900, rows: 5 }],
            ..ViewModel::default()
        };
        let section = tree_window_section(&TreeWindows::for_body(&view, "body"), "ns.rows", window_label(), true, &entries, window_row).expect("bounded fixture");
        assert_eq!(section.children.len(), 5, "a stale offset still shows a full window");
        assert_eq!(section.children[0].key.as_str(), "ns.row.7");
        assert_eq!(section_window(&section), Some(TreeWindow { total: 12, offset: 7 }), "clamped to `total - rows`, so offset + length == total");

        let short = window_entries(3);
        let view = ViewModel {
            tree_windows: vec![TreeWindowRequest { body_key: "body".to_string(), node_key: "ns.rows".to_string(), open: Some(true), offset: 900, rows: 5 }],
            ..ViewModel::default()
        };
        let section = tree_window_section(&TreeWindows::for_body(&view, "body"), "ns.rows", window_label(), true, &short, window_row).expect("bounded fixture");
        assert_eq!(section.children.len(), 3, "a container shorter than the window shows all of it from the top");
        assert_eq!(section_window(&section), Some(TreeWindow { total: 3, offset: 0 }));
    }

    /// 🔑️ One entity id under two different parents is TWO containers, and they must not steer each
    /// other. The window path — parent keys, then own key — is what makes that true by construction,
    /// with the node keys left exactly as the pick target ids the interaction domain registers.
    #[semio_framework_async_macros::async_test]
    async fn one_id_under_two_parents_keeps_two_independent_windows() {
        let cases = window_entries(1);
        let loads = window_entries(30);
        let sep = TREE_WINDOW_PATH_SEPARATOR;
        let view = ViewModel {
            tree_windows: vec![
                TreeWindowRequest { body_key: "body".to_string(), node_key: format!("ns.load-cases{sep}uls"), open: Some(true), offset: 4, rows: 3 },
                TreeWindowRequest { body_key: "body".to_string(), node_key: format!("ns.combinations{sep}uls"), open: Some(false), offset: 0, rows: 0 },
            ],
            ..ViewModel::default()
        };
        let windows = TreeWindows::for_body(&view, "body");
        let mut section = |id: &str| {
            tree_window_section(&windows, id, window_label(), true, &cases, |_| {
                let item = ui::tree_item(window_label()).try_id("uls").map_err(|_| ui_assembly_error("test.uls-id"))?;
                tree_window_item(&windows, item, "uls", true, &loads, window_row)
            })
        };
        let in_cases = section("ns.load-cases").expect("a load case named uls assembles");
        let in_combinations = section("ns.combinations").expect("a combination named uls assembles beside it, not over it");
        let Component::TreeItem(nested_case) = &in_cases.children[0].component else { panic!("expected a nested TreeItem") };
        let Component::TreeItem(nested_combination) = &in_combinations.children[0].component else { panic!("expected a nested TreeItem") };
        assert_eq!(in_cases.children[0].key.as_str(), "uls", "the node key stays the raw pick target id");
        assert_eq!(in_combinations.children[0].key.as_str(), "uls");
        assert_eq!(nested_case.window, Some(TreeWindow { total: 30, offset: 4 }), "the load case honours ITS request");
        assert_eq!(in_cases.children[0].children.len(), 3);
        assert_eq!(in_cases.children[0].children[0].key.as_str(), "ns.row.4");
        assert_eq!(nested_combination.window, Some(TreeWindow { total: 30, offset: 0 }), "the combination is closed by ITS own request and keeps its extent");
        assert!(in_combinations.children[0].children.is_empty(), "a closed container elsewhere in the body is not this one");
    }

    /// 🔑️ A top-level container's path IS its key, so every flat request and law is unchanged, and a
    /// nested path addresses exactly one container.
    #[semio_framework_async_macros::async_test]
    async fn a_window_path_is_the_key_at_the_top_and_the_parent_chain_below() {
        let windows = TreeWindows::unhosted();
        assert_eq!(windows.path_of("ns.rows"), "ns.rows", "a top-level section is addressed by its bare key");
        let entries = window_entries(1);
        let seen = std::cell::RefCell::new(Vec::new());
        tree_window_section(&windows, "a", window_label(), true, &entries, |_| {
            seen.borrow_mut().push(windows.path_of("b"));
            let item = ui::tree_item(window_label()).try_id("b").map_err(|_| ui_assembly_error("test.b-id"))?;
            tree_window_item(&windows, item, "b", true, &entries, |_| {
                seen.borrow_mut().push(windows.path_of("c"));
                window_row(&0)
            })
        })
        .expect("bounded fixture");
        let sep = TREE_WINDOW_PATH_SEPARATOR;
        assert_eq!(*seen.borrow(), vec![format!("a{sep}b"), format!("a{sep}b{sep}c")], "each level prefixes the chain above it");
    }

    /// 🧾️ The headroom is the fleet's measured worst case, not a guess: this rebuilds the fattest
    /// shipped un-windowed block — energy's inspector with a fenestration selected, one windowed
    /// selection list beside a 16-row field form and a 3-row Actions section — and pins that the whole
    /// body still fits the reconciler's record arena.
    #[semio_framework_async_macros::async_test]
    async fn tree_window_headroom_covers_the_fattest_shipped_panel() {
        let selected = window_entries(200);
        let view = ViewModel { tree_viewport_rows: Some(500), ..ViewModel::default() };
        let windows = TreeWindows::for_body(&view, "body");
        let mut builder = PanelTreeBuilder::new("ns").expect("bounded fixture");
        builder = builder.window_section(&windows, "ns.selection", Some(window_label()), true, &selected, window_row).expect("bounded fixture");
        for (id, rows) in [("ns.fenestration", 16usize), ("ns.actions", 3)] {
            let mut fixed = UiFixedList::default();
            for index in 0..rows {
                fixed.try_push(tree_item(format!("{id}.{index}"), format!("Field {index}")).expect("bounded fixture")).expect("bounded fixture");
            }
            builder = builder.section(id, Some(window_label()), true, fixed).expect("bounded fixture");
        }
        let body = builder.build().expect("bounded fixture");
        let un_ledgered = body_nodes(&body) - 1 - (1 + body.children[0].children.len());
        println!("[DEBUG] tree-window-ledger fattest-panel un_ledgered={un_ledgered} body_nodes={}", body_nodes(&body));
        assert_eq!(un_ledgered, 21, "the energy fenestration inspector's own measured figure: 1 + 16 + 1 + 3");
        assert!(un_ledgered <= TREE_WINDOW_FIXED_NODE_HEADROOM, "{un_ledgered} > {TREE_WINDOW_FIXED_NODE_HEADROOM}");
        assert!(body_nodes(&body) <= UI_DOCUMENT_NODES, "a fully spent ledger beside the fattest fixed block still reconciles: {} > {UI_DOCUMENT_NODES}", body_nodes(&body));
        assert_eq!(windows.nodes_remaining(), 0, "the windowed list took the whole ledger and not one record more");
    }

    /// 🔑️ The host keys open state, geometry and windows by the container's window PATH, so a body
    /// that claims one path twice — two TRUE siblings under one parent — would silently steer one
    /// container from the other. The second claim is refused.
    #[semio_framework_async_macros::async_test]
    async fn a_duplicate_node_key_in_one_body_is_refused() {
        let entries = window_entries(4);
        let windows = TreeWindows::unhosted();
        tree_window_section(&windows, "ns.dup", window_label(), true, &entries, window_row).expect("the first claim is the owner");
        let error = tree_window_section(&windows, "ns.dup", window_label(), true, &entries, window_row).expect_err("the second claim is refused");
        assert_eq!(error.code, "ui.tree-window.duplicate-key");

        let nested = TreeWindows::unhosted();
        let Ok(item) = ui::tree_item(window_label()).try_id("ns.item") else { panic!("bounded fixture") };
        tree_window_section(&nested, "ns.shared", window_label(), true, &entries, window_row).expect("bounded fixture");
        let error = tree_window_item(&nested, item, "ns.shared", true, &entries, window_row).expect_err("a nested group may not reuse a section's key");
        assert_eq!(error.code, "ui.tree-window.duplicate-key");

        let empty: Vec<usize> = Vec::new();
        let leaves = TreeWindows::unhosted();
        let Ok(first) = ui::tree_item(window_label()).try_id("ns.leaf") else { panic!("bounded fixture") };
        let Ok(second) = ui::tree_item(window_label()).try_id("ns.leaf") else { panic!("bounded fixture") };
        tree_window_item(&leaves, first, "ns.leaf", true, &empty, window_row).expect("a childless container publishes no window");
        tree_window_item(&leaves, second, "ns.leaf", true, &empty, window_row).expect("so it is not addressable, and two of them cannot steer each other");
        tree_window_section_or_placeholder(&leaves, "ns.empty", window_label(), true, &empty, window_row, "(none)").expect("bounded fixture");
        tree_window_section_or_placeholder(&leaves, "ns.empty", window_label(), true, &empty, window_row, "(none)").expect("the empty-state path publishes no window either");

        let addressed = ViewModel {
            tree_windows: vec![TreeWindowRequest { body_key: "body".to_string(), node_key: "ns.leaf".to_string(), open: Some(true), offset: 0, rows: 4 }],
            ..ViewModel::default()
        };
        let addressed = TreeWindows::for_body(&addressed, "body");
        let Ok(first) = ui::tree_item(window_label()).try_id("ns.leaf") else { panic!("bounded fixture") };
        let Ok(second) = ui::tree_item(window_label()).try_id("ns.leaf") else { panic!("bounded fixture") };
        tree_window_item(&addressed, first, "ns.leaf", true, &empty, window_row).expect("bounded fixture");
        let error = tree_window_item(&addressed, second, "ns.leaf", true, &empty, window_row).expect_err("once the host addresses a key, sharing it is a refusal");
        assert_eq!(error.code, "ui.tree-window.duplicate-key");
    }

    /// 🧾️ A closed container is one record and nothing else, whatever it holds.
    #[semio_framework_async_macros::async_test]
    async fn a_closed_container_costs_exactly_one_node() {
        let entries = window_entries(5_000);
        let windows = TreeWindows::unhosted();
        let section = tree_window_section(&windows, "ns.rows", window_label(), false, &entries, window_row).expect("bounded fixture");
        assert_eq!(body_nodes(&section), 1, "a closed container is its own record and nothing else");
        assert_eq!(windows.nodes_remaining(), LEDGER - 1);
        assert_eq!(section_window(&section), Some(TreeWindow { total: 5_000, offset: 0 }), "and it still publishes the whole extent the host may scroll into");
    }

    /// 🥇️ Request priority — the packet's starvation case. The container the user scrolled to sits
    /// SECOND in document order behind a 200-entry container the host has never addressed. Its window
    /// is seated on the ledger before the body is built, so it is honoured exactly and the first-paint
    /// container takes only what is left over.
    #[semio_framework_async_macros::async_test]
    async fn a_host_request_outranks_a_first_paint_default_earlier_in_document_order() {
        let entries = window_entries(200);
        let view = ViewModel {
            tree_windows: vec![TreeWindowRequest { body_key: "body".to_string(), node_key: "ns.scrolled".to_string(), open: Some(true), offset: 120, rows: 40 }],
            tree_viewport_rows: Some(500),
            ..ViewModel::default()
        };
        let windows = TreeWindows::for_body(&view, "body");
        assert_eq!(windows.nodes_reserved(), 41, "the requested window plus the requested container's own node are held back");
        let mut builder = PanelTreeBuilder::new("ns").expect("bounded fixture");
        for node_key in ["ns.first-paint", "ns.scrolled"] {
            builder = builder.window_section(&windows, node_key, Some(window_label()), true, &entries, window_row).expect("bounded fixture");
        }
        let body = builder.build().expect("bounded fixture");
        let (first_paint, scrolled) = (&body.children[0], &body.children[1]);
        println!("[DEBUG] tree-window-ledger priority first_paint={} scrolled={} body_nodes={}", first_paint.children.len(), scrolled.children.len(), body_nodes(&body));
        assert_eq!(scrolled.children.len(), 40, "the scrolled container is honoured exactly, not starved by the one in front of it");
        assert_eq!(scrolled.children[0].key.as_str(), "ns.row.120");
        assert_eq!(scrolled.children[39].key.as_str(), "ns.row.159");
        assert_eq!(section_window(scrolled), Some(TreeWindow { total: 200, offset: 120 }));
        assert_eq!(first_paint.children.len(), LEDGER - 41 - 1, "a first paint spends only the records no request is holding");
        assert_eq!(section_window(first_paint), Some(TreeWindow { total: 200, offset: 0 }));
        assert!(body_nodes(&body) <= UI_DOCUMENT_NODES, "{} > {UI_DOCUMENT_NODES}", body_nodes(&body));
    }

    /// 🥇️ A host that asks for more rows than a body can hold is clamped deterministically, in the
    /// order it filed the requests: the first requests are seated whole, the tail gets what is left —
    /// zero — and every container still stamps its full extent so the next scroll can stream it.
    #[semio_framework_async_macros::async_test]
    async fn host_requests_beyond_the_ledger_are_clamped_in_request_order() {
        let entries = window_entries(200);
        let keys: Vec<String> = (0..4).map(|index| format!("ns.s{index}")).collect();
        let view = ViewModel {
            tree_windows: keys.iter().map(|node_key| TreeWindowRequest { body_key: "body".to_string(), node_key: node_key.clone(), open: Some(true), offset: 0, rows: UI_BUILT_CHILDREN_MAX as u32 }).collect(),
            ..ViewModel::default()
        };
        let windows = TreeWindows::for_body(&view, "body");
        assert_eq!(windows.nodes_reserved(), LEDGER, "four full-page requests cannot all be seated — the ledger is the ceiling");
        let mut builder = PanelTreeBuilder::new("ns").expect("bounded fixture");
        for node_key in &keys {
            builder = builder.window_section(&windows, node_key, Some(window_label()), true, &entries, window_row).expect("bounded fixture");
        }
        let body = builder.build().expect("bounded fixture");
        let rows: Vec<usize> = body.children.iter().map(|section| section.children.len()).collect();
        println!("[DEBUG] tree-window-ledger clamp rows={rows:?} body_nodes={}", body_nodes(&body));
        assert_eq!(rows, vec![LEDGER - 1, 0, 0, 0], "the first request is seated whole; the clamp falls on the tail");
        for section in body.children.iter() {
            assert_eq!(section_window(section), Some(TreeWindow { total: 200, offset: 0 }), "every container stamps its total, seated or not");
        }
        assert!(body_nodes(&body) <= UI_DOCUMENT_NODES, "{} > {UI_DOCUMENT_NODES}", body_nodes(&body));
    }

    /// 🧾️ fem3d's own shape — a windowed section whose rows are themselves [`tree_window_item`]
    /// containers (load case › loads). The nested containers come off the same ledger as their parent
    /// and the sibling section after them, and the whole body fits the reconciler's record arena.
    #[semio_framework_async_macros::async_test]
    async fn a_nested_tree_window_item_inside_a_window_section_stays_inside_the_arena() {
        let cases = window_entries(40);
        let loads = window_entries(25);
        let view = ViewModel { tree_viewport_rows: Some(500), ..ViewModel::default() };
        let windows = TreeWindows::for_body(&view, "body");
        let body = PanelTreeBuilder::new("ns")
            .expect("bounded fixture")
            .window_section(&windows, "ns.cases", Some(window_label()), true, &cases, |case| {
                let key = format!("ns.case.{case}");
                let item = ui::tree_item(window_label()).try_id(&key).map_err(|_| ui_assembly_error("test.case-id"))?;
                tree_window_item(&windows, item, &key, true, &loads, window_row)
            })
            .expect("bounded fixture")
            .window_section(&windows, "ns.tail", Some(window_label()), true, &cases, window_row)
            .expect("bounded fixture")
            .build()
            .expect("bounded fixture");
        let nested: usize = body.children[0].children.iter().map(|group| group.children.len()).sum();
        println!("[DEBUG] tree-window-ledger nested-section groups={} nested={nested} body_nodes={} remaining={}", body.children[0].children.len(), body_nodes(&body), windows.nodes_remaining());
        assert!(body_nodes(&body) <= UI_DOCUMENT_NODES, "{} > {UI_DOCUMENT_NODES}", body_nodes(&body));
        assert_eq!(section_window(&body.children[0]), Some(TreeWindow { total: 40, offset: 0 }));
        assert_eq!(section_window(&body.children[1]), Some(TreeWindow { total: 40, offset: 0 }), "a container the exhausted ledger could not seat still publishes its extent");
        let Component::TreeItem(group) = &body.children[0].children[0].component else { panic!("expected a nested TreeItem") };
        assert_eq!(group.window, Some(TreeWindow { total: 25, offset: 0 }), "and so does every nested group row");
    }

    /// 🧾️ Only `ui.fixed-capacity` ends a window early; anything else is a real assembly fault and
    /// must reach the caller.
    #[semio_framework_async_macros::async_test]
    async fn a_row_error_that_is_not_fixed_capacity_still_propagates() {
        let entries = window_entries(20);
        let error = tree_window_section(&TreeWindows::unhosted(), "ns.rows", window_label(), true, &entries, |_| Err(PluginAssemblyError::new("test.broken-row", "not a capacity refusal"))).expect_err("a non-capacity error propagates");
        assert_eq!(error.code, "test.broken-row");
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
            revertible: false,
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
        assert_eq!(commands.children.len(), (TREE_WINDOW_DEFAULT_ROWS as usize).min(UI_BUILT_CHILDREN_MAX), "a cold paint materialises one viewport of commands, clamped by the built-children ceiling");
        assert_eq!(section_window(commands), Some(TreeWindow { total: 100, offset: 0 }), "the scrollbar spans the whole log");
        let json = panel_body_json(&panel).to_string();
        assert!(!json.contains(".more"), "no continuation row key survives: {json}");
        assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    }
}
