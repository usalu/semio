//! binary rep for stdio.html 🧬️mutations -- see the sibling `encode_op`/`decode_op` two levels up.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v5::subsets::any::schema::mutations::*;
use crate::standards::v5::subsets::any::io::text::diff::{dec_html_node};
use crate::standards::v5::subsets::any::io::text::diff::{enc_html_node};
use crate::standards::v5::subsets::any::io::text::diff::{strip_brackets};
use crate::standards::v5::subsets::any::io::text::diff::{split_top_level};
use crate::standards::v5::subsets::any::io::text::diff::{decode_option};
use crate::standards::v5::subsets::any::io::text::diff::{encode_option};
use crate::standards::v5::subsets::any::io::text::diff::{dec_str};
use crate::standards::v5::subsets::any::io::text::diff::{enc_str};
use crate::standards::v5::subsets::any::schema::diff::{diff_at_path, HtmlAttrAdded, HtmlAttrModified, HtmlAttributesDiff, HtmlChildAdded, HtmlChildrenDiff, HtmlDiff, HtmlElementDiff, HtmlNodeDiff};
use crate::standards::v5::subsets::any::schema::snapshot::{element_attr, node_at, HtmlNode, HtmlSnapshot, NodePath};
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use semio_s_artifact_stdio_contract::deserialize_double_option;

/// ⚡️ Binary = the text bytes verbatim, same simplification as `HtmlDiff`'s hand-rolled codec.
impl OpBinary for HtmlMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::tagged_text_binary::encode_line(WIRE_PROTOCOL, &self.print_op())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = dsl::tagged_text_binary::decode_line(WIRE_PROTOCOL, bytes)?;
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ `HtmlMutation`'s wire protocol: its `record <kind> tag=<n>` lines are the only source of the op tags.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
//#endregion 🏷️WireTags
