//! 💾️ Binary representation grammar surface for `stdio.semio.video` (mutations): UTF-8 bytes of
//! the text op grammar verbatim, per `protocol::OpBinary::encode_op`'s documented simplification
//! — actual encode/decode lives on `SemioVideoMutation`'s `protocol::OpBinary` impl in the facet
//! root `🦀️component.rs`; this leaf carries the normative protocol description.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
