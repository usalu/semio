/// 🔀️ Tagged union, enum, long path, source draft and nested window laws.

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
            Err(crate::kernel::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unused min-properties fixture", crate::kernel::TextSpan::at(1, 1)))
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
    let rendered = render_snapshot_details_provider_revisioned(&provider, semio_framework_ui_contract::UiPublicationRevision(1), Locale::En, "test.min-properties#editor", &TreeWindows::unhosted()).expect("min properties details");
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
            Err(crate::kernel::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unused tagged fixture", crate::kernel::TextSpan::at(1, 1)))
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
            let rendered = render_snapshot_details_provider_revisioned(&provider, semio_framework_ui_contract::UiPublicationRevision(1), Locale::De, "test.tagged#editor", &TreeWindows::unhosted()).expect("tagged union details");
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
            assert_eq!(semio_framework_pack_json::from_json_str::<DslValue>(&source, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("variant JSON"), provider.variants(&variant_path, Locale::De)[0].value);
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
            Err(crate::kernel::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unused envelope fixture", crate::kernel::TextSpan::at(1, 1)))
        }
        fn print_dsl(&self) -> String {
            "null".into()
        }
    }
    const ENVELOPE: &str = include_str!("../../🧫️fixtures/🔗️external-all-of/✉️envelope/🧾️schema-input.json");
    const MODEL: &str = include_str!("../../🧫️fixtures/🔗️external-all-of/🏛️model/🧾️schema-input.json");
    const TEXT: &str = include_str!("../../🧫️fixtures/🔗️external-all-of/📝️text/🧾️schema-input.json");
    let empty = semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "", proto: "" };
    for (id, source) in [("s.test.details-envelope", ENVELOPE), ("s.test.details-model", MODEL), ("s.test.details-text", TEXT)] {
        semio_framework_schema_registry::register_artifact_schema_descriptor(semio_framework_schema_registry::ArtifactSchemaDescriptor {
            id,
            artifact: empty,
            snapshot: semio_framework_schema_registry::FacetLeaves { json_schema: source, ..empty },
            diff: empty,
            mutations: empty,
        })
        .expect("schema descriptor publication");
    }
    let value = semio_framework_pack_json::from_json_str(include_str!("../../🧫️fixtures/🔗️external-all-of/⬅️current/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("language-agnostic value fixture");
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
    assert_eq!(
        variants[0].value,
        semio_framework_pack_json::from_json_str::<DslValue>(include_str!("../../🧫️fixtures/🔗️external-all-of/➡️text-variant/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("language-agnostic target fixture")
    );
    let candidate = serde_json::json!({
        "schema":"test.details-envelope",
        "subset":serde_json::from_str::<serde_json::Value>(&snapshot_edit_source(&variants[0].value)).expect("variant JSON")
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
    let rendered = render_snapshot_details_provider_revisioned(&EnumValue, semio_framework_ui_contract::UiPublicationRevision(1), Locale::De, "test.enum#editor", &TreeWindows::unhosted()).expect("enum details");
    let select = find_key(&rendered, &format!("{ROOT_PATH_ID}-enum")).expect("enum select");
    let semio_framework_ui_contract::Component::Select(props) = &select.component else { panic!("string enum must use a select") };
    assert_eq!(props.value.as_str(), "review");
    assert_eq!(props.items.len(), 3);
    let binding = select.bindings.iter().find(|binding| binding.trigger == semio_framework_ui_contract::Trigger::Change).expect("enum change binding");
    let Some(UiValue::Map(arguments)) = &binding.args else { panic!("enum path arguments") };
    assert!(arguments.iter().any(|(key, value)| key.as_str() == "path" && matches!(value, UiValue::Text(value) if value.as_str().is_empty())));
    let event = snapshot_edit_event_from_action(SET_SNAPSHOT_VALUE_ACTION_ID, Some(&DslValue::Object(vec![("path".into(), DslValue::String(String::new())), ("value".into(), DslValue::String("published".into()))])))
        .expect("parse enum action")
        .expect("known action");
    assert_eq!(event, SnapshotEditEvent::SetValue { path: String::new(), value: DslValue::String("published".into()) });
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
    let event = snapshot_edit_event_from_action(SET_SNAPSHOT_VALUE_ACTION_ID, Some(&DslValue::Object(vec![(argument.into(), DslValue::Array(decoded)), ("value".into(), DslValue::String("after".into()))])))
        .expect("parse chunked control")
        .expect("known action");
    assert_eq!(event, SnapshotEditEvent::SetValue { path: path.clone(), value: DslValue::String("after".into()) });
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
            Some(snapshot_edit_source(&DslValue::Object(vec![(self.0.clone(), DslValue::String("before".into()))])))
        }
    }
    let provider = HugeKey("🚀".repeat(40_000));
    let path = pointer(&[SnapshotDetailPathSegment::Key(provider.0.clone())]);
    assert!(path.len() > semio_framework_ui_contract::UI_TEXT_MAX_BYTES * semio_framework_ui_contract::UI_VALUE_MAX_ITEMS);
    let rendered = render_snapshot_details_provider_revisioned(&provider, semio_framework_ui_contract::UiPublicationRevision(1), Locale::En, "test.huge-key#editor", &TreeWindows::unhosted()).expect("source fallback");
    let fallback = find_key(&rendered, &format!("{}-source-fallback", path_id(&[SnapshotDetailPathSegment::Key(provider.0.clone())]))).expect("editable source fallback");
    let surface = fallback.children.get(0).expect("draft surface");
    let semio_framework_ui_contract::Component::Surface(props) = &surface.component else { panic!("fallback must be a text draft") };
    let mut scene: semio_framework_ui_scene::TextEditorScene = semio_framework_ui_scene::decode(props).expect("draft scene");
    for carrier in &surface.children {
        let mut payload = String::new();
        let mut frontier = vec![carrier];
        while let Some(node) = frontier.pop() {
            if let semio_framework_ui_contract::Component::Text(text) = &node.component {
                payload.push_str(&text.packed_payload());
            }
            frontier.extend(node.children.iter().rev());
        }
        assert!(semio_framework_ui_scene::SceneDoc::merge_lane(&mut scene, carrier.key.as_str(), payload));
    }
    let settings: serde_json::Value = serde_json::from_str(scene.settings_json.as_deref().expect("draft settings")).expect("settings JSON");
    assert_eq!(settings["readOnly"], false);
    assert_eq!(settings["editAction"], REPLACE_SNAPSHOT_SOURCE_ACTION_ID);
    assert_eq!(settings["commit"], "explicit");
    assert!(scene.buffer.contains("before"));
}

#[test]
fn source_drafts_fill_tree_control_width_for_empty_and_nonempty_documents() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️first-paint/🔣️.json")).unwrap();
    let law = &fixture["sourceDraftLayout"];
    for locale in [Locale::En, Locale::De] {
        for source in law["texts"].as_array().unwrap() {
            let row = source_row(source.as_str().unwrap().into(), labels(locale), Some(ui_contract::UiPublicationRevision(1))).unwrap();
            let draft = find_key(&row, "stdio-snapshot-details-source-draft").unwrap();
            let layout = serde_json::to_value(&draft.layout).unwrap();
            for key in ["kind", "axis", "grow"] { assert_eq!(layout[key], law[key], "source draft {key}"); }
        }
    }
}

fn heavy_item(id: &str, fillers: usize) -> UiAssemblyResult<ui_contract::TreeItemBuilder> {
    let mut item = ui_contract::tree_item(label(id)?).try_id(id).map_err(|_| error("test.id"))?;
    for index in 0..fillers {
        let leaf = ui_contract::tree_item(label("x")?).try_id(&format!("{id}-{index}")).map_err(|_| error("test.leaf-id"))?.try_build().map_err(|_| error("test.leaf"))?;
        item = item.try_child(leaf).map_err(|_| error("test.child"))?;
    }
    Ok(item)
}

#[test]
fn a_nested_window_leaves_its_enclosing_row_its_own_price() {
    let windows = TreeWindows::unhosted();
    let section = tree_window_indexed_section(&windows, "root", label("Root").unwrap(), true, 2, |index| {
        if index == 0 {
            tree_window_indexed_item(&windows, heavy_item("parent", 20)?, "parent-children", true, 48, |child| heavy_item(&format!("child-{child}"), 20)?.try_build().map_err(|_| error("test.child-build")))
        } else {
            heavy_item("tail", 1)?.try_build().map_err(|_| error("test.tail-build"))
        }
    })
    .expect("nested windowed section");
    assert_eq!(section.children.len(), 2, "the enclosing row and the row after it must both be materialised (items left {})", windows.items_remaining());
}
