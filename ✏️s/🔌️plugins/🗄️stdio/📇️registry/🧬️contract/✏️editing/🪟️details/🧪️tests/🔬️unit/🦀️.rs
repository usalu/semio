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

struct NativeLazySnapshot;

impl ToValue for NativeLazySnapshot {
    fn to_value(&self) -> DslValue {
        panic!("details must not materialize a native snapshot")
    }

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, crate::kernel::ValueError> {
        match path {
            ["schema"] => Ok(DslValue::String("stdio.lazy-details-test".into())),
            ["title"] => Ok(DslValue::String("Projected title".into())),
            _ => Err(crate::kernel::ValueError::new("container reads must use shape/key access")),
        }
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, crate::kernel::ValueError> {
        match path {
            [] => Ok(ValueShape::Object { len: 2 }),
            ["schema"] | ["title"] => Ok(ValueShape::String),
            _ => Err(crate::kernel::ValueError::new("unknown test path")),
        }
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, crate::kernel::ValueError> {
        match (path, index) {
            ([], 0) => Ok("schema".into()),
            ([], 1) => Ok("title".into()),
            _ => Err(crate::kernel::ValueError::new("unknown test key")),
        }
    }
}

impl ArtifactDsl for NativeLazySnapshot {
    const EXTENSION: &'static str = "lazy-details-test";

    fn parse_dsl(_text: &str) -> Result<Self, crate::kernel::TextError> {
        Ok(Self)
    }

    fn print_dsl(&self) -> String {
        "lazy-details-test".into()
    }
}

#[test]
fn native_details_use_shape_key_and_scalar_projections_without_materializing_the_snapshot() {
    let provider = DslSnapshotDetailsProvider::new_with_schema_hint(&NativeLazySnapshot, Some("s.stdio.unrelated@test/*#editor"));
    assert_eq!(provider.child_count(&[]), 2);
    assert_eq!(provider.object_key(&[], 1).as_deref(), Some("title"));
    assert_eq!(provider.value(&[SnapshotDetailPathSegment::Key("title".into())]), Some(SnapshotDetailValue::String("Projected title".into())));
}

#[test]
fn lazy_collection_renders_a_bounded_page_without_materializing_every_pixel() {
    let provider = LazyPixels { count: 16_000_000 };
    let result = render_snapshot_details_provider(&provider, Locale::En, "s.stdio.png@test/*#editor", &TreeWindows::unhosted());
    assert!(result.is_ok());
}

#[test]
fn lazy_large_object_controls_do_not_scan_unpaged_keys() {
    use std::cell::Cell;
    struct LazyObject {
        key_reads: Cell<usize>,
    }
    impl SnapshotDetailsProvider for LazyObject {
        fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue> {
            match path {
                [] => Some(SnapshotDetailValue::Object),
                [SnapshotDetailPathSegment::Key(_)] => Some(SnapshotDetailValue::String("value".into())),
                _ => None,
            }
        }
        fn child_count(&self, path: &[SnapshotDetailPathSegment]) -> usize {
            if path.is_empty() {
                16_000_000
            } else {
                0
            }
        }
        fn object_key(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<String> {
            if path.is_empty() && index < 16_000_000 {
                self.key_reads.set(self.key_reads.get() + 1);
                Some(format!("key-{index}"))
            } else {
                None
            }
        }
        fn collection_insertion_key(&self, path: &[SnapshotDetailPathSegment]) -> Option<String> {
            path.is_empty().then(|| "new-16000001".into())
        }
    }
    let provider = LazyObject { key_reads: Cell::new(0) };
    render_snapshot_details_provider(&provider, Locale::En, "test.lazy-object#editor", &TreeWindows::unhosted()).expect("bounded large object");
    assert!(provider.key_reads.get() < 1_024, "visible paging must bound key reads, got {}", provider.key_reads.get());
}

#[test]
fn an_exact_chunk_budget_parent_omits_unbindable_derived_insert_controls_without_failing_the_panel() {
    struct BoundaryPath {
        key: String,
    }
    impl SnapshotDetailsProvider for BoundaryPath {
        fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue> {
            match path {
                [] | [SnapshotDetailPathSegment::Key(_)] => Some(SnapshotDetailValue::Object),
                _ => None,
            }
        }
        fn child_count(&self, path: &[SnapshotDetailPathSegment]) -> usize {
            if path.is_empty() {
                1
            } else {
                0
            }
        }
        fn object_key(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<String> {
            (path.is_empty() && index == 0).then(|| self.key.clone())
        }
        fn has_source(&self) -> bool {
            true
        }
        fn source(&self) -> Option<String> {
            Some("{}".into())
        }
        fn collection_insertion_key(&self, path: &[SnapshotDetailPathSegment]) -> Option<String> {
            (!path.is_empty()).then(|| "new".into())
        }
    }
    let key = "a".repeat(ui_contract::UI_TEXT_MAX_BYTES * ui_contract::UI_VALUE_MAX_ITEMS - 1);
    let path = pointer(&[SnapshotDetailPathSegment::Key(key.clone())]);
    assert!(path_is_bindable(&path));
    assert!(!path_is_bindable(&format!("{path}/new")));
    let provider = BoundaryPath { key };
    let rendered = render_snapshot_details_provider(&provider, Locale::En, "test.boundary-path#editor", &TreeWindows::unhosted()).expect("derived path is safely omitted");
    let id = path_id(&[SnapshotDetailPathSegment::Key(provider.key.clone())]);
    assert!(!contains_key(&rendered, &format!("{id}-add-0")));
    assert!(contains_key(&rendered, "stdio-snapshot-details-source-draft"));
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
    let view = ViewModel { tree_windows: vec![TreeWindowRequest { body_key: SNAPSHOT_DETAILS_BODY_KEY.into(), node_key, open: Some(true), offset: 100, rows: 3 }], tree_viewport_rows: Some(4), ..Default::default() };
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
        ..Default::default()
    };
    let windows = TreeWindows::for_body(&view, SNAPSHOT_DETAILS_BODY_KEY);
    let rendered = render_snapshot_details_provider(&provider, Locale::En, "test.derived-controls#editor", &windows).expect("windowed derived controls");
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
        let rendered = render_snapshot_details_provider(&Numeric(number), Locale::En, "test.number#editor", &TreeWindows::unhosted()).expect("number details");
        let input = find_key(&rendered, &format!("{ROOT_PATH_ID}-number")).expect("number input");
        let semio_framework_ui_contract::Component::Input(props) = &input.component else { panic!("number editor must remain an exact JSON text input") };
        assert_eq!(props.kind, semio_framework_ui_contract::InputKind::Text);
        let binding = input.bindings.iter().find(|binding| binding.trigger == semio_framework_ui_contract::Trigger::Commit).expect("commit binding");
        let Some(UiValue::Map(arguments)) = &binding.args else { panic!("static arguments") };
        assert!(arguments.iter().any(|(key, value)| key.as_str() == "valueEncoding" && matches!(value, UiValue::Text(value) if value.as_str() == "json")));
        let args = DslValue::Object(vec![("path".into(), DslValue::String(String::new())), ("value".into(), DslValue::String(props.value.as_str().into())), ("valueEncoding".into(), DslValue::String("json".into()))]);
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
            if path.is_empty() {
                3
            } else {
                0
            }
        }
        fn object_key(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<String> {
            (path.is_empty()).then(|| ["schema", "records", "selection"].get(index).map(|key| (*key).to_string())).flatten()
        }
        fn creation_template(&self, path: &[SnapshotDetailPathSegment], collection_item: bool) -> Option<DslValue> {
            let root = &self.schema;
            let mut read = |_path: &[SnapshotDetailPathSegment]| None;
            let mut shape = |_path: &[SnapshotDetailPathSegment]| None;
            let mut schema = creation_schema_at_path(root, path, &mut read, &mut shape)?;
            if collection_item {
                schema = object_field(schema, "items")?;
            }
            template_from_schema(root, schema, 0)
        }
        fn allows_untyped_creation(&self, _path: &[SnapshotDetailPathSegment]) -> bool {
            false
        }
    }
    let provider = TypedRecords { schema: crate::pack::json::from_json_str(schema).expect("schema") };
    assert_eq!(provider.creation_template(&[SnapshotDetailPathSegment::Key("records".into())], true), Some(DslValue::Object(vec![("fields".into(), DslValue::Array(Vec::new()))])));
    assert_eq!(provider.creation_template(&[SnapshotDetailPathSegment::Key("selection".into())], false), Some(DslValue::Object(vec![("name".into(), DslValue::String("x".into())), ("enabled".into(), DslValue::Bool(false)),])));
    let rendered = render_snapshot_details_provider(&provider, Locale::En, "test.records#editor", &TreeWindows::unhosted()).expect("schema-aware details");
    let records_id = path_id(&[SnapshotDetailPathSegment::Key("records".into())]);
    let selection_id = path_id(&[SnapshotDetailPathSegment::Key("selection".into())]);
    assert!(contains_key(&rendered, &format!("{records_id}-add-template")));
    assert!(contains_key(&rendered, &format!("{selection_id}-null-template")));
    assert!(!contains_key(&rendered, &format!("{records_id}-add-0")));
    assert!(!contains_key(&rendered, &format!("{selection_id}-null-0")));
    let oracle: serde_json::Value = serde_json::from_str(&super::super::snapshot_edit_source(&provider.creation_template(&[SnapshotDetailPathSegment::Key("selection".into())], false).unwrap())).expect("third-party JSON oracle");
    assert_eq!(oracle, serde_json::json!({"name":"x","enabled":false}));
}

#[test]
fn schema_templates_reject_invalid_defaults_and_satisfy_min_properties() {
    struct ValidTemplateFixture;
    impl ToValue for ValidTemplateFixture {
        fn to_value(&self) -> DslValue {
            DslValue::Null
        }
    }
    impl ArtifactDsl for ValidTemplateFixture {
        const EXTENSION: &'static str = "json";
        fn parse_dsl(_text: &str) -> Result<Self, crate::kernel::TextError> {
            Err(crate::kernel::TextError::new("unused template fixture", crate::kernel::TextSpan::at(1, 1)))
        }
        fn print_dsl(&self) -> String {
            "null".into()
        }
    }
    let schema = r#"{
        "$schema":"http://json-schema.org/draft-07/schema#",
        "type":"object",
        "required":["short","record"],
        "properties":{
            "short":{"type":"string","minLength":2,"default":""},
            "coded":{"type":"string","pattern":"^OK$","default":"invalid"},
            "record":{
                "type":"object",
                "minProperties":1,
                "properties":{"title":{"type":"string","default":"Created"}},
                "additionalProperties":false
            }
        }
    }"#;
    let provider = DslSnapshotDetailsProvider::<ValidTemplateFixture>::from_value_and_schema(DslValue::Object(Vec::new()), schema);
    let short_path = [SnapshotDetailPathSegment::Key("short".into())];
    let coded_path = [SnapshotDetailPathSegment::Key("coded".into())];
    let record_path = [SnapshotDetailPathSegment::Key("record".into())];
    let short = provider.creation_template(&short_path, false).expect("valid string fallback");
    let record = provider.creation_template(&record_path, false).expect("valid min-properties record");
    assert_eq!(short, DslValue::String("xx".into()));
    assert_eq!(provider.creation_template(&coded_path, false), None, "an annotation default rejected by an unmodeled pattern is never offered");
    assert_eq!(record, DslValue::Object(vec![("title".into(), DslValue::String("Created".into()))]));
    let candidate = DslValue::Object(vec![("short".into(), short), ("record".into(), record)]);
    let source = super::super::snapshot_edit_source(&candidate);
    let oracle: serde_json::Value = serde_json::from_str(&source).expect("third-party JSON oracle");
    assert_eq!(oracle, serde_json::json!({"short":"xx","record":{"title":"Created"}}));
    semio_framework_schema::OwnedJsonSchemaValidator::compile(schema).expect("native schema compile").validate_json(&source).expect("offered templates satisfy the authoritative validator");
}

#[test]
fn schema_annotations_localize_fixed_fields_without_translating_user_keys() {
    struct PresentationFixture;
    impl ToValue for PresentationFixture {
        fn to_value(&self) -> DslValue {
            DslValue::Null
        }
    }
    impl ArtifactDsl for PresentationFixture {
        const EXTENSION: &'static str = "json";
        fn parse_dsl(_text: &str) -> Result<Self, crate::kernel::TextError> {
            Err(crate::kernel::TextError::new("unused presentation fixture", crate::kernel::TextSpan::at(1, 1)))
        }
        fn print_dsl(&self) -> String {
            "null".into()
        }
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
    let value = DslValue::Object(vec![("windowBits".into(), DslValue::Number(Number::UInt(15))), ("Projektname".into(), DslValue::String("Semio".into()))]);
    let provider = DslSnapshotDetailsProvider::<PresentationFixture>::from_value_and_schema(value, schema);
    assert_eq!(provider.presentation(&[SnapshotDetailPathSegment::Key("windowBits".into())], Locale::En), Some(SnapshotDetailPresentation { label: "Window size [windowBits]".into(), description: Some("Compression window in bits.".into()) }));
    assert_eq!(provider.presentation(&[SnapshotDetailPathSegment::Key("windowBits".into())], Locale::De), Some(SnapshotDetailPresentation { label: "Fenstergröße [windowBits]".into(), description: Some("Kompressionsfenster in Bit.".into()) }));
    assert_eq!(provider.presentation(&[SnapshotDetailPathSegment::Key("Projektname".into())], Locale::De), None);
    for (locale, expected_label, expected_description) in [(Locale::En, "Window size [windowBits]", "Compression window in bits."), (Locale::De, "Fenstergröße [windowBits]", "Kompressionsfenster in Bit.")] {
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
fn schema_capabilities_hide_impossible_actions_and_offer_missing_optional_fields() {
    struct CapabilityFixture;
    impl ToValue for CapabilityFixture {
        fn to_value(&self) -> DslValue {
            DslValue::Null
        }
    }
    impl ArtifactDsl for CapabilityFixture {
        const EXTENSION: &'static str = "json";
        fn parse_dsl(_text: &str) -> Result<Self, crate::kernel::TextError> {
            Err(crate::kernel::TextError::new("unused capability fixture", crate::kernel::TextSpan::at(1, 1)))
        }
        fn print_dsl(&self) -> String {
            "null".into()
        }
    }
    let schema = r#"{
        "type":"object",
        "required":["schema","requiredField","items"],
        "properties":{
            "schema":{"type":"string"},
            "requiredField":{"type":"string"},
            "optionalLabel":{
                "type":"string",
                "default":"Untitled",
                "x-semio-title":{"en":"Display name","de":"Anzeigename"}
            },
            "items":{"type":"array","minItems":1,"maxItems":1,"items":{"type":"string"}}
        },
        "additionalProperties":true
    }"#;
    let value = DslValue::Object(vec![
        ("schema".into(), DslValue::String("test.capabilities".into())),
        ("requiredField".into(), DslValue::String("required".into())),
        ("items".into(), DslValue::Array(vec![DslValue::String("only".into())])),
        ("custom".into(), DslValue::String("dynamic".into())),
    ]);
    let provider = DslSnapshotDetailsProvider::<CapabilityFixture>::from_value_and_schema(value, schema);
    let rendered = render_snapshot_details_provider(&provider, Locale::En, "test.capabilities#editor", &TreeWindows::unhosted()).expect("capability details");
    let schema_id = path_id(&[SnapshotDetailPathSegment::Key("schema".into())]);
    assert!(!contains_key(&rendered, &format!("{schema_id}-text")));
    assert!(!contains_key(&rendered, &format!("{schema_id}-key")));
    assert!(!contains_key(&rendered, &format!("{schema_id}-remove")));
    let required_id = path_id(&[SnapshotDetailPathSegment::Key("requiredField".into())]);
    assert!(contains_key(&rendered, &format!("{required_id}-text")));
    assert!(!contains_key(&rendered, &format!("{required_id}-key")));
    assert!(!contains_key(&rendered, &format!("{required_id}-remove")));
    let custom_id = path_id(&[SnapshotDetailPathSegment::Key("custom".into())]);
    assert!(contains_key(&rendered, &format!("{custom_id}-key")));
    assert!(contains_key(&rendered, &format!("{custom_id}-remove")));
    let items_path = [SnapshotDetailPathSegment::Key("items".into())];
    let items_id = path_id(&items_path);
    let item_id = path_id(&[SnapshotDetailPathSegment::Key("items".into()), SnapshotDetailPathSegment::Index(0)]);
    assert!(!contains_key(&rendered, &format!("{item_id}-remove")));
    assert!(!contains_key(&rendered, &format!("{items_id}-add-template")));
    assert!(!contains_key(&rendered, &format!("{items_id}-add-0")));
    let optional = find_key(&rendered, &format!("{ROOT_PATH_ID}-add-property-0")).expect("optional property choice");
    let semio_framework_ui_contract::Component::Button(props) = &optional.component else { panic!("optional property choice must be a button") };
    assert_eq!(props.label.0.as_str(), "Add Display name [optionalLabel]");
    let binding = optional.bindings.iter().find(|binding| binding.trigger == semio_framework_ui_contract::Trigger::Activate).expect("optional property insert binding");
    let Some(UiValue::Map(arguments)) = &binding.args else { panic!("optional property typed arguments") };
    assert!(arguments.iter().any(|(key, value)| key.as_str() == "path" && matches!(value, UiValue::Text(path) if path.as_str() == "/optionalLabel")));
    assert!(arguments.iter().any(|(key, value)| key.as_str() == "value" && matches!(value, UiValue::Text(value) if value.as_str() == "\"Untitled\"")));
    assert!(arguments.iter().any(|(key, value)| key.as_str() == "valueEncoding" && matches!(value, UiValue::Text(value) if value.as_str() == "json")));
    let german = render_snapshot_details_provider(&provider, Locale::De, "test.capabilities#editor", &TreeWindows::unhosted()).expect("German capability details");
    let optional = find_key(&german, &format!("{ROOT_PATH_ID}-add-property-0")).expect("German optional property choice");
    let semio_framework_ui_contract::Component::Button(props) = &optional.component else { panic!("optional property choice must be a button") };
    assert_eq!(props.label.0.as_str(), "Hinzufügen Anzeigename [optionalLabel]");
}

#[test]
fn object_min_properties_hides_removal_at_the_boundary() {
    struct MinPropertiesFixture;
    impl ToValue for MinPropertiesFixture {
        fn to_value(&self) -> DslValue {
            DslValue::Null
        }
    }
    impl ArtifactDsl for MinPropertiesFixture {
        const EXTENSION: &'static str = "json";
        fn parse_dsl(_text: &str) -> Result<Self, crate::kernel::TextError> {
            Err(crate::kernel::TextError::new("unused min-properties fixture", crate::kernel::TextSpan::at(1, 1)))
        }
        fn print_dsl(&self) -> String {
            "null".into()
        }
    }
    let schema = r#"{
        "type":"object",
        "minProperties":3,
        "maxProperties":4,
        "required":["schema","fixed"],
        "properties":{"schema":{"type":"string"},"fixed":{"type":"string"}},
        "additionalProperties":true
    }"#;
    let value = DslValue::Object(vec![("schema".into(), DslValue::String("test.min-properties".into())), ("fixed".into(), DslValue::String("fixed".into())), ("dynamic".into(), DslValue::String("dynamic".into()))]);
    let provider = DslSnapshotDetailsProvider::<MinPropertiesFixture>::from_value_and_schema(value, schema);
    let path = [SnapshotDetailPathSegment::Key("dynamic".into())];
    let capabilities = provider.item_capabilities(&path);
    assert!(capabilities.rename);
    assert!(!capabilities.remove);
    assert!(provider.allows_collection_insert(&[]));
    let rendered = render_snapshot_details_provider(&provider, Locale::En, "test.min-properties#editor", &TreeWindows::unhosted()).expect("min properties details");
    assert!(!contains_key(&rendered, &format!("{}-remove", path_id(&path))));
}

#[test]
fn tagged_union_details_follow_the_current_native_discriminator() {
    struct TaggedFixture;
    impl ToValue for TaggedFixture {
        fn to_value(&self) -> DslValue {
            DslValue::Null
        }
    }
    impl ArtifactDsl for TaggedFixture {
        const EXTENSION: &'static str = "json";
        fn parse_dsl(_text: &str) -> Result<Self, crate::kernel::TextError> {
            Err(crate::kernel::TextError::new("unused tagged fixture", crate::kernel::TextSpan::at(1, 1)))
        }
        fn print_dsl(&self) -> String {
            "null".into()
        }
    }
    let schema = r#"{
        "type":"object",
        "properties":{
            "variant":{
                "oneOf":[
                    {
                        "type":"object",
                        "x-semio-title":{"en":"Named","de":"Benannt"},
                        "required":["kind","value","label"],
                        "properties":{
                            "kind":{"const":"named"},
                            "value":{"type":"string","enum":["draft","review"],"title":"Named status"},
                            "label":{"type":"string"}
                        }
                    },
                    {
                        "type":"object",
                        "x-semio-title":{"en":"Automated","de":"Automatisiert"},
                        "required":["kind","value","label"],
                        "properties":{
                            "kind":{"const":"automated"},
                            "value":{"type":"string","enum":["queued","running"],"title":"Automated status"},
                            "label":{"type":"string"}
                        }
                    }
                ]
            }
        }
    }"#;
    let value_path = [SnapshotDetailPathSegment::Key("variant".into()), SnapshotDetailPathSegment::Key("value".into())];
    for (kind, value, expected, title) in [("named", "review", vec!["draft", "review"], "Named status [value]"), ("automated", "running", vec!["queued", "running"], "Automated status [value]")] {
        let value = DslValue::Object(vec![("variant".into(), DslValue::Object(vec![("kind".into(), DslValue::String(kind.into())), ("value".into(), DslValue::String(value.into())), ("label".into(), DslValue::String("Keep me".into()))]))]);
        let provider = DslSnapshotDetailsProvider::<TaggedFixture>::from_value_and_schema(value, schema);
        assert_eq!(provider.enum_values(&value_path), expected.into_iter().map(|value| DslValue::String(value.into())).collect::<Vec<_>>());
        assert_eq!(provider.presentation(&value_path, Locale::En).map(|presentation| presentation.label), Some(title.into()));
        if kind == "named" {
            let variant_path = [SnapshotDetailPathSegment::Key("variant".into())];
            assert_eq!(
                provider.variants(&variant_path, Locale::De),
                vec![SnapshotDetailVariant {
                    label: "Automatisiert".into(),
                    value: DslValue::Object(vec![("kind".into(), DslValue::String("automated".into())), ("value".into(), DslValue::String("queued".into())), ("label".into(), DslValue::String("Keep me".into())),]),
                }]
            );
            let rendered = render_snapshot_details_provider(&provider, Locale::De, "test.tagged#editor", &TreeWindows::unhosted()).expect("tagged union details");
            let kind_id = path_id(&[SnapshotDetailPathSegment::Key("variant".into()), SnapshotDetailPathSegment::Key("kind".into())]);
            assert!(!contains_key(&rendered, &format!("{kind_id}-text")), "the discriminator is switched only through a complete valid variant");
            assert!(!contains_key(&rendered, &format!("{kind_id}-enum")), "the discriminator has no lossy scalar select");
            let variant_id = path_id(&variant_path);
            let switch = find_key(&rendered, &format!("{variant_id}-variant-0")).expect("variant switch");
            let semio_framework_ui_contract::Component::Button(props) = &switch.component else { panic!("variant switch must be a button") };
            assert_eq!(props.label.0.as_str(), "Wechseln zu Automatisiert");
            let binding = switch.bindings.iter().find(|binding| binding.trigger == semio_framework_ui_contract::Trigger::Activate).expect("variant switch binding");
            let Some(UiValue::Map(arguments)) = &binding.args else { panic!("variant switch arguments") };
            assert!(arguments.iter().any(|(key, value)| key.as_str() == "path" && matches!(value, UiValue::Text(path) if path.as_str() == "/variant")));
            let source = arguments
                .iter()
                .find_map(|(key, value)| (key.as_str() == "value").then_some(value))
                .and_then(|value| match value {
                    UiValue::Text(value) => Some(value.as_str().to_owned()),
                    _ => None,
                })
                .expect("variant source");
            assert_eq!(crate::pack::json::from_json_str::<DslValue>(&source).expect("variant JSON"), provider.variants(&variant_path, Locale::De)[0].value);
        }
    }
    let ambiguous = DslValue::Object(vec![("variant".into(), DslValue::Object(vec![("value".into(), DslValue::String("review".into()))]))]);
    let provider = DslSnapshotDetailsProvider::<TaggedFixture>::from_value_and_schema(ambiguous, schema);
    assert!(provider.enum_values(&value_path).is_empty(), "an absent discriminator must not expose the first branch's enum");
}

#[test]
fn external_all_of_union_uses_the_registered_document_shape_for_controls_and_variants() {
    struct EnvelopeFixture;
    impl ToValue for EnvelopeFixture {
        fn to_value(&self) -> DslValue {
            DslValue::Null
        }
    }
    impl ArtifactDsl for EnvelopeFixture {
        const EXTENSION: &'static str = "json";
        fn parse_dsl(_text: &str) -> Result<Self, crate::kernel::TextError> {
            Err(crate::kernel::TextError::new("unused envelope fixture", crate::kernel::TextSpan::at(1, 1)))
        }
        fn print_dsl(&self) -> String {
            "null".into()
        }
    }
    const ENVELOPE: &str = include_str!("../../🧫️fixtures/🔗️external-all-of/✉️envelope/🔣️.json");
    const MODEL: &str = include_str!("../../🧫️fixtures/🔗️external-all-of/🏛️model/🔣️.json");
    const TEXT: &str = include_str!("../../🧫️fixtures/🔗️external-all-of/📝️text/🔣️.json");
    let empty = semio_framework_schema::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" };
    for (id, source) in [("s.test.details-envelope", ENVELOPE), ("s.test.details-model", MODEL), ("s.test.details-text", TEXT)] {
        semio_framework_schema::register_artifact_schema_descriptor(semio_framework_schema::ArtifactSchemaDescriptor {
            id,
            artifact: empty,
            snapshot: semio_framework_schema::FacetLeaves { json_schema: source, ..empty },
            diff: empty,
            mutations: empty,
        });
    }
    let value = crate::pack::json::from_json_str(include_str!("../../🧫️fixtures/🔗️external-all-of/⬅️current/🔣️.json")).expect("language-agnostic value fixture");
    let provider = DslSnapshotDetailsProvider::<EnvelopeFixture>::from_value(value);
    let item_path = [SnapshotDetailPathSegment::Key("subset".into()), SnapshotDetailPathSegment::Key("spatial".into()), SnapshotDetailPathSegment::Index(0)];
    let mut id_path = item_path.to_vec();
    id_path.push(SnapshotDetailPathSegment::Key("id".into()));
    let capabilities = provider.item_capabilities(&id_path);
    assert!(capabilities.edit);
    assert!(!capabilities.rename);
    assert!(!capabilities.remove);
    let nested_schema_path = [SnapshotDetailPathSegment::Key("subset".into()), SnapshotDetailPathSegment::Key("schema".into())];
    assert_eq!(provider.item_capabilities(&nested_schema_path), SnapshotDetailItemCapabilities { edit: false, rename: false, remove: false, reorder: false });
    let spatial_path = [SnapshotDetailPathSegment::Key("subset".into()), SnapshotDetailPathSegment::Key("spatial".into())];
    assert!(!provider.allows_collection_insert(&spatial_path), "allOf intersects the active branch's maxItems with the external document");
    let mut kind_path = item_path.to_vec();
    kind_path.push(SnapshotDetailPathSegment::Key("kind".into()));
    assert_eq!(provider.enum_values(&kind_path), vec![DslValue::String("site".into()), DslValue::String("building".into())]);
    assert_eq!(provider.presentation(&kind_path, Locale::De), Some(SnapshotDetailPresentation { label: "Räumliche Art [kind]".into(), description: None }));
    assert_eq!(provider.missing_property_templates(&item_path), vec![("name".into(), DslValue::String("Untitled".into()))]);
    let subset_path = [SnapshotDetailPathSegment::Key("subset".into())];
    let variants = provider.variants(&subset_path, Locale::En);
    assert_eq!(variants.len(), 1);
    assert_eq!(variants[0].value, crate::pack::json::from_json_str::<DslValue>(include_str!("../../🧫️fixtures/🔗️external-all-of/➡️text-variant/🔣️.json")).expect("language-agnostic target fixture"));
    let candidate = serde_json::json!({
        "schema":"test.details-envelope",
        "subset":serde_json::from_str::<serde_json::Value>(&super::super::snapshot_edit_source(&variants[0].value)).expect("variant JSON")
    });
    let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile_with_documents(ENVELOPE, &[MODEL, TEXT]).expect("cross-document validator");
    validator.validate_json(&serde_json::to_string(&candidate).expect("candidate JSON")).expect("alternate variant is authoritative-schema valid");
}

#[test]
fn enum_values_render_as_an_accessible_typed_select() {
    struct EnumValue;
    impl SnapshotDetailsProvider for EnumValue {
        fn value(&self, path: &[SnapshotDetailPathSegment]) -> Option<SnapshotDetailValue> {
            path.is_empty().then(|| SnapshotDetailValue::String("review".into()))
        }
        fn child_count(&self, _path: &[SnapshotDetailPathSegment]) -> usize {
            0
        }
        fn object_key(&self, _path: &[SnapshotDetailPathSegment], _index: usize) -> Option<String> {
            None
        }
        fn enum_values(&self, path: &[SnapshotDetailPathSegment]) -> Vec<DslValue> {
            path.is_empty().then(|| vec![DslValue::String("draft".into()), DslValue::String("review".into()), DslValue::String("published".into())]).unwrap_or_default()
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
    let event = super::super::snapshot_edit_event_from_action(super::super::SET_SNAPSHOT_VALUE_ACTION_ID, Some(&DslValue::Object(vec![("path".into(), DslValue::String(String::new())), ("value".into(), DslValue::String("published".into()))])))
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
    let event = super::super::snapshot_edit_event_from_action(super::super::SET_SNAPSHOT_VALUE_ACTION_ID, Some(&DslValue::Object(vec![(argument.into(), DslValue::Array(decoded)), ("value".into(), DslValue::String("after".into()))])))
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
        fn child_count(&self, path: &[SnapshotDetailPathSegment]) -> usize {
            usize::from(path.is_empty())
        }
        fn object_key(&self, path: &[SnapshotDetailPathSegment], index: usize) -> Option<String> {
            (path.is_empty() && index == 0).then(|| self.0.clone())
        }
        fn has_source(&self) -> bool {
            true
        }
        fn source(&self) -> Option<String> {
            Some(super::super::snapshot_edit_source(&DslValue::Object(vec![(self.0.clone(), DslValue::String("before".into()))])))
        }
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
