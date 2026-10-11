//! ⚡️ `trinity.graph` semantic mutation aggregate — the parent lane edits the document's own query only.
//!
//! The scene lives in the composed `content` child (`s.stdio.semio@v1/graph`); every scene edit is a child-lane leaf of
//! that shared vocabulary (design §20.15), so no parent leaf reads the child.

use crate::standards::v1::subsets::any::schema::diff::JackDiff;
use crate::JackSnapshot;

pub use super::set_query::{set_query, SetQuery};

//#region 🔖️Aggregate
/// 🧮️ Parent-lane trinity graph mutation vocabulary.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = JackSnapshot, diff = JackDiff, schema = "s.trinity.jack")]
pub enum TrinityGraphMutation {
    SetQuery(SetQuery),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
