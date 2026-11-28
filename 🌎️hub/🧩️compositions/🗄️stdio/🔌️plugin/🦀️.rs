//! 🧩️ Typed runtime assembly published from artifact-owned composition contributions.

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The present owner-authored runtime app contributions.
    pub enum StdioApps: PluginApp {
        StdioHtmlApps(semio_hub_stdio_html_apps::StdioHtmlApps),
        StdioMarkdownApps(semio_hub_stdio_md_apps::StdioMarkdownApps),
        StdioCsvApps(semio_hub_stdio_csv_apps::StdioCsvApps),
        StdioTsvApps(semio_hub_stdio_tsv_apps::StdioTsvApps),
        StdioTextApps(semio_hub_stdio_txt_apps::StdioTextApps),
        StdioJsonApps(semio_hub_stdio_json_any_apps::StdioJsonApps),
        StdioIJsonApps(semio_hub_stdio_json_i_json_apps::StdioIJsonApps),
        StdioXmlApps(semio_hub_stdio_xml_any_apps::StdioXmlApps),
        StdioValidXmlApps(semio_hub_stdio_xml_valid_apps::StdioValidXmlApps),
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_html::editor::html::HtmlEditor>>> for StdioApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_html::editor::html::HtmlEditor>>) -> Self {
        Self::StdioHtmlApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_html::viewer::html::HtmlViewer>>> for StdioApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_html::viewer::html::HtmlViewer>>) -> Self {
        Self::StdioHtmlApps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_md::editor::md::MdEditor>>> for StdioApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_md::editor::md::MdEditor>>) -> Self {
        Self::StdioMarkdownApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_md::viewer::md::MdViewer>>> for StdioApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_md::viewer::md::MdViewer>>) -> Self {
        Self::StdioMarkdownApps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_csv::editor::csv::CsvEditor>>> for StdioApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_csv::editor::csv::CsvEditor>>) -> Self {
        Self::StdioCsvApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_csv::viewer::csv::CsvViewer>>> for StdioApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_csv::viewer::csv::CsvViewer>>) -> Self {
        Self::StdioCsvApps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_tsv::editor::tsv::TsvEditor>>> for StdioApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_tsv::editor::tsv::TsvEditor>>) -> Self {
        Self::StdioTsvApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_tsv::viewer::tsv::TsvViewer>>> for StdioApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_tsv::viewer::tsv::TsvViewer>>) -> Self {
        Self::StdioTsvApps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_txt::editor::txt::TxtEditor>>> for StdioApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_txt::editor::txt::TxtEditor>>) -> Self {
        Self::StdioTextApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_txt::viewer::txt::TxtViewer>>> for StdioApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_txt::viewer::txt::TxtViewer>>) -> Self {
        Self::StdioTextApps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_json::editor::json_any::JsonAnyEditor>>> for StdioApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_json::editor::json_any::JsonAnyEditor>>) -> Self {
        Self::StdioJsonApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_json::viewer::json_any::JsonAnyViewer>>> for StdioApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_json::viewer::json_any::JsonAnyViewer>>) -> Self {
        Self::StdioJsonApps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_json::editor::json_i_json::JsonIJsonEditor>>> for StdioApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_json::editor::json_i_json::JsonIJsonEditor>>) -> Self {
        Self::StdioIJsonApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_json::viewer::json_i_json::JsonIJsonViewer>>> for StdioApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_json::viewer::json_i_json::JsonIJsonViewer>>) -> Self {
        Self::StdioIJsonApps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xml::editor::xml_any::XmlAnyEditor>>> for StdioApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xml::editor::xml_any::XmlAnyEditor>>) -> Self {
        Self::StdioXmlApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xml::viewer::xml_any::XmlAnyViewer>>> for StdioApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xml::viewer::xml_any::XmlAnyViewer>>) -> Self {
        Self::StdioXmlApps(app.into())
    }
}

impl From<VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xml::editor::xml_valid::XmlValidEditor>>> for StdioApps {
    fn from(app: VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xml::editor::xml_valid::XmlValidEditor>>) -> Self {
        Self::StdioValidXmlApps(app.into())
    }
}

impl From<VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xml::viewer::xml_valid::XmlValidViewer>>> for StdioApps {
    fn from(app: VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xml::viewer::xml_valid::XmlValidViewer>>) -> Self {
        Self::StdioValidXmlApps(app.into())
    }
}

/// 🔌️ Builds the exact present artifact and app contributions.
pub fn plugin() -> Result<Plugin<StdioApps>, PluginAssemblyError> {
    let mut builder = semio_plugin::PluginBuilder::<semio_plugin::NeedsLabel, StdioApps>::new("stdio").label(semio_plugin::LocalizedLabel::native("Stdio", "Stdio")).version(env!("CARGO_PKG_VERSION")).package_id(semio_hub_stdio_catalog::component_package_id()?).schema_documents("stdio", semio_s_artifact_stdio_contract::STDIO_REGISTRY_SCHEMA_DOCUMENTS).schema_documents("stdio", semio_hub_stdio_catalog::CATALOG_SCHEMA_DOCUMENTS);
    let registry = semio_hub_stdio_catalog::selected_registry()?;
    let plan = semio_s_plugin_stdio::AssemblyPlan::new(&registry, semio_s_plugin_stdio::AssemblyOwner { plugin_id: "stdio", package_id: semio_hub_stdio_catalog::component_package_id()?, package_version: env!("CARGO_PKG_VERSION") })?;
    let catalog = semio_hub_stdio_catalog::artifact_catalog_contribution(plan.assemblies())?;
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
