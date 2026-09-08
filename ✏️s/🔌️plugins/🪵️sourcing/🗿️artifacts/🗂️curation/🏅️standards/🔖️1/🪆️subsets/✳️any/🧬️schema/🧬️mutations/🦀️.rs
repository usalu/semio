//! 🗂️ Sourcing curation semantic mutation aggregate.

use crate::{CurationDiff, CurationSnapshot};

pub use super::change_curated_item_count::{change_curated_item_count, ChangeCuratedItemCount};
pub use super::create_curated_item::{create_curated_item, CreateCuratedItem};
pub use super::delete_curated_item::{delete_curated_item, DeleteCuratedItem};
pub use crate::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Closed curated-selection mutation vocabulary backed by direct semantic owners.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = CurationSnapshot, diff = CurationDiff, schema = "sourcing.curation")]
pub enum SourcingMutation {
    CreateCuratedItem(CreateCuratedItem),
    DeleteCuratedItem(DeleteCuratedItem),
    ChangeCuratedItemCount(ChangeCuratedItemCount),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
