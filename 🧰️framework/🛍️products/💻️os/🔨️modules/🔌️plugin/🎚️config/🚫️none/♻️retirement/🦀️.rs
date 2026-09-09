//! 🎚️ Exact store owners for the zero-payload configuration lane.

use crate::app::{bounded_config_store_disposer, bounded_config_store_owners, ArtifactOwnedDisposer, NoConfig, NoConfigMutation};
use crate::store;

pub fn no_config_store_owners() -> store::MemberStoreOwners<NoConfig, NoConfigMutation> {
    bounded_config_store_owners::<NoConfig, NoConfigMutation>()
}

pub fn no_config_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<NoConfig, NoConfigMutation>>> {
    bounded_config_store_disposer::<NoConfig, NoConfigMutation>()
}
