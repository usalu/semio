//! 🔺️ Sparse diff builder for `ChangeCatalogGeneration` — a real single-field delta, built
//! directly from the payload (never apply-then-capture).
use crate::artifacts::home::diff::SHomeDiff;
use crate::artifacts::home::SHomeSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeCatalogGeneration, base: &SHomeSnapshot) -> protocol::MutationOutcome<SHomeDiff> {
    if base.catalog_generation == payload.new_catalog_generation {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Catalog generation is already {}.", payload.new_catalog_generation));
    }
    protocol::MutationOutcome::new(SHomeDiff { catalog_generation: Some(payload.new_catalog_generation), ..Default::default() })
}
//#endregion 🔖️Diff
