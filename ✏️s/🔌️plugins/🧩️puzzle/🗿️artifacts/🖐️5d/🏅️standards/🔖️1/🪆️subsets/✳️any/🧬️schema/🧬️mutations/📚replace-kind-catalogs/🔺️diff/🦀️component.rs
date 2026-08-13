//! 🔺️ Sparse diff builder for `ReplaceKindCatalogs` — patches the document `kindCatalogs`. The
//! payload keeps its pre-migration `Option<Puzzle5dKindCatalogs>` shape (recipe's "granular
//! mutations keep unchanged public payload shapes"); internally it now splits into the composed
//! `kind_catalogs` handle + `kind_catalogs_extra` overflow the snapshot actually carries, minting a
//! fresh content-addressed handle and seeding the working-scene cache so the diff is resolvable
//! immediately (see `🗿️artifacts/🖐️5d/🦀️component.rs`'s `🔖️KindCatalogComposition` region).
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::split_and_seed_kind_catalogs;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceKindCatalogs, _base: &Puzzle5dSnapshot) -> Puzzle5dDiff {
    let (kind_catalogs, kind_catalogs_extra) = split_and_seed_kind_catalogs(payload.new_catalogs.clone());
    Puzzle5dDiff { kind_catalogs: Some(kind_catalogs), kind_catalogs_extra: Some(kind_catalogs_extra), ..Default::default() }
}
//#endregion 🔖️Diff
