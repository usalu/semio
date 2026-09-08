//! 🪐️ S Space semantic mutation aggregate.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️.rs` leaf.

use crate::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

pub use super::create_artifact::{create_artifact, CreateArtifact};
pub use super::delete_artifact::{delete_artifact, DeleteArtifact};
pub use super::rename_artifact::{rename_artifact, RenameArtifact};
pub use super::touch_artifact::{touch_artifact, TouchArtifact};
pub use crate::standards::v1::subsets::any::schema::operations::*;

//#region 🔖️Aggregate
/// 🧮️ Semantic S Space index mutation vocabulary.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = SSpaceSnapshot, diff = SSpaceDiff, schema = "s.space.space")]
pub enum SSpaceMutation {
    CreateArtifact(CreateArtifact),
    DeleteArtifact(DeleteArtifact),
    RenameArtifact(RenameArtifact),
    TouchArtifact(TouchArtifact),
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
