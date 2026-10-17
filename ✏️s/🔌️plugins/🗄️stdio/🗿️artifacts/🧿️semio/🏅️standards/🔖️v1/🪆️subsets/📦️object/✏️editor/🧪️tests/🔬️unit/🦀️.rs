use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::Editor;
#[allow(unused_imports)]
use crate::editor::modes::edit;
#[allow(unused_imports)]
use crate::editor::modes::edit::windows::main;
#[allow(unused_imports)]
use crate::schema::mutations::SemioObjectMutation;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioObjectSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::app::InteractionView;
#[allow(unused_imports)]
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
#[allow(unused_imports)]
use semio_framework_plugin::AppOperationContext;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolFactoryRegistry;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationContract;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactToolPublicationLane;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::DraftView;
#[allow(unused_imports)]
use semio_framework_plugin::Emit;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfig;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfigMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraft;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoPresence;
#[allow(unused_imports)]
use semio_framework_plugin::NoPresenceMutation;
#[allow(unused_imports)]
use semio_framework_plugin::NoTransient;
#[allow(unused_imports)]
use semio_framework_plugin::NoTransientMutation;
#[allow(unused_imports)]
use semio_framework_artifact_reference::StandardId;
#[allow(unused_imports)]
use semio_framework_artifact_reference::SubsetId;
#[allow(unused_imports)]
use semio_framework_plugin::ToolExecutionContract;
#[allow(unused_imports)]
use semio_framework_plugin::ToolFactoryKey;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactory;
#[allow(unused_imports)]
use semio_framework_plugin::ToolJobFactoryError;
#[allow(unused_imports)]
use semio_framework_plugin::ToolOperationSpec;
#[allow(unused_imports)]
use semio_framework_plugin::EditorApp;
#[allow(unused_imports)]
use semio_framework_plugin::InteractiveJobClassification;
#[allow(unused_imports)]
use semio_framework_2d::compute::EngineHandles;
#[allow(unused_imports)]
use semio_s_artifact_stdio_contract::editing;
use crate::editor::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_semio_object_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, SEMIO_OBJECT_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SemioObjectEditor as ArtifactEditor>::DIALECT, SEMIO_OBJECT_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<SemioObjectEditor, crate::viewer::SemioObjectViewer>().await;
}

semio_framework_plugin::history_edit_acceptance_law!("stdio/SemioObjectEditor", SemioObjectEditor, || semio_framework_plugin::App { definition: create_semio_object_editor(), examples: Vec::new() }, "../..");
