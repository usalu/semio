//! ⚖️ Sourcing curation artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`).
//!
//! The app's typed `SourcingCurationCommand` enum — which used to share the old `📡️protocol` crate with
//! this codec — is an APP concern, not an artifact one: it now lives in `🎛️apps/🗂️curation/🦀️.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::mutations::SourcingMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `SourcingMutation` to its binary state-patch form.
pub fn encode_op(operation: &SourcingMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `SourcingMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<SourcingMutation, protocol::ProtocolError> {
    SourcingMutation::decode_op(bytes)
}

//#region 🧪️Tests
//#endregion 🧪️Tests


pub const BINARY_TAGS: &[(&str, u8)] =
    &[("CreateCuratedItem", create_curated_item::BINARY_TAG), ("DeleteCuratedItem", delete_curated_item::BINARY_TAG), ("ChangeCuratedItemCount", change_curated_item_count::BINARY_TAG)];

pub use semio_s_artifact_sourcing_curation_examples::standards::v1::subsets::any::io::binary::mutations::delete_curated_item;

pub use semio_s_artifact_sourcing_curation_examples::standards::v1::subsets::any::io::binary::mutations::create_curated_item;

pub use semio_s_artifact_sourcing_curation_examples::standards::v1::subsets::any::io::binary::mutations::change_curated_item_count;

mod native_codec {
use super::*;
use crate::schema::mutations::SourcingMutation;
#[cfg(test)] use crate::schema::mutations::{change_curated_item_count, create_curated_item, delete_curated_item};
#[cfg(test)] use crate::CuratedItem;
#[cfg(test)] use protocol::OpText;
use crate::standards::v1::subsets::any::io::text::mutations::{SourcingMutationDsl,sourcing_mutation_to_dsl,sourcing_mutation_from_dsl};

impl OpBinary for SourcingMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

impl OpBinary for SourcingMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        sourcing_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(sourcing_mutation_from_dsl(SourcingMutationDsl::decode_op(bytes)?))
    }
}
}
