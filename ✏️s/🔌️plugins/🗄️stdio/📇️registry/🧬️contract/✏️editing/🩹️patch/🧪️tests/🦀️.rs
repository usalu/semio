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

    fn inverse(&self, base: &DslValue) -> Vec<Self> {
        vec![Self { snapshot: if self.broken_inverse { self.snapshot.clone() } else { base.clone() }, padding: self.inverse_padding, inverse_padding: self.padding, broken_inverse: false }]
    }
}

impl OpBinary for PublicationMutation {
    fn encode_op(&self) -> Result<Vec<u8>, kernel::ProtocolError> {
        let mut bytes = pack::json::to_json_string(&self.snapshot).into_bytes();
        bytes.resize(bytes.len() + self.padding, b' ');
        Ok(bytes)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, kernel::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|error| kernel::ProtocolError::Malformed { what: "publication fixture utf8", offset: 0, detail: error.to_string() })?;
        let snapshot = pack::json::from_json_str(line).map_err(|error| kernel::ProtocolError::Malformed { what: "publication fixture value", offset: 0, detail: error.to_string() })?;
        Ok(Self { snapshot, padding: 0, inverse_padding: 0, broken_inverse: false })
    }
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("neutral compact snapshot fixture")
}

fn value(input: &serde_json::Value) -> DslValue {
    pack::json::from_json_str(&input.to_string()).unwrap()
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
        let event: SnapshotEditEvent = pack::json::from_json_str(&row["event"].to_string()).unwrap();
        let patch = prepare_snapshot_patch(&base, &event).unwrap_or_else(|error| panic!("{}: {error}", row["id"]));
        let next = apply_snapshot_patch(&base, &patch).unwrap();
        let inverse = inverse_snapshot_patch(&base, &patch).unwrap();
        let mut expected = fixture["base"].clone();
        json_patch::patch(&mut expected, &serde_json::from_value::<json_patch::Patch>(row["oracle"].clone()).unwrap()).unwrap();
        assert_eq!(serde_json::Value::from(next.clone()), expected, "{}", row["id"]);
        assert_eq!(apply_snapshot_patch(&next, &inverse).unwrap(), base, "{} inverse", row["id"]);
        assert_eq!(SnapshotPatch::decode_op(&patch.encode_op().unwrap()).unwrap(), patch);
        assert_eq!(SnapshotPatch::parse_op(&patch.print_op()).unwrap(), patch);
        let field = <SnapshotPatch as kernel::os_dsl::DslField>::to_value(&patch);
        assert_eq!(<SnapshotPatch as kernel::os_dsl::DslField>::from_value(&field).unwrap(), patch);
    }
}

#[test]
fn invalid_compact_snapshot_patches_preserve_original() {
    let fixture = fixture();
    let base = value(&fixture["base"]);
    for row in fixture["rejected"].as_array().unwrap() {
        let event: SnapshotEditEvent = pack::json::from_json_str(&row["event"].to_string()).unwrap();
        let result = prepare_snapshot_patch(&base, &event).and_then(|patch| apply_snapshot_patch(&base, &patch));
        assert!(result.is_err(), "{}", row["id"]);
        assert_eq!(base, value(&fixture["base"]));
    }
    let patch = SnapshotPatch {
        edits: vec![SnapshotValuePatch { path: vec!["title".into()], edit: SnapshotPatchEdit::Set { value: DslValue::String("Changed".into()) } }, SnapshotValuePatch { path: vec!["missing".into()], edit: SnapshotPatchEdit::Remove }],
    };
    assert!(apply_snapshot_patch(&base, &patch).is_err());
    assert_eq!(base, value(&fixture["base"]));
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
    let event: SnapshotEditEvent = pack::json::from_json_str(&admission["event"].to_string()).unwrap();
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
    assert!(!super::super::snapshot_edit_value_is_admitted(&SnapshotEditEvent::ReplaceSource { source: "{\"title\":\"a\",\"title\":\"b\"}".into() }, &base,));
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
    let insert = |snapshot: &LargeSnapshot| prepare_snapshot_patch(snapshot, &SnapshotEditEvent::InsertValue { path: "/bytes/-".into(), value: DslValue::Number(kernel::Number::UInt(255)) }).unwrap();
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
    let event: SnapshotEditEvent = pack::json::from_json_str(&row["event"].to_string()).unwrap();
    let patch = prepare_snapshot_patch(&base, &event).unwrap();
    let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile(&row["schema"].to_string()).unwrap();
    let next = apply_validated_snapshot_patch(&base, &patch, &validator).unwrap();
    let mut expected = row["before"].clone();
    json_patch::patch(&mut expected, &serde_json::from_value::<json_patch::Patch>(row["oracle"].clone()).unwrap()).unwrap();
    assert_eq!(serde_json::Value::from(next.clone()), expected);
    assert_eq!(expected, row["expected"]);
    assert_eq!(apply_snapshot_patch(&next, &inverse_snapshot_patch(&base, &patch).unwrap()).unwrap(), base);
}
