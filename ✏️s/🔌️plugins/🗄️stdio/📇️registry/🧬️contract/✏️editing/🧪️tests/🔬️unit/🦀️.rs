use super::*;
use semio_framework_plugin::plugin_app_close_prelude::store as fixture_store;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
struct NativeCommand(u8);

impl kernel::OpBinary for NativeCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &["native"];
    fn encode_op(&self) -> Result<Vec<u8>, kernel::ProtocolError> {
        Ok(vec![self.0])
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, kernel::ProtocolError> {
        bytes.first().copied().map(Self).ok_or_else(|| kernel::ProtocolError::Malformed { what: "native fixture", offset: 0, detail: "empty".into() })
    }
}

crate::snapshot_editing_command_roster!(NativeCommand, ["native"]);

struct RawNativeCommand(String);

impl kernel::OpBinary for RawNativeCommand {
    fn encode_op(&self) -> Result<Vec<u8>, kernel::ProtocolError> {
        Ok(self.0.as_bytes().to_vec())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, kernel::ProtocolError> {
        String::from_utf8(bytes.to_vec()).map(Self).map_err(|error| kernel::ProtocolError::Malformed { what: "raw native fixture", offset: 0, detail: error.to_string() })
    }
}

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
        Err(kernel::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("native source omits details: {text}"), kernel::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        self.title.clone()
    }
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪆️snapshot-edits/🔣️patch-cases.json")).expect("language-neutral snapshot edit fixture")
}

fn snapshot(value: &serde_json::Value) -> FixtureSnapshot {
    semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("typed fixture snapshot")
}

fn event(value: &serde_json::Value) -> SnapshotEditEvent {
    semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("typed fixture event")
}

struct ProbePreparationFactory {
    accepts: fn(&u8) -> bool,
    retained_bytes: usize,
}

impl fixture_store::ArtifactStoreOneItemPreparationFactory<u8, u8> for ProbePreparationFactory {
    fn preflight(&self, mutation: &u8, _description: Option<&str>, _lane: fixture_store::HistoryLane) -> Result<fixture_store::ArtifactStoreOneItemFootprint, String> {
        if (self.accepts)(mutation) {
            Ok(fixture_store::ArtifactStoreOneItemFootprint { work_items: fixture_store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS, retained_bytes: self.retained_bytes })
        } else {
            Err("probe-refused".into())
        }
    }

    fn begin(&self, request: fixture_store::ArtifactStoreOneItemPreparationRequest<u8, u8>) -> Result<Box<dyn fixture_store::ArtifactStoreOneItemPreparation<u8, u8>>, fixture_store::ArtifactStoreOneItemPreparationRequest<u8, u8>> {
        Err(request)
    }
}

struct ProbePreparation {
    remaining: usize,
    checkpoint: fixture_store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl fixture_store::ArtifactStoreOneItemPreparation<u8, u8> for ProbePreparation {
    fn advance(&mut self, grant: fixture_store::ArtifactStoreOneItemGrant) -> Result<fixture_store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled || self.closing || self.remaining == 0 {
            return Ok(fixture_store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        self.remaining -= 1;
        self.checkpoint.cursor += 1;
        self.checkpoint.completed_items += 1;
        self.checkpoint.completed_bytes += 1;
        Ok(fixture_store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
    }

    fn checkpoint(&self) -> fixture_store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&fixture_store::ArtifactStoreOneItemPrepared<u8, u8>> {
        None
    }

    fn take_prepared(&mut self) -> Option<fixture_store::ArtifactStoreOneItemPrepared<u8, u8>> {
        None
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: fixture_store::ArtifactStoreOneItemGrant) -> Result<fixture_store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing || !grant.permits_one() {
            return Ok(fixture_store::SnapshotRetirementStep::Blocked);
        }
        if self.remaining == 0 {
            return Ok(fixture_store::SnapshotRetirementStep::Complete);
        }
        self.remaining -= 1;
        Ok(fixture_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 1 })
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.remaining == 0
    }
}

#[test]
fn retained_native_route_refuses_without_fallback_and_lifecycle_is_cancelable() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️retained-native/🔣️.json")).expect("retained native fixture");
    let primary: ArtifactPreparationFactory<u8, u8> = std::sync::Arc::new(ProbePreparationFactory { accepts: |mutation| *mutation == 1, retained_bytes: 11 });
    let fallback: ArtifactPreparationFactory<u8, u8> = std::sync::Arc::new(ProbePreparationFactory { accepts: |_| true, retained_bytes: 22 });
    let factory: ArtifactPreparationFactory<u8, u8> = routed_native_edit_preparation_factory(Some(NativeEditPreparationRoute::new(|mutation| *mutation == 1 || *mutation == 2, primary)), fallback);
    for row in fixture["routeCases"].as_array().expect("route cases") {
        let mutation = row["mutation"].as_u64().expect("mutation") as u8;
        let actual = factory.preflight(&mutation, None, fixture_store::HistoryLane::Document);
        match row["expected"].as_str().expect("expected route") {
            "primary" => assert_eq!(actual.expect("primary route").retained_bytes, 11, "{}", row["id"]),
            "fallback" => assert_eq!(actual.expect("fallback route").retained_bytes, 22, "{}", row["id"]),
            "refused" => assert_eq!(actual.expect_err("recognized refusal must not fall back"), "probe-refused", "{}", row["id"]),
            other => panic!("unknown route expectation {other}"),
        }
    }

    let lifecycle = &fixture["lifecycle"];
    let units = lifecycle["units"].as_u64().expect("units") as usize;
    let cancel_after = lifecycle["cancelAfter"].as_u64().expect("cancel after") as usize;
    let mut preparation = ProbePreparation { remaining: units, checkpoint: Default::default(), cancelled: false, closing: false };
    let grant = fixture_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 };
    let mut progress = Vec::new();
    for _ in 0..cancel_after {
        let fixture_store::ArtifactStoreOneItemPreparationStep::Progress(checkpoint) = fixture_store::ArtifactStoreOneItemPreparation::advance(&mut preparation, grant).expect("progress") else {
            panic!("retained preparation did not progress");
        };
        progress.push(checkpoint.cursor as u64);
    }
    assert_eq!(progress, lifecycle["expectedProgress"].as_array().expect("progress").iter().map(|value| value.as_u64().expect("cursor")).collect::<Vec<_>>());
    fixture_store::ArtifactStoreOneItemPreparation::cancel(&mut preparation);
    assert_eq!(fixture_store::ArtifactStoreOneItemPreparation::advance(&mut preparation, grant).expect("cancel blocks"), fixture_store::ArtifactStoreOneItemPreparationStep::Blocked);
    fixture_store::ArtifactStoreOneItemPreparation::begin_close(&mut preparation);
    let mut close_steps = 0usize;
    loop {
        close_steps += 1;
        if fixture_store::ArtifactStoreOneItemPreparation::close_step(&mut preparation, grant).expect("bounded close") == fixture_store::SnapshotRetirementStep::Complete {
            break;
        }
    }
    assert_eq!(close_steps, lifecycle["expectedCloseSteps"].as_u64().expect("close steps") as usize);
    assert!(fixture_store::ArtifactStoreOneItemPreparation::terminal_is_empty(&preparation));

    let text_copy = &fixture["textCopy"];
    let byte_length = text_copy["byteLength"].as_u64().expect("text byte length") as usize;
    let page_bytes = text_copy["pageBytes"].as_u64().expect("text page bytes") as usize;
    let source = "x".repeat(byte_length);
    let mut copy = RetainedTextCopy::default();
    assert_eq!(copy.advance(&source, page_bytes).expect("reserve text owner"), Some(0));
    let mut data_steps = 0usize;
    while !copy.is_complete() {
        copy.advance(&source, page_bytes).expect("copy one text page");
        data_steps += 1;
    }
    assert_eq!(data_steps, text_copy["expectedDataSteps"].as_u64().expect("data steps") as usize);
    assert_eq!(copy.take().expect("completed text owner"), source);

    let unicode = text_copy["interruptedUnicode"].as_str().expect("unicode interruption source");
    for stop in text_copy["partialByteStops"].as_array().expect("partial byte stops").iter().map(|value| value.as_u64().expect("partial byte stop") as usize) {
        let mut interrupted = RetainedTextCopy::default();
        assert_eq!(interrupted.advance(unicode, stop).expect("reserve interrupted text"), Some(0));
        assert_eq!(interrupted.advance(unicode, stop).expect("copy interrupted bytes"), Some(stop));
        let partial = interrupted.take_partial_bytes();
        assert_eq!(partial, unicode.as_bytes()[..stop]);
        assert_eq!(String::from_utf8(partial).is_ok(), unicode.is_char_boundary(stop));
    }

    let command_admission = &fixture["commandAdmission"];
    let maximum_bytes = command_admission["maximumBytes"].as_u64().expect("maximum command bytes") as usize;
    for row in command_admission["cases"].as_array().expect("command admission cases") {
        let value = row["value"].as_str().expect("command value");
        let result = admit_bounded_native_command(&RawNativeCommand(value.into()), maximum_bytes);
        assert_eq!(result.is_ok(), row["accepted"].as_bool().expect("admission result"), "{}", row["id"]);
    }

    let mut cancelled = RetainedTextCopy::default();
    cancelled.advance(&source, page_bytes).expect("reserve cancelled owner");
    let cancel_after = text_copy["cancelAfterBytes"].as_u64().expect("cancel bytes") as usize;
    let mut copied = 0usize;
    while copied < cancel_after {
        copied += cancelled.advance(&source, page_bytes).expect("copy cancelled text").expect("positive byte grant");
    }
    assert_eq!(copied, cancel_after);
    let mut close_steps = 0usize;
    while !cancelled.terminal_is_empty() {
        close_steps += 1;
        cancelled.close_step(1, page_bytes);
    }
    assert_eq!(close_steps, text_copy["expectedCloseSteps"].as_u64().expect("close steps") as usize);

    let document_copy = &fixture["documentCopy"];
    let sibling_length = document_copy["siblingByteLength"].as_u64().expect("sibling byte length") as usize;
    let page_bytes = document_copy["pageBytes"].as_u64().expect("document page bytes") as usize;
    let sibling: Vec<u8> = (0..sibling_length).map(|index| (index % 251) as u8).collect();
    let mut copy = RetainedBytesCopy::default();
    assert_eq!(copy.advance(&sibling, page_bytes).expect("reserve byte owner"), Some(0));
    let mut data_steps = 0usize;
    while !copy.is_complete() {
        assert!(copy.advance(&sibling, page_bytes).expect("copy one byte page").expect("positive byte grant") <= page_bytes);
        data_steps += 1;
    }
    assert!(data_steps > document_copy["minimumCompleteTurns"].as_u64().expect("minimum complete turns") as usize);
    assert_eq!(copy.take().expect("completed byte owner"), sibling);

    let mut cancelled = RetainedBytesCopy::default();
    cancelled.advance(&sibling, page_bytes).expect("reserve cancelled byte owner");
    for _ in 0..document_copy["cancelAfterTurns"].as_u64().expect("cancel after turns") {
        assert!(cancelled.advance(&sibling, page_bytes).expect("copy cancelled byte page").expect("positive byte grant") <= page_bytes);
    }
    while !cancelled.terminal_is_empty() {
        cancelled.close_step(1, page_bytes);
    }
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
    let duplicate = SnapshotEditEvent::SetValue { path: "/labels".into(), value: DslValue::Object(vec![("same".into(), DslValue::String("a".into())), ("same".into(), DslValue::String("b".into()))]) };
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
        let value: DslValue = semio_framework_pack_json::from_json_str(source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
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
    let args = DslValue::object([("path".to_string(), DslValue::String("/choice".into())), ("value".to_string(), DslValue::String(r#"{"kind":"second","name":"chosen"}"#.into())), ("valueEncoding".to_string(), DslValue::String("json".into()))]);
    let parsed = snapshot_edit_event_from_action(SET_SNAPSHOT_VALUE_ACTION_ID, Some(&args)).expect("valid action").expect("known action");
    let text = <SnapshotEditEvent as kernel::OpText>::print_op(&parsed);
    assert_eq!(<SnapshotEditEvent as kernel::OpText>::parse_op(&text).expect("text round trip"), parsed);
    let bytes = <SnapshotEditEvent as kernel::OpBinary>::encode_op(&parsed).expect("binary encode");
    assert_eq!(<SnapshotEditEvent as kernel::OpBinary>::decode_op(&bytes).expect("binary round trip"), parsed);
    let source_args = DslValue::object([("value".to_string(), DslValue::String("invalid local draft".into()))]);
    assert_eq!(snapshot_edit_event_from_action(REPLACE_SNAPSHOT_SOURCE_ACTION_ID, Some(&source_args)).expect("source input").expect("known source action"), SnapshotEditEvent::ReplaceSource { source: "invalid local draft".into() });
    let encoded_args = DslValue::object([("path".to_string(), DslValue::String("/choice".into())), ("value".to_string(), DslValue::String(r#"{"kind":"first","level":7}"#.into())), ("valueEncoding".to_string(), DslValue::String("json".into()))]);
    assert!(matches!(snapshot_edit_event_from_action(SET_SNAPSHOT_VALUE_ACTION_ID, Some(&encoded_args)).expect("encoded control").expect("known action"), SnapshotEditEvent::SetValue { value: DslValue::Object(_), .. }));
    let rename_args = DslValue::object([("path".to_string(), DslValue::String("/labels/z".into())), ("value".to_string(), DslValue::String("renamed".into()))]);
    assert_eq!(snapshot_edit_event_from_action(RENAME_SNAPSHOT_KEY_ACTION_ID, Some(&rename_args)).expect("rename input").expect("known rename action"), SnapshotEditEvent::RenameKey { path: "/labels/z".into(), key: "renamed".into() });
}

#[test]
fn chunked_rfc6901_paths_join_losslessly_and_require_one_canonical_shape() {
    let segment = "é/🚀~field".repeat(96);
    let pointer = format!("/{}", segment.replace('~', "~0").replace('/', "~1"));
    let split = pointer.char_indices().nth(240).map(|(index, _)| index).expect("UTF-8 split boundary");
    let args = DslValue::object([("pathChunks".to_string(), DslValue::Array(vec![DslValue::String(pointer[..split].into()), DslValue::String(pointer[split..].into())])), ("value".to_string(), DslValue::String("updated".into()))]);
    assert_eq!(snapshot_edit_event_from_action(SET_SNAPSHOT_VALUE_ACTION_ID, Some(&args)).expect("chunked path").expect("known action"), SnapshotEditEvent::SetValue { path: pointer.clone(), value: DslValue::String("updated".into()) });
    let ambiguous = DslValue::object([("path".to_string(), DslValue::String(pointer)), ("pathChunks".to_string(), DslValue::Array(vec![DslValue::String("/other".into())])), ("value".to_string(), DslValue::String("updated".into()))]);
    assert_eq!(snapshot_edit_event_from_action(SET_SNAPSHOT_VALUE_ACTION_ID, Some(&ambiguous)).expect_err("two pointer shapes").code.0, "snapshot-edit.path-shape");
    let definition = snapshot_edit_actions().into_iter().find(|definition| definition.id == SET_SNAPSHOT_VALUE_ACTION_ID).expect("set action");
    assert!(definition.args.iter().any(|argument| argument.id == "pathChunks" && matches!(argument.schema, semio_framework_plugin::ArgSchema::Array { .. })));
}

#[test]
fn direct_control_action_inputs_preserve_their_typed_values() {
    let fixture = fixture();
    let definitions = snapshot_edit_actions();
    for case in fixture["actionInputs"].as_array().expect("direct action inputs") {
        let args: DslValue = semio_framework_pack_json::from_json_str(&case["args"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("typed action args");
        let definition = definitions.iter().find(|definition| definition.id == case["action"].as_str().expect("action id")).expect("registered action");
        let effective = semio_framework_plugin::effective_action_args(&definition.args, &args, None);
        assert!(semio_framework_plugin::missing_required_args(&definition.args, &effective).is_empty(), "{}: host action argument admission", case["id"]);
        let actual = snapshot_edit_event_from_action(case["action"].as_str().expect("action id"), Some(&effective)).unwrap_or_else(|error| panic!("{}: {error:?}", case["id"])).expect("known action");
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
fn malformed_source_is_admitted_by_bounded_preflight_then_reports_its_exact_typed_span() {
    let diagnostic_fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🩺️source-diagnostic/🔣️.json")).expect("source diagnostic fixture");
    let edit_fixture = fixture();
    let base = snapshot(&edit_fixture["base"]);
    for row in diagnostic_fixture["cases"].as_array().expect("diagnostic cases") {
        let source = row["source"].as_str().expect("source");
        let event = SnapshotEditEvent::ReplaceSource { source: source.to_owned() };
        assert!(snapshot_edit_value_is_admitted(&event, &base), "{}: bounded preflight admits semantic validation", row["id"]);
        let oracle = serde_json::from_str::<serde_json::Value>(source).expect_err("the independent JSON parser rejects malformed source");
        let error = apply_snapshot_edit(&base, &event).expect_err("the reducer rejects malformed source");
        let expected = &row["expected"];
        let span = error.span.expect("malformed source has a span");
        assert_eq!(error.code, expected["code"].as_str().expect("code"), "{}", row["id"]);
        assert_eq!((u64::from(span.line), u64::from(span.column), u64::from(span.length)), (expected["line"].as_u64().unwrap(), expected["column"].as_u64().unwrap(), expected["length"].as_u64().unwrap()), "{}", row["id"]);
        assert_eq!((oracle.line() as u64, oracle.column() as u64), (u64::from(span.line), u64::from(span.column)), "{}: serde_json independently locates the same refusal", row["id"]);
        let fault = snapshot_edit_fault(error);
        assert_eq!(fault.code.0, "snapshot-edit.invalid-source");
        assert_eq!(fault.span, Some(span));
    }
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
