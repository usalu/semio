//! 🏠️ Space Home semantic mutation aggregate.

use crate::{SHomeDiff, SHomeSnapshot};

pub use super::change_catalog_generation::{change_catalog_generation, ChangeCatalogGeneration};
pub use crate::standards::v1::subsets::any::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Home launcher mutation vocabulary backed by its direct semantic owner.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = SHomeSnapshot, diff = SHomeDiff, schema = "s.space.home")]
pub enum SHomeMutation {
    ChangeCatalogGeneration(ChangeCatalogGeneration),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
