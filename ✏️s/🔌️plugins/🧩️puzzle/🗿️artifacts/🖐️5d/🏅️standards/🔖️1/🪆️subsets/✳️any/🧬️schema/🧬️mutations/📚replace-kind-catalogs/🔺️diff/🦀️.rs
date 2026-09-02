//! 🔺️ Sparse diff builder for `ReplaceKindCatalogs` — patches the document `kindCatalogs`. The
//! payload keeps its pre-migration `Option<Puzzle5dKindCatalogs>` shape (recipe's "granular
//! mutations keep unchanged public payload shapes"); internally it now splits into the composed
//! `kind_catalogs` handle + `kind_catalogs_extra` overflow the snapshot actually carries, minting a
//! fresh content-addressed handle and seeding the working-scene cache so the diff is resolvable
//! immediately (see `🗿️artifacts/🖐️5d/🦀️.rs`'s `🔖️KindCatalogComposition` region).
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::split_and_seed_kind_catalogs;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceKindCatalogs, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let (kind_catalogs, kind_catalogs_extra) = split_and_seed_kind_catalogs(payload.new_catalogs.clone());
    // 🗂️ Content-addressed: identical catalog content always mints the same `child_id`, so comparing
    // the minted handle's id (plus the puzzle5d-owned overflow half) against `base` is a pure,
    // deterministic no-op check that never touches the ephemeral scratch cache.
    let new_handle_id = kind_catalogs.as_ref().map(|handle| handle.child_id.clone());
    let base_handle_id = base.kind_catalogs.as_ref().map(|handle| handle.child_id.clone());
    if new_handle_id == base_handle_id && kind_catalogs_extra == base.kind_catalogs_extra {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Kind catalogs are unchanged.");
    }
    protocol::MutationOutcome::new(Puzzle5dDiff { kind_catalogs: Some(kind_catalogs), kind_catalogs_extra: Some(kind_catalogs_extra), ..Default::default() })
}
//#endregion 🔖️Diff
