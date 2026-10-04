use super::*;
use kernel::{OpBinary, OpText};

#[derive(Clone, value_derive::ToValue, value_derive::FromValue)]
struct PublicationMutation {
    snapshot: DslValue,
    padding: usize,
    inverse_padding: usize,
    broken_inverse: bool,
}

#[derive(Clone, Default, value_derive::ToValue, value_derive::FromValue)]
struct PublicationDiff {
    snapshot: Option<DslValue>,
}

impl kernel::MutationDiff<DslValue> for PublicationDiff {
    fn apply(&self, base: &DslValue) -> Result<DslValue, kernel::MutationApplyError> {
        Ok(self.snapshot.clone().unwrap_or_else(|| base.clone()))
    }

    fn absorb(&mut self, other: Self) {
        if other.snapshot.is_some() {
            *self = other;
        }
    }
}

impl kernel::Mutation<DslValue> for PublicationMutation {
    type Diff = PublicationDiff;
    const DESCRIPTORS: &'static [kernel::MutationLeafDescriptor] = &[];

    fn descriptor(&self) -> &'static kernel::MutationLeafDescriptor {
        unreachable!("publication admission does not consult domain descriptors")
    }

    fn diff(&self, _base: &DslValue) -> kernel::MutationOutcome<Self::Diff> {
        kernel::MutationOutcome::new(PublicationDiff { snapshot: Some(self.snapshot.clone()) })
    }

    fn inverse(&self, base: &DslValue) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok((|| vec![Self { snapshot: if self.broken_inverse { self.snapshot.clone() } else { base.clone() }, padding: self.inverse_padding, inverse_padding: self.padding, broken_inverse: false }])())
    }
}

impl OpBinary for PublicationMutation {
    fn encode_op(&self) -> Result<Vec<u8>, kernel::ProtocolError> {
        let mut bytes = semio_framework_pack_json::to_json_string(&self.snapshot).into_bytes();
        bytes.resize(bytes.len() + self.padding, b' ');
        Ok(bytes)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, kernel::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|error| kernel::ProtocolError::Malformed { what: "publication fixture utf8", offset: 0, detail: error.to_string() })?;
        let snapshot =
            semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| kernel::ProtocolError::Malformed { what: "publication fixture value", offset: 0, detail: error.to_string() })?;
        Ok(Self { snapshot, padding: 0, inverse_padding: 0, broken_inverse: false })
    }
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("neutral compact snapshot fixture")
}

fn value(input: &serde_json::Value) -> DslValue {
    semio_framework_pack_json::from_json_str(&input.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}

#[test]
fn retained_publication_checks_exact_forward_inverse_and_requested_result() {
    let fixture = fixture();
    let rows = &fixture["publication"];
    let before = value(&rows["before"]);
    let expected = value(&rows["expected"]);
    let mut oracle = rows["before"].clone();
    json_patch::patch(&mut oracle, &serde_json::from_value::<json_patch::Patch>(rows["oracle"].clone()).unwrap()).unwrap();
    assert_eq!(oracle, rows["expected"]);
    assert_eq!(rows["maximumBytes"].as_u64().unwrap() as usize, kernel::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
    for row in rows["cases"].as_array().unwrap() {
        let emitted = match row["emitted"].as_str() {
            Some(key) => &rows[key],
            None => &row["emitted"],
        };
        let emitted_bytes = serde_json::to_vec(emitted).unwrap().len();
        let original_bytes = serde_json::to_vec(&rows["before"]).unwrap().len();
        let mutation = PublicationMutation {
            snapshot: value(emitted),
            padding: (row["forwardBytes"].as_u64().unwrap() as usize).saturating_sub(emitted_bytes),
            inverse_padding: (row["inverseBytes"].as_u64().unwrap() as usize).saturating_sub(original_bytes),
            broken_inverse: row["brokenInverse"].as_bool().unwrap_or(false),
        };
        assert_eq!(mutation.encode_op().unwrap().len(), emitted_bytes + mutation.padding);
        assert_eq!(PublicationMutation::decode_op(&mutation.encode_op().unwrap()).unwrap().snapshot, mutation.snapshot);
        let result = super::super::validate_snapshot_edit_publication(&before, &expected, &[mutation]);
        match row["error"].as_str() {
            Some(code) => assert_eq!(result.expect_err(row["id"].as_str().unwrap()).code.0, code),
            None => result.unwrap_or_else(|error| panic!("{}: {error:?}", row["id"])),
        }
        assert_eq!(before, value(&rows["before"]));
    }
}

#[test]
fn compact_snapshot_patches_match_independent_oracle_and_inverse() {
    let fixture = fixture();
    let base = value(&fixture["base"]);
    for row in fixture["cases"].as_array().unwrap() {
        let event: SnapshotEditEvent = semio_framework_pack_json::from_json_str(&row["event"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let patch = prepare_snapshot_patch(&base, &event).unwrap_or_else(|error| panic!("{}: {error}", row["id"]));
        let next = apply_snapshot_patch(&base, &patch).unwrap();
        let inverse = inverse_snapshot_patch(&base, &patch).unwrap();
        let mut expected = fixture["base"].clone();
        json_patch::patch(&mut expected, &serde_json::from_value::<json_patch::Patch>(row["oracle"].clone()).unwrap()).unwrap();
        assert_eq!(serde_json::Value::from(next.clone()), expected, "{}", row["id"]);
        assert_eq!(apply_snapshot_patch(&next, &inverse).unwrap(), base, "{} inverse", row["id"]);
        let id = row["id"].as_str().unwrap();
        assert_eq!(serde_json::Value::from(patch.to_value()), fixture["patches"][id]["patch"], "{id} canonical patch");
        assert_eq!(serde_json::Value::from(inverse.to_value()), fixture["patches"][id]["inverse"], "{id} exact inverse");
        assert_eq!(SnapshotPatch::decode_op(&patch.encode_op().unwrap()).unwrap(), patch);
        assert_eq!(SnapshotPatch::parse_op(&patch.print_op()).unwrap(), patch);
        let field = <SnapshotPatch as semio_framework_dsl_record::DslField>::to_value(&patch);
        assert_eq!(<SnapshotPatch as semio_framework_dsl_record::DslField>::from_value(&field).unwrap(), patch);
    }
}

#[test]
fn invalid_compact_snapshot_patches_preserve_original() {
    let fixture = fixture();
    let base = value(&fixture["base"]);
    for row in fixture["rejected"].as_array().unwrap() {
        let event: SnapshotEditEvent = semio_framework_pack_json::from_json_str(&row["event"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let result = prepare_snapshot_patch(&base, &event).and_then(|patch| apply_snapshot_patch(&base, &patch));
        assert!(result.is_err(), "{}", row["id"]);
        assert_eq!(base, value(&fixture["base"]));
    }
    for patch in [SnapshotPatch::Remove { path: "/missing".into() }, SnapshotPatch::Move { from: "/list/0".into(), path: "/missing/0".into(), index: None }, SnapshotPatch::Rename { path: "/list/0".into(), key: "first".into() }] {
        assert!(apply_snapshot_patch(&base, &patch).is_err(), "{patch:?}");
        assert_eq!(base, value(&fixture["base"]));
    }
}

#[test]
fn intrinsic_maps_restore_exact_values_without_requiring_authored_key_positions() {
    use std::collections::{BTreeMap, HashMap};
    let fixture = fixture();
    for row in fixture["intrinsicObjectOrder"].as_array().unwrap() {
        let before = value(&row["before"]);
        let patch = SnapshotPatch::Insert { path: format!("/{}", row["key"].as_str().unwrap()), value: DslValue::String("Inserted".into()), index: row["index"].as_u64() };
        let mut oracle = row["before"].clone();
        json_patch::patch(&mut oracle, &serde_json::from_value::<json_patch::Patch>(serde_json::json!([{ "op": "add", "path": format!("/{}", row["key"].as_str().unwrap()), "value": "Inserted" }])).unwrap()).unwrap();
        let ordered = apply_snapshot_patch(&before, &patch).unwrap();
        assert_eq!(serde_json::Value::from(ordered.clone()), oracle);
        assert_eq!(apply_snapshot_patch(&ordered, &inverse_snapshot_patch(&before, &patch).unwrap()).unwrap(), before);
        let tree = BTreeMap::<String, DslValue>::from_value(before.clone()).unwrap();
        let changed = apply_snapshot_patch(&tree, &patch).unwrap();
        assert_eq!(serde_json::Value::from(changed.to_value()), oracle);
        assert_eq!(apply_snapshot_patch(&changed, &inverse_snapshot_patch(&tree, &patch).unwrap()).unwrap(), tree);
        for _ in 0..32 {
            let hash = HashMap::<String, DslValue>::from_value(before.clone()).unwrap();
            let changed = apply_snapshot_patch(&hash, &patch).unwrap();
            assert_eq!(serde_json::Value::from(changed.to_value()), oracle);
            assert_eq!(apply_snapshot_patch(&changed, &inverse_snapshot_patch(&hash, &patch).unwrap()).unwrap(), hash);
        }
    }
    for row in fixture["intrinsicRenames"].as_array().unwrap() {
        let before = value(&row["before"]);
        let event = SnapshotEditEvent::RenameKey { path: format!("/{}", row["from"].as_str().unwrap()), key: row["key"].as_str().unwrap().into() };
        let mut oracle = row["before"].clone();
        json_patch::patch(&mut oracle, &serde_json::from_value::<json_patch::Patch>(serde_json::json!([{ "op": "move", "from": format!("/{}", row["from"].as_str().unwrap()), "path": format!("/{}", row["key"].as_str().unwrap()) }])).unwrap())
            .unwrap();
        let tree = BTreeMap::<String, DslValue>::from_value(before.clone()).unwrap();
        let patch = prepare_snapshot_patch(&tree, &event).unwrap();
        let changed = apply_snapshot_patch(&tree, &patch).unwrap();
        assert_eq!(serde_json::Value::from(changed.to_value()), oracle);
        assert_eq!(apply_snapshot_patch(&changed, &inverse_snapshot_patch(&tree, &patch).unwrap()).unwrap(), tree);
        for _ in 0..32 {
            let hash = HashMap::<String, DslValue>::from_value(before.clone()).unwrap();
            let patch = prepare_snapshot_patch(&hash, &event).unwrap();
            let changed = apply_snapshot_patch(&hash, &patch).unwrap();
            assert_eq!(serde_json::Value::from(changed.to_value()), oracle);
            assert_eq!(apply_snapshot_patch(&changed, &inverse_snapshot_patch(&hash, &patch).unwrap()).unwrap(), hash);
        }
    }
}

#[test]
fn positioned_insertions_preserve_nested_typed_object_order() {
    #[derive(Clone, value_derive::ToValue, value_derive::FromValue)]
    struct Document {
        metadata: DslValue,
    }
    let fixture = fixture();
    let before = Document { metadata: value(&fixture["base"]["metadata"]) };
    for row in fixture["positionedInsertions"].as_array().unwrap() {
        let key = row["key"].as_str().unwrap();
        let patch = SnapshotPatch::Insert { path: format!("/metadata/{key}"), value: DslValue::String("Inserted".into()), index: row["index"].as_u64() };
        let after = apply_snapshot_patch(&before, &patch).unwrap();
        let DslValue::Object(entries) = &after.metadata else { panic!("ordered metadata object") };
        assert_eq!(entries.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), row["keys"].as_array().unwrap().iter().map(|key| key.as_str().unwrap()).collect::<Vec<_>>());
        let mut oracle = fixture["base"]["metadata"].clone();
        json_patch::patch(&mut oracle, &serde_json::from_value::<json_patch::Patch>(serde_json::json!([{ "op": "add", "path": format!("/{key}"), "value": "Inserted" }])).unwrap()).unwrap();
        assert_eq!(serde_json::Value::from(after.metadata.clone()), oracle);
        assert_eq!(apply_snapshot_patch(&after, &inverse_snapshot_patch(&before, &patch).unwrap()).unwrap().metadata, before.metadata);
    }
    for index in fixture["invalidPositions"].as_array().unwrap() {
        let wire = serde_json::json!({ "operation": "insert", "path": "/metadata/bad", "value": "Rejected", "index": index }).to_string();
        let result = SnapshotPatch::parse_op(&wire).ok().and_then(|patch| apply_snapshot_patch(&before, &patch).ok());
        assert!(result.is_none());
    }
}

#[derive(Clone, Debug, value_derive::ToValue, value_derive::FromValue)]
struct LargeSnapshot {
    title: String,
    bytes: Vec<u8>,
}

#[test]
fn compact_metadata_publication_does_not_encode_large_siblings() {
    let fixture = fixture();
    let row = &fixture["large"];
    let base = LargeSnapshot { title: "Before".into(), bytes: vec![row["fill"].as_u64().unwrap() as u8; row["bytes"].as_u64().unwrap() as usize] };
    let patch = prepare_snapshot_patch(&base, &SnapshotEditEvent::SetValue { path: row["metadataPath"].as_str().unwrap().into(), value: value(&row["value"]) }).unwrap();
    let inverse = inverse_snapshot_patch(&base, &patch).unwrap();
    assert!(patch.encode_op().unwrap().len() < row["maximumPatchBytes"].as_u64().unwrap() as usize);
    assert!(inverse.encode_op().unwrap().len() < row["maximumPatchBytes"].as_u64().unwrap() as usize);
    let after = apply_snapshot_patch(&base, &patch).unwrap();
    assert_eq!(after.bytes, base.bytes);
    assert_eq!(after.title, row["value"].as_str().unwrap());
    assert_eq!(apply_snapshot_patch(&after, &inverse).unwrap().title, base.title);
    let oracle = serde_json::json!({ "title": row["value"] });
    assert_eq!(serde_json::Value::from(after.value_at_path(&["title"]).unwrap()), oracle["title"]);
}

#[test]
fn retained_admission_bounds_only_the_event_and_selected_paths() {
    let fixture = fixture();
    let large = &fixture["large"];
    let admission = &fixture["admission"];
    let base = LargeSnapshot { title: "Before".into(), bytes: vec![large["fill"].as_u64().unwrap() as u8; large["bytes"].as_u64().unwrap() as usize] };
    let event: SnapshotEditEvent = semio_framework_pack_json::from_json_str(&admission["event"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(super::super::snapshot_edit_value_is_admitted(&event, &base), admission["expected"].as_bool().unwrap());

    let maximum_nodes = admission["maximumNodes"].as_u64().unwrap() as usize;
    let at_node_limit = SnapshotEditEvent::SetValue { path: "/title".into(), value: DslValue::Array(vec![DslValue::Null; maximum_nodes - 1]) };
    assert!(super::super::snapshot_edit_value_is_admitted(&at_node_limit, &base));
    let over_node_limit = SnapshotEditEvent::SetValue { path: "/title".into(), value: DslValue::Array(vec![DslValue::Null; maximum_nodes]) };
    assert!(!super::super::snapshot_edit_value_is_admitted(&over_node_limit, &base));

    let maximum_depth = admission["maximumDepth"].as_u64().unwrap() as usize;
    let nested = |depth: usize| (0..depth).fold(DslValue::Null, |value, _| DslValue::Array(vec![value]));
    assert!(super::super::snapshot_edit_value_is_admitted(&SnapshotEditEvent::SetValue { path: "/title".into(), value: nested(maximum_depth) }, &base,));
    assert!(!super::super::snapshot_edit_value_is_admitted(&SnapshotEditEvent::SetValue { path: "/title".into(), value: nested(maximum_depth + 1) }, &base,));

    let maximum_segments = admission["maximumPointerSegments"].as_u64().unwrap() as usize;
    let over_segment_limit = format!("/{}", vec!["field"; maximum_segments + 1].join("/"));
    assert!(!super::super::snapshot_edit_value_is_admitted(&SnapshotEditEvent::SetValue { path: over_segment_limit, value: DslValue::Null }, &base,));
    assert!(!super::super::snapshot_edit_value_is_admitted(&SnapshotEditEvent::SetValue { path: "missing-leading-slash".into(), value: DslValue::Null }, &base,));
    assert!(!super::super::snapshot_edit_value_is_admitted(&SnapshotEditEvent::SetValue { path: "/title".into(), value: DslValue::Object(vec![("same".into(), DslValue::Null), ("same".into(), DslValue::Null)]) }, &base,));
    assert!(super::super::snapshot_edit_value_is_admitted(&SnapshotEditEvent::ReplaceSource { source: "{\"title\":\"a\",\"title\":\"b\"}".into() }, &base,));
    let maximum_raw_bytes = admission["maximumRawBytes"].as_u64().unwrap() as usize;
    assert!(!super::super::snapshot_edit_value_is_admitted(&SnapshotEditEvent::SetValue { path: "/title".into(), value: DslValue::String("x".repeat(maximum_raw_bytes)) }, &base,));
}

#[test]
fn native_large_array_structure_uses_length_without_projecting_items() {
    let maximum = 2_097_152usize;
    let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile(
        r#"{
            "type":"object",
            "required":["title","bytes"],
            "properties":{
                "title":{"type":"string"},
                "bytes":{"type":"array","minItems":1,"maxItems":2097152,"items":{"type":"integer","minimum":0,"maximum":255}}
            }
        }"#,
    )
    .unwrap();
    let insert = |snapshot: &LargeSnapshot| prepare_snapshot_patch(snapshot, &SnapshotEditEvent::InsertValue { path: "/bytes/-".into(), value: DslValue::Number(semio_framework_value::Number::UInt(255)) }).unwrap();
    let remove = |snapshot: &LargeSnapshot| prepare_snapshot_patch(snapshot, &SnapshotEditEvent::RemoveValue { path: "/bytes/0".into() }).unwrap();

    let boundary = LargeSnapshot { title: "Boundary".into(), bytes: vec![0; maximum - 1] };
    let inserted = apply_validated_snapshot_patch(&boundary, &insert(&boundary), &validator).unwrap();
    assert_eq!(inserted.bytes.len(), maximum);
    assert_eq!(inserted.bytes.last(), Some(&255));

    let overflow = apply_validated_snapshot_patch(&inserted, &insert(&inserted), &validator);
    assert!(overflow.is_err());
    assert_eq!(inserted.bytes.len(), maximum);

    let removed = apply_validated_snapshot_patch(&inserted, &remove(&inserted), &validator).unwrap();
    assert_eq!(removed.bytes.len(), maximum - 1);

    let minimum = LargeSnapshot { title: "Minimum".into(), bytes: vec![0] };
    assert!(apply_validated_snapshot_patch(&minimum, &remove(&minimum), &validator).is_err());
    assert_eq!(minimum.bytes, [0]);
}

#[test]
fn moved_values_resolve_the_destination_parent_after_removal() {
    let fixture = fixture();
    let row = &fixture["shiftedParent"];
    let base = value(&row["before"]);
    let event: SnapshotEditEvent = semio_framework_pack_json::from_json_str(&row["event"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let patch = prepare_snapshot_patch(&base, &event).unwrap();
    let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile(&row["schema"].to_string()).unwrap();
    let next = apply_validated_snapshot_patch(&base, &patch, &validator).unwrap();
    let mut expected = row["before"].clone();
    json_patch::patch(&mut expected, &serde_json::from_value::<json_patch::Patch>(row["oracle"].clone()).unwrap()).unwrap();
    assert_eq!(serde_json::Value::from(next.clone()), expected);
    assert_eq!(expected, row["expected"]);
    assert_eq!(apply_snapshot_patch(&next, &inverse_snapshot_patch(&base, &patch).unwrap()).unwrap(), base);
}

fn location_documents(fixture: &serde_json::Value) -> Vec<String> {
    fixture["locations"]["documents"].as_array().unwrap().iter().map(serde_json::Value::to_string).collect()
}

fn location_resolver(documents: &[String]) -> impl Fn(&str) -> Option<Arc<DslValue>> + '_ {
    move |id: &str| {
        documents
            .iter()
            .map(|text| semio_framework_pack_json::from_json_str::<DslValue>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap())
            .find(|document| document.get("$id").and_then(DslValue::as_str) == Some(id))
            .map(Arc::new)
    }
}

#[test]
fn snapshot_schema_locations_follow_properties_items_maps_unions_and_documents() {
    let fixture = fixture();
    let documents = location_documents(&fixture);
    let resolve = location_resolver(&documents);
    let snapshot = fixture["locations"]["snapshot"].as_str().unwrap();
    for row in fixture["locations"]["cases"].as_array().unwrap() {
        let segments = decode_pointer(row["path"].as_str().unwrap()).unwrap();
        let location = snapshot_schema_location(snapshot, &segments, &resolve).map(|location| location.reference());
        assert_eq!(location.as_deref(), row["expected"].as_str(), "{}", row["id"]);
    }
}

#[test]
fn a_patch_input_schema_types_the_value_by_the_snapshot_sub_schema_and_reads_as_inputs() {
    let fixture = fixture();
    let documents = location_documents(&fixture);
    let resolve = location_resolver(&documents);
    let resolve_value = |id: &str| resolve(id).map(|document| (*document).clone());
    let snapshot = fixture["locations"]["snapshot"].as_str().unwrap();
    for row in fixture["locations"]["cases"].as_array().unwrap().iter().filter(|row| row["expected"].is_string()) {
        let path = row["path"].as_str().unwrap().to_string();
        let insert = row["insert"].as_bool().unwrap_or(false);
        let patch = if insert { SnapshotPatch::Insert { path: path.clone(), value: value(&row["valid"]), index: None } } else { SnapshotPatch::Set { path: path.clone(), value: value(&row["valid"]) } };
        let text = snapshot_patch_input_schema_text(snapshot, &patch, &resolve).unwrap_or_else(|| panic!("{} has an input schema", row["id"]));
        let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile_with_documents(&text, &documents.iter().map(String::as_str).collect::<Vec<_>>()).unwrap();
        let candidate = |payload: &serde_json::Value| serde_json::json!({ "patch": { "operation": patch.operation(), "path": path, "value": payload } }).to_string();
        assert!(validator.validate_json(&candidate(&row["valid"])).is_ok(), "{} admits its valid value", row["id"]);
        assert!(validator.validate_json(&candidate(&row["invalid"])).is_err(), "{} refuses its invalid value", row["id"]);
        let inputs = semio_framework_plugin::mutation_input_defs(&text, &resolve_value).unwrap_or_else(|error| panic!("{}: {error}", row["id"]));
        assert_eq!(inputs.iter().map(|input| input.id.as_str()).collect::<Vec<_>>(), ["/patch"], "{}", row["id"]);
    }
    for (patch, inputs) in
        [(SnapshotPatch::Remove { path: "/title".into() }, 0usize), (SnapshotPatch::Rename { path: "/metadata/k".into(), key: "j".into() }, 1), (SnapshotPatch::Move { from: "/list/0".into(), path: "/list/1".into(), index: None }, 0)]
    {
        let text = snapshot_patch_input_schema_text(snapshot, &patch, &resolve).unwrap();
        let read = semio_framework_plugin::mutation_input_defs(&text, &resolve_value).unwrap();
        let semio_framework_plugin::ArgSchema::Object { fields } = &read[0].schema else { panic!("{patch:?} reads one patch object") };
        assert_eq!(fields.iter().filter(|field| field.presentation != Some(semio_framework_plugin::ArgPresentation::Hidden)).count(), inputs, "{patch:?}");
    }
}

#[test]
fn every_operation_round_trips_its_wire_and_labels_itself_in_every_locale() {
    let fixture = fixture();
    for (id, row) in fixture["patches"].as_object().unwrap() {
        for wire in [&row["patch"], &row["inverse"]] {
            let patch = SnapshotPatch::parse_op(&wire.to_string()).unwrap_or_else(|error| panic!("{id}: {error}"));
            assert_eq!(serde_json::Value::from(patch.to_value()), *wire, "{id}");
            assert_eq!(SnapshotPatch::decode_op(&patch.encode_op().unwrap()).unwrap(), patch, "{id}");
            let label = snapshot_patch_label(&patch);
            for locale in [semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Locale::De] {
                assert!(!label.resolve(semio_framework_ui_locale::Terminology::Native, locale).is_empty(), "{id}");
            }
        }
    }
}

/// 🔁️ The RFC 6902 operations one part performs on `document` (a splice as removals plus additions, a text splice as the
/// replaced text), so an independent applier replays the parts; `ordered` is the same document in member order, which names
/// the object members a positional splice removes.
fn rfc6902(document: &serde_json::Value, ordered: &DslValue, part: &SnapshotPatch) -> json_patch::Patch {
    let operations = match part {
        SnapshotPatch::Set { path, value } => vec![serde_json::json!({ "op": "replace", "path": path, "value": serde_json::Value::from(value.clone()) })],
        SnapshotPatch::Insert { path, value, .. } => vec![serde_json::json!({ "op": "add", "path": path, "value": serde_json::Value::from(value.clone()) })],
        SnapshotPatch::Splice { path, offset, remove, value, .. } => {
            let (offset, remove) = (*offset as usize, *remove as usize);
            match (document.pointer(path).expect("splice container"), serde_json::Value::from(value.clone())) {
                (serde_json::Value::Array(_), serde_json::Value::Array(items)) => std::iter::repeat_n(serde_json::json!({ "op": "remove", "path": format!("{path}/{offset}") }), remove)
                    .chain(items.into_iter().enumerate().map(|(position, item)| serde_json::json!({ "op": "add", "path": format!("{path}/{}", offset + position), "value": item })))
                    .collect(),
                (serde_json::Value::String(text), serde_json::Value::String(inserted)) => {
                    vec![serde_json::json!({ "op": "replace", "path": path, "value": format!("{}{inserted}{}", &text[..offset], &text[offset + remove..]) })]
                }
                (serde_json::Value::Object(_), serde_json::Value::Object(members)) => {
                    let segments = decode_pointer(path).unwrap();
                    let keys = ordered.value_at_path(&segments.iter().map(String::as_str).collect::<Vec<_>>()).ok().and_then(|value| if let DslValue::Object(members) = value { Some(members.into_iter().map(|(key, _)| key).collect::<Vec<_>>()) } else { None }).unwrap_or_default();
                    let removed = (offset..offset + remove).map(|position| serde_json::json!({ "op": "remove", "path": format!("{path}/{}", keys[position].replace('~', "~0").replace('/', "~1")) }));
                    removed.chain(members.into_iter().map(|(key, member)| serde_json::json!({ "op": "add", "path": format!("{path}/{}", key.replace('~', "~0").replace('/', "~1")), "value": member }))).collect()
                }
                other => panic!("unsupported splice oracle {other:?}"),
            }
        }
        other => panic!("pointer-only part {other:?}"),
    };
    serde_json::from_value(serde_json::Value::Array(operations)).unwrap()
}

#[test]
fn chunked_inverses_match_the_typescript_twin_and_replay_through_an_independent_rfc6902_applier() {
    let fixture = fixture();
    let ordered: DslValue = semio_framework_pack_json::from_json_str(include_str!("../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let block = &fixture["chunkedInverses"];
    let budget = block["budget"].as_u64().unwrap() as usize;
    let Some(DslValue::Array(cases)) = ordered.get("chunkedInverses").and_then(|block| block.get("cases")) else { panic!("ordered chunkedInverses cases") };
    for (row, case) in block["cases"].as_array().unwrap().iter().zip(cases) {
        let id = row["id"].as_str().unwrap();
        let base = case.get("base").unwrap().clone();
        let patch = SnapshotPatch::parse_op(&row["patch"].to_string()).unwrap();
        let parts = inverse_snapshot_patches_within(&base, &patch, budget).unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_eq!(DslValue::Array(parts.iter().map(ToValue::to_value).collect()), *case.get("parts").unwrap(), "{id} parts in member order");
        assert!(parts.iter().all(|part| part.encode_op().unwrap().len() <= budget), "{id} budget");
        assert!(parts[..parts.len() - 1].iter().all(|part| !matches!(part, SnapshotPatch::Splice { continued: false, .. })) && !parts.last().unwrap().continued(), "{id} continued");
        let after = apply_snapshot_patch(&base, &patch).unwrap();
        let restored = parts.iter().try_fold(after.clone(), |snapshot, part| apply_snapshot_patch(&snapshot, part)).unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_eq!(restored, base, "{id} exact restore");
        let (mut oracle, mut ordered) = (serde_json::Value::from(after.clone()), after);
        for part in &parts {
            let operations = rfc6902(&oracle, &ordered, part);
            json_patch::patch(&mut oracle, &operations).unwrap_or_else(|error| panic!("{id}: {error}"));
            ordered = apply_snapshot_patch(&ordered, part).unwrap();
            assert_eq!(oracle, serde_json::Value::from(ordered.clone()), "{id} part agrees with the independent applier");
        }
        assert_eq!(oracle, row["base"], "{id} independent replay");
        for part in &parts {
            assert_eq!(SnapshotPatch::parse_op(&part.print_op()).unwrap(), *part, "{id} wire");
        }
    }
}

#[test]
fn an_inverse_needing_more_parts_than_its_bound_is_refused_not_truncated() {
    let words = DslValue::Array((0..4_000).map(|position| DslValue::String(format!("word-{position:05}"))).collect());
    let base = DslValue::Object(vec![("words".into(), words)]);
    let patch = SnapshotPatch::Set { path: "/words".into(), value: DslValue::Array(Vec::new()) };
    let error = inverse_snapshot_patches_within(&base, &patch, 160).unwrap_err();
    assert_eq!(error.code, "snapshot-edit.inverse-limit");
    assert_eq!(error.outcome_code(), kernel::OutcomeCode::TargetMismatch);
    assert!((5..=6).contains(&inverse_snapshot_patches_within(&base, &patch, 16_384).unwrap().len()));
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
struct BlobRow {
    id: u64,
    label: String,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
struct BlobSnapshot {
    title: String,
    body: String,
    octets: DslValue,
    rows: Vec<BlobRow>,
}

fn blob(body: String, octets: usize, rows: usize) -> BlobSnapshot {
    BlobSnapshot { title: "Blob".into(), body, octets: DslValue::Bytes((0..octets).map(|position| (position % 251) as u8).collect()), rows: (0..rows as u64).map(|id| BlobRow { id, label: format!("row {id:08} Grüße") }).collect() }
}

fn assert_exact_undo(base: &BlobSnapshot, patch: &SnapshotPatch) -> usize {
    let after = apply_snapshot_patch(base, patch).unwrap();
    let parts = inverse_snapshot_patches(base, patch).unwrap();
    assert!(parts.len() <= SNAPSHOT_PATCH_MAX_INVERSE_PARTS);
    assert!(parts.iter().all(|part| part.encode_op().unwrap().len() <= SNAPSHOT_PATCH_MAX_BYTES));
    let restored = parts.iter().try_fold(after, |snapshot, part| apply_snapshot_patch_checked(&snapshot, part, |_| Ok::<(), std::convert::Infallible>(()))).unwrap();
    assert!(restored == *base, "the parts restore the publication base exactly");
    parts.len()
}

#[test]
fn an_inverse_at_the_patch_budget_plus_or_minus_one_byte_restores_exactly() {
    let envelope = SnapshotPatch::Set { path: "/body".into(), value: DslValue::String(String::new()) }.encode_op().unwrap().len();
    let fitting = SNAPSHOT_PATCH_MAX_BYTES - envelope;
    for (length, single) in [(fitting - 1, true), (fitting, true), (fitting + 1, false)] {
        let base = blob("b".repeat(length), 16, 4);
        let patch = SnapshotPatch::Set { path: "/body".into(), value: DslValue::String("x".into()) };
        assert_eq!(inverse_snapshot_patch(&base, &patch).is_ok(), single, "{length}");
        let parts = assert_exact_undo(&base, &patch);
        assert_eq!(parts == 1, single, "{length} bytes of prior text take {parts} parts");
    }
}

#[test]
fn sixteen_mebibyte_prior_values_undo_exactly_within_the_part_bound() {
    const SIXTEEN_MIB: usize = 16 * 1_024 * 1_024;
    let text = blob("Grüße 🌍 \"q\"\n".repeat(SIXTEEN_MIB / 16), 0, 0);
    assert!(text.body.len() >= SIXTEEN_MIB);
    let parts = assert_exact_undo(&text, &SnapshotPatch::Set { path: "/body".into(), value: DslValue::String(String::new()) });
    assert!(parts > 16, "{parts}");
    let octets = blob(String::new(), SIXTEEN_MIB, 0);
    let parts = assert_exact_undo(&octets, &SnapshotPatch::Set { path: "/octets".into(), value: DslValue::Bytes(Vec::new()) });
    assert!(parts > 32, "{parts}");
    let rows = blob(String::new(), 0, SIXTEEN_MIB / 40);
    assert!(rows.to_value().get("rows").map(json_len).unwrap() >= SIXTEEN_MIB);
    let parts = assert_exact_undo(&rows, &SnapshotPatch::Set { path: "/rows".into(), value: DslValue::Array(Vec::new()) });
    assert!(parts > 16, "{parts}");
}

#[test]
fn canonical_lengths_equal_the_encoded_part_lengths() {
    let fixture = fixture();
    for row in fixture["chunkedInverses"]["cases"].as_array().unwrap() {
        for part in row["parts"].as_array().unwrap() {
            let patch = SnapshotPatch::parse_op(&part.to_string()).unwrap();
            let (SnapshotPatch::Splice { value, .. } | SnapshotPatch::Set { value, .. }) = &patch else { panic!("value part") };
            assert_eq!(json_len(value), semio_framework_pack_json::to_json_string(value).len(), "{}", row["id"]);
        }
    }
    for value in [DslValue::Bytes(vec![0, 9, 10, 99, 100, 255]), DslValue::float(1.5), DslValue::int(-42), DslValue::String("\u{1}\u{1f}\"\\\t é".into())] {
        assert_eq!(json_len(&value), semio_framework_pack_json::to_json_string(&value).len(), "{value:?}");
    }
}

#[test]
fn a_continued_part_defers_the_whole_snapshot_invariant_to_the_run_end() {
    let base = DslValue::Object(vec![("rows".into(), DslValue::Array(vec![DslValue::uint(1)]))]);
    let refuse = |snapshot: &DslValue| if snapshot.get("rows").and_then(|rows| if let DslValue::Array(items) = rows { Some(items.len()) } else { None }) == Some(2) { Ok(()) } else { Err("rows must hold two items") };
    let part = |offset, continued| SnapshotPatch::Splice { path: "/rows".into(), offset, remove: 0, value: DslValue::Array(vec![DslValue::uint(2)]), continued };
    assert!(apply_snapshot_patch_checked(&base, &part(1, false), refuse).is_ok());
    let empty = DslValue::Object(vec![("rows".into(), DslValue::Array(Vec::new()))]);
    assert!(apply_snapshot_patch_checked(&empty, &part(0, false), refuse).is_err());
    assert!(apply_snapshot_patch_checked(&empty, &part(0, true), refuse).is_ok());
}
