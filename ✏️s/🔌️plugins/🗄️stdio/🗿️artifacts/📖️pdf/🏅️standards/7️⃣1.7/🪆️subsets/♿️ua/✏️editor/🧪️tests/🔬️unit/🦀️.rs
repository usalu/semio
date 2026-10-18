use crate::editor::pdf17ua::modes::edit::windows::main;
use semio_framework_plugin::{ArtifactEditor, Editor};
#[allow(unused_imports)]
use crate::editor::pdf17ua::modes::edit;
#[allow(unused_imports)]
use crate::editor::page::page_text_edit_mutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfSnapshot, STDIO_PDF17_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::PDF_ARTIFACT_SCHEMA_ID;
#[allow(unused_imports)]
use {semio_framework_plugin::built_to_component_tree, semio_framework_plugin::ArtifactView, semio_framework_plugin::ComponentTree, semio_framework_plugin::ConfigView, semio_framework_artifact_reference::Dialect, semio_framework_plugin::DraftView, semio_framework_plugin::Emit, semio_framework_plugin::Fault, semio_framework_plugin::NoConfig, semio_framework_plugin::NoConfigMutation, semio_framework_plugin::NoDraft, semio_framework_plugin::NoDraftMutation, semio_framework_plugin::NoPresence, semio_framework_plugin::NoPresenceMutation, semio_framework_plugin::NoTransient, semio_framework_plugin::NoTransientMutation, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId};
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::editor::pdf17ua::*;

#[semio_framework_async_macros::async_test]
async fn create_pdf17_ua_editor_builds_a_definition_for_the_editor_role() {
    let def = create_pdf17_ua_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, PDF17UA_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Pdf17UaEditor as ArtifactEditor>::DIALECT, PDF17UA_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_pdf17_ua_editor();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn missing_set_page_payload_is_rejected() {
    assert!(<Pdf17UaEditor as ArtifactEditor>::command_from_action("set-page", None, None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn explicit_nonzero_page_payload_is_preserved() {
    let args = semio_framework_value::DslValue::Object(vec![
        ("page".into(), semio_framework_value::DslValue::float(3.0)),
        ("item".into(), semio_framework_value::DslValue::float(0.0)),
        ("revision".into(), semio_framework_value::DslValue::String("0123456789abcdef".into())),
        ("text".into(), semio_framework_value::DslValue::String("replacement".into())),
    ]);
    let command = <Pdf17UaEditor as ArtifactEditor>::command_from_action("set-page", Some((args).clone()), None).expect("typed payload");
    assert!(matches!(command, semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf17UaEditorCommand::SetPage { page: 3, item: 0, revision, text }) if revision == "0123456789abcdef" && text == "replacement"));
}

semio_framework_plugin::history_edit_acceptance_law!("stdio/Pdf17UaEditor", Pdf17UaEditor, || semio_framework_plugin::App { definition: create_pdf17_ua_editor(), examples: Vec::new() }, "../..");
