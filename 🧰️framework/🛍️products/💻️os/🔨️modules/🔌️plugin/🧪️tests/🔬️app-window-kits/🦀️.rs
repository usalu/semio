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
        let cases: [(&str, WindowKindDefinition); 6] = [
            ("replace-text", TextWindowKit::editable_window_kind()),
            ("set-cell", TableWindowKit::editable_window_kind()),
            ("set-node", TreeWindowKit::editable_window_kind()),
            ("set-pixel-region", ImageWindowKit::editable_window_kind()),
            ("set-vertex", MeshWindowKit::editable_window_kind()),
            ("set-page", DocumentWindowKit::editable_window_kind()),
        ];
        for (command_id, def) in cases {
            assert_eq!(def.actions.len(), 1, "{command_id} editable kind must declare exactly one action");
            assert_eq!(def.actions[0].id, command_id);
            assert_eq!(def.actions[0].kind, ActionKind::Mutation);
            assert_eq!(def.actions[0].semantics.execution.interactive_job, semio_framework::InteractiveJobClassification::Migrated);
        }
        assert!(MediaWindowKit::editable_window_kind().actions.is_empty(), "media transport state is host-local and never an artifact mutation");
    }

    #[semio_framework_async_macros::async_test]
    async fn text_kit_renders_buffer_into_component_scene() {
        let view = TextView { text: "hello world".into(), language: Some("en".into()), read_only: false };
        let node = TextWindowKit::render(&view).expect("bounded fixture");
        let Component::Surface(props) = node.component else { panic!("expected Surface") };
        let expected = semio_framework_ui_scene::TextEditorScene {
            buffer: "hello world".into(),
            lanes: Vec::new(),
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
        let (spine, _) = semio_framework_ui_scene::SceneDoc::split_lanes(&expected);
        assert_eq!(props, semio_framework_ui_scene::encode(SurfaceKind::TextEditor, &spine).expect("bounded fixture"));
    }

    #[semio_framework_async_macros::async_test]
    async fn text_kit_read_only_stamps_settings_json() {
        let view = TextView { text: "x".into(), language: None, read_only: true };
        let node = TextWindowKit::render(&view).expect("bounded fixture");
        let Component::Surface(props) = node.component else { panic!("expected Surface") };
        let expected = semio_framework_ui_scene::TextEditorScene {
            buffer: "x".into(),
            lanes: Vec::new(),
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
        let (spine, _) = semio_framework_ui_scene::SceneDoc::split_lanes(&expected);
        assert_eq!(props, semio_framework_ui_scene::encode(SurfaceKind::TextEditor, &spine).expect("bounded fixture"));
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_renders_columns_and_rows_json() {
        let view = TableView { columns: vec!["a".into(), "b".into()], rows: vec![vec!["1".into(), "2".into()]] };
        let node = TableWindowKit::render(&view).expect("bounded fixture");
        let Component::Surface(props) = &node.component else { panic!("expected Surface") };
        let mut scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(props).expect("table scene");
        for carrier in &node.children {
            assert!(semio_framework_ui_scene::SceneDoc::merge_lane(&mut scene, carrier.key.as_str(), artifact_app_laws::built_carrier_text(carrier)));
        }
        assert_eq!(scene.columns_json, r#"[{"id":"0","label":"a"},{"id":"1","label":"b"}]"#);
        assert_eq!(scene.rows_json, r#"[{"0":"1","1":"2","id":"0"}]"#);
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_editable_cells_preserve_stable_address_and_revision_arguments() {
        let mut args = UiMapBuilder::try_new().expect("argument map");
        args.try_insert("sheetName".into(), UiValue::Text(UiText::try_from_str("Sheet 1").unwrap())).unwrap();
        args.try_insert("row".into(), UiValue::Number(41.0)).unwrap();
        args.try_insert("column".into(), UiValue::Number(7.0)).unwrap();
        args.try_insert("revision".into(), UiValue::Text(UiText::try_from_str("0123456789abcdef").unwrap())).unwrap();
        let view = TableView { columns: vec!["Sheet".into(), "Value".into()], rows: vec![vec!["Sheet 1".into(), "before".into()]] };
        let node = TableWindowKit::render_editable_cells(&view, "s.stdio.xlsx@ecma-376/*#editor", &[EditableTableCell::new(0, 1, "set-cell", UiValue::Map(args.finish()))]).expect("editable table scene");
        let Component::Surface(props) = &node.component else { panic!("expected Surface") };
        let scene: semio_framework_ui_scene::TableScene = semio_framework_ui_scene::decode(props).expect("table scene");
        let rows: serde_json::Value = serde_json::from_str(&scene.rows_json).expect("rows json");
        assert_eq!(rows[0]["1"]["kind"], "editableText");
        assert_eq!(rows[0]["1"]["action"]["args"]["sheetName"], "Sheet 1");
        assert_eq!(rows[0]["1"]["action"]["args"]["row"], 41);
        assert_eq!(rows[0]["1"]["action"]["args"]["column"], 7);
        assert_eq!(rows[0]["1"]["action"]["args"]["revision"], "0123456789abcdef");
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_pages_large_tables_without_losing_rows() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️table-lanes/🔣️.json")).unwrap();
        let count = fixture["rowCount"].as_u64().unwrap() as usize;
        let value = fixture["row"]["0"].as_str().unwrap();
        let view = TableView { columns: vec!["Name".into()], rows: (0..count).map(|_| vec![value.into()]).collect() };
        let node = TableWindowKit::render(&view).expect("large table assembles");
        let rows = node.children.iter().find(|child| child.key.as_str() == semio_framework_ui_scene::TABLE_ROWS_LANE_KEY).expect("paged row carrier");
        let decoded: serde_json::Value = serde_json::from_str(&artifact_app_laws::built_carrier_text(rows)).unwrap();
        assert_eq!(decoded.as_array().unwrap().len(), count);
        assert_eq!(decoded[count - 1]["0"], value);
        assert!(rows.children.len() > 1);
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
    async fn table_kit_windowed_editable_rows_materialize_only_the_requested_slice() {
        let view = ViewModel { tree_windows: vec![TreeWindowRequest { body_key: "body".to_string(), node_key: TableWindowKit::KIND_ID.to_string(), open: Some(true), offset: 200, rows: 3 }], locale: Locale::En, ..Default::default() };
        let windows = TreeWindows::for_body(&view, "body");
        let node = TableWindowKit::render_indexed_rows(&windows, "Values", &["Value"], None, 500, |row| {
            let mut args = UiMapBuilder::try_new().expect("arguments");
            args.try_insert("row".into(), UiValue::Number(row as f64)).expect("row");
            args.try_insert("column".into(), UiValue::Number(0.0)).expect("column");
            args.try_insert("revision".into(), UiValue::Text(UiText::try_from_str("0123456789abcdef").unwrap())).expect("revision");
            let mut remove_args = UiMapBuilder::try_new().expect("remove arguments");
            remove_args.try_insert("row".into(), UiValue::Number(row as f64)).expect("row");
            remove_args.try_insert("revision".into(), UiValue::Text(UiText::try_from_str("0123456789abcdef").unwrap())).expect("revision");
            let remove = table_row_action("trash-2", "Remove row", (ActionId::try_v1("s.stdio.csv@rfc4180/*#editor", "remove-row").expect("action"), Some(UiValue::Map(remove_args.finish()))))?;
            editable_table_window_row(&format!("row-{row}"), "s.stdio.csv@rfc4180/*#editor", Locale::En, [WindowedEditableTableCell::new(format!("value-{row}"), "Value", "set-cell", UiValue::Map(args.finish()))], [remove])
        })
        .expect("windowed editable table");
        let Component::Table(props) = &node.component else { panic!("expected table") };
        assert_eq!(props.window.map(|window| (window.total, window.offset)), Some((500, 200)));
        assert_eq!(node.children.len(), 3);
        assert_eq!(node.children[0].key.as_str(), "row-200");
        let input = &node.children[0].children[0];
        let Component::Input(input_props) = &input.component else { panic!("editable row cell is an input") };
        assert_eq!(input_props.value.as_str(), "value-200");
        let binding = input.bindings.iter().find(|binding| binding.trigger == Trigger::Commit).expect("commit binding");
        let Some(UiValue::Map(arguments)) = &binding.args else { panic!("static cell address is a map") };
        assert!(matches!(arguments.iter().find_map(|(key,value)|(key.as_str()=="row").then_some(value)), Some(UiValue::Number(value)) if value == 200.0));
        let Component::TableRow(row_props) = &node.children[0].component else { panic!("editable row is a TableRow") };
        let mut row_actions = row_props.row_actions.iter();
        let (Some(action), None) = (row_actions.next(), row_actions.next()) else { panic!("the row carries exactly its remove action, as a prop") };
        assert_eq!(action.label.as_ref().map(|label| label.0.as_str()), Some("Remove row"));
        let binding = &action.action;
        assert_eq!(binding.trigger, Trigger::Activate);
        assert_eq!(binding.action.name.as_str(), "remove-row");
        let Some(UiValue::Map(arguments)) = &binding.args else { panic!("remove address is a map") };
        assert!(matches!(arguments.iter().find_map(|(key,value)|(key.as_str()=="row").then_some(value)), Some(UiValue::Number(value)) if value == 200.0));
        assert!(matches!(arguments.iter().find_map(|(key,value)|(key.as_str()=="revision").then_some(value)), Some(UiValue::Text(value)) if value.as_str() == "0123456789abcdef"));
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_projects_independent_row_and_column_windows_with_logical_addresses() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🪟️window-kits/📊️table/🧫️fixtures/↔️two-axis/🔣️.json")).expect("two-axis fixture");
        let row_total = fixture["rowTotal"].as_u64().unwrap() as usize;
        let column_total = fixture["columnTotal"].as_u64().unwrap() as usize;
        let row_offset = fixture["rowOffset"].as_u64().unwrap() as u32;
        let row_count = fixture["rowCount"].as_u64().unwrap() as u32;
        let column_offset = fixture["columnOffset"].as_u64().unwrap() as u32;
        let column_count = fixture["columnCount"].as_u64().unwrap() as u32;
        let view = ViewModel {
            tree_windows: vec![
                TreeWindowRequest { body_key: "body".into(), node_key: TableWindowKit::KIND_ID.into(), open: Some(true), offset: row_offset, rows: row_count },
                TreeWindowRequest { body_key: "body".into(), node_key: table_column_window_key(TableWindowKit::KIND_ID), open: Some(true), offset: column_offset, rows: column_count },
            ],
            locale: Locale::En,
            ..Default::default()
        };
        let windows = TreeWindows::for_body(&view, "body");
        let node = TableWindowKit::render_indexed_matrix_with_id(
            &windows,
            TableWindowKit::KIND_ID,
            "Matrix",
            "Row",
            "Column",
            column_total,
            |column| Label::try_from(format!("Column {}", column + 1)).map_err(|_| ui_assembly_error("fixture.column")),
            None,
            row_total,
            |row, columns| {
                let offset = columns.start;
                let cells = columns
                    .map(|column| {
                        let mut args = UiMapBuilder::try_new().expect("arguments");
                        args.try_insert("row".into(), UiValue::Number(row as f64)).expect("row");
                        args.try_insert("column".into(), UiValue::Number(column as f64)).expect("column");
                        Ok(WindowedEditableTableCell::new(format!("r{row}c{column}"), format!("Column {}", column + 1), "set-cell", UiValue::Map(args.finish())))
                    })
                    .collect::<UiAssemblyResult<Vec<_>>>()?;
                editable_table_window_row_at(&format!("row-{row}"), "s.stdio.csv@rfc4180/*#editor", Locale::En, offset, cells, Vec::new())
            },
        )
        .expect("two-axis table");
        let Component::Table(props) = &node.component else { panic!("table") };
        assert_eq!(props.window.map(|window| (window.total, window.offset)), Some((row_total as u32, row_offset)));
        assert_eq!(props.column_window.map(|window| (window.total, window.offset)), Some((column_total as u32, column_offset)));
        assert_eq!(props.row_label.as_ref().map(|label| label.0.as_str()), Some("Row"));
        assert_eq!(props.column_label.as_ref().map(|label| label.0.as_str()), Some("Column"));
        assert_eq!(props.columns.iter().map(|label| label.0.as_str()).collect::<Vec<_>>(), ["Column 701", "Column 702", "Column 703"]);
        assert_eq!(node.children.iter().map(|row| row.key.as_str()).collect::<Vec<_>>(), ["row-400", "row-401"]);
        let row = &node.children[0];
        assert_eq!(row.children.iter().map(|cell| cell.key.as_str()).collect::<Vec<_>>(), ["cell-700", "cell-701", "cell-702"]);
        let Component::Input(input) = &row.children[2].component else { panic!("input") };
        assert_eq!(input.value.as_str(), "r400c702");
        let binding = row.children[2].bindings.iter().find(|binding| binding.trigger == Trigger::Commit).expect("commit");
        let Some(UiValue::Map(args)) = &binding.args else { panic!("args") };
        assert!(matches!(args.iter().find_map(|(key, value)| (key.as_str() == "row").then_some(value)), Some(UiValue::Number(value)) if value == 400.0));
        assert!(matches!(args.iter().find_map(|(key, value)| (key.as_str() == "column").then_some(value)), Some(UiValue::Number(value)) if value == 702.0));
        assert_eq!(fixture["cells"][0][2], "r400c702");
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
        let actions = [("folder-open", "openSpace"), ("pencil", "renameSpace"), ("link", "shareSpace"), ("trash-2", "deleteSpace"), ("users", "manageSpace")]
            .into_iter()
            .map(|(icon, verb)| table_row_action(icon, verb, action(verb)?))
            .collect::<UiAssemblyResult<Vec<_>>>()?;
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

    /// 🏠️ Row actions travel ONLY as `RowAction` props — the one representation every renderer paints in the table's
    /// trailing actions column — never as `row-action-<i>` child records. A Home-shaped row (six cells, five labelled row
    /// actions and a row activation, each binding carrying its own argument map) is census-priced at ~120 reconcile items,
    /// so the body budget serves ⌊budget / row cost⌋ rows of the default window (27 of 48, measured 2026-09-29) and the
    /// window stamp carries the full extent: the host caps its viewport at the served capacity and pages the rest
    /// (ticket 26/09/23 S18, host-side). Filling all 48 at once needs a cheaper row representation or a larger body budget. Fixture and schema
    /// `🪟️window-kits/📊️table/🧫️fixtures/🏠️row-capacity`; oracle: the rows read back are compared with the fixture as
    /// serde_json values.
    #[semio_framework_async_macros::async_test]
    async fn home_shaped_rows_carry_actions_as_props_and_fill_the_default_window() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🪟️window-kits/📊️table/🧫️fixtures/🏠️row-capacity/🔣️.json")).expect("row-capacity fixture");
        let text = |value: &serde_json::Value| value.as_str().expect("fixture text").to_owned();
        let controller = text(&fixture["controller"]);
        let row = &fixture["row"];
        let cells = row["cells"].as_array().expect("cells").iter().map(text).collect::<Vec<_>>();
        let actions = row["actions"].as_array().expect("actions");
        let argument = text(&row["argument"]);
        let id_length = row["idLength"].as_u64().expect("id length") as usize;
        let columns = fixture["columns"].as_array().expect("columns").iter().map(text).collect::<Vec<_>>();
        let build = |index: &usize| -> UiAssemblyResult<BuiltNode> {
            let id = format!("{index:0id_length$}");
            let bind = |verb: &str| -> UiAssemblyResult<(ActionId, Option<UiValue>)> {
                let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("fixture.args"))?;
                args.push(argument.clone(), UiValue::Text(UiText::try_from_str(&id).ok_or_else(|| ui_assembly_error("fixture.arg"))?)).map_err(|_| ui_assembly_error("fixture.arg"))?;
                Ok((ActionId::try_v1(&controller, verb).ok_or_else(|| ui_assembly_error("fixture.action"))?, Some(UiValue::Map(args.finish()))))
            };
            let row_actions = actions.iter().map(|action| table_row_action(action["icon"].as_str().expect("icon"), action["label"].as_str().expect("label"), bind(action["action"].as_str().expect("verb"))?)).collect::<UiAssemblyResult<Vec<_>>>()?;
            table_window_row(&format!("{}{id}", text(&row["keyPrefix"])), &cells.iter().map(String::as_str).collect::<Vec<_>>(), row_actions, Some(bind(row["activation"].as_str().expect("activation"))?))
        };
        let rows = fixture["window"]["rows"].as_u64().expect("window rows") as u32;
        assert_eq!(rows, TREE_WINDOW_DEFAULT_ROWS, "the fixture pins the host's default window");
        let view = requested(rows);
        let windows = TreeWindows::for_body(&view, "body");
        let entries: Vec<usize> = (0..fixture["window"]["total"].as_u64().expect("total") as usize).collect();
        let node = TableWindowKit::render_rows(&windows, &text(&fixture["tableLabel"]), &columns.iter().map(String::as_str).collect::<Vec<_>>(), Some(text(&fixture["actionsLabel"]).as_str()), &entries, &build).expect("windowed table");
        let row_cost = semio_framework_ui_runtime::surface_subtree_items(node.children.iter().next().expect("a served Home row")).expect("row census");
        let admitted = (TREE_WINDOW_BODY_ITEM_BUDGET / row_cost).min(rows as usize);
        assert_eq!(node.children.len(), admitted, "the window serves every Home-shaped row the body's reconcile budget admits");
        let Component::Table(table) = &node.component else { panic!("expected Table") };
        assert_eq!(table.window.map(|window| (window.total, window.offset)), Some((entries.len() as u32, 0)), "the stamp carries the full extent, so the host pages the rest (S18's served-capacity cap)");
        for row_node in node.children.iter() {
            assert_eq!(row_node.children.len() as u64, fixture["expect"]["rowChildren"].as_u64().expect("row children"), "{}: cells and row actions are props, never child records", row_node.key.as_str());
            let Component::TableRow(props) = &row_node.component else { panic!("expected TableRow") };
            assert_eq!(serde_json::Value::from(props.cells.iter().map(|cell| cell.as_str()).collect::<Vec<_>>()), row["cells"], "cells read back as the fixture's");
            let read_back = props.row_actions.iter().map(|action| serde_json::json!({ "icon": action.icon.as_str(), "label": action.label.as_ref().map(|label| label.0.as_str()), "action": action.action.action.name.as_str() })).collect::<Vec<_>>();
            assert_eq!(serde_json::Value::from(read_back), row["actions"], "every row action keeps its icon, accessible label and verb");
        }
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
    async fn tree_kit_editable_rows_bind_stable_static_args_and_commit_the_value() {
        let view = TreeView { roots: vec![TreeNodeView { id: "root".into(), label: "Root".into(), children: Vec::new() }] };
        let mut arguments = UiMapBuilder::try_new().expect("map builder");
        arguments.try_insert("nodeId".into(), UiValue::Text(UiText::try_from_str("root").unwrap())).expect("node id");
        arguments.try_insert("revision".into(), UiValue::Text(UiText::try_from_str("abc").unwrap())).expect("revision");
        let mut args = Some(UiValue::Map(arguments.finish()));
        let tree = TreeWindowKit::render_editable_nodes_windowed(&view, &TreeWindows::unhosted(), "tree-editor", |path, _| {
            assert_eq!(path, "root");
            Some(EditableTreeNode::new("42", "set-node", args.take().expect("one materialized row")))
        })
        .expect("editable tree");
        let input = &tree.children[0].children[0].children[0];
        let Component::Input(props) = &input.component else { panic!("tree row carries input") };
        assert_eq!(props.value.as_str(), "42");
        let binding = input.bindings.iter().find(|binding| binding.trigger == Trigger::Commit).expect("commit binding");
        assert_eq!(binding.action.name.as_str(), "set-node");
        let UiValue::Map(arguments) = binding.args.as_ref().expect("static args") else { panic!("map args") };
        assert!(arguments.iter().any(|(key, value)| key.as_str() == "nodeId" && matches!(value, UiValue::Text(value) if value.as_str() == "root")));
        assert!(arguments.iter().any(|(key, value)| key.as_str() == "revision" && matches!(value, UiValue::Text(value) if value.as_str() == "abc")));
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
        assert!(matches!(stack.component, Component::TreeSection(_)));
        assert_eq!(stack.children.len(), 2);
    }

    fn first_surface(node: &BuiltNode) -> Option<&BuiltNode> {
        matches!(node.component, Component::Surface(_)).then_some(node).or_else(|| node.children.iter().find_map(first_surface))
    }

    #[semio_framework_async_macros::async_test]
    async fn document_kit_declares_required_address_revision_and_text_arguments() {
        let definition = DocumentWindowKit::editable_window_kind();
        let action = definition.actions.first().expect("set-page action");
        assert!(!action.in_palette, "revision-addressed document edits are dispatched only by prefilled page controls");
        assert_eq!(action.args.iter().map(|argument| argument.id.as_str()).collect::<Vec<_>>(), ["page", "item", "revision", "text"]);
        assert!(action.args.iter().all(|argument| argument.required));
        assert!(matches!(&action.args[0].schema, semio_framework::ArgSchema::Number { integer: true, min: Some(0.0), .. }));
        assert!(matches!(&action.args[1].schema, semio_framework::ArgSchema::Number { integer: true, min: Some(0.0), .. }));
    }

    #[semio_framework_async_macros::async_test]
    async fn document_kit_editable_drafts_match_the_language_neutral_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🪟️window-kits/📃️document/🧫️fixtures/✏️editable/🔣️.json")).expect("fixture json");
        for case in fixture["cases"].as_array().expect("cases") {
            let locale = match case["locale"].as_str() {
                Some("de") => Locale::De,
                Some("en") => Locale::En,
                other => panic!("unsupported fixture locale {other:?}"),
            };
            let page_index = case["page"].as_u64().expect("page") as u32;
            let item_index = case["item"].as_u64().expect("item") as u32;
            let text = case["text"].as_str().expect("text");
            assert_eq!(DocumentWindowKit::text_revision(text), case["revision"].as_str().expect("revision"));
            let page = match case.get("staticArguments") {
                Some(arguments) => EditableDocumentPage::with_arguments(page_index, item_index, text, serde_json::from_value(arguments.clone()).expect("typed static arguments")),
                None => EditableDocumentPage::new(page_index, item_index, text),
            };
            let node = DocumentWindowKit::render_editable_windowed(&EditableDocumentView { pages: vec![page] }, &TreeWindows::unhosted(), locale).expect("editable document");
            let surface = first_surface(&node).expect("prefilled draft surface");
            let Component::Surface(props) = &surface.component else { unreachable!() };
            let mut scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(props).expect("text scene");
            for carrier in &surface.children {
                assert!(semio_framework_ui_scene::SceneDoc::merge_lane(&mut scene, carrier.key.as_str(), artifact_app_laws::built_carrier_text(carrier)));
            }
            assert_eq!(scene.buffer, text);
            let settings: serde_json::Value = serde_json::from_str(scene.settings_json.as_deref().expect("draft settings")).expect("settings json");
            assert_eq!(settings["readOnly"], false);
            assert_eq!(settings["editAction"], "set-page");
            assert_eq!(settings["editArgument"], "text");
            assert_eq!(settings["commit"], "explicit");
            match case.get("staticArguments") {
                Some(arguments) => assert_eq!(&settings["editArguments"], arguments),
                None => {
                    assert_eq!(settings["editArguments"]["page"], page_index);
                    assert_eq!(settings["editArguments"]["item"], item_index);
                    assert_eq!(settings["editArguments"]["revision"], case["revision"]);
                }
            }
            assert_eq!(settings["applyLabel"], case["labels"]["apply"]);
            assert_eq!(settings["discardLabel"], case["labels"]["discard"]);
            assert_eq!(settings["cancelLabel"], case["labels"]["cancel"]);
            assert!(settings["conflictLabel"].as_str().is_some_and(|label| !label.is_empty()));
            assert!(settings["failedLabel"].as_str().is_some_and(|label| !label.is_empty()));
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn document_kit_pages_long_unicode_text_without_losing_later_pages() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️text-editor-lanes/🔣️.json")).unwrap();
        let case = &fixture["cases"][2];
        let source = case["text"].as_str().unwrap().repeat(case["repeat"].as_u64().unwrap() as usize);
        let view = DocumentView { pages: (0..500).map(|index| DocumentPage { text: index.to_string() }).collect() };
        let state = ViewModel { tree_windows: vec![TreeWindowRequest { body_key: "body".into(), node_key: DocumentWindowKit::KIND_ID.into(), open: Some(true), offset: 400, rows: 10 }], ..Default::default() };
        let node = DocumentWindowKit::render_windowed(&view, &TreeWindows::for_body(&state, "body")).expect("large document page range");
        let Component::TreeSection(props) = &node.component else { panic!("windowed pages") };
        assert_eq!(props.window.map(|window| (window.total, window.offset)), Some((500, 400)));
        assert_eq!(node.children.len(), 10);
        assert_eq!(node.children.get(0).unwrap().key.as_str(), "page-400");
        let first = DocumentWindowKit::render(&DocumentView { pages: vec![DocumentPage { text: source }] }).expect("long first page");
        assert_eq!(first.children.len(), 1);
        assert!(!first.children.get(0).unwrap().children.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn media_kit_renders_localized_revision_bound_transport_props() {
        let view = MediaView {
            duration_ms: Some(60_000),
            position_ms: 61_000,
            selection_start_ms: Some(1_000),
            selection_end_ms: Some(70_000),
            kind: MediaKind::Video,
            media_type: "video/mp4".into(),
            revision: "9007199254740993".into(),
            locale: Locale::De,
            resource: Some(MediaResource {
                controller_id: "s.stdio.mp4@isobmff/*#editor".into(),
                app_instance_id: 23,
                parent_document_id: "document-mp4-1".into(),
                output_port: MEDIA_PLAYBACK_OUTPUT_PORT.into(),
                revision: "9007199254740993".into(),
                generation: "9007199254740995".into(),
            }),
            capability: MediaCapabilityStatus::Loading,
            capability_reason: None,
            host_content_height: 360.0,
        };
        let node = MediaWindowKit::render(&view).expect("bounded fixture");
        let Component::Extension(extension) = node.component else { panic!("expected Extension") };
        assert_eq!(extension.extension.as_str(), MEDIA_TRANSPORT_EXTENSION_ID);
        let props = serde_json::to_value(extension.props).expect("media props serialize");
        assert_eq!(props["positionMs"], 60_000);
        assert_eq!(props["selectionEndMs"], 60_000);
        assert_eq!(props["labels"]["play"], "Wiedergabe");
        assert_eq!(props["resource"]["revision"], "9007199254740993");
        assert_eq!(props["resource"]["parentDocumentId"], "document-mp4-1");
        assert_eq!(props["resource"]["generation"], "9007199254740995");
    }
}
