//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, FlowExtensionDeclaration, FlowExtensionExecutableIdentity, FlowExtensionManifest, HostMediaHandlerDeclaration, NeedsLabel, Plugin, PluginApp, PluginBuilder};

//#region 🧮️GuestHeapWitness
//#endregion 🧮️GuestHeapWitness

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the procedural 2D and 3D surfaces.
    pub enum ProceduralApps: PluginApp {
        Generation2dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_procedural_2d_editor::Generation2dPlayApp>>),
        Generation2dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_procedural_2d_viewer::Generation2dViewer>>),
        Generation3dEditor(VcsArtifactApp<EditorApp<semio_s_artifact_procedural_3d_editor::Generation3dPlayApp>>),
        Generation3dViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_procedural_3d_viewer::Generation3dViewer>>),
    }
}
//#endregion 🗃️Apps

//#region 🎮️Commands
#[path = "🎮️commands/🦀️.rs"]
mod commands;
//#endregion 🎮️Commands

//#region 🌊️FlowExtensions
/// 🌊️ `(slug, extension id, label)` for every flow extension this plugin installs. The
/// slug is the only free variable in a declaration: the contribution id is
/// `s.procedural.flow-extension.<slug>` and the native executable is
/// `semio.s.plugin.flow.extension.<slug>`, so the table states each extension once instead of
/// spelling those three strings out nine times. Every extension is a member of this tree, so its
/// version is [`FLOW_EXTENSION_VERSION`]. `🎮️commands` reads the same table to answer
/// `listFlowExtensions`.
pub(crate) const FLOW_EXTENSIONS: [(&str, &str, &str); 9] = [
    ("brep", "brep", "Brep"),
    ("math", "math", "Math"),
    ("primitive", "core", "Core"),
    ("logic", "logic", "Logic"),
    ("dictionary", "dictionary", "Dictionary"),
    ("list", "list", "List"),
    ("text", "text", "Text"),
    ("draw", "draw", "Draw"),
    ("bim", "bim", "Bim"),
];

/// 📌️ The version every flow extension of [`FLOW_EXTENSIONS`] is built at — the tree's own workspace version.
pub(crate) const FLOW_EXTENSION_VERSION: &str = env!("CARGO_PKG_VERSION");

/// 🪪️ The stable contribution id one roster row is declared under.
pub(crate) fn flow_extension_declaration_id(slug: &str) -> String {
    format!("s.procedural.flow-extension.{slug}")
}

/// 🧩️ Builds every roster row's typed declaration, in table order.
fn flow_extension_declarations() -> Result<Vec<FlowExtensionDeclaration>, PluginAssemblyError> {
    FLOW_EXTENSIONS
        .iter()
        .map(|(slug, extension, label)| {
            let native = format!("semio.s.plugin.flow.extension.{slug}");
            FlowExtensionDeclaration::new(flow_extension_declaration_id(slug), FlowExtensionManifest::new(*extension, *label, FLOW_EXTENSION_VERSION)?, FlowExtensionExecutableIdentity::native(native.clone(), native, "register")?)
        })
        .collect()
}
//#endregion 🌊️FlowExtensions

/// 🔌️ Builds the plugin surface for host registration.
pub fn plugin() -> Result<Plugin<ProceduralApps>, PluginAssemblyError> {
    let mut builder = PluginBuilder::<NeedsLabel, ProceduralApps>::new("procedural")
        .label("Procedural")
        .version("0.1.0")
        .package_id("semio:procedural")
        .artifact(semio_s_artifact_procedural_generation2d::declaration().map_err(PluginAssemblyError::definition)?)
        .artifact(semio_s_artifact_procedural_generation3d::declaration().map_err(PluginAssemblyError::definition)?)
        .host_media_handler(HostMediaHandlerDeclaration::mesh_import(
            "s.procedural.host-media.mesh-import",
            semio_s_artifact_procedural_3d_model::artifact_kind(),
            semio_s_artifact_procedural_3d_model::GENERATION_3D_SCHEMA,
            semio_s_artifact_procedural_3d_mesh_bridge::generation3d_document_from_mesh,
        )?)
        .plugin_command(commands::list_flow_extensions_command(), Box::new(commands::list_flow_extensions))
        .editor::<semio_s_artifact_procedural_2d_editor::Generation2dPlayApp>(semio_s_artifact_procedural_2d_editor::create_generation2d_app())
        .editor_mutation_roster::<semio_s_artifact_procedural_2d_editor::Generation2dPlayApp>()
        .viewer::<semio_s_artifact_procedural_2d_viewer::Generation2dViewer>(semio_s_artifact_procedural_2d_viewer::create_generation2d_viewer())
        .viewer_mutation_roster::<semio_s_artifact_procedural_2d_viewer::Generation2dViewer>()
        .editor::<semio_s_artifact_procedural_3d_editor::Generation3dPlayApp>(semio_s_artifact_procedural_generation3d::manifest::create_generation3d_app())
        .editor_mutation_roster::<semio_s_artifact_procedural_3d_editor::Generation3dPlayApp>()
        .viewer::<semio_s_artifact_procedural_3d_viewer::Generation3dViewer>(semio_s_artifact_procedural_3d_viewer::create_generation3d_viewer())
        .viewer_mutation_roster::<semio_s_artifact_procedural_3d_viewer::Generation3dViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_procedural_2d_model::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_procedural_3d_model::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("artifacts.write".into()),
            scope: "plugin".into(),
            reason: "persist generation2d/generation3d editor edits to the open document".into(),
            optional: false,
        });
    for declaration in flow_extension_declarations()? {
        builder = builder.flow_extension(declaration);
    }
    builder.try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests

