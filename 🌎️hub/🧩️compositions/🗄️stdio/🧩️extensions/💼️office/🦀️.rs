//! 🧩️ Typed runtime assembly published from artifact-owned composition contributions.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The present owner-authored runtime app contributions.
    pub enum StdioOfficeApps: PluginApp {
        DocxEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::base::DocxEditor>>),
        DocxViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::base::DocxViewer>>),
        DocxStrictEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::strict::DocxStrictEditor>>),
        DocxStrictViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::strict::DocxStrictViewer>>),
        DocxTransitionalEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalEditor>>),
        DocxTransitionalViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalViewer>>),
        PptxEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::base::PptxEditor>>),
        PptxViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::base::PptxViewer>>),
        PptxStrictEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::strict::PptxStrictEditor>>),
        PptxStrictViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::strict::PptxStrictViewer>>),
        PptxTransitionalEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalEditor>>),
        PptxTransitionalViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalViewer>>),
        XlsxEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::base::XlsxEditor>>),
        XlsxViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::base::XlsxViewer>>),
        XlsxStrictEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictEditor>>),
        XlsxStrictViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictViewer>>),
        XlsxTransitionalEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalEditor>>),
        XlsxTransitionalViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalViewer>>),
    }
}

/// 🔌️ Builds the exact present artifact and app contributions.
pub fn plugin() -> Result<Plugin<StdioOfficeApps>, PluginAssemblyError> {
    let mut builder = Plugin::<StdioOfficeApps>::builder("stdio-office").label("Stdio Office").version(env!("CARGO_PKG_VERSION")).package_id("semio:stdio-office").schema_documents("stdio", semio_s_artifact_stdio_contract::STDIO_REGISTRY_SCHEMA_DOCUMENTS).depends_on("stdio", semio_framework::tree_pin!());
    builder = builder.host_artifact(semio_s_artifact_stdio_docx::declaration(semio_s_artifact_stdio_docx::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_pptx::declaration(semio_s_artifact_stdio_pptx::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_xlsx::declaration(semio_s_artifact_stdio_xlsx::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.editor::<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::base::DocxEditor>(semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::base::create_docx_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::base::DocxViewer>(semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::base::create_docx_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::strict::DocxStrictEditor>(semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::strict::create_docx_strict_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::strict::DocxStrictViewer>(semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::strict::create_docx_strict_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalEditor>(semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::transitional::create_docx_transitional_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalViewer>(semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::transitional::create_docx_transitional_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::base::PptxEditor>(semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::base::create_pptx_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::base::PptxViewer>(semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::base::create_pptx_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::strict::PptxStrictEditor>(semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::strict::create_pptx_strict_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::strict::PptxStrictViewer>(semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::strict::create_pptx_strict_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalEditor>(semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::transitional::create_pptx_transitional_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalViewer>(semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::transitional::create_pptx_transitional_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::base::XlsxEditor>(semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::base::create_xlsx_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::base::XlsxViewer>(semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::base::create_xlsx_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictEditor>(semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::strict::create_xlsx_strict_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictViewer>(semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::strict::create_xlsx_strict_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalEditor>(semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::transitional::create_xlsx_transitional_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalViewer>(semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::transitional::create_xlsx_transitional_viewer());
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_docx::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_pptx::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_xlsx::artifact_kind().id });
    builder = builder.execution(ExecutionMode::Isolated).requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist contributed editor operations to the open document".into(), optional: false });
    builder.try_build()
}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, StdioOfficeApps);
