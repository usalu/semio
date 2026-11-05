use crate::editor::pdf14::modes::edit::windows::main;
use crate::standards::v1_4::subsets::base::schema::{mutations::PdfMutation, snapshot::PdfSnapshot};
use crate::standards::v1_4::subsets::base::schema::snapshot::STDIO_PDF_DOCUMENT_SCHEMA;
use crate::editor::pdf14::page;
use semio_plugin::{ArtifactEditor, Editor};
#[allow(unused_imports)]
use crate::editor::pdf14::modes::edit;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::PDF_ARTIFACT_SCHEMA_ID;
#[allow(unused_imports)]
use {semio_plugin::built_to_component_tree, semio_plugin::ArtifactView, semio_plugin::ComponentTree, semio_plugin::ConfigView, semio_framework_artifact_reference::Dialect, semio_plugin::DraftView, semio_plugin::Emit, semio_plugin::Fault, semio_plugin::NoConfig, semio_plugin::NoConfigMutation, semio_plugin::NoDraft, semio_plugin::NoDraftMutation, semio_plugin::NoPresence, semio_plugin::NoPresenceMutation, semio_plugin::NoTransient, semio_plugin::NoTransientMutation, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId};
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::editor::pdf14::*;

#[semio_framework_async_macros::async_test]
async fn create_pdf14_editor_builds_a_definition_for_the_editor_role() {
    let def = create_pdf14_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
    assert_eq!(def.dialect, PDF14_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Pdf14Editor as ArtifactEditor>::DIALECT, PDF14_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_pdf14_editor();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn missing_set_page_payload_is_rejected() {
    assert!(<Pdf14Editor as ArtifactEditor>::command_from_action("set-page", None, None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn explicit_nonzero_page_payload_is_preserved() {
    let args = semio_framework_value::DslValue::Object(vec![
        ("page".into(), semio_framework_value::DslValue::float(3.0)),
        ("item".into(), semio_framework_value::DslValue::float(0.0)),
        ("revision".into(), semio_framework_value::DslValue::String("0123456789abcdef".into())),
        ("text".into(), semio_framework_value::DslValue::String("replacement".into())),
    ]);
    let command = <Pdf14Editor as ArtifactEditor>::command_from_action("set-page", Some((args).clone()), None).expect("typed payload");
    assert!(matches!(command, semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf14EditorCommand::SetPage { page: 3, item: 0, revision, text }) if revision == "0123456789abcdef" && text == "replacement"));
}

#[test]
fn own14_declared_document_and_mutation_owners_are_exact() {
    assert_eq!(<Pdf14Editor as ArtifactEditor>::DOCUMENT_SCHEMA, crate::standards::v1_4::subsets::base::schema::snapshot::STDIO_PDF_DOCUMENT_SCHEMA);
    assert_eq!(std::any::type_name::<<Pdf14Editor as ArtifactEditor>::Snapshot>(), std::any::type_name::<crate::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot>());
    assert_eq!(std::any::type_name::<<Pdf14Editor as ArtifactEditor>::Mutation>(), std::any::type_name::<crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation>());
}

#[test]
fn own14_typed_page_command_uses_the_own_mutation_codec() {
    use protocol::{OpBinary, OpText, ColdMutation, Mutation, MutationDiff};
    let args = semio_framework_value::DslValue::Object(vec![("page".into(), semio_framework_value::DslValue::float(0.0)), ("width".into(), semio_framework_value::DslValue::float(8.0)), ("height".into(), semio_framework_value::DslValue::float(9.0))]);
    let command = <Pdf14Editor as ArtifactEditor>::command_from_action("set-page-size", Some((args).clone()), None).unwrap();
    let semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(native) = command else { panic!("own native page command"); };
    assert_eq!(Pdf14EditorCommand::decode_op(&native.encode_op().unwrap()).unwrap(), native);
    assert_eq!(Pdf14EditorCommand::parse_op(&native.print_op()).unwrap(), native);
    let Pdf14EditorCommand::PageEdit { action, payload } = native else { panic!("actual own page action"); };
    let original = <Pdf14Editor as ArtifactEditor>::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births");
    let emit = page::emit_page_edit(&original, &action, &payload).unwrap();
    assert_eq!(emit.artifact_mutations.len(), 1);
    let op = &emit.artifact_mutations[0];
    let next = protocol::apply_diff_cold(op.diff(&original).diff(), &original).unwrap();
    assert_eq!(next.pages[0].width.to_bits(), 8.0f64.to_bits());
    assert_eq!(next.pages[0].height.to_bits(), 9.0f64.to_bits());
    assert_eq!(next.pages[0].text, original.pages[0].text);
    let mut restored = next;
    for inverse in op.inverse(&original).unwrap() { restored = protocol::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap(); }
    assert_eq!(restored, original);
}

semio_plugin::history_edit_acceptance_law!("stdio/Pdf14Editor", Pdf14Editor, || semio_plugin::App { definition: create_pdf14_editor(), examples: Vec::new() }, "../..");

#[semio_framework_async_macros::async_test]
async fn natural_pdf_14_file_opens_its_own_output_and_refuses_files_the_page_model_would_degrade() {
    let codec = <Pdf14Editor as ArtifactEditor>::natural_file_codec().expect("the PDF 1.4 editor declares a natural file");
    assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ("s.stdio.pdf@1.4", ".pdf", "application/pdf", true));
    let mut snapshot = PdfSnapshot::default();
    snapshot.pages[0].text = "Hello".into();
    let saved = <Pdf14Editor as ArtifactEditor>::encode_natural_file(&snapshot).expect("the page model saves");
    let path = std::env::temp_dir().join(format!("semio-pdf14-{}.pdf", std::process::id()));
    std::fs::write(&path, &saved).unwrap();
    match std::process::Command::new("qpdf").arg("--check").arg(&path).output() {
        Ok(output) => assert!(output.status.success(), "qpdf --check refused the saved PDF: {}", String::from_utf8_lossy(&output.stderr)),
        Err(error) => assert_eq!(error.kind(), std::io::ErrorKind::NotFound, "qpdf could not run: {error}"),
    }
    let _ = std::fs::remove_file(&path);
    let reopened = <Pdf14Editor as ArtifactEditor>::decode_natural_file(&saved).expect("a file this model wrote reopens");
    assert_eq!(reopened.pages[0].text, "Hello");
    let foreign = include_bytes!("../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let refusal = <Pdf14Editor as ArtifactEditor>::decode_natural_file(foreign).err().expect("a foreign PDF would lose content");
    assert!(refusal.to_string().contains("PDF 1.7 editor"), "{refusal}");
    assert!(<Pdf14Editor as ArtifactEditor>::decode_natural_file(b"not a pdf").is_err());
}
