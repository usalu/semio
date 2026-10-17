//! 🪟️ Private editor/viewer fixture owner; public surface is the committed mutation codec only.
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as dsl;
use semio_framework_plugin_decl_app::bounded_first_step_tool_proofs;
use semio_framework_plugin_decl_app::owned_context;
use semio_framework_plugin_decl_views as app_views;
use semio_framework_plugin_decl_views::{child_view,history_view};
use semio_framework_plugin_window_transient as window_surface;
use semio_framework_plugin_window_transient::window_transient;
use semio_framework_plugin_reactor_app as plugin_app;
use semio_framework_plugin_app_builder as app_builder;
use semio_framework_plugin_retained_command as retained_command;
use semio_framework_plugin_decl_emission::child_emit_preparation;
use semio_framework_plugin_decl_views::presence_retirement::{no_presence_store_disposer,no_presence_peer_retirement_factory,no_presence_local_root_retirement_factory};
use semio_framework_plugin_decl_views::transient_retirement::{no_transient_store_disposer,no_transient_local_root_retirement_factory};
mod app_contract {
 pub(crate) use semio_framework_plugin_decl_app::*;
 pub(crate) use semio_framework_plugin_document_media::MediaArtifact;
}
mod surface_contract {
 pub(crate) use semio_framework_plugin_decl_surface::*;
 pub(crate) use semio_framework_plugin_decl_emission::surface_contract::ViewEmit;
}
mod tool_contract {
 pub(crate) use semio_framework_plugin_decl_app::tool_contract::*;
 pub(crate) use semio_framework_plugin_decl_views::tool_contract::{ArtifactInstanceOperationOwner,ArtifactInstanceOperationOwnerHandle,EmptyArtifactInstanceOperationOwner};
}
mod app_emission {
 pub(crate) use semio_framework_plugin_decl_emission::*;
 pub(crate) use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
}
mod child_content {
 pub(crate) use semio_framework_plugin_composition_publication::group::{PrivateChildGroupSource,PrivateChildPublicationGroup};
 pub(crate) use semio_framework_plugin_composition_publication::group::private_publication_group::PrivateOwnedPublicationLane;
}
#[path="../🧬️mutation-fixtures-wire/🦀️.rs"]
mod wire_fixture;
#[path="../🧬️mutation-fixtures-job-close/🦀️.rs"]
mod job_close_fixture;
#[path="../🧬️mutation-fixtures-no-state/🦀️.rs"]
mod no_state_fixture;
mod app_testing {
 pub(crate) use semio_framework_plugin_testing_laws::artifact_app_laws;
 pub(crate) mod mutation_fixture {
  pub(crate) use crate::{wire_fixture as wire,job_close_fixture as job_close,no_state_fixture as no_state};
 }
}
#[path="../🧬️mutation-fixtures-surface/🦀️.rs"]
mod surface;
pub use surface::SurfaceMutation;
#[cfg(test)]
#[global_allocator]
static SURFACE_FIXTURE_HEAP_WITNESS:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
