//! 🗂️ Sourcing curation semantic mutation aggregate.

use crate::{CurationDiff, CurationSnapshot};

pub use crate::standards::v1::subsets::any::schema::mutations::change_curated_item_count::ChangeCuratedItemCount;
pub use crate::standards::v1::subsets::any::schema::mutations::create_curated_item::{create_curated_item, create_curated_item_at, CreateCuratedItem};
pub use crate::standards::v1::subsets::any::schema::mutations::delete_curated_item::DeleteCuratedItem;
pub use crate::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Closed curated-selection mutation vocabulary backed by direct semantic owners.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = CurationSnapshot, diff = CurationDiff, schema = "sourcing.curation")]
pub enum SourcingMutation {
    CreateCuratedItem(CreateCuratedItem),
    DeleteCuratedItem(DeleteCuratedItem),
    ChangeCuratedItemCount(ChangeCuratedItemCount),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence

//#endregion 🧪️StructuralCorrespondence

//#region 🔖️ConcreteBuilders
/// 🏗️ Builder of the `change_curated_item_count` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_curated_item_count(object_id: String, new_count: u32) -> SourcingMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_curated_item_count::change_curated_item_count::<SourcingMutation>(object_id, new_count)
}

/// 🏗️ Builder of the `delete_curated_item` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn delete_curated_item(object_id: String) -> SourcingMutation {
    crate::standards::v1::subsets::any::schema::mutations::delete_curated_item::delete_curated_item::<SourcingMutation>(object_id)
}
//#endregion 🔖️ConcreteBuilders
