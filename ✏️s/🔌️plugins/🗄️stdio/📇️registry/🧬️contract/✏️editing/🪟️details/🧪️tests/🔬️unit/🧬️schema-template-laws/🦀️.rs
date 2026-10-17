/// 🧬️ Schema templates, annotations and capabilities laws.

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
    let provider = TypedRecords { schema: semio_framework_pack_json::from_json_str(schema, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("schema") };
    assert_eq!(provider.creation_template(&[SnapshotDetailPathSegment::Key("records".into())], true), Some(DslValue::Object(vec![("fields".into(), DslValue::Array(Vec::new()))])));
    assert_eq!(provider.creation_template(&[SnapshotDetailPathSegment::Key("selection".into())], false), Some(DslValue::Object(vec![("name".into(), DslValue::String("x".into())), ("enabled".into(), DslValue::Bool(false)),])));
    let rendered = render_snapshot_details_provider_revisioned(&provider, semio_framework_ui_contract::UiPublicationRevision(1), Locale::En, "test.records#editor", &TreeWindows::unhosted()).expect("schema-aware details");
    let records_id = path_id(&[SnapshotDetailPathSegment::Key("records".into())]);
    let selection_id = path_id(&[SnapshotDetailPathSegment::Key("selection".into())]);
    assert!(contains_key(&rendered, &format!("{records_id}-add-template")));
    assert!(contains_key(&rendered, &format!("{selection_id}-null-template")));
    assert!(!contains_key(&rendered, &format!("{records_id}-add-0")));
    assert!(!contains_key(&rendered, &format!("{selection_id}-null-0")));
    let oracle: serde_json::Value = serde_json::from_str(&snapshot_edit_source(&provider.creation_template(&[SnapshotDetailPathSegment::Key("selection".into())], false).unwrap())).expect("third-party JSON oracle");
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
            Err(crate::kernel::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unused template fixture", crate::kernel::TextSpan::at(1, 1)))
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
    let source = snapshot_edit_source(&candidate);
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
            Err(crate::kernel::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unused presentation fixture", crate::kernel::TextSpan::at(1, 1)))
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
        let rendered = render_snapshot_details_provider_revisioned(&provider, semio_framework_ui_contract::UiPublicationRevision(1), locale, "test.presentation#editor", &TreeWindows::unhosted()).expect("localized details");
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
            Err(crate::kernel::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unused capability fixture", crate::kernel::TextSpan::at(1, 1)))
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
    let rendered = render_snapshot_details_provider_revisioned(&provider, semio_framework_ui_contract::UiPublicationRevision(1), Locale::En, "test.capabilities#editor", &TreeWindows::unhosted()).expect("capability details");
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
    let german = render_snapshot_details_provider_revisioned(&provider, semio_framework_ui_contract::UiPublicationRevision(1), Locale::De, "test.capabilities#editor", &TreeWindows::unhosted()).expect("German capability details");
    let optional = find_key(&german, &format!("{ROOT_PATH_ID}-add-property-0")).expect("German optional property choice");
    let semio_framework_ui_contract::Component::Button(props) = &optional.component else { panic!("optional property choice must be a button") };
    assert_eq!(props.label.0.as_str(), "Hinzufügen Anzeigename [optionalLabel]");
}
