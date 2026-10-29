//! 🧬️ PNG publication preparation: every PNG mutation is prepared by the central cold apply over the mutation's own cold semantics.
use crate::{schema::mutations::PngMutation, PngSnapshot};
use std::sync::Arc;

/// 🏭️ The document lane's preparation authority: the shared cold apply over [`PngMutation`]'s cold reference semantics.
pub fn factory()->Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<PngSnapshot,PngMutation>>{store::cold_mutation_apply_preparation_factory::<PngSnapshot,PngMutation>()}
