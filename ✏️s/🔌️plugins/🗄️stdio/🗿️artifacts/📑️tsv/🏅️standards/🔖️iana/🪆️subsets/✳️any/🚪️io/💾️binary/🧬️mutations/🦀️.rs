//! binary rep for stdio.tsv 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::iana::subsets::any::schema::mutations::*;
use crate::standards::iana::subsets::any::schema::diff::{TsvDiff, TsvRowAdded, TsvRowDiff, TsvRowModified, TsvRowsDiff};
use crate::standards::iana::subsets::any::io::text::diff::{dec_str};
use crate::standards::iana::subsets::any::io::text::diff::{enc_str};
use crate::standards::iana::subsets::any::io::text::diff::{dec_row};
use crate::standards::iana::subsets::any::io::text::diff::{enc_row};
use crate::standards::iana::subsets::any::io::text::diff::{strip_brackets};
use crate::standards::iana::subsets::any::io::text::diff::{split_top_level};
use crate::standards::iana::subsets::any::schema::snapshot::{LineEnding, TsvSnapshot};
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};

/// ⚡️ Binary = the text bytes verbatim, same simplification as `TsvDiff`'s hand-rolled codec.
impl OpBinary for TsvMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::tagged_value_binary::encode_op(WIRE_PROTOCOL, dsl::tagged_value_binary::VariantTag::Field("mutation"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::tagged_value_binary::decode_op(WIRE_PROTOCOL, dsl::tagged_value_binary::VariantTag::Field("mutation"), bytes)
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ `TsvMutation`'s wire protocol: its `record <kind> tag=<n>` lines are the only source of the op tags.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
//#endregion 🏷️WireTags
