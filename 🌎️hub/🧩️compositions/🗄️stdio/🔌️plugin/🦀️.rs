//! 🧩️ Typed runtime assembly published from artifact-owned composition contributions.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The present owner-authored runtime app contributions.
    pub enum StdioApps: PluginApp {
        HtmlEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_html::editor::html::HtmlEditor>>),
        HtmlViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_html::viewer::html::HtmlViewer>>),
        MdEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_md::editor::md::MdEditor>>),
        MdViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_md::viewer::md::MdViewer>>),
        CsvEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_csv::editor::csv::CsvEditor>>),
        CsvViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_csv::viewer::csv::CsvViewer>>),
        TsvEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_tsv::editor::tsv::TsvEditor>>),
        TsvViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_tsv::viewer::tsv::TsvViewer>>),
        TxtEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_txt::editor::txt::TxtEditor>>),
        TxtViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_txt::viewer::txt::TxtViewer>>),
        JsonAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_json::editor::json_any::JsonAnyEditor>>),
        JsonAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_json::viewer::json_any::JsonAnyViewer>>),
        JsonIJsonEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_json::editor::json_i_json::JsonIJsonEditor>>),
        JsonIJsonViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_json::viewer::json_i_json::JsonIJsonViewer>>),
        XmlAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xml::editor::xml_any::XmlAnyEditor>>),
        XmlAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xml::viewer::xml_any::XmlAnyViewer>>),
        XmlValidEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xml::editor::xml_valid::XmlValidEditor>>),
        XmlValidViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xml::viewer::xml_valid::XmlValidViewer>>),
    }
}

/// 🔌️ Builds the exact present artifact and app contributions.
pub fn plugin() -> Result<Plugin<StdioApps>, PluginAssemblyError> {
    let mut builder = Plugin::<StdioApps>::builder("stdio").label("Stdio").version(env!("CARGO_PKG_VERSION")).package_id(crate::catalog::component_package_id()?).schema_documents("stdio", semio_s_artifact_stdio_contract::STDIO_REGISTRY_SCHEMA_DOCUMENTS).schema_documents("stdio", crate::catalog::CATALOG_SCHEMA_DOCUMENTS);
    let registry = crate::catalog::selected_registry()?;
    let plan = semio_s_plugin_stdio::AssemblyPlan::new(&registry, semio_s_plugin_stdio::AssemblyOwner { plugin_id: "stdio", package_id: crate::catalog::component_package_id()?, package_version: env!("CARGO_PKG_VERSION") })?;
    let catalog = crate::catalog::artifact_catalog_contribution(plan.assemblies())?;
    builder = plan.apply(builder);
    builder = builder.editor::<semio_s_artifact_stdio_html::editor::html::HtmlEditor>(semio_s_artifact_stdio_html::editor::html::create_html_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_html::viewer::html::HtmlViewer>(semio_s_artifact_stdio_html::viewer::html::create_html_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_md::editor::md::MdEditor>(semio_s_artifact_stdio_md::editor::md::create_md_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_md::viewer::md::MdViewer>(semio_s_artifact_stdio_md::viewer::md::create_md_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_csv::editor::csv::CsvEditor>(semio_s_artifact_stdio_csv::editor::csv::create_csv_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_csv::viewer::csv::CsvViewer>(semio_s_artifact_stdio_csv::viewer::csv::create_csv_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_tsv::editor::tsv::TsvEditor>(semio_s_artifact_stdio_tsv::editor::tsv::create_tsv_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_tsv::viewer::tsv::TsvViewer>(semio_s_artifact_stdio_tsv::viewer::tsv::create_tsv_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_txt::editor::txt::TxtEditor>(semio_s_artifact_stdio_txt::editor::txt::create_txt_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_txt::viewer::txt::TxtViewer>(semio_s_artifact_stdio_txt::viewer::txt::create_txt_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_json::editor::json_any::JsonAnyEditor>(semio_s_artifact_stdio_json::editor::json_any::create_json_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_json::viewer::json_any::JsonAnyViewer>(semio_s_artifact_stdio_json::viewer::json_any::create_json_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_json::editor::json_i_json::JsonIJsonEditor>(semio_s_artifact_stdio_json::editor::json_i_json::create_json_i_json_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_json::viewer::json_i_json::JsonIJsonViewer>(semio_s_artifact_stdio_json::viewer::json_i_json::create_json_i_json_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_xml::editor::xml_any::XmlAnyEditor>(semio_s_artifact_stdio_xml::editor::xml_any::create_xml_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_xml::viewer::xml_any::XmlAnyViewer>(semio_s_artifact_stdio_xml::viewer::xml_any::create_xml_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_xml::editor::xml_valid::XmlValidEditor>(semio_s_artifact_stdio_xml::editor::xml_valid::create_xml_valid_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_xml::viewer::xml_valid::XmlValidViewer>(semio_s_artifact_stdio_xml::viewer::xml_valid::create_xml_valid_viewer());
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_txt::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_xml::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_json::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_csv::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_md::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_tsv::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_html::artifact_kind().id });
    builder = builder.execution(ExecutionMode::Isolated).requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist contributed editor operations to the open document".into(), optional: false });
    builder.contributes_topic(catalog).try_build()
}
