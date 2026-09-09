//! 🧪️ Exact scalar window fixtures use the production Store preparation lifecycle.

use super::window_transient::WindowTransientOwnerBundle;
use crate::publication_fixture::{PublicationTransient as State, PublicationTransientMutation as Mutation};
use crate::store;
use std::sync::Arc;

const _: () = assert!(!std::mem::needs_drop::<State>() && !std::mem::needs_drop::<Mutation>());
const _: () = assert!(std::mem::size_of::<State>() <= 16 && std::mem::size_of::<Mutation>() <= 16);

impl store::retirement::RetireOwned for State {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> { store::retirement::leaf(self.revision) }
}

impl store::retirement::RetireOwned for Mutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        let Self::ChangePublicationTransient(value) = self;
        store::retirement::leaf(value.revision)
    }
}

fn footprint(_: &Mutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: std::mem::size_of::<Mutation>() })
}

fn transfer(mutation: Mutation) -> State {
    let Mutation::ChangePublicationTransient(value) = mutation;
    State { revision: value.revision }
}

/// 📦️ Shares the exact production lifecycle across window and document replacement laws.
pub(crate) fn owners() -> WindowTransientOwnerBundle<State, Mutation> {
    let state: Arc<dyn store::ArtifactOwnedValueRetirementFactory<State>> = Arc::new(store::retirement::OwnedValueRetirementFactory::default());
    let mutation: Arc<dyn store::ArtifactOwnedValueRetirementFactory<Mutation>> = Arc::new(store::retirement::OwnedValueRetirementFactory::default());
    let preparation = Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(footprint, transfer, state.clone(), mutation.clone()));
    WindowTransientOwnerBundle::new(preparation, state, mutation)
}
