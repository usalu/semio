//! 🧪️ Exact scalar window fixtures use the production Store preparation lifecycle.

use semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerBundle;
use crate::publication_fixture::transient::PublicationTransient as State;
use crate::publication_fixture::transient::mutations::PublicationTransientMutation as Mutation;
use semio_framework_os_kernel as store;
use std::sync::Arc;

#[doc(hidden)]
pub const _: () = assert!(!std::mem::needs_drop::<State>() && !std::mem::needs_drop::<Mutation>());
#[doc(hidden)]
pub const _: () = assert!(std::mem::size_of::<State>() <= 16 && std::mem::size_of::<Mutation>() <= 16);

#[doc(hidden)]
pub fn footprint(_: &Mutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_ephemeral_item(std::mem::size_of::<Mutation>()))
}

#[doc(hidden)]
pub fn transfer(mutation: Mutation) -> State {
    let Mutation::ChangePublicationTransient(value) = mutation;
    State { revision: value.revision }
}

/// 📦️ Shares the exact production lifecycle across window and document replacement laws.
#[doc(hidden)]
pub fn owners() -> WindowTransientOwnerBundle<State, Mutation> {
    let state: Arc<dyn store::ArtifactOwnedValueRetirementFactory<State>> = Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::default());
    let mutation: Arc<dyn store::ArtifactOwnedValueRetirementFactory<Mutation>> = Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::default());
    let preparation = Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(footprint, transfer, state.clone(), mutation.clone()));
    WindowTransientOwnerBundle::new(preparation, state, mutation)
}
