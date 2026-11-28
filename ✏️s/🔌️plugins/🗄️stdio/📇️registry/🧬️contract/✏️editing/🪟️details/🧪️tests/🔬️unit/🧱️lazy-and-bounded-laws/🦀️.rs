/// 🧱️ Bounded leaf, lazy native snapshot and lazy collection details laws.

#[test]
fn intrinsic_bytes_details_use_a_bounded_leaf_and_validate_the_json_octet_domain() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️bytes/🧪️tests/🧬️base64/🧫️fixtures/🔣️.json")).unwrap();
    let schema = semio_framework_pack_json::from_json_str::<DslValue>(r#"{"type":"array","items":{"type":"integer","minimum":0,"maximum":255}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let wrong = semio_framework_pack_json::from_json_str::<DslValue>(r#"{"type":"array","items":{"type":"string"}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let bytes: Vec<u8> = serde_json::from_value(row["octets"].clone()).unwrap();
        let value = DslValue::Bytes(bytes.clone());
        let provider = DslSnapshotDetailsProvider::<NativeLazySnapshot>::from_value(DslValue::Object(vec![("octets".into(), value.clone())]));
        let path = [SnapshotDetailPathSegment::Key("octets".into())];
        assert_eq!(provider.value(&path), Some(SnapshotDetailValue::Bytes { len: bytes.len() }));
        assert_eq!(provider.child_count(&path), 0);
        assert!(template_candidate_is_valid(&schema, &schema, &value, 0));
        assert_eq!(template_candidate_is_valid(&wrong, &wrong, &value, 0), bytes.is_empty());
    }
}

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
    assert_eq!(definition.actions.len(), SNAPSHOT_EDIT_ACTION_IDS.len());
    assert_ne!(definition.label.resolve(semio_ui_locale::Terminology::Native, Locale::En), "");
    assert_ne!(definition.label.resolve(semio_ui_locale::Terminology::Native, Locale::De), "");
}

struct NativeLazySnapshot;

impl ToValue for NativeLazySnapshot {
    fn to_value(&self) -> DslValue {
        panic!("details must not materialize a native snapshot")
    }

    fn value_at_path(&self, path: &[&str]) -> Result<DslValue, semio_framework_value::ValueError> {
        match path {
            ["schema"] => Ok(DslValue::String("stdio.lazy-details-test".into())),
            ["title"] => Ok(DslValue::String("Projected title".into())),
            _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "container reads must use shape/key access")),
        }
    }

    fn value_shape_at_path(&self, path: &[&str]) -> Result<ValueShape, semio_framework_value::ValueError> {
        match path {
            [] => Ok(ValueShape::Object { len: 2 }),
            ["schema"] | ["title"] => Ok(ValueShape::String),
            _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unknown test path")),
        }
    }

    fn value_key_at_path(&self, path: &[&str], index: usize) -> Result<String, semio_framework_value::ValueError> {
        match (path, index) {
            ([], 0) => Ok("schema".into()),
            ([], 1) => Ok("title".into()),
            _ => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unknown test key")),
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
    let _arena = ui_arena();
    let provider = LazyPixels { count: 16_000_000 };
    let result = render_snapshot_details_provider_revisioned(&provider, semio_ui_contract::UiPublicationRevision(1), Locale::En, "s.stdio.png@test/*#editor", &TreeWindows::unhosted());
    assert!(result.is_ok());
}

#[test]
fn lazy_large_object_controls_do_not_scan_unpaged_keys() {
    let _arena = ui_arena();
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
    render_snapshot_details_provider_revisioned(&provider, semio_ui_contract::UiPublicationRevision(1), Locale::En, "test.lazy-object#editor", &TreeWindows::unhosted()).expect("bounded large object");
    assert!(provider.key_reads.get() < 1_024, "visible paging must bound key reads, got {}", provider.key_reads.get());
}

#[test]
fn an_exact_chunk_budget_parent_omits_unbindable_derived_insert_controls_without_failing_the_panel() {
    let _arena = ui_arena();
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
    let rendered = render_snapshot_details_provider_revisioned(&provider, semio_ui_contract::UiPublicationRevision(1), Locale::En, "test.boundary-path#editor", &TreeWindows::unhosted()).expect("derived path is safely omitted");
    let id = path_id(&[SnapshotDetailPathSegment::Key(provider.key.clone())]);
    assert!(!contains_key(&rendered, &format!("{id}-add-0")));
    assert!(contains_key(&rendered, SOURCE_ID));
}

fn contains_key(node: &BuiltNode, key: &str) -> bool {
    node.key.as_str() == key || node.children.iter().any(|child| contains_key(child, key))
}

fn find_key<'a>(node: &'a BuiltNode, key: &str) -> Option<&'a BuiltNode> {
    (node.key.as_str() == key).then_some(node).or_else(|| node.children.iter().find_map(|child| find_key(child, key)))
}

/// 🗃️ Builds an XLSX-shaped snapshot: an `opc` object beside an `xmlParts` array of parts holding recursive XML element trees.
fn xml_parts_value(parts: usize, elements: usize) -> DslValue {
    let element = |index: usize| format!(r#"{{"kind":"element","name":"c","attributes":[{{"name":"r","value":"A{index}"}},{{"name":"s","value":"1"}}],"children":[{{"kind":"text","value":"{index}"}}]}}"#);
    let children = (0..elements).map(element).collect::<Vec<_>>().join(",");
    let part = |index: usize| format!(r#"{{"path":"xl/worksheets/sheet{index}.xml","document":{{"root":{{"kind":"element","name":"worksheet","attributes":[],"children":[{children}]}}}}}}"#);
    let parts = (0..parts).map(part).collect::<Vec<_>>().join(",");
    let json = format!(r#"{{"schema":"test","opc":{{"contentTypes":[{{"extension":"xml"}}],"relationships":[{{"id":"rId1"}}]}},"xmlParts":[{parts}]}}"#);
    semio_framework_pack_json::from_json_str::<DslValue>(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}

#[test]
fn xml_parts_collection_renders_every_part_row_and_opens_each_node_tree_on_request() {
    let _arena = ui_arena();
    let part_path = |index: usize| [SnapshotDetailPathSegment::Key("xmlParts".into()), SnapshotDetailPathSegment::Index(index)];
    for (parts, elements) in [(3usize, 4usize), (8, 300)] {
        let provider = DslSnapshotDetailsProvider::<NativeLazySnapshot>::from_value(xml_parts_value(parts, elements));
        let render = |windows: &TreeWindows<'_>| render_snapshot_details_provider_revisioned(&provider, semio_ui_contract::UiPublicationRevision(1), Locale::En, "test.xml-parts#editor", windows).expect("xml parts details");
        let rendered = render(&TreeWindows::unhosted());
        let collection = find_key(&rendered, &path_id(&part_path(0)[..1])).expect("xmlParts item");
        let semio_ui_contract::Component::TreeItem(props) = &collection.component else { panic!("xmlParts tree item") };
        assert_eq!(props.window.as_ref().map(|window| window.total), Some(parts as u32), "the window spans every part");
        assert!(collection.children.iter().filter(|child| child.key.as_str() != path_id(&part_path(0)[..1]) && !child.key.as_str().ends_with("-controls") && !child.key.as_str().ends_with("-collection-controls")).count() >= 2, "first paint materialises part rows");
        assert!(find_key(&rendered, &path_id(&part_path(0))).is_some() && find_key(&rendered, &path_id(&part_path(1))).is_some());
        let last = format!("{}-children", path_id(&part_path(0)[..1]));
        let last_view = ViewModel { tree_windows: vec![TreeWindowRequest { body_key: SNAPSHOT_DETAILS_BODY_KEY.into(), node_key: [ROOT_SECTION_ID.to_string(), last].join(TREE_WINDOW_PATH_SEPARATOR), open: Some(true), offset: parts as u32 - 1, rows: 1 }], ..ViewModel::new(Locale::En, semio_ui_locale::Terminology::Native) };
        assert!(find_key(&render(&TreeWindows::for_body(&last_view, SNAPSHOT_DETAILS_BODY_KEY)), &path_id(&part_path(parts - 1))).is_some(), "the last part is reachable by window offset");
        let parts_children = format!("{}-children", path_id(&[SnapshotDetailPathSegment::Key("xmlParts".into())]));
        let node_key = [ROOT_SECTION_ID.to_string(), parts_children, format!("{}-children", path_id(&part_path(0)))].join(TREE_WINDOW_PATH_SEPARATOR);
        let view = ViewModel { tree_windows: vec![TreeWindowRequest { body_key: SNAPSHOT_DETAILS_BODY_KEY.into(), node_key, open: Some(true), offset: 0, rows: 2 }], ..ViewModel::new(Locale::En, semio_ui_locale::Terminology::Native) };
        let opened = render(&TreeWindows::for_body(&view, SNAPSHOT_DETAILS_BODY_KEY));
        let path = [part_path(0)[0].clone(), part_path(0)[1].clone(), SnapshotDetailPathSegment::Key("path".into())];
        let document = [part_path(0)[0].clone(), part_path(0)[1].clone(), SnapshotDetailPathSegment::Key("document".into())];
        assert!(find_key(&opened, &path_id(&path)).is_some() && find_key(&opened, &path_id(&document)).is_some(), "parts={parts} elements={elements}: opened part must expose path and document");
    }
}
