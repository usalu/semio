//! 🫧️ Exact no-state store retirement owners for the keyed app fixture: the framework's own zero-payload owners.

use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_decl_views::{NoPresence, NoPresenceMutation};
use semio_framework_os_kernel as store;

//#region 🧹️NoStateFixtureOwners
#[doc(hidden)]
pub fn presence_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::PresenceStore<NoPresence, NoPresenceMutation>>> {
    semio_framework_plugin_decl_views::presence_retirement::no_presence_store_disposer()
}

#[doc(hidden)]
pub fn transient_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::TransientStore<semio_framework_plugin_decl_views::NoTransient, semio_framework_plugin_decl_views::NoTransientMutation>>> {
    semio_framework_plugin_decl_views::transient_retirement::no_transient_store_disposer()
}

#[doc(hidden)]
pub fn presence_peer_retirement_factory() -> std::sync::Arc<dyn store::SnapshotRetirementFactory<NoPresence>> {
    semio_framework_plugin_decl_views::presence_retirement::no_presence_peer_retirement_factory()
}

#[doc(hidden)]
pub fn presence_local_root_retirement_factory() -> std::sync::Arc<dyn store::SnapshotRetirementFactory<NoPresence>> {
    semio_framework_plugin_decl_views::presence_retirement::no_presence_local_root_retirement_factory()
}

#[doc(hidden)]
pub fn transient_local_root_retirement_factory() -> std::sync::Arc<dyn store::SnapshotRetirementFactory<semio_framework_plugin_decl_views::NoTransient>> {
    semio_framework_plugin_decl_views::transient_retirement::no_transient_local_root_retirement_factory()
}
//#endregion 🧹️NoStateFixtureOwners
