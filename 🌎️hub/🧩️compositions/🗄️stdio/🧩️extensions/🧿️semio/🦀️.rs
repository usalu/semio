//! 🧩️ Typed runtime assembly published from artifact-owned composition contributions.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The present owner-authored runtime app contributions.
    pub enum StdioSemioApps: PluginApp {
        SemioBrepEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_brep::editor::SemioBrepEditor>>),
        SemioBrepViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_brep::viewer::SemioBrepViewer>>),
        SemioDrawingEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_drawing::editor::SemioDrawingEditor>>),
        SemioDrawingViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_drawing::viewer::SemioDrawingViewer>>),
        SemioGraphEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_graph::editor::SemioGraphEditor>>),
        SemioGraphViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_graph::viewer::SemioGraphViewer>>),
        SemioKitEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_kit::editor::SemioKitEditor>>),
        SemioKitViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_kit::viewer::SemioKitViewer>>),
        SemioMeshEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_mesh::editor::SemioMeshEditor>>),
        SemioMeshViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_mesh::viewer::SemioMeshViewer>>),
        SemioObjectEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_object::editor::SemioObjectEditor>>),
        SemioObjectViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_object::viewer::SemioObjectViewer>>),
        SemioTableEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_table::editor::SemioTableEditor>>),
        SemioTableViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_table::viewer::SemioTableViewer>>),
        SemioTextEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_text::editor::SemioTextEditor>>),
        SemioTextViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_text::viewer::SemioTextViewer>>),
        SemioAnimationEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_animation::editor::SemioAnimationEditor>>),
        SemioAnimationViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_animation::viewer::SemioAnimationViewer>>),
        SemioAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::SemioAnyEditor>>),
        SemioAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::SemioAnyViewer>>),
        SemioAudioEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_audio::editor::SemioAudioEditor>>),
        SemioAudioViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_audio::viewer::SemioAudioViewer>>),
        SemioCadEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_cad::editor::SemioCadEditor>>),
        SemioCadViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_cad::viewer::SemioCadViewer>>),
        SemioDocumentEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_document::editor::SemioDocumentEditor>>),
        SemioDocumentViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_document::viewer::SemioDocumentViewer>>),
        SemioFlowEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_flow::editor::SemioFlowEditor>>),
        SemioFlowViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_flow::viewer::SemioFlowViewer>>),
        SemioImageEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_image::editor::SemioImageEditor>>),
        SemioImageViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_image::viewer::SemioImageViewer>>),
        SemioModelEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_model::editor::SemioModelEditor>>),
        SemioModelViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_model::viewer::SemioModelViewer>>),
        SemioPresentationEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_presentation::editor::SemioPresentationEditor>>),
        SemioPresentationViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_presentation::viewer::SemioPresentationViewer>>),
        SemioValueEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_value::editor::SemioValueEditor>>),
        SemioValueViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_value::viewer::SemioValueViewer>>),
        SemioVideoEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio_video::editor::SemioVideoEditor>>),
        SemioVideoViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio_video::viewer::SemioVideoViewer>>),
    }
}

/// 🔌️ Builds the exact present artifact and app contributions.
pub fn plugin() -> Result<Plugin<StdioSemioApps>, PluginAssemblyError> {
    let mut builder = Plugin::<StdioSemioApps>::builder("stdio-semio").label("Stdio Semio").version(env!("CARGO_PKG_VERSION")).package_id("semio:stdio-semio").schema_documents("stdio", semio_s_artifact_stdio_contract::STDIO_REGISTRY_SCHEMA_DOCUMENTS).depends_on("stdio", semio_framework::tree_pin!());
    builder = builder.host_artifact(semio_s_artifact_stdio_semio_catalog::declaration(semio_s_artifact_stdio_semio::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.editor::<semio_s_artifact_stdio_semio_brep::editor::SemioBrepEditor>(semio_s_artifact_stdio_semio_brep::editor::create_semio_brep_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio_brep::editor::SemioBrepEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio_brep::viewer::SemioBrepViewer>(semio_s_artifact_stdio_semio_brep::viewer::create_semio_brep_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio_brep::viewer::SemioBrepViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio_drawing::editor::SemioDrawingEditor>(semio_s_artifact_stdio_semio_drawing::editor::create_semio_drawing_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio_drawing::editor::SemioDrawingEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio_drawing::viewer::SemioDrawingViewer>(semio_s_artifact_stdio_semio_drawing::viewer::create_semio_drawing_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio_drawing::viewer::SemioDrawingViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio_graph::editor::SemioGraphEditor>(semio_s_artifact_stdio_semio_graph::editor::create_semio_graph_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio_graph::editor::SemioGraphEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio_graph::viewer::SemioGraphViewer>(semio_s_artifact_stdio_semio_graph::viewer::create_semio_graph_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio_graph::viewer::SemioGraphViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio_kit::editor::SemioKitEditor>(semio_s_artifact_stdio_semio_kit::editor::create_semio_kit_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio_kit::editor::SemioKitEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio_kit::viewer::SemioKitViewer>(semio_s_artifact_stdio_semio_kit::viewer::create_semio_kit_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio_kit::viewer::SemioKitViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio_mesh::editor::SemioMeshEditor>(semio_s_artifact_stdio_semio_mesh::editor::create_semio_mesh_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio_mesh::editor::SemioMeshEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio_mesh::viewer::SemioMeshViewer>(semio_s_artifact_stdio_semio_mesh::viewer::create_semio_mesh_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio_mesh::viewer::SemioMeshViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio_object::editor::SemioObjectEditor>(semio_s_artifact_stdio_semio_object::editor::create_semio_object_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio_object::editor::SemioObjectEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio_object::viewer::SemioObjectViewer>(semio_s_artifact_stdio_semio_object::viewer::create_semio_object_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio_object::viewer::SemioObjectViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio_table::editor::SemioTableEditor>(semio_s_artifact_stdio_semio_table::editor::create_semio_table_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio_table::editor::SemioTableEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio_table::viewer::SemioTableViewer>(semio_s_artifact_stdio_semio_table::viewer::create_semio_table_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio_table::viewer::SemioTableViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio_text::editor::SemioTextEditor>(semio_s_artifact_stdio_semio_text::editor::create_semio_text_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio_text::editor::SemioTextEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio_text::viewer::SemioTextViewer>(semio_s_artifact_stdio_semio_text::viewer::create_semio_text_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio_text::viewer::SemioTextViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio_animation::editor::SemioAnimationEditor>(semio_s_artifact_stdio_semio_animation::editor::create_semio_animation_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio_animation::viewer::SemioAnimationViewer>(semio_s_artifact_stdio_semio_animation::viewer::create_semio_animation_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::SemioAnyEditor>(semio_s_artifact_stdio_semio::editor::create_semio_base_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::SemioAnyViewer>(semio_s_artifact_stdio_semio::viewer::create_semio_base_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio_audio::editor::SemioAudioEditor>(semio_s_artifact_stdio_semio_audio::editor::create_semio_audio_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio_audio::viewer::SemioAudioViewer>(semio_s_artifact_stdio_semio_audio::viewer::create_semio_audio_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio_cad::editor::SemioCadEditor>(semio_s_artifact_stdio_semio_cad::editor::create_semio_cad_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio_cad::viewer::SemioCadViewer>(semio_s_artifact_stdio_semio_cad::viewer::create_semio_cad_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio_document::editor::SemioDocumentEditor>(semio_s_artifact_stdio_semio_document::editor::create_semio_document_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio_document::viewer::SemioDocumentViewer>(semio_s_artifact_stdio_semio_document::viewer::create_semio_document_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio_flow::editor::SemioFlowEditor>(semio_s_artifact_stdio_semio_flow::editor::create_semio_flow_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio_flow::viewer::SemioFlowViewer>(semio_s_artifact_stdio_semio_flow::viewer::create_semio_flow_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio_image::editor::SemioImageEditor>(semio_s_artifact_stdio_semio_image::editor::create_semio_image_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio_image::viewer::SemioImageViewer>(semio_s_artifact_stdio_semio_image::viewer::create_semio_image_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio_model::editor::SemioModelEditor>(semio_s_artifact_stdio_semio_model::editor::create_semio_model_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio_model::viewer::SemioModelViewer>(semio_s_artifact_stdio_semio_model::viewer::create_semio_model_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio_presentation::editor::SemioPresentationEditor>(semio_s_artifact_stdio_semio_presentation::editor::create_semio_presentation_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio_presentation::viewer::SemioPresentationViewer>(semio_s_artifact_stdio_semio_presentation::viewer::create_semio_presentation_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio_value::editor::SemioValueEditor>(semio_s_artifact_stdio_semio_value::editor::create_semio_value_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio_value::viewer::SemioValueViewer>(semio_s_artifact_stdio_semio_value::viewer::create_semio_value_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio_video::editor::SemioVideoEditor>(semio_s_artifact_stdio_semio_video::editor::create_semio_video_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio_video::viewer::SemioVideoViewer>(semio_s_artifact_stdio_semio_video::viewer::create_semio_video_viewer());
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_semio::artifact_kind().id });
    builder = builder.execution(ExecutionMode::Isolated).requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist contributed editor operations to the open document".into(), optional: false });
    builder.try_build()
}
