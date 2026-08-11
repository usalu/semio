//! 💾️ Binary representation grammar surface for `stdio.semio.video` (diff): UTF-8 bytes of the
//! text grammar verbatim, per `protocol::DiffCodec::encode_diff`'s documented simplification —
//! actual encode/decode lives on `SemioVideoDiff`'s `protocol::DiffCodec` impl in the facet root
//! `🦀️component.rs`; this leaf carries the normative protocol description.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
