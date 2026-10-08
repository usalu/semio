//! binary rep for stdio.epw 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::energyplus::subsets::any::schema::mutations::*;
use crate::standards::energyplus::subsets::any::schema::diff::{EpwDiff, EpwRecordAdded, EpwRecordDiff, EpwRecordModified, EpwRecordsDiff};
use crate::standards::energyplus::subsets::any::io::text::diff::{dec_record};
use crate::standards::energyplus::subsets::any::io::text::diff::{enc_record};
use crate::standards::energyplus::subsets::any::io::text::diff::{strip_brackets};
use crate::standards::energyplus::subsets::any::io::text::diff::{split_top_level};
use crate::standards::energyplus::subsets::any::io::text::diff::{dec_data_periods};
use crate::standards::energyplus::subsets::any::io::text::diff::{enc_data_periods};
use crate::standards::energyplus::subsets::any::io::text::diff::{dec_location};
use crate::standards::energyplus::subsets::any::io::text::diff::{enc_location};
use crate::standards::energyplus::subsets::any::io::text::diff::{dec_str};
use crate::standards::energyplus::subsets::any::io::text::diff::{enc_str};
use crate::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot};
use protocol::OpBinary;
use protocol::{Mutation, OpText};

/// ⚡️ Binary = the text bytes verbatim, same simplification as `EpwDiff`'s hand-rolled codec.
impl OpBinary for EpwMutation {
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
/// 🏷️ `EpwMutation`'s wire protocol: its `record <kind> tag=<n>` lines are the only source of the op tags.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
//#endregion 🏷️WireTags
