//! 🧬️ BMP publication preparation: every BMP mutation is prepared by the central cold apply over the mutation's own cold semantics.
use crate::{schema::mutations::BmpMutation, BmpSnapshot};
use std::sync::Arc;

/// 🏭️ The document lane's preparation authority: the shared cold apply over [`BmpMutation`]'s cold reference semantics.
pub fn factory()->Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<BmpSnapshot,BmpMutation>>{store::cold_mutation_apply_preparation_factory::<BmpSnapshot,BmpMutation>()}
