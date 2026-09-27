use super::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
struct NativeCommand(u8);

impl kernel::OpBinary for NativeCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &["native"];
    fn encode_op(&self) -> Result<Vec<u8>, kernel::ProtocolError> { Ok(vec![self.0]) }
    fn decode_op(bytes: &[u8]) -> Result<Self, kernel::ProtocolError> {
        bytes.first().copied().map(Self).ok_or_else(|| kernel::ProtocolError::Malformed { what: "native fixture", offset: 0, detail: "empty".into() })
    }
}

crate::snapshot_editing_command_roster!(NativeCommand, ["native"]);

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(deny_unknown_fields)]
struct FixtureSnapshot {
    schema: String,
    title: String,
    active: bool,
    count: u64,
    ratio: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    optional: Option<String>,
    choice: FixtureChoice,
    labels: BTreeMap<String, String>,
    items: Vec<FixtureItem>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
enum FixtureChoice {
    First { level: u64 },
    Second { name: String },
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(deny_unknown_fields)]
struct FixtureItem {
    id: String,
    enabled: bool,
}

impl ArtifactDsl for FixtureSnapshot {
    const EXTENSION: &'static str = "json";

    fn parse_dsl(text: &str) -> Result<Self, kernel::TextError> {
        Err(kernel::TextError::new(format!("native source omits details: {text}"), kernel::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        self.title.clone()
    }
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪆️snapshot-edits/🔣️patch-cases.json")).expect("language-neutral snapshot edit fixture")
}

fn snapshot(value: &serde_json::Value) -> FixtureSnapshot {
    pack::json::from_json_str(&value.to_string()).expect("typed fixture snapshot")
}

fn event(value: &serde_json::Value) -> SnapshotEditEvent {
    pack::json::from_json_str(&value.to_string()).expect("typed fixture event")
}

#[test]
fn accepted_edits_match_json_patch_oracle() {
    let fixture = fixture();
    let base = snapshot(&fixture["base"]);
    for case in fixture["accepted"].as_array().expect("accepted cases") {
        let mut oracle = fixture["base"].clone();
        let patch: json_patch::Patch = serde_json::from_value(case["patch"].clone()).expect("JSON Patch oracle input");
        json_patch::patch(&mut oracle, &patch).expect("JSON Patch oracle accepts case");
        let actual = apply_snapshot_edit(&base, &event(&case["event"])).unwrap_or_else(|error| panic!("{}: {error:?}", case["id"]));
        assert_eq!(serde_json::Value::from(actual.to_value()), oracle, "{}", case["id"]);
    }
}

#[test]
fn rejected_edits_preserve_the_typed_document() {
    let fixture = fixture();
    let base = snapshot(&fixture["base"]);
    let before = base.to_value();
    for case in fixture["rejected"].as_array().expect("rejected cases") {
        let error = apply_snapshot_edit(&base, &event(&case["event"])).expect_err(case["id"].as_str().expect("case id"));
        assert_eq!(error.code, case["code"].as_str().expect("error code"), "{}", case["id"]);
        assert_eq!(base.to_value(), before, "{}", case["id"]);
    }
}

#[test]
fn ambiguous_and_non_finite_values_are_rejected_without_mutation() {
    let fixture = fixture();
    let base = snapshot(&fixture["base"]);
    let duplicate = SnapshotEditEvent::SetValue {
        path: "/labels".into(),
        value: DslValue::Object(vec![("same".into(), DslValue::String("a".into())), ("same".into(), DslValue::String("b".into()))]),
    };
    assert_eq!(apply_snapshot_edit(&base, &duplicate).expect_err("duplicate key").code, "snapshot-edit.ambiguous-object");
    let non_finite = SnapshotEditEvent::SetValue { path: "/ratio".into(), value: DslValue::float(f64::INFINITY) };
    assert_eq!(apply_snapshot_edit(&base, &non_finite).expect_err("non-finite number").code, "snapshot-edit.non-finite-number");
}

#[test]
fn schema_constraints_reject_an_addressed_value_atomically() {
    let fixture = fixture();
    let base = snapshot(&fixture["base"]);
    let cases = &fixture["constraintCases"];
    let schema = cases["schema"].to_string();
    let accepted = apply_snapshot_edit_with_schema(&base, &event(&cases["accepted"]), &schema).expect("boundary value");
    assert_eq!(accepted.count, 4);
    let error = apply_snapshot_edit_with_schema(&base, &event(&cases["rejected"]), &schema).expect_err("maximum");
    assert_eq!(error.code, cases["errorCode"].as_str().expect("error code"));
    assert_eq!(error.path, cases["errorPath"].as_str().expect("error path"));
    assert!(error.message.contains("maximum"));
    assert_eq!(base.count, 2);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&snapshot_edit_source(&base)).expect("third-party JSON oracle")["count"], 2);
}

#[test]
fn integral_json_numbers_edit_float_fields_without_losing_precision() {
    let fixture = fixture();
    let base = snapshot(&fixture["base"]);
    for row in fixture["floatInputs"].as_array().expect("numeric control cases") {
        let source = row["source"].as_str().unwrap();
        let value: DslValue = pack::json::from_json_str(source).unwrap();
        let result = apply_snapshot_edit(&base, &SnapshotEditEvent::SetValue { path: "/ratio".into(), value });
        if row["accepted"].as_bool().unwrap() {
            let oracle: f64 = serde_json::from_str(source).unwrap();
            assert_eq!(result.unwrap_or_else(|error| panic!("{source}: {error}")).ratio, oracle);
        } else {
            assert_eq!(result.expect_err("rounded integer must not reach a float field").code, "snapshot-edit.lossy-conversion");
        }
    }
}

#[test]
fn action_json_and_operation_codecs_preserve_typed_values() {
    let args = DslValue::object([
        ("path".to_string(), DslValue::String("/choice".into())),
        ("value".to_string(), DslValue::String(r#"{"kind":"second","name":"chosen"}"#.into())),
        ("valueEncoding".to_string(), DslValue::String("json".into())),
    ]);
    let parsed = snapshot_edit_event_from_action(SET_SNAPSHOT_VALUE_ACTION_ID, Some(&args)).expect("valid action").expect("known action");
    let text = <SnapshotEditEvent as kernel::OpText>::print_op(&parsed);
    assert_eq!(<SnapshotEditEvent as kernel::OpText>::parse_op(&text).expect("text round trip"), parsed);
    let bytes = <SnapshotEditEvent as kernel::OpBinary>::encode_op(&parsed).expect("binary encode");
    assert_eq!(<SnapshotEditEvent as kernel::OpBinary>::decode_op(&bytes).expect("binary round trip"), parsed);
    let source_args = DslValue::object([("value".to_string(), DslValue::String("invalid local draft".into()))]);
    assert_eq!(
        snapshot_edit_event_from_action(REPLACE_SNAPSHOT_SOURCE_ACTION_ID, Some(&source_args)).expect("source input").expect("known source action"),
        SnapshotEditEvent::ReplaceSource { source: "invalid local draft".into() }
    );
    let encoded_args = DslValue::object([
        ("path".to_string(), DslValue::String("/choice".into())),
        ("value".to_string(), DslValue::String(r#"{"kind":"first","level":7}"#.into())),
        ("valueEncoding".to_string(), DslValue::String("json".into())),
    ]);
    assert!(matches!(snapshot_edit_event_from_action(SET_SNAPSHOT_VALUE_ACTION_ID, Some(&encoded_args)).expect("encoded control").expect("known action"), SnapshotEditEvent::SetValue { value: DslValue::Object(_), .. }));
    let rename_args = DslValue::object([("path".to_string(), DslValue::String("/labels/z".into())), ("value".to_string(), DslValue::String("renamed".into()))]);
    assert_eq!(
        snapshot_edit_event_from_action(RENAME_SNAPSHOT_KEY_ACTION_ID, Some(&rename_args)).expect("rename input").expect("known rename action"),
        SnapshotEditEvent::RenameKey { path: "/labels/z".into(), key: "renamed".into() }
    );
}

#[test]
fn chunked_rfc6901_paths_join_losslessly_and_require_one_canonical_shape() {
    let segment = "é/🚀~field".repeat(96);
    let pointer = format!("/{}", segment.replace('~', "~0").replace('/', "~1"));
    let split = pointer.char_indices().nth(240).map(|(index, _)| index).expect("UTF-8 split boundary");
    let args = DslValue::object([
        (
            "pathChunks".to_string(),
            DslValue::Array(vec![DslValue::String(pointer[..split].into()), DslValue::String(pointer[split..].into())]),
        ),
        ("value".to_string(), DslValue::String("updated".into())),
    ]);
    assert_eq!(
        snapshot_edit_event_from_action(SET_SNAPSHOT_VALUE_ACTION_ID, Some(&args)).expect("chunked path").expect("known action"),
        SnapshotEditEvent::SetValue { path: pointer.clone(), value: DslValue::String("updated".into()) }
    );
    let ambiguous = DslValue::object([
        ("path".to_string(), DslValue::String(pointer)),
        ("pathChunks".to_string(), DslValue::Array(vec![DslValue::String("/other".into())])),
        ("value".to_string(), DslValue::String("updated".into())),
    ]);
    assert_eq!(snapshot_edit_event_from_action(SET_SNAPSHOT_VALUE_ACTION_ID, Some(&ambiguous)).expect_err("two pointer shapes").code.0, "snapshot-edit.path-shape");
    let definition = snapshot_edit_actions().into_iter().find(|definition| definition.id == SET_SNAPSHOT_VALUE_ACTION_ID).expect("set action");
    assert!(definition.args.iter().any(|argument| argument.id == "pathChunks" && matches!(argument.schema, semio_framework_plugin::ArgSchema::Array { .. })));
}

#[test]
fn direct_control_action_inputs_preserve_their_typed_values() {
    let fixture = fixture();
    let definitions = snapshot_edit_actions();
    for case in fixture["actionInputs"].as_array().expect("direct action inputs") {
        let args: DslValue = pack::json::from_json_str(&case["args"].to_string()).expect("typed action args");
        let definition = definitions.iter().find(|definition| definition.id == case["action"].as_str().expect("action id")).expect("registered action");
        let effective = semio_framework_plugin::effective_action_args(&definition.args, &args, None);
        assert!(semio_framework_plugin::missing_required_args(&definition.args, &effective).is_empty(), "{}: host action argument admission", case["id"]);
        let actual = snapshot_edit_event_from_action(case["action"].as_str().expect("action id"), Some(&effective))
            .unwrap_or_else(|error| panic!("{}: {error:?}", case["id"]))
            .expect("known action");
        assert_eq!(actual, event(&case["expectedEvent"]), "{}", case["id"]);
    }
}

#[test]
fn source_replacement_validates_the_complete_snapshot_without_defaulting() {
    let fixture = fixture();
    let base = snapshot(&fixture["base"]);
    let mut source = fixture["base"].clone();
    source["title"] = serde_json::Value::String("Source".into());
    let next = apply_snapshot_edit(&base, &SnapshotEditEvent::ReplaceSource { source: source.to_string() }).expect("valid source");
    assert_eq!(next.title, "Source");
    let error = apply_snapshot_edit(&base, &SnapshotEditEvent::ReplaceSource { source: "not json".into() }).expect_err("invalid source");
    assert_eq!(error.code, "snapshot-edit.invalid-source");
    assert_eq!(base.title, "Alpha");
}

#[test]
fn command_wrapper_frames_native_binary_without_a_native_text_codec() {
    let native = SnapshotEditingCommand::Native(NativeCommand(7));
    let printed = <SnapshotEditingCommand<NativeCommand> as kernel::OpText>::print_op(&native);
    assert_eq!(<SnapshotEditingCommand<NativeCommand> as kernel::OpText>::parse_op(&printed).expect("native text frame"), native);
    let edit = SnapshotEditingCommand::<NativeCommand>::Edit(SnapshotEditEvent::RemoveValue { path: "/optional".into() });
    let bytes = <SnapshotEditingCommand<NativeCommand> as kernel::OpBinary>::encode_op(&edit).expect("edit binary frame");
    assert_eq!(<SnapshotEditingCommand<NativeCommand> as kernel::OpBinary>::decode_op(&bytes).expect("edit binary frame"), edit);
    assert!(<SnapshotEditingCommand<NativeCommand> as kernel::OpBinary>::TOOL_JOB_IDS.contains(&REPLACE_SNAPSHOT_SOURCE_ACTION_ID));
}

#[test]
fn complete_source_preserves_fields_that_native_encoding_cannot_represent() {
    let fixture = fixture();
    let base = snapshot(&fixture["sourceRoundTrip"]);
    let source = snapshot_edit_source(&base);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&source).unwrap(), fixture["sourceRoundTrip"]);
    assert_eq!(snapshot_from_edit_source::<FixtureSnapshot>(&source).unwrap(), base);
    assert_eq!(apply_snapshot_edit(&snapshot(&fixture["base"]), &SnapshotEditEvent::ReplaceSource { source }).unwrap(), base);
    let mut wide_integer = base.clone();
    wide_integer.count = u64::MAX;
    assert_eq!(snapshot_from_edit_source::<FixtureSnapshot>(&snapshot_edit_source(&wide_integer)).unwrap(), wide_integer);
}

#[test]
fn malformed_and_ambiguous_sources_cannot_silently_discard_details() {
    let fixture = fixture();
    let base = snapshot(&fixture["base"]);
    for case in fixture["rejectedSources"].as_array().unwrap() {
        let source = case["source"].as_str().unwrap().to_owned();
        let error = apply_snapshot_edit(&base, &SnapshotEditEvent::ReplaceSource { source }).unwrap_err();
        assert_eq!(error.code, case["code"].as_str().unwrap(), "{}", case["id"]);
    }
}

#[test]
fn source_preserves_wide_integers_with_an_independent_json_oracle() {
    let fixture = fixture();
    let source = fixture["wideIntegerSource"].as_str().unwrap();
    let parsed: DslValue = snapshot_from_edit_source(source).unwrap();
    let oracle: serde_json::Value = serde_json::from_str(source).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&snapshot_edit_source(&parsed)).unwrap(), oracle);
}
