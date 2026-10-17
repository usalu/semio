/// 🌳️ Collection rows, table first paint, paging, derived schema controls and revisioned inputs.

#[test]
fn collection_rows_are_open_windowed_tree_item_descendants_with_edit_controls() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️collection-tree-shape/🔣️.json")).expect("collection tree fixture");
    let source = serde_json::to_string(&fixture["snapshot"]).expect("third-party snapshot oracle");
    let schema = serde_json::to_string(&fixture["input"]["schema"]).expect("third-party schema oracle");
    let collection_key = fixture["collectionKey"].as_str().expect("collection key");
    let expected_values = fixture["expectedValues"].as_array().expect("expected values");
    let snapshot: DslValue = semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("native fixture parse");
    let native_source = snapshot_edit_source(&snapshot);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&native_source).expect("third-party native-source oracle"), fixture["snapshot"]);
    let provider = DslSnapshotDetailsProvider::<NativeLazySnapshot>::from_value_and_schema(snapshot, &schema);
    let records_path = [SnapshotDetailPathSegment::Key(collection_key.into())];
    let records_id = path_id(&records_path);
    let children_id = format!("{records_id}{}", fixture["windowSuffix"].as_str().expect("window suffix"));
    let node_key = [ROOT_SECTION_ID, children_id.as_str()].join(TREE_WINDOW_PATH_SEPARATOR);
    let closed_view = ViewModel {
        tree_windows: vec![TreeWindowRequest { body_key: SNAPSHOT_DETAILS_BODY_KEY.into(), node_key: node_key.clone(), open: Some(false), offset: 0, rows: expected_values.len() as u32 }],
        tree_viewport_rows: Some(16),
        ..ViewModel::new(Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let closed = render_snapshot_details_provider_revisioned(&provider, semio_framework_ui_contract::UiPublicationRevision(1), Locale::En, "test.collection-tree-shape#editor", &TreeWindows::for_body(&closed_view, SNAPSHOT_DETAILS_BODY_KEY)).expect("closed collection render");
    let closed_records = find_key(&closed, &records_id).expect("closed records item");
    let semio_framework_ui_contract::Component::TreeItem(closed_props) = &closed_records.component else { panic!("collection must render as a tree item") };
    assert_eq!(closed_props.default_open, Some(true));
    assert_eq!(closed_props.window.as_ref().map(|window| window.total), Some(expected_values.len() as u32));
    assert!(!closed_records.children.iter().any(|child| matches!(child.component, semio_framework_ui_contract::Component::TreeItem(_))), "closed host state must not materialize entry rows");

    let open_view = ViewModel {
        tree_windows: vec![TreeWindowRequest { body_key: SNAPSHOT_DETAILS_BODY_KEY.into(), node_key, open: Some(true), offset: 0, rows: expected_values.len() as u32 }],
        tree_viewport_rows: Some(16),
        ..ViewModel::new(Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let open = render_snapshot_details_provider_revisioned(&provider, semio_framework_ui_contract::UiPublicationRevision(1), Locale::En, "test.collection-tree-shape#editor", &TreeWindows::for_body(&open_view, SNAPSHOT_DETAILS_BODY_KEY)).expect("open collection render");
    let records = find_key(&open, &records_id).expect("open records item");
    let semio_framework_ui_contract::Component::TreeItem(props) = &records.component else { panic!("collection must remain a tree item") };
    let window = props.window.as_ref().expect("collection tree item window");
    assert_eq!((window.total, window.offset), (expected_values.len() as u32, 0));
    assert_eq!(props.default_open, Some(true));
    assert!(contains_key(records, &format!("{records_id}-add-template")), "typed creation remains independently actionable");
    assert!(!records.children.iter().any(|child| matches!(child.component, semio_framework_ui_contract::Component::TreeSection(_))), "tree sections may only be direct tree-root children");
    for (index, expected) in expected_values.iter().enumerate() {
        let child_path = [SnapshotDetailPathSegment::Key(collection_key.into()), SnapshotDetailPathSegment::Index(index)];
        let child_id = path_id(&child_path);
        let child = records.children.iter().find(|child| child.key.as_str() == child_id).expect("materialized entry must be a direct descendant");
        assert!(matches!(child.component, semio_framework_ui_contract::Component::TreeItem(_)));
        let value_id = path_id(&[SnapshotDetailPathSegment::Key(collection_key.into()), SnapshotDetailPathSegment::Index(index), SnapshotDetailPathSegment::Key("value".into())]);
        let value = find_key(child, &value_id).expect("entry value remains reachable");
        assert!(matches!(value.component, semio_framework_ui_contract::Component::TreeItem(_)));
        let editor = find_key(value, &format!("{value_id}-text")).expect("entry value remains editable");
        let semio_framework_ui_contract::Component::Input(editor_props) = &editor.component else { panic!("entry value text input") };
        assert_eq!(editor_props.value.as_str(), expected.as_str().expect("expected value"));
        assert!(editor_props.draft_target.as_ref().is_some_and(|target| target.as_str().contains(SET_SNAPSHOT_VALUE_ACTION_ID)));
    }
    let rendered_entries = expected_values
        .iter()
        .enumerate()
        .map(|(index, _)| {
            let entry_id = path_id(&[SnapshotDetailPathSegment::Key(collection_key.into()), SnapshotDetailPathSegment::Index(index)]);
            let entry = find_key(records, &entry_id).expect("projected entry");
            let semio_framework_ui_contract::Component::TreeItem(entry_props) = &entry.component else { panic!("projected entry tree item") };
            let value_id = path_id(&[SnapshotDetailPathSegment::Key(collection_key.into()), SnapshotDetailPathSegment::Index(index), SnapshotDetailPathSegment::Key("value".into())]);
            let value = find_key(entry, &value_id).expect("projected value");
            let semio_framework_ui_contract::Component::TreeItem(value_props) = &value.component else { panic!("projected value tree item") };
            let input = find_key(value, &format!("{value_id}-text")).expect("projected value input");
            let semio_framework_ui_contract::Component::Input(input_props) = &input.component else { panic!("projected value input component") };
            let action = input.bindings.iter().find(|binding| binding.trigger == semio_framework_ui_contract::Trigger::Commit).map(|binding| binding.action.name.as_str());
            serde_json::json!({
                "key": entry.key.as_str(),
                "label": entry_props.label.0.as_str(),
                "defaultOpen": entry_props.default_open,
                "value": {
                    "key": value.key.as_str(),
                    "label": value_props.label.0.as_str(),
                    "input": {
                        "key": input.key.as_str(),
                        "label": input.accessibility.label.as_ref().map(|label| label.0.as_str()),
                        "value": input_props.value.as_str(),
                        "action": action,
                        "draftTargetContains": SET_SNAPSHOT_VALUE_ACTION_ID,
                    },
                },
            })
        })
        .collect::<Vec<_>>();
    let rendered_hierarchy = serde_json::json!({
        "key": records.key.as_str(),
        "label": props.label.0.as_str(),
        "defaultOpen": props.default_open,
        "window": { "total": window.total, "offset": window.offset },
        "entries": rendered_entries,
    });
    assert_eq!(rendered_hierarchy, fixture["rendererHierarchy"], "the renderer fixture must remain an exact semantic projection of the native hierarchy");
}

#[test]
fn table_details_first_paint_keeps_fields_reachable_across_repeated_projection() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️first-paint/🔣️.json")).unwrap();
    let source = serde_json::to_string(&fixture["snapshot"]).unwrap();
    let schema = serde_json::to_string(&fixture["input"]["schema"]).unwrap();
    let snapshot: DslValue = semio_framework_pack_json::from_json_str(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&snapshot_edit_source(&snapshot)).unwrap(), fixture["snapshot"]);
    let provider = DslSnapshotDetailsProvider::<NativeLazySnapshot>::from_value_and_schema(snapshot, &schema);
    let folded_collection = fixture["foldedPath"][0].as_str().expect("folded collection key");
    let folded_collection_id = path_id(&[SnapshotDetailPathSegment::Key(folded_collection.into())]);
    let closed_view = ViewModel {
        tree_windows: vec![TreeWindowRequest { body_key: SNAPSHOT_DETAILS_BODY_KEY.into(), node_key: [ROOT_SECTION_ID, &format!("{folded_collection_id}-children")].join(TREE_WINDOW_PATH_SEPARATOR), open: Some(false), offset: 0, rows: 16 }],
        tree_viewport_rows: Some(16),
        ..ViewModel::new(Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    for _ in 0..fixture["repeatCount"].as_u64().unwrap() {
        let tree = render_snapshot_details_provider_revisioned(&provider, semio_framework_ui_contract::UiPublicationRevision(1), Locale::En, "test.table-details#editor", &TreeWindows::for_body(&closed_view, SNAPSHOT_DETAILS_BODY_KEY)).expect("details first paint");
        for key in fixture["firstPaintFields"].as_array().unwrap() {
            assert!(contains_key(&tree, &path_id(&[SnapshotDetailPathSegment::Key(key.as_str().unwrap().into())])), "field {key} must remain reachable");
        }
        let path = fixture["foldedPath"]
            .as_array()
            .unwrap()
            .iter()
            .map(|segment| match segment {
                serde_json::Value::String(key) => SnapshotDetailPathSegment::Key(key.clone()),
                serde_json::Value::Number(index) => SnapshotDetailPathSegment::Index(index.as_u64().unwrap() as usize),
                _ => panic!("fixture path segment"),
            })
            .collect::<Vec<_>>();
        assert!(!contains_key(&tree, &path_id(&path)), "folded collection payload stays unmaterialized");
        let mut retirement = ui_contract::BuiltTreeRetirement::new(tree);
        while !retirement.close_step(1, 4096).unwrap().complete {}
        while !ui_contract::close_ui_value_page_with_grant(1, 4096).unwrap().complete {}
    }
}

#[test]
fn nested_collection_honours_an_explicit_second_page_without_counting_controls_as_rows() {
    let provider = LazyPixels { count: 1_000 };
    let pixels_path = [SnapshotDetailPathSegment::Key("pixels".into())];
    let pixels_id = path_id(&pixels_path);
    let node_key = [ROOT_SECTION_ID, &format!("{pixels_id}-children")].join(TREE_WINDOW_PATH_SEPARATOR);
    let view = ViewModel {
        tree_windows: vec![TreeWindowRequest { body_key: SNAPSHOT_DETAILS_BODY_KEY.into(), node_key, open: Some(true), offset: 100, rows: 3 }],
        tree_viewport_rows: Some(4),
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let windows = TreeWindows::for_body(&view, SNAPSHOT_DETAILS_BODY_KEY);
    let node = render_snapshot_details_provider_revisioned(&provider, semio_framework_ui_contract::UiPublicationRevision(1), Locale::En, "s.stdio.png@test/*#editor", &windows).expect("nested details page");
    for index in 100..103 {
        let path = [SnapshotDetailPathSegment::Key("pixels".into()), SnapshotDetailPathSegment::Index(index)];
        assert!(contains_key(&node, &path_id(&path)), "requested logical row {index} is reachable");
    }
    for index in [0, 99, 103] {
        let path = [SnapshotDetailPathSegment::Key("pixels".into()), SnapshotDetailPathSegment::Index(index)];
        assert!(!contains_key(&node, &path_id(&path)), "row {index} is outside the requested page");
    }
}

#[test]
fn derived_schema_controls_render_only_the_requested_pages() {
    use std::cell::Cell;
    struct DerivedControls {
        variant_reads: Cell<usize>,
        property_reads: Cell<usize>,
        variants: usize,
        properties: usize,
    }
    impl SnapshotDetailsProvider for DerivedControls {
        fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue> {
            path.is_empty().then_some(SnapshotDetailValue::Object)
        }
        fn child_count(&self, _path: &[SnapshotDetailPathSegment]) -> usize {
            0
        }
        fn object_key(&self, _path: &[SnapshotDetailPathSegment], _index: usize) -> Option<String> {
            None
        }
        fn variant_count(&self, path: &[SnapshotDetailPathSegment], _locale: Locale) -> usize {
            if path.is_empty() {
                self.variants
            } else {
                0
            }
        }
        fn variant(&self, path: &[SnapshotDetailPathSegment], _locale: Locale, index: usize) -> Option<SnapshotDetailVariant> {
            if !path.is_empty() || index >= self.variants {
                return None;
            }
            self.variant_reads.set(self.variant_reads.get() + 1);
            Some(SnapshotDetailVariant { label: format!("Variant {index}"), value: DslValue::Object(vec![("kind".into(), DslValue::String(format!("variant-{index}")))]) })
        }
        fn missing_property_count(&self, path: &[SnapshotDetailPathSegment]) -> usize {
            if path.is_empty() {
                self.properties
            } else {
                0
            }
        }
        fn missing_property_template(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<(String, DslValue)> {
            if !path.is_empty() || index >= self.properties {
                return None;
            }
            self.property_reads.set(self.property_reads.get() + 1);
            Some((format!("optional{index}"), DslValue::String(String::new())))
        }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️derived-controls/🔣️.json")).expect("derived-control paging fixture");
    let number = |group: &str, field: &str| fixture[group][field].as_u64().expect("fixture number") as usize;
    let provider = DerivedControls { variant_reads: Cell::new(0), property_reads: Cell::new(0), variants: number("variants", "total"), properties: number("missingProperties", "total") };
    let variant_id = format!("{ROOT_PATH_ID}-variants");
    let property_id = format!("{ROOT_PATH_ID}-missing-properties");
    let view = ViewModel {
        tree_windows: vec![
            TreeWindowRequest {
                body_key: SNAPSHOT_DETAILS_BODY_KEY.into(),
                node_key: [ROOT_SECTION_ID, variant_id.as_str()].join(TREE_WINDOW_PATH_SEPARATOR),
                open: Some(true),
                offset: number("variants", "offset") as u32,
                rows: number("variants", "rows") as u32,
            },
            TreeWindowRequest {
                body_key: SNAPSHOT_DETAILS_BODY_KEY.into(),
                node_key: [ROOT_SECTION_ID, property_id.as_str()].join(TREE_WINDOW_PATH_SEPARATOR),
                open: Some(true),
                offset: number("missingProperties", "offset") as u32,
                rows: number("missingProperties", "rows") as u32,
            },
        ],
        tree_viewport_rows: Some(128),
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let windows = TreeWindows::for_body(&view, SNAPSHOT_DETAILS_BODY_KEY);
    let rendered = render_snapshot_details_provider_revisioned(&provider, semio_framework_ui_contract::UiPublicationRevision(1), Locale::En, "test.derived-controls#editor", &windows).expect("windowed derived controls");
    for index in 100..103 {
        assert!(contains_key(&rendered, &format!("{ROOT_PATH_ID}-variant-{index}")));
    }
    for index in 200..202 {
        assert!(contains_key(&rendered, &format!("{ROOT_PATH_ID}-add-property-{index}")));
    }
    assert_eq!(provider.variant_reads.get(), number("variants", "rows"));
    assert_eq!(provider.property_reads.get(), number("missingProperties", "rows"));
}

#[test]
fn split_layout_keeps_canvas_and_details_visible() {
    let layout = snapshot_details_split_layout("framework.window.tree", "Tree");
    let WindowLayoutRoot::Axis(axis) = layout.root else { panic!("details layout must be split") };
    assert_eq!(axis.children.len(), 2);
}

#[test]
fn numeric_controls_preserve_json_number_kinds_through_the_action_parser() {
    struct Numeric(Number);
    impl SnapshotDetailsProvider for Numeric {
        fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue> {
            path.is_empty().then(|| SnapshotDetailValue::Number(self.0))
        }
        fn child_count(&self, _path: &[SnapshotDetailPathSegment]) -> usize {
            0
        }
        fn object_key(&self, _path: &[SnapshotDetailPathSegment], _index: usize) -> Option<String> {
            None
        }
    }
    for (number, expected) in [(Number::Float(1.0), DslValue::Number(Number::Float(1.0))), (Number::UInt(u64::MAX), DslValue::Number(Number::UInt(u64::MAX))), (Number::Int(i64::MIN), DslValue::Number(Number::Int(i64::MIN)))] {
        let rendered = render_snapshot_details_provider_revisioned(&Numeric(number), semio_framework_ui_contract::UiPublicationRevision(1), Locale::En, "test.number#editor", &TreeWindows::unhosted()).expect("number details");
        let input = find_key(&rendered, &format!("{ROOT_PATH_ID}-number")).expect("number input");
        let semio_framework_ui_contract::Component::Input(props) = &input.component else { panic!("number editor must remain an exact JSON text input") };
        assert_eq!(props.kind, semio_framework_ui_contract::InputKind::Text);
        assert!(props.draft_target.as_ref().is_some_and(|target| target.as_str().contains(SET_SNAPSHOT_VALUE_ACTION_ID)));
        let binding = input.bindings.iter().find(|binding| binding.trigger == semio_framework_ui_contract::Trigger::Commit).expect("commit binding");
        let Some(UiValue::Map(arguments)) = &binding.args else { panic!("static arguments") };
        assert!(arguments.iter().any(|(key, value)| key.as_str() == "valueEncoding" && matches!(value, UiValue::Text(value) if value.as_str() == "json")));
        let args = DslValue::Object(vec![("path".into(), DslValue::String(String::new())), ("value".into(), DslValue::String(props.value.as_str().into())), ("valueEncoding".into(), DslValue::String("json".into()))]);
        let event = snapshot_edit_event_from_action(SET_SNAPSHOT_VALUE_ACTION_ID, Some(&args)).expect("parse").expect("known action");
        assert_eq!(event, SnapshotEditEvent::SetValue { path: String::new(), value: expected });
    }
}

#[test]
fn revisioned_details_inputs_publish_the_exact_committed_document_revision() {
    struct Text;
    impl SnapshotDetailsProvider for Text {
        fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue> {
            path.is_empty().then(|| SnapshotDetailValue::String("value".into()))
        }
        fn child_count(&self, _path: &[SnapshotDetailPathSegment]) -> usize {
            0
        }
        fn object_key(&self, _path: &[SnapshotDetailPathSegment], _index: usize) -> Option<String> {
            None
        }
    }
    let rendered = render_snapshot_details_provider_revisioned(&Text, ui_contract::UiPublicationRevision(u64::MAX), Locale::En, "test.publication#editor", &TreeWindows::unhosted()).expect("revisioned details");
    let input = find_key(&rendered, &format!("{ROOT_PATH_ID}-text")).expect("text input");
    let ui_contract::Component::Input(props) = &input.component else { panic!("text details remain an input") };
    assert_eq!(props.publication_revision, Some(ui_contract::UiPublicationRevision(u64::MAX)));
    assert_eq!(serde_json::to_value(props.publication_revision).expect("third-party serde oracle"), serde_json::Value::String(u64::MAX.to_string()));
}
