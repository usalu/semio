//! 🧬️ PNG publication preparation: every PNG mutation is prepared by the central cold apply over the mutation's own cold semantics.
//!
//! The former hand-driven pixel-region cursor targeted the retired `RetainedCloneEdit` cursor contract; its bounded-turn laws are to be restated against the current contract (see the png ticket notes) rather than carried as dead code.
use crate::{schema::mutations::PngMutation, PngSnapshot};
use std::sync::Arc;

/// 🏭️ The preparation factory every PNG edit is routed through.
pub fn factory() -> Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<PngSnapshot, PngMutation>> {
    store::cold_mutation_apply_preparation_factory::<PngSnapshot, PngMutation>()
}
