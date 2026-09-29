//! 🧿️ `stdio-semio` — the nineteen semio v1 file apps as their own wasm component, over the artifact kinds and codecs the
//! `stdio` package owns.
//!
//! Every registered app monomorphises the whole app runtime and is live code inside its component, so one stdio
//! component cannot assemble all 176 stdio apps; each family ships its bounded fleet as its own package and depends on
//! `stdio` for the kinds it opens (`🗄️stdio/🧪️tests/🚢️shipped-fleet`: every stdio app is shipped by exactly one
//! package, every stdio kind is opened by exactly one package).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet of the semio editors and viewers — one editor and one viewer per subset.
    pub enum StdioSemioApps: PluginApp {
        SemioBrepEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_brep::SemioBrepEditor>>),
        SemioBrepViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_brep::SemioBrepViewer>>),
        SemioDrawingEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_drawing::SemioDrawingEditor>>),
        SemioDrawingViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_drawing::SemioDrawingViewer>>),
        SemioGraphEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_graph::SemioGraphEditor>>),
        SemioGraphViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_graph::SemioGraphViewer>>),
        SemioKitEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_kit::SemioKitEditor>>),
        SemioKitViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_kit::SemioKitViewer>>),
        SemioMeshEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_mesh::SemioMeshEditor>>),
        SemioMeshViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_mesh::SemioMeshViewer>>),
        SemioObjectEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_object::SemioObjectEditor>>),
        SemioObjectViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_object::SemioObjectViewer>>),
        SemioTableEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_table::SemioTableEditor>>),
        SemioTableViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_table::SemioTableViewer>>),
        SemioTextEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_text::SemioTextEditor>>),
        SemioTextViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_text::SemioTextViewer>>),
        SemioAnimationEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_animation::SemioAnimationEditor>>),
        SemioAnimationViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_animation::SemioAnimationViewer>>),
        SemioAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_base::SemioAnyEditor>>),
        SemioAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_base::SemioAnyViewer>>),
        SemioAudioEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_audio::SemioAudioEditor>>),
        SemioAudioViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_audio::SemioAudioViewer>>),
        SemioCadEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_cad::SemioCadEditor>>),
        SemioCadViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_cad::SemioCadViewer>>),
        SemioDocumentEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_document::SemioDocumentEditor>>),
        SemioDocumentViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_document::SemioDocumentViewer>>),
        SemioFlowEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_flow::SemioFlowEditor>>),
        SemioFlowViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_flow::SemioFlowViewer>>),
        SemioImageEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_image::SemioImageEditor>>),
        SemioImageViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_image::SemioImageViewer>>),
        SemioModelEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_model::SemioModelEditor>>),
        SemioModelViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_model::SemioModelViewer>>),
        SemioPresentationEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_presentation::SemioPresentationEditor>>),
        SemioPresentationViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_presentation::SemioPresentationViewer>>),
        SemioValueEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_value::SemioValueEditor>>),
        SemioValueViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_value::SemioValueViewer>>),
        SemioVideoEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_video::SemioVideoEditor>>),
        SemioVideoViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_video::SemioVideoViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the `stdio-semio` bundle: every semio subset's editor and viewer (with the owner-mutation roster where
/// the subset's mutation enum derives one), one activation per artifact kind it opens read live from that kind's own
/// `artifact_kind().id`, the exact-pin runtime dependency on `stdio`, which owns those kinds and their codecs, and the hosted runtime of
/// each of them (`host_artifact`: schemas, inferences, document codecs, composers, formats, subset validators) in this component.
pub fn plugin() -> Result<Plugin<StdioSemioApps>, PluginAssemblyError> {
    Plugin::<StdioSemioApps>::builder("stdio-semio")
        .label("Stdio Semio")
        .version(env!("CARGO_PKG_VERSION"))
        .package_id("semio:stdio-semio")
        .depends_on("stdio", semio_framework::tree_pin!())
        .host_artifact(semio_s_artifact_stdio_semio::declaration(semio_s_artifact_stdio_semio::definition()?).map_err(PluginAssemblyError::definition)?)
        .editor::<semio_s_artifact_stdio_semio::editor::semio_brep::SemioBrepEditor>(semio_s_artifact_stdio_semio::editor::semio_brep::create_semio_brep_editor())
        .editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_brep::SemioBrepEditor>()
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_brep::SemioBrepViewer>(semio_s_artifact_stdio_semio::viewer::semio_brep::create_semio_brep_viewer())
        .viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_brep::SemioBrepViewer>()
        .editor::<semio_s_artifact_stdio_semio::editor::semio_drawing::SemioDrawingEditor>(semio_s_artifact_stdio_semio::editor::semio_drawing::create_semio_drawing_editor())
        .editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_drawing::SemioDrawingEditor>()
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_drawing::SemioDrawingViewer>(semio_s_artifact_stdio_semio::viewer::semio_drawing::create_semio_drawing_viewer())
        .viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_drawing::SemioDrawingViewer>()
        .editor::<semio_s_artifact_stdio_semio::editor::semio_graph::SemioGraphEditor>(semio_s_artifact_stdio_semio::editor::semio_graph::create_semio_graph_editor())
        .editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_graph::SemioGraphEditor>()
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_graph::SemioGraphViewer>(semio_s_artifact_stdio_semio::viewer::semio_graph::create_semio_graph_viewer())
        .viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_graph::SemioGraphViewer>()
        .editor::<semio_s_artifact_stdio_semio::editor::semio_kit::SemioKitEditor>(semio_s_artifact_stdio_semio::editor::semio_kit::create_semio_kit_editor())
        .editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_kit::SemioKitEditor>()
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_kit::SemioKitViewer>(semio_s_artifact_stdio_semio::viewer::semio_kit::create_semio_kit_viewer())
        .viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_kit::SemioKitViewer>()
        .editor::<semio_s_artifact_stdio_semio::editor::semio_mesh::SemioMeshEditor>(semio_s_artifact_stdio_semio::editor::semio_mesh::create_semio_mesh_editor())
        .editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_mesh::SemioMeshEditor>()
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_mesh::SemioMeshViewer>(semio_s_artifact_stdio_semio::viewer::semio_mesh::create_semio_mesh_viewer())
        .viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_mesh::SemioMeshViewer>()
        .editor::<semio_s_artifact_stdio_semio::editor::semio_object::SemioObjectEditor>(semio_s_artifact_stdio_semio::editor::semio_object::create_semio_object_editor())
        .editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_object::SemioObjectEditor>()
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_object::SemioObjectViewer>(semio_s_artifact_stdio_semio::viewer::semio_object::create_semio_object_viewer())
        .viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_object::SemioObjectViewer>()
        .editor::<semio_s_artifact_stdio_semio::editor::semio_table::SemioTableEditor>(semio_s_artifact_stdio_semio::editor::semio_table::create_semio_table_editor())
        .editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_table::SemioTableEditor>()
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_table::SemioTableViewer>(semio_s_artifact_stdio_semio::viewer::semio_table::create_semio_table_viewer())
        .viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_table::SemioTableViewer>()
        .editor::<semio_s_artifact_stdio_semio::editor::semio_text::SemioTextEditor>(semio_s_artifact_stdio_semio::editor::semio_text::create_semio_text_editor())
        .editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_text::SemioTextEditor>()
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_text::SemioTextViewer>(semio_s_artifact_stdio_semio::viewer::semio_text::create_semio_text_viewer())
        .viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_text::SemioTextViewer>()
        .editor::<semio_s_artifact_stdio_semio::editor::semio_animation::SemioAnimationEditor>(semio_s_artifact_stdio_semio::editor::semio_animation::create_semio_animation_editor())
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_animation::SemioAnimationViewer>(semio_s_artifact_stdio_semio::viewer::semio_animation::create_semio_animation_viewer())
        .editor::<semio_s_artifact_stdio_semio::editor::semio_base::SemioAnyEditor>(semio_s_artifact_stdio_semio::editor::semio_base::create_semio_base_editor())
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_base::SemioAnyViewer>(semio_s_artifact_stdio_semio::viewer::semio_base::create_semio_base_viewer())
        .editor::<semio_s_artifact_stdio_semio::editor::semio_audio::SemioAudioEditor>(semio_s_artifact_stdio_semio::editor::semio_audio::create_semio_audio_editor())
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_audio::SemioAudioViewer>(semio_s_artifact_stdio_semio::viewer::semio_audio::create_semio_audio_viewer())
        .editor::<semio_s_artifact_stdio_semio::editor::semio_cad::SemioCadEditor>(semio_s_artifact_stdio_semio::editor::semio_cad::create_semio_cad_editor())
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_cad::SemioCadViewer>(semio_s_artifact_stdio_semio::viewer::semio_cad::create_semio_cad_viewer())
        .editor::<semio_s_artifact_stdio_semio::editor::semio_document::SemioDocumentEditor>(semio_s_artifact_stdio_semio::editor::semio_document::create_semio_document_editor())
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_document::SemioDocumentViewer>(semio_s_artifact_stdio_semio::viewer::semio_document::create_semio_document_viewer())
        .editor::<semio_s_artifact_stdio_semio::editor::semio_flow::SemioFlowEditor>(semio_s_artifact_stdio_semio::editor::semio_flow::create_semio_flow_editor())
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_flow::SemioFlowViewer>(semio_s_artifact_stdio_semio::viewer::semio_flow::create_semio_flow_viewer())
        .editor::<semio_s_artifact_stdio_semio::editor::semio_image::SemioImageEditor>(semio_s_artifact_stdio_semio::editor::semio_image::create_semio_image_editor())
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_image::SemioImageViewer>(semio_s_artifact_stdio_semio::viewer::semio_image::create_semio_image_viewer())
        .editor::<semio_s_artifact_stdio_semio::editor::semio_model::SemioModelEditor>(semio_s_artifact_stdio_semio::editor::semio_model::create_semio_model_editor())
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_model::SemioModelViewer>(semio_s_artifact_stdio_semio::viewer::semio_model::create_semio_model_viewer())
        .editor::<semio_s_artifact_stdio_semio::editor::semio_presentation::SemioPresentationEditor>(semio_s_artifact_stdio_semio::editor::semio_presentation::create_semio_presentation_editor())
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_presentation::SemioPresentationViewer>(semio_s_artifact_stdio_semio::viewer::semio_presentation::create_semio_presentation_viewer())
        .editor::<semio_s_artifact_stdio_semio::editor::semio_value::SemioValueEditor>(semio_s_artifact_stdio_semio::editor::semio_value::create_semio_value_editor())
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_value::SemioValueViewer>(semio_s_artifact_stdio_semio::viewer::semio_value::create_semio_value_viewer())
        .editor::<semio_s_artifact_stdio_semio::editor::semio_video::SemioVideoEditor>(semio_s_artifact_stdio_semio::editor::semio_video::create_semio_video_editor())
        .viewer::<semio_s_artifact_stdio_semio::viewer::semio_video::SemioVideoViewer>(semio_s_artifact_stdio_semio::viewer::semio_video::create_semio_video_viewer())
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_semio::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("artifacts.write".into()),
            scope: "plugin".into(),
            reason: "persist semio editor edits back to the open stdio document".into(),
            optional: false,
        })
        .try_build()
}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, StdioSemioApps);
