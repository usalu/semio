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
        assert_eq!(document_en, "Document");
        assert_eq!(document_de, "Dokument");
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
        assert_eq!(scene.columns_json, "[\"a\",\"b\"]");
        assert_eq!(scene.rows_json, "[[\"1\",\"2\"]]");
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_render_rows_stamps_a_stable_row_id_and_omits_the_actions_column_when_no_row_has_one() {
        let mut view = TableRowsView::new(UiText::try_from_str("Actions").expect("bounded text"));
        view.try_push_column(UiText::try_from_str("Name").expect("bounded text")).expect("fixed column");
        let mut table_row = TableRow::new(UiText::try_from_str("space:abc").expect("bounded text"));
        table_row.try_push_cell(UiText::try_from_str("Atelier").expect("bounded text")).expect("fixed cell");
        view.try_push_row(table_row).expect("fixed row");
        let node = TableWindowKit::render_rows(view).expect("bounded fixture");
        assert!(matches!(node.component, Component::Container(_)));
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children.get(1).expect("row").key.as_str(), "space:abc");
        assert_eq!(node.children.get(0).expect("header").children.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn table_kit_render_rows_renders_row_action_buttons_carrying_their_dispatchable_descriptor() {
        let action = ActionId::try_v1("s.space.home", "delete-space").expect("bounded action");
        let mut view = TableRowsView::new(UiText::try_from_str("Actions").expect("bounded text"));
        view.try_push_column(UiText::try_from_str("Name").expect("bounded text")).expect("fixed column");
        let mut table_row = TableRow::new(UiText::try_from_str("space:abc").expect("bounded text"));
        table_row.try_push_cell(UiText::try_from_str("Atelier").expect("bounded text")).expect("fixed cell");
        table_row.try_push_action(TableRowAction::new(UiText::try_from_str(IconName::Trash2.as_str()).expect("bounded icon"), Label(UiText::try_from_str("Delete").expect("bounded label")), (action.clone(), None))).expect("fixed action");
        view.try_push_row(table_row).expect("fixed row");
        let node = TableWindowKit::render_rows(view).expect("bounded fixture");
        assert_eq!(node.children.get(0).expect("header").children.len(), 2);
        let button = node.children.get(1).expect("row").children.get(1).expect("button");
        assert!(matches!(button.component, Component::Button(_)));
        assert_eq!(button.bindings.get(0).expect("binding").action, action);
    }

    #[test]
    fn table_rows_max_plus_one_returns_the_exact_row_owner() {
        let mut view = TableRowsView::new(UiText::default());
        for index in 0..TABLE_WINDOW_ROWS {
            let id = UiText::try_format(format_args!("row-{index}")).expect("bounded id");
            assert!(view.try_push_row(TableRow::new(id)).is_ok());
        }
        let rejected_id = UiText::try_from_str("row-max-plus-one").expect("bounded id");
        let rejected = view.try_push_row(TableRow::new(rejected_id.clone())).expect_err("fixed row cap");
        assert_eq!(rejected.id, rejected_id);
    }

    #[test]
    fn abandoned_table_rows_retire_one_row_action_or_cell_per_opportunity() {
        while close_table_rows_view_one() {}
        let mut view = TableRowsView::new(UiText::default());
        let mut row = TableRow::new(UiText::try_from_str("row").expect("bounded id"));
        row.try_push_cell(UiText::try_from_str("cell").expect("bounded cell")).expect("fixed cell");
        row.try_push_action(TableRowAction::new(UiText::try_from_str("trash-2").expect("bounded icon"), Label(UiText::try_from_str("Delete").expect("bounded label")), (ActionId::try_v1("fixture", "delete").expect("bounded action"), None)))
            .expect("fixed action");
        view.try_push_row(row).expect("fixed row");
        drop(view);
        let mut opportunities = 0;
        while close_table_rows_view_one() {
            opportunities += 1;
        }
        assert!(opportunities >= 4);
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
