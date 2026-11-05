use semio_plugin::ArtifactEditor;
use semio_plugin::Editor;
#[allow(unused_imports)]
use crate::editor::modes::edit;
#[allow(unused_imports)]
use crate::editor::modes::edit::windows::main;
#[allow(unused_imports)]
use crate::schema::mutations::SemioKitMutation;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioKitSnapshot;
#[allow(unused_imports)]
use semio_plugin::app::InteractionView;
#[allow(unused_imports)]
use semio_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
#[allow(unused_imports)]
use semio_plugin::AppOperationContext;
#[allow(unused_imports)]
use semio_plugin::ArtifactOwnedToolJobFactory;
#[allow(unused_imports)]
use semio_plugin::ArtifactOwnedToolJobRequest;
#[allow(unused_imports)]
use semio_plugin::ArtifactToolFactoryRegistry;
#[allow(unused_imports)]
use semio_plugin::ArtifactToolPublicationContract;
#[allow(unused_imports)]
use semio_plugin::ArtifactToolPublicationLane;
#[allow(unused_imports)]
use semio_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_plugin::ConfigView;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_plugin::DraftView;
#[allow(unused_imports)]
use semio_plugin::Emit;
#[allow(unused_imports)]
use semio_plugin::Fault;
#[allow(unused_imports)]
use semio_ui_locale::Label;
#[allow(unused_imports)]
use semio_plugin::NoConfig;
#[allow(unused_imports)]
use semio_plugin::NoConfigMutation;
#[allow(unused_imports)]
use semio_plugin::NoDraft;
#[allow(unused_imports)]
use semio_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_plugin::NoPresence;
#[allow(unused_imports)]
use semio_plugin::NoPresenceMutation;
#[allow(unused_imports)]
use semio_plugin::NoTransient;
#[allow(unused_imports)]
use semio_plugin::NoTransientMutation;
#[allow(unused_imports)]
use semio_framework_artifact_reference::StandardId;
#[allow(unused_imports)]
use semio_framework_artifact_reference::SubsetId;
#[allow(unused_imports)]
use semio_plugin::ToolExecutionContract;
#[allow(unused_imports)]
use semio_plugin::ToolFactoryKey;
#[allow(unused_imports)]
use semio_plugin::ToolJobFactory;
#[allow(unused_imports)]
use semio_plugin::ToolJobFactoryError;
#[allow(unused_imports)]
use semio_plugin::ToolOperationSpec;
#[allow(unused_imports)]
use semio_plugin::EditorApp;
#[allow(unused_imports)]
use semio_plugin::InteractiveJobClassification;
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
#[allow(unused_imports)]
use semio_s_artifact_stdio_contract::editing;
use crate::editor::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_semio_kit_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
    assert_eq!(def.dialect, SEMIO_KIT_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SemioKitEditor as ArtifactEditor>::DIALECT, SEMIO_KIT_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<SemioKitEditor, crate::viewer::SemioKitViewer>().await;
}

semio_plugin::history_edit_acceptance_law!("stdio/SemioKitEditor", SemioKitEditor, || semio_plugin::App { definition: create_semio_kit_editor(), examples: Vec::new() }, "../..");
