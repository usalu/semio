mod window_kits_tests {
    use super::*;

    async fn label_en_de(label: &LocalizedLabel) -> (String, String) {
        (label.resolve(Terminology::default(), Locale::En).to_string(), label.resolve(Terminology::default(), Locale::De).to_string())
    }

    #[semio_framework_async_macros::async_test]
    async fn kind_ids_match_the_frozen_table() {
        assert_eq!(TextWindowKit::KIND_ID, "framework.window.text");
        assert_eq!(TableWindowKit::KIND_ID, "framework.window.table");
        assert_eq!(TreeWindowKit::KIND_ID, "framework.window.tree");
        assert_eq!(ImageWindowKit::KIND_ID, "framework.window.image");
        assert_eq!(MeshWindowKit::KIND_ID, "framework.window.mesh");
        assert_eq!(DocumentWindowKit::KIND_ID, "framework.window.document");
        assert_eq!(MediaWindowKit::KIND_ID, "framework.window.media");
    }

    #[semio_framework_async_macros::async_test]
    async fn window_kind_ids_and_labels_are_non_empty_and_id_matches_definition() {
        for (kind_id, def) in [
            (TextWindowKit::KIND_ID, TextWindowKit::window_kind()),
            (TableWindowKit::KIND_ID, TableWindowKit::window_kind()),
            (TreeWindowKit::KIND_ID, TreeWindowKit::window_kind()),
            (ImageWindowKit::KIND_ID, ImageWindowKit::window_kind()),
            (MeshWindowKit::KIND_ID, MeshWindowKit::window_kind()),
            (DocumentWindowKit::KIND_ID, DocumentWindowKit::window_kind()),
            (MediaWindowKit::KIND_ID, MediaWindowKit::window_kind()),
        ] {
            assert_eq!(def.id, kind_id);
            assert!(def.actions.is_empty(), "read-only {kind_id} must declare no actions");
            let (en, de) = label_en_de(&def.label).await;
            assert!(!en.is_empty() && !de.is_empty(), "{kind_id} label must have both en and de");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn en_de_labels_differ_except_text() {
        let (text_en, text_de) = label_en_de(&TextWindowKit::window_kind().label).await;
        assert_eq!(text_en, "Text");
        assert_eq!(text_de, "Text");

        let (table_en, table_de) = label_en_de(&TableWindowKit::window_kind().label).await;
        assert_eq!(table_en, "Table");
        assert_eq!(table_de, "Tabelle");
        assert_ne!(table_en, table_de);

        let (tree_en, tree_de) = label_en_de(&TreeWindowKit::window_kind().label).await;
        assert_eq!(tree_en, "Tree");
        assert_eq!(tree_de, "Baum");
        assert_ne!(tree_en, tree_de);

        let (image_en, image_de) = label_en_de(&ImageWindowKit::window_kind().label).await;
        assert_eq!(image_en, "Image");
        assert_eq!(image_de, "Bild");
        assert_ne!(image_en, image_de);

        let (mesh_en, mesh_de) = label_en_de(&MeshWindowKit::window_kind().label).await;
        assert_eq!(mesh_en, "Mesh");
        assert_eq!(mesh_de, "Netz");
        assert_ne!(mesh_en, mesh_de);

        let (document_en, document_de) = label_en_de(&DocumentWindowKit::window_kind().label).await;
        assert_eq!(document_en, "Artifact");
        assert_eq!(document_de, "Artefakt");
        assert_ne!(document_en, document_de);

        let (media_en, media_de) = label_en_de(&MediaWindowKit::window_kind().label).await;
        assert_eq!(media_en, "Media");
        assert_eq!(media_de, "Medien");
        assert_ne!(media_en, media_de);
    }

    #[semio_framework_async_macros::async_test]
    async fn editable_variants_declare_exactly_their_frozen_command_id() {
        let cases: [(&str, WindowKindDefinition); 7] = [
            ("replace-text", TextWindowKit::editable_window_kind()),
            ("set-cell", TableWindowKit::editable_window_kind()),
            ("set-node", TreeWindowKit::editable_window_kind()),
            ("set-pixel-region", ImageWindowKit::editable_window_kind()),
            ("set-vertex", MeshWindowKit::editable_window_kind()),
            ("set-page", DocumentWindowKit::editable_window_kind()),
            ("seek-media", MediaWindowKit::editable_window_kind()),
        ];
        for (command_id, def) in cases {
            assert_eq!(def.actions.len(), 1, "{command_id} editable kind must declare exactly one action");
            assert_eq!(def.actions[0].id, command_id);
            assert_eq!(def.actions[0].kind, ActionKind::Mutation);
            assert_eq!(def.actions[0].semantics.execution.interactive_job, semio_framework::InteractiveJobClassification::Migrated);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn text_kit_renders_buffer_into_component_scene() {
        let view = TextView { text: "hello world".into(), language: Some("en".into()), read_only: false };
        let node = TextWindowKit::render(&view).expect("bounded fixture");
        let Component::Surface(props) = node.component else { panic!("expected Surface") };
        let expected = semio_framework_ui_scene::TextEditorScene {
            buffer: "hello world".into(),
            language: Some("en".into()),
            selection_json: None,
            tokens_json: None,
            diagnostics_json: None,
            completions_json: None,
            overlays_json: None,
            occurrences_json: None,
            placeholders_json: None,
            extra_carets_json: None,
            selectable_spans_json: None,
            settings_json: None,
            camera_json: None,
            hover_json: None,
            newline_gates_json: None,
            rename_json: None,
        };
        assert_eq!(props, semio_framework_ui_scene::encode(SurfaceKind::TextEditor, &expected).expect("bounded fixture"));
    }

    #[semio_framework_async_macros::async_test]
    async fn text_kit_read_only_stamps_settings_json() {
        let view = TextView { text: "x".into(), language: None, read_only: true };
        let node = TextWindowKit::render(&view).expect("bounded fixture");
        let Component::Surface(props) = node.component else { panic!("expected Surface") };
        let expected = semio_framework_ui_scene::TextEditorScene {
            buffer: "x".into(),
            language: None,
            selection_json: None,
            tokens_json: None,
            diagnostics_json: None,
            completions_json: None,
            overlays_json: None,
            occurrences_json: None,
            placeholders_json: None,
            extra_carets_json: None,
            selectable_spans_json: None,
            settings_json: Some("{\"readOnly\":true}".to_string()),
            camera_json: None,
            hover_json: None,
            newline_gates_json: None,
            rename_json: None,
        };
        assert_eq!(props, semio_framework_ui_scene::encode(SurfaceKind::TextEditor, &expected).expect("bounded fixture"));
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_renders_columns_and_rows_json() {
        let view = TableView { columns: vec!["a".into(), "b".into()], rows: vec![vec!["1".into(), "2".into()]] };
        let node = TableWindowKit::render(&view).expect("bounded fixture");
        let Component::Surface(props) = node.component else { panic!("expected Surface") };
        let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(&props).expect("table scene");
        assert_eq!(scene.columns_json, r#"[{"id":"0","label":"a"},{"id":"1","label":"b"}]"#);
        assert_eq!(scene.rows_json, r#"[{"0":"1","1":"2","id":"0"}]"#);
    }

    fn table_fixture_row(index: &usize) -> UiAssemblyResult<BuiltNode> {
        let open = || ActionId::try_v1("s.space.home", "openSpace").expect("bounded action");
        let key = format!("space:{index}");
        let name = format!("Studio {index}");
        table_window_row(&key, &[name.as_str(), "atelier"], [table_row_action(IconName::FolderOpen.as_str(), "Open", (open(), None))?], Some((open(), None)))
    }

    fn table_fixture(windows: &TreeWindows<'_>, total: usize) -> BuiltNode {
        let entries: Vec<usize> = (0..total).collect();
        TableWindowKit::render_rows(windows, "Studios", &["Name", "Kind"], Some("Actions"), &entries, table_fixture_row).expect("windowed table")
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_render_rows_builds_one_table_node_with_one_record_per_row() {
        let node = table_fixture(&TreeWindows::unhosted(), 3);
        let Component::Table(props) = &node.component else { panic!("expected Table") };
        assert_eq!(node.key.as_str(), TableWindowKit::KIND_ID);
        assert_eq!(props.label.0.as_str(), "Studios");
        assert_eq!(props.columns.iter().map(|column| column.0.as_str()).collect::<Vec<_>>(), ["Name", "Kind"]);
        assert_eq!(props.actions_label.as_ref().map(|label| label.0.as_str()), Some("Actions"));
        assert_eq!(props.window.map(|window| (window.total, window.offset)), Some((3, 0)));
        assert_eq!(node.children.len(), 3);
        let row = node.children.get(1).expect("second row");
        assert_eq!(row.key.as_str(), "space:1");
        let Component::TableRow(row_props) = &row.component else { panic!("expected TableRow") };
        assert_eq!(row_props.cells.iter().map(|cell| cell.as_str()).collect::<Vec<_>>(), ["Studio 1", "atelier"]);
        assert!(row.children.is_empty(), "cells and row actions are props, never child records");
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_render_rows_carries_row_actions_and_the_row_activation_as_props() {
        let node = table_fixture(&TreeWindows::unhosted(), 1);
        let row = node.children.get(0).expect("row");
        let Component::TableRow(props) = &row.component else { panic!("expected TableRow") };
        let action = props.row_actions.get(0).expect("row action");
        assert_eq!(action.action.action.name.as_str(), "openSpace");
        assert_eq!(action.label.as_ref().map(|label| label.0.as_str()), Some("Open"));
        let activate = row.bindings.iter().find(|binding| binding.trigger == Trigger::Activate).expect("row activation");
        assert_eq!(activate.action.name.as_str(), "openSpace");
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_render_rows_serves_exactly_the_hosts_window_on_the_shared_ledger() {
        let view = ViewModel { tree_windows: vec![TreeWindowRequest { body_key: "body".to_string(), node_key: TableWindowKit::KIND_ID.to_string(), open: Some(true), offset: 200, rows: 20 }], ..Default::default() };
        let windows = TreeWindows::for_body(&view, "body");
        let node = table_fixture(&windows, 500);
        let Component::Table(props) = &node.component else { panic!("expected Table") };
        assert_eq!(props.window.map(|window| (window.total, window.offset)), Some((500, 200)));
        assert_eq!(node.children.len(), 20);
        assert_eq!(node.children.get(0).expect("first served row").key.as_str(), "space:200");
        assert_eq!(windows.nodes_remaining(), TREE_WINDOW_BODY_NODE_BUDGET - 21, "the table and its rows are charged to the body ledger the tree panels spend");
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_first_paint_stays_inside_the_body_node_budget_at_any_row_count() {
        let windows = TreeWindows::unhosted();
        let node = table_fixture(&windows, 10_000);
        let Component::Table(props) = &node.component else { panic!("expected Table") };
        assert_eq!(props.window.map(|window| window.total), Some(10_000));
        assert!(node.children.len() + 1 <= TREE_WINDOW_BODY_NODE_BUDGET, "a first paint of 10 000 rows builds {} records", node.children.len() + 1);
        assert_eq!(node.children.len(), TREE_WINDOW_DEFAULT_ROWS as usize, "an unhosted first paint serves one default viewport of rows");
    }

    /// 📊️ A Home-shaped row: six cells, five row actions and a row activation, each binding carrying its own
    /// `spaceId` argument map.
    fn heavy_table_row(index: &usize) -> UiAssemblyResult<BuiltNode> {
        let key = format!("space:{index}");
        let action = |name: &str| -> UiAssemblyResult<(ActionId, Option<UiValue>)> {
            let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("fixture.args"))?;
            args.push("spaceId".to_owned(), UiValue::Text(UiText::try_from_str(&key).ok_or_else(|| ui_assembly_error("fixture.arg"))?)).map_err(|_| ui_assembly_error("fixture.arg"))?;
            Ok((ActionId::try_v1("s.space.home", name).expect("bounded action"), Some(UiValue::Map(args.finish()))))
        };
        let name = format!("Studio {index}");
        let actions = [("folder-open", "openSpace"), ("pencil", "renameSpace"), ("link", "shareSpace"), ("trash-2", "deleteSpace"), ("users", "manageSpace")].into_iter().map(|(icon, verb)| table_row_action(icon, verb, action(verb)?)).collect::<UiAssemblyResult<Vec<_>>>()?;
        table_window_row(&key, &[name.as_str(), "Atelier", "Private", "1", "2026-09-25 23:05", "Hub"], actions, Some(action("openSpace")?))
    }

    fn requested(rows: u32) -> ViewModel {
        ViewModel { tree_windows: vec![TreeWindowRequest { body_key: "body".to_string(), node_key: TableWindowKit::KIND_ID.to_string(), open: Some(true), offset: 0, rows }], ..Default::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_ends_a_window_of_heavy_rows_inside_the_body_item_budget() {
        let view = requested(100);
        let windows = TreeWindows::for_body(&view, "body");
        let entries: Vec<usize> = (0..500).collect();
        let node = TableWindowKit::render_rows(&windows, "Studios", &["Name", "Kind", "Visibility", "Members", "Updated", "Origin"], Some("Actions"), &entries, heavy_table_row).expect("windowed table");
        let Component::Table(props) = &node.component else { panic!("expected Table") };
        let rows = node.children.len();
        assert_eq!(props.window.map(|window| (window.total, window.offset)), Some((500, 0)), "a shortened window still spans the whole logical table");
        assert!(rows > 0 && rows < 100, "heavy rows end the window early: {rows} of 100 requested");
        let priced = semio_framework_ui_runtime::surface_subtree_items(&node).expect("bounded census");
        assert!(priced <= TREE_WINDOW_BODY_ITEM_BUDGET + TREE_WINDOW_FIXED_NODE_ITEMS, "{rows} rows priced {priced} items against a body budget of {TREE_WINDOW_BODY_ITEM_BUDGET}");
        assert_eq!(windows.nodes_remaining(), TREE_WINDOW_BODY_NODE_BUDGET - 1 - rows, "the records of the rows that were not built go back to the ledger");
        assert!(windows.items_remaining() < semio_framework_ui_runtime::surface_subtree_items(&heavy_table_row(&0).expect("heavy row")).expect("bounded census"), "the window ends only when the next row no longer fits");
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_serves_a_whole_window_of_light_rows() {
        let view = requested(60);
        let windows = TreeWindows::for_body(&view, "body");
        let node = table_fixture(&windows, 500);
        assert_eq!(node.children.len(), 60, "rows well inside the item budget are all materialised");
        assert!(windows.items_remaining() > 0);
    }

    #[semio_framework_async_macros::async_test]
    async fn tree_kit_renders_nested_items() {
        let view = TreeView { roots: vec![TreeNodeView { id: "root".into(), label: "Root".into(), children: vec![TreeNodeView { id: "child".into(), label: "Child".into(), children: Vec::new() }] }] };
        let tree = TreeWindowKit::render(&view).expect("bounded fixture");
        assert!(matches!(tree.component, Component::Tree(_)));
        assert_eq!(tree.children.len(), 1);
        let section = &tree.children[0];
        assert!(matches!(section.component, Component::TreeSection(_)));
        let root_item = &section.children[0];
        assert_eq!(root_item.key.as_str(), "root");
        assert_eq!(root_item.children[0].key.as_str(), "child");
    }

    #[semio_framework_async_macros::async_test]
    async fn image_kit_renders_data_uri_from_base64() {
        let view = ImageView { width: 4, height: 2, mime: "image/png".into(), base64: "QUJD".into() };
        let node = ImageWindowKit::render(&view).expect("bounded fixture");
        let Component::Image(image) = node.component else { panic!("expected Image") };
        assert_eq!(image.src.as_str(), "data:image/png;base64,QUJD");
    }

    /// 🖼️ A composite larger than one `UiText` renders through pages instead of refusing the window.
    ///
    /// `UI_TEXT_MAX_BYTES` is 512, so a real composited PNG's `data:` URI never fitted the image node's
    /// `src` and EVERY raster viewer window died at assembly with `ui.fixed-capacity …
    /// image-window.source` — not only in tests, the pane's `Viewer` mode could not assemble at all
    /// (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, raster §3). A payload whose size is a
    /// document property must page out of the doc, the way `scene_surface` pages a world scene's lanes.
    #[semio_framework_async_macros::async_test]
    async fn image_kit_pages_a_composite_larger_than_one_ui_text_out_of_the_doc() {
        let base64 = "Q".repeat(128 * UI_TEXT_MAX_BYTES);
        let view = ImageView { width: 512, height: 512, mime: "image/png".into(), base64: base64.clone() };
        let expected = format!("data:image/png;base64,{base64}");
        assert!(expected.len() > UI_TEXT_MAX_BYTES, "the fixture composite really is beyond one UiText");

        let node = ImageWindowKit::render(&view).expect("a composite beyond one UiText still assembles its window");
        assert_eq!(node.key.as_str(), ImageWindowKit::KIND_ID, "the window body key is unchanged by paging");
        assert_eq!(node.children.len(), 2, "the paged window is the image node plus exactly one payload carrier");

        let image_node = node.children.get(0).expect("image node");
        assert_eq!(image_node.key.as_str(), IMAGE_WINDOW_PAGED_NODE_KEY);
        let Component::Image(image) = &image_node.component else { panic!("expected Image") };
        assert_eq!(image.src.as_str(), IMAGE_WINDOW_PIXELS_LANE_KEY, "the src names the lane the pixels ride in");
        assert_eq!(image.alt.as_ref().map(|alt| alt.0.as_str()), Some("512x512"), "the accessible name survives paging");

        let carrier = node.children.get(1).expect("payload carrier");
        assert_eq!(carrier.key.as_str(), IMAGE_WINDOW_PIXELS_LANE_KEY);
        assert!(carrier.children.len() > 1, "the composite really is paged: one packed leaf holds UI_TEXT_MAX_BYTES * (1 + UI_FIXED_LIST_ITEMS) bytes, this fixture needs several");
        assert_eq!(artifact_app_laws::built_carrier_text(carrier), expected, "the exact data URI reassembles out of the carrier's pages");
    }

    #[semio_framework_async_macros::async_test]
    async fn mesh_kit_renders_world3d_component_scene() {
        let view = MeshView { camera_json: "{}".into(), meshes_json: "[]".into(), instances_json: "[]".into(), selection_json: "[]".into() };
        let node = MeshWindowKit::render(&view).expect("bounded fixture");
        let Component::Surface(props) = node.component else { panic!("expected Surface") };
        let scene: semio_framework_ui_scene::World3dScene = semio_framework_ui_scene::decode(&props).expect("world_3d scene");
        assert_eq!(scene.camera_json.as_str(), "{}");
    }

    #[semio_framework_async_macros::async_test]
    async fn document_kit_renders_one_child_per_page() {
        let view = DocumentView { pages: vec![DocumentPage { text: "p1".into() }, DocumentPage { text: "p2".into() }] };
        let stack = DocumentWindowKit::render(&view).expect("bounded fixture");
        assert!(matches!(stack.layout, LayoutSpec::Stack(_)));
        assert_eq!(stack.children.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn media_kit_renders_duration_and_position() {
        let view = MediaView { duration_ms: 60_000, position_ms: 1_500, kind: MediaKind::Video };
        let node = MediaWindowKit::render(&view).expect("bounded fixture");
        let Component::KeyValueList(key_value) = node.component else { panic!("expected KeyValueList") };
        assert_eq!(key_value.entries.len(), 3);
        assert_eq!(key_value.entries[0].value.as_str(), "60000");
        assert_eq!(key_value.entries[1].value.as_str(), "1500");
        assert_eq!(key_value.entries[2].value.as_str(), "video");
    }
}
