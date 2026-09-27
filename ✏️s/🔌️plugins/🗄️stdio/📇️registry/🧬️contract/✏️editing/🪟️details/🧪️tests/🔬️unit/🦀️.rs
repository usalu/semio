use super::*;
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_PATH_SEPARATOR};

struct LazyPixels {
    count: usize,
}

impl SnapshotDetailsProvider for LazyPixels {
    fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue> {
        match path {
            [] => Some(SnapshotDetailValue::Object),
            [SnapshotDetailPathSegment::Key(key)] if key == "pixels" => Some(SnapshotDetailValue::Array),
            [SnapshotDetailPathSegment::Key(key), SnapshotDetailPathSegment::Index(index)] if key == "pixels" && *index < self.count => Some(SnapshotDetailValue::Number(Number::UInt((*index % 256) as u64))),
            _ => None,
        }
    }

    fn child_count(&self, path: &[SnapshotDetailPathSegment]) -> usize {
        match path {
            [] => 1,
            [SnapshotDetailPathSegment::Key(key)] if key == "pixels" => self.count,
            _ => 0,
        }
    }

    fn object_key(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<String> {
        (path.is_empty() && index == 0).then(|| "pixels".into())
    }
}

#[test]
fn details_definition_owns_all_snapshot_actions_in_both_languages() {
    let definition = snapshot_details_window_definition();
    assert_eq!(definition.id, SNAPSHOT_DETAILS_WINDOW_KIND_ID);
    assert_eq!(definition.body_key, SNAPSHOT_DETAILS_BODY_KEY);
    assert_eq!(definition.actions.len(), super::super::SNAPSHOT_EDIT_ACTION_IDS.len());
    assert_ne!(definition.label.resolve(semio_framework_plugin::Terminology::Native, Locale::En), "");
    assert_ne!(definition.label.resolve(semio_framework_plugin::Terminology::Native, Locale::De), "");
}

#[test]
fn schema_resolution_uses_the_exact_editor_dialect_without_prefix_fallbacks() {
    assert_eq!(
        controller_schema_descriptor_ids("s.stdio.json@rfc8259/i-json#editor"),
        Some(("s.stdio.json.rfc8259.i-json".into(), None))
    );
    assert_eq!(
        controller_schema_descriptor_ids("s.stdio.json@rfc8259/*#editor"),
        Some(("s.stdio.json.rfc8259.base".into(), Some("s.stdio.json.rfc8259.any".into())))
    );
    assert_eq!(
        controller_schema_descriptor_ids("s.stdio.deflate@rfc1950/*#editor"),
        Some(("s.stdio.deflate.rfc1950.base".into(), Some("s.stdio.deflate.rfc1950.any".into())))
    );
}

#[test]
fn lazy_collection_renders_a_bounded_page_without_materializing_every_pixel() {
    let provider = LazyPixels { count: 16_000_000 };
    let result = render_snapshot_details_provider(&provider, Locale::En, "s.stdio.png@test/*#editor", &TreeWindows::unhosted());
    assert!(result.is_ok());
}

fn contains_key(node: &BuiltNode, key: &str) -> bool {
    node.key.as_str() == key || node.children.iter().any(|child| contains_key(child, key))
}

fn find_key<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
    (node.key.as_str() == key).then_some(node).or_else(|| node.children.iter().find_map(|child| find_key(child, key)))
}

#[test]
fn nested_collection_honours_an_explicit_second_page_without_counting_controls_as_rows() {
    let provider = LazyPixels { count: 1_000 };
    let pixels_path = [SnapshotDetailPathSegment::Key("pixels".into())];
    let pixels_id = path_id(&pixels_path);
    let node_key = [ROOT_SECTION_ID, &format!("{ROOT_PATH_ID}-children"), &format!("{pixels_id}-children")].join(TREE_WINDOW_PATH_SEPARATOR);
    let view = ViewModel {
        tree_windows: vec![TreeWindowRequest { body_key: SNAPSHOT_DETAILS_BODY_KEY.into(), node_key, open: Some(true), offset: 100, rows: 3 }],
        tree_viewport_rows: Some(4),
        ..Default::default()
    };
    let windows = TreeWindows::for_body(&view, SNAPSHOT_DETAILS_BODY_KEY);
    let node = render_snapshot_details_provider(&provider, Locale::En, "s.stdio.png@test/*#editor", &windows).expect("nested details page");
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
        fn child_count(&self, _path: &[SnapshotDetailPathSegment]) -> usize { 0 }
        fn object_key(&self, _path: &[SnapshotDetailPathSegment], _index: usize) -> Option<String> { None }
    }
    for (number, expected) in [
        (Number::Float(1.0), DslValue::Number(Number::Float(1.0))),
        (Number::UInt(u64::MAX), DslValue::Number(Number::UInt(u64::MAX))),
        (Number::Int(i64::MIN), DslValue::Number(Number::Int(i64::MIN))),
    ] {
        let rendered = render_snapshot_details_provider(&Numeric(number), Locale::En, "test.number#editor", &TreeWindows::unhosted()).expect("number details");
        let input = find_key(&rendered, &format!("{ROOT_PATH_ID}-number")).expect("number input");
        let semio_framework_ui_contract::Component::Input(props) = &input.component else { panic!("number editor must remain an exact JSON text input") };
        assert_eq!(props.kind, semio_framework_ui_contract::InputKind::Text);
        let binding = input.bindings.iter().find(|binding| binding.trigger == semio_framework_ui_contract::Trigger::Commit).expect("commit binding");
        let Some(UiValue::Map(arguments)) = &binding.args else { panic!("static arguments") };
        assert!(arguments.iter().any(|(key, value)| key.as_str() == "valueEncoding" && matches!(value, UiValue::Text(value) if value.as_str() == "json")));
        let args = DslValue::Object(vec![
            ("path".into(), DslValue::String(String::new())),
            ("value".into(), DslValue::String(props.value.as_str().into())),
            ("valueEncoding".into(), DslValue::String("json".into())),
        ]);
        let event = super::super::snapshot_edit_event_from_action(super::super::SET_SNAPSHOT_VALUE_ACTION_ID, Some(&args)).expect("parse").expect("known action");
        assert_eq!(event, super::super::SnapshotEditEvent::SetValue { path: String::new(), value: expected });
    }
}

#[test]
fn schema_templates_create_required_records_in_empty_lists_and_nullable_fields() {
    let schema = r##"{
        "type":"object",
        "required":["schema","records","selection"],
        "properties":{
            "schema":{"type":"string"},
            "records":{"type":"array","items":{"$ref":"#/$defs/Record"}},
            "selection":{"anyOf":[{"type":"null"},{"$ref":"#/$defs/Selection"}]}
        },
        "$defs":{
            "Record":{"type":"object","required":["fields"],"properties":{"fields":{"type":"array","items":{"$ref":"#/$defs/Field"}}}},
            "Field":{"type":"object","required":["value","quoted"],"properties":{"value":{"type":"string"},"quoted":{"type":"boolean"}}},
            "Selection":{"type":"object","required":["name","enabled"],"properties":{"name":{"type":"string","minLength":1},"enabled":{"type":"boolean"}}}
        }
    }"##;
    struct TypedRecords {
        schema: DslValue,
    }
    impl SnapshotDetailsProvider for TypedRecords {
        fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue> {
            match path {
                [] => Some(SnapshotDetailValue::Object),
                [SnapshotDetailPathSegment::Key(key)] if key == "schema" => Some(SnapshotDetailValue::String("test.records".into())),
                [SnapshotDetailPathSegment::Key(key)] if key == "records" => Some(SnapshotDetailValue::Array),
                [SnapshotDetailPathSegment::Key(key)] if key == "selection" => Some(SnapshotDetailValue::Null),
                _ => None,
            }
        }
        fn child_count(&self, path: &[SnapshotDetailPathSegment]) -> usize {
            if path.is_empty() { 3 } else { 0 }
        }
        fn object_key(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<String> {
            (path.is_empty()).then(|| ["schema", "records", "selection"].get(index).map(|key| (*key).to_string())).flatten()
        }
        fn creation_template(&self, path: &[SnapshotDetailPathSegment], collection_item: bool) -> Option<DslValue> {
            let root = &self.schema;
            let mut schema = schema_at_path(root, path)?;
            if collection_item {
                schema = object_field(schema, "items")?;
            }
            template_from_schema(root, schema, 0)
        }
        fn allows_untyped_creation(&self, _path: &[SnapshotDetailPathSegment]) -> bool { false }
    }
    let provider = TypedRecords { schema: crate::pack::json::from_json_str(schema).expect("schema") };
    assert_eq!(
        provider.creation_template(&[SnapshotDetailPathSegment::Key("records".into())], true),
        Some(DslValue::Object(vec![("fields".into(), DslValue::Array(Vec::new()))]))
    );
    assert_eq!(
        provider.creation_template(&[SnapshotDetailPathSegment::Key("selection".into())], false),
        Some(DslValue::Object(vec![
            ("name".into(), DslValue::String("x".into())),
            ("enabled".into(), DslValue::Bool(false)),
        ]))
    );
    let rendered = render_snapshot_details_provider(&provider, Locale::En, "test.records#editor", &TreeWindows::unhosted()).expect("schema-aware details");
    let records_id = path_id(&[SnapshotDetailPathSegment::Key("records".into())]);
    let selection_id = path_id(&[SnapshotDetailPathSegment::Key("selection".into())]);
    assert!(contains_key(&rendered, &format!("{records_id}-add-template")));
    assert!(contains_key(&rendered, &format!("{selection_id}-null-template")));
    assert!(!contains_key(&rendered, &format!("{records_id}-add-0")));
    assert!(!contains_key(&rendered, &format!("{selection_id}-null-0")));
    let oracle: serde_json::Value = serde_json::from_str(&super::super::snapshot_edit_source(
        &provider.creation_template(&[SnapshotDetailPathSegment::Key("selection".into())], false).unwrap(),
    ))
    .expect("third-party JSON oracle");
    assert_eq!(oracle, serde_json::json!({"name":"x","enabled":false}));
}

#[test]
fn schema_annotations_localize_fixed_fields_without_translating_user_keys() {
    struct PresentationFixture;
    impl ToValue for PresentationFixture {
        fn to_value(&self) -> DslValue { DslValue::Null }
    }
    impl ArtifactDsl for PresentationFixture {
        const EXTENSION: &'static str = "json";
        fn parse_dsl(_text: &str) -> Result<Self, crate::kernel::TextError> {
            Err(crate::kernel::TextError::new("unused presentation fixture", crate::kernel::TextSpan::at(1, 1)))
        }
        fn print_dsl(&self) -> String { "null".into() }
    }
    let schema = r#"{
        "type":"object",
        "properties":{
            "windowBits":{
                "type":"integer",
                "x-semio-title":{"en":"Window size","de":"Fenstergröße"},
                "x-semio-description":{"en":"Compression window in bits.","de":"Kompressionsfenster in Bit."}
            }
        },
        "additionalProperties":true
    }"#;
    let value = DslValue::Object(vec![
        ("windowBits".into(), DslValue::Number(Number::UInt(15))),
        ("Projektname".into(), DslValue::String("Semio".into())),
    ]);
    let provider = DslSnapshotDetailsProvider::<PresentationFixture>::from_value_and_schema(value, schema);
    assert_eq!(
        provider.presentation(&[SnapshotDetailPathSegment::Key("windowBits".into())], Locale::En),
        Some(SnapshotDetailPresentation { label: "Window size [windowBits]".into(), description: Some("Compression window in bits.".into()) })
    );
    assert_eq!(
        provider.presentation(&[SnapshotDetailPathSegment::Key("windowBits".into())], Locale::De),
        Some(SnapshotDetailPresentation { label: "Fenstergröße [windowBits]".into(), description: Some("Kompressionsfenster in Bit.".into()) })
    );
    assert_eq!(provider.presentation(&[SnapshotDetailPathSegment::Key("Projektname".into())], Locale::De), None);
    for (locale, expected_label, expected_description) in [
        (Locale::En, "Window size [windowBits]", "Compression window in bits."),
        (Locale::De, "Fenstergröße [windowBits]", "Kompressionsfenster in Bit."),
    ] {
        let rendered = render_snapshot_details_provider(&provider, locale, "test.presentation#editor", &TreeWindows::unhosted()).expect("localized details");
        let fixed = find_key(&rendered, &path_id(&[SnapshotDetailPathSegment::Key("windowBits".into())])).expect("fixed schema field");
        let semio_framework_ui_contract::Component::TreeItem(props) = &fixed.component else { panic!("schema field tree item") };
        assert_eq!(props.label.0.as_str(), expected_label);
        assert_eq!(props.description.as_ref().map(|value| value.as_str()), Some(expected_description));
        let dynamic = find_key(&rendered, &path_id(&[SnapshotDetailPathSegment::Key("Projektname".into())])).expect("dynamic map key");
        let semio_framework_ui_contract::Component::TreeItem(props) = &dynamic.component else { panic!("dynamic field tree item") };
        assert_eq!(props.label.0.as_str(), "Projektname");
        assert!(props.description.is_none());
    }
}

#[test]
fn enum_values_render_as_an_accessible_typed_select() {
    struct EnumValue;
    impl SnapshotDetailsProvider for EnumValue {
        fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue> {
            path.is_empty().then(|| SnapshotDetailValue::String("review".into()))
        }
        fn child_count(&self, _path: &[SnapshotDetailPathSegment]) -> usize { 0 }
        fn object_key(&self, _path: &[SnapshotDetailPathSegment], _index: usize) -> Option<String> { None }
        fn enum_values(&self, path: &[SnapshotDetailPathSegment]) -> Vec<DslValue> {
            path.is_empty()
                .then(|| vec![DslValue::String("draft".into()), DslValue::String("review".into()), DslValue::String("published".into())])
                .unwrap_or_default()
        }
    }
    let rendered = render_snapshot_details_provider(&EnumValue, Locale::De, "test.enum#editor", &TreeWindows::unhosted()).expect("enum details");
    let select = find_key(&rendered, &format!("{ROOT_PATH_ID}-enum")).expect("enum select");
    let semio_framework_ui_contract::Component::Select(props) = &select.component else { panic!("string enum must use a select") };
    assert_eq!(props.value.as_str(), "review");
    assert_eq!(props.items.len(), 3);
    let binding = select.bindings.iter().find(|binding| binding.trigger == semio_framework_ui_contract::Trigger::Change).expect("enum change binding");
    let Some(UiValue::Map(arguments)) = &binding.args else { panic!("enum path arguments") };
    assert!(arguments.iter().any(|(key, value)| key.as_str() == "path" && matches!(value, UiValue::Text(value) if value.as_str().is_empty())));
    let event = super::super::snapshot_edit_event_from_action(
        super::super::SET_SNAPSHOT_VALUE_ACTION_ID,
        Some(&DslValue::Object(vec![("path".into(), DslValue::String(String::new())), ("value".into(), DslValue::String("published".into()))])),
    )
    .expect("parse enum action")
    .expect("known action");
    assert_eq!(event, super::super::SnapshotEditEvent::SetValue { path: String::new(), value: DslValue::String("published".into()) });
}

#[test]
fn unicode_rfc6901_paths_past_one_text_value_use_lossless_chunks() {
    let key = "Pfad/mit~Unicode-🚀é".repeat(64);
    let path = pointer(&[SnapshotDetailPathSegment::Key(key.clone())]);
    assert!(path.len() > semio_framework_ui_contract::UI_TEXT_MAX_BYTES);
    let (argument, value) = pointer_argument(&path, "path", "pathChunks").expect("chunked pointer");
    assert_eq!(argument, "pathChunks");
    let UiValue::List(chunks) = value else { panic!("path chunk list") };
    let mut cursor = chunks.cursor();
    let mut decoded = Vec::new();
    while let Some(chunk) = cursor.next() {
        let UiValue::Text(chunk) = chunk else { panic!("text chunk") };
        assert!(chunk.len() <= semio_framework_ui_contract::UI_TEXT_MAX_BYTES);
        decoded.push(DslValue::String(chunk.to_string()));
    }
    let event = super::super::snapshot_edit_event_from_action(
        super::super::SET_SNAPSHOT_VALUE_ACTION_ID,
        Some(&DslValue::Object(vec![(argument.into(), DslValue::Array(decoded)), ("value".into(), DslValue::String("after".into()))])),
    )
    .expect("parse chunked control")
    .expect("known action");
    assert_eq!(event, super::super::SnapshotEditEvent::SetValue { path: path.clone(), value: DslValue::String("after".into()) });
    let oracle = serde_json::json!({ key: "before" });
    assert_eq!(oracle.pointer(&path), Some(&serde_json::json!("before")));
}

#[test]
fn paths_beyond_the_flat_chunk_carrier_open_the_complete_editable_source() {
    struct HugeKey(String);
    impl SnapshotDetailsProvider for HugeKey {
        fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue> {
            match path {
                [] => Some(SnapshotDetailValue::Object),
                [SnapshotDetailPathSegment::Key(key)] if key == &self.0 => Some(SnapshotDetailValue::String("before".into())),
                _ => None,
            }
        }
        fn child_count(&self, path: &[SnapshotDetailPathSegment]) -> usize { usize::from(path.is_empty()) }
        fn object_key(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<String> { (path.is_empty() && index == 0).then(|| self.0.clone()) }
        fn has_source(&self) -> bool { true }
        fn source(&self) -> Option<String> { Some(super::super::snapshot_edit_source(&DslValue::Object(vec![(self.0.clone(), DslValue::String("before".into()))]))) }
    }
    let provider = HugeKey("🚀".repeat(40_000));
    let path = pointer(&[SnapshotDetailPathSegment::Key(provider.0.clone())]);
    assert!(path.len() > semio_framework_ui_contract::UI_TEXT_MAX_BYTES * semio_framework_ui_contract::UI_VALUE_MAX_ITEMS);
    let rendered = render_snapshot_details_provider(&provider, Locale::En, "test.huge-key#editor", &TreeWindows::unhosted()).expect("source fallback");
    let fallback = find_key(&rendered, &format!("{}-source-fallback", path_id(&[SnapshotDetailPathSegment::Key(provider.0.clone())]))).expect("editable source fallback");
    let surface = fallback.children.get(0).expect("draft surface");
    let semio_framework_ui_contract::Component::Surface(props) = &surface.component else { panic!("fallback must be a text draft") };
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(props).expect("draft scene");
    let settings: serde_json::Value = serde_json::from_str(scene.settings_json.as_deref().expect("draft settings")).expect("settings JSON");
    assert_eq!(settings["readOnly"], false);
    assert_eq!(settings["editAction"], super::super::REPLACE_SNAPSHOT_SOURCE_ACTION_ID);
    assert_eq!(settings["commit"], "explicit");
    assert!(scene.buffer.contains("before"));
}
