//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_plugin::{__semio_dispatch_PluginApp, __value as semio_framework_value};
use semio_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::{ExecutionMode, Plugin, PluginApp};
use semio_plugin::{ArtifactEditor, ArtifactViewer, PluginBuilder};
use semio_s_artifact_layout_layout_editor::LayoutPlayApp;
use semio_s_artifact_layout_layout_viewer::LayoutViewer;

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the layout editor and viewer surfaces.
    pub enum LayoutApps: PluginApp {
        Editor(VcsArtifactApp<EditorApp<LayoutPlayApp>, <LayoutPlayApp as ArtifactEditor>::Members>),
        Viewer(VcsArtifactApp<ViewerApp<LayoutViewer>, <LayoutViewer as ArtifactViewer>::Members>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Registers Layout and its editor, viewer, document-write capability, and font assets.
pub fn plugin() -> Result<Plugin<LayoutApps>, PluginAssemblyError> {
    let mut builder = PluginBuilder::<_, LayoutApps>::new("layout")
        .label(semio_plugin::LocalizedLabel::native("Layout", "Layout"))
        .version("0.1.0")
        .package_id("semio:layout")
        .artifact(semio_s_artifact_layout_layout::declaration().map_err(PluginAssemblyError::definition)?)
        .editor::<semio_s_artifact_layout_layout_editor::LayoutPlayApp>(semio_s_artifact_layout_layout_editor::create_layout_app())
        .editor_mutation_roster::<semio_s_artifact_layout_layout_editor::LayoutPlayApp>()
        .viewer::<semio_s_artifact_layout_layout_viewer::LayoutViewer>(semio_s_artifact_layout_layout_viewer::create_layout_viewer())
        .viewer_mutation_roster::<semio_s_artifact_layout_layout_viewer::LayoutViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: <LayoutPlayApp as ArtifactEditor>::DIALECT.artifact_kind.into() })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist layout edits to the open document".into(), optional: false });
    for declaration in semio_s_artifact_layout_layout_editor::text_font_asset_declarations() {
        builder = builder.asset(declaration);
    }
    builder.try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests
