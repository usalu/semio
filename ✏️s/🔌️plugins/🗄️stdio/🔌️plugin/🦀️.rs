//! 🔌️ Schema-owned stdio library plugin assembly.

use semio_framework_dispatch_macros::dyn_enum_close;
use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet of the stdio component: the nine text/data document subsets, 18 apps. Every app
    /// monomorphises the whole app runtime and is live code inside its component, so the other 79 subsets ship in their
    /// family components (`🧩️extensions/*`, whole artifact kinds each) — see `🧪️tests/🚢️shipped-fleet`.
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

/// 🧾️ Builds all stdio definitions before the typed library assembly boundary: stdio declares and codec-owns all 36
/// well-known file-format artifact kinds, but activates only on the seven kinds its own apps open (csv, tsv, txt, json,
/// xml, md, html) — every other kind activates the family package that ships its editors and viewers and depends on
/// `stdio`, so exactly one catalog row claims each kind (`📓️design-abi.md` §2/§3). The actor runs `Isolated` and asks
/// the broker for document write access, because its editors persist mutations back to the open document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plugin() -> Result<Plugin<StdioApps>, PluginAssemblyError> {
    let mut builder = Plugin::builder("stdio").label("Stdio").version("0.1.0").package_id(crate::registry::component_package_id()?);
    let assemblies = crate::registry::artifact_assemblies()?;
    let catalog = crate::registry::artifact_catalog_contribution(&assemblies)?;
    for assembly in assemblies {
        builder = match assembly {
            crate::registry::ArtifactAssembly::Definition(definition) => builder.artifact_definition(definition),
            crate::registry::ArtifactAssembly::Runtime(declaration) => builder.artifact(*declaration),
        };
    }
    for artifact_kind in crate::registry::native_codec_artifact_kinds() {
        builder = builder.artifact_kind(artifact_kind);
    }
    builder = register_apps(builder);

    //#region 🔖️Descriptor
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_txt::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_json::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_xml::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_csv::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_md::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_tsv::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_html::artifact_kind().id });
    builder = builder.execution(ExecutionMode::Isolated);
    builder = builder.requests(CapabilityRequest {
        id: CapabilityId("artifacts.write".into()),
        scope: "plugin".into(),
        reason: "persist csv/tsv/txt/json/xml/md/html editor edits back to the open stdio document".into(),
        optional: false,
    });
    //#endregion 🔖️Descriptor

    builder.contributes_topic(catalog).try_library()
}

/// 📄️ Registers the stdio component fleet: csv/tsv/txt/json(any, i-json)/xml(any, valid)/md/html — the nine text and
/// data document subsets, 18 apps; the plugin still declares and codec-owns all 36 stdio artifact kinds.
fn register_apps(mut builder: PluginBuilder<Ready, StdioApps>) -> PluginBuilder<Ready, StdioApps> {
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
    builder
}
