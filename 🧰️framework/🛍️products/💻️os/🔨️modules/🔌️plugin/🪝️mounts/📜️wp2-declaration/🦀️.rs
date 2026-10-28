// 📜️ HUB WP2: mount hub of WP2 declaration contracts: decl-views, decl-emission, decl-app, decl-surface, decl-registry, app-builder, plugin-program.
// This file is spliced into the plugin crate root by `include!` from the root, so every item below stays a crate-root item;
// its `#[path]` literals resolve against this folder (`🪝️mounts/<hub>/`), hence the `../../` prefix of the plugin folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-PLUGIN-CRATES.md, section 12.5).

//#region 📄️DeclarationFixtureMutationMount

//#endregion 📄️DeclarationFixtureMutationMount
//#region 🏛️OwnershipFacet

//#endregion 🏛️OwnershipFacet
//#region 📜️DeclarationFacet
pub(crate) extern crate semio_framework_plugin_app_builder as app_builder;
pub(crate) mod app_contract {
    pub use semio_framework_plugin_decl_app::*;
    pub use semio_framework_plugin_document_media::{MediaArtifact, whole_document_media};
    pub use semio_framework_plugin_document_media::copy_media_text_parts;
    #[cfg(test)]
    pub(crate) use semio_framework_plugin_document_media::{receive_media_artifact_fault, receive_media_output};
}
pub(crate) mod app_views {
    pub use semio_framework_plugin_decl_views::*;
    pub use semio_framework_plugin_operation::{ArtifactOperationProgress, CANCEL_TYPED_OPERATION_ACTION_ID};
}
pub(crate) extern crate semio_framework_plugin_decl_registry as artifact_declaration;
pub(crate) use semio_framework_plugin_decl_registry::artifact_registration;
pub(crate) use semio_framework_plugin_decl_views::child_view;
pub(crate) use semio_framework_plugin_decl_views::gesture_contract;
pub(crate) use semio_framework_plugin_decl_views::history_view;
pub(crate) mod media_contract {
    pub use semio_framework_plugin_decl_views::media_contract::*;
    pub use semio_framework_plugin_decl_app::media_contract::{ArtifactMediaExportJobRequest, ArtifactMediaExportRequestRetirement};
}
pub(crate) use semio_framework_plugin_decl_surface::natural_file_import;
pub(crate) use semio_framework_plugin_decl_views::presence_retirement;
pub use presence_retirement::{
    bounded_presence_root_retirement_factory, bounded_presence_store_disposer, no_presence_local_root_retirement_factory, no_presence_peer_retirement_factory, no_presence_store_disposer, NoPresenceRetirementFactory, PresenceStoreOwnedDisposer,
};
pub(crate) mod surface_contract {
    pub use semio_framework_plugin_decl_surface::*;
    pub use semio_framework_plugin_decl_emission::surface_contract::ViewEmit;
}
pub(crate) mod tool_run_contract {
    pub use semio_framework_plugin_decl_views::tool_run_contract::*;
    pub use semio_framework_plugin_decl_app::tool_run_contract::ToolRunJobRequest;
}
pub(crate) mod app_emission {
    pub use semio_framework_plugin_decl_emission::*;
    pub use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
}
pub(crate) mod tool_contract {
    pub use semio_framework_plugin_decl_app::tool_contract::*;
    pub use semio_framework_plugin_decl_views::tool_contract::{ArtifactInstanceOperationOwner, InstanceInferenceSessions, original_inference_close_demands, original_inference_close_step, ArtifactInstanceOperationOwnerHandle, ArtifactInferenceSessionAccess, ArtifactInstanceOperationOwnerAliasRetirement, EmptyArtifactInstanceOperationOwner};
}
//#endregion 📜️DeclarationFacet
//#region 📬️PublicationFacet
pub(crate) use semio_framework_plugin_decl_views::no_config_retirement;
pub use no_config_retirement::{no_config_store_disposer, no_config_store_owners};
pub(crate) use semio_framework_plugin_decl_views::no_draft_retirement;
pub use no_draft_retirement::{no_draft_store_disposer, no_draft_store_owners};
pub(crate) use semio_framework_plugin_decl_views::transient_retirement;
pub use transient_retirement::{no_transient_local_root_retirement_factory, no_transient_store_disposer, NoTransientRetirementFactory};
//#endregion 📬️PublicationFacet
//#region ⚛️ReactorFacet
pub(crate) mod plugin_program {
    pub use semio_framework_plugin_plugin_program::*;
    pub use semio_framework_plugin_decl_registry::plugin_program::AppFactory;
}
//#endregion ⚛️ReactorFacet
//#region 🧵️InstanceFacet
pub mod declarations {
    pub use semio_framework_plugin_decl_registry::declarations::*;
    pub(crate) use semio_framework_plugin_extension::{commit_artifact_declarations, preflight_artifact_declarations};
    pub use semio_framework_plugin_instance_surface::{editor_surface, viewer_surface, native_editor_surface, native_viewer_surface, EditorSurfaceApp, ViewerSurfaceApp};
}
//#endregion 🧵️InstanceFacet
//#region 🏛️OwnershipFacet

//#endregion 🏛️OwnershipFacet
//#region 🧪️WP2TestHosts









//#endregion 🧪️WP2TestHosts
