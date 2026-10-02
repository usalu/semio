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
    let node = main::render(&document, semio_framework_ui_locale::Locale::En).expect("render");
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
fn text_edit_requires_an_explicit_text_argument_and_allows_intentional_empty_bytes() {
    assert!(<BinaryEditor as ArtifactEditor>::command_from_action("textEdit", None).is_err());
    let args = dsl::DslValue::object([("text".into(), dsl::DslValue::String(String::new()))]);
    let command = <BinaryEditor as ArtifactEditor>::command_from_action("textEdit", Some(&args)).expect("explicit empty text");
    let source = BinarySnapshot { bytes: vec![1, 2, 3], ..BinarySnapshot::default() };
    let emitted = binary_text_emit(&command, &source).expect("empty hex intentionally clears the byte buffer");
    assert!(matches!(
        emitted.artifact_mutations.as_slice(),
        [BinaryMutation::ReplaceByteRange(replace_byte_range::ReplaceByteRange { offset: 0, remove_len: 3, insert })] if insert.is_empty()
    ));
}

/// ⚖️ LAW: an explicit Apply of the hex dump is ONE net `replace-byte-range` over exactly the bytes it changed — the shared prefix
/// and suffix stay untouched, an insertion removes nothing, an unchanged dump moves nothing — labelled from its leaf.
#[test]
fn an_applied_hex_dump_is_one_net_byte_range() {
    let source = BinarySnapshot { bytes: vec![0xde, 0xad, 0xbe, 0xef], ..BinarySnapshot::default() };
    let emit = |text: &str| {
        let args = dsl::DslValue::object([("text".into(), dsl::DslValue::String(text.into()))]);
        let command = <BinaryEditor as ArtifactEditor>::command_from_action("textEdit", Some(&args)).expect("explicit text");
        binary_text_emit(&command, &source).expect("well-formed hex")
    };
    let changed = emit("de00beef");
    assert!(matches!(changed.artifact_mutations.as_slice(), [BinaryMutation::ReplaceByteRange(range)] if (range.offset, range.remove_len, range.insert.as_slice()) == (1, 1, &[0x00][..])), "{:?}", changed.artifact_mutations);
    assert!(changed.description.is_none(), "the history row is labelled from its leaf");
    let inserted = emit("deadbe01ef");
    assert!(matches!(inserted.artifact_mutations.as_slice(), [BinaryMutation::ReplaceByteRange(range)] if (range.offset, range.remove_len, range.insert.as_slice()) == (3, 0, &[0x01][..])), "{:?}", inserted.artifact_mutations);
    assert!(emit("deadbeef").artifact_mutations.is_empty(), "an unchanged dump moves nothing");
}

/// ⚖️ LAW (design §20.3): a document-details edit of the bytes is ONE net `replace-byte-range`, never a whole `set-snapshot`, with
/// no description; an unchanged value moves nothing.
#[test]
fn a_document_details_edit_is_one_net_byte_range() {
    use semio_s_artifact_stdio_contract::editing::{snapshot_edit_source, SnapshotEditEvent, SnapshotEditingEditor};
    ::semio_framework_schema_registry::register_artifact_schema_descriptors(vec![crate::standards::v_raw::subsets::any::schema::binary_artifact_schema_descriptor()]).expect("register binary schema");
    let base = BinarySnapshot { bytes: vec![0xde, 0xad, 0xbe, 0xef], ..BinarySnapshot::default() };
    let next = BinarySnapshot { bytes: vec![0xde, 0x00, 0xbe, 0xef], ..BinarySnapshot::default() };
    let edit = |next: &BinarySnapshot| <BinaryEditor as SnapshotEditingEditor>::snapshot_edit_emit(&SnapshotEditEvent::ReplaceSource { source: snapshot_edit_source(next) }, &base).expect("the details edit publishes");
    let changed = edit(&next);
    assert!(matches!(changed.artifact_mutations.as_slice(), [BinaryMutation::ReplaceByteRange(range)] if (range.offset, range.remove_len, range.insert.as_slice()) == (1, 1, &[0x00][..])), "{:?}", changed.artifact_mutations);
    assert!(changed.description.is_none(), "the history row is labelled from its leaf");
    assert!(edit(&base).artifact_mutations.is_empty(), "an unchanged value moves nothing");
}

//#region 🧮️NetLeafLaws
const NET_LEAVES: &str = include_str!("../../../🧫️fixtures/🧫️net-leaves/🔣️.json");

/// ⚖️ LAW (corpus `🧫️fixtures/🧫️net-leaves`, oracle `🧪️tests/🧪️net-leaves/🟦️.ts`): an applied hex dump is exactly the corpus's
/// ONE net byte range (none when no byte changed), and applying it lands exactly on the applied bytes.
#[test]
fn an_applied_hex_dump_is_exactly_the_corpus_net_byte_range() {
    let corpus: serde_json::Value = serde_json::from_str(NET_LEAVES).expect("net-leaves corpus");
    let bytes = |hex: &str| parse_hex_dump(hex).expect("corpus hex");
    for case in corpus["cases"].as_array().expect("cases") {
        let (id, after) = (case["id"].as_str().expect("id"), case["after"].as_str().expect("after"));
        let source = BinarySnapshot { bytes: bytes(case["before"].as_str().expect("before")), ..BinarySnapshot::default() };
        let args = dsl::DslValue::object([("text".into(), dsl::DslValue::String(after.into()))]);
        let command = <BinaryEditor as ArtifactEditor>::command_from_action("textEdit", Some(&args)).expect("explicit text");
        let emit = binary_text_emit(&command, &source).expect("well-formed hex");
        let summary: Vec<serde_json::Value> = emit
            .artifact_mutations
            .iter()
            .map(|leaf| match leaf {
                BinaryMutation::ReplaceByteRange(range) => serde_json::json!({ "kind": "replace-byte-range", "offset": range.offset, "removeLen": range.remove_len, "insert": range.insert.iter().map(|byte| format!("{byte:02x}")).collect::<String>() }),
                other => serde_json::json!({ "kind": format!("{other:?}") }),
            })
            .collect();
        let expected: Vec<serde_json::Value> = case["leaf"].as_object().map(|leaf| serde_json::Value::Object(leaf.clone())).into_iter().collect();
        assert_eq!(summary, expected, "{id}: the net byte range");
        let mut next = source.clone();
        for leaf in &emit.artifact_mutations {
            let outcome = <BinaryMutation as protocol::Mutation<BinarySnapshot>>::diff(leaf, &next);
            next = protocol::MutationDiff::apply(outcome.diff(), &next).expect("the range applies");
        }
        assert_eq!(next.bytes, bytes(after), "{id}: the edit lands on the applied bytes");
    }
}
//#endregion 🧮️NetLeafLaws
