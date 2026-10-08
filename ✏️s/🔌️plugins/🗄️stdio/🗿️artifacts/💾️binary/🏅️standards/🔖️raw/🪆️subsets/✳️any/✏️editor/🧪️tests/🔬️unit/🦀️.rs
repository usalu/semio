use super::*;

#[semio_framework_async_macros::async_test]
async fn create_binary_editor_builds_a_definition_for_the_editor_role() {
    let def = create_binary_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, BINARY_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<BinaryEditor as ArtifactEditor>::DIALECT, BINARY_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_binary_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn parse_hex_dump_round_trips_a_rendered_snapshot() {
    let document = BinarySnapshot { bytes: vec![0xde, 0xad, 0xbe, 0xef], ..BinarySnapshot::default() };
    let node = main::render(&document, semio_framework_ui_locale::Locale::En, semio_framework_plugin::UiPublicationRevision(23)).expect("render");
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("decode the text scene with its lanes");
    let parsed = parse_hex_dump(&scene.buffer).expect("well-formed hex dump must parse");
    assert_eq!(parsed, vec![0xde, 0xad, 0xbe, 0xef]);
}

#[semio_framework_async_macros::async_test]
async fn parse_hex_dump_rejects_odd_length_hex() {
    assert!(parse_hex_dump("abc").is_none());
    assert!(parse_hex_dump("€0").is_none());
}

#[test]
fn text_edit_requires_an_explicit_change_set_and_allows_intentional_empty_bytes() {
    assert!(<BinaryEditor as ArtifactEditor>::command_from_action("textEdit", None).is_err());
    let whole_draft = semio_framework_value::DslValue::object([("text".into(), semio_framework_value::DslValue::String(String::new()))]);
    assert!(<BinaryEditor as ArtifactEditor>::command_from_action("textEdit", Some(&whole_draft)).is_err(), "a whole draft is no gesture");
    let source = BinarySnapshot { bytes: vec![1, 2, 3], ..BinarySnapshot::default() };
    let emit = |splices: &str| {
        let args = semio_framework_value::DslValue::object([("splices".into(), semio_framework_value::DslValue::String(splices.into()))]);
        let command = <BinaryEditor as ArtifactEditor>::command_from_action("textEdit", Some(&args)).expect("explicit change set");
        binary_text_emit(&command, &source)
    };
    assert!(emit("[]").expect("an empty change set").artifact_mutations.is_empty());
    let cleared = emit(r#"[{"offset":0,"delete":6,"insert":""}]"#).expect("deleting every digit intentionally clears the byte buffer");
    assert!(matches!(
        cleared.artifact_mutations.as_slice(),
        [BinaryMutation::ReplaceByteRange(replace_byte_range::ReplaceByteRange { offset: 0, remove_len: 3, insert })] if insert.is_empty()
    ));
    assert!(emit("not json").is_err());
}

/// ⚖️ LAW (corpus `🧫️fixtures/✂️hex-splices`, oracle `🧪️tests/✂️hex-splices/🟦️.ts`): an explicit Apply of the hex window is the
/// corpus's `replace-byte-range` per touched byte run (last first), or refused; applying them lands on the bytes the digits mean.
#[test]
fn an_applied_change_set_is_exactly_the_corpus_byte_ranges() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/✂️hex-splices/🔣️.json")).expect("hex-splices corpus");
    for case in corpus["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let source = BinarySnapshot { bytes: parse_hex_dump(case["before"].as_str().expect("before")).expect("corpus hex"), ..BinarySnapshot::default() };
        let args = semio_framework_value::DslValue::object([("splices".into(), semio_framework_value::DslValue::String(case["splices"].to_string()))]);
        let command = <BinaryEditor as ArtifactEditor>::command_from_action("textEdit", Some(&args)).expect("explicit change set");
        let emit = binary_text_emit(&command, &source);
        let Some(expected) = case["leaves"].as_array() else {
            assert!(emit.is_err(), "{id}: refused");
            continue;
        };
        let emit = emit.expect("a valid change set");
        let summary: Vec<serde_json::Value> = emit
            .artifact_mutations
            .iter()
            .map(|leaf| match leaf {
                BinaryMutation::ReplaceByteRange(range) => serde_json::json!({ "offset": range.offset, "removeLen": range.remove_len, "insert": range.insert.iter().map(|byte| format!("{byte:02x}")).collect::<String>() }),
                other => serde_json::json!({ "kind": format!("{other:?}") }),
            })
            .collect();
        assert_eq!(&summary, expected, "{id}: the byte ranges");
        let mut next = source.clone();
        for leaf in &emit.artifact_mutations {
            let outcome = <BinaryMutation as protocol::Mutation<BinarySnapshot>>::diff(leaf, &next);
            next = protocol::apply_diff(outcome.diff(), &next).expect("the range applies");
            let inverse = <BinaryMutation as protocol::Mutation<BinarySnapshot>>::inverse(leaf, &source).unwrap_or_default();
            assert!(inverse.len() <= 1, "{id}: one splice, one inverse");
        }
    }
}

/// ⚖️ LAW (design §20.3): a document-details edit of the bytes is ONE `replace-byte-range` carrying the gesture — a set of the whole
/// list replaces the buffer, a set/insert/remove/move of one byte is a byte splice; an unchanged value moves nothing.
#[test]
fn a_document_details_edit_is_one_byte_splice() {
    use semio_s_artifact_stdio_contract::editing::{SnapshotEditEvent, SnapshotEditingEditor};
    ::semio_framework_schema_registry::register_artifact_schema_descriptors(vec![crate::standards::v_raw::subsets::any::schema::binary_artifact_schema_descriptor()]).expect("register binary schema");
    let base = BinarySnapshot { bytes: vec![0xde, 0xad, 0xbe, 0xef], ..BinarySnapshot::default() };
    let emit = |event: SnapshotEditEvent| <BinaryEditor as SnapshotEditingEditor>::snapshot_edit_emit(&event, &base).expect("the details edit publishes");
    let number = |value: u64| semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(value));
    let range = |emit: &Emit<BinaryMutation>, index: usize| match &emit.artifact_mutations[index] {
        BinaryMutation::ReplaceByteRange(range) => (range.offset, range.remove_len, range.insert.clone()),
        other => panic!("unexpected {other:?}"),
    };
    let whole = emit(SnapshotEditEvent::SetValue { path: "/bytes".into(), value: semio_framework_value::ToValue::to_value(&vec![0xde_u8, 0x00, 0xbe, 0xef]) });
    assert_eq!(range(&whole, 0), (0, 4, vec![0xde, 0x00, 0xbe, 0xef]));
    assert!(emit(SnapshotEditEvent::SetValue { path: "/bytes".into(), value: semio_framework_value::ToValue::to_value(&base.bytes) }).artifact_mutations.is_empty(), "an unchanged value moves nothing");
    assert_eq!(range(&emit(SnapshotEditEvent::SetValue { path: "/bytes/1".into(), value: number(0) }), 0), (1, 1, vec![0x00]));
    assert_eq!(range(&emit(SnapshotEditEvent::InsertValue { path: "/bytes/-".into(), value: number(1) }), 0), (4, 0, vec![0x01]));
    assert_eq!(range(&emit(SnapshotEditEvent::RemoveValue { path: "/bytes/0".into() }), 0), (0, 1, vec![]));
    let moved = emit(SnapshotEditEvent::MoveValue { from: "/bytes/0".into(), path: "/bytes/2".into() });
    assert_eq!((range(&moved, 0), range(&moved, 1)), ((0, 1, vec![]), (2, 0, vec![0xde])));
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::BinaryEditor, || semio_framework_plugin::App { definition: super::create_binary_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️raw/🪆️subsets/✳️any");
