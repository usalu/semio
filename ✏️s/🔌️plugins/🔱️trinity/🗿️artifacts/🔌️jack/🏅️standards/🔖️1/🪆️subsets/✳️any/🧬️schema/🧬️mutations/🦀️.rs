//! ⚡️ `trinity.graph` semantic mutation aggregate.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️.rs` leaf.

use crate::standards::v1::subsets::any::schema::diff::JackDiff;
use crate::JackSnapshot;

pub use super::change_data_property::{change_data_property, ChangeDataProperty};
pub use super::create_edge::{create_edge, CreateEdge};
pub use super::create_node::{create_node, CreateNode};
pub use super::delete_edge::{delete_edge, DeleteEdge};
pub use super::delete_node::{delete_node, DeleteNode};
pub use super::move_node::{move_node, MoveNode};
pub use super::remove_data_property::{remove_data_property, RemoveDataProperty};
pub use super::rename_node::{rename_node, RenameNode};

//#region 🔖️Aggregate
/// 🧮️ Semantic trinity graph mutation vocabulary.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = JackSnapshot, diff = JackDiff, schema = "s.trinity.jack")]
pub enum TrinityGraphMutation {
    CreateNode(CreateNode),
    DeleteNode(DeleteNode),
    CreateEdge(CreateEdge),
    DeleteEdge(DeleteEdge),
    RenameNode(RenameNode),
    MoveNode(MoveNode),
    ChangeDataProperty(ChangeDataProperty),
    RemoveDataProperty(RemoveDataProperty),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
