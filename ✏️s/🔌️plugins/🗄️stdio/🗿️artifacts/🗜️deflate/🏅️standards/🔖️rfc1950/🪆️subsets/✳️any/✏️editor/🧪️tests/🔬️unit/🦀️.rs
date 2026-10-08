use super::*;

#[semio_framework_async_macros::async_test]
async fn create_deflate_editor_builds_a_definition_for_the_editor_role() {
    let def = create_deflate_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DEFLATE_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DeflateEditor as ArtifactEditor>::DIALECT, DEFLATE_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_deflate_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn parse_header_summary_round_trips_a_rendered_snapshot() {
    let document = DeflateSnapshot { compression_method: 8, window_bits: 9, compression_level_hint: crate::schema::snapshot::DeflateLevelHint::Maximum, dict_id: Some(7), payload: vec![9, 9], ..DeflateSnapshot::default() };
    let node = main::render(&document, semio_framework_ui_locale::Locale::En, semio_framework_plugin::UiPublicationRevision(23)).expect("render");
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("decode the text scene with its lanes");
    let (method, window_bits, level_hint, dict_id) = parse_header_summary(&scene.buffer).expect("well-formed summary must parse");
    assert_eq!(method, 8);
    assert_eq!(window_bits, 9);
    assert_eq!(level_hint, crate::schema::snapshot::DeflateLevelHint::Maximum);
    assert_eq!(dict_id, Some(7));
}

#[semio_framework_async_macros::async_test]
async fn parse_header_summary_rejects_a_missing_required_field() {
    assert!(parse_header_summary("method=8\nwindowBits=7").is_none());
    assert!(parse_header_summary("method=16\nwindowBits=7\nlevelHint=default\npresetDictionary=none").is_none());
    assert!(parse_header_summary("method=8\nwindowBits=16\nlevelHint=default\npresetDictionary=none").is_none());
    assert!(parse_header_summary("method=8\nmethod=9\nwindowBits=7\nlevelHint=default\npresetDictionary=none").is_none());
    assert!(parse_header_summary("method=8\nwindowBits=7\nlevelHint=default\npresetDictionary=none\nunknown=1").is_none());
}

#[test]
fn text_edit_requires_source_text_and_rejects_out_of_schema_headers_atomically() {
    assert!(<DeflateEditor as ArtifactEditor>::command_from_action("textEdit", None).is_err());
    let text = "method=8\nwindowBits=255\nlevelHint=default\npresetDictionary=none";
    let args = semio_framework_value::DslValue::object([("text".into(), semio_framework_value::DslValue::String(text.into()))]);
    let command = <DeflateEditor as ArtifactEditor>::command_from_action("textEdit", Some(&args)).expect("complete text argument");
    assert!(deflate_text_emit(&command, &DeflateSnapshot::default()).is_err());
}

/// ⚖️ LAW: an explicit Apply of the header summary is the header leaves it changed and nothing else — the parameters alone, the
/// dictionary alone, both, or none for an unchanged summary — each labelled from its leaf (no static description).
#[test]
fn an_applied_summary_is_only_the_header_leaves_it_changed() {
    let base = DeflateSnapshot::default();
    let emit = |text: &str| {
        let args = semio_framework_value::DslValue::object([("text".into(), semio_framework_value::DslValue::String(text.into()))]);
        let command = <DeflateEditor as ArtifactEditor>::command_from_action("textEdit", Some(&args)).expect("complete text argument");
        deflate_text_emit(&command, &base).expect("a valid summary")
    };
    let unchanged = emit("method=8\nwindowBits=7\nlevelHint=default\npresetDictionary=none");
    assert!(unchanged.artifact_mutations.is_empty(), "an unchanged summary moves nothing: {:?}", unchanged.artifact_mutations);
    let window = emit("method=8\nwindowBits=5\nlevelHint=default\npresetDictionary=none");
    assert!(matches!(window.artifact_mutations.as_slice(), [DeflateMutation::SetCompressionParams(params)] if params.window_bits == 5), "{:?}", window.artifact_mutations);
    let dictionary = emit("method=8\nwindowBits=7\nlevelHint=default\npresetDictionary=7");
    assert!(matches!(dictionary.artifact_mutations.as_slice(), [DeflateMutation::SetPresetDictionary(set)] if set.dict_id == Some(7)), "{:?}", dictionary.artifact_mutations);
    let both = emit("method=8\nwindowBits=5\nlevelHint=default\npresetDictionary=7");
    assert!(matches!(both.artifact_mutations.as_slice(), [DeflateMutation::SetCompressionParams(_), DeflateMutation::SetPresetDictionary(_)]), "{:?}", both.artifact_mutations);
}

/// ⚖️ LAW (design §20.3): a document-details edit is the domain leaves it changed — `set-compression-params` for the window bits,
/// `set-preset-dictionary` for the dictionary id, `set-payload` for the payload, with no
/// description; an unchanged value moves nothing.
#[test]
fn a_document_details_edit_is_only_the_domain_leaves_it_changed() {
    use semio_s_artifact_stdio_contract::editing::{SnapshotEditEvent, SnapshotEditingEditor};
    semio_framework_schema_registry::register_artifact_schema_descriptors(vec![crate::schema::deflate_artifact_schema_descriptor()]).expect("register deflate schema");
    let base = DeflateSnapshot::default();
    let edit = |path: &str, value: semio_framework_value::DslValue| <DeflateEditor as SnapshotEditingEditor>::snapshot_edit_emit(&SnapshotEditEvent::SetValue { path: path.into(), value }, &base).expect("the details edit publishes");
    let window = edit("/windowBits", semio_framework_value::DslValue::uint(5));
    assert!(matches!(window.artifact_mutations.as_slice(), [DeflateMutation::SetCompressionParams(params)] if params.window_bits == 5), "{:?}", window.artifact_mutations);
    let dictionary = <DeflateEditor as SnapshotEditingEditor>::snapshot_edit_emit(&SnapshotEditEvent::InsertValue { path: "/dictId".into(), value: semio_framework_value::DslValue::uint(7) }, &base).expect("the details edit publishes");
    assert!(matches!(dictionary.artifact_mutations.as_slice(), [DeflateMutation::SetPresetDictionary(set)] if set.dict_id == Some(7)), "{:?}", dictionary.artifact_mutations);
    assert!(edit("/windowBits", semio_framework_value::DslValue::uint(u64::from(base.window_bits))).artifact_mutations.is_empty(), "an unchanged value moves nothing");
    let payload = DeflateSnapshot { payload: b"hello".to_vec(), ..DeflateSnapshot::default() };
    let bytes = edit("/payload", semio_framework_value::ToValue::to_value(&payload.payload));
    assert_eq!(bytes.artifact_mutations, vec![DeflateMutation::SetPayload(crate::schema::mutations::set_payload::SetPayload { payload: b"hello".to_vec() })], "a changed payload is ONE set-payload");
}

#[test]
fn compression_params_reject_header_fields_outside_their_four_bits() {
    use crate::schema::mutations::set_compression_params::SetCompressionParams;
    let base = DeflateSnapshot::default();
    let diff = |window_bits: u8| <SetCompressionParams as protocol::MutationKind<DeflateSnapshot, DeflateMutation>>::diff(&SetCompressionParams { method: 8, window_bits, level_hint: base.compression_level_hint }, &base);
    assert!(diff(15).messages().is_empty(), "the maximum boundary is admitted");
    assert!(!diff(255).messages().is_empty(), "a window size beyond the 4-bit field is refused");
    let identity = semio_s_artifact_stdio_contract::editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.unknown".into()) };
    assert_eq!(<DeflateEditor as semio_s_artifact_stdio_contract::editing::SnapshotEditingEditor>::snapshot_edit_emit(&identity, &base).expect_err("no kind edits the schema identity").code.0, "snapshot-edit.unsupported-path");
    let mut unknown = base.clone();
    unknown.schema = "stdio.unknown".into();
    let adapter_error = semio_s_artifact_stdio_contract::editing::validate_snapshot_schema_for_dialect(&semio_framework_value::ToValue::to_value(&unknown), DEFLATE_EDITOR_DIALECT, STDIO_DEFLATE_DOCUMENT_SCHEMA).expect_err("registered adapter identity");
    assert_eq!(adapter_error.code, "snapshot-edit.schema-identity");
}

semio_framework_plugin::history_edit_acceptance_law!("stdio", super::DeflateEditor, || semio_framework_plugin::App { definition: super::create_deflate_editor(), examples: Vec::new() }, "../../🏅️standards/🔖️rfc1950/🪆️subsets/✳️any");
