//! 🧬️ BMP publication preparation: every BMP mutation is prepared by the central cold apply over the mutation's own cold semantics.
//!
//! The former hand-driven pixel-region cursor targeted the retired `RetainedCloneEditCursor` contract; its bounded-turn laws are to be restated against the current contract rather than carried as dead code.
use crate::{schema::mutations::BmpMutation, BmpSnapshot};
use std::sync::Arc;

/// 🏭️ The preparation factory every BMP edit is routed through.
pub fn factory() -> Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<BmpSnapshot, BmpMutation>> {
    store::cold_mutation_apply_preparation_factory::<BmpSnapshot, BmpMutation>()
}
