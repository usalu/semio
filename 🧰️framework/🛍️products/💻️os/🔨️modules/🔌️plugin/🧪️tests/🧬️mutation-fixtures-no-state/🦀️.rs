//! 🫧️ Exact no-state store retirement owners for the keyed app fixture: the framework's own zero-payload owners.

use crate::app::{ArtifactOwnedDisposer, NoPresence, NoPresenceMutation};
use crate::store;

//#region 🧹️NoStateFixtureOwners
pub(crate) fn presence_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::PresenceStore<NoPresence, NoPresenceMutation>>> {
    crate::no_presence_store_disposer()
}

pub(crate) fn transient_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::TransientStore<crate::app::NoTransient, crate::app::NoTransientMutation>>> {
    crate::no_transient_store_disposer()
}

pub(crate) fn presence_peer_retirement_factory() -> std::sync::Arc<dyn store::SnapshotRetirementFactory<NoPresence>> {
    crate::no_presence_peer_retirement_factory()
}

pub(crate) fn presence_local_root_retirement_factory() -> std::sync::Arc<dyn store::SnapshotRetirementFactory<NoPresence>> {
    crate::no_presence_local_root_retirement_factory()
}

pub(crate) fn transient_local_root_retirement_factory() -> std::sync::Arc<dyn store::SnapshotRetirementFactory<crate::app::NoTransient>> {
    crate::no_transient_local_root_retirement_factory()
}
//#endregion 🧹️NoStateFixtureOwners
