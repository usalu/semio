//! 💾️ Binary representation codec surface for `stdio.semio.drawing` (diff). The real
//! encode/decode is `SemioDrawingDiff`'s hand-rolled `protocol::DiffCodec` impl
//! (../🦀️.rs) -- no separate envelope, the bytes ARE the text-facet grammar verbatim.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
