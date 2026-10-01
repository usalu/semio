//! 🧩️ Typed runtime assembly published from artifact-owned composition contributions.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ The present owner-authored runtime app contributions.
    pub enum StdioImageApps: PluginApp {
        PngEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_png::editor::png::PngEditor>>),
        PngViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_png::viewer::png::PngViewer>>),
        JpgAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_jpg::editor::jpg_any::JpgAnyEditor>>),
        JpgAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_jpg::viewer::jpg_any::JpgAnyViewer>>),
        JpgBaselineEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_jpg::editor::jpg_baseline::JpgBaselineEditor>>),
        JpgBaselineViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_jpg::viewer::jpg_baseline::JpgBaselineViewer>>),
        BmpEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_bmp::editor::bmp::BmpEditor>>),
        BmpViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_bmp::viewer::bmp::BmpViewer>>),
        TiffAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_tiff::editor::tiff_any::TiffAnyEditor>>),
        TiffAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_tiff::viewer::tiff_any::TiffAnyViewer>>),
        TiffBaselineEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_tiff::editor::tiff_baseline::TiffBaselineEditor>>),
        TiffBaselineViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_tiff::viewer::tiff_baseline::TiffBaselineViewer>>),
        Gif87aEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_gif::editor::gif_87a::Gif87aEditor>>),
        Gif87aViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_gif::viewer::gif_87a::Gif87aViewer>>),
        Gif89aEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_gif::editor::gif_89a::Gif89aEditor>>),
        Gif89aViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_gif::viewer::gif_89a::Gif89aViewer>>),
        SvgAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_svg::editor::svg_any::SvgAnyEditor>>),
        SvgAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_svg::viewer::svg_any::SvgAnyViewer>>),
        SvgBasicEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_svg::editor::svg_basic::SvgBasicEditor>>),
        SvgBasicViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_svg::viewer::svg_basic::SvgBasicViewer>>),
        SvgTinyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_svg::editor::svg_tiny::SvgTinyEditor>>),
        SvgTinyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_svg::viewer::svg_tiny::SvgTinyViewer>>),
    }
}

/// 🔌️ Builds the exact present artifact and app contributions.
pub fn plugin() -> Result<Plugin<StdioImageApps>, PluginAssemblyError> {
    let mut builder = Plugin::<StdioImageApps>::builder("stdio-image").label("Stdio Image").version(env!("CARGO_PKG_VERSION")).package_id("semio:stdio-image").schema_documents("stdio", semio_s_artifact_stdio_contract::STDIO_REGISTRY_SCHEMA_DOCUMENTS).depends_on("stdio", semio_framework::tree_pin!());
    builder = builder.host_artifact(semio_s_artifact_stdio_svg::declaration(semio_s_artifact_stdio_svg::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_png::declaration(semio_s_artifact_stdio_png::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_jpg::declaration(semio_s_artifact_stdio_jpg::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_gif::declaration(semio_s_artifact_stdio_gif::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_bmp::declaration(semio_s_artifact_stdio_bmp::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.host_artifact(semio_s_artifact_stdio_tiff::declaration(semio_s_artifact_stdio_tiff::definition()?).map_err(PluginAssemblyError::definition)?);
    builder = builder.editor::<semio_s_artifact_stdio_png::editor::png::PngEditor>(semio_s_artifact_stdio_png::editor::png::create_png_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_png::viewer::png::PngViewer>(semio_s_artifact_stdio_png::viewer::png::create_png_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_jpg::editor::jpg_any::JpgAnyEditor>(semio_s_artifact_stdio_jpg::editor::jpg_any::create_jpg_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_jpg::viewer::jpg_any::JpgAnyViewer>(semio_s_artifact_stdio_jpg::viewer::jpg_any::create_jpg_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_jpg::editor::jpg_baseline::JpgBaselineEditor>(semio_s_artifact_stdio_jpg::editor::jpg_baseline::create_jpg_baseline_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_jpg::viewer::jpg_baseline::JpgBaselineViewer>(semio_s_artifact_stdio_jpg::viewer::jpg_baseline::create_jpg_baseline_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_bmp::editor::bmp::BmpEditor>(semio_s_artifact_stdio_bmp::editor::bmp::create_bmp_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_bmp::viewer::bmp::BmpViewer>(semio_s_artifact_stdio_bmp::viewer::bmp::create_bmp_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_tiff::editor::tiff_any::TiffAnyEditor>(semio_s_artifact_stdio_tiff::editor::tiff_any::create_tiff_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_tiff::viewer::tiff_any::TiffAnyViewer>(semio_s_artifact_stdio_tiff::viewer::tiff_any::create_tiff_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_tiff::editor::tiff_baseline::TiffBaselineEditor>(semio_s_artifact_stdio_tiff::editor::tiff_baseline::create_tiff_baseline_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_tiff::viewer::tiff_baseline::TiffBaselineViewer>(semio_s_artifact_stdio_tiff::viewer::tiff_baseline::create_tiff_baseline_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_gif::editor::gif_87a::Gif87aEditor>(semio_s_artifact_stdio_gif::editor::gif_87a::create_gif_87a_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_gif::viewer::gif_87a::Gif87aViewer>(semio_s_artifact_stdio_gif::viewer::gif_87a::create_gif_87a_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_gif::editor::gif_89a::Gif89aEditor>(semio_s_artifact_stdio_gif::editor::gif_89a::create_gif_89a_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_gif::viewer::gif_89a::Gif89aViewer>(semio_s_artifact_stdio_gif::viewer::gif_89a::create_gif_89a_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_svg::editor::svg_any::SvgAnyEditor>(semio_s_artifact_stdio_svg::editor::svg_any::create_svg_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_svg::viewer::svg_any::SvgAnyViewer>(semio_s_artifact_stdio_svg::viewer::svg_any::create_svg_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_svg::editor::svg_basic::SvgBasicEditor>(semio_s_artifact_stdio_svg::editor::svg_basic::create_svg_basic_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_svg::viewer::svg_basic::SvgBasicViewer>(semio_s_artifact_stdio_svg::viewer::svg_basic::create_svg_basic_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_svg::editor::svg_tiny::SvgTinyEditor>(semio_s_artifact_stdio_svg::editor::svg_tiny::create_svg_tiny_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_svg::viewer::svg_tiny::SvgTinyViewer>(semio_s_artifact_stdio_svg::viewer::svg_tiny::create_svg_tiny_viewer());
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_svg::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_png::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_jpg::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_gif::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_bmp::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_tiff::artifact_kind().id });
    builder = builder.execution(ExecutionMode::Isolated).requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist contributed editor operations to the open document".into(), optional: false });
    builder.try_build()
}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, StdioImageApps);
