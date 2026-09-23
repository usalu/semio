//! 🎚️ Exact store owners for the zero-payload configuration lane.
//!
//! ⛔️ `NoConfig` holds no payload, so its owners are the zero-payload catalogue the `NoDraft` lane
//! uses, never the page-charged `bounded_config_store_owners`: those charge
//! `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` per retired value and answer `Pending { 0, 0 }` to every
//! sub-page grant, so an app closing under a neutral 1-byte grant never finished its config lane
//! (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, `📓️flow.md` §8).

use super::no_draft_retirement::zero_payload_store_owners;
use crate::app::{bounded_config_store_disposer, ArtifactOwnedDisposer, NoConfig, NoConfigMutation};
use crate::store;

pub fn no_config_store_owners() -> store::DocumentStoreOwners<NoConfig, NoConfigMutation> {
    zero_payload_store_owners::<NoConfig, NoConfigMutation>()
}

pub fn no_config_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<NoConfig, NoConfigMutation>>> {
    bounded_config_store_disposer::<NoConfig, NoConfigMutation>()
}
